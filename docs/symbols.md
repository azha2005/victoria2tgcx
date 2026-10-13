# Our own symbols in `v2game.exe`

> **Reference.** Everything this project names or discovers in the
> executable, with its evidence (rule V6). What is already in
> `refs\Victoria2-Reverse-Engineering\Symbols\*.csv` is not repeated here:
> it is imported into the Ghidra project (`ghidra/ImportV2Symbols.java`).
>
> `conf`: **C** read in the code (bytes or decompiled), **I** inferred,
> **R** confirmed at run time. Every address used by our own code also has
> its signature in `tools/check_exe.py`.

## Functions

| VA | name | conf | evidence | date |
|---|---|---|---|---|
| `0x008FD2A0` | `CUpperHouseTrigger::Evaluate` | C | slot 6 of vtable `0x00E32768` (`docs/rtti-classes.md`); bytes disassembled by hand, below | 2026-10-12 |
| `0x008FD780` | `CUpperHouseTrigger::ParseToken` | C | slot 4; token `0x4C6` → ideology at `+0x28` (`FUN_00973BD0` by name), token `0x344` → value at `+0x20/+0x24` | 2026-10-12 |
| `0x008FD300` | `CUpperHouseTrigger::Tooltip` | I | slot 10; shows `value × 0x320000 >> 15` (= ×100, a percentage) | 2026-10-12 |
| `0x005E64D0` | `CCountry::GetUpperHouse` | C | `push ebp; mov ebp,esp; push ecx; mov [ebp-4],ecx; mov eax,[ebp-4]; add eax,0x8B0; …; ret` | 2026-10-12 |
| `0x00685727` (CALL site, RVA `0x285727`) | call `FUN_009DF2B0` (VA `0x009DF2B0`, RVA `0x5DF2B0`) | C | V2DLL `V2TechButton.cpp`/`PATCHES.md` §5.9; `E8 84 9B 35 00`, in `check_exe.SIGNATURES`; DLL-0 retargets only this in-memory CALL | 2026-10-12 |

## Structures

| structure | offset | field | type | conf | evidence |
|---|---|---|---|---|---|
| `CCountry` | `+0x8B0` | `upperHouse` (embedded `CUpperHouse`) | object | R | `GetUpperHouse`; U-4 object `0x2962ECD0` = country `0x2962E420` + offset; runtime vtable RVA `0x9FD7DC` = preferred VA `0x00DFD7DC` |
| `CUpperHouse` | `+0x10` | pointer to `int64[]`, one entry per ideology | `int64*` | R (pointer/end/data, ten-entry vector) | `Evaluate`; U-4 array `0x2D409428`, matching chart |
| `CUpperHouse` | `[+0x10][i]` | ideology `i`'s share of the chamber, 2¹⁵ fixed point (1.0 = 32 768) | `int64` | R for observed entries | U-4 raw 27197/1638/3932 matches 83/5/12% with divisor 32768; sum 32767 |
| `CIdeology` | `+0x28` | the ideology's index | `int` | C/R | `Evaluate`: `mov ecx,[edi+0x28]` as the array index |
| (trigger scope) | `+0x0C` | the scope country's index | `int` | C | `Evaluate`: `mov eax,[eax+0xC]` → `countries[eax]` |

## How `CUpperHouseTrigger::Evaluate` was read

```
558BEC 83EC08        push ebp; mov ebp,esp; sub esp,8
8B4508               mov eax,[ebp+8]           ; scope
8B15 E4872501        mov edx,[0x012587E4]      ; g_countryManager
56 8BF1              push esi; mov esi,ecx     ; this = trigger
8B4808 8B400C        mov ecx,[eax+8]; mov eax,[eax+0xC]   ; country index
894DF8 8B4A04        mov [ebp-8],ecx; mov ecx,[edx+4]     ; CCountry*[]
8B0C81               mov ecx,[ecx+eax*4]       ; CCountry*
57 8B7E28            push edi; mov edi,[esi+0x28]         ; CIdeology*
E8 …                 call 0x005E64D0           ; eax = country + 0x8B0
8B4F28               mov ecx,[edi+0x28]        ; ideology index
8B4010               mov eax,[eax+0x10]        ; int64[]
8B54C804 3B5624      mov edx,[eax+ecx*8+4]; cmp edx,[esi+0x24]   ; high part
…                    (low part against [esi+0x20]) → true if share ≥ value
```

**GH-2 core runtime confirmation (2026-10-12, U-4):** player ENG, index 29,
human marker 1; executable loaded at `0x007D0000`. See
`docs/report-u-4-1012.md` and preserved log outside git at
`E:\v2re\logs\tgcx-u4-1012.log`.

| array index | ideology | confidence | evidence |
|---|---|---|---|
| 1 | `conservative` | R | raw 27197, 82.998657%, chart 83.0% |
| 2 | `reactionary` | R | raw 1638, 4.998779%, chart 5.0% |
| 6 | `liberal` | R | raw 3932, 11.999512%, chart 12.0% |

**Historical U-4 limitation (superseded by full live mapping below):**
U-4 did not identify allocation length or the other ideology mappings.
The nine sampled entries sum to 32767, consistent with per-entry rounding;
an exact sum of 32768 is not a necessary runtime invariant. File order is
**disproved** by the three observed indices; `candidate=` labels in DLL-0
are unconfirmed guesses, not a usable complete mapping. Zero entries do
not identify an ideology or rule out an additional/reserved slot.

The retained log has only one sample (January 2); repeated sampling while
paused or after reload is not established by that log. DLL-1 begins by
verifying frame-callback cadence. The user's stability report and game.log
confirm a successful January 6 reload, separately from sampler coverage.


## DLL-1 / HEMI-1 additions (2026-10-12)

**C, subsequently R (see runtime correction and U-5 below):** scoped Ghidra confirms the vector end, name lookup,
colours, Politics visibility and geometry. Exact evidence and VA↔RVA
conversions are in `report-dll-12-1012.md`; private pseudo-C remains
`E:\v2re\logs\dll12-*.txt`. The DLL resolves and checks live values;
the synthetic fixture map is not evidence of the game's full index order.

| object/site | field or RVA | meaning / evidence |
|---|---|---|
| `CUpperHouse` | `+0x14` | exclusive `int64*` vector end; FUN_009793C0 (RVA 0x5793C0); bounds before mapped reads |
| ideology-manager global | `0xE58A3C` | name-table pointer; parser operand RVA 0x4FD801 |
| name table | `+4/+8` | bucket count / bucket-array pointer; FUN_00973BD0 (RVA 0x573BD0), bounded linked `(object,next)` nodes |
| `CIdeology` | `+0xC/+0x1C/+0x20` | name string data/size/capacity; matching object vtable RVA 0xA36774 |
| `CIdeology` | `+0x34/+0x38/+0x3C` | normalized RGB floats; colour at +0x2C; FUN_00972260/FUN_00972B20/FUN_009B6EA0 |
| `CPoliticsView` | vtable RVA `0xA0E458` slots 1/2/4/10 | destructor/show/hide/tooltip; originals 0x2D2B00 / 0x2D2C10 / 0x2D2C40 / 0x2D2DD0 |
| `CPoliticsView` | `+0x4C` | native root object; show/hide calls verified in FUN_006D2C10/FUN_006D2C40 |
| native root | `+0x67` | visible byte; show/hide methods RVA 0x5E4450 / 0x5E4030 |
| native root | `+0x250/+0x90` | initialized absolute screen XY pair / renderer pointer; FUN_009E3BA0 (RVA 0x5E3BA0) |
| render device | `+0x6B9BC/+0x6B9C0` | screen width/height used by centred position setter |
| Direct3DCreate9 import | IAT RVA `0x88A39C` | hash-verified PE import; runtime original must equal D3D9 export |

P25: time-advance CALL RVA 0x285727 is conditional and is not used by
0.2.0. Present now supplies cadence, EndScene drawing. API COM slots:
factory CreateDevice 16; device Reset 16, Present 17, EndScene 42.
All new executable signatures are shared through check_exe/gen_signatures;
dynamic COM pointers use API-acquired originals, image-owner validation
and compare-and-swap instead of OS-version-specific byte assumptions.

## U-5 missing-chart follow-up (2026-10-12)

