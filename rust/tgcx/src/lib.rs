//! Rust port of dll/tgcx (tgcx.dll, TPL-7 0.11.2): the same hooks, gates and log as
//! the C++ build, module by module. TPL-8 0.11.3 (Non-core only) TPL-11 (Build Army gathering) and TPL-12 0.11.15 (Replace depleted) exist only here. Nothing is installed from DllMain: a loader calls
//! tgcx_Init (our lua51.dll) or V2Mod_Init (V2DLL's plugin API).

// The loops index several parallel arrays as the C++ does.
#![allow(clippy::needless_range_loop)]

mod bill;
mod conform;
mod conform_gui;
mod conform_native;
mod d3d;
mod gather;
mod packages;
mod reforms;
mod renderer;
mod replace;
mod sha256;
mod snapshot;
mod sys;
mod templates;
mod templates_gui;

#[allow(dead_code, clippy::all)]
mod signatures {
    include!(concat!(env!("OUT_DIR"), "/tgcx_signatures.rs"));
}
#[allow(dead_code, clippy::all)]
mod bill_catalog {
    include!(concat!(env!("OUT_DIR"), "/tgcx_bill_catalog.rs"));
}
#[allow(dead_code, clippy::all)]
mod templates_catalog {
    include!(concat!(env!("OUT_DIR"), "/tgcx_templates_catalog.rs"));
}
#[allow(dead_code, clippy::all)]
mod conform_layout {
    include!(concat!(env!("OUT_DIR"), "/tgcx_conform_layout.rs"));
}
mod thunks {
    include!(concat!(env!("OUT_DIR"), "/thunks.rs"));
}

use std::ffi::{CStr, c_char, c_void};
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, Ordering};

use windows_sys::Win32::Foundation::{
    CloseHandle, GENERIC_READ, GENERIC_WRITE, HINSTANCE, HMODULE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_SHARE_READ, OPEN_EXISTING, ReadFile,
};
use windows_sys::Win32::System::Environment::GetCommandLineA;
use windows_sys::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN, GetModuleFileNameW,
    GetModuleHandleExW, GetModuleHandleW,
};
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_DETACH;
use windows_sys::Win32::System::Threading::{
    IsProcessorFeaturePresent, PF_XMMI_INSTRUCTIONS_AVAILABLE,
};

use packages::{MAX_PATH, Mod};
use signatures::{EXE_SHA256, PREFERRED_BASE, SIGNATURES};
use sys::{Bytes, base, check_signature, require_signatures};

/// Sites V2DLL patches (report-v2dll-mod-slot-1018.md §3.2). TGCX writes none of them,
/// so a mismatch there is logged and never blocks a module (MOD-1).
const FOREIGN_RVAS: [u32; 7] = [
    0x285727, 0x35BB50, 0x1D74BE, 0x282EC0, 0x233826, 0x1D77A0, 0xA0E480,
];
/// Native ordered-draw and final-Present stages: every module ticks from them.
const STAGE_RVAS: [u32; 2] = [0x59A520, 0x599F22];

fn hash_file(path: &[u16]) -> Option<String> {
    let file = unsafe {
        CreateFileW(
            path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            null(),
            OPEN_EXISTING,
            0,
            null_mut(),
        )
    };
    if file == INVALID_HANDLE_VALUE {
        return None;
    }
    let mut hash = sha256::Sha256::new();
    let mut bytes = vec![0u8; 8192];
    let mut ok = true;
    loop {
        let mut count = 0;
        if unsafe { ReadFile(file, bytes.as_mut_ptr(), 8192, &mut count, null_mut()) } == 0 {
            ok = false;
            break;
        }
        if count == 0 {
            break;
        }
        hash.update(&bytes[..count as usize]);
    }
    unsafe { CloseHandle(file) };
    ok.then(|| hash.finish())
}

/// Logs every signature; true when all but the foreign ones match.
fn check_signatures() -> bool {
    let mut ok = true;
    for signature in SIGNATURES.iter() {
        let matches = check_signature(signature);
        let foreign = !matches && FOREIGN_RVAS.contains(&signature.rva);
        let state = if matches {
            "OK"
        } else if foreign {
            "FOREIGN"
        } else {
            "BAD"
        };
        log!("{state} RVA 0x{:06X} {}", signature.rva, signature.name);
        ok = (matches || foreign) && ok;
    }
    ok
}

