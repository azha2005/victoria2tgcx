//! Native diplomacy entry and our two independent volunteer windows. Reuses the
//! accepted ATCU factory, observer, placement and deferred-release contracts.
use crate::app::{App, UiAction, Work, WorldView};
use crate::model::Settings;
use crate::native::{GetCurrentThreadId, Native};
use crate::native_call::call;
use crate::native_string::GameString;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

static UI: OnceLock<Mutex<Ui>> = OnceLock::new();
static UPDATE: AtomicUsize = AtomicUsize::new(0);
static HIDE: AtomicUsize = AtomicUsize::new(0);
static DESTROY: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static PENDING_HIDE: AtomicU32 = AtomicU32::new(0);
#[link(name = "bcrypt")]
unsafe extern "system" {
    fn BCryptGenRandom(
        provider: *mut std::ffi::c_void,
        buffer: *mut u8,
        size: u32,
        flags: u32,
    ) -> i32;
}
fn fresh_campaign() -> Option<crate::mission::Campaign> {
    let mut id = [0u8; 16];
    let status = unsafe { BCryptGenRandom(std::ptr::null_mut(), id.as_mut_ptr(), 16, 2) };
    (status >= 0 && id != [0; 16]).then_some(crate::mission::Campaign(id))
}

struct Binding {
    host: u32,
    name: String,
    button: u32,
    _glue: Box<[u8; 44]>,
}
#[derive(Clone, Copy)]
struct WindowLease {
    pointer: u32,
    definition: u32,
    context: u32,
    renderer: u32,
}
#[derive(PartialEq, Eq)]
struct RouteKey {
    epoch: u64,
    player: u32,
    recipient: u32,
    armies: Vec<(crate::model::Identity, u32, bool, bool)>,
}
struct Ui {
    native: Native,
    thread: Option<u32>,
    app: App,
    view: u32,
    root: u32,
    entry: u32,
    chooser: u32,
    forces: u32,
    bindings: Vec<Binding>,
    retired: Vec<Binding>,
    world: WorldView,
    shown: bool,
    owned: Vec<WindowLease>,
    disabled: bool,
    blocked_epoch: Option<u64>,
    text_cache: BTreeMap<(u32, String), String>,
    entry_slot: Option<(i32, i32)>,
    entry_layout: Option<(u32, u32)>,
    entry_layout_missing: bool,
    bound_epoch: u64,
    bound_player: u32,
    store_head: Option<u32>,
    pending_checkpoint: Option<(u32, String)>,
    saved_registry: Option<Vec<u8>>,
    loss_reads: BTreeMap<u64, (u32, u8)>,
    route_key: Option<RouteKey>,
    route_armies: Vec<crate::app::ArmyView>,
    route_chooser: crate::route_chooser::Chooser,
}
impl Ui {
    fn refresh_routes(&mut self) {
        if !self.app.open || self.disabled {
            return;
        }
        if let Err(reason) =
            crate::session::admit(self.world.mode, self.app.capabilities, self.world.epoch)
        {
            self.app.refuse(reason.message());
            return;
        }
        if !crate::native_session::capture(&self.native, self.native.base).is_some_and(|s| {
            s.mode == crate::session::SessionMode::SinglePlayer
                && crate::native_lifecycle::epoch(s.context) == Some(self.world.epoch)
        }) {
            self.world.armies.clear();
            self.app
                .refuse("Waiting for a current single-player session and load epoch.");
            return;
        }
        let recipient = self.app.selected_recipient();
        let key = RouteKey {
            epoch: self.world.epoch,
            player: self.world.player,
            recipient,
            armies: self
                .route_armies
                .iter()
                .map(|a| (a.id, a.province, a.combat, a.moving))
                .collect(),
        };
        if self.route_key.as_ref() != Some(&key) {
            let eligible = self
                .route_armies
                .iter()
                .filter(|a| {
                    a.owner == self.world.player
                        && a.affiliation == self.world.player
                        && !a.combat
                        && !a.moving
                        && !a.regiments.is_empty()
                        && a.regiments.iter().all(|r| r.owner == self.world.player)
                        && self
                            .app
                            .registry
                            .as_ref()
                            .is_some_and(|r| r.live_army(a.id).is_none())
                })
                .collect::<Vec<_>>();
            let destinations = eligible
                .first()
                .and_then(|a| self.native.army(a.id))
                .and_then(|p| self.native.deployment_candidates(p, recipient))
                .unwrap_or_default();
            self.route_chooser =
                crate::route_chooser::Chooser::start(eligible.into_iter().map(|a| {
                    let mut targets = destinations.clone();
                    targets.sort_by_key(|id| u8::from(*id != a.province));
                    (a.id, targets)
                }));
            self.route_key = Some(key);
        }
        // This cache is a UI shortlist only. Send rereads current session,
        // ownership, membership, recipient province and native route below.
        if crate::session::admit(self.world.mode, self.app.capabilities, self.world.epoch).is_ok()
            && !self.world.player_at_war
            && self.world.countries.iter().any(|c| {
                c.index == recipient
                    && c.war == crate::mission::RecipientWar::AtWar
                    && !c.hostile_to_player
            })
        {
            let n = self.native;
            self.route_chooser.advance(8, |id, target| {
                n.army(id)
                    .and_then(|army| n.land_route(army, target, Some(recipient)))
            });
        }
        self.world.armies = self
            .route_armies
            .iter()
            .filter(|a| self.route_chooser.targets.contains_key(&a.id))
            .cloned()
            .collect();
        self.app.reconcile_armies(&self.world);
        if self.world.armies.is_empty() {
            self.app.refuse(if self.route_chooser.pending() != 0 {
                "Checking reachable armies; let a game tick pass."
            } else {
                "No idle army has a verified land route to this recipient."
            });
        }
    }
    fn observe_world(&mut self, mut world: WorldView) {
        self.route_armies = world.armies.clone();
        let prototype = self.app.settings.prototype
            && crate::native_hooks::ready()
            && crate::native_journal::ready()
            && crate::native_lifecycle::ready();
        self.app.capabilities.combat = prototype;
        self.app.capabilities.access = prototype;
        self.app.capabilities.vision = prototype;
        self.app.capabilities.session_detection =
            world.mode != crate::session::SessionMode::Unknown;
        self.app.capabilities.command_queue = self.native.local_queue_ready();
        let epoch = crate::native_session::capture(&self.native, self.native.base)
            .and_then(|s| crate::native_lifecycle::epoch(s.context))
            .unwrap_or(0);
        if self
            .blocked_epoch
            .is_some_and(|blocked| blocked != epoch && epoch != 0)
        {
            self.blocked_epoch = None;
        }
        if self.blocked_epoch == Some(epoch) && epoch != 0 {
            crate::native_hooks::publish(None, None);
            self.app.capabilities = Default::default();
            world.epoch = epoch;
            world.campaign = self.app.registry.as_ref().map(|r| r.campaign);
            self.world = world;
            return;
        }
        if world.mode != crate::session::SessionMode::SinglePlayer || epoch == 0 {
            crate::native_hooks::publish(None, None);
            if let Some(registry) = self.app.registry.as_mut() {
                registry.suspend();
            }
            self.app.capabilities.campaign_binding = false;
            self.app.capabilities.persistence = false;
            world.epoch = 0;
            world.campaign = None;
            self.world = world;
            return;
        } else if self.bound_epoch != epoch
            || self.bound_player != world.player
            || self.app.registry.is_none()
            || self.app.registry.as_ref().is_some_and(|r| r.epoch != epoch)
        {
            self.app.registry = None;
            self.app.capabilities.campaign_binding = false;
            self.app.capabilities.persistence = false;
            self.bound_epoch = epoch;
            self.bound_player = world.player;
            self.store_head = None;
            self.pending_checkpoint = None;
            self.saved_registry = None;
            self.loss_reads.clear();
            match crate::engine_store::load(epoch, |key| self.native.variable(world.player, key)) {
                Ok(Some(loaded)) => {
                    if loaded
                        .registry
                        .entries()
                        .any(|e| e.deployment.key.donor != world.player)
                    {
                        crate::runtime::diagnostic(
                            "Volunteer journal donor does not match its country; refused",
                        );
                        world.epoch = epoch;
                        world.campaign = None;
                        self.world = world;
                        return;
                    }
                    self.store_head = Some(loaded.head);
                    self.saved_registry = loaded.registry.encode().ok();
                    self.app.bind_registry(loaded.registry);
                    self.app.capabilities.campaign_binding = true;
                    crate::runtime::diagnostic(
                        "Engine-saved volunteer registry loaded suspended; no pending commands replayed",
                    );
                }
                Ok(None) if prototype => {
                    if let Some(campaign) = fresh_campaign() {
                        self.app
                            .bind_registry(crate::registry::Registry::new(campaign, epoch));
                        self.store_head = Some(0);
                        // This ID is local until the first engine-command
                        // checkpoint. Empty preview sessions write no variables.
                        self.saved_registry =
                            self.app.registry.as_ref().and_then(|r| r.encode().ok());
                    }
                }
                Ok(None) => (),
                Err(error) => {
                    let message = format!(
                        "Saved volunteers unavailable: {error:?}; retry after a game tick."
                    );
                    self.app.refuse(&message);
                    crate::runtime::diagnostic(&message);
                }
            }
        }
        world.epoch = epoch;
        world.campaign = self.app.registry.as_ref().map(|r| r.campaign);
        self.app.capabilities.campaign_binding = self
            .app
            .registry
            .as_ref()
            .is_some_and(|r| r.epoch == epoch && self.bound_player == world.player);
        self.app.capabilities.persistence = prototype && self.store_head.is_some();
        if let Ok(permit) = crate::session::admit(world.mode, self.app.capabilities, epoch)
            && let Some(registry) = self.app.registry.as_mut()
        {
            let entries = registry.entries().cloned().collect::<Vec<_>>();
            for entry in entries {
                if entry.mission.state() != crate::model::State::NeedsReview {
                    continue;
                }
                let war = world
                    .countries
                    .iter()
                    .find(|c| c.index == entry.deployment.key.recipient)
                    .map_or(crate::mission::RecipientWar::Unknown, |c| c.war);
                if matches!(
                    entry.mission.saved().0,
                    crate::model::State::Lost | crate::model::State::Returned
                ) {
                    registry.revalidate_terminal_vision(permit, entry.deployment.key.serial, war);
                } else if let Some(receipt) = self
                    .native
                    .receipt(registry.campaign, entry.mission.plan().destination())
                {
                    registry.revalidate(entry.deployment.key.serial, &receipt, war);
                } else {
                    registry.revalidate_terminal_vision(permit, entry.deployment.key.serial, war);
                }
            }
            crate::native_hooks::publish(Some(registry), Some(permit));
        }
        self.world = world;
    }
    fn action_slot(&mut self) -> Option<(i32, i32)> {
        let n = self.native;
        // FUN_00622270 creates action0..N as direct CFixedWindow children.
        // Follow their actual native grid, including V2DLL-added actions.
        if n.u32(self.root) != Some(n.base + 0xA3C0D8) || !n.expected_slot(self.root, 27, 0x5E4A20)
        {
            return None;
        }
        // FUN_009E4A20 warns on missing optional names. Read the bounded
        // subwindow vector instead: +424 begin/end, 32-byte name/value pairs
        // with value at +1C (FUN_009E68A0/FUN_009E6960). No native search call.
        let begin = n.field(self.root, 0x424)?;
        let end = n.field(self.root, 0x428)?;
        let bytes = end.checked_sub(begin)?;
        if bytes == 0 || bytes % 32 != 0 || bytes / 32 > 256 {
            return None;
        }
        if self.entry_layout == Some((begin, end)) && self.entry_slot.is_some() {
            return self.entry_slot;
        }
        self.entry_slot = None;
        let mut cells = Vec::new();
        for offset in (0..bytes).step_by(32) {
            let pair = begin.checked_add(offset)?;
            let name = n.string(pair, 128)?;
            let Some(index) = name
                .strip_prefix("action")
                .and_then(|s| s.parse::<u32>().ok())
            else {
                continue;
            };
            if index >= 64 || name != format!("action{index}") {
                return None;
            }
            let row = n.field(pair, 0x1C)?;
            if n.u32(row) != Some(n.base + 0xA3C0D8) {
                return None;
            }
            let position = n.field(row, 0x250)?;
            let x = position as u16 as i16 as i32;
            let y = (position >> 16) as u16 as i16 as i32;
            if n.field(row, 0x4A0) != Some(160) || n.field(row, 0x4A4) != Some(24) || x < 0 || y < 0
            {
                return None;
            }
            cells.push((x, y));
        }
        let bottom = cells.iter().map(|(_, y)| *y).max()?;
        let left = cells.iter().map(|(x, _)| *x).min()?;
        let right = cells.iter().map(|(x, _)| *x).max()?;
        if right - left != 160 {
            return None;
        }
        let position = if !cells.contains(&(right, bottom)) {
            (right, bottom)
        } else if !cells.contains(&(left, bottom)) {
            (left, bottom)
        } else {
            (left, bottom.checked_add(24)?)
        };
        let width = n.field(self.root, 0x4A0)? as i32;
        let height = n.field(self.root, 0x4A4)? as i32;
        if position.0 + 160 > width || position.1 + 24 > height {
            return None;
        }
        self.entry_slot = Some(position);
        self.entry_layout = Some((begin, end));
        Some(position)
    }
    fn live_window(&self, pointer: u32) -> bool {
        let n = self.native;
        self.owned.iter().any(|lease| {
            lease.pointer == pointer
                && n.u32(pointer) == Some(n.base + 0xA3C0D8)
                && n.field(pointer, 0x94) == Some(lease.definition)
                && n.field(pointer, 0x4C) == Some(lease.context)
                && n.field(pointer, 0x90) == Some(lease.renderer)
                && n.byte(pointer + 0x52) == Some(0)
                && n.u32(lease.context) == Some(n.base + 0xA08A44)
                && n.field(lease.context, 0x30) == Some(lease.renderer)
        })
    }
    fn fail(&mut self, reason: &str) {
        crate::native_hooks::publish(None, None);
        if !self.disabled {
            crate::runtime::diagnostic(reason);
        }
        self.disabled = true;
        if let Some(registry) = self.app.registry.as_mut() {
            registry.suspend();
        }
        self.app.capabilities = Default::default();
        self.close();
    }
    fn stop_missions(&mut self, reason: &str) {
        // A checkpoint failure fences simulation work until an explicit load;
        // it does not imply the independently validated native button is unsafe.
        self.blocked_epoch = Some(self.world.epoch);
        self.app.capabilities = Default::default();
        if let Some(registry) = self.app.registry.as_mut() {
            registry.suspend();
        }
        crate::native_hooks::publish(None, None);
        self.app.refuse(reason);
        crate::runtime::diagnostic(reason);
    }
    fn lookup(&self, registry: u32, key: &str) -> Option<u32> {
        let n = self.native;
        if n.u32(registry) != Some(n.base + 0xA41E84) || !n.expected_slot(registry, 1, 0x241BA0) {
            return None;
        }
        let text = GameString::new(key);
        type Lookup = unsafe extern "fastcall" fn(u32, u32, *const GameString) -> u32;
        let result = call(|| unsafe {
            std::mem::transmute::<usize, Lookup>((n.base + 0x241BA0) as usize)(registry, 0, &text)
        })?;
        (result != 0).then_some(result)
    }
    fn schema(&self, context: u32, key: &str) -> Option<u32> {
        let n = self.native;
        let definition = self.lookup(context.checked_add(0x34)?, key)?;
        if n.u32(definition) != Some(n.base + 0xA3B2CC)
            || !n.expected_slot(definition, 11, 0x5D1040)
        {
            return None;
        }
        let buttons: &[&str] = match key {
            "tgcx_vl_entry" => &["open"],
            "tgcx_vl_chooser" => &[
                "chooser_bg",
                "close",
                "cancel",
                "send",
                "army_prev",
                "army_next",
                "recipient_prev",
                "recipient_next",
                "page_prev",
                "page_next",
                "row0",
                "row1",
            ],
            "tgcx_vl_forces" => &[
                "forces_bg",
                "mission_prev",
                "mission_next",
                "recall",
                "history",
                "clear_history",
            ],
            _ => return None,
        };
        // P59/P60: resolve buttons in their owning CWindowType registry before
        // any factory runs, including non-interactive background buttons.
        for button in buttons {
            let kind = self.lookup(definition.checked_add(0x150)?, &format!("tgcx_vl_{button}"))?;
            if n.u32(kind) != Some(n.base + 0xA3CFBC) || !n.expected_slot(kind, 11, 0x5F0310) {
                return None;
            }
        }
        Some(definition)
    }
    fn find(&self, host: u32, index: u32, expected: u32, name: &str) -> Option<u32> {
        if !self.live_window(host) || !self.native.expected_slot(host, index, expected) {
            return None;
        }
        let text = GameString::new(name);
        type Find = unsafe extern "fastcall" fn(u32, u32, *const GameString) -> u32;
        let result = call(|| unsafe {
            std::mem::transmute::<usize, Find>((self.native.base + expected) as usize)(
                host, 0, &text,
            )
        })?;
        (result != 0).then_some(result)
    }
    fn make(&mut self, host: u32, key: &str) -> Option<u32> {
        let n = self.native;
        if self.disabled || self.bindings.len() + self.retired.len() + 17 > 256 {
            self.fail(
                "Volunteer UI construction refused: disabled or observer capacity reached (P47)",
            );
            return None;
        }
        let context = n.field(host, 0x4C)?;
        if n.u32(context) != Some(n.base + 0xA08A44)
            || n.field(context, 0x30) != n.field(host, 0x90)
        {
            return None;
        }
        let Some(definition) = self.schema(context, key) else {
            self.fail(
                "Volunteer UI definition/type preflight refused before construction (P59/P60)",
            );
            return None;
        };
        let name = GameString::new(key);
        if !n.expected_slot(context, 11, 0x5A38F0) {
            return None;
        }
        type Make = unsafe extern "fastcall" fn(u32, u32, *const GameString, u32) -> u32;
        let window = call(|| unsafe {
            std::mem::transmute::<usize, Make>((n.base + 0x5A38F0) as usize)(context, 0, &name, 0)
        })?;
        if n.u32(window) != Some(n.base + 0xA3C0D8)
            || n.field(window, 0x94) != Some(definition)
            || n.field(window, 0x4C) != Some(context)
        {
            self.fail("Volunteer UI factory returned an unrecognized root; no deletion or retry");
            return None;
        }
        let renderer = n.field(host, 0x90)?;
        self.owned.push(WindowLease {
            pointer: window,
            definition,
            context,
            renderer,
        });
        self.visible(window, false);
        Some(window)
    }
    fn visible(&self, object: u32, show: bool) {
        if object == 0 || !self.live_window(object) {
            return;
        }
        let n = self.native;
        type Zero = unsafe extern "fastcall" fn(u32, u32);
        if show && n.byte(object + 0x67) == Some(0) {
            if n.expected_slot(object + 0x18, 13, 0x5E4490) {
                let _ = call(|| unsafe {
                    std::mem::transmute::<usize, Zero>((n.base + 0x5E4490) as usize)(
                        object + 0x18,
                        0,
                    )
                });
            }
        } else if !show && n.byte(object + 0x67) != Some(0) && n.expected_slot(object, 9, 0x5E4030)
        {
            let _ = call(|| unsafe {
                std::mem::transmute::<usize, Zero>((n.base + 0x5E4030) as usize)(object, 0)
            });
        }
    }
    fn repair_entry(&mut self, x: i32, y: i32) {
        let n = self.native;
        let covered =
            crate::entry_visibility::covered_by_host(&n, n.base + 0xA38350, self.root, self.entry)
                == Some(true);
        let child_hidden = self
            .find(self.entry, 13, 0x5E4C80, "tgcx_vl_open")
            .is_some_and(|button| {
                n.expected_slot(button, 13, 0x5EFEF0)
                    && n.expected_slot(button, 14, 0x5EFEB0)
                    && n.byte(button + 0x4F) == Some(0)
                    // Native Show itself refuses +6C != 0. Wait for that gate
                    // rather than retrying/flush-logging a refused Show each poll.
                    && n.byte(button + 0x6C) == Some(0)
            });
        self.position(self.entry, x, y, covered);
        if (covered || child_hidden)
            && n.byte(self.entry + 0x67) == Some(1)
            && self.live_window(self.entry)
            && n.expected_slot(self.entry + 0x18, 13, 0x5E4490)
        {
            type Show = unsafe extern "fastcall" fn(u32, u32);
            let _ = call(|| unsafe {
                std::mem::transmute::<usize, Show>((n.base + 0x5E4490) as usize)(
                    self.entry + 0x18,
                    0,
                )
            });
            crate::runtime::diagnostic(&format!(
                "volunteer_entry_repaired covered={covered} child_hidden={child_hidden}"
            ));
        }
        self.visible(self.entry, true);
    }
    fn forget_expired_windows(&mut self) {
        for expired in [self.entry, self.chooser, self.forces] {
            if expired == 0 || self.live_window(expired) {
                continue;
            }
            let mut keep = Vec::new();
            for binding in self.bindings.drain(..) {
                if binding.host == expired {
                    self.retired.push(binding);
                } else {
                    keep.push(binding);
                }
            }
            self.bindings = keep;
            self.owned.retain(|lease| lease.pointer != expired);
            self.text_cache.retain(|(host, _), _| *host != expired);
            if self.entry == expired {
                self.entry = 0;
                self.entry_layout = None;
                self.entry_slot = None;
            }
            if self.chooser == expired {
                self.chooser = 0;
                self.shown = false;
            }
            if self.forces == expired {
                self.forces = 0;
                self.shown = false;
            }
            crate::runtime::diagnostic(&format!(
                "Volunteer window lease expired window={expired:08X}; rebuilding on UI thread, old observers retained"
            ));
        }
    }
    fn position(&self, window: u32, x: i32, y: i32, promote: bool) {
        let n = self.native;
        if !self.live_window(window)
            || !n.expected_slot(window, 6, 0x5E3BA0)
            || !n.expected_slot(window, 11, 0x5E2FC0)
            || !(0..=32767).contains(&x)
            || !(0..=32767).contains(&y)
        {
            return;
        }
        type Zero = unsafe extern "fastcall" fn(u32, u32);
        type Set = unsafe extern "fastcall" fn(u32, u32, u32);
        if promote || n.byte(window + 0x67) == Some(0) {
            let _ = call(|| unsafe {
                std::mem::transmute::<usize, Zero>((n.base + 0x5E2FC0) as usize)(window, 0)
            });
        }
        let packed = x as u32 | ((y as u32) << 16);
        // Native position is read afresh; no cached layout/visibility authority.
        if n.field(window, 0x250) == Some(packed) {
            return;
        }
        let _ = call(|| unsafe {
            std::mem::transmute::<usize, Set>((n.base + 0x5E3BA0) as usize)(window, 0, packed)
        });
    }
    fn screen(&self) -> Option<(f32, f32, i32, i32)> {
        let n = self.native;
        let renderer = n.field(self.root, 0x90)?;
        let mut inner = n.field(self.root, 0x494)?;
        if n.u32(inner) != Some(n.base + 0xA38350) {
            return None;
        }
        let (mut x, mut y) = (0.0, 0.0);
        for _ in 0..8 {
            if !n.u32(inner).is_some_and(|vt| n.in_image(vt)) {
                return None;
            }
            x += f32::from_bits(n.field(inner, 0x84)?);
            y -= f32::from_bits(n.field(inner, 0x88)?);
            let owner = n.field(inner, 0xE4)?;
            if owner == renderer + 0x6B9E0 {
                let width = n.field(renderer, 0x6B9BC)? as i32;
                let height = n.field(renderer, 0x6B9C0)? as i32;
                return (x.is_finite()
                    && y.is_finite()
                    && x.abs() <= 32767.0
                    && y.abs() <= 32767.0
                    && (686..=32767).contains(&width)
                    && (478..=32767).contains(&height))
                .then_some((x, y, width, height));
            }
            if owner == 0 {
                return None;
            }
            inner = owner;
        }
        None
    }
    fn text(&mut self, host: u32, key: &str, value: &str) {
        let cache_key = (host, key.to_owned());
        if self
            .text_cache
            .get(&cache_key)
            .is_some_and(|old| old == value)
        {
            return;
        }
        let Some(textbox) = self.find(host, 15, 0x5E4D00, key) else {
            return;
        };
        let n = self.native;
        let Some(inner) = n
            .field(textbox, 0xA0)
            .filter(|p| n.u32(*p) == Some(n.base + 0xA41F10))
        else {
            return;
        };
        let text = GameString::new(value);
        type Assign = unsafe extern "fastcall" fn(u32, u32, *const GameString, u32, u32);
        if call(|| unsafe {
            std::mem::transmute::<usize, Assign>((n.base + 0x8ED0) as usize)(
                textbox + 0xDC,
                0,
                &text,
                0,
                u32::MAX,
            )
        })
        .is_none()
        {
            return;
        }
        if call(|| unsafe {
            std::arch::asm!("push esi","push 0","push ecx","mov esi, edx","call eax","pop esi",
            in("ecx") &text,in("edx") inner,in("eax") n.base+0x65ECC0,clobber_abi("C"));
        })
        .is_some()
        {
            self.text_cache.insert(cache_key, value.to_owned());
        }
    }
    fn bind(&mut self, host: u32, name: &str, callback: usize) {
        // Our immutable factory subtree is retired as a unit. An existing
        // binding needs no native name search or second AddObserver every frame.
        if self.live_window(host)
            && self
                .bindings
                .iter()
                .any(|b| b.host == host && b.name == name)
        {
            return;
        }
        let Some(button) = self.find(host, 13, 0x5E4C80, name) else {
            return;
        };
        if self.bindings.iter().any(|b| b.button == button) {
            return;
        }
        if self.bindings.len() + self.retired.len() >= 256 {
            self.fail("Volunteer UI binding capacity reached; UI disabled (P47)");
            return;
        }
        let n = self.native;
        let Some(add) = n.slot(button + 0x54, 1).filter(|p| n.in_image(*p)) else {
            return;
        };
        // from V2DLL: V2TechButton.cpp, proved Politics 44-byte observer template.
        // Its deletion notification is +0x20, left zero. Never use A0A58C here.
        let mut glue = Box::new([0u8; 44]);
        glue[0..4].copy_from_slice(&(n.base + 0xA0E48C).to_le_bytes());
        glue[4..8].copy_from_slice(&self.view.to_le_bytes());
        glue[8..12].copy_from_slice(&(callback as u32).to_le_bytes());
        type Add = unsafe extern "fastcall" fn(u32, u32, *const u8);
        // Retain even if AddObserver faults after registering the pointer.
        let pointer = glue.as_ptr();
        self.bindings.push(Binding {
            host,
            name: name.to_owned(),
            button,
            _glue: glue,
        });
        if call(|| unsafe {
            std::mem::transmute::<usize, Add>(add as usize)(button + 0x54, 0, pointer)
        })
        .is_none()
        {
            self.fail("Volunteer native observer call faulted; UI disabled");
        }
    }
    fn close(&mut self) {
        self.app.open = false;
        self.visible(self.chooser, false);
        self.visible(self.forces, false);
        self.shown = false;
    }
    fn retire(&mut self, touch_native: bool) {
        // A load/teardown callback may arrive on a different thread. Forget
        // cached addresses there; native GUI calls belong to the captured UI thread.
        let touch_native = touch_native && self.thread == Some(unsafe { GetCurrentThreadId() });
        if touch_native {
            self.close();
        } else {
            self.app.open = false;
            self.shown = false;
        }
        let n = self.native;
        for window in [self.entry, self.chooser, self.forces] {
            if touch_native {
                self.visible(window, false);
            }
            if touch_native && self.live_window(window) && n.expected_slot(window, 20, 0x5E3D60) {
                type Release = unsafe extern "fastcall" fn(u32, u32);
                let _ = call(|| unsafe {
                    std::mem::transmute::<usize, Release>((n.base + 0x5E3D60) as usize)(window, 0)
                });
            }
        }
        // Native deferred collection may still reference observers. Retain their
        // boxes for the process lifetime, bounded by the 256-binding ceiling.
        self.retired.append(&mut self.bindings);
        self.entry = 0;
        self.chooser = 0;
        self.forces = 0;
        self.view = 0;
        self.root = 0;
        self.owned.clear();
        self.text_cache.clear();
        self.entry_slot = None;
        self.entry_layout = None;
        self.entry_layout_missing = false;
    }
    fn refresh(&mut self, view: u32) {
        if !READY.load(Ordering::Acquire) {
            return;
        }
        let n = self.native;
        let tid = unsafe { GetCurrentThreadId() };
        if self.thread.is_some_and(|t| t != tid) || n.u32(view) != Some(n.base + 0xA07450) {
            return;
        }
        self.thread.get_or_insert(tid);
        if PENDING_HIDE
            .compare_exchange(view, 0, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.close();
            self.visible(self.entry, false);
            self.entry_layout = None;
            self.entry_slot = None;
        }
        let Some(root) = n
            .field(view, 0x4C)
            .filter(|p| n.u32(*p) == Some(n.base + 0xA3C0D8))
        else {
            self.close();
            return;
        };
        if self.view != view || self.root != root {
            let old_valid = n.u32(self.view) == Some(n.base + 0xA07450)
                && n.field(self.view, 0x4C) == Some(self.root)
                && n.u32(self.root) == Some(n.base + 0xA3C0D8);
            self.retire(old_valid);
            self.disabled = false;
            self.view = view;
            self.root = root;
        }
        if crate::native_call::faulted() {
            self.fail("Volunteer native access violation; further plugin native calls disabled");
            return;
        }
        if self.disabled {
            self.close();
            self.visible(self.entry, false);
            return;
        }
        if n.byte(root + 0x67) != Some(1) {
            self.close();
            self.visible(self.entry, false);
            self.entry_layout = None;
            self.entry_slot = None;
            return;
        }
        self.forget_expired_windows();
        if self.entry == 0 {
            self.entry = self.make(root, "tgcx_vl_entry").unwrap_or(0);
        }
        if self.entry == 0 {
            return;
        }
        self.bind(self.entry, "tgcx_vl_open", open as *const () as usize);
        if let Some((x, y, width, height)) = self.screen() {
            let Some((button_x, button_y)) = self.action_slot() else {
                self.close();
                self.visible(self.entry, false);
                if !self.entry_layout_missing {
                    crate::runtime::diagnostic(
                        "Volunteer entry waiting: native diplomacy action grid unavailable",
                    );
                    self.entry_layout_missing = true;
                }
                return;
            };
            let entry_x = x as i32 + button_x;
            let entry_y = y as i32 + button_y;
            if entry_x < 0 || entry_y < 0 || entry_x + 160 > width || entry_y + 24 > height {
                self.close();
                self.visible(self.entry, false);
                return;
            }
            self.repair_entry(entry_x, entry_y);
            self.text(self.entry, "tgcx_vl_open_label", "Send volunteers");
            self.visible(self.entry, true);
        } else {
            self.close();
            self.visible(self.entry, false);
            return;
        }
        if self.app.open {
            self.refresh_routes();
            self.render();
        }
    }
    fn render(&mut self) {
        if !self.app.open || self.disabled {
            return;
        }
        if self.chooser == 0 {
            self.chooser = self.make(self.root, "tgcx_vl_chooser").unwrap_or(0);
        }
        if self.forces == 0 {
            self.forces = self.make(self.root, "tgcx_vl_forces").unwrap_or(0);
        }
        if self.chooser == 0 || self.forces == 0 {
            return;
        }
        let Some((host_x, host_y, width, height)) = self.screen() else {
            self.close();
            return;
        };
        // Native runtime host width, not CWindowType position fields (P61).
        // Dock the pair flush against Diplomacy and against each other.
        let Some(host_width) = self
            .native
            .field(self.root, 0x4A0)
            .filter(|w| (332..=32767).contains(w))
        else {
            self.close();
            return;
        };
        let right = host_x as i32 + host_width as i32;
        let x = if right + 664 <= width {
            right
        } else if host_x as i32 >= 664 {
            host_x as i32 - 664
        } else {
            (host_x as i32 + (host_width as i32 - 664) / 2).clamp(0, width - 664)
        };
        let y = (host_y as i32).clamp(80, height - 378);
        self.position(self.chooser, x, y, !self.shown);
        self.position(self.forces, x + 332, y, !self.shown);
        self.visible(self.chooser, true);
        self.visible(self.forces, true);
        self.shown = true;
        self.visible(self.entry, true);
        for (name, callback) in CHOOSER_BUTTONS {
            self.bind(self.chooser, name, *callback as *const () as usize);
        }
        for (name, callback) in FORCE_BUTTONS {
            self.bind(self.forces, name, *callback as *const () as usize);
        }
        // The user's live screenshot showed unresolved localisation keys and
        // blank native buttonText. Match Conform: explicit, click-through native
        // textboxes over plain sprite buttons, all strings set through the game.
        for (key, value) in [
            ("title", "Send volunteers"),
            ("recipient_label", "Recipient"),
            (
                "rule_command",
                "You own, command and supply the volunteers.",
            ),
            (
                "rule_war",
                "Fight for the recipient while you stay at peace.",
            ),
            (
                "rule_flag",
                "Your flag and military statistics are retained.",
            ),
            ("cancel_label", "Cancel"),
            ("send_label", "Send volunteers"),
        ] {
            self.text(self.chooser, &format!("tgcx_vl_{key}"), value);
        }
        self.text(self.forces, "tgcx_vl_forces_title", "Volunteer forces");
        self.text(self.forces, "tgcx_vl_recall_label", "Recall selected");
        self.text(self.forces, "tgcx_vl_clear_history_label", "Clear history");
        let p = self.app.presentation(&self.world);
        self.text(self.forces, "tgcx_vl_history_label", &p.history_label);
        for (key, value) in [
            ("donor", format!("From: {}", p.donor)),
            ("recipient", p.recipient),
            ("army", p.army),
            ("selected", p.selected),
            ("status", p.status),
        ] {
            self.text(self.chooser, &format!("tgcx_vl_{key}"), &value);
        }
        for i in 0..2 {
            let value = p
                .rows
                .get(i)
                .map(|r| format!("[{}] {}", if r.checked { "x" } else { " " }, r.text))
                .unwrap_or_else(|| "--".into());
            self.text(self.chooser, &format!("tgcx_vl_row{i}_label"), &value);
        }
        for i in 0..2 {
            self.text(
                self.forces,
                &format!("tgcx_vl_mission{i}"),
                p.missions.get(i).map(String::as_str).unwrap_or(if i == 0 {
                    "No active volunteers"
                } else {
                    ""
                }),
            );
        }
        self.text(self.forces, "tgcx_vl_mission_page", &p.mission_page);
    }
    fn action(&mut self, action: u32) -> Vec<Work> {
        if !READY.load(Ordering::Acquire)
            || self.disabled
            || crate::native_call::faulted()
            || self.thread != Some(unsafe { GetCurrentThreadId() })
            || self.native.u32(self.view) != Some(self.native.base + 0xA07450)
            || self.native.field(self.view, 0x4C) != Some(self.root)
            || self.native.byte(self.root + 0x67) != Some(1)
        {
            return Vec::new();
        }
        let Some(world) = self.native.world() else {
            return Vec::new();
        };
        self.observe_world(world);
        self.refresh_routes();
        if matches!(action, 3 | 14 | 16) && self.blocked_epoch == Some(self.world.epoch) {
            self.app
                .refuse("Mission commands paused after a checkpoint failure. Reload to resume.");
            self.render();
            return Vec::new();
        }
        if matches!(action, 3 | 14 | 16) && self.pending_checkpoint.is_some() {
            self.app
                .refuse("Waiting for the previous engine checkpoint.");
            self.render();
            return Vec::new();
        }
        let action = match action {
            0 => UiAction::Open(self.native.field(self.view, 0x38C).unwrap_or(0)),
            1 | 2 => UiAction::Close,
            3 => UiAction::Send,
            4 => UiAction::PreviousArmy,
            5 => UiAction::NextArmy,
            6 => UiAction::PreviousRecipient,
            7 => UiAction::NextRecipient,
            8 => UiAction::PreviousPage,
            9 => UiAction::NextPage,
            10 => UiAction::ToggleRow(0),
            11 => UiAction::ToggleRow(1),
            12 => UiAction::PreviousMission,
            13 => UiAction::NextMission,
            14 => UiAction::Recall,
            15 => UiAction::ToggleHistory,
            16 => UiAction::ClearHistory,
            _ => return Vec::new(),
        };
        if matches!(action, UiAction::Open(_)) {
            self.route_key = None;
        }
        let new_id = if matches!(action, UiAction::Send) && self.app.can_submit(&self.world) {
            let route = self.app.request(&self.world).and_then(|request| {
                let army = self.native.army(request.source)?;
                let target = *self.route_chooser.targets.get(&request.source)?;
                let recipient = self.native.country(request.recipient)?;
                let candidates = self.native.deployment_candidates(army, request.recipient)?;
                if self.native.byte(recipient + 0x67C)? == 0 || !candidates.contains(&target) {
                    return None;
                }
                if let Some(country) = self
                    .world
                    .countries
                    .iter_mut()
                    .find(|c| c.index == request.recipient)
                {
                    country.landing_province = Some(target);
                }
                self.native
                    .land_route(army, target, Some(request.recipient))
            });
            if route != Some(true) {
                self.route_key = None;
                self.app
                    .refuse("Route changed or is unavailable; choose a reachable idle army.");
                self.render();
                return Vec::new();
            }
            if self.app.whole_army_selection(&self.world) {
                None
            } else {
                crate::session::admit(self.world.mode, self.app.capabilities, self.world.epoch)
                    .ok()
                    .and_then(|p| self.native.allocate_identity(p))
            }
        } else {
            None
        };
        let work = self.app.action(action, &self.world, new_id);
        if self.blocked_epoch == Some(self.world.epoch) {
            self.app
                .refuse("Mission commands paused after a checkpoint failure. Reload to resume.");
        }
        if self.app.open {
            self.refresh_routes();
            self.render();
        } else {
            self.close();
        }
        work
    }
}

