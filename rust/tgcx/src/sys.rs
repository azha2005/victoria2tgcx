//! The pieces every module shares: tgcx's globals, game memory, guarded calls, the CPU
//! state scope, code and pointer patching, signatures, the log and MSVC strings.
//!
//! Game memory is read and written through plain addresses (`usize`), as the C++ does
//! with uintptr_t. `rd`/`wr` are not marked unsafe: like the C++, every use either runs
//! inside `guard` or touches an address the code validated first.

use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::fmt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::{
    GetLastError, HANDLE, HMODULE, INVALID_HANDLE_VALUE, SetLastError,
};
use windows_sys::Win32::Storage::FileSystem::WriteFile;
use windows_sys::Win32::System::Diagnostics::Debug::{FlushInstructionCache, RtlUnwind};
use windows_sys::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN, GetModuleHandleExW,
};
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_IMAGE, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_PROTECTION_FLAGS,
    PAGE_READWRITE, VirtualProtect, VirtualQuery,
};
use windows_sys::Win32::System::Threading::{
    AcquireSRWLockExclusive, AcquireSRWLockShared, GetCurrentProcess, ReleaseSRWLockExclusive,
    ReleaseSRWLockShared, SRWLOCK, SRWLOCK_INIT,
};

use crate::signatures::{IMAGE_SIZE, PREFERRED_BASE, SIGNATURES, Signature};

// ---------------------------------------------------------------- globals

/// A tgcx global. The game calls every hook from its own GUI/render thread, as the
/// C++'s plain statics assume; `get` hands out a fresh reference for each use.
pub struct Global<T>(UnsafeCell<T>);

// SAFETY: see above; values shared with other threads are atomics instead.
unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }
    #[allow(clippy::mut_from_ref)] // the C++ statics this replaces; see the type's comment
    pub fn get(&self) -> &mut T {
        // SAFETY: single game thread; callers do not hold it across reentrant calls.
        unsafe { &mut *self.0.get() }
    }
}

/// An SRW lock, as the C++'s hover and hook tables use (std's Mutex would import
/// WaitOnAddress).
pub struct SrwLock(UnsafeCell<SRWLOCK>);

// SAFETY: an SRWLOCK is made for sharing between threads.
unsafe impl Sync for SrwLock {}

impl SrwLock {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(SRWLOCK_INIT))
    }
    pub fn exclusive(&self) -> SrwGuard<'_> {
        unsafe { AcquireSRWLockExclusive(self.0.get()) };
        SrwGuard(self, true)
    }
    pub fn shared(&self) -> SrwGuard<'_> {
        unsafe { AcquireSRWLockShared(self.0.get()) };
        SrwGuard(self, false)
    }
}

pub struct SrwGuard<'a>(&'a SrwLock, bool);

impl Drop for SrwGuard<'_> {
    fn drop(&mut self) {
        if self.1 {
            unsafe { ReleaseSRWLockExclusive(self.0.0.get()) };
        } else {
            unsafe { ReleaseSRWLockShared(self.0.0.get()) };
        }
    }
}

/// The base of v2game.exe.
static BASE: AtomicUsize = AtomicUsize::new(0);

pub fn base() -> usize {
    BASE.load(Ordering::Relaxed)
}

pub fn set_base(base: usize) {
    BASE.store(base, Ordering::Relaxed);
}

/// The exe address of an RVA.
pub fn at(rva: u32) -> usize {
    base() + rva as usize
}

// ---------------------------------------------------------------- game memory

pub fn rd<T: Copy>(address: usize) -> T {
    // SAFETY: see the module comment.
    unsafe { std::ptr::read_unaligned(address as *const T) }
}

pub fn rdp(address: usize) -> usize {
    rd::<usize>(address)
}

pub fn rd32(address: usize) -> u32 {
    rd::<u32>(address)
}

pub fn rdi(address: usize) -> i32 {
    rd::<i32>(address)
}

