//! TPL-0 army templates (mockup board 5-templates): recruit a composition in one go;
//! port of tgcx_templates.h, tgcx_army_filters.h, tgcx_templates_store.h and
//! tgcx_templates_metadata.h. Every regiment is the engine's own CConstructUnitCommand,
//! built as the build view builds it (FUN_00732520) and posted to the command queue
//! (R5); the engine re-checks each one when it executes (slot 14, FUN_005141D0).
//! Templates are an interface preference kept in a local file (D12). Offsets and ABIs:
//! docs/symbols.md "TPL-0: building a brigade".

use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND, FILETIME, GENERIC_WRITE, GetLastError,
    INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_NEW, CompareFileTime, CopyFileW, CreateFileW, DeleteFileW, FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_NORMAL, FindClose, FindFirstFileW, FindNextFileW, FlushFileBuffers,
    GetFileAttributesExW, GetFileAttributesW, GetFileExInfoStandard, INVALID_FILE_ATTRIBUTES,
    MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW, WIN32_FILE_ATTRIBUTE_DATA,
    WIN32_FIND_DATAW, WriteFile,
};
use windows_sys::Win32::UI::Shell::{SHCreateDirectoryExW, SHGetFolderPathW};

use crate::log;
use crate::packages::{MAX_PATH, Mod, ansi_to_wide, game_path, read_small};
use crate::snapshot::WORLD_RVA;
use crate::sys::{Global, Text, Wide, at, base, guard, rd8, rd32, rdi, rdp, read_game_string};
use crate::templates_catalog::{TPL_CAPACITY, TPL_CATALOG, TPL_KINDS};

/// Bounded engine unit type index (+0x24).
pub const UNITS: usize = 1024;
pub const POPS: usize = 1024;
const ORDERS: usize = 128;
pub const CAPACITY: usize = TPL_CAPACITY as usize;
const COMMAND_SIZE: usize = 0x68;
const COUNTRY_MANAGER_RVA: u32 = 0xE587E4;
const UNIT_DB_RVA: u32 = 0xE5FDD8;
const POP_TYPES_RVA: u32 = 0xF1BB34;
const COMMAND_VTABLE_RVA: u32 = 0xA00CF4;

// ---------------------------------------------------------------- the catalogue

/// One land unit type the panel offers: generated (TPL_CATALOG), read from the
/// package's tgcx_tpl_units.txt, or (Universal) from the running game.
#[derive(Clone, Copy)]
pub struct UnitInfo {
    pub key: Text<64>,
    pub label: Text<96>,
    pub abbr: Text<16>,
    pub days: i32,
}

pub struct Catalogue {
    ready: bool,
    pub units: Vec<UnitInfo>,
    /// Universal: each kind's native sprite frame.
    pub icons: Vec<u32>,
    pub universal_ready: bool,
}

static CATALOGUE: Global<Catalogue> = Global::new(Catalogue {
    ready: false,
    units: Vec::new(),
    icons: Vec::new(),
    universal_ready: false,
});

pub fn catalogue() -> &'static mut Catalogue {
    let c = CATALOGUE.get();
    if !c.ready {
        c.ready = true;
        c.units = TPL_CATALOG[..TPL_KINDS as usize]
            .iter()
            .map(|u| UnitInfo {
                key: Text::from_latin1(u.key),
                label: Text::from_latin1(u.label),
                abbr: Text::from_latin1(u.abbr),
                days: u.days,
            })
            .collect();
    }
    c
}

pub fn kinds() -> usize {
    catalogue().units.len()
}

// ---------------------------------------------------------------- planning (pure)

/// A soldier pop: free regiments it can still raise and the unit types it may build.
#[derive(Clone, Copy)]
pub struct TplPop {
    pub pop: usize,
    pub province_id: i32,
    pub free: i32,
    can_build: [u32; UNITS / 32],
    pub culture: usize,
    pub continent: usize,
}

impl TplPop {
    fn can(&self, unit: usize) -> bool {
        self.can_build[unit / 32] & (1 << (unit % 32)) != 0
    }
}

#[derive(Clone, Copy)]
pub struct TplOrder {
    pub unit: usize,
    pub pop: usize,
}

pub struct TplPlan {
    pub count: usize,
    pub missing: u32,
    pub orders: [TplOrder; ORDERS],
}

/// One order per regiment, from the pop with the most free regiments that may build
/// that unit (ties: the earlier pop, which is the native list order). Scarcer unit
/// types go first so a common type does not take the only pop a rare one could use.
/// What cannot be placed is counted in missing (D12).
pub fn make_plan(counts: &[i32; UNITS], pops: &mut [TplPop], out: &mut TplPlan) {
    out.count = 0;
    out.missing = 0;
    let mut eligible = [0u32; UNITS];
    let mut order = Vec::new();
    for u in 0..UNITS {
        if counts[u] > 0 {
            for p in pops.iter() {
                if p.free > 0 && p.can(u) {
                    eligible[u] += p.free as u32;
                }
            }
            order.push(u);
        }
    }
    order.sort_by_key(|&u| eligible[u]);
    for &u in &order {
        for _ in 0..counts[u] {
            let mut best: Option<usize> = None;
            for (p, pop) in pops.iter().enumerate() {
                if pop.free > 0 && pop.can(u) && best.is_none_or(|b| pop.free > pops[b].free) {
                    best = Some(p);
                }
            }
            match best {
                Some(b) if out.count < ORDERS => {
                    pops[b].free -= 1;
                    out.orders[out.count] = TplOrder { unit: u, pop: b };
                    out.count += 1;
                }
                _ => out.missing += 1,
            }
        }
    }
}

// ---------------------------------------------------------------- native entry points

struct Natives {
    can_build: usize,
    supports: usize,
    has: usize,
    new: usize,
    ctor: usize,
    installed: bool,
}

static NATIVES: Global<Natives> = Global::new(Natives {
    can_build: 0,
    supports: 0,
    has: 0,
    new: 0,
    ctor: 0,
    installed: false,
});

pub fn installed() -> bool {
    NATIVES.get().installed
}

