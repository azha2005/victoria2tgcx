# Session close 2026-10-12 (14): 0.2.5 works, tooltip mismatch handed off

User confirms 0.2.5 works and reports tooltips do not correspond to the
new chart. Saved original 2560x1440 attachment and log outside git.
Functional acceptance recorded; HEMI-3 hover integration is next.
No new build/replacement; no extended coverage inferred.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (13). **0.2.5 HEMI-2 polish built, checked and installed on E:;
live startup/drawing observed, user acceptance pending.** 0.2.4 remains the game-tested functional
baseline. New report: `docs/report-hemi-2-build-1012.md`.

## 1. Scope and decisions

TGC v2.5.2 only; single player first; read-only drawing. Recovered the prior
agent's softer dots, feathered cover and left-to-right bloc seating. No new
mechanic, hook or address. Native tooltip and legend remain. Prefer one
combined test and consolidate any confirmed fixes. REA/RE git remain
references for later army templates; D3 still open, blocking nothing.

## 2. Next session: one HEMI-2 / U-5 game run

1. Only the user starts the usual E: launcher with TGC v2.5.2 (stable) + TGCX.
2. Follow `docs/instructions-u-5.md`: UK Politics screenshot, native
   tooltip/menu overlap, view switching/reopening, date advance, save/reload,
   and Alt-Tab. Keep the game open if a problem needs read-only inspection.
3. Agent follows `docs/instructions-hemi-2.md`: preserve the new log, check
   version/guards/errors, compare the screenshot with Main, and report only
   observed outcomes. No startup rework unless evidence calls for it.
4. If a fault is confirmed, diagnose and consolidate fixes before another
   build. Otherwise record acceptance and remaining visual deviations.
5. Archive this NEXT, update statuses/instructions and write a new report.
   Earlier reports are immutable. No unrelated mechanic automatically starts.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; backups retained | existing reports |
| DLL-1 / HEMI-1 (DLL-2) | functional 0.2.4 tested in game; extended coverage pending | DLL-1/2 instructions; U-5 report |
| HEMI-2 | 0.2.5 built/installed; live drawing observed, acceptance pending | HEMI-2 instructions; build report |
| U-5 | 0.2.4 functional checkpoint passed; polish/extended checks pending | U-5 instructions |
| GH-2 | runtime data/mapping/coordinates/ordered draw confirmed | symbols.md |
| LAW-0 / U-3 | done, tested in game | existing reports |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.5 SHA-256:
`1ec1aea4fee87441a293358451ee15859cdf66435991abebe3a52370f5de5a1b`.
23 signatures, TGC scan, Win32 /W4 /WX build, real GPU/native tests,
8 installer/Lua forwarding tests and law checks passed. Redirected check
log: `E:\v2re\logs\tgcx-hemi2-025-checks-1012.txt`.
Previous submod/record/log: `E:\v2re\logs\tgcx-before-025-1012`.
Original shim and previous proxies retained; no commit requested/made.

No .gui override was needed. The submod descriptor marks release 0.2.5;
law scripts are unchanged. Remaining deviations: generated cover instead
of native textured background; native legend order; native pie hover areas.
P29 documents the check-runner issue; next pitfall P30.

During handoff the user launched 0.2.5: 5945 chart draws, no guarded
failures, both error logs empty. See `docs/report-hemi-2-startup-1012.md`.
Visual/extended checks still pending; continue this run.

---

# Session close 2026-10-12 (13): polish build/check repair, 0.2.5 installed

Recovered the prior agent's HEMI-2 source changes. Reproduced Windows
PowerShell redirected-stderr treating successful unittest progress as an
error; moved normal test output to stdout, retaining exit-code gates.
Fixed the HEMI-2 installer record and added GPU feather-edge coverage.
0.2.5 DLL and submod installed with prior files retained. All automatic
gates passed; in-game polish acceptance pending. No earlier report changed.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (12). **0.2.4 works in game; functional prototype accepted.
It still looks clunky: visual polish is outstanding.** Evidence:
`docs/report-u-5-1012.md`. The user asked to update project/docs, not ship
another build now. Keep the fewer-iterations preference: one consolidated
polish build and acceptance run.

## 1. Scope and decisions

TGC v2.5.2 only (D1); single player first (D5); read-only rendering (R5).
DLL-2 is HEMI-1. The Main mockup and 100-seat/seven-row mechanic are
unchanged. Native ordered GUI drawing works; keep the structured tooltip
untouched. D3 stays open and blocks nothing. No new mechanic selected.
REA and the Victoria II RE repo remain references for later army templates.

## 2. Next implementation session: HEMI-2 visual polish

1. Read `docs/instructions-hemi-2.md` and the U-5 report. Compare the
   current in-game panel with Main, documenting exact remaining differences.
2. Refine the existing chart toward the mockup (background/integration,
   geometry/padding/seat legibility); preserve its proven data/layering path.
   Avoid inventing replacement designs or guessing native tooltip returns.
3. Consolidate supported changes, run appropriate native/GPU gates, then
   install one build with the game closed. No replacement is needed now.
4. In the same polish test, complete remaining U-5 view switch/reopen,
   date advancement, save/reload and Alt-Tab checks. See
   `docs/instructions-u-5.md`; do not repeat already-established startup
   troubleshooting or demand an immediate extra game launch.
5. Preserve evidence, compare the new screenshot with Main, report actual
   acceptance/deviations and update the project on session close.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; backup retained | existing reports |
| DLL-1 | working prototype, tested in game on 0.2.4; extended integration coverage remains | `docs/instructions-dll-1.md`; U-5 report |
| HEMI-1 (DLL-2) | working prototype, tested in game; visual polish outstanding | `docs/instructions-dll-2.md`; U-5 report |
| HEMI-2 | queued visual polish; not implemented | `docs/instructions-hemi-2.md` |
| GH-2 | hemicycle data/mapping/coords/ordered draw confirmed at runtime | `docs/symbols.md` |
| U-5 | functional checkpoint passed; extended checks with polish run | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | existing report |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.4 SHA-256:
`ef2ebc8da4e1e0bac0ef2ba5cba258abd2efef00a451c02b69a62384b1f80755`.
Original shim and prior proxies retained. Only the user starts the game.

Latest preserved run: 11133 frames/2687 chart draws, three identical-date
samples, show/hide and no guarded failures; both active error logs empty.
Reference/23 signatures, TGC scan, zero-warning Win32/native/GPU/installer
checks passed in the implementation session. No code change this session.

User confirms it works; do not label 0.2.4 untested. Do not claim full
reload/Alt-Tab coverage or finished visual polish. Known prototype limits:
flat cover panel, native legend order, native pie hover regions. P28
functional repair confirmed; next pitfall P29. No commit requested/made.

---

# Session close 2026-10-12 (12): working prototype accepted, polish queued

User confirms 0.2.4 works and asks to update project/Markdown, while noting
it still looks clunky. Preserved log: 11133 frames, 2687 ordered draws,
three paused samples, native show/hide, empty errors. Functional checkpoint
recorded as game-tested; extended integration checks remain unconfirmed.
HEMI-2 visual polish queued, with one consolidated future acceptance run.
No code/build/install change; previous reports preserved.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (11). **0.2.3 works, but fails layering. 0.2.4 repair installed
on E:, done, not tested in game.** Report:
`docs/report-dll-12-layering-fix-1012.md`. User wants fewer iterations:
complete one combined acceptance run and inspect it before replacing again.

## 1. Scope

TGC v2.5.2 only (D1), single player first (D5), read-only DLL (R5).
DLL-2 is HEMI-1. Route A and 100 seats/seven rows stay unchanged; draw
within native GUI order, not final Present. D3 remains open, blocks nothing.
REA/Victoria II reverse-engineering references retained for later templates.
No next mechanic selected.

## 2. Next session: U-5 acceptance on 0.2.4

1. User starts TGC v2.5.2 + TGCX on E:, UK 1836/same save, paused Politics.
2. Hover native ideology list/seat area and open a popup over Politics:
   native tooltip/window should fully cover the chart beneath it; only
   one tooltip. Collect a screenshot. Keep game open if anything fails.
3. In the same run, switch Budget/Production/Politics, close/reopen,
   advance/save/reload and Alt-Tab using `docs/instructions-u-5.md`.
4. Agent preserves log/error evidence, checks ordered draw/cadence/lifetime
   and compares the Main mockup. Write a new U-5 report; mark tested only
   once applicable checks pass. Consolidate confirmed fixes before rerunning.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; backup preserved | `docs/report-u-4-1012.md` |
| DLL-1 | 0.2.3 game drawing confirmed; 0.2.4 layering fix installed, not tested in game | `docs/instructions-dll-1.md`; layering report |
| HEMI-1 (DLL-2) | 0.2.3 displayed, layering failed; 0.2.4 installed, not tested in game | `docs/instructions-dll-2.md` |
| GH-2 | complete live data/mapping/coords and named chart/ordered-list identity confirmed | `docs/symbols.md` |
| U-5 | 0.2.3 partial evidence; 0.2.4 combined acceptance next | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | existing report |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA note retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity and gates

Installed/source 0.2.4 SHA-256:
`ef2ebc8da4e1e0bac0ef2ba5cba258abd2efef00a451c02b69a62384b1f80755`.
Original shim and tested DLL-0 .off backup intact. Only user starts game.

Reference hash + 23 signatures, TGC scan, zero-warning Win32 build,
real D3D9/native stage/occlusion/state tests and eight installer/Lua tests
passed. 0.2.3 final log: 147442 frames, 85103 chart draws, normal exit;
active game error/system_error logs empty. It failed visual layering.
0.2.4 draws after the named upper-house overlay and removes the extra
hover label; native structured tooltip untouched. Plain background,
native legend order and native pie hover regions remain prototype limits.
Game acceptance/reload/Alt-Tab remain pending. P28 added, next P29.
No third-party material enters git; no commit requested/made.

---

# Session close 2026-10-12 (11): native chart layering repair

User confirms 0.2.3 works; chart panel and duplicate tooltip cover native UI.
Identified live named chart/overlay and ordered render loop; 0.2.4 draws
at that stage, keeps final Present for sampling only and removes duplicate
hover box. 23 guards, real GPU layering/native state and eight installer
checks passed. Installed after normal user exit; 0.2.4 game acceptance pending.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (10). **0.2.2 showed the hemicycle and crashed in tooltip
copying. 0.2.3 removes the faulty tooltip hook; installed, not tested in game.** Report: `docs/report-dll-12-tooltip-fix-1012.md`.
The user requested fewer iterations; keep one combined game session.

## 1. Decisions and current scope

TGC v2.5.2 only (D1); single player first (D5), read-only DLL (R5).
DLL-2 remains the user's alias for HEMI-1. Interface route A and the
100-seat/seven-row mockup are unchanged. D3 stays open and blocks nothing.
REA and the Victoria II reverse-engineering repo remain references for
later army templates; RE symbols/V2DLL were cross-checked in this repair.
No other mechanic was selected.

## 2. Next session: U-5 retry on the installed build

1. User opens TGC v2.5.2 + TGCX, UK 1836, paused Politics; hover seats
   and wait 20 seconds to check the crash repair.
2. Check the hemicycle and hover; send a screenshot. **Keep the game open**
   if the pie remains. The agent reads the new log and, if needed, performs
   read-only live inspection without another speculative replacement.
3. If the chart appears, complete view switching, save/reload and Alt-Tab
   in that same run using `docs/instructions-u-5.md`.
4. Preserve the log outside git. Record actual drawing/cadence, mapping,
   guards, errors and mockup differences in a new U-5 report. Mark tested
   only after game evidence. Consolidate any confirmed fixes before a rerun.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; prior DLL preserved `.off` | `docs/report-u-4-1012.md` |
| DLL-1 | 0.2.2 drew then crashed; 0.2.3 repair installed, not tested in game | `docs/instructions-dll-1.md`; repair report |
| HEMI-1 (DLL-2 alias) | 0.2.2 displayed, unstable; 0.2.3 installed, not tested in game | `docs/instructions-dll-2.md` |
| GH-2 | core, complete ideology mapping/vector bounds/colours and absolute root coordinates runtime-confirmed; chart acceptance pending | `docs/symbols.md` |
| U-5 | retry next; keep one session open for inspection | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | `docs/report-law-0-1012.md` |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA note retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity and gates

Installed on E: `lua51.dll` version 0.2.3, SHA-256
`c34965b92131150c0430960e051312c3ab142ad458ed75db182d0922ea3c7bc6`.
Original and tested DLL-0 backup remain intact. Only the user starts the game.

Reference hash + 19 signatures, TGC scan, Release build, real D3D9 native
fixtures and eight installer/Lua tests passed. Final engine Present now
supplies cadence and drawing, using a separate scene and restored viewport.
Live 0.2.1 established a rejected CEU3Graphics table and the incorrect
double-centering of window coordinates. 0.2.2 corrected both and displayed the chart, then crashed because the
tooltip hook returned an incomplete structured object. 0.2.3 removes that
hook and leaves native tooltips untouched; do not
describe this build as game-verified yet. Latest pitfall P27, next P28.
No third-party art/data copied into git. No commit was requested.

---

# Session close 2026-10-12 (10): tooltip crash repair

0.2.2 displayed the chart but crashed in engine tooltip copying. Crash dump identified an invalid single-string return assumption. Removed tooltip ABI hook; 0.2.3 built/tested/installed, stability acceptance pending.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (9). **0.2.0 failed U-5: no hemicycle. 0.2.2 repair installed,
not tested in game.** Report: `docs/report-dll-12-live-fix-1012.md`.
The user requested fewer iterations; keep one combined game session.

## 1. Decisions and current scope

