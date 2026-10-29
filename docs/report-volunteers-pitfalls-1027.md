# VOL-AUDIT: volunteer crash/lifecycle review against pitfalls.md

2026-10-27. User request: rule out potential crashes and other problems using
the project's pitfall manual. Read the manual and current volunteer sources,
cross-referenced existing PRU/ATCU code and native evidence, and fixed concrete
source-level hazards. **This is not proof that every crash has been ruled out.**
Tests remain deferred under the prior instruction; no game/desktop control,
installation, fixture execution or multiplayer session was performed.

## Concrete corrections in Rust0.0.4

| Finding | Correction / evidence |
|---|---|
| P59/P60: native root creation had only an existence check, without preflighting each button type | Resolve CWindowType through CEU3Gui+34, require table RVA A3B2CC/factory5D1040, then resolve every owned button including backgrounds through definition+150. Require the previously live-proved CButtonType A3CFBC/factory5F0310. Lookup table/methods guarded. Refuse before factory invocation; never accept the superseded CEU3ButtonType guess. |
| P59: no generated-name collision rejection | Packager rejects duplicate or unprefixed GUI definition names, including roots, backgrounds, buttons and text. Overlaid labels retain distinct names and click-through. |
| P67: small-text font was invented | Installed vanilla/TGC definition search did not find vic_14_black. Replaced it with actual vanilla core.gfx Arial14_black. All generated fonts are existing names in that inspected reference. Appearance still untested. |
| P68: rule text contained an unescaped semicolon | New validator caught the actual malformed value during staging. Reworded it and reject semicolons/CR/LF in localisation values. The incomplete first audit staging directory contains no ZIP and is not the candidate. |
| P47/P54: cached windows could be acted on after a vtable/root change | Track each created window's pointer, definition, GUI context and renderer. Recheck before GUI calls. Forget stale host objects without native teardown; release only verified owned roots through slot20. Keep observer allocations alive through deferred collection. |
| P47: observer capacity silently stopped binding | Reserve capacity before construction, log exhaustion and disable the UI rather than constructing more unbound windows. Process-lifetime retention remains bounded at256 observers. It deliberately does not reclaim an observer whose native lifetime is uncertain. |
| Native calls had bounded reads but no narrow native exception boundary | Adapt the existing Rust ATCU SEH call wrapper, restricted to Copy callbacks/results and one native call per scope. An access violation disables further wrapped calls; caller-owned Rust allocations/locks stay outside the unwind scope. Other exceptions continue searching. This does not repair a heap or make a wrong ABI safe. |
| GUI callbacks could run with the caller's dirty x87/SSE environment | Save caller state, establish FNINIT/default MXCSR for our callback, then restore it before returning or calling the original. Existing register/flags/last-error preservation retained. New thunks are compiled but not executed. |
| Host hide/destruction may arrive from another thread | Gate native GUI operations to the captured UI thread. Foreign-thread hide is deferred; retirement on another thread forgets pointers without native calls. Native-manager collection and cross-thread teardown still require runtime evidence. |
| P66: successful CAS plus failed protection restore could be reported as no hook | Shared patch helper records actual installation, restore and rollback outcomes; preserves the original protection value across retries and rolls back without replacing another owner. UI only becomes ready after all three hooks install. Pinned context/originals remain alive if an OS failure leaves a residual hook. |
| Foreign API table was dereferenced with an aligned Rust reference | Copy the advertised v1 API through bounded ReadProcessMemory, including size checks. Unreadable/truncated/unaligned storage is not directly dereferenced. The loader contract still supplies function pointers. |
| P64/P65: one backward date could suspend all missions | Ignore the inconsistent sample and retain last valid date/records. Only established lifecycle/campaign epochs authorize actual load handling. Main-menu/unknown sessions remain blocked. |
| A transient unreadable stack was labelled destroyed | Defer misses; three consecutive daily-coordinator misses require review, not Lost. A validated empty survivor receipt or explicit destruction remains separate. Review retains history/capacity and does not queue replacements. |
| Native ID could be consumed before Send was valid or wrap to0 | Validate actual request/affiliation/destination before allocation; check counter overflow and release the native lock on the ordinary refusal path. Simulation admission remains closed. |
| Move could replace an existing march despite the recall coordinator's idle policy | Native Move adapter independently rejects moving/combat/foreign-affiliation stacks. Outward/return work still requires observed idle state and native validity. |
| Province-vector read had only a hardcoded numeric maximum | Check actual world vector start/end/alignment and ID bounds before dereferencing the first-owned-land preview target. |
| String reads trusted capacity and omitted a terminator check | Require native SSO capacity15 or a bounded heap capacity and a trailing NUL within the bounded read. Unrecognized names fall back rather than becoming call arguments. |
| Failed temporary-file creation could remove someone else's existing temporary path | Only enter write/cleanup handling after create_new succeeds. Atomic replacement still preserves the last good checkpoint; no engine save is edited. |
| Failed checkpoint load could retain the old lifecycle binding | Invalidate binding/campaign and advance epoch before attempting the new read; require the actual campaign as well as epoch for save. Caller must suspend the old registry before load. Endpoints remain unwired. |
| A DLL could activate with an older package marker | Require exact Cargo version plus unique package/version marker keys. Both configurations continue to refuse gameplay; stale or malformed markers remain inactive. |

