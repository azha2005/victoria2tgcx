//! TPL-1 Conform window (board 6-conform; D12, D13), route B: native GUI from
//! tools/tgcx_conform_gui.py, placed beside the unit panel; port of
//! tgcx_conform_gui.h. Only GUI state is written directly; Conform itself is
//! conform_native::conform_with (engine commands).

use crate::bill::{bind_observer, find, find_named, set_text_native};
use crate::conform::{CfId, CfPlan, Refusal};
use crate::conform_layout::*;
use crate::conform_native::{
    self as native, CfGlobals, CfStack, CfView, VIEW_VT_RVA, VIEWS, VIEWS_STATE,
};
use crate::replace::Why;
use crate::sys::{
    CpuScope, Global, Text, base, guard, rd8, rd32, rdi, rdp, require_signatures, slot, thread_id,
};
use crate::templates::{
    self, ArmyFilter, CULTURE_ACCEPTED, CULTURE_ALL, CULTURE_NONCORE, CULTURE_PRIMARY, POPS,
    TplCountry, TplPlan, TplPop, UNITS, catalogue,
};
use crate::templates_gui::{
    minis, place_window, root_screen, univ_cf_tab, univ_icon, univ_make, univ_pair, univ_release,
    visible,
};
use crate::{log, text};

const SLOTS: usize = CF_SLOTS as usize;
const CHANGES: usize = CF_CHANGES as usize;
const MINIS: usize = CF_MINIS as usize;

// ---------------------------------------------------------------- the Changes list (pure)

/// One row of board 6's Changes: built (green), not buildable now (yellow), a finished
/// regiment brought from nearby (green), or detached (red).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum RowKind {
    #[default]
    Build = 0,
    Short,
    Gather,
    Detach,
}

#[derive(Clone, Copy, Default)]
struct Row {
    kind: RowKind,
    unit: i32,
    count: u32,
}

#[derive(Clone, Copy, Default)]
struct Rows {
    count: usize,
    more: u32,
    row: [Row; CHANGES],
    /// Regiments, every row counted.
    build: u32,
    shortfall: u32,
    gather: u32,
    detach: u32,
}

/// placed[unit]: how many of plan.build TPL-0's recruit planner can place now. Rows are
/// grouped by unit and kind; additions in unit order, then detaches, as on the board.
/// Rows past the list's length are counted in more.
fn rows_of(plan: &CfPlan, placed: &[i32; UNITS]) -> Rows {
    let mut out = Rows::default();
    let add = |out: &mut Rows, kind: RowKind, unit: usize, n: u32| {
        if n == 0 {
            return;
        }
        if out.count < CHANGES {
            out.row[out.count] = Row {
                kind,
                unit: unit as i32,
                count: n,
            };
            out.count += 1;
        } else {
            out.more += 1;
        }
    };
    let mut gathered = [0u32; UNITS];
    let mut detached = [0u32; UNITS];
    for g in &plan.gather {
        if let Ok(u) = usize::try_from(g.unit)
            && u < UNITS
        {
            gathered[u] += 1;
        }
    }
    for d in &plan.detach {
        if let Ok(u) = usize::try_from(d.unit)
            && u < UNITS
        {
            detached[u] += 1;
        }
    }
    for u in 0..UNITS {
        let want = plan.build[u].max(0);
        let fit = want.min(placed[u].max(0));
        add(&mut out, RowKind::Build, u, fit as u32);
        add(&mut out, RowKind::Short, u, (want - fit) as u32);
        add(&mut out, RowKind::Gather, u, gathered[u]);
        out.build += fit as u32;
        out.shortfall += (want - fit) as u32;
        out.gather += gathered[u];
    }
    for u in 0..UNITS {
        add(&mut out, RowKind::Detach, u, detached[u]);
        out.detach += detached[u];
    }
    out
}

// ---------------------------------------------------------------- the window

const SHOW_SEL: usize = 0;
const SHOW_BLANK: usize = 4;
const SHOW_PAGER: usize = 8;
const SHOW_ROW: usize = 9;
const SHOW_ICON: usize = SHOW_ROW + CHANGES;
const SHOW_MINI: usize = SHOW_ICON + SLOTS * templates::CAPACITY;
const SHOW_CHANGE: usize = SHOW_MINI + SLOTS * MINIS * templates::CAPACITY;
const SHOW_CHECK: usize = SHOW_CHANGE + CHANGES * templates::CAPACITY;
const SHOW_END: usize = SHOW_CHECK + 4;
const SHOWN: usize = templates::CAPACITY * 28 + 64;
const _: () = assert!(SHOW_END <= SHOWN, "Conform visibility cache");
const _: () = assert!(
    CF_BUTTONS == crate::thunks::CF_THUNKS,
    "one native observer stub per Conform button"
);

pub struct CfPanel {
    view: usize,
    root: usize,
    window: usize,
    tab: usize,
    thread: u32,
    tick: u32,
    open: bool,
    disabled: bool,
    land: bool,
    bound: [bool; CF_BUTTONS],
    /// The stack the unit panel showed last tick.
    seen: usize,
    /// The stack being conformed, found again by id.
    stack: CfId,
    vtable: usize,
    selected: usize,
    list_page: usize,
    filter_revision: u32,
    preview_date: u32,
    planned: bool,
    refusal: Refusal,
    rows: Rows,
    /// Unit index -> catalogue kind.
    kind: [i32; UNITS],
    glue: [[u8; 44]; CF_BUTTONS],
    text: [Text<192>; CF_TEXTS],
    summary: Text<192>,
    status: Text<192>,
    text_set: [bool; CF_TEXTS],
    shown: [u8; SHOWN],
    /// TPL-12: independent Replace depleted chooser at Conform's usual position.
    chooser: usize,
    rp_open: bool,
    rp_shown: [u8; RP_SHOWN],
    rp_summary: Text<192>,
    rp_list: Text<192>,
    rp_status: Text<192>,
    title_font_checked: bool,
    title_font_ready: bool,
}

const RP_SHOWN: usize = 5 + CF_RP_STEPS as usize;

static PANELS: Global<[CfPanel; VIEWS]> = Global::new(unsafe { std::mem::zeroed() });

/// A view may retain a just-merged stack. Check its small identity under SEH, then the
/// registry, before reading ownership or calling a virtual method.
fn shown_stack(g: &CfGlobals, stack: usize) -> usize {
    let known = guard(|| stack != 0 && native::lookup(g, native::stack_id(stack)) == stack + 8);
    if known == Some(true) { stack } else { 0 }
}