TGC v2.5.2 only (D1); single player first (D5), read-only DLL (R5).
DLL-2 remains the user's alias for HEMI-1. Interface route A and the
100-seat/seven-row mockup are unchanged. D3 stays open and blocks nothing.
REA and the Victoria II reverse-engineering repo remain references for
later army templates; RE symbols/V2DLL were cross-checked in this repair.
No other mechanic was selected.

## 2. Next session: U-5 retry on the installed build

1. User opens TGC v2.5.2 + TGCX, UK 1836, paused Politics; wait 20 seconds.
2. Check the hemicycle and hover; send a screenshot. **Keep the game open**
   if the pie remains. The agent reads the new log and, if needed, performs
   read-only live inspection without another speculative replacement.
3. If the chart appears, complete view switching, save/reload and Alt-Tab
   in that same run using `docs/instructions-u-5.md`.
4. Preserve the log outside git. Record actual drawing/cadence, mapping,
   guards, errors and mockup differences in a new U-5 report. Mark tested
   only after game evidence. Consolidate any confirmed fixes before a rerun.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; prior DLL preserved `.off` | `docs/report-u-4-1012.md` |
| DLL-1 | 0.2.0 failed U-5; 0.2.2 repair installed, not tested in game | `docs/instructions-dll-1.md`; repair report |
| HEMI-1 (DLL-2 alias) | 0.2.0 chart absent; 0.2.2 installed, not tested in game | `docs/instructions-dll-2.md` |
| GH-2 | core, complete ideology mapping/vector bounds/colours and absolute root coordinates runtime-confirmed; chart acceptance pending | `docs/symbols.md` |
| U-5 | retry next; keep one session open for inspection | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | `docs/report-law-0-1012.md` |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA note retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity and gates

Installed on E: `lua51.dll` version 0.2.2, SHA-256
`c2ea4d11bd1b09069e6bad3740a55ced4539296e3ef50ce68f9ba50f0df2b365`.
Original and tested DLL-0 backup remain intact. Only the user starts the game.

Reference hash + 19 signatures, TGC scan, Release build, real D3D9 native
fixtures and eight installer/Lua tests passed. Final engine Present now
supplies cadence and drawing, using a separate scene and restored viewport.
Live 0.2.1 established a rejected CEU3Graphics table and the incorrect
double-centering of window coordinates. Both corrected in 0.2.2; do not
describe this build as game-verified yet. Latest pitfall P26, next P27.
No third-party art/data copied into git. No commit was requested.


---

# Session close 2026-10-12 (9): live renderer/position correction

Live read-only inspection identified CEU3Graphics and absolute root coordinates. Built/tested/installed 0.2.2; game acceptance remains pending. See `../report-dll-12-live-fix-1012.md`.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (8). **0.2.0 failed U-5: no hemicycle. 0.2.1 repair installed,
not tested in game.** Report: `docs/report-dll-12-fix-1012.md`.
The user requested fewer iterations; keep one combined game session.

## 1. Decisions and current scope

TGC v2.5.2 only (D1); single player first (D5), read-only DLL (R5).
DLL-2 remains the user's alias for HEMI-1. Interface route A and the
100-seat/seven-row mockup are unchanged. D3 stays open and blocks nothing.
REA and the Victoria II reverse-engineering repo remain references for
later army templates; RE symbols/V2DLL were cross-checked in this repair.
No other mechanic was selected.

## 2. Next session: U-5 retry on the installed build

1. User opens TGC v2.5.2 + TGCX, UK 1836, paused Politics; wait 20 seconds.
2. Check the hemicycle and hover; send a screenshot. **Keep the game open**
   if the pie remains. The agent reads the new log and, if needed, performs
   read-only live inspection without another speculative replacement.
3. If the chart appears, complete view switching, save/reload and Alt-Tab
   in that same run using `docs/instructions-u-5.md`.
4. Preserve the log outside git. Record actual drawing/cadence, mapping,
   guards, errors and mockup differences in a new U-5 report. Mark tested
   only after game evidence. Consolidate any confirmed fixes before a rerun.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; prior DLL preserved `.off` | `docs/report-u-4-1012.md` |
| DLL-1 | 0.2.0 failed U-5; 0.2.1 repair installed, not tested in game | `docs/instructions-dll-1.md`; repair report |
| HEMI-1 (DLL-2 alias) | 0.2.0 chart absent; 0.2.1 installed, not tested in game | `docs/instructions-dll-2.md` |
| GH-2 | core runtime-confirmed; full mapping/GUI verification pending | `docs/symbols.md` |
| U-5 | retry next; keep one session open for inspection | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | `docs/report-law-0-1012.md` |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA note retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity and gates

Installed on E: `lua51.dll` version 0.2.1, SHA-256
`5c339c2174393839a6943894b2cb49479686c706152c94f233dfb4594ce57356`.
Original and tested DLL-0 backup remain intact. Only the user starts the game.

Reference hash + 18 signatures, TGC scan, Release build, real D3D9 native
fixtures and eight installer/Lua tests passed. Final engine Present now
supplies cadence and drawing, using a separate scene and restored viewport.
The failure's exact live guard/callback cause remains unproven from 0.2.0's
log; do not describe 0.2.1 as game-verified. Latest pitfall P26, next P27.
No third-party art/data copied into git. No commit was requested.


---

# Session close 2026-10-12 (8): U-5 failure and DLL presentation repair

0.2.0 showed no hemicycle. Preserved the failed log; traced the engine final Present, moved rendering there, restored inherited viewport state, added bounded failure diagnostics, passed gates and installed 0.2.1. In-game verification remains pending. See `../report-dll-12-fix-1012.md`.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (7). **DLL-1 + HEMI-1 implemented, installed, not tested in game.**
The user's “DLL-2” request is implemented as HEMI-1; there was no separate
DLL-2 card. The user requested fewer iterations: one U-5 pass covers both.
Evidence: `docs/report-dll-12-1012.md`. Starting instructions and NEXT are
preserved in `docs/archive/`.

## 1. Current decisions and scope

D1: TGC v2.5.2 only. D5: single player first, multiplayer deferred.
D6: our DLL, attributed V2DLL patterns allowed for personal use.
D7/D11: favours and ideological affinity; no simulation change in this
session. D9: interface route A for the hemicycle. D10: English throughout.
D3 (name/prefix) remains open and blocks nothing here.
REA remains recorded for later army templates (`docs/rea-for-templates.md`);
it is not installed. No other mechanic was selected for this session.

## 2. Next session: one combined U-5 test

1. Follow `docs/instructions-u-5.md`: the agent already installed 0.2.0
   on E:, and the user starts TGC v2.5.2 + TGCX.
2. Check the UK 1836 hemicycle, hover, paused sampling, view switching,
   save/reload and Alt-Tab in one game session. Send the chart screenshot
   and the overall result; the agent reads logs locally.
3. Preserve `Logs/tgcx.log` outside git before another launch. Compare
   shares/engine mappings, frame/draw counters, visibility and errors.
   Compare the screenshot with `mockups/1-politics.png`; the current
   prototype's deviations are listed in the implementation report.
4. If this passes, mark DLL-1/HEMI-1 tested in game and write the U-5
   report. Fix concrete failures together before asking for another
   game run. Choose the next feature after this evidence, not a new
   diagnostic rectangle pass.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; prior DLL preserved as `.off` | `docs/report-u-4-1012.md` |
| GH-2 | core runtime-confirmed; vector end/name lookup/colours confirmed in code, U-5 verifies full live mapping | `docs/symbols.md`; `docs/report-dll-12-1012.md` |
| DLL-1 | done, not tested in game; renderer/cadence/visibility installed | `docs/instructions-dll-1.md` |
| HEMI-1 (DLL-2 alias) | done, not tested in game; 100 seats/seven rows/tooltips installed | `docs/instructions-dll-2.md` |
| U-5 | next; one batch tests both implemented cards | `docs/instructions-u-5.md` |
| LAW-0 / U-3 | done, tested in game | `docs/report-law-0-1012.md` |
| GOV-0 / GH-1 step 5 | available, not selected now | existing card instructions |
| GH-4 / TPL-1 | later; REA note retained | `docs/rea-for-templates.md` |
| COMPAT-0 | deferred (D1) | `docs/instructions-compat-0.md` |

## 3. Continuity and gates

Installed: `E:\Victoria.II.v3.04.Inclu.ALL.DLC\lua51.dll`, 0.2.0.
Original remains `lua51_real.dll`; tested 0.1.0 remains `lua51_tgcx.dll.off`.
Agent ran installation and verified hashes. Only the user launches the game.

```powershell
python tools/check_exe.py
python tools/tgc_scan.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC
powershell -NoProfile -ExecutionPolicy Bypass -File tools/build_dll.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File tools/test_dll.ps1
python tools/install_dll.py E:\Victoria.II.v3.04.Inclu.ALL.DLC --dry-run
```

Gate: reference SHA-256 + sixteen signatures; generated relocation table;
Release/Win32 v143 `/MT /W4 /WX`; real D3D9 and eight installer/Lua checks.
No worker threads or simulation writes; the old time-advance hook is not
installed in 0.2.0. Bounded diagnostics run on Present every ten real seconds.
No TGC/GUI files or third-party art copied. No commit without user request.
Latest pitfall P25; next P26. On close, archive this NEXT, update statuses
and write instructions for every selected next card (ROADMAP §7).


---

# Session log (archive)

> Every session, when closing, adds here **the `NEXT.md` it had** and
> **what was done** (rule: `ROADMAP.md` §7). Newest goes **on top**. What
> has been written is not edited.
>
> Translated to English on 2026-10-12 (D10). File and card names in the
> older entries were updated to the current English ones (`PROXIMO.md` →
> `NEXT.md`, `instrucciones-*` → `instructions-*`, LEY → LAW, GOB → GOV…)
> so the links still work; the content is otherwise unchanged.

---

## 2026-10-12 (7) — DLL-1 + HEMI-1 combined and installed

User requested DLL-1 and a second DLL stage with fewer test iterations.
There was no DLL-2 card; the second stage is HEMI-1 (hemicycle).
Replaced the time-advance sampling route with guarded D3D9 presentation,
implemented 100 seats/seven rows, engine-name index/colour lookup,
Politics lifetime/geometry guards and tooltips. Sixteen signature gates,
Win32 build, real D3D9 fixtures and eight installer/Lua checks passed.
Installed 0.2.0 on E:, preserved original and tested DLL-0.
Both implemented cards are **done, not tested in game**; one U-5 batch
covers both. No simulation writes, game launch, TGC changes or commit.
Report: `docs/report-dll-12-1012.md`; P25 records restricted tick cadence.

### NEXT.md at the start (verbatim)

```markdown
# NEXT — what comes next

2026-10-12 (6). **DLL-0 + U-4 done, tested in game; DLL-1 is next.**
Evidence: `docs/report-u-4-1012.md`; preserved log outside git:
`E:\v2re\logs\tgcx-u4-1012.log`. Earlier sessions: `docs/archive/sessions.md`.

## 1. User decisions

**Closed (2026-10-12):**

- **D1 — TGC v2.5.2 only** (revised after U-2; the first answer was "any
  version"). Compatibility with other versions is deferred, maybe for good.
  Nothing from TGC enters git, so the few fixed-name files we override are
  still generated from the installed v2.5.2 at install time.
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.
- **D7 — political favours and ideological affinity** (revised the same
  day: BPM's staged negotiation was archived). Each bloc in the chamber has
  a stance on each law according to its closeness to it; the projected
  vote is deterministic; favours buy abstentions and votes in steps of 5
  seats; whoever is strongly against can be promised a law (a debt with a
  deadline); the Crown signs or vetoes (`docs/law-design.md`). Interface
  mockup: `docs/README.md`.
- **D6 — our own DLL**; since it is for personal use, it may use V2DLL
  code, marked `// from V2DLL: <file>` (R8).
- **D4 — rulers and generals**, starting with those who existed in 1836.
- **D5 — multiplayer later.** The script is still written with nothing
  that would prevent it (R5).
- **D10 — everything in English** (code, docs, file names, in-game text,
  chat). The whole repository was translated on 2026-10-12.

- **D11 — favours** (2026-10-12, after law design v3): purchases are
  virtual and last only for the bill; favours accrue automatically like
  diplomatic points (the DLL sends the monthly tick; a decision is the
  fallback without it); the rate comes from prestige plus stability,
  militancy, consciousness, voting pop and monarch/leader support
  (`docs/law-design.md` §5). The Bill mockup was updated (R10).

**Open:** **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).
It blocks nothing until publishing; renaming later is a text replace.

**Tooling note (user, 2026-10-12):** keep REA in mind, especially for
army templates. `docs/rea-for-templates.md` records its GH-4 use case and
the Windows x86 package-support caveat. REA is not installed; it does not block DLL-1.

## 2. The next session, in order

1. **DLL-1 — callback cadence, drawing and Politics visibility.** Follow
   `docs/instructions-dll-1.md`. First investigate why U-4 retained only
   one January 2 snapshot despite reaching/reloading January 6; repeated
   sampling while paused/after reload is not established by that log.
2. Establish a guarded read-only D3D9 callback and a reliable Politics
   visibility/lifetime route. Draw only a small diagnostic rectangle
   within Politics, preserving CPU and renderer state. Do not implement
   the full hemicycle or write simulation state in this card.
3. Build/verify, write the DLL-1 report and exact U-5 instructions before
   handoff. U-5 is the user's next drawing test after DLL-1 is built.
4. Before **HEMI-1**, resolve the remaining ideology mapping and array
   length. U-4 confirms Conservative=1, Reactionary=2 and Liberal=6;
   the TGC file-order candidates are disproved. Sum 32767 is consistent
   with quantization, not a failed normalization check.
5. Optional independent cards when selected: **GOV-0** (data) and
   **GH-1 step 5** (constructors/sizes). REA evaluation belongs to later
   **GH-4/templates**. COMPAT-0 remains deferred by D1.

| card | status / what to do | instructions |
|---|---|---|
| SUB-0 / U-2 | **done**, tested in game | `docs/report-sub-0-1012.md` |
| LAW-0 / U-3 | **done**, tested in game; normal five-year debts installed | `docs/report-law-0-1012.md` |
| DLL-0 / U-4 | **done**, tested in game; installed on E:\ | `docs/report-u-4-1012.md`; `docs/instructions-u-4.md` |
| GH-2 | **core layout/data confirmed at runtime**; complete mapping/length still pending | `docs/symbols.md`; `docs/report-u-4-1012.md` |
| DLL-1 | **next**, ready; no implementation yet | `docs/instructions-dll-1.md` |
| U-5 | follows DLL-1 build; write exact instructions before its handoff | `docs/instructions-dll-1.md` step 5 |
| HEMI-1 | waits for DLL-1/U-5 and complete ideology mapping | `docs/technical-plan.md` §4 |
| GOV-0 | ready, data only; bulk downloads need count/size approval | `docs/instructions-gov-0.md` |
| GH-1 | steps 1-4 done; step 5 missing | `docs/instructions-gh-1.md` |
| GH-4 / TPL-1 | later; REA candidate noted, not installed | `docs/rea-for-templates.md`; `docs/technical-plan.md` §6 |
| COMPAT-0 | **deferred**, v2.5.2 only | `docs/instructions-compat-0.md` |

## 3. Gates and continuity

Before addresses: `python tools/check_exe.py` → `ALL OK`
(SHA-256 + eight signatures). Before naming TGC things:
`python tools/tgc_scan.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/build_dll.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File tools/test_dll.ps1
python tools/install_dll.py E:\Victoria.II.v3.04.Inclu.ALL.DLC --dry-run
```

The verified DLL-0 proxy remains installed. Preserve it before installing
a later build, and close the game before installer mutations. `--remove`
restores the original and keeps the proxy under an `.off` name. The DLL
log is replaced each startup: preserve test evidence outside git.

Ghidra: one writer at a time in `E:\v2re\ghidra`. No game/TGC binaries,
art, projects, save files or REA snapshots enter git. No commit unless
the user asks. Latest pitfall P24; next number P25.

On close: archive the starting NEXT.md, rewrite it, write current
instructions for each next selected card, update ROADMAP/SUBAGENTS
(`ROADMAP.md` §7). DLL-1 instructions now exist; U-5 is prepared during
DLL-1 once its concrete build/test behavior is known.

```

---

## 2026-10-12 (6) — U-4 passed; DLL-1 prepared

User reported every test step worked and supplied four Politics screens.
Read and preserved the DLL log outside git; all startup gates passed at
base 0x007D0000. Raw values match 83/5/12% after rounding, sum 32767;
runtime indices Conservative=1, Reactionary=2, Liberal=6 confirmed.
Empty game error logs and game.log corroborate a January 6 reload.
DLL-0 is done, tested in game. GH-2 core layout/data is R; full mapping
and allocation length remain unresolved. Only one snapshot was retained,
so DLL-1 must verify callback cadence before relying on a frame service.
Wrote `docs/report-u-4-1012.md` and `docs/instructions-dll-1.md`.
No code or installed DLL was changed, no game launched, no commit made.

### NEXT.md at the start (verbatim)

# NEXT — what comes next

2026-10-12 (5). **DLL-0 done, not tested in game; U-4 is next.**
Implementation: `docs/report-dll-0-1012.md`. Installed on E:\ at the user's
request; hashes verified in `docs/report-dll-0-install-1012.md`.
Earlier sessions: `docs/archive/sessions.md`.

## 1. User decisions

**Closed (2026-10-12):**

- **D1 — TGC v2.5.2 only** (revised after U-2; the first answer was "any
  version"). Compatibility with other versions is deferred, maybe for good.
  Nothing from TGC enters git, so the few fixed-name files we override are
  still generated from the installed v2.5.2 at install time.
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.
- **D7 — political favours and ideological affinity** (revised the same
  day: BPM's staged negotiation was archived). Each bloc in the chamber has
  a stance on each law according to its closeness to it; the projected
  vote is deterministic; favours buy abstentions and votes in steps of 5
  seats; whoever is strongly against can be promised a law (a debt with a
  deadline); the Crown signs or vetoes (`docs/law-design.md`). Interface
  mockup: `docs/README.md`.
- **D6 — our own DLL**; since it is for personal use, it may use V2DLL
  code, marked `// from V2DLL: <file>` (R8).
- **D4 — rulers and generals**, starting with those who existed in 1836.
- **D5 — multiplayer later.** The script is still written with nothing
  that would prevent it (R5).
- **D10 — everything in English** (code, docs, file names, in-game text,
  chat). The whole repository was translated on 2026-10-12.

- **D11 — favours** (2026-10-12, after law design v3): purchases are
  virtual and last only for the bill; favours accrue automatically like
  diplomatic points (the DLL sends the monthly tick; a decision is the
  fallback without it); the rate comes from prestige plus stability,
  militancy, consciousness, voting pop and monarch/leader support
  (`docs/law-design.md` §5). The Bill mockup was updated (R10).

**Open:** **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).
It blocks nothing until publishing; renaming later is a text replace.

**Tooling note (user, 2026-10-12):** keep REA in mind, especially for
army templates. `docs/rea-for-templates.md` records its GH-4 use case and
the Windows x86 package-support caveat. Not installed; does not block U-4.

## 2. The next session, in order

1. **U-4 — user tests DLL-0.** Follow `docs/instructions-u-4.md`: install
   (already completed), check the main-menu log, start the 1836 United
   Kingdom with TGC v2.5.2 + TGCX, compare upper-house percentages, run a
   few days and save/load. Provide `Logs\tgcx.log` and the chart values.
2. **Review U-4 evidence.** Compare raw indices, rounding, the sum and
   player identity. Write a new dated U-4 report. If anything fails,
   repair DLL-0 and repeat U-4 before progressing. GH-2 is not yet
   runtime-confirmed; zero-valued ideologies and array length require
   additional evidence before claiming a complete mapping.
3. After U-4 passes, prepare the next interface card (**DLL-1**) and its
   instructions. It has not started; do not treat automatic fixture tests
   as an in-game pass.
4. Optional independent cards, when selected: **GOV-0** (data, no RE) and
   **GH-1 step 5** (constructors/sizes). Current instructions exist.
   COMPAT-0 remains deferred by D1. Evaluate REA when GH-4 is selected.

