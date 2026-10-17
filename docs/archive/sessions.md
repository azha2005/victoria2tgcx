# Session 2026-10-17 (28) - External research and TPL-0 placement repair

Discord Cheat Engine material and smedley_kernel checked against our build
(`report-research-discord-1017.md`). TPL-0 placement traced to a
parent-relative build-root matrix (P40); 0.4.1 composes the owner chain,
translated-parent fixture added, gates pass, installed on E: with the 0.4.0
DLL retained. Not tested in game. See `report-tpl-0-placement-1017.md`.

## NEXT at session start (verbatim)

# NEXT - what comes next

2026-10-17 (27). **TPL-0 0.4.0 functional prototype accepted in game:**
4 infantry + 4 artillery + 1 hussar recruited; queue and template survive
the user's save/reload. **GUI placement/fidelity remain open.** Read
`docs/report-tpl-0-runtime-1017.md`. No new build or installation.

## Current baseline

- The Military screen's large Build Army view has a final Templates tab,
  template list, New/Delete, composition editor and Recruit. The native
  Being built queue remains. Eleven land types; eight editor rows/page;
  four templates/page, twelve local slots, 128 regiments maximum.
- Local preferences use composition names and Windows Documents under
  `Paradox Interactive/Victoria II/TGC252_TGCX/tgcx_templates.txt` (D12).
  Editing writes atomically; invalid files remain untouched.
- Recruiting only sends CConstructUnitCommand through the engine queue;
  it recruits what fits and reports orders queued/left out. A further
  template batch waits for game time to advance.
- 91 runtime signatures, native/reference-window/GPU fixtures and 8 DLL
  installer/Lua tests pass. Four new Templates GUI/installer tests pass.
  Existing law gates pass (27 catalogue, 8 Bill GUI, 10 submod installer,
  LAW-0, executable and TGC scan).
- All pre-existing installed submod files are byte-identical to 0.3.5.
  U-8 now confirms the nine-regiment queue and template after save/reload;
  proxy command counts match and copied error/system-error logs are empty.
  LAW-2 0.3.5 still needs U-7; the 0.3.4 look was accepted. HEMI-3's accepted
  baseline remains. Static layout previews are not game evidence.
- Only the user launches/exits the game. Preserve uncommitted changes.
  No commit, push or subagent unless asked. The next pitfall is P40.

## Next session in order

1. **TPL-0 visual repair**: `docs/instructions-tpl-0.md`. The screenshot
   shows the promoted panel detached at the upper left, title on the top
   bar, original build list still visible. Verify the nested Military/build
   coordinate hierarchy; do not treat a local matrix as screen coordinates.
   Extend regression coverage with a translated parent, then match board 5.
2. **U-8 follow-up**: `docs/instructions-u-8.md`. Recruitment and save/reload
   passed for nine regiments. Obtain the corrected placement screenshot;
   check the remaining partial-capacity, paused-repeat, close/tab/naval,
   single-regiment and fresh-process cases without repeating accepted work.
3. **U-7 / LAW-2**, still pending: use their existing instructions for the
   short Reforms/buttons/Decisions/monthly-popup check. Prioritize a real
   LAW-2 regression if one appears; do not mark it accepted from the
   Templates test.
4. **TPL-1**, after TPL-0 acceptance: follow
   `docs/instructions-tpl-1.md`. Read-only research of the stack panel and
   move/merge/detach/transfer commands, then the Conform Changes planner.
   D12 already chooses per-click template selection, missing regiments
   built/marched/merged and extras detached. Save/reload tracking still
   needs a verified design; no direct simulation writes.
5. Close under ROADMAP section 7: report, archive/revise NEXT, current
   step-by-step instructions for each next card and updated statuses.

## Evidence and rollback

Private root: `E:/v2re/logs/tgcx-tpl-panel-1017/`.

| What | Where / value |
|---|---|
| Staged source | `stage/` |
| Rollback 0.3.5 DLL | `before-dll/lua51.dll`, SHA `bec50bb7...0bbc7` |
| Complete rollback submod source | `before-submod/` |
| Installed 0.4.0 DLL SHA-256 | `2dcbf718950554df5a3cd7dffe14c471ea10c9d591d6957a07875bf1af939d1b` |
| Generated buildunit GUI SHA-256 | `9e3d23ab253c78cf7c8534ecfa148a21bb429dc6c74002d986bbdeb52bd2e47b` |
| Accepted functional evidence | `work/tgcx-u8-1017/`: original screenshots, log, preferences and manifest |
| Static visual QA | `tgcx_tpl_preview.png`, `tgcx_tpl_comparison.png`; not game evidence |

Rollback both the DLL and submod together, after user exit; exact commands
are in U-8. Original Lua remains verified as `lua51_real.dll`.

| Card | Status | Instructions/evidence |
|---|---|---|
| TPL-0 / GH-4 | 0.4.0 functional prototype accepted; GUI repair open | instructions-tpl-0.md; report-tpl-0-runtime-1017.md |
| U-8 | nine-regiment recruit/save-reload accepted; visual/extended checks open | instructions-u-8.md |
| TPL-1 | D12 approved; research/implementation queued after U-8 | instructions-tpl-1.md |
| LAW-2 / U-7 | 0.3.5 controls/reforms pending game check | existing instructions/reports |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |

# Session 2026-10-17 (27) - TPL-0 functional checkpoint

User confirms the nine-regiment prototype works and the second screenshot
is after save/reload. Screenshots/logs/preferences preserved privately.
Recruitment and persistence accepted; detached panel/top-bar overlap remain
visual work. No build/install/game control. See report-tpl-0-runtime-1017.md.

## NEXT at session start (verbatim)

# NEXT - what comes next

2026-10-17 (26). **TPL-0 0.4.0 is installed on E:: done, not tested in
game.** The native Templates panel, local library and Recruit button are
wired. Read `docs/report-tpl-0-panel-1017.md` and `docs/instructions-u-8.md`.

## Current baseline

- The Military screen's large Build Army view has a final Templates tab,
  template list, New/Delete, composition editor and Recruit. The native
  Being built queue remains. Eleven land types; eight editor rows/page;
  four templates/page, twelve local slots, 128 regiments maximum.
- Local preferences use composition names and Windows Documents under
  `Paradox Interactive/Victoria II/TGC252_TGCX/tgcx_templates.txt` (D12).
  Editing writes atomically; invalid files remain untouched.
- Recruiting only sends CConstructUnitCommand through the engine queue;
  it recruits what fits and reports orders queued/left out. A further
  template batch waits for game time to advance.
- 91 runtime signatures, native/reference-window/GPU fixtures and 8 DLL
  installer/Lua tests pass. Four new Templates GUI/installer tests pass.
  Existing law gates pass (27 catalogue, 8 Bill GUI, 10 submod installer,
  LAW-0, executable and TGC scan).
- All pre-existing installed submod files are byte-identical to 0.3.5.
  LAW-2 0.3.5 still needs U-7; the 0.3.4 look was accepted. HEMI-3's accepted
  baseline remains. Static layout previews are not game evidence.
- Only the user launches/exits the game. Preserve uncommitted changes.
  No commit, push or subagent unless asked. The next pitfall is P40.

## Next session in order

1. **U-8 (the user)**: `docs/instructions-u-8.md`. Recruit one infantry,
   then 4 infantry + 1 artillery + 1 hussar; unpause and verify actual
   queue entries. Check closing/native tabs/naval mode and save/reload.
   Provide a screenshot with the Templates panel and queue.
2. **TPL-0 coordinator**: `docs/instructions-tpl-0.md`. Preserve current
   logs/screenshots, compare with board 5 and investigate every new error,
   rejected command or visual difference. Fix and rerun affected gates;
   install only after a normal user exit. Runtime acceptance stays pending
   until the user confirms it. Do not infer success from the sender log.
3. **U-7 / LAW-2**, still pending: use their existing instructions for the
   short Reforms/buttons/Decisions/monthly-popup check. Prioritize a real
   LAW-2 regression if one appears; do not mark it accepted from the
   Templates test.
4. **TPL-1**, after TPL-0 acceptance: follow
   `docs/instructions-tpl-1.md`. Read-only research of the stack panel and
   move/merge/detach/transfer commands, then the Conform Changes planner.
   D12 already chooses per-click template selection, missing regiments
   built/marched/merged and extras detached. Save/reload tracking still
   needs a verified design; no direct simulation writes.
5. Close under ROADMAP section 7: report, archive/revise NEXT, current
   step-by-step instructions for each next card and updated statuses.

## Evidence and rollback

Private root: `E:/v2re/logs/tgcx-tpl-panel-1017/`.

