//! LAW-2 native Bill (mockup board 2-negotiation): GUI writes and drawing only; port of
//! tgcx_bill.h. Every change to the game goes through the engine's decision click, and
//! each decision's own effect re-checks its conditions (R5; tools/gen_law_catalog.py).

use std::ffi::c_void;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicI32, Ordering};

use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;

use crate::bill_catalog::*;
use crate::d3d::{self, Device, HResult, S_FALSE, S_OK, Viewport, succeeded};
use crate::renderer::{
    self, Chain, DISC_VERTICES, GuiRect, SEAT_VERTICES, append_disc, blend_state, common_state,
    gui_entity, read_politics_rect, seat_geometry, soft_panel, unit_circle,
};
use crate::signatures::IDEOLOGY_CANDIDATES;
use crate::snapshot::{self, IDEOLOGIES, Snapshot};
use crate::sys::{
    Bytes, CpuScope, GameString, Global, Text, at, base, call_one, call_u32, call_zero, guard,
    in_image, rd8, rd32, rdf, rdi, rdp, slot, thread_id,
};
use crate::{log, reforms, text};

// ---------------------------------------------------------------- native GUI helpers

/// The native child lookup in vtable slot `index` of `host`, if it is the one we
/// know (`expected`), for `key` (NUL-terminated).
pub fn find(host: usize, index: usize, expected: u32, key: &[u8]) -> usize {
    let function = slot(host, index);
    if function != at(expected) {
        return 0;
    }
    let name = GameString::borrow(key);
    call_one(function, host, (&raw const name).cast())
}

/// `find` for a catalogue name.
pub fn find_named(host: usize, index: usize, expected: u32, key: &str) -> usize {
    find(
        host,
        index,
        expected,
        Text::<128>::from_latin1(key).with_nul(),
    )
}

// from V2DLL: V2TechButton.cpp, BudgetUpdate string assign/ESI text setter ABI.
pub fn set_text_native(host: usize, name: &[u8], text: &[u8]) {
    let text_box = find(host, 15, 0x5E4D00, name);
    if text_box == 0 {
        return;
    }
    let s = GameString::borrow(text);
    type Assign = unsafe extern "fastcall" fn(usize, usize, *const GameString, u32, u32);
    // SAFETY: the native string assign (signature RVA 0x8ED0).
    unsafe {
        std::mem::transmute::<usize, Assign>(at(0x8ED0))(text_box + 0xDC, 0, &s, 0, 0xFFFFFFFF)
    };
    let inner = rdp(text_box + 0xA0);
    if inner == 0 {
        return;
    }
    // ESI the text object, two stack arguments it pops itself.
    // SAFETY: the native setter (signature RVA 0x65ECC0), as the C++ calls it.
    unsafe {
        std::arch::asm!(
            "push esi",
            "push 0",
            "push ecx",
            "mov esi, edx",
            "call eax",
            "pop esi",
            in("ecx") &raw const s,
            in("edx") inner,
            in("eax") at(0x65ECC0),
            clobber_abi("C"),
        );
    }
}

/// from V2DLL: V2TechButton.cpp, named-button/observable binding pattern. `glue` is our
/// 44-byte observer: Politics' table, the object the native trampoline passes in ECX,
/// and the code it jumps to.
pub fn bind_observer(button: usize, glue: &mut [u8; 44], object: usize, callback: usize) -> bool {
    let observable = button + 0x54;
    let function = slot(observable, 1);
    if !in_image(function) {
        return false;
    }
    glue.fill(0);
    // Politics' 44-byte observer uses +20 for deletion notification; the
    // InGameIdler template reads +38 and is not interchangeable (P31).
    glue[0..4].copy_from_slice(&(at(0xA0E48C) as u32).to_le_bytes());
    glue[4..8].copy_from_slice(&(object as u32).to_le_bytes());
    glue[8..12].copy_from_slice(&(callback as u32).to_le_bytes());
    call_one(function, observable, glue.as_ptr().cast());
    true
}

// from V2DLL: V2TechButton.cpp, MintReadCountryVar. Missing variable is zero.
fn read_var(country: usize, key: &[u8], ok: &mut bool) -> i32 {
    let value = guard(|| {
        let vars = country + 0x1DC;
        let table = rdp(vars);
        if table != base() + 0x9FBB60 || rdp(table + 4) != base() + 0x241BA0 {
            return None;
        }
        let name = GameString::borrow(key);
        let node = call_one(rdp(table + 4), vars, (&raw const name).cast());
        Some(if node != 0 { rdi(node + 0x1C) } else { 0 })
    })
    .flatten();
    value.unwrap_or_else(|| {
        *ok = false;
        0
    })
}

// ---------------------------------------------------------------- timed modifiers

/// Expiry (game hours) of the country's timed modifiers this view needs; 0 = absent.
/// CCountry+0x4D8 is the list has_country_modifier walks (FUN_008CD8C0): node +0 item,
/// +8 next; CTimedModifier +8 modifier, +0xC date (saved as the modifier's date by
/// FUN_0044E340). A modifier's name is a std::string at +0x30.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct BillTimers {
    pub known: bool,
    pub debate: i32,
    pub tick: i32,
    pub cooldown: i32,
    pub debt: [i32; IDEOLOGIES],
    pub broken: [i32; IDEOLOGIES],
}

fn modifier_name(modifier: usize) -> Option<&'static [u8]> {
    let size = rd32(modifier + 0x40) as usize;
    let capacity = rd32(modifier + 0x44) as usize;
    if size > 128 || capacity < size {
        return None;
    }
    let text = if capacity > 15 {
        rdp(modifier + 0x30)
    } else {
        modifier + 0x30
    };
    // SAFETY: inside the caller's guard.
    (text != 0).then(|| unsafe { std::slice::from_raw_parts(text as *const u8, size) })
}

fn timers_inner(country: usize) -> Option<BillTimers> {
    let mut out = BillTimers::default();
    let mut node = rdp(country + 0x4D8);
    let mut n = 0;
    while node != 0 {
        if n >= 4096 {
            return None;
        }
        let item = rdp(node);
        if item == 0 || rdp(item) != base() + 0x9F9978 {
            return None;
        }
        let modifier = rdp(item + 8);
        let raw = if modifier != 0 {
            modifier_name(modifier)?
        } else {
            return None;
        };
        if raw.len() >= 6 && raw.starts_with(b"tgcx_") {
            // Like the C++'s strcmp on a copy: the name ends at a NUL.
            let name = raw.split(|&c| c == 0).next().unwrap_or_default();
            let expiry = rdi(item + 0xC).max(1);
            match name {
                b"tgcx_debate" => out.debate = expiry,
                b"tgcx_favours_tick" => out.tick = expiry,
                b"tgcx_bill_cooldown" => out.cooldown = expiry,
                _ => {
                    for (i, ideology) in IDEOLOGY_CANDIDATES.iter().enumerate() {
                        if name == text!(64, "tgcx_debt_deadline_{ideology}").bytes() {
                            out.debt[i] = expiry;
                        }
                        if name == text!(64, "tgcx_debt_broken_{ideology}").bytes() {
                            out.broken[i] = expiry;
                        }
                    }
                }
            }
        }
        node = rdp(node + 8);
        n += 1;
    }
    out.known = true;
    Some(out)
}

/// The timers, or None (all zero, unknown) when the list cannot be read.
pub fn read_timers(country: usize) -> Option<BillTimers> {
    guard(|| timers_inner(country)).flatten()
}

