//! DLL-1 + HEMI-3: guarded presentation callbacks and native scalar chart hover; port
//! of tgcx_renderer.h.

use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::{HMODULE, RECT};
use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    GetModuleFileNameA, GetModuleHandleExA, GetModuleHandleW, GetProcAddress,
};
use windows_sys::Win32::System::Memory::{
    PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect,
};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

use crate::d3d::{self, Device, HResult, S_FALSE, S_OK, Viewport, succeeded};
use crate::signatures::{IDEOLOGY_CANDIDATES, IMAGE_SIZE};
use crate::snapshot::{self, IDEOLOGIES, Snapshot, find_ideology};
use crate::sys::{
    Bytes, CpuScope, Global, SrwLock, at, base, c_str, check_signature, executable_pointer, guard,
    rd8, rd32, rdi, rdp, rel32, replace_pointer, signature, slot, tick, wr,
};
use crate::{bill, conform_native, log, templates_gui};

pub struct Renderer {
    /// The CPoliticsView shown now, or 0.
    pub politics: usize,
    /// MOD-1: drawing (hemicycle, hover, Bill) has its own gate; the stages tick without it.
    drawing_enabled: bool,
    chart_tick: u32,
    pub chart_drawn: bool,
    guard_log_tick: u32,
    guard_logged: bool,
    gui_stage: &'static str,
    hover_logged: u32,
    hover_stage: &'static str,
    present_count: u32,
    draw_count: u32,
    render_failed: bool,
}

pub static R: Global<Renderer> = Global::new(Renderer {
    politics: 0,
    drawing_enabled: false,
    chart_tick: 0,
    chart_drawn: false,
    guard_log_tick: 0,
    guard_logged: false,
    gui_stage: "not read",
    hover_logged: 0,
    hover_stage: "no rendered frame",
    present_count: 0,
    draw_count: 0,
    render_failed: false,
});

pub fn drawing_enabled() -> bool {
    R.get().drawing_enabled
}

pub fn politics() -> usize {
    R.get().politics
}

static SHOW_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static HIDE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static DESTROY_ORIGINAL: AtomicUsize = AtomicUsize::new(0);

extern "C" fn show_politics(view: usize) {
    crate::sys::keep_last_error(|| {
        let r = R.get();
        r.politics = view;
        r.chart_drawn = false;
        r.guard_logged = false;
        clear_hover();
        log!("Politics shown view={view:08X}");
    })
}

extern "C" fn hide_politics(view: usize) {
    crate::sys::keep_last_error(|| {
        if R.get().politics == view {
            bill::reset(false);
            let r = R.get();
            r.politics = 0;
            r.chart_drawn = false;
            clear_hover();
            log!("Politics hidden/destroyed view={view:08X}");
        }
    })
}

extern "C" fn destroy_politics(view: usize) {
    hide_politics(view);
    bill::reset(true);
}

crate::view_thunk!(show_thunk, show_politics, SHOW_ORIGINAL);
crate::view_thunk!(hide_thunk, hide_politics, HIDE_ORIGINAL);
crate::view_thunk!(destroy_thunk, destroy_politics, DESTROY_ORIGINAL);

#[derive(Clone, Copy, Default, PartialEq)]
pub struct GuiRect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

pub fn read_politics_rect() -> Option<GuiRect> {
    guard(|| {
        let r = R.get();
        r.gui_stage = "Politics identity";
        if r.politics == 0 || rdp(r.politics) != base() + 0xA0E458 {
            return None;
        }
        r.gui_stage = "root visibility";
        let root = rdp(r.politics + 0x4C);
        if root == 0 || rd8(root + 0x67) == 0 {
            return None;
        }
        r.gui_stage = "root methods";
        if slot(root, 8) != base() + 0x5E4450 || slot(root, 9) != base() + 0x5E4030 {
            return None;
        }
        r.gui_stage = "renderer dimensions";
        let renderer = rdp(root + 0x90);
        if renderer == 0 {
            return None;
        }
        let width = rdi(renderer + 0x6B9BC);
        let height = rdi(renderer + 0x6B9C0);
        if !(640..=16384).contains(&width) || !(480..=16384).contains(&height) {
            return None;
        }
        r.gui_stage = "window position";
        let position = rd32(root + 0x250);
        let rect = GuiRect {
            left: position as i16 as i32,
            top: (position >> 16) as i16 as i32,
            width,
            height,
        };
        (rect.left >= 0 && rect.top >= 0 && rect.left + 370 <= width && rect.top + 210 <= height)
            .then_some(rect)
    })
    .flatten()
}

pub fn allocate_seats(sample: &Snapshot) -> Option<[u32; IDEOLOGIES]> {
    if !sample.shares_in_range || sample.sum < 32768 - IDEOLOGIES as i64 || sample.sum > 32768 {
        return None;
    }
    let mut seats = [0u32; IDEOLOGIES];
    let mut remainders = [0i64; IDEOLOGIES];
    let mut assigned = 0;
    for i in 0..IDEOLOGIES {
        if !(0..=32768).contains(&sample.shares[i]) {
            return None;
        }
        let numerator = sample.shares[i] * 100;
        seats[i] = (numerator / sample.sum) as u32;
        assigned += seats[i];
        remainders[i] = numerator % sample.sum;
    }
    while assigned < 100 {
        let mut best = 0;
        for i in 1..IDEOLOGIES {
            if remainders[i] > remainders[best]
                || (remainders[i] == remainders[best] && sample.indices[i] < sample.indices[best])
            {
                best = i;
            }
        }
        seats[best] += 1;
        assigned += 1;
        remainders[best] = -1;
    }
    (assigned == 100).then_some(seats)
}

#[derive(Clone, Copy, Default)]
pub struct Seat {
    pub x: f32,
    pub y: f32,
    pub ideology: usize,
}

pub const SEAT_RADIUS: f32 = 2.3;

#[derive(Clone, Copy)]
struct HoverFrame {
    view: usize,
    entity: usize,
    gui: GuiRect,
    sample: Snapshot,
    dots: [Seat; 100],
    tick: u32,
}

static HOVER_LOCK: SrwLock = SrwLock::new();
static HOVER: Global<HoverFrame> = Global::new(HoverFrame {
    view: 0,
    entity: 0,
    gui: GuiRect {
        left: 0,
        top: 0,
        width: 0,
        height: 0,
    },
    sample: Snapshot {
        date_hours: 0,
        index: 0,
        country_index: 0,
        human_entries: 0,
        human_marker: 0,
        world: 0,
        country: 0,
        upper_house: 0,
        array: 0,
        upper_house_vtable: 0,
        world_tag: [0; 4],
        country_tag: [0; 4],
        shares: [0; IDEOLOGIES],
        sum: 0,
        shares_in_range: false,
        indices: [0; IDEOLOGIES],
        colors: [0; IDEOLOGIES],
        array_entries: 0,
    },
    dots: [Seat {
        x: 0.0,
        y: 0.0,
        ideology: 0,
    }; 100],
    tick: 0,
});

fn clear_hover() {
    let _lock = HOVER_LOCK.exclusive();
    HOVER.get().view = 0;
}

fn seat_at(dots: &[Seat; 100], x: f32, y: f32) -> Option<usize> {
    // Match the visible outline, excluding its transparent feather and spaces between seats.
    dots.iter().position(|d| {
        let (dx, dy) = (x - d.x, y - d.y);
        dx * dx + dy * dy <= SEAT_RADIUS * SEAT_RADIUS
    })
}

/// Left-to-right seating by candidate index (IDEOLOGY_CANDIDATES). Presentation only;
/// engine indices/colours are still resolved by name. Main: Liberal left, Conservative centre, Reactionary right.
const SEATING_ORDER: [usize; 9] = [7, 6, 8, 5, 4, 0, 1, 3, 2];
const _: () = assert!(
    SEATING_ORDER.len() == IDEOLOGIES,
    "every ideology has a seating position"
);

pub fn seat_geometry(seats: &[u32; IDEOLOGIES]) -> [Seat; 100] {
    let mut dots = [Seat::default(); 100];
    // Seven rows, 6 px apart; seats per row roughly proportional to radius (Main).
    const COUNTS: [u32; 7] = [21, 18, 16, 14, 12, 10, 9];
    let mut angles = [0f32; 100];
    let mut n = 0;
    for (ring, &count) in COUNTS.iter().enumerate() {
        let radius = 58.0f32 - 6.0 * ring as f32;
        for seat in 0..count {
            angles[n] = std::f32::consts::PI * (1.0 - seat as f32 / (count - 1) as f32);
            dots[n] = Seat {
                x: 99.0 + radius * angles[n].cos(),
                y: 183.0 - radius * angles[n].sin(),
                ideology: 0,
            };
            n += 1;
        }
    }
    // Sweep left to right: by angle, then by x so the outermost seats close each end of the arc.
    let mut order: [usize; 100] = std::array::from_fn(|i| i);
    for i in 1..100 {
        for j in (1..=i).rev() {
            let (a, b) = (order[j - 1], order[j]);
            if angles[b] > angles[a] || (angles[b] == angles[a] && dots[b].x < dots[a].x) {
                order[j - 1] = b;
                order[j] = a;
            } else {
                break;
            }
        }
    }
    let mut position = 0;
    let mut remaining = seats[SEATING_ORDER[0]];
    for &seat in &order {
        while remaining == 0 && position + 1 < IDEOLOGIES {
            position += 1;
            remaining = seats[SEATING_ORDER[position]];
        }
        dots[seat].ideology = SEATING_ORDER[position];
        remaining = remaining.saturating_sub(1);
    }
    dots
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    rhw: f32,
    color: u32,
}

pub const FVF: u32 = d3d::FVF_XYZRHW | d3d::FVF_DIFFUSE;

fn v(x: f32, y: f32, color: u32) -> Vertex {
    Vertex {
        x: x - 0.5,
        y: y - 0.5,
        z: 0.0,
        rhw: 1.0,
        color,
    }
}

#[allow(clippy::too_many_arguments)]
fn quad(
    out: &mut [Vertex],
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    c00: u32,
    c10: u32,
    c01: u32,
    c11: u32,
) {
    out[0] = v(x0, y0, c00);
    out[1] = v(x1, y0, c10);
    out[2] = v(x0, y1, c01);
    out[3] = v(x1, y0, c10);
    out[4] = v(x1, y1, c11);
    out[5] = v(x0, y1, c01);
}

/// Opaque panel over the native pie, fading to transparent over `feather` px outside it,
/// so it blends into the Politics window instead of ending in a hard edge.
pub fn soft_panel(
    device: Device,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    feather: f32,
    color: u32,
) -> HResult {
    let (c, t) = (color, color & 0x00FFFFFF);
    let (l, r, u, d) = (x, x + width, y, y + height);
    let (ll, rr, uu, dd) = (l - feather, r + feather, u - feather, d + feather);
    let mut vx = [v(0.0, 0.0, 0); 54];
    quad(&mut vx[0..], l, u, r, d, c, c, c, c);
    quad(&mut vx[6..], l, uu, r, u, t, t, c, c);
    quad(&mut vx[12..], l, d, r, dd, c, c, t, t);
    quad(&mut vx[18..], ll, u, l, d, t, c, t, c);
    quad(&mut vx[24..], r, u, rr, d, c, t, c, t);
    quad(&mut vx[30..], ll, uu, l, u, t, t, t, c);
    quad(&mut vx[36..], r, uu, rr, u, t, t, c, t);
    quad(&mut vx[42..], ll, d, l, dd, t, c, t, t);
    quad(&mut vx[48..], r, d, rr, dd, c, t, t, t);
    device.draw_primitive_up(d3d::PT_TRIANGLELIST, 18, &vx)
}

pub const DOT_SEGMENTS: usize = 16;
/// fan (3) + fringe ring (6) per segment
pub const DISC_VERTICES: usize = DOT_SEGMENTS * 9;

pub fn unit_circle() -> ([f32; DOT_SEGMENTS + 1], [f32; DOT_SEGMENTS + 1]) {
    let mut cosine = [0f32; DOT_SEGMENTS + 1];
    let mut sine = [0f32; DOT_SEGMENTS + 1];
    for i in 0..=DOT_SEGMENTS {
        let angle = std::f32::consts::TAU * (i % DOT_SEGMENTS) as f32 / DOT_SEGMENTS as f32;
        cosine[i] = angle.cos();
        sine[i] = angle.sin();
    }
    (cosine, sine)
}

