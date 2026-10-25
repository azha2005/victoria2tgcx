# NEXT - what comes next

2026-10-24 (81). **TPL-12 0.11.14 installed in the test copy. User: 0.11.13
GUI fine, but the new regiment did not go to the stack. Cause: a false
"load" dropped the order (P64); 0.11.14 needs three frames in a row.
Not tested in game.** `work/tpl12-01114/ATCU-0.11.14-candidate.zip`.
Fix: `docs/report-tpl-12-load-1024.md`. 0.11.13 reload fixes and README:
`docs/report-tpl-12-reload-1024.md`.
Earlier visual short pass (0.11.12): `docs/report-tpl-12-visual-check-1024.md`.

## Current decisions and evidence

- User: **0.11.8 works great**, but its buttons were too small. Current
  instruction: **20x20, native Disband size, beside the native header
  actions in the same row, not above**. Green Conform on the right,
  maroon Replace on the left. Current correction: native 21px pitch,
  **2px painted gaps**. Live 0.11.11 borders at y184 versus native y185
  supersede the prior lift: **use native Y**. TGC shown-depleted positions
  (172,32)/(193,32); hidden-depleted positions (193,32)/(214,32).
  Follow native button visibility, closing its empty slot and moving back.
  Long army names have less room; native name width is unchanged.
- Conform list titles are now fitted to one line with native loaded-font
  widths and ellipsis; counts stay below. Full compositions are unchanged.
  User confirms the two visible fixes worked; no fresh final screenshot.
  This accepts alignment/title appearance, not the remaining functional cases.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- Latest log snapshot: `work/tpl12-01112/check-1024/`: 206 runtime guards
  OK, no logged GUI/font faults, active error/system_error logs empty.
  Conform and Replace preview opened. No replacement queue/resume/join/
  detach result in this run; saved-order counts are inventory only.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. 0.11.8 then passed the user's general short check and supplied capture.
  Neither acceptance proves the remaining reload/multiplayer cases.
- **0.11.14 is installed** in `work/Victoria-II-TGC252-own-loader`;
  immediate 0.11.13 rollback (with log and settings): `work/tpl12-01114/before-01114`;
  0.11.12 rollback (with settings): `work/tpl12-01113/before-01113`
  (restore `tgcx_replace_orders.txt` too: 0.11.12 ignores the v2 file);
  user-tested 0.11.9 rollback: `work/tpl12-01110/before-01110`;
  0.11.11 rollback: `work/tpl12-01112/before-01112`.
  Each has 58 verified hashes.
  All 48 payload files match; 25,466 protected files and eight settings
  files unchanged. Main E: install unchanged.
- All new DLL code is Rust (`rust/tgcx`); C++ only receives generated
  headers. Do not build a C++ release from these GUI headers.
- TPL-8 0.11.3 and TPL-11 0.11.6 passed their short player checks.
  TPL-11 is committed at `4ad923d`; current TPL-12 work is uncommitted.
- Session-start NEXT archived verbatim in `docs/archive/sessions.md`.

## Next session in order

| Card | Status | Instructions |
|---|---|---|
| TPL-12-CHECK | 0.11.14 installed; GUI passed; march/merge/detach and reload open | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: use `docs/instructions-tpl-12-check.md` steps 1-4, starting
   with the 9 February save (stranded regiment must resume).
   Confirm choices/persistence; queue a qualifying replacement, save
   during construction and again between merge and detach (0.11.13), reload, then follow march/merge and verified detach
   of the old regiment. Include yellow-pop, culture and land-shortage cases
   when available. Preserve log/screenshots. Remaining lifetime and native
   visibility transitions need explicit confirmation where not observed.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. The Conform long-title visual fix is accepted in 0.11.12.
   Pale-yellow Changes shortfalls remain a deferred look fix, unselected.

## Parked work

- TPL-9 Autodivide: needs the user's split rule, button place and leader
  choice before implementation.
- TPL-10 Carpet siege: depends on TPL-9; user choices still open.
- RUST-CHECK: full Bill/favour/remaining Conform coverage is separate;
  `docs/instructions-rust-check.md`.
- RENAME-026: local ZIP only, publication deferred;
  `docs/instructions-rename-026.md`.

Installation completed on explicit user instruction with clear process
gates. No public release, commit/push or game launch/termination.
Next pitfall is P65.