/// Whether the loaded mods define a modifier: the manager FUN_005661B0 searches
/// (global RVA 0xE5A69C, hash (h+c)*61, bucket node +0 object, +4 next).
fn modifier_defined(key: &str) -> bool {
    guard(|| {
        let manager = rdp(at(0xE5A69C));
        if manager == 0 {
            return false;
        }
        let buckets = rd32(manager + 4);
        let table = rdp(manager + 8);
        if buckets == 0 || buckets > 65536 || table == 0 {
            return false;
        }
        let mut hash = 0u32;
        for &c in key.as_bytes() {
            hash = hash.wrapping_add(c as i8 as i32 as u32).wrapping_mul(61);
        }
        let mut node = rdp(table + (hash % buckets) as usize * 4);
        let mut n = 0;
        while node != 0 && n < 256 {
            let object = rdp(node);
            if object != 0 && modifier_name(object) == Some(key.as_bytes()) {
                return true;
            }
            node = rdp(node + 4);
            n += 1;
        }
        false
    })
    .unwrap_or(false)
}

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const MONTH_LENGTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Game hours to "14 Jan 1836" (the engine's 365-day calendar, FUN_0043ACD0).
fn date(hours: i32, with_day: bool) -> Text<32> {
    let days = hours / 24;
    let year = days / 365 - 5000;
    let mut day = days % 365;
    let mut month = 0;
    while month < 11 && day >= MONTH_LENGTHS[month] {
        day -= MONTH_LENGTHS[month];
        month += 1;
    }
    if with_day {
        text!(32, "{} {} {year}", day + 1, MONTHS[month])
    } else {
        text!(32, "{} {year}", MONTHS[month])
    }
}

// ---------------------------------------------------------------- model

#[derive(Clone, Copy, Default, PartialEq)]
pub struct BillBloc {
    pub seats: i32,
    pub base: i32,
    pub yes: i32,
    pub no: i32,
    pub abstain: i32,
    pub bought: i32,
    pub bought_yes: i32,
    pub opposed: i32,
    pub stance: i32,
    pub promised: bool,
}

const RATE_PARTS: usize = 5;
const RATE_NAMES: [&str; RATE_PARTS] = ["prestige", "house", "order", "franchise", "leader"];

#[derive(Clone, Copy, Default, PartialEq)]
pub struct BillModel {
    pub country: usize,
    pub id: i32,
    pub yes: i32,
    pub no: i32,
    pub abstain: i32,
    pub margin: i32,
    pub quorum: i32,
    pub favours: i32,
    pub rate: i32,
    pub spent: i32,
    pub ticks: i32,
    pub ruling: i32,
    pub government: i32,
    pub rate_part: [i32; RATE_PARTS],
    pub active: bool,
    pub audience: bool,
    pub crown_veto: bool,
    pub crown_signs: bool,
    pub valid: bool,
    pub bloc: [BillBloc; IDEOLOGIES],
}

/// A script variable in whole units (the engine stores thousandths).
pub fn value(country: usize, key: &[u8], ok: &mut bool) -> i32 {
    let raw = read_var(country, key, ok);
    if raw % 1000 != 0 {
        *ok = false;
        return 0;
    }
    raw / 1000
}

/// The Bill's script state; `valid` tells whether it is consistent and active.
pub fn read_model(country: usize) -> BillModel {
    let mut out = BillModel {
        country,
        ..BillModel::default()
    };
    let mut ok = country != 0;
    let v =
        |key: std::fmt::Arguments, ok: &mut bool| value(country, text!(96, "{key}").with_nul(), ok);
    let house = v(format_args!("tgcx_house_total"), &mut ok);
    let schema = v(format_args!("tgcx_gui_schema"), &mut ok);
    out.active = if schema == 1 {
        v(format_args!("tgcx_gui_active"), &mut ok) == 1
    } else {
        schema == 0 && house == 100
    };
    if !ok || !out.active {
        return out;
    }
    out.id = v(format_args!("tgcx_gui_bill_id"), &mut ok);
    out.yes = v(format_args!("tgcx_yes"), &mut ok);
    out.no = v(format_args!("tgcx_no"), &mut ok);
    out.margin = v(format_args!("tgcx_margin"), &mut ok);
    out.quorum = v(format_args!("tgcx_quorum"), &mut ok);
    out.favours = v(format_args!("tgcx_favours"), &mut ok);
    out.rate = v(format_args!("tgcx_rate"), &mut ok);
    out.spent = v(format_args!("tgcx_spent"), &mut ok);
    out.ticks = v(format_args!("tgcx_favours_ticks"), &mut ok);
    out.ruling = v(format_args!("tgcx_gui_ruling"), &mut ok);
    out.government = v(format_args!("tgcx_gui_government"), &mut ok);
    out.audience = v(format_args!("tgcx_gui_crown_audience"), &mut ok) == 1;
    out.crown_veto = v(format_args!("tgcx_gui_crown_veto"), &mut ok) == 1;
    out.crown_signs = v(format_args!("tgcx_gui_crown_signs"), &mut ok) == 1;
    for (p, name) in RATE_NAMES.iter().enumerate() {
        out.rate_part[p] = v(format_args!("tgcx_gui_rate_{name}"), &mut ok);
        if !(-8..=8).contains(&out.rate_part[p]) {
            ok = false;
        }
    }
    let (mut sum, mut aye, mut nay) = (0, 0, 0);
    for (i, ideology) in IDEOLOGY_CANDIDATES.iter().enumerate() {
        let b = &mut out.bloc[i];
        b.seats = v(format_args!("tgcx_seats_{ideology}"), &mut ok);
        b.base = v(format_args!("tgcx_base_yes_{ideology}"), &mut ok);
        b.no = v(format_args!("tgcx_left_{ideology}"), &mut ok);
        b.stance = v(format_args!("tgcx_gui_stance_{ideology}"), &mut ok);
        b.opposed = v(format_args!("tgcx_opposed_{ideology}"), &mut ok);
        b.bought = v(format_args!("tgcx_buy_{ideology}_total"), &mut ok);
        b.bought_yes = v(format_args!("tgcx_buy_{ideology}_yes"), &mut ok);
        b.promised = v(format_args!("tgcx_gui_promised_{ideology}"), &mut ok) == 1;
        if !(0..=100).contains(&b.seats)
            || b.seats % 5 != 0
            || b.base < 0
            || b.base > b.seats
            || b.base % 5 != 0
            || b.no < 0
            || b.no > b.seats
            || b.no % 5 != 0
            || !(0..=100).contains(&b.bought)
            || b.bought_yes < 0
            || b.bought_yes > b.bought
            || !(-2..=2).contains(&b.stance)
            || !(0..=100).contains(&b.opposed)
        {
            ok = false;
        }
        let opposed = (b.seats - b.base).max(0);
        b.abstain = if b.promised {
            0
        } else {
            b.bought.min(opposed) - b.bought_yes.min(opposed)
        };
        b.yes = b.seats - b.no - b.abstain;
        if b.yes < 0 || b.yes > b.seats || b.abstain < 0 || b.abstain > b.seats {
            ok = false;
        }
        sum += b.seats;
        aye += b.yes;
        nay += b.no;
    }
    out.abstain = 100 - out.yes - out.no;
    out.valid = ok
        && house == 100
        && sum == 100
        && aye == out.yes
        && nay == out.no
        && out.yes >= 0
        && out.no >= 0
        && out.abstain >= 0
        && out.margin == out.yes - out.no
        && out.quorum == out.yes + out.no
        && out.id >= 0
        && out.id <= BILL_CATALOG.len() as i32
        && (0..=100).contains(&out.favours)
        && (0..=8).contains(&out.rate)
        && out.spent >= 0
        && out.ticks >= 0
        && out.ruling >= 0
        && out.ruling <= IDEOLOGIES as i32
        && out.government >= 0
        && out.government <= BILL_GOVERNMENTS.len() as i32;
    out
}

// ---------------------------------------------------------------- view plan

const CARDS: usize = BILL_CARDS as usize;

/// One button: the decision it sends, its price and whether it is shown/usable.
/// The DLL only greys out what the script would refuse; the script decides.
#[derive(Clone, Copy, Default)]
pub struct BillAction {
    decision: Text<64>,
    price: i32,
    visible: bool,
    enabled: bool,
}

#[derive(Clone, Copy, Default)]
pub struct BillCardPlan {
    a: BillAction,
    b: BillAction,
    note: Option<&'static str>,
}

#[derive(Clone, Copy, Default)]
pub struct BillView {
    slots: usize,
    slot: [usize; CARDS],
    pages: u32,
    page: u32,
    card: [BillCardPlan; CARDS],
    audience: BillAction,
    count: BillAction,
    withdraw: BillAction,
    vote: BillAction,
}

fn action(decision: &str, price: i32, visible: bool, enabled: bool) -> BillAction {
    BillAction {
        decision: Text::from_latin1(decision),
        price,
        visible,
        enabled,
    }
}

fn price(table: &[i32; 5], stance: i32, lot: i32, broken: bool) -> i32 {
    table[(stance + 2) as usize] * lot / 5 * if broken { BILL_BROKEN_FACTOR } else { 1 }
}

fn plan_card(m: &BillModel, t: &BillTimers, i: usize, lot: i32) -> BillCardPlan {
    let mut out = BillCardPlan::default();
    let b = &m.bloc[i];
    let code = BILL_STANCE_CODES[(b.stance + 2) as usize];
    let broken = t.broken[i] != 0;
    if m.id == 0 {
        out.note = Some("Count the votes to identify this bill.");
        return out;
    }
    if b.promised {
        out.b = action("", 0, true, false);
        return out;
    }
    let suffix = if lot == 25 { "_25" } else { "" };
    let twin = if broken { "_x2" } else { "" };
    let ideology = IDEOLOGY_CANDIDATES[i];
    if b.stance <= -1 && b.no > 0 {
        let a = &mut out.a;
        a.visible = true;
        a.price = price(&BILL_ABSTAIN_PRICE, b.stance, lot, broken);
        a.decision = text!(64, "tgcx_buy_abstain_{ideology}_{code}{suffix}{twin}");
        a.enabled = m.favours >= a.price && b.no >= lot;
    }
    if b.stance == -2 {
        // Securing every opposing seat needs one not already bought to vote Aye.
        let p = &mut out.b;
        p.visible = true;
        p.decision = text!(64, "tgcx_promise_{ideology}");
        p.enabled = (!t.known || t.debt[i] == 0) && b.opposed > b.bought_yes;
    } else if b.no > 0 {
        let v = &mut out.b;
        v.visible = true;
        v.price = price(&BILL_AYE_PRICE, b.stance, lot, broken);
        v.decision = text!(64, "tgcx_buy_vote_{ideology}_{code}{suffix}{twin}");
        v.enabled = m.favours >= v.price && b.no >= lot;
    }
    if !out.a.visible && !out.b.visible {
        out.note = Some(if b.yes == b.seats {
            "Votes Aye on its own: nothing to buy."
        } else {
            "No seats left to buy."
        });
    }
    out
}

/// Sort keys (header buttons): 0 seats, 1 most opposed first, 2 most friendly first
/// (the board's order), 3 most Nay. Clicking the active key pages on.
fn order(m: &BillModel, sort: i32) -> Vec<usize> {
    let mut order: Vec<usize> = (0..IDEOLOGIES).filter(|&i| m.bloc[i].seats != 0).collect();
    let key = |i: usize| {
        let b = &m.bloc[i];
        match sort {
            1 => -b.stance,
            2 => b.stance,
            3 => b.no,
            _ => b.seats,
        }
    };
    for x in 1..order.len() {
        for y in (1..=x).rev() {
            let (p, q) = (order[y - 1], order[y]);
            let swap = key(q) > key(p)
                || (key(q) == key(p)
                    && (m.bloc[q].seats > m.bloc[p].seats
                        || (m.bloc[q].seats == m.bloc[p].seats && q < p)));
            if !swap {
                break;
            }
            order[y - 1] = q;
            order[y] = p;
        }
    }
    order
}

fn build_view(m: &BillModel, t: &BillTimers, sort: i32, page: u32, lot: i32) -> BillView {
    let mut out = BillView::default();
    let order = order(m, sort);
    let n = order.len();
    out.pages = 1.max(n.div_ceil(CARDS)) as u32;
    out.page = page % out.pages;
    let first = out.page as usize * CARDS;
    for s in 0..CARDS {
        if first + s >= n {
            break;
        }
        out.slot[s] = order[first + s];
        out.slots += 1;
        out.card[s] = plan_card(m, t, out.slot[s], lot);
    }
    out.audience = action(
        "tgcx_audience",
        BILL_AUDIENCE_PRICE,
        m.crown_veto,
        m.crown_veto && m.id > 0 && !m.audience && m.favours >= BILL_AUDIENCE_PRICE,
    );
    out.count = action("tgcx_count_votes", 0, true, true);
    out.withdraw = action("tgcx_withdraw", 0, true, true);
    out.vote = action("tgcx_call_vote", 0, true, !t.known || t.debate == 0);
    out
}

// ---------------------------------------------------------------- native window state

/// Window buttons + Politics' Open Bill.
const ACTIONS: usize = B_COUNT_OF + 1;
const OPEN_ACTION: usize = B_COUNT_OF;
const _: () = assert!(
    ACTIONS == crate::thunks::BILL_THUNKS,
    "one native observer stub per Bill button"
);

pub struct Bill {
    pub enabled: bool,
    pub open: bool,
    was_active: bool,
    pub dirty: bool,
    /// Our GUI is loaded and the screen fits the Bill: the Reforms tab and the
    /// Decisions list may then rely on it (reforms.rs).
    pub usable: bool,
    pub size_known: bool,
    msg_suppress_tick: u32,
    pub msg_hooked: bool,
    root: usize,
    window: usize,
    pub tick: u32,
    pub thread: u32,
    model: BillModel,
    timers: BillTimers,
    view: BillView,
    mode: i32,
    lot: i32,
    sort: i32,
    page: u32,
    glue: [[u8; 44]; ACTIONS],
    bound: [bool; ACTIONS],
    decision_fn: usize,
    draws: u32,
    texts: [Text<160>; T_COUNT_OF],
    text_set: [bool; T_COUNT_OF],
    /// Negotiation log: observed changes, stamped with the game date (session only).
    log: [Text<128>; 4],
    log_count: usize,
    logged_bound: u32,
}

pub static BILL: Global<Bill> = Global::new(Bill {
    enabled: false,
    open: false,
    was_active: false,
    dirty: true,
    usable: false,
    size_known: false,
    msg_suppress_tick: 0,
    msg_hooked: false,
    root: 0,
    window: 0,
    tick: 0,
    thread: 0,
    model: BillModel {
        country: 0,
        id: 0,
        yes: 0,
        no: 0,
        abstain: 0,
        margin: 0,
        quorum: 0,
        favours: 0,
        rate: 0,
        spent: 0,
        ticks: 0,
        ruling: 0,
        government: 0,
        rate_part: [0; RATE_PARTS],
        active: false,
        audience: false,
        crown_veto: false,
        crown_signs: false,
        valid: false,
        bloc: [BillBloc {
            seats: 0,
            base: 0,
            yes: 0,
            no: 0,
            abstain: 0,
            bought: 0,
            bought_yes: 0,
            opposed: 0,
            stance: 0,
            promised: false,
        }; IDEOLOGIES],
    },
    timers: BillTimers {
        known: false,
        debate: 0,
        tick: 0,
        cooldown: 0,
        debt: [0; IDEOLOGIES],
        broken: [0; IDEOLOGIES],
    },
    view: BillView {
        slots: 0,
        slot: [0; CARDS],
        pages: 0,
        page: 0,
        card: [BillCardPlan {
            a: EMPTY_ACTION,
            b: EMPTY_ACTION,
            note: None,
        }; CARDS],
        audience: EMPTY_ACTION,
        count: EMPTY_ACTION,
        withdraw: EMPTY_ACTION,
        vote: EMPTY_ACTION,
    },
    mode: 2,
    lot: 25,
    sort: 2,
    page: 0,
    glue: [[0; 44]; ACTIONS],
    bound: [false; ACTIONS],
    decision_fn: 0,
    draws: 0,
    texts: [Text::new(); T_COUNT_OF],
    text_set: [false; T_COUNT_OF],
    log: [Text::new(); 4],
    log_count: 0,
    logged_bound: 0,
});

const EMPTY_ACTION: BillAction = BillAction {
    decision: Text::new(),
    price: 0,
    visible: false,
    enabled: false,
};

/// Decisions the DLL sent whose "decision taken" message is still to come (reforms.rs
/// reads it from assembly).
pub static MSG_SUPPRESS: AtomicI32 = AtomicI32::new(0);

/// Politics passed its gate: the Bill runs and sends decisions through FUN_006DCE10.
pub fn enable(decision_fn: usize) {
    let b = BILL.get();
    b.enabled = true;
    b.decision_fn = decision_fn;
}

fn log_line(hours: u32, text: &str) {
    let b = BILL.get();
    if b.log_count == 4 {
        b.log.copy_within(1..4, 0);
        b.log_count -= 1;
    }
    let date = date(hours as i32, true);
    b.log[b.log_count] = text!(128, "{date}: {text}");
    b.log_count += 1;
}

fn observe(old: &BillModel, now: &BillModel, timers: &BillTimers, hours: u32) {
    let mut favours = now.spent - old.spent;
    let mut price = || -> String {
        if favours <= 0 {
            return String::new();
        }
        let cost = format!(" ({favours} favour{})", if favours == 1 { "" } else { "s" });
        favours = 0;
        cost
    };
    for i in 0..IDEOLOGIES {
        let (a, b) = (&old.bloc[i], &now.bloc[i]);
        if b.promised && !a.promised {
            let due = if timers.debt[i] != 0 {
                date(timers.debt[i], false).to_string()
            } else {
                String::new()
            };
            let before = if due.is_empty() { "" } else { " before " };
            log_line(
                hours,
                &format!(
                    "{} promised a law of their choice{before}{due}.",
                    BILL_BLOC_NAMES[i]
                ),
            );
            continue;
        }
        let abstain = (b.bought - b.bought_yes) - (a.bought - a.bought_yes);
        let aye = b.bought_yes - a.bought_yes;
        if abstain > 0 {
            let p = price();
            log_line(
                hours,
                &format!("{abstain} {} seats will abstain{p}.", BILL_BLOC_SINGULAR[i]),
            );
        }
        if aye > 0 {
            let p = price();
            log_line(
                hours,
                &format!("{aye} {} seats will vote Aye{p}.", BILL_BLOC_SINGULAR[i]),
            );
        }
    }
    if now.audience && !old.audience {
        let p = price();
        log_line(hours, &format!("The Crown granted an audience{p}."));
    }
}

/// The native foreground method detaches the child's render subtree from Politics.
/// Its position setter and the chart then share screen coordinates.
fn foreground(window: usize) -> bool {
    if slot(window, 11) != base() + 0x5E2FC0 || slot(window, 6) != base() + 0x5E3BA0 {
        return false;
    }
    let renderer = rdp(window + 0x90);
    if renderer == 0 {
        return false;
    }
    let (width, height) = (rdi(renderer + 0x6B9BC), rdi(renderer + 0x6B9C0));
    if !(BILL_WIDTH..=16384).contains(&width) || !(BILL_HEIGHT..=16384).contains(&height) {
        return false;
    }
    call_zero(slot(window, 11), window);
    // Native GUI positions are screen pixels; its render transform uses a centred
    // coordinate system. The packed setter supplies that conversion.
    let position = u32::from(((width - BILL_WIDTH) / 2) as u16)
        | (u32::from(((height - BILL_HEIGHT) / 2) as u16) << 16);
    call_u32(slot(window, 6), window, position);
    true
}

fn screen_rect(window: usize, screen: &GuiRect) -> Option<GuiRect> {
    let inner = rdp(window + 0x494);
    if inner == 0 || rdp(inner) != base() + 0xA38350 {
        return None;
    }
    // Live 0.3.2: +FC/+100 are centred render inputs, not screen pixels.
    // The world matrix translation is screen X / negative screen Y.
    let x = rdf(inner + 0x84);
    let y = -rdf(inner + 0x88);
    let fits = x >= 0.0
        && y >= 0.0
        && x + BILL_WIDTH as f32 <= screen.width as f32
        && y + BILL_HEIGHT as f32 <= screen.height as f32;
    fits.then_some(GuiRect {
        left: x as i32,
        top: y as i32,
        width: screen.width,
        height: screen.height,
    })
}

fn visibility(show: bool) {
    let window = BILL.get().window;
    if window == 0 {
        return;
    }
    let index = if show { 8 } else { 9 };
    if slot(window, index) != base() + if show { 0x5E4450 } else { 0x5E4030 } {
        return;
    }
    // Raise only on opening: repeated ticks must not put the Bill above a
    // subsequently opened event/report or change native tooltip ordering.
    if show && rd8(window + 0x67) == 0 && !foreground(window) {
        return;
    }
    call_zero(slot(window, index), window);
}

pub fn reset(destroyed: bool) {
    let _ = guard(|| visibility(false));
    let b = BILL.get();
    b.open = false;
    b.was_active = false;
    b.model.valid = false;
    b.dirty = true;
    if destroyed {
        b.root = 0;
        b.window = 0;
        b.bound = [false; ACTIONS];
        b.text_set = [false; T_COUNT_OF];
    }
}

static DISPATCH_TEXT: Global<Text<96>> = Global::new(Text::new());

/// Generic guarded dispatch: the native Politics decision click with our own decision
/// name. FUN_006DCE10 does not test potential/allow, so every decision the Bill sends
/// checks them in its effect (LAW-2, gen_law_catalog.py).
pub fn dispatch(decision: &[u8]) -> bool {
    let b = BILL.get();
    if b.decision_fn == 0 || !decision.starts_with(b"tgcx_") || decision.len() > 63 {
        return false;
    }
    if !decision
        .iter()
        .all(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
    {
        return false;
    }
    // from V2DLL: V2TechButton.cpp, FireDecision fake element and native prefix.
    let text = DISPATCH_TEXT.get();
    *text = text!(96, "POLITICSVIEW_DECISION{}", Bytes(decision));
    let mut element = [0u8; 0x30];
    let name = GameString::borrow(text.with_nul());
    // SAFETY: GameString is 24 plain bytes; the element has room at +0x14.
    unsafe {
        std::ptr::copy_nonoverlapping(
            (&raw const name).cast::<u8>(),
            element.as_mut_ptr().add(0x14),
            24,
        )
    };
    if b.msg_hooked {
        if MSG_SUPPRESS.load(Ordering::SeqCst) < 4 {
            MSG_SUPPRESS.fetch_add(1, Ordering::SeqCst);
        }
        b.msg_suppress_tick = crate::sys::tick();
    }
    type Decision = unsafe extern "system" fn(*mut c_void);
    // SAFETY: FUN_006DCE10, __stdcall(element), checked by the drawing+laws gate.
    unsafe { std::mem::transmute::<usize, Decision>(b.decision_fn)(element.as_mut_ptr().cast()) };
    true
}

fn send(action: &BillAction, tag: Bytes) -> bool {
    if !action.visible || !action.enabled || !dispatch(action.decision.bytes()) {
        return false;
    }
    BILL.get().tick = 0;
    log!(
        "Bill {} requested through native decision handler tag={tag}",
        action.decision
    );
    true
}

/// Native observer callbacks run in the game's GUI thread, preserving CPU state.
/// Every click re-reads the script state; the view it acts on is rebuilt fresh.
fn click_inner(action: usize) {
    if action == B_CLOSE {
        BILL.get().open = false;
        visibility(false);
        return;
    }
    let Some(sample) = snapshot::snapshot() else {
        return;
    };
    let fresh = read_model(sample.country);
    if !fresh.valid {
        return;
    }
    let timers = read_timers(sample.country).unwrap_or_default();
    let b = BILL.get();
    b.dirty = true;
    b.tick = 0;
    if action == OPEN_ACTION {
        b.open = true;
        visibility(true);
        return;
    }
    if (B_SORT_0..=B_SORT_3).contains(&action) {
        let sort = (action - B_SORT_0) as i32;
        if sort == b.sort {
            b.page += 1;
        } else {
            b.sort = sort;
            b.page = 0;
        }
        return;
    }
    if (B_MODE_0..=B_MODE_2).contains(&action) {
        b.mode = (action - B_MODE_0) as i32;
        return;
    }
    if action == B_LOT_0 || action == B_LOT_1 {
        b.lot = if action == B_LOT_0 { 5 } else { 25 };
        return;
    }
    let view = build_view(&fresh, &timers, b.sort, b.page, b.lot);
    let tag = sample.tag();
    if (B_CARD_0_A..=B_CARD_2_B).contains(&action) {
        let slot = (action - B_CARD_0_A) / 2;
        let second = !(action - B_CARD_0_A).is_multiple_of(2);
        // A card that now shows another bloc was stale: refresh, send nothing.
        if slot >= view.slots || b.view.slots == 0 || view.slot[slot] != b.view.slot[slot] {
            return;
        }
        send(
            if second {
                &view.card[slot].b
            } else {
                &view.card[slot].a
            },
            tag,
        );
        return;
    }
    match action {
        B_AUDIENCE => send(&view.audience, tag),
        B_COUNT => send(&view.count, tag),
        B_WITHDRAW => send(&view.withdraw, tag),
        B_VOTE => send(&view.vote, tag),
        _ => false,
    };
}

extern "C" fn bill_click(action: usize) {
    let _cpu = CpuScope::new();
    let b = BILL.get();
    if !b.enabled || thread_id() != b.thread || renderer::politics() == 0 {
        return;
    }
    if guard(|| click_inner(action)).is_none() {
        log!("Bill click rejected: native GUI/command read fault");
    }
}

/// Every Bill stub pushes its action and jumps here: `bill_click(action)` with all
/// registers and the FPU preserved, then back to the native caller.
#[unsafe(naked)]
pub unsafe extern "C" fn bill_thunk_common() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+36]",
        "call {click}",
        "add esp, 4",
        crate::restore_cpu!(),
        "lea esp, [esp+4]",
        "ret",
        click = sym bill_click,
    )
}