| card | status / what to do | instructions |
|---|---|---|
| SUB-0 / U-2 | **done**, tested in game | `docs/report-sub-0-1012.md` |
| LAW-0 / U-3 | **done**, tested in game; normal five-year debts installed | `docs/report-law-0-1012.md` |
| DLL-0 | **done, not tested in game**; Release/Win32 and automatic gates passed; installed on E:\ | `docs/instructions-dll-0.md`; `docs/report-dll-0-1012.md` |
| U-4 | **next**: install, main-menu check, chart/log comparison, save/load | `docs/instructions-u-4.md` |
| GH-2 | code evidence C/I; runtime confirmation awaits U-4 | `docs/symbols.md`; `docs/instructions-u-4.md` §4 |
| DLL-1 | waits for U-4; instructions to be written when selected | `docs/technical-plan.md` §1 |
| GOV-0 | ready, data only; bulk portrait downloads need count/size approval | `docs/instructions-gov-0.md` |
| GH-1 | steps 1-4 done; step 5 missing | `docs/instructions-gh-1.md` |
| GH-4 / TPL-1 | later; REA tooling note recorded, not installed | `docs/rea-for-templates.md`; `docs/technical-plan.md` §6 |
| COMPAT-0 | **deferred**, v2.5.2 only | `docs/instructions-compat-0.md` |
| ECO-A | fallback charts from saves, if selected | `docs/technical-plan.md` §3 |

## 3. Gates, installation and continuity

Before touching addresses: `python tools/check_exe.py` → `ALL OK`
(SHA-256 + eight signatures, nine checks). Before naming TGC things:
`python tools/tgc_scan.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/build_dll.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File tools/test_dll.ps1
python tools/install_dll.py E:\Victoria.II.v3.04.Inclu.ALL.DLC --dry-run
```

Installation is complete; U-4 starts at the main-menu check (§2).
The same installer with `--remove` restores the
original and retains the proxy under an `.off` name. Close the game first.
The log is replaced each startup; preserve useful U-4 logs outside git.

Ghidra: one writer at a time in `E:\v2re\ghidra`. Nothing from the game,
TGC, Ghidra projects or REA snapshots/caches enters git. No commit is
made unless the user asks. New pitfall: P24; the next number is P25.

On close, archive the starting NEXT.md, rewrite it, write current
instructions for the next selected cards and update ROADMAP/SUBAGENTS
(`ROADMAP.md` §7). U-4 is the next selected card; its instructions exist.

---

## 2026-10-12 (5) — DLL-0 installed at the user's request

Ran the installer on E:\Victoria.II.v3.04.Inclu.ALL.DLC. Verified the
active proxy and preserved original hashes and the installation record.
No game launch; U-4 starts at its main-menu check. Current U-4 instructions
already exist. See `docs/report-dll-0-install-1012.md`.

### NEXT.md at the start (verbatim)

# NEXT — what comes next

2026-10-12 (4). **DLL-0 done, not tested in game; U-4 is next.**
Implementation and measurements: `docs/report-dll-0-1012.md`.
Earlier sessions: `docs/archive/sessions.md`.

## 1. User decisions

**Closed (2026-10-12):**

