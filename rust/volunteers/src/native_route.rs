//! Reuses our accepted ATCU land-route ABI, predicate and engine list cleanup.
//! A preview projects only a private input image, never the live army owner.
use crate::native::Native;
use crate::native_call::call;
#[repr(C)]
#[derive(Default)]
struct Path {
    head: u32,
    tail: u32,
    count: i32,
    flags: u32,
    extra: u32,
}
unsafe fn query(base: u32, start: u32, stack: u32, destination: u32, path: *mut Path) -> u32 {
    let finder = base + 0xA03794;
    let result: u32;
    unsafe {
        std::arch::asm!(
        "push {path}","push {destination}","push {stack}","push {finder}","call {function}",
        path=in(reg) path,destination=in(reg) destination,stack=in(reg) stack,
        finder=in(reg) &finder,function=in(reg) base+0x1B8500,
        inlateout("eax") start=>result,clobber_abi("C"));
    }
    result
}
impl Native {
    pub fn return_destination(&self, army: u32, donor: u32, preferred: u32) -> Option<u32> {
        if self.land_route(army, preferred, None) == Some(true) {
            return Some(preferred);
        }
        let country = self.country(donor)?;
        let start = self.field(country, 0x9D8)?;
        let end = self.field(country, 0x9DC)?;
        let bytes = end.checked_sub(start)?;
        if start == 0 || bytes % 4 != 0 || bytes / 4 > 65536 {
            return None;
        }
        let world = self.u32(self.base + 0xE588E8)?;
        let provinces = self.field(world, 0xACC)?;
        let count = self.field(world, 0xAD0)?.checked_sub(provinces)? / 4;
        let mut attempts = 0;
        for offset in (0..bytes).step_by(4) {
            let id = self.field(start, offset)?;
            if id == preferred || id == 0 || id >= count {
                continue;
            }
            let province = self.field(provinces, id * 4)?;
            if self.field(province, 0x12C)? != donor
                || self.byte(self.field(province, 0x5C)? + 0x2A)? == 0
            {
                continue;
            }
            // Bound native pathfinder work per command. No sea/teleport fallback.
            attempts += 1;
            if attempts > 128 {
                break;
            }
            if self.land_route(army, id, None) == Some(true) {
                return Some(id);
            }
        }
        None
    }
    pub fn land_route(
        &self,
        army: u32,
        destination: u32,
        projected_country: Option<u32>,
    ) -> Option<bool> {
        if self.u32(army)? != self.base + 0x9F88CC || destination == 0 || destination > 65535 {
            return None;
        }
        let world = self.u32(self.base + 0xE588E8)?;
        let provinces = self.field(world, 0xACC)?;
        let count = self.field(world, 0xAD0)?.checked_sub(provinces)? / 4;
        if count > 65536 || destination >= count {
            return None;
        }
        let start = self.field(army, 0xDC)?;
        let target = self.field(provinces, destination * 4)?;
        for province in [start, target] {
            if self.byte(self.field(province, 0x5C)? + 0x2A)? == 0 {
                return Some(false);
            }
        }
        if start == target {
            return Some(true);
        }
        let mut projection = [0u32; 0x1B8 / 4];
        let stack = if let Some(index) = projected_country {
            let country = self.country(index)?;
            let bytes = self.bytes(army, 0x1B8)?;
            for (word, chunk) in projection.iter_mut().zip(bytes.as_chunks::<4>().0) {
                *word = u32::from_le_bytes(*chunk);
            }
            projection[0xC0 / 4] = self.field(country, 0x1C)?;
            projection[0xC4 / 4] = index;
            projection.as_ptr() as u32
        } else {
            army
        };
        let mut path = Path::default();
        let pointer = &raw mut path;
        let found = call(|| unsafe { query(self.base, start, stack, target, pointer) });
        let result = (|| {
            if found? as u8 == 0 {
                return Some(false);
            }
            if path.count < 0 || path.count > 65536 {
                return None;
            }
            let mut node = path.head;
            let mut seen = std::collections::BTreeSet::new();
            while node != 0 {
                if seen.len() >= 65536 || !seen.insert(node) {
                    return None;
                }
                let id = self.u32(node)?;
                if id >= count {
                    return None;
                }
                let province = self.field(provinces, id * 4)?;
                if self.byte(self.field(province, 0x5C)? + 0x2A)? == 0 {
                    return Some(false);
                }
                node = self.field(node, 8)?;
            }
            (seen.len() == path.count as usize).then_some(true)
        })();
        type Clear = unsafe extern "fastcall" fn(*mut Path, u32);
        let cleared = call(|| unsafe {
            std::mem::transmute::<usize, Clear>((self.base + 0x449E90) as usize)(pointer, 0)
        });
        cleared?;
        result
    }
}
