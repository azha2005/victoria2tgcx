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
use crate::sys::{CpuScope, Global, Text, base, guard, rd8, rd32, rdi, rdp, slot, thread_id};
use crate::templates::{
    self, ArmyFilter, CULTURE_ACCEPTED, CULTURE_ALL, CULTURE_PRIMARY, POPS, TplCountry, TplPlan,
    TplPop, UNITS, catalogue,
};
use crate::templates_gui::{
    minis, place_window, root_screen, univ_cf_tab, univ_icon, univ_pair, univ_release, visible,
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
const SHOW_END: usize = SHOW_CHECK + 3;
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
}

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
    set_text_native(
        p.window,
        Text::<128>::from_latin1(CF_TEXT_NAMES[id]).with_nul(),
        t.with_nul(),
    );
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
}

/// Beside the unit panel (its Select button ends at x=435), inside the screen. The
/// owner chain and the native setters are TPL-0's (P37, P40).
fn place(p: &CfPanel, promote: bool) -> bool {
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
    let (width, height) = (rdi(renderer + 0x6B9BC), rdi(renderer + 0x6B9C0));
    if width < CF_WIDTH || height < CF_HEIGHT {
        return false;
    }
    if promote {
        log!(
            "Conform anchor: unit panel screen {x:.0},{y:.0} through {depth} owner levels, screen {width}x{height}"
        );
    }
    x += 331.0;
    y += 26.0;
    if x + CF_WIDTH as f32 > width as f32 {
        x = (width - CF_WIDTH) as f32;
    }
    if y + CF_HEIGHT as f32 > height as f32 {
        y = (height - CF_HEIGHT) as f32;
    }
    x = x.max(0.0);
    y = y.max(0.0);
    place_window(p.window, x, y, promote);
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
        text(p, t, format_args!("{name}"));
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

fn click_inner(p: &mut CfPanel, action: usize) {
    if p.disabled || !panel_live(p) {
        return;
    }
    if action == CF_B_OPEN {
        let stack = shown_stack(&native::live_globals(), rdp(p.view + 0x28));
        if p.open && stack != 0 && native::stack_id(stack) == p.stack {
            close(p);
            return;
        }
        if stack != 0 && p.land {
            open(p, stack);
        }
        return;
    }
    if !p.open {
        return;
    }
    p.tick = 0;
    let library = *templates::library();
    if action == CF_B_CULTURE_ACCEPTED || action == CF_B_CULTURE_PRIMARY {
        if p.selected >= library.size {
            return;
        }
        let mut next = library;
        let mode = if action == CF_B_CULTURE_ACCEPTED {
            CULTURE_ACCEPTED
        } else {
            CULTURE_PRIMARY
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
    let host = if action == CF_B_OPEN { p.tab } else { p.window };
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
        let _ = guard(|| {
            if window != 0 {
                visible(window, false);
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
        if p.open {
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
            visible(p.window, false);
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
    if p.open && (stack == 0 || native::stack_id(stack) != p.stack) {
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
    if p.open {
        set_step("place");
        if !place(p, false) {
            close(p);
            return;
        }
        // Once per game day: regiments arrive, merge, or finish building.
        set_step("preview");
        let g = native::live_globals();
        if g.world != 0
            && (rd32(g.world + 0xB0C) != p.preview_date
                || p.filter_revision != templates::STORE.get().revision)
        {
            preview(p);
        }
        set_step("labels");
        labels(p);
    }
}

fn panel_tick(p: &mut CfPanel) {
    set_step("view/root");
    if guard(|| panel_tick_inner(p)).is_none() {
        p.open = false;
        let window = p.window;
        let _ = guard(|| {
            if window != 0 {
                visible(window, false);
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