- **D1 — TGC v2.5.2 only** (revised after U-2; the first answer was "any
  version"). Compatibility with other versions is deferred, maybe for good.
  Nothing from TGC enters git, so the few fixed-name files we override are
  still generated from the installed v2.5.2 at install time.
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.
- **D7 — political favours and ideological affinity** (revised the same
  day: BPM's staged negotiation was archived). Each bloc in the chamber has
  a stance on each law according to its closeness to it; the projected
  vote is deterministic; favours buy abstentions and votes in steps of 5
  seats; whoever is strongly against can be promised a law (a debt with a
  deadline); the Crown signs or vetoes (`docs/law-design.md`). Interface
  mockup: `docs/README.md`.
- **D6 — our own DLL**; since it is for personal use, it may use V2DLL
  code, marked `// from V2DLL: <file>` (R8).
- **D4 — rulers and generals**, starting with those who existed in 1836.
- **D5 — multiplayer later.** The script is still written with nothing
  that would prevent it (R5).
- **D10 — everything in English** (code, docs, file names, in-game text,
  chat). The whole repository was translated on 2026-10-12.

- **D11 — favours** (2026-10-12, after law design v3): purchases are
  virtual and last only for the bill; favours accrue automatically like
  diplomatic points (the DLL sends the monthly tick; a decision is the
  fallback without it); the rate comes from prestige plus stability,
  militancy, consciousness, voting pop and monarch/leader support
  (`docs/law-design.md` §5). The Bill mockup was updated (R10).

**Open:** **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).
It blocks nothing until publishing; renaming later is a text replace.

**Tooling note (user, 2026-10-12):** keep REA in mind, especially for
army templates. `docs/rea-for-templates.md` records its GH-4 use case and
the Windows x86 package-support caveat. Not installed; does not block U-4.

## 2. The next session, in order

1. **U-4 — user tests DLL-0.** Follow `docs/instructions-u-4.md`: install
   with the game closed, check the main-menu log, start the 1836 United
   Kingdom with TGC v2.5.2 + TGCX, compare upper-house percentages, run a
   few days and save/load. Provide `Logs\tgcx.log` and the chart values.
2. **Review U-4 evidence.** Compare raw indices, rounding, the sum and
   player identity. Write a new dated U-4 report. If anything fails,
   repair DLL-0 and repeat U-4 before progressing. GH-2 is not yet
   runtime-confirmed; zero-valued ideologies and array length require
   additional evidence before claiming a complete mapping.
3. After U-4 passes, prepare the next interface card (**DLL-1**) and its
   instructions. It has not started; do not treat automatic fixture tests
   as an in-game pass.
4. Optional independent cards, when selected: **GOV-0** (data, no RE) and
   **GH-1 step 5** (constructors/sizes). Current instructions exist.
   COMPAT-0 remains deferred by D1. Evaluate REA when GH-4 is selected.

| card | status / what to do | instructions |
|---|---|---|
| SUB-0 / U-2 | **done**, tested in game | `docs/report-sub-0-1012.md` |
| LAW-0 / U-3 | **done**, tested in game; normal five-year debts installed | `docs/report-law-0-1012.md` |
| DLL-0 | **done, not tested in game**; Release/Win32 and automatic gates passed; not installed | `docs/instructions-dll-0.md`; `docs/report-dll-0-1012.md` |
| U-4 | **next**: install, main-menu check, chart/log comparison, save/load | `docs/instructions-u-4.md` |
| GH-2 | code evidence C/I; runtime confirmation awaits U-4 | `docs/symbols.md`; `docs/instructions-u-4.md` §4 |
| DLL-1 | waits for U-4; instructions to be written when selected | `docs/technical-plan.md` §1 |
| GOV-0 | ready, data only; bulk portrait downloads need count/size approval | `docs/instructions-gov-0.md` |
| GH-1 | steps 1-4 done; step 5 missing | `docs/instructions-gh-1.md` |
| GH-4 / TPL-1 | later; REA tooling note recorded, not installed | `docs/rea-for-templates.md`; `docs/technical-plan.md` §6 |
| COMPAT-0 | **deferred**, v2.5.2 only | `docs/instructions-compat-0.md` |
| ECO-A | fallback charts from saves, if selected | `docs/technical-plan.md` §3 |

## 3. Gates, installation and continuity

Before touching addresses: `python tools/check_exe.py` → `ALL OK`
(SHA-256 + eight signatures, nine checks). Before naming TGC things:
`python tools/tgc_scan.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/build_dll.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File tools/test_dll.ps1
python tools/install_dll.py E:\Victoria.II.v3.04.Inclu.ALL.DLC --dry-run
```

U-4 runs the same installer without `--dry-run`; `--remove` restores the
original and retains the proxy under an `.off` name. Close the game first.
The log is replaced each startup; preserve useful U-4 logs outside git.

Ghidra: one writer at a time in `E:\v2re\ghidra`. Nothing from the game,
TGC, Ghidra projects or REA snapshots/caches enters git. No commit is
made unless the user asks. New pitfall: P24; the next number is P25.

On close, archive the starting NEXT.md, rewrite it, write current
instructions for the next selected cards and update ROADMAP/SUBAGENTS
(`ROADMAP.md` §7). U-4 is the next selected card; its instructions exist.

---

## 2026-10-12 (4) — DLL-0 implementation, U-4 pending

Built DLL-0 (Release/Win32, v143, /MT) and the reversible installer.
Shared signatures account for ASLR relocations (P24); native fixtures,
eight installer/real-Lua tests, executable/TGC gates and the installer
dry run passed. No game files changed and no game was launched. DLL-0 is
done, not tested in game; GH-2 remains C/I. U-4 instructions were written.
The user's REA suggestion was reviewed and recorded for GH-4/templates;
REA was not installed. No commit was made.

### NEXT.md at the start (verbatim)

# NEXT — what comes next

2026-10-12 (3). **U-2 passed; D1 revised to TGC v2.5.2 only; law design v3.**
Earlier work (bootstrap, Ghidra, upper house at `CCountry+0x8B0`) is in
`docs/archive/sessions.md` and `docs/report-initial-1012.md`.

## 1. User decisions

**Closed (2026-10-12):**

- **D1 — TGC v2.5.2 only** (revised after U-2; the first answer was "any
  version"). Compatibility with other versions is deferred, maybe for good.
  Nothing from TGC enters git, so the few fixed-name files we override are
  still generated from the installed v2.5.2 at install time.
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.
- **D7 — political favours and ideological affinity** (revised the same
  day: BPM's staged negotiation was archived). Each bloc in the chamber has
  a stance on each law according to its closeness to it; the projected
  vote is deterministic; favours buy abstentions and votes in steps of 5
  seats; whoever is strongly against can be promised a law (a debt with a
  deadline); the Crown signs or vetoes (`docs/law-design.md`). Interface
  mockup: `docs/README.md`.
- **D6 — our own DLL**; since it is for personal use, it may use V2DLL
  code, marked `// from V2DLL: <file>` (R8).
- **D4 — rulers and generals**, starting with those who existed in 1836.
- **D5 — multiplayer later.** The script is still written with nothing
  that would prevent it (R5).
- **D10 — everything in English** (code, docs, file names, in-game text,
  chat). The whole repository was translated on 2026-10-12.

- **D11 — favours** (2026-10-12, after law design v3): purchases are
  virtual and last only for the bill; favours accrue automatically like
  diplomatic points (the DLL sends the monthly tick; a decision is the
  fallback without it); the rate comes from prestige plus stability,
  militancy, consciousness, voting pop and monarch/leader support
  (`docs/law-design.md` §5). The Bill mockup was updated (R10).

**Open:** **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).
It blocks nothing until publishing; renaming later is a text replace.

## 2. The next session, in order

1. ~~SUB-0 + U-2~~ **done** (2026-10-12): loads on v2.5.2 and v2.3.4,
   `000_tgcx_` override wins, one event per click, logs clean.
2. ~~LAW-0~~ **done** (2026-10-12,
   `docs/report-law-0-1012.md`): the press-rights bill generated by
   `tools/gen_laws.py`, installed; the simulation of the 1836 United
   Kingdom passes. **U-3 done** (the user, 2026-10-12: "everything done. it
   all works"; veto, pass, 25-seat lots and a broken promise in game;
   P22, P23). **LAW-0 done.** The normal build (5-year debts) is installed
   again.
3. **DLL-0** (started 2026-10-12: research only, handoff at the end of
   `docs/instructions-dll-0.md`) — our own proxy that logs signatures and the upper house, and
   **U-4** (the user installs it and sends the log): confirms GH-2.
4. In parallel, when there is time: **GOV-0** (data, no RE),
   **GH-1 step 5**. (COMPAT-0 deferred by D1.)

| card | status / what to do | instructions |
|---|---|---|
| SUB-0 | **done** (tested in U-2) | `docs/instructions-sub-0.md`; `docs/report-sub-0-1012.md` |
| U-2 | **done** (2026-10-12): v2.5.2 and v2.3.4 load (`TGC252_TGCX`, `TGC_TGCX`; `error.log` empty), "[TGCX] Denmark" (P20), one event per click with the right version (P21 fix confirmed) | `docs/report-sub-0-1012.md` §4; `docs/scripting-reference.md` |
| LAW-0 | **done** (U-3 passed): the bill, purchases, promise, audience, vote, favours tick (D11), generated `event_modifiers.txt`; `tools/test_gen_laws.py` ALL OK; report `docs/report-law-0-1012.md` | `docs/instructions-law-0.md` (v3); `docs/law-design.md` (v3) |
| U-3 | **done** (2026-10-12): every point of step 6, screenshots in `mockups\u3\` (not in git); P22, P23 | `docs/instructions-law-0.md` step 6; `docs/report-law-0-1012.md` addenda |
| DLL-0 | `lua51.dll` proxy, Win32 `v143`, log of signatures and upper house | `docs/instructions-dll-0.md` |
| U-4 | the user installs DLL-0 and sends `Logs\tgcx.log` | `docs/instructions-dll-0.md` step 5 |
| GOV-0 | 1836 rulers and generals' portraits: data only | `docs/instructions-gov-0.md` |
| GH-1 | steps 1-4 done; step 5 missing (constructors, sizes) | `docs/instructions-gh-1.md` |
| GH-2 | read in the code; U-4 confirms it | `docs/symbols.md`, "Missing" |
| COMPAT-0 | **deferred** (D1 revised) | `docs/instructions-compat-0.md` |
| ECO-A | charts from save files (no RE): the fallback if the user cannot test in game | `docs/technical-plan.md` §3 |

## 3. Safety net and continuity

Before touching addresses: `python tools/check_exe.py` (it must print
`ALL OK`; 7 checks: hash + 6 signatures, 2 of them for the upper house).
Before touching anything that names TGC things:
`python tools/tgc_scan.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`
(the target, D1).

Ghidra: one writer at a time in `E:\v2re\ghidra`. Nothing from `E:\v2re`,
the game or TGC enters git. The repository has no commits yet: the first
one when the user asks. Bulk downloads (GOV-0 portraits): ask first, with
count and size.

On close: archive this file in `docs/archive/sessions.md` and write the
instructions for the next session's cards (`ROADMAP.md` §7).

---

## 2026-10-12 (2) — Decisions D4-D7 and staged law design

### The `NEXT.md` we had (verbatim)

### NEXT — what comes next

2026-10-12. **Bootstrap done (I-0).** TGC v2.5.2 (stable) cloned and
installed next to the user's TGC without touching it; Ghidra 12.1.4 with
`v2game.exe` analysed and the RE repo's symbols imported; V2DLL as a
reference; gates `tools/check_exe.py` and `tools/tgc_scan.py`; documents.
**At the end, a preview of GH-1 and GH-2:** RTTI inventory exported
(`docs/rtti-classes.md`, 1 500 vtables) and **the upper house found in the
code**: `CCountry+0x8B0`, `int64[]` by ideology at `+0x10`
(`docs/symbols.md`). Report: `docs/report-initial-1012.md`.

#### 1. User decisions

**Closed (2026-10-12):**

- **D1 — compatible with any TGC version.** Nothing copied from TGC;
  whatever has to be overridden is generated from the installed version
  (`docs/tgc-compatibility.md`).
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.

**Open (U-1), in order of urgency:**

1. **D7 — what "negotiating a law" means**: A (buy support), B
   (concessions) or C (staged negotiation); `docs/technical-plan.md` §2.
   Blocks LAW-0, the first visible mechanic.
2. **D6 — DLL base**: a V2DLL fork, our own chained with V2DLL, or our own
   without V2DLL. V2DLL has no published licence (P13).
3. **D4 — "leaders"**: new rulers, the generals that already exist, or
   both.
4. **D5 — multiplayer** from the start or single player first.
5. **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).

#### 2. The next session, in order

1. **U-1**: ask the §1 decisions (those the user has not answered).
2. **GH-1 step 5** — constructors and sizes of the mechanics' classes
   (steps 1-4 done on 2026-10-12).
3. **SUB-0** — submod skeleton and load test (with **U-2**: the user starts
   the game). It decides whether the compatibility design works in
   practice (P10).
4. **COMPAT-0** — version matrix.
5. Depending on D7: instructions for **LAW-0**. Depending on D6: for
   **DLL-0**.

| card | status / what to do | instructions |
|---|---|---|
| U-1 | decisions D3-D7 | §1 |
| GH-1 | steps 1-4 done; step 5 missing (constructors, sizes) | `docs/instructions-gh-1.md` |
| GH-2 | read in the code; confirm with the game running (the user's Cheat Engine or DLL-0's log) | `docs/symbols.md`, "Missing" |
| SUB-0 | minimal submod, reversible installer, who wins when a key is overridden | `docs/instructions-sub-0.md` |
| U-2 | the user tests SUB-0 with v2.3.4 and v2.5.2 | `docs/instructions-sub-0.md` §5 |
| COMPAT-0 | `master` + old versions, `tgc_scan.py --matrix` | `docs/instructions-compat-0.md` |
| ECO-A | charts from save files (no RE, no pending decisions): the fallback if the user cannot test in game | `docs/technical-plan.md` §3 |

#### 3. Safety net and continuity

Before touching addresses: `python tools/check_exe.py` (it must print
`ALL OK`). Before touching anything that names TGC things:
`python tools/tgc_scan.py --diff E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`
(today: only `core.gfx`, `news.gui`, `topbar.gui` differ; anchors and
range OK). `check_exe.py` has 7 checks (hash + 6 signatures, 2 of them for
the upper house).

Ghidra: one writer at a time in `E:\v2re\ghidra`. Nothing from `E:\v2re`,
the game or TGC enters git (R8). The repository has no commits yet: the
first one when the user asks.

On close: archive this file in `docs/archive/sessions.md` and write the
instructions for the next session's cards (`ROADMAP.md` §7).

### What was asked

"Staged negotiation like in Victoria 3 mods such as Better Politics Mod;
our own DLL, but since it is for my own use you can use the other one too;
both [rulers and generals], but focusing on the ones there were in 1836;
MP later."

### What was done

- D4, D5, D6 and D7 closed in `AGENTS.md` §9, `ROADMAP.md` §3,
  `docs/technical-plan.md`. R8 allows marked V2DLL code; P13 resolved for
  personal use.
- Research: Vic3 dev diary 80 (enactment phases and outcomes) and Better
  Politics Mod (projected vote, the ruler's veto, manoeuvres). Nothing
  copied.
- `docs/law-design.md`: the design translated into what Vic2 script can do
  (no `if`, fixed weights in `random_list`, generated support steps).
- Instructions for LAW-0, DLL-0 and GOV-0.
- Measured in TGC v2.5.2: 297 historical leaders, 75 with `picture` (44 of
  their own); 223 portraits (45 of its own); no rulers.
- New pitfalls: P16 (there is no `if`), P17 (V2DLL asks for toolset
  `v145`; here there is `v143`).

---

## 2026-10-12 — Bootstrap: feasibility, downloads, Ghidra and documents

There was no `NEXT.md`: it is the first session.

### What was asked

1. Can rulers with portraits, a hemicycle, law negotiation, EU4-style army
   templates and more economy charts be added to Victoria II, starting
   from `maxioten/Victoria2-Reverse-Engineering`, on top of HPM? Or is a
   more extensive decompilation needed?
2. Would it be easier to decompile the game with Ghidra first? Integrate it
   with the latest TGC version.
3. Download stable TGC and everything needed; a submod compatible with any
   version; documents following the logic of the SMW-to-Amiga port's
   (`C:\Users\JC\Downloads\sma\port-amiga`).

### What was answered and done

- Feasibility: the RE repo covers economy, AI, events, technology; **not**
  GUI, upper house, reforms, leaders or recruitment. A full decompilation
  is not needed, but RE per subsystem is. The practical base is the V2DLL
  technique (a `lua51.dll` proxy). Five V2DLL signatures match the user's
  `v2game.exe`.
- Ghidra does help: 1 165 named RTTI classes, among them each mechanic's
  (`CConstructUnitCommand`, `CLeader`, `CIssueChange`, `CGuiObject`…).
- TGC: the user's is v2.3.4; latest stable v2.5.2; `master` is still
  active. The user chose TGC as the base (D2) and compatibility with any
  version (D1).
- Downloaded and installed: TGC v2.5.2 (`mod\TGC_v2.5.2`, its own `.mod`),
  Ghidra 12.1.4, V2DLL. Ghidra analysis with the repo's symbols.
- Tools: `check_exe.py`, `tgc_scan.py`, `ImportV2Symbols.java`,
  `ExportRtti.java`.
- Documents: `AGENTS.md`, `NEXT.md`, `ROADMAP.md`, `SUBAGENTS.md`, `docs/`
  (index, technical plan, compatibility, pitfalls, three instruction files,
  report, symbols, RTTI inventory). No `CLAUDE.md`: the user asked for
  `AGENTS.md` to replace it.
- Ghidra: analysis 752 s; 260/260 symbols; 1 500 vtables. Preview of GH-2:
  upper house at `CCountry+0x8B0` (read in the code, not run).

Report: `docs/report-initial-1012.md`.
