//! The player's country and its upper house, read without engine calls (DLL-1); port of
//! tgcx_dllmain.cpp's snapshot and its 10-second diagnostic sample.

use windows_sys::Win32::System::SystemInformation::GetTickCount;

use crate::signatures::IDEOLOGY_CANDIDATES;
use crate::sys::{Bytes, Global, at, base, guard, rd8, rd32, rdf, rdp};

pub const IDEOLOGIES: usize = IDEOLOGY_CANDIDATES.len();
pub const WORLD_RVA: u32 = 0xE588E8;
pub const COUNTRY_MANAGER_RVA: u32 = 0xE587E4;
const IDEOLOGY_MANAGER_RVA: u32 = 0xE58A3C;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Snapshot {
    pub date_hours: u32,
    pub index: u32,
    pub country_index: u32,
    pub human_entries: u32,
    pub human_marker: u32,
    pub world: usize,
    pub country: usize,
    pub upper_house: usize,
    pub array: usize,
    pub upper_house_vtable: usize,
    pub world_tag: [u8; 4],
    pub country_tag: [u8; 4],
    pub shares: [i64; IDEOLOGIES],
    pub sum: i64,
    pub shares_in_range: bool,
    pub indices: [u32; IDEOLOGIES],
    pub colors: [u32; IDEOLOGIES],
    pub array_entries: u32,
}

impl Snapshot {
    /// The country tag as printf's "%.4s" prints it.
    pub fn tag(&self) -> Bytes<'_> {
        let end = self.country_tag.iter().position(|&b| b == 0).unwrap_or(4);
        Bytes(&self.country_tag[..end])
    }
}

/// Read-only equivalent of FUN_00973BD0's bounded name lookup. No engine allocation.
pub fn find_ideology(manager: usize, name: &str) -> usize {
    let buckets = rd32(manager + 4);
    let table = rdp(manager + 8);
    if buckets == 0 || buckets > 65536 || table == 0 {
        return 0;
    }
    let mut hash = 0u32;
    for &c in name.as_bytes() {
        hash = hash.wrapping_add(u32::from(c)).wrapping_mul(61);
    }
    let mut node = rdp(table + (hash % buckets) as usize * 4);
    let mut n = 0;
    while node != 0 && n < 64 {
        let object = rdp(node);
        if object == 0 {
            return 0;
        }
        let size = rd32(object + 0x1C) as usize;
        if size == name.len() {
            let text = if rd32(object + 0x20) > 15 {
                rdp(object + 0xC)
            } else {
                object + 0xC
            };
            // SAFETY: inside the caller's guard.
            if text != 0
                && unsafe { std::slice::from_raw_parts(text as *const u8, size) } == name.as_bytes()
            {
                return object;
            }
        }
        node = rdp(node + 4);
        n += 1;
    }
    0
}

fn read(sample: &mut Snapshot) -> bool {
    let world = rdp(at(WORLD_RVA));
    let manager = rdp(at(COUNTRY_MANAGER_RVA));
    if world == 0 || manager == 0 {
        return false;
    }
    let count = rd32(manager);
    let index = rd32(world + 0xB60);
    if count == 0 || count > 65536 || index >= count {
        return false;
    }
    let countries = rdp(manager + 4);
    if countries == 0 || rdp(countries + index as usize * 4) == 0 {
        return false;
    }
    let country = rdp(countries + index as usize * 4);
    let country_index = rd32(country + 0x20);
    let tag_matches = (0..3).all(|i| rd8(world + 0xB5C + i) == rd8(country + 0x1C + i));
    if country_index != index || !tag_matches {
        return false;
    }
    // A selected tag with no provinces is not a playable country.
    let provinces_begin = rdp(country + 0x9D8);
    let provinces_end = rdp(country + 0x9DC);
    if provinces_begin == 0 || provinces_end <= provinces_begin {
        return false;
    }
    let array = rdp(country + 0x8B0 + 0x10);
    let end_array = rdp(country + 0x8B0 + 0x14);
    if array == 0 || end_array < array || !(end_array - array).is_multiple_of(8) {
        return false;
    }
    if rdp(country + 0x8B0) != base() + 0x9FD7DC {
        return false;
    }
    // Nine named ideologies plus the null ideology. Never infer the allocation from a file order.
    sample.array_entries = ((end_array - array) / 8) as u32;
    if sample.array_entries as usize != IDEOLOGIES + 1 {
        return false;
    }
    let ideology_manager = rdp(at(IDEOLOGY_MANAGER_RVA));
    if ideology_manager == 0 {
        return false;
    }
    let mut used = 0u32;
    sample.sum = 0;
    sample.shares_in_range = true;
    for (i, name) in IDEOLOGY_CANDIDATES.iter().enumerate() {
        let ideology = find_ideology(ideology_manager, name);
        if ideology == 0 || rdp(ideology) != base() + 0xA36774 {
            return false;
        }
        let slot = rd32(ideology + 0x28);
        if slot == 0 || slot >= sample.array_entries || used & (1 << slot) != 0 {
            return false;
        }
        used |= 1 << slot;
        sample.indices[i] = slot;
        let mut color = 0xFF000000u32;
        for component in 0..3 {
            let value = rdf(ideology + 0x34 + component * 4);
            if !(0.0..=1.0).contains(&value) {
                return false;
            }
            color |= ((value * 255.0 + 0.5) as u32) << (16 - component * 8);
        }
        sample.colors[i] = color;
        let value = crate::sys::rd::<i64>(array + slot as usize * 8);
        sample.shares[i] = value;
        if !(0..=32768).contains(&value) {
            sample.shares_in_range = false;
        } else {
            sample.sum += value;
        }
    }
    if crate::sys::rd::<i64>(array) != 0 || (sample.shares_in_range && sample.sum == 0) {
        return false;
    }
    sample.world = world;
    sample.country = country;
    sample.upper_house = country + 0x8B0;
    sample.upper_house_vtable = rdp(country + 0x8B0);
    sample.array = array;
    sample.index = index;
    sample.country_index = country_index;
    sample.date_hours = rd32(world + 0xB0C);
    for i in 0..4 {
        sample.world_tag[i] = rd8(world + 0xB5C + i);
        sample.country_tag[i] = rd8(country + 0x1C + i);
    }
    let begin = rdp(world + 0xAEC);
    let end = rdp(world + 0xAF0);
    sample.human_entries = 0;
    sample.human_marker = 0;
    if begin != 0
        && end >= begin
        && (end - begin).is_multiple_of(4)
        && (end - begin) / 4 <= count as usize
    {
        sample.human_entries = ((end - begin) / 4) as u32;
        if index < sample.human_entries {
            sample.human_marker = rd32(begin + index as usize * 4);
        }
    }
    true
}

