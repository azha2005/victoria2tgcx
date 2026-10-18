# NEXT - what comes next

2026-10-17 (28). **TPL-0 0.4.1 placement confirmed in game** (log: root
local 336,20 → screen 1104,490, P40). The user found the details still off
board 5; **0.4.2 (native sprites, selection ring, empty-row covers,
miniatures, board texts, footer Delete) is installed on E:: done, not
tested in game.** Read `docs/report-tpl-0-visual-1017.md`.
**0.4.2 game check: all good except Recruit, which did not draw or click
(P41).** The fix is staged in `E:/v2re/logs/tgcx-tpl-place-1017/stage`;
**0.4.3 is installed and confirmed in game.** Recruit through the panel
works and reopening shows the right sprites (report §9-10). Its only
defect: the Templates tab covered the last Build Navy folder. **0.4.4**
hides the tab in navy mode and is **accepted in game** ("works flawlessly",
queue clicks included; report §11). **TPL-1 (Conform) research started:** the engine commands for create/transfer/move/merge are mapped (`docs/report-tpl-1-research-1017.md`); save/reload is **D13 = A** (session tracking; re-Conform after a reload). Research §3.1, §3.2 and §3.4 is done (report §8). Signatures, the pure planner/tracker, the native command wrappers, the daily driver and the `CUnitView` hooks are done in DLL **0.5.0** (0.5.0 installed on E:; **U-9 passed**, except the province field, P43, fixed in **0.5.1**: **installed on E:**, 0.5.0 backed up in `before-051-dll`; report §9-11). 0.5.1 is confirmed in game (user). **The Conform button and window (board 6) are done in DLL 0.6.0: built and staged in `E:/v2re/logs/tgcx-tpl1-1017/stage-060`, on 0.6.0 the button did nothing (P44); 0.6.1 fixed that and opened the window, then crashed after a Merge (P45); **0.6.2 (Merge size 0x4C, hover, build-only per revised D13) is installed on E:** (backups `before-062-*` for 0.6.1) and **works in game** (user, report §8: builds march and merge, extras detach, no crash). Next (user): make the Conform GUI look more like Victoria II's own, and fix the guarded GUI read fault (report §8) (backups `before-061-*` for 0.6.0, `before-060-*` for 0.5.1), **not tested in game** (`docs/report-tpl-1-window-1017.md`). Next: the player runs **U-10** (`docs/instructions-u-10.md`). External
research: `docs/report-research-discord-1017.md`.

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
- 0.4.1 fixed the anchor (`tgcx_TplRootScreen`), confirmed in game. 0.4.2
  changes only the Templates GUI layout/assets and labels; all gates pass.
  Installed DLL SHA-256 `bdc3fbf9...c9a24c`.
- Only the user launches/exits the game. Preserve uncommitted changes.
  No commit, push or subagent unless asked. The next pitfall is P43.

## Next session in order

0. **U-10, Conform in game** (coordinator installs after a normal exit,
   then the player checks): `docs/instructions-u-10.md`. Back up 0.5.1 and
   the submod first. Fix what the game shows before anything else.
1. **U-8 short 0.4.4 check** (player): Build Navy shows no Templates tab,
   Build Army still does, and clicks on Units Being Built entries and
   their cancel button still work through the panel. Recruit and the
   reopen sprites are already confirmed on 0.4.3.
2. **TPL-0 acceptance** (coordinator): `docs/instructions-tpl-0.md`.
   Compare with board 5 against the retained deviations (report §4), fix
   the queue hit area if needed. Then the remaining U-8 cases: close and
   reopen Military, a one-regiment template, a partial recruit, and a
   reload in a fresh process.
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

Private roots: `E:/v2re/logs/tgcx-tpl-place-1017/` (0.4.1: `before-dll/` is the 0.4.0 DLL) and `E:/v2re/logs/tgcx-tpl-panel-1017/` (0.4.0 staging and the 0.3.5 rollback).

| What | Where / value |
|---|---|
| Staged source | `stage/` |
| Rollback 0.3.5 DLL | `before-dll/lua51.dll`, SHA `bec50bb7...0bbc7` |
| Complete rollback submod source | `before-submod/` |
| Installed 0.4.4 DLL SHA-256 | `39aafb4729eb6da004c2d12f37c0207481813c9a3ec6bfcbb092e968079272fb` |
| Rollback 0.4.3 | `tgcx-tpl-place-1017/before-044-dll/lua51.dll` (`215aa405...c88d4`) + `before-044-submod/` |
| Rollback 0.4.2 | `before-043-dll/lua51.dll` (`bdc3fbf9...c9a24c`) + `before-043-submod/` |
| 0.4.2/0.4.3 game evidence | `work/tgcx-u8-042/`, `work/tgcx-u8-043/` |
| Rollback 0.4.1 | `tgcx-tpl-place-1017/before-042-dll/lua51.dll` (`350299ef...eb8ca`) + `before-042-submod/` |
| Rollback 0.4.0 DLL | `tgcx-tpl-place-1017/before-dll/lua51.dll`, SHA `2dcbf718...39d1b` |
| 0.4.1 game evidence | `work/tgcx-u8-0411/` |
| Generated buildunit GUI SHA-256 (0.4.4) | `269e603ecaa08b1aa60c7d3e3beb0eb89ae7ed608af7747e564ef794bed89df7` |
| Accepted functional evidence | `work/tgcx-u8-1017/`: original screenshots, log, preferences and manifest |
| Static visual QA | `tgcx_tpl_preview.png`, `tgcx_tpl_comparison.png`; not game evidence |

Roll back to 0.4.1, 0.4.0 or 0.3.5 after user exit; exact commands
are in U-8. Original Lua remains verified as `lua51_real.dll`.

| Card | Status | Instructions/evidence |
|---|---|---|
| TPL-0 / GH-4 | 0.4.0 function accepted; 0.4.1 placement confirmed; 0.4.2 details installed, not tested in game | instructions-tpl-0.md; report-tpl-0-visual-1017.md |
| U-8 | recruit/save-reload (0.4.0) and placement (0.4.1) accepted; 0.4.2 short check next; extended cases open | instructions-u-8.md |
| TPL-1 | U-9 passed on 0.5.0; 0.5.1 confirmed; Conform window in 0.6.0, staged, not installed, not tested in game | instructions-tpl-1.md, instructions-u-10.md, report-tpl-1-window-1017.md |
| TPL-2 | on hold (user, 2026-10-17): Conform (TPL-1) covers it | instructions-tpl-2.md |
| LAW-2 / U-7 | 0.3.5 controls/reforms pending game check | existing instructions/reports |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |
