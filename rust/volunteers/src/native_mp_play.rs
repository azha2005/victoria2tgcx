//! Network gameplay coordination. Only native execution/barriers publish policy;
//! each admitted country's local client alone emits its native order/checkpoint.
use crate::{
    mp_authority::{self, Admission, IssuerAuthority, Roster},
    native::Native,
};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
static ENABLED: AtomicBool = AtomicBool::new(false);
static STATE: OnceLock<Mutex<State>> = OnceLock::new();
struct State {
    epoch: u64,
    world: u32,
    context: u32,
    roster: Option<Roster>,
    fp: String,
    admission: Admission,
    journals: BTreeMap<u32, crate::engine_store::Loaded>,
    last_hello: Option<u32>,
    last_digest: Option<String>,
}
pub fn initialize(enabled: bool, plugin: &str, max: usize) {
    let Some(fp) = mp_authority::fingerprint(plugin, max) else {
        return;
    };
    if STATE
        .set(Mutex::new(State {
            epoch: 0,
            world: 0,
            context: 0,
            roster: None,
            fp,
            admission: Admission::default(),
            journals: BTreeMap::new(),
            last_hello: None,
            last_digest: None,
        }))
        .is_ok()
    {
        ENABLED.store(enabled, Ordering::Release);
    }
}
pub fn invalidate() {
    if let Some(lock) = STATE.get()
        && let Ok(mut s) = lock.lock()
    {
        s.epoch = 0;
        s.world = 0;
        s.context = 0;
        s.admission = Admission::default();
        s.roster = None;
        s.journals.clear();
        s.last_hello = None;
        s.last_digest = None;
    }
}
fn session(n: Native) -> Option<(crate::native_session::Session, u64)> {
    if !ENABLED.load(Ordering::Acquire) {
        return None;
    }
    let session = crate::native_session::probe_queue(&n, n.base)?;
    let epoch = crate::native_lifecycle::epoch(session.context)?;
    Some((session, epoch))
}
fn bind(s: &mut State, session: crate::native_session::Session, epoch: u64, roster: Roster) {
    if s.epoch != epoch || s.world != session.world || s.context != session.context {
        s.epoch = epoch;
        s.world = session.world;
        s.context = session.context;
        s.admission = Admission::default();
        s.journals.clear();
        s.last_hello = None;
        s.last_digest = None;
    }
    let changed = s.roster.as_ref().and_then(Roster::canonical) != roster.canonical();
    s.admission.bind(epoch, &roster, &s.fp);
    if changed {
        s.last_hello = None;
        s.last_digest = None;
        s.journals
            .retain(|country, _| roster.peers.iter().any(|p| p.country == *country));
    }
    s.roster = Some(roster);
}
/// Cheap UI view, never sufficient to post a command. Posting checks a fresh
/// native roster as well as this sealed current-load execution admission.
pub fn issuer(n: Native) -> Option<IssuerAuthority> {
    let (session, epoch) = session(n)?;
    let country = n.field(session.world, 0xB60)?;
    let source = n.field(session.manager, 0x54)?;
    let s = STATE.get()?.lock().ok()?;
    let local = s.roster.as_ref()?.local;
    if s.epoch != epoch
        || s.world != session.world
        || s.context != session.context
        || local.country != country
        || u32::from(local.source) != source
    {
        return None;
    }
    s.admission.issuer(country)
}
pub fn fresh_issuer(n: Native) -> Option<IssuerAuthority> {
    let actual = crate::native_roster::capture(&n, n.base)?;
    let proof = issuer(n)?;
    let s = STATE.get()?.lock().ok()?;
    (s.roster.as_ref()?.canonical() == actual.canonical()).then_some(proof)
}
pub fn validate_permit(n: Native, permit: crate::session::SimulationPermit) -> bool {
    fresh_issuer(n).is_some_and(|proof| permit.authorized_by(proof))
}
fn rebuild(n: Native, s: &mut State) {
    let Some(proof) = s.admission.policy() else {
        crate::native_hooks::publish_network(None, None);
        return;
    };
    let Some(roster) = s.roster.as_ref() else {
        return;
    };
    // Initial load/late-join reads the engine save, not a local sidecar. There
    // is no replay. Thereafter HEAD execution updates only that donor's cache.
    for peer in &roster.peers {
        if s.journals.contains_key(&peer.country) {
            continue;
        }
        match crate::engine_store::load(s.epoch, |k| n.variable(peer.country, k)) {
            Ok(Some(loaded)) => {
                s.journals.insert(peer.country, loaded);
            }
            Ok(None) => (),
            Err(error) => {
                s.admission.fail();
                crate::native_hooks::publish_network(None, None);
                crate::runtime::diagnostic(&format!(
                    "mp_gameplay_store_refused country={} error={error:?}",
                    peer.country
                ));
                return;
            }
        }
    }
    let groups = roster
        .peers
        .iter()
        .filter_map(|p| {
            s.journals
                .get(&p.country)
                .map(|loaded| (p.country, loaded.registry.clone()))
        })
        .collect();
    let policy = crate::registry::Registry::committed_policy(
        proof,
        groups,
        |campaign, id| n.receipt(campaign, id),
        |country| {
            n.country(country).and_then(|p| n.byte(p + 0x67C)).map_or(
                crate::mission::RecipientWar::Unknown,
                |b| {
                    if b == 0 {
                        crate::mission::RecipientWar::AtPeace
                    } else {
                        crate::mission::RecipientWar::AtWar
                    }
                },
            )
        },
    );
    if let Some(policy) = policy {
        let digest = policy.policy_digest();
        if s.last_digest.as_ref() != Some(&digest) {
            crate::runtime::diagnostic(&format!(
                "mp_policy_active date={:?} digest={digest} missions={} (engine boundary)",
                n.field(s.world, 0xB0C),
                policy.entries().count()
            ));
            s.last_digest = Some(digest);
        }
        crate::native_hooks::publish_network(
            Some(&policy),
            Some(crate::session::admit_policy(proof)),
        );
    } else {
        // An admitted empty campaign has no borrowed rules. With journals,
        // projection failure is a shared refusal, not a UI-selected fallback.
        if !s.journals.is_empty() {
            s.admission.fail();
        }
        crate::native_hooks::publish_network(None, None);
    }
}
pub fn before_tick(n: Native) {
    let Some((session, epoch)) = session(n) else {
        return;
    };
    let Some(roster) = crate::native_roster::capture(&n, n.base) else {
        crate::native_hooks::publish_network(None, None);
        return;
    };
    if let Some(lock) = STATE.get()
        && let Ok(mut s) = lock.lock()
    {
        bind(&mut s, session, epoch, roster);
        rebuild(n, &mut s);
    }
}
/// Returns true when the gameplay-mode execution observer handled this key.
pub fn variable(n: Native, command: u32, key: &str) -> bool {
    let Some((session, epoch)) = session(n) else {
        return false;
    };
    if !key.starts_with("tgcx_vl_") {
        return false;
    }
    let hello = key.starts_with(mp_authority::HELLO_PREFIX);
    // Capture a new native roster only at hello/HEAD/barriers, not per word of
    // a large journal transaction. Source checks use the last admitted roster.
    let roster = (hello || key == crate::engine_store::HEAD)
        .then(|| crate::native_roster::capture(&n, n.base))
        .flatten();
    let Some(lock) = STATE.get() else {
        return true;
    };
    let Ok(mut s) = lock.lock() else {
        return true;
    };
    if let Some(roster) = roster {
        bind(&mut s, session, epoch, roster);
    }
    if s.epoch != epoch || s.world != session.world {
        return true;
    }
    let (Some(country), Some(source)) = (
        n.field(command, 0x40),
        n.field(command, 0x34).map(|v| v as u16),
    ) else {
        return true;
    };
    if !s.roster.as_ref().is_some_and(|r| r.sender(source, country)) {
        s.admission.fail();
        crate::native_hooks::publish_network(None, None);
        crate::runtime::diagnostic(&format!(
            "mp_sender_refused country={country} source={source} key={key}"
        ));
        return true;
    }
    if hello {
        let value = n
            .field(command, 0x60)
            .and_then(|raw| mp_authority::hello_value(key, raw));
        if let Some((fp, sequence)) = value {
            let accepted = s.admission.receive(source, country, sequence, fp);
            crate::runtime::diagnostic(&format!(
                "mp_gameplay_hello country={country} source={source} sequence={sequence} accepted={accepted} ready={}",
                s.admission.ready()
            ));
        } else {
            s.admission.fail();
        }
        rebuild(n, &mut s);
        return true;
    }
    if key == crate::engine_store::HEAD {
        match crate::engine_store::load(epoch, |k| n.variable(country, k)) {
            Ok(Some(loaded)) => {
                if loaded
                    .registry
                    .entries()
                    .any(|e| e.deployment.key.donor != country)
                {
                    s.admission.fail();
                } else {
                    crate::runtime::diagnostic(&format!(
                        "journal_commit country={country} head={} sha256={} (MP engine execution)",
                        loaded.head, loaded.journal_sha256
                    ));
                    let head = loaded.head;
                    let digest = loaded.journal_sha256.clone();
                    s.journals.insert(country, loaded);
                    rebuild(n, &mut s);
                    drop(s);
                    crate::native_gui::checkpoint_changed(country, head, &digest);
                    return true;
                }
            }
            _ => s.admission.fail(),
        }
        crate::native_hooks::publish_network(None, None);
    }
    true
}
pub fn after_tick(n: Native) {
    let Some((session, epoch)) = session(n) else {
        return;
    };
    let Some(date) = n.field(session.world, 0xB0C) else {
        return;
    };
    let Some(roster) = crate::native_roster::capture(&n, n.base) else {
        return;
    };
    let local = roster.local;
    let (fp, send) = {
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut s) = lock.lock() else {
            return;
        };
        bind(&mut s, session, epoch, roster);
        let send = !s.admission.ready() && s.last_hello != Some(date);
        s.last_hello = Some(date);
        (s.fp.clone(), send)
    };
    if !send {
        return;
    }
    let Some(key) = mp_authority::hello_key(&fp) else {
        return;
    };
    let sequence = match n.variable(local.country, &key) {
        Ok(None) => Some(1),
        Ok(Some(raw)) => mp_authority::hello_value(&key, raw as u32)
            .and_then(|(_, seq)| seq.checked_add(1))
            .filter(|v| *v <= crate::mp_sync::MAX_SEQUENCE),
        Err(_) => None,
    };
    let posted = sequence
        .and_then(|seq| n.prepare_mp_probe(epoch, local.country, &key, seq))
        .is_some_and(|b| b.post());
    crate::runtime::diagnostic(&format!(
        "mp_gameplay_hello_post country={} source={} sequence={sequence:?} queued={posted}",
        local.country, local.source
    ));
}