/// Solid disc of `radius` plus a transparent-going fringe of `fringe` px: one-pixel anti-aliasing.
#[allow(clippy::too_many_arguments)]
pub fn append_disc(
    out: &mut [Vertex],
    cosine: &[f32],
    sine: &[f32],
    x: f32,
    y: f32,
    radius: f32,
    fringe: f32,
    color: u32,
) {
    let t = color & 0x00FFFFFF;
    let outer = radius + fringe;
    for i in 0..DOT_SEGMENTS {
        let o = &mut out[i * 9..i * 9 + 9];
        let (x0, y0) = (x + radius * cosine[i], y + radius * sine[i]);
        let (x1, y1) = (x + radius * cosine[i + 1], y + radius * sine[i + 1]);
        let (ox0, oy0) = (x + outer * cosine[i], y + outer * sine[i]);
        let (ox1, oy1) = (x + outer * cosine[i + 1], y + outer * sine[i + 1]);
        o[0] = v(x, y, color);
        o[1] = v(x0, y0, color);
        o[2] = v(x1, y1, color);
        o[3] = v(x0, y0, color);
        o[4] = v(ox0, oy0, t);
        o[5] = v(x1, y1, color);
        o[6] = v(x1, y1, color);
        o[7] = v(ox0, oy0, t);
        o[8] = v(ox1, oy1, t);
    }
}

/// Two discs per seat (outline, fill). Static: the ordered callback is serialised by RENDER_BUSY.
pub static SEAT_VERTICES: Global<[Vertex; 100 * 2 * DISC_VERTICES]> = Global::new(
    [Vertex {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        rhw: 0.0,
        color: 0,
    }; 100 * 2 * DISC_VERTICES],
);

/// Applies `call` while every call so far succeeded, as the C++'s TGCX_SET.
pub struct Chain(pub HResult);

impl Chain {
    pub fn set(&mut self, call: impl FnOnce() -> HResult) {
        if succeeded(self.0) {
            self.0 = call();
        }
    }
}

/// The fixed-function state both overlays draw with.
pub fn common_state(c: &mut Chain, device: Device) {
    c.set(|| device.set_vertex_shader(0));
    c.set(|| device.set_pixel_shader(0));
    c.set(|| device.set_fvf(FVF));
    c.set(|| device.set_texture(0, 0));
    c.set(|| device.set_render_state(d3d::RS_ZENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_ZWRITEENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_CULLMODE, d3d::CULL_NONE));
    c.set(|| device.set_render_state(d3d::RS_LIGHTING, 0));
    c.set(|| device.set_render_state(d3d::RS_FOGENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_STENCILENABLE, 0));
}

pub fn blend_state(c: &mut Chain, device: Device) {
    c.set(|| device.set_render_state(d3d::RS_ALPHATESTENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_ALPHABLENDENABLE, 1));
    c.set(|| device.set_render_state(d3d::RS_SRCBLEND, d3d::BLEND_SRCALPHA));
    c.set(|| device.set_render_state(d3d::RS_DESTBLEND, d3d::BLEND_INVSRCALPHA));
    c.set(|| device.set_render_state(d3d::RS_BLENDOP, d3d::BLENDOP_ADD));
    c.set(|| device.set_render_state(d3d::RS_SEPARATEALPHABLENDENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_SRGBWRITEENABLE, 0));
    c.set(|| device.set_render_state(d3d::RS_COLORWRITEENABLE, 15));
    c.set(|| device.set_render_state(d3d::RS_FILLMODE, d3d::FILL_SOLID));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_COLOROP, d3d::TOP_SELECTARG1));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_COLORARG1, d3d::TA_DIFFUSE));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_ALPHAOP, d3d::TOP_SELECTARG1));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_ALPHAARG1, d3d::TA_DIFFUSE));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_TEXTURETRANSFORMFLAGS, d3d::TTFF_DISABLE));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_TEXCOORDINDEX, 0));
    c.set(|| device.set_texture_stage_state(0, d3d::TSS_RESULTARG, d3d::TA_CURRENT));
    c.set(|| device.set_texture_stage_state(1, d3d::TSS_COLOROP, d3d::TOP_DISABLE));
}

/// Primitives only; no game art or font assets enter git.
fn draw_chart(device: Device, sample: &Snapshot, gui: &GuiRect) -> HResult {
    let Some(seats) = allocate_seats(sample) else {
        return S_FALSE;
    };
    // Draw immediately after the native upper-house overlay, within the existing ordered UI scene.
    // Verify the active target, then set and restore our own viewport.
    match d3d::backbuffer_size(device) {
        Err(result) => return result,
        Ok(Some((w, h))) if w == gui.width as u32 && h == gui.height as u32 => {}
        Ok(_) => return S_FALSE,
    }
    // politics_bg.dds at (18,9) ends the Upper House border at root y=100.
    // Start at its first background row, not the text box's maximum height.
    let chart_clip = RECT {
        left: 0.max(gui.left + 37),
        top: 0.max(gui.top + 101),
        right: gui.width.min(gui.left + 162),
        bottom: gui.height.min(gui.top + 209),
    };
    if chart_clip.left >= chart_clip.right || chart_clip.top >= chart_clip.bottom {
        return S_FALSE;
    }
    let viewport = Viewport {
        x: 0,
        y: 0,
        width: gui.width as u32,
        height: gui.height as u32,
        min_z: 0.0,
        max_z: 1.0,
    };
    let mut state = 0usize;
    let result = device.create_state_block(&mut state);
    if !succeeded(result) {
        return result;
    }
    let result = d3d::state_capture(state);
    if !succeeded(result) {
        d3d::release(state);
        return result;
    }
    // Configure only captured state. No render target/depth surface changes or persistent GPU resources.
    let mut c = Chain(result);
    c.set(|| device.set_viewport(&viewport));
    common_state(&mut c, device);
    c.set(|| device.set_scissor_rect(&chart_clip));
    c.set(|| device.set_render_state(d3d::RS_SCISSORTESTENABLE, 1));
    blend_state(&mut c, device);
    let dots = seat_geometry(&seats);
    // Covers the 108x108 chart_upper_house_overlay at (42,100) (TGC v2.5.2 country_politics.gui).
    c.set(|| {
        soft_panel(
            device,
            (gui.left + 40) as f32,
            (gui.top + 101) as f32,
            119.0,
            105.0,
            3.0,
            0xFFCBC9C1,
        )
    });
    let (cosine, sine) = unit_circle();
    let vertices = SEAT_VERTICES.get();
    for (i, dot) in dots.iter().enumerate() {
        let (x, y) = (gui.left as f32 + dot.x, gui.top as f32 + dot.y);
        let out = &mut vertices[i * 2 * DISC_VERTICES..];
        append_disc(out, &cosine, &sine, x, y, SEAT_RADIUS, 0.8, 0xFF34332B);
        append_disc(
            &mut out[DISC_VERTICES..],
            &cosine,
            &sine,
            x,
            y,
            1.75,
            0.6,
            sample.colors[dot.ideology],
        );
    }
    c.set(|| {
        device.draw_primitive_up(
            d3d::PT_TRIANGLELIST,
            (100 * 2 * DISC_VERTICES / 3) as u32,
            &vertices[..],
        )
    });
    let restore = d3d::state_apply(state);
    d3d::release(state);
    if !succeeded(restore) {
        log!(
            "Renderer state restore failed HRESULT={:08X}",
            restore as u32
        );
        return restore;
    }
    if succeeded(c.0) && c.0 != S_FALSE {
        let r = R.get();
        r.chart_tick = tick();
        r.chart_drawn = true;
    }
    c.0
}

