//! Native combat/access hooks. Installation is separate from admission: no
//! callback borrows war context without a published, admitted SP registry.
use crate::combat::Decision;
use crate::native::Native;
use crate::native_call::call;
use crate::registry::Registry;
use crate::session::SinglePlayerPermit;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{OnceLock, RwLock};

static BASE: AtomicU32 = AtomicU32::new(0);
static PAIR: AtomicUsize = AtomicUsize::new(0);
static SIDE: AtomicUsize = AtomicUsize::new(0);
static ACCESS: AtomicUsize = AtomicUsize::new(0);
static VISION: AtomicUsize = AtomicUsize::new(0);
static COUNTRY: AtomicUsize = AtomicUsize::new(0);
static COUNTRY_CALLER: AtomicU32 = AtomicU32::new(0);
static EXILE_ACCESS: AtomicUsize = AtomicUsize::new(0);
static COUNTRY_BYPASS: AtomicU32 = AtomicU32::new(0);
static COUNTRY_TARGET: AtomicU32 = AtomicU32::new(0);
static EXILE_GRANTED: AtomicU32 = AtomicU32::new(0);
static VISION_RUNS: AtomicU32 = AtomicU32::new(0);
static VISION_GRANTS: AtomicU32 = AtomicU32::new(0);
static THREAD_REFUSALS: AtomicU32 = AtomicU32::new(0);
static SIEGE: AtomicUsize = AtomicUsize::new(0);
static SIEGE_JOIN: AtomicUsize = AtomicUsize::new(0);
static SIEGE_GRANTED: AtomicU32 = AtomicU32::new(0);
static SIEGE_JOIN_GRANTED: AtomicU32 = AtomicU32::new(0);
static VISION_REFUSED: AtomicU32 = AtomicU32::new(0);
static VISION_EMPTY: AtomicU32 = AtomicU32::new(0);
static VISION_SKIPPED: AtomicU32 = AtomicU32::new(0);
static READY: AtomicBool = AtomicBool::new(false);
// Assembly reads this aligned word only to skip the plugin when no scope is
// published. A nonzero value never grants permission; Rust retains every gate.
static ACTIVE: AtomicU32 = AtomicU32::new(0);
struct Published {
    registry: Registry,
    permit: SinglePlayerPermit,
    thread: u32,
}
static SCOPE: OnceLock<RwLock<Option<Published>>> = OnceLock::new();
pub fn publish(registry: Option<&Registry>, permit: Option<SinglePlayerPermit>) {
    ACTIVE.store(0, Ordering::Release);
    if let Ok(mut scope) = SCOPE.get_or_init(|| RwLock::new(None)).try_write() {
        *scope = registry
            .filter(|r| r.has_native_scope())
            .zip(permit)
            .map(|(r, p)| Published {
                registry: r.clone(),
                permit: p,
                thread: unsafe { crate::native::GetCurrentThreadId() },
            });
        ACTIVE.store(u32::from(scope.is_some()), Ordering::Release);
    }
}
fn with_scope<R>(f: impl FnOnce(Native, &Registry, SinglePlayerPermit) -> Option<R>) -> Option<R> {
    with_filtered_scope(|_, _| true, f)
}
fn with_filtered_scope<R>(
    relevant: impl FnOnce(Native, &Registry) -> bool,
    f: impl FnOnce(Native, &Registry, SinglePlayerPermit) -> Option<R>,
) -> Option<R> {
    if !READY.load(Ordering::Acquire) || crate::native_call::faulted() {
        return None;
    }
    let scope = SCOPE.get()?.try_read().ok()?;
    let scope = scope.as_ref()?;
    if scope.thread != unsafe { crate::native::GetCurrentThreadId() } {
        THREAD_REFUSALS.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    if crate::native_lifecycle::current_epoch() != Some(scope.registry.epoch) {
        return None;
    }
    let n = Native {
        base: BASE.load(Ordering::Acquire),
    };
    if !relevant(n, &scope.registry) {
        return None;
    }
    let session = crate::native_session::capture(&n, n.base)?;
    if session.mode != crate::session::SessionMode::SinglePlayer
        || crate::native_lifecycle::epoch(session.context) != Some(scope.registry.epoch)
        || scope.permit.epoch() != scope.registry.epoch
    {
        return None;
    }
    f(n, &scope.registry, scope.permit)
}
fn result(native: u32, decision: Decision) -> u32 {
    match decision {
        Decision::Native => native,
        Decision::Reject => native & !255,
        Decision::Scoped { hostile, .. } => (native & !255) | u32::from(hostile),
    }
}
extern "C" fn pair(left: u32, right: u32, province: u32, native: u32) -> u32 {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::PAIR.begin();
    #[cfg(test)]
    tests::PAIR_CALLBACKS.fetch_add(1, Ordering::Relaxed);
    with_filtered_scope(
        |n, r| crate::native_combat::pair_references_loan(&n, r, left, right) == Some(true),
        |n, r, p| {
            Some(result(
                native,
                crate::native_combat::evaluate(&n, n.base, r, p, left, right, province),
            ))
        },
    )
    .unwrap_or(native)
}
unsafe fn side_native(header: *const u32, army: u32) -> u32 {
    let value: u32;
    unsafe {
        std::arch::asm!("push {army}","call {original}",
        army=in(reg) army,original=in(reg) SIDE.load(Ordering::Acquire),
        inlateout("eax") header=>value,clobber_abi("C"));
    }
    value
}
extern "C" fn side(header: u32, candidate: u32, native: u32) -> u32 {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::SIDE.begin();
    with_filtered_scope(
        |n, r| {
            if crate::native_combat::references_loan(&n, r, candidate) == Some(true) {
                return true;
            }
            n.list(header, 2048).is_some_and(|members| {
                members
                    .into_iter()
                    .any(|army| crate::native_combat::references_loan(&n, r, army) == Some(true))
            })
        },
        |n, r, p| {
            let members = n.list(header, 2048)?;
            if members.is_empty() {
                return Some(native);
            }
            let mut scoped = false;
            for member in members {
                let province = n.field(member, 0xDC)?;
                match crate::native_combat::evaluate(&n, n.base, r, p, member, candidate, province)
                {
                    Decision::Reject => return Some(native & !255),
                    Decision::Scoped { hostile, .. } => {
                        scoped = true;
                        if !hostile {
                            return Some(native & !255);
                        }
                    }
                    Decision::Native => {
                        // Run the unmodified native join predicate for this actual
                        // pair. A private one-node list does not edit side membership.
                        let node = [member, 0, 0, 0];
                        let list = [node.as_ptr() as u32, node.as_ptr() as u32, 1];
                        if call(|| unsafe { side_native(list.as_ptr(), candidate) })? as u8 == 0 {
                            return Some(native & !255);
                        }
                    }
                }
            }
            Some(if scoped { (native & !255) | 1 } else { native })
        },
    )
    .unwrap_or(native)
}
extern "C" fn access(army: u32, province: u32, native: u32) -> u32 {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::ACCESS.begin();
    if native as u8 != 0 {
        return native;
    }
    with_filtered_scope(
        |n, r| crate::native_combat::references_loan(&n, r, army) == Some(true),
        |n, r, p| {
            n.recipient_land_access(r, p, army, province, false)
                .map(|allowed| (native & !255) | u32::from(allowed))
        },
    )
    .unwrap_or(native)
}
// Only the native CCombatant add-army country insertion call is remapped.
// Native battle finalization reads side+34's country pair, then relation+34
// (FUN_005A0180 VA5A0212..5A0238) to associate the result with a war.
extern "C" fn country_ref(vector: u32, source: u32, caller: u32, output: *mut u32) -> u32 {
    let _error = crate::native::LastError::preserve();
    if caller != COUNTRY_CALLER.load(Ordering::Acquire) {
        return 0;
    }
    COUNTRY_TARGET.fetch_add(1, Ordering::Relaxed);
    with_filtered_scope(
        |n, r| {
            source.checked_sub(0xC0).is_some_and(|army| {
                crate::native_combat::references_loan(&n, r, army) == Some(true)
            })
        },
        |n, r, _| {
            if caller != n.base + 0x19456E {
                return None;
            }
            let army = source.checked_sub(0xC0)?;
            let side = vector.checked_sub(0x34)?;
            if n.u32(army)? != n.base + 0x9F88CC || n.u32(side)? != n.base + 0xA02294 {
                return None;
            }
            let id = crate::model::Identity {
                kind: n.field(army, 0x10)?,
                id: n.field(army, 0x14)?,
            };
            let entry = r.live_army(id)?;
            let donor = n.country(entry.deployment.key.donor)?;
            let recipient = n.country(entry.deployment.key.recipient)?;
            if n.byte(donor + 0x67C)? != 0 || n.byte(recipient + 0x67C)? == 0 {
                return None;
            }
            let receipt = n.receipt(r.campaign, id)?;
            let index = r.context(
                &receipt,
                crate::mission::Use::Combat,
                crate::mission::RecipientWar::AtWar,
            )?;
            let start = n.u32(vector)?;
            let end = n.field(vector, 4)?;
            let bytes = end.checked_sub(start)?;
            if bytes % 8 != 0 || bytes / 8 > 4096 {
                return None;
            }
            for offset in (0..bytes).step_by(8) {
                if n.field(start + offset, 4)? == index {
                    return Some(2);
                }
            }
            let tag = n.field(recipient, 0x1C)?;
            // output points to this wrapper's own eight-byte stack buffer.
            unsafe {
                output.write(tag);
                output.add(1).write(index);
            }
            Some(1)
        },
    )
    .unwrap_or(0)
}
#[unsafe(naked)]
unsafe extern "C" fn country_hook() {
    std::arch::naked_asm!(
    // This vector helper is shared by unrelated systems. Preserve registers
    // and flags, and tail-call the original before FXSAVE/Rust for other callers.
    "pushfd","push eax","mov eax,[{caller}]","cmp [esp+8],eax","je 4f",
    "lock inc dword ptr [{bypass}]","pop eax","popfd","jmp dword ptr [{original}]","4:","pop eax","popfd",
    "sub esp,8","pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "lea eax,[ebx+36]","push eax","push dword ptr [ebx+44]","push dword ptr [ebx+28]","push dword ptr [ebx]","call {callback}","add esp,16",
    "cmp eax,2","je 3f","cmp eax,1","jne 2f","lea eax,[ebx+36]","mov [ebx+28],eax",
    "2:","fxrstor [esp]","mov esp,ebx","popad","popfd","call dword ptr [{original}]","lea esp,[esp+8]","ret",
    "3:","fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+8]","ret",
    original=sym COUNTRY,callback=sym country_ref,caller=sym COUNTRY_CALLER,bypass=sym COUNTRY_BYPASS);
}
/// Only the four proven exile-coordinator callers carry the army in EDI/ESI.
/// The original engine still computes and writes its own exile state. We only
/// supply recipient permission to its country/province read predicate.
extern "C" fn exile_access(edi: u32, esi: u32, province: u32, caller: u32, native: u32) -> u32 {
    #[cfg(test)]
    tests::capture_args(edi, esi, province, caller);
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::EXILE.begin();
    let base = BASE.load(Ordering::Acquire);
    let army = match caller.wrapping_sub(base) {
        0x1C8288 | 0x1C8314 => edi,
        0x1DBE62 | 0x1DBED2 => esi,
        _ => return native,
    };
    if native as u8 != 0 {
        return native;
    }
    with_filtered_scope(
        |n, r| crate::native_combat::references_loan(&n, r, army) == Some(true),
        |n, r, p| {
            if n.field(army, 0xDC)? != province {
                return None;
            }
            let allowed = n.recipient_land_access(r, p, army, province, false)?;
            if allowed {
                EXILE_GRANTED.fetch_add(1, Ordering::Relaxed);
            }
            Some((native & !255) | u32::from(allowed))
        },
    )
    .unwrap_or(native)
}
#[unsafe(naked)]
unsafe extern "C" fn exile_access_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "pushfd","push eax","mov eax,[{base}]","add eax,0x1C8288","cmp [esp+8],eax","je 5f",
    "mov eax,[{base}]","add eax,0x1C8314","cmp [esp+8],eax","je 5f",
    "mov eax,[{base}]","add eax,0x1DBE62","cmp [esp+8],eax","je 5f",
    "mov eax,[{base}]","add eax,0x1DBED2","cmp [esp+8],eax","je 5f",
    "pop eax","popfd","jmp dword ptr [{original}]","5:","pop eax","popfd",
    // Original EAX=province, one stack country pointer, RET4. Save EDI/ESI
    // because they identify different coordinators, and keep the original AL.
    "push edi","push esi","push eax","push dword ptr [esp+16]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+48]","push dword ptr [ebx+36]","push dword ptr [ebx+40]","push dword ptr [ebx+44]","call {callback}","add esp,20","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+12]","ret 4",
    base=sym BASE,active=sym ACTIVE,original=sym EXILE_ACCESS,callback=sym exile_access);
}
extern "C" fn siege_permission(army: u32, province: u32, native: u32) -> u32 {
    #[cfg(test)]
    tests::capture_args(army, province, 0, 0);
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::SIEGE.begin();
    if native as u8 != 0 {
        return native;
    }
    with_filtered_scope(
        |n, r| crate::native_combat::references_loan(&n, r, army) == Some(true),
        |n, r, p| {
            if n.field(army, 0xDC)? != province {
                return None;
            }
            let input =
                n.recipient_army_image(r, p, army, province, crate::mission::Use::Combat)?;
            let original = SIEGE.load(Ordering::Acquire);
            let allowed = call(|| unsafe {
                let result: u32;
                // FUN_005CC200: ESIprovince, stack army image, ALresult, RET4.
                std::arch::asm!("push esi","mov esi,edx","push ecx","call eax","pop esi",
                in("ecx") input.as_ptr(), in("edx") province,
                inlateout("eax") original => result, clobber_abi("C"));
                result as u8 != 0
            })?;
            if allowed {
                SIEGE_GRANTED.fetch_add(1, Ordering::Relaxed);
            }
            Some((native & !255) | u32::from(allowed))
        },
    )
    .unwrap_or(native)
}
#[unsafe(naked)]
unsafe extern "C" fn siege_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push esi","push dword ptr [esp+8]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+36]","push dword ptr [ebx+44]","call {callback}","add esp,12","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret 4",
    active=sym ACTIVE,original=sym SIEGE,callback=sym siege_permission);
}
extern "C" fn siege_join(side: u32, army: u32, native: u32) -> u32 {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::JOIN.begin();
    if native as u8 != 0 {
        return native;
    }
    with_filtered_scope(
        |n, r| crate::native_combat::references_loan(&n, r, army) == Some(true),
        |n, r, p| {
            if n.u32(side)? != n.base + 0xA022F4 {
                return None;
            }
            let combat = n.field(side, 0x1C)?;
            if n.u32(combat)? != n.base + 0xA02434 {
                return None;
            }
            let province = n.field(combat, 0x18)?;
            if n.field(army, 0xDC)? != province {
                return None;
            }
            if n.byte(army + 0x164)? != 0 || n.byte(army + 0x104)? != 0 || n.field(army, 0x74)? != 0
            {
                return None;
            }
            let input =
                n.recipient_army_image(r, p, army, province, crate::mission::Use::Combat)?;
            // The outer join routine compared the real donor to the siege leader.
            // Preserve that friendly-side gate using the scoped recipient as well.
            let attacker = n.field(combat, 0x10)?;
            let start = n.field(attacker, 0x34)?;
            let end = n.field(attacker, 0x38)?;
            let size = end.checked_sub(start)?;
            if size % 8 != 0 || size / 8 > 4096 {
                return None;
            }
            if size != 0 {
                let leader = n.field(start, 4)?;
                let host = input[0xC4 / 4];
                if leader != host {
                    let country = n.country(host)?;
                    let relations = n.field(country, 0xBE8)?;
                    let relation = n.field(relations, leader.checked_mul(4)?)?;
                    let reb = u32::from_le_bytes(*b"REB\0") & 0xFFFFFF;
                    if n.field(relation, 0x34)? != 0
                        || n.u32(start)? & 0xFFFFFF == reb
                        || input[0xC0 / 4] & 0xFFFFFF == reb
                    {
                        return Some(native & !255);
                    }
                }
            }
            type Predicate = unsafe extern "fastcall" fn(u32, u32, *const u32) -> u8;
            let allowed = call(|| unsafe {
                std::mem::transmute::<usize, Predicate>(SIEGE_JOIN.load(Ordering::Acquire))(
                    side,
                    0,
                    input.as_ptr(),
                )
            })? != 0;
            if allowed {
                SIEGE_JOIN_GRANTED.fetch_add(1, Ordering::Relaxed);
            }
            Some((native & !255) | u32::from(allowed))
        },
    )
    .unwrap_or(native)
}
#[unsafe(naked)]
unsafe extern "C" fn siege_join_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push ecx","push dword ptr [esp+8]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+44]","push dword ptr [ebx+36]","call {callback}","add esp,12","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret 4",
    active=sym ACTIVE,original=sym SIEGE_JOIN,callback=sym siege_join);
}
pub fn diagnostics() {
    crate::runtime::diagnostic(&format!(
        "scope_counters country_bypass={} country_target={} exile_granted={} vision_runs={} vision_grants={} thread_refusals={} siege_granted={} siege_join_granted={} vision_refused={} vision_empty={}",
        COUNTRY_BYPASS.load(Ordering::Relaxed),
        COUNTRY_TARGET.load(Ordering::Relaxed),
        EXILE_GRANTED.load(Ordering::Relaxed),
        VISION_RUNS.load(Ordering::Relaxed),
        VISION_GRANTS.load(Ordering::Relaxed),
        THREAD_REFUSALS.load(Ordering::Relaxed),
        SIEGE_GRANTED.load(Ordering::Relaxed),
        SIEGE_JOIN_GRANTED.load(Ordering::Relaxed),
        VISION_REFUSED.load(Ordering::Relaxed),
        VISION_EMPTY.load(Ordering::Relaxed)
    ));
    crate::runtime::diagnostic(&format!(
        "vision_skipped_provinces={}",
        VISION_SKIPPED.load(Ordering::Relaxed)
    ));
    crate::runtime::diagnostic(&format!(
        "plugin_timing active={} {} (cumulative; sampled Rust callback wall time, excludes native originals and assembly save/restore)",
        ACTIVE.load(Ordering::Acquire),
        crate::timing::report()
    ));
}
pub fn ready() -> bool {
    READY.load(Ordering::Acquire)
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn WriteProcessMemory(
        process: *mut c_void,
        address: *mut c_void,
        data: *const c_void,
        size: usize,
        written: *mut usize,
    ) -> i32;
}
fn write_ui(address: u32, bytes: &[u8]) -> bool {
    if address < 0x10000 || address.checked_add(bytes.len() as u32).is_none() {
        return false;
    }
    let mut written = 0;
    unsafe {
        WriteProcessMemory(
            GetCurrentProcess(),
            address as *mut c_void,
            bytes.as_ptr().cast(),
            bytes.len(),
            &mut written,
        ) != 0
            && written == bytes.len()
    }
}
extern "C" fn vision(world: u32) {
    let _error = crate::native::LastError::preserve();
    let _timing = crate::timing::VISION.begin();
    let _ = with_scope(|n, r, p| {
        VISION_RUNS.fetch_add(1, Ordering::Relaxed);
        let fronts = r
            .entries()
            .filter_map(|entry| {
                let receipt = n.receipt(r.campaign, entry.mission.plan().destination())?;
                let recipient = n.country(entry.deployment.key.recipient)?;
                if n.byte(recipient + 0x67C)? == 0 {
                    return None;
                }
                entry.mission.context_country(
                    &receipt,
                    crate::mission::Use::Access,
                    crate::mission::RecipientWar::AtWar,
                )?;
                n.field(n.army(receipt.army)?, 0xDC)
            })
            .collect::<Vec<_>>();
        let Some(plan) = crate::native_vision::plan_with_fronts(&n, n.base, r, p, &fronts) else {
            VISION_REFUSED.fetch_add(1, Ordering::Relaxed);
            return None;
        };
        VISION_SKIPPED.fetch_add(plan.skipped_provinces, Ordering::Relaxed);
        if plan.world != world || plan.grants.is_empty() {
            VISION_EMPTY.fetch_add(1, Ordering::Relaxed);
            return Some(());
        }
        let context = n.field(world, 0xB24)?;
        let holder = n.field(context, 0x17E8)?;
        // The original visibility updater calls slot14 (FUN_00651CE0) and
        // marks this map-view render texture dirty. Never edit province+174.
        let map_view = n.field(holder, 0x124)?;
        let texture = n.field(map_view, 0x3648)?;
        n.bytes(texture + 0x3D, 1)?;
        let flags = n.field(context, 0x74)?;
        for grant in plan.grants {
            if n.field(grant.map, 0x360)? != grant.native_level {
                return None;
            }
            if !write_ui(grant.map + 0x360, &grant.requested_level.to_le_bytes()) {
                return None;
            }
            VISION_GRANTS.fetch_add(1, Ordering::Relaxed);
        }
        if !write_ui(texture + 0x3D, &[1])
            || !write_ui(context + 0x74, &(flags | 0x100).to_le_bytes())
        {
            return None;
        }
        Some(())
    });
}
// Native world visibility recompute: stack world,RET4; original first.
#[unsafe(naked)]
unsafe extern "C" fn vision_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push dword ptr [esp+4]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+40]","call {callback}","add esp,4",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","ret 4",
    active=sym ACTIVE,original=sym VISION,callback=sym vision);
}