pub fn rd8(address: usize) -> u8 {
    rd::<u8>(address)
}

pub fn rdf(address: usize) -> f32 {
    rd::<f32>(address)
}

pub fn wr<T>(address: usize, value: T) {
    // SAFETY: see the module comment.
    unsafe { std::ptr::write_unaligned(address as *mut T, value) }
}

/// Vtable slot `index` of the object at `object`.
pub fn slot(object: usize, index: usize) -> usize {
    rdp(rdp(object) + index * 4)
}

/// Inside the exe image.
pub fn in_image(address: usize) -> bool {
    address >= base() && address < base() + IMAGE_SIZE as usize
}

// ---------------------------------------------------------------- guarded calls

/// Runs `f` like the C++'s `__try { f } __except(EXCEPTION_EXECUTE_HANDLER)`: None
/// when any exception (an access violation reading the game, mostly) left it. The
/// frames above are unwound by RtlUnwind first, as MSVC's handler does; nothing in
/// `f` is dropped then, as nothing in a C++ `__try` body may have a destructor.
pub fn guard<R, F: FnOnce() -> R>(f: F) -> Option<R> {
    struct Call<F, R> {
        f: Option<F>,
        result: Option<R>,
    }
    extern "C" fn run<F: FnOnce() -> R, R>(call: *mut c_void) {
        // SAFETY: `call` is the Call below, alive for the whole guarded call.
        let call = unsafe { &mut *call.cast::<Call<F, R>>() };
        if let Some(f) = call.f.take() {
            call.result = Some(f());
        }
    }
    let mut call = Call {
        f: Some(f),
        result: None,
    };
    // SAFETY: seh_call runs `run` with `call` and returns 0 if an exception left it.
    let completed = unsafe { seh_call(run::<F, R>, (&raw mut call).cast()) };
    if completed == 0 { None } else { call.result }
}

static RTL_UNWIND: unsafe extern "system" fn(
    *const c_void,
    *const c_void,
    *const windows_sys::Win32::System::Diagnostics::Debug::EXCEPTION_RECORD,
    *const c_void,
) = RtlUnwind;

/// cdecl `u32 seh_call(fn, context)`: calls `fn(context)` under an x86 SEH frame and
/// returns 1, or 0 when seh_handler caught an exception. The frame is the standard
/// EXCEPTION_REGISTRATION {prev, handler} plus the EBP to restore.
#[unsafe(naked)]
unsafe extern "C" fn seh_call(f: extern "C" fn(*mut c_void), context: *mut c_void) -> u32 {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp, esp",
        "push ebx",
        "push esi",
        "push edi",
        "push ebp",
        "push dword ptr [{handler}]",
        "push dword ptr fs:[0]",
        "mov dword ptr fs:[0], esp",
        "push dword ptr [ebp+12]",
        "call dword ptr [ebp+8]",
        "add esp, 4",
        "mov eax, 1",
        "pop ecx",
        "mov dword ptr fs:[0], ecx",
        "add esp, 8",
        "pop edi",
        "pop esi",
        "pop ebx",
        "pop ebp",
        "ret",
        handler = sym SEH_HANDLER,
    )
}

static SEH_HANDLER: unsafe extern "C" fn() = seh_handler;

/// The frame's exception handler (cdecl, as Windows calls it): on the first pass it
/// unwinds every frame above its own, then returns from seh_call with 0. Written in
/// assembly because RtlUnwind does not preserve EBX/ESI/EDI on x86.
#[unsafe(naked)]
unsafe extern "C" fn seh_handler() {
    std::arch::naked_asm!(
        "mov eax, [esp+4]",
        // EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND: nothing to do while unwinding.
        "test dword ptr [eax+4], 6",
        "jnz 2f",
        "mov ecx, [esp+8]",
        "push ecx",
        "push 0",
        "push eax",
        "push 0",
        "push ecx",
        "call dword ptr [{unwind}]",
        "pop ecx",
        // Back in seh_call's frame: ESP at the registration, EBP saved after it.
        "mov esp, ecx",
        "mov ebp, [ecx+8]",
        "xor eax, eax",
        "pop ecx",
        "mov dword ptr fs:[0], ecx",
        "add esp, 8",
        "pop edi",
        "pop esi",
        "pop ebx",
        "pop ebp",
        "ret",
        "2:",
        "mov eax, 1",
        "ret",
        unwind = sym RTL_UNWIND,
    )
}

