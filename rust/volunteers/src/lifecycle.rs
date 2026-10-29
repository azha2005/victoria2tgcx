//! Save/load integration endpoints. The native save-complete/load-complete
//! hooks are intentionally not guessed: callers must provide verified events.
use crate::mission::Campaign;
use crate::persistence::{SaveBinding, Store};
use crate::registry::Registry;
use std::io;
use std::path::{Path, PathBuf};

pub struct Lifecycle {
    store: Store,
    binding: Option<SaveBinding>,
    epoch: u64,
    campaign: Option<Campaign>,
}
impl Lifecycle {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            store: Store::new(directory),
            binding: None,
            epoch: 0,
            campaign: None,
        }
    }
    pub fn binding(&self) -> Option<&SaveBinding> {
        self.binding.as_ref()
    }
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    pub fn begin_verified_campaign(&mut self, campaign: Campaign) -> Option<Registry> {
        if campaign.0 == [0; 16] {
            return None;
        }
        self.epoch = self.epoch.checked_add(1)?;
        self.binding = None;
        self.campaign = Some(campaign);
        Some(Registry::new(campaign, self.epoch))
    }
    /// Must run after the engine closed and completed this exact save. The
    /// checkpoint names its content digest; it cannot attach to a different save.
    pub fn save_completed(&mut self, path: &Path, registry: &Registry) -> io::Result<PathBuf> {
        if self.epoch == 0
            || registry.epoch != self.epoch
            || self.campaign != Some(registry.campaign)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "stale campaign epoch",
            ));
        }
        let binding = SaveBinding::from_saved_file(registry.campaign, path)?;
        let saved = self.store.save(&binding, registry)?;
        self.binding = Some(binding);
        Ok(saved)
    }
    /// The caller obtains campaign from its verified saved-game association,
    /// not a date, tag or process address. Loaded missions remain suspended.
    pub fn load_completed(&mut self, path: &Path, campaign: Campaign) -> io::Result<Registry> {
        let epoch = self
            .epoch
            .checked_add(1)
            .ok_or_else(|| io::Error::other("epoch exhausted"))?;
        // Invalidate the old binding even if hashing or checkpoint loading
        // fails. The caller must also suspend the old registry before load.
        self.epoch = epoch;
        self.binding = None;
        self.campaign = None;
        let binding = SaveBinding::from_saved_file(campaign, path)?;
        let registry = self.store.load(&binding, epoch)?;
        self.epoch = epoch;
        self.binding = Some(binding);
        self.campaign = Some(campaign);
        Ok(registry)
    }
    pub fn leave_world(&mut self, registry: &mut Registry) {
        registry.suspend();
        self.binding = None;
        self.campaign = None;
        self.epoch = self.epoch.saturating_add(1);
    }
}
