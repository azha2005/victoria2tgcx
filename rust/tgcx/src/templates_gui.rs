//! TPL-0 native Templates panel, and MOD-6 Universal Army's shared native GUI; port of
//! tgcx_templates_gui.h and tgcx_army_universal.h. Only GUI state and a local
//! preference file are written directly. Recruit sends guarded CConstructUnitCommand
//! objects.

use std::sync::atomic::{AtomicUsize, Ordering};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_CONTROL, VK_MENU};

use crate::bill::{bind_observer, find, find_named, set_text_native};
use crate::sys::{
    CpuScope, GameString, Global, Text, at, base, call_one, call_u32, call_zero, guard, in_image,
    keep_last_error, rd8, rd32, rdf, rdi, rdp, replace_pointer, require_signatures, slot,
    thread_id,
};
use crate::templates::{
    self, ArmyFilter, CONTINENTS, CONTINENTS_MAX, CULTURE_ACCEPTED, CULTURE_ALL, CULTURE_PRIMARY,
    Composition, SLOTS, TplCountry, UNITS, catalogue, catalogue_fits, kinds, live_globals, scan,
    unit_by_name,
};
use crate::templates_catalog::*;
use crate::{log, text};

// ---------------------------------------------------------------- shared native GUI helpers

/// Show or hide a native window through its own vtable (slots 8/9).
pub fn visible(object: usize, show: bool) {
    if object == 0 {
        return;
    }
    // Independent Universal roots never receive their host's recursive Show. Primary
    // slot 8 only exposes the root renderer; the CGui subobject's Show activates
    // buttons/icons/child windows too. Do this on transition only: repeating it every
    // poll would reset intentionally hidden covers.
    if templates::army_universal() && show && rd8(object + 0x67) == 0 {
        let gui = object + 0x18;
        let secondary = rdp(gui);
        if secondary == 0 || rdp(secondary + 13 * 4) != base() + 0x5E4490 {
            log!("Universal Army: subtree Show refused object={object:08X}");
            return;
        }
        call_zero(rdp(secondary + 13 * 4), gui);
        return;
    }
    let index = if show { 8 } else { 9 };
    if slot(object, index) == base() + if show { 0x5E4450 } else { 0x5E4030 } {
        call_zero(slot(object, index), object);
    }
}

/// An inner matrix (+0x84, -0x88) is relative to its owner (+0xE4). The large build
/// root is a child of the Military window, so its screen position is the sum along the
/// owner chain up to the renderer's scene (+0x6B9E0), whose direct children are in
/// screen pixels (P40). Refuse any other topology.
pub fn root_screen(mut inner: usize, scene: usize) -> Option<(f32, f32, u32)> {
    let (mut sx, mut sy) = (0f32, 0f32);
    for level in 0..8 {
        if !in_image(rdp(inner)) {
            return None;
        }
        sx += rdf(inner + 0x84);
        sy -= rdf(inner + 0x88);
        let owner = rdp(inner + 0xE4);
        if owner == scene {
            return Some((sx, sy, level + 1));
        }
        if owner == 0 {
            return None;
        }
        inner = owner;
    }
    None
}

/// Foreground (slot 11) and packed screen position (slot 6) of a window, as the Bill.
pub fn place_window(window: usize, x: f32, y: f32, promote: bool) {
    if promote {
        call_zero(slot(window, 11), window);
    }
    let position = u32::from(x as u16) | (u32::from(y as u16) << 16);
    call_u32(slot(window, 6), window, position);
}

// ---------------------------------------------------------------- Universal Army (MOD-6)

// Shared native GUI definitions, constructed without replacing a base GUI. GUI
// ownership/state only; all simulation changes still use engine commands.

static UNIV_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);

fn univ_ready() -> bool {
    if !templates::army_universal() || catalogue().universal_ready {
        return true;
    }
    let country = UNIV_COUNTRY.get();
    scan(&live_globals(), country) && templates::universal_catalogue(country)
}

pub fn univ_context(root: usize) -> usize {
    let context = rdp(root + 0x4C);
    if context == 0 || rdp(context) != base() + 0xA08A44 || rdp(context + 0x30) != rdp(root + 0x90)
    {
        return 0;
    }
    context
}

fn univ_make(root: usize, key: &str) -> usize {
    let context = univ_context(root);
    if context == 0 {
        return 0;
    }
    let key = Text::<64>::from_latin1(key);
    let name = GameString::borrow(key.with_nul());
    let registry = context + 0x34;
    let lookup = slot(registry, 1);
    if !in_image(lookup) {
        return 0;
    }
    if call_one(lookup, registry, (&raw const name).cast()) == 0 {
        return 0;
    }
    if slot(context, 11) != base() + 0x5A38F0 {
        return 0;
    }
    type Make = unsafe extern "fastcall" fn(usize, usize, *const GameString, usize) -> usize;
    // SAFETY: CEU3Gui's named window factory (signature RVA 0x5A38F0), RET 8.
    let result =
        unsafe { std::mem::transmute::<usize, Make>(slot(context, 11))(context, 0, &name, 0) };
    if result != 0 && rdp(result) != base() + 0xA3C0D8 {
        return 0;
    }
    if result != 0 {
        visible(result, false);
    }
    result
}