/// FUN_005141D0: ECX unit, EAX pop, stack country then province, RET 8, AL.
fn can_build(unit: usize, soldiers: usize, country: usize, province: usize) -> bool {
    let result: u32;
    // SAFETY: the native check, as the C++ calls it.
    unsafe {
        std::arch::asm!(
            "push {province}",
            "push {country}",
            "call {f}",
            province = in(reg) province,
            country = in(reg) country,
            f = in(reg) NATIVES.get().can_build,
            in("ecx") unit,
            inlateout("eax") soldiers => result,
            clobber_abi("C"),
        );
    }
    result as u8 != 0
}

/// FUN_00965870(pop,0) __stdcall RET 8 minus FUN_00965930: EAX pop, one stack byte, RET 4.
fn free_regiments(soldiers: usize) -> i32 {
    let has: i32;
    // SAFETY: the native count, as the C++ calls it.
    unsafe {
        std::arch::asm!(
            "push 0",
            "call {f}",
            f = in(reg) NATIVES.get().has,
            inlateout("eax") soldiers => has,
            clobber_abi("C"),
        );
    }
    type Supports = unsafe extern "system" fn(usize, i32) -> i32;
    // SAFETY: FUN_00965870, __stdcall.
    let supports =
        unsafe { std::mem::transmute::<usize, Supports>(NATIVES.get().supports)(soldiers, 0) };
    supports - has
}

/// The game's operator new, cdecl.
pub fn game_new(size: usize) -> usize {
    type New = unsafe extern "C" fn(usize) -> usize;
    // SAFETY: operator new (RVA 0x6AE9AF), checked by the Templates gate.
    unsafe { std::mem::transmute::<usize, New>(NATIVES.get().new)(size) }
}

/// operator new(0x68) cdecl, then FUN_0057D1B0 __stdcall RET 0x18 (returns this).
fn make_command(unit: usize, province_id: i32, pop: usize, tag: u32, country: i32) -> usize {
    let memory = game_new(COMMAND_SIZE);
    if memory == 0 {
        return 0;
    }
    type Construct = unsafe extern "system" fn(usize, usize, i32, usize, u32, i32) -> usize;
    // SAFETY: the CConstructUnitCommand constructor.
    unsafe {
        std::mem::transmute::<usize, Construct>(NATIVES.get().ctor)(
            memory,
            unit,
            province_id,
            pop,
            tag,
            country,
        )
    }
}

/// (g_world+0xB24) slot 0x44 → +0x38 → slot 0x18(command): the reform/decision queue.
pub fn post(command: usize) -> bool {
    let world = rdp(at(WORLD_RVA));
    let front = if world != 0 { rdp(world + 0xB24) } else { 0 };
    if front == 0 {
        return false;
    }
    type Get = unsafe extern "fastcall" fn(usize, usize) -> usize;
    type Post = unsafe extern "fastcall" fn(usize, usize, usize);
    // SAFETY: the client's session getter and command queue, as the C++ calls them.
    let session =
        unsafe { std::mem::transmute::<usize, Get>(crate::sys::slot(front, 0x44 / 4))(front, 0) };
    let queue = if session != 0 { rdp(session + 0x38) } else { 0 };
    if queue == 0 {
        return false;
    }
    unsafe {
        std::mem::transmute::<usize, Post>(crate::sys::slot(queue, 0x18 / 4))(queue, 0, command)
    };
    true
}

// ---------------------------------------------------------------- reading the game (no writes)

#[derive(Clone, Copy)]
pub struct TplGlobals {
    pub world: usize,
    country_manager: usize,
    unit_db: usize,
    pop_types: usize,
    command_vtable: usize,
}

/// The country's unit types, by index, and its soldier pops.
pub struct TplCountry {
    pub country: usize,
    pub tag: u32,
    pub index: i32,
    pub unit: [usize; UNITS],
    pub name: [Text<64>; UNITS],
    pub icon: [i32; UNITS],
    pub days: [i32; UNITS],
    pub pop_count: usize,
    pub pops: [TplPop; POPS],
}

impl TplCountry {
    pub const ZERO: TplCountry = unsafe { std::mem::zeroed() };
}

pub fn live_globals() -> TplGlobals {
    TplGlobals {
        world: rdp(at(WORLD_RVA)),
        country_manager: rdp(at(COUNTRY_MANAGER_RVA)),
        unit_db: rdp(at(UNIT_DB_RVA)),
        pop_types: rdp(at(POP_TYPES_RVA)),
        command_vtable: at(COMMAND_VTABLE_RVA),
    }
}