0.2.0 failed visual acceptance. `report-dll-12-fix-1012.md` records the
limits of that log. 0.2.1 was untested when the repair report was written. **C:** scoped
Ghidra shows FUN_0099A520 (RVA 0x59A520), ESI renderer, INC frame counter
+0x6BC8C (six overwritten bytes replicated), focus +0x6BCA0 and active
D3D9 device +0x64 used for Present. Continuation RVA 0x59A526. Primary
renderer vtable RVA 0xA38324 is confirmed by constructor FUN_009941E0
(RVA 0x5941E0). Both new sites have shared expected-byte guards.
The sampler/drawing entry is now final engine Present; COM callbacks
only forward/reset, and drawing uses a separate short scene. Native GUI
factory confirms the existing root field/method checks; live validation
still awaits U-5. The fixture tests an inherited smaller viewport and
restoration; it does not establish the failing game's actual viewport.

## Runtime correction and full mapping (2026-10-12)

**R:** `report-dll-12-live-fix-1012.md` supplies paused 0.2.1 log and live
read-only process evidence for all nine names/indices/RGB values and the
ten-entry upper-house allocation, null slot zero, sum 32767. CIdeology
indices are 1 conservative, 2 reactionary, 3 tribalist, 4 fascist,
5 anarcho_liberal, 6 liberal, 7 anarchist, 8 communist, 9 socialist.
Active renderer primary table is CEU3Graphics RVA **0xA19B8C**, confirmed
by live pointer 0x011E9B8C at executable base 0x007D0000 and existing RTTI.
A new signature covers it. The base renderer table is not the only family.
**Correction to the earlier root-position inference:** active root +0x250
already holds absolute screen coordinates (768,470) at 2560 x 1440. It
must not receive another half-screen offset. Root table/methods, visibility,
resolution and geometry match the live window. This initialized-window
measurement does not establish every future resize/reposition behavior.
Repeated unchanged-date samples confirm the final native presentation
cadence. 0.2.2 corrects those identities/coordinates; drawing acceptance
remains pending. Existing reports were preserved rather than rewritten.

## Tooltip return correction (2026-10-12)

**C / crash evidence:** engine tooltip result is a structure containing
multiple strings, not a single 24-byte string. FUN_004115D0 (RVA 0x115D0)
copies fields at 0/0x1C/0x38 and +0x54/+0x58. 0.2.2 crashed after the first
chart draw in this copy path; see `report-dll-12-tooltip-fix-1012.md` for
dump addresses and private evidence. The earlier synthetic tooltip ABI
test did not establish this layout. 0.2.3 removes all tooltip-return writes
and leaves CPoliticsView slot 10 unchanged. Do not restore suppression
without verifying the full return structure, constructors and ownership.


## Ordered GUI layering (2026-10-12)

0.2.3 was confirmed drawing and stable by the user's screenshot/description
and repeated live log frames. Its final-Present placement painted over
native tooltips. 0.2.4 moves drawing to the existing GUI scene; final
Present supplies only sampling/logging. User confirmed 0.2.4 works;
U-5 functional evidence below. Visual polish/extended coverage remain.

| site/field | evidence | confidence |
|---|---|---|
| FUN_00999D00, RVA `0x599D00` | ordered render list `renderer+0xF8/+0xFC`; virtual Draw slot `+0x28` | C |
| post-Draw RVA `0x599F22` (VA `0x00999F22`) | entity ESI, renderer EBX; seven-byte `CMP byte [ESI+0xF0],0`; original CMP replicated, resume RVA `0x599F29` | C |
| root vtable slot `+0x44`, RVA `0xA3C11C` | FUN_009E4D40 (`0x5E4D40`) calls name lookup FUN_009E6B30 (`0x5E6B30`) | C/R |
| root `+0x2AC/+0x2B0`, `+0x2BC/+0x2C0` | parallel icon-pointer and 28-byte name-record begin/end vectors; fourteen entries in each live vector | C/R |
| name record `+0x10/+0x14` | size/capacity; inline <=15, otherwise record+0 points to string | C/R |
| native icon vtable RVA `0xA3A930`, icon `+0x94` | chart initialization FUN_006D79F0, root name lookup, live icon/entity cross-check | C/R |
| overlay entity vtable RVA `0xA3FF08`, Draw slot RVA `0xA3FF30` | native function FUN_00A2F120 (`0x62F120`), live `chart_upper_house_overlay` entity pointer | C/R |

