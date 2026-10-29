//! Read-only observer after the native CSetVariableCommand execute slot. A
//! committed journal is identified at engine execution, never by a UI click or
//! local sidecar. Multiplayer metadata is diagnosed, not admitted to gameplay.
use crate::native::Native;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
static BASE: AtomicU32 = AtomicU32::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
pub fn ready() -> bool {
    READY.load(Ordering::Acquire)
}
extern "C" fn committed(command: u32) {
    let _error = crate::native::LastError::preserve();
    if !READY.load(Ordering::Acquire) || crate::native_call::faulted() {
        return;
    }
    let n = Native {
        base: BASE.load(Ordering::Acquire),
    };
    if n.u32(command) != Some(n.base + 0xA0121C) {
        return;
    }
    let Some(key) = n.string(command + 0x44, 96) else {
        return;
    };
    if key.starts_with(crate::mp_sync::PROBE_PREFIX) {
        crate::native_mp::probe_executed(n, command, &key);
        return;
    }
    if key != crate::engine_store::HEAD {
        return;
    }
    let Some(country) = n.field(command, 0x40) else {
        return;
    };
    let Some(session) = crate::native_session::capture(&n, n.base) else {
        return;
    };
    let Some(epoch) = crate::native_lifecycle::epoch(session.context) else {
        return;
    };
    match crate::engine_store::load(epoch, |key| n.variable(country, key)) {
        Ok(Some(loaded)) => {
            crate::native_mp::journal(n, country, &loaded);
            crate::runtime::diagnostic(&format!(
                "journal_commit country={country} head={} sha256={} date={:?} session={:?} (native execute observed; no gameplay admission)",
                loaded.head,
                loaded.journal_sha256,
                n.field(session.world, 0xB0C),
                session.mode
            ));
            // The GUI restores a fresh suspended view on its own thread. Do not
            // mutate mission authority or enqueue derived work from this hook.
            crate::native_gui::checkpoint_changed(country, loaded.head, &loaded.journal_sha256);
        }
        Ok(None) => crate::runtime::diagnostic(
            "Native journal head commit observed without a readable journal",
        ),
        Err(error) => {
            crate::runtime::diagnostic(&format!("Native journal commit refused: {error:?}"))
        }
    }
}
#[unsafe(naked)]
unsafe extern "C" fn execute_hook() {
    std::arch::naked_asm!(
    "push ecx","call dword ptr [{original}]","pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+36]","call {callback}","add esp,4","fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret",
    original=sym ORIGINAL,callback=sym committed);
}
/// Initial native guards and DLL pinning must have passed before installation.
pub fn install(base: u32) -> bool {
    let n = Native { base };
    let slot = base + 0xA0121C + 6 * 4;
    let original = base + 0x1839C0;
    if n.u32(slot) != Some(original) {
        return false;
    }
    BASE.store(base, Ordering::Release);
    ORIGINAL.store(original as usize, Ordering::Release);
    let result = crate::patch::replace(slot, original, execute_hook as *const () as u32);
    READY.store(result.complete(), Ordering::Release);
    crate::runtime::diagnostic(&format!(
        "native_journal_observer installed={} protection_restored={} rolled_back={}",
        result.installed, result.protection_restored, result.rolled_back
    ));
    result.complete()
}