// SafeSEH: the image's table must list the handler, or Windows refuses to call it.
std::arch::global_asm!(".safeseh {}", sym seh_handler);

// ---------------------------------------------------------------- CPU state

/// The C++ tgcx_CpuScope: saves x87/SSE state and the last error, gives the callback
/// a clean FPU (FNINIT) and MXCSR, and restores everything when dropped.
#[repr(C, align(16))]
pub struct CpuScope {
    state: [u8; 512],
    error: u32,
}

impl CpuScope {
    pub fn new() -> Self {
        let mut scope = CpuScope {
            state: [0; 512],
            error: unsafe { GetLastError() },
        };
        let mxcsr: u32 = 0x1F80;
        // SAFETY: FXSAVE into our 16-aligned 512-byte area; FNINIT and LDMXCSR only
        // change FPU state, which drop restores.
        unsafe {
            std::arch::asm!(
                "fxsave [{area}]",
                "fninit",
                "ldmxcsr [{csr}]",
                area = in(reg) scope.state.as_mut_ptr(),
                csr = in(reg) &mxcsr,
            );
        }
        scope
    }
}

impl Drop for CpuScope {
    fn drop(&mut self) {
        // SAFETY: restores the area FXSAVE wrote (moves keep the 16-byte alignment).
        unsafe {
            std::arch::asm!("fxrstor [{area}]", area = in(reg) self.state.as_ptr());
            SetLastError(self.error);
        }
    }
}

// from V2DLL: V2TechButton.cpp, register-preservation pattern; flags and FXSAVE added.
// EBX keeps the PUSHAD frame: [ebx+0] EDI .. [ebx+28] EAX, [ebx+32] flags, [ebx+36] the
// stack the hook was entered with.
#[macro_export]
macro_rules! save_cpu {
    () => {
        "pushfd\npushad\nmov ebx, esp\nsub esp, 528\nand esp, -16\nfxsave [esp]\ncld"
    };
}

#[macro_export]
macro_rules! restore_cpu {
    () => {
        "fxrstor [esp]\nmov esp, ebx\npopad\npopfd"
    };
}

/// from V2DLL: V2TechButton.cpp, View hook/thunk pattern: `callback(ECX)` with every
/// register and the FPU preserved, then on to the original method.
#[macro_export]
macro_rules! view_thunk {
    ($name:ident, $callback:path, $original:path) => {
        #[unsafe(naked)]
        unsafe extern "C" fn $name() {
            std::arch::naked_asm!(
                $crate::save_cpu!(),
                "push ecx",
                "call {callback}",
                "add esp, 4",
                $crate::restore_cpu!(),
                "jmp dword ptr [{original}]",
                callback = sym $callback,
                original = sym $original,
            )
        }
    };
}

/// Runs `f` with the last error preserved, as the C++'s capture callbacks do.
pub fn keep_last_error<R>(f: impl FnOnce() -> R) -> R {
    let error = unsafe { GetLastError() };
    let result = f();
    unsafe { SetLastError(error) };
    result
}

// ---------------------------------------------------------------- patching