extern "C" fn clicked(action: u32) {
    let _error = crate::native::LastError::preserve();
    let Some(lock) = UI.get() else {
        return;
    };
    let (native, work, permit) = {
        let Ok(mut ui) = lock.try_lock() else {
            return;
        };
        let work = ui.action(action);
        let permit = crate::session::admit(ui.world.mode, ui.app.capabilities, ui.world.epoch).ok();
        (ui.native, work, permit)
    };
    let Some(permit) = permit else {
        return;
    };
    execute(native, work, permit);
}
fn execute(native: Native, work: Vec<Work>, permit: crate::session::SinglePlayerPermit) {
    let Some(lock) = UI.get() else {
        return;
    };
    // Commit intent through engine commands before the first unit command.
    // A load between these operations restores pending intent suspended; it
    // never manufactures a second army by replaying Create.
    if !sync_checkpoint(native, permit) {
        return;
    }
    for work in work {
        match work {
            Work::Detach { serial, plan } => {
                let posted = native
                    .prepare_detach(permit, &plan)
                    .is_some_and(|batch| batch.post());
                crate::runtime::diagnostic(&format!(
                    "deployment_post serial={serial} source={:?} destination={:?} reuse={} brigades={} queued={posted}",
                    plan.source(),
                    plan.destination(),
                    plan.source() == plan.destination(),
                    plan.selected_regiments().len()
                ));
                if let Ok(mut ui) = lock.try_lock()
                    && let Some(r) = ui.app.registry.as_mut()
                {
                    r.deployment_posted(serial, posted);
                }
            }
            Work::Move {
                serial,
                donor,
                army,
                destination,
                returning,
            } => {
                let destination = if returning {
                    native
                        .army(army)
                        .and_then(|p| native.return_destination(p, donor, destination))
                } else {
                    Some(destination)
                };
                let posted = destination
                    .and_then(|destination| native.prepare_move(permit, army, donor, destination))
                    .is_some_and(|batch| batch.post());
                crate::runtime::diagnostic(&format!(
                    "movement_post serial={serial} army={army:?} destination={destination:?} returning={returning} queued={posted}"
                ));
                if let Ok(mut ui) = lock.try_lock()
                    && let Some(r) = ui.app.registry.as_mut()
                {
                    if returning {
                        r.return_posted(serial, true, posted);
                        if !posted {
                            r.command_refused(serial);
                        }
                    } else {
                        r.outward_posted(serial, posted);
                    }
                }
            }
        }
    }
    sync_checkpoint(native, permit);
}
fn sync_checkpoint(native: Native, permit: crate::session::SinglePlayerPermit) -> bool {
    let Some(lock) = UI.get() else { return false };
    let candidate = {
        let Ok(ui) = lock.try_lock() else {
            return false;
        };
        if ui.world.epoch != permit.epoch()
            || ui.disabled
            || ui.blocked_epoch == Some(permit.epoch())
        {
            return false;
        }
        let Some(registry) = ui.app.registry.as_ref() else {
            return false;
        };
        let Ok(bytes) = registry.encode() else {
            return false;
        };
        if ui.saved_registry.as_ref() == Some(&bytes) {
            return true;
        }
        if ui.pending_checkpoint.is_some() {
            return false;
        }
        (
            ui.world.player,
            ui.store_head.unwrap_or(0),
            registry.clone(),
            bytes,
        )
    };
    let Some((batch, head, digest)) =
        native.prepare_checkpoint(permit, candidate.0, &candidate.2, candidate.1)
    else {
        if let Ok(mut ui) = lock.try_lock() {
            ui.stop_missions(
                "Volunteer checkpoint preparation refused; reload before further mission commands",
            );
        }
        return false;
    };
    {
        let Ok(mut ui) = lock.try_lock() else {
            return false;
        };
        if ui.world.epoch != permit.epoch()
            || ui.world.player != candidate.0
            || ui.pending_checkpoint.is_some()
            || ui
                .app
                .registry
                .as_ref()
                .and_then(|r| r.encode().ok())
                .as_ref()
                != Some(&candidate.3)
        {
            return false;
        }
        ui.pending_checkpoint = Some((head, digest));
        ui.saved_registry = Some(candidate.3);
    }
    if !batch.post() {
        if let Ok(mut ui) = lock.try_lock() {
            ui.pending_checkpoint = None;
            ui.saved_registry = None;
            ui.stop_missions("Volunteer checkpoint queue failed or partial; reload/review required, no automatic retry");
        }
        return false;
    }
    true
}
extern "C" fn updated(view: u32) {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::GUI.begin();
    if let Some(lock) = UI.get()
        && let Ok(mut ui) = lock.try_lock()
    {
        ui.refresh(view);
    }
}
extern "C" fn hidden(view: u32) {
    let _error = crate::native::LastError::preserve();
    // A worker/reentrant hide is deferred, never a permanent UI failure. Only
    // the captured UI thread touches native windows; real fail() gates remain.
    PENDING_HIDE.store(view, Ordering::Release);
    if let Some(lock) = UI.get()
        && let Ok(mut ui) = lock.try_lock()
        && ui.view == view
    {
        if ui.thread == Some(unsafe { GetCurrentThreadId() }) {
            ui.close();
            ui.visible(ui.entry, false);
            let _ = PENDING_HIDE.compare_exchange(view, 0, Ordering::AcqRel, Ordering::Acquire);
            ui.entry_layout = None;
            ui.entry_slot = None;
        } else {
            ui.app.open = false;
            crate::runtime::diagnostic(
                "Volunteer host-hide observed off the UI thread; GUI calls deferred",
            );
        }
    }
}
extern "C" fn destroyed(view: u32) {
    let _error = crate::native::LastError::preserve();
    if let Some(lock) = UI.get()
        && let Ok(mut ui) = lock.try_lock()
        && ui.view == view
    {
        ui.retire(true);
    }
}

