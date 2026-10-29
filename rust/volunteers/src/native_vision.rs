//! Read-only plan for local map visibility. FUN_0068C040 establishes two
//! distinct fields: province+174 is per-country simulation knowledge, while
//! map-province+360 follows the selected local viewer. Never write +174 here.
//! The renderer application/dirty-texture callback remains a separate adapter.
use crate::registry::Registry;
use crate::session::{SessionMode, SinglePlayerPermit};
use crate::snapshot::Memory;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grant {
    pub province: u32,
    pub map: u32,
    pub native_level: u32,
    pub requested_level: u32,
}
#[derive(Clone, Debug)]
pub struct Plan {
    pub world: u32,
    pub viewer: u32,
    pub epoch: u64,
    pub grants: Vec<Grant>,
    pub skipped_provinces: u32,
}
fn field(m: &impl Memory, p: u32, offset: u32) -> Option<u32> {
    m.read32(p.checked_add(offset)?)
}
pub fn plan(
    m: &impl Memory,
    base: u32,
    registry: &Registry,
    permit: SinglePlayerPermit,
) -> Option<Plan> {
    plan_with_fronts(m, base, registry, permit, &[])
}
/// `fronts` contains receipt-validated volunteer province pointers. Local
/// display vision follows recipient knowledge and these armies' surroundings;
/// the simulation's per-country visibility array is read, never written.
pub fn plan_with_fronts(
    m: &impl Memory,
    base: u32,
    registry: &Registry,
    permit: SinglePlayerPermit,
    fronts: &[u32],
) -> Option<Plan> {
    let session = crate::native_session::capture(m, base)?;
    if session.mode != SessionMode::SinglePlayer || registry.epoch != permit.epoch() {
        return None;
    }
    let world = session.world;
    let viewer = field(m, world, 0xB60)?;
    let manager = m.read32(base.checked_add(0xE587E4)?)?;
    let count = m.read32(manager)?;
    if count > 4096 {
        return None;
    }
    let countries = field(m, manager, 4)?;
    let start = field(m, world, 0xACC)?;
    let end = field(m, world, 0xAD0)?;
    let bytes = end.checked_sub(start)?;
    if start == 0 || bytes % 4 != 0 || bytes / 4 > 65536 {
        return None;
    }
    let recipients = registry
        .entries()
        .filter_map(|entry| {
            let owner = entry.deployment.key.recipient;
            (owner > 0 && owner < count && registry.sees(viewer, owner, false)).then_some(owner)
        })
        .collect::<BTreeSet<_>>();
    let recipients = recipients
        .into_iter()
        .map(|owner| {
            let country = field(m, countries, owner.checked_mul(4)?)?;
            Some((owner, field(m, country, 0x67C)? as u8 != 0))
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .filter_map(|(i, war)| war.then_some(i))
        .collect::<Vec<_>>();
    let mut grants = Vec::new();
    if recipients.is_empty() {
        return Some(Plan {
            world,
            viewer,
            epoch: registry.epoch,
            grants,
            skipped_provinces: 0,
        });
    }
    // The global pointer table is one bounded bulk read, not one allocation and
    // ReadProcessMemory syscall for every slot. Scalar Native reads use SEH.
    let provinces = m.read_words(start, bytes as usize / 4)?;
    let mut near = BTreeSet::new();
    for &province in fronts {
        near.insert(field(m, province, 0x58)?);
        let map = field(m, province, 0x5C)?;
        let edges = field(m, map, 0xC8)?;
        let end = field(m, map, 0xCC)?;
        let size = end.checked_sub(edges)?;
        if size % 0x24 != 0 || size / 0x24 > 256 {
            return None;
        }
        for offset in (0..size).step_by(0x24) {
            let id = field(m, edges, offset + 4)?;
            if id < provinces.len() as u32 {
                near.insert(id);
            }
        }
    }
    let mut skipped_provinces = 0;
    for (id, &province) in provinces.iter().enumerate() {
        if province == 0 {
            continue;
        }
        // A malformed/dummy province cannot discard independently validated
        // grants elsewhere. Skip only that province, never force unknown data.
        let grant = (|| -> Option<Option<Grant>> {
            let levels = field(m, province, 0x174)?;
            let length = field(m, province, 0x178)?;
            if length > 4096 || (length != 0 && levels == 0) {
                return None;
            }
            let mut requested = u32::from(near.contains(&(id as u32))) * 2;
            // Native alliance sight reveals recipient-controlled land too. The
            // REB controller special case follows FUN_0068C040 VA68C1EA..68C231.
            let owner = field(m, province, 0x12C)?;
            let mut controller = field(m, province, 0x134)?;
            if field(m, province, 0x130)? & 0xFFFFFF == u32::from_le_bytes(*b"REB\0") & 0xFFFFFF {
                controller = owner;
            }
            if recipients.contains(&controller) {
                requested = 2;
            }
            for &recipient in &recipients {
                // Native recompute explicitly permits empty per-country arrays
                // (dummy/uninitialized provinces). One such province must not
                // discard valid grants for the entire world.
                if length == 0 {
                    continue;
                }
                if recipient >= length {
                    continue;
                }
                let value = field(m, levels, recipient.checked_mul(4)?)?;
                if value > 2 {
                    return None;
                }
                requested = requested.max(value);
            }
            if requested != 0 {
                let map = field(m, province, 0x5C)?;
                if ![base.checked_add(0x9FD310)?, base.checked_add(0x9FD338)?]
                    .contains(&m.read32(map)?)
                {
                    return None;
                }
                let level = field(m, map, 0x360)?;
                if level > 2 {
                    return None;
                }
                if level < requested {
                    return Some(Some(Grant {
                        province: field(m, province, 0x58)?,
                        map,
                        native_level: level,
                        requested_level: requested,
                    }));
                }
            }
            Some(None)
        })();
        match grant {
            Some(Some(grant)) => grants.push(grant),
            Some(None) => (),
            None => skipped_provinces += 1,
        }
    }
    if m.read32(base.checked_add(0xE588E8)?)? != world || field(m, world, 0xB60)? != viewer {
        return None;
    }
    Some(Plan {
        world,
        viewer,
        epoch: registry.epoch,
        grants,
        skipped_provinces,
    })
}
