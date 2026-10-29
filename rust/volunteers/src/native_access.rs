//! Scoped land access using the original native predicate and a private input
//! image. No live army owner, province, list or diplomacy relation is written.
use crate::mission::{RecipientWar, Use};
use crate::native::Native;
use crate::native_call::call;
use crate::registry::Registry;
use crate::session::{SessionMode, SinglePlayerPermit};

impl Native {
    /// An additive land-only override for CArmy slot33 / FUN_005D5E30.
    /// The native caller retains transport/exile/path gates; a true native
    /// result never needs replacement. No hook is installed here yet.
    pub fn recipient_land_access(
        &self,
        registry: &Registry,
        permit: SinglePlayerPermit,
        army: u32,
        province: u32,
        native_allowed: bool,
    ) -> Option<bool> {
        if native_allowed {
            return Some(true);
        }
        let input = self.recipient_army_image(registry, permit, army, province, Use::Access)?;
        type Predicate = unsafe extern "fastcall" fn(*const u32, u32, u32) -> u8;
        call(|| unsafe {
            std::mem::transmute::<usize, Predicate>((self.base + 0x1CD2F0) as usize)(
                input.as_ptr(),
                0,
                province,
            )
        })
        .map(|value| value != 0)
    }
    /// A read-only input projection. All fields except the owner reference stay
    /// unchanged; callers must never pass this image to a mutating function.
    pub(crate) fn recipient_army_image(
        &self,
        registry: &Registry,
        permit: SinglePlayerPermit,
        army: u32,
        province: u32,
        purpose: Use,
    ) -> Option<[u32; 0x1B8 / 4]> {
        if permit.epoch() != registry.epoch
            || crate::native_session::capture(self, self.base)?.mode != SessionMode::SinglePlayer
            || self.u32(army)? != self.base + 0x9F88CC
            || self.byte(self.field(province, 0x5C)? + 0x2A)? == 0
        {
            return None;
        }
        let id = crate::model::Identity {
            kind: self.field(army, 0x10)?,
            id: self.field(army, 0x14)?,
        };
        let entry = registry.live_army(id)?;
        let donor = self.country(entry.deployment.key.donor)?;
        let recipient = self.country(entry.deployment.key.recipient)?;
        if self.byte(donor + 0x67C)? != 0 || self.byte(recipient + 0x67C)? == 0 {
            return None;
        }
        let receipt = self.receipt(registry.campaign, id)?;
        if self.army(id)? != army
            || registry.context(&receipt, purpose, RecipientWar::AtWar)
                != Some(entry.deployment.key.recipient)
        {
            return None;
        }
        // Disassembly of FUN_005CD2F0 proves this call only reads the army's
        // +DC, +C0/+C4. It performs controller/overlord gates and delegates the
        // final country access check to FUN_0051B040. Use the original predicate
        // with a local full-size input image; never temporarily edit the army.
        let bytes = self.bytes(army, 0x1B8)?;
        let mut input = [0u32; 0x1B8 / 4];
        for (word, chunk) in input.iter_mut().zip(bytes.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*chunk);
        }
        input[0xC0 / 4] = self.field(recipient, 0x1C)?;
        input[0xC4 / 4] = entry.deployment.key.recipient;
        Some(input)
    }
}