// ---------------------------------------------------------------- D3D9 device hooks

type Create9Fn = unsafe extern "system" fn(u32) -> usize;
type CreateDeviceFn =
    unsafe extern "system" fn(usize, u32, u32, usize, u32, usize, *mut usize) -> HResult;
type EndFn = unsafe extern "system" fn(usize) -> HResult;
type PresentFn = unsafe extern "system" fn(usize, usize, usize, usize, usize) -> HResult;
type ResetFn = unsafe extern "system" fn(usize, usize) -> HResult;

#[derive(Clone, Copy)]
struct FactoryHooks {
    table: usize,
    create: usize,
}

#[derive(Clone, Copy)]
struct DeviceHooks {
    table: usize,
    end: usize,
    present: usize,
    reset: usize,
}

struct Hooks {
    factories: [FactoryHooks; 8],
    devices: [DeviceHooks; 8],
}

/// Records are written under HOOK_LOCK and read without it, as the C++ does; a
/// record's originals are published before any callback can reach it.
static HOOKS: Global<Hooks> = Global::new(Hooks {
    factories: [FactoryHooks {
        table: 0,
        create: 0,
    }; 8],
    devices: [DeviceHooks {
        table: 0,
        end: 0,
        present: 0,
        reset: 0,
    }; 8],
});
static HOOK_LOCK: SrwLock = SrwLock::new();
static CREATE9_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static RENDER_BUSY: AtomicI32 = AtomicI32::new(0);

fn device_hooks(device: usize) -> Option<DeviceHooks> {
    let table = rdp(device);
    HOOKS
        .get()
        .devices
        .iter()
        .find(|e| e.table == table)
        .copied()
}

extern "system" fn end_scene(device: usize) -> HResult {
    match device_hooks(device) {
        // SAFETY: the original EndScene of this device's table.
        Some(entry) => unsafe { std::mem::transmute::<usize, EndFn>(entry.end)(device) },
        None => d3d::D3DERR_INVALIDCALL,
    }
}

extern "system" fn present(
    device: usize,
    source: usize,
    destination: usize,
    window: usize,
    dirty: usize,
) -> HResult {
    let Some(entry) = device_hooks(device) else {
        return d3d::D3DERR_INVALIDCALL;
    };
    // SAFETY: the original Present of this device's table.
    let result = unsafe {
        std::mem::transmute::<usize, PresentFn>(entry.present)(
            device,
            source,
            destination,
            window,
            dirty,
        )
    };
    if !succeeded(result) {
        let _cpu = CpuScope::new();
        R.get().chart_drawn = false;
        clear_hover();
    }
    result
}

extern "system" fn reset(device: usize, parameters: usize) -> HResult {
    let Some(entry) = device_hooks(device) else {
        return d3d::D3DERR_INVALIDCALL;
    };
    {
        let _cpu = CpuScope::new();
        R.get().chart_drawn = false;
        clear_hover();
        log!("Device Reset: no persistent chart resources");
    }
    // SAFETY: the original Reset of this device's table.
    let result = unsafe { std::mem::transmute::<usize, ResetFn>(entry.reset)(device, parameters) };
    {
        let _cpu = CpuScope::new();
        R.get().render_failed = false;
        log!("Device Reset result={:08X}", result as u32);
    }
    result
}

// from V2DLL: V2TechButton.cpp, device-acquisition pattern; guards/renderer are our implementation.
fn hook_device(device: usize) -> bool {
    let table = rdp(device);
    let lock = HOOK_LOCK.exclusive();
    if device_hooks(device).is_some() {
        return true;
    }
    let hooks = HOOKS.get();
    let free = hooks.devices.iter().position(|c| c.table == 0);
    let ptr = |i: usize| rdp(table + i * 4);
    let mut ok = free.is_some()
        && executable_pointer(ptr(16))
        && executable_pointer(ptr(17))
        && executable_pointer(ptr(42));
    if let (true, Some(i)) = (ok, free) {
        let entry = DeviceHooks {
            table,
            end: ptr(42),
            present: ptr(17),
            reset: ptr(16),
        };
        hooks.devices[i] = entry;
        // Originals are published before any callback is reachable. Shared COM tables need no object-lifetime storage.
        let end = replace_pointer(table + 42 * 4, entry.end, end_scene as *const () as usize);
        let pres = replace_pointer(table + 17 * 4, entry.present, present as *const () as usize);
        let res = replace_pointer(table + 16 * 4, entry.reset, reset as *const () as usize);
        ok = end && pres && res;
        if !ok {
            if end {
                replace_pointer(table + 42 * 4, end_scene as *const () as usize, entry.end);
            }
            if pres {
                replace_pointer(table + 17 * 4, present as *const () as usize, entry.present);
            }
            if res {
                replace_pointer(table + 16 * 4, reset as *const () as usize, entry.reset);
            }
            // Retain originals even after rollback in case an already-entered callback completes.
        }
    }
    drop(lock);
    log!(
        "D3D9 device callbacks {} table={table:08X}",
        if ok { "installed" } else { "rejected" }
    );
    ok
}

