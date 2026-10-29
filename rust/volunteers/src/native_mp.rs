//! Opt-in native MP queue probe and read-only replicated journal diagnostics.
//! No troop commands, registry authority, combat scope or checksum masking.
use crate::{
    mp_sync::{self, Mirror},
    native::Native,
    session::SessionMode,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

static STATE: OnceLock<Mutex<State>> = OnceLock::new();
static MISSED: AtomicU32 = AtomicU32::new(0);
struct State {
    probe: bool,
    fingerprint: String,
    epoch: u64,
    world: u32,
    mirror: Mirror,
    cursor: u32,
    refused: u32,
    last_date: Option<u32>,
    attempted: BTreeSet<u32>,
    seen: BTreeMap<u32, (String, u32)>,
}
pub fn initialize(probe: bool, fingerprint: String) {
    let _ = STATE.set(Mutex::new(State {
        probe,
        fingerprint,
        epoch: 0,
        world: 0,
        mirror: Mirror::default(),
        cursor: 1,
        refused: 0,
        last_date: None,
        attempted: BTreeSet::new(),
        seen: BTreeMap::new(),
    }));
}
pub fn invalidate() {
    if let Some(lock) = STATE.get() {
        if let Ok(mut s) = lock.try_lock() {
            s.epoch = 0;
            s.world = 0;
            s.mirror.reset(0);
        } else {
            MISSED.fetch_add(1, Ordering::Relaxed);
        }
    }
}
fn bind(s: &mut State, world: u32, epoch: u64) {
    if s.epoch != epoch || s.world != world {
        s.epoch = epoch;
        s.world = world;
        s.cursor = 1;
        s.refused = 0;
        s.last_date = None;
        s.attempted.clear();
        s.seen.clear();
        s.mirror.reset(epoch);
        MISSED.store(0, Ordering::Relaxed);
    }
}
pub fn journal(n: Native, country: u32, loaded: &crate::engine_store::Loaded) {
    let Some(session) = crate::native_session::capture(&n, n.base) else {
        return;
    };
    if session.mode != SessionMode::Multiplayer {
        return;
    }
    let Some(epoch) = crate::native_lifecycle::epoch(session.context) else {
        return;
    };
    let Some(lock) = STATE.get() else {
        return;
    };
    let Ok(mut s) = lock.try_lock() else {
        MISSED.fetch_add(1, Ordering::Relaxed);
        return;
    };
    bind(&mut s, session.world, epoch);
    match s.mirror.commit(country, loaded) {
        Ok(changed) => crate::runtime::diagnostic(&format!(
            "mp_journal country={country} head={} changed={changed} digest={} (native execute; mirror only)",
            loaded.head, loaded.journal_sha256
        )),
        Err(error) => {
            s.refused += 1;
            crate::runtime::diagnostic(&format!(
                "mp_journal_refused country={country} reason={error:?}"
            ));
        }
    }
}
pub fn probe_executed(n: Native, command: u32, key: &str) {
    let Some(raw) = n.field(command, 0x60) else {
        return;
    };
    let Some((digest, sequence)) = mp_sync::probe_value(key, raw) else {
        return;
    };
    let Some(session) = crate::native_session::capture(&n, n.base) else {
        return;
    };
    if session.mode != SessionMode::Multiplayer {
        return;
    }
    let Some(epoch) = crate::native_lifecycle::epoch(session.context) else {
        return;
    };
    let Some(country) = n.field(command, 0x40) else {
        return;
    };
    let Some(country_ptr) = n.country(country) else {
        return;
    };
    if n.field(command, 0x3C) != n.field(country_ptr, 0x1C)
        || n.variable(country, key).ok().flatten() != Some(raw as i32)
    {
        return;
    }
    let Some(lock) = STATE.get() else {
        return;
    };
    let Ok(mut s) = lock.try_lock() else {
        MISSED.fetch_add(1, Ordering::Relaxed);
        return;
    };
    bind(&mut s, session.world, epoch);
    let repeated = s
        .seen
        .get(&country)
        .is_some_and(|old| old.0 == digest && old.1 >= sequence);
    if !repeated {
        s.seen.insert(country, (digest.to_owned(), sequence));
    }
    let source = n.field(command, 0x34).map(|raw| raw as u16);
    crate::runtime::diagnostic(&format!(
        "mp_probe_execute date={:?} country={country} sequence={sequence} fingerprint={digest} matches_local={} repeated={repeated} native_source={source:?} (source recorded, country authority NOT established)",
        n.field(session.world, 0xB0C),
        digest == s.fingerprint
    ));
}
pub fn tick(n: Native) {
    if !crate::native_journal::ready() || !crate::native_lifecycle::ready() {
        return;
    }
    let Some(session) = crate::native_session::capture(&n, n.base) else {
        return;
    };
    if session.mode != SessionMode::Multiplayer {
        return;
    }
    let Some(epoch) = crate::native_lifecycle::epoch(session.context) else {
        return;
    };
    let Some(player) = n.field(session.world, 0xB60) else {
        return;
    };
    let Some(date) = n.field(session.world, 0xB0C) else {
        return;
    };
    let Some(manager) = n.u32(n.base + 0xE587E4) else {
        return;
    };
    let Some(count) = n.u32(manager).filter(|v| *v <= 4096) else {
        return;
    };
    let Some(lock) = STATE.get() else {
        return;
    };
    let (probe, fingerprint) = {
        let Ok(mut s) = lock.try_lock() else {
            MISSED.fetch_add(1, Ordering::Relaxed);
            return;
        };
        bind(&mut s, session.world, epoch);
        if s.last_date == Some(date) {
            return;
        }
        s.last_date = Some(date);
        // Load/late-join recovery discovers country journals independently of
        // the local player's GUI. Bounded scanning; no replay or unit commands.
        for _ in 0..16 {
            if s.cursor >= count {
                break;
            }
            let country = s.cursor;
            s.cursor += 1;
            if n.country(country).is_none() {
                continue;
            }
            match n.variable(country, crate::engine_store::HEAD) {
                Ok(None | Some(0)) => (),
                Ok(Some(_)) => match crate::engine_store::load(epoch, |k| n.variable(country, k)) {
                    Ok(Some(loaded)) => {
                        if let Err(error) = s.mirror.commit(country, &loaded) {
                            s.refused += 1;
                            crate::runtime::diagnostic(&format!(
                                "mp_bootstrap_refused country={country} reason={error:?}"
                            ));
                        }
                    }
                    _ => s.refused += 1,
                },
                Err(_) => s.refused += 1,
            }
        }
        crate::runtime::diagnostic(&format!(
            "mp_sync_sample date={date} journals={} digest={} bootstrap_complete={} refused={} missed={} probe_countries={} (diagnostic only; no gameplay admission)",
            s.mirror.count(),
            s.mirror.digest(),
            s.cursor >= count,
            s.refused,
            MISSED.load(Ordering::Relaxed),
            s.seen.len()
        ));
        let probe = s.probe && s.attempted.insert(player);
        (probe, s.fingerprint.clone())
    };
    if !probe {
        return;
    }
    let Some(key) = mp_sync::probe_key(&fingerprint) else {
        return;
    };
    let sequence = match n.variable(player, &key) {
        Ok(None) => Some(1),
        Ok(Some(raw)) => mp_sync::probe_value(&key, raw as u32)
            .and_then(|(_, seq)| seq.checked_add(1))
            .filter(|v| *v <= mp_sync::MAX_SEQUENCE),
        Err(_) => None,
    };
    let queued = sequence
        .and_then(|seq| n.prepare_mp_probe(epoch, player, &key, seq))
        .is_some_and(|batch| batch.post());
    crate::runtime::diagnostic(&format!(
        "mp_probe_post date={date} country={player} sequence={sequence:?} queued={queued} fingerprint={fingerprint} (one variable command; no automatic retry; execution receipt required)"
    ));
}