| What | Where / value |
|---|---|
| Staged source | `stage/` |
| Rollback 0.3.5 DLL | `before-dll/lua51.dll`, SHA `bec50bb7...0bbc7` |
| Complete rollback submod source | `before-submod/` |
| Installed 0.4.0 DLL SHA-256 | `2dcbf718950554df5a3cd7dffe14c471ea10c9d591d6957a07875bf1af939d1b` |
| Generated buildunit GUI SHA-256 | `9e3d23ab253c78cf7c8534ecfa148a21bb429dc6c74002d986bbdeb52bd2e47b` |
| Static visual QA | `tgcx_tpl_preview.png`, `tgcx_tpl_comparison.png`; not game evidence |

Rollback both the DLL and submod together, after user exit; exact commands
are in U-8. Original Lua remains verified as `lua51_real.dll`.

| Card | Status | Instructions/evidence |
|---|---|---|
| TPL-0 / GH-4 | 0.4.0 installed; done, not tested in game | instructions-tpl-0.md; report-tpl-0-panel-1017.md |
| U-8 | first live recruitment/visual check pending | instructions-u-8.md |
| TPL-1 | D12 approved; research/implementation queued after U-8 | instructions-tpl-1.md |
| LAW-2 / U-7 | 0.3.5 controls/reforms pending game check | existing instructions/reports |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |


---

# Session 2026-10-17 (26) - TPL-0 native panel 0.4.0

Native Templates panel, local preference file and engine-command recruit are built,
fixture-tested and installed on E:. Done, not tested in game (U-8). All
pre-existing installed submod files remain byte-identical. Paired 0.3.5
rollback retained. No game control, subagent, commit or push. See
`docs/report-tpl-0-panel-1017.md`. Next: U-8, repairs/acceptance, then TPL-1.

## NEXT at session start (verbatim)

# NEXT - what comes next

