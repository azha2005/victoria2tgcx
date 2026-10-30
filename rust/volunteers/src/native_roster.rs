//! Native human list: 68B470/64A460/41A100; controller table: 68B6C0/68B870.
//! No field is written. Local source is cross-checked against manager+54.
use crate::{
    mp_authority::{Peer, Roster},
    snapshot::Memory,
};
use std::collections::BTreeSet;
fn field(m: &impl Memory, p: u32, o: u32) -> Option<u32> {
    m.read32(p.checked_add(o)?)
}
pub fn capture(m: &impl Memory, base: u32) -> Option<Roster> {
    let session = crate::native_session::probe_queue(m, base)?;
    let world = session.world;
    let local_country = field(m, world, 0xB60)?;
    let local_source = u16::try_from(field(m, session.manager, 0x54)?).ok()?;
    if local_source > 0x7FFF {
        return None;
    }
    let countries = m.read32(base.checked_add(0xE587E4)?)?;
    let count = m.read32(countries)?;
    if count > 4096 {
        return None;
    }
    let country_table = field(m, countries, 4)?;
    let start = field(m, world, 0xAEC)?;
    let end = field(m, world, 0xAF0)?;
    let len = end.checked_sub(start)?;
    if start == 0 || len % 4 != 0 || len / 4 != count {
        return None;
    }
    let head = field(m, world, 0x18)?;
    let tail = field(m, world, 0x1C)?;
    let size = field(m, world, 0x20)?;
    if !(2..=32).contains(&size) {
        return None;
    }
    let mut node = head;
    let mut previous = 0;
    let mut seen = BTreeSet::new();
    let mut peers = Vec::new();
    while node != 0 {
        if peers.len() >= size as usize || !seen.insert(node) || field(m, node, 0x38)? != previous {
            return None;
        }
        let source = u16::try_from(field(m, node, 0x1C)?).ok()?;
        let country = field(m, node, 0x24)?;
        let tag = field(m, node, 0x20)?;
        if country == 0 || country >= count || field(m, start, country.checked_mul(4)?)? == 0 {
            return None;
        }
        let p = field(m, country_table, country * 4)?;
        if field(m, p, 0x20)? != country || field(m, p, 0x1C)? != tag {
            return None;
        }
        peers.push(Peer {
            source,
            country,
            tag,
        });
        previous = node;
        node = field(m, node, 0x3C)?;
    }
    if previous != tail || peers.len() != size as usize {
        return None;
    }
    let mut human = BTreeSet::new();
    for country in 0..count {
        if field(m, start, country * 4)? != 0 {
            human.insert(country);
        }
    }
    if human != peers.iter().map(|p| p.country).collect() {
        return None;
    }
    let local = *peers
        .iter()
        .find(|p| p.country == local_country && p.source == local_source)?;
    if field(m, start, local_country * 4)? != 1 {
        return None;
    }
    let roster = Roster { peers, local };
    roster.canonical()?;
    if m.read32(base + 0xE588E8)? != world
        || field(m, world, 0x18)? != head
        || field(m, world, 0x1C)? != tail
        || field(m, world, 0x20)? != size
        || field(m, world, 0xAEC)? != start
        || field(m, world, 0xAF0)? != end
    {
        return None;
    }
    Some(roster)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    struct Fixture(BTreeMap<u32, u32>);
    impl Memory for Fixture {
        fn read32(&self, p: u32) -> Option<u32> {
            self.0.get(&p).copied()
        }
    }
    fn fixture(local: u32) -> Fixture {
        let base = 0x80000;
        let w = 0x2000000;
        let manager = 0x2600000;
        let mut f = Fixture(BTreeMap::from([
            (base + 0xE588E8, w),
            (base + 0xE587E4, 0x2100000),
            (0x2100000, 3),
            (0x2100004, 0x2200000),
            (w + 0xB60, local),
            (w + 0xB24, 0x2400000),
            (0x2400000, base + 0xA0A34C),
            (0x24017E8, 0x2500000),
            (0x250012C, manager),
            (manager, base + 0xA390D0),
            (manager + 0x114, base + 0xA0A168),
            (manager + 0x118, 0x18D),
            (manager + 0x2C, 4),
            (manager + 0x180, if local == 1 { 0 } else { 1 }),
            (manager + 0x38, 0x2700000),
            (manager + 0x54, if local == 1 { 7 } else { 9 }),
            (
                0x2700000,
                base + if local == 1 { 0xA42728 } else { 0xA427C8 },
            ),
            (0x270001C, manager),
            (base + 0xA42728 + 24, base + 0x662EA0),
            (base + 0xA427C8 + 24, base + 0x666B30),
            (w + 0xAEC, 0x2800000),
            (w + 0xAF0, 0x280000C),
            (0x2800000, 0),
            (0x2800004, if local == 1 { 1 } else { 2 }),
            (0x2800008, if local == 2 { 1 } else { 2 }),
            (w + 0x18, 0x2900000),
            (w + 0x1C, 0x2910000),
            (w + 0x20, 2),
            (0x2900038, 0),
            (0x290003C, 0x2910000),
            (0x290001C, 7),
            (0x2900020, 0x435241),
            (0x2900024, 1),
            (0x2910038, 0x2900000),
            (0x291003C, 0),
            (0x291001C, 9),
            (0x2910020, 0x415053),
            (0x2910024, 2),
        ]));
        for (c, tag) in [(1, 0x435241), (2, 0x415053)] {
            let ptr = 0x2300000 + c * 0x1000;
            f.0.insert(0x2200000 + c * 4, ptr);
            f.0.insert(ptr + 0x20, c);
            f.0.insert(ptr + 0x1C, tag);
        }
        f
    }
    #[test]
    fn host_and_client_capture_same_canonical_roster_with_different_local_flags() {
        let a = capture(&fixture(1), 0x80000).unwrap();
        let b = capture(&fixture(2), 0x80000).unwrap();
        assert_eq!(a.canonical(), b.canonical());
        assert_ne!(a.local, b.local);
    }
    #[test]
    fn incomplete_ambiguous_or_changed_membership_refuses() {
        for (field, value) in [
            (0x291003C, 0x2900000),
            (0x291001C, 7),
            (0x2910024, 0),
            (0x2600054, 9),
            (0x2800008, 0),
            (0x2000020, 3),
        ] {
            let mut f = fixture(1);
            f.0.insert(field, value);
            assert!(capture(&f, 0x80000).is_none(), "{field:x}");
        }
    }
}
