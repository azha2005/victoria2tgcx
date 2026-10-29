# VOL-0: independent volunteer submod foundation

2026-10-27. User selects HoI4-like volunteers, styled/packaged with PRU and ATCU.
Implementation started in a separate Rust crate and additive V2DLL package.
**Built, not tested in game. No troop sending or donor-control patch enabled.**

## Deliverables

- Source: rust/volunteers (model, bounded snapshot reader, runtime observer,
  exported V2Mod_Init and x86 unsupported-host DLL handshake harness).
- Reproducible checks/build: tools/build_volunteers.ps1; package generation
  tools/prepare_volunteers.py; tools/test_volunteers.py.
- Working specification docs/volunteers-design.md and native-style proposal
  docs/volunteers-layout.svg. Rendered/visually inspected PNG:
  work/volunteers-0001/volunteers-layout.png. The accepted Conform reference
  mockups/6-conform-current-1019.png was opened before designing the new panels.
- Checked candidate:
  work/volunteers-0001/Volunteers-technical-preview-0.0.1-checked.zip.
  Six files,129784 bytes.
  SHA-256:75c1373854b0ff2200fa58375d41b78b09a175514f1f494631814331b2ec046d.
- DLL SHA-256:
  0400d766869a43e61fced231a83dcad959ef1b8b4eaacbaa86004c92a07b26ea.

The earlier local stage without the -checked suffix is retained as an interim
artifact; use only the checked candidate for the next probe. Final public name
is not decided. No dependency on ATCU, PRU, Politics or TGC; no loader included,
no base GUI/data replacement, user_dir or replace_path.

## Implemented behavior

VOL-0 wraps only CHourlyTickCommand execute vtable slot6, VA0x00E1242C /
RVA0xA1242C, expecting original VA0x0072ECF0 / RVA0x32ECF0. It calls the original
with ECX/stack unchanged and observes after return. CPU wrapper preserves
flags, general registers, x87/SSE; observation preserves Windows last-error.
Full EXE SHA, relocated method signature, API base, original slot and FXSAVE
availability gate installation. Modified/foreign slots are refused, not stolen.
The DLL is pinned before installing its pointer; no inline instructions change.

Each sample logs tick count/thread/depth/date, world/player, country war count
and selected CArmy IDs, province, owner, affiliation_plus_CC and first supporting
pop owner. ReadProcessMemory handles stale/unreadable addresses without a raw
game-pointer dereference; traversals have cycle/count/type checks and limits.
The config accepts observe only; no switch enables an unfinished mutation path.

The separate pure model validates proposed donor/recipient/land/busy/limit rules
and transitions through request, posted, verified arrival, serving, pending
return, posted return and verified return. Duplicate events do not repeat
command intents. Lost objects and unverified reloads are not marked returned.
It does not submit game commands, persist actual loans or implement gameplay UI.

## Important correction to the prior research

The DayTick entry also has callers in Game_FinalizeLoadAndEnterGameplay at
VA0x00663234 and0x0066339D (reference export references.csv). Therefore the
entry detour proposed in the research is not by itself one gameplay-day hook.
The new probe observes the native tick command instead. Earlier "first five
instructions/bytes" wording means **five bytes across three instructions**.
No earlier report was rewritten.

The design does not yet prove a native ownership-command adapter, selective
control/list inclusion, foreign-pop exemption, stable loan persistence or return.
No direct owner/pop/list writes were added to manufacture a working example.

## Verification

- x86 Rust release builds; format and strict Clippy pass.
- 16 Rust tests pass: source/recipient/limits, duplicate command intents,
  receipt verification, peace/busy return, loss/reload, bounded foreign-pop
  snapshots, wrong selectable/cycle/read failures, SHA vectors, ASLR-aware
  signature/refusal and null/undersized/wrong-version API calls.
- Native x86 wrapper fixture verifies original called once with original ECX,
  unchanged EAX/ECX/EDX results, carry flag, SSE value and last-error after it.
- Five Python checks pass: package boundaries, hashes/ZIP payload equality,
  reproducibility, exact x86 exports, incorrect DLL refusal, and actual release
  DLL initialization on selected/unticked unsupported-host harness processes.
  This is an API-shape/DLL test, not the upstream loader or a game test.
- Reference check_exe: ALL OK; TGC252 scan: anchors and reserved range OK.
- Working mockup rendered with Inkscape and visually inspected. Inkscape's
  optional AppData preference/recent-file writes were denied; PNG export passed.
- git diff --check passes. Summary in work/volunteers-0001/validation.json.

No game launch/termination, install, release, upload, commit/push or subagents.
Existing user work and accepted packages/installations preserved.
No gameplay, MP, loader coexistence or complete feasibility claim.

## Next

VOL-0 user check: docs/instructions-volunteers-check.md. VOL-1 static command
and affiliation work can continue independently: docs/instructions-volunteers-commands.md.
The SVG is a working design target, not accepted gameplay or implemented controls.
No remote historical artifact was republished; new volunteer layout is local.
