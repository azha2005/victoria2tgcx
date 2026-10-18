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
- The order below is the coordinator's recommendation. Finishing laws first
  follows the standing rule "finish laws before starting another mechanic";
  the user can reorder it.

## Current baseline

- **0.6.3** (DLL and submod) installed on E:, accepted in game. It carries
  the 0.3.5 law layer unchanged (`country_politics.gui` `41cb9356…b8ce`,
  144 guards OK).
- Stage: `E:\v2re\logs\tgcx-tpl1-1017\stage-063`; DLL
  `E:\v2re\logs\tgcx-tpl1-1017\lua51-063.dll`. Paired 0.6.2 backups:
  `E:\v2re\logs\tgcx-tpl-place-1017\before-063-dll`, `before-063-submod`.
- U-7 and U-8 evidence (private): screenshots with the user;
  `E:\v2re\logs\tgcx-u8-ext-1018\`.

## Next session in order

1. **LAW-3, Reforms tab and decisions fixes:**
   `docs/instructions-law-3.md`. It covers F1 rows not re-evaluated, F2 the
   top-bar tooltip, F3 removing the load test, F4/D14 the offer rule with
   the board 1 update, and the Bill text overflow.
2. **U-11, short game check of LAW-3:** `docs/instructions-u-11.md`.
3. **V2DLL mod slot:** still an open user decision.
   `docs/report-v2dll-mod-slot-1018.md` recommends option (a), our own
   proxy.
4. Close under ROADMAP §7. The next pitfall is **P47**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| LAW-3 | **next**: F1, F2, F3, F4/D14 and Bill text overflow | instructions-law-3.md; report-u-7-1018.md |
| U-11 | after LAW-3 | instructions-u-11.md |
| LAW-2 / U-7 | 0.3.5 checked in game 2026-10-18: partly passed; failures go to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** (all extended cases passed on 0.6.3) | report-u-8-extended-1018.md |
| TPL-0 / GH-4 | 0.4.4 accepted; U-8 complete; no further evidence needed | report-tpl-0-visual-1017.md |
| TPL-1 | 0.6.3 accepted in game; extended evidence optional | report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6 | short smoke accepted; extended coverage unconfirmed | existing instructions/reports |
| HEMI-3 / U-5 | 0.2.8 accepted; extended integration unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |
