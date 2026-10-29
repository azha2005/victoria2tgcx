//! Scoped Win32/x86 adapters reused from the proved ATCU command/GUI contracts.
//! No ownership, province, membership, strength, money or pop writes. Native
//! commands remain gated by an admitted single-player permit from the controller.
use crate::app::{ArmyView, CountryView, RegimentView, WorldView};
use crate::mission::{DeploymentPlan, RecipientWar};
use crate::model::Identity;
use crate::native_call::call;
use crate::native_signatures::SIGNATURES;
use crate::native_string::GameString;
use crate::session::{SessionMode, SinglePlayerPermit};
use crate::snapshot::{Memory, capture_stack_receipt};
use std::collections::BTreeSet;
use std::ffi::c_void;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> *mut c_void;
    fn ReadProcessMemory(
        process: *mut c_void,
        address: *const c_void,
        out: *mut c_void,
        size: usize,
        copied: *mut usize,
    ) -> i32;
    pub fn GetCurrentThreadId() -> u32;
    fn GetLastError() -> u32;
    fn SetLastError(error: u32);
}
pub struct LastError(u32);
impl LastError {
    pub fn preserve() -> Self {
        Self(unsafe { GetLastError() })
    }
}
impl Drop for LastError {
    fn drop(&mut self) {
        unsafe { SetLastError(self.0) }
    }
}

#[derive(Clone, Copy)]
pub struct Native {
    pub base: u32,
}
impl Memory for Native {
    fn read32(&self, address: u32) -> Option<u32> {
        self.u32(address)
    }
    fn read_words(&self, address: u32, count: usize) -> Option<Vec<u32>> {
        if count > 65536 {
            return None;
        }
        Some(
            self.bytes(address, count.checked_mul(4)?)?
                .as_chunks::<4>()
                .0
                .iter()
                .map(|b| u32::from_le_bytes(*b))
                .collect(),
        )
    }
}
impl Native {
    pub fn prepare_checkpoint(
        &self,
        permit: SinglePlayerPermit,
        donor: u32,
        registry: &crate::registry::Registry,
        previous: u32,
    ) -> Option<(Batch, u32, String)> {
        let session = crate::native_session::capture(self, self.base)?;
        if session.mode != SessionMode::SinglePlayer
            || registry.epoch != permit.epoch()
            || crate::native_lifecycle::epoch(session.context) != Some(registry.epoch)
            || registry.entries().any(|e| e.deployment.key.donor != donor)
        {
            return None;
        }
        let head = self
            .variable(donor, crate::engine_store::HEAD)
            .ok()?
            .unwrap_or(0);
        if head < 0 || head % 1000 != 0 || head as u32 / 1000 != previous {
            return None;
        }
        let tx = crate::engine_store::Transaction::new(registry, previous).ok()?;
        let mut writes = Vec::new();
        for write in &tx.writes {
            if self.variable(donor, &write.key).ok()? != Some(write.raw_thousandths) {
                writes.push(write.clone());
            }
        }
        let batch = self.prepare_variables(permit, donor, &writes)?;
        Some((batch, tx.committed_head, tx.journal_sha256))
    }
    pub fn variable(
        &self,
        country: u32,
        key: &str,
    ) -> Result<Option<i32>, crate::engine_store::Error> {
        use crate::engine_store::Error;
        if !key.starts_with("tgcx_vl_") || key.len() > 96 || !key.is_ascii() {
            return Err(Error::Invalid);
        }
        let country = self.country(country).ok_or(Error::Missing)?;
        let variables = country.checked_add(0x1DC).ok_or(Error::Invalid)?;
        if self.u32(variables) != Some(self.base + 0x9FBB60)
            || !self.expected_slot(variables, 1, 0x241BA0)
        {
            return Err(Error::Invalid);
        }
        let name = GameString::new(key);
        type Find = unsafe extern "fastcall" fn(u32, u32, *const GameString) -> u32;
        let node = call(|| unsafe {
            std::mem::transmute::<usize, Find>((self.base + 0x241BA0) as usize)(variables, 0, &name)
        })
        .ok_or(Error::Missing)?;
        if node == 0 {
            return Ok(None);
        }
        self.field(node, 0x1C)
            .map(|v| Some(v as i32))
            .ok_or(Error::Missing)
    }
    pub fn prepare_variables(
        &self,
        permit: SinglePlayerPermit,
        donor: u32,
        writes: &[crate::engine_store::Write],
    ) -> Option<Batch> {
        if permit.epoch() == 0
            || writes.is_empty()
            || writes.len() > crate::engine_store::MAX_BYTES / 2 + 2
        {
            return None;
        }
        let country = self.country(donor)?;
        let tag = self.field(country, 0x1C)?;
        if self.u32(country + 0x1DC) != Some(self.base + 0x9FBB60) {
            return None;
        }
        let mut batch = Batch {
            native: *self,
            queue: self.queue()?,
            commands: Vec::new(),
        };
        for write in writes {
            if !write.key.starts_with("tgcx_vl_")
                || write.key.len() > 96
                || !write.key.is_ascii()
                || write.raw_thousandths < 0
                || write.raw_thousandths % 1000 != 0
            {
                return None;
            }
            let name = GameString::new(&write.key);
            let allocation = self.allocate(0x64)?;
            // Native clone caller RVA183C1E pushes0x64. Constructor RVA183900
            // takes (allocation,tag,index,string*,fixed_point_value), RET0x14.
            type Create = unsafe extern "system" fn(u32, u32, u32, *const GameString, i32) -> u32;
            let command = call(|| unsafe {
                std::mem::transmute::<usize, Create>((self.base + 0x183900) as usize)(
                    allocation,
                    tag,
                    donor,
                    &name,
                    write.raw_thousandths,
                )
            })?;
            if self.u32(command) != Some(self.base + 0xA0121C) {
                return None;
            }
            batch.commands.push(command);
        }
        Some(batch)
    }
    pub fn new(base: u32) -> Option<Self> {
        let n = Self { base };
        for signature in SIGNATURES {
            let mut expected = signature.bytes.to_vec();
            for &offset in signature.relocations {
                let preferred = u32::from_le_bytes(expected[offset..offset + 4].try_into().ok()?);
                let loaded = preferred.wrapping_sub(0x400000).wrapping_add(base);
                expected[offset..offset + 4].copy_from_slice(&loaded.to_le_bytes());
            }
            if n.bytes(base.checked_add(signature.rva)?, expected.len())? != expected {
                return None;
            }
        }
        Some(n)
    }
    pub fn bytes(&self, address: u32, count: usize) -> Option<Vec<u8>> {
        if address < 0x10000 || count > 1024 * 1024 || address.checked_add(count as u32).is_none() {
            return None;
        }
        let mut data = vec![0u8; count];
        let mut copied = 0;
        let ok = unsafe {
            ReadProcessMemory(
                GetCurrentProcess(),
                address as *const c_void,
                data.as_mut_ptr().cast(),
                count,
                &mut copied,
            )
        };
        (ok != 0 && copied == count).then_some(data)
    }
    pub fn u32(&self, address: u32) -> Option<u32> {
        if address < 0x10000 || address.checked_add(4).is_none() {
            return None;
        }
        // These callbacks run inside this process. No allocation or kernel
        // roundtrip per scalar; SEH still fails closed on an invalid pointer.
        call(|| unsafe { std::ptr::read_unaligned(address as *const u32) })
    }
    pub fn field(&self, object: u32, offset: u32) -> Option<u32> {
        self.u32(object.checked_add(offset)?)
    }
    pub fn byte(&self, address: u32) -> Option<u8> {
        if address < 0x10000 || address.checked_add(1).is_none() {
            return None;
        }
        call(|| unsafe { std::ptr::read_unaligned(address as *const u8) })
    }
    pub fn in_image(&self, address: u32) -> bool {
        let Some(pe) = self.field(self.base, 0x3C) else {
            return false;
        };
        let Some(size) = self.field(self.base, pe.saturating_add(24 + 56)) else {
            return false;
        };
        address >= self.base && address < self.base.saturating_add(size)
    }
    pub fn slot(&self, object: u32, index: u32) -> Option<u32> {
        self.field(self.u32(object)?, index.checked_mul(4)?)
    }
    pub fn expected_slot(&self, object: u32, index: u32, rva: u32) -> bool {
        self.slot(object, index) == self.base.checked_add(rva)
    }
    pub fn string(&self, address: u32, max: usize) -> Option<String> {
        let len = self.field(address, 0x10)? as usize;
        let cap = self.field(address, 0x14)? as usize;
        if len > max || cap < len || cap > 65536 || (cap < 16 && cap != 15) {
            return None;
        }
        let data = if cap > 15 {
            self.u32(address)?
        } else {
            address
        };
        let bytes = self.bytes(data, len.checked_add(1)?)?;
        if bytes[len] != 0 {
            return None;
        }
        Some(bytes[..len].iter().map(|b| cp1252(*b)).collect())
    }
    pub fn list(&self, header: u32, limit: usize) -> Option<Vec<u32>> {
        let count = self.field(header, 8)? as usize;
        if count > limit {
            return None;
        }
        let mut node = self.u32(header)?;
        let mut seen = BTreeSet::new();
        let mut items = Vec::new();
        while node != 0 {
            if seen.len() >= limit || !seen.insert(node) || self.byte(node.checked_add(12)?)? != 0 {
                return None;
            }
            items.push(self.u32(node)?);
            node = self.field(node, 8)?;
        }
        (items.len() == count).then_some(items)
    }
    pub fn country(&self, index: u32) -> Option<u32> {
        let manager = self.u32(self.base + 0xE587E4)?;
        let count = self.u32(manager)?;
        if count > 4096 || index >= count {
            return None;
        }
        self.field(self.field(manager, 4)?, index.checked_mul(4)?)
            .filter(|p| *p != 0)
    }
    pub fn lookup(&self, id: Identity) -> Option<u32> {
        if !id.valid() {
            return None;
        }
        let table = self.u32(self.base + if id.kind > 0x1268 { 0xE58A94 } else { 0xE58A98 })?;
        let count = self.field(table, 4)?;
        let buckets = self.field(table, 8)?;
        if count == 0 || count > 1_000_000 {
            return None;
        }
        let key = (id.kind << 16).wrapping_add(id.id);
        let mut node = self.field(buckets, (key % count) * 4)?;
        let mut seen = BTreeSet::new();
        while node != 0 && seen.len() < 4096 && seen.insert(node) {
            let entry = self.u32(node)?;
            if self.field(entry, 8)? == id.kind && self.field(entry, 12)? == id.id {
                return Some(entry);
            }
            node = self.field(node, 4)?;
        }
        None
    }
    /// Absence is only established after a fully readable, terminated hash
    /// chain and unchanged table/bucket. A failed read or cycle is not death.
    pub fn definitely_absent(&self, id: Identity) -> bool {
        let probe = || -> Option<bool> {
            if !id.valid() {
                return None;
            }
            let global = self.base + if id.kind > 0x1268 { 0xE58A94 } else { 0xE58A98 };
            let table = self.u32(global)?;
            let count = self.field(table, 4)?;
            let buckets = self.field(table, 8)?;
            if count == 0 || count > 1_000_000 {
                return None;
            }
            let bucket = buckets.checked_add(((id.kind << 16).wrapping_add(id.id) % count) * 4)?;
            let head = self.u32(bucket)?;
            let mut node = head;
            let mut seen = BTreeSet::new();
            while node != 0 {
                if seen.len() >= 4096 || !seen.insert(node) {
                    return None;
                }
                let entry = self.u32(node)?;
                if self.field(entry, 8)? == id.kind && self.field(entry, 12)? == id.id {
                    return Some(false);
                }
                node = self.field(node, 4)?;
            }
            Some(
                self.u32(global)? == table
                    && self.field(table, 4)? == count
                    && self.field(table, 8)? == buckets
                    && self.u32(bucket)? == head,
            )
        };
        probe() == Some(true)
    }
    pub fn army(&self, id: Identity) -> Option<u32> {
        let p = self.lookup(id)?.checked_sub(8)?;
        (self.u32(p)? == self.base + 0x9F88CC
            && self.field(p, 0x10)? == id.kind
            && self.field(p, 0x14)? == id.id)
            .then_some(p)
    }

    /// Read-only preview. Session mode uses the exact local/network transport;
    /// campaign identity and gameplay admission are independent prerequisites.
    pub fn world(&self) -> Option<WorldView> {
        self.world_subset(None, true)
    }
    /// Closed panels need fresh mission policy, not a full chooser inventory.
    /// No native pointers/results are cached: all relevant countries are read
    /// again. Opening/clicking the UI always takes the full world snapshot.
    pub fn mission_world(&self, registry: &crate::registry::Registry) -> Option<WorldView> {
        let world = self.u32(self.base + 0xE588E8)?;
        let mut countries = BTreeSet::from([self.field(world, 0xB60)?]);
        for e in registry.entries() {
            countries.insert(e.deployment.key.donor);
            countries.insert(e.deployment.key.recipient);
        }
        self.world_subset(Some(&countries), false)
    }
    fn world_subset(
        &self,
        subset: Option<&BTreeSet<u32>>,
        include_armies: bool,
    ) -> Option<WorldView> {
        let world = self.u32(self.base + 0xE588E8)?;
        let player = self.field(world, 0xB60)?;
        let donor = self.country(player)?;
        let manager = self.u32(self.base + 0xE587E4)?;
        let count = self.u32(manager)?;
        let mut result = WorldView {
            mode: crate::native_session::capture(self, self.base)
                .filter(|s| s.world == world)
                .map_or(SessionMode::Unknown, |s| s.mode),
            player,
            date: self.field(world, 0xB0C)?,
            player_at_war: self.byte(donor + 0x67C)? != 0,
            ..WorldView::default()
        };
        let relations = self.field(donor, 0xBE8)?;
        let selected = subset.into_iter().flat_map(|set| set.iter().copied());
        let all = 1..if subset.is_none() { count } else { 1 };
        for index in selected.chain(all) {
            if index == 0 || index >= count {
                continue;
            }
            let Some(country) = self.country(index) else {
                continue;
            };
            let Some(tag) = self.field(country, 0x1C) else {
                continue;
            };
            let caption = tag
                .to_le_bytes()
                .iter()
                .take(3)
                .filter(|b| b.is_ascii_graphic())
                .map(|b| char::from(*b))
                .collect::<String>();
            if caption.is_empty() {
                continue;
            }
            let relation = self.field(relations, index * 4).filter(|p| *p != 0);
            let hostile = index != player
                && relation
                    .and_then(|p| self.field(p, 0x34))
                    .is_none_or(|h| h != 0);
            let war = match self.byte(country + 0x67C) {
                Some(0) => RecipientWar::AtPeace,
                Some(_) => RecipientWar::AtWar,
                None => RecipientWar::Unknown,
            };
            result.countries.push(CountryView {
                index,
                tag,
                caption,
                war,
                hostile_to_player: hostile,
                landing_province: self.first_owned_land(country, index),
            });
        }
        if !include_armies {
            return Some(result);
        }
        for army in self.list(donor + 0x7B4, 2048)? {
            if self.u32(army) != Some(self.base + 0x9F88CC)
                || self.field(army, 0xC4) != Some(player)
            {
                continue;
            }
            let Some(entries) = self.list(army + 0x38, 1024) else {
                continue;
            };
            let mut regiments = Vec::new();
            let mut valid = true;
            for reg in entries {
                if self.u32(reg) != Some(self.base + 0x9F8884)
                    || self.field(reg, 0x74) != Some(army)
                {
                    valid = false;
                    break;
                }
                let Some(owner) = self.field(reg, 0x64) else {
                    valid = false;
                    break;
                };
                let Some(kind) = self.field(reg, 8) else {
                    valid = false;
                    break;
                };
                let Some(id) = self.field(reg, 12) else {
                    valid = false;
                    break;
                };
                regiments.push(RegimentView {
                    id: Identity { kind, id },
                    owner,
                    name: self
                        .string(reg + 0x44, 96)
                        .unwrap_or_else(|| format!("Brigade {id}")),
                });
            }
            if !valid {
                continue;
            }
            let province = self.field(self.field(army, 0xDC)?, 0x58)?;
            result.armies.push(ArmyView {
                id: Identity {
                    kind: self.field(army, 0x10)?,
                    id: self.field(army, 0x14)?,
                },
                owner: player,
                affiliation: self.field(army, 0xCC)?,
                province,
                name: self
                    .string(army + 0x1C, 96)
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| format!("Army {}", self.field(army, 0x14).unwrap_or(0))),
                combat: self.field(army, 0x74)? != 0,
                moving: self.field(army, 0xEC)? != 0,
                regiments,
            });
        }
        (self.u32(self.base + 0xE588E8)? == world && self.field(world, 0xB60)? == player)
            .then_some(result)
    }
    fn first_owned_land(&self, country: u32, index: u32) -> Option<u32> {
        let start = self.field(country, 0x9D8)?;
        let end = self.field(country, 0x9DC)?;
        if start == 0 || end < start || (end - start) % 4 != 0 || (end - start) / 4 > 65536 {
            return None;
        }
        let world = self.u32(self.base + 0xE588E8)?;
        let provinces = self.field(world, 0xACC)?;
        let last = self.field(world, 0xAD0)?;
        let province_count = last.checked_sub(provinces)? / 4;
        if provinces == 0 || (last - provinces) % 4 != 0 || province_count > 65536 {
            return None;
        }
        for slot in (start..end).step_by(4) {
            let id = self.u32(slot)?;
            if id == 0 || id >= province_count {
                continue;
            }
            let province = self.field(provinces, id * 4)?;
            if self.field(province, 0x12C) == Some(index)
                && self
                    .field(province, 0x5C)
                    .and_then(|p| self.byte(p + 0x2A))
                    .is_some_and(|v| v != 0)
            {
                return Some(id);
            }
        }
        None
    }

    pub fn replace_slot(&self, address: u32, expected: u32, replacement: u32) -> bool {
        if !self.in_image(address) {
            return false;
        }
        let result = crate::patch::replace(address, expected, replacement);
        if !result.protection_restored {
            crate::runtime::diagnostic(
                "Volunteer vtable protection/patch refusal; DLL remains pinned (P66)",
            );
        }
        result.complete()
    }

    /// Native client identity allocation, matching the new-stack button. This
    /// touches only its UI allocation counter under the native lock (ATCU path).
    pub fn allocate_identity(&self, permit: SinglePlayerPermit) -> Option<Identity> {
        if permit.epoch() == 0 || crate::native_call::faulted() {
            return None;
        }
        let lock = self.base + 0xF1CA68;
        self.bytes(lock, 1)?;
        self.bytes(self.base + 0xE58A9C, 4)?;
        let kind = u16::from_le_bytes(self.bytes(self.base + 0xE58A90, 2)?.try_into().ok()?) as u32
            + 0x1268;
        let release: u32;
        unsafe {
            std::arch::asm!("call {f}",f=in(reg) self.base+0x5C12D0,inlateout("eax") lock=>release,clobber_abi("C"));
        }
        let counter = (self.base + 0xE58A9C) as *mut u32;
        let id = unsafe {
            let next = std::ptr::read_volatile(counter).checked_add(1);
            if let Some(next) = next {
                std::ptr::write_volatile(counter, next);
            }
            next
        };
        unsafe {
            std::ptr::write_volatile(lock as *mut u8, release as u8);
        }
        Some(Identity { kind, id: id? })
    }
    pub fn prepare_detach(
        &self,
        permit: SinglePlayerPermit,
        plan: &DeploymentPlan,
    ) -> Option<Batch> {
        if permit.epoch() == 0 {
            return None;
        }
        let source = self.army(plan.source())?;
        if self.field(source, 0xC4) != Some(plan.key().donor)
            || self.field(source, 0xCC) != Some(plan.key().donor)
            || self.field(source, 0x74) != Some(0)
            || self.field(source, 0xEC) != Some(0)
            || self.field(self.field(source, 0xDC)?, 0x58) != Some(plan.origin_province())
        {
            return None;
        }
        let mut registers = Vec::new();
        for id in plan.selected_regiments() {
            let reg = self.lookup(*id)?;
            if self.u32(reg) != Some(self.base + 0x9F8884)
                || self.field(reg, 0x74) != Some(source)
                || self.field(reg, 0x64) != Some(plan.key().donor)
            {
                return None;
            }
            registers.push(reg);
        }
        let queue = self.queue()?;
        let mut batch = Batch {
            native: *self,
            queue,
            commands: Vec::new(),
        };
        if plan.destination() == plan.source() {
            // Reusing an unchanged stack needs no engine command. Prove the
            // complete membership; a subset must use Create + Transfers.
            let actual = self.list(source + 0x38, 1024)?;
            return (actual.len() == registers.len()
                && actual.iter().all(|r| registers.contains(r)))
            .then_some(batch);
        }
        let mem = self.allocate(0x60)?;
        let a = plan.create_arguments(mem);
        type Create =
            unsafe extern "system" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
        let command = call(|| unsafe {
            std::mem::transmute::<usize, Create>((self.base + 0x17DB30) as usize)(
                a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9],
            )
        })?;
        if self.u32(command) != Some(self.base + 0xA00D34) {
            return None;
        }
        batch.commands.push(command);
        for reg in registers {
            let mem = self.allocate(0x6C)?;
            type Transfer = unsafe extern "system" fn(u32, u32, u32, u32) -> u32;
            let id = plan.destination();
            let command = call(|| unsafe {
                std::mem::transmute::<usize, Transfer>((self.base + 0x1E1EE0) as usize)(
                    mem, id.kind, id.id, reg,
                )
            })?;
            if self.u32(command) != Some(self.base + 0xA03554) {
                return None;
            }
            batch.commands.push(command);
        }
        Some(batch)
    }
    pub fn prepare_move(
        &self,
        permit: SinglePlayerPermit,
        army: Identity,
        donor: u32,
        destination: u32,
    ) -> Option<Batch> {
        if permit.epoch() == 0 || destination == 0 || destination > 65535 {
            return None;
        }
        let stack = self.army(army)?;
        if self.field(stack, 0xC4) != Some(donor)
            || self.field(stack, 0xCC) != Some(donor)
            || self.field(stack, 0x74) != Some(0)
            || self.field(stack, 0xEC) != Some(0)
        {
            return None;
        }
        let queue = self.queue()?;
        if self.land_route(stack, destination, None) != Some(true) {
            return None;
        }
        let mem = self.allocate(0x90)?;
        type Move = unsafe extern "system" fn(u32, u32, u32, u32, u32, u32, u32) -> u32;
        let command = call(|| unsafe {
            std::mem::transmute::<usize, Move>((self.base + 0x1DE7D0) as usize)(
                mem,
                stack,
                destination,
                1,
                1,
                1,
                0,
            )
        })?;
        if self.u32(command) != Some(self.base + 0xA03394) {
            return None;
        }
        Some(Batch {
            native: *self,
            queue,
            commands: vec![command],
        })
    }
    fn allocate(&self, size: u32) -> Option<u32> {
        type New = unsafe extern "C" fn(u32) -> u32;
        let p = call(|| unsafe {
            std::mem::transmute::<usize, New>((self.base + 0x6AE9AF) as usize)(size)
        })?;
        (p != 0).then_some(p)
    }
    fn queue(&self) -> Option<u32> {
        let session = crate::native_session::capture(self, self.base)?;
        if session.mode != SessionMode::SinglePlayer
            || !self.expected_slot(session.queue, 6, 0x668560)
        {
            return None;
        }
        Some(session.queue)
    }
    pub fn local_queue_ready(&self) -> bool {
        self.queue().is_some()
    }
    pub fn receipt(
        &self,
        campaign: crate::mission::Campaign,
        id: Identity,
    ) -> Option<crate::mission::StackReceipt> {
        capture_stack_receipt(self, self.base, campaign, self.army(id)?, 64)
    }
}