Private bounded exports: `E:\v2re\logs\dll12-layer-render.txt`,
`dll12-layer-loop-asm.txt`, `dll12-layer-chart.txt`, and live read-only
`dll12-layer-live-widgets.json`. The build adds four guards (23 total).
The common callback forwards every original virtual draw; it draws only
after the exact named overlay, without replacing its ABI or tooltip
objects. No engine-generated data enters git. No simulation writes.


## U-5 functional confirmation (2026-10-12)

`report-u-5-1012.md`: user confirms 0.2.4 works but looks clunky. Runtime
log identifies the installed version, 23 passing guards, first ordered
hemicycle draw at root (768,470) with ten entries, 2687 repeated draws,
three paused samples and native show/hide. Thus the ordered native render
callback is exercised in the game, not only in fixtures. Active error logs
are empty. Prototype is game-tested; appearance refinement and unconfirmed
view-switch/save-reload/Alt-Tab coverage remain explicit follow-up work.
No new addresses or source changes in this status update.

## HEMI-3 scalar hover path (2026-10-12)

**C (bounded Ghidra; later runtime evidence below):** native tooltip objects
need no replacement. Politics calls a scalar pie-selection helper and then
constructs its own complete result, including the normal empty result.
Private evidence: `E:\v2re\logs\hemi3-{tooltip,hit,route-asm,pie-bounds-asm,hit-tail}-1012.txt`.

| site/field | evidence / calling convention |
|---|---|
| VA `0x006D52C6`, RVA `0x2D52C6` | CALL within FUN_006D2DD0 to FUN_009E0C90; entity in EAX; four stack args: screen X/Y integers, slice-output pointer, weight-output pointer; AL boolean, RET 16 |
| VA `0x009E0C90`, RVA `0x5E0C90` | computes circle/angle; slice records at entity `+0x158/+0x15C`, stride 36; writes record `+0x20` slice ordinal and `+0` integer weight only on hit |
| VA `0x00E3BDA8`, RVA `0xA3BDA8` | pie entity vtable `0xE3BD58` (RVA `0xA3BD58`), slot +0x50; original FUN_009E0B70 |
| VA `0x009E0B70`, RVA `0x5E0B70` | ECX entity; first two stack args are screen-space floats (CVTTSS2SI); returns FLD1/FLDZ in ST0; **RET 32**, six further stack words unused by this implementation |
| ideology manager `+0xC/+0x10` | engine-indexed pointer-vector begin/end used by Politics; selected slice is the ordinal among **positive-share** entries in increasing engine index, not the engine index itself |

FUN_006D2DD0 at VA `0x006D52D5` (RVA `0x2D52D5`) obtains the ideology
vector; `0x006D5340..0x006D55F8` (RVAs `0x2D5340..0x2D55F8`) skips zero
shares, compares the compressed ordinal, and builds native ideology and
percentage text. Native chart entity/table identity also appears in the
prior live `dll12-layer-live-order.json` (base `0x007D0000`, entity table
`0x0120BD58`, RVA `0xA3BD58`). Root absolute screen coordinates and rendered
geometry remain the U-5 convention. Four new expected-byte signatures guard
the CALL, hit-test slot and original function entries. Slot 10 remains intact.

## HEMI-3 runtime hover confirmation (2026-10-12)

User confirms 0.2.6 works; accepted log preserved in
`E:\v2re\logs\tgcx-before-027-1012\tgcx.log`. All 27 guards pass,
5555 ordered draws, native scalar hover exercised for Conservative
index 1/slice 0, Liberal index 6/slice 2, Reactionary index 2/slice 1.
Thus these remapped scalar dispatch paths are exercised in game (R),
in addition to the prior disassembly/fixture evidence. Both active error
logs are empty. Extended U-5 interactions remain unconfirmed. Separate
0.2.7 header-bound clipping adds no addresses. See
`report-hemi-3-header-1012.md`.

## LAW-1 read-only support research (2026-10-12)

Reference SHA/check_exe passed before inspection. These are research
sites, not new DLL hooks; six separate `RESEARCH_SIGNATURES` gate their
file bytes without expanding the installed DLL's 27 runtime guards.
Preferred base `0x400000`; RVA = VA - base. Static body behavior is C;
end-to-end native vote parity and parser mapping remain unconfirmed.

