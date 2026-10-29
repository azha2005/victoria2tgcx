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
    CampaignUnbound,
    NativeCommands,
    Combat,
    Access,
    Vision,
    Persistence,
}

#[derive(Clone, Copy, Debug)]
pub struct SinglePlayerPermit {
    epoch: u64,
}

impl SinglePlayerPermit {
    pub fn epoch(self) -> u64 {
        self.epoch
    }
}

pub fn admit(
    mode: SessionMode,
    capabilities: Capabilities,
    epoch: u64,
) -> Result<SinglePlayerPermit, Unavailable> {
    match mode {
        SessionMode::Multiplayer => return Err(Unavailable::MultiplayerUnsupported),
        SessionMode::Unknown => return Err(Unavailable::SessionUnknown),
        SessionMode::SinglePlayer if !capabilities.session_detection => {
            return Err(Unavailable::SessionUnknown);
        }
        _ => (),
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
    Ok(SinglePlayerPermit { epoch })
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
