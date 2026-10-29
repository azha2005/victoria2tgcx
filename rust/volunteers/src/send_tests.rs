//! Regression scenarios from the ARC -> RGS five-brigade runtime failure.
use crate::app::{App, ArmyView, CountryView, RegimentView, UiAction, Work, WorldView};
use crate::mission::{Campaign, RecipientWar, StackReceipt};
use crate::model::{Identity, Settings, State};
use crate::registry::Registry;
use crate::session::{Capabilities, SessionMode};

fn id(n: u32) -> Identity {
    Identity { kind: 40, id: n }
}
pub(super) fn setup(count: u32) -> (App, WorldView) {
    let campaign = Campaign([9; 16]);
    let world = WorldView {
        epoch: 1,
        campaign: Some(campaign),
        mode: SessionMode::SinglePlayer,
        player: 625,
        date: 100,
        player_at_war: false,
        countries: vec![
            CountryView {
                index: 625,
                tag: 0x435241,
                caption: "ARC".into(),
                war: RecipientWar::AtPeace,
                hostile_to_player: false,
                landing_province: Some(2348),
            },
            CountryView {
                index: 648,
                tag: 0x534752,
                caption: "RGS".into(),
                war: RecipientWar::AtWar,
                hostile_to_player: false,
                landing_province: Some(2360),
            },
        ],
        armies: vec![ArmyView {
            id: id(2183),
            owner: 625,
            affiliation: 625,
            province: 2348,
            name: "Test army".into(),
            combat: false,
            moving: false,
            regiments: (0..count)
                .map(|n| RegimentView {
                    id: id(2178 + n),
                    owner: 625,
                    name: format!("Brigade {n}"),
                })
                .collect(),
        }],
    };
    let mut app = App::new(Settings::default());
    app.capabilities = Capabilities {
        session_detection: true,
        campaign_binding: true,
        command_queue: true,
        combat: true,
        access: true,
        vision: true,
        persistence: true,
    };
    app.bind_registry(Registry::new(campaign, 1));
    app.action(UiAction::Open(648), &world, None);
    (app, world)
}
pub(super) fn select(app: &mut App, world: &WorldView, count: usize) {
    for n in 0..count {
        if n > 0 && n % 2 == 0 {
            app.action(UiAction::NextPage, world, None);
        }
        app.action(UiAction::ToggleRow(n % 2), world, None);
    }
}
pub(super) fn deploy(
    app: &mut App,
    world: &WorldView,
    destination: Option<Identity>,
) -> (u64, StackReceipt) {
    let work = app.action(UiAction::Send, world, destination);
    let [Work::Detach { serial, plan }] = work.as_slice() else {
        panic!("missing deployment: {work:?}")
    };
    let receipt = StackReceipt {
        campaign: world.campaign.unwrap(),
        army: plan.destination(),
        owner: 625,
        affiliation: 625,
        regiments: plan
            .selected_regiments()
            .iter()
            .map(|r| (*r, 625))
            .collect(),
    };
    app.registry
        .as_mut()
        .unwrap()
        .deployment_posted(*serial, true);
    (*serial, receipt)
}
#[test]
fn whole_five_and_single_brigade_armies_reuse_source_then_move() {
    for count in [1, 5] {
        let (mut app, mut world) = setup(count);
        select(&mut app, &world, count as usize);
        assert!(app.whole_army_selection(&world));
        let (serial, receipt) = deploy(&mut app, &world, None);
        assert_eq!(receipt.army, world.armies[0].id);
        world.date += 24;
        let work = app.day(&world, |_| Some((receipt.clone(), 2348, 625, false)));
        assert!(
            matches!(work.as_slice(), [Work::Move { army, destination: 2360, returning: false, .. }] if *army == receipt.army)
        );
        assert_eq!(
            app.registry
                .as_ref()
                .unwrap()
                .entry(serial)
                .unwrap()
                .mission
                .state(),
            State::Serving
        );
    }
}
#[test]
fn partial_selection_still_detaches_to_new_army() {
    let (mut app, world) = setup(5);
    select(&mut app, &world, 4);
    assert!(!app.whole_army_selection(&world));
    let (_, receipt) = deploy(&mut app, &world, Some(id(9000)));
    assert_eq!(receipt.army, id(9000));
    assert_eq!(receipt.regiments.len(), 4);
}
#[test]
fn unchanged_day_does_not_rewrite_journal_but_peace_does() {
    let (mut app, world) = setup(5);
    select(&mut app, &world, 5);
    let (serial, receipt) = deploy(&mut app, &world, None);
    assert!(app.registry.as_mut().unwrap().receipt(serial, &receipt));
    let r = app.registry.as_mut().unwrap();
    let before = r.encode().unwrap();
    for day in 1..=300 {
        r.observe_day(100 + day * 24, |_| RecipientWar::AtWar);
    }
    assert_eq!(r.encode().unwrap(), before);
    r.observe_day(7400, |_| RecipientWar::AtPeace);
    assert_ne!(r.encode().unwrap(), before);
}
#[test]
fn restored_complete_assembly_continues_without_replaying_split() {
    let (mut app, world) = setup(5);
    select(&mut app, &world, 5);
    let (serial, receipt) = deploy(&mut app, &world, None);
    let bytes = app.registry.as_ref().unwrap().encode().unwrap();
    let mut restored = Registry::decode(&bytes, receipt.campaign, 2).unwrap();
    assert!(!restored.has_native_scope());
    let mut incomplete = receipt.clone();
    incomplete.regiments.pop();
    assert!(!restored.revalidate(serial, &incomplete, RecipientWar::AtWar));
    assert!(restored.revalidate(serial, &receipt, RecipientWar::AtWar));
    assert!(restored.entry(serial).unwrap().assembly_verified);
    assert!(restored.receipt(serial, &receipt));
    assert!(restored.has_native_scope());
}
#[test]
fn incomplete_transfer_never_grants_recipient_scope() {
    let (mut app, mut world) = setup(5);
    select(&mut app, &world, 5);
    let (_, mut receipt) = deploy(&mut app, &world, None);
    receipt.regiments.pop();
    world.date += 24;
    assert!(
        app.day(&world, |_| Some((receipt.clone(), 2348, 625, false)))
            .is_empty()
    );
    assert!(!app.registry.as_ref().unwrap().has_native_scope());
}

