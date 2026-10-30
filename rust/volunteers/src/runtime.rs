//! Observer and optional native GUI preview. Game mutation remains separately
//! gated. Tick uses CHourlyTickCommand, not the load-invoked DayTick entry.
use crate::{
    model::Settings,
    sha256::Sha256,
    snapshot::{self, Memory, Snapshot},
};
use std::ffi::{c_char, c_void};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

type Handle = *mut c_void;
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> Handle;
    fn GetModuleFileNameW(module: Handle, buffer: *mut u16, size: u32) -> u32;
    fn GetModuleHandleExW(flags: u32, address: *const u16, module: *mut Handle) -> i32;
    fn ReadProcessMemory(
        process: Handle,
        address: *const c_void,
        buffer: *mut c_void,
        size: usize,
        count: *mut usize,
    ) -> i32;
    fn GetCurrentProcess() -> Handle;
    fn GetCurrentThreadId() -> u32;
    fn GetLastError() -> u32;
    fn SetLastError(error: u32);
    fn IsProcessorFeaturePresent(feature: u32) -> i32;
}

// from V2DLL: V2ModAPI.h, plugin API v1 layout only. No upstream implementation copied.
#[repr(C)]
pub struct V2ModApi {
    size: u32,
    version: u32,
    game_base: u32,
    mod_folder: *const c_char,
    log: Option<unsafe extern "C" fn(*const c_char, ...)>,
    patch_slot: usize,
    steal_to_trampoline: usize,
}

const EXE_SHA: &str = "62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648";
const TICK_SLOT: u32 = 0xA1242C;
const TICK_EXECUTE: u32 = 0x32ECF0;
const FOLDER: &str = "tgcx_volunteers";
static INITIALIZED: AtomicU32 = AtomicU32::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static DEPTH: AtomicU32 = AtomicU32::new(0);
static MAX_DEPTH: AtomicU32 = AtomicU32::new(0);
static SKIPPED: AtomicU32 = AtomicU32::new(0);
static CONTEXT: OnceLock<Context> = OnceLock::new();
struct Context {
    base: u32,
    state: Mutex<Observation>,
}
struct Observation {
    log: File,
    settings: Settings,
    calls: u64,
    last: Option<Snapshot>,
    last_stamp: Option<(u32, u32)>,
    first_thread: Option<u32>,
}
pub(crate) fn diagnostic(message: &str) {
    if let Some(context) = CONTEXT.get()
        && let Ok(mut state) = context.state.try_lock()
    {
        let _ = writeln!(state.log, "{message}");
        let _ = state.log.flush();
    }
}
struct LiveMemory;
impl Memory for LiveMemory {
    fn read32(&self, address: u32) -> Option<u32> {
        let mut value = 0u32;
        read_bytes(address, unsafe {
            std::slice::from_raw_parts_mut((&raw mut value).cast::<u8>(), 4)
        })
        .then_some(value)
    }
}
fn read_bytes(address: u32, buffer: &mut [u8]) -> bool {
    if address < 0x10000 || address.checked_add(buffer.len() as u32).is_none() {
        return false;
    }
    let mut copied = 0;
    unsafe {
        ReadProcessMemory(
            GetCurrentProcess(),
            address as *const c_void,
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            &mut copied,
        ) != 0
            && copied == buffer.len()
    }
}
fn module_path() -> Option<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    let mut path = [0u16; 32768];
    let n =
        unsafe { GetModuleFileNameW(null_mut(), path.as_mut_ptr(), path.len() as u32) } as usize;
    (n > 0 && n < path.len()).then(|| PathBuf::from(std::ffi::OsString::from_wide(&path[..n])))
}
fn exe_hash(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Some(hash.finish())
}

fn tick_signature_matches(base: u32, code: &[u8]) -> bool {
    let Some(global) = base.checked_add(0xE588E8) else {
        return false;
    };
    code.len() == 16
        && code[0] == 0xA1
        && code[1..5] == global.to_le_bytes()
        && code[5..]
            == [
                0x83, 0xC1, 0x3C, 0x51, 0x50, 0xE8, 0xC1, 0x41, 0xF5, 0xFF, 0xC3,
            ]
}

