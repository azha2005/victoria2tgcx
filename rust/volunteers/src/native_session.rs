//! Read-only session identity. Evidence: configuration copy FUN_00648AF0,
//! manager constructor FUN_009AE5F0 and local transport FUN_00A682C0.
//! A host, a client, an unknown transport and a replay cannot become local SP
//! merely because their player count is one. This is not a campaign/save ID.
use crate::session::SessionMode;
use crate::snapshot::Memory;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Session {
    pub world: u32,
    pub context: u32,
    pub manager: u32,
    pub queue: u32,
    pub mode: SessionMode,
}
fn field(m: &impl Memory, object: u32, offset: u32) -> Option<u32> {
    m.read32(object.checked_add(offset)?)
}
/// Network-only posting gate. Slot 6 is ECX=queue, one stack command, RET 4.
/// VAs A62EA0 (server.cpp) and A66B30 (client) were inspected independently.
pub fn probe_queue(m: &impl Memory, base: u32) -> Option<Session> {
    let session = capture(m, base)?;
    if session.mode != SessionMode::Multiplayer {
        return None;
    }
    let expected = match m.read32(session.queue)?.checked_sub(base)? {
        0xA42728 => 0x662EA0,
        0xA427C8 => 0x666B30,
        _ => return None,
    };
    (field(m, m.read32(session.queue)?, 6 * 4)? == base.checked_add(expected)?).then_some(session)
}
pub fn capture(m: &impl Memory, base: u32) -> Option<Session> {
    let world_global = base.checked_add(0xE588E8)?;
    let world = m.read32(world_global)?;
    let player = field(m, world, 0xB60)?;
    let countries = m.read32(base.checked_add(0xE587E4)?)?;
    let count = m.read32(countries)?;
    if player == 0
        || count > 4096
        || player >= count
        || field(m, field(m, countries, 4)?, player.checked_mul(4)?)? == 0
    {
        return None;
    }
    let context = field(m, world, 0xB24)?;
    if m.read32(context)? != base.checked_add(0xA0A34C)? {
        return None;
    }
    // In-game slot17 (FUN_00651D10) is exactly this two-load chain.
    let holder = field(m, context, 0x17E8)?;
    let manager = field(m, holder, 0x12C)?;
    if m.read32(manager)? != base.checked_add(0xA390D0)?
        || field(m, manager, 0x114)? != base.checked_add(0xA0A168)?
        || field(m, manager, 0x118)? != 0x18D
        || field(m, manager, 0x2C)? != 4
    {
        return None;
    }
    let kind = field(m, manager, 0x180)?;
    let queue = field(m, manager, 0x38)?;
    if field(m, queue, 0x1C)? != manager {
        return None;
    }
    let table = m.read32(queue)?;
    let mode = if kind == 2 && table == base.checked_add(0xA42840)? && field(m, queue, 0x18)? == 0 {
        SessionMode::SinglePlayer
    } else if (kind == 0 || kind == 1)
        && [base.checked_add(0xA42728)?, base.checked_add(0xA427C8)?].contains(&table)
    {
        SessionMode::Multiplayer
    } else {
        SessionMode::Unknown
    };
    // Reject a torn observation while the engine replaces its current session.
    if m.read32(world_global)? != world
        || field(m, world, 0xB60)? != player
        || m.read32(base.checked_add(0xE587E4)?)? != countries
        || field(m, world, 0xB24)? != context
        || field(m, context, 0x17E8)? != holder
        || field(m, holder, 0x12C)? != manager
        || field(m, manager, 0x38)? != queue
        || field(m, manager, 0x180)? != kind
        || m.read32(queue)? != table
    {
        return None;
    }
    Some(Session {
        world,
        context,
        manager,
        queue,
        mode,
    })
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
    fn fixture(kind: u32, transport: u32) -> Fixture {
        let base = 0x80000;
        Fixture(BTreeMap::from([
            (base + 0xE588E8, 0x2000000),
            (base + 0xE587E4, 0x2100000),
            (0x2100000, 4),
            (0x2100004, 0x2200000),
            (0x2200004, 0x2300000),
            (0x2000000 + 0xB60, 1),
            (0x2000000 + 0xB24, 0x2400000),
            (0x2400000, base + 0xA0A34C),
            (0x2400000 + 0x17E8, 0x2500000),
            (0x2500000 + 0x12C, 0x2600000),
            (0x2600000, base + 0xA390D0),
            (0x2600000 + 0x114, base + 0xA0A168),
            (0x2600000 + 0x118, 0x18D),
            (0x2600000 + 0x2C, 4),
            (0x2600000 + 0x180, kind),
            (0x2600000 + 0x38, 0x2700000),
            (0x2700000 + 0x1C, 0x2600000),
            (0x2700000, base + transport),
            (0x2700000 + 0x18, 0),
        ]))
    }
    #[test]
    fn local_queue_is_not_inferred_from_a_single_host() {
        assert_eq!(
            capture(&fixture(2, 0xA42840), 0x80000).unwrap().mode,
            SessionMode::SinglePlayer
        );
        assert_eq!(
            capture(&fixture(1, 0xA42728), 0x80000).unwrap().mode,
            SessionMode::Multiplayer
        );
        assert_eq!(
            capture(&fixture(0, 0xA427C8), 0x80000).unwrap().mode,
            SessionMode::Multiplayer
        );
        assert_eq!(
            capture(&fixture(2, 0xA42728), 0x80000).unwrap().mode,
            SessionMode::Unknown
        );
        assert_eq!(
            capture(&fixture(3, 0xA42840), 0x80000).unwrap().mode,
            SessionMode::Unknown
        );
    }
    #[test]
    fn menus_unreadable_fields_and_wrong_backlinks_are_refused() {
        let mut data = fixture(2, 0xA42840);
        data.0.insert(0x2700000 + 0x1C, 0);
        assert!(capture(&data, 0x80000).is_none());
        let mut data = fixture(2, 0xA42840);
        data.0.insert(0x2000000 + 0xB60, 0);
        assert!(capture(&data, 0x80000).is_none());
        let mut data = fixture(2, 0xA42840);
        data.0.remove(&(0x2600000 + 0x180));
        assert!(capture(&data, 0x80000).is_none());
    }
    #[test]
    fn mp_probe_uses_only_the_exact_host_or_client_post_slot() {
        for (kind, table, post) in [(1, 0xA42728, 0x662EA0), (0, 0xA427C8, 0x666B30)] {
            let mut data = fixture(kind, table);
            data.0.insert(0x80000 + table + 24, 0x80000 + post);
            assert!(probe_queue(&data, 0x80000).is_some());
            data.0.insert(0x80000 + table + 24, 0x80000 + 0x668560);
            assert!(probe_queue(&data, 0x80000).is_none());
        }
        assert!(probe_queue(&fixture(2, 0xA42840), 0x80000).is_none());
    }
}