/// Compare-and-swap one pointer slot, restoring its protection.
pub fn replace_pointer(slot: usize, expected: usize, replacement: usize) -> bool {
    if rdp(slot) != expected {
        return false;
    }
    let mut protection: PAGE_PROTECTION_FLAGS = 0;
    if unsafe { VirtualProtect(slot as *const c_void, 4, PAGE_READWRITE, &mut protection) } == 0 {
        return false;
    }
    // SAFETY: `slot` is writable now and 4-aligned in every table we patch.
    let previous = unsafe { &*(slot as *const AtomicUsize) }
        .compare_exchange(expected, replacement, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap_or_else(|value| value);
    let mut ignored = 0;
    let restored =
        unsafe { VirtualProtect(slot as *const c_void, 4, protection, &mut ignored) } != 0;
    if !restored {
        crate::log!("Pointer protection restore failed slot={slot:08X}");
    }
    previous == expected
}

/// Committed, executable image code; its module is pinned so it stays loaded.
pub fn executable_pointer(pointer: usize) -> bool {
    // SAFETY: MEMORY_BASIC_INFORMATION is plain data.
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let size = size_of::<MEMORY_BASIC_INFORMATION>();
    if pointer == 0
        || unsafe { VirtualQuery(pointer as *const c_void, &mut info, size) } != size
        || info.State != MEM_COMMIT
    {
        return false;
    }
    let protection = info.Protect & 0xFF;
    if info.Protect & PAGE_GUARD != 0
        || info.Type != MEM_IMAGE
        || !matches!(
            protection,
            PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY
        )
    {
        return false;
    }
    // Shared COM-table records outlive device instances; keep each original code owner resident.
    let mut owner: HMODULE = null_mut();
    let flags = GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN;
    unsafe { GetModuleHandleExW(flags, pointer as *const u16, &mut owner) != 0 }
}

/// Writes `bytes` over code at `address`, restoring protection and flushing.
/// Returns (protection restored, cache flushed), or None when it could not write.
pub fn write_code(address: usize, bytes: &[u8]) -> Option<(bool, bool)> {
    let mut protection: PAGE_PROTECTION_FLAGS = 0;
    let mut ignored = 0;
    let site = address as *const c_void;
    if unsafe { VirtualProtect(site, bytes.len(), PAGE_EXECUTE_READWRITE, &mut protection) } == 0 {
        return None;
    }
    // SAFETY: the range is writable now.
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), address as *mut u8, bytes.len()) };
    let restored = unsafe { VirtualProtect(site, bytes.len(), protection, &mut ignored) } != 0;
    let flushed = unsafe { FlushInstructionCache(GetCurrentProcess(), site, bytes.len()) } != 0;
    Some((restored, flushed))
}

/// A JMP/CALL rel32 at `rva` to `target`, NOP-padded to `size` (5..8) bytes.
pub fn code_patch(rva: u32, target: usize, size: usize, opcode: u8) -> bool {
    if !(5..=8).contains(&size) {
        return false;
    }
    let mut patch = [opcode, 0, 0, 0, 0, 0x90, 0x90, 0x90];
    patch[1..5].copy_from_slice(&rel32(at(rva), target).to_le_bytes());
    write_code(at(rva), &patch[..size]).is_some()
}

/// The rel32 of a 5-byte JMP/CALL at `site` reaching `target`.
pub fn rel32(site: usize, target: usize) -> u32 {
    target.wrapping_sub(site + 5) as u32
}

// ---------------------------------------------------------------- signatures

pub fn signature(rva: u32) -> Option<&'static Signature> {
    SIGNATURES.iter().find(|s| s.rva == rva)
}