/// Exact package activation: descriptor filename and its marker must match.
/// Alternate plugin placement may load this DLL even while its mod is unticked.
fn selected_package(game: &Path) -> Option<PathBuf> {
    for arg in std::env::args_os() {
        let text = arg.to_string_lossy();
        let Some(mod_name) = text.strip_prefix("-mod=") else {
            continue;
        };
        let path = game.join(mod_name.trim_matches('"'));
        if !path
            .file_name()?
            .to_string_lossy()
            .eq_ignore_ascii_case("tgcx_volunteers.mod")
        {
            continue;
        }
        let descriptor = std::fs::read_to_string(&path).ok()?;
        let path_line = descriptor
            .lines()
            .filter_map(|line| line.split_once('='))
            .find(|(key, _)| key.trim() == "path")?
            .1
            .trim()
            .trim_matches('"');
        let package = game.join(path_line);
        if !package
            .file_name()?
            .to_string_lossy()
            .eq_ignore_ascii_case(FOLDER)
        {
            return None;
        }
        let marker = std::fs::read_to_string(package.join("tgcx_package.txt")).ok()?;
        let (mut named, mut versioned) = (false, false);
        for line in marker.lines().filter(|line| !line.trim().is_empty()) {
            let (key, value) = line.split_once('=')?;
            match (key.trim(), value.trim()) {
                ("package", "volunteers") if !named => named = true,
                ("version", value) if !versioned && value == env!("CARGO_PKG_VERSION") => {
                    versioned = true
                }
                _ => return None,
            }
        }
        if named && versioned {
            return Some(package);
        }
    }
    None
}

/// Calls around the original preserve flags, general registers, x87/SSE and
/// Windows last-error. The stack and ECX passed to the original are unchanged.
/// from V2DLL: V2TechButton.cpp, pushad/popad preservation pattern; FXSAVE added.
#[unsafe(naked)]
unsafe extern "C" fn tick_observer() {
    std::arch::naked_asm!(
        "pushfd", "pushad", "mov ebx, esp", "sub esp, 528", "and esp, -16",
        "fxsave [esp]", "fninit", "mov dword ptr [esp+512], 0x1F80", "ldmxcsr [esp+512]", "cld", "call {before}",
        "fxrstor [esp]", "mov esp, ebx", "popad", "popfd",
        "call dword ptr [{original}]",
        "pushfd", "pushad", "mov ebx, esp", "sub esp, 528", "and esp, -16",
        "fxsave [esp]", "fninit", "mov dword ptr [esp+512], 0x1F80", "ldmxcsr [esp+512]", "cld", "call {after}",
        "fxrstor [esp]", "mov esp, ebx", "popad", "popfd", "ret",
        before=sym before_tick, after=sym after_tick, original=sym ORIGINAL,
    );
}
extern "C" fn before_tick() {
    let _error = crate::native::LastError::preserve();
    let previous = DEPTH.fetch_add(1, Ordering::Relaxed);
    MAX_DEPTH.fetch_max(previous.saturating_add(1), Ordering::Relaxed);
    if previous == 0
        && let Some(context) = CONTEXT.get()
    {
        crate::native_mp_play::before_tick(crate::native::Native { base: context.base });
    }
}
extern "C" fn after_tick() {
    let error = unsafe { GetLastError() };
    let depth = DEPTH.fetch_sub(1, Ordering::Relaxed);
    if depth == 1 {
        let report = observe();
        if let Some(context) = CONTEXT.get() {
            crate::native_mp::tick(crate::native::Native { base: context.base });
            crate::native_mp_play::after_tick(crate::native::Native { base: context.base });
        }
        crate::native_gui::day_tick();
        if report {
            crate::native_hooks::diagnostics();
        }
    } else {
        SKIPPED.fetch_add(1, Ordering::Relaxed);
    }
    unsafe { SetLastError(error) };
}
fn observe() -> bool {
    let _timing = crate::timing::OBSERVER.begin();
    let Some(context) = CONTEXT.get() else {
        return false;
    };
    let Ok(mut s) = context.state.try_lock() else {
        SKIPPED.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    s.calls = s.calls.saturating_add(1);
    let thread = unsafe { GetCurrentThreadId() };
    let changed_thread = s.first_thread.is_some_and(|t| t != thread);
    s.first_thread.get_or_insert(thread);
    let memory = crate::native::Native { base: context.base };
    let stamp = memory
        .u32(context.base + 0xE588E8)
        .and_then(|world| Some((world, memory.field(world, 0xB0C)?)));
    let transition =
        s.last_stamp
            .zip(stamp)
            .is_some_and(|((old_world, old_date), (world, date))| {
                old_world != world || i64::from(date) - i64::from(old_date) != 24
            });
    s.last_stamp = stamp;
    let should_log = s.calls == 1
        || s.calls % u64::from(s.settings.sample_every_days) == 0
        || transition
        || changed_thread;
    if !should_log {
        return false;
    }
    // Diagnostic snapshots never authorize gameplay. Only sampled days build
    // the country/selection inventory, using the existing bounded SEH adapter.
    let Some(sample) = snapshot::capture(&memory, context.base) else {
        let calls = s.calls;
        let _ = writeln!(
            s.log,
            "sample_unavailable command={calls} thread={thread} (world/UI transition or bounded-read refusal)"
        );
        let _ = s.log.flush();
        return true;
    };
    let previous = s.last.as_ref();
    let date_delta = previous
        .filter(|p| p.world == sample.world)
        .map(|p| i64::from(sample.date) - i64::from(p.date));
    if should_log {
        let session = crate::native_session::capture(&memory, context.base);
        let calls = s.calls;
        let max = MAX_DEPTH.load(Ordering::Relaxed);
        let skipped = SKIPPED.load(Ordering::Relaxed);
        let _ = writeln!(
            s.log,
            "native_session={session:?} (read-only classification; campaign binding and gameplay gates independent)"
        );
        let _ = writeln!(
            s.log,
            "tick command={calls} thread={thread} thread_changed={changed_thread} depth_max={max} skipped={skipped} world={:08X} date={} delta={date_delta:?} transition={transition} player={} countries_at_war={} selected_armies={}",
            sample.world,
            sample.date,
            sample.player,
            sample.countries_at_war,
            sample.selected.len()
        );
        for army in &sample.selected {
            let _ = writeln!(
                s.log,
                "army id={}/{} owner={} affiliation_plus_CC={} province={} regiments={} combat={} first_pop_owner={:?}",
                army.identity.kind,
                army.identity.id,
                army.owner,
                army.affiliation,
                army.province,
                army.regiments,
                army.in_combat,
                army.first_pop_owner
            );
        }
        let _ = s.log.flush();
    }
    s.last = Some(sample);
    true
}

fn initialize(api: &V2ModApi) -> bool {
    let previous = INITIALIZED.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst);
    if let Err(state) = previous {
        return state == 2 || state == 4;
    }
    let result = initialize_once(api);
    if INITIALIZED.load(Ordering::SeqCst) != 4 {
        INITIALIZED.store(if result { 2 } else { 3 }, Ordering::SeqCst);
    }
    result
}
fn initialize_once(api: &V2ModApi) -> bool {
    let Some(exe) = module_path() else {
        return false;
    };
    let Some(game) = exe.parent() else {
        return false;
    };
    let Some(package) = selected_package(game) else {
        INITIALIZED.store(4, Ordering::SeqCst);
        return true;
    };
    let logs = game.join("Logs");
    if std::fs::create_dir_all(&logs).is_err() {
        return false;
    }
    let Ok(mut log) = File::create(logs.join("tgcx_volunteers.log")) else {
        return false;
    };
    let _ = writeln!(
        log,
        "TGCX Volunteers 0.0.16 development; experimental MP whole-army gameplay; current-load peer admission required"
    );
    if exe_hash(&exe).as_deref() != Some(EXE_SHA) {
        let _ = writeln!(
            log,
            "Unsupported executable; no game memory inspected or hooks installed"
        );
        return false;
    }
    let base = unsafe { GetModuleHandleW(null()) } as u32;
    if base == 0 || api.game_base != base {
        let _ = writeln!(log, "API base mismatch; no hook");
        return false;
    }
    let settings_text = match std::fs::read_to_string(package.join("tgcx_volunteers.cfg")) {
        Ok(s) => s,
        Err(e) => {
            let _ = writeln!(log, "Cannot read settings: {e}");
            return false;
        }
    };
    let settings = match Settings::parse(&settings_text) {
        Ok(s) => s,
        Err(e) => {
            let _ = writeln!(log, "Invalid settings: {e}");
            return false;
        }
    };
    if unsafe { IsProcessorFeaturePresent(6) } == 0 {
        let _ = writeln!(log, "SSE/FXSAVE unavailable");
        return false;
    }
    let mut code = [0u8; 16];
    if !read_bytes(base + TICK_EXECUTE, &mut code) || !tick_signature_matches(base, &code) {
        let _ = writeln!(log, "Tick execute signature mismatch; original left alone");
        return false;
    }
    let original = (base + TICK_EXECUTE) as usize;
    if LiveMemory.read32(base + TICK_SLOT) != Some(original as u32) {
        let _ = writeln!(log, "Tick slot already changed; original left alone");
        return false;
    }
    let mut pinned = null_mut();
    if unsafe { GetModuleHandleExW(5, initialize as *const () as *const u16, &mut pinned) } == 0 {
        return false;
    }
    let _ = writeln!(
        log,
        "Supported EXE; base={base:08X}; observing native command slot only; max_brigades={} (planned policy)",
        settings.max_brigades
    );
    let _ = log.flush();
    let gui_settings = settings.clone();
    if CONTEXT
        .set(Context {
            base,
            state: Mutex::new(Observation {
                log,
                settings,
                calls: 0,
                last: None,
                last_stamp: None,
                first_thread: None,
            }),
        })
        .is_err()
    {
        return false;
    }
    ORIGINAL.store(original, Ordering::Release);
    let patch = crate::patch::replace(
        base + TICK_SLOT,
        original as u32,
        tick_observer as *const () as u32,
    );
    let installed = patch.installed;
    let restored = patch.protection_restored;
    if let Some(c) = CONTEXT.get()
        && let Ok(mut s) = c.state.lock()
    {
        let _ = writeln!(
            s.log,
            "observer_installed={installed} protection_restored={restored} rolled_back={}",
            patch.rolled_back
        );
        let _ = s.log.flush();
    }
    // No inline instructions changed, so no instruction-cache flush is needed.
    if installed && restored {
        let prototype = gui_settings.prototype;
        let mp_probe = gui_settings.mp_probe;
        let mp_play = gui_settings.mp_play;
        let max_brigades = gui_settings.max_brigades;
        diagnostic(&format!(
            "selected_package={} settings={gui_settings:?}",
            package.display()
        ));
        // Check before our own vtable replacements. An unrelated gameplay
        // mismatch must not prevent an independently validated UI from loading.
        let gameplay_native = crate::native::Native::new(base).is_some();
        let lifecycle_native =
            crate::native::Native::checked(base, |r| [0x262010, 0xA0A34C].contains(&r)).is_some();
        let journal_native = crate::native::Native::checked(base, |r| {
            [
                0x183900, 0x1839C0, 0x183C00, 0x183C1E, 0x183C7F, 0xA0121C, 0x241BA0, 0x9FBB60,
            ]
            .contains(&r)
        })
        .is_some();
        let transport_native = crate::native::Native::checked(base, |r| {
            [0x662EA0, 0x666B30, 0x666D63, 0xA42728, 0xA427C8, 0xA42840].contains(&r)
        })
        .is_some();
        let plugin_hash = exe_hash(&package.join("v2dll_mod.dll"));
        if let Some(digest) = exe_hash(&package.join("v2dll_mod.dll"))
            .and_then(|digest| crate::mp_sync::fingerprint(&digest, gui_settings.max_brigades))
        {
            diagnostic(&format!(
                "mp_sync_initialized fingerprint={digest} probe_requested={mp_probe} (journal mirror only; no MP gameplay authority)"
            ));
            if transport_native && journal_native && lifecycle_native {
                crate::native_mp::initialize(mp_probe, digest);
            }
        } else {
            diagnostic("MP sync diagnostic fingerprint unavailable; no MP probe initialized");
        }
        let gui_installed =
            gui_settings.ui_preview && crate::native_gui::install(base, gui_settings);
        let load_fence =
            gui_installed && lifecycle_native && crate::native_lifecycle::install(base);
        let journal_observer = load_fence && journal_native && crate::native_journal::install(base);
        let scoped_hooks = prototype
            && gameplay_native
            && transport_native
            && journal_observer
            && api.steal_to_trampoline != 0
            && crate::native_hooks::install(base, unsafe {
                std::mem::transmute::<usize, crate::native_hooks::Steal>(api.steal_to_trampoline)
            });
        if let Some(hash) = plugin_hash {
            crate::native_mp_play::initialize(mp_play && scoped_hooks, &hash, max_brigades);
        }
        diagnostic(&format!(
            "native_module_gates gameplay={gameplay_native} lifecycle={lifecycle_native} journal={journal_native} transport={transport_native} mp_gameplay_requested={mp_play}"
        ));
        if let Some(c) = CONTEXT.get()
            && let Ok(mut s) = c.state.lock()
        {
            let _ = writeln!(
                s.log,
                "native_gui_installed={gui_installed} native_load_fence={load_fence} native_journal_observer={journal_observer} prototype_requested={prototype} scoped_hooks={scoped_hooks}; session/load/queue/registry/peer gates still required"
            );
            let _ = s.log.flush();
        }
    }
    // A rare residual pointer must keep a valid pinned hook/context. Report it
    // explicitly; a false return must not invite a loader to unload active code.
    installed
}