#[test]
fn closed_panel_snapshot_preserves_daily_orders_and_journal() {
    for event in ["serving", "recipient_peace", "donor_war"] {
        let (mut full, mut world) = setup(5);
        let (mut compact, _) = setup(5);
        select(&mut full, &world, 5);
        select(&mut compact, &world, 5);
        let (serial, receipt) = deploy(&mut full, &world, None);
        deploy(&mut compact, &world, None);
        if event == "recipient_peace" {
            world.countries[1].war = RecipientWar::AtPeace;
        }
        if event == "donor_war" {
            world.player_at_war = true;
        }
        world.date += 24;
        let mut narrowed = world.clone();
        narrowed.armies.clear();
        let expected = full.day(&world, |_| Some((receipt.clone(), 2348, 625, false)));
        let actual = compact.day(&narrowed, |_| Some((receipt.clone(), 2348, 625, false)));
        assert_eq!(actual, expected, "{event}");
        let a = full.registry.as_ref().unwrap();
        let b = compact.registry.as_ref().unwrap();
        assert_eq!(a.encode().unwrap(), b.encode().unwrap(), "{event}");
        assert_eq!(
            a.entry(serial).unwrap().mission.state(),
            b.entry(serial).unwrap().mission.state()
        );
    }
}

#[test]
fn returned_whole_army_can_be_sent_again_without_old_scope_shadowing_it() {
    let (mut app, world) = setup(1);
    select(&mut app, &world, 1);
    let (first, receipt) = deploy(&mut app, &world, None);
    let registry = app.registry.as_mut().unwrap();
    assert!(registry.receipt(first, &receipt));
    assert!(registry.request_return(first, 2348));
    registry.return_posted(first, true, true);
    assert!(registry.return_receipt(first, &receipt, 625));
    assert!(registry.live_army(receipt.army).is_none());
    select(&mut app, &world, 1);
    let (second, receipt) = deploy(&mut app, &world, None);
    let registry = app.registry.as_mut().unwrap();
    assert!(registry.receipt(second, &receipt));
    assert_eq!(
        registry
            .live_army(receipt.army)
            .unwrap()
            .deployment
            .key
            .serial,
        second
    );
    assert_eq!(
        registry.context(&receipt, crate::mission::Use::Combat, RecipientWar::AtWar),
        Some(648)
    );
    let restored = Registry::decode(&registry.encode().unwrap(), receipt.campaign, 2).unwrap();
    assert_eq!(restored.entries().count(), 2);
}