/// The signature's bytes, relocated to the running base, are in memory.
pub fn check_signature(signature: &Signature) -> bool {
    guard(|| {
        let length = signature.length as usize;
        let mut expected = [0u8; 32];
        expected[..length].copy_from_slice(&signature.bytes[..length]);
        for &offset in &signature.reloc[..signature.reloc_count as usize] {
            let offset = offset as usize;
            let value = u32::from_le_bytes(expected[offset..offset + 4].try_into().unwrap())
                .wrapping_add((base() as u32).wrapping_sub(PREFERRED_BASE));
            expected[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        // SAFETY: inside guard; a fault reads as a mismatch.
        let live = unsafe { std::slice::from_raw_parts(at(signature.rva) as *const u8, length) };
        live == &expected[..length]
    })
    .unwrap_or(false)
}

/// Every listed RVA has a signature and it matches.
pub fn require_signatures(rvas: &[u32]) -> bool {
    rvas.iter().all(|&rva| {
        let mut found = false;
        for s in SIGNATURES.iter().filter(|s| s.rva == rva) {
            found = true;
            if !check_signature(s) {
                return false;
            }
        }
        found
    })
}

// ---------------------------------------------------------------- the log

static LOG: AtomicIsize = AtomicIsize::new(-1);

pub fn set_log(handle: HANDLE) {
    LOG.store(handle as isize, Ordering::Relaxed);
}

pub fn take_log() -> HANDLE {
    LOG.swap(INVALID_HANDLE_VALUE as isize, Ordering::Relaxed) as HANDLE
}

/// One log line, like tgcx_Log: at most 1021 bytes (the C++'s buffer), then CRLF.
pub fn log_args(args: fmt::Arguments) {
    let handle = LOG.load(Ordering::Relaxed) as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        return;
    }
    let mut line = Text::<1022>::new();
    let _ = fmt::write(&mut line, args);
    let mut bytes = line.bytes().to_vec();
    bytes.extend_from_slice(b"\r\n");
    let mut written = 0;
    unsafe {
        WriteFile(
            handle,
            bytes.as_ptr(),
            bytes.len() as u32,
            &mut written,
            null_mut(),
        )
    };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { $crate::sys::log_args(format_args!($($arg)*)) };
}

// ---------------------------------------------------------------- text

/// A NUL-terminated byte string of at most N-1 bytes, written with `write!`. Game
/// text is in the ANSI code page: each char below 256 becomes that byte (so a Rust
/// "\u{A7}" is the game's colour escape 0xA7), anything else '?'. Longer text is cut,
/// like the C++'s _TRUNCATE.
#[derive(Clone, Copy)]
pub struct Text<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Text<N> {
    pub const fn new() -> Self {
        Text {
            buf: [0; N],
            len: 0,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut text = Self::new();
        text.push_bytes(bytes);
        text
    }
    pub fn from_latin1(text: &str) -> Self {
        let mut out = Self::new();
        let _ = fmt::Write::write_str(&mut out, text);
        out
    }
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        let n = bytes.len().min(N - 1 - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
        self.len += n;
        self.buf[self.len] = 0;
    }
    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    /// The bytes and their NUL.
    pub fn with_nul(&self) -> &[u8] {
        &self.buf[..=self.len]
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.buf[0] = 0;
    }
}

impl<const N: usize> Default for Text<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for Text<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if self.len >= N - 1 {
                break;
            }
            self.buf[self.len] = u8::try_from(u32::from(c)).unwrap_or(b'?');
            self.len += 1;
        }
        self.buf[self.len] = 0;
        Ok(())
    }
}

impl<const N: usize> fmt::Display for Text<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Bytes(self.bytes()).fmt(f)
    }
}

/// `format!` into a Text.
#[macro_export]
macro_rules! text {
    ($n:expr, $($arg:tt)*) => {{
        let mut t = $crate::sys::Text::<$n>::new();
        let _ = ::std::fmt::Write::write_fmt(&mut t, format_args!($($arg)*));
        t
    }};
}

/// Game bytes shown as text (each byte its own char), as printf's %s writes them.
pub struct Bytes<'a>(pub &'a [u8]);

impl fmt::Display for Bytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().map(|&b| char::from(b)).collect();
        f.pad(&s)
    }
}

/// printf's "%.Ns" of a packed tag: up to N bytes, ending at a NUL.
pub struct Tag(pub u32, pub usize);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.0.to_le_bytes();
        let end = bytes
            .iter()
            .take(self.1)
            .position(|&b| b == 0)
            .unwrap_or(self.1.min(4));
        Bytes(&bytes[..end]).fmt(f)
    }
}

