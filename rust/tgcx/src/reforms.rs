//! LAW-2 Reforms tab and Decisions list (mockup board 1-politics): GUI only; port of
//! tgcx_reforms.h. A bill is presented from the Reforms tab through the engine's
//! decision command, which validates the decision's potential and allow itself
//! (CExecuteDecisionCommand slot 14, FUN_00906B40; P39). The Decisions tab hides the
//! tgcx_ decisions the Bill and the Reforms tab already offer, and the "decision
//! taken" popup is skipped for decisions the DLL sent (R5, R6).

use std::sync::atomic::{AtomicI32, AtomicU8, AtomicUsize, Ordering};

use crate::bill::{self, BILL, BillModel, MSG_SUPPRESS};
use crate::bill_catalog::{
    BILL_CATALOG, BILL_GROUP_KEYS, BILL_GROUP_POLITICAL, BILL_GROUPS, BILL_OPTION_KEYS,
};
use crate::snapshot;
use crate::sys::{
    Bytes, CpuScope, Global, at, base, call_zero, code_patch, guard, rd8, rdi, rdp,
    read_game_string, require_signatures, slot, thread_id,
};
use crate::{log, text};

const GROUPS: usize = BILL_GROUPS.len();

/// The law group and option of an option key.
fn find(key: &[u8]) -> Option<(usize, usize)> {
    (0..GROUPS).find_map(|g| {
        (0..BILL_GROUPS[g].count as usize)
            .find(|&i| BILL_OPTION_KEYS[g][i].as_bytes() == key)
            .map(|i| (g, i))
    })
}

/// The catalogue bill (tgcx_gui_bill_id, 1-based) from the current law to a target; 0 = none.
fn bill_for(group: usize, from: usize, to: usize) -> usize {
    BILL_CATALOG
        .iter()
        .position(|b| {
            b.group as usize == group && b.from_index as usize == from && b.to_index as usize == to
        })
        .map_or(0, |n| n + 1)
}

/// Refreshed by the Bill's tick from fresh script state. offer_from[g] is the monthly
/// collection's tgcx_gui_offer_<group>: 1 + the law whose next step passes its own TGC
/// allow (D14 B), 0 = none or not collected yet.
#[derive(Clone, Copy, Default, PartialEq)]
struct Offer {
    known: bool,
    legislature: bool,
    cooldown: bool,
    active: bool,
    delay_open: bool,
    target_group: i32,
    target_index: i32,
    offer_from: [i32; GROUPS],
}

struct Reforms {
    offer: Offer,
    rebuild: bool,
    refresh_ok: bool,
    pass_lit: u32,
    pass_dark: u32,
    pass_ours: u32,
    current: [i32; GROUPS],
    native: [[bool; 8]; GROUPS],
    installed: bool,
    rows: u32,
    decisions_hidden: u32,
    ignored_installed: bool,
    decisions_ignored: u32,
}

static REFORMS: Global<Reforms> = Global::new(Reforms {
    offer: Offer {
        known: false,
        legislature: false,
        cooldown: false,
        active: false,
        delay_open: false,
        target_group: 0,
        target_index: 0,
        offer_from: [0; GROUPS],
    },
    rebuild: false,
    refresh_ok: false,
    pass_lit: 0,
    pass_dark: 0,
    pass_ours: 0,
    current: [0; GROUPS],
    native: [[false; 8]; GROUPS],
    installed: false,
    rows: 0,
    decisions_hidden: 0,
    ignored_installed: false,
    decisions_ignored: 0,
});

/// The board's tan row is the native enabled reform button (politics_reform_select).
/// With a bill open only its target is lit, native rows included. Otherwise (D14) the
/// next step the chamber blocks today, where a chamber exists, no cooldown or native
/// reform delay runs and TGC's allow for it holds. Steps back are never negotiated:
/// the ones the game allows are lit natively.
fn offers(o: &Offer, group: usize, current: usize, index: usize) -> bool {
    if !o.known {
        return false;
    }
    if o.active {
        return o.target_group == group as i32 && o.target_index == index as i32;
    }
    if !o.legislature || o.cooldown || !o.delay_open || index != current + 1 {
        return false;
    }
    o.offer_from[group] == current as i32 + 1 && bill_for(group, current, index) > 0
}

