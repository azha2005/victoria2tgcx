//! TPL-1 Conform to template (mockup board 6-conform; D12, D13); port of
//! tgcx_conform.h. Pure planning and session tracking: nothing here reads or writes the
//! game. Every change it asks for is an engine command (R5): CCreateUnitCommand +
//! CTransferSubUnitCommand detach extras, TPL-0's CConstructUnitCommand builds,
//! CMoveCommand (safe path) then CMergeCommand bring the new stacks. Offsets and ABIs:
//! docs/symbols.md "TPL-1".

use crate::templates::UNITS;

/// A registry id: type < 0x1269 lives in the low table, otherwise the high one.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CfId {
    pub kind: u32,
    pub id: u32,
}

impl CfId {
    pub fn none(&self) -> bool {
        self.kind == 0 && self.id == 0
    }
}

pub const REGIMENTS: usize = 128;
const IDLE: usize = 256;
pub const TASKS: usize = 256;
/// Longest wait for a construction never seen.
const BUILD_DAYS: u32 = 730;

// ---------------------------------------------------------------- planning (pure)

/// unit is the unit type index (+0x24); -1 when the catalogue cannot name it.
#[derive(Clone, Copy, Default)]
pub struct CfRegiment {
    pub id: CfId,
    pub unit: i32,
}

/// A finished one-regiment stack the player owns (D13: gathered after a reload).
/// cost orders the choice, lower first; the caller supplies it.
#[derive(Clone, Copy, Default)]
pub struct CfIdle {
    pub stack: CfId,
    pub unit: i32,
    pub cost: i32,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Refusal {
    #[default]
    Ok = 0,
    Matches = 1,
    NothingKept = 2,
    TooLarge = 3,
}

pub struct CfPlan {
    pub refusal: Refusal,
    /// Leave into one new stack.
    pub detach: Vec<CfRegiment>,
    /// March and join.
    pub gather: Vec<CfIdle>,
    /// To TPL-0's recruit planner.
    pub build: [i32; UNITS],
    pub build_total: u32,
}

impl CfPlan {
    pub const EMPTY: CfPlan = CfPlan {
        refusal: Refusal::Ok,
        detach: Vec::new(),
        gather: Vec::new(),
        build: [0; UNITS],
        build_total: 0,
    };
}

fn valid_unit(unit: i32) -> Option<usize> {
    usize::try_from(unit).ok().filter(|&u| u < UNITS)
}

/// Extras of each type are the last ones of that type in the stack's order, so the
/// first regiments stay. Regiments the catalogue cannot name are kept and never
/// counted against the template. Missing regiments come first from pending ones
/// (already on their way this session), then from idle stacks (cheapest first), then
/// are built. A plan that would empty the stack is refused: nothing is disbanded, and
/// an empty stack is not a conform.
pub fn make_plan(
    regs: &[CfRegiment],
    want: &[i32; UNITS],
    pending: Option<&[i32; UNITS]>,
    idle: &[CfIdle],
    out: &mut CfPlan,
) {
    out.refusal = Refusal::Ok;
    out.detach.clear();
    out.gather.clear();
    out.build_total = 0;
    out.build = [0; UNITS];
    let total: i32 = want.iter().map(|&w| w.max(0)).sum();
    if regs.len() > REGIMENTS || total as usize > REGIMENTS {
        out.refusal = Refusal::TooLarge;
        return;
    }
    let mut have = [0i32; UNITS];
    for r in regs {
        if let Some(u) = valid_unit(r.unit) {
            have[u] += 1;
        }
    }
    let mut kept = 0;
    let mut seen = [0i32; UNITS];
    for r in regs {
        match valid_unit(r.unit) {
            Some(u) => {
                let s = seen[u];
                seen[u] += 1;
                if s < have[u].min(want[u].max(0)) {
                    kept += 1;
                } else {
                    out.detach.push(*r);
                }
            }
            None => kept += 1,
        }
    }
    if kept == 0 && !regs.is_empty() {
        out.refusal = Refusal::NothingKept;
        out.detach.clear();
        return;
    }
    let mut used = [false; IDLE];
    for u in 0..UNITS {
        let mut missing = want[u].max(0) - have[u] - pending.map_or(0, |p| p[u]);
        while missing > 0 {
            let mut best: Option<usize> = None;
            for (k, candidate) in idle.iter().enumerate().take(IDLE) {
                if !used[k]
                    && candidate.unit == u as i32
                    && best.is_none_or(|b| candidate.cost < idle[b].cost)
                {
                    best = Some(k);
                }
            }
            let Some(best) = best else { break };
            used[best] = true;
            out.gather.push(idle[best]);
            missing -= 1;
        }
        if missing > 0 {
            out.build[u] = missing;
            out.build_total += missing as u32;
        }
    }
    if out.detach.is_empty() && out.gather.is_empty() && out.build_total == 0 {
        out.refusal = Refusal::Matches;
    }
}

// ---------------------------------------------------------------- session tracking (pure)

/// JOIN: a stack (one being built, or an idle one) marches to the target and merges
/// into it. FILL: a stack created for the extras receives them.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Kind {
    #[default]
    Join,
    Fill,
}