fn bind(host: usize, name: &str, action: usize) {
    let b = BILL.get();
    if b.bound[action] {
        return;
    }
    let button = find_named(host, 13, 0x5E4C80, name);
    if button == 0 {
        return;
    }
    // SAFETY: the stub table build.rs generated, one per action.
    let stub = unsafe { crate::thunks::tgcx_bill_table[action] };
    let politics = renderer::politics();
    if bind_observer(button, &mut BILL.get().glue[action], politics, stub) {
        BILL.get().bound[action] = true;
    }
}

/// Text writes are cached: the setter runs only when a label changes.
fn put(id: usize, text: &[u8]) {
    let b = BILL.get();
    let text = Text::<160>::from_bytes(text);
    if b.text_set[id] && b.texts[id].bytes() == text.bytes() {
        return;
    }
    b.texts[id] = text;
    b.text_set[id] = true;
    let window = b.window;
    set_text_native(
        window,
        Text::<128>::from_latin1(BILL_TEXT_NAMES[id]).with_nul(),
        text.with_nul(),
    );
}

fn putf(id: usize, args: std::fmt::Arguments) {
    let t = text!(160, "{args}");
    put(id, t.bytes());
}

fn puts(id: usize, s: &str) {
    put(id, Text::<160>::from_latin1(s).bytes());
}