fn text(p: &mut CfPanel, id: usize, args: std::fmt::Arguments) {
    let t = text!(192, "{args}");
    if p.text_set[id] && p.text[id].bytes() == t.bytes() {
        return;
    }
    p.text[id] = t;
    p.text_set[id] = true;
    let host = if CF_TEXT_NAMES[id].starts_with("tgcx_cf_text_rp_") {
        p.chooser
    } else {
        p.window
    };
    if host == 0 {
        return;
    }
    set_text_native(
        host,
        Text::<128>::from_latin1(CF_TEXT_NAMES[id]).with_nul(),
        t.with_nul(),
    );
}

/// One CP1252 display line, measured as a whole prefix plus ellipsis so
/// proportional glyphs and kerning cannot turn it into a second line.
fn fit_line(source: &[u8], limit: i32, mut measure: impl FnMut(&[u8]) -> Option<i32>) -> Text<192> {
    if source.is_empty() {
        return Text::new();
    }
    let end = source
        .iter()
        .position(|&b| matches!(b, b'\r' | b'\n'))
        .unwrap_or(source.len());
    let line = &source[..end];
    if end == source.len() && measure(line).is_some_and(|width| width >= 0 && width <= limit) {
        return Text::from_bytes(line);
    }
    for end in (0..=line.len()).rev() {
        let mut end = end;
        while end > 0 && line[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        let mut out = Text::from_bytes(&line[..end]);
        out.push_bytes(b"...");
        if measure(out.bytes()).is_some_and(|width| width >= 0 && width <= limit) {
            return out;
        }
    }
    Text::new()
}

fn title_label(p: &mut CfPanel, id: usize, name: &Text<160>) {
    if name.is_empty() {
        text(p, id, format_args!(""));
        return;
    }
    if !p.title_font_checked {
        p.title_font_ready = require_signatures(&[0x624E20, 0x624F56, 0x65ED44]);
        p.title_font_checked = true;
        if !p.title_font_ready {
            log!("Conform title measurement unavailable: font guards; long labels suppressed");
        }
    }
    let key = Text::<128>::from_latin1(CF_TEXT_NAMES[id]);
    let control = find(p.window, 15, 0x5E4D00, key.with_nul());
    let inner = if control != 0 { rdp(control + 0xA0) } else { 0 };
    let font = if inner != 0 && slot(inner, 10) == base() + 0x65EB00 {
        rdp(inner + 0x1BC)
    } else {
        0
    };
    let shown = if p.title_font_ready && font != 0 && slot(font, 10) == base() + 0x624E20 {
        fit_line(name.bytes(), CF_SLOT_NAME_WIDTH - 4, |bytes| {
            type Width = unsafe extern "fastcall" fn(usize, usize, *const u8, i32) -> i32;
            // Native CEU3BitmapFont::width: ECX font, two stack arguments,
            // RET8. Existing Rename evidence covers the same text/font path.
            guard(|| unsafe {
                std::mem::transmute::<usize, Width>(base() + 0x624E20)(
                    font,
                    0,
                    bytes.as_ptr(),
                    bytes.len() as i32,
                )
            })
            .filter(|&width| (0..=4096).contains(&width))
        })
    } else {
        Text::from_latin1("...")
    };
    text(p, id, format_args!("{shown}"));
}

fn status(p: &mut CfPanel, s: &str) {
    p.status = Text::from_latin1(s);
}

fn summary(p: &mut CfPanel, s: &str) {
    p.summary = Text::from_latin1(s);
}

/// Child windows only, each one's last state cached; showing the window clears the
/// cache (P42).
fn show(p: &mut CfPanel, id: usize, key: &[u8], on: bool) {
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

fn close(p: &mut CfPanel) {
    p.open = false;
    if p.window != 0 {
        visible(p.window, false);
    }
    rp_close(p);
}

/// Beside the unit panel (its Select button ends at x=435), inside the screen. The
/// owner chain and the native setters are TPL-0's (P37, P40).
fn place(p: &mut CfPanel, promote: bool) -> bool {
    place_panel(p, p.window, CF_WIDTH, CF_HEIGHT, promote)
}

fn place_panel(
    p: &CfPanel,
    window: usize,
    panel_width: i32,
    panel_height: i32,
    promote: bool,
) -> bool {
    if window == 0 || slot(window, 11) != base() + 0x5E2FC0 || slot(window, 6) != base() + 0x5E3BA0
    {
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
    let (width, height) = (rdi(renderer + 0x6B9BC), rdi(renderer + 0x6B9C0));
    if width < panel_width || height < panel_height {
        return false;
    }
    if promote {
        log!(
            "Conform anchor: unit panel screen {x:.0},{y:.0} through {depth} owner levels, screen {width}x{height}"
        );
    }
    x += 331.0;
    y += 26.0;
    if x + panel_width as f32 > width as f32 {
        x = (width - panel_width) as f32;
    }
    if y + panel_height as f32 > height as f32 {
        y = (height - panel_height) as f32;
    }
    x = x.max(0.0);
    y = y.max(0.0);
    place_window(window, x, y, promote);
    true
}

fn panel_live(p: &CfPanel) -> bool {
    if p.view == 0
        || rdp(p.view) != base() + VIEW_VT_RVA as usize
        || p.root == 0
        || rdp(p.view + 0x20) != p.root
    {
        return false;
    }
    rd8(p.root + 0x67) != 0
}

/// The conformed stack, found again by id each time (never a kept pointer).
fn target(g: &CfGlobals, p: &CfPanel) -> usize {
    if p.vtable != 0 {
        native::stack_of(g, p.stack, p.vtable)
    } else {
        0
    }
}

/// The selected template as want[unit index], for this country's unit types.
fn want(p: &mut CfPanel, country: &TplCountry) -> [i32; UNITS] {
    let mut want = [0; UNITS];
    p.kind = [-1; UNITS];
    let library = templates::library();
    for (k, unit) in catalogue().units.iter().enumerate() {
        let Some(u) = templates::unit_by_name(country, unit.key.bytes()) else {
            continue;
        };
        p.kind[u] = k as i32;
        if p.selected < library.size {
            want[u] = library.item[p.selected].count[k];
        }
    }
    want
}

static PREVIEW_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static PREVIEW_PLAN: Global<CfPlan> = Global::new(CfPlan::EMPTY);
static PREVIEW_TARGET: Global<CfStack> = Global::new(CfStack::EMPTY);
static PREVIEW_POPS: Global<[TplPop; POPS]> = Global::new(unsafe { std::mem::zeroed() });
static PREVIEW_BUILDS: Global<TplPlan> = Global::new(unsafe { std::mem::zeroed() });

/// The preview: the plan Conform would carry out now, and how many of its builds
/// TPL-0's recruit planner can place on free soldier pops.
fn preview(p: &mut CfPanel) {
    p.planned = false;
    p.summary.clear();
    p.rows = Rows::default();
    p.filter_revision = templates::STORE.get().revision;
    let g = native::live_globals();
    p.preview_date = if g.world != 0 {
        rd32(g.world + 0xB0C)
    } else {
        0
    };
    let library = templates::library();
    if p.selected >= library.size {
        let s = if library.size != 0 {
            "Select a template."
        } else {
            "No templates yet: create one in Build Army, Templates."
        };
        summary(p, s);
        return;
    }
    let country = PREVIEW_COUNTRY.get();
    if !templates::scan(&templates::live_globals(), country) {
        summary(p, "Cannot read the player's unit types.");
        return;
    }
    if !templates::catalogue_fits(country) {
        summary(p, "Unit catalogue changed; regenerate Army for this base.");
        return;
    }
    let mut want = want(p, country);
    let marked = native::mobo_quota(p.stack).is_some();
    if marked && let Some(u) = templates::unit_by_name(country, b"infantry") {
        want[u] = 0;
    }
    let stack = target(&g, p);
    let plan = PREVIEW_PLAN.get();
    let target = PREVIEW_TARGET.get();
    if stack == 0 || !native::prepare(&g, stack, Some(&country.unit), &want, plan, target) {
        summary(p, "This army can no longer be read: select it again.");
        return;
    }
    p.refusal = plan.refusal;
    match plan.refusal {
        Refusal::Matches => {
            summary(
                p,
                if marked {
                    "This mobo stack already matches the template without its infantry."
                } else {
                    "This army already matches the template (counting regiments on their way)."
                },
            );
            return;
        }
        Refusal::NothingKept => {
            summary(
                p,
                if marked {
                    "A mobo stack keeps no infantry and this army has nothing else. Nothing is changed."
                } else {
                    "No regiment of this army is in the template: nothing would stay. Nothing is changed."
                },
            );
            return;
        }
        Refusal::TooLarge => {
            summary(p, "Armies and templates are limited to 128 regiments.");
            return;
        }
        Refusal::Ok => {}
    }
    let mut placed = [0i32; UNITS];
    let pops = PREVIEW_POPS.get();
    let count = country.pop_count;
    pops[..count].copy_from_slice(&country.pops[..count]);
    let filter = ArmyFilter {
        culture: library.item[p.selected].culture,
        continent: 0,
    };
    if !templates::army_apply(country.country, &mut pops[..count], &filter) {
        summary(
            p,
            "Culture eligibility unavailable: nothing can be built now.",
        );
        return;
    }
    native::scope_pops(stack, &mut pops[..count]);
    let builds = PREVIEW_BUILDS.get();
    templates::make_plan(&plan.build, &mut pops[..count], builds);
    for order in &builds.orders[..builds.count] {
        placed[order.unit] += 1;
    }
    p.rows = rows_of(plan, &placed);
    p.planned = true;
    let leaving = !plan.detach.is_empty() && native::leaving(target.id);
    p.summary = text!(
        192,
        "{} to build within land reach, {} to detach{}. Nothing is disbanded.",
        p.rows.build,
        p.rows.detach,
        if leaving { " (already leaving)" } else { "" }
    );
}

fn labels(p: &mut CfPanel) {
    let library = *templates::library();
    let pages = 1.max(library.size.div_ceil(SLOTS));
    p.list_page %= pages;
    let units = catalogue().units.clone();
    let kinds = units.len();
    let universal = templates::army_universal();
    for row in 0..SLOTS {
        let s = p.list_page * SLOTS + row;
        let used = s < library.size;
        show(
            p,
            SHOW_SEL + row,
            text!(64, "tgcx_cf_slot_sel_{row}").with_nul(),
            used && s == p.selected,
        );
        show(
            p,
            SHOW_BLANK + row,
            text!(64, "tgcx_cf_slot_blank_{row}").with_nul(),
            !used,
        );
        let t = CF_T_SLOT_0 + row * (CF_T_SLOT_1 - CF_T_SLOT_0);
        let name = if used {
            templates::name(&library.item[s])
        } else {
            Text::new()
        };
        title_label(p, t, &name);
        let c = &library.item[if used { s } else { 0 }];
        let total = templates::total(c);
        if used {
            text(
                p,
                t + 1,
                format_args!(
                    "{total} {}",
                    if total == 1 { "regiment" } else { "regiments" }
                ),
            );
        } else {
            text(p, t + 1, format_args!(""));
        }
        let (leading, mini, minis) = if used { minis(c) } else { (kinds, [0; 4], 0) };
        for k in 0..MINIS {
            if k < minis {
                text(p, t + 2 + k, format_args!("\u{D7}{}", c.count[mini[k]]));
            } else {
                text(p, t + 2 + k, format_args!(""));
            }
        }
        if universal {
            let key = text!(128, "tgcx_cf_slot_icon_{row}_unit");
            let sprite = text!(96, "tgcx_cf_slot_sprite_{row}_unit");
            univ_icon(
                p.window,
                key.with_nul(),
                sprite.with_nul(),
                leading,
                leading < kinds,
            );
            for k in 0..MINIS {
                let key = text!(128, "tgcx_cf_mini_{row}_{k}_unit");
                let sprite = text!(96, "tgcx_cf_mini_sprite_{row}_{k}_unit");
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
                let key = text!(128, "tgcx_cf_slot_icon_{row}_{}", unit.key);
                show(p, SHOW_ICON + row * kinds + u, key.with_nul(), u == leading);
                for k in 0..MINIS {
                    let key = text!(128, "tgcx_cf_mini_{row}_{k}_{}", unit.key);
                    show(
                        p,
                        SHOW_MINI + (row * MINIS + k) * kinds + u,
                        key.with_nul(),
                        k < minis && mini[k] == u,
                    );
                }
            }
        }
    }
    show(p, SHOW_PAGER, b"tgcx_cf_pager_blank\0", pages == 1);
    if pages > 1 {
        text(
            p,
            CF_T_LIST_PAGE,
            format_args!("{} / {pages}", p.list_page + 1),
        );
    } else {
        text(p, CF_T_LIST_PAGE, format_args!(""));
    }
    for r in 0..CHANGES {
        let used = p.planned && r < p.rows.count;
        let row = p.rows.row[r];
        let k = if used && (0..UNITS as i32).contains(&row.unit) {
            p.kind[row.unit as usize]
        } else {
            -1
        };
        let label = if k >= 0 {
            units[k as usize].label
        } else {
            Text::from_latin1("Regiment")
        };
        show(
            p,
            SHOW_ROW + r,
            text!(64, "tgcx_cf_change_row_{r}").with_nul(),
            used,
        );
        if universal {
            let key = text!(128, "tgcx_cf_change_icon_{r}_unit");
            let sprite = text!(96, "tgcx_cf_change_sprite_{r}_unit");
            let kind = if used && k >= 0 { k as usize } else { kinds };
            univ_icon(
                p.window,
                key.with_nul(),
                sprite.with_nul(),
                kind,
                used && k >= 0,
            );
        } else {
            for (u, unit) in units.iter().enumerate() {
                let key = text!(128, "tgcx_cf_change_icon_{r}_{}", unit.key);
                show(
                    p,
                    SHOW_CHANGE + r * kinds + u,
                    key.with_nul(),
                    used && k == u as i32,
                );
            }
        }
        let t = CF_T_CHANGE_0 + r * (CF_T_CHANGE_1 - CF_T_CHANGE_0);
        if !used {
            text(p, t, format_args!(""));
            text(p, t + 1, format_args!(""));
            continue;
        }
        match row.kind {
            RowKind::Build => {
                text(p, t, format_args!("\u{A7}G+{} {label}\u{A7}!", row.count));
                let days = if k >= 0 { units[k as usize].days } else { 0 };
                text(
                    p,
                    t + 1,
                    format_args!("built; marches and joins ({days} days)"),
                );
            }
            RowKind::Short => {
                text(p, t, format_args!("\u{A7}Y+{} {label}\u{A7}!", row.count));
                text(
                    p,
                    t + 1,
                    format_args!("no eligible pop in land reach: not built"),
                );
            }
            RowKind::Gather => {
                text(p, t, format_args!("\u{A7}G+{} {label}\u{A7}!", row.count));
                text(
                    p,
                    t + 1,
                    format_args!("a finished regiment nearby marches and joins"),
                );
            }
            RowKind::Detach => {
                text(p, t, format_args!("\u{A7}R-{} {label}\u{A7}!", row.count));
                text(p, t + 1, format_args!("detached into a new stack here"));
            }
        }
    }
    text(p, CF_T_TITLE, format_args!("Conform to template"));
    let mode = if p.selected < library.size {
        library.item[p.selected].culture
    } else {
        0
    };
    // A ticked box is the native "on" frame shown beneath its button.
    show(
        p,
        SHOW_CHECK,
        b"tgcx_cf_check_on_accepted\0",
        mode == CULTURE_ACCEPTED,
    );
    show(
        p,
        SHOW_CHECK + 1,
        b"tgcx_cf_check_on_primary\0",
        mode == CULTURE_PRIMARY,
    );
    show(
        p,
        SHOW_CHECK + 3,
        b"tgcx_cf_check_on_noncore\0",
        mode == CULTURE_NONCORE,
    );
    let quota = native::mobo_quota(p.stack);
    show(
        p,
        SHOW_CHECK + 2,
        b"tgcx_cf_check_on_mobo\0",
        quota.is_some(),
    );
    text(p, CF_T_MOBO, format_args!("Mobo stack"));
    text(p, CF_T_CHANGES, format_args!("Changes"));
    let s = p.summary;
    if p.planned && p.rows.more != 0 {
        let more = p.rows.more;
        text(p, CF_T_SUMMARY, format_args!("{s} ({more} more rows)"));
    } else {
        text(p, CF_T_SUMMARY, format_args!("{s}"));
    }
    if !p.status.is_empty() {
        let s = p.status;
        text(p, CF_T_STATUS, format_args!("{s}"));
    } else if let Some(n) = quota {
        text(
            p,
            CF_T_STATUS,
            format_args!("Mobo stack: {n} infantry join when you mobilize."),
        );
    } else if p.planned && p.rows.shortfall != 0 {
        let shortfall = p.rows.shortfall;
        text(
            p,
            CF_T_STATUS,
            format_args!("{shortfall} cannot be built now and are left out."),
        );
    } else {
        text(p, CF_T_STATUS, format_args!(""));
    }
    text(p, CF_T_CONFORM, format_args!("Conform"));
    text(p, CF_T_CANCEL, format_args!("Cancel"));
}

fn open(p: &mut CfPanel, stack: usize) {
    rp_close(p);
    templates::load_library();
    p.stack = native::stack_id(stack);
    p.vtable = rdp(stack);
    if p.selected >= templates::library().size {
        p.selected = 0;
    }
    p.list_page = p.selected / SLOTS;
    p.status.clear();
    if !place(p, !p.open) {
        log!("Conform window does not fit beside the unit panel");
        return;
    }
    // Showing a window re-shows its whole subtree (P42).
    p.open = true;
    visible(p.window, true);
    p.shown = [0; SHOWN];
    p.text_set = [false; CF_TEXTS];
    preview(p);
    p.tick = 0;
    log!(
        "Conform window opened for stack {}/{}",
        p.stack.kind,
        p.stack.id
    );
}

static CLICK_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static CLICK_PLAN: Global<CfPlan> = Global::new(CfPlan::EMPTY);

fn conform_click(p: &mut CfPanel) {
    let library = *templates::library();
    if p.selected >= library.size {
        p.planned = false;
        status(p, "Select a template again.");
        return;
    }
    if !p.planned {
        status(p, "Nothing to do: see above.");
        return;
    }
    let g = native::live_globals();
    let stack = target(&g, p);
    let country = CLICK_COUNTRY.get();
    if stack == 0 || !templates::scan(&templates::live_globals(), country) {
        status(p, "Cannot read this army now.");
        return;
    }
    if !templates::catalogue_fits(country) {
        status(p, "Unit catalogue changed; regenerate Army for this base.");
        return;
    }
    let mut want = want(p, country);
    if native::mobo_quota(p.stack).is_some()
        && let Some(u) = templates::unit_by_name(country, b"infantry")
    {
        want[u] = 0;
    }
    let filter = ArmyFilter {
        culture: library.item[p.selected].culture,
        continent: 0,
    };
    let count = country.pop_count;
    if !templates::army_apply(country.country, &mut country.pops[..count], &filter) {
        status(p, "Culture eligibility unavailable; nothing sent.");
        return;
    }
    let r = native::conform_with(
        &g,
        stack,
        &country.unit,
        &want,
        CLICK_PLAN.get(),
        filter.culture,
    );
    if !r.read {
        status(p, "Cannot read this army now.");
    } else if r.refusal != Refusal::Ok {
        status(p, "Nothing sent: the plan changed. See above.");
    } else {
        let refused = if r.failed != 0 {
            " (some orders refused: see the log)"
        } else {
            ""
        };
        p.status = text!(
            192,
            "{} to detach, {} orders queued, {} left out{refused}. Unpause to process them.",
            r.detached,
            r.queued,
            r.missing
        );
    }
    preview(p);
}

/// The Mobo stack box: marks the open stack with the infantry the selected template asks
/// for, or removes the mark. Conform then leaves infantry out of that stack.
fn mobo_click(p: &mut CfPanel) {
    let g = native::live_globals();
    let stack = target(&g, p);
    if stack == 0 {
        status(p, "Cannot read this army now.");
        return;
    }
    if !native::mobo_enabled() {
        status(
            p,
            "Mobo stacks are unavailable: the executable was not recognised.",
        );
        return;
    }
    if native::mobo_quota(p.stack).is_some() {
        native::mobo_unmark(&g, p.stack);
        status(p, "Mobo mark removed.");
        preview(p);
        return;
    }
    if p.selected >= templates::library().size {
        status(p, "Select a template first.");
        return;
    }
    let country = CLICK_COUNTRY.get();
    if !templates::scan(&templates::live_globals(), country) {
        status(p, "Cannot read the player's unit types.");
        return;
    }
    let want = want(p, country);
    let Some(u) = templates::unit_by_name(country, b"infantry") else {
        status(p, "This country has no infantry unit type.");
        return;
    };
    if want[u] <= 0 {
        status(p, "This template has no infantry to bring in.");
    } else if native::mobo_mark(&g, stack, want[u]) {
        p.status = text!(
            192,
            "Marked: {} infantry join when you mobilize. Conform builds the rest.",
            want[u]
        );
    } else {
        status(p, "Could not keep the mark: see the log.");
    }
    preview(p);
}

// ---------------------------------------------------------------- Replace depleted (TPL-12)

// Board 6-replace-button-1024: Replace's header button opens an independent chooser.
// Its culture boxes and nine-step slider choose the recruits and depletion rule (saved
// in tgcx_replace_settings.txt); Replace builds one regiment for each depleted one and
// conform_native detaches the old one once its replacement has merged.

fn rp_close(p: &mut CfPanel) {
    p.rp_open = false;
    if p.chooser != 0 {
        visible(p.chooser, false);
    }
}

fn rp_place(p: &mut CfPanel, promote: bool) -> bool {
    place_panel(p, p.chooser, CF_RP_WIDTH, CF_RP_HEIGHT, promote)
}

fn rp_show(p: &mut CfPanel, id: usize, key: &[u8], on: bool) {
    let want = if on { 1 } else { 2 };
    if p.rp_shown[id] == want {
        return;
    }
    let window = find(p.chooser, 27, 0x5E4A20, key);
    if window == 0 {
        return;
    }
    visible(window, on);
    p.rp_shown[id] = want;
}

static RP_COUNTRY: Global<TplCountry> = Global::new(TplCountry::ZERO);
static RP_FOUND: Global<Vec<(crate::replace::Health, Why)>> = Global::new(Vec::new());

/// What Replace would do now under the saved choice.
fn rp_preview(p: &mut CfPanel) {
    p.rp_summary.clear();
    p.rp_list.clear();
    let g = native::live_globals();
    p.preview_date = if g.world != 0 {
        rd32(g.world + 0xB0C)
    } else {
        0
    };
    let country = RP_COUNTRY.get();
    let stack = target(&g, p);
    if stack == 0 || !templates::scan(&templates::live_globals(), country) {
        p.rp_summary = Text::from_latin1("This army can no longer be read: select it again.");
        return;
    }
    want(p, country); // the unit -> catalogue map the labels use
    let choice = native::replace_choice();
    let found = RP_FOUND.get();
    if !native::depleted_of(&g, stack, &country.unit, &choice, found) {
        p.rp_summary = Text::from_latin1("This army can no longer be read: select it again.");
        return;
    }
    if !choice.strength && !choice.pop {
        p.rp_summary = Text::from_latin1("Tick what counts as depleted.");
        return;
    }
    match found.len() {
        0 => p.rp_summary = Text::from_latin1("No depleted regiment under this choice."),
        1 => p.rp_summary = Text::from_latin1("1 regiment to replace"),
        n => p.rp_summary = text!(192, "{n} regiments to replace"),
    }
    let units = &catalogue().units;
    let mut list = String::new();
    for (i, (h, why)) in found.iter().enumerate() {
        let k = p.kind[h.unit as usize];
        let label = if k >= 0 {
            units[k as usize].label.to_string()
        } else {
            "Regiment".to_string()
        };
        let item = match why {
            Why::Strength => format!("{label} ({}%)", h.strength),
            Why::Pop => format!("{label} (pop)"),
        };
        if list.len() + item.len() + 2 > 100 {
            list.push_str(&format!(" and {} more", found.len() - i));
            break;
        }
        if !list.is_empty() {
            list.push_str(", ");
        }
        list.push_str(&item);
    }
    if !list.is_empty() {
        list.push('.');
    }
    p.rp_list = Text::from_latin1(&list);
}

fn rp_labels(p: &mut CfPanel) {
    let choice = native::replace_choice();
    rp_show(p, 0, b"tgcx_cf_check_on_rp_strength\0", choice.strength);
    rp_show(p, 1, b"tgcx_cf_check_on_rp_pop\0", choice.pop);
    for (i, name, mode) in [
        (
            2,
            b"tgcx_cf_check_on_rp_culture_accepted\0".as_slice(),
            CULTURE_ACCEPTED,
        ),
        (
            3,
            b"tgcx_cf_check_on_rp_culture_primary\0".as_slice(),
            CULTURE_PRIMARY,
        ),
        (
            4,
            b"tgcx_cf_check_on_rp_culture_noncore\0".as_slice(),
            CULTURE_NONCORE,
        ),
    ] {
        rp_show(p, i, name, choice.culture == mode);
    }
    for i in 0..CF_RP_STEPS as usize {
        let key = text!(64, "tgcx_cf_rp_handle_{i}");
        rp_show(p, 5 + i, key.with_nul(), i as u32 == choice.step());
    }
    text(p, CF_T_RP_TITLE, format_args!("Replace depleted"));
    text(p, CF_T_RP_STRENGTH, format_args!("Low strength, below"));
    text(p, CF_T_RP_VALUE, format_args!("{}%", choice.below));
    text(p, CF_T_RP_POP, format_args!("Pop too small"));
    text(p, CF_T_RP_CULTURE, format_args!("Culture"));
    text(
        p,
        CF_T_RP_CULTURE_ACCEPTED,
        format_args!("Primary + accepted"),
    );
    text(p, CF_T_RP_CULTURE_PRIMARY, format_args!("Primary only"));
    text(p, CF_T_RP_CULTURE_NONCORE, format_args!("Non-core only"));
    let (summary, list) = (p.rp_summary, p.rp_list);
    text(p, CF_T_RP_SUMMARY, format_args!("{summary}"));
    text(p, CF_T_RP_LIST, format_args!("{list}"));
    let status = p.rp_status;
    if status.is_empty() {
        text(
            p,
            CF_T_RP_STATUS,
            format_args!("Built first within land reach; old ones are detached as they merge."),
        );
    } else {
        text(p, CF_T_RP_STATUS, format_args!("{status}"));
    }
    text(p, CF_T_RP_REPLACE, format_args!("Replace"));
    text(p, CF_T_RP_CANCEL, format_args!("Cancel"));
}

fn rp_open(p: &mut CfPanel) {
    templates::load_library();
    if !native::replace_enabled() {
        status(
            p,
            "Replace depleted is unavailable: the executable was not recognised.",
        );
        return;
    }
    if p.chooser == 0 {
        status(
            p,
            "The Replace depleted window could not be created: see the log.",
        );
        return;
    }
    if !rp_place(p, true) {
        log!("Replace depleted window does not fit beside the unit panel");
        return;
    }
    close(p);
    p.rp_open = true;
    p.rp_status.clear();
    visible(p.chooser, true);
    // Showing a window re-shows its whole subtree (P42).
    p.rp_shown = [0; RP_SHOWN];
    for id in CF_T_RP_TITLE..=CF_T_RP_CANCEL {
        p.text_set[id] = false;
    }
    rp_preview(p);
    rp_labels(p);
}

fn rp_replace(p: &mut CfPanel) {
    let g = native::live_globals();
    let stack = target(&g, p);
    let country = RP_COUNTRY.get();
    if stack == 0 || !templates::scan(&templates::live_globals(), country) {
        p.rp_status = Text::from_latin1("Cannot read this army now.");
        return;
    }
    if !templates::catalogue_fits(country) {
        p.rp_status = Text::from_latin1("Unit catalogue changed; regenerate Army for this base.");
        return;
    }
    let choice = native::replace_choice();
    let culture = choice.culture;
    let filter = ArmyFilter {
        culture,
        continent: 0,
    };
    let count = country.pop_count;
    if !templates::army_apply(country.country, &mut country.pops[..count], &filter) {
        p.rp_status = Text::from_latin1("Culture eligibility unavailable; nothing sent.");
        return;
    }
    let r = native::replace_with(&g, stack, &country.unit, &choice, culture);
    p.rp_status = if !r.read {
        Text::from_latin1("Cannot read this army now.")
    } else if r.found == 0 {
        Text::from_latin1("Nothing to replace under this choice.")
    } else {
        let refused = if r.failed != 0 {
            " (some refused: see the log)"
        } else {
            ""
        };
        text!(
            192,
            "{} replacements queued, {} left out{refused}. Unpause to process them.",
            r.queued,
            r.missing
        )
    };
    rp_preview(p);
}

fn rp_click(p: &mut CfPanel, action: usize) {
    if !p.rp_open {
        return;
    }
    let mut choice = native::replace_choice();
    match action {
        CF_B_RP_CANCEL => {
            rp_close(p);
            return;
        }
        CF_B_RP_REPLACE => {
            rp_replace(p);
            return;
        }
        CF_B_RP_STRENGTH => choice.strength = !choice.strength,
        CF_B_RP_POP => choice.pop = !choice.pop,
        CF_B_RP_CULTURE_ACCEPTED => choice.toggle_culture(CULTURE_ACCEPTED),
        CF_B_RP_CULTURE_PRIMARY => choice.toggle_culture(CULTURE_PRIMARY),
        CF_B_RP_CULTURE_NONCORE => choice.toggle_culture(CULTURE_NONCORE),
        CF_B_RP_DOWN => choice.nudge(false),
        CF_B_RP_UP => choice.nudge(true),
        CF_B_RP_STEP_0..=CF_B_RP_STEP_8 => choice.set_step((action - CF_B_RP_STEP_0) as u32),
        _ => return,
    }
    p.rp_status = if native::set_replace_choice(choice) {
        Text::new()
    } else {
        Text::from_latin1("Could not save this choice; it holds for this session.")
    };
    rp_preview(p);
}

fn click_inner(p: &mut CfPanel, action: usize) {
    if p.disabled || !panel_live(p) {
        return;
    }
    if action == CF_B_OPEN || action == CF_B_REPLACE {
        let stack = shown_stack(&native::live_globals(), rdp(p.view + 0x28));
        let active = if action == CF_B_OPEN {
            p.open
        } else {
            p.rp_open
        };
        if active && stack != 0 && native::stack_id(stack) == p.stack {
            close(p);
            return;
        }
        if stack != 0 && p.land {
            if action == CF_B_OPEN {
                open(p, stack);
            } else {
                p.stack = native::stack_id(stack);
                p.vtable = rdp(stack);
                rp_open(p);
            }
        }
        return;
    }
    if (CF_B_RP_STRENGTH..=CF_B_RP_CANCEL).contains(&action) {
        rp_click(p, action);
        p.tick = 0;
        return;
    }
    if !p.open {
        return;
    }
    p.tick = 0;
    let library = *templates::library();
    if (CF_B_CULTURE_ACCEPTED..=CF_B_CULTURE_NONCORE).contains(&action) {
        if p.selected >= library.size {
            return;
        }
        let mut next = library;
        let mode = match action {
            CF_B_CULTURE_ACCEPTED => CULTURE_ACCEPTED,
            CF_B_CULTURE_PRIMARY => CULTURE_PRIMARY,
            _ => CULTURE_NONCORE,
        };
        let choice = &mut next.item[p.selected].culture;
        *choice = if *choice == mode { CULTURE_ALL } else { mode };
        if !templates::commit(&next) {
            status(
                p,
                "Could not save culture preference; previous choice kept.",
            );
            return;
        }
        p.status.clear();
        preview(p);
        return;
    }
    if action == CF_B_CLOSE || action == CF_B_CANCEL {
        close(p);
        return;
    }
    if (CF_B_SLOT_0..=CF_B_SLOT_3).contains(&action) {
        let s = p.list_page * SLOTS + action - CF_B_SLOT_0;
        if s < library.size && s != p.selected {
            p.selected = s;
            p.status.clear();
            preview(p);
        }
        return;
    }
    if action == CF_B_LIST_PREV || action == CF_B_LIST_NEXT {
        let pages = 1.max(library.size.div_ceil(SLOTS));
        p.list_page = (p.list_page
            + if action == CF_B_LIST_NEXT {
                1
            } else {
                pages - 1
            })
            % pages;
        return;
    }
    if action == CF_B_MOBO {
        mobo_click(p);
        return;
    }
    if action == CF_B_CONFORM {
        conform_click(p);
    }
}

extern "C" fn cf_click(p: *mut CfPanel, action: usize) {
    let _cpu = CpuScope::new();
    if !native::enabled() || p.is_null() || action >= CF_BUTTONS {
        return;
    }
    // SAFETY: the panel this button's glue was bound to, one of PANELS.
    let p = unsafe { &mut *p };
    if thread_id() != p.thread {
        return;
    }
    if guard(|| click_inner(p, action)).is_none() {
        log!("Conform click rejected: guarded native read fault");
    }
}

/// Every Conform stub pushes its action and jumps here; the native trampoline passes
/// our panel in ECX: `cf_click(panel, action)` with every register and the FPU
/// preserved.
#[unsafe(naked)]
pub unsafe extern "C" fn cf_thunk_common() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+36]",
        "push ecx",
        "call {click}",
        "add esp, 8",
        crate::restore_cpu!(),
        "lea esp, [esp+4]",
        "ret",
        click = sym cf_click,
    )
}