/// The native minimum delay between reforms (FUN_005268F0: EAX = country, AL = open,
/// plain RET). Read-only; ESP is restored whatever the callee does.
static REFORM_DELAY_FN: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
unsafe extern "C" fn call_reform_delay(_country: usize) -> u32 {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp, esp",
        "push ebx",
        "push esi",
        "push edi",
        "mov eax, [ebp+8]",
        "call dword ptr [{f}]",
        "movzx eax, al",
        "lea esp, [ebp-12]",
        "pop edi",
        "pop esi",
        "pop ebx",
        "pop ebp",
        "ret",
        f = sym REFORM_DELAY_FN,
    )
}

fn delay_open(country: usize) -> bool {
    if REFORM_DELAY_FN.load(Ordering::Relaxed) == 0 {
        return true; // guard failed: the script cooldown alone applies
    }
    // SAFETY: FUN_005268F0, guarded by its signatures.
    guard(|| unsafe { call_reform_delay(country) } != 0).unwrap_or(true)
}

pub fn refresh(country: usize, model: &BillModel, model_valid: bool) {
    let mut o = Offer::default();
    let mut ok = country != 0;
    let mut timers = bill::BillTimers::default();
    if ok {
        match bill::read_timers(country) {
            Some(t) => timers = t,
            None => ok = false,
        }
    }
    let legislature = if ok {
        bill::value(country, b"tgcx_gui_legislature\0", &mut ok)
    } else {
        0
    };
    if ok {
        o.known = true;
        o.legislature = legislature == 1;
        o.cooldown = timers.cooldown != 0;
        o.active = model_valid && model.active;
        o.target_group = -1;
        o.target_index = -1;
        o.delay_open = delay_open(country);
        for g in 0..GROUPS {
            let mut has = true;
            let key = text!(64, "tgcx_gui_offer_{}", BILL_GROUP_KEYS[g]);
            let from = bill::value(country, key.with_nul(), &mut has);
            o.offer_from[g] = if has && from > 0 && from <= i32::from(BILL_GROUPS[g].count) {
                from
            } else {
                0
            };
        }
        if o.active && model.id > 0 && model.id as usize <= BILL_CATALOG.len() {
            let b = &BILL_CATALOG[(model.id - 1) as usize];
            o.target_group = i32::from(b.group);
            o.target_index = i32::from(b.to_index);
        }
    }
    // P46: the rows keep their state until the view rebuilds them.
    let r = REFORMS.get();
    if o != r.offer {
        r.rebuild = true;
    }
    r.offer = o;
}

/// Ask the native view to rebuild every row (slot 11, as its show does), on the GUI
/// thread, after the offer changed. Logs each pass's tally.
pub fn after_tick(view: usize) {
    let r = REFORMS.get();
    if r.pass_lit != 0 || r.pass_dark != 0 {
        log!(
            "Reforms rows evaluated: lit={} (ours={}) dark={} active={}",
            r.pass_lit,
            r.pass_ours,
            r.pass_dark,
            u32::from(r.offer.active)
        );
        r.pass_lit = 0;
        r.pass_dark = 0;
        r.pass_ours = 0;
    }
    if !r.rebuild || !r.refresh_ok || !r.installed || view == 0 {
        return;
    }
    r.rebuild = false;
    let rebuilt = guard(|| {
        if rdp(view) != base() + 0xA0E458 || slot(view, 11) != base() + 0x2D7180 {
            return;
        }
        call_zero(slot(view, 11), view);
    });
    if rebuilt.is_none() {
        REFORMS.get().refresh_ok = false;
        log!("Reforms rebuild disabled after a read fault");
    }
}