fn button_label(id: usize, action: &BillAction, label: &str) {
    puts(id, if action.visible { label } else { "" });
}

/// The three option rows a card shows for the bill's law group.
fn option_window(law: &BillInfo) -> (usize, &'static BillGroup) {
    let g = &BILL_GROUPS[law.group as usize];
    let lo = law.from_index.min(law.to_index) as usize;
    let hi = law.from_index.max(law.to_index) as usize;
    let count = g.count as usize;
    let mut start = if count <= 3 {
        0
    } else {
        (count - 3).min(lo.saturating_sub(1))
    };
    if hi >= start + 3 {
        start = hi - 2;
    }
    (start, g)
}

fn labels(m: &BillModel, t: &BillTimers, v: &BillView) {
    let lot = BILL.get().lot;
    let law = (m.id > 0).then(|| &BILL_CATALOG[(m.id - 1) as usize]);
    puts(T_LAW, law.map_or("Active bill", |l| l.title));
    // A transition too wide for vic_18 goes to the Arial12 bold line in the same place.
    let compact = law.is_some_and(|l| l.transition_small);
    match law {
        Some(l) => putf(
            if compact {
                T_TRANSITION_SMALL
            } else {
                T_TRANSITION
            },
            format_args!("{} \u{BB} {}", l.transition_from, l.to),
        ),
        None => puts(T_TRANSITION, "Count once to identify the bill."),
    }
    puts(
        if compact {
            T_TRANSITION
        } else {
            T_TRANSITION_SMALL
        },
        "",
    );
    let by = (m.ruling > 0).then(|| BILL_BLOC_NAMES[(m.ruling - 1) as usize]);
    if t.debate > 1 {
        let d = date(t.debate - BILL_DEBATE_DAYS * 24, true);
        match by {
            Some(by) => putf(T_INTRO, format_args!("Introduced {d} by the {by}")),
            None => putf(T_INTRO, format_args!("Introduced {d}")),
        }
    } else if t.known {
        puts(T_INTRO, "Debate over: the vote can be called");
    } else {
        match by {
            Some(by) => putf(T_INTRO, format_args!("Introduced by the {by}")),
            None => puts(T_INTRO, "Under debate"),
        }
    }
    if v.pages > 1 {
        putf(T_SORT_0, format_args!("Bloc {}/{}", v.page + 1, v.pages));
    } else {
        puts(T_SORT_0, "Bloc");
    }
    putf(T_LEGEND_0, format_args!("{} Aye", m.yes));
    putf(T_LEGEND_1, format_args!("{} Abstain", m.abstain));
    putf(T_LEGEND_2, format_args!("{} Nay", m.no));
    puts(T_CHART_NOTE, "TGC starting votes + negotiated seats.");
    putf(T_FAVOURS, format_args!("{}", m.favours));
    putf(T_RATE, format_args!("\u{A7}G+{} per month\u{A7}!", m.rate));
    putf(T_SPENT, format_args!("Spent on this bill: {}", m.spent));
    let sources: Vec<&str> = (0..RATE_PARTS)
        .filter(|&p| m.rate_part[p] > 0)
        .map(|p| RATE_NAMES[p])
        .collect();
    if sources.is_empty() {
        puts(T_SOURCES, "From the base rate");
    } else {
        putf(T_SOURCES, format_args!("From {}", sources.join(", ")));
    }
    // Price table at the chosen lot (catalogue friendly Aye = the undecided price).
    puts(T_TABLE_1_ABSTAIN, "\u{96}");
    putf(
        T_TABLE_1_AYE,
        format_args!("{}", price(&BILL_AYE_PRICE, 0, lot, false)),
    );
    putf(
        T_TABLE_2_ABSTAIN,
        format_args!("{}", price(&BILL_ABSTAIN_PRICE, -1, lot, false)),
    );
    putf(
        T_TABLE_2_AYE,
        format_args!("{}", price(&BILL_AYE_PRICE, -1, lot, false)),
    );
    putf(
        T_TABLE_3_ABSTAIN,
        format_args!("{}", price(&BILL_ABSTAIN_PRICE, -2, lot, false)),
    );
    puts(T_TABLE_3_AYE, "promise");
    // Debts: one entry in the board's long form, several one line each.
    let mut lines: [Text<64>; 4] = [Text::new(); 4];
    let mut count = 0;
    let entries: usize = (0..IDEOLOGIES)
        .map(|i| usize::from(t.debt[i] != 0) + usize::from(t.broken[i] != 0))
        .sum();
    for i in 0..IDEOLOGIES {
        if count >= 4 {
            break;
        }
        if t.debt[i] != 0 {
            let d = date(t.debt[i], false);
            if entries == 1 {
                lines[0] = text!(64, "{}: enact a law they want", BILL_BLOC_NAMES[i]);
                lines[1] = text!(64, "before {d}. If not: +{BILL_BROKEN_MILITANCY}");
                lines[2] = text!(64, "militancy in their pops, and their");
                lines[3] = text!(64, "votes cost double for {BILL_BROKEN_YEARS} years.");
                count = 4;
            } else {
                lines[count] = text!(64, "{}: law owed before {d}", BILL_BLOC_NAMES[i]);
                count += 1;
            }
        }
        if t.broken[i] != 0 && count < 4 {
            let d = date(t.broken[i], false);
            if entries == 1 {
                lines[0] = text!(64, "{}: a promise was broken.", BILL_BLOC_NAMES[i]);
                lines[1] = text!(64, "Their votes cost double until {d}.");
                count = 2;
            } else {
                lines[count] = text!(64, "{}: double price until {d}", BILL_BLOC_NAMES[i]);
                count += 1;
            }
        }
    }
    if !t.known {
        lines[0] = text!(64, "Debts are listed under Decisions.");
    } else if entries == 0 {
        lines[0] = text!(64, "No promises owed.");
    }
    for (l, line) in lines.iter().enumerate() {
        put(T_DEBT_0 + l, line.bytes());
    }
    // Bloc cards.
    let stride = T_CARD_1_NAME - T_CARD_0_NAME;
    for s in 0..CARDS {
        let at = |field: usize| T_CARD_0_NAME + s * stride + field - T_CARD_0_NAME;
        if s >= v.slots {
            for f in T_CARD_0_NAME..T_CARD_1_NAME {
                puts(at(f), "");
            }
            continue;
        }
        let i = v.slot[s];
        let b = &m.bloc[i];
        let c = &v.card[s];
        puts(at(T_CARD_0_NAME), BILL_BLOC_SINGULAR[i]);
        putf(at(T_CARD_0_SEATS), format_args!("{} seats", b.seats));
        putf(
            at(T_CARD_0_NATIVE),
            format_args!("TGC: {} Aye, {} Nay", b.base, b.seats - b.base),
        );
        for r in 0..3 {
            let mut option = "";
            if let Some(law) = law {
                let (start, g) = option_window(law);
                if start + r < g.count as usize {
                    option = g.short_options[start + r];
                }
            }
            puts(at(T_CARD_0_OPTION_0) + r, option);
        }
        const STANCE_NOTE: [&str; 5] = [
            "\u{96}2, strongly",
            "\u{96}1",
            "undecided",
            "+1",
            "+2, strongly",
        ];
        if b.promised {
            puts(at(T_CARD_0_STANCE), "\u{A7}GPromised\u{A7}!");
            puts(at(T_CARD_0_STANCE_NOTE), "law owed");
        } else {
            let stance = if m.id == 0 {
                "Count"
            } else if b.stance > 0 {
                "\u{A7}GFor\u{A7}!"
            } else if b.stance < 0 {
                "\u{A7}RAgainst\u{A7}!"
            } else {
                "Undecided"
            };
            puts(at(T_CARD_0_STANCE), stance);
            puts(
                at(T_CARD_0_STANCE_NOTE),
                if m.id != 0 {
                    STANCE_NOTE[(b.stance + 2) as usize]
                } else {
                    ""
                },
            );
        }
        putf(
            at(T_CARD_0_AYE),
            format_args!("\u{A7}G{} Aye\u{A7}!", b.yes),
        );
        putf(at(T_CARD_0_ABSTAIN), format_args!("{} Abstain", b.abstain));
        putf(at(T_CARD_0_NAY), format_args!("\u{A7}R{} Nay\u{A7}!", b.no));
        puts(at(T_CARD_0_BOUGHT_LABEL), "Bought");
        if b.promised {
            puts(at(T_CARD_0_BOUGHT), "a promise");
        } else if b.bought != 0 {
            putf(
                at(T_CARD_0_BOUGHT),
                format_args!("{} abstain, {} aye", b.bought - b.bought_yes, b.bought_yes),
            );
        } else {
            puts(
                at(T_CARD_0_BOUGHT),
                if b.no != 0 {
                    "nothing yet"
                } else {
                    "nothing needed"
                },
            );
        }
        puts(at(T_CARD_0_NOTE), c.note.unwrap_or(""));
        button_label(
            at(T_CARD_0_A),
            &c.a,
            &format!("Abstain {lot} ({})", c.a.price),
        );
        let label = if b.promised {
            "Law promised".to_string()
        } else if b.stance == -2 {
            "Promise a law".to_string()
        } else {
            format!("Aye {lot} ({})", c.b.price)
        };
        button_label(at(T_CARD_0_B), &c.b, &label);
    }
    // The Crown.
    let government = if m.government > 0 {
        BILL_GOVERNMENTS[(m.government - 1) as usize]
    } else {
        ""
    };
    puts(T_CROWN_NAME, government);
    if !m.crown_veto {
        puts(T_CROWN_DESC_0, "The Crown has no veto");
        puts(T_CROWN_DESC_1, "under this government.");
        puts(T_CROWN_STANCE, "");
        puts(T_CROWN_STANCE_NOTE, "");
        puts(T_CROWN_VETO_0, "Cannot veto");
        puts(T_CROWN_VETO_1, "");
    } else {
        let ruling = (m.ruling > 0).then(|| (m.ruling - 1) as usize);
        let distrust = ruling.is_some_and(|r| BILL_DISTRUSTED[r]);
        match ruling {
            Some(r) => {
                let trust = if distrust { "Distrusts" } else { "Trusts" };
                putf(
                    T_CROWN_DESC_0,
                    format_args!("{trust} the {}:", BILL_BLOC_NAMES[r]),
                );
                if distrust {
                    putf(
                        T_CROWN_DESC_1,
                        format_args!("starts against {} bills", BILL_BLOC_SINGULAR[r]),
                    );
                } else {
                    putf(
                        T_CROWN_DESC_1,
                        format_args!("signs {} bills", BILL_BLOC_SINGULAR[r]),
                    );
                }
            }
            None => {
                puts(T_CROWN_DESC_0, "");
                puts(T_CROWN_DESC_1, "");
            }
        }
        puts(
            T_CROWN_STANCE,
            if m.crown_signs {
                "\u{A7}GFor\u{A7}!"
            } else {
                "\u{A7}RAgainst\u{A7}!"
            },
        );
        puts(
            T_CROWN_STANCE_NOTE,
            if m.crown_signs {
                "would sign"
            } else {
                "would veto"
            },
        );
        puts(T_CROWN_VETO_0, "May veto");
        puts(T_CROWN_VETO_1, "(monarchy)");
    }
    puts(T_AUDIENCE_LABEL, if m.crown_veto { "Audience" } else { "" });
    puts(
        T_AUDIENCE_STATE,
        if !m.crown_veto {
            ""
        } else if m.audience {
            "granted"
        } else {
            "not asked"
        },
    );
    button_label(
        T_AUDIENCE,
        &v.audience,
        &format!("Audience ({BILL_AUDIENCE_PRICE})"),
    );
    let b = BILL.get();
    for l in 0..4 {
        let line = if l < b.log_count {
            b.log[l]
        } else {
            Text::new()
        };
        put(T_LOG_0 + l, line.bytes());
    }
    let status = if m.quorum < 50 {
        format!("No quorum: only {} seats would vote.", m.quorum)
    } else if m.margin > 0 {
        let veto = if m.crown_veto && !m.crown_signs {
            ", but the Crown would veto."
        } else {
            "."
        };
        format!("Would pass {} to {}{veto}", m.yes, m.no)
    } else if m.margin < 0 {
        format!("Would fail {} to {}.", m.yes, m.no)
    } else {
        format!("Tied {} to {}: the bill would fail.", m.yes, m.no)
    };
    putf(T_STATUS, format_args!("\u{A7}Y{status}\u{A7}!"));
    if t.debate > 1 && !v.vote.enabled {
        putf(T_VOTE, format_args!("Vote from {}", date(t.debate, true)));
    } else {
        puts(T_VOTE, "Call the Vote");
    }
}