/// The player's country, its land unit types, and every soldier pop in its provinces
/// with its free regiments: what FUN_004B81D0/FUN_007353D0 list.
fn scan_inner(g: &TplGlobals, out: &mut TplCountry) -> bool {
    out.pop_count = 0;
    out.unit = [0; UNITS];
    for name in out.name.iter_mut() {
        name.clear();
    }
    if g.world == 0 || g.country_manager == 0 || g.unit_db == 0 || g.pop_types == 0 {
        return false;
    }
    let index = rdi(g.world + 0xB60);
    let countries = rdp(g.country_manager + 4);
    if index < 0 || index >= rdi(g.country_manager) || countries == 0 {
        return false;
    }
    let country = rdp(countries + index as usize * 4);
    if country == 0 {
        return false;
    }
    out.country = country;
    out.tag = rd32(country + 0x1C);
    out.index = rdi(country + 0x20);
    let stats = rdp(country + 0xBCC);
    let types = if stats != 0 { rdp(stats + 0x310) } else { 0 };
    let (first, last) = (rdp(g.unit_db + 0xC), rdp(g.unit_db + 0x10));
    if types == 0 || last < first || (last - first) / 4 > UNITS {
        return false;
    }
    // Positions 0-2 are never buildable (FUN_004B81D0).
    let mut at_type = first + 3 * 4;
    while at_type < last {
        let base_type = rdp(at_type);
        at_type += 4;
        let i = if base_type != 0 {
            rdi(base_type + 0x24)
        } else {
            -1
        };
        if i < 3 {
            continue;
        }
        if i as usize >= UNITS {
            return false;
        }
        let i = i as usize;
        let unit = rdp(types + i * 4);
        if unit == 0 || rd8(unit + 0x39) == 0 {
            continue; // land only
        }
        out.unit[i] = unit;
        match read_game_string::<64>(base_type + 8) {
            Some(name) => out.name[i] = name,
            None => return false,
        }
        if army_universal() {
            out.icon[i] = rdi(base_type + 0x28);
            let days = rdi(unit + 0xC0);
            if days <= 0 || days > 100000000 {
                return false;
            }
            out.days[i] = (days + 999) / 1000;
        }
    }
    if rd8(country + 0xBB8) != 0 {
        return true; // the native list shows no rows then
    }
    let provinces = rdp(g.world + 0xACC);
    let (owned, owned_end) = (rdp(country + 0x9D8), rdp(country + 0x9DC));
    let (kinds, kinds_end) = (rdp(g.pop_types + 0xC), rdp(g.pop_types + 0x10));
    if provinces == 0 || owned_end < owned || kinds_end < kinds || (kinds_end - kinds) / 4 > 256 {
        return false;
    }
    let filters = filters_enabled();
    let mut p = owned;
    while p < owned_end {
        let province = rdp(provinces + rdi(p) as usize * 4);
        p += 4;
        let buckets = if province != 0 {
            rdp(province + 0x194)
        } else {
            0
        };
        if buckets == 0 {
            continue;
        }
        for k in 1..(kinds_end - kinds) / 4 {
            let kind = rdp(kinds + k * 4);
            if kind == 0 || rd8(kind + 0x42) == 0 {
                continue; // the soldier type
            }
            let mut pop = rdp(buckets + k * 0x10);
            let mut guard_count = 0;
            while pop != 0 && guard_count < 4096 {
                if out.pop_count >= POPS {
                    return true;
                }
                let mut entry = TplPop {
                    pop,
                    province_id: rdi(province + 0x58),
                    free: free_regiments(pop),
                    can_build: [0; UNITS / 32],
                    culture: 0,
                    continent: 0,
                };
                if entry.free > 0 {
                    if filters {
                        entry.culture = rdp(pop + 0x6C);
                        entry.continent = rdp(province + 0x16C);
                    }
                    for u in 0..UNITS {
                        if out.unit[u] != 0 && can_build(out.unit[u], pop, country, province) {
                            entry.can_build[u / 32] |= 1 << (u % 32);
                        }
                    }
                    if entry.can_build.iter().any(|&w| w != 0) {
                        out.pops[out.pop_count] = entry;
                        out.pop_count += 1;
                    }
                }
                pop = rdp(pop + 0x27C);
                guard_count += 1;
            }
        }
    }
    true
}

pub fn scan(g: &TplGlobals, out: &mut TplCountry) -> bool {
    guard(|| scan_inner(g, out)).unwrap_or(false)
}

pub fn unit_by_name(c: &TplCountry, name: &[u8]) -> Option<usize> {
    (0..UNITS).find(|&u| c.unit[u] != 0 && c.name[u].bytes() == name)
}

// ---------------------------------------------------------------- army filters (TPL-5/TPL-6)

// Read-only recruitment preferences. No simulation state writes. Field evidence and
// guards: docs/report-army-filters-research-1019.md.
pub const CULTURE_ALL: u32 = 0;
pub const CULTURE_ACCEPTED: u32 = 1;
pub const CULTURE_PRIMARY: u32 = 2;

#[derive(Clone, Copy, Default)]
pub struct ArmyFilter {
    pub culture: u32,
    pub continent: usize,
}

static FILTERS_ENABLED: Global<bool> = Global::new(false);

pub fn filters_enabled() -> bool {
    *FILTERS_ENABLED.get()
}

pub fn set_filters_enabled(enabled: bool) {
    *FILTERS_ENABLED.get() = enabled;
}

const FILTER_CULTURES_MAX: usize = 1024;
pub const CONTINENTS_MAX: usize = 128;

struct Cultures {
    primary: usize,
    accepted: Vec<usize>,
}

fn read_cultures_inner(country: usize) -> Option<Cultures> {
    if country == 0 {
        return None;
    }
    let primary = rdp(country + 0xE64);
    let count = rd32(country + 0xE5C) as usize;
    if primary == 0 || count > FILTER_CULTURES_MAX {
        return None;
    }
    let mut accepted = Vec::with_capacity(count);
    let (mut node, mut last) = (rdp(country + 0xE54), 0);
    for _ in 0..count {
        if node == 0 || rdp(node + 4) != last {
            return None;
        }
        let value = rdp(node);
        if value == 0 {
            return None;
        }
        accepted.push(value);
        last = node;
        node = rdp(node + 8);
    }
    if node != 0 || last != rdp(country + 0xE58) {
        return None;
    }
    accepted.sort_unstable();
    Some(Cultures { primary, accepted })
}

pub fn army_apply(country: usize, pops: &mut [TplPop], filter: &ArmyFilter) -> bool {
    if filter.culture > CULTURE_PRIMARY || pops.len() > POPS {
        return false;
    }
    if filter.culture == 0 && filter.continent == 0 {
        return true;
    }
    if !filters_enabled() {
        return false;
    }
    let cultures = if filter.culture != 0 {
        match guard(|| read_cultures_inner(country)).flatten() {
            Some(c) => Some(c),
            None => return false,
        }
    } else {
        None
    };
    for pop in pops.iter_mut() {
        if filter.culture != 0 && pop.culture == 0 {
            return false;
        }
        let matches = match &cultures {
            None => true,
            Some(c) => {
                pop.culture == c.primary
                    || (filter.culture == CULTURE_ACCEPTED
                        && c.accepted.binary_search(&pop.culture).is_ok())
            }
        };
        if !matches || (filter.continent != 0 && pop.continent != filter.continent) {
            pop.free = 0;
        }
    }
    true
}

#[derive(Clone, Copy)]
pub struct Continent {
    pub object: usize,
    pub key: Text<64>,
    pub label: Text<128>,
}

pub struct ContinentCatalogue {
    world: usize,
    manager: usize,
    begin: usize,
    end: usize,
    pub valid: bool,
    pub items: Vec<Continent>,
}

pub static CONTINENTS: Global<ContinentCatalogue> = Global::new(ContinentCatalogue {
    world: 0,
    manager: 0,
    begin: 0,
    end: 0,
    valid: false,
    items: Vec::new(),
});