/// printf's "%ls" of a wide path in the C locale: chars above 255 become '?'.
pub struct Wide<'a>(pub &'a [u16]);

impl fmt::Display for Wide<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let end = self.0.iter().position(|&c| c == 0).unwrap_or(self.0.len());
        let s: String = self.0[..end]
            .iter()
            .map(|&c| if c < 256 { char::from(c as u8) } else { '?' })
            .collect();
        f.write_str(&s)
    }
}

// ---------------------------------------------------------------- MSVC std::string

/// from V2DLL: V2TechButton.cpp, GStr memory layout (borrowed inputs only).
#[repr(C)]
pub struct GameString {
    data: [u8; 16],
    length: u32,
    capacity: u32,
}

impl GameString {
    /// from V2DLL: V2TechButton.cpp, GStr/MakeStr borrowed-string ABI. `text` ends in
    /// its NUL and must outlive the string.
    pub fn borrow(text: &[u8]) -> GameString {
        let length = text.len() - 1;
        debug_assert_eq!(text[length], 0);
        let mut s = GameString {
            data: [0; 16],
            length: length as u32,
            capacity: 15,
        };
        if length < 16 {
            s.data[..=length].copy_from_slice(text);
        } else {
            s.data[..4].copy_from_slice(&(text.as_ptr() as usize as u32).to_le_bytes());
            s.capacity = length as u32;
        }
        s
    }
}

/// An MSVC std::string in game memory: buffer/pointer +0, size +0x10, capacity +0x14.
/// Read-only; None unless it fits in N-1 bytes.
pub fn read_game_string<const N: usize>(at: usize) -> Option<Text<N>> {
    let length = rd32(at + 0x10) as usize;
    let capacity = rd32(at + 0x14) as usize;
    if length >= N || capacity < length || capacity > 0x10000 {
        return None;
    }
    let text = if capacity > 15 { rdp(at) } else { at };
    if text == 0 {
        return None;
    }
    // SAFETY: callers guard the read.
    let bytes = unsafe { std::slice::from_raw_parts(text as *const u8, length) };
    // The C++ copies all of it, then compares and prints it as a C string.
    Some(Text::from_bytes(
        bytes.split(|&c| c == 0).next().unwrap_or_default(),
    ))
}

// ---------------------------------------------------------------- native calls

/// A thiscall method through fastcall, as the C++ does: ECX this, EDX unused.
pub type GuiOne = unsafe extern "fastcall" fn(usize, usize, *const c_void) -> usize;
pub type GuiZero = unsafe extern "fastcall" fn(usize, usize);

pub fn call_one(function: usize, this: usize, argument: *const c_void) -> usize {
    // SAFETY: callers check `function` against the native method they expect.
    unsafe { std::mem::transmute::<usize, GuiOne>(function)(this, 0, argument) }
}

pub fn call_zero(function: usize, this: usize) {
    // SAFETY: as call_one.
    unsafe { std::mem::transmute::<usize, GuiZero>(function)(this, 0) }
}

pub fn call_u32(function: usize, this: usize, argument: u32) {
    type F = unsafe extern "fastcall" fn(usize, usize, u32);
    // SAFETY: as call_one.
    unsafe { std::mem::transmute::<usize, F>(function)(this, 0, argument) }
}

/// The NUL-terminated string at `address` (callers guard the read).
pub fn c_str(address: usize) -> &'static [u8] {
    // SAFETY: see the module comment.
    unsafe { std::ffi::CStr::from_ptr(address as *const std::ffi::c_char) }.to_bytes()
}

pub fn tick() -> u32 {
    unsafe { windows_sys::Win32::System::SystemInformation::GetTickCount() }
}

pub fn thread_id() -> u32 {
    unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() }
}