// from V2DLL: V2TechButton.cpp, HookCreateDevice acquisition pattern; creation arguments forwarded unchanged.
extern "system" fn create_device(
    factory: usize,
    adapter: u32,
    kind: u32,
    window: usize,
    flags: u32,
    parameters: usize,
    output: *mut usize,
) -> HResult {
    let table = rdp(factory);
    let Some(original) = HOOKS
        .get()
        .factories
        .iter()
        .rev()
        .find(|e| e.table == table)
        .map(|e| e.create)
    else {
        return d3d::D3DERR_INVALIDCALL;
    };
    // SAFETY: the original CreateDevice of this factory's table.
    let result = unsafe {
        std::mem::transmute::<usize, CreateDeviceFn>(original)(
            factory, adapter, kind, window, flags, parameters, output,
        )
    };
    // SAFETY: CreateDevice's out parameter.
    if succeeded(result) && !output.is_null() && unsafe { *output } != 0 {
        let _cpu = CpuScope::new();
        hook_device(unsafe { *output });
    }
    result
}

// from V2DLL: V2TechButton.cpp, HookDirect3DCreate9 acquisition pattern; bounded shared-table records added.
extern "system" fn create9(version: u32) -> usize {
    // SAFETY: the IAT's original (or a chained hook's) Direct3DCreate9.
    let factory = unsafe {
        std::mem::transmute::<usize, Create9Fn>(CREATE9_ORIGINAL.load(Ordering::Relaxed))(version)
    };
    if factory != 0 {
        let _cpu = CpuScope::new();
        let table = rdp(factory);
        let _lock = HOOK_LOCK.exclusive();
        let hooks = HOOKS.get();
        let known = hooks.factories.iter().any(|e| e.table == table);
        if !known && let Some(entry) = hooks.factories.iter_mut().find(|e| e.table == 0) {
            let create = rdp(table + 16 * 4);
            if executable_pointer(create) {
                *entry = FactoryHooks { table, create };
                if !replace_pointer(table + 16 * 4, create, create_device as *const () as usize) {
                    log!("D3D9 CreateDevice hook rejected");
                }
            }
        }
    }
    factory
}

// ---------------------------------------------------------------- native GUI entities

/// Native icon lookup: parallel vectors of 28-byte strings and object pointers.
/// FUN_009E4D40/FUN_009E6B30; bounded read only, no engine string construction.
pub fn gui_entity(root: usize, renderer: usize, expected: &str, entity_table: u32) -> usize {
    if root == 0 || rdp(root + 0x90) != renderer {
        return 0;
    }
    if slot(root, 17) != base() + 0x5E4D40 {
        return 0;
    }
    let objects = rdp(root + 0x2AC);
    let object_end = rdp(root + 0x2B0);
    let names = rdp(root + 0x2BC);
    let name_end = rdp(root + 0x2C0);
    if objects == 0
        || names == 0
        || object_end < objects
        || name_end < names
        || !(name_end - names).is_multiple_of(28)
        || (name_end - names) / 28 > 256
        || (object_end - objects) / 4 != (name_end - names) / 28
        || !(object_end - objects).is_multiple_of(4)
    {
        return 0;
    }
    for i in 0..(name_end - names) / 28 {
        let name = names + 28 * i;
        if rd32(name + 16) as usize != expected.len() {
            continue;
        }
        let text = if rd32(name + 20) > 15 {
            rdp(name)
        } else {
            name
        };
        // SAFETY: inside the caller's guard.
        if text == 0
            || unsafe { std::slice::from_raw_parts(text as *const u8, expected.len()) }
                != expected.as_bytes()
        {
            continue;
        }
        let icon = rdp(objects + 4 * i);
        if icon == 0 || rdp(icon) != base() + 0xA3A930 {
            return 0;
        }
        let entity = rdp(icon + 0x94);
        return if entity != 0 && rdp(entity) == at(entity_table) {
            entity
        } else {
            0
        };
    }
    0
}

fn politics_entity(renderer: usize, expected: &str, entity_table: u32) -> usize {
    let politics = R.get().politics;
    if politics == 0 {
        return 0;
    }
    gui_entity(rdp(politics + 0x4C), renderer, expected, entity_table)
}

fn is_upper_house_overlay(renderer: usize, entity: usize) -> bool {
    entity != 0 && politics_entity(renderer, "chart_upper_house_overlay", 0xA3FF08) == entity
}

fn native_slices(entity: usize, sample: &Snapshot) -> Option<usize> {
    let r = R.get();
    r.hover_stage = "native ideology vector";
    let manager = rdp(at(0xE58A3C));
    if manager == 0 {
        return None;
    }
    let (begin, end) = (rdp(manager + 0xC), rdp(manager + 0x10));
    if begin == 0 || end < begin || end - begin != sample.array_entries as usize * 4 {
        return None;
    }
    let mut positive = 0;
    for (i, name) in IDEOLOGY_CANDIDATES.iter().enumerate() {
        if rdp(begin + sample.indices[i] as usize * 4) != find_ideology(manager, name) {
            return None;
        }
        if sample.shares[i] > 0 {
            positive += 1;
        }
    }
    r.hover_stage = "native slice vector";
    let (slices, end_slices) = (rdp(entity + 0x158), rdp(entity + 0x15C));
    if slices == 0 || end_slices < slices || end_slices - slices != positive * 36 {
        return None;
    }
    if (0..positive).any(|i| rdi(slices + i * 36 + 32) != i as i32) {
        return None;
    }
    Some(slices)
}

fn publish_hover(renderer: usize, gui: &GuiRect, sample: &Snapshot) {
    let chart = politics_entity(renderer, "chart_upper_house", 0xA3BD58);
    let Some(seats) = allocate_seats(sample) else {
        return;
    };
    if chart == 0 {
        return;
    }
    let frame = HoverFrame {
        view: R.get().politics,
        entity: chart,
        gui: *gui,
        sample: *sample,
        dots: seat_geometry(&seats),
        tick: tick(),
    };
    let _lock = HOVER_LOCK.exclusive();
    *HOVER.get() = frame;
}

