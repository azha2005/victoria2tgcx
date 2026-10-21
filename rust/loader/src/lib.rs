//! Rust port of dll/tgcx/loader/tgcx_loader.cpp (MOD-2 loader, D15, D16).
//!
//! lua51.dll forwards the original's 128 Lua exports to lua51_real.dll (see build.rs)
//! and loads tgcx.dll from the folder of a ticked TGCX package, found through the
//! -mod= list (last first). With no package ticked it is a plain Lua proxy and says
//! so in Logs\tgcx.log.

// Only the Win32 build has an entry point; host builds run the parsing tests.
#![cfg_attr(any(test, not(target_arch = "x86")), allow(dead_code))]

mod mods;

use std::ffi::{CStr, c_char, c_void};
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_FILE_NOT_FOUND, FreeLibrary, GENERIC_READ, GENERIC_WRITE, GetLastError,
    HANDLE, HINSTANCE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Globalization::{CP_ACP, MultiByteToWideChar};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_SHARE_READ, OPEN_EXISTING, ReadFile,
    WriteFile,
};
use windows_sys::Win32::System::Environment::GetCommandLineA;
use windows_sys::Win32::System::LibraryLoader::{
    GetModuleFileNameW, GetProcAddress, LOAD_WITH_ALTERED_SEARCH_PATH, LoadLibraryExW,
};
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

use mods::{MAX_PATH, PACKAGE_SIZE, mod_value, parse_mod_args};

#[cfg(not(test))]
include!(concat!(env!("OUT_DIR"), "/forwards.rs"));

// tools/install_dll.py recognises the loader by this marker; tgcx.dll logs it.
const MARKER: &CStr = c"TGCX loader MOD-2 0.8.0";

const BACKSLASH: u16 = b'\\' as u16;

// A #[no_mangle] DllMain would also be exported, and install_dll.py accepts only the
// original's 128 exports, so the symbol the CRT calls is defined here instead.
#[cfg(all(not(test), target_arch = "x86"))]
std::arch::global_asm!(
    ".globl \"_DllMain@12\"",
    "\"_DllMain@12\":",
    "jmp {dll_main}",
    dll_main = sym dll_main,
);

extern "system" fn dll_main(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        attach();
    }
    1
}

fn attach() {
    let Some(directory) = exe_directory() else {
        return;
    };
    // SAFETY: the process command line is a NUL-terminated string that lives forever.
    let command_line = unsafe { CStr::from_ptr(GetCommandLineA().cast()) };

    let mut error = ERROR_FILE_NOT_FOUND;
    let logic = parse_mod_args(command_line.to_bytes())
        .into_iter()
        .rev()
        .find_map(|file| {
            let folder = package_folder(&directory, file)?;
            let path = game_path(&directory, &folder, Some("tgcx.dll"))?;
            // A full path: never picks up a tgcx.dll from the search path.
            let module =
                unsafe { LoadLibraryExW(path.as_ptr(), null_mut(), LOAD_WITH_ALTERED_SEARCH_PATH) };
            if module.is_null() {
                error = unsafe { GetLastError() };
                return None;
            }
            Some(module)
        });
    let Some(logic) = logic else {
        log(
            &directory,
            "no ticked TGCX package with tgcx.dll in the -mod= list; tgcx.dll not loaded",
            error,
        );
        return;
    };

    let Some(init) = (unsafe { GetProcAddress(logic, c"tgcx_Init".as_ptr().cast()) }) else {
        let error = unsafe { GetLastError() };
        unsafe { FreeLibrary(logic) };
        log(&directory, "tgcx.dll has no tgcx_Init", error);
        return;
    };
    // SAFETY: tgcx.dll exports `int __cdecl tgcx_Init(const char*)`. It logs its own
    // result and stays loaded either way.
    unsafe {
        let init: unsafe extern "C" fn(*const c_char) -> i32 = std::mem::transmute(init);
        init(MARKER.as_ptr());
    }
}