pub fn day_tick() {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::DAY.begin();
    if !READY.load(Ordering::Acquire) || crate::native_call::faulted() {
        return;
    }
    let Some(lock) = UI.get() else {
        return;
    };
    let (native, work, permit) = {
        let Ok(mut ui) = lock.try_lock() else {
            return;
        };
        if ui.thread != Some(unsafe { GetCurrentThreadId() }) {
            return;
        }
        let n = ui.native;
        // A load/player change must see every recipient of the newly loaded
        // journal, not just countries from the previous local registry.
        let same_binding = n.u32(n.base + 0xE588E8).is_some_and(|world| {
            n.field(world, 0xB60) == Some(ui.bound_player)
                && n.field(world, 0xB24)
                    .and_then(crate::native_lifecycle::epoch)
                    == Some(ui.bound_epoch)
                && ui
                    .app
                    .registry
                    .as_ref()
                    .is_some_and(|r| r.epoch == ui.bound_epoch && r.epoch != 0)
        });
        let world = if !ui.app.open && same_binding {
            ui.app
                .registry
                .as_ref()
                .and_then(|registry| n.mission_world(registry))
        } else {
            None
        };
        let Some(world) = world.or_else(|| n.world()) else {
            return;
        };
        ui.observe_world(world.clone());
        let world = ui.world.clone();
        if ui.app.registry.is_none() {
            return;
        }
        let Some(campaign) = world.campaign else {
            return;
        };
        let Ok(permit) = crate::session::admit(world.mode, ui.app.capabilities, world.epoch) else {
            crate::native_hooks::publish(None, None);
            return;
        };
        let entries = ui
            .app
            .registry
            .as_ref()
            .map(|r| r.entries().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        for entry in entries {
            let serial = entry.deployment.key.serial;
            if !matches!(
                entry.mission.saved().0,
                crate::model::State::Serving
                    | crate::model::State::ReturnPending
                    | crate::model::State::ReturnReady
                    | crate::model::State::AwaitingReturn
            ) {
                continue;
            }
            let absent = n.definitely_absent(entry.mission.plan().destination())
                && entry
                    .mission
                    .remaining()
                    .iter()
                    .all(|id| n.definitely_absent(*id));
            if !absent {
                ui.loss_reads.remove(&serial);
                continue;
            }
            let sample = ui.loss_reads.entry(serial).or_insert((world.date, 1));
            if sample.0 != world.date {
                sample.0 = world.date;
                sample.1 = sample.1.saturating_add(1);
            }
            if sample.1 >= 3
                && let Some(registry) = ui.app.registry.as_mut()
            {
                registry.destroyed(serial);
            }
        }
        let work = ui.app.day(&world, |id| {
            let receipt = n.receipt(campaign, id)?;
            let army = n.army(id)?;
            let province = n.field(army, 0xDC)?;
            Some((
                receipt,
                n.field(province, 0x58)?,
                n.field(province, 0x12C)?,
                n.field(army, 0x74)? != 0 || n.field(army, 0xEC)? != 0,
            ))
        });
        ui.world = world;
        ui.refresh_routes();
        crate::native_hooks::publish(ui.app.registry.as_ref(), Some(permit));
        (n, work, permit)
    };
    execute(native, work, permit);
}

pub fn checkpoint_changed(country: u32, head: u32, digest: &str) {
    if let Some(lock) = UI.get()
        && let Ok(mut ui) = lock.try_lock()
        && ui.bound_player == country
    {
        if ui
            .pending_checkpoint
            .as_ref()
            .is_some_and(|p| p.0 == head && p.1 == digest)
        {
            ui.store_head = Some(head);
            ui.pending_checkpoint = None;
            return;
        }
        if let Some(registry) = ui.app.registry.as_mut() {
            registry.suspend();
        }
        crate::native_hooks::publish(None, None);
        ui.bound_epoch = 0;
        ui.app.capabilities.campaign_binding = false;
        ui.app.capabilities.persistence = false;
    }
}

// from V2DLL: V2TechButton.cpp, PUSHAD observer pattern; flags and FXSAVE added.
#[unsafe(naked)]
unsafe extern "C" fn common() {
    std::arch::naked_asm!(
    "pushfd","pushad","mov ebx, esp","sub esp, 528","and esp, -16","fxsave [esp]","fninit","mov dword ptr [esp+512], 0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+36]","call {click}","add esp, 4","fxrstor [esp]","mov esp, ebx","popad","popfd","lea esp, [esp+4]","ret",click=sym clicked);
}
macro_rules! button{($name:ident,$index:literal)=>{#[unsafe(naked)]unsafe extern "C" fn $name(){std::arch::naked_asm!(concat!("push ",stringify!($index)),"jmp {common}",common=sym common);}};}
button!(open, 0);
button!(close, 1);
button!(cancel, 2);
button!(send, 3);
button!(army_prev, 4);
button!(army_next, 5);
button!(recipient_prev, 6);
button!(recipient_next, 7);
button!(page_prev, 8);
button!(page_next, 9);
button!(row0, 10);
button!(row1, 11);
button!(mission_prev, 12);
button!(mission_next, 13);
button!(recall, 14);
button!(history, 15);
button!(clear_history, 16);
const CHOOSER_BUTTONS: &[(&str, unsafe extern "C" fn())] = &[
    ("tgcx_vl_close", close),
    ("tgcx_vl_cancel", cancel),
    ("tgcx_vl_send", send),
    ("tgcx_vl_army_prev", army_prev),
    ("tgcx_vl_army_next", army_next),
    ("tgcx_vl_recipient_prev", recipient_prev),
    ("tgcx_vl_recipient_next", recipient_next),
    ("tgcx_vl_page_prev", page_prev),
    ("tgcx_vl_page_next", page_next),
    ("tgcx_vl_row0", row0),
    ("tgcx_vl_row1", row1),
];
const FORCE_BUTTONS: &[(&str, unsafe extern "C" fn())] = &[
    ("tgcx_vl_mission_prev", mission_prev),
    ("tgcx_vl_mission_next", mission_next),
    ("tgcx_vl_recall", recall),
    ("tgcx_vl_history", history),
    ("tgcx_vl_clear_history", clear_history),
];

#[unsafe(naked)]
unsafe extern "C" fn update_hook() {
    std::arch::naked_asm!(
    "push ecx","call dword ptr [{original}]","pushfd","pushad","mov ebx, esp","sub esp, 528","and esp, -16","fxsave [esp]","fninit","mov dword ptr [esp+512], 0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+36]","call {callback}","add esp, 4","fxrstor [esp]","mov esp, ebx","popad","popfd","lea esp, [esp+4]","ret",
    original=sym UPDATE,callback=sym updated);
}
macro_rules! before_hook{($name:ident,$callback:path,$original:path)=>{#[unsafe(naked)]unsafe extern "C" fn $name(){std::arch::naked_asm!(
    "pushfd","pushad","mov ebx, esp","sub esp, 528","and esp, -16","fxsave [esp]","fninit","mov dword ptr [esp+512], 0x1F80","ldmxcsr [esp+512]","cld","push ecx","call {callback}","add esp, 4",
    "fxrstor [esp]","mov esp, ebx","popad","popfd","jmp dword ptr [{original}]",callback=sym $callback,original=sym $original);}};}
before_hook!(hide_hook, hidden, HIDE);
before_hook!(destroy_hook, destroyed, DESTROY);

pub fn install(base: u32, settings: Settings) -> bool {
    let Some(native) = Native::new(base) else {
        return false;
    };
    if UI
        .set(Mutex::new(Ui {
            native,
            thread: None,
            app: App::new(settings),
            view: 0,
            root: 0,
            entry: 0,
            chooser: 0,
            forces: 0,
            bindings: Vec::new(),
            retired: Vec::new(),
            world: WorldView::default(),
            shown: false,
            owned: Vec::new(),
            disabled: false,
            blocked_epoch: None,
            text_cache: BTreeMap::new(),
            entry_slot: None,
            entry_layout: None,
            entry_layout_missing: false,
            bound_epoch: 0,
            bound_player: 0,
            store_head: None,
            pending_checkpoint: None,
            saved_registry: None,
            loss_reads: BTreeMap::new(),
            route_key: None,
            route_armies: Vec::new(),
            route_chooser: crate::route_chooser::Chooser::default(),
        }))
        .is_err()
    {
        return false;
    }
    UPDATE.store((base + 0x221C10) as usize, Ordering::Release);
    HIDE.store((base + 0x3539D0) as usize, Ordering::Release);
    DESTROY.store((base + 0x21BE80) as usize, Ordering::Release);
    let slots = [
        (11, base + 0x221C10, update_hook as *const () as u32),
        (4, base + 0x3539D0, hide_hook as *const () as u32),
        (1, base + 0x21BE80, destroy_hook as *const () as u32),
    ];
    let mut changed = Vec::new();
    for (slot, expected, replacement) in slots {
        let address = base + 0xA07450 + slot * 4;
        if !native.replace_slot(address, expected, replacement) {
            // Protection restoration can fail after a successful CAS. Roll back
            // this slot too; never leave a partially installed hook untracked.
            native.replace_slot(address, replacement, expected);
            for (address, old, new) in changed {
                native.replace_slot(address, new, old);
            }
            return false;
        }
        changed.push((address, expected, replacement));
    }
    READY.store(true, Ordering::Release);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_hide_is_recoverable_on_the_original_ui_thread() {
        // Empty roots guarantee no real game/GUI call. Execute the actual hide
        // callback on a worker, then resume on its captured UI thread.
        let mut app = App::new(Settings::default());
        app.open = true;
        let ui = Ui {
            native: Native { base: 0 },
            thread: Some(unsafe { GetCurrentThreadId() }),
            app,
            view: 0x10000,
            root: 0,
            entry: 0,
            chooser: 0,
            forces: 0,
            bindings: Vec::new(),
            retired: Vec::new(),
            world: WorldView::default(),
            shown: false,
            owned: Vec::new(),
            disabled: false,
            blocked_epoch: None,
            text_cache: BTreeMap::new(),
            entry_slot: None,
            entry_layout: None,
            entry_layout_missing: false,
            bound_epoch: 0,
            bound_player: 0,
            store_head: None,
            pending_checkpoint: None,
            saved_registry: None,
            loss_reads: BTreeMap::new(),
            route_key: None,
            route_armies: Vec::new(),
            route_chooser: crate::route_chooser::Chooser::default(),
        };
        assert!(UI.set(Mutex::new(ui)).is_ok());
        std::thread::spawn(|| hidden(0x10000)).join().unwrap();
        assert!(!UI.get().unwrap().lock().unwrap().disabled);
        assert_eq!(PENDING_HIDE.load(Ordering::Acquire), 0x10000);
        hidden(0x10000);
        assert_eq!(PENDING_HIDE.load(Ordering::Acquire), 0);
        let mut ui = UI.get().unwrap().lock().unwrap();
        ui.app.open = true;
        assert!(!ui.disabled && ui.app.open);
        ui.world.epoch = 1;
        ui.stop_missions("Checkpoint failed; reload required");
        assert!(!ui.disabled && ui.app.open);
        assert_eq!(ui.blocked_epoch, Some(1));
        assert_eq!(ui.app.capabilities, Default::default());
    }
}