fn bind(p: &mut CfPanel, action: usize) {
    if p.bound[action] {
        return;
    }
    // Lookups are flat: the open button lives in the tab window.
    let host = if action == CF_B_OPEN || action == CF_B_REPLACE {
        p.tab
    } else if CF_BUTTON_NAMES[action].starts_with("tgcx_cf_rp_") {
        p.chooser
    } else {
        p.window
    };
    if host == 0 {
        return;
    }
    let button = find_named(host, 13, 0x5E4C80, CF_BUTTON_NAMES[action]);
    if button == 0 {
        return;
    }
    // SAFETY: the stub table build.rs generated, one per action.
    let stub = unsafe { crate::thunks::tgcx_cf_table[action] };
    // from V2DLL: V2TechButton.cpp, 44-byte Politics observer. Its deletion
    // notification uses +0x20 (left zero); +4 is our stable panel, +8 callback.
    let panel = &raw mut *p as usize;
    if bind_observer(button, &mut p.glue[action], panel, stub) {
        p.bound[action] = true;
    }
}

fn reset(p: &mut CfPanel) {
    univ_release(&mut p.chooser);
    p.chooser = 0;
    p.rp_open = false;
    p.rp_shown = [0; RP_SHOWN];
    univ_release(&mut p.window);
    univ_release(&mut p.tab);
    p.root = 0;
    p.window = 0;
    p.tab = 0;
    p.seen = 0;
    p.open = false;
    p.land = false;
    p.disabled = false;
    p.tick = 0;
    p.bound = [false; CF_BUTTONS];
    p.text_set = [false; CF_TEXTS];
    p.shown = [0; SHOWN];
    p.title_font_checked = false;
    p.title_font_ready = false;
    // Observer memory stays valid until native GUI destruction.
}