| Ghidra function | VA | RVA | Observed behavior |
|---|---|---|---|
| `FUN_008AF810` | `0x008AF810` | `0x4AF810` | `CAdditiveForUpperHouseMTTH` evaluator via known vtable slot 6: initial value plus matching direct modifiers and matching ordered prefix of each group |
| `FUN_008AEC00` | `0x008AEC00` | `0x4AEC00` | general additive evaluator comparison; final scaling differs from upper-house evaluator |
| `FUN_008AF210` | `0x008AF210` | `0x4AF210` | additive parser body; full base/factor field mapping still to verify |
| `FUN_00972B20` | `0x00972B20` | `0x572B20` | CIdeology parser dispatch to embedded rule objects; token-to-rule mapping still to verify |
| `FUN_008FE3C0` | `0x008FE3C0` | `0x4FE3C0` | scoped-country social movement comparison reads `CCountry+0x15A8` |
| `FUN_008FE6E0` | `0x008FE6E0` | `0x4FE6E0` | scoped-country political movement comparison reads `CCountry+0x15A4` |

Private decompilation/ref outputs: `E:\v2re\logs\law1-support-*-1012.txt`.
No derived executable source enters git. Full findings, approved design
and remaining limits: `report-law-1-start-1012.md` sections 2/4/5.

### LAW-1 completed static support trace (2026-10-12)

This supersedes the earlier parser-mapping uncertainty. Runtime numerical
comparison still requires U-6. `report-law-1-native-1012.md` contains the
full evidence/quantization distinction. File gates: 18 read-only research
sites, separate from the retained 27 DLL runtime sites.

| Ghidra function/site | VA | RVA | Confirmed static behavior |
|---|---|---|---|
| `FUN_00972260` | `0x00972260` | `0x572260` | ideology constructor embeds additive support objects |
| `FUN_00972B20` | `0x00972B20` | `0x572B20` | tokens `0x1F3/1F4/1F5/1F6` dispatch add/remove political/social to `+0x50/+0x80/+0xB0/+0xE0` |
| token registration | `0x00C3D41C` | `0x83D41C` | add-political literal registration corroborates the token map |
| `FUN_008AE290` | `0x008AE290` | `0x4AE290` | modifier factor token `0x3BE`, numeric reader |
| `FUN_009A1CE0` | `0x009A1CE0` | `0x5A1CE0` | numeric field reader used by support base/factor parser |
| `FUN_009A0F40` | `0x009A0F40` | `0x5A0F40` | unexpected-token fallback, not modifier arithmetic |
| `FUN_0098FC80` | `0x0098FC80` | `0x58FC80` | signed decimal reader retains three fractional digits as thousandths |
| `FUN_00525A20` | `0x00525A20` | `0x125A20` | social eligibility selects add/remove object by current/target order; country scope; Q15 weighted aggregation |
| `FUN_00525BE0` | `0x00525BE0` | `0x125BE0` | political counterpart with the same arithmetic |
| `FUN_00526A30` | `0x00526A30` | `0x126A30` | social reform support tooltip path |
| `FUN_005273A0` | `0x005273A0` | `0x1273A0` | political counterpart |
| `FUN_008E8AB0` | `0x008E8AB0` | `0x4E8AB0` | `militancy` in country scope uses the country-average helper |
| `FUN_008FB3E0` | `0x008FB3E0` | `0x4FB3E0` | `average_militancy` uses the same country-average helper |

The previously gated `FUN_008AF210` writes base token `0x56A` to `+0x1C`;
`FUN_008AF810` starts from that field and adds modifiers/prefix groups.
Neither that evaluator nor the traced political/social aggregation clamps
support. Installed TGC's supported rule outcomes are nonnegative `0..1000`.
Native weighting: `q = floor(s * 32768 / 1000)`, then
`floor(h * q / 32768)` per ideology. Script packet rounding is separate.
Private evidence: `E:\v2re\logs\law1-support-*-1012.txt`.

## LAW-2 native Bill GUI/decision path (2026-10-13)

Reference SHA and expected bytes passed before inspection. All addresses
below use preferred base 0x400000, with RVA = VA - base. Body/layout/ABI
evidence is **C (static)**; actual Bill GUI/command execution is not yet R.
0.3.0 has 38 relocation-aware runtime guards and 32 separate research
gates (70 file checks). Existing hemicycle addresses remain unchanged.

