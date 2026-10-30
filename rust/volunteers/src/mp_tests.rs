//! Two replica fixtures exercise the same saved-variable transport as native MP.
use crate::{
    app::{UiAction, Work},
    engine_store,
    mission::{Campaign, RecipientWar, StackReceipt, Use},
    model::{Identity, State},
    mp_authority::{Admission, Peer, Roster},
    registry::Registry,
    session::{self, SessionMode},
};
use std::collections::BTreeMap;
fn admitted(epoch: u64) -> Admission {
    let peers = vec![
        Peer {
            source: 7,
            country: 625,
            tag: 0x435241,
        },
        Peer {
            source: 9,
            country: 626,
            tag: 0x415053,
        },
    ];
    let roster = Roster {
        local: peers[0],
        peers,
    };
    let mut a = Admission::default();
    let fp = "a".repeat(64);
    assert!(a.bind(epoch, &roster, &fp));
    for p in roster.peers {
        assert!(a.receive(p.source, p.country, 1, &fp));
    }
    a
}
fn mission(
    donor: u32,
    campaign: Campaign,
) -> (crate::app::App, crate::app::WorldView, StackReceipt) {
    let (mut app, mut world) = crate::send_tests::setup(2);
    world.mode = SessionMode::Multiplayer;
    world.player = donor;
    world.campaign = Some(campaign);
    world.mp_authority = admitted(1).issuer(donor);
    world.countries[0].index = donor;
    world.armies[0].owner = donor;
    world.armies[0].affiliation = donor;
    world.armies[0].id.id += donor - 625;
    for r in &mut world.armies[0].regiments {
        r.owner = donor;
        r.id.id += (donor - 625) * 100;
    }
    app.settings.mp_play = true;
    app.bind_registry(Registry::new(campaign, 1));
    app.action(UiAction::Open(648), &world, None);
    crate::send_tests::select(&mut app, &world, 2);
    let work = app.action(UiAction::Send, &world, None);
    let [Work::Detach { serial, plan }] = work.as_slice() else {
        panic!("{work:?}")
    };
    assert_eq!(
        plan.source(),
        plan.destination(),
        "MP never allocates a new army identity"
    );
    let receipt = StackReceipt {
        campaign,
        army: plan.destination(),
        owner: donor,
        affiliation: donor,
        regiments: plan
            .selected_regiments()
            .iter()
            .map(|id| (*id, donor))
            .collect(),
    };
    app.registry
        .as_mut()
        .unwrap()
        .deployment_posted(*serial, true);
    world.date += 24;
    assert!(matches!(
        app.day(&world, |_| Some((receipt.clone(), 2348, donor, false)))
            .as_slice(),
        [Work::Move {
            returning: false,
            ..
        }]
    ));
    (app, world, receipt)
}
#[test]
fn committed_journals_give_two_peers_identical_multi_donor_policy() {
    let (a, _, ra) = mission(625, Campaign([8; 16]));
    let (b, _, rb) = mission(626, Campaign([9; 16]));
    let mut stores = BTreeMap::new();
    for (donor, app) in [(625, a), (626, b)] {
        let tx = engine_store::Transaction::new(app.registry.as_ref().unwrap(), 0).unwrap();
        let mut vars = BTreeMap::new();
        for w in &tx.writes[..tx.writes.len() - 1] {
            vars.insert(w.key.clone(), w.raw_thousandths);
        }
        assert!(
            engine_store::load(1, |k| Ok(vars.get(k).copied()))
                .unwrap()
                .is_none(),
            "uncommitted words cannot activate policy"
        );
        let head = tx.writes.last().unwrap();
        vars.insert(head.key.clone(), head.raw_thousandths);
        stores.insert(donor, vars);
    }
    let policy = |epoch, reverse| {
        let mut groups = stores
            .iter()
            .map(|(c, v)| {
                (
                    *c,
                    engine_store::load(epoch, |k| Ok(v.get(k).copied()))
                        .unwrap()
                        .unwrap()
                        .registry,
                )
            })
            .collect::<Vec<_>>();
        if reverse {
            groups.reverse();
        }
        Registry::committed_policy(
            admitted(epoch).policy().unwrap(),
            groups,
            |campaign, id| {
                [&ra, &rb]
                    .into_iter()
                    .find(|r| r.campaign == campaign && r.army == id)
                    .cloned()
            },
            |_| RecipientWar::AtWar,
        )
        .unwrap()
    };
    let left = policy(1, false);
    let right = policy(88, true);
    assert_eq!(
        left.policy_digest(),
        right.policy_digest(),
        "local load epoch and iteration order never enter shared policy"
    );
    for p in [&left, &right] {
        assert_eq!(
            p.entries().count(),
            2,
            "serial 1 from two donors must not collide"
        );
        assert_eq!(p.context(&ra, Use::Combat, RecipientWar::AtWar), Some(648));
        assert_eq!(p.context(&rb, Use::Combat, RecipientWar::AtWar), Some(648));
        assert!(p.has_native_scope());
        assert!(
            p.encode().is_err(),
            "merged read-only policy cannot become a saved donor registry"
        );
        assert_eq!(
            crate::combat::pair(
                p,
                &ra,
                &rb,
                crate::combat::OtherGates::Passed,
                |_| RecipientWar::AtWar,
                |_, _| Some(true)
            ),
            crate::combat::Decision::Reject,
            "donors serving the same recipient do not fight each other"
        );
    }
}
#[test]
fn only_admitted_local_donor_can_submit_and_derive_orders() {
    let (mut app, mut world) = crate::send_tests::setup(2);
    app.settings.mp_play = true;
    world.mode = SessionMode::Multiplayer;
    crate::send_tests::select(&mut app, &world, 2);
    assert!(app.action(UiAction::Send, &world, None).is_empty());
    world.mp_authority = admitted(1).issuer(626);
    assert!(
        app.action(UiAction::Send, &world, None).is_empty(),
        "foreign donor proof is refused"
    );
    world.mp_authority = admitted(1).issuer(625);
    app.action(UiAction::ToggleRow(1), &world, None);
    assert!(
        app.action(
            UiAction::Send,
            &world,
            Some(Identity { kind: 40, id: 9000 })
        )
        .is_empty(),
        "MP subsets cannot allocate IDs"
    );
    app.action(UiAction::ToggleRow(1), &world, None);
    assert!(app.can_submit(&world));
    app.settings.mp_play = false;
    assert!(
        app.action(UiAction::Send, &world, None).is_empty(),
        "a proof cannot opt another package into MP"
    );
    let (mut app, mut world, receipt) = mission(625, Campaign([8; 16]));
    world.date += 24;
    world.countries[1].war = RecipientWar::AtPeace;
    world.mp_authority = admitted(1).issuer(626);
    assert!(
        app.day(&world, |_| Some((receipt.clone(), 2360, 648, false)))
            .is_empty(),
        "a remote client cannot emit automatic return"
    );
    world.mp_authority = admitted(1).issuer(625);
    let work = app.day(&world, |_| Some((receipt.clone(), 2360, 648, false)));
    assert!(
        matches!(
            work.as_slice(),
            [Work::Move {
                returning: true,
                ..
            }]
        ),
        "{work:?}"
    );
}
#[test]
fn reloading_requires_new_admission_and_never_replays_assembly() {
    let (app, _, receipt) = mission(625, Campaign([8; 16]));
    let bytes = app.registry.as_ref().unwrap().encode().unwrap();
    let mut restored = Registry::decode(&bytes, receipt.campaign, 90).unwrap();
    assert!(!restored.has_native_scope());
    let serial = restored.entries().next().unwrap().deployment.key.serial;
    let mut corrupt = receipt.clone();
    corrupt.affiliation = 626;
    assert!(!restored.revalidate(serial, &corrupt, RecipientWar::AtWar));
    assert!(!restored.has_native_scope());
    assert!(restored.revalidate(serial, &receipt, RecipientWar::AtWar));
    assert_eq!(
        restored.entries().next().unwrap().mission.state(),
        State::Serving
    );
    let old = admitted(1).issuer(625).unwrap();
    assert!(session::admit_issuer(old, app.capabilities, 90, 625).is_err());
    let new = admitted(90).issuer(625).unwrap();
    let permit = session::admit_issuer(new, app.capabilities, 90, 625).unwrap();
    assert!(permit.commands_for(625));
    assert!(!permit.commands_for(626));
}