#[test]
fn history_cleanup_releases_slots_retains_wartime_sight_and_never_reuses_serials() {
    let (mut app, mut world) = setup(5);
    select(&mut app, &world, 5);
    let (first, receipt) = deploy(&mut app, &world, None);
    let r = app.registry.as_mut().unwrap();
    assert!(r.receipt(first, &receipt));
    assert!(r.request_return(first, 2348));
    r.return_posted(first, true, true);
    assert!(r.return_receipt(first, &receipt, 625));
    assert_eq!(r.deployed(625), 0);
    assert_eq!(r.clear_completed(), 0); // War still grants vision.
    assert!(r.sees(625, 648, false));
    assert!(app.presentation(&world).missions.is_empty());
    app.action(UiAction::ToggleHistory, &world, None);
    assert_eq!(app.presentation(&world).missions.len(), 1);
    app.action(UiAction::ClearHistory, &world, None);
    assert_eq!(app.registry.as_ref().unwrap().entries().count(), 1);
    let r = app.registry.as_mut().unwrap();
    r.observe_day(200, |_| RecipientWar::AtPeace);
    assert_eq!(r.clear_completed(), 1);
    let next = r.next_serial().unwrap();
    assert!(next > first);
    let loaded = Registry::decode(&r.encode().unwrap(), receipt.campaign, 2).unwrap();
    assert_eq!(loaded.next_serial(), Some(next));
    assert_eq!(loaded.deployed(625), 0);
    app.bind_registry(loaded);
    world.epoch = 2;
    world.date = 50;
    app.action(UiAction::Open(648), &world, None);
    select(&mut app, &world, 5);
    let (second, _) = deploy(&mut app, &world, None);
    assert!(second > first);
}

#[test]
fn restored_terminal_records_do_not_count_as_active_brigades_and_old_day_is_reset() {
    let (mut app, mut world) = setup(1);
    select(&mut app, &world, 1);
    let (serial, receipt) = deploy(&mut app, &world, None);
    world.date = 1000;
    app.day(&world, |_| Some((receipt.clone(), 2348, 625, false)));
    app.registry.as_mut().unwrap().destroyed(serial);
    let loaded = Registry::decode(
        &app.registry.as_ref().unwrap().encode().unwrap(),
        receipt.campaign,
        2,
    )
    .unwrap();
    assert_eq!(loaded.deployed(625), 0); // NeedsReview carries saved Lost phase.
    app.bind_registry(loaded);
    world.epoch = 2;
    world.date = 100;
    app.day(&world, |_| panic!("terminal stack cannot issue a command"));
    assert_eq!(
        app.registry
            .as_ref()
            .unwrap()
            .entry(serial)
            .unwrap()
            .last_processed_date,
        Some(100)
    );
}

#[test]
fn multiplayer_and_unknown_sessions_report_distinct_reasons_without_outbox_work() {
    let (mut app, mut world) = setup(1);
    let before = app.registry.as_ref().unwrap().encode().unwrap();
    for (mode, expected) in [
        (SessionMode::Multiplayer, "Multiplayer"),
        (SessionMode::Unknown, "recognized game session"),
    ] {
        world.mode = mode;
        select(&mut app, &world, 1);
        assert!(app.action(UiAction::Send, &world, None).is_empty());
        assert!(app.presentation(&world).status.contains(expected));
        assert_eq!(app.registry.as_ref().unwrap().encode().unwrap(), before);
    }
}

