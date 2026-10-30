//! Admission to simulation work is separate from UI preferences and observation.
//! An unknown session is never assumed to be single player.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionMode {
    #[default]
    Unknown,
    SinglePlayer,
    Multiplayer,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Capabilities {
    pub session_detection: bool,
    pub campaign_binding: bool,
    pub command_queue: bool,
    pub combat: bool,
    pub access: bool,
    pub vision: bool,
    pub persistence: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unavailable {
    SessionUnknown,
    MultiplayerUnsupported,
    MultiplayerHandshake,
    NativeCompatibility,
    CampaignUnbound,
    NativeCommands,
    Combat,
    Access,
    Vision,
    Persistence,
}
impl Unavailable {
    pub fn message(self) -> &'static str {
        match self {
            Self::SessionUnknown => "Waiting for a recognized game session.",
            Self::MultiplayerUnsupported => "Multiplayer sending is not supported in this build.",
            Self::MultiplayerHandshake => {
                "Waiting for all MP players to enable the same Volunteers build."
            }
            Self::NativeCompatibility => {
                "Native hook compatibility failed; check the Volunteers log."
            }
            Self::CampaignUnbound => "Waiting for the saved volunteer journal to bind.",
            Self::NativeCommands => "Waiting for the native command queue.",
            Self::Combat => "Volunteer combat hook is unavailable; check the log.",
            Self::Access => "Volunteer access hook is unavailable; check the log.",
            Self::Vision => "Volunteer vision hook is unavailable; check the log.",
            Self::Persistence => "Waiting for the saved volunteer journal.",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SimulationPermit {
    epoch: u64,
    mode: SessionMode,
    issuer: Option<u32>,
    round: u64,
}

impl SimulationPermit {
    pub fn epoch(self) -> u64 {
        self.epoch
    }
    pub fn matches_mode(self, mode: SessionMode) -> bool {
        self.mode == mode
    }
    pub fn multiplayer(self) -> bool {
        self.mode == SessionMode::Multiplayer
    }
    pub fn policy(self) -> bool {
        !self.multiplayer() || self.issuer.is_none()
    }
    pub fn commands_for(self, country: u32) -> bool {
        !self.multiplayer() || self.issuer == Some(country)
    }
    pub(crate) fn authorized_by(self, proof: crate::mp_authority::IssuerAuthority) -> bool {
        self.multiplayer()
            && self.issuer == Some(proof.country())
            && self.epoch == proof.epoch()
            && self.round == proof.round()
    }
}

pub fn admit(
    mode: SessionMode,
    capabilities: Capabilities,
    epoch: u64,
) -> Result<SimulationPermit, Unavailable> {
    match mode {
        SessionMode::Multiplayer => return Err(Unavailable::MultiplayerUnsupported),
        SessionMode::Unknown => return Err(Unavailable::SessionUnknown),
        SessionMode::SinglePlayer if !capabilities.session_detection => {
            return Err(Unavailable::SessionUnknown);
        }
        _ => (),
    }
    check_capabilities(capabilities, epoch)?;
    Ok(SimulationPermit {
        epoch,
        mode: SessionMode::SinglePlayer,
        issuer: None,
        round: 0,
    })
}
fn check_capabilities(capabilities: Capabilities, epoch: u64) -> Result<(), Unavailable> {
    if !capabilities.session_detection {
        return Err(Unavailable::SessionUnknown);
    }
    for (ready, reason) in [
        (
            capabilities.campaign_binding && epoch != 0,
            Unavailable::CampaignUnbound,
        ),
        (capabilities.command_queue, Unavailable::NativeCommands),
        (capabilities.combat, Unavailable::Combat),
        (capabilities.access, Unavailable::Access),
        (capabilities.vision, Unavailable::Vision),
        (capabilities.persistence, Unavailable::Persistence),
    ] {
        if !ready {
            return Err(reason);
        }
    }
    Ok(())
}
pub(crate) fn admit_issuer(
    proof: crate::mp_authority::IssuerAuthority,
    capabilities: Capabilities,
    epoch: u64,
    country: u32,
) -> Result<SimulationPermit, Unavailable> {
    if proof.epoch() != epoch || proof.country() != country {
        return Err(Unavailable::CampaignUnbound);
    }
    check_capabilities(capabilities, epoch)?;
    Ok(SimulationPermit {
        epoch,
        mode: SessionMode::Multiplayer,
        issuer: Some(country),
        round: proof.round(),
    })
}
pub(crate) fn admit_policy(proof: crate::mp_authority::PolicyAuthority) -> SimulationPermit {
    SimulationPermit {
        epoch: proof.epoch(),
        mode: SessionMode::Multiplayer,
        issuer: None,
        round: 0,
    }
}

/// Data carried through the eventual engine transport. A local sidecar or GUI
/// click cannot mint proof that a command executed on every simulation peer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerFingerprint {
    pub protocol: u32,
    pub exe_sha256: String,
    pub plugin_sha256: String,
    pub settings_sha256: String,
}

impl PeerFingerprint {
    pub fn compatible(&self, other: &Self) -> bool {
        self == other
    }
}
