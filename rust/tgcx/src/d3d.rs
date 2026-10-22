//! The few Direct3D 9 calls the renderer and the Bill make, through raw vtables. Slot
//! numbers are the method order of d3d9.h (IDirect3DDevice9 and friends).

use std::ffi::c_void;

use windows_sys::Win32::Foundation::RECT;

use crate::sys::slot;

pub type HResult = i32;
pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const D3DERR_INVALIDCALL: HResult = 0x8876086Cu32 as i32;
pub const D3DERR_DEVICELOST: HResult = 0x88760868u32 as i32;
pub const D3DERR_DEVICENOTRESET: HResult = 0x88760869u32 as i32;

pub fn succeeded(result: HResult) -> bool {
    result >= 0
}

pub const FVF_XYZRHW: u32 = 0x004;
pub const FVF_DIFFUSE: u32 = 0x040;
pub const FVF_TEX1: u32 = 0x100;
pub const PT_TRIANGLELIST: u32 = 4;
pub const PT_TRIANGLESTRIP: u32 = 5;
pub const BACKBUFFER_TYPE_MONO: u32 = 0;
pub const SBT_ALL: u32 = 1;
pub const FMT_A8R8G8B8: u32 = 21;
pub const POOL_MANAGED: u32 = 1;

pub const RS_ZENABLE: u32 = 7;
pub const RS_FILLMODE: u32 = 8;
pub const RS_ZWRITEENABLE: u32 = 14;
pub const RS_ALPHATESTENABLE: u32 = 15;
pub const RS_SRCBLEND: u32 = 19;
pub const RS_DESTBLEND: u32 = 20;
pub const RS_CULLMODE: u32 = 22;
pub const RS_ALPHABLENDENABLE: u32 = 27;
pub const RS_FOGENABLE: u32 = 28;
pub const RS_STENCILENABLE: u32 = 52;
pub const RS_LIGHTING: u32 = 137;
pub const RS_COLORWRITEENABLE: u32 = 168;
pub const RS_BLENDOP: u32 = 171;
pub const RS_SCISSORTESTENABLE: u32 = 174;
pub const RS_SRGBWRITEENABLE: u32 = 194;
pub const RS_SEPARATEALPHABLENDENABLE: u32 = 206;
pub const CULL_NONE: u32 = 1;
pub const BLEND_SRCALPHA: u32 = 5;
pub const BLEND_INVSRCALPHA: u32 = 6;
pub const BLENDOP_ADD: u32 = 1;
pub const FILL_SOLID: u32 = 3;

pub const TSS_COLOROP: u32 = 1;
pub const TSS_COLORARG1: u32 = 2;
pub const TSS_COLORARG2: u32 = 3;
pub const TSS_ALPHAOP: u32 = 4;
pub const TSS_ALPHAARG1: u32 = 5;
pub const TSS_ALPHAARG2: u32 = 6;
pub const TSS_TEXCOORDINDEX: u32 = 11;
pub const TSS_TEXTURETRANSFORMFLAGS: u32 = 24;
pub const TSS_RESULTARG: u32 = 28;
pub const TOP_DISABLE: u32 = 1;
pub const TOP_SELECTARG1: u32 = 2;
pub const TOP_MODULATE: u32 = 4;
pub const TA_DIFFUSE: u32 = 0;
pub const TA_CURRENT: u32 = 1;
pub const TA_TEXTURE: u32 = 2;
pub const TTFF_DISABLE: u32 = 0;

pub const SAMP_ADDRESSU: u32 = 1;
pub const SAMP_ADDRESSV: u32 = 2;
pub const SAMP_MAGFILTER: u32 = 5;
pub const SAMP_MINFILTER: u32 = 6;
pub const SAMP_MIPFILTER: u32 = 7;
pub const TEXF_NONE: u32 = 0;
pub const TEXF_POINT: u32 = 1;
pub const TADDRESS_CLAMP: u32 = 3;

#[repr(C)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub min_z: f32,
    pub max_z: f32,
}

#[repr(C)]
#[derive(Default)]
pub struct SurfaceDesc {
    pub format: u32,
    pub kind: u32,
    pub usage: u32,
    pub pool: u32,
    pub multi_sample_type: u32,
    pub multi_sample_quality: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct LockedRect {
    pub pitch: i32,
    pub bits: *mut u8,
}

/// One COM call: slot `index` of `object`, this first, stdcall.
macro_rules! com {
    ($object:expr, $index:expr, fn($($ty:ty),*) $(, $arg:expr)*) => {{
        type F = unsafe extern "system" fn(usize $(, $ty)*) -> HResult;
        let object: usize = $object;
        // SAFETY: a live COM object of the interface whose slot this is.
        unsafe { std::mem::transmute::<usize, F>(slot(object, $index))(object $(, $arg)*) }
    }};
}

pub fn release(object: usize) {
    com!(object, 2, fn());
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Device(pub usize);

impl Device {
    pub fn create_texture(
        &self,
        width: u32,
        height: u32,
        format: u32,
        pool: u32,
        out: &mut usize,
    ) -> HResult {
        com!(
            self.0,
            23,
            fn(u32, u32, u32, u32, u32, u32, *mut usize, *mut c_void),
            width,
            height,
            1,
            0,
            format,
            pool,
            out,
            std::ptr::null_mut()
        )
    }
    pub fn get_back_buffer(&self, out: &mut usize) -> HResult {
        com!(
            self.0,
            18,
            fn(u32, u32, u32, *mut usize),
            0,
            0,
            BACKBUFFER_TYPE_MONO,
            out
        )
    }
    pub fn get_render_target(&self, out: &mut usize) -> HResult {
        com!(self.0, 38, fn(u32, *mut usize), 0, out)
    }
    pub fn set_viewport(&self, viewport: &Viewport) -> HResult {
        com!(self.0, 47, fn(*const Viewport), viewport)
    }
    pub fn set_render_state(&self, state: u32, value: u32) -> HResult {
        com!(self.0, 57, fn(u32, u32), state, value)
    }
    pub fn create_state_block(&self, out: &mut usize) -> HResult {
        com!(self.0, 59, fn(u32, *mut usize), SBT_ALL, out)
    }
    pub fn set_texture(&self, stage: u32, texture: usize) -> HResult {
        com!(self.0, 65, fn(u32, usize), stage, texture)
    }
    pub fn set_texture_stage_state(&self, stage: u32, kind: u32, value: u32) -> HResult {
        com!(self.0, 67, fn(u32, u32, u32), stage, kind, value)
    }
    pub fn set_sampler_state(&self, sampler: u32, kind: u32, value: u32) -> HResult {
        com!(self.0, 69, fn(u32, u32, u32), sampler, kind, value)
    }
    pub fn set_scissor_rect(&self, rect: &RECT) -> HResult {
        com!(self.0, 75, fn(*const RECT), rect)
    }
    pub fn draw_primitive_up<T>(&self, kind: u32, count: u32, data: &[T]) -> HResult {
        com!(
            self.0,
            83,
            fn(u32, u32, *const c_void, u32),
            kind,
            count,
            data.as_ptr().cast(),
            size_of::<T>() as u32
        )
    }
    pub fn set_fvf(&self, fvf: u32) -> HResult {
        com!(self.0, 89, fn(u32), fvf)
    }
    pub fn set_vertex_shader(&self, shader: usize) -> HResult {
        com!(self.0, 92, fn(usize), shader)
    }
    pub fn set_pixel_shader(&self, shader: usize) -> HResult {
        com!(self.0, 107, fn(usize), shader)
    }
}

pub fn surface_desc(surface: usize, desc: &mut SurfaceDesc) -> HResult {
    com!(surface, 12, fn(*mut SurfaceDesc), desc)
}

pub fn texture_lock(texture: usize, rect: &mut LockedRect) -> HResult {
    com!(
        texture,
        19,
        fn(u32, *mut LockedRect, *const RECT, u32),
        0,
        rect,
        std::ptr::null(),
        0
    )
}

pub fn texture_unlock(texture: usize) -> HResult {
    com!(texture, 20, fn(u32), 0)
}

pub fn state_capture(block: usize) -> HResult {
    com!(block, 4, fn())
}

pub fn state_apply(block: usize) -> HResult {
    com!(block, 5, fn())
}

/// The active render target is the back buffer and its size; Err for a failed call,
/// Ok(None) when it is not.
pub fn backbuffer_size(device: Device) -> Result<Option<(u32, u32)>, HResult> {
    let (mut target, mut back) = (0usize, 0usize);
    let mut result = device.get_render_target(&mut target);
    if succeeded(result) {
        result = device.get_back_buffer(&mut back);
    }
    let mut desc = SurfaceDesc::default();
    let correct = target != 0 && back != 0 && target == back;
    if correct {
        result = surface_desc(back, &mut desc);
    }
    if target != 0 {
        release(target);
    }
    if back != 0 {
        release(back);
    }
    if !succeeded(result) {
        return Err(result);
    }
    Ok(correct.then_some((desc.width, desc.height)))
}