/// The previous root can already be destroyed: never hide its cached children.
fn root_changed(p: &mut CfPanel, root: usize) {
    if root == p.root {
        return;
    }
    reset(p);
    p.root = root;
}

pub fn panel_release(view: usize) {
    for p in PANELS.get().iter_mut().filter(|p| p.view == view) {
        let window = p.window;
        let chooser = p.chooser;
        let _ = guard(|| {
            if window != 0 {
                visible(window, false);
            }
            if chooser != 0 {
                visible(chooser, false);
            }
        });
        reset(p);
        p.view = 0;
    }
}

/// The step the panel tick was at, for the fault log line.
static STEP: Global<&'static str> = Global::new("view/root");

fn set_step(step: &'static str) {
    // SAFETY: a plain store to our own static, kept before every native call.
    unsafe { std::ptr::write_volatile(STEP.get(), step) };
}

fn panel_tick_inner(p: &mut CfPanel) {
    if thread_id() != p.thread {
        return;
    }
    if p.view == 0 || rdp(p.view) != base() + VIEW_VT_RVA as usize {
        return;
    }
    let root = rdp(p.view + 0x20);
    root_changed(p, root);
    set_step("view/stack");
    let shown = rdp(p.view + 0x28);
    if p.disabled {
        if shown == p.seen {
            return;
        }
        // A new selection can recover without re-registering observers.
        p.disabled = false;
        p.tick = 0;
        p.land = false;
    }
    set_step("visibility");
    let universal = templates::army_universal();
    if !panel_live(p) {
        if p.open || p.rp_open {
            close(p);
        }
        // Universal's entry is a separate scene root: hiding its host does not hide
        // it. Reset visibility so reopening promotes it.
        if universal {
            visible(p.tab, false);
        }
        return;
    }
    let now = crate::sys::tick();
    if p.tick != 0 && now.wrapping_sub(p.tick) < 100 {
        return;
    }
    p.tick = now;
    if p.window == 0 {
        if universal {
            let (mut window, mut tab) = (0, 0);
            if !univ_pair(
                p.root,
                "tgcx_cf_window",
                "tgcx_cf_tab",
                &mut window,
                &mut tab,
            ) {
                return;
            }
            p.window = window;
            p.tab = tab;
            p.chooser = univ_make(p.root, "tgcx_cf_rp_window");
        } else {
            set_step("marker/window/tab lookup");
            // Without our GUI fragment there is nothing to bind: the button lookup is
            // silent, a window lookup is not. It searches only the panel's own
            // children, so the probe is the top-level marker.
            if find_named(p.root, 13, 0x5E4C80, "tgcx_cf_marker") == 0 {
                return;
            }
            p.window = find_named(p.root, 27, 0x5E4A20, "tgcx_cf_window");
            if p.window == 0 {
                return;
            }
            p.tab = find_named(p.root, 27, 0x5E4A20, "tgcx_cf_tab");
            p.chooser = find_named(p.root, 27, 0x5E4A20, "tgcx_cf_rp_window");
            visible(p.window, false);
            if p.chooser != 0 {
                visible(p.chooser, false);
            }
            log!(
                "Conform native window found view={:08X} root={:08X} tab={}",
                p.view,
                p.root,
                i32::from(p.tab != 0)
            );
        }
    }
    set_step("stack registry/owner/land");
    let g = native::live_globals();
    let stack = shown_stack(&g, shown);
    p.seen = shown;
    p.land = stack != 0 && rdi(stack + 0xC4) == native::player(&g) && native::land(stack);
    // Another army selected: the window belonged to the previous one.
    if (p.open || p.rp_open) && (stack == 0 || native::stack_id(stack) != p.stack) {
        close(p);
    }
    // Fleets and foreign stacks have no Conform. Re-applied every tick: showing the
    // unit panel re-shows its subtree (P42).
    set_step("tab visibility");
    let placed = !universal || univ_cf_tab(p.root, p.tab);
    visible(p.tab, p.land && placed);
    set_step("bind");
    for b in 0..CF_BUTTONS {
        bind(p, b);
    }
    if p.open || p.rp_open {
        set_step("place");
        if !(if p.rp_open {
            rp_place(p, false)
        } else {
            place(p, false)
        }) {
            close(p);
            return;
        }
        // Once per game day: regiments arrive, merge, or finish building.
        set_step("preview");
        let g = native::live_globals();
        if g.world != 0
            && (rd32(g.world + 0xB0C) != p.preview_date
                || (p.open && p.filter_revision != templates::STORE.get().revision))
        {
            if p.rp_open {
                rp_preview(p);
            } else {
                preview(p);
            }
        }
        set_step("labels");
        if p.rp_open {
            set_step("chooser");
            rp_labels(p);
        } else {
            labels(p);
        }
    }
}

