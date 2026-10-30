//! Observer after the native CSetVariableCommand execute slot. A
//! committed journal is identified at engine execution, never by a UI click or
//! local sidecar. Preserve sender metadata on our private command clones.
use crate::native::Native;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
static BASE: AtomicU32 = AtomicU32::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_CLONE: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
pub fn ready() -> bool {
    READY.load(Ordering::Acquire)
}
extern "C" fn clone_metadata(source: u32, clone: u32) {
    let _error = crate::native::LastError::preserve();
    if !READY.load(Ordering::Acquire) || crate::native_call::faulted() {
        return;
    }
    let n = Native {
        base: BASE.load(Ordering::Acquire),
    };
    let table = n.base + 0xA0121C;
    if source == clone || n.u32(source) != Some(table) || n.u32(clone) != Some(table) {
        return;
    }
    let Some(key) = n.string(source + 0x44, 96) else {
        return;
    };
    if !key.starts_with("tgcx_vl_") || n.string(clone + 0x44, 96).as_ref() != Some(&key) {
        return;
    }
    let Some(metadata) = n.u32(source + 0x34) else {
        return;
    };
    // FUN_00583C00 reconstructs +34 as FFFF. This is a newly allocated
    // command, not world state. Leave native flags (+36) and sequence (+38).
    let sender = metadata as u16;
    let _ = crate::native_call::call(|| unsafe {
        std::ptr::write_unaligned((clone + 0x34) as *mut u16, sender);
    });
}
#[unsafe(naked)]
unsafe extern "C" fn clone_hook() {
    // Slot 13 is ECX=this, seven opaque stack words, EAX=clone, RET 1C.
    // Transfer the by-value argument blob once; the original owns its cleanup.
    std::arch::naked_asm!(
        "push ecx", "mov edx,esp",
        "push dword ptr [edx+32]", "push dword ptr [edx+28]",
        "push dword ptr [edx+24]", "push dword ptr [edx+20]",
        "push dword ptr [edx+16]", "push dword ptr [edx+12]", "push dword ptr [edx+8]",
        "call dword ptr [{original}]",
        "pushfd", "pushad", "mov ebx,esp", "sub esp,528", "and esp,-16",
        "fxsave [esp]", "fninit", "mov dword ptr [esp+512],0x1F80", "ldmxcsr [esp+512]", "cld",
        "push dword ptr [ebx+28]", "push dword ptr [ebx+36]", "call {callback}", "add esp,8",
        "fxrstor [esp]", "mov esp,ebx", "popad", "popfd", "lea esp,[esp+4]", "ret 0x1c",
        original=sym ORIGINAL_CLONE, callback=sym clone_metadata,
    );
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
    if crate::native_mp_play::variable(n, command, &key) {
        return;
    }
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
    let clone_slot = base + 0xA0121C + 13 * 4;
    let original_clone = base + 0x183C00;
    if n.u32(slot) != Some(original) || n.u32(clone_slot) != Some(original_clone) {
        return false;
    }
    BASE.store(base, Ordering::Release);
    ORIGINAL.store(original as usize, Ordering::Release);
    ORIGINAL_CLONE.store(original_clone as usize, Ordering::Release);
    let cloned = crate::patch::replace(clone_slot, original_clone, clone_hook as *const () as u32);
    if !cloned.complete() {
        crate::runtime::diagnostic(&format!(
            "native_journal_clone refused installed={} protection_restored={} rolled_back={}",
            cloned.installed, cloned.protection_restored, cloned.rolled_back
        ));
        return false;
    }
    let result = crate::patch::replace(slot, original, execute_hook as *const () as u32);
    if !result.complete() {
        let rollback =
            crate::patch::replace(clone_slot, clone_hook as *const () as u32, original_clone);
        crate::runtime::diagnostic(&format!(
            "native_journal_clone rollback complete={} protection_restored={}",
            rollback.complete(),
            rollback.protection_restored
        ));
    }
    READY.store(result.complete(), Ordering::Release);
    crate::runtime::diagnostic(&format!(
        "native_journal_observer installed={} protection_restored={} rolled_back={}",
        result.installed, result.protection_restored, result.rolled_back
    ));
    result.complete()
}

