# VOL-NATIVE: Rust 0.0.3 implementation, tests deferred

2026-10-27. The user requested as much coding as possible before tests, keeping
donor ownership, sending-player control and our existing volunteer layout.
The supplied multiplayer screenshot shows an actual "Game is no longer in
synch!" dialog. It does not identify the mismatching state, instruction or peer.

**Result:** the expanded independent x86 plugin compiles. Native UI preview,
command wrappers, registry, recall coordination and checkpoint endpoints are
coded. Gameplay remains blocked by missing native prerequisites. This is an
untested development candidate, not a working volunteer release or OOS fix.

## Implemented source

| Source under rust/volunteers/src | Result |
|---|---|
| app.rs | Two-row brigade chooser, army/recipient paging, explicit selected brigade IDs, mission paging, Send/Recall handlers and presentation. No all-armies auto-send. One daily coordination pass, receipt/loss inspection and peace/donor-war return requests. Recall waits for an idle stack through the daily adapter. |
| native_gui.rs | Optional diplomacy entry and independent 332x378 chooser/332x266 forces windows. Existing ATCU factory, name lookup, observer, text, owner-chain positioning, foreground promotion and host-hide synchronization contracts reused. Deferred release; observer storage retained and bounded. |
| native.rs | Bounded native reads and reference guards; donor-owned Create plus selected Transfers posted together; native Move wrapper for outward/return travel; pre-post native validity and disposal of unposted recognized commands. Native ID allocation reuses ATCU's engine-lock/counter pattern. |
| registry.rs | Bounded mission registry, authenticated donor/sequence checks, duplicate/membership rejection, command-post versus observed-receipt distinction, survivor subsets, suspension and serialized checkpoint consistency checks. |
| mission.rs | Existing donor ownership/accounting/control contracts retained; original versus surviving IDs, return receipt, suspended load restoration and separate terminal-record vision restoration added. No replacements created for losses. |
| session.rs | Explicit Unknown/SP/MP states, campaign epoch and mandatory capability gates. Unknown and MP always reject simulation work. Settings cannot override admission. |
| protocol.rs | Deterministic bounded Deploy/Recall envelopes, version, campaign/mission identity, donor/recipient, issuer sequence and exact regiment IDs. Rejects malformed, duplicate and trailing data. This is not an engine-registered command. |
| replication.rs | EngineTransport interface, explicit unregistered transport refusal and protocol/executable/plugin/settings peer fingerprints. No guessed native command ID or network shim. |
| combat.rs | Pure scoped recipient hostility policy, requiring unrelated eligibility gates to have passed. Same-owner/identity exclusion, native fallback and no global diplomacy change. No combat hook installed. |
| persistence.rs / lifecycle.rs | Save-content SHA-256 plus campaign binding, bounded checksummed checkpoint, atomic replacement and explicit save-complete/load-complete endpoints. Loaded records suspend; pending commands are not replayed. Native lifecycle signals are not wired. |
| runtime.rs | Existing guarded tick observer retained; optional GUI admission and day-coordinator call after original tick execution. |

All new runtime code is Rust. Existing ATCU SHA-256/native ABI contracts are
reused. Attributed V2DLL observer patterns retain source comments. The supplied
Maxioten C++ is not built or installed. No SetOwner, country war-state write,
pop rewrite, maintenance skip, teleport or local-file order transport is added.

## What is active and what is blocked

The package defaults to `mode=preview`: the observer plus an attempt to install
three guarded diplomacy UI slots. These slots wrap update, hide and destruction.
`mode=observe` omits the new UI hooks. Native UI allocation changes UI state;
the preview is not an in-game validated window implementation.

Native world capture intentionally supplies `SessionMode::Unknown`, no campaign
binding and epoch zero. Capabilities default to false. Therefore the runtime
cannot allocate a gameplay identity, post a troop command, grant special combat
or vision, or write a mission checkpoint through an ordinary preview click.
The native wrappers exist but no admitted gameplay path reaches them yet.

These prerequisites remain **not established**:

1. Native, reliable single-player/multiplayer detection and sender authority.
2. Campaign identity association and exact engine save/load-complete signals.
3. Both repeated combat hostility boundaries, side assignment, battle-result
   attribution and all unrelated eligibility checks for donor-owned stacks.
