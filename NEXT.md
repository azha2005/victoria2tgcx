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