fn tick_inner() {
    let Some(gui) = read_politics_rect() else {
        visibility(false);
        return;
    };
    let politics = renderer::politics();
    let root = rdp(politics + 0x4C);
    let b = BILL.get();
    if root != b.root {
        b.root = root;
        b.window = 0;
        b.bound = [false; ACTIONS];
        b.thread = thread_id();
        b.was_active = false;
    }
    if b.window == 0 {
        // A DLL with a script-only submod should not ask the native
        // window lookup for a missing child (that prints a game warning).
        if find_named(root, 13, 0x5E4C80, "tgcx_bill_open") == 0 {
            return;
        }
        let window = find_named(root, 27, 0x5E4A20, "tgcx_bill_window");
        let b = BILL.get();
        b.window = window;
        if window == 0 {
            return;
        }
        b.text_set = [false; T_COUNT_OF];
        b.dirty = true;
        visibility(false);
        log!("Bill child window found root={root:08X} child={window:08X}");
    }
    bind(root, "tgcx_bill_open", OPEN_ACTION);
    let window = BILL.get().window;
    for (n, button) in BILL_BUTTONS.iter().enumerate() {
        bind(window, button.name, n);
    }
    let b = BILL.get();
    let bound = b.bound.iter().filter(|&&on| on).count() as u32;
    if bound != b.logged_bound {
        b.logged_bound = bound;
        log!("Bill native buttons bound={bound} of {ACTIONS}");
    }
    b.usable = gui.width >= BILL_WIDTH && gui.height >= BILL_HEIGHT;
    b.size_known = true;
    let sample = snapshot::snapshot();
    let model = sample.map(|s| read_model(s.country)).unwrap_or_default();
    let valid = sample.is_some() && model.valid;
    reforms::refresh(sample.map_or(0, |s| s.country), &model, valid);
    reforms::after_tick(politics);
    let b = BILL.get();
    let (Some(sample), true) = (sample, valid) else {
        b.model.valid = false;
        b.open = false;
        b.was_active = false;
        b.log_count = 0;
        visibility(false);
        return;
    };
    let timers = read_timers(sample.country).unwrap_or_default();
    let old = b.model;
    if !old.valid || old.country != model.country || old.id != model.id {
        if !old.valid || old.country != model.country {
            b.log_count = 0;
        }
        if old.country != model.country {
            b.page = 0;
        }
    } else {
        observe(&old, &model, &timers, sample.date_hours);
    }
    let b = BILL.get();
    let changed = b.dirty || !old.valid || old != model || b.timers != timers;
    b.model = model;
    b.timers = timers;
    if !b.was_active {
        b.open = true;
    }
    b.was_active = true;
    if changed {
        b.view = build_view(&model, &timers, b.sort, b.page, b.lot);
        b.page = b.view.page;
        b.dirty = false;
        let view = b.view;
        labels(&model, &timers, &view);
    }
    let b = BILL.get();
    visibility(b.open && gui.width >= BILL_WIDTH && gui.height >= BILL_HEIGHT);
}