fn continents_read_inner(
    world: usize,
    manager: usize,
    expected_vt: usize,
    out: &mut ContinentCatalogue,
) -> bool {
    if world == 0 || manager == 0 {
        out.valid = false;
        return false;
    }
    let (begin, end) = (rdp(manager + 0x2A78), rdp(manager + 0x2A7C));
    if begin == 0
        || end <= begin
        || !(end - begin).is_multiple_of(4)
        || (end - begin) / 4 > CONTINENTS_MAX
    {
        out.valid = false;
        return false;
    }
    if out.valid
        && out.world == world
        && out.manager == manager
        && out.begin == begin
        && out.end == end
    {
        return true;
    }
    out.valid = false;
    out.items.clear();
    let mut at_item = begin;
    while at_item < end {
        let object = rdp(at_item);
        at_item += 4;
        if object == 0 || rdp(object) != expected_vt {
            return false;
        }
        let Some(key) = read_game_string::<64>(object + 0x40).filter(|k| !k.is_empty()) else {
            return false;
        };
        let Some(mut label) = read_game_string::<128>(object + 0x5C) else {
            return false;
        };
        if label.is_empty() {
            label = Text::from_bytes(key.bytes());
        }
        if out.items.iter().any(|i| i.key.bytes() == key.bytes()) {
            return false;
        }
        out.items.push(Continent { object, key, label });
    }
    out.items.sort_by(|a, b| a.key.bytes().cmp(b.key.bytes()));
    out.world = world;
    out.manager = manager;
    out.begin = begin;
    out.end = end;
    out.valid = true;
    true
}

pub fn continents_live(world: usize) -> bool {
    if !filters_enabled() {
        return false;
    }
    let out = CONTINENTS.get();
    let read = guard(|| {
        continents_read_inner(
            world,
            rdp(at(0xE5870C)),
            base() + 0x9FC1DC,
            CONTINENTS.get(),
        )
    });
    match read {
        Some(ok) => ok,
        None => {
            out.valid = false;
            false
        }
    }
}

pub fn continent_by_key<'a>(
    catalogue: &'a ContinentCatalogue,
    key: &[u8],
) -> Option<&'a Continent> {
    if !catalogue.valid || key.is_empty() {
        return None;
    }
    catalogue.items.iter().find(|i| i.key.bytes() == key)
}

/// The builder offers only continents with a province the country owns (country
/// +0x9D8/+0x9DC province ids, g_world+0xACC province table). Only the selector's
/// choices narrow; the filter itself is unchanged.
fn continents_present_inner(
    world: usize,
    country: usize,
    catalogue: &ContinentCatalogue,
    present: &mut [bool],
) -> bool {
    present.fill(false);
    if world == 0 || country == 0 || !catalogue.valid {
        return false;
    }
    let provinces = rdp(world + 0xACC);
    let (owned, owned_end) = (rdp(country + 0x9D8), rdp(country + 0x9DC));
    if provinces == 0
        || owned_end < owned
        || !(owned_end - owned).is_multiple_of(4)
        || (owned_end - owned) / 4 > 65536
    {
        return false;
    }
    let mut p = owned;
    while p < owned_end {
        let id = rdi(p);
        p += 4;
        if id < 0 {
            return false;
        }
        let province = rdp(provinces + id as usize * 4);
        if province == 0 {
            continue;
        }
        let continent = rdp(province + 0x16C);
        for (i, item) in catalogue.items.iter().enumerate() {
            if item.object == continent {
                present[i] = true;
            }
        }
    }
    true
}

pub fn continents_present(
    world: usize,
    country: usize,
    present: &mut [bool; CONTINENTS_MAX],
) -> bool {
    let catalogue = CONTINENTS.get();
    match guard(|| continents_present_inner(world, country, catalogue, present)) {
        Some(ok) => ok,
        None => {
            present.fill(false);
            false
        }
    }
}

/// Next choice from current (0 = Any, i+1 = catalogue item i), skipping continents not
/// present. With no presence data every continent is offered.
pub fn continent_step(
    catalogue: &ContinentCatalogue,
    present: Option<&[bool]>,
    current: usize,
    forward: bool,
) -> usize {
    let n = catalogue.items.len() + 1;
    let mut next = current % n;
    for _ in 0..n {
        next = (next + if forward { 1 } else { n - 1 }) % n;
        if next == 0 || present.is_none_or(|p| p[next - 1]) {
            return next;
        }
    }
    0
}

// ---------------------------------------------------------------- recruiting

#[derive(Clone, Copy, Default)]
pub struct TplResult {
    pub scanned: bool,
    pub queued: u32,
    pub missing: u32,
    pub failed: u32,
}

/// Told about each posted order with the command's ids, read before posting: +0x48
/// construction (I), +0x50 the stack the regiment will appear in (TPL-1).
pub type Queued<'a> = &'a mut dyn FnMut(usize, i32, [u32; 2], [u32; 2]);
pub type Eligible<'a> = &'a mut dyn FnMut(i32) -> bool;

/// Scope one recruitment/preview without changing native pop capacity. Cache the
/// predicate per province within this snapshot, including negative results.
pub fn filter_pops(pops: &mut [TplPop], eligible: Option<Eligible>) -> u32 {
    let Some(eligible) = eligible else { return 0 };
    let mut seen: Vec<(i32, bool)> = Vec::new();
    let mut excluded = 0;
    for pop in pops.iter_mut() {
        let allowed = match seen.iter().find(|(p, _)| *p == pop.province_id) {
            Some(&(_, allowed)) => allowed,
            None => {
                let allowed = eligible(pop.province_id);
                seen.push((pop.province_id, allowed));
                allowed
            }
        };
        if !allowed {
            pop.free = 0;
            excluded += 1;
        }
    }
    excluded
}

fn send_order(g: &TplGlobals, c: &TplCountry, o: &TplOrder, queued: &mut Option<Queued>) -> bool {
    let pop = &c.pops[o.pop];
    guard(|| {
        let command = make_command(c.unit[o.unit], pop.province_id, pop.pop, c.tag, c.index);
        // Only a real CConstructUnitCommand goes to the queue; the queue owns it after posting.
        if command == 0 || rdp(command) != g.command_vtable {
            return false;
        }
        let construction = [rd32(command + 0x48), rd32(command + 0x4C)];
        let stack = [rd32(command + 0x50), rd32(command + 0x54)];
        if !post(command) {
            return false;
        }
        if let Some(queued) = queued {
            queued(o.unit, pop.province_id, construction, stack);
        }
        log!(
            "Template regiment {} province={} pop={:08X} command={command:08X} tag={}",
            c.name[o.unit],
            pop.province_id,
            pop.pop,
            crate::sys::Tag(c.tag, 4)
        );
        true
    })
    .unwrap_or(false)
}