// ---------------------------------------------------------------- native hooks

fn usable() -> bool {
    let b = BILL.get();
    b.enabled && b.usable
}

/// Reform row (FUN_006DA930 at RVA 0x2DB22D, after the native enable decision):
/// frame -0x3C option, -0x1C group, -0x18 country, -0x11 enabled byte (P39).
fn row_inner(frame: usize) {
    let option = rdp(frame - 0x3C);
    let group = rdp(frame - 0x1C);
    let country = rdp(frame - 0x18);
    let enabled = frame - 0x11;
    if option == 0 || group == 0 || country == 0 {
        return;
    }
    let Some(key) = read_game_string::<64>(option + 0x34) else {
        return;
    };
    let Some((g, index)) = find(key.bytes()) else {
        return;
    };
    // Current law: the per-category arrays the native check reads (+0x860 political, +0x850 social).
    let list = rdp(country
        + if BILL_GROUP_POLITICAL[g] {
            0x860
        } else {
            0x850
        });
    let slot = rdi(group + 0x50);
    if list == 0 || !(0..64).contains(&slot) {
        return;
    }
    let current = rdp(list + 4 * slot as usize);
    if current == 0 {
        return;
    }
    let Some(now) = read_game_string::<64>(current + 0x34) else {
        return;
    };
    let Some((cg, ci)) = find(now.bytes()) else {
        return;
    };
    if cg != g {
        return;
    }
    let r = REFORMS.get();
    r.current[g] = ci as i32;
    r.native[g][index] = rd8(enabled) != 0;
    let offered = offers(&r.offer, g, ci, index);
    if r.offer.known && r.offer.active {
        crate::sys::wr(enabled, u8::from(offered)); // board 1: only the target
    } else if rd8(enabled) == 0 && offered {
        crate::sys::wr(enabled, 1u8);
        r.pass_ours += 1;
        r.rows += 1;
        if r.rows == 1 {
            log!("Reforms tab offers {key} (first row lit)");
        }
    }
    if rd8(enabled) != 0 {
        r.pass_lit += 1;
    } else {
        r.pass_dark += 1;
    }
}

extern "C" fn reform_row(frame: usize) {
    let _cpu = CpuScope::new();
    if !usable() {
        return;
    }
    let _ = guard(|| row_inner(frame));
}

/// Reform click (FUN_006DC990, element +0x14 "POLITICSVIEW_<POLITICAL|SOCIAL>_REFORM<key>").
/// 1 = handled here; 0 = the native reform the chamber already allows.
fn click_inner(element: usize) -> i32 {
    let Some(text) = read_game_string::<128>(element + 0x14) else {
        return 0;
    };
    const PREFIXES: [&[u8]; 2] = [
        b"POLITICSVIEW_POLITICAL_REFORM",
        b"POLITICSVIEW_SOCIAL_REFORM",
    ];
    let mut key: Option<&[u8]> = None;
    for p in PREFIXES {
        if let Some(rest) = text.bytes().strip_prefix(p) {
            key = Some(rest);
        }
    }
    let Some(key) = key else { return 0 };
    let Some((g, index)) = find(key) else {
        return 0;
    };
    let r = REFORMS.get();
    let current = r.current[g];
    // With a bill open every row belongs to the Bill, native ones too: none enacts directly.
    if current < 0 || (r.native[g][index] && !r.offer.active) {
        return 0;
    }
    // Natively this row cannot be clicked; whatever happens next, the reform command is not sent.
    let Some(sample) = snapshot::snapshot() else {
        return 1;
    };
    if bill::read_model(sample.country).valid {
        let b = BILL.get();
        b.open = true;
        b.dirty = true;
        b.tick = 0;
        log!(
            "Reforms tab opened the Bill tag={} row={}",
            sample.tag(),
            Bytes(key)
        );
        return 1;
    }
    let r = REFORMS.get();
    if r.native[g][index] {
        return 0;
    }
    let current = current as usize;
    if !offers(&r.offer, g, current, index) {
        return 1;
    }
    let bill_id = bill_for(g, current, index);
    if bill_id == 0 {
        return 1;
    }
    let decision = text!(
        64,
        "tgcx_bill_{}_{}",
        BILL_GROUP_KEYS[g],
        if index > current { "up" } else { "down" }
    );
    if bill::dispatch(decision.bytes()) {
        BILL.get().tick = 0;
        log!(
            "Reforms tab presented {decision} (bill {bill_id}) tag={}",
            sample.tag()
        );
    }
    1
}