fn panel_tick(p: &mut CfPanel) {
    set_step("view/root");
    if guard(|| panel_tick_inner(p)).is_none() {
        p.open = false;
        p.rp_open = false;
        let (window, chooser) = (p.window, p.chooser);
        let _ = guard(|| {
            if window != 0 {
                visible(window, false);
            }
            if chooser != 0 {
                visible(chooser, false);
            }
        });
        if !p.disabled {
            log!(
                "Conform window disabled after guarded GUI read fault: step={} view={:08X} root={:08X} stack={:08X}",
                *STEP.get(),
                p.view,
                p.root,
                p.seen
            );
        }
        p.disabled = true;
    }
}

pub fn panel_forget(view: usize) {
    for p in PANELS.get().iter_mut().filter(|p| p.view == view) {
        reset(p);
        p.view = 0;
    }
}

/// The panel of a captured view; a panel whose view left the table is freed first
/// (P47), so slots never run out while views come and go.
fn panel_for(v: &CfView) -> Option<&'static mut CfPanel> {
    let views = &VIEWS_STATE.get().views;
    for p in PANELS.get().iter_mut() {
        if p.view != 0 && !views.iter().any(|w| w.view == p.view) {
            reset(p);
            p.view = 0;
        }
    }
    let panels = PANELS.get();
    if let Some(i) = panels.iter().position(|p| p.view == v.view) {
        return Some(&mut PANELS.get()[i]);
    }
    let i = panels.iter().position(|p| p.view == 0)?;
    let p = &mut PANELS.get()[i];
    reset(p);
    p.view = v.view;
    p.thread = v.thread;
    p.disabled = false;
    Some(p)
}

