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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub stage: &'static str,
    pub values: Vec<u32>,
}
fn refuse(stage: &'static str, values: &[u32]) -> Refusal {
    Refusal {
        stage,
        values: values.to_vec(),
    }
}
fn read(m: &impl Memory, p: u32, o: u32, stage: &'static str) -> Result<u32, Refusal> {
    field(m, p, o).ok_or_else(|| refuse(stage, &[p, o]))
}
pub fn capture(m: &impl Memory, base: u32) -> Option<Roster> {
    capture_checked(m, base).ok()
}
pub fn capture_checked(m: &impl Memory, base: u32) -> Result<Roster, Refusal> {
    let session = crate::native_session::probe_queue(m, base)
        .ok_or_else(|| refuse("native_queue", &[base]))?;
    let world = session.world;
    let local_country = read(m, world, 0xB60, "local_country")?;
    let raw_source = read(m, session.manager, 0x54, "local_source")?;
    let local_source = u16::try_from(raw_source)
        .ok()
        .filter(|s| *s <= 0x7FFF)
        .ok_or_else(|| refuse("local_source_range", &[raw_source, local_country]))?;
    let countries = read(m, base, 0xE587E4, "country_manager")?;
    let count = read(m, countries, 0, "country_count")?;
    if count == 0 || count > 4096 {
        return Err(refuse("country_count", &[count]));
    }
    let country_table = read(m, countries, 4, "country_table")?;
    let start = read(m, world, 0xAEC, "controller_vector")?;
    let end = read(m, world, 0xAF0, "controller_vector")?;
    let world_start = read(m, world, 0xADC, "world_country_vector")?;
    let world_end = read(m, world, 0xAE0, "world_country_vector")?;
    let len = end
        .checked_sub(start)
        .filter(|n| *n > 0 && *n <= 4096 * 4 && n % 4 == 0)
        .ok_or_else(|| refuse("controller_bounds", &[start, end, count]))?;
    // FUN_00680450 constructs these parallel WORLD vectors. The global manager
    // is a separate index bound; its size is not their allocation/length proof.
    if start == 0 || world_start == 0 || world_end.checked_sub(world_start) != Some(len) {
        return Err(refuse(
            "parallel_world_vectors",
            &[start, end, world_start, world_end, count],
        ));
    }
    let slots = len / 4;
    let head = read(m, world, 0x18, "human_list")?;
    let tail = read(m, world, 0x1C, "human_list")?;
    let size = read(m, world, 0x20, "human_list")?;
    if !(2..=32).contains(&size) {
        return Err(refuse(
            "human_list_size",
            &[size, head, tail, local_country, raw_source],
        ));
    }
    let mut node = head;
    let mut previous = 0;
    let mut seen = BTreeSet::new();
    let mut peers = Vec::new();
    while node != 0 {
        if peers.len() >= size as usize || !seen.insert(node) {
            return Err(refuse("human_list_cycle", &[node, size]));
        }
        let back = read(m, node, 0x38, "human_backlink")?;
        if back != previous {
            return Err(refuse("human_backlink", &[node, back, previous]));
        }
        let raw = read(m, node, 0x1C, "human_source")?;
        let source = u16::try_from(raw)
            .ok()
            .filter(|s| *s <= 0x7FFF)
            .ok_or_else(|| refuse("human_source_range", &[node, raw]))?;
        let country = read(m, node, 0x24, "human_country")?;
        let tag = read(m, node, 0x20, "human_tag")?;
        if country == 0 || country >= count || country >= slots {
            return Err(refuse(
                "human_country_bounds",
                &[node, country, count, slots],
            ));
        }
        let flag = read(m, start, country * 4, "human_controller")?;
        if flag == 0 {
            return Err(refuse(
                "human_controller_missing",
                &[node, country, source.into()],
            ));
        }
        let ptr = read(m, country_table, country * 4, "country_pointer")?;
        let world_ptr = read(m, world_start, country * 4, "world_country_pointer")?;
        let index = read(m, ptr, 0x20, "country_index")?;
        let actual_tag = read(m, ptr, 0x1C, "country_tag")?;
        if ptr == 0 || ptr != world_ptr || index != country || actual_tag != tag {
            return Err(refuse(
                "country_identity",
                &[country, ptr, world_ptr, index, tag, actual_tag],
            ));
        }
        peers.push(Peer {
            source,
            country,
            tag,
        });
        previous = node;
        node = read(m, node, 0x3C, "human_next")?;
    }
    if previous != tail || peers.len() != size as usize {
        return Err(refuse(
            "human_list_end",
            &[previous, tail, peers.len() as u32, size],
        ));
    }
    let mut human = BTreeSet::new();
    for country in 0..slots {
        if read(m, start, country * 4, "controller_read")? != 0 {
            human.insert(country);
        }
    }
    let listed = peers.iter().map(|p| p.country).collect();
    if human != listed {
        return Err(refuse(
            "human_controller_set",
            &human.into_iter().collect::<Vec<_>>(),
        ));
    }
    let local = *peers
        .iter()
        .find(|p| p.country == local_country && p.source == local_source)
        .ok_or_else(|| {
            let mut values = vec![local_country, raw_source];
            for p in &peers {
                values.extend([p.country, p.source.into()]);
            }
            refuse("local_source_country", &values)
        })?;
    let local_flag = read(m, start, local_country * 4, "local_controller")?;
    if local_flag != 1 {
        return Err(refuse(
            "local_controller",
            &[local_country, local_flag, raw_source],
        ));
    }
    let roster = Roster { peers, local };
    roster
        .canonical()
        .ok_or_else(|| refuse("duplicate_or_invalid_peer", &[size]))?;
    if read(m, base, 0xE588E8, "stable_world")? != world
        || read(m, base, 0xE587E4, "stable_country_manager")? != countries
        || read(m, countries, 0, "stable_country_count")? != count
        || read(m, countries, 4, "stable_country_table")? != country_table
        || read(m, world, 0x18, "stable_head")? != head
        || read(m, world, 0x1C, "stable_tail")? != tail
        || read(m, world, 0x20, "stable_size")? != size
        || read(m, world, 0xAEC, "stable_controllers")? != start
        || read(m, world, 0xAF0, "stable_controllers")? != end
        || read(m, world, 0xADC, "stable_countries")? != world_start
        || read(m, world, 0xAE0, "stable_countries")? != world_end
        || read(m, world, 0xB60, "stable_local_country")? != local_country
        || read(m, session.manager, 0x54, "stable_local_source")? != raw_source
    {
        return Err(refuse(
            "changed_during_capture",
            &[world, local_country, raw_source],
        ));
    }
    Ok(roster)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    pub(super) struct Fixture(pub(super) BTreeMap<u32, u32>);
    impl Memory for Fixture {
        fn read32(&self, p: u32) -> Option<u32> {
            self.0.get(&p).copied()
        }
    }
    pub(super) fn fixture(local: u32) -> Fixture {
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
            (w + 0xADC, 0x2200000),
            (w + 0xAE0, 0x220000C),
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

#[cfg(test)]
mod diagnostic_tests {
    use super::tests::fixture;
    use super::*;
    #[test]
    fn global_capacity_is_independent_but_world_vectors_and_identities_must_agree() {
        let mut f = fixture(1);
        f.0.insert(0x2100000, 4);
        assert!(capture_checked(&f, 0x80000).is_ok());
        f.0.insert(0x2000000 + 0xAE0, 0x2200010);
        assert_eq!(
            capture_checked(&f, 0x80000).unwrap_err().stage,
            "parallel_world_vectors"
        );
        let mut f = fixture(1);
        f.0.insert(0x2000000 + 0xADC, 0x2210000);
        f.0.insert(0x2000000 + 0xAE0, 0x221000C);
        f.0.insert(0x2210004, 0xABCD0000);
        assert_eq!(
            capture_checked(&f, 0x80000).unwrap_err().stage,
            "country_identity"
        );
    }
    #[test]
    fn explicit_stages_distinguish_invalid_source_local_flag_and_missing_human() {
        for (p, v, stage) in [
            (0x2600054, 0xFFFF, "local_source_range"),
            (0x2600054, 9, "local_source_country"),
            (0x2800004, 2, "local_controller"),
            (0x2800008, 0, "human_controller_missing"),
        ] {
            let mut f = fixture(1);
            f.0.insert(p, v);
            assert_eq!(capture_checked(&f, 0x80000).unwrap_err().stage, stage);
        }
    }
}