/// Match CUnitView teardown: hide, then mark the complete subtree for the GUI
/// manager's deferred collection. Never free a registered window directly.
pub fn univ_release(window: &mut usize) {
    if !templates::army_universal() || *window == 0 {
        return;
    }
    let w = *window;
    let released = guard(|| {
        if slot(w, 20) == base() + 0x5E3D60 && univ_context(w) != 0 {
            visible(w, false);
            call_zero(slot(w, 20), w);
        }
    });
    if released.is_none() {
        log!("Universal Army: expired GUI during release; discarded reference");
    }
    *window = 0;
}

static UNIV_DIAGNOSTICS: Global<u32> = Global::new(0);

pub fn univ_pair(
    root: usize,
    window_name: &str,
    tab_name: &str,
    window: &mut usize,
    tab: &mut usize,
) -> bool {
    if !univ_ready() {
        return false;
    }
    *window = univ_make(root, window_name);
    if *window != 0 {
        *tab = univ_make(root, tab_name);
    }
    if *window == 0 || *tab == 0 {
        let diagnostics = UNIV_DIAGNOSTICS.get();
        if *diagnostics < 8 {
            log!(
                "Universal Army: shared GUI creation refused for {window_name}, context={:08X} window={:08X} tab={:08X}",
                univ_context(root),
                *window,
                *tab
            );
        }
        *diagnostics += 1;
        univ_release(window);
        univ_release(tab);
        return false;
    }
    log!("Universal Army: constructed {window_name} + {tab_name} from shared definitions");
    true
}

fn univ_position(window: usize, x: i32, y: i32, promote: bool) -> bool {
    if slot(window, 6) != base() + 0x5E3BA0
        || slot(window, 11) != base() + 0x5E2FC0
        || !(0..=32767).contains(&x)
        || !(0..=32767).contains(&y)
    {
        return false;
    }
    // These are independent roots, not children drawn with the host panel. On first
    // show/reopen they must be promoted after that panel, just as the accepted
    // Templates/Conform bodies are promoted when opened.
    if promote || rd8(window + 0x67) == 0 {
        call_zero(slot(window, 11), window);
    }
    call_u32(slot(window, 6), window, x as u32 | ((y as u32) << 16));
    true
}

struct Seen {
    root: usize,
    stage: &'static str,
}

static REFUSED: Global<([Seen; 24], usize)> =
    Global::new(([const { Seen { root: 0, stage: "" } }; 24], 0));

/// Rate-limit by root and failed stage, not by frame. A missing control must leave a
/// useful diagnostic rather than silently returning every 100ms.
fn placement_refused(root: usize, stage: &'static str) -> bool {
    let (seen, next) = REFUSED.get();
    if seen
        .iter()
        .any(|s| s.root == root && std::ptr::eq(s.stage, stage))
    {
        return false;
    }
    seen[*next % 24] = Seen { root, stage };
    *next += 1;
    log!("Universal Army: entry placement refused stage={stage} root={root:08X}");
    false
}

fn univ_entry(root: usize, tab: usize, x: i32, y: i32, promote: bool) -> bool {
    let hidden = rd8(tab + 0x67) == 0;
    if !univ_position(tab, x, y, promote) {
        return placement_refused(root, "entry position/foreground");
    }
    if hidden {
        visible(tab, true);
        log!(
            "Universal Army: entry activated root={root:08X} tab={tab:08X} screen={x},{y} foreground=1 subtree={}",
            rd8(tab + 0x67)
        );
        if rd8(tab + 0x67) == 0 {
            return false;
        }
    }
    true
}

fn univ_button_position(button: usize) -> Option<(i32, i32)> {
    if slot(button, 15) != base() + 0x5EFE20 {
        return None;
    }
    let mut packed = 0u32;
    type Get = unsafe extern "fastcall" fn(usize, usize, *mut u32);
    // SAFETY: CEU3Button's packed position getter (signature RVA 0x5EFE20), RET 4.
    unsafe { std::mem::transmute::<usize, Get>(slot(button, 15))(button, 0, &mut packed) };
    Some((i32::from(packed as i16), i32::from((packed >> 16) as i16)))
}

fn univ_root_position(root: usize) -> Option<(f32, f32)> {
    let (renderer, inner) = (rdp(root + 0x90), rdp(root + 0x494));
    if renderer == 0 || inner == 0 || rdp(inner) != base() + 0xA38350 {
        return None;
    }
    root_screen(inner, renderer + 0x6B9E0).map(|(x, y, _)| (x, y))
}

/// Dense DoD rows leave no room for the 34px folder. The same compact 20px control
/// fits after ordinary rows; dense rows use the free title-band cell.
fn univ_tab_cell(right: i32, row_y: i32) -> (i32, i32) {
    if right + 3 + 20 <= 405 {
        (right + 3, row_y + 6)
    } else {
        (370, 0.max(row_y - 24))
    }
}