extern "C" fn reform_click(element: usize) -> i32 {
    let _cpu = CpuScope::new();
    if !usable() || thread_id() != BILL.get().thread {
        return 0;
    }
    guard(|| click_inner(element)).unwrap_or(0)
}

/// One of our decisions, by its name (std::string at +8).
fn ours(decision: usize) -> Option<crate::sys::Text<96>> {
    if decision == 0 {
        return None;
    }
    read_game_string::<96>(decision + 8).filter(|name| name.bytes().starts_with(b"tgcx_"))
}

/// Decisions list (FUN_006DB2E0): skip the tgcx_ decisions.
extern "C" fn decision_hidden(decision: usize) -> i32 {
    let _cpu = CpuScope::new();
    if !usable() || !REFORMS.get().installed {
        return 0;
    }
    guard(|| {
        let Some(name) = ours(decision) else { return 0 };
        let r = REFORMS.get();
        r.decisions_hidden += 1;
        if r.decisions_hidden == 1 {
            log!("Decisions list hides {name} (and every tgcx_ decision while the Bill is usable)");
        }
        1
    })
    .unwrap_or(0)
}

/// Top-bar decisions alert and Politics tooltip (FUN_0053A6A0): our decisions count
/// as ignored while the DLL offers them, unless the Bill cannot fit (R6).
extern "C" fn decision_ignored(decision: usize) -> i32 {
    let _cpu = CpuScope::new();
    let b = BILL.get();
    if !b.enabled || !REFORMS.get().installed || (b.size_known && !b.usable) {
        return 0;
    }
    guard(|| {
        let Some(name) = ours(decision) else { return 0 };
        let r = REFORMS.get();
        r.decisions_ignored += 1;
        if r.decisions_ignored == 1 {
            log!("Decisions alert skips {name} (and every tgcx_ decision)");
        }
        1
    })
    .unwrap_or(0)
}

static REFORM_CLICK_RESUME: AtomicUsize = AtomicUsize::new(0);
static REFORM_ROW_RESUME: AtomicUsize = AtomicUsize::new(0);
static DECISION_RESUME0: AtomicUsize = AtomicUsize::new(0);
static DECISION_SKIP0: AtomicUsize = AtomicUsize::new(0);
static DECISION_RESUME1: AtomicUsize = AtomicUsize::new(0);
static DECISION_SKIP1: AtomicUsize = AtomicUsize::new(0);
static REFORM_HANDLED: AtomicI32 = AtomicI32::new(0);
static DECISION_HIDE: AtomicI32 = AtomicI32::new(0);
static IGNORED_HIDE: AtomicI32 = AtomicI32::new(0);
static IGNORED_RESUME: AtomicUsize = AtomicUsize::new(0);

// Original PUSH ESI / MOV ESI,[EAX+0x188] replicated; ours returns AL = 1 (ignored).
#[unsafe(naked)]
unsafe extern "C" fn ignored_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push edi",
        "call {callback}",
        "add esp, 4",
        "mov dword ptr [{hide}], eax",
        crate::restore_cpu!(),
        "cmp dword ptr [{hide}], 0",
        "jne 2f",
        "push esi",
        "mov esi, [eax+0x188]",
        "jmp dword ptr [{resume}]",
        "2:",
        "mov eax, 1",
        "ret",
        callback = sym decision_ignored,
        hide = sym IGNORED_HIDE,
        resume = sym IGNORED_RESUME,
    )
}