/// # Safety
/// V2DLL must supply its v1 table, alive for this initialization call.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn V2Mod_Init(api: *const V2ModApi) -> i32 {
    if api.is_null() {
        return 0;
    }
    // The supported loader owns this pointer. Check the advertised size before
    // accessing trailing fields, including function pointers.
    let mut size = [0u8; 4];
    if !read_bytes(api as u32, &mut size) {
        return 0;
    }
    let size = u32::from_le_bytes(size);
    if size < size_of::<V2ModApi>() as u32 {
        return 0;
    }
    // Copy through the bounded reader: the loader pointer may be unaligned or
    // truncated. Never form an aligned Rust reference to foreign storage.
    let mut local = std::mem::MaybeUninit::<V2ModApi>::zeroed();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(local.as_mut_ptr().cast::<u8>(), size_of::<V2ModApi>())
    };
    if !read_bytes(api as u32, bytes) {
        return 0;
    }
    let local = unsafe { local.assume_init() };
    let api = &local;
    if api.version != 1 {
        return 0;
    }
    let ok = initialize(api);
    if let Some(log) = api.log {
        let message = if INITIALIZED.load(Ordering::SeqCst) == 4 {
            c"package not selected; inactive"
        } else if ok {
            c"development initialized; inspect native admission gates in Logs/tgcx_volunteers.log"
        } else {
            c"refused; no volunteer commands enabled"
        };
        let _ = crate::native_call::call(|| unsafe {
            log(c"TGCX Volunteers: %s".as_ptr(), message.as_ptr())
        });
    }
    i32::from(ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    static HOOK_TEST_LOCK: Mutex<()> = Mutex::new(());
    static SELF_SEEN: AtomicU32 = AtomicU32::new(0);
    static FIXTURE_CALLS: AtomicU32 = AtomicU32::new(0);
    #[unsafe(naked)]
    unsafe extern "C" fn fixture_tick() {
        std::arch::naked_asm!(
            "mov dword ptr [{seen}], ecx",
            "lock inc dword ptr [{calls}]",
            "mov eax, 0x12345678", "mov ecx, 0x23456789", "mov edx, 0x3456789A",
            "movss xmm0, dword ptr [{sse}]", "stc", "ret",
            seen=sym SELF_SEEN, sse=sym SSE_VALUE,calls=sym FIXTURE_CALLS,
        );
    }
    static SSE_VALUE: u32 = 0x3F123456;
    #[test]
    fn observer_calls_original_once_and_preserves_x86_results() {
        let _lock = HOOK_TEST_LOCK.lock().unwrap();
        ORIGINAL.store(fixture_tick as *const () as usize, Ordering::SeqCst);
        DEPTH.store(0, Ordering::SeqCst);
        FIXTURE_CALLS.store(0, Ordering::SeqCst);
        let (a, c, d, flags, sse): (u32, u32, u32, u32, u32);
        unsafe {
            SetLastError(0x54321);
            std::arch::asm!(
                "call {entry}", "pushfd", "pop {flags:e}", "movd {sse:e}, xmm0",
                entry=sym tick_observer,
                inlateout("ecx") 0x76543210u32=>c, lateout("eax") a,lateout("edx") d,
                flags=lateout(reg) flags, sse=lateout(reg) sse,
                out("xmm0") _,
            );
        }
        assert_eq!((a, c, d), (0x12345678, 0x23456789, 0x3456789A));
        assert_eq!(flags & 1, 1);
        assert_eq!(sse, SSE_VALUE);
        assert_eq!(SELF_SEEN.load(Ordering::Relaxed), 0x76543210);
        assert_eq!(DEPTH.load(Ordering::Relaxed), 0);
        assert_eq!(FIXTURE_CALLS.load(Ordering::Relaxed), 1);
        assert_eq!(unsafe { GetLastError() }, 0x54321);
        ORIGINAL.store(0, Ordering::SeqCst);
    }
    #[test]
    fn loader_rejects_null_small_and_wrong_api_without_patching() {
        let _lock = HOOK_TEST_LOCK.lock().unwrap();
        assert_eq!(unsafe { V2Mod_Init(null()) }, 0);
        let tiny = 4u32;
        assert_eq!(unsafe { V2Mod_Init((&raw const tiny).cast()) }, 0);
        let api = V2ModApi {
            size: size_of::<V2ModApi>() as u32,
            version: 2,
            game_base: 0,
            mod_folder: null(),
            log: None,
            patch_slot: 0,
            steal_to_trampoline: 0,
        };
        assert_eq!(unsafe { V2Mod_Init(&api) }, 0);
        assert_eq!(ORIGINAL.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn known_hash_vectors() {
        for (text, digest) in [
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
        ] {
            let mut s = Sha256::new();
            s.update(text.as_bytes());
            assert_eq!(s.finish(), digest);
        }
    }
    #[test]
    fn signatures_accept_relocation_but_refuse_foreign_code() {
        for base in [0x400000u32, 0x80000] {
            let mut code = [
                0xA1, 0, 0, 0, 0, 0x83, 0xC1, 0x3C, 0x51, 0x50, 0xE8, 0xC1, 0x41, 0xF5, 0xFF, 0xC3,
            ];
            code[1..5].copy_from_slice(&(base + 0xE588E8).to_le_bytes());
            assert!(tick_signature_matches(base, &code));
            assert!(!tick_signature_matches(base + 0x10000, &code));
            code[0] = 0xE9;
            assert!(!tick_signature_matches(base, &code));
        }
        assert!(!tick_signature_matches(u32::MAX, &[0; 16]));
    }
}
