# NEXT - what comes next

2026-10-18 (32). **U-8 complete; U-7 partly passed** on the installed
0.6.3. Nothing was built or installed this session. Read
`docs/report-u-7-1018.md` (findings F1-F4) and
`docs/report-u-8-extended-1018.md`.

## User decisions this session

- **D14 (Reforms tab offer):** both rules together. A row is offered only
  when its bill passes that bill decision's own `allow` (TGC's conditions,
  written as a flag by the monthly favour collection). Backward bills only
  where the native game would allow a backward reform. This changes the
  mechanic, so mockup board 1 changes in the session that implements it
  (R10).
- **D15:** Templates + Conform are the most important part and become
  their own package that works almost anywhere. Work follows the proposed
  order: LAW-3, then MOD-0 to MOD-5 (`docs/report-modular-scope-1018.md`
  §6.2). The open sub-decisions are in that report's §6.3.
- The order below is the coordinator's recommendation. Finishing laws first
  follows the standing rule "finish laws before starting another mechanic";
  the user can reorder it.

## Current baseline

- **Installed on E: (2026-10-18): DLL 0.7.0** (MOD-1: `lua51.dll` loader
  `2805cfcd…7144` + `tgcx.dll` `ec9ed8b9…36d7`) over the 0.6.4 submod;
  game check passed. Rollback: 0.6.5 in `before-070-dll`. 0.6.4 passed U-11.
- Earlier: **0.6.3** (DLL and submod), accepted in game. It carries
  the 0.3.5 law layer unchanged (`country_politics.gui` `41cb9356…b8ce`,
  144 guards OK).
- Stage: `E:\v2re\logs\tgcx-tpl1-1017\stage-063`; DLL
  `E:\v2re\logs\tgcx-tpl1-1017\lua51-063.dll`. Paired 0.6.2 backups:
  `E:\v2re\logs\tgcx-tpl-place-1017\before-063-dll`, `before-063-submod`.
- U-7 and U-8 evidence (private): screenshots with the user;
  `E:\v2re\logs\tgcx-u8-ext-1018\`.

## Next session in order

1. **LAW-3, Reforms tab and decisions fixes:** **built and staged
   2026-10-18 (0.6.4), done, not tested in game**
   (`docs/report-law-3-1018.md`). Stage:
   `E:\v2re\logs\tgcx-law3-1018\stage-064\` with `lua51-0.6.4.dll`.
   Install after the user's normal game exit, backups `before-064-dll` and
   `before-064-submod`.
2. **U-11, short game check of LAW-3:** **passed 2026-10-18** with one small
   text defect, G1: the » arrow is missing in the small font
   (`docs/report-u-11-1018.md`). 0.6.4 is installed on E:. Next: fix G1 and
   reset the offer on a new game (G3) as 0.6.5. G2 (Withdraw starts the
   365-day cooldown) changes only if the user decides so.
3. **V2DLL mod slot:** **decided (D16, user, 2026-10-18):** keep our own
   `lua51.dll` proxy now (a); build the `v2dll_mod.dll` plugin (b) as a card
   after MOD-1 (`docs/report-v2dll-mod-slot-1018.md` §5(b)).
4. **MOD-0 to MOD-5** (D15), per `docs/report-modular-scope-1018.md` §6.2.
   **MOD-0 in progress:** `docs/instructions-mod-0.md`. Run 1 passed
   except that Conform stopped after about a year (P47). Fix staged as
   **0.6.5** (`docs/report-tpl-1-slots-1018.md`), **installed 2026-10-18**
   (backup `E:\v2re\logs\tgcx-law3-1018\before-065-dll`); runs 2-3 next. 
   **MOD-1 built 2026-10-18 (0.7.0)**, installed, game check passed
   (`docs/report-mod-1-1018.md`): per-module gates, `tgcx.dll` behind a
   `lua51.dll` loader. **Installed 2026-10-18** after a normal exit
   (0.6.5 backed up to `before-070-dll`); **game check passed** the same day (report §6). Stage `E:\v2re\logs\tgcx-mod1-1018\stage-070\`.
5. Close under ROADMAP §7. The next pitfall is **P50**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| V2D-1 | **passed** 2026-10-18: V2DLL `de66ee61` loaded tgcx.dll 0.7.0 (`-> ok`), 7 FOREIGN, all modules on; E: restored to 0.7.0 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-2 | **ready**: instructions written, D15 sub-decisions S1-S8 answered 2026-10-18 | instructions-mod-2.md |
| MOD-1 | 0.7.0 installed on E:, **passed the game check** 2026-10-18; plugin entry checked without the game (step A) | instructions-mod-1.md; report-mod-1-1018.md |
| MOD-0 | **done** 2026-10-18 (`-mod=` order, three mods OK, TGCX alone crashed on vanilla: P49); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 | 0.6.4 installed and **passed U-11** 2026-10-18; G1 open | instructions-law-3.md; report-law-3-1018.md |
| U-11 | **passed** 2026-10-18 (G1 open) | instructions-u-11.md; report-u-11-1018.md | instructions-u-11.md |
| LAW-2 / U-7 | 0.3.5 checked in game 2026-10-18: partly passed; failures go to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** (all extended cases passed on 0.6.3) | report-u-8-extended-1018.md |
| TPL-0 / GH-4 | 0.4.4 accepted; U-8 complete; no further evidence needed | report-tpl-0-visual-1017.md |
| TPL-1 | 0.6.3 accepted in game; extended evidence optional | report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |
