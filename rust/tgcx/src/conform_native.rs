//! TPL-1 Conform, native layer; port of tgcx_conform_native.h: the engine commands
//! Conform posts, built exactly as their native callers build them; the read-only
//! registry walk that turns stored ids back into objects; the daily driver that steps
//! the pure tracker (conform.rs); and the CUnitView capture the Conform window
//! (conform_gui.rs) binds to. The DLL writes no unit membership, position or strength
//! (R5): every change is a command the engine validates when it executes. The only
//! direct write is the client's next-object-id counter, taken under the engine's own
//! lock exactly as the native new-stack button takes it. Evidence: docs/symbols.md
//! "TPL-1" and "TPL-1 native wrappers".

use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_FILE_NOT_FOUND, GENERIC_READ, GENERIC_WRITE, GetLastError,
    INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateFileW, DeleteFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ,
    FlushFileBuffers, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    OPEN_EXISTING, ReadFile, WriteFile,
};

use crate::conform::{
    self, Action, CfId, CfMark, CfOrder, CfPlan, CfRegiment, CfSeen, Kind, MoboCandidate,
    REGIMENTS, Refusal, Tracker,
};
use crate::conform_gui;
use crate::gather::{self, GtOrder, Site};
use crate::replace::{self, Choice, Health, Load, RpOrder, Why};
use crate::snapshot::{COUNTRY_MANAGER_RVA, WORLD_RVA};
use crate::sys::{
    Bytes, Global, Tag, Text, at, base, guard, in_image, keep_last_error, rd8, rd32, rdi, rdp,
    read_game_string, slot, thread_id,
};
use crate::templates::{self, ArmyFilter, TplCountry, TplPop, TplResult, UNITS};
use crate::{log, sys};

const LOW_RVA: u32 = 0xE58A98;
const HIGH_RVA: u32 = 0xE58A94;
const COUNTER_RVA: u32 = 0xE58A9C;
const ID_BASE_RVA: u32 = 0xE58A90;
const LOCK_RVA: u32 = 0xF1CA68;
const MOVE_VT_RVA: u32 = 0xA03394;
const MERGE_VT_RVA: u32 = 0xA03514;
const TRANSFER_VT_RVA: u32 = 0xA03554;
const CREATE_VT_RVA: u32 = 0xA00D34;
pub const VIEW_VT_RVA: u32 = 0xA16850;
// Allocation sizes as every native caller pushes them (0.6.1 used 0x48 for Merge; its
// ctor writes a byte at +0x48, a heap overflow that crashed, P45).
const MOVE_SIZE: usize = 0x90;
const MERGE_SIZE: usize = 0x4C;
const TRANSFER_SIZE: usize = 0x6C;
const CREATE_SIZE: usize = 0x60;

#[derive(Clone, Copy)]
pub struct CfGlobals {
    /// g_world and the two registry tables (pointers already read).
    pub world: usize,
    low: usize,
    high: usize,
    /// Addresses: ++dword, word, spinlock byte.
    counter: usize,
    id_base: usize,
    lock: usize,
    vt_move: usize,
    vt_merge: usize,
    vt_transfer: usize,
    vt_create: usize,
}

pub fn live_globals() -> CfGlobals {
    CfGlobals {
        world: rdp(at(WORLD_RVA)),
        low: rdp(at(LOW_RVA)),
        high: rdp(at(HIGH_RVA)),
        counter: at(COUNTER_RVA),
        id_base: at(ID_BASE_RVA),
        lock: at(LOCK_RVA),
        vt_move: at(MOVE_VT_RVA),
        vt_merge: at(MERGE_VT_RVA),
        vt_transfer: at(TRANSFER_VT_RVA),
        vt_create: at(CREATE_VT_RVA),
    }
}

// ---------------------------------------------------------------- registry and stacks (reads only)

/// ObjectRegistry_HashLookup (FUN_009C1D40), walked by hand: type > 0x1268 in the high
/// table; bucket (type<<16)+id mod +4 in +8; node +0 entry, +4 next; entry +8 type,
/// +0xC id. A stack object is entry-8; a regiment is the entry itself, as it appears
/// in a stack's regiment list.
pub fn lookup(g: &CfGlobals, id: CfId) -> usize {
    let table = if id.kind > 0x1268 { g.high } else { g.low };
    if table == 0 || id.none() {
        return 0;
    }
    let count = rd32(table + 4);
    let buckets = rdp(table + 8);
    if count == 0 || buckets == 0 {
        return 0;
    }
    let key = (id.kind << 16).wrapping_add(id.id);
    let mut node = rdp(buckets + (key % count) as usize * 4);
    let mut n = 0;
    while node != 0 && n < 4096 {
        let entry = rdp(node);
        if entry != 0 && rd32(entry + 8) == id.kind && rd32(entry + 0xC) == id.id {
            return entry;
        }
        node = rdp(node + 4);
        n += 1;
    }
    0
}

pub fn stack_id(stack: usize) -> CfId {
    CfId {
        kind: rd32(stack + 0x10),
        id: rd32(stack + 0x14),
    }
}

fn entry_id(entry: usize) -> CfId {
    CfId {
        kind: rd32(entry + 8),
        id: rd32(entry + 0xC),
    }
}

pub fn player(g: &CfGlobals) -> i32 {
    if g.world != 0 {
        rdi(g.world + 0xB60)
    } else {
        -1
    }
}

/// Where a stack is: +0xDC its province object, whose +0x58 is the province id (Move's
/// ctor stores it as the start, the AI merge compares it). +0xC0 is the owner's tag and
/// +0xC4 the owner's country index (U-9: "FRA", 80).
fn province(stack: usize) -> i32 {
    let province = rdp(stack + 0xDC);
    if province != 0 {
        rdi(province + 0x58)
    } else {
        -1
    }
}

/// A stack of the player's, of the same class as the reference (a land army), found
/// again by id. Anything else does not resolve.
pub fn stack_of(g: &CfGlobals, id: CfId, vtable: usize) -> usize {
    let entry = lookup(g, id);
    if entry == 0 {
        return 0;
    }
    let stack = entry - 8;
    if rdp(stack) != vtable || rdi(stack + 0xC4) != player(g) {
        return 0;
    }
    stack
}

/// Regiment list +0x38 (node +0 regiment, +8 next); a regiment's unit type is +0x38,
/// matched against the country's own unit types (TPL-0 scan) for the index.
pub struct CfStack {
    stack: usize,
    pub id: CfId,
    province: i32,
    country: i32,
    regiment: Vec<CfRegiment>,
    entry: Vec<usize>,
    kind: Vec<usize>,
}

impl CfStack {
    pub const EMPTY: CfStack = CfStack {
        stack: 0,
        id: CfId { kind: 0, id: 0 },
        province: 0,
        country: 0,
        regiment: Vec::new(),
        entry: Vec::new(),
        kind: Vec::new(),
    };
}

fn read_stack(stack: usize, units: Option<&[usize; UNITS]>, out: &mut CfStack) -> bool {
    out.stack = stack;
    out.regiment.clear();
    out.entry.clear();
    out.kind.clear();
    out.id = stack_id(stack);
    out.province = province(stack);
    out.country = rdi(stack + 0xC4);
    let mut node = rdp(stack + 0x38);
    let mut n = 0;
    while node != 0 && n < 1024 {
        let entry = rdp(node);
        if entry != 0 {
            if out.regiment.len() >= REGIMENTS {
                return false;
            }
            let kind = rdp(entry + 0x38);
            let unit = units
                .and_then(|u| (0..UNITS).find(|&i| u[i] != 0 && u[i] == kind))
                .map_or(-1, |i| i as i32);
            out.entry.push(entry);
            out.kind.push(kind);
            out.regiment.push(CfRegiment {
                id: entry_id(entry),
                unit,
            });
        }
        node = rdp(node + 8);
        n += 1;
    }
    n < 1024
}

fn stack_has(stack: usize, entry: usize) -> bool {
    let mut node = rdp(stack + 0x38);
    let mut n = 0;
    while node != 0 && n < 1024 {
        if rdp(node) == entry {
            return true;
        }
        node = rdp(node + 8);
        n += 1;
    }
    false
}

// ---------------------------------------------------------------- commands (native entry points)

struct Natives {
    move_fn: usize,
    merge_fn: usize,
    transfer_fn: usize,
    create_fn: usize,
    lock_fn: usize,
    path_fn: usize,
    path_clear_fn: usize,
}

static NATIVES: Global<Natives> = Global::new(Natives {
    move_fn: 0,
    merge_fn: 0,
    transfer_fn: 0,
    create_fn: 0,
    lock_fn: 0,
    path_fn: 0,
    path_clear_fn: 0,
});

/// Like the rally point (FUN_00491E80): Move(stack, province, replace 1, safe 1, very
/// safe 1, attach 0), RET 0x1C.
fn make_move(stack: usize, province: i32) -> usize {
    let memory = templates::game_new(MOVE_SIZE);
    if memory == 0 {
        return 0;
    }
    type Construct = unsafe extern "system" fn(usize, usize, i32, i32, i32, i32, i32) -> usize;
    // SAFETY: CMoveCommand's constructor, guarded by the Conform signatures.
    unsafe {
        std::mem::transmute::<usize, Construct>(NATIVES.get().move_fn)(
            memory, stack, province, 1, 1, 1, 0,
        )
    }
}

/// FUN_005E13B0(mem, list) RET 8: list {head, tail, count}, node {stack, prev, next,
/// byte}; the first stack stays. The ctor copies references, so the list can live here.
#[repr(C)]
struct Node {
    item: usize,
    prev: *const Node,
    next: *const Node,
    flag: u32,
}

#[repr(C)]
struct List {
    head: *const Node,
    tail: *const Node,
    count: u32,
}

fn make_merge(target: usize, other: usize) -> usize {
    let memory = templates::game_new(MERGE_SIZE);
    if memory == 0 {
        return 0;
    }
    let mut node = [
        Node {
            item: target,
            prev: null(),
            next: null(),
            flag: 0,
        },
        Node {
            item: other,
            prev: null(),
            next: null(),
            flag: 0,
        },
    ];
    let first: *const Node = &node[0];
    let second: *const Node = &node[1];
    node[0].next = second;
    node[1].prev = first;
    let list = List {
        head: &node[0],
        tail: &node[1],
        count: 2,
    };
    type Construct = unsafe extern "system" fn(usize, *const List) -> usize;
    // SAFETY: CMergeCommand's constructor with the list it copies.
    unsafe { std::mem::transmute::<usize, Construct>(NATIVES.get().merge_fn)(memory, &list) }
}

/// As the reorganize drop (FUN_00794350): FUN_005E1EE0(mem, target type, target id,
/// regiment) RET 0x10.
fn make_transfer(target: CfId, regiment: usize) -> usize {
    let memory = templates::game_new(TRANSFER_SIZE);
    if memory == 0 {
        return 0;
    }
    type Construct = unsafe extern "system" fn(usize, u32, u32, usize) -> usize;
    // SAFETY: CTransferSubUnitCommand's constructor.
    unsafe {
        std::mem::transmute::<usize, Construct>(NATIVES.get().transfer_fn)(
            memory,
            target.kind,
            target.id,
            regiment,
        )
    }
}

/// As the new-stack button (FUN_0079DB10): FUN_0057DB30(mem, tag +0xC0, country +0xC4,
/// id, id, province id +0xDC->+0x58, kind 0 (land), split 0) RET 0x28: an empty stack
/// beside the source.
fn make_create(source: usize, id: CfId) -> usize {
    let memory = templates::game_new(CREATE_SIZE);
    if memory == 0 {
        return 0;
    }
    type Construct =
        unsafe extern "system" fn(usize, u32, i32, u32, u32, u32, u32, i32, i32, i32) -> usize;
    // SAFETY: CCreateUnitCommand's constructor.
    unsafe {
        std::mem::transmute::<usize, Construct>(NATIVES.get().create_fn)(
            memory,
            rd32(source + 0xC0),
            rdi(source + 0xC4),
            id.kind,
            id.id,
            id.kind,
            id.id,
            province(source),
            0,
            0,
        )
    }
}

/// FUN_009C12D0: EAX the lock byte, spins until taken, returns the byte to store back.
fn lock_native(lock_byte: usize) -> u8 {
    let release: u32;
    // SAFETY: the engine's spinlock helper, as the C++ calls it.
    unsafe {
        std::arch::asm!(
            "call {f}",
            f = in(reg) NATIVES.get().lock_fn,
            inlateout("eax") lock_byte => release,
            clobber_abi("C"),
        );
    }
    release as u8
}

/// The client's next id, as the native new-stack button takes it: under the lock,
/// ++[counter], type = word [base] + 0x1268. Not simulation state; it names the stack
/// the Create command will make.
fn new_id(g: &CfGlobals) -> CfId {
    let release = lock_native(g.lock);
    // SAFETY: the engine's counter, written under its lock as the native button does.
    let id = unsafe {
        let counter = g.counter as *mut u32;
        let id = std::ptr::read_volatile(counter).wrapping_add(1);
        std::ptr::write_volatile(counter, id);
        id
    };
    let kind = u32::from(sys::rd::<u16>(g.id_base)) + 0x1268;
    // SAFETY: releasing the lock byte as the native caller does.
    unsafe { std::ptr::write_volatile(g.lock as *mut u8, release) };
    CfId { kind, id }
}