static RECRUIT_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static RECRUIT_PLAN: Global<TplPlan> = Global::new(TplPlan {
    count: 0,
    missing: 0,
    orders: [TplOrder { unit: 0, pop: 0 }; ORDERS],
});

/// counts[unit index] regiments; recruit what fits and report the rest (D12).
pub fn recruit(
    counts: &[i32; UNITS],
    mut queued: Option<Queued>,
    eligible: Option<Eligible>,
    filter: Option<&ArmyFilter>,
) -> TplResult {
    let mut r = TplResult::default();
    if !installed() {
        return r;
    }
    let g = live_globals();
    let country = RECRUIT_COUNTRY.get();
    let plan = RECRUIT_PLAN.get();
    let mut wanted = [0i32; UNITS];
    let mut total = 0;
    for u in 0..UNITS {
        wanted[u] = counts[u].max(0);
        total += wanted[u] as u32;
    }
    r.scanned = scan(&g, country);
    if !r.scanned {
        r.missing = total;
        log!("Template recruit refused: game read failed");
        return r;
    }
    if let Some(filter) = filter
        && !army_apply(
            country.country,
            &mut country.pops[..country.pop_count],
            filter,
        )
    {
        r.scanned = false;
        r.missing = total;
        log!("Template recruit refused: culture/continent eligibility unavailable");
        return r;
    }
    for u in 0..UNITS {
        if wanted[u] != 0 && country.unit[u] == 0 {
            r.missing += wanted[u] as u32;
            wanted[u] = 0;
        }
    }
    let excluded = filter_pops(&mut country.pops[..country.pop_count], eligible);
    if excluded != 0 {
        log!(
            "Template recruitment scope: {excluded} soldier pops excluded (no land route to target)"
        );
    }
    make_plan(&wanted, &mut country.pops[..country.pop_count], plan);
    r.missing += plan.missing;
    for i in 0..plan.count {
        if send_order(&g, country, &plan.orders[i], &mut queued) {
            r.queued += 1;
        } else {
            r.failed += 1;
            log!(
                "Template regiment {} not queued (command refused)",
                country.name[plan.orders[i].unit]
            );
            break;
        }
    }
    if r.failed != 0 {
        r.missing += plan.count as u32 - r.queued;
    }
    log!(
        "Template recruit: {} queued, {} left out, {} soldier pops",
        r.queued,
        r.missing,
        country.pop_count
    );
    r
}

/// Every RVA the recruit reads or calls (check_exe.SIGNATURES, TPL-0).
pub const TPL_RVAS: [u32; 25] = [
    0x17D1B0, 0x17D1EF, 0x17D20E, 0x17D2C6, 0xA00CF4, 0xA00D14, 0x17DA45, 0x3325D0, 0x6AE9AF,
    0x33261B, 0x1141D0, 0x114284, 0x565870, 0x56589D, 0x565930, 0x5659E6, 0x11424B, 0x0B8285,
    0x0B82C3, 0x0B8313, 0x0B8343, 0x0B838B, 0x335416, 0x335773, 0x41EEB5,
];

pub fn install() {
    if !crate::sys::require_signatures(&TPL_RVAS) {
        log!("Templates rejected: guards; nothing is recruited");
        return;
    }
    let n = NATIVES.get();
    n.can_build = at(0x1141D0);
    n.supports = at(0x565870);
    n.has = at(0x565930);
    n.new = at(0x6AE9AF);
    n.ctor = at(0x17D1B0);
    n.installed = true;
    log!("Templates ready: CConstructUnitCommand via the engine queue");
}

// ---------------------------------------------------------------- local preferences (D12)

// Keys, never engine unit indices, are persisted. Bounded parsing and atomic
// replacement preserve a bad file.
pub const SLOTS: usize = 12;

#[derive(Clone, Copy)]
pub struct Composition {
    pub count: [i32; CAPACITY],
    pub culture: u32,
}

impl Composition {
    pub const EMPTY: Composition = Composition {
        count: [0; CAPACITY],
        culture: 0,
    };
}

#[derive(Clone, Copy)]
pub struct Library {
    pub size: usize,
    pub item: [Composition; SLOTS],
}

impl Library {
    pub const EMPTY: Library = Library {
        size: 0,
        item: [Composition::EMPTY; SLOTS],
    };
}

pub struct Store {
    pub library: Library,
    loaded: bool,
    pub storage_ok: bool,
    pub revision: u32,
    pub path: Vec<u16>,
    /// MOD-2 (S4): the settings folder of the ticked combination, set at initialisation.
    settings: Vec<u8>,
}

pub static STORE: Global<Store> = Global::new(Store {
    library: Library::EMPTY,
    loaded: false,
    storage_ok: false,
    revision: 0,
    path: Vec::new(),
    settings: Vec::new(),
});

pub fn set_settings_folder(settings: &[u8]) {
    STORE.get().settings = settings.to_vec();
}

pub fn library() -> &'static Library {
    &STORE.get().library
}

pub fn total(c: &Composition) -> u32 {
    c.count.iter().filter(|&&v| v > 0).map(|&v| v as u32).sum()
}

pub fn name(c: &Composition) -> Text<160> {
    let mut out = Text::<160>::new();
    for (i, unit) in catalogue().units.iter().enumerate() {
        if c.count[i] != 0 {
            let sep = if out.is_empty() { "" } else { " / " };
            let part = crate::text!(32, "{sep}{} {}", c.count[i], unit.abbr);
            out.push_bytes(part.bytes());
        }
    }
    if out.is_empty() {
        out = Text::from_latin1("Empty template");
    }
    out
}

pub fn base_days(c: &Composition) -> i32 {
    let mut days = 0;
    for (i, unit) in catalogue().units.iter().enumerate() {
        if c.count[i] != 0 {
            days = days.max(unit.days);
        }
    }
    days
}

