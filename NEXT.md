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
User requested commit/push after handoff. The local commit is prepared;
public push awaits a distribution decision under AGENTS R8/§13 because
marked V2DLL-derived code is included. No game assets/build outputs staged.