#[test]
fn many_terminal_wartime_missions_refuse_full_journal_without_half_accepting() {
    let (mut app, world) = setup(5);
    let mut previous_bytes = Vec::new();
    for _ in 0..256 {
        app.action(UiAction::Open(648), &world, None);
        select(&mut app, &world, 5);
        let before = app.registry.as_ref().unwrap().encode().unwrap();
        let work = app.action(UiAction::Send, &world, None);
        if work.is_empty() {
            assert!(app.presentation(&world).status.contains("journal is full"));
            assert_eq!(app.registry.as_ref().unwrap().encode().unwrap(), before);
            assert!(!previous_bytes.is_empty());
            return;
        }
        let [Work::Detach { serial, plan }] = work.as_slice() else {
            panic!("unexpected work")
        };
        let receipt = StackReceipt {
            campaign: world.campaign.unwrap(),
            army: plan.destination(),
            owner: 625,
            affiliation: 625,
            regiments: plan
                .selected_regiments()
                .iter()
                .map(|id| (*id, 625))
                .collect(),
        };
        let r = app.registry.as_mut().unwrap();
        r.deployment_posted(*serial, true);
        assert!(r.receipt(*serial, &receipt));
        assert!(r.request_return(*serial, 2348));
        r.return_posted(*serial, true, true);
        assert!(r.return_receipt(*serial, &receipt, 625));
        previous_bytes = r.encode().unwrap();
    }
    panic!("fixture never reached bounded journal size");
}

