//! Narrow, read-only compatibility with the official CreatineCore6.36 role probe.
//! from V2DLL: V2TechButton.cpp, InstallServerRoleProbe/ServerSendThunk contract.
use crate::snapshot::Memory;
fn bytes(m: &impl Memory, p: u32, len: usize) -> Option<Vec<u8>> {
    (0..len)
        .map(|i| m.read32(p.checked_add(i as u32)?).map(|v| v as u8))
        .collect()
}
fn branch(m: &impl Memory, p: u32, opcode: u8) -> Option<u32> {
    if m.read32(p)? as u8 != opcode {
        return None;
    }
    Some(p.wrapping_add(5).wrapping_add(m.read32(p.checked_add(1)?)?))
}
fn chain(m: &impl Memory, game: u32, core: u32, original: &[u8]) -> bool {
    let valid = (|| -> Option<bool> {
        let entry = game.checked_add(0x662EA0)?;
        if original.len() < 8
            || bytes(m, entry + 5, 3)? != [0x90; 3]
            || bytes(m, entry + 8, original.len() - 8)? != original[8..]
        {
            return Some(false);
        }
        let thunk = branch(m, entry, 0xE9)?;
        if thunk != core.checked_add(0x12A80)? {
            return Some(false);
        }
        let mut expected = vec![
            0x9C, 0x60, 0xE8, 0xE9, 0xFF, 0xFF, 0xFF, 0x61, 0x9D, 0xFF, 0x25,
        ];
        expected.extend_from_slice(&core.checked_add(0x82EE50)?.to_le_bytes());
        if bytes(m, thunk, 15)? != expected {
            return Some(false);
        }
        let mut mark = vec![0xB8, 1, 0, 0, 0, 0xB9];
        mark.extend_from_slice(&core.checked_add(0x9931FC)?.to_le_bytes());
        mark.extend_from_slice(&[0x87, 1, 0xC3]);
        if bytes(m, core + 0x12A70, 13)? != mark {
            return Some(false);
        }
        let trampoline = m.read32(core + 0x82EE50)?;
        if trampoline < core
            || trampoline.checked_add(16)? > core.checked_add(0xD8D000)?
            || bytes(m, trampoline, 8)? != original[..8]
        {
            return Some(false);
        }
        Some(branch(m, trampoline + 8, 0xE9)? == entry + 8)
    })();
    valid == Some(true)
}
#[cfg(all(windows, target_arch = "x86"))]
pub fn server_role_probe(n: crate::native::Native, original: &[u8]) -> bool {
    use std::{ffi::c_void, io::Read, os::windows::ffi::OsStringExt};
    unsafe extern "system" {
        fn GetModuleHandleExW(flags: u32, address: *const u16, module: *mut *mut c_void) -> i32;
        fn GetModuleFileNameW(module: *mut c_void, path: *mut u16, size: u32) -> u32;
    }
    let Some(target) = branch(&n, n.base + 0x662EA0, 0xE9) else {
        return false;
    };
    let mut module = std::ptr::null_mut();
    if unsafe { GetModuleHandleExW(6, target as *const u16, &mut module) } == 0 {
        return false;
    }
    let core = module as u32;
    if target != core.saturating_add(0x12A80) {
        return false;
    }
    let mut name = [0u16; 32768];
    let len = unsafe { GetModuleFileNameW(module, name.as_mut_ptr(), name.len() as u32) } as usize;
    if len == 0 || len >= name.len() {
        return false;
    }
    let path = std::path::PathBuf::from(std::ffi::OsString::from_wide(&name[..len]));
    let Ok(mut file) = std::fs::File::open(&path) else {
        return false;
    };
    let mut hash = crate::sha256::Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let Ok(count) = file.read(&mut buffer) else {
            return false;
        };
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    if hash.finish() != "a36cc0339e83db2cab2d6c60047d84c1abf5e90b9fb267d54cbcee7d7191c5a1" {
        return false;
    }
    chain(&n, n.base, core, original)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    struct Fixture(BTreeMap<u32, u8>);
    impl Memory for Fixture {
        fn read32(&self, p: u32) -> Option<u32> {
            Some(u32::from_le_bytes([
                *self.0.get(&p)?,
                *self.0.get(&(p + 1))?,
                *self.0.get(&(p + 2))?,
                *self.0.get(&(p + 3))?,
            ]))
        }
    }
    fn put(m: &mut Fixture, p: u32, b: &[u8]) {
        for (i, &v) in b.iter().enumerate() {
            m.0.insert(p + i as u32, v);
        }
    }
    fn jump(from: u32, to: u32) -> Vec<u8> {
        let mut b = vec![0xE9];
        b.extend_from_slice(&to.wrapping_sub(from + 5).to_le_bytes());
        b
    }
    #[test]
    fn known_probe_preserves_original_tail_and_trampoline_and_rejects_other_hooks() {
        let game = 0x30000;
        let core = 0x10000000;
        let entry = game + 0x662EA0;
        let trampoline = core + 0x82EE60;
        let original = [
            0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8, 0x6A, 0xFF, 0x68, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77,
        ];
        let mut m = Fixture(BTreeMap::new());
        put(&mut m, entry, &original);
        put(&mut m, entry, &jump(entry, core + 0x12A80));
        put(&mut m, entry + 5, &[0x90; 3]);
        let mut thunk = vec![
            0x9C, 0x60, 0xE8, 0xE9, 0xFF, 0xFF, 0xFF, 0x61, 0x9D, 0xFF, 0x25,
        ];
        thunk.extend_from_slice(&(core + 0x82EE50).to_le_bytes());
        thunk.extend_from_slice(&[0; 4]);
        put(&mut m, core + 0x12A80, &thunk);
        let mut mark = vec![0xB8, 1, 0, 0, 0, 0xB9];
        mark.extend_from_slice(&(core + 0x9931FC).to_le_bytes());
        mark.extend_from_slice(&[0x87, 1, 0xC3, 0, 0, 0]);
        put(&mut m, core + 0x12A70, &mark);
        put(&mut m, core + 0x82EE50, &trampoline.to_le_bytes());
        put(&mut m, trampoline, &original[..8]);
        let mut tail = jump(trampoline + 8, entry + 8);
        tail.extend_from_slice(&[0; 3]);
        put(&mut m, trampoline + 8, &tail);
        put(&mut m, entry + 16, &[0; 4]);
        assert!(chain(&m, game, core, &original));
        m.0.insert(entry + 8, 0x90);
        assert!(!chain(&m, game, core, &original));
    }
}
