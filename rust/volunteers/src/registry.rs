//! Single-player mission coordinator. Native execution and disk I/O are explicit
//! adapters; this module never alters simulation objects or silently replays work.
use crate::mission::{
    Campaign, DeploymentPlan, Mission, MissionKey, RecipientWar, StackReceipt, Use,
};
use crate::model::{Event, Identity, Intent, SendRequest, Settings, State};
use crate::protocol::{Action, DecodeError, Envelope, Reader, Writer};
use crate::session::SinglePlayerPermit;
use std::collections::BTreeMap;

pub const MAX_MISSIONS: usize = 256;

#[derive(Clone, Debug)]
pub struct Entry {
    pub deployment: Envelope,
    pub mission: Mission,
    pub target: u32,
    pub return_province: Option<u32>,
    pub last_processed_date: Option<u32>,
    pub outward_posted: bool,
    pub assembly_verified: bool,
}

#[derive(Clone, Debug)]
pub struct Registry {
    pub campaign: Campaign,
    pub epoch: u64,
    entries: BTreeMap<u64, Entry>,
    sequences: BTreeMap<u32, u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    WrongCampaign,
    StaleEpoch,
    Unauthorized,
    Duplicate,
    SequenceGap,
    Full,
    Invalid,
    Missing,
}

#[derive(Clone, Debug)]
pub enum Effect {
    Detach {
        key: MissionKey,
        plan: DeploymentPlan,
        target: u32,
    },
    Return {
        key: MissionKey,
        army: Identity,
        province: u32,
    },
}

