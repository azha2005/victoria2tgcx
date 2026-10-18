# NEXT - what comes next

2026-10-18 (31). **0.6.3 (332x594 native army-panel look and flush placement) installed on E:, tested and accepted in game** (user screenshot + "works great!").
0.6.3 is the working baseline; paired 0.6.2 backups are retained. See
`docs/report-tpl-1-visual-1017.md`.

## This session

- Refined `tools/tgcx_conform_gui.py` to match the native `332 px` army panel:
  top and bottom gold braided headers from `GFX_unit_toppanel_bg`, unstretched
  parchment middle and recessed bottom tray from `GFX_unit_bottom_bg`, clean
  `317x30` regiment rows (`GFX_unit_entry_bg` without the right-hand POP/bar
  stripes), `GFX_unit_bg_28` section header for `Changes`, and `148x28`
  bevelled unit-panel footer buttons (`GFX_unit_gunboat_diplo_button`).
- Placed the Conform window flush beside the unit panel at `(x+331, y+26)` in
  `tgcx_CfPlace` (`dll/tgcx/tgcx_conform_gui.h`) so its top gold border aligns
  with `unitpanel_bg`.
- Tested and accepted in game by the user.

## Next session in order

1. U-8 extended cases: `docs/instructions-u-8.md`, `docs/instructions-tpl-0.md`.
3. U-7 short law/reforms check: `docs/instructions-u-7.md`.
4. V2DLL compatibility remains an open user decision (own proxy recommended).
5. Close under ROADMAP section 7. Next pitfall remains P46.

Only the user launches/exits the game. Installation requires their normal
exit confirmation. No commit, push or subagents unless asked.

## Artifacts

- Stage: `E:\v2re\logs\tgcx-tpl1-1017\stage-063`.
- Candidate DLL: `E:\v2re\logs\tgcx-tpl1-1017\lua51-063.dll`.
- Hashes and preview limits: `docs/report-tpl-1-visual-1017.md`.
- Baseline log retained beside stage as `baseline-before-063.log`.
- Paired 0.6.2 backups: `E:\v2re\logs\tgcx-tpl-place-1017\before-063-dll` and `before-063-submod`; installed hashes verified.
- Previous rollback paths are in the archived session.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| TPL-1 | **0.6.3 Conform army-style GUI accepted in game**; extended evidence remains optional | instructions-tpl-1-063-check.md; report-tpl-1-visual-1017.md |
| U-10 | passed on 0.6.2 | instructions-u-10.md |
| TPL-0 / GH-4 | 0.4.4 accepted in game; extended U-8 cases open | instructions-tpl-0.md; report-tpl-0-visual-1017.md |
| U-8 | recruit, save/reload, placement accepted; extended cases open | instructions-u-8.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-2 / U-7 | 0.3.5 controls/reforms pending game check | instructions-u-7.md |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | existing instructions |
| COMPAT-0 | deferred D1 | existing instructions |
