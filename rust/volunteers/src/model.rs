//! Volunteer policy and lifecycle. No native calls or simulation writes.
//! The runtime probe does not execute these plans. A later command adapter must
//! prove ownership, control, pop maintenance and persistence before enabling Send.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity {
    pub kind: u32,
    pub id: u32,
}

impl Identity {
    pub fn valid(self) -> bool {
        self.kind != 0 && self.id != 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub max_brigades: usize,
    pub sample_every_days: u32,
    pub ui_preview: bool,
    pub prototype: bool,
    pub mp_probe: bool,
    pub mp_play: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_brigades: 5,
            sample_every_days: 7,
            ui_preview: false,
            prototype: false,
            mp_probe: false,
            mp_play: false,
        }
    }
}

impl Settings {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut result = Self::default();
        let mut keys = std::collections::BTreeSet::new();
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once('=').ok_or("expected key=value")?;
            let (key, value) = (key.trim(), value.trim());
            if !keys.insert(key) {
                return Err(format!("duplicate key: {key}"));
            }
            match key {
                "mode" if value == "observe" => result.ui_preview = false,
                "mode" if value == "preview" => result.ui_preview = true,
                "mode" if value == "prototype" => {
                    result.ui_preview = true;
                    result.prototype = true;
                }
                "mode" if value == "mp_probe" => {
                    result.ui_preview = true;
                    result.mp_probe = true;
                }
                "mode" if value == "mp" => {
                    result.ui_preview = true;
                    result.prototype = true;
                    result.mp_play = true;
                }
                "mode" => {
                    return Err(
                        "mode must be observe, preview, prototype, mp_probe or mp; native admission gates always apply"
                            .into(),
                    );
                }
                "max_brigades" => {
                    result.max_brigades = value.parse().map_err(|_| "invalid max_brigades")?;
                    if !(1..=64).contains(&result.max_brigades) {
                        return Err("max_brigades must be 1..64".into());
                    }
                }
                "sample_every_days" => {
                    result.sample_every_days =
                        value.parse().map_err(|_| "invalid sample_every_days")?;
                    if !(1..=30).contains(&result.sample_every_days) {
                        return Err("sample_every_days must be 1..30".into());
                    }
                }
                _ => return Err(format!("unknown setting: {key}")),
            }
        }
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct SendRequest {
    pub donor: u32,
    pub recipient: u32,
    pub donor_at_war: bool,
    pub recipient_at_war: bool,
    pub recipient_enemy_of_donor: bool,
    pub source_owner: u32,
    pub source: Identity,
    pub regiments: Vec<Identity>,
    pub source_in_combat: bool,
    pub source_moving: bool,
    pub source_is_land: bool,
    pub already_deployed_by_donor: usize,
    pub multiplayer: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    MultiplayerUnverified,
    InvalidCountry,
    SameCountry,
    DonorAtWar,
    RecipientAtPeace,
    HostileRecipient,
    ForeignSource,
    InvalidIdentity,
    EmptySelection,
    DuplicateRegiment,
    LimitExceeded,
    BusySource,
    NotLand,
}