fn univ_tpl_tab(root: usize, tab: usize, promote: bool) -> bool {
    let (mut right, mut row_y) = (0, 31);
    let icons = catalogue().icons.clone();
    for k in 0..kinds() {
        let name = text!(48, "unit_folder_{}", icons.get(k).copied().unwrap_or(0));
        let button = find(root, 13, 0x5E4C80, name.with_nul());
        if button != 0
            && let Some((x, y)) = univ_button_position(button)
            && (0..4096).contains(&x)
            && (0..4096).contains(&y)
            && x + 32 > right
        {
            right = x + 32;
            row_y = y;
        }
    }
    if right == 0 {
        return placement_refused(root, "Templates native folders/getter");
    }
    let (tx, ty) = univ_tab_cell(right, row_y);
    let Some((x, y)) = univ_root_position(root) else {
        return placement_refused(root, "Templates root screen chain");
    };
    univ_entry(root, tab, x as i32 + tx, y as i32 + ty, promote)
}

pub fn univ_cf_tab(root: usize, tab: usize) -> bool {
    let icon = find_named(root, 17, 0x5E4D40, "unitattrition_icon");
    if icon == 0 {
        return placement_refused(root, "Conform attrition icon lookup");
    }
    // CIcon slot 15 copies its packed definition position from +0xBC.
    if slot(icon, 15) != base() + 0x246620 {
        return placement_refused(root, "Conform attrition position getter");
    }
    let mut packed = 0u32;
    type Get = unsafe extern "fastcall" fn(usize, usize, *mut u32);
    // SAFETY: CIcon's packed position getter (signature RVA 0x246620).
    unsafe { std::mem::transmute::<usize, Get>(slot(icon, 15))(icon, 0, &mut packed) };
    let Some((x, y)) = univ_root_position(root) else {
        return placement_refused(root, "Conform root screen chain");
    };
    univ_entry(
        root,
        tab,
        x as i32 + i32::from(packed as i16) - 23,
        y as i32 + i32::from((packed >> 16) as i16),
        false,
    )
}

pub fn univ_icon(parent: usize, window_name: &[u8], sprite_name: &[u8], kind: usize, show: bool) {
    let window = find(parent, 27, 0x5E4A20, window_name);
    if window == 0 {
        return;
    }
    visible(window, show);
    if !show || kind >= kinds() {
        return;
    }
    let icon = find(window, 17, 0x5E4D40, sprite_name);
    if icon == 0 || slot(icon, 18) != base() + 0x5C4410 {
        return;
    }
    let frame = catalogue().icons.get(kind).copied().unwrap_or(0);
    call_u32(slot(icon, 18), icon, frame);
}

// ---------------------------------------------------------------- the panel

const SHOW_SEL: usize = 0;
const SHOW_BLANK: usize = 4;
const SHOW_PAGER: usize = 8;
const SHOW_PAGES: usize = 9;
const SHOW_ICON: usize = SHOW_PAGES + templates::CAPACITY.div_ceil(8);
const SHOW_MINI: usize = SHOW_ICON + 4 * templates::CAPACITY;
const SHOW_CHECK: usize = SHOW_MINI + 4 * 4 * templates::CAPACITY;
const SHOW_END: usize = SHOW_CHECK + 2;
const SHOWN: usize = templates::CAPACITY * 21 + 64;
const _: () = assert!(SHOW_END <= SHOWN, "Templates visibility cache");
const _: () = assert!(
    TPL_BUTTONS == crate::thunks::TPL_THUNKS,
    "one native observer stub per Templates button"
);

pub struct TplPanel {
    view: usize,
    root: usize,
    window: usize,
    tab: usize,
    thread: u32,
    tick: u32,
    open: bool,
    disabled: bool,
    bound: [bool; TPL_BUTTONS],
    selected: usize,
    list_page: usize,
    unit_page: usize,
    glue: [[u8; 44]; TPL_BUTTONS],
    text: [Text<192>; TPL_TEXTS],
    status: Text<192>,
    text_set: [bool; TPL_TEXTS],
    shown: [u8; SHOWN],
    /// Current action only; never saved with a template.
    continent_world: usize,
    continent: Text<64>,
}

impl TplPanel {
    fn forget(&mut self) {
        self.bound = [false; TPL_BUTTONS];
        self.text_set = [false; TPL_TEXTS];
        self.shown = [0; SHOWN];
    }
}

static PANELS: Global<[TplPanel; 8]> = Global::new(unsafe { std::mem::zeroed() });
static UPDATE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static DESTROY_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static GUI_ENABLED: Global<bool> = Global::new(false);

pub fn enabled() -> bool {
    *GUI_ENABLED.get()
}

fn text(p: &mut TplPanel, id: usize, args: std::fmt::Arguments) {
    let t = text!(192, "{args}");
    if p.text_set[id] && p.text[id].bytes() == t.bytes() {
        return;
    }
    p.text[id] = t;
    p.text_set[id] = true;
    set_text_native(
        p.window,
        Text::<128>::from_latin1(TPL_TEXT_NAMES[id]).with_nul(),
        t.with_nul(),
    );
}

fn status(p: &mut TplPanel, s: &str) {
    p.status = Text::from_latin1(s);
}

