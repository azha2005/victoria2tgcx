//! Bounded read-only observations. Offsets: volunteer research 2026-10-26.
//! Read failures are explicit; this module never dereferences a game pointer.
use crate::mission::{Campaign, StackReceipt};
use crate::model::Identity;

pub trait Memory {
    fn read32(&self, address: u32) -> Option<u32>;
    fn read_words(&self, address: u32, count: usize) -> Option<Vec<u32>> {
        if count > 65536 {
            return None;
        }
        (0..count)
            .map(|i| self.read32(address.checked_add((i as u32).checked_mul(4)?)?))
            .collect()
    }
}
fn field<M: Memory>(m: &M, p: u32, offset: u32) -> Option<u32> {
    m.read32(p.checked_add(offset)?)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Army {
    pub identity: Identity,
    pub owner: u32,
    pub affiliation: u32,
    pub province: u32,
    pub regiments: u32,
    pub in_combat: bool,
    pub first_pop_owner: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub world: u32,
    pub date: u32,
    pub player: u32,
    pub countries_at_war: u32,
    pub selected: Vec<Army>,
}

/// Read a complete dedicated stack for a mission receipt. The native observer
/// does not activate missions with this helper. Callers must establish campaign
/// identity independently; an opaque stamp is not save/load persistence.
pub fn capture_stack_receipt<M: Memory>(
    m: &M,
    base: u32,
    campaign: Campaign,
    army: u32,
    limit: usize,
) -> Option<StackReceipt> {
    if !(1..=64).contains(&limit)
        || army == 0
        || field(m, army, 0)? != base.checked_add(0x9F88CC)?
    {
        return None;
    }
    let identity = Identity {
        kind: field(m, army, 0x10)?,
        id: field(m, army, 0x14)?,
    };
    if !identity.valid() {
        return None;
    }
    let count = field(m, army, 0x40)? as usize;
    if count > limit {
        return None;
    }
    let mut node = field(m, army, 0x38)?;
    let mut seen = Vec::new();
    let mut regiments = Vec::new();
    while node != 0 {
        if seen.len() >= limit || seen.contains(&node) || field(m, node, 0xC)? & 0xFF != 0 {
            return None;
        }
        seen.push(node);
        let regiment = field(m, node, 0)?;
        if regiment == 0
            || field(m, regiment, 0)? != base.checked_add(0x9F8884)?
            || field(m, regiment, 0x74)? != army
        {
            return None;
        }
        let id = Identity {
            kind: field(m, regiment, 8)?,
            id: field(m, regiment, 0xC)?,
        };
        if !id.valid() || regiments.iter().any(|(previous, _)| *previous == id) {
            return None;
        }
        regiments.push((id, field(m, regiment, 0x64)?));
        node = field(m, node, 8)?;
    }
    if regiments.len() != count {
        return None;
    }
    Some(StackReceipt {
        campaign,
        army: identity,
        owner: field(m, army, 0xC4)?,
        affiliation: field(m, army, 0xCC)?,
        regiments,
    })
}

/// Only exact CArmy selections are inspected; invalid/cyclic/excessive lists
/// invalidate the sample instead of silently presenting a partial inventory.
pub fn capture<M: Memory>(m: &M, base: u32) -> Option<Snapshot> {
    let world = m.read32(base.checked_add(0xE588E8)?)?;
    let manager = m.read32(base.checked_add(0xE587E4)?)?;
    if world == 0 || manager == 0 {
        return None;
    }
    let date = field(m, world, 0xB0C)?;
    let player = field(m, world, 0xB60)?;
    let count = field(m, manager, 0)?;
    let countries = field(m, manager, 4)?;
    if count == 0 || count > 4096 || player == 0 || player >= count || countries == 0 {
        return None;
    }
    let mut wars = 0;
    for i in 1..count {
        let country = m.read32(countries.checked_add(i.checked_mul(4)?)?)?;
        if country != 0 {
            wars += u32::from(field(m, country, 0x67C)? & 0xFF != 0);
        }
    }
    let context = field(m, world, 0xB24)?;
    if context == 0 || field(m, context, 0)? != base.checked_add(0xA0A34C)? {
        return None;
    }
    let selected_count = field(m, context, 0x1608)?;
    if selected_count > 64 {
        return None;
    }
    let mut node = field(m, context, 0x1600)?;
    let mut seen = Vec::new();
    let mut selected = Vec::new();
    while node != 0 {
        if seen.len() >= 64 || seen.contains(&node) {
            return None;
        }
        seen.push(node);
        let unit = field(m, node, 0)?;
        let deferred = field(m, node, 0xC)? & 0xFF != 0;
        if unit != 0 && !deferred && field(m, unit, 0)? == base.checked_add(0x9F88CC)? {
            let province = field(m, unit, 0xDC)?;
            let regiments = field(m, unit, 0x40)?;
            if province == 0 || regiments > 10000 {
                return None;
            }
            let first = field(m, unit, 0x38)?;
            let first_pop_owner = if first != 0 {
                let reg = field(m, first, 0)?;
                let pop = if reg != 0 { field(m, reg, 0x30)? } else { 0 };
                let p = if pop != 0 { field(m, pop, 0x64)? } else { 0 };
                if p != 0 {
                    Some(field(m, p, 0x12C)?)
                } else {
                    None
                }
            } else {
                None
            };
            let identity = Identity {
                kind: field(m, unit, 0x10)?,
                id: field(m, unit, 0x14)?,
            };
            if !identity.valid() {
                return None;
            }
            selected.push(Army {
                identity,
                owner: field(m, unit, 0xC4)?,
                affiliation: field(m, unit, 0xCC)?,
                province: field(m, province, 0x58)?,
                regiments,
                in_combat: field(m, unit, 0x74)? != 0,
                first_pop_owner,
            });
        }
        node = field(m, node, 8)?;
    }
    if seen.len() != selected_count as usize {
        return None;
    }
    Some(Snapshot {
        world,
        date,
        player,
        countries_at_war: wars,
        selected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    impl Memory for BTreeMap<u32, u32> {
        fn read32(&self, a: u32) -> Option<u32> {
            self.get(&a).copied()
        }
    }
    fn fixture() -> BTreeMap<u32, u32> {
        BTreeMap::from([
            (0xE588E8, 0x10000),
            (0xE587E4, 0x20000),
            (0x10B0C, 43800024),
            (0x10B60, 1),
            (0x20000, 3),
            (0x20004, 0x30000),
            (0x30004, 0x40000),
            (0x30008, 0x50000),
            (0x4067C, 0),
            (0x5067C, 1),
            (0x10B24, 0x60000),
            (0x60000, 0xA0A34C),
            (0x61608, 1),
            (0x61600, 0x70000),
            (0x70000, 0x80000),
            (0x70008, 0),
            (0x7000C, 0),
            (0x80000, 0x9F88CC),
            (0x800DC, 0x90000),
            (0x90058, 42),
            (0x80040, 1),
            (0x80038, 0xA0000),
            (0xA0000, 0xB0000),
            (0xB0030, 0xC0000),
            (0xC0064, 0xD0000),
            (0xD012C, 1),
            (0x80010, 40),
            (0x80014, 3),
            (0x800C4, 2),
            (0x800CC, 2),
            (0x80074, 0),
        ])
    }
    #[test]
    fn captures_foreign_pop_affiliation_without_changing_any_memory() {
        let m = fixture();
        let before = m.clone();
        let s = capture(&m, 0).unwrap();
        assert_eq!(s.countries_at_war, 1);
        assert_eq!(s.selected[0].owner, 2);
        assert_eq!(s.selected[0].first_pop_owner, Some(1));
        assert_eq!(m, before);
    }
    #[test]
    fn rejects_stale_ui_cycles_counts_and_missing_reads() {
        for (address, value) in [
            (0x60000, 0),
            (0x70008, 0x70000),
            (0x61608, 65),
            (0x61608, 2),
        ] {
            let mut m = fixture();
            m.insert(address, value);
            assert!(capture(&m, 0).is_none());
        }
        let mut m = fixture();
        m.remove(&0x80014);
        assert!(capture(&m, 0).is_none());
    }
    #[test]
    fn does_not_use_a_regiment_or_other_selectable_as_an_army() {
        let mut m = fixture();
        m.insert(0x80000, 0x9F8884);
        assert!(capture(&m, 0).unwrap().selected.is_empty());
    }
    #[test]
    fn full_receipt_requires_native_regiment_type_parent_and_exact_list() {
        let mut m = fixture();
        m.extend([
            (0xA000C, 0),
            (0xA0008, 0),
            (0xB0000, 0x9F8884),
            (0xB0008, 40),
            (0xB000C, 7),
            (0xB0074, 0x80000),
            (0xB0064, 2),
        ]);
        let receipt = capture_stack_receipt(&m, 0, Campaign([1; 16]), 0x80000, 5).unwrap();
        assert_eq!(receipt.regiments, vec![(Identity { kind: 40, id: 7 }, 2)]);
        assert_eq!(receipt.owner, 2);
        for (address, invalid) in [
            (0xB0000, 0x9F88CC),
            (0xB0074, 0x90000),
            (0xA0008, 0xA0000),
            (0xA000C, 1),
            (0x80040, 2),
        ] {
            let mut invalid_memory = m.clone();
            invalid_memory.insert(address, invalid);
            assert!(
                capture_stack_receipt(&invalid_memory, 0, Campaign([1; 16]), 0x80000, 5).is_none()
            );
        }
    }
}