2026-10-17 (25). **LAW-2 0.3.5 is installed on E:. Its state is done, not
tested in game.** On top of 0.3.4 (whose look the user accepted: "this is
splendid"), it adds:
- visible Bill buttons (text-sprite buttons declare `buttonText`/`buttonFont`);
- the Reforms tab as on board 1: blocked one-step reforms are lit, and a
  click presents the bill; with a bill open, only its target is lit and it
  opens the Bill;
- no `tgcx_` decisions in the Decisions tab, and no "decision taken" popup
  for decisions the DLL sends.

Read `docs/report-law-2-reforms-1017.md`. **The user then selected army
templates** ("i'd like to actually move on to the army templates"): its
first card is TPL-0 (`docs/instructions-tpl-0.md`), and its static research
has started.

## Current baseline

- **Every change still goes through the engine's decision command.** The
  engine itself checks `potential`/`allow` (P39), so hiding is the DLL's
  list filter, never script.
- **62 runtime guards** (14 new: reform click/rows, option name, decisions
  list, decision name, decision message). `check_exe.py` ALL OK.
- **Tests pass:** 27 catalogue, 8 Bill GUI, 10 script installer;
  `tgcx_tests` ALL OK (new Reforms tests) and 8 DLL installer/Lua tests; the
  law gates.
- **14 deviations from the boards:** the 10 of the controls report plus 4
  new ones in the reforms report.
- Only the user launches or exits the game. Preserve uncommitted changes. No
  commit, push or subagent unless asked. The next pitfall is P40.

## Next session in order

1. **U-7 (the user), short:** check 0.3.5 in game (`docs/instructions-u-7.md`):
   - the Reforms tab and a click on a lit row;
   - the Bill buttons;
   - the Decisions tab;
   - a month passing with no popup;
   - screenshots.
2. **The coordinator** reads the log and screenshots against boards 1 and 2,
   and fixes or records each difference (`docs/instructions-law-2.md`,
   "Next"). With the user's side-by-side acceptance, LAW-2 closes with a new
   report.
3. **TPL-0** (`docs/instructions-tpl-0.md`), which the user has selected:
   1. finish the static reading of `CBuildUnitView` and the recruit path
      (read-only Ghidra);
   2. settle the open board questions with the user [user];
   3. then the first DLL recruit of one brigade, and of N brigades.

   It can start while U-7 waits, since it touches other files. A U-7 fix
   comes first if one is needed.
4. Close with a report, archived/revised NEXT, the instructions and the
   statuses.

## Evidence and rollback

| What | Where / value |
|---|---|
| Stage | `E:\v2re\logs\tgcx-law2-reforms-1017\stage` |
| Ghidra evidence | `E:\v2re\logs\tgcx-law2-reforms-1017\` (reforms), `E:\v2re\logs\tgcx-tpl-research-1017\` (templates) |
| Rollback | 0.3.4 DLL in `…\before\lua51.dll` (SHA `2792cbd1…`) and the 0.3.4 submod in `…\before-submod`; roll back both together (U-7) |
| Installed DLL 0.3.5 SHA-256 | `bec50bb774395f02f71c041c134e0a626b6723b4fd8259b7d402ed8a3890bbc7` (equal to the build) |
| Generated GUI SHA-256 | `41cb93566f404dc22a3d504128fc8a1126a0aec51a99f16d3a961b4a13e5b8ce` |

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 / U-6 | short script smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| LAW-2 | 0.3.4 look accepted; 0.3.5 buttons, Reforms tab, Decisions filter: done, not tested in game | instructions-law-2.md; report-law-2-reforms-1017.md |
| U-7 | 0.3.5 game check pending | instructions-u-7.md |
| TPL-0 (GH-4) | selected by the user; static research started | instructions-tpl-0.md |
| HEMI-3 | accepted 0.2.8 retained; extended combined integration coverage unconfirmed | existing reports |
| U-5 | extended integration coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |


---

# Session 2026-10-17 (25) - LAW-2 buttons, Reforms tab and Decisions 0.3.5

The user accepted the 0.3.4 look ("this is splendid") and asked for visible
buttons, a Reforms tab like board 1, no cluttered decisions, and then army
templates.
- Text-sprite buttons now declare `buttonText`/`buttonFont`; the Open Bill
  button moved off-screen (probe only).
- Reforms tab: the DLL lights blocked one-step catalogue reforms (and only
  the target while a bill is open); a click presents the bill or opens the
  Bill.
- The Decisions tab hides `tgcx_` decisions in the DLL's list filter (P39),
  and the popup of DLL-sent decisions is skipped.
- 14 new guards (62 runtime). TPL-0 research started: `CConstructUnitCommand`
  constructor and the UI recruit path.
All automatic gates pass, and 0.3.5 is installed: done, not tested in game.
See `docs/report-law-2-reforms-1017.md` and `docs/instructions-tpl-0.md`.

## NEXT at the start of the session (verbatim)

# NEXT - what comes next

2026-10-17 (24). **LAW-2 0.3.4 is installed on E:. Its state is done, not
tested in game.** It contains:
- the Bill rebuilt on the board's 900 x 772 layout (`mockups/2-negotiation.png`);
- every control: the card buttons Abstain/Aye/Promise, the 5/25 lots,
  Audience, Recount, Withdraw, Call the Vote, the sort headers and the chart
  modes;
- Crown, debt and timer status;
- automatic favour collection.

Read `docs/report-law-2-controls-1017.md`. The user asked for steps 1, 2
and (optionally) 3 of the previous NEXT; all three are implemented.

## Current baseline

- **Every change goes through the native decision click** with a guarded
  `tgcx_` name. The decisions the DLL sends check their own conditions
  (P38).
- **The timers come from the timed-modifier list:** `CCountry+0x4D8`, item
  +8 is the modifier and +0xC its date. Six new guards make 48 runtime
  guards; `check_exe` reports 81 OK.
- **Automatic favours** are requested only while the date advances. One
  request is sent per absence of `tgcx_favours_tick`; it is retried after
  two days and reset on a country change.
- **The art is composed at staging time** from the installed TGC/game
  textures: `tgcx_bill_bg.dds` and `tgcx_bill_atlas.dds`. Nothing from the
  game is in git.
- **Tests pass:**
  - 27 catalogue, 6 Bill GUI and 10 script installer tests;
  - `tgcx_tests` ALL OK (controls, placement, GPU) and 8 DLL installer/Lua
    tests;
  - the 7 law gates.
- **10 deviations from the board** are listed in the report; the main ones
  are Recount for Reset, the government's name for the monarch's, and a
  session-only log.
- Only the user launches or exits the game. Preserve uncommitted changes. No
  commit, push, subagent or other mechanic has been selected. The next
  pitfall is P39.

## Next session in order

1. **U-7 (the user):** a short in-game check of 0.3.4
   (`docs/instructions-u-7.md`):
   - a screenshot next to the board;
   - a few controls;
   - a month passing (automatic favours);
   - save and reload.
2. **The coordinator** reads the logs and screenshot and compares them with
   the board side by side. Fix the differences, or record each engine limit
   with its reason (`docs/instructions-law-2.md`, "Next").
3. **LAW-2 closes only with the user's side-by-side acceptance** (R10 and
   §11). Then choose the next mechanic with the user.
4. Close with an immutable report, archived/revised NEXT, the instructions
   and the statuses.

## Evidence and rollback

| What | Where / value |
|---|---|
| Stage | `E:\v2re\logs\tgcx-law2-controls-1017\stage` |
| Ghidra evidence | `modifier-1.txt`, `modifier-2.txt` in the same folder |
| Rollback | 0.3.3 DLL in `…\before\lua51.dll` (SHA `9483ea3a…`) and the previous submod in `…\before-submod`; roll back both together (U-7) |
| Installed DLL 0.3.4 SHA-256 | `2792cbd1bf32b201471b…` (equal to the build) |
| Generated GUI SHA-256 | `f7cdd86c8f51349008ed0223c36f0e3b96252c3241026439372b42e7d11c8fe5` |

The accepted 0.3.3 screenshot and the 0.2.8/LAW-1 rollback are as before
(`docs/archive/instructions-u-7-before-controls-1017.md`).

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 / U-6 | short script smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| LAW-2 | 0.3.4 board layout, all controls, automatic favours: done, not tested in game | instructions-law-2.md; report-law-2-controls-1017.md |
| U-7 | 0.3.4 game check pending | instructions-u-7.md |
| HEMI-3 | accepted 0.2.8 retained; extended combined integration coverage unconfirmed | existing reports |
| U-5 | extended integration coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |

---

# Session 2026-10-17 (24) - LAW-2 controls 0.3.4

The user asked for steps 1, 2 and optionally 3 of the NEXT below.
- The Bill was rebuilt on the board's 900 x 772 layout, with the art composed
  at staging time from installed textures.
- Every control was added: card Abstain/Aye/Promise, the 5/25 lots, Audience,
  Recount, Withdraw, Call the Vote, the sort headers and the chart modes.
- Crown, debt and timer status come from the timed-modifier list (six new
  guards).
- Favours are collected automatically through the decision command.
- The script guards every DLL-sent decision (P38).
All automatic gates pass, and 0.3.4 is installed: done, not tested in game.
See `docs/report-law-2-controls-1017.md`.

## NEXT at the start of the session (verbatim)

# NEXT - what comes next

2026-10-16 (23). **LAW-2 0.3.3 is installed on E:; the user accepted its
position/chart-placement/layering repair in game. Mockup fidelity is NOT
accepted: the Bill must still match `mockups/2-negotiation.png` (R10).** Bill is centred, projected seats fill the correct
panel, and Bill draws above Politics without native labels crossing it.
Read `docs/report-law-2-visual-1016.md`, then `docs/report-law-2-live-1016.md`
for implementation and evidence; `docs/report-law-2-mockup-scope-1016.md`
records the user's acceptance-scope correction. The full mechanic/interface
is unfinished.

## Current baseline

- Native Bill labels/Open/Close/Count, projected chart and movable native
  background. Count enters the engine decision-command path; no direct
  simulation writes. Purchases/promises/vote/favours still use Decisions.
- Correct native route: window slot 11 / RVA 0x5E2FC0 appends after Politics.
  Packed position setter RVA 0x5E3BA0 takes positive absolute screen x/y;
  native conversion supplies screen centre. Chart pixels come from inner
  matrix +0x84 / negative +0x88. Inner +0xFC/+0x100 are not screen pixels.
  Never restore the 0.3.1/0.3.2 position or scene-order assumptions.
- 74 executable file gates / 42 runtime guards; 25 catalogue, 4 GUI,
  10 script installer and 8 DLL installer/Lua tests pass. Native engine
  window/lifecycle/order fixtures and real D3D9 pixel/state/hover/Reset
  checks pass. Installed DLL matches the build; law scripts unchanged.
- Visual placement/chart/layering accepted. Native Count callback is logged;
  complete report ordering, drag-following and close/reopen coverage beyond
  supplied screenshots is not inferred. Flat art, compact rows and missing
  controls/debt/Crown detail are unfinished requirements, not an approved
  replacement for the mockup. Match its frame, fonts, spacing, panels,
  controls and information as closely as the engine allows.
- Only the user launches/exits the game. Preserve uncommitted changes.
  No commit, push, subagent or other mechanic selected. Next pitfall: P38.

## Next session in order

1. Keep 0.3.3 as the working position/layering baseline; its appearance is
   provisional. Open the Bill mockup and implement against it (R10). Do not repeat the failed-layout
   assignment or extended U-6. Read current LAW-2/U-7 instructions.
2. Match the mockup while finishing LAW-2 controls: generic guarded native decision dispatch, withdraw/
   vote/audience, bloc abstain/Aye/promise buttons and 5/25 lot selector.
   Use fresh script eligibility on clicks; keep Decisions fallback.
3. Add accurate Crown/debt status, then automatic favour collection via
   synchronized command and authoritative persisted script cooldown. Cover
   save/load, duplicate submissions and country changes; retain manual fallback.
4. For changed controls collect brief relevant game evidence, preserve logs/
   screenshot and run applicable existing gates. Do not infer full mechanic
   acceptance from this visual repair. Finish laws before economy/rulers/templates.
5. R10 completion gate: compare actual game screenshot and Bill board side
   by side; fix differences or document a concrete engine limitation and
   obtain acceptance. Mockup fidelity is required before LAW-2 completion,
   not optional later polish. Online republishing remains pending browser
   access; the existing local board is sufficient to implement against.
6. Close with immutable report, archived/revised NEXT, current instructions
   and statuses.

## Evidence and rollback

Accepted screenshot: `mockups/u7/tgcx-law2-033-layout-accepted-2026-10-16.png`.
SHA-256: `d8699777135e5ce3c1774e000a63736aa8d9f30a0b4892d42fea90384a0deb63`.
Accepted-run logs: ignored `work/law2-layout-1016/accepted-033`.
Live diagnosis/build/install evidence: `E:\v2re\logs\tgcx-law2-live-1016`;
`before/` retains complete 0.3.2 DLL/source/logs and original modifier fragment.
Current movable script/GUI stage: `E:\v2re\logs\tgcx-law2-position-1016\stage`.
Accepted 0.2.8/LAW-1 baseline rollback remains in the original 1013 folders.
U-7 lists exact immediate and accepted-baseline rollback commands.

Installed 0.3.3 DLL SHA-256:
`9483ea3ab83811f591fa2b71a1d9abc0e75327deb7859a03e9cd22ce6c483b7c`.
Generated GUI SHA-256 (unchanged between 0.3.2/0.3.3):
`066cd7485d20d47af2393a575563e29376e129f27d8c74c767240a04b2a910bb`.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 / U-6 | short script smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| LAW-2 | 0.3.3 placement/layering repair accepted; mockup fidelity, full controls/ticking unfinished | instructions-law-2.md; report-law-2-visual-1016.md |
| U-7 | visual placement/chart/layering accepted; broader interaction coverage partial | instructions-u-7.md |
| HEMI-3 | accepted 0.2.8 retained; extended combined integration coverage unconfirmed | existing reports |
| U-5 | extended integration coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |

---

# Handoff scope correction 2026-10-16: mockup fidelity remains required

The user accepted the placement/layering repair, not the prototype as the
final interface. The Bill must match the agreed mockup (R10).

## Prior handoff NEXT (verbatim)

# NEXT - what comes next

2026-10-16 (23). **LAW-2 0.3.3 is installed on E:; the user accepted its
visual layout in game.** Bill is centred, projected seats fill the correct
panel, and Bill draws above Politics without native labels crossing it.
Read `docs/report-law-2-visual-1016.md`, then `docs/report-law-2-live-1016.md`
for implementation and evidence. The full law mechanic is unfinished.

## Current baseline

- Native Bill labels/Open/Close/Count, projected chart and movable native
  background. Count enters the engine decision-command path; no direct
  simulation writes. Purchases/promises/vote/favours still use Decisions.
- Correct native route: window slot 11 / RVA 0x5E2FC0 appends after Politics.
  Packed position setter RVA 0x5E3BA0 takes positive absolute screen x/y;
  native conversion supplies screen centre. Chart pixels come from inner
  matrix +0x84 / negative +0x88. Inner +0xFC/+0x100 are not screen pixels.
  Never restore the 0.3.1/0.3.2 position or scene-order assumptions.
- 74 executable file gates / 42 runtime guards; 25 catalogue, 4 GUI,
  10 script installer and 8 DLL installer/Lua tests pass. Native engine
  window/lifecycle/order fixtures and real D3D9 pixel/state/hover/Reset
  checks pass. Installed DLL matches the build; law scripts unchanged.
- Visual placement/chart/layering accepted. Native Count callback is logged;
  complete report ordering, drag-following and close/reopen coverage beyond
  supplied screenshots is not inferred. Flat art, compact rows and missing
  full controls/debt/Crown detail remain documented prototype deviations.
- Only the user launches/exits the game. Preserve uncommitted changes.
  No commit, push, subagent or other mechanic selected. Next pitfall: P38.

## Next session in order

1. Keep 0.3.3 as the accepted visual baseline. Do not repeat the failed-layout
   assignment or extended U-6. Read current LAW-2/U-7 instructions.
2. Finish LAW-2 controls: generic guarded native decision dispatch, withdraw/
   vote/audience, bloc abstain/Aye/promise buttons and 5/25 lot selector.
   Use fresh script eligibility on clicks; keep Decisions fallback.
3. Add accurate Crown/debt status, then automatic favour collection via
   synchronized command and authoritative persisted script cooldown. Cover
   save/load, duplicate submissions and country changes; retain manual fallback.
4. For changed controls collect brief relevant game evidence, preserve logs/
   screenshot and run applicable existing gates. Do not infer full mechanic
   acceptance from this visual repair. Finish laws before economy/rulers/templates.
5. Apply remaining mockup polish/deviations; online mockup republishing remains
   pending browser access. Close with immutable report, archived/revised NEXT,
   current card instructions and statuses.

## Evidence and rollback

Accepted screenshot: `mockups/u7/tgcx-law2-033-layout-accepted-2026-10-16.png`.
SHA-256: `d8699777135e5ce3c1774e000a63736aa8d9f30a0b4892d42fea90384a0deb63`.
Accepted-run logs: ignored `work/law2-layout-1016/accepted-033`.
Live diagnosis/build/install evidence: `E:\v2re\logs\tgcx-law2-live-1016`;
`before/` retains complete 0.3.2 DLL/source/logs and original modifier fragment.
Current movable script/GUI stage: `E:\v2re\logs\tgcx-law2-position-1016\stage`.
Accepted 0.2.8/LAW-1 baseline rollback remains in the original 1013 folders.
U-7 lists exact immediate and accepted-baseline rollback commands.

Installed 0.3.3 DLL SHA-256:
`9483ea3ab83811f591fa2b71a1d9abc0e75327deb7859a03e9cd22ce6c483b7c`.
Generated GUI SHA-256 (unchanged between 0.3.2/0.3.3):
`066cd7485d20d47af2393a575563e29376e129f27d8c74c767240a04b2a910bb`.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 / U-6 | short script smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| LAW-2 | 0.3.3 visual repair accepted in game; full controls/ticking unfinished | instructions-law-2.md; report-law-2-visual-1016.md |
| U-7 | visual placement/chart/layering accepted; broader interaction coverage partial | instructions-u-7.md |
| HEMI-3 | accepted 0.2.8 retained; extended combined integration coverage unconfirmed | existing reports |
| U-5 | extended integration coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |


---

# Session 23 final addendum, 2026-10-16

0.3.1 and 0.3.2 were tested during work and failed. Read-only live 0.3.2
snapshot resolved projection/list-order assumptions. 0.3.3 installed and
visually accepted by the user's final screenshot, saved byte-identically.
Full controls/automatic favours remain unfinished. See report-law-2-visual-1016.md.
Earlier session notes below are historical intermediate checkpoints; the
starting NEXT is retained verbatim.

# Session close 2026-10-16 (23): LAW-2 layout repair

User rejected 0.3.0 after testing; screenshot and logs retained. Native
foreground/coordinate repair 0.3.1 built, checked and installed on E:.
Repair not tested in game. See report-law-2-layout-1016.md.
No game launch, commit or subagents.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-13 (22). **LAW-2 Bill prototype 0.3.0 installed on E:, done, not
tested in game.** LAW-1's short Present/Count/save-reload path is accepted
by the user; extended coverage is not inferred. Finish the law mechanic
before economy, rulers or templates. Read `docs/report-law-2-bill-1013.md`,
`docs/instructions-law-2.md` and `docs/instructions-u-7.md`.

## Current result

- Native Bill child with live law, vote/favours/bloc/affinity/promise data.
  Open/Close/Count wired; Count queues the native engine decision command.
  Projected chart draws in native GUI order. Native tooltip slot untouched.
- Purchases, promises, audience, withdrawal, vote and manual favour collection
  remain in Decisions. Automatic favour ticking and the full Bill controls
  are unfinished. The mechanic itself has not changed.
- 25 catalogue, 3 GUI, 10 script installer tests, LAW-0 byte identity,
  reference/TGC/signature gates and native/real-GPU/eight DLL installer/Lua
  tests pass. 70 file gates, 38 runtime guards. Catalogue 37,775,061 bytes.
- Generated GUI comes from installed TGC plus our own fragment. No copied
  TGC GUI or art enters git. Private layout preview is not a game screenshot.
- Only the user launches/exits the game. Preserve existing uncommitted work.
  No new mechanic, commit or subagent is selected. Next pitfall is P35.

## Next session in order

1. U-7: user loads an active bill, Counts once in Decisions, opens Politics,
   checks the Bill and its Count/Close/Open controls. One screenshot and a
   brief description suffice. Do not repeat the extended U-6 assignment.
2. After normal exit retain game/DLL logs; use `tools/tgcx_check_laws.py`
   for seven script/GUI gates and a snapshot. Actual GUI/command execution
   and native text placement need evidence; fixtures alone cannot close U-7.
3. Fix concrete failures and keep rollback. Once the GUI path works, finish
   LAW-2: bloc buys/promises, lot selector, audience, withdraw/vote and
   accurate Crown/debt status. Reuse native decision dispatch and fresh
   script eligibility; never write simulation fields directly.
4. Add automatic favour collection through the engine command path,
   persisted script timer and manual fallback. Handle save/load and
   duplicate commands without a second scheduler changing game state.
5. Apply Bill mockup polish and recorded deviations. Earlier online mockup
   update remains awaiting browser access. Do not replace actual GUI work
   with another mockup or label the whole mechanic complete prematurely.
6. Write an immutable report with observed evidence, archive/rewrite NEXT,
   update statuses and step-by-step instructions for the next cards.

## Installation and rollback

Current source: `E:\v2re\logs\tgcx-law2-bill-stage-1013`.
Script rollback: `E:\v2re\logs\tgcx-before-law2-submod-1013`.
DLL/log rollback: `E:\v2re\logs\tgcx-before-law2-1013` (accepted 0.2.8).
LAW-1 smoke evidence: `E:\v2re\logs\tgcx-law1-smoke-accepted-1013`.
Install/build/native gates and verification: `tgcx-law2-*-1013` private logs.
Seven-gate bundle: `E:\v2re\logs\tgcx-law-check-20261013T053026919692Z`.
Its copied game logs predate the prototype; no new game evidence is inferred.
Rollback commands are in U-7; restore both script and DLL after normal exit.

Installed DLL SHA-256:
`43bb3e7d3d0133b3bdb316fe9dd4253923297143ad0b113724534be5ed2f6ca5`.
Accepted 0.2.8 rollback SHA-256:
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 | script smoke accepted; broader native/mechanic coverage unconfirmed | instructions-law-1.md; report-law-2-bill-1013.md |
| U-6 | short smoke passed; extended list not assigned again | instructions-u-6.md |
| LAW-2 | prototype installed, done, not tested in game; full controls/ticking unfinished | instructions-law-2.md; report-law-2-bill-1013.md |
| U-7 | brief Bill visual/Count check pending | instructions-u-7.md |
| HEMI-3 | accepted 0.2.8 retained; behavior carried into unconfirmed 0.3.0 | existing reports |
| U-5 | extended integration coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |


---

# Session close 2026-10-13 (22): LAW-2 Bill prototype

User accepted the LAW-1 smoke path and asked to finish laws before starting
another mechanic. Requested the in-game Bill prototype; bounded Ghidra
research, native GUI/command bridge, guarded live model and ordered chart
implemented. 0.3.0 built, gates passed, installed on E: with complete
script/DLL rollback. Prototype is not tested in game. See report-law-2-bill-1013.md.
No game launch, commit or subagents.

## Starting NEXT with active steering (verbatim)

# Active steering 2026-10-13

**LAW-2 Bill GUI prototype is now selected.** User confirmed the LAW-1
smoke check, asked to move on without more manual law tests, then corrected
the proposed economy switch: finish the law mechanic first. Explicitly
requested an in-game Bill prototype and authorized Ghidra/RE references.
Read docs/instructions-law-2.md. Prototype work is in progress; not installed.
Existing installed DLL 0.2.8 and LAW-1 report fix remain the baseline until
prototype gates pass. Earlier session-21 handoff is retained below until
normal session close; its requests for further U-6 tests are superseded.

---

# NEXT - what comes next

2026-10-13 (21). **LAW-1 report fix installed on E:, awaiting U-6 retest.**
User's screenshot exposed every conditional result option at once. Read
`docs/report-law-1-report-fix-1013.md`, then `docs/instructions-u-6.md`.
The earlier native implementation report remains an immutable snapshot.
All 23 base affinity groups remain approved; DLL 0.2.8 remains accepted.

## 1. Current implementation and evidence

- Present/Count now fully count in the decision, then fire exactly one
  ordinary result event with one button. No conditional options. Obsolete
  event 420000194 is retained only for a queued-save recovery recount.
- Screenshot confirms the earlier candidate loaded and opened its report,
  but failed presentation. Current run's error/system_error logs are empty.
  Do not infer tally, purchases, save/load or full LAW-1 acceptance.
- Corrected generated decisions are 35,319,087 bytes. The catalogue ceiling
  is now 40 MiB; counting cannot be shared through unsupported conditional
  options. Actual loading/performance needs another user run.
- Native TGC starting votes, balanced five-seat counts, fresh spending and
  target validation are retained. 24 catalogue tests, seven installer
  tests, LAW-0 and TGC scan pass. Default LAW-0 output stays byte-identical.
- Only the user launches/exits the game. No DLL rebuild, address change,
  commit or subagents. Preserve all existing uncommitted HEMI/LAW work.

## 2. Next session, in order

1. Use U-6's quick player check: fresh UK, Press Rights forward bill,
   Count again, save/reload and Count once more. Exactly one report/button;
   low-pressure 83/12/5 example says Nay leads by 80. Only ask the player for
   this short path first, plus whether loading was slow and one screenshot.
   After normal exit, retain logs with `python tools/tgcx_check_laws.py`.
   The agent can run the command; no reinstall needed.
2. Treat U-6's extended list as coordinator coverage, not a whole manual
   assignment to the player. Existing automated tests cover route/rounding/
   bargaining/debt/guard logic. Arrange targeted live bargains/Crown,
   forward/reverse paths and native comparisons after the smoke test works.
   A short smoke pass does not close full LAW-1 acceptance. Five-seat
   quantization intentionally differs from native Q15 percentages.
3. Fix concrete failures, rerun relevant gates, stage and install reversibly
   after normal exit. Never simulate option hiding again (P34); branch
   before firing the result (P21). Preserve accepted native DLL behavior.
4. Republish the already updated private Bill artifact when a browser
   session becomes available. Local PNGs are current; online artifact is
   still stale. This fix changes report routing, not the Bill mechanic.
5. Write new immutable evidence reports. Mark LAW-1 accepted only after
   actual game checks; U-5 extended coverage remains separately unconfirmed.
6. Archive/rewrite NEXT, update ROADMAP/SUBAGENTS and next-card instructions.
   Further mechanics are not selected. Next pitfall P35.

## 3. Candidate, rollback and retained gates

Current stage: `E:\v2re\logs\tgcx-law1-report-stage-1013`.
Pre-fix candidate rollback: `E:\v2re\logs\tgcx-law1-before-report-fix-1013`.
Accepted LAW-0 rollback: `E:\v2re\logs\tgcx-law1-before-native-1012`.
Failure evidence: `E:\v2re\logs\tgcx-law1-report-failure-1013` plus the
user's screenshot in conversation. Current game logs preserved there.
Installation/gate logs use `tgcx-law1-report-*-1013` in the private logs.
Installer prepares generation, retains backups and refuses a running game.

Installed candidate matches staging; generated modifiers still use installed
TGC plus our nine-bloc block. DLL SHA-256 remains
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.
No address/runtime change; retained 45 file gates / 27 runtime guards.
No claim of in-game confirmation of this report fix.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 | prior presentation failed; fix installed, retest pending; all affinities approved | instructions-law-1.md; report-law-1-report-fix-1013.md |
| U-6 | in progress; report-fix retest first | instructions-u-6.md |
| LAW-0 / U-3 | accepted baseline retained in rollback | existing reports |
| HEMI-3 | accepted DLL 0.2.8 unchanged | existing reports |
| U-5 / DLL-1 / HEMI-1 | extended coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred (D1) | existing instructions |

Latest one-command automatic bundle (all six gates pass, 11 game logs copied):
`E:\v2re\logs\tgcx-law-check-20261013T031729187160Z`.
This is a new snapshot of existing logs, not a new live game run.


---

# Session close 2026-10-13 (21): shorten U-6 player testing

User asked for a simpler test workflow. U-6 now begins with a short UK
Present/Count/save-reload smoke test; the extended list is coordinator
coverage, not a request to run every case manually. Added one command to
run existing automatic gates and retain private game logs. The installed
report-fix candidate and accepted DLL are unchanged. No game launch,
installation, commit or subagents. Live mechanic acceptance is still pending.
See docs/instructions-u-6.md; current next card remains U-6.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-13 (20). **LAW-1 report fix installed on E:, awaiting U-6 retest.**
User's screenshot exposed every conditional result option at once. Read
`docs/report-law-1-report-fix-1013.md`, then `docs/instructions-u-6.md`.
The earlier native implementation report remains an immutable snapshot.
All 23 base affinity groups remain approved; DLL 0.2.8 remains accepted.

## 1. Current implementation and evidence

- Present/Count now fully count in the decision, then fire exactly one
  ordinary result event with one button. No conditional options. Obsolete
  event 420000194 is retained only for a queued-save recovery recount.
- Screenshot confirms the earlier candidate loaded and opened its report,
  but failed presentation. Current run's error/system_error logs are empty.
  Do not infer tally, purchases, save/load or full LAW-1 acceptance.
- Corrected generated decisions are 35,319,087 bytes. The catalogue ceiling
  is now 40 MiB; counting cannot be shared through unsupported conditional
  options. Actual loading/performance needs another user run.
- Native TGC starting votes, balanced five-seat counts, fresh spending and
  target validation are retained. 24 catalogue tests, seven installer
  tests, LAW-0 and TGC scan pass. Default LAW-0 output stays byte-identical.
- Only the user launches/exits the game. No DLL rebuild, address change,
  commit or subagents. Preserve all existing uncommitted HEMI/LAW work.

## 2. Next session, in order

1. Follow U-6's report-fix retest first: new UK game, Press Rights forward
   bill, exactly one report/one button; low-pressure 83/12/5 example should
   state Nay leads by 80. Count again gives the same single report. Preserve
   new logs and report slow loading or missing counts. No reinstall needed.
2. Continue U-6 purchases/promises, Crown, save/load, political/social
   forward/reverse paths, pressure/party/chamber changes and stale guards.
   Collect live native comparisons separately; five-seat quantization is
   intentionally different from native Q15 percentages.
3. Fix concrete failures, rerun relevant gates, stage and install reversibly
   after normal exit. Never simulate option hiding again (P34); branch
   before firing the result (P21). Preserve accepted native DLL behavior.
4. Republish the already updated private Bill artifact when a browser
   session becomes available. Local PNGs are current; online artifact is
   still stale. This fix changes report routing, not the Bill mechanic.
5. Write new immutable evidence reports. Mark LAW-1 accepted only after
   actual game checks; U-5 extended coverage remains separately unconfirmed.
6. Archive/rewrite NEXT, update ROADMAP/SUBAGENTS and next-card instructions.
   Further mechanics are not selected. Next pitfall P35.

## 3. Candidate, rollback and retained gates

Current stage: `E:\v2re\logs\tgcx-law1-report-stage-1013`.
Pre-fix candidate rollback: `E:\v2re\logs\tgcx-law1-before-report-fix-1013`.
Accepted LAW-0 rollback: `E:\v2re\logs\tgcx-law1-before-native-1012`.
Failure evidence: `E:\v2re\logs\tgcx-law1-report-failure-1013` plus the
user's screenshot in conversation. Current game logs preserved there.
Installation/gate logs use `tgcx-law1-report-*-1013` in the private logs.
Installer prepares generation, retains backups and refuses a running game.

Installed candidate matches staging; generated modifiers still use installed
TGC plus our nine-bloc block. DLL SHA-256 remains
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.
No address/runtime change; retained 45 file gates / 27 runtime guards.
No claim of in-game confirmation of this report fix.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 | prior presentation failed; fix installed, retest pending; all affinities approved | instructions-law-1.md; report-law-1-report-fix-1013.md |
| U-6 | in progress; report-fix retest first | instructions-u-6.md |
| LAW-0 / U-3 | accepted baseline retained in rollback | existing reports |
| HEMI-3 | accepted DLL 0.2.8 unchanged | existing reports |
| U-5 / DLL-1 / HEMI-1 | extended coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred (D1) | existing instructions |

---

# Session close 2026-10-13 (20): repair LAW-1 report options

U-6 screenshot revealed every conditional result button at once. Preserved
current game logs privately (error and system_error empty); do not infer
numeric acceptance. Restored LAW-0 count-before-fire result dispatch and
corrected the interpreter's unsupported option filtering. All 24 catalogue
and seven installer tests, LAW-0 and TGC scan pass. Corrected candidate
installed on E: after game exit, with complete pre-fix rollback; DLL 0.2.8
unchanged. New report presentation and larger script load remain untested
in game. P34 added, next P35. No game launch, commit or subagents.
See docs/report-law-1-report-fix-1013.md; U-6 remains the next card.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-12 (19). **LAW-1 native-support candidate installed on E:;
not tested in game. All base affinities approved.** Read
`docs/report-law-1-native-1012.md`, then `docs/instructions-u-6.md`.
The accepted DLL 0.2.8 is unchanged; complete LAW-0 rollback is retained.

## 1. Decisions and completed implementation

- User approved all proposed base affinity rows for the 23 groups; later
  corrections remain possible. Do not request the same review again.
- TGC starting votes plus affinity bargaining is implemented. Import
  conditional add/remove political/social rules from installed v2.5.2;
  conserve 100 five-seat parliament seats with balanced rounding.
- Full counts at introduction, Count and Vote; stale purchases/promises
  spend nothing and require Count. Friendly native Nay uses the existing
  2-favour price. Old purchases clip without refund; promises include
  earlier abstentions. Target/current law/government checks remain fresh.
- Only the user launches or exits the game. Single player first; R5 holds.
- No DLL rebuild or install. No commit or subagents. Preserve all existing
  uncommitted HEMI/LAW work and immutable reports.

## 2. Next session, in order

1. Perform U-6 from `docs/instructions-u-6.md`: load time/errors, UK low-
   pressure count and purchases, Crown, save/load, political/social forward
   and reverse steps, pressure/party/chamber changes and stale restrictions.
   Candidate is already installed; do not reinstall before collecting data.
2. Preserve screenshots/logs privately. Native percentage arithmetic is
   statically traced; live numeric comparisons remain outstanding. Expected
   UK 83/12/5 allocation is 85/10/5; low-pressure count 10/90, high
   Conservative political support 60/40. Exact native shares differ from
   this five-seat quantization. The report button states margin/quorum;
   the full Bill GUI is later DLL work.
3. Inspect new game errors and respond to any odd affinity rows the user
   identifies. Fix concrete failures; rerun relevant gates, stage and
   install reversibly after normal exit. Do not loosen checks or change
   accepted native tooltip/ordered-draw behavior.
4. Republish the updated private Bill artifact when a browser session is
   available. Local exports, including `mockups/2c-law-pressure.png`, are
   current. Inventory still had no apps/browsers despite full filesystem
   access. Online artifact remains stale; do not claim R10 is fully met.
5. Write a new immutable U-6 report. Mark LAW-1 game-tested only after
   actual evidence; preserve unconfirmed extended U-5 coverage separately.
6. Archive/rewrite NEXT, update ROADMAP/SUBAGENTS and next-card instructions.
   Further mechanics are not selected by this handoff. Next pitfall P34.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 | implemented and installed candidate; not tested in game; all 23 affinities approved | instructions-law-1.md; report-law-1-native-1012.md |
| U-6 | next: actual LAW-1 game evidence | instructions-u-6.md |
| LAW-0 / U-3 | accepted baseline retained in rollback; repository default byte-identical | existing reports |
| HEMI-3 | accepted DLL 0.2.8 unchanged | existing instructions/reports |
| U-5 / DLL-1 / HEMI-1 | extended coverage unconfirmed | instructions-u-5.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity and gates

Candidate: `E:\v2re\logs\tgcx-law1-native-stage-1012`.
Complete rollback source: `E:\v2re\logs\tgcx-law1-before-native-1012`.
After normal exit, rollback command is in `docs/instructions-u-6.md`.
`install_submod.py` supports `--source`, `--dry-run` and retained backups;
prepares generation before replacement and refuses a running game.

23 catalogue tests, seven installer tests, LAW-0 ALL OK, TGC scan and
reference SHA/45 file signatures pass. Runtime header retains 27 guards;
18 separate read-only research gates. Installed candidate matches staging.
Default LAW-0 generation remains byte-identical; production catalogue
output stays outside git. Generated modifiers use installed TGC plus our
staged nine-bloc block, not the repository's three-bloc LAW-0 block.

Source/installed DLL SHA-256:
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.
No new live game log is claimed. The earlier accepted HEMI log still has
27 guards, 6332 frames, 1451 ordered draws and empty active errors.

---

# Session close 2026-10-12 (19): LAW-1 native votes and installed candidate

User approved every base affinity row. Imported native conditional support,
implemented conserved five-seat counts and guarded negotiations, retained
legacy IDs/default LAW-0 byte identity, and prepared a reversible installer.
23 catalogue tests, 7 installer tests and all executable/TGC/signature gates
pass. Candidate installed on E: with complete LAW-0 rollback; accepted
DLL 0.2.8 unchanged. Local mockup/pressure export current; online artifact
still inaccessible (no connected browser). U-6 game evidence remains.
No game launch, DLL rebuild, commit or subagent. P33 added; next P34.
See docs/report-law-1-native-1012.md and docs/instructions-u-6.md.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-12 (18). **LAW-1 selected and in progress. HEMI-3 0.2.8 accepted
in game.** User requested a handoff. No LAW-1 preview installed; installed
LAW-0 and accepted DLL 0.2.8 are unchanged. Read the handoff:
`docs/report-law-1-start-1012.md`, then `docs/instructions-law-1.md`.

## 1. Decisions already made

- **TGC starting votes + affinity bargaining approved.** Import TGC's
  conditional add/remove political/social support for starting votes;
  use reviewed affinity for bargaining prices and promised laws. Do not
  ask for this approval again. The current preview is still affinity-only.
- Ten affinity groups approved: Press Rights, Public Meetings, Voting
  Franchise, Voting System, Minimum Wage, Work Hours, Safety Regulations,
  Unemployment Subsidies, Pensions and Health Care. Exact rows are in
  `data/tgcx_law_catalog.txt`. Thirteen remain pending.
- TGC v2.5.2 only; single player first. Only the user launches the game.

## 2. Next session, in order

1. Read the two reports and LAW-1 instructions. Preserve accepted 0.2.8
   evidence in `E:\v2re\logs\tgcx-hemi3-028-accepted-1012`.
2. Finish bounded native support verification from existing private
   `law1-support-*-1012.txt` outputs. Additive evaluator is identified;
   parser mapping, clamping and full numeric parity remain to verify.
   Run check_exe before addresses; six research gates are separate from
   the DLL's 27 runtime guards. Do not replace them with vanilla rules.
3. Implement the approved starting-vote formula in the catalogue generator
   and meaningful tests. Recount pressure/ruling-party/chamber changes,
   prevent double-counting supportive seats, preserve fresh target allows.
4. Resolve independent five-seat rounding: nine blocs can currently sum
   above/below 100. Enforce chamber conservation and document quantization.
5. Continue user affinity review for the thirteen pending groups. Do not
   infer approval. Refresh the review document and statuses after answers.
6. Local Bill board/PNGs now distinguish TGC starting votes and affinity.
   Republish the private artifact when browser access exists; no browsers
   were available this session. Update/export further mechanic changes.
7. Run TGC scan, LAW-0 checks and catalogue tests; stage outside live submod.
   Current twelve tests pass but prove the old affinity-only preview.
   Keep LAW-0 default output byte-identical unless deliberately migrated.
8. Once all release gates are met, prepare reversible installation and a
   concrete user game-test checklist. No immediate rebuild/install needed.
   Keep LAW-1 in progress until its remaining work and game evidence exist.
9. Close with an immutable report, archive/rewrite NEXT and update statuses
   and next-card instructions. Remaining U-5 coverage can be collected in
   a future user run; it does not block this selected LAW-1 research.

| card | status | instructions/evidence |
|---|---|---|
| LAW-1 | in progress; staged affinity-only preview; approved native model not implemented | instructions-law-1.md; report-law-1-start-1012.md |
| LAW-0 / U-3 | done, tested in game; installed baseline retained | existing reports |
| HEMI-3 | 0.2.8 hover/header accepted in game | updated instructions; LAW-1 start report section 1 |
| U-5 / DLL-1 / HEMI-1 | functional checkpoint passed; extended coverage unconfirmed | instructions-u-5.md; DLL-1/2 instructions |
| DLL-0 / U-4 | done, tested in game | existing reports |
| GH-2 | hemicycle data/mapping/ordered draw/scalar hover exercised in game | symbols.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Source/installed DLL 0.2.8 SHA-256:
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.
Accepted log: 27 runtime guards, 6332 frames, 1451 ordered draws, empty
active errors. No new DLL build or install. Runtime signature list remains
27; check_exe now also checks six separate read-only research signatures.

Preview: `E:\v2re\logs\tgcx-law1-preview-1012`. Gates:
`tgcx-law1-{tests,law0-tests,exe}-1012.txt`, `tgcx-law1-tgc252-1012.json`.
Twelve catalogue tests pass; LAW-0 checks ALL OK; TGC anchors/range and
reference hash/33 file signatures pass. Thirteen affinities, native votes,
rounding and game evidence are release gates. No production script changed.

No commit made; existing uncommitted HEMI and LAW-1 work must be preserved.
No subagents used. Native structured tooltip slot 10 remains untouched.
Next pitfall P33. The online mockup is stale; local board/export is updated.

---

# Session close 2026-10-12 (18): LAW-1 preview and native-support decision

User accepted HEMI-3 0.2.8 and selected LAW-1. Added a staged catalogue
(23 groups, 46 direction entries, 144 transitions, nine blocs), twelve
passing behavioral tests and backward-compatible LAW-0 generator hooks.
Ten affinity groups approved; thirteen pending. Inspected installed TGC
and vanilla ideology rules. User approved TGC starting votes + affinity
bargaining. Native arithmetic research started; conditional votes and
100-seat normalization remain unimplemented. No LAW-1 output installed.
Local Bill mockup updated/exported; online republish unavailable because
no browser is connected. User requested handoff due to little usage left.
See docs/report-law-1-start-1012.md and docs/instructions-law-1.md.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-12 (17). **HEMI-3 0.2.6 works in game, confirmed by the user.
0.2.8 header seam correction built, checked and installed on E:;
this visual correction is done, not tested in game.** 0.2.6 retained.
Report: `docs/report-hemi-3-seam-1012.md`.

## 1. Scope and user direction

TGC v2.5.2 only, single player first, read-only presentation. User accepted
hover; 0.2.7 then left a thin seam below the header. Inspected the actual
TGC header artwork: border ends at root y=100, cover now starts at y=101.
Seats and hover geometry are unchanged.
LAW-1 is for discussion afterwards, not selected or started. D3 blocks nothing.

## 2. Next session

1. Read `docs/instructions-hemi-3.md`, `docs/instructions-u-5.md` and the
   header report. No immediate rebuild/reinstall needed.
2. Only the user starts the usual E: game with TGC v2.5.2 + TGCX. Check
   that the cover meets the Upper House border with no seam or overlap,
   the legend/reform row remain intact, and seat hover still works.
3. Preserve the 0.2.8 runtime log before restarting; inspect 27 guards,
   ordered draws, hover mappings and active errors. Compare screenshot
   with Main. Record the visual result without inferring other passes.
4. Collect remaining U-5 coverage during the same run if convenient:
   gaps, legend/lower pies, overlapping popup, view reopen, days,
   save/reload and Alt-Tab. Unreported coverage remains unconfirmed.
5. Write a new immutable report; archive/rewrite NEXT and update statuses
   and next-card instructions. Discuss LAW-1 after visual acceptance;
   do not expand laws or decide user affinity choices during this check.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game | existing reports |
| DLL-1 / HEMI-1 | functional prototype tested in game; extended coverage remains | DLL-1/2 instructions; U-5 report |
| HEMI-2 | 0.2.5 accepted; earlier baseline retained | HEMI-2 handoff |
| HEMI-3 | 0.2.6 hover accepted; 0.2.8 header seam correction installed, not tested in game | instructions-hemi-3.md; report-hemi-3-seam-1012.md |
| U-5 | functional hover checkpoint passed; header check and remaining integration next | instructions-u-5.md |
| GH-2 | data/mapping/ordered draw/scalar hover exercised in game | symbols.md |
| LAW-0 / U-3 | done, tested in game | existing reports |
| LAW-1 | discuss afterwards; implementation not started | SUBAGENTS.md; law-design.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.8 DLL SHA-256:
`61824e4bc1b72fc5792533ede1cd14ea3f759c458c59d35407cef973db0ccdc5`.
Install record: DLL-1+HEMI-3; descriptor release 0.2.8. Reference hash/27
signatures, TGC anchors/range, Win32 /W4 /WX, native/real-GPU checks and
eight installer/Lua tests pass. Logs:
`E:\v2re\logs\tgcx-hemi3-028-{build,tests,exe}-1012.txt` and install JSON.

Accepted 0.2.6 DLL/record/descriptor/log/error logs/acceptance metadata:
`E:\v2re\logs\tgcx-before-027-1012`. 0.2.7 and its seam-feedback log:
`E:\v2re\logs\tgcx-before-028-1012`. Older 0.2.5 backup remains in
`tgcx-before-026-1012`; original Lua shim retained. Latest screenshot is
in the user's message; original 0.2.5 PNG remains outside git.

No new hooks/addresses or simulation writes. Native structured tooltip
slot 10 remains untouched. Only the user launches the game. P32 refined,
next P33. Generated background/native legend-order deviations remain.


---

# Session close 2026-10-12 (17): Cover seam corrected from native artwork

User reports 0.2.7 cover approximately two pixels short. Preserved runtime
log/proxy/metadata and inspected installed TGC artwork: the header border
ends at root y=100, earlier than its text layout maximum. 0.2.8 starts the
cover at y=101 instead of y=104. GPU tests check preserved border and all
three seam rows. Build/reference/TGC/native/GPU/installer checks pass;
installed with game closed, visual confirmation pending. 0.2.6 remains
accepted hover baseline. P32 refined, next P33. LAW-1 not started.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-12 (16). **HEMI-3 0.2.6 works in game, confirmed by the user.
0.2.7 header-overlap correction built, checked and installed on E:;
this visual correction is done, not tested in game.** 0.2.6 retained.
Report: `docs/report-hemi-3-header-1012.md`.

## 1. Scope and user direction

TGC v2.5.2 only, single player first, read-only presentation. User accepted
hover and reported a small overlap with the Upper House block. The cover
is now clipped below the header; seats and hover geometry are unchanged.
LAW-1 is for discussion afterwards, not selected or started. D3 blocks nothing.

## 2. Next session

1. Read `docs/instructions-hemi-3.md`, `docs/instructions-u-5.md` and the
   header report. No immediate rebuild/reinstall needed.
2. Only the user starts the usual E: game with TGC v2.5.2 + TGCX. Check
   that the entire Upper House title strip is visible above the chart,
   the legend/reform row remain intact, and seat hover still works.
3. Preserve the 0.2.7 runtime log before restarting; inspect 27 guards,
   ordered draws, hover mappings and active errors. Compare screenshot
   with Main. Record the visual result without inferring other passes.
4. Collect remaining U-5 coverage during the same run if convenient:
   gaps, legend/lower pies, overlapping popup, view reopen, days,
   save/reload and Alt-Tab. Unreported coverage remains unconfirmed.
5. Write a new immutable report; archive/rewrite NEXT and update statuses
   and next-card instructions. Discuss LAW-1 after visual acceptance;
   do not expand laws or decide user affinity choices during this check.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game | existing reports |
| DLL-1 / HEMI-1 | functional prototype tested in game; extended coverage remains | DLL-1/2 instructions; U-5 report |
| HEMI-2 | 0.2.5 accepted; earlier baseline retained | HEMI-2 handoff |
| HEMI-3 | 0.2.6 hover accepted; 0.2.7 header correction installed, not tested in game | instructions-hemi-3.md; report-hemi-3-header-1012.md |
| U-5 | functional hover checkpoint passed; header check and remaining integration next | instructions-u-5.md |
| GH-2 | data/mapping/ordered draw/scalar hover exercised in game | symbols.md |
| LAW-0 / U-3 | done, tested in game | existing reports |
| LAW-1 | discuss afterwards; implementation not started | SUBAGENTS.md; law-design.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.7 DLL SHA-256:
`6c83fb8c4ff47efd2df6d8e5aefedd4f028c12d4423491dbf53975aa3635eb40`.
Install record: DLL-1+HEMI-3; descriptor release 0.2.7. Reference hash/27
signatures, TGC anchors/range, Win32 /W4 /WX, native/real-GPU checks and
eight installer/Lua tests pass. Logs:
`E:\v2re\logs\tgcx-hemi3-027-{build,tests,exe}-1012.txt` and install JSON.

Accepted 0.2.6 DLL/record/descriptor/log/error logs/acceptance metadata:
`E:\v2re\logs\tgcx-before-027-1012`. Older 0.2.5 backup remains in
`tgcx-before-026-1012`; original Lua shim retained. Latest screenshot is
in the user's message; original 0.2.5 PNG remains outside git.

No new hooks/addresses or simulation writes. Native structured tooltip
slot 10 remains untouched. Only the user launches the game. New P32,
next P33. Generated background/native legend-order deviations remain.

---

# Session close 2026-10-12 (16): HEMI-3 accepted, header bounds repaired

User confirms 0.2.6 works and supplies a crop showing overlap with the
Upper House block. Preserved accepted DLL/log/metadata outside git: all
27 guards, 5555 ordered draws, three native ideology hover mappings and
empty errors. Clipped the cover below the native header without moving
seats or hover. 0.2.7 built, all automatic gates passed and installed with
game closed; visual correction not yet tested in game. 0.2.6 retained.
P32, next P33. Extended U-5 coverage remains; LAW-1 not started.

## Starting NEXT (verbatim)

# NEXT - what comes next

2026-10-12 (15). **HEMI-3 0.2.6 built, checked and installed on E:;
done, not tested in game.** 0.2.5 remains the accepted working baseline,
retained outside git. Report: `docs/report-hemi-3-1012.md`.

## 1. Scope and user direction

TGC v2.5.2 only; single player first; read-only presentation. User requested
HEMI-3 now and consideration of LAW-1 afterwards. No LAW-1 implementation
has started; review that card after HEMI-3/U-5 acceptance. D3 blocks nothing.

## 2. Next session: one HEMI-3/U-5 acceptance run

1. Read `docs/instructions-u-5.md`, `docs/instructions-hemi-3.md` and the
   HEMI-3 report. No immediate rebuild/reinstall is needed.
2. Only the user launches the usual E: game with TGC v2.5.2 + TGCX.
   Hover actual seat dots in each UK bloc, especially the left/right ends.
   Verify ideology matches colour, gaps/hole have no chart tooltip, native
   legend remains correct and menus/popups occlude normally.
3. In the same run, switch/reopen views, advance days, save/reload, wait
   paused and Alt-Tab once. Follow the U-5 checklist; preserve evidence.
4. Inspect the actual 0.2.6 log (27 guards, ordered draws, native hover
   ideology/index/slice lines), active errors and screenshot beside Main.
   If something fails, preserve log/dump before restarting; diagnose that
   failure and consolidate corrections. Do not infer unobserved passes.
5. Write a new immutable acceptance/failure report. Archive/rewrite NEXT,
   update statuses and write instructions for every selected next card.
6. After acceptance, discuss LAW-1 (up to 48 bills and affinity review).
   Do not expand laws during hover validation or decide user affinities.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; backups retained | existing reports |
| DLL-1 / HEMI-1 | functional prototype tested in game; extended coverage remains | DLL-1/2 instructions; U-5 report |
| HEMI-2 | 0.2.5 accepted; retained baseline, background/legend deviations | HEMI-2 handoff |
| HEMI-3 | 0.2.6 built/installed; done, not tested in game | instructions-hemi-3.md; report-hemi-3-1012.md |
| U-5 | one combined hover/remaining integration run next | instructions-u-5.md |
| GH-2 | runtime data/mapping/ordered draw confirmed; scalar hover ABI confirmed in code | symbols.md |
| LAW-0 / U-3 | done, tested in game | existing reports |
| LAW-1 | consider after HEMI-3 acceptance; implementation not started | SUBAGENTS.md; law-design.md |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.6 DLL SHA-256:
`35465f6375813b8b3e71a71560fa382e31ccd3aba69ec35ded35e5155f3aba94`.
Install record: DLL-1+HEMI-3; submod release comment 0.2.6.
All 27 signatures, TGC anchors/range, Win32 /W4 /WX, native/real-GPU tests,
eight installer/Lua tests and law checks passed. Logs:
`E:\v2re\logs\tgcx-hemi3-026-{build,tests,exe}-1012.txt` and fixture log.

Accepted 0.2.5 DLL/record/descriptor/log retained in
`E:\v2re\logs\tgcx-before-026-1012`; original shim and previous proxies remain.
Original screenshot: `E:\v2re\logs\tgcx-hemi2-025-user-screenshot-1012.png`.

Two verified scalar hooks (native pie bounds and Politics slice selection)
share the rendered seats; native structured tooltip slot 10 stays untouched.
Unsupported layout retains the native pie. No simulation writes. No game
launch or new runtime acceptance is claimed. New P31, next P32.

---

# Session close 2026-10-12 (15): HEMI-3 native hover built and installed

User requested HEMI-3 now and LAW-1 consideration afterwards. Implemented
verified native scalar picking/slice hooks, shared seat geometry and guarded
fallback. 0.2.6 built/installed on E: with game closed; 27 signatures, native/
GPU/Lua/eight installer tests and law checks pass. Not tested in game.
0.2.5 retained; one combined U-5 run next. P31, next P32.

## Starting NEXT (verbatim)

# NEXT — what comes next

2026-10-12 (14). **0.2.5 works in game, confirmed by the user.** Tooltip
hover does not correlate with the new chart. Handoff:
`docs/report-hemi-2-handoff-1012.md`. Preserve the working build; next card
is HEMI-3 hover integration. No immediate rebuild or repeat game test.

## 1. Scope

TGC v2.5.2 only; single player first; read-only presentation. No new mechanic
selected. Keep fewer iterations: one consolidated hover fix/build/test.
REA and the Victoria II RE git remain references for later army templates.
D3 remains open and blocks nothing.

## 2. Next implementation session: HEMI-3

1. Read `docs/instructions-hemi-3.md`, the handoff, Main mockup, renderer/tests,
   symbols and P27-P30. Inspect the saved original game screenshot.
2. Diagnose chart hover ownership and native tooltip ABI with bounded RE.
   Current new geometry is drawn over unchanged native pie hit regions.
   Do not confuse that mismatch with stale upper-house data.
3. Align chart hover with shared rendered geometry and correct engine
   ideology; preserve native legend, layering, fallback and CPU/D3D state.
   No guessed structured string returns or duplicate tooltip overlays.
4. Consolidate verified changes, run reference/TGC/native/GPU/installer
   gates and document mockup deviations. No unrelated polish expansion.
5. Only after a concrete safe build is ready, install reversibly with the
   game closed. Only the user starts the game. Combine hover checks with
   remaining U-5 integration coverage; see its instructions.
6. Preserve evidence and write a new immutable report; archive NEXT,
   rewrite it and update statuses/next-card instructions.

| card | status | instructions/evidence |
|---|---|---|
| DLL-0 / U-4 | done, tested in game; backups retained | existing reports |
| DLL-1 / HEMI-1 | functional prototype tested in game; extended coverage remains | DLL-1/2 instructions; U-5 report |
| HEMI-2 | 0.2.5 works in game; tooltip mismatch and background/legend deviations retained | HEMI-2 handoff/instructions |
| HEMI-3 | queued: correlate hover with actual rendered seats | instructions-hemi-3.md |
| U-5 | functional checkpoint passed; remaining checks with hover build | instructions-u-5.md |
| GH-2 | runtime data/mapping/coordinates/ordered draw confirmed | symbols.md |
| LAW-0 / U-3 | done, tested in game | existing reports |
| GOV-0 / GH-1 step 5 | available, not selected | existing instructions |
| GH-4 / TPL-1 | later; REA reference retained | rea-for-templates.md |
| COMPAT-0 | deferred (D1) | existing instructions |

## 3. Continuity

Active/source 0.2.5 DLL SHA-256:
`1ec1aea4fee87441a293358451ee15859cdf66435991abebe3a52370f5de5a1b`.
Original shim and previous proxies retained; submod release comment 0.2.5.
Checks: 23 signatures, TGC anchors/range, Win32 /W4 /WX build, real GPU/native,
8 installer/Lua forwarding tests and law checks passed.
Accepted log: `E:\v2re\logs\tgcx-hemi2-025-accepted-1012.log`:
10032 ordered draws, paused samples, final Politics hidden, empty errors.

Original screenshot (2560x1440):
`E:\v2re\logs\tgcx-hemi2-025-user-screenshot-1012.png`.
Metadata beside it; keep game art outside git. Native legend is 83/5/12;
chart wedge hover still follows the old pie. P30 records this; next P31.
No save/reload/date-advance/Alt-Tab coverage or tooltip repair is claimed.
User authorized pushing to the existing public repository and removed the
publication approval requirement from AGENTS §13. V2DLL provenance comments
remain. No game assets or build outputs are included in the commit.

---

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
