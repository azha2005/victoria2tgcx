//! Explicit load fence at the native in-game slot3. Date jumps are observations,
//! never campaign identities. No file, unit or country variable is written here.
use crate::native::Native;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
static BASE: AtomicU32 = AtomicU32::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static EPOCH: AtomicU64 = AtomicU64::new(0);
static LOADED: AtomicU32 = AtomicU32::new(0);
static DEPTH: AtomicU32 = AtomicU32::new(0);
pub fn ready() -> bool {
    READY.load(Ordering::Acquire)
}

pub fn epoch(context: u32) -> Option<u64> {
    if !READY.load(Ordering::Acquire) || LOADED.load(Ordering::Acquire) != context {
        return None;
    }
    let epoch = EPOCH.load(Ordering::Acquire);
    (epoch != 0 && LOADED.load(Ordering::Acquire) == context).then_some(epoch)
}
/// Cheap load barrier before inspecting a callback's unit arguments. Positive
/// admission must still recheck the actual current native context/session.
pub fn current_epoch() -> Option<u64> {
    let context = LOADED.load(Ordering::Acquire);
    if context == 0 {
        return None;
    }
    epoch(context)
}
extern "C" fn before(_: u32) {
    let _error = crate::native::LastError::preserve();
    LOADED.store(0, Ordering::Release);
    if DEPTH.fetch_add(1, Ordering::AcqRel) != 0
        || EPOCH
            .try_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_add(1))
            .is_err()
    {
        READY.store(false, Ordering::Release);
        crate::runtime::diagnostic(
            "Nested/exhausted native load fence; campaign admission disabled",
        );
    }
}
extern "C" fn after(context: u32) {
    let _error = crate::native::LastError::preserve();
    if DEPTH.fetch_sub(1, Ordering::AcqRel) != 1 || !READY.load(Ordering::Acquire) {
        return;
    }
    let n = Native {
        base: BASE.load(Ordering::Acquire),
    };
    let valid = n.u32(n.base + 0xE588E8).and_then(|w| n.field(w, 0xB24)) == Some(context)
        && n.u32(context) == Some(n.base + 0xA0A34C);
    if valid {
        LOADED.store(context, Ordering::Release);
    }
    crate::runtime::diagnostic(&format!(
        "native_load_completed context={context:08X} epoch={} valid={valid}",
        EPOCH.load(Ordering::Acquire)
    ));
}

// Same accepted native UI wrapper contract: ECX=this, no stack parameters, RET.
// The original result, flags, x87/SSE state and last-error value survive both
// callbacks. Raw FUN_00662010 decompilation timed out; ABI is from instructions.
#[unsafe(naked)]
unsafe extern "C" fn loaded_hook() {
    std::arch::naked_asm!(
        "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
        "push dword ptr [ebx+24]","call {before}","add esp,4","fxrstor [esp]","mov esp,ebx","popad","popfd",
        "push ecx","call dword ptr [{original}]","pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
        "push dword ptr [ebx+36]","call {after}","add esp,4","fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret",
        before=sym before,after=sym after,original=sym ORIGINAL);
}
/// Caller already verified the native signature set, SSE/FXSAVE and DLL pinning.
pub fn install(base: u32) -> bool {
    let n = Native { base };
    let slot = base + 0xA0A358;
    let original = base + 0x262010;
    if n.u32(slot) != Some(original) {
        return false;
    }
    BASE.store(base, Ordering::Release);
    ORIGINAL.store(original as usize, Ordering::Release);
    let outcome = crate::patch::replace(slot, original, loaded_hook as *const () as u32);
    READY.store(outcome.complete(), Ordering::Release);
    crate::runtime::diagnostic(&format!(
        "native_load_fence installed={} protection_restored={} rolled_back={}",
        outcome.installed, outcome.protection_restored, outcome.rolled_back
    ));
    outcome.complete()
}
