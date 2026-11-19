//! PROVID: the `provid` console command's province tooltip line ("# 1234") also
//! names the owner's tag, and the controller's when another country or the rebels
//! hold the province: "# 1234 FRA", "# 1234 FRA (GER)". Display only.
//!
//! The tooltip FUN_004AAFD0 (thiscall, this = province+8) builds that line with
//! `call FUN_009888F0` at RVA 0xB1541 (ECX the id, one pushed out-string, RET 4),
//! with EDI still `this`. Our stub makes the same call, then reassigns the string
//! through the engine's own std::string::assign (FUN_00409350).

use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::log;
use crate::sys::{at, code_patch, guard, rd8, rd32, require_signatures};

pub const PROVID_RVAS: [u32; 3] = [0xB1536, 0x5888F0, 0x9350];
const CALL_SITE: u32 = 0xB1541;
const INT_TO_STRING: u32 = 0x5888F0;
const STRING_ASSIGN: u32 = 0x9350;

/// Offsets from the tooltip's `this` (province+8).
const OWNER_TAG: usize = 0x120;
const OWNER: usize = 0x124;
const CONTROLLER_TAG: usize = 0x128;
const CONTROLLER: usize = 0x12C;

static INT_TO_STRING_FN: AtomicUsize = AtomicUsize::new(0);

/// Replaces the native call: ECX id, [ESP+4] out string, EDI the tooltip's `this`.
#[unsafe(naked)]
unsafe extern "C" fn provid_thunk() {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp, esp",
        "push ebx",
        "push esi",
        "push edi",
        "push dword ptr [ebp+8]",
        "call dword ptr [{f}]", // the original, ECX the id: RET 4, EAX the out string
        "push edi",
        "push eax",
        "call {tag}",
        "add esp, 8",
        "mov eax, [ebp+8]",
        "pop edi",
        "pop esi",
        "pop ebx",
        "pop ebp",
        "ret 4",
        f = sym INT_TO_STRING_FN,
        tag = sym append_tags,
    )
}

/// A three-letter tag stored in place (NUL-padded char[4]).
fn tag(address: usize) -> Option<[u8; 3]> {
    let bytes = [rd8(address), rd8(address + 1), rd8(address + 2)];
    bytes.iter().all(u8::is_ascii_alphanumeric).then_some(bytes)
}

extern "C" fn append_tags(out: usize, this: usize) {
    guard(|| {
        if out == 0 || rd32(this + OWNER) == 0 {
            return; // sea and unowned land: the id alone
        }
        let Some(owner) = tag(this + OWNER_TAG) else {
            return;
        };
        let id = rd32(this + 0x50) as i32;
        let mut text = String::with_capacity(24);
        let _ = write!(text, "{id} {}", std::str::from_utf8(&owner).unwrap_or("?"));
        let controller = rd32(this + CONTROLLER);
        if controller != 0
            && controller != rd32(this + OWNER)
            && let Some(by) = tag(this + CONTROLLER_TAG)
        {
            let _ = write!(text, " ({})", std::str::from_utf8(&by).unwrap_or("?"));
        }
        type Assign = unsafe extern "thiscall" fn(usize, *const u8, usize) -> usize;
        // SAFETY: FUN_00409350, gated by its signature; `out` was just built by the original.
        let assign: Assign = unsafe { std::mem::transmute(at(STRING_ASSIGN)) };
        unsafe { assign(out, text.as_ptr(), text.len()) };
    });
}

pub fn install() {
    if !require_signatures(&PROVID_RVAS) {
        log!("PROVID: signatures BAD; province tooltip id line unchanged");
        return;
    }
    INT_TO_STRING_FN.store(at(INT_TO_STRING), Ordering::Relaxed);
    let ok = code_patch(CALL_SITE, provid_thunk as *const () as usize, 5, 0xE8);
    log!(
        "PROVID: province tooltip id line with owner/controller tags {}",
        if ok { "installed" } else { "FAILED" }
    );
}