/// MOD-1: drawing and laws (hemicycle, hover, Bill, Reforms) need every signature
/// that neither V2DLL, the stages nor Templates/Conform own; those gate themselves.
fn drawing_signatures() -> bool {
    SIGNATURES.iter().all(|s| {
        let rva = s.rva;
        FOREIGN_RVAS.contains(&rva)
            || STAGE_RVAS.contains(&rva)
            || templates::TPL_RVAS.contains(&rva)
            || templates_gui::TPL_GUI_RVAS.contains(&rva)
            || conform_native::CF_RVAS.contains(&rva)
            || check_signature(s)
    })
}

/// from V2DLL: V2ModAPI.h (upstream 9821c27), plugin API v1 layout; personal use, D6/R8.
#[repr(C)]
pub struct V2ModApi {
    size: u32,
    api_version: u32,
    game_base: u32,
    mod_folder: *const c_char,
    log: Option<unsafe extern "C" fn(*const c_char, ...)>,
    patch_slot: usize,
    steal_to_trampoline: usize,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Runs once, whichever loader calls first (our lua51.dll or V2DLL's plugin API). D16.
fn initialize(origin: &[u8]) -> bool {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        log!(
            "Initialisation from {} ignored: already initialised",
            Bytes(origin)
        );
        return false;
    }
    let mut exe = [0u16; MAX_PATH];
    let count =
        unsafe { GetModuleFileNameW(null_mut(), exe.as_mut_ptr(), MAX_PATH as u32) } as usize;
    if count == 0 || count >= MAX_PATH {
        return false;
    }
    let Some(slash) = exe[..count].iter().rposition(|&c| c == u16::from(b'\\')) else {
        return false;
    };
    let directory = exe[..slash].to_vec();
    let join = |tail: &str| -> Vec<u16> {
        let mut path = directory.clone();
        path.extend(tail.encode_utf16().chain([0]));
        path
    };
    unsafe { CreateDirectoryW(join(r"\Logs").as_ptr(), null()) };
    let log_path = join(r"\Logs\tgcx.log");
    if log_path.len() > MAX_PATH {
        return false;
    }
    let log = unsafe {
        CreateFileW(
            log_path.as_ptr(),
            GENERIC_WRITE,
            FILE_SHARE_READ,
            null(),
            CREATE_ALWAYS,
            0,
            null_mut(),
        )
    };
    if log == INVALID_HANDLE_VALUE {
        return false;
    }
    sys::set_log(log);
    sys::set_base(unsafe { GetModuleHandleW(null()) } as usize);
    log!(
        "TGCX TPL-12 0.11.15 tgcx.dll loaded_by={} base={:08X} preferred={PREFERRED_BASE:08X} simulation_writes=engine_commands_only",
        Bytes(origin),
        base()
    );
    let digest = hash_file(&exe[..=count]);
    let hash_ok = digest.as_deref() == Some(EXE_SHA256);
    log!(
        "{} sha256 {}",
        if hash_ok { "OK" } else { "BAD" },
        digest.as_deref().unwrap_or("unavailable")
    );
    if !hash_ok {
        log!(
            "Unsupported executable: no game addresses read, no hooks installed; Lua forwarding remains active"
        );
        return false;
    }
    if !check_signatures() {
        log!("Signature mismatch: each module checks its own signatures below");
    }
    if unsafe { IsProcessorFeaturePresent(PF_XMMI_INSTRUCTIONS_AVAILABLE) } == 0 {
        log!("FXSAVE unavailable: no hooks installed");
        return false;
    }
    // Prevent a late FreeLibrary from leaving patched code pointing into an unloaded DLL.
    let mut pinned: HMODULE = null_mut();
    let flags = GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN;
    if unsafe { GetModuleHandleExW(flags, initialize as *const () as *const u16, &mut pinned) } == 0
    {
        log!("Cannot pin tgcx.dll: no hooks installed");
        return false;
    }
    // MOD-2: modules follow the packages ticked in the launcher (D15).
    // SAFETY: the process command line is a NUL-terminated string that lives forever.
    let command_line = unsafe { CStr::from_ptr(GetCommandLineA().cast()) };
    let mods: Vec<Mod> = packages::read_mods(&directory, command_line.to_bytes());
    let or = |value: &[u8], absent: &'static str| -> String {
        if value.is_empty() {
            absent.to_string()
        } else {
            Bytes(value).to_string()
        }
    };
    for m in &mods {
        log!(
            "Mod {}: path={} user_dir={} package={}",
            Bytes(&m.file),
            or(&m.path, "?"),
            or(&m.user_dir, "-"),
            or(&m.package, "-")
        );
    }
    let army = packages::ticked(&mods, "army");
    let politics = packages::ticked(&mods, "politics");
    let settings = packages::settings_folder(&mods);
    templates::set_settings_folder(&settings);
    log!(
        "Packages: army={} politics={} settings={}",
        u32::from(army),
        u32::from(politics),
        if settings.is_empty() {
            "(root)".to_string()
        } else {
            Bytes(&settings).to_string()
        }
    );
    if !army && !politics {
        log!("No TGCX package in the -mod= list: no hooks installed");
        return false;
    }
    if !renderer::install_native_stages() {
        log!("Module gate stages: BAD (ordered draw/final Present); no module installed");
        return false;
    }
    if politics {
        let drawing = drawing_signatures();
        log!(
            "Module gate drawing+laws: signatures {}",
            if drawing { "OK" } else { "BAD" }
        );
        if drawing && renderer::install_renderer() {
            bill::enable(base() + 0x2DCE10);
            reforms::install();
        } else {
            log!("Drawing and laws disabled: original pie, no Bill or Reforms tab");
        }
    } else {
        log!("Politics not ticked: drawing+laws off, original pie");
    }
    if army && templates::package_metadata(&directory, &mods) {
        const UNIVERSAL_RVAS: [u32; 14] = [
            0x5A38F0, 0x5C4410, 0x5E3D60, 0x5EFE20, 0x246620, 0x5A6A6D, 0xA08A70, 0x8453FC,
            0x1BAB1C, 0x1BA89E, 0x5E4490, 0xA3C1DC, 0x5EFEF0, 0x5EFEB0,
        ];
        if !templates::army_universal() || require_signatures(&UNIVERSAL_RVAS) {
            const FILTER_RVAS: [u32; 7] = [
                0x122310, 0x4F84D0, 0x4CF4C5, 0x9D040, 0x94520, 0x94550, 0x9FC1DC,
            ];
            // Without the guards a restricted template or continent refuses
            // to recruit (never widens to All/Any); unrestricted ones still work.
            let filters = require_signatures(&FILTER_RVAS);
            templates::set_filters_enabled(filters);
            if !filters {
                log!("Army filters disabled: native culture/continent guards failed");
            }
            templates_gui::install();
            conform_native::install();
        } else {
            log!("Universal Army disabled: native GUI/metadata signature gate failed");
        }
    } else {
        log!("Army unavailable or not ticked: Templates and Conform off");
    }
    log!(
        "Modules: drawing+laws={} templates={} conform={}",
        u32::from(renderer::drawing_enabled()),
        u32::from(templates_gui::enabled()),
        u32::from(conform_native::enabled())
    );
    true
}