#[derive(Clone, Copy)]
pub struct CfTask {
    pub used: bool,
    pub kind: Kind,
    pub target: CfId,
    pub stack: CfId,
    /// JOIN of a regiment still being built.
    pub construction: CfId,
    pub unit: i32,
    /// JOIN: the recruitment province.
    pub origin: i32,
    pub regiments: usize,
    /// FILL: the extras.
    pub regiment: [CfId; REGIMENTS],
    pub appeared: bool,
    pub merge_sent: bool,
    pub construction_seen: bool,
    pub moved_to: i32,
    pub merge_date: u32,
    pub created_date: u32,
    pub misses: u32,
    /// FILL: TPL-4 checks the extras arrived.
    pub transfer_sent: bool,
    pub transfer_date: u32,
    pub retries: u32,
    /// 1-based saved order (TPL-7), 0 for none.
    pub record: usize,
}

impl CfTask {
    pub const EMPTY: CfTask = CfTask {
        used: false,
        kind: Kind::Join,
        target: CfId { kind: 0, id: 0 },
        stack: CfId { kind: 0, id: 0 },
        construction: CfId { kind: 0, id: 0 },
        unit: 0,
        origin: 0,
        regiments: 0,
        regiment: [CfId { kind: 0, id: 0 }; REGIMENTS],
        appeared: false,
        merge_sent: false,
        construction_seen: false,
        moved_to: 0,
        merge_date: 0,
        created_date: 0,
        misses: 0,
        transfer_sent: false,
        transfer_date: 0,
        retries: 0,
        record: 0,
    };
}

/// What the native layer reads each game day; ids only, never pointers kept.
#[derive(Clone, Copy, Default)]
pub struct CfSeen {
    pub target_alive: bool,
    pub target_province: i32,
    pub stack_alive: bool,
    pub stack_province: i32,
    pub construction_alive: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Wait,
    Move,
    Merge,
    Transfer,
    Done,
    TargetGone,
    Cancelled,
    Verify,
}