/// Native scalar outputs only. -1 delegates to the original pie; 0 uses native no-hover; 1 selects a seat.
fn query_hover(entity: usize, x: f32, y: f32, out: Option<(&mut i32, &mut i32)>) -> i32 {
    let frame = {
        let _lock = HOVER_LOCK.shared();
        *HOVER.get()
    };
    let politics = R.get().politics;
    if entity == 0
        || frame.view == 0
        || frame.view != politics
        || entity != frame.entity
        || !R.get().chart_drawn
        || tick().wrapping_sub(frame.tick) > 1000
    {
        return -1;
    }
    guard(|| {
        let r = R.get();
        r.hover_stage = "view/position";
        if read_politics_rect() != Some(frame.gui) {
            return -1;
        }
        let gui = frame.gui;
        let renderer = rdp(rdp(politics + 0x4C) + 0x90);
        R.get().hover_stage = "entity/snapshot";
        if rd8(renderer + 0x6BCA0) == 0 || politics_entity(renderer, "chart_upper_house", 0xA3BD58) != entity {
            return -1;
        }
        let Some(sample) = snapshot::snapshot() else { return -1 };
        // A loading/country/share change invalidates the published frame until a new successful draw.
        R.get().hover_stage = "changed frame";
        if sample.country != frame.sample.country
            || sample.world != frame.sample.world
            || sample.array != frame.sample.array
            || sample.date_hours != frame.sample.date_hours
            || !sample.shares_in_range
            || sample.indices != frame.sample.indices
            || sample.shares != frame.sample.shares
        {
            return -1;
        }
        let Some(slices) = native_slices(entity, &sample) else { return -1 };
        let Some(hit) = seat_at(&frame.dots, x - gui.left as f32, y - gui.top as f32) else { return 0 };
        let ideology = frame.dots[hit].ideology;
        let ordinal = (0..IDEOLOGIES)
            .filter(|&i| sample.shares[i] > 0 && sample.indices[i] < sample.indices[ideology])
            .count();
        let record = slices + ordinal * 36;
        if let Some((slice, weight)) = out {
            *slice = ordinal as i32;
            *weight = rdi(record);
            let r = R.get();
            if r.hover_logged & (1 << ideology) == 0 {
                r.hover_logged |= 1 << ideology;
                log!(
                    "Hemicycle native hover ideology={} index={} slice={ordinal} screen=({x:.0},{y:.0})",
                    IDEOLOGY_CANDIDATES[ideology],
                    sample.indices[ideology]
                );
            }
        }
        1
    })
    .unwrap_or(-1)
}

extern "C" fn query_bounds(entity: usize, x: f32, y: f32) -> i32 {
    let _cpu = CpuScope::new();
    query_hover(entity, x, y, None)
}

extern "C" fn query_slice(entity: usize, x: i32, y: i32, slice: *mut i32, weight: *mut i32) -> i32 {
    let _cpu = CpuScope::new();
    if slice.is_null() || weight.is_null() {
        return -1;
    }
    // SAFETY: the native caller's two output ints.
    query_hover(
        entity,
        x as f32,
        y as f32,
        Some(unsafe { (&mut *slice, &mut *weight) }),
    )
}

static BOUNDS_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static SLICE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);

// from V2DLL: V2TechButton.cpp, register-preservation pattern. Native scalar ABIs verified in HEMI-3.
#[unsafe(naked)]
unsafe extern "C" fn bounds_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+44]",
        "push dword ptr [ebx+40]",
        "push dword ptr [ebx+24]",
        "call {query}",
        "add esp, 12",
        "cmp eax, -1",
        "je 3f",
        "test eax, eax",
        "jz 2f",
        crate::restore_cpu!(),
        "fld1",
        "ret 32",
        "2:",
        crate::restore_cpu!(),
        "fldz",
        "ret 32",
        "3:",
        crate::restore_cpu!(),
        "jmp dword ptr [{original}]",
        query = sym query_bounds,
        original = sym BOUNDS_ORIGINAL,
    )
}

// The original helper receives the entity in EAX and pops four stack arguments.
#[unsafe(naked)]
unsafe extern "C" fn slice_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push dword ptr [ebx+52]",
        "push dword ptr [ebx+48]",
        "push dword ptr [ebx+44]",
        "push dword ptr [ebx+40]",
        "push dword ptr [ebx+28]",
        "call {query}",
        "add esp, 20",
        "cmp eax, -1",
        "je 2f",
        "mov [ebx+28], eax",
        crate::restore_cpu!(),
        "ret 16",
        "2:",
        crate::restore_cpu!(),
        "jmp dword ptr [{original}]",
        query = sym query_slice,
        original = sym SLICE_ORIGINAL,
    )
}

fn install_hover() -> bool {
    let guards = [0x2D52C6, 0xA3BDA8, 0x5E0B70, 0x5E0C90];
    let mut found = 0;
    for s in crate::signatures::SIGNATURES
        .iter()
        .filter(|s| guards.contains(&s.rva))
    {
        if !check_signature(s) {
            return false;
        }
        found += 1;
    }
    if found != 4 {
        return false;
    }
    let site = at(0x2D52C6);
    let slot_address = at(0xA3BDA8);
    let mut protection: PAGE_PROTECTION_FLAGS = 0;
    let mut ignored = 0;
    if unsafe { VirtualProtect(site as *const _, 5, PAGE_EXECUTE_READWRITE, &mut protection) } == 0
    {
        return false;
    }
    BOUNDS_ORIGINAL.store(at(0x5E0B70), Ordering::Relaxed);
    SLICE_ORIGINAL.store(at(0x5E0C90), Ordering::Relaxed);
    if !replace_pointer(
        slot_address,
        at(0x5E0B70),
        bounds_thunk as *const () as usize,
    ) {
        unsafe { VirtualProtect(site as *const _, 5, protection, &mut ignored) };
        return false;
    }
    wr(site + 1, rel32(site, slice_thunk as *const () as usize));
    let restored = unsafe { VirtualProtect(site as *const _, 5, protection, &mut ignored) } != 0;
    let flushed = unsafe { FlushInstructionCache(GetCurrentProcess(), site as *const _, 5) } != 0;
    log!(
        "Native scalar hover installed CALL_RVA=2D52C6 bounds_slot_RVA=A3BDA8 protection_restored={} cache_flushed={}; tooltip slot 10 untouched",
        u32::from(restored),
        u32::from(flushed)
    );
    true
}

fn native_renderer(renderer: usize) -> bool {
    renderer != 0 && (rdp(renderer) == base() + 0xA38324 || rdp(renderer) == base() + 0xA19B8C)
}