/// Foreground promotion is the same confirmed native path as the Bill (P37). Anchor
/// to the build view's screen position, so the native right queue stays where the
/// engine placed it. Promotion happens only on opening.
fn place(p: &TplPanel, promote: bool) -> bool {
    if slot(p.window, 11) != base() + 0x5E2FC0 || slot(p.window, 6) != base() + 0x5E3BA0 {
        return false;
    }
    let inner = rdp(p.root + 0x494);
    let renderer = rdp(p.root + 0x90);
    if inner == 0 || renderer == 0 || rdp(inner) != base() + 0xA38350 {
        return false;
    }
    let Some((mut x, mut y, depth)) = root_screen(inner, renderer + 0x6B9E0) else {
        return false;
    };
    if promote {
        log!(
            "Templates anchor: root local {:.0},{:.0}, screen {x:.0},{y:.0} through {depth} owner levels",
            rdf(inner + 0x84),
            -rdf(inner + 0x88)
        );
    }
    x += 5.0;
    y += 69.0;
    let (width, height) = (rdi(renderer + 0x6B9BC), rdi(renderer + 0x6B9C0));
    if !(x >= 0.0 && y >= 64.0 && x + 652.0 <= width as f32 && y + 528.0 <= height as f32) {
        return false;
    }
    place_window(p.window, x, y, promote);
    true
}

fn panel_live(p: &TplPanel) -> bool {
    if p.view == 0
        || rdp(p.view) != base() + 0xA12A14
        || p.root == 0
        || rdp(p.view + 0x60) != p.root
    {
        return false;
    }
    rd8(p.root + 0x67) != 0 && rd8(p.view + 0x78) != 0 && rd8(p.view + 0x5C) == 0
}

/// Child windows only: buttons have another class and the guarded window show/hide
/// refuses them. Each window's last state is cached so a tick does no lookup unless
/// something changed; rebinding or showing the panel clears the cache.
fn show(p: &mut TplPanel, id: usize, key: &[u8], on: bool) {
    let want = if on { 1 } else { 2 };
    if id < SHOWN && p.shown[id] == want {
        return;
    }
    let window = find(p.window, 27, 0x5E4A20, key);
    if window == 0 {
        return;
    }
    visible(window, on);
    if id < SHOWN {
        p.shown[id] = want;
    }
}

/// Board 5: the leading icon is the largest group (first on a tie); the miniatures
/// list the non-empty groups in catalogue order, as many as fit.
pub fn minis(c: &Composition) -> (usize, [usize; 4], usize) {
    let kinds = kinds();
    let (mut leading, mut mini, mut n) = (kinds, [0; 4], 0);
    for u in 0..kinds {
        if c.count[u] == 0 {
            continue;
        }
        if leading == kinds || c.count[u] > c.count[leading] {
            leading = u;
        }
        if n < 4 {
            mini[n] = u;
            n += 1;
        }
    }
    (leading, mini, n)
}

fn plural(n: u32) -> &'static str {
    if n == 1 { "regiment" } else { "regiments" }
}