/// Proposed SP-first policy: neutral donor, recipient at war, whole dedicated
/// volunteer stack. This validates a proposal; it never declares a deployment.
pub fn validate_send(r: &SendRequest, settings: &Settings) -> Result<(), Refusal> {
    if r.multiplayer {
        return Err(Refusal::MultiplayerUnverified);
    }
    validate_policy(r, settings)
}
pub(crate) fn validate_authorized(
    r: &SendRequest,
    settings: &Settings,
    permit: crate::session::SimulationPermit,
) -> Result<(), Refusal> {
    if r.multiplayer && (!permit.multiplayer() || permit.policy() || !permit.commands_for(r.donor))
    {
        return Err(Refusal::MultiplayerUnverified);
    }
    validate_policy(r, settings)
}
fn validate_policy(r: &SendRequest, settings: &Settings) -> Result<(), Refusal> {
    if r.donor == 0 || r.recipient == 0 {
        return Err(Refusal::InvalidCountry);
    }
    if r.donor == r.recipient {
        return Err(Refusal::SameCountry);
    }
    if r.donor_at_war {
        return Err(Refusal::DonorAtWar);
    }
    if !r.recipient_at_war {
        return Err(Refusal::RecipientAtPeace);
    }
    if r.recipient_enemy_of_donor {
        return Err(Refusal::HostileRecipient);
    }
    if r.source_owner != r.donor {
        return Err(Refusal::ForeignSource);
    }
    if !r.source.valid() || r.regiments.iter().any(|id| !id.valid()) {
        return Err(Refusal::InvalidIdentity);
    }
    if !r.source_is_land {
        return Err(Refusal::NotLand);
    }
    if r.source_in_combat || r.source_moving {
        return Err(Refusal::BusySource);
    }
    if r.regiments.is_empty() {
        return Err(Refusal::EmptySelection);
    }
    let unique: std::collections::BTreeSet<_> = r.regiments.iter().collect();
    if unique.len() != r.regiments.len() {
        return Err(Refusal::DuplicateRegiment);
    }
    if r.already_deployed_by_donor
        .checked_add(r.regiments.len())
        .is_none_or(|n| n > settings.max_brigades)
    {
        return Err(Refusal::LimitExceeded);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Proposed,
    DeploymentPending,
    AwaitingArrival,
    Serving,
    ReturnPending,
    ReturnReady,
    AwaitingReturn,
    Returned,
    Lost,
    NeedsReview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    None,
    QueueDeployment,
    QueueReturn,
    Review,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Approved,
    DeploymentPosted,
    ArrivalVerified,
    RecipientPeace,
    Recall,
    DonorWar,
    SourceBusy,
    SourceReady,
    ReturnPosted,
    ReturnVerified,
    ObjectsDestroyed,
    LoadOrCampaignChange,
    CommandRefused,
}

pub fn transition(state: State, event: Event) -> (State, Intent) {
    use Event::*;
    use Intent::*;
    use State::*;
    if event == LoadOrCampaignChange {
        return (NeedsReview, Review);
    }
    if matches!(state, Returned | Lost | NeedsReview) {
        return (state, None);
    }
    if event == ObjectsDestroyed {
        return (Lost, None);
    }
    if event == CommandRefused {
        return (NeedsReview, Review);
    }
    match (state, event) {
        (Proposed, Approved) => (DeploymentPending, QueueDeployment),
        (DeploymentPending, DeploymentPosted) => (AwaitingArrival, None),
        (AwaitingArrival, ArrivalVerified) => (Serving, None),
        (Serving | AwaitingArrival, RecipientPeace | Recall | DonorWar) => (ReturnPending, None),
        (ReturnPending, SourceReady) => (ReturnReady, QueueReturn),
        (ReturnReady, ReturnPosted) => (AwaitingReturn, None),
        (AwaitingReturn, ReturnVerified) => (Returned, None),
        _ => (state, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> SendRequest {
        SendRequest {
            donor: 1,
            recipient: 2,
            donor_at_war: false,
            recipient_at_war: true,
            recipient_enemy_of_donor: false,
            source_owner: 1,
            source: Identity { kind: 40, id: 2 },
            regiments: vec![Identity { kind: 40, id: 3 }],
            source_in_combat: false,
            source_moving: false,
            source_is_land: true,
            already_deployed_by_donor: 0,
            multiplayer: false,
        }
    }
    #[test]
    fn settings_cannot_enable_writes() {
        let probe = Settings::parse("mode=mp_probe").unwrap();
        assert!(probe.mp_probe && probe.ui_preview && !probe.prototype);
        for text in [
            "mode=send",
            "mode=observe\nmode=observe",
            "unknown=1",
            "max_brigades=0",
            "sample_every_days=0",
        ] {
            assert!(Settings::parse(text).is_err(), "{text}");
        }
        assert_eq!(
            Settings::parse("# comment\r\nmode=observe\nmax_brigades=3")
                .unwrap()
                .max_brigades,
            3
        );
    }
    #[test]
    fn neutral_donor_can_propose_but_not_if_recipient_peace() {
        let mut r = request();
        assert_eq!(validate_send(&r, &Settings::default()), Ok(()));
        r.recipient_at_war = false;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::RecipientAtPeace)
        );
    }
    #[test]
    fn donor_total_limit_counts_other_deployments_and_overflow() {
        let mut r = request();
        r.already_deployed_by_donor = 5;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::LimitExceeded)
        );
        r.already_deployed_by_donor = usize::MAX;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::LimitExceeded)
        );
    }
    #[test]
    fn duplicate_foreign_busy_and_multiplayer_refused() {
        let mut r = request();
        r.regiments.push(r.regiments[0]);
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::DuplicateRegiment)
        );
        r = request();
        r.source_owner = 2;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::ForeignSource)
        );
        r = request();
        r.source_in_combat = true;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::BusySource)
        );
        r = request();
        r.multiplayer = true;
        assert_eq!(
            validate_send(&r, &Settings::default()),
            Err(Refusal::MultiplayerUnverified)
        );
    }
    #[test]
    fn a_posted_command_does_not_prove_arrival_or_return() {
        let (s, _) = transition(State::DeploymentPending, Event::DeploymentPosted);
        assert_eq!(s, State::AwaitingArrival);
        assert_eq!(transition(s, Event::ArrivalVerified).0, State::Serving);
        let s = transition(State::ReturnReady, Event::ReturnPosted).0;
        assert_eq!(s, State::AwaitingReturn);
        assert_eq!(transition(s, Event::ReturnVerified).0, State::Returned);
    }
    #[test]
    fn peace_during_arrival_still_requests_return() {
        let s = transition(State::AwaitingArrival, Event::RecipientPeace).0;
        assert_eq!(s, State::ReturnPending);
        assert_eq!(transition(s, Event::SourceBusy), (s, Intent::None));
        assert_eq!(
            transition(s, Event::SourceReady),
            (State::ReturnReady, Intent::QueueReturn)
        );
    }
    #[test]
    fn destruction_and_load_do_not_mark_returned() {
        assert_eq!(
            transition(State::Serving, Event::ObjectsDestroyed).0,
            State::Lost
        );
        let s = transition(State::Serving, Event::LoadOrCampaignChange).0;
        assert_eq!(s, State::NeedsReview);
        assert_eq!(transition(s, Event::SourceReady), (s, Intent::None));
        assert_eq!(
            transition(State::Serving, Event::CommandRefused).0,
            State::NeedsReview
        );
    }
    #[test]
    fn repeated_ui_or_tick_events_do_not_duplicate_command_intents() {
        let (s, intent) = transition(State::Proposed, Event::Approved);
        assert_eq!(intent, Intent::QueueDeployment);
        assert_eq!(transition(s, Event::Approved), (s, Intent::None));
        let (s, intent) = transition(State::ReturnPending, Event::SourceReady);
        assert_eq!(intent, Intent::QueueReturn);
        assert_eq!(transition(s, Event::SourceReady), (s, Intent::None));
    }
    #[test]
    fn recall_and_donor_entry_into_war_recall_only_the_loan() {
        for e in [Event::Recall, Event::DonorWar] {
            assert_eq!(
                transition(State::Serving, e),
                (State::ReturnPending, Intent::None)
            );
        }
        assert_eq!(
            transition(State::Returned, Event::Recall),
            (State::Returned, Intent::None)
        );
    }
}
