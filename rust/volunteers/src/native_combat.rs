//! Donor-owned land combat eligibility at FUN_005CBF60 / FUN_005DBB50.
//! Read-only adapter; no inline hook is installed by this module. It never
//! changes real owner fields and cannot borrow context in a network session.
use crate::combat::{Decision, OtherGates};
use crate::mission::{RecipientWar, StackReceipt};
use crate::model::Identity;
use crate::registry::Registry;
use crate::session::{SessionMode, SinglePlayerPermit};
use crate::snapshot::{Memory, capture_stack_receipt};

fn field(m: &impl Memory, p: u32, offset: u32) -> Option<u32> {
    m.read32(p.checked_add(offset)?)
}
fn byte(m: &impl Memory, p: u32, offset: u32) -> Option<u8> {
    field(m, p, offset).map(|v| v as u8)
}
fn country(m: &impl Memory, base: u32, index: u32) -> Option<u32> {
    let manager = m.read32(base.checked_add(0xE587E4)?)?;
    let count = m.read32(manager)?;
    if count > 4096 || index >= count {
        return None;
    }
    let p = field(m, field(m, manager, 4)?, index.checked_mul(4)?)?;
    (p != 0 && field(m, p, 0x20)? == index).then_some(p)
}
fn war(m: &impl Memory, base: u32, index: u32) -> RecipientWar {
    match country(m, base, index).and_then(|c| byte(m, c, 0x67C)) {
        Some(0) => RecipientWar::AtPeace,
        Some(_) => RecipientWar::AtWar,
        None => RecipientWar::Unknown,
    }
}
fn receipt(m: &impl Memory, base: u32, r: &Registry, army: u32) -> Option<StackReceipt> {
    let id = Identity {
        kind: field(m, army, 0x10)?,
        id: field(m, army, 0x14)?,
    };
    if r.live_army(id).is_some() {
        capture_stack_receipt(m, base, r.campaign, army, 64)
    } else {
        Some(StackReceipt {
            campaign: r.campaign,
            army: id,
            owner: field(m, army, 0xC4)?,
            affiliation: field(m, army, 0xCC)?,
            regiments: Vec::new(),
        })
    }
}
/// A negative filter only. It never grants authority or caches a native result;
/// relevant stacks must still pass the complete fresh session/receipt gates.
pub fn references_loan(m: &impl Memory, registry: &Registry, army: u32) -> Option<bool> {
    let id = Identity {
        kind: field(m, army, 0x10)?,
        id: field(m, army, 0x14)?,
    };
    Some(registry.live_army(id).is_some())
}
pub fn pair_references_loan(
    m: &impl Memory,
    registry: &Registry,
    left: u32,
    right: u32,
) -> Option<bool> {
    Some(references_loan(m, registry, left)? || references_loan(m, registry, right)?)
}
fn land_gates(m: &impl Memory, base: u32, army: u32, province: u32) -> Option<bool> {
    if m.read32(army)? != base.checked_add(0x9F88CC)? {
        return None;
    }
    let vt = m.read32(army)?;
    // Land-only scope. Native slot11 returns ECX, slot14 returns1, slots15/16
    // return0. Refuse changed virtual semantics instead of guessing their ABI.
    if field(m, vt, 0x2C)? != base.checked_add(0x1EC4A0)?
        || field(m, vt, 0x38)? != base.checked_add(0x631490)?
        || field(m, vt, 0x3C)? != base.checked_add(0x18CFD0)?
        || field(m, vt, 0x40)? != base.checked_add(0x18CFD0)?
    {
        return None;
    }
    let map = field(m, province, 0x5C)?;
    Some(
        byte(m, army, 0x164)? == 0
            && byte(m, army, 0x104)? == 0
            && byte(m, map, 0x2A)? != 0
            && ((field(m, army, 0xEC)? as i32) > 0 || field(m, army, 0x1B4)? == 0),
    )
}

/// `province` is the initial-pair argument, or the existing side member's +DC
/// province for a join check. Combat side construction/result attribution remain
/// separate prerequisites; this predicate alone must never enable Send.
pub fn evaluate(
    m: &impl Memory,
    base: u32,
    registry: &Registry,
    permit: SinglePlayerPermit,
    left: u32,
    right: u32,
    province: u32,
) -> Decision {
    if pair_references_loan(m, registry, left, right) != Some(true) {
        return Decision::Native;
    }
    let Some(session) = crate::native_session::capture(m, base) else {
        return Decision::Native;
    };
    if session.mode != SessionMode::SinglePlayer || permit.epoch() != registry.epoch {
        return Decision::Native;
    }
    let (Some(a), Some(b)) = (
        receipt(m, base, registry, left),
        receipt(m, base, registry, right),
    ) else {
        return Decision::Native;
    };
    // Donor entry into war ends neutral service; do not wait for a UI frame to
    // update the local coordinator before ceasing to borrow recipient context.
    if registry.entries().any(|e| {
        [a.army, b.army].contains(&e.mission.plan().destination())
            && war(m, base, e.deployment.key.donor) != RecipientWar::AtPeace
    }) {
        return Decision::Native;
    }
    let gates = match (
        land_gates(m, base, left, province),
        land_gates(m, base, right, province),
    ) {
        (Some(true), Some(true)) => match (field(m, left, 0xD4), field(m, right, 0xD4)) {
            (Some(x), Some(y)) if x != b.owner && y != a.owner => OtherGates::Passed,
            (Some(_), Some(_)) => OtherGates::Rejected,
            _ => OtherGates::Unestablished,
        },
        (Some(false), _) | (_, Some(false)) => OtherGates::Rejected,
        _ => OtherGates::Unestablished,
    };
    crate::combat::pair(
        registry,
        &a,
        &b,
        gates,
        |i| war(m, base, i),
        |x, y| {
            let a = country(m, base, x)?;
            let b = country(m, base, y)?;
            if x == y {
                return Some(false);
            }
            let rebel = u32::from_le_bytes(*b"REB\0") & 0xFFFFFF;
            if field(m, a, 0x1C)? & 0xFFFFFF == rebel || field(m, b, 0x1C)? & 0xFFFFFF == rebel {
                return Some(true);
            }
            let relations = field(m, a, 0xBE8)?;
            let relation = field(m, relations, y.checked_mul(4)?)?;
            if relation == 0 {
                return None;
            }
            Some(field(m, relation, 0x34)? != 0)
        },
    )
}