pub struct Batch {
    native: Native,
    queue: u32,
    commands: Vec<u32>,
}
impl Batch {
    /// The queue takes ownership of every posted command. Create and Transfers
    /// are posted together, never delayed across days while the target is empty.
    pub fn post(mut self) -> bool {
        // A permit captured earlier cannot cross a session replacement.
        if self.native.queue() != Some(self.queue) {
            return false;
        }
        let Some(post) = self
            .native
            .slot(self.queue, 6)
            .filter(|p| self.native.in_image(*p))
        else {
            return false;
        };
        // Native validity slot14. Transfer validates its source reference, so
        // it can be checked before the preceding Create reaches execution.
        type Valid = unsafe extern "fastcall" fn(u32, u32) -> u8;
        for command in &self.commands {
            let Some(valid) = self
                .native
                .slot(*command, 14)
                .filter(|p| self.native.in_image(*p))
            else {
                return false;
            };
            if call(|| unsafe { std::mem::transmute::<usize, Valid>(valid as usize)(*command, 0) })
                .unwrap_or(0)
                == 0
            {
                return false;
            }
        }
        type Post = unsafe extern "fastcall" fn(u32, u32, u32);
        while !self.commands.is_empty() {
            // Remove before invoking Post: an exception after ownership passed
            // must never make Drop delete a possibly queued command.
            let command = self.commands.remove(0);
            if call(|| unsafe {
                std::mem::transmute::<usize, Post>(post as usize)(self.queue, 0, command)
            })
            .is_none()
            {
                return false;
            }
        }
        true
    }
}
impl Drop for Batch {
    fn drop(&mut self) {
        for command in self.commands.drain(..) {
            let Some(vt) = self.native.u32(command) else {
                continue;
            };
            if ![0xA00D34, 0xA03554, 0xA03394, 0xA0121C]
                .iter()
                .any(|r| vt == self.native.base + r)
            {
                continue;
            }
            if let Some(delete) = self
                .native
                .slot(command, 0)
                .filter(|p| self.native.in_image(*p))
            {
                type Delete = unsafe extern "fastcall" fn(u32, u32, u32) -> u32;
                let _ = call(|| unsafe {
                    std::mem::transmute::<usize, Delete>(delete as usize)(command, 0, 1)
                });
            }
        }
    }
}

fn cp1252(byte: u8) -> char {
    const EXTRA: [char; 32] = [
        '€', '\u{81}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{8d}', 'Ž',
        '\u{8f}', '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\u{9d}',
        'ž', 'Ÿ',
    ];
    if (0x80..0xA0).contains(&byte) {
        EXTRA[(byte - 0x80) as usize]
    } else {
        char::from(byte)
    }
}