fn labels(p: &mut TplPanel) {
    let world = live_globals().world;
    if p.continent_world != world {
        p.continent_world = world;
        p.continent.clear();
    }
    let continents = templates::continents_live(world);
    let region = if continents {
        templates::continent_by_key(CONTINENTS.get(), p.continent.bytes()).copied()
    } else {
        None
    };
    let library = *templates::library();
    let mode = if p.selected < library.size {
        library.item[p.selected].culture
    } else {
        0
    };
    // A ticked box is the native "on" frame shown beneath its button.
    show(
        p,
        SHOW_CHECK,
        b"tgcx_tpl_check_on_accepted\0",
        mode == CULTURE_ACCEPTED,
    );
    show(
        p,
        SHOW_CHECK + 1,
        b"tgcx_tpl_check_on_primary\0",
        mode == CULTURE_PRIMARY,
    );
    let continent = if p.continent.is_empty() {
        Text::<128>::from_latin1("Any continent")
    } else {
        region.map_or(Text::from_latin1("Unavailable"), |r| r.label)
    };
    text(p, TPL_TEXT_CONTINENT, format_args!("{continent}"));
    let pages = 1.max(library.size.div_ceil(4));
    p.list_page %= pages;
    let units = catalogue().units.clone();
    let kinds = units.len();
    let universal = templates::army_universal();
    for row in 0..4 {
        let s = p.list_page * 4 + row;
        let used = s < library.size;
        show(
            p,
            SHOW_SEL + row,
            text!(64, "tgcx_tpl_slot_sel_{row}").with_nul(),
            used && s == p.selected,
        );
        show(
            p,
            SHOW_BLANK + row,
            text!(64, "tgcx_tpl_slot_blank_{row}").with_nul(),
            !used,
        );
        let t = TPL_TEXT_SLOT_0 + row * (TPL_TEXT_SLOT_1 - TPL_TEXT_SLOT_0);
        let name = if used {
            templates::name(&library.item[s])
        } else {
            Text::new()
        };
        text(p, t, format_args!("{name}"));
        let c = &library.item[if used { s } else { 0 }];
        let total = templates::total(c);
        if used {
            text(p, t + 1, format_args!("{total} {}", plural(total)));
            text(p, t + 2, format_args!("{} Days", templates::base_days(c)));
        } else {
            text(p, t + 1, format_args!(""));
            text(p, t + 2, format_args!(""));
        }
        let (leading, mini, minis) = if used { minis(c) } else { (kinds, [0; 4], 0) };
        for k in 0..4 {
            if k < minis {
                text(p, t + 3 + k, format_args!("\u{D7}{}", c.count[mini[k]]));
            } else {
                text(p, t + 3 + k, format_args!(""));
            }
        }
        if universal {
            let key = text!(128, "tgcx_tpl_slot_icon_{row}_unit");
            let sprite = text!(96, "tgcx_tpl_slot_sprite_{row}_unit");
            univ_icon(
                p.window,
                key.with_nul(),
                sprite.with_nul(),
                leading,
                leading < kinds,
            );
            for k in 0..4 {
                let key = text!(128, "tgcx_tpl_mini_{row}_{k}_unit");
                let sprite = text!(96, "tgcx_tpl_mini_sprite_{row}_{k}_unit");
                univ_icon(
                    p.window,
                    key.with_nul(),
                    sprite.with_nul(),
                    mini[k],
                    k < minis,
                );
            }
        } else {
            for (u, unit) in units.iter().enumerate() {
                let key = text!(128, "tgcx_tpl_slot_icon_{row}_{}", unit.key);
                show(p, SHOW_ICON + row * kinds + u, key.with_nul(), u == leading);
                for k in 0..4 {
                    let key = text!(128, "tgcx_tpl_mini_{row}_{k}_{}", unit.key);
                    show(
                        p,
                        SHOW_MINI + (row * 4 + k) * kinds + u,
                        key.with_nul(),
                        k < minis && mini[k] == u,
                    );
                }
            }
        }
    }
    show(p, SHOW_PAGER, b"tgcx_tpl_pager_blank\0", pages == 1);
    if pages > 1 {
        text(
            p,
            TPL_TEXT_LIST_PAGE,
            format_args!("{} / {pages}", p.list_page + 1),
        );
    } else {
        text(p, TPL_TEXT_LIST_PAGE, format_args!(""));
    }
    let selected = p.selected < library.size;
    let c = &library.item[if selected { p.selected } else { 0 }];
    if selected {
        text(
            p,
            TPL_TEXT_EDITOR,
            format_args!("{}: composition", templates::name(c)),
        );
    } else {
        text(p, TPL_TEXT_EDITOR, format_args!("Create a new template"));
    }
    text(
        p,
        TPL_TEXT_UNIT_PAGE,
        format_args!("{}/{}", p.unit_page + 1, kinds.div_ceil(8)),
    );
    for r in 0..8 {
        let u = p.unit_page * 8 + r;
        let shown = u < kinds;
        if shown {
            text(
                p,
                TPL_TEXT_UNIT_0 + r * 3,
                format_args!("{}", units[u].label),
            );
            text(
                p,
                TPL_TEXT_DAYS_0 + r * 3,
                format_args!("{} Days", units[u].days),
            );
        } else {
            text(p, TPL_TEXT_UNIT_0 + r * 3, format_args!(""));
            text(p, TPL_TEXT_DAYS_0 + r * 3, format_args!(""));
        }
        if shown && selected {
            text(p, TPL_TEXT_COUNT_0 + r * 3, format_args!("{}", c.count[u]));
        } else {
            text(p, TPL_TEXT_COUNT_0 + r * 3, format_args!(""));
        }
        if universal {
            let key = text!(128, "tgcx_tpl_edit_icon_{r}");
            let sprite = text!(80, "tgcx_tpl_edit_sprite_{r}");
            univ_icon(p.window, key.with_nul(), sprite.with_nul(), u, shown);
        }
    }
    if !universal {
        for page in 0..kinds.div_ceil(8) {
            let key = text!(128, "tgcx_tpl_icons_{page}");
            show(p, SHOW_PAGES + page, key.with_nul(), page == p.unit_page);
        }
    }
    let total = if selected { templates::total(c) } else { 0 };
    if total != 0 {
        text(
            p,
            TPL_TEXT_SUMMARY,
            format_args!(
                "{total} {}, {},000 men, ready in {} days",
                plural(total),
                total * 3,
                templates::base_days(c)
            ),
        );
    } else if selected {
        text(
            p,
            TPL_TEXT_SUMMARY,
            format_args!("Add regiments to the composition"),
        );
    } else {
        text(
            p,
            TPL_TEXT_SUMMARY,
            format_args!("Select or create a template"),
        );
    }
    if !templates::STORE.get().storage_ok {
        text(
            p,
            TPL_TEXT_STATUS,
            format_args!("Local template file unavailable or invalid; existing file preserved."),
        );
    } else if !p.status.is_empty() {
        let s = p.status;
        text(p, TPL_TEXT_STATUS, format_args!("{s}"));
    } else {
        text(
            p,
            TPL_TEXT_STATUS,
            format_args!("One build order per regiment, from provinces with free soldier pops."),
        );
    }
    text(p, TPL_TEXT_RECRUIT, format_args!("Recruit Template"));
    text(p, TPL_TEXT_TITLE, format_args!("Build Army: Templates"));
}

struct Recruiting {
    sent_world: usize,
    sent_date: u32,
    pending: bool,
}

static OPEN_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static OWNER_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static RECRUIT_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static PRESENT: Global<[bool; CONTINENTS_MAX]> = Global::new([false; CONTINENTS_MAX]);
static RECRUITING: Global<Recruiting> = Global::new(Recruiting {
    sent_world: 0,
    sent_date: 0,
    pending: false,
});

