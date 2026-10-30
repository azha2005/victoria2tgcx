# Known pitfalls (full text)

> The index, one line per pitfall, is in `AGENTS.md` §8. Here is the full
> text: cause, symptom, fix and how to check it. **Read the ones for the
> area you are touching before changing anything.** A new pitfall goes here
> and in the index, with the next number; never renumber.

---

### P1 — `git clone` with `core.autocrlf=true` converts TGC to CRLF

**Found:** 2026-10-12, when cloning v2.5.2.

**Symptom:** every `.gui` in v2.5.2 "differs" from v2.3.4's, but the diff
marks all 2 649 lines of `country_politics.gui`: it is only the line
ending. Worse: git decides what is binary by content, and a `.tga` or
`.bmp` with no null bytes at the start can be taken for text and get
corrupted with added `\r`.

**Cause:** the user's git has `core.autocrlf=true` globally.

**Fix:** in every TGC clone,

```bash
git config core.autocrlf false
git rm --cached -r -q . && git reset -q --hard
```

or clone directly with `git -c core.autocrlf=false clone …`. Check with
`git ls-files --eol` (the `w/` column must match `i/`). `tgc_scan.py` also
ignores `\r` when hashing the interface files.

---

### P2 — `TGC.mod` does not state the real version

**Found:** 2026-10-12. Tag `v2.5.2` ships `current_release = "v2.5.1"` in
`TGC.mod`.

**Fix:** take the version from git (`git describe --tags`) or, in a folder
without git, from the hash of the files the submod uses (`tgc_scan.py`).
Never from the `.mod`.

---

### P3 — The Windows console is not UTF-8

**Symptom:** a tool prints `sincron�a` instead of `sincronía` from bash or
PowerShell (seen with `check_exe.py` when its messages were in Spanish).

**Fix:** every tool in `tools/` starts `main()` with
`sys.stdout.reconfigure(encoding="utf-8")`. Since D10 the messages are in
English, but TGC names and paths can still carry non-ASCII characters.

---

### P4 — `HPM4.5.mod` points to a folder that does not exist

`path = "mod/HPM4.5"` but the folder is `mod\HPM` (on `D:\` and on `E:\`).
If HPM does not load, this is why. Not fixed (R1): HPM is no longer the
base (D2).

---

### P5 — `victoria2.exe` is not the game

`victoria2.exe` (26 KB) is the launcher. The game, what is analysed and
what loads the DLL, is `v2game.exe` (12 MB).

---

### P6 — The user's `mod\TGC` has junk in its root

The installed v2.3.4 has the save analyser's scripts in the root of
`mod\TGC` (`Artisans.py`, `Country.py`, … `startscript.bat`, `outputs\`),
besides its copy in `Save_Game_Analyzer\`. It is harmless to the game, but
that folder is not a clean reference for v2.3.4: to compare versions, a
clone of the tag is better.

---

### P7 — ASLR: the executable does not load at `0x400000`

`v2game.exe` has a relocation table and `DYNAMIC_BASE`. Ghidra shows VAs
with base `0x400000`; at run time it was seen at `0x00080000`. In the DLL:
`g_base = GetModuleHandle(NULL)` and `address = g_base + RVA`, with
`RVA = VA - 0x400000`.

---

### P8 — Addresses from V2DLL and from the RE repo may come from different builds

The RE repo's `PATCHES.md` warns about it: "These projects may target
different Victoria II executable builds". On 2026-10-12 five signatures
were checked against the user's `v2game.exe` and they match
(`tools/check_exe.py`), but that does not guarantee every address. **Every
new address is added to `check_exe.py` with its expected bytes before it is
used.**

---

### P9 — TGC's official submods override whole files

`TGCDirectImmigration` ships its own `common/buildings.txt`,
`common/pop_types.txt` and `interface/province_interface.gui`, copied from
one TGC version and edited. With another TGC version, those files hide the
new ones: the submod "works" but undoes TGC's changes in those files,
without any error. It is exactly what R3 forbids. Fix:
`docs/tgc-compatibility.md` §3.

---

### P10 — Load order and `dependencies`: unverified

TGC's submods declare `dependencies = { "TGC - The Grand Combination" }`.
It is not verified in this game (3.04, the launcher in `launcher\`) whether
the launcher uses that for the load order, nor what happens if the
installed TGC has a different name (like `TGC v2.5.2 (stable)`, created on
2026-10-12). Until SUB-0 verifies it (U-2), do not assume the submod loads
afterwards.

---

### P11 — The Vic2 GUI is static

A `.gui` defines named elements; the engine fills them according to their
name (`chart_upper_house`, `upper_house`…). There is no way, through
script, to show an image according to a variable or a flag. Consequence: a
ruler's portrait or a hemicycle **need the DLL**. Through script there are
only alternatives: an event's picture, a national modifier's icon
(`icon = N`), or a decision's text.

---

### P12 — Changing the simulation from the DLL desyncs multiplayer

In MP each client simulates on its own and compares a checksum per day
(`FUN_00682EC0`). If one client's DLL writes a field that does not change
on the other, the game is cut with "Games out of synch". Also, every player
needs the same DLL and the same configuration (V2DLL says so for its own
patches). Hence R5.

---

### P13 — V2DLL has no published licence

The repo `artyom-kuznetsov/V2DLL_Victoria-2-Extension` has no licence file;
the README invites people to "investigate v2dll under the hood to use for
your own work", but that is not a licence. Resolved for this project by D6:
it is for personal use, so V2DLL code may be used, marked
`// from V2DLL: <file>` (R8) so it can be removed if the submod is ever
published.

---

### P14 — `cmd` splits `analyzeHeadless.bat` arguments at commas

**Found:** 2026-10-12, when exporting the RTTI.

**Symptom:** `-postScript ExportRtti.java output.md 'CPoliticsView,CBudgetView,…'`
exports the detail of **only one** class. The log shows the arguments
already split (`'CPoliticsView' 'CBudgetView' …`).

**Cause:** `analyzeHeadless.bat` goes through `cmd.exe`, which treats the
comma as an argument separator even though PowerShell sent it in quotes.

**Fix:** the scripts in `ghidra/` accept the list in one argument or in
several (`ExportRtti.java`, `DecompileAt.java` join `args[1..]`).

---

### P15 — A Ghidra script cannot redeclare a `GhidraScript` method

**Found:** 2026-10-12. `ImportV2Symbols.java` declared its own
`setPlateComment(Address, String)`; `GhidraScript` already has one with
another return type. The script **does not compile**, and in headless mode
that does not stop the analysis: the log says `skipping …
ImportV2Symbols.java` and the analysis finishes "fine" without importing
anything.

**Fix:** use `GhidraScript`'s methods and, after any headless run with
scripts, search the console for `REPORT SCRIPT ERROR` and for the script's
summary line (`ImportV2Symbols: N rows OK`). Without that line, the script
did not run.

---

### P16 — Victoria II script has no `if`

**Measured:** 2026-10-12, in TGC v2.5.2's events and decisions: 0 uses of
`if`/`else_if`, against 5 304 of `random_owned = { limit = { owner = { … } }
owner = { … } }`, the usual trick for a conditional effect.

**Consequence:** every conditional branch of an effect is (a) a different
event or option with its own `trigger`/`allow`, or (b) the `random_owned`
trick (it only works if the country has at least one province).
Probabilities that depend on state are done in steps: one event per step
with its `random_list` of fixed weights (`docs/law-design.md` §2).

SUB-0 found that TGC's event options never use `trigger` (0 uses in
v2.3.4's `events\`); whether the engine evaluates the `trigger` of an
`is_triggered_only` event fired by an effect is measured in U-2.

---

### P17 — The V2DLL project uses the `v145` toolset

`V2.vcxproj` asks for `PlatformToolset` `v145` (Visual Studio 2026). This
machine has Visual Studio 2022 (MSVC 14.43, toolset `v143`) and Build Tools
2019 (`v142`). When building V2DLL code: retarget to `v143`, **Win32**
platform. If something uses C++ from a version `v143` lacks, note it.

---

### P18 — `tgc_scan.py --diff` is not a file diff

**Measured:** 2026-10-12 (SUB-0). Between `mod\TGC` (v2.3.4) and
`TGC_v2.5.2\TGC`, `tgc_scan.py --diff` lists only 3 interface files;
`diff -rq` gives **223** differing files (decisions, events, history,
`countries.txt` with `dynamic_tags = yes` and the new tag `JLS`, etc.).

**Cause:** the tool compares the **inventory** (ideologies, laws,
governments, event range, hashes of the `.gui`/`.gfx` files, anchors), not
the tree.

**Consequence:** "no differences" from `--diff` means "nothing the submod
takes for granted changed", not "the versions are the same". To look for
something that tells versions apart (or a name the submod will use outside
the inventory), run `diff -rq --strip-trailing-cr` on the two folders. If
the submod starts depending on something new, add it to the inventory.

### P19 — A submod loaded before TGC is wiped by TGC's `replace_path`

**Measured:** 2026-10-12 (U-2, first run, TGCX + "TGC v2.5.2 (stable)").
The game made the settings folder `TGCX_TGC252` (TGCX first), and
`setup.log` lists the 105 decision files of TGC and not
`tgcx_loadtest.txt`. Denmark kept TGC's name; the load-test decision did not
exist.

**Cause:** `TGCX.mod` named only `"TGC - The Grand Combination"` in
`dependencies`. The v2.5.2 test copy is called `"TGC v2.5.2 (stable)"`, so
the launcher did not order it before TGCX. TGC loaded second, and its
`replace_path` on `decisions`, `events`, `localisation` (and the rest)
discarded everything loaded before it, TGCX's files included.

**Fix:** `dependencies` lists every name TGC can have here:
`{ "TGC - The Grand Combination" "TGC v2.5.2 (stable)" }`. To check a run,
the settings folder must be named `TGC…_TGCX` (TGC first) and `setup.log`
must list `decisions/tgcx_*.txt`. A TGC installed under another `name`
needs that name added.

### P20 — Localisation: the file that sorts first wins

**Measured:** 2026-10-12 (U-2, first run) and documented on the Vic2 wiki,
`Localisation` page (`docs/report-research-1012.md` §1).

**Cause:** when a key is defined in several `.csv` files, the game keeps the
one from the file whose name sorts **first** (inside one file, the last line
wins). TGC defines country names in `00_countries-names.csv`; a
`tgcx_*.csv` sorts after every `00_*.csv`, so its value for an existing key
is ignored whatever the mod load order.

**Fix:** keys that override TGC or vanilla go in a file named
`000_tgcx_<mechanic>.csv` (`0` sorts before `_`). Keys that are ours alone
stay in `tgcx_*.csv`. `tools/install_submod.py` accepts the `000_` prefix.

### P21 — An event's `trigger` is not checked when an effect fires it

**Measured:** 2026-10-12 (U-2, run 2, TGC v2.5.2). The load-test decision
fired 420000001 and 420000002 with `country_event`; their triggers are
opposite (`ENG` has / has not `great_gamer`), and **both** appeared.

**Cause:** an `is_triggered_only` event fired by an effect skips its
`trigger`. (A search summary attributed this to the Vic2 wiki, with events
the game fires itself as the exception; the wiki pages read did not say it.
The measurement is the evidence.)

**Fix:** decide in the effect that fires the event, with the IF emulation
(`random_owned = { limit = { … } owner = { country_event = … } }`), as TGC
does. Keep the event's `trigger` only as documentation or for other ways of
firing it. `docs/scripting-reference.md`.

### P22 — Victoria II has no hidden events: every event opens a window

**Measured:** 2026-10-12. The user, from play: every event pops up. Neither
the wiki's [Event modding](https://vic2.paradoxwikis.com/Event_modding) nor
[How to make an event](https://vic2.paradoxwikis.com/How_to_make_an_event)
lists a `hidden` (or `hide_window`, `is_hidden`) key, and every event needs
an `option`. TGC v2.5.2 has 0 uses. (Later Paradox games have
`hidden = yes`; Vic2 does not.) The user also fired the LAW-0 test event
(`hidden = yes`) before it was removed: `game.log` records "Event: 'Hidden
Event Test' for player" and its window opened; `error.log` stayed empty, so
the unknown key is silently ignored.

**Cause:** the engine predates hidden events; an event is always a window
with at least one button.

**Fix:** silent work goes into **decision effects** (a decision opens no
window), with the IF emulation (P16, P21). Events only for news the player
should see anyway. Something that must run on its own (the favours tick) is
a decision the DLL sends (`CExecuteDecisionCommand`, R5), clicked by hand
without the DLL (R6). The `hidden = yes` test was then dropped from the
generator.

### P23 — A polled event with `mean_time_to_happen = { days = 1 }` can fire a month late

**Measured:** 2026-10-12 (U-3, two runs, TGC v2.5.2, test build with
`debt_days` 60). Promise made on 1 January 1836, so the
`tgcx_debt_deadline_reactionary` modifier ended around 2 March; the broken-
promise event (trigger: the debt flag set and that modifier gone, MTTH 1
day) fired on **2 April 1836** in both runs (`game.log`). The modifier
durations themselves are exact: the broken-debt modifier added on 2 April
1836 with 3 650 days says "Expires on 2 April, 1846".

**Cause:** not documented. The engine evaluates MTTH country events less
often than daily (about monthly here); a search found no Vic2 source for
the interval.

**Fix:** a polled event is "soon after", not "on the day". What must
happen on the day goes into a decision's `allow` (it reads the modifier
live): the vote already checks `tgcx_debate` that way. Deadlines measured in
years (the debts) can absorb the month; show the deadline from the
modifier's own date, not from the event.

### P24 — Loaded signatures contain ASLR-relocated operands

**Measured:** 2026-10-12, DLL-0. PE HIGHLOW relocation metadata places
absolute operands at signature offsets 8 (RVA `0x233826`, menu label)
and 11 (RVA `0x4FD2A0`, upper-house trigger). `gen_signatures.py` measured
them from the hash-verified reference executable; the native fixture
passed both at the nonpreferred base `0x01700000` and rejected a corrupted
relocated operand. This is fixture evidence, not an in-game observation.

**Cause:** `check_exe.py` compares disk bytes at the preferred base. Windows
adjusts absolute operands when loading at another base, so comparing
those same expected bytes directly against loaded memory falsely rejects
the correct executable (P7).

**Fix:** generate relocation offsets alongside the shared signature table.
Adjust each expected 32-bit operand by `loaded_base - preferred_base`
before comparing. Preserve the full comparison and the executable hash
gate; do not mask or wildcard the relocated address bytes.


### P25 — A time-advance pump call is not a general render tick

**Measured:** 2026-10-12, DLL-1. U-4 retained one January 2 snapshot;
scoped decompilation of UpdateTimeThrottle at VA 0x00685620 (RVA 0x285620)
shows DLL-0's CALL RVA 0x285727 inside advancement/throttle conditions.
This explains why the site cannot guarantee paused/menu sampling.

**Fix:** use guarded Direct3D9 Present for read-only periodic diagnostics
and EndScene for drawing inside the native scene. Test unchanged-date
Present callbacks in a real D3D9 fixture, then verify pause/reload in U-5.
Keep original CPU/renderer state, signatures, callbacks and fallback;
do not introduce a polling thread or simulation writes.

### P26 — Intermediate callbacks and inherited viewports can hide an overlay

**Observed:** 2026-10-12, U-5 screenshot: original pie, no chart. 0.2.0
logged no successful draw and did not name failed guards, so its exact
runtime blocker is unproven. EndScene can be an intermediate/loading pass;
assuming its viewport is already full-screen is an unnecessary restriction.

**Code/test evidence:** the final engine Present function is FUN_0099A520,
RVA 0x59A520, renderer in ESI and active device at +0x64. A real D3D9
fixture confirms a restored smaller viewport can coexist with full-screen
overlay pixels when drawing gets its own captured viewport and short scene.

**Repair:** 0.2.1 uses guarded final engine presentation, verifies the active
backbuffer, sets/restores its viewport, and logs the first failed GUI/data
stage plus bounded follow-up diagnostics. This supersedes P25's initial
EndScene drawing route. Do not call the game defect resolved until U-5
passes; preserve the failed log and keep the game open for live inspection.

**P26 follow-up, live evidence:** 0.2.1's runtime guard rejected the derived
CEU3Graphics renderer table RVA 0xA19B8C. The Politics root +0x250 already
contained absolute (768,470) at 2560 x 1440; adding half-screen again was
incorrect. 0.2.2 guards both renderer families and reads absolute geometry.
All ideology data and paused cadence are now runtime-confirmed. See
`report-dll-12-live-fix-1012.md`; 0.2.4 functional drawing is now
confirmed in `report-u-5-1012.md`. Visual polish/extended checks remain.

### P27 — A tooltip return can be a structured object, not one string

**Observed:** 2026-10-12, 0.2.2 drew the chart and crashed. The dump shows
an invalid large memcpy in the engine tooltip-copy path. Scoped helper
FUN_004115D0 (RVA 0x115D0) copies three string-bearing fields plus flags.
The DLL's guessed empty 24-byte string left the rest uninitialized. A
synthetic callback that mirrored that guess did not validate the real ABI.

**Fix:** 0.2.3 removes tooltip suppression/return-buffer writes entirely
and leaves the native callback untouched. 0.2.3 kept a separate drawn
tooltip; 0.2.4 removes that duplicate box and retains only native tooltips.
A regression test guards the unchanged pointer. Any future suppression
must establish the full structure, constructors, destruction/ownership
and caller ABI using the engine, not a same-assumption fixture. Stability
was reported for 0.2.3 and functional acceptance for 0.2.4; see
`report-u-5-1012.md`. Extended integration checks remain unconfirmed.


## P28 — Final presentation drawing ignores GUI layering

**Observed 2026-10-12, 0.2.3:** the user confirmed the chart works, but the
beige chart panel covers the native tooltip and a second drawn tooltip
appears above it. Final Present comes after the whole interface, so
painting there cannot respect the engine's window/tooltip draw order.

**Repair in 0.2.4:** final Present only samples/logs. Draw after the native
`chart_upper_house_overlay` entity in FUN_00999D00's ordered GUI loop,
with its existing scene, verified object name/vtables and restored D3D
state. Preserve all original drawing; failed guards retain the pie.
Remove the separate seat tooltip and leave the native structured tooltip
untouched (P27). A real D3D9 pixel test confirms a later opaque UI element
occludes seats and final presentation does not repaint them. The user
confirmed 0.2.4 works, and its log confirms ordered drawing in game
(`report-u-5-1012.md`). Appearance is still clunky; visual polish and
individual extended coverage checks remain open.

## P29 — Successful unittest stderr fails a redirected PowerShell check

2026-10-12. Symptom: `tools/test_dll.ps1 *> <log>` stops with
`NativeCommandError` on a test progress line ending in `... ok`, despite
the same tests passing directly. Windows PowerShell turns redirected
native stderr into error records; `ErrorActionPreference=Stop` aborts.
Python unittest normally writes progress/results to stderr.

Fix: explicitly give TextTestRunner stdout in test_install_dll.py. Keep
unittest's failing exit code and test_dll.ps1's nonzero-exit gate. The
full redirected suite passed after the change. Do not weaken signatures,
compiler warnings or assertions to work around output routing.

## P30 — New chart geometry retains old native pie hover regions

2026-10-12. User confirms 0.2.5 works but chart tooltips do not correlate
with the seats. The DLL replaces the visible pie with a hemicycle while
leaving native pie hit-testing/tooltip selection unchanged. Political
wedge seating differs from old pie sectors, so the hovered ideology can
be wrong for the dot beneath the cursor. This is not evidence of stale
upper-house numbers.

HEMI-3 0.2.6 implements the repair and is accepted in game. Share rendered seat geometry for hit-testing and
use a verified native tooltip path/ownership. Preserve native legend and
UI ordering. Never restore the guessed structured return that caused P27
or add duplicate overlay tooltips that recreate P28.

## P31 — Native hover helpers have different ABIs and compressed slice IDs

2026-10-12. Bounded Ghidra shows the pie entity bounds method takes ECX
and eight stack words, returns an x87 ST0 value and uses **RET 32**, although
its decompiled body uses only the first two floats. The Politics slice
helper takes EAX and four stack words, returns AL and uses **RET 16**.
The output slice is the ordinal among positive-share ideologies, not the
engine ideology index or its political seat position. Treating these as
ordinary interchangeable C callbacks corrupts the stack or selects wrong text.

An initial isolated fixture also called the mid-function Politics CALL site
as though it were a function entry, adding an extra return word. Its dump
showed a return into stack memory; the corrected harness resolves the installed
CALL target instead. No game was launched in that failure.

Fix: verify disassembly, argument space, output ownership, result register/
floating stack and RET cleanup. Test the actual thunks at a relocated fixture
base with exact native stack shape and state preservation. HEMI-3 retains
structured tooltip slot 10 and lets the native caller construct its result;
it remaps only scalar bounds/slice selection. Evidence: `report-hemi-3-1012.md`
and `symbols.md`; private `E:\v2re\logs\hemi3-*-1012.txt` exports.

## P32 - A feathered pie cover paints into an earlier native header

2026-10-12. User accepts 0.2.6 hover but shows the cover overlapping the
Upper House block. The native title occupies root y=83..103; a cover at
y=101 with three-pixel feather starts painting at y=98. Native ordered
rendering preserves later popups, but an oversized cover still erases
an earlier title. Checking only left-edge feather pixels missed this.

0.2.7 clips the chart to content bounds x=[37,162), y=[104,209), intersected
with the framebuffer. Captured D3D state restores the scissor rectangle and
enable flag. Real GPU readback checks the whole header and reform row,
plus the existing core/fringe coverage and hover tests. Seat geometry is
unchanged. Installed, game confirmation pending. See
`report-hemi-3-header-1012.md`; native TGC anchors stay outside git.

2026-10-12 correction: 0.2.7 left a seam. Text `maxHeight` does not locate
the painted header border. Inspecting installed `politics_bg.dds` at its
GUI offset (18,9) places the last border row at root y=100. 0.2.8 clips
at the first background row y=101 and checks both intact border and
opaque coverage of the three previously exposed rows on the GPU.
Installed, game confirmation pending; `report-hemi-3-seam-1012.md`.
# P33 — Negate a compound gate as one AND

2026-10-12, LAW-1. Vic2's `NOT` uses none-of-children semantics. Wrapping
several generated gate clauses directly in `NOT = { ... }` does not mean
"the complete gate failed" when some clauses remain true. After the native
count added chamber validity to the target gate, the stale-target test
caught a missing changed-count result.

Wrap the compound condition in `AND = { ... }`, then negate that single
node. `vote_validation()` now returns one AND node, so pass, veto and
changed-count branches remain mutually consistent. Evidence is the
catalogue interpreter's stale blocker/law/government/technology test,
before/after the fix; no additional game test is claimed.

### P34 - Conditional event options were all displayed

**Found:** 2026-10-13, U-6 user screenshot of the first LAW-1 candidate.
The Whips Report listed every quorum and margin result simultaneously,
extending below the screen. The active error and system_error logs were
empty. Installed TGC has no option-level trigger/allow/limit examples.

**Cause:** the shared report used option-level `trigger` blocks to choose
one button. The real game did not hide the other options. The catalogue
interpreter filtered them as though the feature worked, allowing automatic
tests to pass despite the unsupported presentation assumption.

**Fix:** count in the firing decision and use `random_owned`/`limit` branches
to select one ordinary result event (the established LAW-0 route, P21).
Never rely on a fired event's trigger or hidden dispatcher (P21/P22).
Keep obsolete event 420000194 as a one-option recovery for queued saves;
new decisions never fire it. Generated events reject conditional options.

**Verification:** every generated event has one ordinary option. All 231
valid five-seat Aye/Nay combinations choose one correct report, including
ties and no quorum. UK introduction and repeated Count each emit one event.
These are script regression checks; corrected in-game presentation and the
larger inline-count script load remain pending. See
`report-law-1-report-fix-1013.md`.

### P35 - Child-window render coordinates and foreground order

**Found:** 2026-10-16, U-7 screenshot of 0.3.0. The Bill was displaced
above-left of Politics, its chart painted over bloc/Crown content, and
native party labels/button drew across its lower controls. Empty game
error logs and passing offscreen GPU fixtures did not validate the native
window hierarchy.

**Cause:** the Bill's inner render subtree remained attached to Politics.
Its position setter used screen-centred coordinates, while the chart
independently reconstructed a screen-centred origin. The native scene
hierarchy still supplied the parent's transform and draw order.

**Correction:** use native foreground method FUN_009E3070 (RVA 0x5E3070)
on opening, then the native position setter. Keep logical GUI ownership
and observers. Read actual inner render translation for chart placement.
Do not raise on ordinary ticks, which could cover later native dialogs;
hide the detached Bill when Politics closes or its visibility read fails.

**Verification:** 0.3.1 reference-engine foreground/unlink/position/show/
hide fixtures and deliberately displaced real-GPU chart pixel checks pass.
Installed on E:; repair game acceptance pending. See
`report-law-2-layout-1016.md`. No fixture replaces the user screenshot.

### P36 - Immediate centred rendering is not persistent absolute placement

2026-10-16, 0.3.1 game check. Native FUN_009E3BA0 (RVA 0x5E3BA0)
stores packed input unchanged at window +0x250, but adds screen centre
only when setting the immediate inner render translation. Supplying
(-416,-340) produced an initially centred chart log and later left most
of the detached Bill off-screen. The fixture had checked only that first
render translation, missing the persistent negative position.

Use FUN_009E3D20 (RVA 0x5E3D20), this in ECX, integer x/y on the stack,
RET 8. It sets both persistent packed absolute position and inner render
translation to the same values. Check both after placement, across screen
sizes, and after user movement. 0.3.2 uses this setter and enables native
background dragging; automatic gates pass, integrated game check pending.
See `report-law-2-position-1016.md`.


### P37 - Native render inputs are not pixels; prepend is not foreground

Live 0.3.2 inspection corrected the preceding P35/P36 implementation advice.
Raw inner +0xFC/+0x100 include renderer screen-centre coordinates. Read
world matrix +0x84 / negative +0x88 for pixels. Use the packed setter with
positive absolute screen x/y, not negative half-window offsets or the
unconverted absolute setter. The prior P36 absolute-setter recommendation
is superseded. FUN_009E3070 inserts before active head; FUN_009E2FC0 plus
inner FUN_009F9050 appends after Politics. Test actual root-container list
order and projection, not a guessed sentinel/position callback model.
0.3.3 visual layout is accepted by the user's screenshot; broader interaction
coverage remains separate. `report-law-2-live-1016.md`,
`report-law-2-visual-1016.md`.


### P38 - The native decision click skips potential and allow

**Cause:** `FUN_006DCE10`, the Politics decision click the DLL reuses,
strips the `POLITICSVIEW_DECISION` prefix and executes the named decision for
the player. It does not test the decision's `potential` or `allow`.

**Symptom:** a stale or repeated button press (the vote during the debate,
a second collection in a month, an audience already granted) runs the
effect anyway.

**Fix:** every decision the DLL sends checks its own conditions inside its
effect.
- Purchases and promises use the fresh guard.
- `tgcx_audience`, `tgcx_withdraw`, `tgcx_call_vote` and
  `tgcx_collect_favours` wrap their effect in their `potential` plus
  `allow` (`CatalogGen.DLL_SENT`).
- The DLL may grey out a button, but the script decides.

`report-law-2-controls-1017.md`.

### P39 - The engine validates decision potential and allow

**Cause:** the decision click (P38) does not test `potential`/`allow`, but
the command it posts does. `CExecuteDecisionCommand` slot 14,
`FUN_00906B40`, evaluates the decision's enabled flag (+0x18), `potential`
and `allow` before executing.

**Symptom:** a decision hidden from the list by `potential = { always = no }`
(or by any trigger that is false for the player) also rejects the commands
the DLL sends for it. Hiding the 46 bill presentations and the Bill's own
decisions in script would silently break every Bill control.

**Fix:**
- keep the decisions' real `potential`/`allow`;
- hide them in the DLL, in the Decisions list builder `FUN_006DB2E0`
  (by the `tgcx_` name prefix), only while the Bill GUI is usable;
- without the DLL the list is unchanged (R6);
- P38's own-guard rule still holds for stale presses.

`report-law-2-reforms-1017.md`.

### P40 - An inner matrix is relative to its owner

**Cause:** a GUI window's inner object (`window+0x494`) stores its matrix
translation `+0x84/-0x88` in its owner's frame (`+0xE4`). Only the direct
children of the renderer scene (`renderer+0x6B9E0`) are in screen pixels.
Politics and the Bill are such children, so P37's "matrix = pixels" held
there by coincidence.

**Symptom:** TPL-0 0.4.0 read the build root's matrix as screen pixels. The
build root is a child of the Military window, so the promoted panel opened
at the upper left, shifted by Military's (768,470), with its title on the
top bar.

**Fix:**
- add the translations along the owner chain up to the scene; refuse a
  chain that does not reach it;
- every placement fixture includes a translated parent;
- log the live chain when the panel opens.

`report-tpl-0-placement-1017.md`.

### P41 - A textSpriteType does not make a bare button

**Cause:** `GFX_button_standard_177` is declared as a `textSpriteType`.
Vanilla only puts it on buttons that also carry `buttonText`.

**Symptom:** TPL-0 0.4.2 used it, without `buttonText`, for Recruit
Template. In game the button art did not draw and the click did nothing;
only our text box over it showed.

**Fix:**
- bare buttons use only plain `spriteType`s (here a generated copy of the
  same art);
- `test_templates_gui.py` refuses any `quadTextureSprite` that names a
  `textSpriteType` or `corneredTileSpriteType`.

`report-tpl-0-visual-1017.md` §8.

### P42 - Showing a window re-shows its whole subtree

**Cause:** the native window show (slot 8) also makes every child window
visible again. TPL-0 0.4.2 cached each child's last show/hide state and
skipped the call when it matched.

**Symptom:** the first opening was right. After leaving Templates for
another folder and coming back, every unit icon of every row (the last
declared one on top), every empty-row cover and the ring were all visible:
one wrong icon in every place, including empty ones, and the blue row art
hidden under the covers.

**Fix:** clear the visibility cache whenever the panel window is shown
(0.4.3), so the next labels pass re-hides what must be hidden.

`report-tpl-0-visual-1017.md` §9.

### P43 - A field named after the argument it is passed as is not identified

**Cause:** the native new-stack button passes stack `+0xC0`, `+0xC4` and
`+0xDC`→`+0x58` to `CCreateUnitCommand`. The research named the first one
"province" from its position. It was never checked against any other
reader.

**Symptom:** the U-9 probe logged "province 4280902" for every army. That
is `0x415246`, the string "FRA": `+0xC0` is the owner's tag. The Conform
driver would have sent Moves to a non-province and never merged.

**Fix:** the province is `+0xDC` (province object) → `+0x58` (id). Two
other readers confirm it: Move's constructor stores it as the start
(`+0x8C`), and the AI merge compares it. It is guarded (0.5.1). Name a
field only from a reader that uses it as that thing, and probe it in game
before building on it.

`report-tpl-1-research-1017.md` §11.

### P44 - A GUI name lookup searches only the host's direct children

**Cause:** the native find-by-name methods (button slot 13, window slot 27,
text slot 15) look only among the host window's own children; they do not
descend into nested windows.

**Symptom:** on 0.6.0 the Conform button drew and clicked, but nothing
happened. The DLL probed for `tgcx_cf_open` on the unit panel, and that
button is inside the `tgcx_cf_tab` window, so the lookup returned nothing.
The panel was never set up, and the log had no "Conform native window
found" line.

**Fix:** look each element up on the window that directly contains it.
Probe for our GUI with an off-screen marker button that is a direct child
of the native window, as TPL-0's `tgcx_tpl_marker` does (0.6.1,
`tgcx_cf_marker`). `test_conform_gui.py` checks the marker's level.

`report-tpl-1-window-1017.md` §6.

### P45 - A command's size comes from its native allocation, not from its fields

**Cause:** the `CMergeCommand` size was recorded as `0x48`, but every
native caller allocates `0x4C` (`push 0x4C; call operator new` at RVA
`0x26EC40` and `0x426A1F`). The ctor writes a byte at `+0x48`, one byte
past a `0x48` block.

**Symptom:** on 0.6.1 the game crashed with heap corruption
(`0xC0000374`) right after the first Conform Merge ("stack 6891 joins
597"). The fault surfaced later, in a native free, while the merged
stack's map avatar was being destroyed. The fixture had asserted the same
wrong size, so it passed.

**Fix:** 0.6.2 allocates `0x4C`, and both native pushes are guarded. Take
the size of any object the DLL allocates for the engine from a native
`push size; call new` next to a ctor call, and guard that push. Never
infer the size from the last field the ctor writes.

`report-tpl-1-window-1017.md` §7.

### P46 - A native row builder that ran once keeps its element state

**Cause:** `tgcx_ReformRowImpl` sets each Reforms row's enable byte only
while the native builder `FUN_006DA930` runs. The builder runs once per
view: not when a bill starts, and not on Politics hide/show (the same view
is reused).

**Symptom:** in U-7 on 0.6.3, every row lit with no bill stayed lit after a
bill opened. Board 1 lights only the target.

**Fix:** a hook that only sets state never clears what it set earlier.
Either make the native view rebuild when our state changes, or own the
elements' state from then on. LAW-3 step 2.

`report-u-7-1018.md` F1.

### P47 - A fixed table of native objects freed only by our destructor hook fills up

**Cause:** Conform kept the unit panels it saw in two tables of 8
(`tgcx_cf_views`, `tgcx_cf_panels`). A slot was freed by the hooked slot-0
destructor. A panel whose vtable simply changed (destroyed by a path the
hook does not see) left only the view table; its panel slot stayed taken.
The game makes many `CUnitView`s over a long game: 11 distinct ones in one
year of MOD-0 run 1.

**Symptom:** after about a year of play on 0.6.4 the Conform button did
nothing, and `tgcx.log` said nothing (no free panel, so the button was
never bound).

**Fix:** treat a changed vtable as destruction and free every slot tied to
that object without touching its GUI. Never let a table fill silently: log
it. 0.6.5, `report-tpl-1-slots-1018.md`.

### P48 - V2DLL writes `common\defines_v2dll.txt` into the first mod's folder

**Cause:** V2DLL keeps its game-data settings in
`<mod>\common\defines_v2dll.txt` and creates the file automatically
("Created automatically, edit freely"). The mod is the one named by the
first `-mod=` (the same `ResolveModFolder` as its plugin loader).

**Symptom:** after V2D-1, `mod\TGCX\common\defines_v2dll.txt` (3297 bytes,
written 2026-10-18 19:38) was an extra file in our submod. It was found by
re-checking MOD-0's baseline hashes. In a normal launch, the same file would
appear in TGC's own `common\`. That is V2DLL editing TGC's folder, not us,
but it changes what a re-hash of TGC finds.

**Fix:** after any run with V2DLL, check the first mod's `common\` and move
the file out (it was moved to `E:\v2re\logs\tgcx-v2d1-1018\v2dll-run\`).
`install_submod.py` and any R1 hash check must expect this file when V2DLL is
installed. MOD-2's base profiles must too. `report-v2d-1-1018.md` §3.

### P49 - The launcher does not enforce `dependencies`

**Cause:** with only TGCX ticked, the launcher passes `-mod=mod/TGCX.mod`
alone, with no warning, although `TGCX.mod` names TGC v2.5.2 in
`dependencies`. The base is then vanilla. A mod whose dependency is not
ticked is also placed **first** on the command line (MOD-0 run 2).

**Symptom:** MOD-0 run 3 crashed right after loading, before the first GUI
frame. The likely cause (inferred) is TGCX's TGC-generated
`country_politics.gui`, which uses 14 sprites only TGC defines.

**Fix:** never rely on `dependencies` to keep base-generated files away from
another base. MOD-2 installs each package for the base it was generated
from, and the DLL acts only when one of our packages is in the `-mod=` list.
`report-mod-0-1018.md` §2, §4.

### P50 - The launcher orders `-mod=` by display name, not by dependencies

**Cause:** every recorded command line (MOD-0 runs 1-2, the 0.8.0 game
check, `E:\v2re\logs\tgcx-mod0-1018\cmdlines.txt`) lists the ticked mods
sorted by their `.mod` `name`. "Army Templates and Conform (…)" came before
"TGC v2.5.2 (stable)" although it depends on it.

**Symptom:** none so far. This corrects MOD-0's reading (P49 and
`report-mod-0-1018.md` Q1, "an unticked dependency goes first"). Run 2 of the
0.8.0 check worked with Army listed first, more evidence against P19.

**Fix:** never rely on `-mod=` order. The loader already searches every
`-mod=`. The settings folder still joins the `user_dir`s in command-line
order, so a package with a `user_dir` decides it by its name. With
"TGCX Politics" after "TGC v2.5.2" the folder is `TGC252_TGCX`.

### P51 - Do not leave a created detach destination empty until the next day

**Evidence:** 0.8.1 queued Create, waited a day, then posted Transfers; seven
attempts left extras behind and produced no verification. The tracker silently
dropped destinations that vanished after appearing. Logged movement/location
checks passed. Exact engine cleanup timing is inferred, not fully traced.

**Fix:** queue Create and its Transfers together using the engine queue; the
regiment reference exists already and destination identity is resolved at
execution. Verify actual membership rather than interpreting queued as done.
Report vanished destinations and do not retry into missing ones. Native split
and load performs Create/Transfer in one execution, supporting this ordering.
0.8.2 confirmed 3/3, 1/1 and 6/6 moved in game, including pending recruitment.
See report-tpl-4-1019.md and report-tpl-4-runtime-1019.md.

### P52 - TGC's hardcoded map path survives a descriptor folder change

**Cause:** stable TGC map/default.map sets definitions to
../mod/TGC/map/definition.csv. A descriptor pointing at mod/TGC_v2.5.2/TGC
does not redirect that explicit relative path. An existing mod/TGC on E:
masked it; the fresh nested-only copy had no such folder.

**Symptom:** file exception in discfile.cpp line 83, could not open
map/../mod/TGC/map/definition.csv, after V2DLL/Army initialized successfully.

**Fix:** prepare the fresh base at canonical mod/TGC and point its generated
descriptor there. Do not edit TGC's map data. Verify actual referenced paths,
not only source hashes. This fresh-copy repair retained the old nested folder
and descriptor backup; 16,411 canonical files are byte-identical. Game retry
still required. report-fresh-map-path-1019.md.

### P53 - TGC header coordinates hide Conform on vanilla

**Cause:** the 20x20 opening button was fixed at (222,68), beside TGC's
attrition icon. Vanilla, HPM and DoD use a header 26px higher. At y=68 the
button falls under the statistics row, so the user cannot open Conform.

**Evidence:** vanilla log finds Conform window/tab and valid land armies,
but has no window-open record; screenshot shows no header button. Effective
unitpanel.gui places unitattrition_icon at y=42, versus TGC y=68.

**Fix:** generate our tab from that direct named native anchor, x minus23
and the same y, leaving a 3px gap. Ambiguous/missing anchors fail preparation.
Tests cover actual vanilla/TGC and a translated override. Do not hardcode
one mod's header coordinates for other bases. Installed GUI-only correction
awaits runtime retry: report-mod-3-button-1019.md.


### P54 - A factory-created GUI root remains registered with the manager

2026-10-19, MOD-6, static RE finding before runtime installation. Native
factory FUN_009A38F0 (RVA 0x5A38F0) registers root+0x18 in CEU3Gui+0x5C.
Calling the root's scalar destructor directly would leave a dangling
registration. Native CUnitView teardown FUN_0079D490 (RVA 0x39D490) hides
the root then calls primary slot 20, FUN_009E3D60 (RVA 0x5E3D60), which
sets +0x52 and propagates release through children for manager collection.
Universal uses that checked path, never direct deletion/array edits.
Fixture coverage validates the wrapper/dispatch; real turnover and manager
collection still require game testing. See symbols.md and report-mod-6-1019.md.


### P55 - Free soldier pops cannot necessarily reach the army

2026-10-19. User found this in vanilla/HPM/DoD 0.9.0. The planner chose
the most free capacity country-wide, including disconnected colonies,
and pending tracking counted those recruits even after a failed march.
0.10.1 uses the same native land-route filter in preview and recruitment;
unreachable pops contribute no capacity. Stranded origins/finished stacks
are removed from pending tracking without cancelling/disbanding/relocating
them. Native query direction is recruit province to actual target; clean
its temporary result with the game's allocator. Never replace reachability
with continent-name equality. Runtime verification remains pending.


### P56 - Independent entry windows can sit behind their native host

2026-10-19. Universal0.10.1 DoD loaded and constructed shared roots, but
the user saw no controls. Entry positioning/showing omitted native
foreground promotion; unlike old nested fragments, independent roots also
need explicit host-hide handling. 0.10.2 promotes on first show/reopen,
keeps regular visible polls below later dialogs, hides on host close and
logs placement refusals/success pixels. Actual native scene-order fixture
covers behind-host/reopen/later-dialog cases; live retry still pending.
See report-universal-visibility-1019.md. Successful factory logs alone do
not establish display or clickability.


### P57 - Root visibility is not child-control activation

2026-10-19. Universal0.10.2 DoD still failed with root construction and
placement success logged. Native CFixedWindow primary Show (0x009E4450,
RVA 0x5E4450) only toggles root flags. CGui secondary Show (0x009E4490,
RVA 0x5E4490) takes window+0x18 and traverses its child controls/subwindows.
Actual engine bytes on an initially hidden synthetic button reproduce the
difference. 0.10.3 activates subtrees on hidden-to-visible transitions;
regular polling must not reset intentionally hidden covers (P42). Runtime
retry pending. See report-universal-subtree-1019.md.


### P58 - A fixture can repeat an inverted field interpretation

2026-10-19. DoD0.10.3 GUI works, but the route filter excluded all18 pops
and built none of4 requested regiments. Map province+0x2A is IS LAND,
nonzero; code and fixture both assumed sea. 0.10.4 reverses endpoint and
path-node checks. Independent actual-native naval/coastal policy test
accepts water0/water0, rejects land1/land1 for a navy, and permits mixed
endpoints only through land's linked port. Use such independent semantic
evidence; mock ABI/call tests cannot validate unknown field meaning.
See report-universal-land-flag-1019.md; live correction retry pending.

### P59 - Cross-type GUI names can dispatch a destructor during construction

2026-10-20. Province Rename0.1.0 used the same names for its opening,
Apply and Cancel buttons and their text labels. User selected a province
and clicked its name; the UI timer crashed before any rename command.
V2DLL's C0000374 stack passed through named-root/button construction and
FUN_00A593A0, freeing an invalid field at+0x144.

Native button builder FUN_009CFF20 (RVA0x5CFF20) resolves a definition by
name then calls slot11. The colliding definition resolves to another type;
its slot11 at VA0x00A5A430/RVA0x65A430 subtracts this by0x64 and jumps to
the scalar destructor. This matches the fault chain. The registry does not
enforce the requested control type. Braces/font/sprite validation alone
cannot detect this ABI violation.

Fix: unique names for every generated definition, including overlaid text
labels; labels pass clicks through with allwaystransparent=yes. Reject
duplicate names during packaging. Before native root construction, resolve
our button keys and require the measured concrete type and factory. For
these plain buttons,0.1.3 uses CButtonType's tableRVA0xA3CFBC and factory
RVA0x5F0310. The interim CEU3ButtonType assumption was wrong; see P60.
Resolve the owner through CEU3Gui+0x34, then use that CWindowType definition's
button registry at+0x150, matching FUN_009CFF20. These are separate registries;
the preflight fixture must distinguish them.0.1.2 corrects the interim0.1.1
preflight scope; report-province-rename-preflight-1021.md.
0.1.1's regression executes the actual native button builder and wrong-type
thunk, intercepting the destructor endpoint before destructive effects;
the new runtime preflight refuses the collision. Game retry remains open.
See report-province-rename-crash-1020.md. The earlier30native checks did
not exercise root construction and therefore missed this failure.

### P60 - A related GUI class is not the concrete parsed button type

2026-10-21. Rename0.1.2 required CEU3ButtonType's tableRVA0xA08D2C.
The screenshot had no entry and the log refused construction. Read-only
inspection of the live game shows all three plain guiButtonType definitions
use native CButtonType tableRVA0xA3CFBC and slot11 factoryRVA0x5F0310.
The earlier fixture supplied the same guessed CEU3ButtonType as the guard,
so it could not reveal the mismatch. Root/child registry scope was correct.

0.1.3 checks the measured standard type and factory. New fixtures execute
actual native registry wrapper/lookup/trie traversal, then standard-type
builder dispatch; wrong-type and changed-factory cases still refuse.
Match a concrete class to live or actual parser/constructor evidence, not
just a similar RTTI name. The P59 recommendation's CEU3ButtonType constant
was incorrect for these plain buttons and is superseded by this finding.
Do not weaken the guard to accept arbitrary types. Runtime display/input
retry remains pending; report-province-rename-button-type-1021.md.

### P61 - Window size and text origin are not interchangeable layout fields

2026-10-21. Rename0.2.0 created both roots but refused ABC placement because
it read definition+0xA0 as width. The live CWindowType there stores X=0;
width380 is at+0xA8. A constructor's ESI-relative field had been applied to
another object without establishing the register's precise base. Actual
CFixedWindow constructor stores runtime size at+0x4A0/+0x4A4, confirmed as
380/525 in the selected live panel. 0.2.1 uses that runtime width.

A second assumption would have misplaced the button: text's graphical
matrix was identity, while native Draw separately uses text inner+0x188/
+0x18C for local17,45. Sum those with the P40 owner chain before measuring
the province name. Root/header/name lookup and font methods were valid.

The old fixture supplied width at the wrong definition offset and put
text offsets in the matrix, repeating both assumptions (P58). Corrected
fixture retains live X=0, runtime width, identity matrix and separate local
origin; an additional test executes actual native Draw and observes17,45
at the font/GPU boundary. Actual native font measurement remains covered.
Check concrete object layout and semantic consumers before using offsets;
log the precise failed placement stage. Do not relax bounds to accept zero
width or compensate with one base's fixed coordinates. 0.2.1 installed,
53native checks pass; player retry pending. Report-province-rename-anchor-1021.md.


### P62 - Native edit insets affect both raster padding and entity position

2026-10-21. Province Rename0.2.3 centers by changing CEditBox+0x150/+0x154
before display refresh. It passes native fixtures, but the user reports
flickering/off-center input. The native refresh FUN_009B6400 (RVA0x5B6400)
forwards the insets to text texture setter FUN_009DDCF0 (RVA0x5DDCF0), which
passes padding to the rasterizer. Separately, native packed position method
FUN_009B5A40 (RVA0x5B5A40) adds the insets to the text entity origin.
The centered offset therefore applies twice; parent positioning can repeat
it. The prior spy checked only inset values before refresh, so it missed
the second consumer. A shared caret field is not proof of rendered layout.

0.2.4 runs the original refresh with zero texture padding, restores the
centered shared text/caret insets and calls native placement synchronously.
Repeated placement now keeps one offset. The regression executes actual
native refresh, line/cursor getters, texture setter and position method,
intercepting only raster/GPU endpoints. It reproduces old visible X314
instead of182 for origin50/name width32, then verifies new X182 and stable
empty/short/accented names through repeated parent position calls.
Live keyboard/caret/flicker acceptance remains pending. See
report-province-rename-centering-1021.md. Trace all downstream layout
consumers and execute them in a regression; do not mirror field writes.

### P63 - Native cursor visible ink is offset inside its sprite container

2026-10-21. Province Rename0.2.4 centers text stably, but the user sees its
caret displaced. Native cursor is32x32 with visible ink at(7,6)..(8,22),
only1x16; native pixel measurement adds3px after the measured text width.
Aligning the texture container with the font row does not align its ink.
The CEditBox constructor initializes separate caret-only offsets at+0x184/
+0x188, used by both native caret update paths in addition to shared text
insets.0.2.5 sets X=1-7-3=-9 and Y=(fontHeight-16)/2-6. Text centering,
blink, cursor indices and keyboard handling stay native and unchanged.

Static five-base checks verify the native sprite bounds. The regression
executes actual native caret update and its pixel getter with a seeded
cache, tests visible ink at16px/18px font heights and intercepts clock/final
position endpoints. Its initial call omitted the native update's frame
argument (RET4 despite the decompiler's no-argument signature), crashing
only the fixture executable. Corrected/guarded ABI now passes; P31 applies.
Do not equate sprite size/origin with visible pixels, or fixture-only error
with a game crash. See report-province-rename-readability-1021.md.0.2.5
player check remains pending.

### P64 - A single odd frame looks like a game load

**Cause.** The Conform tracker treated any frame whose world, player or date
differed from the last one (date going back, or a jump) as a load: it
dropped every task and resumed only the saved orders that applied to the
date read on that frame. Around a save during construction the world date
read once as earlier than the order's own creation (inferred: the
0.11.13 run continued Jan 1 to June 1 with no load in game.log), so the
resume skipped the order and the next normal frame did not count as a load
again.

**Symptom.** "N pending orders forgotten (another game or a load)" with no
"saved orders resumed" line, right after a Replace, Conform or Build Army
click; the new regiment stays where it was built. Seen in TPL-12 0.11.13
and earlier in TPL-11 0.11.5 (`tpl11-0115/before-0115/Logs/tgcx.log`).

**Fix (0.11.14).** A load must hold for three frames in a row
(`conform::confirm_load`); dates before year 0 are ignored; the log
records what changed. A stranded order still resumes from a save taken
during construction. See report-tpl-12-load-1024.md.

### P65 - The main menu's world carries the bookmark date

**Cause.** In the main menu the world shows the bookmark date (1 January
1836) with player 0; picking a country changes the player, which counts
as a load with nothing to resume. Loading a save then moves the date
forward. A forward jump counted as a load only once a day step was known,
and right after a load the step is unknown, so a menu-loaded save never
resumed its orders.

**Symptom.** One `load seen ... player 0->65` at the bookmark date, no
later load line, no "saved orders resumed"; regiments built for Replace,
Conform or Build Army stay where they were built (TPL-12 0.11.14).

**Fix (0.11.15).** With no step known, a forward change of more than two
days is a load (`conform::date_jump`). See report-tpl-12-menu-load-1024.md.

### P66 - A failed protection restore can leave a successful hook installed

**Found:** 2026-10-27, static volunteer audit. No observed game crash or Windows
API failure is attributed to this case.

**Cause:** a pointer compare-and-swap succeeds, then VirtualProtect restoring
the original page protection fails. Returning `changed && restored` reports
failure while the pointer can still target our callback. A multi-slot installer
that rolls back only earlier successful slots misses the current changed slot.

**Fix:** track pointer installation, protection restoration and rollback
separately. Attempt rollback with compare-and-swap, preserving the first page
protection value across retries. Include the current slot in transaction cleanup.
Keep the DLL pinned and originals/context alive if any residual hook exists;
enable UI callbacks only after all slots install. Report uncertain protection
instead of claiming no hook was installed. Do not overwrite another hook owner.

**Verification:** Rust0.0.4 compiles; source audit only. Fault-injection checks
for protection/CAS/rollback and actual loader teardown are deferred by the user.
Severe OS failures can still leave uncertain protection and need diagnosis.
See report-volunteers-pitfalls-1027.md and rust/volunteers/src/patch.rs.

### P67 - A plausible font name need not exist in the loaded game

**Found:** 2026-10-27, static volunteer audit. The generated UI requested
`vic_14_black`; it is absent from the inspected vanilla and TGC .gfx definitions.

**Risk:** missing labels, parse/construction failure or a bad native font path.
A particular crash from this font has not been observed or reproduced.

**Fix:** resolve names against actual bitmapfont definitions and their assets.
Use the existing `Arial14_black`, declared in vanilla interface/core.gfx, for
the small black volunteer labels. Never invent a font by combining a known
family, size and colour suffix. Unknown effective-base overrides still require
the native/parser and in-game check. Rust compilation does not check GUI fonts.

**Verification:** installed definitions inspected; generated font names corrected.
Game rendering remains deferred. See report-volunteers-pitfalls-1027.md.

### P68 - An unescaped semicolon shifts localisation CSV columns

**Found:** 2026-10-27. The volunteer rule text contained a semicolon; the
generator concatenated it directly into the semicolon-delimited CSV. Packaging's
new validation rejected that value before creating a ZIP.

**Risk:** the English phrase is split into language columns and the trailing
marker/column count moves. This is malformed localisation, not a proved crash.

**Fix:** use punctuation that does not introduce a delimiter for generated
single-line text. Reject semicolons and CR/LF in values before emitting rows;
retain cp1252 and the game's existing language-column order. Do not assume
ordinary spreadsheet CSV quoting is supported by the game's parser.

**Verification:** the validator caught the actual bad value during staging;
the corrected package completed. No gameplay or parser execution is claimed.

### P69 - Native commands do not synchronize a plugin's local simulation policy

**Found:**2026-10-29, static source inspection of Maxioten's volunteer prototype.
The particular OOS screenshot was not reproduced and its cause is unestablished.

**Cause:** a sender posts native Create/Transfer commands, then stores a mission
only in its local g_recs/text file. HookReconcile consults that record and skips
native pop/regiment maintenance. Another peer can execute the original maintenance
on the same army. Matching DLLs or native membership commands do not synchronize
the local record. Independent per-peer daily Recall can also duplicate commands.

**Fix:** keep ordinary maintenance with donor ownership. Simulation-affecting
registry/scope must derive from the same synchronized execution/authority on all
peers. Use proved session/issuer/sequence gates and command-backed data; a digest
is diagnostic, not transport. Our experimental SP prototype refuses MP Send and
uses no maintenance bypass. Do not suppress CRC checks or claim that refusing MP
is equivalent to implementing multiplayer. Reproduce against paired same-save
artifacts before claiming the reported OOS fixed.

**Verification:** source path inspected; new SP code compiles/static-checks only.
No two-peer reproduction. report-volunteers-oos-1029.md; instructions-volunteers-oos.md.

### P70 - A valid Transfer command can refuse the final source brigade at execution

2026-10-29. User's ARC -> RGS Send selected five brigades. Read-only inspection
found four in the new army and one in the original army, all donor-owned and idle.
FUN_00513F30 / RVA0x113F30 contains a land-stack/leader predicate and refuses when
the source's regiment count is below2. The inspected source leader slot8 points
to the guarded return-true helper RVA0x631490. Transfer execute calls this gate
separately from the validity method checked before posting the batch.

0.0.8 keeps an existing army for whole-stack selection, including single-brigade
armies and the leader. Native preparation revalidates exact full membership;
partial selections still queue Create+Transfers and leave source members. Never
bypass can-transfer, write membership directly or mark a partial receipt complete.
Regression scenarios pass; new game acceptance remains open. Preserve incomplete
old records for review instead of replaying queued work automatically. See
report-volunteers-send-assembly-1029.md and instructions-volunteers-send-0008.md.

### P71 - A persisted observation date turns idle days into simulation commands

2026-10-29. Registry encoded last_processed_date, so every daily sample changed
the journal bytes even if mission/vision/membership stayed unchanged. User reports
lag; preserved log has325 ARC journal commits while the mission is unresolved.
Each commit entails native variable commands and repeated bounded lookups/checksums.
This is avoidable work, not a measured explanation of all game lag.

0.0.8 keeps the watermark runtime-only and emits0 in its existing reserved wire
slot, retaining old checkpoint decoding. Real transitions still persist. A regression
observes300 unchanged days without changing bytes, then verifies peace changes them.
Empty/unverified scopes no longer enter native callbacks, and vision visits only
entitled recipients' owned provinces. A sparse40000-province fixture reads fewer
than100 fields for one entitled province. Measure actual game responsiveness before
claiming performance fixed; never remove synchronization to reduce work. Same report.


### P72 - A caller filter inside Rust is too late for a shared hot helper

2026-10-29. The volunteer country-vector hook entered FXSAVE/Rust/session checking
before checking its unique battle caller. The helper serves unrelated systems.
0.0.9 bypasses other return addresses in assembly, preserving input registers,
flags and the original path.100000-call fixture confirms no Rust validation for
unrelated calls; real log later shows1512820 bypasses versus173 targeted calls.
This supports removing avoidable work, not a measured claim that all lag is fixed.
Scalar field reads also formerly allocated a Vec/kernel read each time; bounded
local SEH reads and a real PAGE_NOACCESS fixture retain fail-closed behavior.
Never remove native/session validation from the relevant path to improve speed.
See report-volunteers-controls-1029.md and report-volunteers-siege-1029.md.

### P73 - Movement permission alone does not prevent native exile

2026-10-29. Neutral donor ownership can pass the patched movement path while
native exile coordinators still consult donor-based country permission. Exile
+164 independently blocks combat, supply and reinforcement. User saw organization
fall after Send; the old process exited before its flag could be read, so that
particular cause was not runtime-proved.0.0.9 scopes four proven country-permission
callers to a receipt-validated recipient input image. Native engine still computes
and writes the flag; no direct clearing or maintenance skip. Actual0.0.9 log has
288 permission grants and combat becomes observable. Older exiled stacks may still
need normal return to accessible territory; never force organization/flags. Same reports.

### P74 - A display plan can reject the whole world on one empty visibility array

2026-10-29.0.0.9 log shows389 visibility callbacks but zero custom grants. Native
world recompute permits a non-null dummy/uninitialized province with a length0
per-country visibility array. Our planner instead returned None, discarding other
valid grants.0.0.10 skips empty knowledge while preserving bounds for nonempty
arrays, accepts both guarded CMapProvince primary/secondary tables, and separates
vision_refused from vision_empty counters. The empty-dummy fixture reproduces the
source failure; the exact offending live province was not captured. Fog-grant
acceptance remains open. Never write the province+174 simulation array or assume
own-army local sight proves recipient-wide sharing. report-volunteers-siege-1029.md.

### P75 - Battle hostility overrides do not authorize siege or determine credit

2026-10-29. User confirms0.0.9 battles but no siege. Initial siege gate005CC200
(RVA1CC200) and fortress join slot16/FUN005A9E80 (RVA1A9E80) compare real army
owner hostility separately.0.0.10 evaluates their original read-only predicates
with a private recipient image, preserving native busy/exile/retreat/strength and
friendly-attacker conditions. Siege progress/control attribution reads the attacker's
country list; the validated CLandCombatant AddArmy reference must name the recipient
once. Do not merely return true, change ownership/controller directly or let a
neutral donor join a siege led by its recipient's enemy. Native engine retains
creation, progress and capture. Wrapper fixtures pass; actual siege/capture-credit
verification remains required. report-volunteers-siege-1029.md.

### P76 - An optimization filter must not become stale permission or hide a loaded journal

2026-10-29.0.0.11 skips unrelated unit work by identity and narrows closed-panel
snapshots to mission countries. Identity is only a negative filter: a match must
still read current SP/session/context/epoch and exact owner/member receipts. Never
cache a positive permission, combat result or native pointer across a load.

A compact snapshot made from the OLD registry can omit a NEW journal's recipient
when loading or switching player. That can restore suspended vision with unknown
war state. Use compact polling only for the same explicitly bound player/load
epoch; rebind and UI actions take full snapshots. A cheap atomic load barrier runs
before unit argument reads, followed by full native context checks for positive
admission. Tests cover wrong membership, fresh MP transport changes and equal
orders/journal under service, peace and donor war. Benchmarks demonstrate helper
cost only; in-game timing/save-load acceptance still required. See
report-volunteers-optimization-1029.md and instructions-volunteers-optimization.md.


### P77 - Diagnostic probes can dominate a supposedly normal performance test

2026-10-29. User says 0.0.11 is slightly faster but still poor. The isolated
V2DLL 5.98 copy inherited ENABLE_STATE_PROBE=1 and ENABLE_RNG_PROBE=1. Its own
startup log confirms active daily state inventories and random/event wrappers;
the latest January 1..27 state log is 25351284 bytes, older runs hundreds of MB.
Plugin helper speedups did not measure that whole-run workload. Inspect the
actual configuration, installed-probe messages and log growth before blaming a
specific hook or extrapolating fixture ratios to game FPS. In the backed-up test
copy only, disable these two diagnostic probes and retain normal OOS/crash logs
and simulation settings. Preserve logs; their mere continued existence after a
restart does not prove active dumping. Compare same-save timings and separate
DLL versus configuration effects before claiming causality. Use sampled callback
timings/call counts for the next change; microsecond sample zeros do not measure
sub-microsecond paths or native-original work. Inactive assembly shortcuts are
negative-only and must preserve flags, registers, RET cleanup and fresh positive
admission. See report-volunteers-performance-probes-1029.md and its instructions.


### P78 - A loader upgrade can change settings, plugin layout and the first mod's files

2026-10-29. CreatineCore 6.36 replaces the V2DLL 5.98 root proxy/core path with
dinput8.dll -> z_CreatineCore/creatinecore.dll. Performance options moved into
dll_Submods/ccore_performance_patch.ini; its distributed DEBUG_LOG is 1, and the
core still distributes enabled state/RNG probes and new gameplay defaults. Migrate
by reviewed keys, keep probes/verbose logs off, and don't silently enable unrelated
features. Upstream now defaults FIX_ARMY_WINDOW_LAG off after an incomplete stutter
fix. Retain core API1/28-byte layout and per-active-mod plugin compatibility.

The source ResolveModFolder picks the FIRST -mod= descriptor for generated data.
EnsureCCoreDefines can rename common/defines_v2dll.txt and append missing keys.
Merely replacing the DLL while TGC is first would therefore modify its tree at
startup. The isolated migration uses a tiny tgcx_loader_data carrier first, then
TGC and Volunteers; all expected physical defines keys are generated there.
Don't reverse TGC/Volunteers to fix the generated-file destination: TGC's
replace_path would wipe earlier submod content. Do not copy bundled topbar/menu
files over a mod. Preserve rollback, full TGC/Volunteers hashes and explicit
launchers. A non-game proxy/API fixture is not game startup or performance proof.
See report-volunteers-loader-636-1029.md and instructions-volunteers-loader-636.md.


### P79 - Failed journal binding and old daily cursors can strand a loaded UI

2026-10-29. observe_world stored bound player/epoch before a journal read. A
failed read left registry=None but the same binding wasn't retried. Loaded
registries were also assigned directly, preserving an old last_day that could
make all earlier reloaded dates wait forever. Retry when registry is absent and
bind via the reset path. Malformed nonzero journals still refuse; never create a
new campaign over corrupt saved data or replay unit commands. Distinguish mode,
queue, epoch and journal errors in UI/logs. This is a concrete source correction,
not proof of the tester screenshot's exact cause. report-volunteers-feedback-1029.md.

### P80 - Terminal history needs lifetime, serial and journal-capacity invariants

2026-10-29. Loaded records temporarily use NeedsReview, so cap accounting must
inspect their saved terminal phase. Hide completed rows without dropping retained
wartime vision. Prune only returned/lost records without active/suspended sight,
after known peace. Serial allocation must survive removal of the greatest/all
records; retained per-country sequences form the high-water. Refuse journal
overflow atomically before publishing an accepted Send, preserving old missions
and sequence state. A 16-KiB journal can fill before the 256-row record ceiling.
Tests cover repeated wartime returns, load, cleanup and atomic capacity refusal;
the tester's unspecified many-send symptom remains unknown. Same report.

### P81 - Local visibility grants can fail independently per province

2026-10-29. The old planner returned None for the entire world after one bad map,
array or level. Skip that province and report it, retaining separately validated
grants. Native controller+134/tag+130 with REB owner+12C fallback is established
by FUN_0068C040 VA68C1EA..68C2B9; read recipient knowledge to share foreign fronts.
Never grant unknown data, write simulation knowledge or skip global torn-world/
session/epoch checks. Fixture success isn't actual recipient-wide game sight;
new grant/skipped counters and a runtime retest are required. Same report.

### P82 - Reachability previews need bounded fairness and fresh Send validation

2026-10-29. Checking only first-owned-land can reject a reachable recipient.
Enumerate owned land destinations and round-robin armies in bounded query batches
so an unreachable candidate cannot starve another. Cache only IDs/UI targets,
not native pointers or permissions; fresh mode/epoch gates precede work and Send
rechecks current destination ownership/native route. No arbitrary country-count/
distance cap or teleport follows from a no-route result. Search advances on the
existing callbacks/ticks, so pause responsiveness and many-army work still need
runtime checks. Reopen on access changes; don't claim a cache alone authorizes
deployment or an unmeasured performance gain. Same report.


### P83 - A native network Post is not multiplayer volunteer authority

2026-10-29. Host/client slot6 accepts native serialized commands, but its use
does not synchronize a local registry or prove sender-country/peer membership.
The0.0.14 probe posts only an own-country fingerprint variable and observes
execution. It never mints a SinglePlayerPermit, publishes MP combat/access scope
or posts unit orders. Treat command+34 as observed source metadata until mapping
and authority are established in instructions. Fingerprints/mirrors cannot stand
in for shared simulation-thread activation and one authority for derived orders.
Read-only bootstrap must not replay saved intents. Raw ABI evidence and tests
are in report-volunteers-mp-sync-1029.md; actual paired execution remains untested.

### P84 - Comparing the wrong load or observation boundary invents OOS evidence

2026-10-29. Logs can contain repeated dates and journal generations from several
loads; one date can contain multiple committed states. Extract the last explicit
load, require complete bootstrap with zero refused/missed observations, and
flag differing same-date samples as ambiguous. Missing samples/receipts are
unknown. A journal digest is not the engine CRC, and identical empty mirrors
do not establish volunteer gameplay. Compare matching saves/runs/native logs
before attributing the original OOS. Collector fixtures cover these exclusions.


### P85 - Entry roots can be visible while the button is unavailable

2026-10-29. A worker hide previously set Ui.disabled forever until view/root
replacement. Defer worker/reentrant hides to the captured UI thread. Independent
roots can also stay behind a raised host, retain hidden child controls, or have
expired while their cached pointer is nonzero. Use checked native scene links,
child Show/Hide slots/flags and owned-root leases. Promote only the proven covered
entry, preserve later visible dialogs, and activate the subtree only on actual
loss. Unknown/cyclic/torn data never authorizes promotion. Recreate expired roots
without touching/deleting stale memory; keep deferred observers and their bound.
Actual callback/scene regressions pass, but the tester screenshot's precise cause
and runtime recovery remain unconfirmed. report-volunteers-entry-1029.md.

### P86 - A command failure does not establish a broken native UI

2026-10-29. Checkpoint preparation/post used the same fail() as unsafe UI types,
making the entry disappear after an unrelated persistence refusal. Keep the
validated button/panels and an explanatory message, but suspend/unpublish policy
and refuse all further mission work/checkpoints for that explicit load epoch.
Only a new recognized load may recover; never retry a partial queue or replay
Create/Transfer/Move automatically. Genuine native access/type/capacity failures
retain their hard UI fences. The callback regression checks command suspension
does not disable/close the UI. Same report; actual game retest remains required.

### P87 - A loader's existing inline hook can reject the whole native GUI

2026-10-30. Official Core6.36 optionally hooks native RVA662EA0 for host-role
observation.0.0.15's all-or-nothing native signature gate expected untouched bytes,
so a gameplay transport mismatch could also suppress the UI. Log exact expected/
observed relocated bytes, split independent UI/module gates, and verify only an
explicit supported loader file hash/thunk/original trampoline/tail. Never accept
arbitrary E9 hooks or blindly skip a mismatching guard. Real0.0.16 startup accepted
the exact supported probe and all module hooks; this isn't paired gameplay proof.

### P88 - Native cloning can drop sender metadata, and local scheduling is not MP policy

2026-10-30. CSetVariableCommand Clone VA583C00 reconstructs source+34 asFFFF;
network service restores sequence+38 only. Sender-to-country checks would refuse
valid traffic if metadata were stamped before Post then silently lost by Clone.
Our namespaced-only wrapper copies sender into the new private clone, forwarding
seven opaque stack words once and preserving EAX/flags/FXSAVE state/RET1C cleanup.
Do not guess a plainRET from the decompiler. Fixture checks argument canaries,
cleanup and namespace exclusions. Command flags and sequence remain native.

MP policy is projected from native-executed committed journals at command
boundaries. UI refresh, local worker IDs and try_read contention cannot choose
simulation rules. Keep immutable snapshots, release locks before native calls,
and allow only the admitted local donor to emit derived orders. Whole-stack reuse
avoids racing new IDs. Fresh complete-roster receipt admission is separate from
probe diagnostics and saved markers. Two-replica fixtures are not two real peers
or evidence that the original OOS has been fixed. See0.0.16 report/instructions.

### P89 - A supported loader label is not a stable private layout or a peer handshake

2026-10-30. The0.0.16 uploaded log rejects RVA662EA0 before any MP HELLO, yet the
panel says waiting for peers. Display actual local native compatibility failure
first. Exact loader hashes/private RVAs also overconstrain unchanged transparent
role probes after relinking.0.0.17 recognizes only the complete bounded register/
flags-preserving private-marker and original-continuation contract, at arbitrary
module addresses. Verify executable/writable/image bounds and cycles; never call
foreign hooks to inspect them, blindly accept E9, or bypass the existing hook.
Other plugin names don't prove conflict or authorize a permanent disable policy.
Unknown gameplay-changing contracts still need evidence and separate validation.

New diagnostics include owner/path/hash/refusal stage. Repeated version labels,
matching files or synthetic layouts are not real paired compatibility. Two local
processes writing the same fixed log can interleave/truncate traces; use separate
install directories and profiles. Preserve clean full logs from both peers before
claiming an OOS cause or fix. See report-volunteers-coexistence-1030.md.

### P90 - Passing hook checks doesn't establish roster capture or HELLO execution

2026-10-30. Tester0.0.17 accepts the third-party role hook and every native gate,
yet has no HELLO post/receipt. Silent native roster/queue returns cannot be diagnosed
as a missing peer plugin.0.0.18 reports initializer status, exact refused capture
stage/raw values or accepted peer/source list. UI names local roster refusal.

FUN_00680450 constructs parallel WORLD country/controller vectors ADC/AE0 and
AEC/AF0. Their mutual lengths, active identity/pointer/source checks and complete
human set matter; the independent global manager size is an index bound, not their
length invariant. Correct the source model without inventing controller/source
fallbacks. Actual tester first refusal is not established by the old silent log.

Failed roster capture withdraws current receipt authority; successful recapture
requires fresh native HELLOs and a new issuer round. Saved markers or old permits
cannot replace admission. Canonical PID logs preserve each process independently
of a shared legacy file. Later SP Send in mixed traces isn't MP acceptance. Select
matching PID/save/load before comparing. See report-volunteers-roster-1030.md.