/// The lines fgets(buffer of `size`) would return: at most size-1 bytes each, up to
/// and including a newline.
fn fgets_lines(data: &[u8], size: usize) -> impl Iterator<Item = &[u8]> {
    let mut at = 0;
    std::iter::from_fn(move || {
        if at >= data.len() {
            return None;
        }
        let rest = &data[at..];
        let mut n = rest
            .iter()
            .take(size - 1)
            .position(|&c| c == b'\n')
            .map_or(rest.len().min(size - 1), |i| i + 1);
        n = n.max(1);
        at += n;
        Some(&rest[..n])
    })
}

/// A C string view of an fgets line: up to its first NUL.
fn c_line(line: &[u8]) -> &[u8] {
    line.split(|&c| c == 0).next().unwrap_or_default()
}

fn trim_newlines(mut line: &[u8]) -> &[u8] {
    while let [rest @ .., b'\r' | b'\n'] = line {
        line = rest;
    }
    line
}

fn wide(path: &[u16]) -> String {
    let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
    String::from_utf16_lossy(&path[..end])
}

/// No defaults invented: start empty. Empty compositions may be saved while editing;
/// limits keep both the command count and the parser's arithmetic bounded.
pub fn load_file(path: &[u16]) -> Option<Library> {
    let data = std::fs::read(wide(path)).ok()?;
    let units = &catalogue().units;
    let mut candidate = Library::EMPTY;
    let (mut valid, mut inside, mut header, mut version2, mut culture_seen) =
        (true, false, false, false, false);
    for raw in fgets_lines(&data, 128) {
        let line = c_line(raw);
        if line.len() == 127 && line[126] != b'\n' {
            valid = false;
            break;
        }
        let line = trim_newlines(line);
        if !header {
            version2 = line == b"tgcx_templates 2";
            header = version2 || line == b"tgcx_templates 1";
            if !header {
                valid = false;
                break;
            }
            continue;
        }
        if line == b"template" {
            if inside || candidate.size >= SLOTS {
                valid = false;
                break;
            }
            inside = true;
            culture_seen = false;
            candidate.size += 1;
            continue;
        }
        if line == b"end" {
            if !inside || (version2 && !culture_seen) {
                valid = false;
                break;
            }
            inside = false;
            continue;
        }
        if line.starts_with(b"@culture=") {
            if !version2
                || !inside
                || culture_seen
                || line.len() != 10
                || !(b'0'..=b'2').contains(&line[9])
            {
                valid = false;
                break;
            }
            candidate.item[candidate.size - 1].culture = u32::from(line[9] - b'0');
            culture_seen = true;
            continue;
        }
        let Some(equal) = line.iter().position(|&c| c == b'=') else {
            valid = false;
            break;
        };
        if !inside || equal == 0 || equal >= 64 || equal + 1 >= line.len() {
            valid = false;
            break;
        }
        let key = &line[..equal];
        let mut value = 0u32;
        for &digit in &line[equal + 1..] {
            if !digit.is_ascii_digit() || value > 128 {
                valid = false;
                break;
            }
            value = value * 10 + u32::from(digit - b'0');
        }
        if !valid || value == 0 || value > 128 {
            valid = false;
            break;
        }
        let item = &mut candidate.item[candidate.size - 1];
        let Some(u) = units.iter().position(|unit| unit.key.bytes() == key) else {
            valid = false;
            break;
        };
        if item.count[u] != 0 {
            valid = false;
            break;
        }
        item.count[u] = value as i32;
        if total(item) > 128 {
            valid = false;
            break;
        }
    }
    (valid && header && !inside).then_some(candidate)
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}

