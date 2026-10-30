//! Admission from current-load native execution receipts, never saved markers.
use crate::sha256::Sha256;
use std::collections::BTreeMap;
pub const HELLO_PREFIX: &str = "tgcx_vl_mp2_";
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peer {
    pub source: u16,
    pub country: u32,
    pub tag: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roster {
    pub peers: Vec<Peer>,
    pub local: Peer,
}
impl Roster {
    pub fn canonical(&self) -> Option<Vec<Peer>> {
        let mut peers = self.peers.clone();
        peers.sort();
        if !(2..=32).contains(&peers.len())
            || !peers.contains(&self.local)
            || peers
                .iter()
                .any(|p| p.country == 0 || p.source > 0x7FFF || p.tag == 0 || p.tag == 0x2D2D2D)
            || peers.windows(2).any(|w| w[0].source == w[1].source)
            || peers
                .iter()
                .map(|p| p.country)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != peers.len()
        {
            return None;
        }
        Some(peers)
    }
    pub fn sender(&self, source: u16, country: u32) -> bool {
        self.peers
            .iter()
            .any(|p| p.source == source && p.country == country)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IssuerAuthority {
    epoch: u64,
    country: u32,
    round: u64,
}
impl IssuerAuthority {
    pub fn epoch(self) -> u64 {
        self.epoch
    }
    pub fn country(self) -> u32 {
        self.country
    }
    pub(crate) fn round(self) -> u64 {
        self.round
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PolicyAuthority {
    epoch: u64,
}
impl PolicyAuthority {
    pub fn epoch(self) -> u64 {
        self.epoch
    }
}
#[derive(Default)]
pub struct Admission {
    epoch: u64,
    roster: Vec<Peer>,
    fingerprint: String,
    received: BTreeMap<u16, u32>,
    failed: bool,
    round: u64,
}
impl Admission {
    pub fn bind(&mut self, epoch: u64, roster: &Roster, fingerprint: &str) -> bool {
        let Some(peers) = roster.canonical() else {
            self.failed = true;
            return false;
        };
        if epoch == 0 || fingerprint.len() != 64 {
            self.failed = true;
            return false;
        }
        if self.epoch != epoch || self.roster != peers || self.fingerprint != fingerprint {
            self.epoch = epoch;
            self.roster = peers;
            self.fingerprint = fingerprint.to_owned();
            let Some(round) = self.round.checked_add(1) else {
                self.failed = true;
                return false;
            };
            self.round = round;
            self.received.clear();
            self.failed = false;
        }
        !self.failed
    }
    pub fn receive(&mut self, source: u16, country: u32, sequence: u32, fingerprint: &str) -> bool {
        if self.epoch == 0
            || self.failed
            || sequence == 0
            || !self
                .roster
                .iter()
                .any(|p| p.source == source && p.country == country)
        {
            return false;
        }
        if fingerprint != self.fingerprint {
            self.failed = true;
            return false;
        }
        let old = self.received.entry(source).or_default();
        if sequence < *old {
            self.failed = true;
            return false;
        }
        *old = sequence;
        true
    }
    pub fn fail(&mut self) {
        self.failed = true;
    }
    /// A failed roster observation withdraws receipts without reusing an old
    /// authority round. A later complete roster needs fresh native HELLOs.
    pub fn withdraw(&mut self) {
        self.received.clear();
        self.roster.clear();
    }
    pub fn ready(&self) -> bool {
        !self.failed
            && self.epoch != 0
            && !self.roster.is_empty()
            && self
                .roster
                .iter()
                .all(|p| self.received.contains_key(&p.source))
    }
    pub fn issuer(&self, country: u32) -> Option<IssuerAuthority> {
        (self.ready() && self.roster.iter().any(|p| p.country == country)).then_some(
            IssuerAuthority {
                epoch: self.epoch,
                country,
                round: self.round,
            },
        )
    }
    pub fn policy(&self) -> Option<PolicyAuthority> {
        self.ready()
            .then_some(PolicyAuthority { epoch: self.epoch })
    }
}
pub fn fingerprint(plugin: &str, max: usize) -> Option<String> {
    if crate::mp_sync::probe_key(plugin).is_none() || !(1..=64).contains(&max) {
        return None;
    }
    let mut hash = Sha256::new();
    hash.update(b"TGCX MP gameplay v1; whole-army; native journal02;");
    hash.update(plugin.as_bytes());
    hash.update(&(max as u32).to_le_bytes());
    Some(hash.finish())
}
pub fn hello_key(fingerprint: &str) -> Option<String> {
    crate::mp_sync::probe_key(fingerprint).map(|_| format!("{HELLO_PREFIX}{fingerprint}"))
}
pub fn hello_value(key: &str, raw: u32) -> Option<(&str, u32)> {
    let digest = key.strip_prefix(HELLO_PREFIX)?;
    if crate::mp_sync::probe_key(digest).is_none()
        || raw == 0
        || !raw.is_multiple_of(1000)
        || raw / 1000 > crate::mp_sync::MAX_SEQUENCE
    {
        return None;
    }
    Some((digest, raw / 1000))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn roster(local: usize) -> Roster {
        let peers = vec![
            Peer {
                source: 0,
                country: 1,
                tag: 0x414141,
            },
            Peer {
                source: 1,
                country: 2,
                tag: 0x424242,
            },
        ];
        Roster {
            local: peers[local],
            peers,
        }
    }
    #[test]
    fn withdrawn_roster_requires_fresh_receipts_and_invalidates_old_issuer_round() {
        let roster = roster(0);
        let fp = "a".repeat(64);
        let mut a = Admission::default();
        a.bind(1, &roster, &fp);
        for p in &roster.peers {
            assert!(a.receive(p.source, p.country, 1, &fp));
        }
        let old = a.issuer(roster.local.country).unwrap();
        let caps = crate::session::Capabilities {
            session_detection: true,
            campaign_binding: true,
            command_queue: true,
            combat: true,
            access: true,
            vision: true,
            persistence: true,
        };
        let permit = crate::session::admit_issuer(old, caps, 1, roster.local.country).unwrap();
        a.withdraw();
        assert!(!a.ready());
        assert!(a.issuer(roster.local.country).is_none());
        a.bind(1, &roster, &fp);
        assert!(!a.ready());
        for p in &roster.peers {
            assert!(a.receive(p.source, p.country, 2, &fp));
        }
        assert!(!permit.authorized_by(a.issuer(roster.local.country).unwrap()));
    }
    #[test]
    fn all_current_load_peers_and_correct_sources_are_required() {
        let mut a = Admission::default();
        let fp = "a".repeat(64);
        assert!(a.bind(1, &roster(0), &fp));
        assert!(!a.ready());
        assert!(!a.receive(0, 2, 1, &fp));
        assert!(a.receive(0, 1, 1, &fp));
        assert!(!a.ready());
        assert!(a.receive(1, 2, 1, &fp));
        assert!(a.ready());
        assert_eq!(a.issuer(2).unwrap().country(), 2);
        assert!(a.bind(2, &roster(0), &fp));
        assert!(!a.ready());
    }
    #[test]
    fn different_local_players_same_receipts_and_reload_invalidate_authority() {
        let fp = "b".repeat(64);
        let mut a = Admission::default();
        let mut b = Admission::default();
        a.bind(1, &roster(0), &fp);
        b.bind(8, &roster(1), &fp);
        for p in roster(0).peers {
            a.receive(p.source, p.country, 1, &fp);
            b.receive(p.source, p.country, 1, &fp);
        }
        assert!(a.ready() && b.ready());
        a.receive(1, 2, 2, &"c".repeat(64));
        assert!(!a.ready());
        b.bind(9, &roster(1), &fp);
        assert!(!b.ready());
    }
}