pub fn tick() {
    let b = BILL.get();
    if !b.enabled || renderer::politics() == 0 {
        return;
    }
    let now = crate::sys::tick();
    if b.tick != 0 && now.wrapping_sub(b.tick) < 250 {
        return;
    }
    b.tick = now;
    if guard(tick_inner).is_none() {
        let _ = guard(|| visibility(false));
        let b = BILL.get();
        b.model.valid = false;
        b.enabled = false;
        log!("Bill disabled after native GUI read fault; Decisions remain available");
    }
}

// ---------------------------------------------------------------- automatic favours

// Collect favours like diplomatic points (D11): when the month's tick modifier is
// gone, send the same decision the Decisions tab offers. Only while game time
// advances; one request per absence (retry after two game days); the decision's
// effect re-checks its own conditions, so a duplicate does nothing.
struct Favours {
    country: usize,
    date: u32,
    pending: u32,
    defined: i32,
    check: u32,
    sent: u32,
}

static FAVOURS: Global<Favours> = Global::new(Favours {
    country: 0,
    date: 0,
    pending: 0,
    defined: -1,
    check: 0,
    sent: 0,
});

fn favour_step(sample: &Snapshot, timers: &BillTimers) -> bool {
    let f = FAVOURS.get();
    if sample.country != f.country {
        f.country = sample.country;
        f.date = sample.date_hours;
        f.pending = 0;
        f.defined = -1;
        return false;
    }
    if sample.date_hours == f.date {
        return false; // paused, menu or loading
    }
    f.date = sample.date_hours;
    if !timers.known {
        return false;
    }
    if timers.tick != 0 {
        f.pending = 0;
        return false;
    }
    if f.pending != 0 && sample.date_hours < f.pending + 48 {
        return false;
    }
    f.pending = sample.date_hours;
    true
}

fn favour_inner() {
    let Some(sample) = snapshot::snapshot() else {
        return;
    };
    let f = FAVOURS.get();
    let mut timers = BillTimers::default();
    if sample.country == f.country && sample.date_hours != f.date {
        timers = read_timers(sample.country).unwrap_or_default();
    }
    if !favour_step(&sample, &timers) {
        return;
    }
    // Without the submod the decision would not exist: ask the engine's
    // modifier table for ours first (once per country).
    let f = FAVOURS.get();
    if f.defined < 0 {
        f.defined = i32::from(modifier_defined("tgcx_favours_tick"));
    }
    if f.defined == 0 {
        f.pending = 0;
        return;
    }
    if dispatch(b"tgcx_collect_favours") {
        let f = FAVOURS.get();
        f.sent += 1;
        if f.sent <= 12 {
            log!(
                "Favours collection requested through native decision handler tag={} hours={}",
                sample.tag(),
                sample.date_hours
            );
        }
    }
}

pub fn favour_tick() {
    let b = BILL.get();
    if !b.enabled || b.decision_fn == 0 {
        return;
    }
    let now = crate::sys::tick();
    let f = FAVOURS.get();
    if f.check != 0 && now.wrapping_sub(f.check) < 250 {
        return;
    }
    f.check = now;
    // A sent decision's message arrives at once; never let a stale count hide a later one.
    if MSG_SUPPRESS.load(Ordering::SeqCst) != 0
        && now.wrapping_sub(BILL.get().msg_suppress_tick) > 10000
    {
        MSG_SUPPRESS.store(0, Ordering::SeqCst);
    }
    if guard(favour_inner).is_none() {
        BILL.get().enabled = false;
        log!("Automatic favours disabled after native read fault; the decision remains available");
    }
}

// ---------------------------------------------------------------- drawing

/// Our two staged textures (uncompressed A8R8G8B8 DDS written by
/// tools/tgcx_bill_gui.py), loaded into the managed pool per device.
#[derive(Clone, Copy)]
struct BillTexture {
    texture: usize,
    width: i32,
    height: i32,
}

struct Textures {
    bg: BillTexture,
    atlas: BillTexture,
    device: usize,
    failed: bool,
}

const NO_TEXTURE: BillTexture = BillTexture {
    texture: 0,
    width: 0,
    height: 0,
};
static TEXTURES: Global<Textures> = Global::new(Textures {
    bg: NO_TEXTURE,
    atlas: NO_TEXTURE,
    device: 0,
    failed: false,
});

