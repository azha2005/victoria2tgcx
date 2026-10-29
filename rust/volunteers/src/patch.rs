//! One guarded pointer patch. A protection failure after CAS must not be
//! reported as an untouched slot. Callers pin the DLL before entering here.
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> *mut c_void;
    fn ReadProcessMemory(
        process: *mut c_void,
        address: *const c_void,
        out: *mut c_void,
        size: usize,
        copied: *mut usize,
    ) -> i32;
    fn VirtualProtect(address: *const c_void, size: usize, protection: u32, old: *mut u32) -> i32;
}
#[derive(Clone, Copy, Default)]
pub struct Outcome {
    pub installed: bool,
    pub protection_restored: bool,
    pub rolled_back: bool,
}
impl Outcome {
    pub fn complete(self) -> bool {
        self.installed && self.protection_restored
    }
}
fn read(address: u32) -> Option<u32> {
    let mut value = 0u32;
    let mut count = 0;
    let ok = unsafe {
        ReadProcessMemory(
            GetCurrentProcess(),
            address as *const c_void,
            (&raw mut value).cast(),
            4,
            &mut count,
        )
    };
    (ok != 0 && count == 4).then_some(value)
}
/// The caller has validated a static, aligned vtable slot in the pinned EXE.
pub fn replace(address: u32, expected: u32, replacement: u32) -> Outcome {
    if address < 0x10000 || !address.is_multiple_of(4) || read(address) != Some(expected) {
        return Outcome::default();
    }
    let slot = address as *const AtomicU32;
    let mut old = 0;
    if unsafe { VirtualProtect(slot.cast(), 4, 4, &mut old) } == 0 {
        return Outcome::default();
    }
    let changed = unsafe { &*slot }
        .compare_exchange(expected, replacement, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok();
    let mut ignored = 0;
    if unsafe { VirtualProtect(slot.cast(), 4, old, &mut ignored) } != 0 {
        return Outcome {
            installed: changed,
            protection_restored: true,
            rolled_back: false,
        };
    }
    // Re-establish writable protection before rollback; retain the first old
    // protection rather than accidentally restoring this retry's PAGE_READWRITE.
    let writable = unsafe { VirtualProtect(slot.cast(), 4, 4, &mut ignored) } != 0;
    let rolled_back = changed
        && writable
        && unsafe { &*slot }
            .compare_exchange(replacement, expected, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok();
    let restored = unsafe { VirtualProtect(slot.cast(), 4, old, &mut ignored) } != 0;
    Outcome {
        installed: read(address) == Some(replacement),
        protection_restored: restored,
        rolled_back,
    }
}