fn save_file(path: &[u16], library: &Library) -> bool {
    if library.size > SLOTS {
        return false;
    }
    let temporary = to_wide(&format!("{}.pending", wide(path)));
    if temporary.len() > MAX_PATH {
        return false;
    }
    // A stale pending file belongs to an interrupted write; do not replace it.
    let handle = unsafe {
        CreateFileW(
            temporary.as_ptr(),
            GENERIC_WRITE,
            0,
            null(),
            CREATE_NEW,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return false;
    }
    let units = &catalogue().units;
    let mut bytes: Vec<u8> = b"tgcx_templates 2\r\n".to_vec();
    let mut valid = true;
    for item in &library.item[..library.size] {
        valid = total(item) <= 128 && item.culture <= CULTURE_PRIMARY;
        if !valid {
            break;
        }
        bytes.extend_from_slice(b"template\r\n");
        bytes.extend_from_slice(format!("@culture={}\r\n", item.culture).as_bytes());
        if item.count[units.len()..].iter().any(|&n| n != 0) {
            valid = false;
            break;
        }
        for (u, unit) in units.iter().enumerate() {
            let n = item.count[u];
            if !(0..=128).contains(&n) {
                valid = false;
                break;
            }
            if n != 0 {
                bytes.extend_from_slice(unit.key.bytes());
                bytes.extend_from_slice(format!("={n}\r\n").as_bytes());
            }
        }
        if !valid {
            break;
        }
        bytes.extend_from_slice(b"end\r\n");
    }
    valid = valid && bytes.len() <= 262144;
    let mut written = 0;
    let mut ok = valid
        && unsafe {
            WriteFile(
                handle,
                bytes.as_ptr(),
                bytes.len() as u32,
                &mut written,
                null_mut(),
            )
        } != 0
        && written as usize == bytes.len()
        && unsafe { FlushFileBuffers(handle) } != 0;
    unsafe { CloseHandle(handle) };
    if ok {
        let flags = MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH;
        ok = unsafe { MoveFileExW(temporary.as_ptr(), path.as_ptr(), flags) } != 0;
    }
    if !ok {
        unsafe { DeleteFileW(temporary.as_ptr()) };
    }
    ok
}

/// Some(exists) when the path is a file or plainly absent; None for anything else.
fn file_exists(path: &[u16]) -> Option<bool> {
    let value = unsafe { GetFileAttributesW(path.as_ptr()) };
    let error = unsafe { GetLastError() };
    let exists = value != INVALID_FILE_ATTRIBUTES;
    if exists {
        (value & FILE_ATTRIBUTE_DIRECTORY == 0).then_some(true)
    } else {
        (error == ERROR_FILE_NOT_FOUND || error == ERROR_PATH_NOT_FOUND).then_some(false)
    }
}

/// Prefer v2, import v1 only when v2 is absent. Invalid v2 never falls back.
fn load_preferred(preferred: &[u16], legacy: &[u16], imported: &mut bool) -> Option<Library> {
    *imported = false;
    if file_exists(preferred)? {
        return load_file(preferred);
    }
    if file_exists(legacy)? {
        *imported = true;
        return load_file(legacy);
    }
    Some(Library::EMPTY)
}

/// A combination without templates yet takes the newest file another combination
/// saved (e.g. TGC252_TGCX -> TGC252 when only Army is ticked). Nothing is deleted.
fn carry_over(root: &[u16], path: &[u16]) {
    if unsafe { GetFileAttributesW(path.as_ptr()) } != INVALID_FILE_ATTRIBUTES {
        return;
    }
    let pattern = to_wide(&format!("{}\\*", wide(root)));
    if pattern.len() > MAX_PATH {
        return;
    }
    // SAFETY: plain data structures the API fills.
    let mut entry: WIN32_FIND_DATAW = unsafe { std::mem::zeroed() };
    let find = unsafe { FindFirstFileW(pattern.as_ptr(), &mut entry) };
    if find == INVALID_HANDLE_VALUE {
        return;
    }
    let mut newest = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut best: Option<Vec<u16>> = None;
    loop {
        let name = wide(&entry.cFileName);
        if entry.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0 && !name.starts_with('.') {
            let candidate = to_wide(&format!("{}\\{name}\\tgcx_templates.txt", wide(root)));
            let mut data: WIN32_FILE_ATTRIBUTE_DATA = unsafe { std::mem::zeroed() };
            let same = wide(&candidate).to_lowercase() == wide(path).to_lowercase();
            if candidate.len() <= MAX_PATH
                && !same
                && unsafe {
                    GetFileAttributesExW(
                        candidate.as_ptr(),
                        GetFileExInfoStandard,
                        (&raw mut data).cast(),
                    )
                } != 0
                && load_file(&candidate).is_some()
                && unsafe { CompareFileTime(&data.ftLastWriteTime, &newest) } > 0
            {
                newest = data.ftLastWriteTime;
                best = Some(candidate);
            }
        }
        if unsafe { FindNextFileW(find, &mut entry) } == 0 {
            break;
        }
    }
    unsafe { FindClose(find) };
    if let Some(best) = best
        && unsafe { CopyFileW(best.as_ptr(), path.as_ptr(), 1) } != 0
    {
        log!("Templates carried over from {}", Wide(&best));
    }
}

pub fn load_library() {
    let s = STORE.get();
    if s.loaded {
        return;
    }
    s.loaded = true;
    let mut docs = [0u16; MAX_PATH];
    // CSIDL_PERSONAL, SHGFP_TYPE_CURRENT.
    if unsafe { SHGetFolderPathW(null_mut(), 5, null_mut(), 0, docs.as_mut_ptr()) } < 0 {
        return;
    }
    let Some(settings) = ansi_to_wide(&s.settings) else {
        return;
    };
    let settings = String::from_utf16_lossy(&settings);
    let root = format!("{}\\Paradox Interactive\\Victoria II", wide(&docs));
    let legacy = if settings.is_empty() {
        format!("{root}\\tgcx_templates.txt")
    } else {
        format!("{root}\\{settings}\\tgcx_templates.txt")
    };
    if legacy.encode_utf16().count() >= MAX_PATH {
        return;
    }
    s.path = to_wide(&legacy);
    let folder = to_wide(&legacy[..legacy.rfind('\\').unwrap()]);
    if unsafe { SHCreateDirectoryExW(null_mut(), folder.as_ptr(), null()) } != 0
        && unsafe { GetFileAttributesW(folder.as_ptr()) } == INVALID_FILE_ATTRIBUTES
    {
        return;
    }
    // New settings live separately: the accepted v1 file is never rewritten.
    let preferred = format!(
        "{}\\tgcx_templates_v2.txt",
        &legacy[..legacy.rfind('\\').unwrap()]
    );
    if preferred.encode_utf16().count() >= MAX_PATH {
        return;
    }
    let (legacy, preferred, root) = (to_wide(&legacy), to_wide(&preferred), to_wide(&root));
    s.path = preferred.clone();
    if file_exists(&preferred) == Some(false) {
        carry_over(&root, &legacy);
    }
    let mut imported = false;
    let loaded = load_preferred(&preferred, &legacy, &mut imported);
    let s = STORE.get();
    s.storage_ok = loaded.is_some();
    if let Some(library) = loaded {
        s.library = library;
    }
    if imported {
        log!("Templates v1 import: {}; original retained", Wide(&legacy));
    }
    log!(
        "Templates local file: {} loaded={} editable={}",
        Wide(&preferred),
        s.library.size,
        u32::from(s.storage_ok)
    );
}

pub fn commit(next: &Library) -> bool {
    let s = STORE.get();
    if !s.storage_ok || !save_file(&s.path, next) {
        return false;
    }
    s.library = *next;
    s.revision += 1;
    true
}

// ---------------------------------------------------------------- metadata (MOD-5)

// Generated per-base metadata is read at runtime by the same DLL. The engine's current
// country still supplies every recruitable unit pointer.
static UNIVERSAL: Global<bool> = Global::new(false);

pub fn army_universal() -> bool {
    *UNIVERSAL.get()
}

fn load_metadata(path: &[u16]) -> bool {
    let Ok(data) = std::fs::read(wide(path)) else {
        return false;
    };
    let mut lines = fgets_lines(&data, 256);
    let first = lines.next().map(c_line);
    let mut ok = first == Some(b"tgcx_units 1\n") || first == Some(b"tgcx_units 1\r\n");
    let mut candidate: Vec<UnitInfo> = Vec::new();
    while ok {
        let Some(raw) = lines.next() else { break };
        let line = c_line(raw);
        if line.is_empty()
            || (line.len() == 255 && line[254] != b'\n')
            || candidate.len() >= CAPACITY
        {
            ok = false;
            break;
        }
        let line = trim_newlines(line);
        let fields: Vec<&[u8]> = line.splitn(4, |&c| c == b'|').collect();
        if fields.len() != 4 {
            ok = false;
            break;
        }
        let (key, label, abbr, days_text) = (fields[0], fields[1], fields[2], fields[3]);
        if key.is_empty()
            || key.len() >= 64
            || label.is_empty()
            || label.len() >= 96
            || abbr.is_empty()
            || abbr.len() >= 16
        {
            ok = false;
            break;
        }
        if !key.iter().all(|&c| c.is_ascii_alphanumeric() || c == b'_') {
            ok = false;
        }
        if candidate.iter().any(|u| u.key.bytes() == key) {
            ok = false;
        }
        let mut days = 0u32;
        for &c in days_text {
            if !c.is_ascii_digit() || days > 100000 {
                ok = false;
                break;
            }
            days = days * 10 + u32::from(c - b'0');
        }
        if days == 0 || days > 100000 {
            ok = false;
        }
        if !ok {
            break;
        }
        candidate.push(UnitInfo {
            key: Text::from_bytes(key),
            label: Text::from_bytes(label),
            abbr: Text::from_bytes(abbr),
            days: days as i32,
        });
    }
    if !ok || candidate.is_empty() {
        return false; // Invalid metadata leaves the current catalogue intact.
    }
    catalogue().units = candidate;
    true
}

/// Called once at initialisation, before any GUI/library state exists. Two
/// simultaneous Army packages could disagree on sprites/keys, so refuse them.
pub fn package_metadata(game: &[u16], mods: &[Mod]) -> bool {
    let mut selected: Option<&Mod> = None;
    for m in mods.iter().filter(|m| m.package == b"army") {
        if selected.is_some() {
            log!("Army disabled: select only one base's Army package");
            return false;
        }
        selected = Some(m);
    }
    let Some(selected) = selected else {
        return false;
    };
    if let Some(path) = game_path(game, &selected.path, Some("tgcx_universal.txt"))
        && let Some(marker) = read_small(&path, 64)
        && c_line(&marker) == b"army_gui 1\n"
    {
        *UNIVERSAL.get() = true;
        log!("Universal Army: native shared GUI; catalogue will come from the running game");
        return true;
    }
    let Some(path) = game_path(game, &selected.path, Some("tgcx_tpl_units.txt")) else {
        return false;
    };
    if load_metadata(&path) {
        log!(
            "Army catalogue: {} land types loaded from {}",
            kinds(),
            Wide(&path)
        );
        return true;
    }
    // Retain the accepted 0.8.2 TGC package as a rollback. No silent TGC defaults on
    // a different base or when a present catalogue is malformed.
    if file_exists_absent(&path) && selected.path == b"mod/tgcx_army_tgc252" {
        log!(
            "Army catalogue: legacy TGC v2.5.2 package ({} land types)",
            kinds()
        );
        return true;
    }
    log!(
        "Army disabled: missing or invalid unit catalogue {}",
        Wide(&path)
    );
    false
}

fn file_exists_absent(path: &[u16]) -> bool {
    let attributes = unsafe { GetFileAttributesW(path.as_ptr()) };
    let error = unsafe { GetLastError() };
    attributes == INVALID_FILE_ATTRIBUTES
        && (error == ERROR_FILE_NOT_FOUND || error == ERROR_PATH_NOT_FOUND)
}

/// Unit indices and pointers come from the running game, never the metadata. Refuse a
/// package generated for another active unit set, including an extra unit-changing
/// submod, rather than displaying or recruiting the wrong list.
pub fn catalogue_fits(country: &TplCountry) -> bool {
    let units = &catalogue().units;
    if units
        .iter()
        .any(|k| unit_by_name(country, k.key.bytes()).is_none())
    {
        return false;
    }
    (0..UNITS).filter(|&u| country.unit[u] != 0).all(|u| {
        units
            .iter()
            .any(|k| k.key.bytes() == country.name[u].bytes())
    })
}

const PREFERRED: [&str; 11] = [
    "infantry",
    "guard",
    "irregular",
    "artillery",
    "engineer",
    "cuirassier",
    "dragoon",
    "hussar",
    "cavalry",
    "tank",
    "plane",
];
const ABBREV: [&str; 11] = [
    "Inf", "Gds", "Irr", "Art", "Eng", "Cui", "Dra", "Hus", "Cav", "Tnk", "Air",
];

pub fn universal_catalogue(country: &TplCountry) -> bool {
    if !army_universal() {
        return true;
    }
    if catalogue().universal_ready {
        return catalogue_fits(country);
    }
    let mut indices = Vec::new();
    for u in (0..UNITS).filter(|&u| country.unit[u] != 0) {
        if indices.len() == CAPACITY
            || country.name[u].is_empty()
            || !(1..=1024).contains(&country.icon[u])
            || !(1..=100000).contains(&country.days[u])
        {
            return false;
        }
        if indices
            .iter()
            .any(|&k: &usize| country.name[k].bytes() == country.name[u].bytes())
        {
            return false;
        }
        indices.push(u);
    }
    if indices.is_empty() {
        return false;
    }
    let rank = |key: &[u8]| {
        PREFERRED
            .iter()
            .position(|p| p.as_bytes() == key)
            .unwrap_or(PREFERRED.len())
    };
    indices.sort_by(|&a, &b| {
        let (ra, rb) = (rank(country.name[a].bytes()), rank(country.name[b].bytes()));
        ra.cmp(&rb)
            .then_with(|| country.name[a].bytes().cmp(country.name[b].bytes()))
    });
    let mut units = Vec::with_capacity(indices.len());
    let mut icons = Vec::with_capacity(indices.len());
    for &u in &indices {
        let key = country.name[u];
        let mut label = Vec::with_capacity(key.bytes().len());
        let mut capital = true;
        for &c in key.bytes() {
            if c == b'_' {
                label.push(b' ');
                capital = true;
            } else {
                label.push(if capital && c.is_ascii_lowercase() {
                    c - 32
                } else {
                    c
                });
                capital = false;
            }
        }
        let r = rank(key.bytes());
        let abbr = if r < ABBREV.len() {
            ABBREV[r].as_bytes().to_vec()
        } else {
            label[..label.len().min(3)].to_vec()
        };
        units.push(UnitInfo {
            key: Text::from_bytes(key.bytes()),
            label: Text::from_bytes(&label),
            abbr: Text::from_bytes(&abbr),
            days: country.days[u],
        });
        icons.push(country.icon[u] as u32);
    }
    let c = catalogue();
    let count = units.len();
    c.units = units;
    c.icons = icons;
    c.universal_ready = true;
    log!("Universal Army: {count} live land types; names/icons/build times read from engine");
    true
}
