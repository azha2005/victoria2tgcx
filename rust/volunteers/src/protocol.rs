//! Bounded deterministic mission payloads. This is not a registered native
//! command ID or a multiplayer transport. No socket or local orders file.
use crate::mission::{Campaign, MissionKey};
use crate::model::Identity;

pub const VERSION: u32 = 1;
pub const MAX_REGIMENTS: usize = 64;
pub const MAX_BYTES: usize = 4096;
const MAGIC: &[u8; 8] = b"TGCXVOL1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Deploy {
        donor_tag: u32,
        source: Identity,
        destination: Identity,
        origin: u32,
        target: u32,
        regiments: Vec<Identity>,
    },
    Recall {
        return_province: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Envelope {
    pub key: MissionKey,
    pub sequence: u64,
    pub issued_on: u32,
    pub action: Action,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    Length,
    Version,
    Invalid,
    Trailing,
}

impl Envelope {
    pub fn valid(&self) -> bool {
        if self.sequence == 0
            || self.key.serial == 0
            || self.key.campaign.0 == [0; 16]
            || self.key.donor == 0
            || self.key.recipient == 0
            || self.key.donor == self.key.recipient
        {
            return false;
        }
        match &self.action {
            Action::Recall { return_province } => *return_province != 0,
            Action::Deploy {
                donor_tag,
                source,
                destination,
                origin,
                target,
                regiments,
            } => {
                *donor_tag != 0
                    && source.valid()
                    && destination.valid()
                    && *origin != 0
                    && *target != 0
                    && !regiments.is_empty()
                    && regiments.len() <= MAX_REGIMENTS
                    && regiments.iter().all(|r| r.valid() && r != destination)
                    && regiments
                        .iter()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        == regiments.len()
            }
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, DecodeError> {
        if !self.valid() {
            return Err(DecodeError::Invalid);
        }
        let mut w = Writer(Vec::new());
        w.0.extend_from_slice(MAGIC);
        w.u32(VERSION);
        w.0.extend_from_slice(&self.key.campaign.0);
        w.u64(self.key.serial);
        w.u32(self.key.donor);
        w.u32(self.key.recipient);
        w.u64(self.sequence);
        w.u32(self.issued_on);
        match &self.action {
            Action::Deploy {
                donor_tag,
                source,
                destination,
                origin,
                target,
                regiments,
            } => {
                w.u32(1);
                w.u32(*donor_tag);
                w.identity(*source);
                w.identity(*destination);
                w.u32(*origin);
                w.u32(*target);
                w.u32(regiments.len() as u32);
                for id in regiments {
                    w.identity(*id);
                }
            }
            Action::Recall { return_province } => {
                w.u32(2);
                w.u32(*return_province);
            }
        }
        Ok(w.0)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() > MAX_BYTES {
            return Err(DecodeError::Length);
        }
        let mut r = Reader { bytes, cursor: 0 };
        if r.take(8)? != MAGIC || r.u32()? != VERSION {
            return Err(DecodeError::Version);
        }
        let campaign = Campaign(r.take(16)?.try_into().map_err(|_| DecodeError::Length)?);
        let key = MissionKey {
            campaign,
            serial: r.u64()?,
            donor: r.u32()?,
            recipient: r.u32()?,
        };
        let sequence = r.u64()?;
        let issued_on = r.u32()?;
        let action = match r.u32()? {
            1 => {
                let donor_tag = r.u32()?;
                let source = r.identity()?;
                let destination = r.identity()?;
                let origin = r.u32()?;
                let target = r.u32()?;
                let count = r.u32()? as usize;
                if count == 0 || count > MAX_REGIMENTS {
                    return Err(DecodeError::Length);
                }
                let mut regiments = Vec::with_capacity(count);
                for _ in 0..count {
                    regiments.push(r.identity()?);
                }
                Action::Deploy {
                    donor_tag,
                    source,
                    destination,
                    origin,
                    target,
                    regiments,
                }
            }
            2 => Action::Recall {
                return_province: r.u32()?,
            },
            _ => return Err(DecodeError::Invalid),
        };
        if r.cursor != bytes.len() {
            return Err(DecodeError::Trailing);
        }
        let result = Self {
            key,
            sequence,
            issued_on,
            action,
        };
        if !result.valid() {
            return Err(DecodeError::Invalid);
        }
        Ok(result)
    }
}

pub(crate) struct Writer(pub Vec<u8>);
impl Writer {
    pub fn u32(&mut self, n: u32) {
        self.0.extend_from_slice(&n.to_le_bytes());
    }
    pub fn u64(&mut self, n: u64) {
        self.0.extend_from_slice(&n.to_le_bytes());
    }
    pub fn identity(&mut self, id: Identity) {
        self.u32(id.kind);
        self.u32(id.id);
    }
}
pub(crate) struct Reader<'a> {
    pub bytes: &'a [u8],
    pub cursor: usize,
}
impl<'a> Reader<'a> {
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        let end = self.cursor.checked_add(n).ok_or(DecodeError::Length)?;
        let bytes = self
            .bytes
            .get(self.cursor..end)
            .ok_or(DecodeError::Length)?;
        self.cursor = end;
        Ok(bytes)
    }
    pub fn u32(&mut self) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().map_err(|_| DecodeError::Length)?,
        ))
    }
    pub fn u64(&mut self) -> Result<u64, DecodeError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().map_err(|_| DecodeError::Length)?,
        ))
    }
    pub fn identity(&mut self) -> Result<Identity, DecodeError> {
        Ok(Identity {
            kind: self.u32()?,
            id: self.u32()?,
        })
    }
}