fn texture_from_dds(device: Device, data: &[u8]) -> Option<BillTexture> {
    if data.len() < 128 || &data[..4] != b"DDS " {
        return None;
    }
    let u32_at = |at: usize| u32::from_le_bytes(data[at..at + 4].try_into().unwrap());
    let (height, width) = (u32_at(12), u32_at(16));
    if u32_at(4) != 124
        || u32_at(76) != 32
        || u32_at(80) & 0x40 == 0
        || u32_at(88) != 32
        || u32_at(92) != 0x00FF0000
        || u32_at(96) != 0x0000FF00
        || u32_at(100) != 0x000000FF
        || u32_at(104) != 0xFF000000
    {
        return None;
    }
    if width == 0
        || height == 0
        || width > 4096
        || height > 4096
        || data.len() < 128 + width as usize * height as usize * 4
    {
        return None;
    }
    let mut texture = 0usize;
    if !succeeded(device.create_texture(
        width,
        height,
        d3d::FMT_A8R8G8B8,
        d3d::POOL_MANAGED,
        &mut texture,
    )) {
        return None;
    }
    let mut rect = d3d::LockedRect {
        pitch: 0,
        bits: std::ptr::null_mut(),
    };
    if !succeeded(d3d::texture_lock(texture, &mut rect)) {
        d3d::release(texture);
        return None;
    }
    let row = width as usize * 4;
    for y in 0..height as usize {
        // SAFETY: the locked level has `height` rows of `pitch` bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                data[128 + y * row..].as_ptr(),
                rect.bits.add(y * rect.pitch as usize),
                row,
            );
        }
    }
    d3d::texture_unlock(texture);
    Some(BillTexture {
        texture,
        width: width as i32,
        height: height as i32,
    })
}

fn load_texture(device: Device, file: &str) -> Option<BillTexture> {
    let mut path = [0u16; 260];
    let count =
        unsafe { GetModuleFileNameW(std::ptr::null_mut(), path.as_mut_ptr(), 260) } as usize;
    if count == 0 || count >= 260 {
        return None;
    }
    let slash = path[..count].iter().rposition(|&c| c == u16::from(b'\\'))?;
    let mut full = String::from_utf16_lossy(&path[..slash]);
    write!(full, "\\mod\\TGCX\\gfx\\interface\\{file}").ok()?;
    if full.len() >= 260 {
        return None;
    }
    let metadata = std::fs::metadata(&full).ok()?;
    if metadata.len() > 64 << 20 {
        return None;
    }
    let data = std::fs::read(&full).ok()?;
    texture_from_dds(device, &data)
}

fn release_textures() {
    let t = TEXTURES.get();
    if t.bg.texture != 0 {
        d3d::release(t.bg.texture);
    }
    if t.atlas.texture != 0 {
        d3d::release(t.atlas.texture);
    }
    *t = Textures {
        bg: NO_TEXTURE,
        atlas: NO_TEXTURE,
        device: 0,
        failed: false,
    };
}

fn textures(device: Device) {
    if device.0 == TEXTURES.get().device {
        return;
    }
    release_textures();
    TEXTURES.get().device = device.0;
    let bg = load_texture(device, BILL_BG_FILE);
    let atlas = load_texture(device, BILL_ATLAS_FILE);
    let t = TEXTURES.get();
    t.bg = bg.unwrap_or(NO_TEXTURE);
    t.atlas = atlas.unwrap_or(NO_TEXTURE);
    t.failed = bg.is_none() || atlas.is_none();
    log!(
        "Bill textures background={} atlas={}",
        if bg.is_some() { "loaded" } else { "missing" },
        if atlas.is_some() { "loaded" } else { "missing" }
    );
}

#[repr(C)]
struct TexVertex {
    x: f32,
    y: f32,
    z: f32,
    rhw: f32,
    color: u32,
    u: f32,
    v: f32,
}

const TEX_FVF: u32 = d3d::FVF_XYZRHW | d3d::FVF_DIFFUSE | d3d::FVF_TEX1;

#[allow(clippy::too_many_arguments)]
fn blit(
    device: Device,
    t: &BillTexture,
    sx: i32,
    sy: i32,
    sw: i32,
    sh: i32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: u32,
) -> HResult {
    if t.texture == 0 {
        return S_FALSE;
    }
    let (u0, v0) = (sx as f32 / t.width as f32, sy as f32 / t.height as f32);
    let (u1, v1) = (
        (sx + sw) as f32 / t.width as f32,
        (sy + sh) as f32 / t.height as f32,
    );
    let vertex = |x: f32, y: f32, u: f32, v: f32| TexVertex {
        x: x - 0.5,
        y: y - 0.5,
        z: 0.0,
        rhw: 1.0,
        color,
        u,
        v,
    };
    let vx = [
        vertex(x, y, u0, v0),
        vertex(x + w, y, u1, v0),
        vertex(x, y + h, u0, v1),
        vertex(x + w, y + h, u1, v1),
    ];
    let mut c = Chain(device.set_texture(0, t.texture));
    c.set(|| device.set_fvf(TEX_FVF));
    c.set(|| device.draw_primitive_up(d3d::PT_TRIANGLESTRIP, 2, &vx));
    c.0
}

fn cell(device: Device, cell: &[i32; 4], x: f32, y: f32, color: u32) -> HResult {
    let atlas = TEXTURES.get().atlas;
    blit(
        device,
        &atlas,
        cell[0],
        cell[1],
        cell[2],
        cell[3],
        x,
        y,
        cell[2] as f32,
        cell[3] as f32,
        color,
    )
}

fn untextured(device: Device) -> HResult {
    let mut c = Chain(device.set_texture(0, 0));
    c.set(|| device.set_fvf(renderer::FVF));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_COLOROP, d3d::TOP_SELECTARG1));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_ALPHAOP, d3d::TOP_SELECTARG1));
    c.0
}

fn textured(device: Device) -> HResult {
    let mut c = Chain(device.set_texture_stage_state(0, d3d::TSS_COLOROP, d3d::TOP_MODULATE));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_ALPHAOP, d3d::TOP_MODULATE));
    c.0
}

/// The background under a rectangle: hides a native button or marker sprite.
/// Without our texture, a flat panel in the board's section colour.
fn patch(device: Device, left: i32, top: i32, x: i32, y: i32, w: i32, h: i32) -> HResult {
    let bg = TEXTURES.get().bg;
    if bg.texture != 0 {
        return blit(
            device,
            &bg,
            x,
            y,
            w,
            h,
            (left + x) as f32,
            (top + y) as f32,
            w as f32,
            h as f32,
            0xFFFFFFFF,
        );
    }
    let mut c = Chain(untextured(device));
    c.set(|| {
        soft_panel(
            device,
            (left + x) as f32,
            (top + y) as f32,
            w as f32,
            h as f32,
            0.0,
            0xFFCECBC3,
        )
    });
    c.set(|| textured(device));
    c.0
}

const AYE: u32 = 0xFF3F8F2F;
const ABSTAIN: u32 = 0xFFE4E0D0;
const NAY: u32 = 0xFFB8322A;
const RULE: u32 = 0xFF9C9E94;
const WHITE: u32 = 0xFFFFFFFF;