impl Registry {
    pub fn new(campaign: Campaign, epoch: u64) -> Self {
        Self {
            campaign,
            epoch,
            entries: BTreeMap::new(),
            sequences: BTreeMap::new(),
        }
    }
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.values()
    }
    pub fn has_native_scope(&self) -> bool {
        self.epoch != 0
            && self.entries.values().any(|e| {
                matches!(
                    e.mission.state(),
                    State::Serving
                        | State::ReturnPending
                        | State::ReturnReady
                        | State::AwaitingReturn
                ) || e.mission.sees_province(
                    self.campaign,
                    e.deployment.key.donor,
                    e.deployment.key.recipient,
                    false,
                )
            })
    }
    pub fn entry(&self, serial: u64) -> Option<&Entry> {
        self.entries.get(&serial)
    }
    pub fn live_army(&self, army: crate::model::Identity) -> Option<&Entry> {
        self.entries.values().find(|e| {
            e.mission.plan().destination() == army
                && !matches!(e.mission.saved().0, State::Returned | State::Lost)
        })
    }
    pub fn next_serial(&self) -> Option<u64> {
        self.entries
            .keys()
            .next_back()
            .copied()
            .unwrap_or(0)
            .checked_add(1)
    }
    pub fn next_sequence(&self, donor: u32) -> Option<u64> {
        self.sequences
            .get(&donor)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
    }
    pub fn deployed(&self, donor: u32) -> usize {
        self.entries
            .values()
            .filter(|e| {
                e.deployment.key.donor == donor
                    && !matches!(e.mission.state(), State::Returned | State::Lost)
            })
            .map(|e| e.mission.remaining().len())
            .sum()
    }
    pub fn accept(
        &mut self,
        permit: SinglePlayerPermit,
        issuer: u32,
        event: Envelope,
        request: Option<&SendRequest>,
        settings: &Settings,
    ) -> Result<Effect, Error> {
        if permit.epoch() != self.epoch || self.epoch == 0 {
            return Err(Error::StaleEpoch);
        }
        if event.key.campaign != self.campaign {
            return Err(Error::WrongCampaign);
        }
        if !event.valid() {
            return Err(Error::Invalid);
        }
        if issuer != event.key.donor {
            return Err(Error::Unauthorized);
        }
        let previous = self.sequences.get(&issuer).copied().unwrap_or(0);
        if event.sequence <= previous {
            return Err(Error::Duplicate);
        }
        if event.sequence != previous.checked_add(1).ok_or(Error::Invalid)? {
            return Err(Error::SequenceGap);
        }
        let effect = match &event.action {
            Action::Deploy {
                donor_tag,
                destination,
                origin,
                target,
                regiments,
                source,
            } => {
                if self.entries.len() >= MAX_MISSIONS {
                    return Err(Error::Full);
                }
                if self.entries.contains_key(&event.key.serial) {
                    return Err(Error::Duplicate);
                }
                let request = request.ok_or(Error::Invalid)?;
                if request.source != *source
                    || request.regiments != *regiments
                    || request.already_deployed_by_donor != self.deployed(issuer)
                {
                    return Err(Error::Invalid);
                }
                if self.entries.values().any(|e| {
                    (e.mission.plan().destination() == *destination
                        && !matches!(e.mission.state(), State::Returned | State::Lost))
                        || (!matches!(e.mission.state(), State::Returned | State::Lost)
                            && e.mission.remaining().iter().any(|r| regiments.contains(r)))
                }) {
                    return Err(Error::Duplicate);
                }
                let plan = DeploymentPlan::new(
                    event.key,
                    request,
                    settings,
                    *donor_tag,
                    *destination,
                    *origin,
                )
                .map_err(|_| Error::Invalid)?;
                let mut mission = Mission::new(plan.clone());
                if mission.apply(Event::Approved) != Intent::QueueDeployment {
                    return Err(Error::Invalid);
                }
                self.entries.insert(
                    event.key.serial,
                    Entry {
                        deployment: event.clone(),
                        mission,
                        target: *target,
                        return_province: None,
                        last_processed_date: None,
                        outward_posted: false,
                        assembly_verified: false,
                    },
                );
                Effect::Detach {
                    key: event.key,
                    plan,
                    target: *target,
                }
            }
            Action::Recall { return_province } => {
                let e = self
                    .entries
                    .get_mut(&event.key.serial)
                    .ok_or(Error::Missing)?;
                if e.deployment.key != event.key {
                    return Err(Error::Unauthorized);
                }
                if !matches!(
                    e.mission.state(),
                    State::Serving | State::AwaitingArrival | State::ReturnPending
                ) {
                    return Err(Error::Invalid);
                }
                e.mission.apply(Event::Recall);
                e.return_province = Some(*return_province);
                Effect::Return {
                    key: event.key,
                    army: e.mission.plan().destination(),
                    province: *return_province,
                }
            }
        };
        self.sequences.insert(issuer, event.sequence);
        Ok(effect)
    }

    pub fn deployment_posted(&mut self, serial: u64, accepted: bool) {
        if let Some(e) = self.entries.get_mut(&serial) {
            e.mission.apply(if accepted {
                Event::DeploymentPosted
            } else {
                Event::CommandRefused
            });
        }
    }
    pub fn outward_posted(&mut self, serial: u64, posted: bool) {
        if let Some(e) = self.entries.get_mut(&serial) {
            e.outward_posted = posted;
            if !posted {
                e.mission.apply(Event::CommandRefused);
            }
        }
    }
    pub fn command_refused(&mut self, serial: u64) {
        if let Some(e) = self.entries.get_mut(&serial) {
            e.mission.apply(Event::CommandRefused);
        }
    }
    pub fn request_return(&mut self, serial: u64, province: u32) -> bool {
        if province == 0 {
            return false;
        }
        let Some(e) = self.entries.get_mut(&serial) else {
            return false;
        };
        if !matches!(
            e.mission.state(),
            State::Serving | State::AwaitingArrival | State::ReturnPending
        ) {
            return false;
        }
        e.mission.apply(Event::Recall);
        e.return_province = Some(province);
        true
    }
    pub fn receipt(&mut self, serial: u64, receipt: &StackReceipt) -> bool {
        let Some(e) = self.entries.get_mut(&serial) else {
            return false;
        };
        if !e.assembly_verified {
            let ok = e.mission.verify_assembly(receipt).is_ok();
            e.assembly_verified = ok;
            ok
        } else {
            e.mission.reconcile_survivors(receipt).is_ok()
        }
    }
    pub fn return_posted(&mut self, serial: u64, ready: bool, posted: bool) {
        if let Some(e) = self.entries.get_mut(&serial) {
            if ready {
                e.mission.apply(Event::SourceReady);
            }
            if posted {
                e.mission.apply(Event::ReturnPosted);
            }
        }
    }
    pub fn return_receipt(
        &mut self,
        serial: u64,
        receipt: &StackReceipt,
        province_owner: u32,
    ) -> bool {
        self.entries
            .get_mut(&serial)
            .is_some_and(|e| e.mission.verify_return(receipt, province_owner).is_ok())
    }
    pub fn observe_day(&mut self, date: u32, mut war: impl FnMut(u32) -> RecipientWar) {
        for e in self.entries.values_mut() {
            if e.last_processed_date.is_some_and(|old| date < old) {
                continue;
            }
            // Vision is inspected for terminal troop records too.
            e.mission.observe_recipient(war(e.deployment.key.recipient));
            e.last_processed_date = Some(date);
        }
    }
    pub fn destroyed(&mut self, serial: u64) {
        if let Some(e) = self.entries.get_mut(&serial) {
            e.mission.apply(Event::ObjectsDestroyed);
        }
    }
    pub fn suspend(&mut self) {
        for e in self.entries.values_mut() {
            e.mission.apply(Event::LoadOrCampaignChange);
        }
        self.epoch = 0;
    }
    pub fn revalidate(&mut self, serial: u64, receipt: &StackReceipt, war: RecipientWar) -> bool {
        if self.epoch == 0 {
            return false;
        }
        self.entries.get_mut(&serial).is_some_and(|e| {
            if e.mission.resume_validated(receipt, war).is_err() {
                return false;
            }
            // A complete receipt can finish an interrupted assembly on load.
            // Keep the coordinator in agreement; do not verify assembly twice.
            if matches!(
                e.mission.state(),
                State::Serving
                    | State::ReturnPending
                    | State::ReturnReady
                    | State::AwaitingReturn
                    | State::Returned
            ) {
                e.assembly_verified = true;
            }
            true
        })
    }
    pub fn revalidate_terminal_vision(
        &mut self,
        permit: SinglePlayerPermit,
        serial: u64,
        war: RecipientWar,
    ) -> bool {
        if self.epoch == 0 || permit.epoch() != self.epoch {
            return false;
        }
        self.entries
            .get_mut(&serial)
            .is_some_and(|e| e.mission.resume_terminal_vision(self.campaign, war))
    }
    pub fn context(&self, receipt: &StackReceipt, purpose: Use, war: RecipientWar) -> Option<u32> {
        if self.epoch == 0 {
            return None;
        }
        let e = self.live_army(receipt.army)?;
        e.mission.context_country(receipt, purpose, war)
    }
    pub fn sees(&self, viewer: u32, province_owner: u32, native: bool) -> bool {
        native
            || (self.epoch != 0
                && self.entries.values().any(|e| {
                    e.mission
                        .sees_province(self.campaign, viewer, province_owner, false)
                }))
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, DecodeError> {
        let mut w = Writer(Vec::new());
        w.0.extend_from_slice(&self.campaign.0);
        w.u32(self.sequences.len() as u32);
        for (country, sequence) in &self.sequences {
            w.u32(*country);
            w.u64(*sequence);
        }
        w.u32(self.entries.len() as u32);
        for e in self.entries.values() {
            let payload = e.deployment.encode()?;
            w.u32(payload.len() as u32);
            w.0.extend_from_slice(&payload);
            let (phase, vision) = e.mission.saved();
            w.u32(phase_code(phase));
            w.u32(vision);
            w.u32(e.return_province.unwrap_or(0));
            // Runtime sampling watermark, not mission state. Keep the reserved
            // word for old saves but do not journal a date-only change daily.
            w.u32(0);
            w.u32(u32::from(e.outward_posted));
            w.u32(u32::from(e.assembly_verified));
            w.u32(e.mission.remaining().len() as u32);
            for id in e.mission.remaining() {
                w.identity(*id);
            }
        }
        Ok(w.0)
    }
    pub(crate) fn decode(
        bytes: &[u8],
        campaign: Campaign,
        epoch: u64,
    ) -> Result<Self, DecodeError> {
        let mut r = Reader { bytes, cursor: 0 };
        if r.take(16)? != campaign.0 {
            return Err(DecodeError::Invalid);
        }
        let mut result = Self::new(campaign, epoch);
        let n = r.u32()? as usize;
        if n > 4096 {
            return Err(DecodeError::Length);
        }
        for _ in 0..n {
            let country = r.u32()?;
            let seq = r.u64()?;
            if country == 0 || seq == 0 || result.sequences.insert(country, seq).is_some() {
                return Err(DecodeError::Invalid);
            }
        }
        let n = r.u32()? as usize;
        if n > MAX_MISSIONS {
            return Err(DecodeError::Length);
        }
        for _ in 0..n {
            let len = r.u32()? as usize;
            if len > crate::protocol::MAX_BYTES {
                return Err(DecodeError::Length);
            }
            let event = Envelope::decode(r.take(len)?)?;
            let Action::Deploy {
                donor_tag,
                source,
                destination,
                origin,
                target,
                ref regiments,
            } = event.action
            else {
                return Err(DecodeError::Invalid);
            };
            if event.key.campaign != campaign || result.entries.contains_key(&event.key.serial) {
                return Err(DecodeError::Invalid);
            }
            let phase = phase_from_code(r.u32()?).ok_or(DecodeError::Invalid)?;
            let vision = r.u32()?;
            let return_province = r.u32()?;
            let last_date = r.u32()?;
            let outward_posted = match r.u32()? {
                0 => false,
                1 => true,
                _ => return Err(DecodeError::Invalid),
            };
            let assembly_verified = match r.u32()? {
                0 => false,
                1 => true,
                _ => return Err(DecodeError::Invalid),
            };
            let count = r.u32()? as usize;
            if count > 64 {
                return Err(DecodeError::Length);
            }
            let mut remaining = Vec::with_capacity(count);
            for _ in 0..count {
                remaining.push(r.identity()?);
            }
            let request = SendRequest {
                donor: event.key.donor,
                recipient: event.key.recipient,
                donor_at_war: false,
                recipient_at_war: true,
                recipient_enemy_of_donor: false,
                source_owner: event.key.donor,
                source,
                regiments: regiments.clone(),
                source_in_combat: false,
                source_moving: false,
                source_is_land: true,
                already_deployed_by_donor: 0,
                multiplayer: false,
            };
            let settings = Settings {
                max_brigades: 64,
                sample_every_days: 1,
                ui_preview: false,
                prototype: false,
            };
            let plan = DeploymentPlan::new(
                event.key,
                &request,
                &settings,
                donor_tag,
                destination,
                origin,
            )
            .map_err(|_| DecodeError::Invalid)?;
            let mission =
                Mission::restore(plan, phase, vision, remaining).ok_or(DecodeError::Invalid)?;
            // A checksum verifies bytes, not registry consistency. Do not let
            // a malformed checkpoint restore overlapping live mission scopes.
            if result.sequences.get(&event.key.donor).copied().unwrap_or(0) < event.sequence
                || result.entries.values().any(|e| {
                    (e.mission.plan().destination() == destination
                        && !matches!(e.mission.saved().0, State::Returned | State::Lost)
                        && !matches!(phase, State::Returned | State::Lost))
                        || (e.deployment.key.donor == event.key.donor
                            && e.deployment.sequence == event.sequence)
                        || (!matches!(e.mission.saved().0, State::Returned | State::Lost)
                            && !matches!(phase, State::Returned | State::Lost)
                            && e.mission
                                .remaining()
                                .iter()
                                .any(|r| mission.remaining().contains(r)))
                })
                || (assembly_verified
                    && matches!(
                        phase,
                        State::Proposed | State::DeploymentPending | State::AwaitingArrival
                    ))
            {
                return Err(DecodeError::Invalid);
            }
            result.entries.insert(
                event.key.serial,
                Entry {
                    deployment: event,
                    mission,
                    target,
                    return_province: (return_province != 0).then_some(return_province),
                    last_processed_date: (last_date != 0).then_some(last_date),
                    outward_posted,
                    assembly_verified,
                },
            );
        }
        if r.cursor != bytes.len() {
            return Err(DecodeError::Trailing);
        }
        Ok(result)
    }
}

fn phase_code(s: State) -> u32 {
    match s {
        State::Proposed => 0,
        State::DeploymentPending => 1,
        State::AwaitingArrival => 2,
        State::Serving => 3,
        State::ReturnPending => 4,
        State::ReturnReady => 5,
        State::AwaitingReturn => 6,
        State::Returned => 7,
        State::Lost => 8,
        State::NeedsReview => 9,
    }
}
fn phase_from_code(n: u32) -> Option<State> {
    Some(match n {
        0 => State::Proposed,
        1 => State::DeploymentPending,
        2 => State::AwaitingArrival,
        3 => State::Serving,
        4 => State::ReturnPending,
        5 => State::ReturnReady,
        6 => State::AwaitingReturn,
        7 => State::Returned,
        8 => State::Lost,
        9 => State::NeedsReview,
        _ => return None,
    })
}