/// A stack that does not appear while its construction lives is still being built.
/// Two consecutive days with neither, once the construction was seen (or for a
/// gathered stack, which has none), means it was cancelled; one day of grace covers a
/// read between the two. The construction's own identity is inferred (symbols.md), so
/// one never seen only times out. A FILL stack is given a week to appear. A JOIN
/// re-sends Move only when the target changes province, so a user's own orders to the
/// new stack are not fought; it merges when both are in one province, at most one
/// Merge per game day.
pub fn step(t: &mut CfTask, s: &CfSeen, date: u32, province: &mut i32) -> Action {
    if !t.used {
        return Action::Done;
    }
    if !s.stack_alive {
        if t.kind == Kind::Fill && t.appeared {
            t.used = false;
            return Action::Verify;
        }
        if t.appeared {
            t.used = false;
            return Action::Done; // merged, or gone after it existed
        }
        if t.kind == Kind::Fill {
            if date >= t.created_date + 7 {
                t.used = false;
                return Action::Cancelled;
            }
            return Action::Wait;
        }
        if s.construction_alive {
            t.construction_seen = true;
            t.misses = 0;
            return Action::Wait;
        }
        if t.construction_seen || t.construction.none() {
            t.misses += 1;
            if t.misses >= 2 {
                t.used = false;
                return Action::Cancelled;
            }
            return Action::Wait;
        }
        if date >= t.created_date + BUILD_DAYS {
            t.used = false;
            return Action::Cancelled;
        }
        return Action::Wait;
    }
    t.appeared = true;
    t.misses = 0;
    if !s.target_alive {
        t.used = false;
        return Action::TargetGone;
    }
    // Create and Transfers are queued together. The daily driver verifies the
    // result; TRANSFER is also used for a retry into a surviving stack.
    if t.kind == Kind::Fill {
        if !t.transfer_sent {
            t.transfer_sent = true;
            t.transfer_date = date;
            return Action::Transfer;
        }
        if date == t.transfer_date {
            return Action::Wait;
        }
        t.used = false;
        return Action::Verify;
    }
    if s.stack_province == s.target_province {
        if t.merge_sent && t.merge_date == date {
            return Action::Wait;
        }
        t.merge_sent = true;
        t.merge_date = date;
        return Action::Merge;
    }
    if t.moved_to != s.target_province {
        t.moved_to = s.target_province;
        *province = s.target_province;
        return Action::Move;
    }
    Action::Wait
}

pub struct Tracker {
    pub task: [CfTask; TASKS],
}

/// A new task, as its index; None when the stack has one already or the table is full.
pub fn add(
    k: &mut Tracker,
    kind: Kind,
    target: CfId,
    stack: CfId,
    construction: CfId,
    unit: i32,
    date: u32,
) -> Option<usize> {
    if k.task.iter().any(|t| t.used && t.stack == stack) {
        return None; // one task per stack
    }
    let i = k.task.iter().position(|t| !t.used)?;
    k.task[i] = CfTask {
        used: true,
        kind,
        target,
        stack,
        construction,
        unit,
        origin: -1,
        moved_to: -1,
        created_date: date,
        ..CfTask::EMPTY
    };
    Some(i)
}

/// Regiments of each type already coming to this target (planner input). A recruit is
/// one regiment; a mobo join (MOBO-1) records how many its stack holds.
pub fn pending(k: &Tracker, target: CfId) -> [i32; UNITS] {
    let mut pending = [0; UNITS];
    for t in k
        .task
        .iter()
        .filter(|t| t.used && t.kind == Kind::Join && t.target == target)
    {
        if let Some(u) = valid_unit(t.unit) {
            pending[u] += t.regiments.max(1) as i32;
        }
    }
    pending
}

// ---------------------------------------------------------------- saved orders (pure; TPL-7, D13 revised)

// A JOIN of a regiment being built, kept in a local file so a reload resumes it. The
// save stores a construction's own id and its future stack id ("unit"), so both
// survive. An order applies to a loaded game of the same tag whose date lies from its
// creation up to (not including) its end: a save made before the order, or after it
// finished, never resumes it.
pub const ORDERS_MAX: usize = 512;

#[derive(Clone, Copy, Default)]
pub struct CfOrder {
    pub tag: u32,
    pub created: u32,
    pub done: u32,
    pub target: CfId,
    pub stack: CfId,
    pub construction: CfId,
    pub unit: i32,
    pub active: bool,
}

pub fn order_applies(r: &CfOrder, tag: u32, date: u32) -> bool {
    r.tag == tag && r.created <= date && (r.done == 0 || date < r.done)
}

/// Full: the oldest finished order goes first, else the oldest open one. Returns the
/// new order's index and the index removed (later ones moved down one), if any.
pub fn order_add(k: &mut Vec<CfOrder>, r: CfOrder) -> (usize, Option<usize>) {
    let mut dropped = None;
    if k.len() >= ORDERS_MAX {
        let (mut drop, mut done) = (0, false);
        for (i, c) in k.iter().enumerate() {
            let d = c.done != 0;
            if (d && !done) || (d == done && c.created < k[drop].created) {
                drop = i;
                done = d;
            }
        }
        k.remove(drop);
        dropped = Some(drop);
    }
    k.push(CfOrder { active: false, ..r });
    (k.len() - 1, dropped)
}

