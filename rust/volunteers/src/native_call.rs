//! Narrow x86 SEH boundary, adapted from our rust/tgcx/src/sys.rs guard.
//! Only Copy arguments/results are allowed. Keep Rust allocations and guards
//! outside the callback: a foreign exception does not run Rust destructors.
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

static FAULTED: AtomicBool = AtomicBool::new(false);
pub fn faulted() -> bool {
    FAULTED.load(Ordering::Acquire)
}

/// Invoke one native operation. An access violation disables subsequent calls.
/// This does not repair engine state, catch heap fail-fast or validate an ABI.
pub fn call<R: Copy, F: FnOnce() -> R + Copy>(f: F) -> Option<R> {
    if faulted() {
        return None;
    }
    let result = attempt(f);
    if result.is_none() {
        FAULTED.store(true, Ordering::Release);
    }
    result
}
fn attempt<R: Copy, F: FnOnce() -> R + Copy>(f: F) -> Option<R> {
    struct Call<F, R> {
        function: F,
        result: Option<R>,
    }
    extern "C" fn run<F: FnOnce() -> R + Copy, R: Copy>(context: *mut c_void) {
        let call = unsafe { &mut *context.cast::<Call<F, R>>() };
        call.result = Some((call.function)());
    }
    let mut context = Call {
        function: f,
        result: None,
    };
    let completed = unsafe { seh_call(run::<F, R>, (&raw mut context).cast()) };
    if completed == 0 { None } else { context.result }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn VirtualAlloc(p: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
        fn VirtualFree(p: *mut c_void, size: usize, kind: u32) -> i32;
    }
    #[test]
    fn scalar_probe_reads_values_and_contains_access_violations() {
        let value = 0xCAFE1234u32;
        let ptr = &value as *const u32;
        assert_eq!(
            attempt(|| unsafe { std::ptr::read_unaligned(ptr) }),
            Some(value)
        );
        let page = unsafe { VirtualAlloc(std::ptr::null_mut(), 4096, 0x3000, 1) };
        assert!(!page.is_null());
        assert_eq!(
            attempt(|| unsafe { std::ptr::read_unaligned(page.cast::<u32>()) }),
            None
        );
        assert_eq!(
            attempt(|| unsafe { std::ptr::read_unaligned(ptr) }),
            Some(value)
        );
        assert_ne!(unsafe { VirtualFree(page, 0, 0x8000) }, 0);
    }
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn RtlUnwind(
        frame: *const c_void,
        ip: *const c_void,
        record: *const c_void,
        value: *const c_void,
    );
}
static UNWIND: unsafe extern "system" fn(
    *const c_void,
    *const c_void,
    *const c_void,
    *const c_void,
) = RtlUnwind;
static HANDLER: unsafe extern "C" fn() = seh_handler;

#[unsafe(naked)]
unsafe extern "C" fn seh_call(f: extern "C" fn(*mut c_void), context: *mut c_void) -> u32 {
    std::arch::naked_asm!(
        "push ebp", "mov ebp, esp", "push ebx", "push esi", "push edi",
        "push ebp", "push dword ptr [{handler}]", "push dword ptr fs:[0]",
        "mov dword ptr fs:[0], esp", "push dword ptr [ebp+12]", "call dword ptr [ebp+8]",
        "add esp, 4", "mov eax, 1", "pop ecx", "mov dword ptr fs:[0], ecx",
        "add esp, 8", "pop edi", "pop esi", "pop ebx", "pop ebp", "ret",
        handler = sym HANDLER,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn seh_handler() {
    std::arch::naked_asm!(
        "mov eax, [esp+4]", "test dword ptr [eax+4], 6", "jnz 2f",
        // Continue searching for stack overflow, C++ exceptions and other
        // failures. Never claim that continuing after heap corruption is safe.
        "cmp dword ptr [eax], 0xC0000005", "jne 2f",
        "mov ecx, [esp+8]", "push ecx", "push 0", "push eax", "push 0", "push ecx",
        "call dword ptr [{unwind}]", "pop ecx", "mov esp, ecx", "mov ebp, [ecx+8]",
        "xor eax, eax", "pop ecx", "mov dword ptr fs:[0], ecx", "add esp, 8",
        "pop edi", "pop esi", "pop ebx", "pop ebp", "ret",
        "2:", "mov eax, 1", "ret", unwind = sym UNWIND,
    );
}
std::arch::global_asm!(".safeseh {}", sym seh_handler);
