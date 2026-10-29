//! Pure scoped combat-policy bridge. Native adapters must establish the point
//! where all unrelated eligibility checks passed; this never bypasses them.
use crate::mission::{RecipientWar, StackReceipt, Use};
use crate::registry::Registry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtherGates {
    Unestablished,
    Rejected,
    Passed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Native,
    Reject,
    Scoped {
        left_country: u32,
        right_country: u32,
        hostile: bool,
    },
}

pub fn pair(
    registry: &Registry,
    left: &StackReceipt,
    right: &StackReceipt,
    gates: OtherGates,
    mut war: impl FnMut(u32) -> RecipientWar,
    mut hostile: impl FnMut(u32, u32) -> Option<bool>,
) -> Decision {
    if left.campaign != registry.campaign || right.campaign != registry.campaign {
        return Decision::Native;
    }
    let left_context = registry.context(left, Use::Combat, war_for(registry, left, &mut war));
    let right_context = registry.context(right, Use::Combat, war_for(registry, right, &mut war));
    if left_context.is_none() && right_context.is_none() {
        return Decision::Native;
    }
    if gates == OtherGates::Rejected {
        return Decision::Reject;
    }
    if gates != OtherGates::Passed {
        return Decision::Native;
    }
    // Do not manufacture a civil war between brigades of the same donor, even
    // if two loan missions name opposing hosts. Never fight the host as itself.
    if left.owner == right.owner || left.army == right.army {
        return Decision::Reject;
    }
    let a = left_context.unwrap_or(left.owner);
    let b = right_context.unwrap_or(right.owner);
    if a == b {
        return Decision::Reject;
    }
    match hostile(a, b) {
        Some(value) => Decision::Scoped {
            left_country: a,
            right_country: b,
            hostile: value,
        },
        None => Decision::Native,
    }
}
fn war_for(
    registry: &Registry,
    receipt: &StackReceipt,
    war: &mut impl FnMut(u32) -> RecipientWar,
) -> RecipientWar {
    registry
        .live_army(receipt.army)
        .map_or(RecipientWar::Unknown, |e| war(e.deployment.key.recipient))
}