fn order_line(r: &CfOrder) -> String {
    format!(
        "order tag={:08X} created={} done={} target={}/{} stack={}/{} construction={}/{} unit={}\r\n",
        r.tag,
        r.created,
        r.done,
        r.target.kind,
        r.target.id,
        r.stack.kind,
        r.stack.id,
        r.construction.kind,
        r.construction.id,
        r.unit
    )
}

const HEADER: &str = "tgcx_conform_orders 1\r\n";

/// The file's text, or None when it would not fit in `size` bytes.
pub fn orders_write(k: &[CfOrder], size: usize) -> Option<String> {
    let mut out = String::from(HEADER);
    for r in k {
        out.push_str(&order_line(r));
    }
    (out.len() < size).then_some(out)
}

fn id_pair(s: &str) -> Option<CfId> {
    let (kind, id) = s.split_once('/')?;
    Some(CfId {
        kind: kind.parse().ok()?,
        id: id.parse().ok()?,
    })
}

/// The fields of one canonical order line; the round trip below rejects anything else.
fn parse_line(line: &str) -> Option<CfOrder> {
    let mut r = CfOrder::default();
    let rest = line.strip_prefix("order tag=")?;
    let (tag, rest) = rest.split_once(" created=")?;
    r.tag = u32::from_str_radix(tag, 16).ok()?;
    let (created, rest) = rest.split_once(" done=")?;
    r.created = created.parse().ok()?;
    let (done, rest) = rest.split_once(" target=")?;
    r.done = done.parse().ok()?;
    let (target, rest) = rest.split_once(" stack=")?;
    r.target = id_pair(target)?;
    let (stack, rest) = rest.split_once(" construction=")?;
    r.stack = id_pair(stack)?;
    let (construction, unit) = rest.split_once(" unit=")?;
    r.construction = id_pair(construction)?;
    r.unit = unit.parse().ok()?;
    Some(r)
}

/// Strict: every line must print back byte for byte; anything else refuses the whole
/// file (nothing is resumed from a damaged one).
pub fn orders_parse(text: &[u8]) -> Option<Vec<CfOrder>> {
    let text = text.split(|&c| c == 0).next().unwrap_or_default();
    let rest = text.strip_prefix(HEADER.as_bytes())?;
    let mut out = Vec::new();
    let mut at = rest;
    while !at.is_empty() {
        let end = at.windows(2).position(|w| w == b"\r\n")?;
        if end >= 190 || out.len() >= ORDERS_MAX {
            return None;
        }
        let line = std::str::from_utf8(&at[..end]).ok()?;
        let r = parse_line(line)?;
        if order_line(&r).as_bytes() != &at[..end + 2] {
            return None;
        }
        if r.target.none() || r.stack.none() || valid_unit(r.unit).is_none() {
            return None;
        }
        out.push(r);
        at = &at[end + 2..];
    }
    Some(out)
}

// ---------------------------------------------------------------- mobo stacks (pure; MOBO-1)

// A stack marked to be completed with infantry when its country mobilizes. The mark is
// kept in a local file like the saved orders (outside the save), with the same date
// window: it applies to a loaded game of the same tag whose date lies from its creation
// up to (not including) its end, so a save made before the mark, or after it was
// removed, never brings it back. `infantry` is how many infantry regiments the template
// the stack was marked with asks for.
pub const MARKS_MAX: usize = 256;

#[derive(Clone, Copy, Default)]
pub struct CfMark {
    pub tag: u32,
    pub created: u32,
    pub done: u32,
    pub stack: CfId,
    pub infantry: i32,
    /// This session tracks it: only an active mark is ended when its stack is gone, so a
    /// mark of another save's timeline is left as it is.
    pub active: bool,
}