fn click_body(p: &mut TplPanel, action: usize) {
    if !panel_live(p) {
        return;
    }
    // Board 5 has no close button: another native folder closes the panel.
    if action >= TPL_NATIVE_17 {
        p.open = false;
        visible(p.window, false);
        return;
    }
    if action == TPL_OPEN {
        let current = OPEN_COUNTRY.get();
        if !scan(&live_globals(), current) || !catalogue_fits(current) {
            log!(
                "Templates: package catalogue does not match active land units; regenerate Army for this base"
            );
            return;
        }
        templates::load_library();
        // Showing a window re-shows its whole subtree (icons, covers, ring), so the
        // cached child states are stale from here (P42).
        if place(p, !p.open) {
            p.open = true;
            visible(p.window, true);
            p.shown = [0; SHOWN];
            p.tick = 0;
            if templates::army_universal() {
                univ_tpl_tab(p.root, p.tab, true);
            }
        } else {
            log!("Templates panel does not fit the native build-view position");
        }
        return;
    }
    if !p.open {
        return;
    }
    if action == TPL_CONTINENT_PREV || action == TPL_CONTINENT_NEXT {
        let world = live_globals().world;
        if p.continent_world != world {
            p.continent_world = world;
            p.continent.clear();
        }
        if !templates::continents_live(world) {
            status(p, "Continent catalogue unavailable; no selection changed.");
            return;
        }
        let items = &CONTINENTS.get().items;
        let current = items
            .iter()
            .rposition(|i| i.key.bytes() == p.continent.bytes())
            .map_or(0, |i| i + 1);
        // Only continents where the country owns a province; if that read fails,
        // every continent is offered rather than none.
        let owner = OWNER_COUNTRY.get();
        let present = PRESENT.get();
        let known = scan(&live_globals(), owner)
            && templates::continents_present(world, owner.country, present);
        let catalogue = CONTINENTS.get();
        let next = templates::continent_step(
            catalogue,
            known.then_some(&present[..]),
            current,
            action == TPL_CONTINENT_NEXT,
        );
        p.continent = if next != 0 {
            catalogue.items[next - 1].key
        } else {
            Text::new()
        };
        status(
            p,
            "Continent choice applies to this recruitment action only.",
        );
        p.tick = 0;
        return;
    }
    p.tick = 0;
    let library = *templates::library();
    if (TPL_SLOT_0..=TPL_SLOT_3).contains(&action) {
        let s = p.list_page * 4 + action - TPL_SLOT_0;
        if s < library.size {
            p.selected = s;
        }
        p.status.clear();
        return;
    }
    if action == TPL_LIST_PREV || action == TPL_LIST_NEXT {
        let pages = 1.max(library.size.div_ceil(4));
        p.list_page = (p.list_page
            + if action == TPL_LIST_NEXT {
                1
            } else {
                pages - 1
            })
            % pages;
        return;
    }
    if action == TPL_UNIT_PREV || action == TPL_UNIT_NEXT {
        let pages = kinds().div_ceil(8);
        p.unit_page = (p.unit_page
            + if action == TPL_UNIT_NEXT {
                1
            } else {
                pages - 1
            })
            % pages;
        return;
    }
    if action == TPL_RECRUIT {
        if p.selected >= library.size || templates::total(&library.item[p.selected]) == 0 {
            return;
        }
        // Do not issue another plan against stale pop capacity while the first batch
        // is still paused in the engine queue. Release only after game time
        // advances, for every panel and country.
        let g = live_globals();
        if g.world == 0 {
            return;
        }
        let date = rd32(g.world + 0xB0C);
        let r = RECRUITING.get();
        if r.pending && g.world == r.sent_world && date == r.sent_date {
            status(p, "Unpause once before recruiting another template.");
            return;
        }
        let country = RECRUIT_COUNTRY.get();
        if !scan(&g, country) {
            status(p, "Cannot read the player's recruitable units.");
            return;
        }
        if !catalogue_fits(country) {
            status(p, "Unit catalogue changed; regenerate Army for this base.");
            return;
        }
        let mut counts = [0i32; UNITS];
        let mut unknown = 0u32;
        for (k, unit) in catalogue().units.iter().enumerate() {
            let n = library.item[p.selected].count[k];
            match unit_by_name(country, unit.key.bytes()) {
                Some(u) => counts[u] = n,
                None => unknown += n as u32,
            }
        }
        let mut filter = ArmyFilter {
            culture: library.item[p.selected].culture,
            continent: 0,
        };
        if p.continent_world != g.world {
            p.continent_world = g.world;
            p.continent.clear();
        }
        if !p.continent.is_empty() {
            if !templates::continents_live(g.world) {
                status(p, "Continent catalogue unavailable; nothing queued.");
                return;
            }
            let Some(region) = templates::continent_by_key(CONTINENTS.get(), p.continent.bytes())
            else {
                status(
                    p,
                    "Selected continent unavailable; choose again. Nothing queued.",
                );
                return;
            };
            filter.continent = region.object;
        }
        let result = templates::recruit(&counts, None, None, Some(&filter));
        if !result.scanned {
            status(p, "Recruitment eligibility unavailable; nothing queued.");
            return;
        }
        if result.queued != 0 {
            let r = RECRUITING.get();
            r.pending = true;
            r.sent_world = g.world;
            r.sent_date = date;
        }
        p.status = text!(
            192,
            "{} orders queued; {} left out. Unpause to process them.",
            result.queued,
            result.missing + unknown
        );
        return;
    }
    let mut next = library;
    let mut selected = p.selected;
    if !templates::STORE.get().storage_ok {
        return;
    }
    if action == TPL_NEW {
        if next.size >= SLOTS {
            status(p, "All 12 template slots are in use.");
            return;
        }
        selected = next.size;
        next.size += 1;
        next.item[selected] = Composition::EMPTY;
    } else if action == TPL_DELETE {
        if selected >= next.size {
            return;
        }
        for i in selected + 1..next.size {
            next.item[i - 1] = next.item[i];
        }
        next.size -= 1;
        next.item[next.size] = Composition::EMPTY;
        selected = if next.size != 0 {
            selected.min(next.size - 1)
        } else {
            0
        };
    } else if action == TPL_CULTURE_ACCEPTED || action == TPL_CULTURE_PRIMARY {
        if selected >= next.size {
            return;
        }
        let mode = if action == TPL_CULTURE_ACCEPTED {
            CULTURE_ACCEPTED
        } else {
            CULTURE_PRIMARY
        };
        let culture = &mut next.item[selected].culture;
        *culture = if *culture == mode { CULTURE_ALL } else { mode };
    } else if (TPL_MINUS_0..=TPL_PLUS_7).contains(&action) {
        if selected >= next.size {
            return;
        }
        let u = p.unit_page * 8 + (action - TPL_MINUS_0) / 2;
        if u >= kinds() {
            return;
        }
        let add = !(action - TPL_MINUS_0).is_multiple_of(2);
        let total = templates::total(&next.item[selected]);
        if add && total >= 128 {
            status(p, "A template may contain at most 128 regiments.");
            return;
        }
        // TPL-3: Ctrl-click changes 5 regiments, Alt-click 2 (Ctrl wins), within the 128 cap.
        let mut step = if unsafe { GetKeyState(i32::from(VK_CONTROL)) } < 0 {
            5
        } else if unsafe { GetKeyState(i32::from(VK_MENU)) } < 0 {
            2
        } else {
            1
        };
        if add {
            step = step.min(128 - total as i32);
        }
        let value = &mut next.item[selected].count[u];
        *value = 0.max(*value + if add { step } else { -step });
    } else {
        return;
    }
    if templates::commit(&next) {
        p.selected = selected;
        p.list_page = selected / 4;
        status(p, "Template saved locally.");
    } else {
        status(
            p,
            "Could not save the template; the previous composition was kept.",
        );
    }
}

