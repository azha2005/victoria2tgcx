//! Save-backed registry storage through country variables. Every write is a
//! native CSetVariableCommand; readers never edit the engine's variable tree.
use crate::mission::Campaign;
use crate::registry::Registry;
use crate::sha256::Sha256;

pub const MAX_BYTES: usize = 16 * 1024;
pub const HEAD: &str = "tgcx_vl_head";
const MAGIC: &[u8; 8] = b"TVLREG02";
const MAX_GENERATION: u32 = 1_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Write {
    pub key: String,
    pub raw_thousandths: i32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Invalid,
    TooLarge,
    Missing,
    Changed,
    Exhausted,
    Digest,
}
pub struct Transaction {
    pub expected_head: u32,
    pub committed_head: u32,
    pub writes: Vec<Write>,
    pub journal_sha256: String,
}
fn integer(raw: Option<i32>) -> Result<u32, Error> {
    let raw = raw.ok_or(Error::Missing)?;
    if raw < 0 || raw % 1000 != 0 {
        return Err(Error::Invalid);
    }
    Ok((raw / 1000) as u32)
}
fn checksum(bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(bytes);
    hash.finish()
}
fn word_key(bank: u32, index: usize) -> String {
    format!("tgcx_vl_b{bank}_w{index}")
}
fn length_key(bank: u32) -> String {
    format!("tgcx_vl_b{bank}_len")
}

impl Transaction {
    pub fn new(registry: &Registry, previous: u32) -> Result<Self, Error> {
        if previous > MAX_GENERATION || registry.epoch == 0 {
            return Err(Error::Invalid);
        }
        let next = previous
            .checked_add(1)
            .filter(|n| *n <= MAX_GENERATION)
            .ok_or(Error::Exhausted)?;
        let payload = registry.encode().map_err(|_| Error::Invalid)?;
        let mut bytes = Vec::from(*MAGIC);
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&payload);
        let journal_sha256 = checksum(&bytes);
        bytes.extend_from_slice(journal_sha256.as_bytes());
        if bytes.len() > MAX_BYTES {
            return Err(Error::TooLarge);
        }
        let bank = next & 1;
        let mut writes = Vec::with_capacity(bytes.len().div_ceil(2) + 2);
        for (i, pair) in bytes.chunks(2).enumerate() {
            let word = u16::from_le_bytes([pair[0], *pair.get(1).unwrap_or(&0)]);
            writes.push(Write {
                key: word_key(bank, i),
                raw_thousandths: i32::from(word) * 1000,
            });
        }
        writes.push(Write {
            key: length_key(bank),
            raw_thousandths: (bytes.len() as i32) * 1000,
        });
        // Commit last. An interrupted uncommitted bank cannot replace the old
        // head. The engine's own save captures these variables with the world.
        writes.push(Write {
            key: HEAD.into(),
            raw_thousandths: (next as i32) * 1000,
        });
        Ok(Self {
            expected_head: previous,
            committed_head: next,
            writes,
            journal_sha256,
        })
    }
}

pub struct Loaded {
    pub head: u32,
    pub registry: Registry,
    pub journal_sha256: String,
}
/// Missing head means no volunteer registry. Missing/partial content under a
/// nonzero head is an error, never a fresh empty campaign or a replay request.
pub fn load(
    epoch: u64,
    mut read: impl FnMut(&str) -> Result<Option<i32>, Error>,
) -> Result<Option<Loaded>, Error> {
    if epoch == 0 {
        return Err(Error::Invalid);
    }
    let head = match read(HEAD)? {
        None | Some(0) => return Ok(None),
        n => integer(n)?,
    };
    if head == 0 || head > MAX_GENERATION {
        return Err(Error::Invalid);
    }
    let bank = head & 1;
    let count = integer(read(&length_key(bank))?)? as usize;
    if !(12 + 16 + 64..=MAX_BYTES).contains(&count) {
        return Err(Error::TooLarge);
    }
    let mut bytes = Vec::with_capacity(count.div_ceil(2) * 2);
    for i in 0..count.div_ceil(2) {
        let value = integer(read(&word_key(bank, i))?)?;
        let word = u16::try_from(value).map_err(|_| Error::Invalid)?;
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes.truncate(count);
    if integer(read(HEAD)?)? != head {
        return Err(Error::Changed);
    }
    let (body, digest) = bytes.split_at(bytes.len() - 64);
    let journal_sha256 = checksum(body);
    if digest != journal_sha256.as_bytes() {
        return Err(Error::Digest);
    }
    if body.get(..8) != Some(MAGIC) {
        return Err(Error::Invalid);
    }
    let length = u32::from_le_bytes(body[8..12].try_into().map_err(|_| Error::Invalid)?) as usize;
    if length != body.len() - 12 {
        return Err(Error::Invalid);
    }
    let payload = &body[12..];
    let campaign = Campaign(
        payload
            .get(..16)
            .ok_or(Error::Invalid)?
            .try_into()
            .map_err(|_| Error::Invalid)?,
    );
    if campaign.0 == [0; 16] {
        return Err(Error::Invalid);
    }
    let registry = Registry::decode(payload, campaign, epoch).map_err(|_| Error::Invalid)?;
    Ok(Some(Loaded {
        head,
        registry,
        journal_sha256,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    #[test]
    fn native_words_roundtrip_suspended_and_commit_is_last() {
        let registry = Registry::new(Campaign([7; 16]), 3);
        let tx = Transaction::new(&registry, 0).unwrap();
        assert_eq!(tx.writes.last().unwrap().key, HEAD);
        let vars = tx
            .writes
            .iter()
            .map(|w| (w.key.clone(), w.raw_thousandths))
            .collect::<BTreeMap<_, _>>();
        let result = load(9, |key| Ok(vars.get(key).copied())).unwrap().unwrap();
        assert_eq!(result.registry.campaign, registry.campaign);
        assert_eq!(result.registry.epoch, 9);
        assert_eq!(result.head, 1);
    }
    #[test]
    fn partial_second_bank_preserves_previous_registry() {
        let registry = Registry::new(Campaign([7; 16]), 3);
        let first = Transaction::new(&registry, 0).unwrap();
        let mut vars = first
            .writes
            .iter()
            .map(|w| (w.key.clone(), w.raw_thousandths))
            .collect::<BTreeMap<_, _>>();
        let next = Transaction::new(&Registry::new(Campaign([8; 16]), 3), 1).unwrap();
        for w in next.writes.iter().take(next.writes.len() / 2) {
            vars.insert(w.key.clone(), w.raw_thousandths);
        }
        assert_eq!(
            load(9, |key| Ok(vars.get(key).copied()))
                .unwrap()
                .unwrap()
                .registry
                .campaign,
            registry.campaign
        );
    }
    #[test]
    fn malformed_or_changed_head_is_never_empty_campaign() {
        assert!(load(1, |_| Ok(Some(1234))).is_err());
        assert!(load(1, |key| Ok(if key == HEAD { Some(1000) } else { None })).is_err());
        assert!(Transaction::new(&Registry::new(Campaign([1; 16]), 1), MAX_GENERATION).is_err());
    }
}