## Other pitfall coverage and remaining gates

| Pitfalls | Audit disposition |
|---|---|
| P1/P3/P17 | Own generated binary textures, explicit text encodings, x86 Rust/MSVC build. No base checkout conversion or upstream retargeting. |
| P2/P5/P7/P8/P24 | Reference v2game.exe SHA gate, RVA plus loaded base, relocation-aware expected bytes, concrete method/table guards. tools/check_exe.py ALL OK. |
| P9/P10/P19/P20/P48/P49/P50/P52 | Additive uniquely prefixed GUI/data; no copied core/base GUI, dependency/order assumption, loader, user_dir or replace_path. No installed game launch or base changes. Actual load order, V2DLL-generated defines and coexistence remain future checks. |
| P11/P25/P26/P28/P30/P32 | No custom final-Present painting, pie geometry or render hooks. Native independent windows/order only; no claim that source placement proves visible/clickable UI. |
| P12 | Unknown and MP simulation admission remains refused. No local sidecar/order-file transport. OOS screenshot still has no diagnosed cause or verified fix. |
| P13/P14/P15 | No supplied foreign C++ built; attributed observer pattern retained. No Ghidra execution this pass and no broad decompilation. Existing raw evidence reused. |
| P16/P18/P21/P22/P23/P33/P34/P38/P39/P46 | No law/event/decision script, TGC inventory comparison or native row-builder mutation in this change. Those mechanisms were not rewritten. |
| P27/P31 | Tooltip slot10 remains untouched; native signatures/RET contracts preserved. Guarding one native call is not an ABI test. New CPU/SEH/GUI wrappers still need exact-stack fixture execution. |
| P29 | No test runner executed. Do not infer a passed behavioral check from compilation/staging. |
| P35/P36/P37/P40/P53/P56/P57/P61 | Positive packed coordinates, owner-chain screen conversion, scene promotion on show, host-hide synchronization and secondary subtree activation retained. Entry offset is still provisional; no fixed offset can establish compatibility with every base. Unknown placement hides the plugin windows. |
| P41/P42/P44 | Plain spriteType only, direct owning-window lookups, subtree Show only on hidden-to-visible transitions. No visibility cache is trusted across a Show. Exact visuals/interaction remain unverified. |
| P43/P45/P51/P55/P58 | Donor tag/province distinction retained. Native allocations0x60/0x6C/0x90 have independently inspected push-size caller sites now guarded. Create+Transfer stays one batch; membership is verified, not assumed from queueing. Land flag nonzero preserved. Reachability/transport and battle-result semantics remain unestablished, so Send stays blocked. |
| P54/P59/P60/P62/P63 | Deferred release and definition preflight above; no native edit/caret feature. Native construction/collection must still be exercised later. |

The three newly documented pitfalls are P66 (partial hook bookkeeping),
P67 (missing font names) and P68 (localisation delimiter corruption).
AGENTS.md index is updated; next number is P69.

## Validation strength and candidate

- Reference hash/bytes: tools/check_exe.py ALL OK, including40 scoped volunteer
  guards:29 function/caller sites and11 tables. Added three allocation push/call
  sequences from existing raw disassembly, not inferred allocation sizes.
- cargo fmt applied; i686-pc-windows-msvc release compilation/link succeeds.
- Packaging rejects colliding names/unsafe CSV values and checks exact manifest,
  ZIP CRC and staged bytes. Successful staging is artifact integrity only.
- No cargo test, Python test suite, Clippy, native fixture, game, save/reload or
  multiplayer check. Earlier0.0.2 results do not validate0.0.4.

Final candidate:
`work/volunteers-0004/Volunteers-0.0.4-audit-untested-r2.zip`.
ZIP SHA-256 `4a644dcfb526741162d52a639a4973aa3b14ab9fd7e696f27cd6c0837fc73df2`.
DLL SHA-256 `9813de11a98c5518faaf392ec5f92a2ae7419432baedf8679061a048dc44d49e`.
15 files. Default **mode=observe**; explicit mode=preview opts into the untested
UI. Send/Recall remain unavailable. Earlier audit staging/ZIPs are interim;
old0.0.3/checked0.0.2 packages remain unchanged. Nothing installed.

## What cannot yet be ruled out

Native callback/stack correctness, manager collection, pointer reuse during
load, off-thread teardown, modified parsed GUI/foreign hooks, display order,
long-session turnover and OOM/heap fail-fast require actual fixtures and game
checks. The SEH guard intentionally does not catch every exception. A caught
access violation can leave an incomplete native allocation/queued operation;
calls are disabled and uncertain objects are not blindly freed or retried.
This may leave inert UI or allocations until process exit and requires restart
and logs, not a claim of recovery. Native engine originals remain responsible
for their own execution.

SP session/campaign/save signals, scoped combat/access/vision and MP transport
are still unestablished. Default observation and closed admission reduce the
exposed paths; they do not prove the future volunteer mechanic crash-free.
Continue via instructions-volunteers-audit.md and instructions-volunteers-native.md.
