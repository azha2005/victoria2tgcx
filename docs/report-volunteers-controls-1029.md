# Volunteers0.0.9: persistent entry, front visibility, exile permission and hot-path work

2026-10-29. User's0.0.8 screenshots/log establish whole-army sending and serving.
They report disappearing Send entry, fog obstructing control, organization falling
AFTER sending, no effective combat, and continued lag. No complete prototype,
measured performance fix or multiplayer acceptance is claimed.

## Evidence and corrected paths

- Preserved0.0.8 log: deployment serial1, reuse=true, army40/2183, five brigades;
  native Move to2463 queued. Journal commits reach head3 and then remain stable
  through283 executed ticks. Therefore the earlier date-only checkpoint issue
  stopped; it does not explain all remaining lag. User supplied three screenshots.
- UI code explicitly hid the independent diplomacy entry whenever panels opened.
  Entry now stays visible while Diplomacy is visible, retaining native thin160x24
  art, text and the user's action-grid slot. Host hide/teardown still synchronize.
- Shared country-vector helper RVA0xD61D0 ran the Rust scope/session work before
  testing its unique battle caller. New assembly filter checks the return address
  FIRST, preserves EAX/flags, then tail-calls the original for unrelated callers.
 100000-call x86 fixture verifies native result/original count, bypass count and
  zero Rust validation callbacks for unrelated calls. This is operation-count
  evidence, not a real-game FPS benchmark.
- Scalar Native reads formerly allocated a Vec and invoked ReadProcessMemory per
  word/byte. They now use bounded in-process reads inside the existing x86 SEH
  boundary. A real PAGE_NOACCESS fixture verifies fault containment and subsequent
  successful reads; production faults still disable plugin native work. Bulk byte
  reads remain bounded ReadProcessMemory, including the province pointer table.
- Visibility now inherits the entitled recipient's actual province+174 knowledge
  (length+178, values0..2), including foreign fronts. It also reveals receipt-validated
  volunteer current/adjacent provinces using map+C8/CC stride0x24, neighbor ID+4.
  The per-country array is READ ONLY. Only the current viewer's map+360 and original
  render dirty fields are written. Preserve native visibility by taking the maximum;
  recipient peace stops grants. Vision entitlement still survives troop return/loss.
  A bulk40000-slot fixture covers foreign province knowledge, edge visibility,
  volunteer-neighbor visibility, peace and unchanged simulated knowledge.

## Exile/supply/combat research (S unless explicitly R)

The0.0.8 process exited between inventory and the read-only probe (OpenProcess87).
Its live exile/organization/supply fields were NOT captured; no runtime exile flag
cause is proved. The user's organization fell after sending. The following is a
concrete native failure path consistent with that observation, now addressed:

| Function Ghidra VA | RVA | Established behavior / ABI |
|---|---|---|
|0x005C81E0|0x1C81E0| EDIarmy, RET; reads/writes exile byte+164; checks owner-based country/province permission |
|0x005DBD90|0x1DBD90| EAXlist header, RET; recomputes exile for its armies; current army held in ESI |
|0x0051B040|0x11B040| EAXprovince, stack country pointer, ALresult, RET4; read-only permission by owner/controller/relations |
|0x005CF880|0x1CF880| EAXarmy, CLvalue, RET; native exile setter; NEVER called by our DLL to force state |
|0x005D63E0|0x1D63E0| native supply calculation returns0 for exile+164 |
|0x005D7420|0x1D7420| native reinforcement calculation returns0 for exile+164 |
|0x005CBF60 /0x005DBB50|0x1CBF60 /0x1DBB50| initial/join combat gates reject exile+164 |

New permission hook filters exactly four established return addresses BEFORE
Rust/FXSAVE: RVAs1C8288/1C8314 (EDIarmy),1DBE62/1DBED2 (ESIarmy). It checks current
province, admitted SP epoch and exact registered receipt, then evaluates the
existing read-only land predicate with a private recipient input image. Native
engine code still decides/writes exile; no real owner, flag, organization, supply,
pop, relation or maintenance writes are introduced. Ordinary callers/armies retain
the original result. The complete hook set includes this gate before admission.
85 signature records pass, with relocation-aware new caller/prologue guards.

Raw .c/.asm.txt exports: work/volunteers-0009/research. Displacement inventories
field164.txt/field174.txt are search leads, not class-type proof; exile-refs.txt
contains inspected callers. Read-only headless project was used. Signature check
ran before address work and after regeneration. Native wrapper fixture verifies
EDI/ESI/EAX/stack country inputs, AL/EAX result, flags, callee registers and RET4.
The fixture initially used checked subtraction on a deliberately wrapping synthetic
base; corrected to wrapping RVA arithmetic. No fixture error occurred in the game.
Older exiled armies may still require native return to accessible territory; clean
checks should use a pre-send save. We do not force their flag or organization.

## Build, installation and actual runtime limit

39 Rust tests pass; strict all-target Clippy, format and release build pass. Five
package/actual-loader checks pass. Private17-file ZIP:
work/volunteers-0009/Volunteers-0.0.9-SP-controls-private.zip.
ZIP SHA256: 47afd87f51c14b0d924320527d26bfa99fb6004c5d50ac64805af428a225e26e.
DLL SHA256: e421cc435492cbbdf137acc60f7c7a6686f71dbe18214b5e2be853cc5f5e93a6.
Installed15 mod files in E:/Victoria-II-Volunteers-0004, preserving0.0.8 backup at
work/volunteers-0009/before-0009. Nine protected base/loader/TGC files unchanged.
Installer evidence: work/volunteers-0009/install-result.json and E:manifest.

Launched the existing prototype launcher, PID7692, loaded base0x00CF0000. Startup
log admits all six scoped hooks, GUI/load/journal guards; game is responsive by
process status. Computer-use skill was initialized. activate_window timed out;
one fresh-window capture retry failed with FrameArrived timed out. No clicks,
keys, typing, forced exit or campaign manipulation were performed. User then said
"ill try ity out myself"; UI work stopped and game left to them. No native Send,
combat, fog, organization recovery or timing check was completed by the agent.

New scope_counters log battle-target/bypassed calls, exile grants, vision updates
and thread refusals. These guide follow-up without weakening thread/session gates.
MP remains refused; no paired OOS reproduction. Follow instructions-volunteers-controls.md.