/// The folder of v2game.exe.
fn exe_directory() -> Option<Vec<u16>> {
    let mut exe = [0u16; MAX_PATH];
    let len = unsafe { GetModuleFileNameW(null_mut(), exe.as_mut_ptr(), MAX_PATH as u32) } as usize;
    if len == 0 || len >= MAX_PATH {
        return None;
    }
    let slash = exe[..len].iter().rposition(|&c| c == BACKSLASH)?;
    Some(exe[..slash].to_vec())
}

/// The `path` of a .mod whose folder holds a tgcx_package.txt with a `package` line.
fn package_folder(game: &[u16], file: &[u8]) -> Option<Vec<u8>> {
    let text = read_small(&game_path(game, file, None)?)?;
    let folder = mod_value(&text, "path", MAX_PATH).filter(|p| !p.is_empty())?;
    let package = read_small(&game_path(game, folder, Some("tgcx_package.txt"))?)?;
    mod_value(&package, "package", PACKAGE_SIZE).filter(|p| !p.is_empty())?;
    Some(folder.to_vec())
}

/// "<game>\<relative>[\<leaf>]", NUL-terminated, slashes turned round. `relative`
/// is in the ANSI code page, as on the command line.
fn game_path(game: &[u16], relative: &[u8], leaf: Option<&str>) -> Option<Vec<u16>> {
    let mut path = game.to_vec();
    path.push(BACKSLASH);
    for c in ansi_to_wide(relative)? {
        path.push(if c == u16::from(b'/') { BACKSLASH } else { c });
    }
    if let Some(leaf) = leaf {
        path.push(BACKSLASH);
        path.extend(leaf.encode_utf16());
    }
    path.push(0);
    (path.len() <= MAX_PATH).then_some(path)
}

fn ansi_to_wide(s: &[u8]) -> Option<Vec<u16>> {
    if s.is_empty() {
        return Some(Vec::new());
    }
    let mut wide = vec![0u16; MAX_PATH];
    let (len, cap) = (s.len() as i32, MAX_PATH as i32);
    let n = unsafe { MultiByteToWideChar(CP_ACP, 0, s.as_ptr(), len, wide.as_mut_ptr(), cap) };
    if n <= 0 {
        return None;
    }
    wide.truncate(n as usize);
    Some(wide)
}

fn open(path: &[u16], access: u32, disposition: u32) -> Option<HANDLE> {
    let file = unsafe {
        CreateFileW(
            path.as_ptr(),
            access,
            FILE_SHARE_READ,
            null(),
            disposition,
            0,
            null_mut(),
        )
    };
    (file != INVALID_HANDLE_VALUE).then_some(file)
}

/// Up to 16 KiB of a small text file, like tgcx_ReadSmall's buffer.
fn read_small(path: &[u16]) -> Option<Vec<u8>> {
    let file = open(path, GENERIC_READ, OPEN_EXISTING)?;
    let mut buf = vec![0u8; 16383];
    let mut n = 0;
    let ok = unsafe { ReadFile(file, buf.as_mut_ptr(), buf.len() as u32, &mut n, null_mut()) };
    unsafe { CloseHandle(file) };
    if ok == 0 {
        return None;
    }
    buf.truncate(n as usize);
    Some(buf)
}

/// Only written when tgcx.dll never opened the log itself.
fn log(directory: &[u16], text: &str, error: u32) {
    unsafe { CreateDirectoryW(join(directory, r"\Logs").as_ptr(), null()) };
    let path = join(directory, r"\Logs\tgcx.log");
    let Some(file) = open(&path, GENERIC_WRITE, CREATE_ALWAYS) else {
        return;
    };
    let marker = MARKER.to_str().unwrap_or_default();
    let line = format!("{marker}: {text} (error {error}); Lua forwarding only\r\n");
    let mut written = 0;
    unsafe {
        WriteFile(
            file,
            line.as_ptr(),
            line.len() as u32,
            &mut written,
            null_mut(),
        );
        CloseHandle(file);
    }
}

/// `base` followed by `tail` and a NUL.
fn join(base: &[u16], tail: &str) -> Vec<u16> {
    let mut path = base.to_vec();
    path.extend(tail.encode_utf16().chain([0]));
    path
}