4. Per-stack access/path behavior, including transport and return routes.
5. Recipient-owned province vision integration without changing other viewers.
6. Registered engine transport and replicated registry for multiplayer.

All of 1-5 are required before SP Send admission. Item 6 is a separate MP card;
SP integration does not enable MP. A checkbox or count of one player is not
evidence of SP. A local sidecar cannot synchronize a mission registry.

The screenshot alone cannot prove that Maxioten's direct ownership writes,
local registry or maintenance hooks caused this OOS. Those are plausible paths
from the supplied source, not a verified diagnosis. Need both peers' exact DLL,
config, executable, command history and divergence logs to identify the cause.
Preserved screenshot: work/volunteers-0003/maxioten-oos-reference.png.

## Native evidence and UI differences

`tools/gen_volunteers_natives.py` generates 28 scoped, relocation-aware guards
from the exact reference executable: 21 function prefixes and seven tables.
Both runtime and tools/check_exe.py use them. Runtime compares loaded base plus
RVA; Ghidra VAs and calling conventions are listed in docs/symbols.md.
Four targeted functions were exported read-only from the copied Ghidra project
to check UI hook roles/ABI: VA0x007539D0, 0x0061BE80, 0x0061BF70 and 0x006D2C10.
Raw .c/.asm.txt pairs are in work/volunteers-0003/gui-hook-audit. Static review
corrected the interim slot assumptions: diplomacy slot2 is Show, slot0 is a
boolean method; Hide is slot4/RVA0x3539D0 and the deleting destructor is
slot1/RVA0x21BE80, RET4. Final hooks use slots11/4/1. No fixture/game proof is implied.

The green/gold/parchment chooser and forces pane retain our dimensions/layout.
`tools/tgcx_volunteers_gui.py` generates six original procedural textures plus
additive .gui/.gfx/localisation. No base diplomacy GUI is copied or overridden.
The SVG now labels the compiled, untested preview and shows paging arrows.
It was exported to work/volunteers-0003/volunteers-layout.png and visually
inspected. Optional Inkscape AppData log/recent-file writes were denied;
the PNG export completed. This is a mockup check, not a game/UI test.

Differences from the mockup: recipient selection uses arrows rather than a
dropdown; two brigade rows and two mission records use paging; regiment names
are read but type/strength artwork is not added; mission text uses native text
layout rather than painted cards. Diplomacy entry placement uses a clamped,
provisional offset and needs the eventual visual check. No product acceptance
is claimed for those differences. Button handlers refuse unavailable actions;
native disabled-state styling remains open.

The daily coordinator preserves native busy movement/combat, checks exact
membership before granting serving context, and observes donor territory before
marking return. A posted Move is not proof of arrival. Native path rejection,
interruption and long-running pending work still need behavioral coverage and
timeout/reconciliation policy before gameplay is enabled. Checkpoints provide
atomic disk replacement, not an atomic transaction with the engine save.

## Compilation and staged candidate

- Reference gate: tools/check_exe.py reports ALL OK, including 28 new scoped guards.
- cargo check --offline succeeds; cargo fmt applied; final cargo build --release
  --offline succeeds for i686-pc-windows-msvc.
- **No Rust/Python tests, Clippy run, fixture execution, game launch, desktop
  input, install or multiplayer session during this coding pass.** Prior 0.0.2
  test counts do not apply to these new changes. Package tests were updated
  for the generated assets but deliberately not executed.
- Packager checks ZIP CRC and exact staged payload bytes. This is archive
  integrity only, not native behavior or runtime validation.

Final candidate:
`work/volunteers-0003/Volunteers-0.0.3-native-preview-untested-r2.zip`.
ZIP SHA-256 `a9d7c156334136d30fe5433cb4156422b1f75a8bdb087b18b4262fff31517d30`.
DLL SHA-256 `53923bc969ce23e65bdebf4164e3aee0b2920ea6472212f6d439a1de2854e303`.
Fifteen files, including manifest; no loader supplied. Earlier 0.0.3 development
and native-preview stages are interim and predate the final UI hook correction.
Use only the native-preview-untested-r2 candidate for later testing.
The checked 0.0.1/0.0.2 packages and installed disposable copy remain unchanged.

Next: docs/instructions-volunteers-native.md. Testing stays deferred under the
latest user instruction; continue native evidence/integration first.