fn after_entity_inner(renderer: usize, entity: usize) {
    if !native_renderer(renderer) || rd8(renderer + 0x6BCA0) == 0 {
        return;
    }
    bill::after_entity(renderer, entity);
    let Some(gui) = read_politics_rect() else {
        return;
    };
    if !is_upper_house_overlay(renderer, entity) {
        return;
    }
    R.get().chart_drawn = false;
    clear_hover();
    let Some(sample) = snapshot::snapshot() else {
        return;
    };
    let device = rdp(renderer + 0x64);
    if device == 0 {
        return;
    }
    let chart = politics_entity(renderer, "chart_upper_house", 0xA3BD58);
    if chart == 0 || native_slices(chart, &sample).is_none() {
        let r = R.get();
        if !r.guard_logged {
            log!(
                "Hemicycle blocked: hover {}; original pie retained",
                r.hover_stage
            );
        }
        r.guard_logged = true;
        return;
    }
    // No BeginScene/EndScene: this callback is already inside the native UI scene.
    let result = draw_chart(Device(device), &sample, &gui);
    let r = R.get();
    if result == S_OK {
        publish_hover(renderer, &gui, &sample);
        r.draw_count += 1;
        if r.draw_count == 1 {
            log!(
                "Hemicycle first ordered draw tag={} seats=100 rows=7 root=({},{}) vector_entries={}",
                sample.tag(),
                gui.left,
                gui.top,
                sample.array_entries
            );
        }
    } else if !succeeded(result)
        && result != d3d::D3DERR_DEVICELOST
        && result != d3d::D3DERR_DEVICENOTRESET
    {
        log!(
            "Chart draw disabled HRESULT={:08X}; original pie retained",
            result as u32
        );
        r.render_failed = true;
    }
}

extern "C" fn after_entity(renderer: usize, entity: usize) {
    let _cpu = CpuScope::new();
    let r = R.get();
    if !r.drawing_enabled
        || r.politics == 0
        || r.render_failed
        || RENDER_BUSY
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
    {
        return;
    }
    if guard(|| after_entity_inner(renderer, entity)).is_none() {
        let r = R.get();
        if !r.guard_logged {
            log!("Hemicycle blocked: ordered GUI read fault");
        }
        r.guard_logged = true;
    }
    RENDER_BUSY.store(0, Ordering::SeqCst);
}

static ENTITY_RESUME: AtomicUsize = AtomicUsize::new(0);

// from V2DLL: V2TechButton.cpp, entry-hook preservation pattern; original CMP is replicated exactly.
#[unsafe(naked)]
unsafe extern "C" fn ordered_entity_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push esi",
        "push dword ptr [ebx+16]",
        "call {callback}",
        "add esp, 8",
        crate::restore_cpu!(),
        "cmp byte ptr [esi+0xF0], 0",
        "jmp dword ptr [{resume}]",
        callback = sym after_entity,
        resume = sym ENTITY_RESUME,
    )
}

/// Final presentation supplies sampling and diagnostics only; never paints over completed UI.
fn before_present_inner(renderer: usize) {
    bill::tick();
    bill::favour_tick();
    templates_gui::tick();
    conform_native::tick();
    let r = R.get();
    r.present_count += 1;
    if r.politics == 0 || tick().wrapping_sub(r.chart_tick) > 1000 {
        r.chart_drawn = false;
    }
    let previous = snapshot::PUMP.get().last_sample;
    if r.drawing_enabled {
        snapshot::on_frame_pump();
    }
    let r = R.get();
    let report =
        r.politics != 0 && (!r.guard_logged || tick().wrapping_sub(r.guard_log_tick) >= 10000);
    if report && !r.chart_drawn {
        if !native_renderer(renderer) {
            log!("Hemicycle blocked: native renderer identity");
        } else if read_politics_rect().is_none() {
            log!("Hemicycle blocked: GUI {}", R.get().gui_stage);
        } else {
            log!(
                "Hemicycle waiting: ordered upper-house overlay draw (or rejected snapshot/backbuffer)"
            );
        }
    }
    let r = R.get();
    if report {
        r.guard_logged = true;
        r.guard_log_tick = tick();
    }
    if previous != snapshot::PUMP.get().last_sample || report {
        log!(
            "render final_frames={} ordered_draws={} politics={:08X} chart={}",
            r.present_count,
            r.draw_count,
            r.politics,
            u32::from(r.chart_drawn)
        );
    }
}

extern "C" fn before_present(renderer: usize) {
    let _cpu = CpuScope::new();
    if guard(|| before_present_inner(renderer)).is_none() {
        let r = R.get();
        if !r.guard_logged {
            log!("Hemicycle blocked: native presentation read fault");
        }
        r.guard_logged = true;
    }
}

static PRESENT_RESUME: AtomicUsize = AtomicUsize::new(0);

// from V2DLL: V2TechButton.cpp, entry-hook preservation pattern; original INC is replicated exactly.
#[unsafe(naked)]
unsafe extern "C" fn native_present_thunk() {
    std::arch::naked_asm!(
        crate::save_cpu!(),
        "push esi",
        "call {callback}",
        "add esp, 4",
        crate::restore_cpu!(),
        "inc dword ptr [esi+0x6BC8C]",
        "jmp dword ptr [{resume}]",
        callback = sym before_present,
        resume = sym PRESENT_RESUME,
    )
}

pub fn install_native_stages() -> bool {
    let (Some(present), Some(ordered)) = (signature(0x59A520), signature(0x599F22)) else {
        return false;
    };
    if !check_signature(present) || !check_signature(ordered) {
        return false;
    }
    let final_site = at(0x59A520);
    let draw_site = at(0x599F22);
    let (mut final_protection, mut draw_protection, mut ignored): (
        PAGE_PROTECTION_FLAGS,
        PAGE_PROTECTION_FLAGS,
        u32,
    ) = (0, 0, 0);
    if unsafe {
        VirtualProtect(
            draw_site as *const _,
            7,
            PAGE_EXECUTE_READWRITE,
            &mut draw_protection,
        )
    } == 0
    {
        return false;
    }
    if unsafe {
        VirtualProtect(
            final_site as *const _,
            6,
            PAGE_EXECUTE_READWRITE,
            &mut final_protection,
        )
    } == 0
    {
        unsafe { VirtualProtect(draw_site as *const _, 7, draw_protection, &mut ignored) };
        return false;
    }
    PRESENT_RESUME.store(at(0x59A526), Ordering::Relaxed);
    ENTITY_RESUME.store(at(0x599F29), Ordering::Relaxed);
    let mut final_patch = [0xE9, 0, 0, 0, 0, 0x90];
    let mut draw_patch = [0xE9, 0, 0, 0, 0, 0x90, 0x90];
    final_patch[1..5].copy_from_slice(
        &rel32(final_site, native_present_thunk as *const () as usize).to_le_bytes(),
    );
    draw_patch[1..5].copy_from_slice(
        &rel32(draw_site, ordered_entity_thunk as *const () as usize).to_le_bytes(),
    );
    // SAFETY: both sites are writable now.
    unsafe {
        std::ptr::copy_nonoverlapping(draw_patch.as_ptr(), draw_site as *mut u8, 7);
        std::ptr::copy_nonoverlapping(final_patch.as_ptr(), final_site as *mut u8, 6);
    }
    let draw_restored =
        unsafe { VirtualProtect(draw_site as *const _, 7, draw_protection, &mut ignored) } != 0;
    let final_restored =
        unsafe { VirtualProtect(final_site as *const _, 6, final_protection, &mut ignored) } != 0;
    let draw_flushed =
        unsafe { FlushInstructionCache(GetCurrentProcess(), draw_site as *const _, 7) } != 0;
    let final_flushed =
        unsafe { FlushInstructionCache(GetCurrentProcess(), final_site as *const _, 6) } != 0;
    log!(
        "Native ordered draw/Present hooks installed RVA=599F22/59A520 protection_restored={} cache_flushed={}",
        u32::from(draw_restored && final_restored),
        u32::from(draw_flushed && final_flushed)
    );
    true
}