/// Our loader (lua51.dll) calls this right after LoadLibrary.
///
/// # Safety
/// `loader` is null or a NUL-terminated string.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn tgcx_Init(loader: *const c_char) -> i32 {
    // SAFETY: the loader passes a NUL-terminated marker or null.
    let name = if loader.is_null() {
        &[][..]
    } else {
        unsafe { CStr::from_ptr(loader) }.to_bytes()
    };
    let origin = if name.is_empty() {
        b"unknown loader".as_slice()
    } else {
        name
    };
    i32::from(initialize(origin))
}

/// V2DLL's plugin entry. TGCX uses only size, version and Log: its own hooks are guarded as above.
///
/// # Safety
/// `api` is null or V2DLL's plugin API table.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn V2Mod_Init(api: *const V2ModApi) -> i32 {
    // SAFETY: V2DLL passes its API table, checked by size and version first.
    let Some(api) = (unsafe { api.as_ref() }) else {
        return 0;
    };
    if (api.size as usize) < size_of::<V2ModApi>() || api.api_version != 1 {
        return 0;
    }
    let ok = initialize(b"V2DLL plugin API v1");
    if let Some(log) = api.log {
        let state = if ok {
            c"initialised"
        } else {
            c"not initialised"
        };
        // SAFETY: V2DLL's printf-style logger.
        unsafe { log(c"TGCX: %s; see Logs\\tgcx.log".as_ptr(), state.as_ptr()) };
    }
    i32::from(ok)
}

// A #[no_mangle] DllMain would also be exported; the CRT finds this symbol instead.
std::arch::global_asm!(
    ".globl \"_DllMain@12\"",
    "\"_DllMain@12\":",
    "jmp {dll_main}",
    dll_main = sym dll_main,
);

extern "system" fn dll_main(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> i32 {
    // The static CRT needs thread notifications; do not call DisableThreadLibraryCalls.
    // Nothing is installed here: a loader calls tgcx_Init or V2Mod_Init.
    if reason == DLL_PROCESS_DETACH {
        let log = sys::take_log();
        if log != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(log) };
        }
    }
    1
}