#[test]
fn visibility_uses_recipient_knowledge_and_bulk_world_table() {
    use crate::snapshot::Memory;
    use std::cell::Cell;
    use std::collections::BTreeMap;
    struct Reads {
        data: BTreeMap<u32, u32>,
        calls: Cell<usize>,
    }
    impl Memory for Reads {
        fn read32(&self, address: u32) -> Option<u32> {
            self.calls.set(self.calls.get() + 1);
            self.data.get(&address).copied()
        }
        fn read_words(&self, address: u32, count: usize) -> Option<Vec<u32>> {
            self.calls.set(self.calls.get() + 1);
            if address != 0x2800000 || count != 40000 {
                return None;
            }
            Some(
                (0..count)
                    .map(|i| {
                        self.data
                            .get(&(address + i as u32 * 4))
                            .copied()
                            .unwrap_or(0)
                    })
                    .collect(),
            )
        }
    }
    let (mut app, world) = setup(1);
    select(&mut app, &world, 1);
    let (serial, receipt) = deploy(&mut app, &world, None);
    let registry = app.registry.as_mut().unwrap();
    assert!(registry.receipt(serial, &receipt));
    let base = 0x80000;
    let mut memory = Reads {
        calls: Cell::new(0),
        data: BTreeMap::from([
            (base + 0xE588E8, 0x2000000),
            (base + 0xE587E4, 0x2100000),
            (0x2100000, 700),
            (0x2100004, 0x2200000),
            (0x2200000 + 625 * 4, 0x2300000),
            (0x2200000 + 648 * 4, 0x2310000),
            (0x2000000 + 0xB60, 625),
            (0x2000000 + 0xB24, 0x2400000),
            (0x2400000, base + 0xA0A34C),
            (0x2400000 + 0x17E8, 0x2500000),
            (0x2500000 + 0x12C, 0x2600000),
            (0x2600000, base + 0xA390D0),
            (0x2600000 + 0x114, base + 0xA0A168),
            (0x2600000 + 0x118, 0x18D),
            (0x2600000 + 0x2C, 4),
            (0x2600000 + 0x180, 2),
            (0x2600000 + 0x38, 0x2700000),
            (0x2700000 + 0x1C, 0x2600000),
            (0x2700000, base + 0xA42840),
            (0x2700000 + 0x18, 0),
            // Bulk pointer table has40k slots; only this province is populated.
            (0x2000000 + 0xACC, 0x2800000),
            (0x2000000 + 0xAD0, 0x2800000 + 40000 * 4),
            (0x2310000 + 0x67C, 1),
            (0x2310000 + 0x9D8, 0x2900000),
            (0x2310000 + 0x9DC, 0x2900004),
            (0x2900000, 2360),
            (0x2800000 + 2360 * 4, 0x3000000),
            // This foreign province is seen by the recipient, not owned by it.
            (0x3000000 + 0x12C, 123),
            (0x3000000 + 0x134, 123),
            (0x3000000 + 0x130, 0),
            (0x3000000 + 0x58, 2360),
            (0x3000000 + 0x5C, 0x3100000),
            (0x3000000 + 0x174, 0x3200000),
            (0x3000000 + 0x178, 700),
            (0x3200000 + 648 * 4, 2),
            (0x3100000, base + 0x9FD310),
            (0x3100000 + 0x360, 0),
        ]),
    };
    let permit = crate::session::admit(world.mode, app.capabilities, world.epoch).unwrap();
    let plan = crate::native_vision::plan(&memory, base, registry, permit).unwrap();
    assert_eq!(plan.grants.len(), 1);
    assert_eq!(plan.grants[0].province, 2360);
    assert!(memory.calls.get() < 100, "{} reads", memory.calls.get());
    // Native visibility recompute tolerates a non-null dummy with zero levels.
    memory.data.insert(0x2800000, 0x3020000);
    memory.data.insert(0x3020000 + 0x174, 0);
    memory.data.insert(0x3020000 + 0x178, 0);
    assert_eq!(
        crate::native_vision::plan(&memory, base, registry, permit)
            .unwrap()
            .grants
            .len(),
        1
    );
    memory.data.insert(0x3100000, base + 0x9FD338);
    assert_eq!(
        crate::native_vision::plan(&memory, base, registry, permit)
            .unwrap()
            .grants
            .len(),
        1
    );
    // Recipient sees only an edge here. The actual volunteer's neighboring
    // province must still be selectable/visible for the donor locally.
    memory.data.insert(0x3200000 + 648 * 4, 1);
    memory.data.insert(0x3100000 + 0xC8, 0x3300000);
    memory.data.insert(0x3100000 + 0xCC, 0x3300024);
    memory.data.insert(0x3300004, 2361);
    for (address, value) in [
        (0x2800000 + 2361 * 4, 0x3010000),
        (0x3010000 + 0x174, 0x3210000),
        (0x3010000 + 0x12C, 123),
        (0x3010000 + 0x134, 123),
        (0x3010000 + 0x130, 0),
        (0x3010000 + 0x178, 700),
        (0x3210000 + 648 * 4, 0),
        (0x3010000 + 0x58, 2361),
        (0x3010000 + 0x5C, 0x3110000),
        (0x3110000, base + 0x9FD310),
        (0x3110000 + 0x360, 0),
    ] {
        memory.data.insert(address, value);
    }
    let without_fronts = crate::native_vision::plan(&memory, base, registry, permit).unwrap();
    assert_eq!(without_fronts.grants.len(), 1);
    assert_eq!(without_fronts.grants[0].requested_level, 1);
    let with_fronts =
        crate::native_vision::plan_with_fronts(&memory, base, registry, permit, &[0x3000000])
            .unwrap();
    assert_eq!(with_fronts.grants.len(), 2);
    assert!(with_fronts.grants.iter().all(|g| g.requested_level == 2));
    assert_eq!(memory.data[&(0x3210000 + 648 * 4)], 0); // simulation knowledge untouched
    // One short/bad/dummy knowledge array must not erase other provinces.
    memory.data.insert(0x3020000 + 0x178, 8000);
    let partial =
        crate::native_vision::plan_with_fronts(&memory, base, registry, permit, &[0x3000000])
            .unwrap();
    assert_eq!(partial.grants.len(), 2);
    assert_eq!(partial.skipped_provinces, 1);
    // Direct native controller sight is available even if this recipient has
    // no entry in a short knowledge array. Occupied enemy land isn't guessed.
    memory.data.insert(0x3010000 + 0x134, 648);
    memory.data.insert(0x3010000 + 0x178, 1);
    let controlled = crate::native_vision::plan(&memory, base, registry, permit).unwrap();
    assert!(
        controlled
            .grants
            .iter()
            .any(|g| g.province == 2361 && g.requested_level == 2)
    );
    memory.data.insert(0x2310000 + 0x67C, 0);
    assert!(
        crate::native_vision::plan_with_fronts(&memory, base, registry, permit, &[0x3000000])
            .unwrap()
            .grants
            .is_empty()
    );
}