/// The snapshot, or None; `fault` is set when a guarded read faulted.
pub fn read_snapshot(fault: &mut bool) -> Option<Snapshot> {
    let mut sample = Snapshot::default();
    match guard(|| read(&mut sample)) {
        Some(true) => Some(sample),
        Some(false) => None,
        None => {
            *fault = true;
            None
        }
    }
}

pub fn snapshot() -> Option<Snapshot> {
    read_snapshot(&mut false)
}

pub struct Pump {
    pub last_sample: u32,
    sampled: bool,
    fault_reported: bool,
}

pub static PUMP: Global<Pump> = Global::new(Pump {
    last_sample: 0,
    sampled: false,
    fault_reported: false,
});

/// Logs the upper house every ten seconds (DLL-1 diagnostics).
pub fn on_frame_pump() {
    let pump = PUMP.get();
    let now = unsafe { GetTickCount() };
    if pump.sampled && now.wrapping_sub(pump.last_sample) < 10000 {
        return;
    }
    pump.sampled = true;
    pump.last_sample = now;
    let mut fault = false;
    let Some(s) = read_snapshot(&mut fault) else {
        if fault && !pump.fault_reported {
            crate::log!(
                "Read skipped: guarded game-state access fault (loading or unsupported layout)"
            );
            pump.fault_reported = true;
        }
        return;
    };
    pump.fault_reported = false;
    let tag = |t: &[u8; 4]| {
        let end = t.iter().position(|&b| b == 0).unwrap_or(4);
        Bytes(&t[..end]).to_string()
    };
    crate::log!(
        "upper_house hours={} year={} day_of_year={} world={:08X} country={:08X} tag={} world_tag={} index={} country_index={} humans={} human_marker={:08X}",
        s.date_hours,
        (s.date_hours / 8760) as i32 - 5000,
        s.date_hours / 24 % 365 + 1,
        s.world,
        s.country,
        tag(&s.country_tag),
        tag(&s.world_tag),
        s.index,
        s.country_index,
        s.human_entries,
        s.human_marker
    );
    crate::log!(
        "upper_house object={:08X} vtable={:08X} array={:08X} vector_entries={} mapping=engine_names",
        s.upper_house,
        s.upper_house_vtable,
        s.array,
        s.array_entries
    );
    for (i, name) in IDEOLOGY_CANDIDATES.iter().enumerate() {
        let share = s.shares[i];
        if !(0..=32768).contains(&share) {
            crate::log!(
                "  index={} candidate={name} raw={share} hex={:016X} INVALID: expected share in [0,32768]",
                s.indices[i],
                share as u64
            );
            continue;
        }
        let millionths = share * 1000000 / 32768;
        let basis_points = share * 10000 / 32768;
        crate::log!(
            "  index={} candidate={name} raw={share} hex={:016X} fraction={}.{:06} percent={}.{:02}",
            s.indices[i],
            share as u64,
            millionths / 1000000,
            millionths % 1000000,
            basis_points / 100,
            basis_points % 100
        );
    }
    let sum = s.sum * 1000000 / 32768;
    crate::log!(
        "  sum_raw={} sum_fraction={}.{:06} expected_raw=32768 all_entries_in_range={}",
        s.sum,
        sum / 1000000,
        sum % 1000000,
        u32::from(s.shares_in_range)
    );
}