extern "C" fn tpl_click(p: *mut TplPanel, action: usize) {
    let _cpu = CpuScope::new();
    if !enabled() || p.is_null() || action >= TPL_BUTTONS {
        return;
    }
    // SAFETY: the panel this button's glue was bound to, one of PANELS.
    let p = unsafe { &mut *p };
    if thread_id() != p.thread {
        return;
    }
    if guard(|| click_body(p, action)).is_none() {
        log!("Templates click rejected: guarded native read fault");
    }
}

/// Every Templates stub pushes its action and jumps here; the native trampoline
/// passes our panel in ECX: `tpl_click(panel, action)` with every register and the
/// FPU preserved.
#[unsafe(naked)]
pub unsafe extern "C" fn tpl_thunk_common() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+36]",
        "push ecx",
        "call {click}",
        "add esp, 8",
        crate::restore_cpu!(),
        "lea esp, [esp+4]",
        "ret",
        click = sym tpl_click,
    )
}

fn bind(p: &mut TplPanel, action: usize) {
    if p.bound[action] {
        return;
    }
    // Lookups are flat: the open button lives in the tab window.
    let host = if action == TPL_OPEN {
        p.tab
    } else if action >= TPL_NATIVE_17 {
        p.root
    } else {
        p.window
    };
    if host == 0 {
        return;
    }
    let button = find_named(host, 13, 0x5E4C80, TPL_BUTTON_NAMES[action]);
    if button == 0 {
        // Native folders are static children of this root. Do not repeat up to 1024
        // absent-folder searches every 100ms on a small vanilla list.
        if action >= TPL_NATIVE_17 {
            p.bound[action] = true;
        }
        return;
    }
    // SAFETY: the stub table build.rs generated, one per action.
    let stub = unsafe { crate::thunks::tgcx_tpl_table[action] };
    // from V2DLL: V2TechButton.cpp, 44-byte Politics observer. Its deletion
    // notification uses +0x20 (left zero); +4 is our stable panel, +8 callback.
    let panel = &raw mut *p as usize;
    if bind_observer(button, &mut p.glue[action], panel, stub) {
        p.bound[action] = true;
    }
}

extern "C" fn tpl_capture(view: usize) {
    // Only record lifetime here: all GUI work runs after native updates, at final
    // presentation. No GUI/library allocation in DllMain.
    keep_last_error(|| {
        if !enabled() {
            return;
        }
        let panels = PANELS.get();
        if panels.iter().any(|p| p.view == view) {
            return;
        }
        if let Some(empty) = panels.iter_mut().find(|p| p.view == 0) {
            empty.view = view;
            empty.thread = thread_id();
        }
    })
}

fn reset(p: &mut TplPanel) {
    univ_release(&mut p.window);
    univ_release(&mut p.tab);
    p.view = 0;
    p.root = 0;
    p.window = 0;
    p.tab = 0;
    p.open = false;
    p.forget();
}