// FUN_005CBF60: ECX=left,EAX=right,stack=province,AL result,RET4.
// Save input references separately from PUSHAD because the native result
// overwrites EAX. Every callback preserves flags, x87/SSE and all other registers.
#[unsafe(naked)]
unsafe extern "C" fn pair_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push ecx","push eax","push dword ptr [esp+12]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+48]","push dword ptr [ebx+36]","push dword ptr [ebx+40]","call {callback}","add esp,16","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+8]","ret 4",
    active=sym ACTIVE,original=sym PAIR,callback=sym pair);
}
// FUN_005DBB50: EAX=list header,stack=candidate,AL result,RET4.
#[unsafe(naked)]
unsafe extern "C" fn side_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push eax","push dword ptr [esp+8]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+44]","push dword ptr [ebx+36]","call {callback}","add esp,12","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret 4",
    active=sym ACTIVE,original=sym SIDE,callback=sym side);
}
// FUN_005D5E30: ECX=army,stack=province,AL result,RET4.
#[unsafe(naked)]
unsafe extern "C" fn access_hook() {
    std::arch::naked_asm!(
    "pushfd","cmp dword ptr [{active}],0","jne 9f","popfd","jmp dword ptr [{original}]","9:","popfd",
    "push ecx","push dword ptr [esp+8]","call dword ptr [{original}]",
    "pushfd","pushad","mov ebx,esp","sub esp,528","and esp,-16","fxsave [esp]","fninit","mov dword ptr [esp+512],0x1F80","ldmxcsr [esp+512]","cld",
    "push dword ptr [ebx+28]","push dword ptr [ebx+44]","push dword ptr [ebx+36]","call {callback}","add esp,12","mov [ebx+28],eax",
    "fxrstor [esp]","mov esp,ebx","popad","popfd","lea esp,[esp+4]","ret 4",
    active=sym ACTIVE,original=sym ACCESS,callback=sym access);
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(address: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn GetCurrentProcess() -> *mut c_void;
    fn FlushInstructionCache(process: *mut c_void, address: *const c_void, size: usize) -> i32;
}
// from V2DLL: V2ModAPI.h, plugin API1 declaration only.
pub type Steal = unsafe extern "C" fn(
    u32,
    u32,
    *mut u8,
    i32,
    *const u8,
    *const c_void,
    *mut *mut c_void,
    *const c_char,
) -> i32;
fn inline(
    base: u32,
    steal: Steal,
    rva: u32,
    hook: usize,
    original: &AtomicUsize,
    expect: [u8; 6],
    tag: &'static std::ffi::CStr,
) -> bool {
    let n = Native { base };
    if n.bytes(base + rva, 6).as_deref() != Some(&expect) {
        return false;
    }
    let trampoline = unsafe { VirtualAlloc(std::ptr::null_mut(), 64, 0x3000, 0x40) }.cast::<u8>();
    if trampoline.is_null() {
        return false;
    }
    // The stolen prologue has no address operand. Seed a valid original path
    // before asking the SDK to redirect, including its uncertain-failure case.
    let mut fallback = [0u8; 11];
    fallback[..6].copy_from_slice(&expect);
    fallback[6] = 0xE9;
    fallback[7..].copy_from_slice(
        &(base + rva + 6)
            .wrapping_sub(trampoline as u32 + 11)
            .to_le_bytes(),
    );
    unsafe {
        std::ptr::copy_nonoverlapping(fallback.as_ptr(), trampoline, 11);
    }
    if unsafe { FlushInstructionCache(GetCurrentProcess(), trampoline.cast(), 11) } == 0 {
        return false;
    }
    // Retain for the pinned process lifetime, including an uncertain SDK result.
    original.store(trampoline as usize, Ordering::Release);
    let mut returned = std::ptr::null_mut();
    let output = &raw mut returned;
    let success = call(|| unsafe {
        steal(
            rva,
            6,
            trampoline,
            64,
            expect.as_ptr(),
            hook as *const c_void,
            output,
            tag.as_ptr(),
        )
    });
    success == Some(1) && returned == trampoline.cast()
}
/// Exact native signature preflight and process-lifetime DLL pinning must pass.
/// This does not mark battle attribution or multiplayer as established.
pub fn install(base: u32, steal: Steal) -> bool {
    BASE.store(base, Ordering::Release);
    COUNTRY_CALLER.store(base + 0x19456E, Ordering::Release);
    let a = inline(
        base,
        steal,
        0x1CBF60,
        pair_hook as *const () as usize,
        &PAIR,
        [0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8],
        c"tgcx_volunteer_pair",
    );
    let b = a
        && inline(
            base,
            steal,
            0x1DBB50,
            side_hook as *const () as usize,
            &SIDE,
            [0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8],
            c"tgcx_volunteer_side",
        );
    let n = Native { base };
    let c = if b && n.u32(base + 0x9F88CC + 33 * 4) == Some(base + 0x1D5E30) {
        ACCESS.store((base + 0x1D5E30) as usize, Ordering::Release);
        crate::patch::replace(
            base + 0x9F88CC + 33 * 4,
            base + 0x1D5E30,
            access_hook as *const () as u32,
        )
        .complete()
    } else {
        false
    };
    let d = c
        && inline(
            base,
            steal,
            0x28C040,
            vision_hook as *const () as usize,
            &VISION,
            [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x30],
            c"tgcx_volunteer_vision",
        );
    let e = d
        && inline(
            base,
            steal,
            0xD61D0,
            country_hook as *const () as usize,
            &COUNTRY,
            [0x56, 0x8B, 0xF0, 0x8B, 0x47, 0x04],
            c"tgcx_volunteer_battle_country",
        );
    let f = e
        && inline(
            base,
            steal,
            0x11B040,
            exile_access_hook as *const () as usize,
            &EXILE_ACCESS,
            [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10],
            c"tgcx_volunteer_exile_access",
        );
    let g = f
        && inline(
            base,
            steal,
            0x1CC200,
            siege_hook as *const () as usize,
            &SIEGE,
            [0x55, 0x8B, 0xEC, 0x83, 0xE4, 0xF8],
            c"tgcx_volunteer_siege_permission",
        );
    let h = if g && n.u32(base + 0xA022F4 + 16 * 4) == Some(base + 0x1A9E80) {
        SIEGE_JOIN.store((base + 0x1A9E80) as usize, Ordering::Release);
        crate::patch::replace(
            base + 0xA022F4 + 16 * 4,
            base + 0x1A9E80,
            siege_join_hook as *const () as u32,
        )
        .complete()
    } else {
        false
    };
    READY.store(a && b && c && d && e && f && g && h, Ordering::Release);
    crate::runtime::diagnostic(&format!(
        "native_scoped_hooks pair={a} side={b} access={c} vision={d} battle_country={e} exile_access={f} siege={g} siege_join={h}; admission remains separate"
    ));
    a && b && c && d && e && f && g && h
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    static LOCK: Mutex<()> = Mutex::new(());
    pub(super) static PAIR_CALLBACKS: AtomicU32 = AtomicU32::new(0);
    static SPY_COUNT: AtomicU32 = AtomicU32::new(0);
    static SPY_PROVINCE: AtomicU32 = AtomicU32::new(0);
    static SPY_COUNTRY: AtomicU32 = AtomicU32::new(0);
    static SPY_EDI: AtomicU32 = AtomicU32::new(0);
    static SPY_ESI: AtomicU32 = AtomicU32::new(0);
    static SPY_CALLBACK_PROVINCE: AtomicU32 = AtomicU32::new(0);
    static SPY_CALLBACK_CALLER: AtomicU32 = AtomicU32::new(0);
    static SPY_FLAGS: AtomicU32 = AtomicU32::new(0);
    #[unsafe(naked)]
    unsafe extern "C" fn pair_timing_spy() {
        std::arch::naked_asm!("mov eax,0xAABBCC01", "cmp eax,eax", "ret 4");
    }
    #[test]
    fn inactive_wrappers_keep_native_results_flags_and_skip_callbacks() {
        let _lock = LOCK.lock().unwrap();
        let active = ACTIVE.swap(0, Ordering::AcqRel);
        let old_pair = PAIR.swap(pair_timing_spy as *const () as usize, Ordering::AcqRel);
        let before = PAIR_CALLBACKS.load(Ordering::Relaxed);
        // Invalid fake pointers must never be read on the inactive route.
        for switch in [0, 1, 0] {
            ACTIVE.store(switch, Ordering::Release);
            let result: u32;
            let flags: u32;
            unsafe {
                std::arch::asm!("push 3", "call {hook}", "pushfd", "pop edx",
                    hook=sym pair_hook, inlateout("eax") 2u32 => result,
                    in("ecx") 1u32, lateout("edx") flags, clobber_abi("C"));
            }
            assert_eq!(result, 0xAABBCC01);
            assert_ne!(flags & 0x40, 0);
        }
        assert_eq!(PAIR_CALLBACKS.load(Ordering::Relaxed) - before, 1);
        let idle_slots = [
            (&SIDE, side_hook as *const () as usize),
            (&ACCESS, access_hook as *const () as usize),
            (&SIEGE, siege_hook as *const () as usize),
            (&SIEGE_JOIN, siege_join_hook as *const () as usize),
            (&EXILE_ACCESS, exile_access_hook as *const () as usize),
            (&VISION, vision_hook as *const () as usize),
        ];
        for (slot, hook) in idle_slots {
            let old = slot.swap(pair_timing_spy as *const () as usize, Ordering::AcqRel);
            let result: u32;
            let flags: u32;
            unsafe {
                std::arch::asm!("push 3", "call {hook}", "pushfd", "pop edx",
                    hook=in(reg) hook, inlateout("eax") 2u32 => result,
                    in("ecx") 1u32, lateout("edx") flags, clobber_abi("C"));
            }
            assert_eq!(result, 0xAABBCC01);
            assert_ne!(flags & 0x40, 0);
            slot.store(old, Ordering::Release);
        }
        PAIR.store(old_pair, Ordering::Release);
        ACTIVE.store(active, Ordering::Release);
    }
    #[test]
    #[ignore = "explicit full-wrapper benchmark on a fake native function"]
    fn idle_pair_wrapper_benchmark() {
        let _lock = LOCK.lock().unwrap();
        let old = PAIR.swap(pair_timing_spy as *const () as usize, Ordering::AcqRel);
        let started = std::time::Instant::now();
        for _ in 0..1_000_000 {
            let result: u32;
            unsafe {
                std::arch::asm!("push 0x55556666", "call {hook}", hook=sym pair_hook,
                    inlateout("eax") 0x33334444u32 => result,
                    in("ecx") 0x11112222u32, clobber_abi("C"));
            }
            std::hint::black_box(result);
            assert_eq!(result, 0xAABBCC01);
        }
        println!(
            "IDLE_PAIR_WRAPPER loops=1000000 elapsed_ns={}",
            started.elapsed().as_nanos()
        );
        PAIR.store(old, Ordering::Release);
    }
    pub(super) fn capture_args(edi: u32, esi: u32, province: u32, caller: u32) {
        SPY_EDI.store(edi, Ordering::Relaxed);
        SPY_ESI.store(esi, Ordering::Relaxed);
        SPY_CALLBACK_PROVINCE.store(province, Ordering::Relaxed);
        SPY_CALLBACK_CALLER.store(caller, Ordering::Relaxed);
    }
    extern "C" fn country_spy() -> u32 {
        SPY_COUNT.fetch_add(1, Ordering::Relaxed);
        0xCAFE1234
    }
    #[test]
    fn unrelated_country_vector_calls_bypass_rust_validation() {
        let _lock = LOCK.lock().unwrap();
        let old = COUNTRY.swap(country_spy as *const () as usize, Ordering::AcqRel);
        let caller = COUNTRY_CALLER.swap(0, Ordering::AcqRel);
        let target_before = COUNTRY_TARGET.load(Ordering::Relaxed);
        let before = COUNTRY_BYPASS.load(Ordering::Relaxed);
        SPY_COUNT.store(0, Ordering::Relaxed);
        for _ in 0..100000 {
            let result: u32;
            unsafe {
                std::arch::asm!("call {hook}", hook=sym country_hook,
                lateout("eax") result, clobber_abi("C"));
            }
            assert_eq!(result, 0xCAFE1234);
        }
        assert_eq!(SPY_COUNT.load(Ordering::Relaxed), 100000);
        assert_eq!(COUNTRY_BYPASS.load(Ordering::Relaxed) - before, 100000);
        assert_eq!(COUNTRY_TARGET.load(Ordering::Relaxed), target_before);
        COUNTRY.store(old, Ordering::Release);
        COUNTRY_CALLER.store(caller, Ordering::Release);
    }
    #[unsafe(naked)]
    unsafe extern "C" fn exile_spy() {
        std::arch::naked_asm!(
            "mov [{province}],eax","mov eax,[esp+4]","mov [{country}],eax",
            "mov eax,0xAABBCC01","pxor xmm0,xmm0","cmp eax,eax","ret 4",
            province=sym SPY_PROVINCE,country=sym SPY_COUNTRY);
    }
    #[unsafe(naked)]
    unsafe extern "C" fn invoke_exile() -> u32 {
        std::arch::naked_asm!(
            "push ebx","push esi","push edi",
            "lea eax,[2f]","sub eax,0x1C8314","mov [{base}],eax",
            "mov edi,0x11112222","mov esi,0x33334444","push 0x55556666",
            "mov eax,0x77778888","call {hook}","2:",
            "pushfd","pop edx","mov [{flags}],edx",
            "cmp edi,0x11112222","jne 3f","cmp esi,0x33334444","jne 3f",
            "pop edi","pop esi","pop ebx","ret",
            "3:","xor eax,eax","pop edi","pop esi","pop ebx","ret",
            base=sym BASE,hook=sym exile_access_hook,flags=sym SPY_FLAGS);
    }
    #[test]
    fn exile_wrapper_keeps_register_arguments_native_result_flags_and_ret4() {
        let _lock = LOCK.lock().unwrap();
        let old_active = ACTIVE.swap(1, Ordering::AcqRel);
        let old = EXILE_ACCESS.swap(exile_spy as *const () as usize, Ordering::AcqRel);
        let old_base = BASE.load(Ordering::Acquire);
        let value = unsafe { invoke_exile() };
        assert_eq!(value, 0xAABBCC01);
        assert_eq!(SPY_PROVINCE.load(Ordering::Relaxed), 0x77778888);
        assert_eq!(SPY_COUNTRY.load(Ordering::Relaxed), 0x55556666);
        assert_eq!(SPY_EDI.load(Ordering::Relaxed), 0x11112222);
        assert_eq!(SPY_ESI.load(Ordering::Relaxed), 0x33334444);
        assert_eq!(SPY_CALLBACK_PROVINCE.load(Ordering::Relaxed), 0x77778888);
        assert_eq!(
            SPY_CALLBACK_CALLER
                .load(Ordering::Relaxed)
                .wrapping_sub(BASE.load(Ordering::Relaxed)),
            0x1C8314
        );
        assert_ne!(SPY_FLAGS.load(Ordering::Relaxed) & 0x40, 0); // native ZF
        BASE.store(old_base, Ordering::Release);
        EXILE_ACCESS.store(old, Ordering::Release);
        ACTIVE.store(old_active, Ordering::Release);
    }
    #[unsafe(naked)]
    unsafe extern "C" fn siege_spy() {
        std::arch::naked_asm!(
            "mov [{province}],esi","mov eax,[esp+4]","mov [{army}],eax",
            "mov eax,0xEEFFAA01","cmp eax,eax","ret 4",
            province=sym SPY_PROVINCE,army=sym SPY_COUNTRY);
    }
    #[unsafe(naked)]
    unsafe extern "C" fn invoke_siege() -> u32 {
        std::arch::naked_asm!(
            "push esi","mov esi,0x77778888","push 0x55556666","call {hook}",
            "cmp esi,0x77778888","je 2f","xor eax,eax","2:","pop esi","ret",hook=sym siege_hook);
    }
    #[test]
    fn siege_wrapper_preserves_esi_province_stack_army_and_ret4() {
        let _lock = LOCK.lock().unwrap();
        let old_active = ACTIVE.swap(1, Ordering::AcqRel);
        let old = SIEGE.swap(siege_spy as *const () as usize, Ordering::AcqRel);
        let result = unsafe { invoke_siege() };
        assert_eq!(result, 0xEEFFAA01);
        assert_eq!(SPY_PROVINCE.load(Ordering::Relaxed), 0x77778888);
        assert_eq!(SPY_COUNTRY.load(Ordering::Relaxed), 0x55556666);
        assert_eq!(SPY_EDI.load(Ordering::Relaxed), 0x55556666);
        assert_eq!(SPY_ESI.load(Ordering::Relaxed), 0x77778888);
        SIEGE.store(old, Ordering::Release);
        ACTIVE.store(old_active, Ordering::Release);
    }
    #[unsafe(naked)]
    unsafe extern "C" fn fortress_spy() {
        std::arch::naked_asm!(
            "mov [{side}],ecx","mov eax,[esp+4]","mov [{army}],eax",
            "mov eax,0xBEEFCA01","cmp eax,eax","ret 4",
            side=sym SPY_PROVINCE,army=sym SPY_COUNTRY);
    }
    #[test]
    fn fortress_join_wrapper_preserves_ecx_side_stack_army_and_ret4() {
        let _lock = LOCK.lock().unwrap();
        let old_active = ACTIVE.swap(1, Ordering::AcqRel);
        let old = SIEGE_JOIN.swap(fortress_spy as *const () as usize, Ordering::AcqRel);
        let result: u32;
        unsafe {
            std::arch::asm!("push 0x55556666","mov ecx,0x77778888","call {hook}",
            hook=sym siege_join_hook,lateout("eax") result,clobber_abi("C"));
        }
        assert_eq!(result, 0xBEEFCA01);
        assert_eq!(SPY_PROVINCE.load(Ordering::Relaxed), 0x77778888);
        assert_eq!(SPY_COUNTRY.load(Ordering::Relaxed), 0x55556666);
        SIEGE_JOIN.store(old, Ordering::Release);
        ACTIVE.store(old_active, Ordering::Release);
    }
}
