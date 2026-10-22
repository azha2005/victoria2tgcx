//! Port of dll/tgcx/tgcx_packages.h (MOD-2, D15): which of our packages the launcher
//! ticked. The launcher passes one -mod=<file>.mod per ticked mod; each package folder
//! holds tgcx_package.txt and its own copy of tgcx.dll. The parsing is the loader's
//! (rust/loader/src/mods.rs).

use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Globalization::{CP_ACP, MultiByteToWideChar};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_SHARE_READ, OPEN_EXISTING, ReadFile,
};

pub const MAX_PATH: usize = 260;
const MODS_MAX: usize = 24;
const BACKSLASH: u16 = b'\\' as u16;

/// One ticked mod. Each field is empty when absent or too long for the C++'s buffer.
#[derive(Clone, Default)]
pub struct Mod {
    /// As on the command line, e.g. mod/TGCX.mod.
    pub file: Vec<u8>,
    /// The .mod's path line, e.g. mod/tgcx_army_tgc252.
    pub path: Vec<u8>,
    pub user_dir: Vec<u8>,
    /// "army", "politics" or empty when not ours.
    pub package: Vec<u8>,
}

/// Every -mod= value, in command-line order. A value runs to its ".mod", so a file
/// name may hold spaces whether or not the launcher quotes it.
fn parse_mod_args(line: &[u8]) -> Vec<&[u8]> {
    let line = until_nul(line);
    let mut mods = Vec::new();
    let mut at = 0;
    while mods.len() < MODS_MAX {
        let Some(found) = line[at..].windows(5).position(|w| w == b"-mod=") else {
            break;
        };
        let start = at + found + 5;
        let Some(end) = (start..line.len()).find(|&i| ends_mod(line, i)) else {
            break;
        };
        at = end + 4;
        let value = &line[start..at];
        let value = value.strip_prefix(b"\"").unwrap_or(value);
        if !value.is_empty() && value.len() < MAX_PATH {
            mods.push(value);
        }
    }
    mods
}

/// ".mod" (any case) at `i`, followed by the end of the line, a space or a quote.
fn ends_mod(line: &[u8], i: usize) -> bool {
    line.get(i..i + 4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b".mod"))
        && matches!(line.get(i + 4), None | Some(b' ' | b'"'))
}

/// The value of the first `key = "value"` (or unquoted) line. The key must start the
/// line, so "path" never matches "replace_path". None when the value and its NUL do
/// not fit in `size` bytes.
fn mod_value<'a>(text: &'a [u8], key: &str, size: usize) -> Option<&'a [u8]> {
    for line in until_nul(text).split(|&c| c == b'\n') {
        let Some((name, rest)) = skip_blanks(line).split_at_checked(key.len()) else {
            continue;
        };
        if !name.eq_ignore_ascii_case(key.as_bytes())
            || !matches!(rest.first(), Some(b' ' | b'\t' | b'='))
        {
            continue;
        }
        let Some(value) = skip_blanks(rest).strip_prefix(b"=") else {
            continue;
        };
        let value = skip_blanks(value);
        let (value, quoted) = match value.strip_prefix(b"\"") {
            Some(value) => (value, true),
            None => (value, false),
        };
        let ends = |c: u8| c == b'\r' || if quoted { c == b'"' } else { is_blank(c) };
        let len = value.iter().position(|&c| ends(c)).unwrap_or(value.len());
        return (len < size).then(|| &value[..len]);
    }
    None
}

fn is_blank(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn skip_blanks(s: &[u8]) -> &[u8] {
    &s[s.iter().take_while(|&&c| is_blank(c)).count()..]
}

fn until_nul(s: &[u8]) -> &[u8] {
    s.split(|&c| c == 0).next().unwrap_or_default()
}

/// "<game>\<relative>[\<leaf>]", NUL-terminated, slashes turned round. `relative` is
/// in the ANSI code page, as on the command line.
pub fn game_path(game: &[u16], relative: &[u8], leaf: Option<&str>) -> Option<Vec<u16>> {
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

pub fn ansi_to_wide(s: &[u8]) -> Option<Vec<u16>> {
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

/// Up to size-1 bytes of a small file, like tgcx_ReadSmall.
pub fn read_small(path: &[u16], size: usize) -> Option<Vec<u8>> {
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
    let mut buf = vec![0u8; size - 1];
    let mut n = 0;
    let ok = unsafe { ReadFile(file, buf.as_mut_ptr(), buf.len() as u32, &mut n, null_mut()) };
    unsafe { CloseHandle(file) };
    if ok == 0 {
        return None;
    }
    buf.truncate(n as usize);
    Some(buf)
}

/// Reads every ticked .mod and, in its folder, our tgcx_package.txt.
pub fn read_mods(game: &[u16], command_line: &[u8]) -> Vec<Mod> {
    let mut list = Vec::new();
    for file in parse_mod_args(command_line) {
        let mut m = Mod {
            file: file.to_vec(),
            ..Mod::default()
        };
        if let Some(text) = game_path(game, file, None).and_then(|p| read_small(&p, 16384)) {
            m.path = mod_value(&text, "path", MAX_PATH)
                .unwrap_or_default()
                .to_vec();
            m.user_dir = mod_value(&text, "user_dir", 64)
                .unwrap_or_default()
                .to_vec();
            if !m.path.is_empty()
                && let Some(package) = game_path(game, &m.path, Some("tgcx_package.txt"))
                    .and_then(|p| read_small(&p, 16384))
            {
                m.package = mod_value(&package, "package", 16)
                    .unwrap_or_default()
                    .to_vec();
            }
        }
        list.push(m);
    }
    list
}

pub fn ticked(list: &[Mod], package: &str) -> bool {
    list.iter().any(|m| m.package == package.as_bytes())
}

/// The game's settings folder joins the ticked mods' user_dirs with "_" in
/// command-line order (TGC252 + TGCX -> TGC252_TGCX; none -> the root).
pub fn settings_folder(list: &[Mod]) -> Vec<u8> {
    let mut out = Vec::new();
    for m in list.iter().filter(|m| !m.user_dir.is_empty()) {
        if !out.is_empty() {
            out.push(b'_');
        }
        out.extend_from_slice(&m.user_dir);
    }
    out
}