extern "C" fn tpl_destroy(view: usize) {
    keep_last_error(|| {
        for p in PANELS.get().iter_mut().filter(|p| p.view == view) {
            let window = p.window;
            let _ = guard(|| visible(window, false));
            reset(p);
            p.disabled = false;
            // Keep observer memory/vtables valid until native GUI destruction.
        }
    })
}

crate::view_thunk!(tpl_update_thunk, tpl_capture, UPDATE_ORIGINAL);
crate::view_thunk!(tpl_destroy_thunk, tpl_destroy, DESTROY_ORIGINAL);

fn hide_all(p: &mut TplPanel) {
    p.open = false;
    visible(p.window, false);
    visible(p.tab, false);
}

fn panel_tick_inner(p: &mut TplPanel) {
    // Build Navy shares the view and has more folders: no Templates tab.
    if !panel_live(p) {
        hide_all(p);
        return;
    }
    if templates::army_universal() && rdp(p.view + 0x6C) != 0 {
        hide_all(p);
        return;
    }
    let now = crate::sys::tick();
    if p.tick != 0 && now.wrapping_sub(p.tick) < 100 {
        return;
    }
    p.tick = now;
    if p.window == 0 {
        if templates::army_universal() {
            let (mut window, mut tab) = (0, 0);
            if !univ_pair(
                p.root,
                "tgcx_tpl_window",
                "tgcx_tpl_tab",
                &mut window,
                &mut tab,
            ) {
                return;
            }
            p.window = window;
            p.tab = tab;
        } else {
            // The province build view has no Templates tab. Lookup the optional
            // button first to avoid native missing-window warnings.
            if find_named(p.root, 13, 0x5E4C80, "tgcx_tpl_marker") == 0 {
                return;
            }
            p.window = find_named(p.root, 27, 0x5E4A20, "tgcx_tpl_window");
            if p.window == 0 {
                return;
            }
            p.tab = find_named(p.root, 27, 0x5E4A20, "tgcx_tpl_tab");
            visible(p.window, false);
            log!(
                "Templates native panel found view={:08X} root={:08X} tab={}",
                p.view,
                p.root,
                i32::from(p.tab != 0)
            );
        }
    }
    if templates::army_universal() && !univ_tpl_tab(p.root, p.tab, false) {
        visible(p.tab, false);
        return;
    }
    // Re-shown each tick: showing the build root re-shows its subtree (P42).
    visible(p.tab, true);
    for b in 0..TPL_BUTTONS {
        bind(p, b);
    }
    if p.open {
        if !place(p, false) {
            p.open = false;
            visible(p.window, false);
            return;
        }
        labels(p);
    }
}

fn panel_tick(p: &mut TplPanel) {
    if guard(|| panel_tick_inner(p)).is_none() {
        p.open = false;
        let window = p.window;
        let _ = guard(|| visible(window, false));
        log!("Templates panel disabled after guarded GUI read fault");
        p.disabled = true;
    }
}

pub fn tick() {
    if !enabled() {
        return;
    }
    for p in PANELS.get().iter_mut() {
        if p.view == 0 || p.disabled {
            continue;
        }
        let checked = guard(|| {
            // P47: destroyed outside the hook; its GUI went with it, free the slot.
            if rdp(p.view) != base() + 0xA12A14 {
                reset(p);
                log!("Templates: build view gone without its destructor; slot freed");
                return false;
            }
            let root = rdp(p.view + 0x60);
            if root != p.root {
                univ_release(&mut p.window);
                univ_release(&mut p.tab);
                visible(p.window, false);
                visible(p.tab, false);
                p.root = root;
                p.window = 0;
                p.tab = 0;
                p.open = false;
                p.forget();
            }
            true
        });
        match checked {
            Some(true) => panel_tick(p),
            Some(false) => {}
            None => p.disabled = true,
        }
    }
}

pub const TPL_GUI_RVAS: [u32; 17] = [
    0xA12A14, 0x334E24, 0x333C0C, 0x3374D3, 0xA0E48C, 0x5E4C80, 0x5E4A20, 0x5E4D00, 0x8ED0,
    0x65ECC0, 0x5E4450, 0x5E4030, 0x5E2FC0, 0x5E3BA0, 0xA383C4, 0xA383D0, 0xA3841C,
];

pub fn install() {
    if !require_signatures(&TPL_GUI_RVAS) {
        log!("Templates GUI rejected: guards");
        return;
    }
    templates::install();
    if !templates::installed() {
        return;
    }
    UPDATE_ORIGINAL.store(at(0x334E00), Ordering::Relaxed);
    DESTROY_ORIGINAL.store(at(0x3340E0), Ordering::Relaxed);
    let table = at(0xA12A14);
    let destroy = tpl_destroy_thunk as *const () as usize;
    let update = tpl_update_thunk as *const () as usize;
    if !replace_pointer(table, at(0x3340E0), destroy) {
        return;
    }
    if !replace_pointer(table + 4, at(0x334E00), update) {
        replace_pointer(table, destroy, at(0x3340E0));
        return;
    }
    *GUI_ENABLED.get() = true;
    log!(
        "Templates GUI hooks installed: native build-view update/destructor; engine commands only"
    );
}