pub fn mark_applies(r: &CfMark, tag: u32, date: u32) -> bool {
    r.tag == tag && r.created <= date && (r.done == 0 || date < r.done)
}

/// Full: the oldest ended mark goes first, else the oldest open one. Returns the new
/// mark's index (an eviction moves later marks down one).
pub fn mark_add(k: &mut Vec<CfMark>, r: CfMark) -> usize {
    if k.len() >= MARKS_MAX {
        let (mut drop, mut done) = (0, false);
        for (i, c) in k.iter().enumerate() {
            let d = c.done != 0;
            if (d && !done) || (d == done && c.created < k[drop].created) {
                drop = i;
                done = d;
            }
        }
        k.remove(drop);
    }
    k.push(CfMark { active: false, ..r });
    k.len() - 1
}

fn mark_line(r: &CfMark) -> String {
    format!(
        "mark tag={:08X} created={} done={} stack={}/{} infantry={}\r\n",
        r.tag, r.created, r.done, r.stack.kind, r.stack.id, r.infantry
    )
}

const MARKS_HEADER: &str = "tgcx_mobo_marks 1\r\n";

/// The file's text, or None when it would not fit in `size` bytes.
pub fn marks_write(k: &[CfMark], size: usize) -> Option<String> {
    let mut out = String::from(MARKS_HEADER);
    for r in k {
        out.push_str(&mark_line(r));
    }
    (out.len() < size).then_some(out)
}

fn parse_mark(line: &str) -> Option<CfMark> {
    let mut r = CfMark::default();
    let rest = line.strip_prefix("mark tag=")?;
    let (tag, rest) = rest.split_once(" created=")?;
    r.tag = u32::from_str_radix(tag, 16).ok()?;
    let (created, rest) = rest.split_once(" done=")?;
    r.created = created.parse().ok()?;
    let (done, rest) = rest.split_once(" stack=")?;
    r.done = done.parse().ok()?;
    let (stack, infantry) = rest.split_once(" infantry=")?;
    r.stack = id_pair(stack)?;
    r.infantry = infantry.parse().ok()?;
    Some(r)
}

/// Strict, as the orders: every line must print back byte for byte, or the whole file
/// is refused.
pub fn marks_parse(text: &[u8]) -> Option<Vec<CfMark>> {
    let text = text.split(|&c| c == 0).next().unwrap_or_default();
    let mut at = text.strip_prefix(MARKS_HEADER.as_bytes())?;
    let mut out = Vec::new();
    while !at.is_empty() {
        let end = at.windows(2).position(|w| w == b"\r\n")?;
        if end >= 190 || out.len() >= MARKS_MAX {
            return None;
        }
        let r = parse_mark(std::str::from_utf8(&at[..end]).ok()?)?;
        if mark_line(&r).as_bytes() != &at[..end + 2] || r.stack.none() || r.infantry <= 0 {
            return None;
        }
        out.push(r);
        at = &at[end + 2..];
    }
    Some(out)
}

/// An infantry stack that appeared while its country mobilized: how many regiments it
/// holds, and for each marked stack the length of its safe land route there (None: no
/// route).
pub struct MoboCandidate {
    pub regiments: i32,
    pub route: Vec<Option<u32>>,
}

/// Pairs (candidate, marked stack), nearest route first, each candidate once. A stack
/// joins only if all its regiments fit in what the marked stack still needs, so no
/// template is overshot; one that fits nowhere stays where it is.
pub fn mobo_assign(need: &[i32], candidates: &[MoboCandidate]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for (c, candidate) in candidates.iter().enumerate() {
        for (m, route) in candidate.route.iter().enumerate().take(need.len()) {
            if let Some(length) = route {
                pairs.push((*length, c, m));
            }
        }
    }
    pairs.sort_unstable();
    let mut need = need.to_vec();
    let mut taken = vec![false; candidates.len()];
    let mut out = Vec::new();
    for (_, c, m) in pairs {
        let regiments = candidates[c].regiments;
        if !taken[c] && regiments > 0 && need[m] >= regiments {
            taken[c] = true;
            need[m] -= regiments;
            out.push((c, m));
        }
    }
    out
}
