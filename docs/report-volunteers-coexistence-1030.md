# Volunteers0.0.17: coexistence contracts

2026-10-30. User requires Volunteers to coexist with other Core plugins. This
candidate removes a concrete exact-loader layout restriction, preserves other
hooks, and improves refusal diagnostics. Actual paired plugin-combination and
original OOS acceptance remain open. No game or installed files changed.

## Evidence and scope

The new uploaded tgcx_volunteers (1).log is26255 bytes. It reports version0.0.16,
reference EXE, MP mode, and an E9 detour plus three NOPs at server Post RVA662EA0
(Ghidra VAA62EA0; runtime=base+RVA). gameplay=false,transport=false,scoped_hooks=false,
UI/lifecycle/journal=true. No MP HELLO or volunteer Send executes. Thus waiting
for peers is not an adequate explanation: local native admission already failed.

The file contains interleaved player189 and180/world/thread streams with broken
lines. They are not clean per-peer logs and cannot establish command agreement or
an OOS mechanism. The screenshot lists yearly cache, performance, Baltic personal
patches and Volunteers; footer6.36 is not proof of a specific loaded DLL hash.

The user initially suspected an outdated loader; the official release API checked
2026-10-30 still points to6.36 with archive SHA256
f3d81c5bb03081a8b75ca9e0ca9537a6850eeb512e695188db9bc78d26cfc263.
The exact owner of their rejected detour remains unestablished. No third-party
plugin is blamed or required to be removed as the permanent solution.

## Correction

0.0.16 recognized only one Core binary SHA256 and fixed private Core RVAs. That
was unnecessarily tied to one link layout even for identical transparent code.
0.0.17 replaces it with structural verification of the already inspected role-probe
contract from V2DLL V2TechButton.cpp. No new game offsets or guessed ABIs added.
Reference EXE/hash and all114 scoped native signature records remain unchanged.

The one accepted modified entry remains original five-byte E9 plus three NOPs,
with exact remaining original signature bytes. Direct E9 linker veneers are
traversed under visited-address and24-node bounds. At most four role probes can
be chained. Each must have exactly PUSHFD;PUSHAD;CALL marker;POPAD;POPFD;JMP [slot].
The marker is exactly MOV EAX,1;MOV ECX,&flag;XCHG [ECX],EAX;RET. Slot and aligned
writable flag must be within the probe's own mapped x86 image, outside the game
image. Code spans must be committed executable pages. Markers cannot mutate an
engine/global field or replace command behavior under this accepted contract.

The final continuation must contain the exact eight stolen original bytes and a
direct jump to original entry+8. Nonexecutables, missing reads, cycles, excessive
chains, altered state preservation, arbitrary handlers, marker writes outside the
owning image and wrong continuations refuse. ReadProcessMemory and VirtualQuery
inspect foreign pointers without poisoning our native-call AV fence. No foreign
code is executed for validation. No detour is removed, replaced or bypassed.

DLL filename/version/hash/private RVAs no longer grant or deny this contract.
Owner path/RVA/hash are diagnostic only. Other plugins' presence isn't a gate.
Unknown behavior at a required point still needs its own inspected compatible
contract. This is broader coexistence, not proof that arbitrary conflicting mods
can be combined or that incompatible gameplay changes are harmless.

UI gate order now reports native-hook compatibility failure before handshake
waiting. A recognized native configuration still needs complete peer receipt,
issuer/country/sequence/load/session/command admission. No MP safety check is
removed and no game-state, maintenance or CRC bypass introduced. Existing SP and
whole-army-only MP policy remains; no new ID allocation or replay behavior.

## Verification

-76 Rust tests pass,2 ignored benchmarks; format, strict all-target Clippy and
  x86 release pass. Five generic contract fixtures replace the prior exact-layout
  fixture: independent ASLR/link layouts/flag states; direct veneers/two role
  probes; mutated code/tail; wrong private-data and memory protections; loops/
  budget refusal. Gate regression checks true local failure versus peer waiting.
- An additional real Win32/x86 fixture uses compiled marker/thunk code inside the
  test executable's mapped PE image, private data and VirtualAlloc/VirtualProtect
  pages. Matcher accepts the relocated contract without calling its marker; then
  removing execute permission from the synthetic original trampoline refuses.
  This is a native-memory/ABI-layout fixture, not a game or real two-peer test.
- Six stage/x86 loader checks and two MP-package checks pass. Exact EXE gate/
 114 signature records pass. Final ZIP CRC/manifest and actual DLL bytes checked.
- No game launch, installation, desktop input or running-file replacement. E:
  accepted copy remains0.0.12; workspace game-host remains0.0.16. No test of the
  screenshot's full third-party plugin combination, gameplay or OOS completed.

## Artifact and next work

C:\Users\JC\Downloads\victoria2template\work\volunteers-mp-0017\final\Volunteers-0.0.17-MP-coexistence.zip
SHA256: 2010910f6ec5a6fde16bd59e4fa79c5a36f48445e5bc024362dbc029777b00a5
499732 bytes,19 files,mode=mp,8 native textures. No installer,EXE,BAT,CMD,PS1
or other scripts. DLL SHA256: 92bc4a34f307fb58252ab83530fdaad7e555ce1c968cbed5c8a3229d51074900
Old archives retained. Native art/raw RE/binaries stay private and outside git.
No public release, third-party message or memory writes. Commit/push authorized.
Use instructions-volunteers-coexistence.md with all intended plugins enabled on
both peers. Actual failed-hook ownership and paired gameplay remain next evidence.