| function/site | VA | RVA | Confirmed static behavior |
|---|---|---|---|
| `FUN_006DCE10` | `0x006DCE10` | `0x2DCE10` | native Politics decision click; input element string +0x14; removes 21-character POLITICSVIEW_DECISION prefix, builds player/date scope, dispatches decision; RET 4 |
| `FUN_00874F80` | `0x00874F80` | `0x474F80` | decision command dispatch, calls command construction/queue path |
| `FUN_00905B30` | `0x00905B30` | `0x505B30` | CExecuteDecisionCommand construction observed in that path |
| `FUN_009E4C80` | `0x009E4C80` | `0x5E4C80` | native named button lookup, root vtable slot 13, native std::string input |
| `FUN_009E4D00` | `0x009E4D00` | `0x5E4D00` | native named text lookup, root slot 15, vectors +0x2CC/+0x2DC |
| `FUN_009E4A20` | `0x009E4A20` | `0x5E4A20` | named child window lookup, slot 27, map +0x424; missing window prints a warning |
| `FUN_00641BA0` | `0x00641BA0` | `0x241BA0` | CVariables Find wrapper, native string input; converts inline/heap data, dispatches inner lookup; RET 4 |
| CVariables primary table | `0x00DFBB60` | `0x9FBB60` | guarded primary table at country +0x1DC, slot 1 points to Find wrapper |
| `FUN_00408ED0` | `0x00408ED0` | `0x8ED0` | native std::string assign; textbox stored string +0xDC |
| `FUN_00A5ECC0` | `0x00A5ECC0` | `0x65ECC0` | native graphical text setter; receiver ESI from textbox +0xA0; string pointer and flag on stack, RET 8 |
| `FUN_009E3BA0` | `0x009E3BA0` | `0x5E3BA0` | centered-position setter: packed signed x/y at +0x250, renderer dimensions +0x6B9BC/+0x6B9C0; updates inner +0x494 |
| `FUN_009E4450` / `FUN_009E4030` | `0x009E4450` / `0x009E4030` | `0x5E4450` / `0x5E4030` | show/hide flag +0x67 and inner visibility +0x124; slots 8/9 |
| Politics observer table | `0x00E0E48C` | `0xA0E48C` | construction region FUN_006D26D0 confirms 44-byte instances; owner +4, click callback +8, deletion callback +0x20 |
| `FUN_00737D00` | `0x00737D00` | `0x337D00` | observer click slot 2, tail-calls callback +8 |
| `FUN_006DFFA0` | `0x006DFFA0` | `0x2DFFA0` | observer deletion notification slot 1, callback +0x20; nil is safe |

Native borrowed std::string inputs are 24 bytes: inline 16-byte storage or
pointer, length +0x10, capacity +0x14; capacity below 16 selects inline.
Country variable values remain signed thousandths at returned node +0x1C,
corroborated by `FUN_008DA5C0` (VA 0x008DA5C0, RVA 0x4DA5C0).
Only native GUI fields are written by the DLL. Simulation writes use the
native decision handler. Native callback/string patterns taken from
V2DLL are marked in source.

**ABI exclusion:** InGameIdler observer table VA 0x00E0A58C (RVA 0xA0A58C)
uses slot 1 `FUN_00641C40` (VA 0x00641C40, RVA 0x241C40), which reads
callback +0x38/owner +0x3C. Its slot 2 `FUN_009CD610` (RVA 0x5CD610) uses
a different template layout. Neither is used for Bill binding. Do not
substitute it for the independently verified 44-byte Politics observer.

Private evidence: `tgcx-law2-decision-click-1013.txt`,
`tgcx-law2-gui-command-1013.txt`, `tgcx-law2-root-lookups-1013.txt`,
`tgcx-law2-position-visibility-1013.txt`, `tgcx-law2-abi-1013.txt`,
`tgcx-law2-message-observer-1013.txt`, `tgcx-law2-politics-glue-1013.txt`,
`tgcx-law2-politics-observer-abi-1013.txt` and
`tgcx-law2-politics-observer-fields-1013.txt`, all under E:\v2re\logs.
No decompiled bodies or binary material are retained in git.