/// The Bill's state over its native art, drawn right after the marker icon (declared
/// after every button, before every text): hidden and disabled buttons, checks,
/// closeness rows, seats. gui.left/top are the Bill's origin.
fn draw_bill_chart(
    device: Device,
    model: &BillModel,
    sample: &Snapshot,
    gui: &GuiRect,
    view: &BillView,
) -> HResult {
    if !model.valid || gui.width < BILL_WIDTH || gui.height < BILL_HEIGHT {
        return S_FALSE;
    }
    let mut seats = [0u32; IDEOLOGIES];
    for i in 0..IDEOLOGIES {
        seats[i] = model.bloc[i].seats as u32;
    }
    let dots = seat_geometry(&seats);
    match d3d::backbuffer_size(device) {
        Err(result) => return result,
        Ok(Some((w, h))) if w == gui.width as u32 && h == gui.height as u32 => {}
        Ok(_) => return S_FALSE,
    }
    let mut state = 0usize;
    let result = device.create_state_block(&mut state);
    if !succeeded(result) {
        return result;
    }
    let result = d3d::state_capture(state);
    if !succeeded(result) {
        d3d::release(state);
        return result;
    }
    let (left, top) = (gui.left, gui.top);
    let clip = RECT {
        left,
        top,
        right: left + BILL_WIDTH,
        bottom: top + BILL_HEIGHT,
    };
    let viewport = Viewport {
        x: 0,
        y: 0,
        width: gui.width as u32,
        height: gui.height as u32,
        min_z: 0.0,
        max_z: 1.0,
    };
    let mut c = Chain(result);
    c.set(|| device.set_viewport(&viewport));
    c.set(|| device.set_scissor_rect(&clip));
    c.set(|| device.set_render_state(d3d::RS_SCISSORTESTENABLE, 1));
    common_state(&mut c, device);
    blend_state(&mut c, device);
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_COLORARG2, d3d::TA_TEXTURE));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_ALPHAARG2, d3d::TA_TEXTURE));
    c.set(|| device.set_sampler_state(0, d3d::SAMP_MINFILTER, d3d::TEXF_POINT));
    c.set(|| device.set_sampler_state(0, d3d::SAMP_MAGFILTER, d3d::TEXF_POINT));
    c.set(|| device.set_sampler_state(0, d3d::SAMP_MIPFILTER, d3d::TEXF_NONE));
    c.set(|| device.set_sampler_state(0, d3d::SAMP_ADDRESSU, d3d::TADDRESS_CLAMP));
    c.set(|| device.set_sampler_state(0, d3d::SAMP_ADDRESSV, d3d::TADDRESS_CLAMP));
    // Textured pass: art over the native widgets.
    c.set(|| textured(device));
    let p = &BILL_CHART_PATCH;
    c.set(|| patch(device, left, top, p[0], p[1], p[2], p[3]));
    let fx = |x: i32| (left + x) as f32;
    let fy = |y: i32| (top + y) as f32;
    let button = |c: &mut Chain, id: usize, action: &BillAction| {
        let b = &BILL_BUTTONS[id];
        if action.visible && action.enabled {
            return;
        }
        let (x, y, w, h) = (
            i32::from(b.x),
            i32::from(b.y),
            i32::from(b.w),
            i32::from(b.h),
        );
        c.set(|| patch(device, left, top, x, y, w, h));
        let off = match w {
            111 => &BILL_CELL_OFF111,
            177 => &BILL_CELL_OFF177,
            _ => &BILL_CELL_OFF144,
        };
        if action.visible {
            c.set(|| cell(device, off, fx(x), fy(y), WHITE));
        }
    };
    let hidden = BillAction::default();
    for s in 0..CARDS {
        let (a, b) = if s < view.slots {
            (&view.card[s].a, &view.card[s].b)
        } else {
            (&hidden, &hidden)
        };
        button(&mut c, B_CARD_0_A + 2 * s, a);
        button(&mut c, B_CARD_0_B + 2 * s, b);
    }
    button(&mut c, B_AUDIENCE, &view.audience);
    button(&mut c, B_VOTE, &view.vote);
    let (mode, lot) = (BILL.get().mode, BILL.get().lot);
    for n in 0..3 {
        if n as i32 == mode {
            c.set(|| {
                cell(
                    device,
                    &BILL_CELL_CHECKED,
                    fx(BILL_MODE_X[n]),
                    fy(BILL_MODE_Y),
                    WHITE,
                )
            });
        }
    }
    c.set(|| {
        cell(
            device,
            &BILL_CELL_CHECKED,
            fx(BILL_LOT_X[usize::from(lot == 25)]),
            fy(BILL_LOT_Y),
            WHITE,
        )
    });
    let law = (model.id > 0).then(|| &BILL_CATALOG[(model.id - 1) as usize]);
    if let Some(law) = law {
        let size = BILL_ICON_CELL;
        let (column, row) = (
            i32::from(law.group) % BILL_ICON_COLUMNS,
            i32::from(law.group) / BILL_ICON_COLUMNS,
        );
        let atlas = TEXTURES.get().atlas;
        c.set(|| {
            blit(
                device,
                &atlas,
                size * column,
                BILL_ICON_Y + size * row,
                size,
                size,
                fx(66),
                fy(116),
                32.0,
                32.0,
                WHITE,
            )
        });
        let (start, g) = option_window(law);
        for s in 0..view.slots {
            let y0 = BILL_CARD_Y + BILL_CARD_STEP * s as i32;
            let i = view.slot[s];
            c.set(|| {
                cell(
                    device,
                    &BILL_CELL_PLUPP,
                    fx(BILL_CB + 2),
                    fy(y0 + 6),
                    sample.colors[i],
                )
            });
            for r in 0..3 {
                if start + r >= g.count as usize {
                    break;
                }
                let (x, y) = (fx(BILL_CC + 1), fy(y0 + 6 + 14 * r as i32));
                let option = start + r;
                c.set(|| cell(device, &BILL_CELL_ROW, x, y, WHITE));
                if option == law.from_index as usize {
                    c.set(|| cell(device, &BILL_CELL_SELECTED, x, y, WHITE));
                } else if option == law.to_index as usize {
                    c.set(|| cell(device, &BILL_CELL_SELECT, x + 15.0, y + 1.0, WHITE));
                }
                if option == law.ideal_index[i] as usize {
                    c.set(|| cell(device, &BILL_CELL_DIAMOND, x + 136.0, y + 2.0, WHITE));
                }
            }
        }
    }
    // Untextured pass: rules, seats and the legend.
    c.set(|| untextured(device));
    for s in 0..view.slots {
        let y = BILL_CARD_Y + BILL_CARD_STEP * s as i32 + 50;
        c.set(|| {
            soft_panel(
                device,
                fx(BILL_CB + 4),
                fy(y),
                (BILL_CE - BILL_CB - 8) as f32,
                1.0,
                0.0,
                RULE,
            )
        });
    }
    let (cosine, sine) = unit_circle();
    let mut used = [0i32; IDEOLOGIES];
    // Geometry's angular sweep is separate from storage order. Each bloc still has
    // exactly its cached Aye/abstain/Nay seats; no new vote is calculated.
    let cx = BILL_CHART[0] + BILL_CHART[2] / 2;
    let cy = BILL_CHART[1] + BILL_CHART[3] - 6;
    let vertices = SEAT_VERTICES.get();
    for (i, dot) in dots.iter().enumerate() {
        let b = dot.ideology;
        let n = used[b];
        used[b] += 1;
        let bloc = &model.bloc[b];
        let vote = if n < bloc.yes {
            AYE
        } else if n < bloc.yes + bloc.abstain {
            ABSTAIN
        } else {
            NAY
        };
        let x = fx(cx) + (dot.x - 99.0) * 2.0;
        let y = fy(cy) + (dot.y - 183.0) * 2.0;
        let out = &mut vertices[i * 2 * DISC_VERTICES..];
        let outer = if mode == 0 {
            0xFF4A4636
        } else {
            sample.colors[b]
        };
        let inner = if mode == 1 { sample.colors[b] } else { vote };
        append_disc(out, &cosine, &sine, x, y, 5.0, 0.7, outer);
        append_disc(
            &mut out[DISC_VERTICES..],
            &cosine,
            &sine,
            x,
            y,
            if mode == 2 { 2.8 } else { 4.0 },
            0.5,
            inner,
        );
    }
    c.set(|| {
        device.draw_primitive_up(
            d3d::PT_TRIANGLELIST,
            (100 * 2 * DISC_VERTICES / 3) as u32,
            &vertices[..],
        )
    });
    let legend = [AYE, ABSTAIN, NAY];
    for (n, color) in legend.iter().enumerate() {
        let out = &mut vertices[n * 2 * DISC_VERTICES..];
        let (x, y) = (fx(BILL_LEGEND_X[n] + 6), fy(BILL_LEGEND_Y + 5));
        append_disc(out, &cosine, &sine, x, y, 5.0, 0.7, 0xFF4A4636);
        append_disc(
            &mut out[DISC_VERTICES..],
            &cosine,
            &sine,
            x,
            y,
            4.0,
            0.5,
            *color,
        );
    }
    c.set(|| {
        device.draw_primitive_up(
            d3d::PT_TRIANGLELIST,
            (3 * 2 * DISC_VERTICES / 3) as u32,
            &vertices[..3 * 2 * DISC_VERTICES],
        )
    });
    let restore = d3d::state_apply(state);
    d3d::release(state);
    if succeeded(restore) { c.0 } else { restore }
}

fn after_entity_inner(renderer: usize, entity: usize) {
    let b = BILL.get();
    let window = b.window;
    if rd8(window + 0x67) == 0 {
        return;
    }
    let marker = gui_entity(window, renderer, "tgcx_bill_chart_overlay", 0xA3FF08);
    if marker != entity {
        return;
    }
    let Some(gui) = read_politics_rect() else {
        return;
    };
    let Some(sample) = snapshot::snapshot() else {
        return;
    };
    if sample.country != BILL.get().model.country {
        return;
    }
    let device = rdp(renderer + 0x64);
    if device == 0 {
        return;
    }
    let Some(bill) = screen_rect(window, &gui) else {
        return;
    };
    let device = Device(device);
    textures(device);
    let b = BILL.get();
    let (model, view) = (b.model, b.view);
    if draw_bill_chart(device, &model, &sample, &bill, &view) == S_OK {
        let b = BILL.get();
        b.draws += 1;
        if b.draws == 1 {
            log!(
                "Bill first ordered overlay draw origin={},{} Aye={} abstain={} Nay={} cards={}",
                bill.left,
                bill.top,
                model.yes,
                model.abstain,
                model.no,
                view.slots
            );
        }
    }
}

pub fn after_entity(renderer: usize, entity: usize) {
    let b = BILL.get();
    if !b.enabled || !b.open || !b.model.valid || b.window == 0 {
        return;
    }
    if guard(|| after_entity_inner(renderer, entity)).is_none() {
        BILL.get().model.valid = false;
        log!("Bill chart rejected: native marker read fault");
    }
}
