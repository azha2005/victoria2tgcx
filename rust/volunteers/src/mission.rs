//! Donor-owned deployment contracts and independent recipient-vision lifetime.
//! These are offline policies. No native memory, commands or hooks are invoked.
use crate::model::{self, Event, Identity, Intent, Refusal, SendRequest, Settings, State};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Campaign(pub [u8; 16]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MissionKey {
    pub campaign: Campaign,
    pub serial: u64,
    pub donor: u32,
    pub recipient: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentPlan {
    key: MissionKey,
    owner_tag: u32,
    source: Identity,
    destination: Identity,
    origin_province: u32,
    regiments: Vec<Identity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanError {
    Request(Refusal),
    InvalidMission,
    InvalidDestination,
    InvalidOrigin,
}

impl DeploymentPlan {
    pub fn key(&self) -> MissionKey {
        self.key
    }
    pub fn destination(&self) -> Identity {
        self.destination
    }
    pub fn origin_province(&self) -> u32 {
        self.origin_province
    }
    pub fn donor_tag(&self) -> u32 {
        self.owner_tag
    }
    /// A native Create+Transfer may detach selected brigades, but the new stack
    /// MUST remain donor-owned. Destination location initially equals the source
    /// province; deployment travel still needs its own native movement adapter.
    pub fn new(
        key: MissionKey,
        request: &SendRequest,
        settings: &Settings,
        donor_tag: u32,
        destination: Identity,
        origin_province: u32,
    ) -> Result<Self, PlanError> {
        model::validate_send(request, settings).map_err(PlanError::Request)?;
        if key.campaign.0 == [0; 16]
            || key.serial == 0
            || key.donor != request.donor
            || key.recipient != request.recipient
            || donor_tag == 0
        {
            return Err(PlanError::InvalidMission);
        }
        // Whole-stack reuse requires exact membership proof in the adapter.
        if !destination.valid() || request.regiments.contains(&destination) {
            return Err(PlanError::InvalidDestination);
        }
        if origin_province == 0 {
            return Err(PlanError::InvalidOrigin);
        }
        Ok(Self {
            key,
            owner_tag: donor_tag,
            source: request.source,
            destination,
            origin_province,
            regiments: request.regiments.clone(),
        })
    }

    /// Exact stack argument order of RVA 0x17DB30 (RET 0x28), reusing ATCU's
    /// native Create contract. This only describes arguments; it calls nothing.
    pub fn create_arguments(&self, allocation: u32) -> [u32; 10] {
        [
            allocation,
            self.owner_tag,
            self.key.donor,
            self.destination.kind,
            self.destination.id,
            self.destination.kind,
            self.destination.id,
            self.origin_province,
            0, // land stack
            0, // explicit selected transfers, no native automatic split
        ]
    }

    pub fn source(&self) -> Identity {
        self.source
    }

    pub fn selected_regiments(&self) -> &[Identity] {
        &self.regiments
    }
}

#[derive(Clone, Debug)]
pub struct StackReceipt {
    pub campaign: Campaign,
    pub army: Identity,
    pub owner: u32,
    pub affiliation: u32,
    pub regiments: Vec<(Identity, u32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiptError {
    WrongPhase,
    WrongCampaign,
    WrongArmy,
    OwnershipChanged,
    MembershipChanged,
    NotInDonorTerritory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecipientWar {
    AtWar,
    AtPeace,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Use {
    Orders,
    Ownership,
    Goods,
    Reinforcement,
    Flag,
    MilitaryStatistics,
    Combat,
    Access,
}

#[derive(Clone, Debug)]
pub struct Mission {
    plan: DeploymentPlan,
    state: State,
    vision: Vision,
    remaining: Vec<Identity>,
    resume_state: Option<State>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Vision {
    NotGranted,
    Active,
    Suspended,
    Expired,
}

impl Mission {
    pub fn new(plan: DeploymentPlan) -> Self {
        Self {
            remaining: plan.regiments.clone(),
            plan,
            state: State::Proposed,
            vision: Vision::NotGranted,
            resume_state: None,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }
    pub fn plan(&self) -> &DeploymentPlan {
        &self.plan
    }
    pub fn remaining(&self) -> &[Identity] {
        &self.remaining
    }
    pub(crate) fn saved(&self) -> (State, u32) {
        let vision = match self.vision {
            Vision::NotGranted => 0,
            Vision::Active => 1,
            Vision::Suspended => 2,
            Vision::Expired => 3,
        };
        (self.resume_state.unwrap_or(self.state), vision)
    }
    pub(crate) fn restore(
        plan: DeploymentPlan,
        phase: State,
        vision: u32,
        remaining: Vec<Identity>,
    ) -> Option<Self> {
        if remaining.iter().any(|id| !plan.regiments.contains(id))
            || remaining
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != remaining.len()
        {
            return None;
        }
        let vision = match vision {
            0 => Vision::NotGranted,
            1 | 2 => Vision::Suspended,
            3 => Vision::Expired,
            _ => return None,
        };
        Some(Self {
            plan,
            state: State::NeedsReview,
            vision,
            remaining,
            resume_state: Some(phase),
        })
    }
    /// A loaded sidecar never grants authority until the exact native stack and
    /// campaign have been re-observed. Pending queue work is not replayed here.
    pub fn resume_validated(
        &mut self,
        receipt: &StackReceipt,
        war: RecipientWar,
    ) -> Result<(), ReceiptError> {
        if self.state != State::NeedsReview {
            return Err(ReceiptError::WrongPhase);
        }
        self.validate(receipt)?;
        let phase = self.resume_state.ok_or(ReceiptError::WrongPhase)?;
        if matches!(phase, State::DeploymentPending | State::AwaitingArrival) {
            // A complete real stack can prove an interrupted deployment. This
            // observes the receipt only; no Create/Transfer is replayed on load.
            self.state = State::AwaitingArrival;
            self.resume_state = None;
            return self.verify_assembly(receipt);
        }
        if !matches!(
            phase,
            State::Serving
                | State::ReturnPending
                | State::ReturnReady
                | State::AwaitingReturn
                | State::Returned
        ) {
            return Err(ReceiptError::WrongPhase);
        }
        self.state = phase;
        self.resume_state = None;
        if self.vision == Vision::Suspended && war == RecipientWar::AtWar {
            self.vision = Vision::Active;
        }
        self.observe_recipient(war);
        Ok(())
    }
    /// Losses may remove known members, never introduce new ones. Original IDs
    /// remain in the deployment plan for history; no replacement is manufactured.
    pub fn reconcile_survivors(&mut self, receipt: &StackReceipt) -> Result<(), ReceiptError> {
        if !matches!(
            self.state,
            State::Serving | State::ReturnPending | State::ReturnReady | State::AwaitingReturn
        ) {
            return Err(ReceiptError::WrongPhase);
        }
        if receipt.campaign != self.plan.key.campaign {
            return Err(ReceiptError::WrongCampaign);
        }
        if receipt.army != self.plan.destination {
            return Err(ReceiptError::WrongArmy);
        }
        if receipt.owner != self.plan.key.donor || receipt.affiliation != self.plan.key.donor {
            return Err(ReceiptError::OwnershipChanged);
        }
        let ids = receipt
            .regiments
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        if receipt
            .regiments
            .iter()
            .any(|(id, owner)| *owner != self.plan.key.donor || !self.remaining.contains(id))
            || ids.iter().collect::<std::collections::BTreeSet<_>>().len() != ids.len()
        {
            return Err(ReceiptError::MembershipChanged);
        }
        self.remaining = ids;
        if self.remaining.is_empty() {
            self.apply(Event::ObjectsDestroyed);
        }
        Ok(())
    }

    /// Receipt-only events cannot bypass validation via the ordinary event API.
    pub fn apply(&mut self, event: Event) -> Intent {
        if matches!(event, Event::ArrivalVerified | Event::ReturnVerified) {
            return Intent::None;
        }
        if event == Event::LoadOrCampaignChange && self.vision == Vision::Active {
            self.vision = Vision::Suspended;
        }
        if event == Event::LoadOrCampaignChange && self.state != State::NeedsReview {
            self.resume_state = Some(self.state);
        }
        if event == Event::RecipientPeace {
            self.vision = Vision::Expired;
        }
        let (state, intent) = model::transition(self.state, event);
        self.state = state;
        intent
    }

    fn validate(&self, receipt: &StackReceipt) -> Result<(), ReceiptError> {
        if receipt.campaign != self.plan.key.campaign {
            return Err(ReceiptError::WrongCampaign);
        }
        if receipt.army != self.plan.destination {
            return Err(ReceiptError::WrongArmy);
        }
        if receipt.owner != self.plan.key.donor || receipt.affiliation != self.plan.key.donor {
            return Err(ReceiptError::OwnershipChanged);
        }
        let mut actual = receipt
            .regiments
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        let mut expected = self.remaining.clone();
        actual.sort_unstable();
        expected.sort_unstable();
        if actual != expected
            || receipt
                .regiments
                .iter()
                .any(|(_, owner)| *owner != self.plan.key.donor)
        {
            return Err(ReceiptError::MembershipChanged);
        }
        Ok(())
    }

    /// Grants mission context only after exact donor-owned membership is seen.
    /// A successful queue post alone never starts the mission or grants vision.
    pub fn verify_arrival(&mut self, receipt: &StackReceipt) -> Result<(), ReceiptError> {
        if self.state != State::AwaitingArrival {
            return Err(ReceiptError::WrongPhase);
        }
        self.validate(receipt)?;
        self.state = model::transition(self.state, Event::ArrivalVerified).0;
        if self.vision == Vision::NotGranted {
            self.vision = Vision::Active;
        }
        Ok(())
    }
    pub fn verify_assembly(&mut self, receipt: &StackReceipt) -> Result<(), ReceiptError> {
        if self.state == State::AwaitingArrival {
            return self.verify_arrival(receipt);
        }
        if self.state != State::ReturnPending {
            return Err(ReceiptError::WrongPhase);
        }
        self.validate(receipt)?;
        if self.vision == Vision::NotGranted {
            self.vision = Vision::Active;
        }
        Ok(())
    }

    /// With donor ownership retained, return means observed physical return,
    /// not transferring ownership back. The adapter must read province owner
    /// from the current province rather than substituting the army owner.
    pub fn verify_return(
        &mut self,
        receipt: &StackReceipt,
        province_owner: u32,
    ) -> Result<(), ReceiptError> {
        if self.state != State::AwaitingReturn {
            return Err(ReceiptError::WrongPhase);
        }
        self.validate(receipt)?;
        if province_owner != self.plan.key.donor {
            return Err(ReceiptError::NotInDonorTerritory);
        }
        self.state = model::transition(self.state, Event::ReturnVerified).0;
        Ok(())
    }

    /// Ordinary armies are handled by the caller's unchanged native path.
    /// For this verified loan only, command permission belongs to the donor.
    pub fn player_may_command(&self, receipt: &StackReceipt, player: u32) -> Option<bool> {
        self.context_country(receipt, Use::Orders, RecipientWar::Unknown)?;
        Some(player == self.plan.key.donor)
    }

    /// Only combat/access may borrow B's war context. Accounting, appearance
    /// and command authority always remain A. This is not a native patch.
    pub fn context_country(
        &self,
        receipt: &StackReceipt,
        purpose: Use,
        war: RecipientWar,
    ) -> Option<u32> {
        if !matches!(
            self.state,
            State::Serving | State::ReturnPending | State::ReturnReady | State::AwaitingReturn
        ) {
            return None;
        }
        self.validate(receipt).ok()?;
        if matches!(purpose, Use::Combat | Use::Access) {
            (war == RecipientWar::AtWar).then_some(self.plan.key.recipient)
        } else {
            Some(self.plan.key.donor)
        }
    }

    /// Scope is B-owned provinces. Preserve native visibility from every other
    /// source. Destruction/recall does not revoke a granted recipient entitlement.
    pub fn sees_province(&self, campaign: Campaign, viewer: u32, owner: u32, native: bool) -> bool {
        native
            || (self.vision == Vision::Active
                && campaign == self.plan.key.campaign
                && viewer == self.plan.key.donor
                && owner == self.plan.key.recipient)
    }

    pub fn observe_recipient(&mut self, war: RecipientWar) {
        if war == RecipientWar::AtPeace {
            self.apply(Event::RecipientPeace);
        }
    }
    /// A save-bound terminal troop record can still carry vision. Restoring
    /// that entitlement must not require a destroyed army to exist again.
    pub fn resume_terminal_vision(&mut self, campaign: Campaign, war: RecipientWar) -> bool {
        if campaign != self.plan.key.campaign || self.state != State::NeedsReview {
            return false;
        }
        let Some(phase @ (State::Lost | State::Returned)) = self.resume_state else {
            return false;
        };
        self.state = phase;
        self.resume_state = None;
        if self.vision == Vision::Suspended && war == RecipientWar::AtWar {
            self.vision = Vision::Active;
        }
        self.observe_recipient(war);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> (DeploymentPlan, StackReceipt) {
        let key = MissionKey {
            campaign: Campaign([1; 16]),
            serial: 1,
            donor: 80,
            recipient: 90,
        };
        let r = SendRequest {
            donor: 80,
            recipient: 90,
            donor_at_war: false,
            recipient_at_war: true,
            recipient_enemy_of_donor: false,
            source_owner: 80,
            source: Identity { kind: 40, id: 568 },
            regiments: vec![Identity { kind: 40, id: 7 }, Identity { kind: 40, id: 8 }],
            source_in_combat: false,
            source_moving: false,
            source_is_land: true,
            already_deployed_by_donor: 0,
            multiplayer: false,
        };
        let destination = Identity {
            kind: 5000,
            id: 6000,
        };
        let p =
            DeploymentPlan::new(key, &r, &Settings::default(), 0x415246, destination, 425).unwrap();
        let receipt = StackReceipt {
            campaign: key.campaign,
            army: destination,
            owner: 80,
            affiliation: 80,
            regiments: r.regiments.iter().map(|id| (*id, 80)).collect(),
        };
        (p, receipt)
    }
    fn serving() -> (Mission, StackReceipt) {
        let (plan, receipt) = setup();
        let mut m = Mission::new(plan);
        assert_eq!(m.apply(Event::Approved), Intent::QueueDeployment);
        m.apply(Event::DeploymentPosted);
        m.verify_arrival(&receipt).unwrap();
        (m, receipt)
    }
    #[test]
    fn native_detach_plan_keeps_donor_and_selected_regiments() {
        let (p, receipt) = setup();
        let a = p.create_arguments(0x10000);
        assert_eq!(
            a,
            [0x10000, 0x415246, 80, 5000, 6000, 5000, 6000, 425, 0, 0]
        );
        assert_eq!(p.source(), Identity { kind: 40, id: 568 });
        assert_eq!(
            p.selected_regiments(),
            receipt
                .regiments
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn only_donor_commands_and_accounting_never_uses_recipient() {
        let (m, receipt) = serving();
        assert_eq!(m.player_may_command(&receipt, 80), Some(true));
        assert_eq!(m.player_may_command(&receipt, 90), Some(false));
        assert_eq!(m.player_may_command(&receipt, 100), Some(false));
        for purpose in [
            Use::Orders,
            Use::Ownership,
            Use::Goods,
            Use::Reinforcement,
            Use::Flag,
            Use::MilitaryStatistics,
        ] {
            assert_eq!(
                m.context_country(&receipt, purpose, RecipientWar::AtWar),
                Some(80)
            );
        }
        for purpose in [Use::Combat, Use::Access] {
            assert_eq!(
                m.context_country(&receipt, purpose, RecipientWar::AtWar),
                Some(90)
            );
            assert_eq!(
                m.context_country(&receipt, purpose, RecipientWar::Unknown),
                None
            );
            assert_eq!(
                m.context_country(&receipt, purpose, RecipientWar::AtPeace),
                None
            );
        }
    }
    #[test]
    fn posting_does_not_grant_control_context_or_vision() {
        let (p, receipt) = setup();
        let mut m = Mission::new(p);
        m.apply(Event::Approved);
        m.apply(Event::DeploymentPosted);
        m.apply(Event::ArrivalVerified); // ordinary event cannot forge receipt
        assert_eq!(m.state(), State::AwaitingArrival);
        assert_eq!(m.player_may_command(&receipt, 80), None);
        assert!(!m.sees_province(receipt.campaign, 80, 90, false));
    }
    #[test]
    fn recipient_ownership_mixed_membership_and_wrong_campaign_are_refused() {
        for change in 0..5 {
            let (p, mut r) = setup();
            let mut m = Mission::new(p);
            m.apply(Event::Approved);
            m.apply(Event::DeploymentPosted);
            match change {
                0 => r.owner = 90,
                1 => r.affiliation = 90,
                2 => r.regiments[0].1 = 90,
                3 => r.regiments.push((Identity { kind: 40, id: 9 }, 80)),
                _ => r.campaign = Campaign([2; 16]),
            }
            assert!(m.verify_arrival(&r).is_err());
            assert_eq!(m.state(), State::AwaitingArrival);
        }
    }
    #[test]
    fn vision_survives_recall_and_destruction_until_all_wars_end() {
        for event in [Event::Recall, Event::ObjectsDestroyed] {
            let (mut m, r) = serving();
            m.apply(event);
            m.observe_recipient(RecipientWar::AtWar);
            assert!(m.sees_province(r.campaign, 80, 90, false));
            assert!(!m.sees_province(r.campaign, 80, 100, false));
            assert!(!m.sees_province(r.campaign, 90, 90, false));
            m.observe_recipient(RecipientWar::Unknown);
            assert!(m.sees_province(r.campaign, 80, 90, false));
            m.observe_recipient(RecipientWar::AtPeace);
            assert!(!m.sees_province(r.campaign, 80, 90, false));
            assert!(m.sees_province(r.campaign, 80, 90, true));
            m.observe_recipient(RecipientWar::AtWar); // no automatic new grant
            assert!(!m.sees_province(r.campaign, 80, 90, false));
        }
    }
    #[test]
    fn changed_membership_and_load_cannot_reuse_old_permission() {
        let (mut m, mut r) = serving();
        r.regiments.pop();
        assert_eq!(m.player_may_command(&r, 80), None);
        m.apply(Event::LoadOrCampaignChange);
        assert_eq!(m.state(), State::NeedsReview);
        assert!(!m.sees_province(r.campaign, 80, 90, false));
        assert!(!m.sees_province(Campaign([2; 16]), 80, 90, false));
        assert_eq!(m.player_may_command(&r, 80), None);
    }
    #[test]
    fn return_requires_physical_donor_territory_and_keeps_vision_until_peace() {
        let (mut m, receipt) = serving();
        m.apply(Event::Recall);
        assert_eq!(m.apply(Event::SourceReady), Intent::QueueReturn);
        m.apply(Event::ReturnPosted);
        assert_eq!(
            m.verify_return(&receipt, 90),
            Err(ReceiptError::NotInDonorTerritory)
        );
        assert_eq!(m.state(), State::AwaitingReturn);
        m.verify_return(&receipt, 80).unwrap();
        assert_eq!(m.state(), State::Returned);
        assert!(m.sees_province(receipt.campaign, 80, 90, false));
        m.observe_recipient(RecipientWar::AtPeace);
        assert!(!m.sees_province(receipt.campaign, 80, 90, false));
    }
}
