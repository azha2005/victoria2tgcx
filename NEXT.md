# NEXT - what comes next

2026-10-17 (29). **TPL-1 Conform works in game on 0.6.2** (user: "it
worked"). Built regiments march on safe paths and merge into the army,
extras detach into a new stack, there was no crash, and the error logs are empty.
Committed and pushed as `421bfc9`. Read `docs/report-tpl-1-window-1017.md`
(§8 is the acceptance). **Nothing of the next card has been started** (user:
"dont start anything, just write it in the handoff").

## User decisions this session

- **D13 revised:** Conform only builds the missing regiments. It never
  takes nearby stacks. After a reload, builds in progress are no longer
  tracked; pressing Conform again builds again.
- **Next work:** make the Conform GUI look more like Victoria II's own
  ("after this we gotta work on making the gui look more like v2's").

## Current baseline

- DLL **0.6.2** installed on E: (`63FB05F5…23A3`); the original Lua is
  still `lua51_real.dll` (`B2BA1934…A7B0`). Generated GUI:
  `unitpanel.gui` `058ecdc8…46fa`, `buildunit.gui` `269e603e…9df7`,
  `country_politics.gui` `41cb9356…b8ce`.
- Conform: a 20x20 button in the stack panel's free cell opens a 460x624
  window beside the panel. It holds the template list (pager past four),
  change rows (build / not built / detach) and Conform/Cancel. Everything is
  sent as engine commands: Construct, Create + Transfer, Move (safe path),
  Merge (0x4C, P45).
- 144 runtime signatures (`check_exe.py` ALL OK); DLL fixtures and 5
  Conform GUI tests pass; existing law/template gates pass.
- **Known issue:** "Conform window disabled after guarded GUI read fault",
  twice in the 0.6.2 log, each right after a new unit view appeared.
- Only the user launches or exits the game. Install only after a normal
  exit, with backups. No commit, push or subagent unless asked. The next
  pitfall is **P46**.

## Next session in order

1. **TPL-1 visual pass**: `docs/instructions-tpl-1-visual.md`.
   - First, find and fix the guarded GUI read fault (Part 1).
   - Then make the Conform button and window look like native Victoria II
     windows, against board 6 (Part 2, R10).
   - Build 0.6.3, install after a normal exit (back up 0.6.2 to
     `before-063-*`), and the player checks it.
2. **U-8 short 0.4.4 check / TPL-0 extended cases** (player):
   `docs/instructions-u-8.md`, `docs/instructions-tpl-0.md`.
3. **U-7 / LAW-2**, still pending: the short Reforms/buttons/Decisions/
   monthly-popup check (`docs/instructions-u-7.md`).
4. Open [user] question: V2DLL compatibility (the research report
   recommends keeping our own proxy).
5. Close under ROADMAP §7.

## Evidence and rollback

| What | Where / value |
|---|---|
| 0.6.2 stage | `E:\v2re\logs\tgcx-tpl1-1017\stage-062` |
| Rollback 0.6.1 | `E:\v2re\logs\tgcx-tpl-place-1017\before-062-dll\` + `before-062-submod\` |
| Rollback 0.6.0 | `before-061-dll\` + `before-061-submod\` |
| Rollback 0.5.1 | `before-060-dll\` + `before-060-submod\` |
| 0.6.1 crash analysis | `E:\v2re\logs\tgcx-tpl1-1017\merge_crash.txt` |
| Older TPL-0 rollbacks | `before-044-*` (0.4.3), `before-043-*`, `before-042-*`, `before-dll/` (0.4.0) |

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| TPL-1 | **0.6.2 Conform accepted in game**; visual pass and read fault next | instructions-tpl-1-visual.md; report-tpl-1-window-1017.md |
| U-10 | passed on 0.6.2 | instructions-u-10.md |
| TPL-0 / GH-4 | 0.4.4 accepted in game; extended U-8 cases open | instructions-tpl-0.md; report-tpl-0-visual-1017.md |
| U-8 | recruit, save/reload, placement accepted; extended cases open | instructions-u-8.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-2 / U-7 | 0.3.5 controls/reforms pending game check | instructions-u-7.md |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |
