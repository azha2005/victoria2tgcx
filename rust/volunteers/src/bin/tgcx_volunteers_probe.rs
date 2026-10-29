//! Unsupported-host loader handshake. Never loads the game or installs a hook.
use std::ffi::{c_char, c_void};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
#[link(name = "kernel32")]
unsafe extern "system" {
    fn LoadLibraryW(path: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
    fn FreeLibrary(module: *mut c_void) -> i32;
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
}
// from V2DLL: V2ModAPI.h, API v1 field order only.
#[repr(C)]
struct Api {
    size: u32,
    version: u32,
    base: u32,
    folder: *const c_char,
    log: usize,
    patch_slot: usize,
    trampoline: usize,
}
fn main() {
    let path = PathBuf::from(std::env::args_os().nth(1).expect("DLL path"));
    let expected = if std::env::args().any(|arg| arg == "--active") {
        0
    } else {
        1
    };
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
    unsafe {
        let module = LoadLibraryW(wide.as_ptr());
        assert!(!module.is_null(), "LoadLibrary failed");
        let entry = GetProcAddress(module, c"V2Mod_Init".as_ptr());
        assert!(!entry.is_null());
        let init: unsafe extern "C" fn(*const Api) -> i32 = std::mem::transmute(entry);
        assert_eq!(init(std::ptr::null()), 0);
        let mut api = Api {
            size: size_of::<Api>() as u32,
            version: 2,
            base: GetModuleHandleW(std::ptr::null()) as u32,
            folder: c"mod/tgcx_volunteers".as_ptr(),
            log: 0,
            patch_slot: 0,
            trampoline: 0,
        };
        assert_eq!(init(&api), 0);
        api.version = 1;
        assert_eq!(init(&api), expected);
        assert_eq!(init(&api), expected);
        assert_ne!(FreeLibrary(module), 0);
    }
    println!(
        "V2DLL API shape/load-once probe passed; active={}; this is an unsupported host, not a game check",
        expected == 0
    );
}