/// The IAT slot of d3d9.dll's Direct3DCreate9 in v2game.exe.
fn create9_slot() -> Option<usize> {
    let nt = base() + rd32(base() + 0x3C) as usize;
    // IMAGE_NT_HEADERS32: OptionalHeader +0x18, DataDirectory[IMPORT] +0x68 within it.
    let (address, size) = (rd32(nt + 0x80) as usize, rd32(nt + 0x84) as usize);
    let image = IMAGE_SIZE as usize;
    if address == 0 || size > image || address > image - size {
        return None;
    }
    let mut slot = None;
    for n in 0..size / 20 {
        let item = base() + address + n * 20;
        let (original_first_thunk, name, first_thunk) =
            (rd32(item), rd32(item + 0xC), rd32(item + 0x10));
        if name == 0 {
            break;
        }
        if original_first_thunk == 0
            || !c_str(base() + name as usize).eq_ignore_ascii_case(b"d3d9.dll")
        {
            continue;
        }
        let lookup = base() + original_first_thunk as usize;
        for i in 0..1024 {
            let entry = rd32(lookup + i * 4);
            if entry == 0 {
                break;
            }
            if entry & 0x80000000 != 0 {
                continue;
            }
            if c_str(base() + entry as usize + 2) == b"Direct3DCreate9" {
                slot = Some(base() + first_thunk as usize + i * 4);
            }
        }
    }
    slot
}

pub fn install_renderer() -> bool {
    guard(install_renderer_inner).unwrap_or(false)
}

fn install_renderer_inner() -> bool {
    let Some(slot) = create9_slot() else {
        return false;
    };
    let graphics = unsafe { GetModuleHandleW(windows_sys::w!("d3d9.dll")) };
    let native = if graphics.is_null() {
        0
    } else {
        unsafe { GetProcAddress(graphics, c"Direct3DCreate9".as_ptr().cast()) }
            .map_or(0, |f| f as usize)
    };
    if native == 0 || !executable_pointer(rdp(slot)) {
        return false;
    }
    let expected = rdp(slot);
    // MOD-1: another DLL (V2DLL) may already hook Direct3DCreate9; chain through it.
    if expected != native {
        let mut owner: HMODULE = std::ptr::null_mut();
        let mut name = [0u8; 260];
        name[..7].copy_from_slice(b"unknown");
        let flags =
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
        if unsafe { GetModuleHandleExA(flags, expected as *const u8, &mut owner) } != 0
            && !owner.is_null()
        {
            unsafe { GetModuleFileNameA(owner, name.as_mut_ptr(), 260) };
        }
        let end = name.iter().position(|&c| c == 0).unwrap_or(260);
        log!(
            "Direct3DCreate9 already hooked by {}; chaining",
            Bytes(&name[..end])
        );
    }
    let view = at(0xA0E458);
    let destroy_original = at(0x2D2B00);
    let show_original = at(0x2D2C10);
    let hide_original = at(0x2D2C40);
    SHOW_ORIGINAL.store(show_original, Ordering::Relaxed);
    HIDE_ORIGINAL.store(hide_original, Ordering::Relaxed);
    DESTROY_ORIGINAL.store(destroy_original, Ordering::Relaxed);
    if rdp(view + 4) != destroy_original
        || rdp(view + 8) != show_original
        || rdp(view + 16) != hide_original
    {
        log!("Drawing rejected: CPoliticsView slots 1/2/4 are not native");
        return false;
    }
    // Slot 10 (tooltip) is never written by TGCX (P27); another DLL may own it.
    if rdp(view + 40) != at(0x2D2DD0) {
        log!("CPoliticsView slot 10 is not native (FOREIGN); left alone");
    }
    // All replacements compare-and-swap the validated original; independent failures are logged and reversible.
    let destroy = replace_pointer(
        view + 4,
        destroy_original,
        destroy_thunk as *const () as usize,
    );
    let show = replace_pointer(view + 8, show_original, show_thunk as *const () as usize);
    let hide = replace_pointer(view + 16, hide_original, hide_thunk as *const () as usize);
    CREATE9_ORIGINAL.store(expected, Ordering::Relaxed);
    let create =
        destroy && show && hide && replace_pointer(slot, expected, create9 as *const () as usize);
    let hover = create && install_hover();
    if !hover {
        if create {
            log!(
                "Drawing rejected: native scalar hover guards or installation failed; original pie retained"
            );
            replace_pointer(slot, create9 as *const () as usize, expected);
        }
        if destroy {
            replace_pointer(
                view + 4,
                destroy_thunk as *const () as usize,
                destroy_original,
            );
        }
        if show {
            replace_pointer(view + 8, show_thunk as *const () as usize, show_original);
        }
        if hide {
            replace_pointer(view + 16, hide_thunk as *const () as usize, hide_original);
        }
        return false;
    }
    R.get().drawing_enabled = true;
    log!(
        "Drawing installed: Direct3DCreate9 IAT_RVA={:06X}; Politics lifecycle and scalar hover guarded; native tooltip untouched",
        slot - base()
    );
    true
}