pub fn panels_tick() {
    if !native::enabled() {
        return;
    }
    for i in 0..VIEWS {
        let v = VIEWS_STATE.get().views[i];
        if v.view == 0 {
            continue;
        }
        if let Some(p) = panel_for(&v) {
            panel_tick(p);
        }
    }
}

#[cfg(test)]
mod title_tests {
    use super::fit_line;

    #[test]
    fn long_composition_cannot_wrap_over_the_regiment_count() {
        let name = b"5 Inf / 4 Art / 1 Eng / 1 Hus";
        let measure = |s: &[u8]| Some(s.len() as i32 * 6);
        let shown = fit_line(name, 128, measure);
        assert!(shown.bytes().ends_with(b"..."));
        assert!(measure(shown.bytes()).unwrap() <= 128);
        assert!(!shown.bytes().contains(&b'\n'));
        assert_eq!(
            fit_line(b"1 Inf / 1 Art / 1 Hus", 128, measure).bytes(),
            b"1 Inf / 1 Art / 1 Hus"
        );
    }

    #[test]
    fn proportional_widths_and_cp1252_are_measured_not_character_counted() {
        let measure = |s: &[u8]| {
            Some(
                s.iter()
                    .map(|&c| match c {
                        b'W' => 10,
                        b'i' => 2,
                        b'.' => 1,
                        _ => 6,
                    })
                    .sum(),
            )
        };
        assert_eq!(fit_line(b"iiiiiiii", 16, measure).bytes(), b"iiiiiiii");
        assert_eq!(fit_line(b"WWWW", 16, measure).bytes(), b"W...");
        assert_eq!(fit_line(b"\xC9lite", 40, measure).bytes(), b"\xC9lite");
    }

    #[test]
    fn failed_measurement_and_embedded_line_breaks_cannot_leak_overflow() {
        assert!(fit_line(b"Any title", 128, |_| None).is_empty());
        assert!(fit_line(b"Any title", 1, |_| Some(99)).is_empty());
        let shown = fit_line(b"Infantry\nHussar", 128, |s| Some(s.len() as i32));
        assert_eq!(shown.bytes(), b"Infantry...");
    }
}
