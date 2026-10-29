//! Deterministic, read-only mirror of journals observed at native execution.
//! This is diagnostic replication, not authority to alter combat or issue orders.
use crate::{engine_store::Loaded, mission::Campaign, model::Identity, sha256::Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const PROBE_PREFIX: &str = "tgcx_vl_mp1_";
pub const MAX_SEQUENCE: u32 = 1_000_000;

pub fn fingerprint(plugin: &str, max_brigades: usize) -> Option<String> {
    if !valid_digest(plugin) || !(1..=64).contains(&max_brigades) {
        return None;
    }
    let mut hash = Sha256::new();
    hash.update(b"TGCX MP sync probe v1; diagnostic only; EXE62d48c20; journal02;");
    hash.update(plugin.as_bytes());
    hash.update(&(max_brigades as u32).to_le_bytes());
    Some(hash.finish())
}
fn valid_digest(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn probe_key(digest: &str) -> Option<String> {
    valid_digest(digest).then(|| format!("{PROBE_PREFIX}{digest}"))
}
pub fn probe_value(key: &str, raw: u32) -> Option<(&str, u32)> {
    let digest = key.strip_prefix(PROBE_PREFIX)?;
    if !valid_digest(digest) || raw == 0 || !raw.is_multiple_of(1000) || raw / 1000 > MAX_SEQUENCE {
        return None;
    }
    Some((digest, raw / 1000))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Unbound,
    Invalid,
    StaleGeneration,
    ConflictingGeneration,
    CampaignChanged,
    DuplicateObjects,
    SequenceRegression,
}
#[derive(Clone)]
struct Journal {
    head: u32,
    campaign: Campaign,
    digest: String,
    armies: BTreeSet<Identity>,
    regiments: BTreeSet<Identity>,
    sequence: u64,
}
#[derive(Default)]
pub struct Mirror {
    epoch: u64,
    journals: BTreeMap<u32, Journal>,
}
impl Mirror {
    /// Epoch is a local load barrier only and is deliberately absent from hashes.
    pub fn reset(&mut self, epoch: u64) {
        self.epoch = epoch;
        self.journals.clear();
    }
    pub fn commit(&mut self, country: u32, loaded: &Loaded) -> Result<bool, Refusal> {
        if self.epoch == 0 || loaded.registry.epoch != self.epoch {
            return Err(Refusal::Unbound);
        }
        if country == 0 || loaded.head == 0 || !valid_digest(&loaded.journal_sha256) {
            return Err(Refusal::Invalid);
        }
        let sequence = loaded.registry.sequence(country);
        let mut armies = BTreeSet::new();
        let mut regiments = BTreeSet::new();
        for entry in loaded.registry.entries() {
            if entry.deployment.key.donor != country
                || entry.deployment.key.campaign != loaded.registry.campaign
            {
                return Err(Refusal::Invalid);
            }
            if matches!(
                entry.mission.saved().0,
                crate::model::State::Returned | crate::model::State::Lost
            ) {
                continue;
            }
            if !armies.insert(entry.mission.plan().destination())
                || entry
                    .mission
                    .remaining()
                    .iter()
                    .any(|id| !regiments.insert(*id))
            {
                return Err(Refusal::DuplicateObjects);
            }
        }
        if let Some(old) = self.journals.get(&country) {
            if loaded.registry.campaign != old.campaign {
                return Err(Refusal::CampaignChanged);
            }
            if loaded.head < old.head {
                return Err(Refusal::StaleGeneration);
            }
            if loaded.head == old.head {
                return if loaded.journal_sha256 == old.digest {
                    Ok(false)
                } else {
                    Err(Refusal::ConflictingGeneration)
                };
            }
            if sequence < old.sequence {
                return Err(Refusal::SequenceRegression);
            }
        }
        for (&other, old) in &self.journals {
            if other != country
                && (!armies.is_disjoint(&old.armies) || !regiments.is_disjoint(&old.regiments))
            {
                return Err(Refusal::DuplicateObjects);
            }
        }
        self.journals.insert(
            country,
            Journal {
                head: loaded.head,
                campaign: loaded.registry.campaign,
                digest: loaded.journal_sha256.clone(),
                armies,
                regiments,
                sequence,
            },
        );
        Ok(true)
    }
    pub fn count(&self) -> usize {
        self.journals.len()
    }
    /// Same native committed state produces this hash despite ASLR, local player,
    /// local load epoch, UI activity or the order of independent country commits.
    pub fn digest(&self) -> String {
        let mut hash = Sha256::new();
        hash.update(b"TGCX MP journal mirror v1\0");
        for (&country, journal) in &self.journals {
            hash.update(&country.to_le_bytes());
            hash.update(&journal.head.to_le_bytes());
            hash.update(&journal.campaign.0);
            hash.update(journal.digest.as_bytes());
        }
        hash.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        engine_store::{self, Transaction},
        registry::Registry,
    };
    fn load(country: u8, epoch: u64, previous: u32) -> Loaded {
        let tx =
            Transaction::new(&Registry::new(Campaign([country; 16]), epoch), previous).unwrap();
        let variables = tx
            .writes
            .iter()
            .map(|w| (w.key.as_str(), w.raw_thousandths))
            .collect::<BTreeMap<_, _>>();
        engine_store::load(epoch, |k| Ok(variables.get(k).copied()))
            .unwrap()
            .unwrap()
    }
    fn active(country: u32) -> Loaded {
        let (mut app, mut world) = crate::send_tests::setup(1);
        world.player = country;
        world.countries[0].index = country;
        world.armies[0].owner = country;
        world.armies[0].affiliation = country;
        world.armies[0].regiments[0].owner = country;
        crate::send_tests::select(&mut app, &world, 1);
        assert_eq!(
            app.action(crate::app::UiAction::Send, &world, None).len(),
            1
        );
        let tx = Transaction::new(app.registry.as_ref().unwrap(), 0).unwrap();
        let vars = tx
            .writes
            .iter()
            .map(|w| (w.key.as_str(), w.raw_thousandths))
            .collect::<BTreeMap<_, _>>();
        engine_store::load(1, |k| Ok(vars.get(k).copied()))
            .unwrap()
            .unwrap()
    }
    #[test]
    fn foreign_donor_and_duplicate_stack_cannot_replace_a_valid_mirror() {
        let mut mirror = Mirror::default();
        mirror.reset(1);
        let first = active(625);
        assert_eq!(mirror.commit(1, &first), Err(Refusal::Invalid));
        assert_eq!(mirror.count(), 0);
        mirror.commit(625, &first).unwrap();
        let hash = mirror.digest();
        assert_eq!(
            mirror.commit(626, &active(626)),
            Err(Refusal::DuplicateObjects)
        );
        assert_eq!(mirror.digest(), hash);
    }
    #[test]
    fn higher_generation_cannot_erase_the_issuer_sequence() {
        let mut mirror = Mirror::default();
        mirror.reset(1);
        mirror.commit(625, &active(625)).unwrap();
        let hash = mirror.digest();
        // Same campaign, later journal head, but sequence zero rather than one.
        assert_eq!(
            mirror.commit(625, &load(9, 1, 1)),
            Err(Refusal::SequenceRegression)
        );
        assert_eq!(mirror.digest(), hash);
    }
    #[test]
    fn native_commit_order_and_local_epoch_do_not_change_replica_digest() {
        let mut host = Mirror::default();
        host.reset(1);
        let mut client = Mirror::default();
        client.reset(7);
        host.commit(1, &load(1, 1, 0)).unwrap();
        host.commit(2, &load(2, 1, 0)).unwrap();
        client.commit(2, &load(2, 7, 0)).unwrap();
        client.commit(1, &load(1, 7, 0)).unwrap();
        assert_eq!(host.digest(), client.digest());
        assert_eq!(client.count(), 2);
        assert!(!client.commit(1, &load(1, 7, 0)).unwrap());
        assert_eq!(client.commit(1, &load(1, 1, 0)), Err(Refusal::Unbound));
    }
    #[test]
    fn stale_conflicting_or_rebound_journal_is_not_published() {
        let mut mirror = Mirror::default();
        mirror.reset(1);
        let second = load(1, 1, 1);
        mirror.commit(1, &second).unwrap();
        let digest = mirror.digest();
        assert_eq!(
            mirror.commit(1, &load(1, 1, 0)),
            Err(Refusal::StaleGeneration)
        );
        let mut conflict = load(1, 1, 1);
        conflict.journal_sha256 = "a".repeat(64);
        assert_eq!(
            mirror.commit(1, &conflict),
            Err(Refusal::ConflictingGeneration)
        );
        assert_eq!(
            mirror.commit(1, &load(2, 1, 2)),
            Err(Refusal::CampaignChanged)
        );
        assert_eq!(mirror.digest(), digest);
        mirror.reset(2);
        assert_eq!(mirror.count(), 0);
        assert!(mirror.commit(1, &load(1, 2, 0)).unwrap());
    }
    #[test]
    fn probe_names_are_bounded_and_unknown_or_fractional_values_refuse() {
        let digest = fingerprint(&"a".repeat(64), 5).unwrap();
        let key = probe_key(&digest).unwrap();
        assert!(key.len() <= 96);
        assert_eq!(probe_value(&key, 2000), Some((digest.as_str(), 2)));
        for raw in [0, 1, 999, 1001, 1_000_001_000, u32::MAX] {
            assert!(probe_value(&key, raw).is_none());
        }
        assert!(probe_key(&"A".repeat(64)).is_none());
        assert!(probe_value("tgcx_vl_head", 1000).is_none());
        assert_ne!(fingerprint(&"b".repeat(64), 5), Some(digest.clone()));
        assert_ne!(fingerprint(&"a".repeat(64), 6), Some(digest));
    }
}
