//! Read-only recognition of transparent host-role probes, independent of DLL
//! version, filename and link layout. Never redirects or bypasses another mod.
//! from V2DLL: V2TechButton.cpp, ServerSendThunk/MarkServerSide instruction contract.
use crate::snapshot::Memory;
use std::collections::BTreeSet;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Image {
    base: u32,
    end: u32,
}
impl Image {
    fn contains(self, p: u32, len: u32) -> bool {
        p >= self.base && p.checked_add(len).is_some_and(|end| end <= self.end)
    }
}
trait HookMemory: Memory {
    fn image(&self, p: u32) -> Option<Image>;
    fn executable(&self, p: u32, len: u32) -> bool;
    fn writable(&self, p: u32, len: u32) -> bool;
}
#[derive(Debug, PartialEq, Eq)]
enum Refusal {
    Read,
    Entry,
    Tail,
    Code,
    Cycle,
    Budget,
    Marker,
    Data,
    Continuation,
}
fn bytes(m: &impl Memory, p: u32, len: usize) -> Option<Vec<u8>> {
    (0..len)
        .map(|i| m.read32(p.checked_add(i as u32)?).map(|v| v as u8))
        .collect()
}
fn direct(m: &impl Memory, p: u32) -> Option<u32> {
    if m.read32(p)? as u8 != 0xE9 {
        return None;
    }
    Some(p.wrapping_add(5).wrapping_add(m.read32(p.checked_add(1)?)?))
}
struct Walk {
    seen: BTreeSet<u32>,
    probes: u32,
}
impl Walk {
    fn step(&mut self, p: u32) -> Result<(), Refusal> {
        if self.seen.len() >= 24 {
            return Err(Refusal::Budget);
        }
        if !self.seen.insert(p) {
            return Err(Refusal::Cycle);
        }
        Ok(())
    }
}
/// Only bounded direct JMP veneers are allowed before known code. Arbitrary
/// indirect dispatch is not evidence of a transparent function contract.
fn veneers(m: &impl HookMemory, mut p: u32, w: &mut Walk) -> Result<u32, Refusal> {
    loop {
        w.step(p)?;
        if !m.executable(p, 5) {
            return Err(Refusal::Code);
        }
        match direct(m, p) {
            Some(next) => p = next,
            None => return Ok(p),
        }
    }
}
fn chain(m: &impl HookMemory, game: u32, original: &[u8]) -> Result<u32, Refusal> {
    let entry = game.checked_add(0x662EA0).ok_or(Refusal::Entry)?;
    if original.len() < 8 || bytes(m, entry + 5, 3).ok_or(Refusal::Read)? != [0x90; 3] {
        return Err(Refusal::Entry);
    }
    if bytes(m, entry + 8, original.len() - 8).ok_or(Refusal::Read)? != original[8..] {
        return Err(Refusal::Tail);
    }
    let mut p = direct(m, entry).ok_or(Refusal::Entry)?;
    let mut walk = Walk {
        seen: BTreeSet::from([entry]),
        probes: 0,
    };
    loop {
        p = veneers(m, p, &mut walk)?;
        // Final original trampoline: exact eight stolen bytes + direct return.
        if bytes(m, p, 8).as_deref() == Some(&original[..8]) {
            if !m.executable(p, 13) || direct(m, p + 8) != Some(entry + 8) || walk.probes == 0 {
                return Err(Refusal::Continuation);
            }
            return Ok(walk.probes);
        }
        if walk.probes >= 4 || !m.executable(p, 15) {
            return Err(Refusal::Code);
        }
        let b = bytes(m, p, 15).ok_or(Refusal::Read)?;
        if b[..3] != [0x9C, 0x60, 0xE8] || b[7..11] != [0x61, 0x9D, 0xFF, 0x25] {
            return Err(Refusal::Code);
        }
        let image = m
            .image(p)
            .filter(|i| i.base != game && i.contains(p, 15))
            .ok_or(Refusal::Code)?;
        let marker = p
            .wrapping_add(7)
            .wrapping_add(u32::from_le_bytes(b[3..7].try_into().unwrap()));
        let marker = veneers(m, marker, &mut walk)?;
        if m.image(marker) != Some(image)
            || !image.contains(marker, 13)
            || !m.executable(marker, 13)
        {
            return Err(Refusal::Marker);
        }
        let mark = bytes(m, marker, 13).ok_or(Refusal::Read)?;
        // MOV EAX,1; MOV ECX,&private_flag; XCHG [ECX],EAX; RET.
        // Only private aligned DLL data changes; PUSHFD/PUSHAD restores state.
        if mark[..6] != [0xB8, 1, 0, 0, 0, 0xB9] || mark[10..] != [0x87, 1, 0xC3] {
            return Err(Refusal::Marker);
        }
        let flag = u32::from_le_bytes(mark[6..10].try_into().unwrap());
        let slot = u32::from_le_bytes(b[11..15].try_into().unwrap());
        if !flag.is_multiple_of(4)
            || !slot.is_multiple_of(4)
            || !image.contains(flag, 4)
            || !image.contains(slot, 4)
            || !m.writable(flag, 4)
        {
            return Err(Refusal::Data);
        }
        p = m.read32(slot).ok_or(Refusal::Read)?;
        walk.probes += 1;
    }
}
#[cfg(all(windows, target_arch = "x86"))]
mod live {
    use super::*;
    use crate::native::Native;
    use std::ffi::c_void;
    #[repr(C)]
    struct Region {
        base: *mut c_void,
        allocation: *mut c_void,
        allocation_protect: u32,
        size: usize,
        state: u32,
        protect: u32,
        kind: u32,
    }
    unsafe extern "system" {
        fn VirtualQuery(address: *const c_void, info: *mut Region, size: usize) -> usize;
        fn GetModuleFileNameW(module: *mut c_void, path: *mut u16, size: u32) -> u32;
    }
    fn region(p: u32, len: u32) -> Option<Region> {
        let mut r = std::mem::MaybeUninit::uninit();
        if unsafe {
            VirtualQuery(
                p as *const c_void,
                r.as_mut_ptr(),
                std::mem::size_of::<Region>(),
            )
        } != std::mem::size_of::<Region>()
        {
            return None;
        }
        let r = unsafe { r.assume_init() };
        (r.state == 0x1000
            && r.protect & 0x101 == 0
            && p.checked_add(len).is_some_and(|end| {
                (r.base as u32)
                    .checked_add(r.size as u32)
                    .is_some_and(|limit| end <= limit)
            }))
        .then_some(r)
    }
    // ReadProcessMemory avoids poisoning the native-call fault fence when an
    // unrelated hook contains an unreadable pointer. No foreign code executes.
    struct Probe(Native);
    impl Memory for Probe {
        fn read32(&self, p: u32) -> Option<u32> {
            Some(u32::from_le_bytes(self.0.bytes(p, 4)?.try_into().ok()?))
        }
    }
    impl HookMemory for Probe {
        fn image(&self, p: u32) -> Option<Image> {
            let r = region(p, 1)?;
            if r.kind != 0x1000000 {
                return None;
            }
            let base = r.allocation as u32;
            if self.read32(base)? & 0xFFFF != 0x5A4D {
                return None;
            }
            let offset = self.read32(base.checked_add(0x3C)?)?;
            if offset > 0x100000 {
                return None;
            }
            let nt = base.checked_add(offset)?;
            if self.read32(nt)? != 0x4550
                || self.read32(nt + 4)? & 0xFFFF != 0x14C
                || self.read32(nt + 24)? & 0xFFFF != 0x10B
            {
                return None;
            }
            let size = self.read32(nt + 0x50)?;
            if !(0x1000..=0x10000000).contains(&size) {
                return None;
            }
            let image = Image {
                base,
                end: base.checked_add(size)?,
            };
            image.contains(p, 1).then_some(image)
        }
        fn executable(&self, p: u32, len: u32) -> bool {
            region(p, len).is_some_and(|r| matches!(r.protect & 0xFF, 0x10 | 0x20 | 0x40 | 0x80))
        }
        fn writable(&self, p: u32, len: u32) -> bool {
            region(p, len).is_some_and(|r| matches!(r.protect & 0xFF, 0x04 | 0x08 | 0x40 | 0x80))
        }
    }
    fn module_label(probe: &Probe, target: u32) -> String {
        use std::os::windows::ffi::OsStringExt;
        let Some(image) = probe.image(target) else {
            return format!("address={target:08X} owner=unmapped-or-private");
        };
        let mut name = [0u16; 32768];
        let len = unsafe {
            GetModuleFileNameW(
                image.base as *mut c_void,
                name.as_mut_ptr(),
                name.len() as u32,
            )
        } as usize;
        if len == 0 || len >= name.len() {
            return format!(
                "base={:08X} rva={:X} owner=unknown",
                image.base,
                target - image.base
            );
        }
        let path = std::path::PathBuf::from(std::ffi::OsString::from_wide(&name[..len]));
        format!(
            "path={} base={:08X} rva={:X} sha256={}",
            path.display(),
            image.base,
            target - image.base,
            crate::runtime::exe_hash(&path)
                .as_deref()
                .unwrap_or("unreadable")
        )
    }
    pub fn verify(n: Native, original: &[u8]) -> bool {
        let probe = Probe(n);
        let target = direct(&probe, n.base + 0x662EA0);
        let label = target.map_or_else(|| "owner=unresolved".into(), |p| module_label(&probe, p));
        match chain(&probe, n.base, original) {
            Ok(probes) => {
                crate::runtime::diagnostic(&format!(
                    "native_hook_chain_verified rva=662EA0 transparent_role_probes={probes} {label}"
                ));
                true
            }
            Err(reason) => {
                crate::runtime::diagnostic(&format!(
                    "native_hook_chain_refused rva=662EA0 reason={reason:?} {label}"
                ));
                false
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicU32, Ordering};
        static FLAG: AtomicU32 = AtomicU32::new(0);
        static CONTINUATION: AtomicU32 = AtomicU32::new(0);
        #[unsafe(naked)]
        unsafe extern "C" fn marker() {
            std::arch::naked_asm!("mov eax,1","mov ecx,offset {flag}","xchg [ecx],eax","ret",flag=sym FLAG);
        }
        #[unsafe(naked)]
        unsafe extern "C" fn thunk() {
            std::arch::naked_asm!("pushfd","pushad","call {marker}","popad","popfd","jmp dword ptr [{next}]",marker=sym marker,next=sym CONTINUATION);
        }
        unsafe extern "system" {
            fn VirtualAlloc(p: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
            fn VirtualFree(p: *mut c_void, size: usize, kind: u32) -> i32;
            fn VirtualProtect(p: *const c_void, size: usize, protect: u32, old: *mut u32) -> i32;
        }
        #[test]
        fn actual_x86_image_and_page_permissions_accept_relinked_probe_without_running_it() {
            let block = unsafe { VirtualAlloc(std::ptr::null_mut(), 0x664000, 0x3000, 4) };
            assert!(!block.is_null());
            let game = block as u32;
            let entry = game + 0x662EA0;
            let trampoline = game + 0x663000;
            let original = [
                0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8, 0x6A, 0xFF, 0x68, 0x11, 0x22, 0x33, 0x44, 0x55,
                0x66, 0x77,
            ];
            unsafe {
                std::ptr::copy_nonoverlapping(original.as_ptr(), entry as *mut u8, 16);
                std::ptr::write_unaligned(entry as *mut u8, 0xE9);
                std::ptr::write_unaligned(
                    (entry + 1) as *mut u32,
                    (thunk as *const () as u32).wrapping_sub(entry + 5),
                );
                std::ptr::write_bytes((entry + 5) as *mut u8, 0x90, 3);
                std::ptr::copy_nonoverlapping(original.as_ptr(), trampoline as *mut u8, 8);
                std::ptr::write_unaligned((trampoline + 8) as *mut u8, 0xE9);
                std::ptr::write_unaligned(
                    (trampoline + 9) as *mut u32,
                    (entry + 8).wrapping_sub(trampoline + 13),
                );
            }
            CONTINUATION.store(trampoline, Ordering::Release);
            let mut old = 0;
            assert_ne!(
                unsafe { VirtualProtect(trampoline as *mut c_void, 4096, 0x20, &mut old) },
                0
            );
            let probe = Probe(Native { base: game });
            assert_eq!(chain(&probe, game, &original), Ok(1));
            assert_eq!(
                FLAG.load(Ordering::Acquire),
                0,
                "verification must never execute the foreign marker"
            );
            assert_ne!(
                unsafe { VirtualProtect(trampoline as *mut c_void, 4096, 4, &mut old) },
                0
            );
            assert_eq!(chain(&probe, game, &original), Err(Refusal::Code));
            assert_ne!(unsafe { VirtualFree(block, 0, 0x8000) }, 0);
        }
    }
}
#[cfg(all(windows, target_arch = "x86"))]
pub use live::verify as server_role_probe;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    const ORIGINAL: [u8; 16] = [
        0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8, 0x6A, 0xFF, 0x68, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x77,
    ];
    struct Fixture {
        data: BTreeMap<u32, u8>,
        images: Vec<Image>,
        code: Vec<(u32, u32)>,
        write: Vec<(u32, u32)>,
    }
    impl Memory for Fixture {
        fn read32(&self, p: u32) -> Option<u32> {
            Some(u32::from_le_bytes([
                *self.data.get(&p)?,
                *self.data.get(&p.checked_add(1)?)?,
                *self.data.get(&p.checked_add(2)?)?,
                *self.data.get(&p.checked_add(3)?)?,
            ]))
        }
    }
    impl HookMemory for Fixture {
        fn image(&self, p: u32) -> Option<Image> {
            self.images.iter().find(|i| i.contains(p, 1)).copied()
        }
        fn executable(&self, p: u32, n: u32) -> bool {
            self.code
                .iter()
                .any(|(start, end)| p >= *start && p.checked_add(n).is_some_and(|x| x <= *end))
        }
        fn writable(&self, p: u32, n: u32) -> bool {
            self.write
                .iter()
                .any(|(start, end)| p >= *start && p.checked_add(n).is_some_and(|x| x <= *end))
        }
    }
    fn put(m: &mut Fixture, p: u32, b: &[u8]) {
        for (i, v) in b.iter().enumerate() {
            m.data.insert(p + i as u32, *v);
        }
    }
    fn word(m: &mut Fixture, p: u32, v: u32) {
        put(m, p, &v.to_le_bytes())
    }
    fn jump(m: &mut Fixture, p: u32, to: u32) {
        put(m, p, &[0xE9]);
        word(m, p + 1, to.wrapping_sub(p + 5));
    }
    fn probe(m: &mut Fixture, thunk: u32, marker: u32, slot: u32, flag: u32, continuation: u32) {
        let mut b = vec![0x9C, 0x60, 0xE8];
        b.extend_from_slice(&marker.wrapping_sub(thunk + 7).to_le_bytes());
        b.extend_from_slice(&[0x61, 0x9D, 0xFF, 0x25]);
        b.extend_from_slice(&slot.to_le_bytes());
        b.extend_from_slice(&[0; 4]);
        put(m, thunk, &b);
        let mut mark = vec![0xB8, 1, 0, 0, 0, 0xB9];
        mark.extend_from_slice(&flag.to_le_bytes());
        mark.extend_from_slice(&[0x87, 1, 0xC3, 0, 0, 0]);
        put(m, marker, &mark);
        word(m, slot, continuation);
        word(m, flag, 0);
        m.write.push((flag, flag + 4));
    }
    fn fixture(game: u32, core: u32, offset: u32) -> Fixture {
        let entry = game + 0x662EA0;
        let thunk = core + offset;
        let marker = thunk + 0x80;
        let slot = core + 0x8000;
        let flag = core + 0x9000;
        let trampoline = core + 0x7000;
        let mut m = Fixture {
            data: BTreeMap::new(),
            images: vec![
                Image {
                    base: game,
                    end: game + 0x1000000,
                },
                Image {
                    base: core,
                    end: core + 0x10000,
                },
            ],
            code: vec![(entry, entry + 32), (core, core + 0x8000)],
            write: vec![],
        };
        put(&mut m, entry, &ORIGINAL);
        put(&mut m, entry + 16, &[0; 4]);
        jump(&mut m, entry, thunk);
        put(&mut m, entry + 5, &[0x90; 3]);
        put(&mut m, trampoline, &ORIGINAL[..8]);
        jump(&mut m, trampoline + 8, entry + 8);
        put(&mut m, trampoline + 13, &[0; 4]);
        probe(&mut m, thunk, marker, slot, flag, trampoline);
        m
    }
    #[test]
    fn transparent_contract_survives_aslr_different_link_layouts_and_marker_state() {
        for (game, core, offset) in [
            (0x30000, 0x10000000, 0x1280),
            (0x5F0000, 0x66A50000, 0x2AD0),
            (0x90000, 0x6F000000, 0x5000),
        ] {
            let mut m = fixture(game, core, offset);
            word(&mut m, core + 0x9000, 1);
            assert_eq!(chain(&m, game, &ORIGINAL), Ok(1));
        }
    }
    #[test]
    fn follows_bounded_linker_veneers_and_preserves_two_existing_role_probes() {
        let game = 0x30000;
        let core = 0x10000000;
        let mut m = fixture(game, core, 0x1280);
        jump(&mut m, game + 0x662EA0, core + 0x100);
        jump(&mut m, core + 0x100, core + 0x1280);
        jump(&mut m, core + 0x1400, core + 0x1300);
        word(
            &mut m,
            core + 0x1283,
            (core + 0x1400u32).wrapping_sub(core + 0x1287),
        );
        probe(
            &mut m,
            core + 0x4000,
            core + 0x4080,
            core + 0x8010,
            core + 0x9010,
            core + 0x7000,
        );
        word(&mut m, core + 0x8000, core + 0x4000);
        assert_eq!(chain(&m, game, &ORIGINAL), Ok(2));
    }
    #[test]
    fn rejects_gameplay_code_and_modified_original_continuations() {
        let game = 0x30000;
        let core = 0x10000000;
        for (p, b) in [
            (game + 0x662EA8, 0x90),
            (core + 0x1287, 0x90),
            (core + 0x1301, 2),
            (core + 0x130A, 0x89),
            (core + 0x7000, 0x90),
            (core + 0x7008, 0xC3),
        ] {
            let mut m = fixture(game, core, 0x1280);
            put(&mut m, p, &[b]);
            assert!(chain(&m, game, &ORIGINAL).is_err(), "{p:X}");
        }
    }
    #[test]
    fn refuses_writes_outside_own_module_and_non_executable_unreadable_targets() {
        let game = 0x30000;
        let core = 0x10000000;
        let mut m = fixture(game, core, 0x1280);
        word(&mut m, core + 0x1306, game + 0x9000);
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Data));
        let mut m = fixture(game, core, 0x1280);
        m.write.clear();
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Data));
        let mut m = fixture(game, core, 0x1280);
        m.code.retain(|(p, _)| *p != core);
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Code));
        let mut m = fixture(game, core, 0x1280);
        word(&mut m, core + 0x8000, 0x60000000);
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Code));
    }
    #[test]
    fn loops_and_long_dispatch_chains_are_bounded_and_never_repaired() {
        let game = 0x30000;
        let core = 0x10000000;
        let mut m = fixture(game, core, 0x1280);
        jump(&mut m, core + 0x1280, core + 0x1280);
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Cycle));
        let mut m = fixture(game, core, 0x1280);
        jump(&mut m, game + 0x662EA0, core + 0x100);
        for i in 0..30 {
            jump(&mut m, core + 0x100 + i * 8, core + 0x108 + i * 8)
        }
        assert_eq!(chain(&m, game, &ORIGINAL), Err(Refusal::Budget));
    }
}