#[cfg(test)]
mod tests {
    use super::*;
    static ARGUMENTS: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];
    static CLONE: AtomicU32 = AtomicU32::new(0);
    #[unsafe(naked)]
    unsafe extern "C" fn original_fixture() {
        std::arch::naked_asm!(
            "mov dword ptr [{args}],ecx",
            "mov eax,[esp+4]", "mov dword ptr [{args}+4],eax",
            "mov eax,[esp+8]", "mov dword ptr [{args}+8],eax",
            "mov eax,[esp+12]", "mov dword ptr [{args}+12],eax",
            "mov eax,[esp+16]", "mov dword ptr [{args}+16],eax",
            "mov eax,[esp+20]", "mov dword ptr [{args}+20],eax",
            "mov eax,[esp+24]", "mov dword ptr [{args}+24],eax",
            "mov eax,[esp+28]", "mov dword ptr [{args}+28],eax",
            "mov eax,dword ptr [{clone}]", "stc", "ret 0x1c",
            args=sym ARGUMENTS, clone=sym CLONE,
        );
    }
    #[unsafe(naked)]
    unsafe extern "C" fn invoke_fixture(_source: u32, _out: *mut u32) {
        std::arch::naked_asm!(
            "push ebp", "mov ebp,esp", "push ebx", "push esi", "push edi", "mov ebx,esp",
            "mov ecx,[ebp+8]", "push 7", "push 6", "push 5", "push 4", "push 3", "push 2", "push 1",
            "call {hook}", "mov edx,[ebp+12]", "mov [edx],eax", "pushfd", "pop eax", "mov [edx+4],eax",
            "mov [edx+8],esp", "mov [edx+12],ebx", "mov esp,ebx", "pop edi", "pop esi", "pop ebx", "pop ebp", "ret",
            hook=sym clone_hook,
        );
    }
    #[test]
    fn clone_preserves_opaque_ret1c_abi_and_only_namespaced_sender() {
        let mut source = [0u32; 25];
        let mut clone = [0u32; 25];
        source[0] = 0x10000 + 0xA0121C;
        clone[0] = source[0];
        source[0x34 / 4] = 0xABCD0007;
        clone[0x34 / 4] = 0xCAFEFFFF;
        clone[0x38 / 4] = 0x12345678;
        let key = crate::native_string::GameString::new("tgcx_vl_head");
        unsafe {
            std::ptr::copy_nonoverlapping(
                (&raw const key).cast::<u8>(),
                source.as_mut_ptr().cast::<u8>().add(0x44),
                24,
            );
            std::ptr::copy_nonoverlapping(
                (&raw const key).cast::<u8>(),
                clone.as_mut_ptr().cast::<u8>().add(0x44),
                24,
            );
        }
        BASE.store(0x10000, Ordering::Release);
        ORIGINAL_CLONE.store(original_fixture as *const () as usize, Ordering::Release);
        CLONE.store(clone.as_mut_ptr() as u32, Ordering::Release);
        READY.store(true, Ordering::Release);
        let mut result = [0u32; 4];
        unsafe {
            invoke_fixture(source.as_ptr() as u32, result.as_mut_ptr());
        }
        assert_eq!(result[0], clone.as_ptr() as u32);
        assert_ne!(result[1] & 1, 0, "native CF survives callback");
        assert_eq!(
            result[2], result[3],
            "RET1C consumes exactly the original blob"
        );
        assert_eq!(ARGUMENTS[0].load(Ordering::Acquire), source.as_ptr() as u32);
        for (i, argument) in ARGUMENTS.iter().enumerate().skip(1) {
            assert_eq!(argument.load(Ordering::Acquire), i as u32);
        }
        assert_eq!(clone[0x34 / 4], 0xCAFE0007);
        assert_eq!(clone[0x38 / 4], 0x12345678);
        source[0x44 / 4] = u32::from_le_bytes(*b"base");
        clone[0x34 / 4] = 0xCAFEFFFF;
        clone_metadata(source.as_ptr() as u32, clone.as_ptr() as u32);
        assert_eq!(clone[0x34 / 4], 0xCAFEFFFF);
        READY.store(false, Ordering::Release);
    }
}
