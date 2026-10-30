//! Volunteer chooser, mission list and command outbox. Native GUI callbacks
//! translate into these actions; simulation changes leave only as engine work.
use crate::mission::{Campaign, DeploymentPlan, MissionKey, RecipientWar, StackReceipt};
use crate::model::{Identity, SendRequest, Settings, State};
use crate::protocol::{Action, Envelope};
use crate::registry::{Effect, Registry};
use crate::session::{self, Capabilities, SessionMode, Unavailable};
use std::collections::{BTreeMap, BTreeSet};

pub const ROWS: usize = 2;
#[derive(Clone, Debug)]
pub struct RegimentView {
    pub id: Identity,
    pub owner: u32,
    pub name: String,
}
#[derive(Clone, Debug)]
pub struct ArmyView {
    pub id: Identity,
    pub owner: u32,
    pub affiliation: u32,
    pub province: u32,
    pub name: String,
    pub combat: bool,
    pub moving: bool,
    pub regiments: Vec<RegimentView>,
}
#[derive(Clone, Debug)]
pub struct CountryView {
    pub index: u32,
    pub tag: u32,
    pub caption: String,
    pub war: RecipientWar,
    pub hostile_to_player: bool,
    pub landing_province: Option<u32>,
}
#[derive(Clone, Debug)]
pub struct WorldView {
    pub epoch: u64,
    pub campaign: Option<Campaign>,
    pub mode: SessionMode,
    pub mp_authority: Option<crate::mp_authority::IssuerAuthority>,
    pub player: u32,
    pub date: u32,
    pub player_at_war: bool,
    pub countries: Vec<CountryView>,
    pub armies: Vec<ArmyView>,
}
impl Default for WorldView {
    fn default() -> Self {
        Self {
            epoch: 0,
            campaign: None,
            mode: SessionMode::Unknown,
            mp_authority: None,
            player: 0,
            date: 0,
            player_at_war: false,
            countries: Vec::new(),
            armies: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UiAction {
    Open(u32),
    Close,
    PreviousArmy,
    NextArmy,
    PreviousRecipient,
    NextRecipient,
    PreviousPage,
    NextPage,
    ToggleRow(usize),
    Send,
    PreviousMission,
    NextMission,
    Recall,
    ToggleHistory,
    ClearHistory,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Work {
    Detach {
        serial: u64,
        plan: DeploymentPlan,
    },
    Move {
        serial: u64,
        donor: u32,
        army: Identity,
        destination: u32,
        returning: bool,
    },
}
#[derive(Clone, Debug, Default)]
pub struct Row {
    pub text: String,
    pub checked: bool,
    pub enabled: bool,
}
#[derive(Clone, Debug, Default)]
pub struct Presentation {
    pub open: bool,
    pub donor: String,
    pub recipient: String,
    pub army: String,
    pub rows: Vec<Row>,
    pub selected: String,
    pub status: String,
    pub send_enabled: bool,
    pub missions: Vec<String>,
    pub recall_enabled: bool,
    pub mission_page: String,
    pub history_label: String,
}

pub struct App {
    pub settings: Settings,
    pub capabilities: Capabilities,
    pub registry: Option<Registry>,
    pub open: bool,
    recipient: u32,
    army: Option<Identity>,
    chosen: BTreeSet<Identity>,
    page: usize,
    mission_index: usize,
    status: String,
    pub last_unavailable: Option<Unavailable>,
    last_day: Option<u32>,
    unreadable: BTreeMap<u64, u8>,
    show_history: bool,
}
impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            capabilities: Capabilities::default(),
            registry: None,
            open: false,
            recipient: 0,
            army: None,
            chosen: BTreeSet::new(),
            page: 0,
            mission_index: 0,
            status: String::new(),
            last_unavailable: None,
            last_day: None,
            unreadable: BTreeMap::new(),
            show_history: false,
        }
    }
    pub fn bind_registry(&mut self, registry: Registry) {
        self.registry = Some(registry);
        self.page = 0;
        self.mission_index = 0;
        self.chosen.clear();
        self.status.clear();
        self.last_day = None;
        self.unreadable.clear();
    }
    fn army<'a>(&self, w: &'a WorldView) -> Option<&'a ArmyView> {
        w.armies
            .iter()
            .find(|a| Some(a.id) == self.army && a.owner == w.player)
    }
    fn recipient<'a>(&self, w: &'a WorldView) -> Option<&'a CountryView> {
        w.countries.iter().find(|c| c.index == self.recipient)
    }
    pub fn selected_recipient(&self) -> u32 {
        self.recipient
    }
    pub fn reconcile_armies(&mut self, w: &WorldView) {
        if self.army(w).is_none() {
            self.army = w.armies.iter().find(|a| a.owner == w.player).map(|a| a.id);
            self.chosen.clear();
            self.page = 0;
        }
    }
    fn mission_rows(&self) -> impl Iterator<Item = &crate::registry::Entry> {
        self.registry.iter().flat_map(|r| r.entries()).filter(|e| {
            self.show_history || !matches!(e.mission.saved().0, State::Returned | State::Lost)
        })
    }
    pub(crate) fn gate(&self, w: &WorldView) -> Result<session::SimulationPermit, Unavailable> {
        if w.mode == SessionMode::Multiplayer && !self.settings.mp_play {
            return Err(Unavailable::MultiplayerUnsupported);
        }
        if w.mode == SessionMode::Multiplayer && self.settings.mp_play && w.mp_authority.is_none() {
            return Err(Unavailable::MultiplayerHandshake);
        }
        let permit = if w.mode == SessionMode::Multiplayer
            && let Some(proof) = w.mp_authority
        {
            session::admit_issuer(proof, self.capabilities, w.epoch, w.player)?
        } else {
            session::admit(w.mode, self.capabilities, w.epoch)?
        };
        if w.campaign.is_none()
            || self
                .registry
                .as_ref()
                .is_none_or(|r| Some(r.campaign) != w.campaign || r.epoch != w.epoch)
        {
            return Err(Unavailable::CampaignUnbound);
        }
        Ok(permit)
    }
    pub fn can_submit(&self, w: &WorldView) -> bool {
        self.gate(w)
            .is_ok_and(|permit| !permit.multiplayer() || self.whole_army_selection(w))
            && self.request(w).is_some_and(|request| {
                self.gate(w).is_ok_and(|permit| {
                    crate::model::validate_authorized(&request, &self.settings, permit).is_ok()
                })
            })
            && self.army(w).is_some_and(|a| a.affiliation == w.player)
            && self
                .recipient(w)
                .is_some_and(|c| c.landing_province.is_some())
    }
    pub(crate) fn whole_army_selection(&self, w: &WorldView) -> bool {
        self.request(w).zip(self.army(w)).is_some_and(|(r, a)| {
            r.regiments.len() == a.regiments.len()
                && a.regiments
                    .iter()
                    .all(|member| r.regiments.contains(&member.id))
        })
    }
    pub(crate) fn request(&self, w: &WorldView) -> Option<SendRequest> {
        let army = self.army(w)?;
        let recipient = self.recipient(w)?;
        let regiments = army
            .regiments
            .iter()
            .filter(|r| self.chosen.contains(&r.id) && r.owner == w.player)
            .map(|r| r.id)
            .collect::<Vec<_>>();
        if regiments.len() != self.chosen.len() {
            return None;
        }
        Some(SendRequest {
            donor: w.player,
            recipient: self.recipient,
            donor_at_war: w.player_at_war,
            recipient_at_war: recipient.war == RecipientWar::AtWar,
            recipient_enemy_of_donor: recipient.hostile_to_player,
            source_owner: army.owner,
            source: army.id,
            regiments,
            source_in_combat: army.combat,
            source_moving: army.moving,
            source_is_land: true,
            already_deployed_by_donor: self.registry.as_ref().map_or(0, |r| r.deployed(w.player)),
            multiplayer: w.mode != SessionMode::SinglePlayer,
        })
    }
    pub(crate) fn refuse(&mut self, message: &str) {
        self.status = message.into();
    }
    /// `new_identity` is supplied only after native identity allocation, never
    /// from a UI text field or sidecar. No allocation is needed to preview.
    pub fn action(
        &mut self,
        action: UiAction,
        w: &WorldView,
        new_identity: Option<Identity>,
    ) -> Vec<Work> {
        let mut work = Vec::new();
        self.status.clear();
        match action {
            UiAction::Open(recipient) => {
                self.open = true;
                self.recipient = recipient;
                if self.army(w).is_none() {
                    self.army = w.armies.iter().find(|a| a.owner == w.player).map(|a| a.id);
                    self.chosen.clear();
                    self.page = 0;
                }
            }
            UiAction::Close => self.open = false,
            UiAction::PreviousArmy | UiAction::NextArmy => {
                let armies = w
                    .armies
                    .iter()
                    .filter(|a| a.owner == w.player)
                    .collect::<Vec<_>>();
                if !armies.is_empty() {
                    let current = armies
                        .iter()
                        .position(|a| Some(a.id) == self.army)
                        .unwrap_or(0);
                    let step = if matches!(action, UiAction::NextArmy) {
                        1
                    } else {
                        armies.len() - 1
                    };
                    self.army = Some(armies[(current + step) % armies.len()].id);
                    self.chosen.clear();
                    self.page = 0;
                }
            }
            UiAction::PreviousRecipient | UiAction::NextRecipient => {
                let countries = w
                    .countries
                    .iter()
                    .filter(|c| c.index != w.player && c.war == RecipientWar::AtWar)
                    .collect::<Vec<_>>();
                if !countries.is_empty() {
                    let current = countries
                        .iter()
                        .position(|c| c.index == self.recipient)
                        .unwrap_or(0);
                    let step = if matches!(action, UiAction::NextRecipient) {
                        1
                    } else {
                        countries.len() - 1
                    };
                    self.recipient = countries[(current + step) % countries.len()].index;
                }
            }
            UiAction::PreviousPage => self.page = self.page.saturating_sub(1),
            UiAction::NextPage => {
                let max = self
                    .army(w)
                    .map_or(0, |a| a.regiments.len().saturating_sub(1) / ROWS);
                self.page = (self.page + 1).min(max);
            }
            UiAction::ToggleRow(row) => {
                if row < ROWS
                    && let Some(regiment) = self
                        .army(w)
                        .and_then(|a| a.regiments.get(self.page * ROWS + row))
                {
                    let id = regiment.id;
                    if !self.chosen.remove(&id) && self.chosen.len() < self.settings.max_brigades {
                        self.chosen.insert(id);
                    }
                }
            }
            UiAction::PreviousMission => self.mission_index = self.mission_index.saturating_sub(1),
            UiAction::NextMission => {
                let count = self.mission_rows().count();
                self.mission_index = (self.mission_index + 1).min(count.saturating_sub(1));
            }
            UiAction::ToggleHistory => {
                self.show_history = !self.show_history;
                self.mission_index = 0;
            }
            UiAction::ClearHistory => {
                if self.gate(w).is_ok() {
                    let count = self.registry.as_mut().map_or(0, Registry::clear_completed);
                    self.mission_index = 0;
                    self.status =
                        format!("Cleared {count} completed records; wartime vision is retained.");
                }
            }
            UiAction::Send => {
                let permit = match self.gate(w) {
                    Ok(p) => p,
                    Err(e) => {
                        self.last_unavailable = Some(e);
                        self.status = e.message().into();
                        return work;
                    }
                };
                let Some(request) = self.request(w) else {
                    self.status = "Select an army and brigades first.".into();
                    return work;
                };
                if permit.multiplayer() && !self.whole_army_selection(w) {
                    self.status="MP currently sends whole armies. Split the army with native controls first.".into();
                    return work;
                }
                if let Err(reason) =
                    crate::model::validate_authorized(&request, &self.settings, permit)
                {
                    use crate::model::Refusal::*;
                    self.status = match reason {
                        DonorAtWar => "Your country must be at peace.",
                        RecipientAtPeace => "The recipient must be at war.",
                        HostileRecipient => "You cannot support an enemy.",
                        BusySource => "Choose an army that is idle and out of combat.",
                        LimitExceeded => "The selection exceeds your volunteer limit.",
                        EmptySelection => "Select at least one brigade.",
                        ForeignSource => "Choose an army owned by your country.",
                        _ => "This selection cannot be sent; choose your brigades again.",
                    }
                    .into();
                    return work;
                }
                let Some(target) = self.recipient(w).and_then(|c| c.landing_province) else {
                    self.status = "No deployment destination is available.".into();
                    return work;
                };
                let Some(donor) = w.countries.iter().find(|c| c.index == w.player) else {
                    return work;
                };
                let Some(army) = self.army(w) else {
                    return work;
                };
                let origin = army.province;
                // FUN_00513F30 can reject the last transfer from a land stack.
                // Preserve a whole selected army, including its leader.
                let destination = if self.whole_army_selection(w) {
                    Some(request.source)
                } else {
                    new_identity
                };
                let Some(destination) = destination else {
                    self.status = "Could not prepare the volunteer army.".into();
                    return work;
                };
                let Some(registry) = self.registry.as_mut() else {
                    return work;
                };
                let (Some(serial), Some(sequence)) =
                    (registry.next_serial(), registry.next_sequence(w.player))
                else {
                    return work;
                };
                let event = Envelope {
                    key: MissionKey {
                        campaign: registry.campaign,
                        serial,
                        donor: w.player,
                        recipient: self.recipient,
                    },
                    sequence,
                    issued_on: w.date,
                    action: Action::Deploy {
                        donor_tag: donor.tag,
                        source: request.source,
                        destination,
                        origin,
                        target,
                        regiments: request.regiments.clone(),
                    },
                };
                match registry.accept(permit, w.player, event, Some(&request), &self.settings) {
                    Ok(Effect::Detach { key, plan, .. }) => {
                        work.push(Work::Detach {
                            serial: key.serial,
                            plan,
                        });
                        self.status = "Preparing volunteer brigades.".into();
                        self.chosen.clear();
                        self.page = 0;
                    }
                    Err(crate::registry::Error::Full) => {
                        self.status =
                            "Volunteer journal is full; clear completed history after peace.".into()
                    }
                    _ => self.status = "The volunteer request could not be accepted.".into(),
                }
            }
            UiAction::Recall => {
                let permit = match self.gate(w) {
                    Ok(p) => p,
                    Err(e) => {
                        self.last_unavailable = Some(e);
                        self.status = e.message().into();
                        return work;
                    }
                };
                let Some(destination) = w
                    .countries
                    .iter()
                    .find(|c| c.index == w.player)
                    .and_then(|c| c.landing_province)
                else {
                    return work;
                };
                let selected_key = self
                    .mission_rows()
                    .nth(self.mission_index)
                    .map(|e| e.deployment.key);
                let Some(registry) = self.registry.as_mut() else {
                    return work;
                };
                let Some(key) = selected_key else {
                    return work;
                };
                let Some(sequence) = registry.next_sequence(w.player) else {
                    return work;
                };
                let event = Envelope {
                    key,
                    sequence,
                    issued_on: w.date,
                    action: Action::Recall {
                        return_province: destination,
                    },
                };
                if let Ok(Effect::Return { .. }) =
                    registry.accept(permit, w.player, event, None, &self.settings)
                {
                    // The daily adapter waits for combat/movement to finish.
                    // A recall click must not replace an in-flight native order.
                    self.status = "Return requested.".into();
                }
            }
        }
        work
    }
    pub fn presentation(&self, w: &WorldView) -> Presentation {
        let army = self.army(w);
        let recipient = self.recipient(w);
        let rows = army
            .map(|a| {
                a.regiments
                    .iter()
                    .skip(self.page * ROWS)
                    .take(ROWS)
                    .map(|r| Row {
                        text: r.name.clone(),
                        checked: self.chosen.contains(&r.id),
                        enabled: r.owner == w.player,
                    })
                    .collect()
            })
            .unwrap_or_default();
        let ready = self.gate(w).is_ok();
        let missions = self
            .mission_rows()
            .skip(self.mission_index)
            .take(2)
            .map(|e| {
                let assembly = w
                    .armies
                    .iter()
                    .find(|a| a.id == e.mission.plan().destination());
                let phase = if e.mission.state() == State::NeedsReview && !e.assembly_verified {
                    assembly.map(|a| {
                        let present = a
                            .regiments
                            .iter()
                            .filter(|member| e.mission.remaining().contains(&member.id))
                            .count();
                        format!(
                            "assembly incomplete: {present}/{}",
                            e.mission.remaining().len()
                        )
                    })
                } else {
                    None
                };
                format!(
                    "{} -> {} | {} brigades | {}",
                    caption(w, e.deployment.key.donor),
                    caption(w, e.deployment.key.recipient),
                    e.mission.remaining().len(),
                    phase.unwrap_or_else(|| self::phase(e.mission.state()).into())
                )
            })
            .collect();
        Presentation {
            open: self.open,
            donor: caption(w, w.player),
            recipient: recipient
                .map(|c| {
                    format!(
                        "{} - {}",
                        c.caption,
                        match c.war {
                            RecipientWar::AtWar => "at war",
                            RecipientWar::AtPeace => "at peace",
                            RecipientWar::Unknown => "war status unknown",
                        }
                    )
                })
                .unwrap_or_else(|| "Choose recipient".into()),
            army: army
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "No army selected".into()),
            rows,
            selected: format!(
                "{} brigades selected - limit {}",
                self.chosen.len(),
                self.settings.max_brigades
            ),
            status: if !self.status.is_empty() {
                self.status.clone()
            } else if !ready {
                self.gate(w)
                    .err()
                    .map_or("Volunteers unavailable.", Unavailable::message)
                    .into()
            } else {
                String::new()
            },
            send_enabled: self.can_submit(w),
            missions,
            recall_enabled: ready
                && self
                    .mission_rows()
                    .nth(self.mission_index)
                    .is_some_and(|e| {
                        e.deployment.key.donor == w.player
                            && matches!(
                                e.mission.state(),
                                State::Serving | State::AwaitingArrival | State::ReturnPending
                            )
                    }),
            mission_page: format!("{}", self.mission_index + 1),
            history_label: if self.show_history {
                "Hide history"
            } else {
                "Show history"
            }
            .into(),
        }
    }
    pub fn update_receipt(&mut self, serial: u64, receipt: &StackReceipt) -> bool {
        self.registry
            .as_mut()
            .is_some_and(|r| r.receipt(serial, receipt))
    }
    /// One simulation-day pass; queue work is returned to the native executor.
    /// Lost/returned records still receive recipient-war updates for vision.
    pub fn day(
        &mut self,
        w: &WorldView,
        mut observe: impl FnMut(Identity) -> Option<(StackReceipt, u32, u32, bool)>,
    ) -> Vec<Work> {
        if self.gate(w).is_err() {
            return Vec::new();
        }
        if self.last_day == Some(w.date) {
            return Vec::new();
        }
        if self.last_day.is_some_and(|date| w.date < date) {
            // P64/P65: a transient date sample is not a load signal. Native
            // lifecycle/epoch changes suspend records; this sample only waits.
            self.status = "Waiting for a consistent world sample.".into();
            return Vec::new();
        }
        self.last_day = Some(w.date);
        let Some(registry) = self.registry.as_mut() else {
            return Vec::new();
        };
        registry.observe_day(w.date, |country| {
            w.countries
                .iter()
                .find(|c| c.index == country)
                .map_or(RecipientWar::Unknown, |c| c.war)
        });
        registry.clear_completed();
        let entries = registry.entries().cloned().collect::<Vec<_>>();
        let mut work = Vec::new();
        for old in entries {
            let key = old.deployment.key;
            if matches!(
                old.mission.state(),
                State::Returned | State::Lost | State::NeedsReview
            ) {
                continue;
            }
            let Some((receipt, province, province_owner, busy)) =
                observe(old.mission.plan().destination())
            else {
                // A failed read is neither proof of destruction nor a load.
                // Defer transient misses; repeated misses need explicit review.
                let misses = self.unreadable.entry(key.serial).or_default();
                *misses = misses.saturating_add(1);
                if *misses >= 3 {
                    registry.command_refused(key.serial);
                    self.status = "Volunteer stack needs verification.".into();
                }
                continue;
            };
            self.unreadable.remove(&key.serial);
            if !registry.receipt(key.serial, &receipt) {
                registry.command_refused(key.serial);
                continue;
            }
            if old.mission.state() == State::AwaitingReturn {
                if !busy {
                    registry.return_receipt(key.serial, &receipt, province_owner);
                }
                continue;
            }
            let recipient_at_peace = w
                .countries
                .iter()
                .find(|c| c.index == key.recipient)
                .is_some_and(|c| c.war == RecipientWar::AtPeace);
            if (recipient_at_peace || (key.donor == w.player && w.player_at_war))
                && let Some(home) = w
                    .countries
                    .iter()
                    .find(|c| c.index == key.donor)
                    .and_then(|c| c.landing_province)
            {
                registry.request_return(key.serial, home);
            }
            let Some(current) = registry.entry(key.serial) else {
                continue;
            };
            if busy {
                continue;
            }
            if current.mission.state() == State::ReturnPending {
                if let Some(home) = current.return_province {
                    work.push(Work::Move {
                        serial: key.serial,
                        donor: key.donor,
                        army: receipt.army,
                        destination: home,
                        returning: true,
                    });
                }
            } else if current.mission.state() == State::Serving
                && !current.outward_posted
                && province != current.target
            {
                work.push(Work::Move {
                    serial: key.serial,
                    donor: key.donor,
                    army: receipt.army,
                    destination: current.target,
                    returning: false,
                });
            }
        }
        self.mission_index = self
            .mission_index
            .min(self.mission_rows().count().saturating_sub(1));
        work
    }
}
fn caption(w: &WorldView, country: u32) -> String {
    w.countries
        .iter()
        .find(|c| c.index == country)
        .map(|c| c.caption.clone())
        .unwrap_or_else(|| format!("Country {country}"))
}
fn phase(state: State) -> &'static str {
    match state {
        State::Proposed | State::DeploymentPending => "preparing",
        State::AwaitingArrival => "assembling",
        State::Serving => "serving",
        State::ReturnPending | State::ReturnReady => "return pending",
        State::AwaitingReturn => "returning",
        State::Returned => "returned",
        State::Lost => "lost",
        State::NeedsReview => "awaiting verification",
    }
}