/// The stack's own class tests, as the new-stack button asks them (vfunc +0x3C gives
/// kind 1, +0x40 kind 2): Conform works on land stacks, kind 0, only.
pub fn land(stack: usize) -> bool {
    let (a, b) = (slot(stack, 0x3C / 4), slot(stack, 0x40 / 4));
    if !in_image(a) || !in_image(b) {
        return false;
    }
    type Test = unsafe extern "fastcall" fn(usize, usize) -> u8;
    // SAFETY: the stack class's own tests, inside the exe.
    unsafe {
        std::mem::transmute::<usize, Test>(a)(stack, 0) == 0
            && std::mem::transmute::<usize, Test>(b)(stack, 0) == 0
    }
}

/// FUN_005B8500: EAX the explicit start province, four stack arguments, RET 0x10. Use
/// the same VerySafe predicate as our Move, without issuing orders or changing the
/// army's province. Its output list belongs to the game heap.
#[repr(C)]
#[derive(Default)]
struct Path {
    head: usize,
    tail: usize,
    count: i32,
    flags: u32,
    extra: u32,
}

fn path_query(
    start: usize,
    stack: usize,
    destination: usize,
    finder: usize,
    path: &mut Path,
) -> bool {
    let result: u32;
    // SAFETY: the native route query, as the C++ calls it.
    unsafe {
        std::arch::asm!(
            "push {path}",
            "push {destination}",
            "push {stack}",
            "push {finder}",
            "call {f}",
            path = in(reg) &raw mut *path,
            destination = in(reg) destination,
            stack = in(reg) stack,
            finder = in(reg) finder,
            f = in(reg) NATIVES.get().path_fn,
            inlateout("eax") start => result,
            clobber_abi("C"),
        );
    }
    result as u8 != 0
}

pub fn reachable(stack: usize, province: i32) -> bool {
    route_len(stack, province).is_some()
}

/// How many provinces the stack's safe land route from `province` to where it stands
/// passes through (0 when it is already there); None when there is no such route.
pub fn route_len(stack: usize, province: i32) -> Option<u32> {
    route(stack, province, None)
}

/// TPL-11: the same safe land route from `province` to province `to`, asked as the
/// stack would march it (its owner's access); the stack itself stays where it is.
pub fn route_between(stack: usize, province: i32, to: i32) -> Option<u32> {
    if !(0..=65535).contains(&to) {
        return None;
    }
    route(stack, province, Some(to))
}

fn route(stack: usize, province: i32, to: Option<i32>) -> Option<u32> {
    let n = NATIVES.get();
    if n.path_fn == 0 || n.path_clear_fn == 0 || stack == 0 || !(0..=65535).contains(&province) {
        return None;
    }
    let mut path = Path::default();
    let result = guard(|| {
        let world = rdp(at(WORLD_RVA));
        let provinces = if world != 0 { rdp(world + 0xACC) } else { 0 };
        let start = if provinces != 0 {
            rdp(provinces + province as usize * 4)
        } else {
            0
        };
        let destination = match to {
            Some(to) if provinces != 0 => rdp(provinces + to as usize * 4),
            Some(_) => 0,
            None => rdp(stack + 0xDC),
        };
        let start_map = if start != 0 { rdp(start + 0x5C) } else { 0 };
        let end_map = if destination != 0 {
            rdp(destination + 0x5C)
        } else {
            0
        };
        // +0x2A is IS LAND (nonzero), not is sea. Native naval edge policy rejects
        // two nonzero endpoints and permits two zero endpoints; a mixed edge is
        // allowed only through the land province's port. Straits remain native land
        // edges; no embarked/sea node is admitted.
        if start_map == 0 || end_map == 0 || rd8(start_map + 0x2A) == 0 || rd8(end_map + 0x2A) == 0
        {
            return None;
        }
        if start == destination {
            return Some(0);
        }
        let finder = base() + 0xA03794;
        let mut result = path_query(
            start,
            stack,
            destination,
            (&raw const finder) as usize,
            &mut path,
        );
        let mut visited = 0;
        let mut node = path.head;
        while result && node != 0 {
            let id = rdi(node);
            visited += 1;
            if !(0..=65535).contains(&id) || visited > 65536 {
                result = false;
                break;
            }
            let p = rdp(provinces + id as usize * 4);
            let map = if p != 0 { rdp(p + 0x5C) } else { 0 };
            if map == 0 || rd8(map + 0x2A) == 0 {
                result = false;
            }
            node = rdp(node + 8);
        }
        result.then_some(visited)
    });
    let mut result = result.flatten();
    // Match Move's cleanup on both successful and failed queries. Never cross
    // allocators or retain native list nodes after this snapshot.
    let cleared = guard(|| {
        type Clear = unsafe extern "fastcall" fn(*mut Path, usize);
        // SAFETY: the native list cleanup (signature RVA 0x449E90).
        unsafe { std::mem::transmute::<usize, Clear>(NATIVES.get().path_clear_fn)(&mut path, 0) };
    });
    if cleared.is_none() {
        result = None;
        log!("Conform: route cleanup fault; recruitment refused");
    }
    result
}

fn recruit(
    target: usize,
    counts: &[i32; UNITS],
    queued: templates::Queued,
    culture: u32,
) -> TplResult {
    let filter = ArmyFilter {
        culture,
        continent: 0,
    };
    let mut eligible = |province: i32| reachable(target, province);
    templates::recruit(counts, Some(queued), Some(&mut eligible), Some(&filter))
}

pub fn scope_pops(stack: usize, pops: &mut [TplPop]) -> u32 {
    let mut eligible = |province: i32| reachable(stack, province);
    templates::filter_pops(pops, Some(&mut eligible))
}

/// Only a command with its own class's vtable is posted; the queue owns it then.
fn send(command: usize, vtable: usize) -> bool {
    if command == 0 || rdp(command) != vtable {
        return false;
    }
    templates::post(command)
}

// ---------------------------------------------------------------- the session (D13; TPL-7 saved orders)

pub struct Session {
    world: usize,
    date: u32,
    step: u32,
    player: i32,
    /// Consecutive frames a load has been seen (conform::confirm_load).
    load_frames: u32,
    /// One step per game day for every task. The class of land stacks, remembered from
    /// the stack the player conformed (or one resumed).
    stack_vtable: usize,
    // TPL-7 (D13 revised): regiments being built for a Conform are also kept in a
    // local file beside the templates, an interface preference outside the save.
    // Resuming only sends the same Move/Merge commands (R5).
    orders: Vec<CfOrder>,
    orders_loaded: bool,
    orders_ok: bool,
    orders_dirty: bool,
    orders_failed: bool,
    orders_path: Vec<u16>,
    // TPL-11: Build Army clicks still gathering, and their saved orders (own file).
    groups: Vec<Group>,
    gathers: Vec<GtOrder>,
    gathers_loaded: bool,
    gathers_ok: bool,
    gathers_dirty: bool,
    gathers_failed: bool,
    gathers_path: Vec<u16>,
}

/// One Build Army click: where its army gathers, and the army once a regiment appears.
#[derive(Clone, Copy)]
struct Group {
    id: u32,
    province: i32,
    army: CfId,
}

pub static TRACKER: Global<Tracker> = Global::new(Tracker {
    task: [conform::CfTask::EMPTY; conform::TASKS],
});
static SESSION: Global<Session> = Global::new(Session {
    world: 0,
    date: 0,
    step: 0,
    player: -1,
    load_frames: 0,
    stack_vtable: 0,
    orders: Vec::new(),
    orders_loaded: false,
    orders_ok: false,
    orders_dirty: false,
    orders_failed: false,
    orders_path: Vec::new(),
    groups: Vec::new(),
    gathers: Vec::new(),
    gathers_loaded: false,
    gathers_ok: false,
    gathers_dirty: false,
    gathers_failed: false,
    gathers_path: Vec::new(),
});

const ORDERS_TEXT: usize = 131072;
const MARKS_TEXT: usize = 65536;

/// A file in the settings folder beside the templates, as a NUL-terminated wide path.
fn settings_path(name: &str) -> Option<Vec<u16>> {
    templates::load_library(); // sets the settings folder the templates live in
    let tpl_path = templates::STORE.get().path.clone();
    let end = tpl_path
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(tpl_path.len());
    let leaf = tpl_path[..end]
        .iter()
        .rposition(|&c| c == u16::from(b'\\'))?;
    let mut path = tpl_path[..=leaf].to_vec();
    path.extend(name.encode_utf16());
    if path.len() >= 260 {
        return None;
    }
    path.push(0);
    Some(path)
}

enum Stored {
    Absent,
    Unreadable(u32),
    TooBig,
    Text(Vec<u8>),
}