// from V2DLL: V2TechButton.cpp, entry-hook preservation pattern; the overwritten
// prologue (push ebp / mov ebp,esp / and esp,-8) is replicated exactly; RET 4 as native.
#[unsafe(naked)]
unsafe extern "C" fn reform_click_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+40]",
        "call {callback}",
        "add esp, 4",
        "mov dword ptr [{handled}], eax",
        crate::restore_cpu!(),
        "cmp dword ptr [{handled}], 0",
        "jne 2f",
        "push ebp",
        "mov ebp, esp",
        "and esp, -8",
        "jmp dword ptr [{resume}]",
        "2:",
        "ret 4",
        callback = sym reform_click,
        handled = sym REFORM_HANDLED,
        resume = sym REFORM_CLICK_RESUME,
    )
}

// Original CMP byte [ebp-0x11],0 / MOV EDX,[EAX] replicated after the call.
#[unsafe(naked)]
unsafe extern "C" fn reform_row_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+8]",
        "call {callback}",
        "add esp, 4",
        crate::restore_cpu!(),
        "cmp byte ptr [ebp-0x11], 0",
        "mov edx, [eax]",
        "jmp dword ptr [{resume}]",
        callback = sym reform_row,
        resume = sym REFORM_ROW_RESUME,
    )
}

// Original TEST AL,AL / JZ next / MOV ECX,[EBP-0x80] replicated; hidden = next.
macro_rules! decision_filter {
    ($name:ident, $resume:ident, $skip:ident) => {
        #[unsafe(naked)]
        unsafe extern "C" fn $name() {
            std::arch::naked_asm!(
                "test al, al",
                "jz 2f",
                crate::save_cpu!(),
                "push dword ptr [ebp-0x80]",
                "call {callback}",
                "add esp, 4",
                "mov dword ptr [{hide}], eax",
                crate::restore_cpu!(),
                "cmp dword ptr [{hide}], 0",
                "jne 2f",
                "mov ecx, [ebp-0x80]",
                "jmp dword ptr [{resume}]",
                "2:",
                "jmp dword ptr [{skip}]",
                callback = sym decision_hidden,
                hide = sym DECISION_HIDE,
                resume = sym $resume,
                skip = sym $skip,
            )
        }
    };
}
decision_filter!(decision_filter0, DECISION_RESUME0, DECISION_SKIP0);
decision_filter!(decision_filter1, DECISION_RESUME1, DECISION_SKIP1);

// "Decision taken" message (FUN_00906F00, by-value message, RET 0x8C) for the
// player's decisions: for one the DLL sent, its gate byte is cleared for that one
// call, so the poster takes its own early exit and destroys its argument.
static MSG_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static MSG_GATE: AtomicUsize = AtomicUsize::new(0);
static MSG_RETURN: AtomicUsize = AtomicUsize::new(0);
static MSG_SAVED: AtomicU8 = AtomicU8::new(0);

#[unsafe(naked)]
unsafe extern "C" fn message_thunk() {
    std::arch::naked_asm!(
        "cmp dword ptr [{suppress}], 0",
        "je 2f",
        "lock dec dword ptr [{suppress}]",
        "push eax",
        "push ecx",
        "mov ecx, dword ptr [{gate}]",
        "mov al, [ecx]",
        "mov byte ptr [{saved}], al",
        "mov byte ptr [ecx], 0",
        "pop ecx",
        "pop eax",
        "pop dword ptr [{ret}]",
        "call 2f",
        "push eax",
        "push ecx",
        "mov ecx, dword ptr [{gate}]",
        "mov al, byte ptr [{saved}]",
        "mov [ecx], al",
        "pop ecx",
        "pop eax",
        "jmp dword ptr [{ret}]",
        "2:",
        "jmp dword ptr [{original}]",
        suppress = sym MSG_SUPPRESS,
        gate = sym MSG_GATE,
        saved = sym MSG_SAVED,
        ret = sym MSG_RETURN,
        original = sym MSG_ORIGINAL,
    )
}

pub fn install() {
    REFORMS.get().current = [-1; GROUPS];
    const TAB: [u32; 10] = [
        0x2DC990, 0x2DCCEE, 0x2DB00E, 0x2DB02D, 0x2DB0AE, 0x2DB22D, 0x575F15, 0x2DB52A, 0x2DB5E8,
        0x1C2B15,
    ];
    if !require_signatures(&TAB) {
        log!("Reforms tab hooks rejected: guards; bills stay under Decisions");
    } else {
        REFORM_CLICK_RESUME.store(at(0x2DC996), Ordering::Relaxed);
        REFORM_ROW_RESUME.store(at(0x2DB233), Ordering::Relaxed);
        DECISION_RESUME0.store(at(0x2DB531), Ordering::Relaxed);
        DECISION_SKIP0.store(at(0x2DB59D), Ordering::Relaxed);
        DECISION_RESUME1.store(at(0x2DB5EF), Ordering::Relaxed);
        DECISION_SKIP1.store(at(0x2DB649), Ordering::Relaxed);
        let click = code_patch(0x2DC990, reform_click_thunk as *const () as usize, 6, 0xE9);
        let row = click && code_patch(0x2DB22D, reform_row_thunk as *const () as usize, 6, 0xE9);
        let list = row
            && code_patch(0x2DB52A, decision_filter0 as *const () as usize, 7, 0xE9)
            && code_patch(0x2DB5E8, decision_filter1 as *const () as usize, 7, 0xE9);
        REFORMS.get().installed = list;
        log!(
            "Reforms tab hooks click={} row={} decisions={} RVA=2DC990/2DB22D/2DB52A/2DB5E8",
            u32::from(click),
            u32::from(row),
            u32::from(list)
        );
        const REBUILD: [u32; 4] = [0xA0E484, 0x2D7180, 0x2D71AE, 0x2D2C2B];
        REFORMS.get().refresh_ok = list && require_signatures(&REBUILD);
        const DELAY: [u32; 2] = [0x1268F0, 0x12693C];
        if list && require_signatures(&DELAY) {
            REFORM_DELAY_FN.store(at(0x1268F0), Ordering::Relaxed);
        }
        log!(
            "Reforms rows rebuild={} reform delay gate={} RVA=2D7180/1268F0",
            u32::from(REFORMS.get().refresh_ok),
            u32::from(REFORM_DELAY_FN.load(Ordering::Relaxed) != 0)
        );
        const ALERT: [u32; 4] = [0x13A6A0, 0x13C100, 0x13C2A1, 0x1C2B15];
        if list && require_signatures(&ALERT) {
            IGNORED_RESUME.store(at(0x13A6A7), Ordering::Relaxed);
            REFORMS.get().ignored_installed =
                code_patch(0x13A6A0, ignored_thunk as *const () as usize, 7, 0xE9);
        }
        log!(
            "Decisions alert filter {} RVA=13A6A0",
            if REFORMS.get().ignored_installed {
                "installed"
            } else {
                "rejected: guards"
            }
        );
    }
    const MESSAGE: [u32; 4] = [0x506690, 0x506F00, 0x506F55, 0x507542];
    if !require_signatures(&MESSAGE) {
        log!("Decision message filter rejected: guards; popups remain");
        return;
    }
    MSG_ORIGINAL.store(at(0x506F00), Ordering::Relaxed);
    MSG_GATE.store(at(0xB07DA6), Ordering::Relaxed);
    let hooked = code_patch(0x506691, message_thunk as *const () as usize, 5, 0xE8);
    BILL.get().msg_hooked = hooked;
    log!(
        "Decision message filter {} CALL_RVA=506691 gate_RVA=B07DA6",
        if hooked { "installed" } else { "rejected" }
    );
}