fn read_settings(path: &[u16], size: usize) -> Stored {
    let file = unsafe {
        CreateFileW(
            path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if file == INVALID_HANDLE_VALUE {
        let error = unsafe { GetLastError() };
        return if error == ERROR_FILE_NOT_FOUND {
            Stored::Absent
        } else {
            Stored::Unreadable(error)
        };
    }
    let mut text = vec![0u8; size - 1];
    let mut read = 0;
    let whole = unsafe {
        ReadFile(
            file,
            text.as_mut_ptr(),
            text.len() as u32,
            &mut read,
            null_mut(),
        )
    } != 0
        && (read as usize) < size - 1;
    unsafe { CloseHandle(file) };
    if !whole {
        return Stored::TooBig;
    }
    text.truncate(read as usize);
    Stored::Text(text)
}

/// Written whole to a .pending file, then moved over the old one.
fn write_settings(path: &[u16], text: Option<String>) -> bool {
    let mut temporary: Vec<u16> = path[..path.len() - 1].to_vec();
    temporary.extend(".pending".encode_utf16());
    temporary.push(0);
    let mut ok = text.is_some() && temporary.len() <= 260;
    let file = if ok {
        unsafe {
            CreateFileW(
                temporary.as_ptr(),
                GENERIC_WRITE,
                0,
                null(),
                CREATE_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            )
        }
    } else {
        INVALID_HANDLE_VALUE
    };
    let bytes = text.unwrap_or_default();
    let mut written = 0;
    ok = file != INVALID_HANDLE_VALUE
        && unsafe {
            WriteFile(
                file,
                bytes.as_ptr(),
                bytes.len() as u32,
                &mut written,
                null_mut(),
            )
        } != 0
        && written as usize == bytes.len()
        && unsafe { FlushFileBuffers(file) } != 0;
    if file != INVALID_HANDLE_VALUE {
        unsafe { CloseHandle(file) };
    }
    let flags = MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH;
    ok = ok && unsafe { MoveFileExW(temporary.as_ptr(), path.as_ptr(), flags) } != 0;
    if !ok && file != INVALID_HANDLE_VALUE {
        unsafe { DeleteFileW(temporary.as_ptr()) };
    }
    ok
}

fn orders_load() {
    let s = SESSION.get();
    if s.orders_loaded {
        return;
    }
    s.orders_loaded = true;
    s.orders.clear();
    s.orders_ok = false;
    let Some(path) = settings_path("tgcx_conform_orders.txt") else {
        return;
    };
    s.orders_path = path.clone();
    match read_settings(&path, ORDERS_TEXT) {
        Stored::Absent => s.orders_ok = true,
        Stored::Unreadable(error) => {
            log!("Conform orders file unreadable ({error}); orders are not resumed after a load")
        }
        Stored::TooBig => {
            log!("Conform orders file invalid: kept as is, not resumed or overwritten")
        }
        Stored::Text(text) => match conform::orders_parse(&text) {
            Some(orders) => {
                s.orders = orders;
                s.orders_ok = true;
                log!("Conform orders file: {} saved orders", s.orders.len());
            }
            None => log!("Conform orders file invalid: kept as is, not resumed or overwritten"),
        },
    }
}

fn orders_save() {
    let s = SESSION.get();
    if !s.orders_dirty || !s.orders_ok || s.orders_path.is_empty() {
        return;
    }
    if !write_settings(
        &s.orders_path,
        conform::orders_write(&s.orders, ORDERS_TEXT),
    ) {
        if !s.orders_failed {
            log!(
                "Conform orders file not saved; this session's orders will not resume after a load"
            );
        }
        s.orders_failed = true;
        return;
    }
    s.orders_dirty = false;
}

/// A dropped (oldest) record moves the later ones down: keep the tasks' links.
fn order_new(task: usize, r: CfOrder) {
    orders_load();
    let s = SESSION.get();
    if !s.orders_ok {
        return;
    }
    let (index, dropped) = conform::order_add(&mut s.orders, r);
    if let Some(dropped) = dropped {
        for t in TRACKER
            .get()
            .task
            .iter_mut()
            .filter(|t| t.used && t.group == 0 && t.record != 0)
        {
            if t.record == dropped + 1 {
                t.record = 0;
            } else if t.record > dropped + 1 {
                t.record -= 1;
            }
        }
    }
    TRACKER.get().task[task].record = index + 1;
    s.orders[index].active = true;
    s.orders_dirty = true;
}

fn forget(why: &str) {
    let mut n = 0;
    for t in TRACKER.get().task.iter_mut().filter(|t| t.used) {
        t.used = false;
        n += 1;
    }
    let s = SESSION.get();
    for r in s.orders.iter_mut() {
        r.active = false;
    }
    for r in s.gathers.iter_mut() {
        r.active = false;
    }
    s.groups.clear();
    let r = REPLACING.get();
    r.retire.clear();
    for o in r.orders.iter_mut() {
        o.active = false;
    }
    if n != 0 {
        log!("Conform: {n} pending orders forgotten ({why})");
    }
}

/// After a load: every saved order of the player's tag whose date window holds the
/// loaded date, and whose target is again the player's land stack, is tracked again. A
/// regiment still being built waits for its construction; one already built (perhaps
/// sent to a rally point) marches and merges as usual; one gone is dropped after two
/// days, as in the session.
fn resume(g: &CfGlobals, date: u32) {
    orders_load();
    let s = SESSION.get();
    if !s.orders_ok || s.orders.is_empty() {
        return;
    }
    let (mut resumed, mut building, mut refused) = (0, 0, 0);
    let player = player(g);
    for i in 0..s.orders.len() {
        let r = s.orders[i];
        s.orders[i].active = false;
        if r.created > date || (r.done != 0 && date >= r.done) {
            continue;
        }
        let entry = lookup(g, r.target);
        let target = if entry != 0 { entry - 8 } else { 0 };
        if target == 0
            || rdi(target + 0xC4) != player
            || !conform::order_applies(&r, rd32(target + 0xC0), date)
        {
            continue;
        }
        let vt = rdp(target);
        let accepted = if s.stack_vtable != 0 {
            vt == s.stack_vtable
        } else {
            land(target)
        };
        if !accepted {
            refused += 1;
            continue;
        }
        s.stack_vtable = vt;
        let Some(t) = conform::add(
            TRACKER.get(),
            Kind::Join,
            r.target,
            r.stack,
            r.construction,
            r.unit,
            r.created,
        ) else {
            refused += 1;
            continue;
        };
        let alive = lookup(g, r.construction) != 0;
        let joining = stack_of(g, r.stack, vt);
        let task = &mut TRACKER.get().task[t];
        if joining != 0 {
            task.regiments = rdi(joining + 0x40).max(0) as usize; // a mobo join may hold several
        }
        task.record = i + 1;
        task.construction_seen = true;
        s.orders[i].active = true;
        resumed += 1;
        building += u32::from(alive);
    }
    if resumed != 0 || refused != 0 {
        log!(
            "Conform: {resumed} saved orders resumed after a load ({building} still being built), {refused} refused"
        );
    }
}

/// Records whose task ended are finished from today: a save made from now on does not
/// resume them.
fn orders_close(g: &CfGlobals, date: u32) {
    let s = SESSION.get();
    for i in 0..s.orders.len() {
        if !s.orders[i].active {
            continue;
        }
        let live = TRACKER
            .get()
            .task
            .iter()
            .any(|t| t.used && t.group == 0 && t.record == i + 1);
        if !live {
            s.orders[i].active = false;
            s.orders[i].done = date;
            s.orders_dirty = true;
        }
    }
    orders_save();
    gathers_close(date);
    replace_close(g, date);
}

/// A changed target location/access can strand an earlier reinforcement. Release only
/// our tracking; keep the unit/construction and all user orders.
fn prune_routes(g: &CfGlobals, target_id: CfId, target: usize) {
    let vtable = SESSION.get().stack_vtable;
    for t in TRACKER.get().task.iter_mut() {
        if !t.used || t.kind != Kind::Join || t.target != target_id {
            continue;
        }
        let stack = stack_of(g, t.stack, vtable);
        let p = if stack != 0 {
            province(stack)
        } else {
            t.origin
        };
        if p >= 0 && !reachable(target, p) {
            t.used = false;
            log!(
                "Conform: reinforcement {} cannot reach target {} by land; left in place, no longer counted as pending",
                t.stack.id,
                target_id.id
            );
        }
    }
}

// ---------------------------------------------------------------- mobo stacks (MOBO-1)

// A mobo stack is a stack marked (conform_gui) to be completed with infantry when its
// country mobilizes. Mobilization raises its regiments from a schedule, a few per day,
// each into the land stack of its province if there is one and otherwise into a new
// stack that the engine sends to the country's rally point (FUN_00446A30). So the
// infantry that can be brought to a mobo stack are the new all-infantry stacks that
// appear while the schedule is being worked off: each is told to march to the nearest
// mobo stack still short of infantry and join it, by the same Move and Merge tracking
// as a recruit. CCountry fields (docs/symbols.md "MOBO-1"): +0x120 the mobilized flag
// (written by CToggleMobilizationCommand, read by is_mobilised); +0x15DC/+0x15E0 the
// schedule, a vector of pointers cleared once every entry has raised its regiments;
// +0x7B4 the unit list (head; node +0 unit, +8 next).
const MOBILISED: usize = 0x120;
const SCHEDULE: usize = 0x15DC;
const UNIT_LIST: usize = 0x7B4;
const MOBO_RVAS: [u32; 6] = [0x1804BE, 0x50003E, 0x108FB0, 0x113CA8, 0x113CCF, 0x10834B];

struct Mobo {
    enabled: bool,
    marks: Vec<CfMark>,
    loaded: bool,
    ok: bool,
    dirty: bool,
    failed: bool,
    path: Vec<u16>,
    /// The player's land stacks seen the last game day; only meaningful once seeded.
    known: Vec<CfId>,
    seeded: bool,
    /// The schedule still had regiments to raise the last game day.
    raising: bool,
}

static MOBO: Global<Mobo> = Global::new(Mobo {
    enabled: false,
    marks: Vec::new(),
    loaded: false,
    ok: false,
    dirty: false,
    failed: false,
    path: Vec::new(),
    known: Vec::new(),
    seeded: false,
    raising: false,
});
static MOBO_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static MOBO_STACK: Global<CfStack> = Global::new(CfStack::EMPTY);

pub fn mobo_enabled() -> bool {
    MOBO.get().enabled
}

fn marks_load() {
    let m = MOBO.get();
    if m.loaded {
        return;
    }
    m.loaded = true;
    m.marks.clear();
    m.ok = false;
    let Some(path) = settings_path("tgcx_mobo_marks.txt") else {
        return;
    };
    m.path = path.clone();
    match read_settings(&path, MARKS_TEXT) {
        Stored::Absent => m.ok = true,
        Stored::Unreadable(error) => {
            log!("Mobo marks file unreadable ({error}); marks are not kept")
        }
        Stored::TooBig => log!("Mobo marks file invalid: kept as is, no mobo stacks this session"),
        Stored::Text(text) => match conform::marks_parse(&text) {
            Some(marks) => {
                m.marks = marks;
                m.ok = true;
                log!("Mobo marks file: {} marks", m.marks.len());
            }
            None => log!("Mobo marks file invalid: kept as is, no mobo stacks this session"),
        },
    }
}

fn marks_save() {
    let m = MOBO.get();
    if !m.dirty || !m.ok || m.path.is_empty() {
        return;
    }
    if !write_settings(&m.path, conform::marks_write(&m.marks, MARKS_TEXT)) {
        if !m.failed {
            log!("Mobo marks file not saved; this session's marks will not survive a load");
        }
        m.failed = true;
        return;
    }
    m.dirty = false;
}

/// The infantry a marked stack is to be completed with (None: not marked).
pub fn mobo_quota(id: CfId) -> Option<i32> {
    MOBO.get()
        .marks
        .iter()
        .find(|r| r.active && r.done == 0 && r.stack == id)
        .map(|r| r.infantry)
}

/// Marks the stack, or changes how much infantry it is marked for. False when marks
/// cannot be kept (guards rejected, or the marks file is unusable).
pub fn mobo_mark(g: &CfGlobals, stack: usize, infantry: i32) -> bool {
    let m = MOBO.get();
    if !m.enabled || infantry <= 0 {
        return false;
    }
    marks_load();
    if !m.ok {
        return false;
    }
    let id = stack_id(stack);
    if let Some(r) = m
        .marks
        .iter_mut()
        .find(|r| r.active && r.done == 0 && r.stack == id)
    {
        r.infantry = infantry;
    } else {
        let mark = CfMark {
            tag: rd32(stack + 0xC0),
            created: rd32(g.world + 0xB0C),
            done: 0,
            stack: id,
            infantry,
            active: false,
        };
        SESSION.get().stack_vtable = rdp(stack);
        let i = conform::mark_add(&mut m.marks, mark);
        m.marks[i].active = true;
        m.seeded = false;
        log!(
            "Mobo: stack {}/{} marked for {infantry} infantry",
            id.kind,
            id.id
        );
    }
    m.dirty = true;
    marks_save();
    true
}

pub fn mobo_unmark(g: &CfGlobals, id: CfId) {
    let m = MOBO.get();
    let date = rd32(g.world + 0xB0C);
    for r in m
        .marks
        .iter_mut()
        .filter(|r| r.active && r.done == 0 && r.stack == id)
    {
        r.active = false;
        r.done = date;
        m.dirty = true;
        log!("Mobo: stack {}/{} no longer marked", id.kind, id.id);
    }
    marks_save();
}

/// A load: nothing is tracked until the saved marks have been looked at again.
fn mobo_reset() {
    let m = MOBO.get();
    m.known.clear();
    m.seeded = false;
    m.raising = false;
    for r in m.marks.iter_mut() {
        r.active = false;
    }
}

/// After a load: every mark of the player's tag whose date window holds the loaded
/// date, and whose stack is again the player's land stack, is tracked again.
fn mobo_resume(g: &CfGlobals, date: u32) {
    let m = MOBO.get();
    if !m.enabled {
        return;
    }
    marks_load();
    if !m.ok || m.marks.is_empty() {
        return;
    }
    let player = player(g);
    let (mut resumed, mut refused) = (0, 0);
    for i in 0..m.marks.len() {
        let r = m.marks[i];
        if r.created > date || (r.done != 0 && date >= r.done) {
            continue;
        }
        let entry = lookup(g, r.stack);
        let stack = if entry != 0 { entry - 8 } else { 0 };
        if stack == 0
            || rdi(stack + 0xC4) != player
            || !conform::mark_applies(&r, rd32(stack + 0xC0), date)
        {
            continue;
        }
        let s = SESSION.get();
        let vt = rdp(stack);
        let accepted = if s.stack_vtable != 0 {
            vt == s.stack_vtable
        } else {
            land(stack)
        };
        if !accepted {
            refused += 1;
            continue;
        }
        s.stack_vtable = vt;
        m.marks[i].active = true;
        resumed += 1;
    }
    if resumed != 0 || refused != 0 {
        log!("Mobo: {resumed} marks resumed after a load, {refused} refused");
    }
}

fn player_country(g: &CfGlobals) -> usize {
    let manager = rdp(at(COUNTRY_MANAGER_RVA));
    let index = player(g);
    if manager == 0 || index < 0 || index >= rdi(manager) {
        return 0;
    }
    let countries = rdp(manager + 4);
    if countries != 0 {
        rdp(countries + index as usize * 4)
    } else {
        0
    }
}

/// Regiments the country's mobilization schedule still has to raise.
fn schedule_len(country: usize) -> u32 {
    let (begin, end) = (rdp(country + SCHEDULE), rdp(country + SCHEDULE + 4));
    if end > begin {
        ((end - begin) / 4) as u32
    } else {
        0
    }
}

/// The player's land stacks from the country's unit list, each checked against the
/// registry so a stale entry is never used.
fn land_stacks(g: &CfGlobals, country: usize, vtable: usize) -> Vec<(usize, CfId)> {
    let player = player(g);
    let mut out = Vec::new();
    let mut node = rdp(country + UNIT_LIST);
    let mut n = 0;
    while node != 0 && n < 20000 {
        let unit = rdp(node);
        node = rdp(node + 8);
        n += 1;
        if unit == 0 || rdp(unit) != vtable || rdi(unit + 0xC4) != player {
            continue;
        }
        let id = stack_id(unit);
        if lookup(g, id) == unit + 8 {
            out.push((unit, id));
        }
    }
    out
}

/// A new all-infantry stack that appeared while the country mobilized.
struct Raised {
    id: CfId,
    province: i32,
    regiments: i32,
}

/// A marked stack and how much infantry it still lacks.
struct Marked {
    stack: usize,
    id: CfId,
    need: i32,
}

/// One game day of mobo stacks. Marked stacks that are gone are ended. While the country
/// is mobilized and its schedule is being worked off (and the day after it empties, for
/// the last regiments), each new all-infantry stack is sent to the nearest marked stack
/// that still needs infantry, if it fits all of them.
fn mobo_daily(g: &CfGlobals, date: u32) {
    let m = MOBO.get();
    if !m.enabled || !m.marks.iter().any(|r| r.active) {
        m.seeded = false;
        m.known.clear();
        m.raising = false;
        return;
    }
    let country = player_country(g);
    let vtable = SESSION.get().stack_vtable;
    if country == 0 || vtable == 0 {
        return;
    }
    for r in m.marks.iter_mut().filter(|r| r.active) {
        if stack_of(g, r.stack, vtable) == 0 {
            r.active = false;
            r.done = date;
            m.dirty = true;
            log!(
                "Mobo: marked stack {}/{} is gone; mark ended",
                r.stack.kind,
                r.stack.id
            );
        }
    }
    marks_save();
    let stacks = land_stacks(g, country, vtable);
    let mobilized = rd8(country + MOBILISED) != 0;
    let scheduled = schedule_len(country) != 0;
    let window = mobilized && (scheduled || m.raising);
    m.raising = mobilized && scheduled;
    let was_seeded = m.seeded;
    let known = std::mem::take(&mut m.known);
    m.known = stacks.iter().map(|&(_, id)| id).collect();
    m.seeded = true;
    if !window || !was_seeded {
        return;
    }
    let tracker = TRACKER.get();
    let fresh: Vec<(usize, CfId)> = stacks
        .iter()
        .copied()
        .filter(|(_, id)| {
            !known.contains(id)
                && !tracker
                    .task
                    .iter()
                    .any(|t| t.used && (t.stack == *id || t.target == *id))
                && !m.marks.iter().any(|r| r.active && r.stack == *id)
        })
        .collect();
    if fresh.is_empty() {
        return;
    }
    let scanned = MOBO_COUNTRY.get();
    if !templates::scan(&templates::live_globals(), scanned) {
        log!("Mobo: cannot read the player's unit types");
        return;
    }
    let Some(infantry) = templates::unit_by_name(scanned, b"infantry") else {
        log!("Mobo: this country has no infantry unit type");
        return;
    };
    let read = MOBO_STACK.get();
    let mut raised = Vec::new();
    for &(stack, id) in &fresh {
        if read_stack(stack, Some(&scanned.unit), read)
            && !read.regiment.is_empty()
            && read.regiment.iter().all(|r| r.unit == infantry as i32)
        {
            raised.push(Raised {
                id,
                province: read.province,
                regiments: read.regiment.len() as i32,
            });
        }
    }
    if raised.is_empty() {
        return;
    }
    let mut marked = Vec::new();
    for r in m.marks.iter().filter(|r| r.active && r.done == 0) {
        let stack = stack_of(g, r.stack, vtable);
        if stack == 0 || !read_stack(stack, Some(&scanned.unit), read) {
            continue;
        }
        let have = read
            .regiment
            .iter()
            .filter(|c| c.unit == infantry as i32)
            .count() as i32;
        let coming = conform::pending(TRACKER.get(), r.stack)[infantry];
        marked.push(Marked {
            stack,
            id: r.stack,
            need: r.infantry - have - coming,
        });
    }
    if marked.iter().all(|t| t.need <= 0) {
        return;
    }
    let candidates: Vec<MoboCandidate> = raised
        .iter()
        .map(|c| MoboCandidate {
            regiments: c.regiments,
            route: marked
                .iter()
                .map(|t| route_len(t.stack, c.province))
                .collect(),
        })
        .collect();
    let need: Vec<i32> = marked.iter().map(|t| t.need).collect();
    let tag = rd32(marked[0].stack + 0xC0);
    for (c, t) in conform::mobo_assign(&need, &candidates) {
        let (from, to) = (&raised[c], &marked[t]);
        let Some(task) = conform::add(
            TRACKER.get(),
            Kind::Join,
            to.id,
            from.id,
            CfId::default(),
            infantry as i32,
            date,
        ) else {
            log!(
                "Mobo: tracker full; infantry stack {} will not march by itself",
                from.id.id
            );
            continue;
        };
        let joining = &mut TRACKER.get().task[task];
        joining.regiments = from.regiments as usize;
        joining.origin = from.province;
        order_new(
            task,
            CfOrder {
                tag,
                created: date,
                done: 0,
                target: to.id,
                stack: from.id,
                construction: CfId::default(),
                unit: infantry as i32,
                active: false,
            },
        );
        log!(
            "Mobo: infantry stack {} ({} regiments) goes to mobo stack {}",
            from.id.id,
            from.regiments,
            to.id.id
        );
    }
    orders_save();
}

fn daily(g: &CfGlobals) {
    if g.world == 0 {
        return;
    }
    let date = rd32(g.world + 0xB0C);
    if date < conform::DATE_MIN {
        return;
    }
    let player = player(g);
    let s = SESSION.get();
    // A load: another world or country, the date going back, or a jump no game speed
    // makes between two frames (45 times the smallest day step), seen on several
    // frames in a row; until then nothing steps.
    let jump = conform::date_jump(s.date, date, s.step);
    let load = g.world != s.world || player != s.player || date < s.date || jump;
    let confirmed = conform::confirm_load(&mut s.load_frames, load);
    if load && !confirmed {
        return;
    }
    if confirmed {
        if s.world != 0 {
            log!(
                "Conform: load seen: world {:08X}->{:08X} player {}->{} date {}->{} step {}",
                s.world,
                g.world,
                s.player,
                player,
                s.date,
                date,
                s.step
            );
        }
        forget("another game or a load");
        let s = SESSION.get();
        s.world = g.world;
        s.player = player;
        s.step = 0;
        s.date = date;
        mobo_reset();
        resume(g, date);
        replace_resume(g, date);
        gathers_resume(g, date);
        mobo_resume(g, date);
        return;
    }
    if date == s.date {
        return;
    }
    if s.step == 0 || date - s.date < s.step {
        s.step = date - s.date;
    }
    s.date = date;
    // TPL-11: regiments resumed for a Build Army click need the class of land stacks,
    // known only from one of the player's armies when no Conform ran this session.
    // TPL-12: so do old regiments sent out again after a load.
    if s.stack_vtable == 0
        && (TRACKER.get().task.iter().any(|t| t.used && t.group != 0)
            || !REPLACING.get().retire.is_empty())
    {
        s.stack_vtable = player_armies(g).first().map_or(0, |&a| rdp(a));
    }
    let vtable = s.stack_vtable;
    for i in 0..conform::TASKS {
        let t = &mut TRACKER.get().task[i];
        if !t.used {
            continue;
        }
        if t.group != 0 && !gather_lead(g, i, vtable) {
            continue;
        }
        let t = &mut TRACKER.get().task[i];
        let who = if t.group != 0 { "Gather" } else { "Conform" };
        let target = stack_of(g, t.target, vtable);
        let stack = stack_of(g, t.stack, vtable);
        let seen = CfSeen {
            target_alive: target != 0,
            target_province: if target != 0 { province(target) } else { -1 },
            stack_alive: stack != 0,
            stack_province: if stack != 0 { province(stack) } else { -1 },
            construction_alive: !t.construction.none() && lookup(g, t.construction) != 0,
        };
        if t.kind == Kind::Join
            && target != 0
            && stack != 0
            && !reachable(target, seen.stack_province)
        {
            let t = &mut TRACKER.get().task[i];
            t.used = false;
            log!(
                "{who}: reinforcement {} has no land route to target {}; left in place",
                t.stack.id,
                t.target.id
            );
            continue;
        }
        let mut destination = -1;
        let t = &mut TRACKER.get().task[i];
        // TPL-12: the replacement's own regiment, so its merge can be confirmed.
        if !t.replaces.none() && stack != 0 {
            let first = rdp(rdp(stack + 0x38));
            if first != 0 {
                t.fresh = entry_id(first);
            }
        }
        let action = conform::step(t, &seen, date, &mut destination);
        let t = TRACKER.get().task[i];
        match action {
            Action::Move => {
                if !send(make_move(stack, destination), g.vt_move) {
                    log!(
                        "{who}: Move {} to province {destination} not queued",
                        t.stack.id
                    );
                } else {
                    log!(
                        "{who}: stack {} marches to province {destination} (safe path)",
                        t.stack.id
                    );
                }
            }
            Action::Merge => {
                if !send(make_merge(target, stack), g.vt_merge) {
                    log!("{who}: Merge of {} not queued", t.stack.id);
                } else {
                    log!("{who}: stack {} joins {}", t.stack.id, t.target.id);
                }
            }
            Action::Transfer => {
                // Only regiments still in the conformed stack, and only while the new
                // stack is beside it; the engine re-checks each one (FUN_00513F30).
                let (mut sent, mut skipped) = (0, 0);
                if seen.target_province != seen.stack_province {
                    log!(
                        "Conform: the conformed stack moved before its extras left; nothing detached"
                    );
                    TRACKER.get().task[i].used = false;
                    continue;
                }
                // TPL-4: what FUN_00513F30 checks, logged so a refusal can be read
                // from tgcx.log: both stacks' +0x74 and +0x194 must be 0 and their
                // +0xDC locations equal; each regiment's +0x74 is its stack.
                log!(
                    "Conform: transfer check new {} (+74 {:X} +194 {:X} +DC {:X} +40 {}) source {} (+74 {:X} +194 {:X} +DC {:X} +40 {})",
                    t.stack.id,
                    rd32(stack + 0x74),
                    rd32(stack + 0x194),
                    rd32(stack + 0xDC),
                    rdi(stack + 0x40),
                    t.target.id,
                    rd32(target + 0x74),
                    rd32(target + 0x194),
                    rd32(target + 0xDC),
                    rdi(target + 0x40)
                );
                for r in &t.regiment[..t.regiments] {
                    let entry = lookup(g, *r);
                    if entry == 0 || !stack_has(target, entry) {
                        skipped += 1;
                        continue;
                    }
                    if send(make_transfer(t.stack, entry), g.vt_transfer) {
                        sent += 1;
                    } else {
                        skipped += 1;
                        break;
                    }
                }
                log!(
                    "Conform: {sent} extra transfers sent to stack {}, {skipped} skipped",
                    t.stack.id
                );
            }
            Action::Verify => {
                // The engine refuses a Transfer silently: count what actually moved.
                let (mut arrived, mut still) = (0, 0);
                for r in &t.regiment[..t.regiments] {
                    let entry = lookup(g, *r);
                    if entry != 0 && stack != 0 && stack_has(stack, entry) {
                        arrived += 1;
                    } else if entry != 0 && target != 0 && stack_has(target, entry) {
                        still += 1;
                        if still == 1 {
                            log!(
                                "Conform: regiment {} still in stack {} (+74 {:X}, its +38 type +42 {})",
                                r.id,
                                t.target.id,
                                rd32(entry + 0x74),
                                rd8(rdp(entry + 0x38) + 0x42)
                            );
                        }
                    }
                }
                log!(
                    "Conform: {arrived} of {} extras arrived in stack {}, {still} still in stack {}",
                    t.regiments,
                    t.stack.id,
                    t.target.id
                );
                if stack == 0 {
                    log!(
                        "Conform: detach stack {} disappeared before verification; {still} extras remain in the source",
                        t.stack.id
                    );
                }
                if stack != 0 && still != 0 && t.retries == 0 {
                    let t = &mut TRACKER.get().task[i];
                    t.used = true;
                    t.transfer_sent = false;
                    t.retries += 1;
                    log!("Conform: sending the transfers again tomorrow");
                }
            }
            Action::TargetGone => log!(
                "{who}: stack {} stays: the stack it was joining is gone",
                t.stack.id
            ),
            Action::Cancelled => {
                log!(
                    "{who}: order for stack {} dropped (construction cancelled or stack not created)",
                    t.stack.id
                )
            }
            Action::Done if !t.replaces.none() => replace_done(g, &t, target),
            Action::Wait | Action::Done => {}
        }
    }
    replace_retire(g, date);
    if guard(|| mobo_daily(g, date)).is_none() {
        log!("Mobo driver fault; nothing sent today");
    }
    orders_close(g, date);
}

// ---------------------------------------------------------------- Build Army gathering (TPL-11, D20)

// One Recruit click is one army. Its regiments are tracked like Conform's builds, with
// no target until one of them appears: that stack becomes the army and marches by a
// safe path to the click's province; every later regiment marches to the army and
// merges, as a Conform build does. Every read here is read only (docs/symbols.md
// "TPL-11"); the only commands are the same Move and Merge (R5).
const GATHER_TEXT: usize = 131072;
/// Best supply asks routes only for this many provinces, best first.
const GATHER_TRIES: usize = 12;
pub const GATHER_RVAS: [u32; 10] = [
    0xB3C40, 0xB3DF3, // supply limit, RET 8
    0x42DB08, 0x42DB40, // map province adjacency
    0x13F3B0, 0x13F3D3, 0x33F03C, // rally points
    0x4CF4A9, // capital
    0xAAF90, 0xC4558, // province name
];

struct Gathering {
    enabled: bool,
    supply_fn: usize,
}

static GATHERING: Global<Gathering> = Global::new(Gathering {
    enabled: false,
    supply_fn: 0,
});

pub fn gather_enabled() -> bool {
    GATHERING.get().enabled
}

fn province_at(world: usize, id: i32) -> usize {
    let provinces = rdp(world + 0xACC);
    if provinces == 0 || !(0..=65535).contains(&id) {
        return 0;
    }
    rdp(provinces + id as usize * 4)
}

/// FUN_004B3C40(province, ref) RET 8, the result in thousandths written through ESI.
/// Owner and controller both `country`, so only ref+4 (the country index) is read.
fn supply_limit(province: usize, country: i32) -> i32 {
    let owner: [i32; 2] = [0, country];
    let mut out: i32 = 0;
    // SAFETY: the native supply limit (signatures GATHER_RVAS), as the map tooltip
    // calls it: ESI saved around the call, the callee pops its two arguments.
    unsafe {
        std::arch::asm!(
            "push esi",
            "mov esi, edx",
            "push ecx",
            "push edi",
            "call eax",
            "pop esi",
            in("ecx") owner.as_ptr(),
            in("edx") &raw mut out,
            in("edi") province,
            in("eax") GATHERING.get().supply_fn,
            clobber_abi("C"),
        );
    }
    out
}

/// A land province the country owns and controls, with its supply limit.
fn site(world: usize, country: i32, id: i32) -> Option<i32> {
    let province = province_at(world, id);
    if province == 0 || rdi(province + 0x12C) != country || rdi(province + 0x134) != country {
        return None;
    }
    let map = rdp(province + 0x5C);
    if map == 0 || rd8(map + 0x2A) == 0 {
        return None;
    }
    Some(supply_limit(province, country))
}

/// Land neighbours: map province +0xC8/+0xCC, 0x24-byte entries, +0 type (4 is
/// impassable), +4 the neighbour's id.
fn neighbours(world: usize, id: i32) -> Vec<i32> {
    let province = province_at(world, id);
    let map = if province != 0 {
        rdp(province + 0x5C)
    } else {
        0
    };
    let mut out = Vec::new();
    if map == 0 {
        return out;
    }
    let (begin, end) = (rdp(map + 0xC8), rdp(map + 0xCC));
    if begin == 0 || end < begin || (end - begin) / 0x24 > 64 {
        return out;
    }
    let mut entry = begin;
    while entry + 0x24 <= end {
        if rdi(entry) != 4 {
            out.push(rdi(entry + 4));
        }
        entry += 0x24;
    }
    out
}

/// The country's land rally points in the game's order: country +0x1570/+0x1574
/// holds pointers to rally points, +8 the province id, +0x20 the kind (0 land, 1 naval).
fn rally_points(country: usize) -> Vec<i32> {
    let mut out = Vec::new();
    let (begin, end) = (rdp(country + 0x1570), rdp(country + 0x1574));
    if begin == 0 || end < begin || (end - begin) / 4 > 4096 {
        return out;
    }
    let mut p = begin;
    while p < end {
        let rally = rdp(p);
        p += 4;
        if rally != 0 && rd8(rally + 0x20) == 0 {
            out.push(rdi(rally + 8));
        }
    }
    out
}

fn player_rally_points() -> Vec<i32> {
    let country = player_country(&live_globals());
    if country == 0 || !gather_enabled() {
        return Vec::new();
    }
    rally_points(country)
}

/// How many land rally points the arrows cycle through; logs them.
pub fn gather_rallies() -> u32 {
    use std::fmt::Write as _;
    let rallies = guard(player_rally_points).unwrap_or_default();
    let world = rdp(at(WORLD_RVA));
    let mut names = Text::<256>::new();
    for &id in &rallies {
        let name = guard(|| province_name(world, id)).unwrap_or_default();
        let _ = write!(names, " {id} {name};");
    }
    log!("Gather: {} land rally points:{names}", rallies.len());
    rallies.len() as u32
}

/// The name the map shows (map province +0xE4), else its id.
fn province_name(world: usize, id: i32) -> Text<64> {
    let province = province_at(world, id);
    let map = if province != 0 {
        rdp(province + 0x5C)
    } else {
        0
    };
    match (map != 0)
        .then(|| read_game_string::<64>(map + 0xE4))
        .flatten()
        .filter(|n| !n.is_empty())
    {
        Some(name) => name,
        None => crate::text!(64, "province {id}"),
    }
}

/// The player's land armies from the country's unit list, checked against the registry.
fn player_armies(g: &CfGlobals) -> Vec<usize> {
    let country = player_country(g);
    if country == 0 || !mobo_enabled() {
        return Vec::new();
    }
    let player = player(g);
    let mut out = Vec::new();
    let mut node = rdp(country + UNIT_LIST);
    let mut n = 0;
    while node != 0 && n < 20000 {
        let unit = rdp(node);
        node = rdp(node + 8);
        n += 1;
        if unit != 0
            && rdi(unit + 0xC4) == player
            && lookup(g, stack_id(unit)) == unit + 8
            && land(unit)
        {
            out.push(unit);
        }
    }
    out
}

/// What the footer's label shows for a mode: the rally point's name in that mode.
pub fn gather_label(mode: u32) -> Text<64> {
    match mode {
        gather::MODE_CAPITAL => Text::from_latin1("Capital"),
        _ if mode >= gather::MODE_RALLY => {
            let world = rdp(at(WORLD_RVA));
            let k = (mode - gather::MODE_RALLY) as usize;
            guard(|| {
                (world != 0)
                    .then(|| player_rally_points().get(k).copied())
                    .flatten()
                    .map(|id| province_name(world, id))
            })
            .flatten()
            .unwrap_or(Text::from_latin1("Rally point"))
        }
        _ => Text::from_latin1("Best supply"),
    }
}

#[derive(Default)]
pub struct GatherOutcome {
    pub result: TplResult,
    /// Where the army gathers, its name and supply limit (whole units).
    pub province: i32,
    pub name: Text<64>,
    pub supply: i32,
    /// The chosen province, when an army there moved this one beside it.
    pub moved_from: Option<Text<64>>,
    /// Why nothing was queued.
    pub refusal: Option<&'static str>,
}

fn gathers_load() {
    let s = SESSION.get();
    if s.gathers_loaded {
        return;
    }
    s.gathers_loaded = true;
    s.gathers.clear();
    s.gathers_ok = false;
    let Some(path) = settings_path("tgcx_gather_orders.txt") else {
        return;
    };
    s.gathers_path = path.clone();
    match read_settings(&path, GATHER_TEXT) {
        Stored::Absent => s.gathers_ok = true,
        Stored::Unreadable(error) => {
            log!("Gather orders file unreadable ({error}); armies are not resumed after a load")
        }
        Stored::TooBig => {
            log!("Gather orders file invalid: kept as is, not resumed or overwritten")
        }
        Stored::Text(text) => match gather::gather_parse(&text, UNITS) {
            Some(orders) => {
                s.gathers = orders;
                s.gathers_ok = true;
                log!("Gather orders file: {} saved orders", s.gathers.len());
            }
            None => log!("Gather orders file invalid: kept as is, not resumed or overwritten"),
        },
    }
}

fn gathers_save() {
    let s = SESSION.get();
    if !s.gathers_dirty || !s.gathers_ok || s.gathers_path.is_empty() {
        return;
    }
    if !write_settings(
        &s.gathers_path,
        gather::gather_write(&s.gathers, GATHER_TEXT),
    ) {
        if !s.gathers_failed {
            log!(
                "Gather orders file not saved; this session's armies will not resume after a load"
            );
        }
        s.gathers_failed = true;
        return;
    }
    s.gathers_dirty = false;
}

/// A dropped (oldest) record moves the later ones down: keep the tasks' links.
fn gather_new(task: usize, r: GtOrder) {
    gathers_load();
    let s = SESSION.get();
    if !s.gathers_ok {
        return;
    }
    let (index, dropped) = gather::gather_add(&mut s.gathers, r);
    if let Some(dropped) = dropped {
        for t in TRACKER
            .get()
            .task
            .iter_mut()
            .filter(|t| t.used && t.group != 0 && t.record != 0)
        {
            if t.record == dropped + 1 {
                t.record = 0;
            } else if t.record > dropped + 1 {
                t.record -= 1;
            }
        }
    }
    TRACKER.get().task[task].record = index + 1;
    s.gathers[index].active = true;
    s.gathers_dirty = true;
}

/// The group's army, in the session and in every open saved order of its click.
fn set_army(k: usize, army: CfId) {
    let s = SESSION.get();
    s.groups[k].army = army;
    let id = s.groups[k].id;
    for r in s.gathers.iter_mut().filter(|r| r.active && r.group == id) {
        if r.army != army {
            r.army = army;
            s.gathers_dirty = true;
        }
    }
}

/// Ended records are finished from today, and a click with no regiment left to track
/// is forgotten (its army, if any, stands on its own).
fn gathers_close(date: u32) {
    let s = SESSION.get();
    let tasks = &TRACKER.get().task;
    for i in 0..s.gathers.len() {
        if s.gathers[i].active
            && !tasks
                .iter()
                .any(|t| t.used && t.group != 0 && t.record == i + 1)
        {
            s.gathers[i].active = false;
            s.gathers[i].done = date;
            s.gathers_dirty = true;
        }
    }
    s.groups
        .retain(|grp| tasks.iter().any(|t| t.used && t.group == grp.id));
    gathers_save();
}

/// A gathering regiment's target for today: its click's army. The first regiment to
/// appear becomes that army and marches to the click's province; false then (its task
/// is done), and when the click is unknown.
fn gather_lead(g: &CfGlobals, i: usize, vtable: usize) -> bool {
    let s = SESSION.get();
    let t = &mut TRACKER.get().task[i];
    let Some(k) = s.groups.iter().position(|grp| grp.id == t.group) else {
        t.used = false;
        return false;
    };
    let grp = s.groups[k];
    if !grp.army.none() && stack_of(g, grp.army, vtable) == 0 {
        log!(
            "Gather: army {} of click {} is gone; the next regiment to appear leads",
            grp.army.id,
            grp.id
        );
        set_army(k, CfId::default());
    }
    let army = s.groups[k].army;
    let stack = stack_of(g, t.stack, vtable);
    if army.none() && stack != 0 {
        set_army(k, t.stack);
        t.used = false;
        let at = province(stack);
        if at == grp.province {
            log!(
                "Gather: stack {} forms army {} at province {at}",
                t.stack.id,
                grp.id
            );
        } else if send(make_move(stack, grp.province), g.vt_move) {
            log!(
                "Gather: stack {} leads army {} and marches to province {} (safe path)",
                t.stack.id,
                grp.id,
                grp.province
            );
        } else {
            log!(
                "Gather: Move of leading stack {} to province {} not queued",
                t.stack.id,
                grp.province
            );
        }
        return false;
    }
    t.target = army;
    true
}

/// After a load, as Conform's resume: every saved order of the player's tag whose date
/// window holds the loaded date is tracked again, with its click's army if it has one.
fn gathers_resume(g: &CfGlobals, date: u32) {
    gathers_load();
    let s = SESSION.get();
    if !s.gathers_ok || s.gathers.is_empty() {
        return;
    }
    let country = player_country(g);
    if country == 0 {
        return;
    }
    let tag = rd32(country + 0x1C);
    let (mut resumed, mut building, mut refused) = (0, 0, 0);
    for i in 0..s.gathers.len() {
        let r = s.gathers[i];
        s.gathers[i].active = false;
        if !gather::gather_applies(&r, tag, date) {
            continue;
        }
        let k = match s.groups.iter().position(|grp| grp.id == r.group) {
            Some(k) => k,
            None => {
                s.groups.push(Group {
                    id: r.group,
                    province: r.province,
                    army: CfId::default(),
                });
                s.groups.len() - 1
            }
        };
        if !r.army.none() {
            s.groups[k].army = r.army;
        }
        let Some(t) = conform::add(
            TRACKER.get(),
            Kind::Join,
            CfId::default(),
            r.stack,
            r.construction,
            r.unit,
            r.created,
        ) else {
            refused += 1;
            continue;
        };
        let task = &mut TRACKER.get().task[t];
        task.group = r.group;
        task.record = i + 1;
        task.construction_seen = true;
        s.gathers[i].active = true;
        resumed += 1;
        building += u32::from(lookup(g, r.construction) != 0);
    }
    if resumed != 0 || refused != 0 {
        log!(
            "Gather: {resumed} saved orders resumed after a load ({building} still being built), {refused} refused"
        );
    }
}

fn gather_inner(
    g: &CfGlobals,
    counts: &[i32; UNITS],
    filter: &ArmyFilter,
    mode: u32,
    out: &mut GatherOutcome,
) {
    let country = player_country(g);
    if country == 0 {
        out.refusal = Some("Cannot read the player's country; nothing queued.");
        return;
    }
    let index = player(g);
    let armies = player_armies(g);
    let Some(&reference) = armies.first() else {
        out.refusal = Some("Needs one army of yours to plan land routes; nothing queued.");
        return;
    };
    SESSION.get().stack_vtable = rdp(reference);
    let world = g.world;
    let (owned, owned_end) = (rdp(country + 0x9D8), rdp(country + 0x9DC));
    if owned_end < owned || (owned_end - owned) / 4 > 65536 {
        out.refusal = Some("Cannot read the player's provinces; nothing queued.");
        return;
    }
    let mut sites = Vec::new();
    let mut p = owned;
    while p < owned_end {
        let id = rdi(p);
        p += 4;
        if let Some(supply) = site(world, index, id) {
            sites.push(Site {
                province: id,
                supply,
            });
        }
    }
    let base = match mode {
        gather::MODE_CAPITAL => {
            let capital = rdi(country + 0xBE4);
            if !sites.iter().any(|s| s.province == capital) {
                out.refusal =
                    Some("The capital is not a land province you own and control; nothing queued.");
                return;
            }
            capital
        }
        _ if mode >= gather::MODE_RALLY => {
            let k = (mode - gather::MODE_RALLY) as usize;
            let Some(rally) = rally_points(country).get(k).copied() else {
                out.refusal = Some(
                    "No land rally point at this choice; set one or click the arrows. Nothing queued.",
                );
                return;
            };
            if !sites.iter().any(|s| s.province == rally) {
                out.refusal = Some(
                    "The rally point is not in a province you own and control; nothing queued.",
                );
                return;
            }
            rally
        }
        _ => {
            let Some(sources) = templates::plan_sources(counts, Some(filter)) else {
                out.refusal = Some("Recruitment eligibility unavailable; nothing queued.");
                return;
            };
            if sources.is_empty() {
                out.refusal = Some("No free soldier pops can build this template; nothing queued.");
                return;
            }
            let mut routes: Vec<(i32, i32, bool)> = Vec::new();
            let mut reach = |to: i32| {
                let mut n = 0;
                for &from in &sources {
                    let ok = match routes.iter().find(|r| r.0 == from && r.1 == to) {
                        Some(r) => r.2,
                        None => {
                            let ok = route_between(reference, from, to).is_some();
                            routes.push((from, to, ok));
                            ok
                        }
                    };
                    n += u32::from(ok);
                }
                n
            };
            let Some(best) = gather::best(&sites, sources.len() as u32, GATHER_TRIES, &mut reach)
            else {
                out.refusal = Some("No province of yours can be reached by land from these pops.");
                return;
            };
            best
        }
    };
    // Any army of ours, or an earlier click still gathering, takes a province.
    let mut taken: Vec<i32> = armies.iter().map(|&a| province(a)).collect();
    taken.extend(SESSION.get().groups.iter().map(|grp| grp.province));
    let mut near = |id: i32| neighbours(world, id);
    let mut supply_of = |id: i32| site(world, index, id);
    let Some(target) = gather::around(base, &mut near, &mut supply_of, &|id| taken.contains(&id))
    else {
        out.refusal = Some("Every province around the target holds an army; nothing queued.");
        return;
    };
    out.province = target;
    out.name = province_name(world, target);
    out.supply = sites
        .iter()
        .find(|s| s.province == target)
        .map_or(0, |s| s.supply / 1000);
    if target != base {
        out.moved_from = Some(province_name(world, base));
    }
    gathers_load();
    let s = SESSION.get();
    let live: Vec<u32> = s.groups.iter().map(|grp| grp.id).collect();
    let group = gather::next_group(&s.gathers, &live);
    let date = rd32(world + 0xB0C);
    let tag = rd32(country + 0x1C);
    let mut on_queued = |unit: usize, province: i32, construction: [u32; 2], stack: [u32; 2]| {
        let s = CfId {
            kind: stack[0],
            id: stack[1],
        };
        let k = CfId {
            kind: construction[0],
            id: construction[1],
        };
        match conform::add(
            TRACKER.get(),
            Kind::Join,
            CfId::default(),
            s,
            k,
            unit as i32,
            date,
        ) {
            Some(task) => {
                let t = &mut TRACKER.get().task[task];
                t.origin = province;
                t.group = group;
                if !k.none() && !s.none() {
                    gather_new(
                        task,
                        GtOrder {
                            tag,
                            created: date,
                            done: 0,
                            province: target,
                            group,
                            army: CfId::default(),
                            stack: s,
                            construction: k,
                            unit: unit as i32,
                            active: false,
                        },
                    );
                }
            }
            None => log!(
                "Gather: tracker full; regiment from province {province} will not march by itself"
            ),
        }
    };
    let mut eligible = |province: i32| route_between(reference, province, target).is_some();
    out.result = templates::recruit(
        counts,
        Some(&mut on_queued),
        Some(&mut eligible),
        Some(filter),
    );
    if out.result.queued != 0 {
        SESSION.get().groups.push(Group {
            id: group,
            province: target,
            army: CfId::default(),
        });
    }
    gathers_save();
    log!(
        "Gather: army {group} at province {target} ({}, supply {}{}): {} queued, {} left out",
        out.name,
        out.supply,
        if target != base {
            ", beside a taken one"
        } else {
            ""
        },
        out.result.queued,
        out.result.missing
    );
}

/// TPL-11: recruit one template army that gathers in one province. counts[unit index]
/// regiments; sources without a land route to the target are left out (D17, D20).
pub fn gather_recruit(counts: &[i32; UNITS], filter: &ArmyFilter, mode: u32) -> GatherOutcome {
    let mut out = GatherOutcome::default();
    if !gather_enabled() {
        out.refusal = Some("Gathering unavailable (guards); nothing queued.");
        return out;
    }
    let g = live_globals();
    if g.world == 0 || guard(|| gather_inner(&g, counts, filter, mode, &mut out)).is_none() {
        log!("Gather rejected: guarded native read fault");
        out = GatherOutcome {
            refusal: Some("Game read failed; nothing queued."),
            ..GatherOutcome::default()
        };
    }
    out
}

// ---------------------------------------------------------------- conforming one stack

#[derive(Clone, Copy, Default)]
pub struct CfResult {
    pub refusal: Refusal,
    pub read: bool,
    pub detached: u32,
    pub gathered: u32,
    pub queued: u32,
    pub missing: u32,
    pub failed: u32,
}

/// The plan for one stack, read and planned but nothing sent: what the Conform window
/// previews and what Conform then carries out. False when the stack is not a readable
/// land stack of the player's. Not guarded: callers are.
pub fn prepare(
    g: &CfGlobals,
    stack: usize,
    units: Option<&[usize; UNITS]>,
    want: &[i32; UNITS],
    plan: &mut CfPlan,
    target: &mut CfStack,
) -> bool {
    if g.world == 0
        || stack == 0
        || rdi(stack + 0xC4) != player(g)
        || !land(stack)
        || !read_stack(stack, units, target)
    {
        return false;
    }
    prune_routes(g, target.id, stack);
    let pending = conform::pending(TRACKER.get(), target.id);
    // Missing regiments are always built (user, 2026-10-17): no idle stacks are
    // gathered, so the planner gets none.
    conform::make_plan(&target.regiment, want, Some(&pending), &[], plan);
    true
}

/// Extras already on their way out of this stack: one Create per stack at a time.
pub fn leaving(target: CfId) -> bool {
    TRACKER
        .get()
        .task
        .iter()
        .any(|t| t.used && t.kind == Kind::Fill && t.target == target)
}

/// One Create, immediately followed by a Transfer for each regiment still in `stack`,
/// in the same command queue submission (P51), tracked as a FILL task the daily driver
/// verifies. Returns (transfers queued, failures).
fn queue_detach(
    g: &CfGlobals,
    stack: usize,
    target_id: CfId,
    ids: &[CfId],
    date: u32,
    what: &str,
) -> (u32, u32) {
    let (mut detached, mut failed) = (0, 0);
    let id = new_id(g);
    match conform::add(
        TRACKER.get(),
        Kind::Fill,
        target_id,
        id,
        CfId::default(),
        -1,
        date,
    ) {
        None => {
            failed += 1;
            log!("Conform: tracker full; no Create queued");
        }
        Some(t) if !send(make_create(stack, id), g.vt_create) => {
            TRACKER.get().task[t].used = false;
            failed += 1;
            log!("Conform: Create not queued; nothing detached");
        }
        Some(t) => {
            // Waiting a game day left the destination empty and unselected.
            // Transfer stores the existing regiment's reference; the new
            // destination id is resolved later, when the command executes. Keep
            // Create first, then all Transfers, in the engine queue.
            for (i, regiment) in ids.iter().enumerate().take(REGIMENTS) {
                let task = &mut TRACKER.get().task[t];
                task.regiment[task.regiments] = *regiment;
                task.regiments += 1;
                let entry = lookup(g, *regiment);
                if i == 0 && entry != 0 {
                    let kind = rdp(entry + 0x38);
                    log!(
                        "Conform: first {what} {} unit flags +40 {} +42 {}",
                        regiment.id,
                        rd8(kind + 0x40),
                        rd8(kind + 0x42)
                    );
                }
                if entry != 0
                    && stack_has(stack, entry)
                    && send(make_transfer(id, entry), g.vt_transfer)
                {
                    detached += 1;
                } else {
                    failed += 1;
                }
            }
            let task = &mut TRACKER.get().task[t];
            task.transfer_sent = true;
            task.transfer_date = date;
            log!(
                "Conform: Create then {detached} of {} {what} transfers queued together for stack {}/{}",
                task.regiments,
                id.kind,
                id.id
            );
        }
    }
    (detached, failed)
}

/// A regiment Conform (or Replace) had built: tracked to march and merge into the
/// target, and kept as a TPL-7 order so a reload resumes it. The task's index.
fn track_build(
    target_id: CfId,
    tag: u32,
    date: u32,
    unit: usize,
    province: i32,
    construction: [u32; 2],
    stack: [u32; 2],
) -> Option<usize> {
    let s = CfId {
        kind: stack[0],
        id: stack[1],
    };
    let k = CfId {
        kind: construction[0],
        id: construction[1],
    };
    let Some(task) = conform::add(
        TRACKER.get(),
        Kind::Join,
        target_id,
        s,
        k,
        unit as i32,
        date,
    ) else {
        log!("Conform: tracker full; regiment from province {province} will not march by itself");
        return None;
    };
    TRACKER.get().task[task].origin = province;
    if !k.none() && !s.none() {
        let order = CfOrder {
            tag,
            created: date,
            done: 0,
            target: target_id,
            stack: s,
            construction: k,
            unit: unit as i32,
            active: false,
        };
        order_new(task, order);
    }
    Some(task)
}

static CONFORM_TARGET: Global<CfStack> = Global::new(CfStack::EMPTY);

fn conform_inner(
    g: &CfGlobals,
    stack: usize,
    units: &[usize; UNITS],
    want: &[i32; UNITS],
    plan: &mut CfPlan,
    culture: u32,
    r: &mut CfResult,
) {
    let target = CONFORM_TARGET.get();
    if !prepare(g, stack, Some(units), want, plan, target) {
        log!("Conform refused: not a readable land stack of the player");
        return;
    }
    r.read = true;
    r.refusal = plan.refusal;
    if r.refusal != Refusal::Ok {
        log!("Conform: plan refused ({})", plan.refusal as i32);
        return;
    }
    SESSION.get().stack_vtable = rdp(stack);
    let date = rd32(g.world + 0xB0C);
    let target_id = target.id;
    let is_leaving = leaving(target_id);
    if !plan.detach.is_empty() && is_leaving {
        log!(
            "Conform: extras of stack {} are already leaving",
            target_id.id
        );
    } else if !plan.detach.is_empty() {
        let ids: Vec<CfId> = plan.detach.iter().map(|d| d.id).collect();
        let (detached, failed) = queue_detach(g, stack, target_id, &ids, date, "extra");
        r.detached += detached;
        r.failed += failed;
    }
    for idle in &plan.gather {
        if conform::add(
            TRACKER.get(),
            Kind::Join,
            target_id,
            idle.stack,
            CfId::default(),
            idle.unit,
            date,
        )
        .is_some()
        {
            r.gathered += 1;
        } else {
            r.failed += 1;
        }
    }
    if plan.build_total != 0 {
        let tag = rd32(stack + 0xC0);
        let mut on_queued =
            |unit: usize, province: i32, construction: [u32; 2], stack: [u32; 2]| {
                track_build(target_id, tag, date, unit, province, construction, stack);
            };
        let built = recruit(stack, &plan.build, &mut on_queued, culture);
        r.queued = built.queued;
        r.missing = built.missing;
        r.failed += built.failed;
        orders_save();
    }
    log!(
        "Conform stack {}: {} detach transfers queued, {} gathered, {} built, {} left out, {} failed",
        target_id.id,
        r.detached,
        r.gathered,
        r.queued,
        r.missing,
        r.failed
    );
}

/// want[unit index] regiments. Plans first; nothing is sent unless the plan is
/// accepted. Extras: one Create immediately followed by the Transfers in the same
/// command queue submission, before an empty stack can be removed. Gathered stacks:
/// tracked, marched and merged by the daily driver. Builds: TPL-0's recruit, each
/// future stack tracked the same way.
pub fn conform_with(
    g: &CfGlobals,
    stack: usize,
    units: &[usize; UNITS],
    want: &[i32; UNITS],
    plan: &mut CfPlan,
    culture: u32,
) -> CfResult {
    let mut r = CfResult::default();
    if guard(|| conform_inner(g, stack, units, want, plan, culture, &mut r)).is_none() {
        log!("Conform rejected: guarded native read fault");
    }
    r
}

// ---------------------------------------------------------------- Replace depleted (TPL-12)

// The chooser (conform_gui) asks which regiments of the open stack are depleted, then
// has each replaced: a new regiment of its type is built by Conform's recruit and
// tracked as a JOIN that carries the old regiment's id. Once the new one has merged,
// the old one leaves into its own stack by the same Create + Transfers as Conform's
// extras. Reads only (docs/symbols.md "TPL-12"): strength +0x3C against the type's
// +0xB0 (FUN_005BFF80), the pop +0x30, and the pop's two native counts the unit panel
// colours with. The only commands are Conform's (R5).
pub const REPLACE_RVAS: [u32; 7] = [
    0x1BFF80, 0x1E0C82, 0x565878, 0x565921, 0x565930, 0x56598A, 0x394048,
];
const REPLACE_TEXT: usize = 131072;

struct Replacing {
    enabled: bool,
    supports_fn: usize,
    has_fn: usize,
    choice: Choice,
    choice_loaded: bool,
    logged: u32,
    orders: Vec<RpOrder>,
    loaded: bool,
    ok: bool,
    dirty: bool,
    failed: bool,
    path: Vec<u16>,
    /// (target, old regiment): replacements merged, the old regiment still to leave.
    retire: Vec<(CfId, CfId)>,
}

static REPLACING: Global<Replacing> = Global::new(Replacing {
    enabled: false,
    supports_fn: 0,
    has_fn: 0,
    choice: Choice::DEFAULT,
    choice_loaded: false,
    logged: 0,
    orders: Vec::new(),
    loaded: false,
    ok: false,
    dirty: false,
    failed: false,
    path: Vec::new(),
    retire: Vec::new(),
});

pub fn replace_enabled() -> bool {
    REPLACING.get().enabled
}

/// The chooser's last choice, from tgcx_replace_settings.txt (defaults when absent or
/// invalid; an invalid file is left as is until the player changes the choice).
pub fn replace_choice() -> Choice {
    let r = REPLACING.get();
    if !r.choice_loaded {
        r.choice_loaded = true;
        if let Some(path) = settings_path("tgcx_replace_settings.txt") {
            match read_settings(&path, 256) {
                Stored::Text(text) => match replace::settings_parse(&text) {
                    Some(c) => r.choice = c,
                    None => log!("Replace settings file invalid: defaults used"),
                },
                Stored::Absent => {}
                _ => log!("Replace settings file unreadable: defaults used"),
            }
        }
    }
    r.choice
}

pub fn set_replace_choice(c: Choice) -> bool {
    replace_choice();
    REPLACING.get().choice = c;
    settings_path("tgcx_replace_settings.txt")
        .is_some_and(|path| write_settings(&path, Some(replace::settings_write(&c))))
}

/// FUN_00965870(pop, 0), stdcall RET 8: the regiments the pop can support (0 below the
/// minimum size); FUN_00965930, EAX the pop and a pushed 0, RET 4: the regiments (and
/// queued constructions) it backs. The unit panel's red/yellow (RVA 0x394048).
fn pop_counts(pop: usize) -> (i32, i32) {
    let r = REPLACING.get();
    type Supports = unsafe extern "system" fn(usize, u32) -> i32;
    // SAFETY: the native count, guarded by REPLACE_RVAS; reads only.
    let supports = unsafe { std::mem::transmute::<usize, Supports>(r.supports_fn)(pop, 0) };
    let has: i32;
    // SAFETY: the native count with its EAX argument, as the unit panel calls it.
    unsafe {
        std::arch::asm!(
            "push 0",
            "call {f}",
            f = in(reg) r.has_fn,
            inlateout("eax") pop => has,
            clobber_abi("C"),
        );
    }
    (supports, has)
}

fn health(entry: usize, unit: i32) -> Health {
    let kind = rdp(entry + 0x38);
    let strength = rdi(entry + 0x3C);
    let full = if kind != 0 { rdi(kind + 0xB0) } else { 0 };
    let percent = if full > 0 && strength >= 0 {
        (i64::from(strength) * 100 / i64::from(full)) as i32
    } else {
        -1
    };
    let pop = rdp(entry + 0x30);
    let (supports, has) = if pop != 0 { pop_counts(pop) } else { (1, 0) };
    let r = REPLACING.get();
    if r.logged < 8 {
        r.logged += 1;
        log!(
            "Replace probe: regiment {} strength +3C {strength} of type +B0 {full} = {percent}%, pop {pop:08X} size {} supports {supports} has {has}",
            rd32(entry + 0xC),
            if pop != 0 { rdi(pop + 0x58) } else { -1 }
        );
    }
    Health {
        id: entry_id(entry),
        unit,
        strength: percent,
        pop,
        supports,
        has,
    }
}

/// Old regiments already being replaced or leaving: never chosen twice.
fn replacing() -> Vec<CfId> {
    let mut busy: Vec<CfId> = TRACKER
        .get()
        .task
        .iter()
        .filter(|t| t.used && !t.replaces.none())
        .map(|t| t.replaces)
        .collect();
    busy.extend(REPLACING.get().retire.iter().map(|&(_, old)| old));
    // Already queued to leave (Conform's extras or a replaced regiment).
    for t in TRACKER
        .get()
        .task
        .iter()
        .filter(|t| t.used && t.kind == Kind::Fill)
    {
        busy.extend_from_slice(&t.regiment[..t.regiments]);
    }
    busy
}

static REPLACE_STACK: Global<CfStack> = Global::new(CfStack::EMPTY);

/// The open stack's depleted regiments under `choice`, in stack order. False when it is
/// not a readable land stack of the player's. Not guarded: callers are.
pub fn depleted_of(
    g: &CfGlobals,
    stack: usize,
    units: &[usize; UNITS],
    choice: &Choice,
    out: &mut Vec<(Health, Why)>,
) -> bool {
    out.clear();
    let target = REPLACE_STACK.get();
    if !replace_enabled()
        || g.world == 0
        || stack == 0
        || rdi(stack + 0xC4) != player(g)
        || !land(stack)
        || !read_stack(stack, Some(units), target)
    {
        return false;
    }
    let regs: Vec<Health> = (0..target.regiment.len())
        .map(|i| health(target.entry[i], target.regiment[i].unit))
        .collect();
    for (i, why) in replace::depleted(&regs, choice, &replacing()) {
        out.push((regs[i], why));
    }
    true
}

#[derive(Clone, Copy, Default)]
pub struct RpResult {
    pub read: bool,
    pub found: u32,
    pub queued: u32,
    pub missing: u32,
    pub failed: u32,
}

static REPLACE_FOUND: Global<Vec<(Health, Why)>> = Global::new(Vec::new());

fn replace_inner(
    g: &CfGlobals,
    stack: usize,
    units: &[usize; UNITS],
    choice: &Choice,
    culture: u32,
    r: &mut RpResult,
) {
    let found = REPLACE_FOUND.get();
    if !depleted_of(g, stack, units, choice, found) {
        log!("Replace refused: not a readable land stack of the player");
        return;
    }
    r.read = true;
    r.found = found.len() as u32;
    if found.is_empty() {
        return;
    }
    SESSION.get().stack_vtable = rdp(stack);
    let date = rd32(g.world + 0xB0C);
    let tag = rd32(stack + 0xC0);
    let target_id = stack_id(stack);
    let mut counts = [0i32; UNITS];
    for (h, _) in found.iter() {
        counts[h.unit as usize] += 1;
    }
    // Recruit queues in unit order; each one takes the first old regiment of its type.
    let mut olds: Vec<(usize, CfId)> = found.iter().map(|(h, _)| (h.unit as usize, h.id)).collect();
    let mut on_queued = |unit: usize, province: i32, construction: [u32; 2], new: [u32; 2]| {
        let Some(i) = olds.iter().position(|&(u, _)| u == unit) else {
            return;
        };
        let (_, old) = olds.remove(i);
        if let Some(task) = track_build(target_id, tag, date, unit, province, construction, new) {
            TRACKER.get().task[task].replaces = old;
            let stack = TRACKER.get().task[task].stack;
            if !stack.none() {
                replace_order_new(
                    task,
                    RpOrder {
                        tag,
                        created: date,
                        done: 0,
                        target: target_id,
                        stack,
                        old,
                        merged: false,
                        active: false,
                    },
                );
            }
        }
    };
    let built = recruit(stack, &counts, &mut on_queued, culture);
    r.queued = built.queued;
    r.missing = built.missing;
    r.failed = built.failed;
    orders_save();
    replace_save();
    log!(
        "Replace stack {}: {} depleted, {} replacements built, {} left out, {} failed",
        target_id.id,
        r.found,
        r.queued,
        r.missing,
        r.failed
    );
}

/// Builds one regiment for each depleted regiment of the stack (Conform's recruit: the
/// template's culture, land reach); the old ones leave as their replacements merge.
pub fn replace_with(
    g: &CfGlobals,
    stack: usize,
    units: &[usize; UNITS],
    choice: &Choice,
    culture: u32,
) -> RpResult {
    let mut r = RpResult::default();
    if guard(|| replace_inner(g, stack, units, choice, culture, &mut r)).is_none() {
        log!("Replace rejected: guarded native read fault");
    }
    r
}

fn replace_load() {
    let r = REPLACING.get();
    if r.loaded {
        return;
    }
    r.loaded = true;
    r.orders.clear();
    r.ok = false;
    let Some(path) = settings_path("tgcx_replace_orders.txt") else {
        return;
    };
    r.path = path.clone();
    match read_settings(&path, REPLACE_TEXT) {
        Stored::Absent => r.ok = true,
        Stored::Text(text) => match replace::parse(&text) {
            Some(orders) => {
                r.orders = orders;
                r.ok = true;
            }
            None => log!("Replace orders file invalid: kept as is, not resumed or overwritten"),
        },
        _ => log!(
            "Replace orders file unreadable or too big: replacements are not resumed after a load"
        ),
    }
}

fn replace_save() {
    let r = REPLACING.get();
    if !r.dirty || !r.ok || r.path.is_empty() {
        return;
    }
    if !write_settings(&r.path, replace::write(&r.orders, REPLACE_TEXT)) {
        if !r.failed {
            log!(
                "Replace orders file not saved; this session's replacements will not resume after a load"
            );
        }
        r.failed = true;
        return;
    }
    r.dirty = false;
}

/// A dropped (oldest) record moves the later ones down: keep the tasks' links.
fn replace_order_new(task: usize, order: RpOrder) {
    replace_load();
    let r = REPLACING.get();
    if !r.ok {
        return;
    }
    let (index, dropped) = replace::add(&mut r.orders, order);
    if let Some(dropped) = dropped {
        for t in TRACKER
            .get()
            .task
            .iter_mut()
            .filter(|t| t.used && t.rp_record != 0)
        {
            if t.rp_record == dropped + 1 {
                t.rp_record = 0;
            } else if t.rp_record > dropped + 1 {
                t.rp_record -= 1;
            }
        }
    }
    TRACKER.get().task[task].rp_record = index + 1;
    r.orders[index].active = true;
    r.dirty = true;
}

/// The record's old regiment is still in its target stack (read by id only, so it works
/// before any stack class is known after a load).
fn old_in_target(g: &CfGlobals, o: &RpOrder) -> bool {
    let target = lookup(g, o.target);
    let old = lookup(g, o.old);
    target != 0 && old != 0 && stack_has(target - 8, old)
}

/// After a load (and the Conform orders' resume): the JOINs resumed for a replacement
/// carry their old regiment again; a replacement saved after its merge sends the old
/// regiment out again; one nothing can resume is finished from the save's date.
fn replace_resume(g: &CfGlobals, date: u32) {
    let r = REPLACING.get();
    r.retire.clear();
    if !r.enabled {
        return;
    }
    replace_load();
    if !r.ok || r.orders.is_empty() {
        return;
    }
    let (mut resumed, mut retired, mut closed) = (0, 0, 0);
    for i in 0..r.orders.len() {
        let o = r.orders[i];
        r.orders[i].active = false;
        let entry = lookup(g, o.target);
        let applies = entry != 0 && replace::applies(&o, rd32(entry - 8 + 0xC0), date);
        let joining = TRACKER.get().task.iter().position(|t| {
            t.used
                && t.kind == Kind::Join
                && t.group == 0
                && t.stack == o.stack
                && t.target == o.target
        });
        match replace::on_load(&o, applies, joining.is_some(), old_in_target(g, &o)) {
            Load::Skip => {}
            Load::Resume => {
                let t = &mut TRACKER.get().task[joining.unwrap_or_default()];
                t.replaces = o.old;
                t.rp_record = i + 1;
                r.orders[i].active = true;
                resumed += 1;
            }
            Load::Retire => {
                r.retire.push((o.target, o.old));
                r.orders[i].active = true;
                retired += 1;
            }
            Load::Close => {
                r.orders[i].done = date;
                r.dirty = true;
                closed += 1;
            }
        }
    }
    if resumed + retired + closed != 0 {
        log!(
            "Replace: after a load {resumed} replacements resumed, {retired} old regiments to detach again, {closed} records finished"
        );
    }
    replace_save();
}

/// Records whose task ended are finished from today; a merged one only once its old
/// regiment has left the target.
fn replace_close(g: &CfGlobals, date: u32) {
    let r = REPLACING.get();
    for i in 0..r.orders.len() {
        let o = r.orders[i];
        if !o.active {
            continue;
        }
        let joining = TRACKER
            .get()
            .task
            .iter()
            .any(|t| t.used && t.rp_record == i + 1);
        if replace::finished(&o, joining, o.merged && old_in_target(g, &o)) {
            r.orders[i].active = false;
            r.orders[i].done = date;
            r.dirty = true;
        }
    }
    replace_save();
}

/// A replacement's JOIN ended: if its regiment is now in the target, the old one leaves.
/// Its record is marked merged so a save taken before the old one leaves still sends it.
fn replace_done(g: &CfGlobals, t: &conform::CfTask, target: usize) {
    let fresh = lookup(g, t.fresh);
    if target == 0 || t.fresh.none() || fresh == 0 || !stack_has(target, fresh) {
        log!(
            "Replace: replacement {} did not join {}; regiment {} stays",
            t.stack.id,
            t.target.id,
            t.replaces.id
        );
        return;
    }
    let r = REPLACING.get();
    if let Some(o) = t.rp_record.checked_sub(1).and_then(|i| r.orders.get_mut(i)) {
        o.merged = true;
        r.dirty = true;
    }
    r.retire.push((t.target, t.replaces));
}

/// Old regiments whose replacement merged leave their target, one Create per target
/// and day, never while that target's extras are still leaving.
fn replace_retire(g: &CfGlobals, date: u32) {
    if REPLACING.get().retire.is_empty() {
        return;
    }
    let vtable = SESSION.get().stack_vtable;
    let mut targets: Vec<CfId> = Vec::new();
    for &(target, _) in REPLACING.get().retire.iter() {
        if !targets.contains(&target) {
            targets.push(target);
        }
    }
    for target_id in targets {
        if leaving(target_id) {
            continue;
        }
        let r = REPLACING.get();
        let olds: Vec<CfId> = r
            .retire
            .iter()
            .filter(|&&(t, _)| t == target_id)
            .map(|&(_, old)| old)
            .collect();
        r.retire.retain(|&(t, _)| t != target_id);
        let target = stack_of(g, target_id, vtable);
        if target == 0 {
            log!(
                "Replace: stack {} is gone; {} old regiments not detached",
                target_id.id,
                olds.len()
            );
            continue;
        }
        let here: Vec<CfId> = olds
            .iter()
            .copied()
            .filter(|&old| {
                let entry = lookup(g, old);
                entry != 0 && stack_has(target, entry)
            })
            .collect();
        if here.is_empty() {
            continue;
        }
        let (detached, failed) = queue_detach(g, target, target_id, &here, date, "replaced");
        log!(
            "Replace: {detached} replaced regiments leave stack {} ({failed} failed); disband them there if you wish",
            target_id.id
        );
    }
}

// ---------------------------------------------------------------- CUnitView capture

// Primary vtable 0x00E16850: slot 8 refresh (FUN_0079B800, called each frame by the
// secondary slot 9 while visible, ECX the view) and slot 0 the deleting destructor
// (FUN_00799290). The shown stack is view+0x28. The first newly shown stacks are also
// described in the log (read only), so the regiment layout and the registry walk stay
// checkable in game (U-9).
#[derive(Clone, Copy, Default)]
pub struct CfView {
    pub view: usize,
    shown: usize,
    pub thread: u32,
}

pub const VIEWS: usize = 32;

/// P47: a unit panel can die without our destructor hook (its vtable just changes).
/// Its slot is freed then too, and a full table is logged.
pub struct Views {
    pub views: [CfView; VIEWS],
    lost: u32,
    released: u32,
    full_logged: bool,
    enabled: bool,
    described: u32,
}

pub static VIEWS_STATE: Global<Views> = Global::new(Views {
    views: [CfView {
        view: 0,
        shown: 0,
        thread: 0,
    }; VIEWS],
    lost: 0,
    released: 0,
    full_logged: false,
    enabled: false,
    described: 0,
});

pub fn enabled() -> bool {
    VIEWS_STATE.get().enabled
}

static REFRESH_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static DESTROY_ORIGINAL: AtomicUsize = AtomicUsize::new(0);

extern "C" fn capture(view: usize) {
    keep_last_error(|| {
        let v = VIEWS_STATE.get();
        if !v.enabled || rdp(view) != base() + VIEW_VT_RVA as usize {
            return;
        }
        let known = v.views.iter().any(|w| w.view == view);
        if known {
            return;
        }
        if let Some(empty) = v.views.iter_mut().find(|w| w.view == 0) {
            *empty = CfView {
                view,
                shown: 0,
                thread: thread_id(),
            };
        } else if !v.full_logged {
            v.full_logged = true;
            log!("Conform: unit panel table full ({VIEWS} views); a new panel has no Conform");
        }
    })
}

extern "C" fn release(view: usize) {
    keep_last_error(|| {
        conform_gui::panel_release(view);
        let v = VIEWS_STATE.get();
        for w in v.views.iter_mut().filter(|w| w.view == view) {
            *w = CfView::default();
            v.released += 1;
        }
    })
}

crate::view_thunk!(refresh_thunk, capture, REFRESH_ORIGINAL);
crate::view_thunk!(destroy_thunk, release, DESTROY_ORIGINAL);

static DESCRIBE_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static DESCRIBE_STACK: Global<CfStack> = Global::new(CfStack::EMPTY);

fn describe(g: &CfGlobals, stack: usize) {
    let country = DESCRIBE_COUNTRY.get();
    let s = DESCRIBE_STACK.get();
    let scanned = templates::scan(&templates::live_globals(), country);
    if !read_stack(stack, scanned.then_some(&country.unit), s) {
        log!("Conform probe: stack {stack:08X} regiment list unreadable");
        return;
    }
    let back = lookup(g, s.id);
    let tag = rd32(stack + 0xC0);
    log!(
        "Conform probe: stack id {}/{} province {} tag {} country {} (player {}) regiments {} (+0x40 {}) land {} registry {}",
        s.id.kind,
        s.id.id,
        s.province,
        Tag(tag, 3),
        s.country,
        player(g),
        s.regiment.len(),
        rdi(stack + 0x40),
        i32::from(land(stack)),
        if back == stack + 8 {
            "round trip"
        } else {
            "MISMATCH"
        }
    );
    for i in 0..s.regiment.len().min(12) {
        let kind = s.kind[i];
        let name = if kind != 0 {
            read_game_string::<48>(kind + 8).unwrap_or_default()
        } else {
            Default::default()
        };
        let r = &s.regiment[i];
        let shown = if r.unit >= 0 {
            Bytes(country.name[r.unit as usize].bytes()).to_string()
        } else {
            name.to_string()
        };
        log!(
            "  regiment {}/{} type {kind:08X} +0x24 {} unit {} {shown} registry {}",
            r.id.kind,
            r.id.id,
            if kind != 0 { rdi(kind + 0x24) } else { -1 },
            r.unit,
            if lookup(g, r.id) == s.entry[i] {
                "round trip"
            } else {
                "MISMATCH"
            }
        );
    }
}

/// Views whose vtable changed were destroyed by a path the slot-0 hook does not see:
/// free the view and its Conform panel (whose GUI is gone with it).
fn sweep_views() {
    for i in 0..VIEWS {
        let view = VIEWS_STATE.get().views[i].view;
        if view == 0 {
            continue;
        }
        let alive = guard(|| rdp(view) == base() + VIEW_VT_RVA as usize).unwrap_or(false);
        if alive {
            continue;
        }
        conform_gui::panel_forget(view);
        let v = VIEWS_STATE.get();
        v.lost += 1;
        if v.lost <= 8 {
            log!(
                "Conform: unit panel {view:08X} gone without its destructor ({} so far, {} by destructor); slot freed",
                v.lost,
                v.released
            );
        }
        v.views[i] = CfView::default();
    }
}

pub fn tick() {
    if !enabled() {
        return;
    }
    let g = live_globals();
    if guard(|| daily(&g)).is_none() {
        log!("Conform driver fault; pending orders forgotten");
        forget("fault");
    }
    sweep_views();
    for i in 0..VIEWS {
        let v = VIEWS_STATE.get().views[i];
        if v.view == 0 {
            continue;
        }
        let probed = guard(|| {
            let stack = rdp(v.view + 0x28);
            if stack == v.shown {
                return;
            }
            let state = VIEWS_STATE.get();
            state.views[i].shown = stack;
            if stack != 0 && state.described < 24 {
                state.described += 1;
                describe(&g, stack);
            }
        });
        if probed.is_none() {
            log!("Conform probe: guarded read fault; view released");
            VIEWS_STATE.get().views[i] = CfView::default();
        }
    }
    conform_gui::panels_tick();
}

pub const CF_RVAS: [u32; 58] = [
    0x1B8500, 0x1B8830, 0xA03794, 0x449E90,
    0x1B984F, // read-only land route, ABI, predicate, cleanup and sea flag
    0x1DE80F, 0x1DE83E, 0x1DE878, 0x1DE897, 0x1DE8D3, 0x092342, // Move
    0x1E13F3, 0x1E143B, 0x1E1450, 0x1E1580, 0x26EC40, 0x426A1F, // Merge
    0x1E1F1E, 0x1E1F4D, 0x1E1F67, 0x1E1F81, 0x1E1FBC, 0x3944FF, // Transfer
    0x17DB6E, 0x17DB8D, 0x17DBA5, 0x17DBBE, 0x17DBE7, 0x39DBD7, 0x39DB84,
    0x39DB98, // Create and kind
    0x39DB4C, 0x39DB56, 0x39DB70, 0x5C12D0, // new id under the lock
    0xA03394, 0xA033B4, 0xA03514, 0xA03534, 0xA03554, 0xA03574, 0xA00D34,
    0xA00D54, // command vtables
    0x5C16D6, 0x5C1D47, 0x5C1D65, 0x39DBC5, 0x39DC05, 0x1DE8B2, // registry, stack fields
    0x17D283, 0x17D334, 0x17D35B, // recruit ids
    0x398B3F, 0xA16850, 0xA16870, 0xA168B0, 0x398AE8, 0x3993B0, // CUnitView
];

pub fn install() {
    if !templates::installed() {
        log!("Conform rejected: templates not installed");
        return;
    }
    if !sys::require_signatures(&CF_RVAS) {
        log!("Conform rejected: guards; nothing conformed");
        return;
    }
    let n = NATIVES.get();
    n.move_fn = at(0x1DE7D0);
    n.merge_fn = at(0x1E13B0);
    n.transfer_fn = at(0x1E1EE0);
    n.create_fn = at(0x17DB30);
    n.lock_fn = at(0x5C12D0);
    n.path_fn = at(0x1B8500);
    n.path_clear_fn = at(0x449E90);
    REFRESH_ORIGINAL.store(at(0x39B800), Ordering::Relaxed);
    DESTROY_ORIGINAL.store(at(0x399290), Ordering::Relaxed);
    let table = at(VIEW_VT_RVA);
    let destroy = destroy_thunk as *const () as usize;
    let refresh = refresh_thunk as *const () as usize;
    if !sys::replace_pointer(table, at(0x399290), destroy) {
        return;
    }
    if !sys::replace_pointer(table + 8 * 4, at(0x39B800), refresh) {
        sys::replace_pointer(table, destroy, at(0x399290));
        return;
    }
    VIEWS_STATE.get().enabled = true;
    log!("Conform hooks installed: unit panel refresh/destructor; engine commands only");
    if sys::require_signatures(&MOBO_RVAS) {
        MOBO.get().enabled = true;
        log!("Mobo stacks enabled");
    } else {
        log!("Mobo stacks rejected: guards; no stack can be marked");
    }
    if sys::require_signatures(&REPLACE_RVAS) {
        let r = REPLACING.get();
        r.supports_fn = at(0x565870);
        r.has_fn = at(0x565930);
        r.enabled = true;
        log!("Replace depleted enabled: strength and pop support read only");
    } else {
        log!("Replace depleted rejected: guards; the chooser stays closed");
    }
    // TPL-11 needs the unit list the mobo guards cover.
    if mobo_enabled() && sys::require_signatures(&GATHER_RVAS) {
        let gathering = GATHERING.get();
        gathering.supply_fn = at(0xB3C40);
        gathering.enabled = true;
        log!("Build Army gathering enabled: supply limit, adjacency and rally point read only");
    } else {
        log!("Build Army gathering rejected: guards; Recruit Template recruits without gathering");
    }
}
