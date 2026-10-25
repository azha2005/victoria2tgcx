# Session80 - 0.11.12 visual short pass and log audit (2026-10-24)

User: it worked, still lacks testing; asks to check logs. Preserved latest
custom/active-profile logs. 206 runtime guards pass, error logs empty,
Conform/Replace preview opened, no queue/resume/merge/detach outcomes.
Record alignment/title visual acceptance only; functional checks remain.
report-tpl-12-visual-check-1024.md. No code/install/game controls.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (79). **TPL-12 native Y and single-line titles, Rust 0.11.12, installed and verified
in the own-loader TGC test copy; not tested in game.**
`work/tpl12-01112/ATCU-0.11.12-candidate.zip` includes README and changelog.
Build/install: `docs/report-tpl-12-title-alignment-1024.md`.

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
  This fixes the reported Hus/count collision, pending the player retry.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. 0.11.8 then passed the user's general short check and supplied capture.
  Neither acceptance proves the remaining reload/multiplayer cases.
- **0.11.12 is installed** in `work/Victoria-II-TGC252-own-loader`;
  User-tested 0.11.9 rollback: `work/tpl12-01110/before-01110`;
  immediate 0.11.11 rollback: `work/tpl12-01112/before-01112`.
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
| TPL-12-CHECK | 0.11.12 installed/verified; alignment/title check next | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: the user launches the existing test copy and follows
   `docs/instructions-tpl-12-check.md` from step 4. Check the two header
   buttons' native border alignment/gaps and shifting as the native depleted button
   hides/shows, long/short one-line titles and separate counts, independent
   chooser, culture choices and persistence,
   replacement merge/detach and save/reload. Preserve screenshots and log.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. The Conform long-title issue is implemented in 0.11.12; verify it above.
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
Next pitfall remains P64.


---

# Session79 - TPL-12 live alignment and title wrapping fix (2026-10-24)

Live 0.11.11 borders at y184 versus native y185 prove the lift wrong.
User also reports Hus wrapping over the count. Rust 0.11.12 uses
native Y and one-line titles fitted with the actual loaded font.
Built/tested/packaged/installed with 0.11.11 rollback; player retry next.
report-tpl-12-title-alignment-1024.md.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (78). **TPL-12 adaptive header, Rust 0.11.11, installed and verified
in the own-loader TGC test copy; not tested in game.**
`work/tpl12-01111/ATCU-0.11.11-candidate.zip` includes README and changelog.
Build/install: `docs/report-tpl-12-spacing-1024.md`.

## Current decisions and evidence

- User: **0.11.8 works great**, but its buttons were too small. Current
  instruction: **20x20, native Disband size, beside the native header
  actions in the same row, not above**. Green Conform on the right,
  maroon Replace on the left. Current correction: native 21px pitch,
  **2px painted gaps**, and **up 1px**. TGC shown-depleted positions
  (172,31)/(193,31); hidden-depleted positions (193,31)/(214,31).
  Follow native button visibility, closing its empty slot and moving back.
  Long army names have less room; native name width is unchanged.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. 0.11.8 then passed the user's general short check and supplied capture.
  Neither acceptance proves the remaining reload/multiplayer cases.
- **0.11.11 is installed** in `work/Victoria-II-TGC252-own-loader`;
  User-tested 0.11.9 rollback: `work/tpl12-01110/before-01110`;
  immediate 0.11.10 rollback: `work/tpl12-01111/before-01111`.
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
| TPL-12-CHECK | 0.11.11 installed/verified; spacing/shift/lift check next | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: the user launches the existing test copy and follows
   `docs/instructions-tpl-12-check.md` from step 4. Check the two header
   buttons' native gaps/1px lift and shifting as the native depleted button
   hides/shows, independent chooser, culture choices and persistence,
   replacement merge/detach and save/reload. Preserve screenshots and log.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. Preserve the two deferred small look fixes: long template names can
   wrap over the regiment-count line; Changes shortfalls are pale yellow.
   No implementation selected for them in this session.

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
Next pitfall remains P64.


---

# Session78 - TPL-12 native gap, adaptive slot and 1px lift (2026-10-24)

0.11.9 captures expose excess gap and unused native depleted slot. User
requests native spacing, following native visibility, then a 1px lift.
Rust 0.11.11 built/tested/packaged/installed with 0.11.9 and interim
0.11.10 rollbacks. New appearance/transitions await player verification.
report-tpl-12-spacing-1024.md.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (77). **TPL-12 larger header buttons, Rust 0.11.9, installed and verified
in the own-loader TGC test copy; not tested in game.**
`work/tpl12-0119/ATCU-0.11.9-candidate.zip` includes README and changelog.
Build/install: `docs/report-tpl-12-beside-1024.md`.

## Current decisions and evidence

- User: **0.11.8 works great**, but its buttons were too small. Current
  instruction: **20x20, native Disband size, beside the native header
  actions in the same row, not above**. Green Conform on the right,
  maroon Replace on the left, 3px gaps. TGC positions (168,32)/(191,32).
  Long army names have less room; native name width is unchanged.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. 0.11.8 then passed the user's general short check and supplied capture.
  Neither acceptance proves the remaining reload/multiplayer cases.
- **0.11.9 is installed** in `work/Victoria-II-TGC252-own-loader`;
  0.11.8 rollback: `work/tpl12-0119/before-0119` (58 verified hashes).
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
| TPL-12-CHECK | 0.11.9 installed/verified; new placement check next | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: the user launches the existing test copy and follows
   `docs/instructions-tpl-12-check.md` from step 4. Check the two header
   buttons beside the native actions, independent chooser, culture choices and persistence,
   replacement merge/detach and save/reload. Preserve screenshots and log.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. Preserve the two deferred small look fixes: long template names can
   wrap over the regiment-count line; Changes shortfalls are pale yellow.
   No implementation selected for them in this session.

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
Next pitfall remains P64.


---

# Session77 - TPL-12 0.11.9 native-sized buttons beside header actions (2026-10-24)

User accepts 0.11.8 short check and requests larger buttons at the top;
clarifies same row beside the native actions, not above. Rust 0.11.9
built/tested/packaged/installed in the existing test copy. 0.11.8 rollback
retained; new placement awaits player check. report-tpl-12-beside-1024.md.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (76). **TPL-12 option A, Rust 0.11.8, installed and verified
in the own-loader TGC test copy; not tested in game.**
`work/tpl12-0118/ATCU-0.11.8-candidate.zip` includes README and changelog.
Build: `docs/report-tpl-12-header-1024.md`.
Install: `docs/report-tpl-12-install-1024.md`.

## Current decisions and evidence

- The user chose **A**: both army-header buttons 16x16, 3px gap, clear of
  the braid; green Conform on the right, maroon Replace on the left.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. This does not validate the new 0.11.8 GUI or reload.
- **0.11.8 is installed** in `work/Victoria-II-TGC252-own-loader`;
  0.11.7 rollback: `work/tpl12-0118/before-0118` (59 verified hashes).
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
| TPL-12-CHECK | 0.11.8 installed/verified; player check next | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: the user launches the existing test copy and follows
   `docs/instructions-tpl-12-check.md` from step 4. Check the two header
   buttons, independent chooser, culture choices and persistence,
   replacement merge/detach and save/reload. Preserve screenshots and log.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. Preserve the two deferred small look fixes: long template names can
   wrap over the regiment-count line; Changes shortfalls are pale yellow.
   No implementation selected for them in this session.

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
Next pitfall remains P64.


---

# Session76 - TPL-12 0.11.8 installed (2026-10-24)

User explicitly selected the own-loader TGC test copy for installation.
Process and executable gates passed; 59-file rollback verified, 48 payload
hashes matched, 25,466 protected files and eight settings files unchanged.
No game launch. Player check next; report-tpl-12-install-1024.md.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (75). **TPL-12 option A finished in Rust 0.11.8; local ZIP ready,
not installed or tested in game.**
`work/tpl12-0118/ATCU-0.11.8-candidate.zip` includes README and changelog.
Report: `docs/report-tpl-12-header-1024.md`.

## Current decisions and evidence

- The user chose **A**: both army-header buttons 16x16, 3px gap, clear of
  the braid; green Conform on the right, maroon Replace on the left.
- Replace opens independently where Conform opens. Each closes the other.
  Conform returns to its Conform/Cancel footer. Replace has its own culture
  boxes; none means all, selected box clicks clear it. Choice settings v2
  import v1. Replacement orders keep their format.
- **0.11.7 passed a short gameplay check**, including replacement merge and
  old-regiment detach (report-tpl-12-replace-1024.md section 9). Strength
  scale was confirmed. This does not validate the new 0.11.8 GUI or reload.
- **0.11.7 remains installed** in `work/Victoria-II-TGC252-own-loader`;
  its rollback is `work/tpl12-0117/before-0117`. Main E: install unchanged.
- All new DLL code is Rust (`rust/tgcx`); C++ only receives generated
  headers. Do not build a C++ release from these GUI headers.
- TPL-8 0.11.3 and TPL-11 0.11.6 passed their short player checks.
  TPL-11 is committed at `4ad923d`; current TPL-12 work is uncommitted.
- Session-start NEXT archived verbatim in `docs/archive/sessions.md`.

## Next session in order

| Card | Status | Instructions |
|---|---|---|
| TPL-12-CHECK | 0.11.8 built/packaged; install and player check next | `docs/instructions-tpl-12-check.md` |
| TPL-11-CHECK | Rally selector/gather short check passed; save/reload open | `docs/instructions-tpl-11-check.md` |

1. TPL-12-CHECK: after the user's normal-exit confirmation, back up and
   install only in the existing isolated own-loader TGC test copy. Verify
   payload hashes. The user launches and checks the two header buttons,
   independent chooser, culture choices and their persistence, replacement
   merge/detach and save/reload. Record screenshots and log.
2. TPL-11-CHECK: when selected, finish save/reload of a rally gather.
3. Preserve the two deferred small look fixes: long template names can
   wrap over the regiment-count line; Changes shortfalls are pale yellow.
   No implementation selected for them in this session.

## Parked work

- TPL-9 Autodivide: needs the user's split rule, button place and leader
  choice before implementation.
- TPL-10 Carpet siege: depends on TPL-9; user choices still open.
- RUST-CHECK: full Bill/favour/remaining Conform coverage is separate;
  `docs/instructions-rust-check.md`.
- RENAME-026: local ZIP only, publication deferred;
  `docs/instructions-rename-026.md`.

No public release, commit/push, game launch/termination or installation
was performed. Next pitfall remains P64.


---

# Session75 - TPL-12 option A finished, 0.11.8 local ZIP (2026-10-24)

Resumed the unfinished independent header-button layout after usage ended.
Option A implemented in Rust: two 16px buttons, restored Conform footer,
independent 332x270 Replace chooser and saved culture settings v2 (v1 import).
Built/tested/package-verified; README and changelog in local ZIP.
Not installed or tested in game. report-tpl-12-header-1024.md.

## NEXT at the start (verbatim)

# NEXT - what comes next

2026-10-24 (74). **TPL-12 Replace depleted built in Rust; 0.11.7 installed
in `work/Victoria-II-TGC252-own-loader` (backup `work/tpl12-0117/before-0117/`),
not tested in game** (`docs/report-tpl-12-replace-1024.md` §6-8; check in §7). TPL-11 is committed (`4ad923d`).
Before it:
2026-10-24 (74). **TPL-11 0.11.6 passed in game** (user: "done. wonderful"):
Gather at cycles every land rally point
(candidate `work/tpl11-0116/`, installed in the test copy, backup
`work/tpl11-0116/before-0116/`; `docs/report-tpl-11-rally-1024.md` §8).
Before it: **TPL-11 0.11.5: Gather at's third option is the land
rally point** (`docs/report-tpl-11-rally-1024.md`), candidate
`work/tpl11-0115/`, installed in `work/Victoria-II-TGC252-own-loader`
(backup `work/tpl11-0115/before-0115/`), not tested in game. 0.11.4 worked in game for Best
supply, Capital and armies beside a taken target.
Before it: **TPL-11 Build Army gathering built in Rust; 0.11.4
candidate installed in `work/Victoria-II-TGC252-own-loader` (backup
`work/tpl11-0114/before-0114/`), not tested in game**
(`docs/report-tpl-11-gather-1024.md`, check `docs/instructions-tpl-11-check.md`).
Before it: **TPL-8 Non-core only built in Rust; 0.11.3 candidate ready.**
Installed in `work/Victoria-II-TGC252-own-loader` (backup
`work/noncore-0113/before-0113/`). **Passed in game** (user): Non-core
recruit, Conform detach/build, resume after load and merge; v2 untouched.
Not committed. Session 73 NEXT
archived verbatim in docs/archive/sessions.md.

## Current scope and evidence

- **All new DLL code is Rust** (user, 2026-10-24): `rust/tgcx/`, in
  fedacking's style. The C++ DLL is not extended; only its generated headers
  follow. These headers now describe the 0.11.3 GUI. Do not rebuild the C++
  DLL from them for a release: its Non-core box would do nothing.
- TPL-8 (D19): third exclusive box in Templates and Conform; Conform uses two
  lines and shows 6 Changes rows (was 7), with overflow in the summary. Settings
  in `tgcx_templates_v3.txt`; v2/v1 are imported, never rewritten.
  `docs/report-tpl-8-noncore-1024.md`.
- Candidate `work/noncore-0113/ATCU-noncore-0.11.3-test.zip` (`9801d0b1...`):
  the **first packaged Rust logic DLL** (`4960aa2f...`), with the accepted C++
  loader. Previews: `mockups/5-templates-noncore-1024.png`,
  `6-conform-noncore-1024.png` (Arial, not captures).
- PR #3 mobo stacks merged at `abc6806`. Player crash report on a low-pop
  Recruit is not reproduced; reporter's `tgcx.log`, ATCU version and mod list
  are still awaited.
- The release is still C++ v0.0.2. The Rust DLL is not deployed or released.

## Next session in order

1. **Small look fixes seen during TPL-8 (pre-existing):** long template names
   wrap over their "N regiments" line in Conform's list; the left-out
   Changes title is pale yellow on parchment.
2. **TPL-11** passed in game on 0.11.6. Open: save/reload of a rally
   gather; commit when the user asks. `docs/instructions-tpl-11-check.md`.
   Built 2026-10-24 (D20). Install only after a normal-exit confirmation.
3. **TPL-9 Autodivide.** Base: Conform's detach (Create + Transfers together,
   P51). Open [user]: the rule (single regiments, N equal stacks, template
   copies), the button's place, who keeps the leader.
4. **TPL-10 Carpet siege.** Needs TPL-9 plus one safe-path Move per stack.
   Open [user]: which provinces, regiments per province, one-off click or a
   running order that re-targets.
5. **TPL-12 Replace depleted regiments.** Built 2026-10-24 (0.11.7,
   D21): Conform's Replace depleted opens a chooser (low strength below a
   10-90% slider, pop too small red or yellow); replacements built first,
   the old regiment detached once its replacement merged. Installed in
   the test copy; next: the check in `docs/report-tpl-12-replace-1024.md` §7.

Every Army card is Rust, needs its mockup (R10) and its [user] questions
answered (R9) before code. Full card text: docs/archive/sessions.md,
"Next up after 0.0.2".

Alongside, when selected:

- **RUST-CHECK:** `docs/instructions-rust-check.md` (Bill, favours, the
  rest of Conform; the TPL-8 check covers part of it).
- **RENAME-026:** `docs/instructions-rename-026.md` (C++, packaged; optional
  final 2px look check). No publication implied.

Next pitfall remains P64.


---

# Session74 - TPL-8 Non-core only built in Rust, 0.11.3 candidate (2026-10-24)

Implemented D19 in Rust: culture mode 3 (neither primary nor accepted) in the
shared filter, third box in Templates, two-line culture band in Conform (6
Changes rows), tgcx_templates_v3.txt importing v2 then v1 and never rewriting
them. Headers regenerated; marker TGCX TPL-8 0.11.3. 5 new Rust tests, layout
tests, check_exe and the Python suite pass (Rename package test needs its own
DLL). Local candidate work/noncore-0113/ATCU-noncore-0.11.3-test.zip, first
packaged Rust logic DLL. Not installed, not tested in game, not committed.
report-tpl-8-noncore-1024.md; instructions-tpl-8-check.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-24 (73). **PR #3 (fedacking, mobo stacks) reviewed and merged.**
Local master and origin/master are at `abc6806`. Session 72 NEXT archived
verbatim in docs/archive/sessions.md.

## Current scope and evidence

- **All new DLL code is Rust** (user, 2026-10-24): `rust/tgcx/`, in
  fedacking's style (`guard(..)` around native work, `log!` lines, short doc
  comments; `cargo clippy --release -- -D warnings` and `cargo fmt` clean).
  The C++ DLL is not extended; only its generated headers follow.
- PR #3: a "Mobo stack" box in the Conform title marks a stack; during a
  mobilization, new all-infantry stacks march to the nearest marked stack and
  merge if they fit. Marks in `tgcx_mobo_marks.txt`. Six MOBO signatures in
  `check_exe.py`, all OK on the reference exe; clippy/fmt clean. Tested in
  game by fedacking only; `docs/symbols.md` "MOBO-1".
- Player report (Discord, 2026-10-24): crash on Recruit Template with too
  few soldier pops. **Not reproduced** in `work/Victoria-II-TGC252-own-loader`
  (exact v0.0.2 DLL `7afefc61...`): 92 recruits, including partial (7 queued,
  5 left out) and zero-pop clicks, no fault. Reporter's `tgcx.log`, ATCU
  version and mod list requested; read them when they arrive.
- Release builds still use C++; the Rust DLL is not deployed or released.

## Next session in order

The Army feature queue from the v0.0.2 release notes, order set by the user
2026-10-24. Every card is Rust, needs its mockup (R10) and its [user]
questions answered (R9) before code. Full card text: docs/archive/sessions.md,
"Next up after 0.0.2".

1. **TPL-8 Non-core culture mode.** Design approved (D19): third exclusive
   checkbox in Templates and Conform, none = All, clicking the selected box
   clears it, Conform may use two lines. Ready to build: follow
   `docs/instructions-tpl-8.md` (the Rust-coordination hold is over), in
   `templates.rs`, `templates_gui.rs`, `conform_gui.rs`. Fit around the mobo
   box in the Conform title.
2. **TPL-11 Build Army gathers into one army.** Reuses Conform march/merge,
   TPL-7 saved orders and the mobo assign-and-march path. Open [user]: the
   gathering province (player pick, capital, largest source province or the
   native rally point); regiments without a land route (D17: leave them).
3. **TPL-9 Autodivide.** Base: Conform's detach (Create + Transfers together,
   P51). Open [user]: the rule (single regiments, N equal stacks, template
   copies), the button's place, who keeps the leader.
4. **TPL-10 Carpet siege.** Needs TPL-9 plus one safe-path Move per stack.
   Open [user]: which provinces, regiments per province, one-off click or a
   running order that re-targets.
5. **TPL-12 Replace depleted regiments.** Needs research first: the regiment
   strength field and the native disband command (R5). Open [user]: the
   threshold, disband vs detach to a reserve, build-first order, separate
   button or Conform option.

Alongside, when selected:

- **RUST-CHECK:** `docs/instructions-rust-check.md`. Full Rust Templates
  recruit, Conform with save/reload, mobo stacks, Bill and favours in a
  separate candidate copy; no release default switched automatically.
- **RENAME-026:** `docs/instructions-rename-026.md` (C++, packaged; optional
  final 2px look check). No publication implied.

Next pitfall remains P64.

# Session73 - PR 3 mobo stacks merged; Army feature order set (2026-10-24)

Reviewed and merged fedacking PR 3 (mobo stacks, abc6806): clippy, fmt and
all exe signatures pass. Local master fast-forwarded with local edits kept
(symbols.md: MOBO-1 and RENAME-0 sections both retained). User rule: all
new DLL code in Rust, fedacking style. A player-reported crash on Recruit
with too few soldier pops did not reproduce on the exact v0.0.2 DLL (92
recruits incl. partial and zero-pop, no fault); logs requested from the
reporter. User set the Army feature order below.

## NEXT.md at session start

# NEXT - what comes next

2026-10-22 (72). **PR #2 reviewed, merged and pulled.** Local master and
origin/master are at `09a361d68dbdfaca3a11dd8b93948c9315edc8f4`, incorporating
both Rust PRs. All pre-existing unpublished Province Rename work is restored.
Starting NEXT archived verbatim in docs/archive/sessions.md.

## Current scope and evidence

- PR #2 adds `rust/tgcx/`, the full TPL-7 0.11.2 DLL port. PR #1 adds
  `rust/loader/` and C++ compatibility fixes. Build/release tools still
  use C++; pulling does not switch the deployed implementation.
- Rust loader and logic release builds, fmt and strict Clippy pass. The
  Rust DLL passes the actual plugin probe and three isolated review tests
  (repeated/nested SEH recovery, SHA vectors, parser/planning). Existing
  C++ native/GPU regression and 12 installer tests pass. These do not
  establish full Rust gameplay parity.
- Local work backed up in `work/pr2-local-backup-1022/local-work.zip` with
  a verified 61-file manifest; a named Git stash is also retained. Sixty
  files restored byte-for-byte; `tools/check_exe.py` combines the upstream
  environment override and local Rename signatures, and passes its gate.
- No game installation, launch, termination or release occurred. Province
  Rename remains unpublished. The TPL-8 implementation hold remains until
  explicit resumption; the prior loader-only scope is now superseded by PR #2.
- Report: `docs/report-pr-2-review-1022.md`.

## Next session in order

1. **RUST-CHECK, only when selected:** follow `docs/instructions-rust-check.md`.
   Full Rust Templates recruit, Conform detach/build/march/merge/save-reload,
   Bill drawing/purchases/count and favour accrual remain unverified here.
   Use a separate candidate; do not switch release defaults automatically.
2. **TPL-8 held:** follow `docs/instructions-tpl-8.md`. Approved Non-core only
   design is unchanged. Reassess both C++ and Rust implementations when the
   user resumes work; upstream integration itself is complete.
3. **RENAME-026:** follow `docs/instructions-rename-026.md` for the existing
   local ZIP and optional final 2px appearance check. No publication or new
   install implied. Existing package, backup and remaining coverage details
   are retained in `docs/report-province-rename-package-1021.md`.

The unselected future feature queue is retained in the archived session 71
NEXT and ROADMAP/SUBAGENTS. Next pitfall remains P64.

---

# Session70 - TPL-8 design approved, held for Rust coordination (2026-10-21)

User selects easier ATCU work and approves a third mutually exclusive
Non-core only checkbox in both Templates/Conform, none=All/click clears,
two lines allowed in Conform. Before implementation user says wait for
fedacking's Rust port. Verified merged PR#1 ports the loader and includes
C++ build fixes; local HEAD is its pre-merge base045bbfd5. No ATCU source,
headers/assets/build/game/settings changes made. Approved design retained;
feature work held pending port coordination/upstream reconciliation.
Province Rename local ZIP/installed0.2.6 untouched and unpublished.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (70). **Province Rename0.2.5 accepted; final ABC-adjusted0.2.6
installed and local ZIP ready. User explicitly says DO NOT RELEASE YET.**
Session69 starting NEXT archived verbatim in docs/archive/sessions.md.

## Current scope and evidence

- User says0.2.5 works awesome. Screenshot/log confirms short dialog,
  typing/caret and repeated native renames with map/header updates.
- 0.2.6 raises24px ABC2px and changes name gap8px to10px. Native font/name
  anchor and clamping retained; accepted editor/caret/commands unchanged.
  Final2px visual check itself remains pending. No new addresses/pitfall.
- Initial request to release was superseded explicitly: dont release it
  yet, just give me the zip. No commit/push/tag/remote creation/publication.
  Public-release README/notes are prepared prospective material only.
- Package is Universal additive V2DLL, reference3.04/current V2DLL required.
  No dependencies/user_dir/replace_path/base GUI/data override. Native
  command0x707 only changes simulation. Other-base gameplay, ATCU
  coexistence, extended lifetime/save-reload and multiplayer remain untested.
- Current report docs/report-province-rename-package-1021.md. Current board
  docs/province-rename-layout.svg; previous025 board archived verbatim in
  docs/archive/province-rename-layout-025.svg. Native art stays ignored.

## Local deliverable and installed copy

- ZIP work/releases/province-rename/0.2.6/Province-Rename-Universal-0.2.6.zip:
  12files,264502bytes. SHA256
  fcc68a16c0987c5f947d8ada1fde7060c85a2e2b3f51da4d703f8ed05055ce9f.
  Adjacent .zip.sha256, SHA256.json, PROVENANCE.json and RELEASE.json.
  published=false; no destination or source commit selected. ZIP's11mod
  payload members byte-identical to checked0.2.6 candidate; README only differs.
- work/Victoria-II-Province-Rename-TGC252 updated after fresh user Yes,
  closed normally confirmation and process gate.11payload/25,388protected
  hashes pass; protected unchanged. Full0.2.5/guide rollback before-026 in
  work/province-rename/release-026-1021/, exact patch-026-installed.json.
  Earlier backups retained. No game launch/termination/main/other-copy change.
- Stock Lua, canonical TGC, V2DLL5.98/cdad55c7 and independent profile kept.
- 301guards/78Rename runtime, Win32 build,66native/4package/4actual-loader
  checks pass. Five-base checks are static, not gameplay. Native-art pixels
  and binaries remain in ignored work/. No memory update. Next pitfall P64.

## Next session in order

1. Deliver local ZIP (and optional .sha256). No publication or new install.
   Current instructions docs/instructions-rename-026.md.
2. If user wants final appearance check, they launch dedicated copy with
   Province Rename (Universal)+TGC v2.5.2 (stable), inspect2px higher ABC/
   10px name gap and recheck opening/typing/rename. Expect0.2.6 Universal,
   78native signatures in Logs/tgcx_province_rename.log. Preserve fresh logs/
   unedited captures after normal exit. Final positioning unconfirmed yet.
3. Full limits/ownership/lifetime/save-reload per instructions-rename-020.md
   sections2-3; other-base gameplay/ATCU/MP only when selected later.
4. Publishing remains expressly deferred. If user asks again, confirm exact
   destination then finish source/release identity before publishing. Do not
   treat the old release request as standing authorization. Installation
   still requires fresh user exit/install instruction and process gate.
   Never launch/terminate the game autonomously; keep all rollbacks.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0/U/025 | short rename/editor/typing/caret accepted; full cases pending | instructions-rename-020.md |
| RENAME-026 | final ABC tweak installed, ZIP ready; publication deferred by user | instructions-rename-026.md |
| RENAME-MP | theoretical only; no host/client/OOS test | instructions-rename-020.md section4 |
| TPL-5/6 |0.11.2 retained; extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; separate from Rename | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6/3/4/5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".

---

# Session69 - accepted Rename, final ABC adjustment and ZIP only (2026-10-21)

User accepts0.2.5, requests better ABC placement and initially release.
0.2.6 raises ABC2px and increases name gap2px.301guards/66native/4package/
4loader checks pass. Publication then explicitly cancelled: just give ZIP.
Local12-file beta ZIP prepared/verified with README/provenance/hash sidecars;
no source commit/push/publication. Installed in dedicated copy after fresh
normal-exit confirmation,11payload/25,388protected hashes pass, before-026
retained. Final2px visual check pending; accepted dialog unchanged.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (69). **Province Rename0.2.4 short operation nearly perfect;
checked0.2.5 caret/ABC/readability refinement installed, player retry next.**
Session68 starting NEXT archived verbatim in docs/archive/sessions.md.

## Current decisions and evidence

- User's0.2.4 screenshot/log confirms working centered typing and short
  native rename/header/map update and reopen. Remaining feedback: caret
  displaced, ABC a little too big, input font too thin/hard to read.
- 0.2.5 retains the working single-origin text refresh, compensates native
  cursor transparent margins and its3px gap using caret-only offsets,
  changes native ABC28x28 to24x24 and uses native Main_16_black (Garamond16
  bold) for typed input. Native white/green/gold/parchment redesign unchanged.
- Cursor is32x32 with1x16 visible ink at(7,6)..(8,22). P63 documents why
  container alignment did not align that line. Actual native caret update
  consumes a frame argument, RET4; the initial added fixture omitted it and
  crashed only tgcx_rename_tests.exe. Corrected ABI is guarded.66native
  checks pass, including native caret update/getter at16/18px font heights.
- Universal additive V2DLL package, reference3.04/current V2DLL required;
  no dependencies/user_dir/replace_path/base GUI override. Simulation only
  via native command0x707. Other-base gameplay, ATCU coexistence and MP
  remain untested. Army/Politics follow-ups remain separate.
- Current report docs/report-province-rename-readability-1021.md; historical
  aesthetic/implemented/centering reports retained unchanged. Current board
  docs/province-rename-layout.svg; previous024 board archived verbatim in
  docs/archive/province-rename-layout-024.svg. Native pixels stay ignored
  in versioned work/ assets. Preview is not game acceptance.

## Installed copy and validation

- work/Victoria-II-Province-Rename-TGC252, canonical mod/TGC, stock Lua,
  V2DLL5.98/cdad55c7; profile TGC252_ProvinceRename_20261020 retained.
- User answered done to fresh normal-exit confirmation for0.2.5. Process
  gate clear throughout update.11payload/25,388protected hashes verified;
  protected unchanged. Full0.2.4/guide backup readability-025-1021/before-025;
  exact patch-025-installed.json beside it. All previous backups retained.
- ZIP work/province-rename/candidate/Province-Rename-Universal-0.2.5-test.zip,
  12files/263759bytes; SHA256
  9ffd4a032687348ab88285d519350f47099f77e7bbb2a29994f7907acf0d3c24.
- 301 executable guards/78 Rename runtime, Win32 build,66native/4package/
  4 actual-loader checks pass. Five-base font/sprite/native cursor bounds
  checks are static. Keyboard/caret cache/scrolling timing still need player
  verification. No full lifetime/persistence/gameplay matrix claim.
- Evidence work/province-rename/readability-025-1021/. User screenshots,
  0.2.4 log, native exports/build/tests/loader results retained.
- No game launch/termination or main E:/D:/other-copy change. Only our own
  crashed isolated test process was closed. No release/publication, source
  commit, push or memory update. Next pitfall P64.

## Next session in order

1. **RENAME-025 player retry**, docs/instructions-rename-025.md. No install
   needed. User opens dedicated copy with Province Rename (Universal) and
   TGC v2.5.2 (stable), selects owned land and opens24px ABC. Check clearer
   centered Garamond typing, visible caret beside/vertically aligned with
   text, no flicker through typing/deletion/idle, mouse/Home/End/accents/
   long names/paste and Cancel/Rename/reopen/map/header behavior. Expect
   0.2.5 Universal,78 native signatures and single native text/caret origin
   in Logs/tgcx_province_rename.log; V2DLL logs in V2DLL/Logs/.
2. Complete full limits/stale selection/ownership/lifetime/save-reload cases
   per instructions-rename-020.md sections2-3 with0.2.5/78 expectations.
   Previous short success does not prove these extended cases.
3. Preserve fresh logs/unedited captures after user normal exit. Review
   applicable pitfalls, especially P31/P58-P63 before fixes. Native fixture
   calls need actual RET/ABI evidence. New installation requires fresh user
   exit/install instruction and clear process gate. Never launch/terminate
   the game autonomously; keep every rollback.
4. Other-base/ATCU/MP remain later selected work; no additional copy update
   or publication automatically. Army follow-ups below remain separate.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0/U/021 | short rename/ABC accepted; extended checks pending | instructions-rename-020.md |
| RENAME-023/024 | redesign and short centered typing work; caret refinement next | report-province-rename-centering-1021.md |
| RENAME-025 | caret/24px ABC/Garamond refinement installed, retry next | instructions-rename-025.md |
| RENAME-MP | theoretical only; no host/client/OOS test | instructions-rename-020.md section4 |
| TPL-5/6 |0.11.2 retained; extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6/3/4/5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".

---

# Session68 - caret alignment and readability refinement (2026-10-21)

User confirms short0.2.4 operation nearly perfect; requests caret alignment,
smaller ABC and more readable font.0.2.5 uses caret-only offsets for native
sprite ink,24x24 ABC and native Garamond16 bold. Added native caret regression
initially crashed the isolated test from missing RET4 argument; corrected
and guarded.301guards/66native/4package/4loader checks pass. Installed after
fresh normal-exit confirmation,11payload/25,388protected hashes verified;
full before-025 rollback. P63, next P64; player retry next. No game control,
main/other-copy update, publication, commit or push.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (68). **Province Rename redesign works; checked 0.2.4 single-offset
centering correction installed. Player retry next.** Session67 starting
NEXT archived verbatim in docs/archive/sessions.md.

## Decisions and current evidence

- The aesthetic redesign handoff is implemented: native white title on
  green/gold, continuous parchment, no divider/footer shelf, double input
  braids, centered black serif captions and blue/gold native buttons.
  Latest user instruction requires typed text centered horizontally too.
  Icon-only native28x28 ABC remains right of the actual province name.
- User tried installed0.2.3: works, but recenter flickers and is off-center.
  Log confirms a successful province425 Jijolandia rename/header update
  and reopen. The input border offsets were used twice, in text texture
  rasterization and native entity placement. P62 documents the failure.
- 0.2.4 supplies zero texture padding during native refresh, then restores
  centered text/caret insets and performs native placement synchronously.
  Actual native refresh/setter/position regression reproduces the old
  double offset and validates a stable origin across repeated positioning.
  Full live keyboard/mouse/scrolling/flicker acceptance remains pending.
- Universal additive V2DLL package, reference3.04/current V2DLL required.
  No dependencies/user_dir/replace_path/base GUI override. Simulation uses
  native command0x707 only. Other-base gameplay, ATCU coexistence and MP
  remain untested. No new work on Army/Politics selected.
- Current report docs/report-province-rename-centering-1021.md; preceding
  report-province-rename-redesign-implemented-1021.md and user-owned
  report-province-rename-aesthetic-redesign-1021.md retained unchanged.
  Current board docs/province-rename-layout.svg; old022/023 boards archived
  verbatim. Native pixels stay ignored under versioned work/ paths.

## Installed copy and checks

- work/Victoria-II-Province-Rename-TGC252, canonical mod/TGC, stock Lua,
  V2DLL5.98/cdad55c7; profile TGC252_ProvinceRename_20261020 retained.
- 0.2.3 installed after fresh normal-exit confirmation. After the0.2.3
  report, game observed exited; user explicitly said install it and ill
  check.0.2.4 installation used that fresh instruction and a clear process
  gate; no literal second normal-exit answer asserted. No game control.
- 11 payload and25,388 protected hashes verified; protected unchanged.
  Evidence work/province-rename/centering-024-1021/, exact record
  patch-024-installed.json and full0.2.3/guide backup before-024/.
  Earlier before-023 (0.2.2), before-022 and older backups retained.
- ZIP work/province-rename/candidate/Province-Rename-Universal-0.2.4-test.zip,
  12files; SHA256
  829974db16c5d58b16b6bdd0619618347642fa284ba1bdaad2e8e75d07f70025.
- 295 executable guards/72 Rename runtime, Win32 build,65 native/4 package/
  4 actual-loader checks pass. Five-base font/sprite checks are static.
  Native/GPU-boundary regression does not establish full gameplay.
- Main E:, D:, other copies untouched. No release/publication, source
  commit, push or memory update. Next pitfall P63.

## Next session in order

1. **RENAME-024 player retry**, docs/instructions-rename-024.md. No install
   needed. User opens dedicated copy with Province Rename (Universal) and
   TGC v2.5.2 (stable), selects owned land and opens ABC. Check stable
   centering while typing, deleting and idling, caret/click/Home/End,
   accents/long names/paste and Cancel/Rename/reopen/map/header behavior.
   Expect0.2.4 Universal,72 native signatures and centered input enabled;
   single native text/caret origin in Logs/tgcx_province_rename.log.
2. Complete remaining full limits/stale selection/ownership/lifetime and
   save/reload checks per instructions-rename-020.md sections2-3 with
   0.2.4/72 expectations. Earlier short success does not prove these.
3. Preserve fresh logs/unedited captures after user normal exit. Before
   fixes read pitfalls, especially P58-P62; fixtures must execute independent
   native consumers. New installation requires fresh user exit/install
   instruction and clear process gate. Never launch/terminate autonomously.
4. Other-base/ATCU/MP remain later selected work; no additional copy update
   or publication. Army follow-ups below remain separate.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0/U/021 | short rename/ABC accepted; full behavior pending | instructions-rename-020.md |
| RENAME-023 | redesign works; centering failed, superseded by024 | report-province-rename-redesign-implemented-1021.md |
| RENAME-024 | correction installed, player retry next | instructions-rename-024.md |
| RENAME-MP | theoretical only; no host/client/OOS test | instructions-rename-020.md section4 |
| TPL-5/6 |0.11.2 retained; extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6/3/4/5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".

---

# Session67 - native redesign and centered-input correction (2026-10-21)

Implemented the user's redesign handoff as 0.2.3, installed after fresh
normal exit. User reports working UI/rename but flickering/off-center typing.
Traced the second native inset consumer; 0.2.4 renders zero-padded text and
positions text/caret once. Actual native regression, 295 guards/65 native/
4 package/4 loader checks pass. Installed on fresh explicit instruction,
11 payload/25,388 protected hashes verified; full before-024 rollback.
P62 added, player retry next. No game control or other-copy update.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (67). **Province Rename0.2.1 short behavior accepted;
white-title refinement0.2.2 installed, player appearance check next.**
Session66 NEXT archived verbatim in docs/archive/sessions.md.

## Current decisions and evidence

- User screenshot shows ABC right of Jijolandia, editor open and map name
  updated. Log confirms native command/readback/header refresh and reopen.
  Short0.2.1 behavior accepted; full persistence/lifetime remains untested.
- User requests white title and no beige title background.0.2.2 uses native
  vic_22 white/cream directly on green, keeping native gold braid. Other
  text/input stays black.332x224 editor,28x28 ABC, spacing, proven placement
  and native command0x707 unchanged. Runtime source only updates version.
- One Universal additive V2DLL package, reference3.04 + current V2DLL.
  No dependencies, user_dir, replace_path or base GUI override; no ATCU
  changes. Static five-base checks retained; other-base gameplay/ATCU/MP
  remain untested. MP theory is native command serialization, not a test.
- Current board docs/province-rename-layout.svg; previous board archived
  verbatim in docs/archive/province-rename-layout-021.svg. Current private
  native-font preview work/province-rename/candidate/layout-preview-022-native.png.
  Game-derived pixels stay ignored; versioned external SVG art preserves
  prior prepared images. No remote artifact publication.
- Current report docs/report-province-rename-title-1021.md. Accepted capture/
  log, build/native tests/loader probe/check records/backup under
  work/province-rename/polish-022-1021/. Earlier reports/candidates historical.

## Installed copy and validation

- work/Victoria-II-Province-Rename-TGC252, canonical mod/TGC, stock Lua,
  current V2DLL5.98/cdad55c7; profile TGC252_ProvinceRename_20261020 retained.
  User confirmed Closed normally now; process gate clear before/through
  update. No launch/termination. Main E:, D: and other copies untouched.
- All11payload and25,388protected hashes verified; full0.2.1/guide backup
  polish-022-1021/before-022/. Record patch-022-installed.json beside it.
  Native panel pixels differ only in the old title plate rectangle; entry,
  field/button art, GUI placement and command logic stay unchanged.
- ZIP work/province-rename/candidate/Province-Rename-Universal-0.2.2-test.zip,
  12files,255324bytes; SHA256
  f02656a991dcef5db7e9b5556d4ed8e40c12e19678117ee5d1481d55288294fa.
- 282reference guards/59Rename, Win32 build,53native/4package/4loader checks
  pass. Font check requires white vic_22 title, black other fonts. Five-base
  static font/sprite checks pass. Previous TGC scan still applies.
- No new addresses or pitfall; P61 placement correction retained. Next P62.
  No source commit, push, release/publication or memory update.

## Next session in order

1. **RENAME-022 appearance**, docs/instructions-rename-022.md. No new install.
   User launches dedicated copy with Province Rename (Universal) plus
   TGC v2.5.2 (stable), selects owned land and opens ABC. Check white title
   on green, no beige title plate, centring/clipping, black body/input/button
   text and Rename/Cancel/reopen. Expect0.2.2 Universal and59native signatures
   in Logs/tgcx_province_rename.log; V2DLL logs in V2DLL/Logs/.
2. Complete remaining full behavior/limits/accents/stale selection/ownership
   and save/reload checks per instructions-rename-020.md sections2-3 with
   0.2.2/59 expectations. Short0.2.1 success does not prove those cases.
3. Preserve fresh logs/unedited screenshots after user normal exit. Read
   applicable pitfalls before fixes; add independent semantic evidence for
   any new field interpretation. New installation requires exit/process
   gate. Do not launch/terminate autonomously. Keep all prior rollbacks.
4. Other-base gameplay, ATCU coexistence and MP remain later selected work.
   No extra copy update, release/publication, commit or push automatically.
   Existing Army follow-ups below remain separate.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 / RENAME-U / RENAME-021 |0.2.1 short native ABC/rename accepted; extended cases pending | report-province-rename-title-1021.md |
| RENAME-022 | white title/no plate installed; player appearance check next | instructions-rename-022.md |
| RENAME-MP | theoretical assessment only, gameplay untested | instructions-rename-020.md section4 |
| TPL-5 / TPL-6 |0.11.2 retained; acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P62. Private art/binaries/native exports/backups stay ignored
under work/ or dll/**/Release/.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".

---

# Session66 - 0.2.1 accepted, white-title refinement installed (2026-10-21)

User accepts0.2.1 short functionality and supplies live Jijolandia/editor
capture. Requests white title and no beige title background.0.2.2 removes
plate and uses native vic_22 white on green; other text black, runtime
behavior/placement/art outside title unchanged.282guards/53native/4package/
4loader checks pass. Installed after confirmed normal exit,11payload and
25,388protected hashes verified, before-022 retained. No game control or
other-copy changes/publication. Report-province-rename-title-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (66). **Province Rename0.2.0 ABC missing: live geometry error
corrected in checked/installed0.2.1. Player retry next.** Session65 NEXT
archived verbatim in docs/archive/sessions.md.

## Findings, decisions and boundaries

- Standalone Universal V2DLL free-text rename. User's native28x28 ABC right
  of province name, black text and native green/braid/parchment unchanged.
  Keep actual province name; no extra Rename label. Native command0x707 is
  the only simulation writer. ATCU implementation unchanged.
- 0.2.0 log showed plugin/56guards/both roots, then placement refusal. User
  reopened selected owned land for read-only diagnosis. Verified PID16424,
  dedicated-copy path, Amiens402, viewport2560x1440. No injected calls or
  memory writes. Game control remained with user.
- Root/header/name/font all valid. Definition+0xA0 was X=0, not width380.
  Runtime CFixedWindow size is+0x4A0/+0x4A4. Name matrix is identity; native
  Draw separately uses local17,45 at text+0x188/+0x18C. Both previous fixture
  assumptions repeated the implementation error. Correct both, preserve
  owner chain and native font measurement. P61 records cause/fix; next P62.
- 0.2.1 uses actual runtime width and text-local origin, validates Draw type,
  and logs precise refusal stage/root/width. No UI/art/GUI/localisation
  changes. New fixture executes actual native Draw at font/GPU boundary,
  plus captured-layout regression; actual native font routines still run.
- Report docs/report-province-rename-anchor-1021.md supersedes0.2.0 geometry
  assumptions. Read-only live dumps/native exports/failed log/build/tests
  in work/province-rename/missing-020-1021/; all ignored. Prior reports and
  candidates retained. Main E:, D: and other copies unchanged.

## Installed copy and checked candidate

- Game: work/Victoria-II-Province-Rename-TGC252. User selects Province Rename
  (Universal) + TGC v2.5.2 (stable); profile TGC252_ProvinceRename_20261020.
  Canonical mod/TGC, stock Lua, current V2DLL5.98/cdad55c7 retained.
- User confirmed Closed normally now after diagnostic session. Process gate
  clear before/through update. All11payload/25,388protected hashes pass.
  Backup0.2.0/guide: missing-020-1021/before-021/. Installed record:
  missing-020-1021/patch-021-installed.json.0.1.3 rollback before-020 retained.
- ZIP: work/province-rename/candidate/Province-Rename-Universal- 0.2.1-test.zip,
  12files,250713bytes, SHA256
  d3f9ba6881a21780149cdc91a2b9a0a8f1cd2de42b6ce7b556f01b9f8ebdca47.
- 282reference guards,59Rename runtime, Win32 build,53native checks,
  4package/4loader checks pass. Static five-base fonts/anchors retained;
  prior TGC scan still applies (no TGC data changes).0.2.1 appearance/input,
  full save/lifetime, other-base gameplay, ATCU coexistence and MP untested.
- Board docs/province-rename-layout.svg; current private generated preview
  work/province-rename/candidate/layout-preview-021-native.png. Layout and
  native pixels unchanged; preview is not live acceptance.

## Next session in order

1. **RENAME-021 retry**, docs/instructions-rename-021.md. No new install.
   User launches dedicated copy and selects owned land. Expect0.2.1 Universal,
   59native signatures and Universal Rename ABC icon with screen/editor
   pixels and measured font width in Logs/tgcx_province_rename.log. V2DLL
   logs in V2DLL/Logs/. ABC must appear right of the actual province name.
2. Check native look/black text and clickable entry, editor, Rename, Cancel,
   reopen and host/selection changes. Then full behavior/limits/accents and
   save/reload per instructions-rename-020.md sections2-3, using0.2.1/59.
3. Preserve fresh logs/unedited capture after user normal exit. Diagnose
   actual stage before changes, consult P37/P40/P44/P53-P61, add independent
   semantic regression. Never launch/terminate without explicit request.
   Any new install needs fresh exit confirmation and process gate. Old
   PID47976 force-close authorization never extends to another process.
4. Other bases/ATCU/MP are later selected checks. MP remains theoretical via
   native queue/serializer/parser; no host/client/OOS proof. No commit,
   push, release, publication or other game-copy update follows automatically.
   Existing Army follow-ups below stay separate.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 / RENAME-U |0.1.3 short SP rename accepted; extended checks pending | report-province-rename-universal-ui-1021.md |
| RENAME-020 |0.2.0 ABC missing; actual width/text-origin correction0.2.1 installed | report-province-rename-anchor-1021.md |
| RENAME-021 | checked/installed, user ABC retry next | instructions-rename-021.md |
| RENAME-MP | theoretical assessment only; gameplay untested | instructions-rename-020.md section4 |
| TPL-5 / TPL-6 |0.11.2 retained; acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P62. Private game-derived art/binaries/native exports/backups
stay ignored under work/ or dll/**/Release/. No memory updates.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session65 - missing Universal ABC diagnosed live; 0.2.1 installed (2026-10-21)

0.2.0 installed, but user saw no ABC. Audited ATCU pitfalls; read-only selected
province inspection found definition+0xA0 was X, not width, and text-local17,45
was separate from its identity matrix. Use runtime width+0x4A0 and native Draw
local offsets+0x188/+0x18C. Independent actual Draw regression added,53native
checks/282guards/4package/4loader probes pass.0.2.1 installed after fresh normal
exit confirmation,11payload/25,388protected hashes pass; before-021 retained.
P61, next P62; player ABC retry next. No game control/other-copy changes or
publication. Report-province-rename-anchor-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (65). **Province Rename Universal 0.2.0 installed in the
dedicated TGC/V2DLL copy; player UI/input/persistence check next.**
Session64 NEXT archived verbatim in docs/archive/sessions.md.

## Current state and boundaries

- User accepted0.1.3 short behavior: three renames, map/header/readback and
  editor reopen. Universal0.2.0 adds native28x28 ABC right of the province
  name (no extra Rename label), black added text and native green/braid/
  parchment. Actual province name remains; native font metrics/owner chain
  and host width drive generic placement. Native332x224 editor.
- One additive Province Rename (Universal) mod, no dependency, user_dir,
  replace_path or base GUI/data override. Reference3.04 + V2DLL required.
  Engine command0x707 remains the only simulation writer. ATCU unchanged.
- User explicitly said "install it" after the pending exit gate was
  explained. This fresh direction authorized the update; no literal normal-
  exit confirmation is asserted. No v2game.exe before/through installation.
  No launch or termination. Future game control stays with the user.
- Installed: work/Victoria-II-Province-Rename-TGC252. Canonical mod/TGC,
  stock Lua, V2DLL5.98/cdad55c7 and profile TGC252_ProvinceRename_20261020
  retained. No ATCU/Politics; main E:, D: and other copies untouched.
- All11payload files match final r3;25,388protected baseline hashes match
  before/after. Backup: work/province-rename/ui-universal-020-1021/before-020/.
  Installation evidence: patch-020-installed.json beside it. Target game
  guide tgcx_province_rename_test.txt updated. Do not rerun the old updater
  against now-installed0.2.0; its previous-payload gate intentionally refuses.
- Exact ZIP: work/province-rename/candidate/Province-Rename-Universal-0.2.0-test-r3.zip,
  12files,250479bytes, SHA256
  5c450ba99c1f2496e936251c6c23e368faea4c5dbf73855391b5fb31f4a959da.
- 279guards (56Rename/179Army runtime/44research), Win32 build,51native,
  4package/4loader checks and TGC scan pass. Five-base static anchors/fonts/
  sprites pass (vanilla, TGC2.3.4/2.5.2, HPM, DoD), not gameplay. MP is
  theoretically feasible via native queue/serializer/parser, still untested.
- Current board docs/province-rename-layout.svg uses external private art;
  exact native-font preview work/province-rename/candidate/layout-preview-020-native.png.
  Reports: docs/report-province-rename-universal-ui-1021.md (implementation)
  and docs/report-province-rename-020-installed-1021.md (installation).

## Next session in order

1. **RENAME-020 player check**, docs/instructions-rename-020.md sections2-3.
   No new install needed. User opens victoria2.exe in the dedicated copy,
   selecting Province Rename (Universal) + TGC v2.5.2 (stable). Expected
   0.2.0 Universal,56native signatures and Universal Rename ABC icon in
   Logs/tgcx_province_rename.log; V2DLL logs in V2DLL/Logs/.
2. Check ABC right of short/long/accented province names, actual province
   name retained, black native editor text, green/braid/parchment, field and
   buttons usable. Check Cancel/reopen, host/selection changes, rename/map/
   header, input limits/accents, then save/reload and different-save cleanup.
3. Preserve fresh logs/unedited captures after normal exit. Diagnose actual
   failures, consult ATCU pitfalls/P59/P60, research/build before fixing.
   Any further game-copy update requires the exit/process gate; the old
   PID47976 force-close authorization never extends to another process.
4. Other-base gameplay, ATCU coexistence and real MP test remain follow-ups
   after user selection. No launch, termination, extra installation, release,
   publication, commit or push follows automatically. Army follow-ups below
   remain separate.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 / RENAME-U | 0.1.3 short SP rename accepted; extended persistence/lifetime pending | report-province-rename-universal-ui-1021.md |
| RENAME-020 | Universal native ABC UI installed/verified; player check next | instructions-rename-020.md; report-province-rename-020-installed-1021.md |
| RENAME-MP | theoretical native-command assessment only; gameplay untested | report-province-rename-universal-ui-1021.md; instructions-rename-020.md section4 |
| TPL-5 / TPL-6 | 0.11.2 retained; acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P61. Game-derived art/binaries/native exports/backups stay
ignored under work/ or dll/**/Release/. No memory updates.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session64 - Province Rename Universal 0.2.0 installed (2026-10-21)

User explicitly instructed installation after the pending exit gate was
explained. No v2game.exe was running; no game launched/terminated. Exact r3
candidate installed in the dedicated TGC/V2DLL copy, all 11 payload and
25,388 protected hashes verified. Full before-020 backup retained. No
literal normal-exit confirmation is asserted. Main E:, D: and other copies
untouched. Player UI/persistence check next; no new build or publication.
Report-province-rename-020-installed-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (64). **Province Rename 0.1.3 short behavior accepted;
Universal 0.2.0 native ABC UI checked and staged, installation pending.**
Session63 NEXT archived verbatim in docs/archive/sessions.md.

## Selected work and decisions

- Standalone free-text rename using V2DLL API1; native command0x707 remains
  the only simulation writer. No ATCU implementation changes.
- User reports it worked; screenshot and log confirm three name changes,
  map/header updates and editor reopen. Full persistence/lifetime untested.
- User selects universal compatibility like ATCU, black text, actual native
  green/braid/parchment assets and icon-only opening control to the RIGHT
  of the province name. Final icon choice: native ABC sprite,28x28. Keep
  the actual province name, remove the extra Rename label.
- One additive Province Rename (Universal) package: no dependency, user_dir,
  replace_path or base GUI/data override. Reference3.04 + V2DLL required.
- Actual native name width/kerning, render owner chain and host width drive
  placement. Unsupported fonts use logged header-right fallback; missing
  anchors refuse. Native332x224 editor with all added text black.
- Static anchors/fonts/sprites pass on vanilla, TGC2.3.4/2.5.2, HPM, DoD.
  These are not gameplay passes. MP is feasible in theory through the
  native command/serializer/parser; no host/client or OOS test performed.
- Report: docs/report-province-rename-universal-ui-1021.md. Current board:
  docs/province-rename-layout.svg, external private native-art links only.
  Exact bitmap-font preview: work/province-rename/candidate/layout-preview-020-native.png.
  Prior board retained in docs/archive/province-rename-layout-0100.svg.

## Candidate and dedicated copy

- Target: work/Victoria-II-Province-Rename-TGC252. Still installed0.1.3,
  canonical mod/TGC, stock Lua, V2DLL5.98/cdad55c7; no ATCU/Politics.
  Profile TGC252_ProvinceRename_20261020. Main E:, D: and other copies stay
  unchanged. No game launched or terminated this session.
- Final candidate: work/province-rename/candidate/Province-Rename-Universal-0.2.0-test-r3.zip.
  12files (11mod payload + README),250479bytes. SHA256:
  5c450ba99c1f2496e936251c6c23e368faea4c5dbf73855391b5fb31f4a959da.
  Earlier0.2.0 test/r2 candidates are superseded; use r3 with native ABC.
- 279 executable guards (56Rename/179Army runtime/44research), Win32 build,
  51native checks,4package checks and4current-V2DLL loader probes pass.
  TGC scan passes. Native construction/input/rendering still need player QA.
- Evidence: work/province-rename/ui-universal-020-1021/, including accepted
  0.1.3 capture/log, native exports, base-matrix.json, candidate-final.json,
  final test logs and install-preflight.json.
- Read-only installation preflight passes exact old payload/guide, target
  executable and25,388protected hashes. Fresh normal-exit question remains
  pending. Process absence alone does not grant confirmation. No game-copy
  writes have occurred. Old PID47976 force-close authorization was one-time.

## Next session in order

1. **RENAME-020 install**, docs/instructions-rename-020.md section1. Get
   the pending fresh normal-exit answer, check no v2game.exe, rerun private
   update_020.py preflight, then apply only to the dedicated copy. Preserve
   complete before-020 backup and verify11payload/25,388protected hashes.
   Add a dated installation addendum; do not rewrite the historical report.
2. **RENAME-020 player UI/behavior**, same instructions sections2-3. User
   launches Province Rename (Universal) + TGC v2.5.2 (stable). Expected0.2.0,
   56native signatures, ABC28x28 right of name, black editor text, native
   braids/green/parchment. Check clipping/clicks/focus, Cancel, rename/map/
   header, host/selection changes, accents/limits and save/reload.
3. Preserve fresh logs/screenshots after normal exit. Research/build before
   further fixes; consult ATCU pitfalls/P59/P60. Never launch/terminate the
   game without explicit user authorization. No new install without a fresh
   exit confirmation. Other-base/ATCU/MP gameplay tests remain follow-ups
   after the user selects them, not automatic actions.
4. Existing Army follow-ups below stay separate. No release/publication,
   commit, push or new game-copy staging follows automatically.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 / RENAME-U | 0.1.3 short SP rename accepted; persistence/lifetime pending | report-province-rename-universal-ui-1021.md |
| RENAME-020 | Universal native ABC UI checked/staged; fresh exit answer pending before install | instructions-rename-020.md |
| RENAME-MP | theoretical native-command assessment only; gameplay untested | report-province-rename-universal-ui-1021.md; instructions-rename-020.md section4 |
| TPL-5 / TPL-6 | 0.11.2 retained; acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P61. Private game-derived artwork/binaries/native exports and
backups remain ignored under work/ or dll/**/Release/. No memory updates.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session63 - Province Rename accepted and Universal native UI staged (2026-10-21)

User accepted the short 0.1.3 behavior. Implemented Universal0.2.0 with native
28x28 ABC entry right of measured province name, native green/braids/parchment
and black text. 279 guards, 51 native checks, 4 package checks and 4 loader
probes pass; five-base static matrix. Exact r3 candidate and private native-font
preview ready. Read-only target preflight passes 25,388 protected hashes;
fresh normal-exit confirmation is pending, so no game-copy writes or launch.
MP assessed as feasible through native command0x707, not tested. No ATCU
changes, commit/push or publication. Report-province-rename-universal-ui-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (63). **Province Rename 0.1.3 installed; player retry next.**
0.1.2 showed no button because its type guard refused valid standard native
buttons. Session62 NEXT archived verbatim in docs/archive/sessions.md.

## Selected work and boundaries

- Standalone free-text Province Rename using V2DLL API1. Native command0x707
  remains the only simulation writer. No ATCU implementation changes.
- User supplied missing-entry screenshot. Read-only live inspection of the
  selected owned province438 confirmed correct root/child registry lookup,
  but all three guiButtonType controls resolve to CButtonType RVA0xA3CFBC,
  factory0x5F0310. The related CEU3ButtonType guard was wrong (P60).
- 0.1.3 checks measured type plus factory and logs actual refusal addresses.
  Unique transparent labels and deferred GUI release remain. Actual native
  registry wrapper/lookup/traversal fixtures replace the earlier stub as the
  only evidence for schema acceptance. GUI/layout unchanged.
- Report: docs/report-province-rename-button-type-1021.md. Screenshot/logs,
  read-only live records and private native exports:
  work/province-rename/missing-012-1021/.
- User confirmed normal exit; hidden PID47976 remained with no window and
  blocked installation. User then explicitly requested force-close. Only the
  verified test-copy process was terminated, and process gate passed before
  installation. This exception is for that stuck process only. No game
  launched, commit/push or publication. Future game control stays with user.

## Installed copy and candidate

- Game: work/Victoria-II-Province-Rename-TGC252/.
  User opens victoria2.exe and ticks **Province Rename** plus
  **TGC v2.5.2 (stable)**. Profile TGC252_ProvinceRename_20261020.
- Canonical mod/TGC, stock Lua and current V2DLL5.98/cdad55c7 retained.
  No ATCU/Politics in this copy; main E: and other copies unchanged.
- All10package files match0.1.3;25,388protected hashes match original staging
  baseline before/after update. Only plugin/version marker/test guide changed.
  Evidence: missing-012-1021/patch-013-installed.json; rollback before-013/.
- ZIP: work/province-rename/candidate/Province-Rename-0.1.3-test.zip.
  11files,179361bytes; SHA256
  774d6fbb2dc446efd431a380005d6ac8ff6db9724cc901bf35bfb30859f2c89a.
- 272executable guards (unchanged179Army runtime +44research +49Rename),
  Win32 Release build,42native checks,4package tests and4loader probes pass.
  Same unchanged TGC source retains its passed scan. Native registry/builder
  fixtures do not prove full GUI construction, focus, rendering or persistence.
- Plugin log: Logs/tgcx_province_rename.log. V2DLL logs: V2DLL/Logs/.
  Current guide: tgcx_province_rename_test.txt in the game copy.

## Next session in order

1. **RENAME-U retry**, docs/instructions-rename-013.md. User launches the
   installed copy, selects owned land while paused, then tests Rename,
   Cancel/reopen/selection change and host close/reopen. Expected0.1.3,
   49native guards, two constructed roots, entry placement and editor-open
   records. No additional installation is needed.
2. Once the entry works, complete docs/instructions-rename-u.md sections3-4:
   real typing, Apply/readback/header/map, accents/limits, Cancel, stale
   selection/ownership and save/reload. Compare actual screenshots with
   docs/province-rename-layout.svg. ATCU coexistence remains a later check.
3. Preserve fresh logs on any failure in a new evidence folder. Diagnose the
   actual failed stage and read applicable ATCU pitfalls before patching.
   Research/build first; fresh exit confirmation plus process gate before
   installation. If exit hangs, do not terminate without new authorization.
4. Existing Army follow-ups below stay separate. No release/publication work
   follows automatically.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 | 0.1.3 checked/installed; previous0.1.2 guard refused valid buttons | report-province-rename-button-type-1021.md |
| RENAME-U | next: player retry entry/open/close, then full behavior | instructions-rename-013.md; instructions-rename-u.md |
| TPL-5 / TPL-6 | 0.11.2 retained; acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P61. Prior reports and candidates retained; binaries/native
exports/live dumps/backups remain ignored under work/ or dll/**/Release/.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session62 - missing Rename entry and live button-type evidence (2026-10-21)

User screenshot and log show0.1.2 refusing valid standard-button definitions. Read-only live inspection measured the actual type/factory/registry.0.1.3 built/checked and installed;25,388protected files unchanged. User explicitly authorized force-close of stuck test PID47976 after normal exit hung. No game launched. Player retry pending; report-province-rename-button-type-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-21 (62). **Province Rename 0.1.0 failed during GUI construction.
Correction 0.1.2 is checked and installed in the dedicated TGC/V2DLL copy;
player retry next.** Session61 NEXT archived verbatim in docs/archive/sessions.md.

## Selected work and boundaries

- Separate free-text Province Rename mod using V2DLL API 1. Native engine
  command0x707 remains the sole simulation writer. No ATCU source changes.
- User requested the ATCU pitfall review and a broader missed-issues audit.
  See docs/report-province-rename-crash-1020.md and
  docs/report-province-rename-preflight-1021.md, including limits.
- The 0.1.0 GUI reused button names for labels. Native name lookup dispatched
  a destructor while constructing the opening entry. No rename was queued.
  Full logs/plugin/native exports retained in work/province-rename/crash-0100-1020/.
- 0.1.2 fixes all three collisions, makes labels click-transparent and checks
  button definition types in each owning window before native construction.
  Interim 0.1.1 preflight registry scope was corrected before handoff. Adds native allocation
  push guard, click-time stale-root refusal, aligned observers and diagnostics.
- User confirmed fully closed after the crash. Process/executable gates passed
  before update; only Rename GUI/version/plugin/test guide changed. No game
  launched/terminated, commit/push or publication. Future updates need fresh
  exit confirmation and process gate. User controls game launch/exit.

## Installed copy and candidate

- Game: work/Victoria-II-Province-Rename-TGC252/.
  User opens victoria2.exe and ticks **Province Rename** and
  **TGC v2.5.2 (stable)**. Profile TGC252_ProvinceRename_20261020.
- Canonical mod/TGC, stock Lua and V2DLL5.98 distribution cdad55c7 retained.
  Loader uses dinput8.dll plus V2DLL/v2dll_core.dll. No ATCU/Politics here.
- 10/10 package hashes match 0.1.2;25,388 protected files match original baseline
  before/after update. Evidence: crash-0100-1020/patch-012-installed.json.
  Rollback of changed files: crash-0100-1020/before-012/.
- ZIP: work/province-rename/candidate/Province-Rename-0.1.2-test.zip.
  11 files, 179059 bytes; SHA-256
  101bd8f220a4ba51f6cce3153cea247fe9d55d0531146030a2da46303956ded1.
- 265 executable guards pass (unchanged 179 Army runtime +44 prior research
  +42 Rename). Win32 build,38 native checks, 4 package tests,4 current-loader
  probes and TGC scan pass. Native collision fixture executes actual builder
  and destructor thunk with destructive endpoint intercepted. This does not
  prove full GUI integration, text input, save/reload or multiplayer.
- Current player guide: tgcx_province_rename_test.txt in the copy.
  Rename log: Logs/tgcx_province_rename.log; V2DLL logs: V2DLL/Logs/.

## Next session in order

1. **RENAME-U retry**, docs/instructions-rename-012.md. User launches the
   installed copy; select an owned land province while paused, then test
   Rename open/Cancel/reopen/selection changes/host close. Verify 0.1.2,
   42 signature guards, both native roots and placement in the log.
2. If stable, complete docs/instructions-rename-u.md sections3-4: typing,
   Apply, header/map readback, ownership/selection invalidation, accents,
   boundaries and save/reload. Compare actual screenshots with
   docs/province-rename-layout.svg. ATCU coexistence remains a later check.
3. Preserve fresh full logs on failure. Do not overwrite the first crash
   evidence. Read applicable ATCU pitfalls before another correction.
   Research/build first; obtain fresh exit confirmation only before install.
4. Existing Army follow-ups below remain separate; no release or publication
   work follows automatically.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 |0.1.0 construction failed; checked 0.1.2 correction installed | report-province-rename-preflight-1021.md |
| RENAME-U | next: player retry selection/open/close, then full behavior | instructions-rename-012.md; instructions-rename-u.md |
| TPL-5 / TPL-6 |0.11.2 retained; prior acceptance and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU 0.0.2 published previously; no new release authorized | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU 0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted 0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

Next pitfall P60. Native exports, binaries, loader fixtures and backups remain
ignored under work/ or dll/**/Release/. Prior reports remain historical.


## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session61 - Province Rename construction crash and correction (2026-10-21)

User reported0.1.0 crash on province selection/name click and requested an ATCU pitfall audit. Preserved crash evidence, traced native cross-type dispatch, added P59, built/checked0.1.1 and installed after fresh fully-closed confirmation. Final preflight audit produced0.1.2;25,388protected hashes unchanged. Player retry remains pending. report-province-rename-preflight-1021.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-20 (61). **Province Rename 0.1.0 installed in a NEW dedicated game
copy with TGC2.5.2 and current V2DLL. Player check next.** Session60 archived
verbatim in docs/archive/sessions.md. User corrected the target: a new copy,
not reuse of the existing Army test copy. That initial addition was reverted.

## Selected work and boundaries

- RENAME-0: new standalone package, no ATCU/Politics source changes, no base
  GUI override, no replacement Lua proxy. Native CChangeProvinceNameCommand
  (0x707) performs renames; arbitrary text never becomes generated script.
- Owned land provinces only; native edit field, Apply/Cancel, 1-80 Windows-1252
  characters. Selection/ownership/country/world changes invalidate the edit.
- Current V2DLL supports each active mod's v2dll_mod.dll and dll_Submods.
  Older 5.82 only loads the first mod plugin. Use current V2DLL for combining
  this with ATCU. See the report and package README for the alternative route.
- User confirmed normal exit; after the process gate, the new copy was
  staged and verified. No game launch/exit, commit/push or publication.
  User alone launches/exits; every subsequent update needs fresh normal-exit
  confirmation and a process gate.

## Ready-to-test game copy

- Target: work/Victoria-II-Province-Rename-TGC252/.
- User opens victoria2.exe there and selects **Province Rename** plus
  **TGC v2.5.2 (stable)**. Only these two descriptors are present.
- Independent profile: TGC252_ProvinceRename_20261020. Base from D: and
  canonical mod/TGC from the verified stable source; 25,399 hashes checked.
- Current V2DLL distribution pinned at cdad55c7 uses dinput8.dll plus
  V2DLL/v2dll_core.dll. Stock Lua retained. LOAD_DLL_FROM_MOD_FOLDER=1.
- No ATCU/Politics installed in this new copy. Earlier Army test copy
  restored before staging. No main install or original game files modified.
- Guide: work/Victoria-II-Province-Rename-TGC252/tgcx_province_rename_test.txt.
- Installation report: docs/report-province-rename-fresh-tgc-1020.md.
- Evidence: work/province-rename/fresh-tgc252-1020/{installed,verification}.json.

## Candidate and evidence

- ZIP: work/province-rename/candidate/Province-Rename-0.1.0-test.zip.
  11 files, 178193 bytes; SHA-256
  dfd67d998aa2c62d15b539b383a638c48a8766ddec81f22c21ce6561e4e8f082.
- Source: dll/province_rename/; packaging: tools/prepare_province_rename.py.
- 259 executable guards pass (179 Army/Politics runtime +44 prior research
  +36 separate Rename runtime). Existing Army generated header unchanged.
- 30 x86 native fixture checks, three package tests and four probes through
  unmodified current V2DLL loader source pass. Loader probes deliberately
  refuse an unsupported host before patches; they do not prove game loading.
- Native reference-image execution confirms command allocation/id/name/id
  fields, queue handoff, province setter and both map-label update paths.
  Province header refresh occurs only after the applied name is observed.
- Report: docs/report-province-rename-1020.md. Local layout spec:
  docs/province-rename-layout.svg; generated preview in candidate/.

## Next session in order

1. **RENAME-U**, docs/instructions-rename-u.md: installation is complete in
   the new copy above. User launches victoria2.exe, selects the two mods,
   then verifies entry, input, Apply/Cancel, ownership/selection guards,
   header/map labels and save/reload. Do not reinstall before this check.
2. Preserve the full rename and V2DLL logs after normal exit. Compare actual
   screenshots against the local layout spec; diagnose any failures before
   another installation. Runtime and visual acceptance remain open.
3. Army requests below stay separate. Existing TPL-5/TPL-6 extended checks
   retain docs/instructions-tpl-5.md and docs/instructions-tpl-6.md; TPL-7
   pre-Conform-save coverage remains unconfirmed. No release/publication
   work follows automatically.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 | implemented, checked and installed in the new copy; game check pending | report-province-rename-1020.md; report-province-rename-fresh-tgc-1020.md |
| RENAME-U | next: user launches the NEW TGC/V2DLL copy and verifies | instructions-rename-u.md |
| TPL-5 / TPL-6 | 0.11.2 retained; prior acceptance scope and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload check passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized here | report-release-002-1019.md |
| ATCU-MAIN | main E: remains previously installed ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted 0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | respective existing instructions |

Next pitfall P59. Binaries, native exports, loader research and test fixtures
remain ignored under work/ or dll/**/Release/.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session60 - dedicated Province Rename TGC/V2DLL copy (2026-10-20)

User requested a NEW game copy, not reuse of an existing test installation. The initial Rename-only addition to the old test copy was removed with exact hash checks. After normal-exit confirmation and process gates, staged a new independent stock game + canonical TGC2.5.2 + current V2DLL + Rename copy.25,399payload hashes verified. No game launched. report-province-rename-fresh-tgc-1020.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-20 (60). **Province Rename 0.1.0 implemented as a separate V2DLL
plugin and staged. Not installed or tested in game.** Session59 archived
verbatim in docs/archive/sessions.md. User selected free text in the province
window and explicitly requested V2DLL integration.

## Selected work and boundaries

- RENAME-0: new standalone package, no ATCU/Politics source changes, no base
  GUI override, no replacement Lua proxy. Native CChangeProvinceNameCommand
  (0x707) performs renames; arbitrary text never becomes generated script.
- Owned land provinces only; native edit field, Apply/Cancel, 1-80 Windows-1252
  characters. Selection/ownership/country/world changes invalidate the edit.
- Current V2DLL supports each active mod's v2dll_mod.dll and dll_Submods.
  Older 5.82 only loads the first mod plugin. Use current V2DLL for combining
  this with ATCU. See the report and package README for the alternative route.
- No installation, game launch/exit, commit, push or publication performed.
  User alone launches/exits. A fresh normal-exit confirmation and process
  gate are required before game-copy installation writes.

## Candidate and evidence

- ZIP: work/province-rename/candidate/Province-Rename-0.1.0-test.zip.
  11 files, 178193 bytes; SHA-256
  dfd67d998aa2c62d15b539b383a638c48a8766ddec81f22c21ce6561e4e8f082.
- Source: dll/province_rename/; packaging: tools/prepare_province_rename.py.
- 259 executable guards pass (179 Army/Politics runtime +44 prior research
  +36 separate Rename runtime). Existing Army generated header unchanged.
- 30 x86 native fixture checks, three package tests and four probes through
  unmodified current V2DLL loader source pass. Loader probes deliberately
  refuse an unsupported host before patches; they do not prove game loading.
- Native reference-image execution confirms command allocation/id/name/id
  fields, queue handoff, province setter and both map-label update paths.
  Province header refresh occurs only after the applied name is observed.
- Report: docs/report-province-rename-1020.md. Local layout spec:
  docs/province-rename-layout.svg; generated preview in candidate/.

## Next session in order

1. **RENAME-U**, docs/instructions-rename-u.md: select an isolated game copy
   and suitable V2DLL loader; after fresh normal exit, install only this
   package. User launches it and verifies entry, text input, Apply/Cancel,
   ownership/selection guards, header/map labels and save/reload.
2. Preserve the full rename and V2DLL logs after normal exit. Compare actual
   screenshots against the local layout spec; diagnose any failures before
   another installation. Runtime and visual acceptance remain open.
3. Army requests below stay separate. Existing TPL-5/TPL-6 extended checks
   retain docs/instructions-tpl-5.md and docs/instructions-tpl-6.md; TPL-7
   pre-Conform-save coverage remains unconfirmed. No release/publication
   work follows automatically.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0 | implemented and automatically checked; not installed or tested in game | report-province-rename-1020.md |
| RENAME-U | next: installation choice and player verification | instructions-rename-u.md |
| TPL-5 / TPL-6 | 0.11.2 retained; prior acceptance scope and extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload check passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; no new release authorized here | report-release-002-1019.md |
| ATCU-MAIN | main E: remains previously installed ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | accepted 0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | respective existing instructions |

Next pitfall P59. Binaries, native exports, loader research and test fixtures
remain ignored under work/ or dll/**/Release/.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".


---

# Session59 - separate V2DLL province renaming (2026-10-20)

User selected free-text province-window renaming and V2DLL integration. Implemented and staged Province Rename 0.1.0 independently of ATCU/Politics. Native command, loader and package checks pass; no game installation or launch. In-game verification next. See report-province-rename-1020.md.

## NEXT.md at session start (including pre-existing user changes)

# NEXT - what comes next

2026-10-19 (59). **ATCU culture and continent filters implemented as
internal0.11.0 and installed in the isolated TGC copy. Player test next.**
Session58 archived in docs/archive/sessions.md.

**Update 2026-10-19 (release): ATCU 0.0.2 prepared from internal 0.11.2.** work/releases/final/ATCU-0.0.2.zip, SHA-256 e656c58d...becb4a6, source a90050c, pushed; published 2026-10-19 as pre-release v0.0.2 (tag on cae2a68) with the ZIP and its .sha256, re-download hash verified. docs/report-release-002-1019.md. The 0.11.x filters were not rechecked on vanilla, HPM or DoD.

**Update 2026-10-19 (later): TPL-7, 0.11.2, Conform orders survive a load; PASSED in game (2 orders resumed and merged after a load; pre-Conform save case unconfirmed); installed in the isolated TGC copy 2026-10-19 (41/41 verified, backup work/orders-0112/before-tgc/).** A reload had sent a Conform regiment to the rally point (D13 forgot it). The user chose option 2 (D13 revised): tgcx_conform_orders.txt beside the templates records tag, date, stack and construction ids; a load of that tag dated within an order resumes the march and merge. docs/report-tpl-7-orders-1019.md. Candidate work/orders-0112/ATCU-orders-0.11.2-test.zip; Run the report's section 7 check (marker TGCX TPL-7 0.11.2).

**Update 2026-10-19: 0.11.1 GUI refinement installed in the isolated TGC copy; user reports it works great (checklist items not individually confirmed).** Native
checkboxes with a "Culture" caption (no band in the builder), compact
`Recruit from < Any continent >`, continents limited to those with owned
provinces, and filter-guard failure keeps unrestricted Army on.
docs/report-army-filters-gui-1019.md. Candidate
work/filters-0111/ATCU-filters-0.11.1-test.zip; backup work/filters-0111/before-tgc/.
Run steps 1-2 below on 0.11.1 (marker TGCX TPL-5+TPL-6 0.11.1).

## User decisions and boundaries

- Culture modes per template: Primary + accepted / Primary only; neither
  means All. Used by Build Army and Conform missing recruitment. Existing
  regiments remain counted by type, no culture-based purge.
- Continent choice: current recruitment action, default Any. Session-only
  builder selector, never saved per template. Conform retains independent
  D17 land reach and ignores this continent selector.
- User supplied current GUI screenshots, retained as visual references.
  Keep652x528builder/eight unit rows and332x594Conform. Filter band reduces
  visible Conform changes to seven rows, with overflow/totals retained.
- Name ATCU - Army Template and Conform Universal. No em dashes. Files-only
  ZIP, one README.txt, no recipient scripts. License remains undecided.
- User alone launches/exits. Fresh normal-exit confirmation/process gate
  before any further game-copy writes. No new commit/push/publication or
  Politics expansion authorized.

## Installed candidate

User confirmed normal exit and chose isolated TGC installation.39/39mod
files verified in work/Victoria-II-clean-TGC252. Existing V2DLL5.82/Lua,
old packages and protected base anchors unchanged. No game launched.

Open that copy's victoria2.exe, select **ATCU - Army Template and Conform
Universal** plus **TGC v2.5.2 (stable)** only. Untick old Army/Politics.
Expected log: TGCX TPL-5+TPL-6 0.11.0, V2DLL plugin API v1.
Backup: work/filters-0110/before-tgc/. Installation and test evidence:
work/filters-0110/{installed-tgc,verification}.json. Test guide in game root
was refreshed with its original retained in the backup.

Main E:/Victoria.II.v3.04.Inclu.ALL.DLC remains ATCU0.0.1/internal0.10.4;
vanilla/HPM/DoD test copies remain0.10.4. Four-base acceptance applies to
0.10.4, not these filters. Main install instructions remain separate.

## Implementation and checks

- report-army-filters-implementation-1019.md: per-pop culture snapshot,
  cached loaded continent catalogue, exact engine-command pop selection,
  shared preview/action filtering, paused preview revision invalidation.
- Imports old tgcx_templates.txt when v2 absent. Edits save atomically to
  tgcx_templates_v2.txt. Original v1 stays intact; malformed v2 refuses,
  never falls back to unrestricted. Old build reads v1, so v2-only edits
  remain unavailable there. No continents or native pointers persisted.
-223executable checks pass:179runtime +44research, generated header matches;
  TGC scan, Win32 build, native/real-D3D9 fixtures pass. Python109full-suite
  cases plus added package-mismatch guard pass,110distinct cases checked.
  Packaged loader and V2DLL API probes pass. No filter game acceptance yet.
- Visual previews use actual staged art/control geometry. Current supplied
  captures and new PNG/SVG references are indexed in visual-references.md;
  native font/visibility/hit-testing still require player verification.
  Remote historical artifact was not republished.
- Files-only candidate: work/filters-0110/ATCU-filters-0.11.0-test.zip,
  41files,1,052,283bytes; SHA256
  04ca8a523016ce0d55099c049c76a6b81a7a75f3d3c92ec675541a0ce0ddf572.
  Accepted slim0.0.1 ZIP and main installation retained unchanged.

## Next session in order

1. **TPL-5 player check**, docs/instructions-tpl-5.md. User launches isolated
   TGC copy; compare All / Primary + accepted / Primary only from mixed
   culture soldier pops, actual source pops and shortage. Toggle while
   paused, save/reload; Conform existing units unchanged by culture.
2. **TPL-6 player check**, docs/instructions-tpl-6.md. Same template with
   Any / Europe / another owned continent; compare actual source continent
   and shortage. Check combinations with culture, and Conform land reach
   independently. No sea routing implied by continent membership.
3. After normal exit preserve full Logs/tgcx.log and Logs/v2dll.log. Verify
   marker/catalogue/commands and user visuals. Diagnose failures before
   any additional installation; fresh exit confirmation for every update.
4. Once TGC filters accepted, stage focused vanilla/HPM/DoD checks of the
   changed controls/source selection. Keep previous packages as rollback;
   do not claim acceptance from fixtures or previous0.10.4matrix.
5. Record actual test scope and close under ROADMAP7. Public release work
   follows instructions-release-0.md separately; no automatic upload/push.

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| TPL-5 |0.11.0implemented, installed isolated TGC, player verification next | instructions-tpl-5.md; report-army-filters-implementation-1019.md |
| TPL-6 |0.11.0implemented, session-only selector, player verification next | instructions-tpl-6.md; same report |
| ATCU-MAIN |0.0.1retained on main E: | instructions-atcu-main.md; report-atcu-main-install-1019.md |
| RELEASE-0 | slim local0.0.1 retained; publication separate | instructions-release-0.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 |0.10.4four-base short accepted; filters unaccepted | instructions-mod-6.md |
| TPL-3 / TPL-4 / V2D-082 | accepted baselines | existing reports |
| LAW-4 / U-12 Bill | deferred | respective instructions |
| V2D-2 | separate author-side enhancement | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Native exports, art and binaries stay ignored work/.


---

# Session58 - ATCU culture and continent filters (2026-10-19)

User authorized both filters, chose a session-only continent choice defaulting to Any, supplied current GUI references, then confirmed normal exit and selected installation in the isolated TGC copy. Implemented0.11.0, validated and installed39mod files with rollback. Main0.0.1 retained. Player verification next. See report-army-filters-implementation-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (58). **Culture and continent filter research completed.
Both are feasible. Culture implementation first; no new runtime code yet.**
Session57 archived in docs/archive/sessions.md.

## User decisions / boundaries

- User requested viability research for both next features. Culture modes:
  Primary + accepted / Primary only, neither means unrestricted; builder
  and Conform missing builds. Continent selector is for builder initially;
  its action-versus-template preference semantics remain a design choice.
- Name ATCU - Army Template and Conform Universal. No em dashes.
- Keep one files-only Universal package; no recipient scripts/installer.
- Conform D17 land reach stays independent from continent membership.
- User alone launches/exits. Fresh normal-exit confirmation/process gate
  before any game-copy writes. Research made no game changes.
- Politics waits. No new commit/push/tag/release/upload/contact authorized.


## Main installation update

ATCU0.0.1 is installed at E:/Victoria.II.v3.04.Inclu.ALL.DLC after fresh
normal-exit confirmation;37/37files verified, old packages/loader/base
anchors unchanged. Evidence work/main-atcu-001-1019/installed.json.
User launcher: ATCU plus base only; untick old Army and old TGCX Politics
(the older Politics DLL can take precedence). Main gameplay check remains
unconfirmed; no game launched. instructions-atcu-main.md gives verification
and rollback. Culture/continent filters are not installed.

## Research evidence

- report-army-filters-research-1019.md and docs/symbols.md final section:
  CPop+0x6C culture; country primary+0xE64 and accepted list+0xE54/+0xE58/
  +0xE5C; node value+0,next+8. Native construction retains exact chosen pop.
- Province+0x16C continent pointer; loaded catalogue in provinceSource
  +0x2A78..+0x2A7C, key+0x40, display+0x5C. Read engine data, no per-click
  parsing or fixed six-continent list. Static inventory: vanilla6,HPM8,
  DoD6,TGC2.5.2 fourteen. Work exports/inventory: filters-research-1019/.
- Crucial: current tgcx_TplEligible caches per province. Culture is per-pop;
  add pop-aware eligibility for mixed-culture provinces, shared across
  preview and actual build. Preserve native unit eligibility/shortages.
- v1 template parser rejects new non-unit/non-numeric fields. Need settings
  migration/backup/rollback before adding culture modes or continent keys.
- Thirteen new research-only byte guards pass. check_exe222total checks;
  runtime table172 unchanged, gen_signatures --check passes. TGC scan green.
  No filter implementation, DLL rebuild, live memory check or game test.
- Viewed current Templates mockup/accepted Conform screenshot. New controls
  need an intentional filter band and reference update before implementation;
  existing eight rows/footer leave little spare space. No visual edits yet.

## Existing release / baseline

- Accepted gameplay remains Universal0.10.4 on vanilla/HPM/DoD/TGC+V2DLL5.82.
- Slim ATCU0.0.1 ZIP: work/releases/final/ATCU-0.0.1.zip,39files,
  1,041,444bytes,SHA256
  9ee654e7b33fbe62444892a55388e24ecfc7246ace2fdef9bf1007585e4f799e.
  One README.txt only in ZIP; extra manifests/release text stay alongside.
  No release artifacts changed by research. Own license remains undecided;
  V2DLL user clarification retained. Publication remains separate.

## Next session in order

1. **TPL-5 culture implementation**, docs/instructions-tpl-5.md: bounded
   culture snapshot, pop-aware shared eligibility, v1 migration/rollback,
   UI reference/control band, modes and immediate paused-preview invalidation.
   Keep count-only treatment of existing regiments, no culture-based purge.
2. Fixtures: mixed-culture same province, primary/accepted changes, empty
   list, invalid reads, restricted unit types, shortage, preview/action
   equality, old-file import/rollback and culture AND land reach. Stage
   candidate; get fresh exit confirmation only when installation is ready.
3. **TPL-6 continent selector**, docs/instructions-tpl-6.md: recommend Any
   plus live catalogue, initial selection per recruitment action. Confirm
   semantics if implementing persistent per-template geography instead.
   Validate6/8/14catalogues/custom/absent keys; no silent fallback to Any.
4. Preserve accepted rollback and slim ZIP. Public release review remains
   docs/instructions-release-0.md, not a prerequisite to filter research.
   No publication or repeated short matrix without relevant changes.
5. Report actual static/runtime scope; close under ROADMAP7.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| ATCU-MAIN | installed0.0.1; main-copy runtime unconfirmed | instructions-atcu-main.md; report-atcu-main-install-1019.md |
| TPL-5 | feasibility established; next implementation, not runtime tested | instructions-tpl-5.md; report-army-filters-research-1019.md |
| TPL-6 | feasibility established; UI/preference semantics next | instructions-tpl-6.md; report-army-filters-research-1019.md |
| RELEASE-0 | slim local0.0.1 candidate retained; publication separate | instructions-release-0.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | four-base short accepted; extended pending | instructions-mod-6.md |
| TPL-3 / TPL-4 / V2D-082 | accepted baselines | existing reports |
| LAW-4 / U-12 Bill | deferred | respective instructions |
| V2D-2 | separate author-side enhancement | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Native exports, art and binaries stay ignored work/.


---

# Session57 - ATCU0.0.1 main installation (2026-10-19)

User requested main E: installation and confirmed normal exit. Staged/backed up and verified existing loader with actual package in isolated probe, then added37verified files. Protected originals/old packages/base anchors unchanged. No game launch. See report-atcu-main-install-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (57). **Culture and continent filter research completed.
Both are feasible. Culture implementation first; no new runtime code yet.**
Session56 archived in docs/archive/sessions.md.

## User decisions / boundaries

- User requested viability research for both next features. Culture modes:
  Primary + accepted / Primary only, neither means unrestricted; builder
  and Conform missing builds. Continent selector is for builder initially;
  its action-versus-template preference semantics remain a design choice.
- Name ATCU - Army Template and Conform Universal. No em dashes.
- Keep one files-only Universal package; no recipient scripts/installer.
- Conform D17 land reach stays independent from continent membership.
- User alone launches/exits. Fresh normal-exit confirmation/process gate
  before any game-copy writes. Research made no game changes.
- Politics waits. No new commit/push/tag/release/upload/contact authorized.

## Research evidence

- report-army-filters-research-1019.md and docs/symbols.md final section:
  CPop+0x6C culture; country primary+0xE64 and accepted list+0xE54/+0xE58/
  +0xE5C; node value+0,next+8. Native construction retains exact chosen pop.
- Province+0x16C continent pointer; loaded catalogue in provinceSource
  +0x2A78..+0x2A7C, key+0x40, display+0x5C. Read engine data, no per-click
  parsing or fixed six-continent list. Static inventory: vanilla6,HPM8,
  DoD6,TGC2.5.2 fourteen. Work exports/inventory: filters-research-1019/.
- Crucial: current tgcx_TplEligible caches per province. Culture is per-pop;
  add pop-aware eligibility for mixed-culture provinces, shared across
  preview and actual build. Preserve native unit eligibility/shortages.
- v1 template parser rejects new non-unit/non-numeric fields. Need settings
  migration/backup/rollback before adding culture modes or continent keys.
- Thirteen new research-only byte guards pass. check_exe222total checks;
  runtime table172 unchanged, gen_signatures --check passes. TGC scan green.
  No filter implementation, DLL rebuild, live memory check or game test.
- Viewed current Templates mockup/accepted Conform screenshot. New controls
  need an intentional filter band and reference update before implementation;
  existing eight rows/footer leave little spare space. No visual edits yet.

## Existing release / baseline

- Accepted gameplay remains Universal0.10.4 on vanilla/HPM/DoD/TGC+V2DLL5.82.
- Slim ATCU0.0.1 ZIP: work/releases/final/ATCU-0.0.1.zip,39files,
  1,041,444bytes,SHA256
  9ee654e7b33fbe62444892a55388e24ecfc7246ace2fdef9bf1007585e4f799e.
  One README.txt only in ZIP; extra manifests/release text stay alongside.
  No release artifacts changed by research. Own license remains undecided;
  V2DLL user clarification retained. Publication remains separate.

## Next session in order

1. **TPL-5 culture implementation**, docs/instructions-tpl-5.md: bounded
   culture snapshot, pop-aware shared eligibility, v1 migration/rollback,
   UI reference/control band, modes and immediate paused-preview invalidation.
   Keep count-only treatment of existing regiments, no culture-based purge.
2. Fixtures: mixed-culture same province, primary/accepted changes, empty
   list, invalid reads, restricted unit types, shortage, preview/action
   equality, old-file import/rollback and culture AND land reach. Stage
   candidate; get fresh exit confirmation only when installation is ready.
3. **TPL-6 continent selector**, docs/instructions-tpl-6.md: recommend Any
   plus live catalogue, initial selection per recruitment action. Confirm
   semantics if implementing persistent per-template geography instead.
   Validate6/8/14catalogues/custom/absent keys; no silent fallback to Any.
4. Preserve accepted rollback and slim ZIP. Public release review remains
   docs/instructions-release-0.md, not a prerequisite to filter research.
   No publication or repeated short matrix without relevant changes.
5. Report actual static/runtime scope; close under ROADMAP7.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| TPL-5 | feasibility established; next implementation, not runtime tested | instructions-tpl-5.md; report-army-filters-research-1019.md |
| TPL-6 | feasibility established; UI/preference semantics next | instructions-tpl-6.md; report-army-filters-research-1019.md |
| RELEASE-0 | slim local0.0.1 candidate retained; publication separate | instructions-release-0.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | four-base short accepted; extended pending | instructions-mod-6.md |
| TPL-3 / TPL-4 / V2D-082 | accepted baselines | existing reports |
| LAW-4 / U-12 Bill | deferred | respective instructions |
| V2D-2 | separate author-side enhancement | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Native exports, art and binaries stay ignored work/.

---

# Session56 - Culture and continent filter feasibility (2026-10-19)

User requested research into both features. Bounded read-only Ghidra confirms culture fields/list, exact command pop selection, province continent pointer and dynamic loaded catalogue/key/label. Inventory: vanilla6/HPM8/DoD6/TGC14continents. Thirteen research guards added;222total checks and unchanged172-runtime header pass. Main integration trap: culture cannot use province-cached eligibility. No feature code, binary, install, release, commit/push or game controls. report-army-filters-research-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (56). **ATCU0.0.1 release candidate prepared locally using the
accepted internal0.10.4 build. User license choice: undecided for candidate.**
Session55 archived in docs/archive/sessions.md. No release/tag/upload,
commit/push or game-copy changes during preparation.

## User decisions / boundaries

- Public name: **ATCU - Army Template and Conform Universal**. Internal
  tgcx paths remain for upgrade/library compatibility. No em dashes.
- License stays undecided for this candidate. Keep V2DLL attribution and
  owner's clarification that its code use is no problem; no invented license.
- Same files-only package; no recipient installer/PS1/BAT/Python needed.
- User alone launches/exits; installs require fresh normal-exit confirmation
  and process gate. Existing accepted game copies are unchanged.
- Primary+accepted / primary-only culture buttons remain next feature
  (TPL-5), then consider continent choice in builder (TPL-6). Neither is in
  this candidate. Politics/LAW-4 still wait. Conform D17 land reach retained.
- Preparation does not authorize publication, new push, tag or messages.

## Candidate and evidence

- **Superseded 2026-10-19 (user: too many docs in the ZIP):**
  work/releases/final/ATCU-0.0.1.zip, 1,041,444 bytes, 39 files: one
  README.txt (CRLF, ASCII), lua51.dll and mod/ only. SHA256
  9ee654e7b33fbe62444892a55388e24ecfc7246ace2fdef9bf1007585e4f799e,
  reproduced. SHA256/RELEASE/PROVENANCE JSON now sit beside the ZIP.
  Split docs archived in docs/archive/release-0.0.1-candidate-1019/;
  RELEASE_NOTES.md is the GitHub release body. Mod payload unchanged.
- Earlier candidate: ATCU-0.0.1-candidate.zip, 1,054,050 bytes, 49 files,
  SHA256 4e84a34b167b3fc97aef52fffd58dc2d75d2ae3305f0ca3d08d49aa491a28eaf.
- 36/37mod files identical to accepted payload; only descriptor display name
  changed. DLL build0.10.4/loader unchanged. Distribution version0.0.1
  explicitly separate; logs retain old internal build. No game integration
  claim for new branding beyond the previously accepted behavior.
- ZIP includes README/INSTALL/CHANGELOG/CREDITS/KNOWN_ISSUES/BUG_REPORT/
  RIGHTS_STATUS/RELEASE_NOTES plus RELEASE/PROVENANCE/SHA256 JSON. No game
  executable, original Lua, base mods, V2DLL distribution or recipient scripts.
- tools/prepare_atcu_release.py reproduces candidate from accepted ZIP.
  Five release tests pass, CRC/manifest/content pass, reproducible ZIP hash
  matches. Clean proxy and packaged plugin probes/file-layout/removal
  simulations pass; evidence work/releases/recipient-probes/verification.json.
  These are not second-PC or full-game tests. No game launched.
- report-release-0-candidate-1019.md records exact scope and provenance.
  28textures:9original drawn,19game/effective-base-derived. Public derived-art
  permission status/ancestry still to record; preserve accepted appearance.
- Accepted gameplay matrix remains vanilla/HPM/DoD/TGC2.5.2 + V2DLL5.82,
  report-universal-tgc-passed-1019.md. Other mods and MP not established.

## Next session in order

1. **RELEASE-0 candidate review**, docs/instructions-release-0.md: review
   docs/art provenance and check candidate branding/install with recipient.
   Record artwork publication status or propose user-approved alternatives.
   Do not repeat the V2DLL code-use question or select a license silently.
2. No public upload until exact final ZIP/hash/tag/destination authorized.
   Proposed release title ATCU0.0.1, tag v0.0.1, existing repository
   github.com/azha2005/victoria2tgcx. Keep prerelease status and precise scope.
3. **TPL-5 next feature**, docs/instructions-tpl-5.md: verify native culture
   fields, update visual references, implement shared preview/build filters
   with backwards compatible preferences. Stage/tests/normal-exit/game check.
4. **TPL-6 prospective**, docs/instructions-tpl-6.md: dynamic continent
   selector proposal in template builder after culture; distinct from land
   reach. Extended regression remains instructions-mod-6.md.
5. Report actual results; close under ROADMAP7 and preserve rollback.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RELEASE-0 | local0.0.1 candidate ready for review; not published | instructions-release-0.md; report-release-0-candidate-1019.md |
| TPL-5 | next feature; not implemented | instructions-tpl-5.md |
| TPL-6 | prospective continent-builder design | instructions-tpl-6.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 | four-base short accepted; extended pending | instructions-mod-6.md; report-universal-tgc-passed-1019.md |
| TPL-3 / TPL-4 / V2D-082 | accepted baselines | existing reports |
| LAW-4 / U-12 Bill | user-deferred | respective instructions |
| V2D-2 | separate author-side enhancement | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Art/binaries/local release outputs remain ignored work/.

---

# Session55 - ATCU0.0.1 release candidate prepared (2026-10-19)

User selected release preparation, chose license undecided for this candidate, named it ATCU - Army Template and Conform Universal, and requested no em dashes. Files-only candidate and release docs prepared from accepted0.10.4 binaries; manifest/CRC, five release tests, clean/plugin probes and reproducible ZIP hash verified. No publication/tag/commit/push/game changes. See report-release-0-candidate-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (55). **Universal0.10.4 short checks accepted on vanilla,
HPM, DoD and TGC2.5.2 (with V2DLL5.82). Culture buttons are next.**
Session54 preserved in docs/archive/sessions.md. User authorized this
source checkpoint commit/push; no public0.0.1 release/tag requested yet.

## User decisions / boundaries

- Next: primary+accepted and primary-only culture buttons for Templates
  and Conform (TPL-5). Politics/LAW-4 still wait.
- Then consider continent selection in the template builder (TPL-6).
  "Perhaps" is a proposal to assess; final design is not yet selected.
- Same files-only Universal package; no recipient installer/PS1/BAT.
- Conform recruits within land reach and reports shortage (D17).
- User says V2DLL-derived code use is no problem; author supports open
  source and has already been told of its use. Keep attribution. Do not
  treat earlier permission uncertainty as blocking this source checkpoint.
  Our own release license is still unselected; no invented upstream license.
- User alone launches/exits. Fresh normal-exit confirmation + process gate
  before game-copy writes. No new game changes needed for this checkpoint.
- Current Conform screenshot supersedes old mockup. Update visual references
  when adding controls; keep native styling and DoD compatibility.

## Evidence / accepted baseline

- report-universal-tgc-passed-1019.md: TGC user acceptance, V2DLL header,
  11 live land types, two missing builds queued and3/3extras detached.
 16 soldier pops excluded by land reach. New-build final joins absent in
  snapshot, so do not claim that log proves exhaustive completion.
- Logs/hash manifest: work/universal-tgc-0104-passed-1019/. Other accepted
  logs/user statement: work/universal-0104-confirmed-three-bases/.
  Vanilla latest log overwritten by launcher; acceptance is user-reported.
- Accepted ZIP work/Army-Templates-Conform-Universal-0.10.4.zip,
  SHA256 2e1d8cf974f2a7eea92e82067c0f505c891d0b517f8e7c9b60fb9696c029dff1.
  tgcx.dll/v2dll_mod.dll SHA256
  b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67.
- 172 signatures/native/reference GUI/actual D3D9 checks pass. Python
  discovery97 passed,2 probe/build-race errors resolved by complete12-test
  installer/Lua/plugin rerun. Do not rebuild probe concurrently with tests.
- Universal gets units from live engine definitions, template TXT cached
  in memory; capacity/routes refresh for preview/actions. Tracking remains
  session-only; after reload Conform builds anew, never gathers idle troops.
- All four short checks passed; extended straits/moving-target/selection/
  Navy/library cases, olderTGC2.3.4/HFM and MP are not fully validated.

## Next session in order

1. **TPL-5 culture buttons**, docs/instructions-tpl-5.md: verify native
   culture fields/command pop, update visuals, implement shared filtering
   and backwards compatible per-template setting, test/stage/game-check.
2. **TPL-6 continent proposal**, docs/instructions-tpl-6.md: after culture,
   inspect dynamic continent data and propose a builder selector/default/
   persistence behavior. Keep continent distinct from Conform land reach.
3. Preserve accepted0.10.4 rollback. No repeated four-base short test until
   changed behavior warrants it. Extended regression instructions remain
   docs/instructions-mod-6.md. Never launch or close the game for the user.
4. Public0.0.1 preparation when selected: docs/instructions-release-0.md.
   Credits/license text, bundled-art provenance, recipient install and
   release documentation remain. Source push is authorized this session;
   no release asset upload/tag/publication/external message authorized.
5. Record actual scope/results; close under ROADMAP7 with instructions.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| TPL-5 | selected next; not implemented | instructions-tpl-5.md |
| TPL-6 | prospective continent-selector design after TPL-5 | instructions-tpl-6.md |
| MOD-6 | four-base short checks passed including TGC/V2DLL; extended pending | instructions-mod-6.md; report-universal-tgc-passed-1019.md |
| MOD-3 / MOD-4 / MOD-5 | vanilla/HPM/DoD/TGC short accepted; extended pending | instructions-mod-6.md |
| RELEASE-0 | readiness assessed; public preparation not selected | instructions-release-0.md |
| TPL-3 / TPL-4 | Ctrl/Alt/detach accepted | report-tpl-4-runtime-1019.md |
| LAW-4 / U-12 Bill | user-deferred | respective instructions |
| V2D-082 | accepted; Universal route now also accepted | report-universal-tgc-passed-1019.md |
| V2D-2 | author-side enhancement D16, separate from working loader route | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / Politics compatibility deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Third-party art, binaries and decompiled output stay ignored.

---

# Session 54 - TGC Universal passed; culture filters selected (2026-10-19)

User accepts TGC2.5.2 + V2DLL5.82, authorizes commit/push and selects culture buttons next, with continent recruitment selection a prospective follow-up. Logs preserved; checkpoint checks passed after environment/build-order retries. V2DLL provenance clarified by user. No game modifications. report-universal-tgc-passed-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (54). **Universal 0.10.4 works on vanilla, HPM and DoD per
user. Identical package installed in the isolated TGC2.5.2 + V2DLL5.82
copy after fresh normal-exit confirmation; player test next.**
Session 53 preserved in docs/archive/sessions.md.

## User decisions / boundaries

- Army compatibility first. Culture filters/TPL-5 and Politics/LAW-4 wait.
- Same files-only Universal package; no recipient installer/PS1/BAT.
- Conform recruits within land reach of its army and reports shortage.
- Current Conform screenshot supersedes old mockup; report observed GUI
  defects before redesigning. DoD overlap remains an extended visual case.
- User alone launches/exits. Fresh normal-exit confirmation and process
  gate before further game-copy writes. No commit/push/subagents unless asked.

## Current evidence / artifacts

- report-universal-tgc-1019.md: three-base acceptance, exact installation
  evidence and remaining limits. Available logs/statement preserved in
  work/universal-0104-confirmed-three-bases/. HPM five recruits and all five
  joins recorded; vanilla latest log is launcher-only, so acceptance is
  user-reported. Do not request those short tests again.
- Target work/Victoria-II-clean-TGC252: canonical mod/TGC, existing
  V2DLL5.82, original Lua and old Army0.8.2/vanilla0.9.0 preserved.
  37/37 Universal files verified; all protected hashes unchanged.
  Evidence work/universal-tgc-0104-prepared/{plan,installed}.json.
  Supported executable passes 172 signatures/hash. No game launched.
- ZIP work/Army-Templates-Conform-Universal-0.10.4.zip,1,041,843 bytes,
  SHA256 2e1d8cf974f2a7eea92e82067c0f505c891d0b517f8e7c9b60fb9696c029dff1.
  tgcx.dll and v2dll_mod.dll SHA256
  b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67.
- 0.10.4 corrected inverted map+2A land flag; 0.10.3 fixed native child
  activation. Prior native/GPU/installer/ZIP checks passed; no code changes
  this session. Short checks do not establish every extended edge case.
- Templates TXT loads once; edits update memory/save. Universal uses live
  engine definitions. Dynamic capacity/routes refresh on previews/actions,
  with province results cached within a snapshot. Tracking is session-only.

## Next steps in order

1. **MOD-6 TGC/V2DLL player test**, docs/instructions-mod-6.md. Already
   installed; do not reinstall. User opens target victoria2.exe, selects
   only Army Templates and Conform (Universal) + TGC v2.5.2 (stable).
   Untick both older Army entries. Expect MOD-6 0.10.4 loaded_by=V2DLL
   plugin API v1,11 land types. Check Templates and Conform/land scope.
2. Preserve Logs/tgcx.log and Logs/v2dll.log after normal exit before
   another launcher run. Check exact loader/order/version if a defect
   occurs. V2DLL may generate common/defines_v2dll.txt in Universal (P48).
3. Extended cases under MOD-6: shortage/straits/target moves, Navy/fleet
   hiding, selection churn and library reload. After reload earlier builds
   finish independently; Conform builds missing units anew, never gathers
   idle stacks. OlderTGC2.3.4/HFM remain separate untested targets.
4. Public beta preparation only when selected: instructions-release-0.md
   and report-release-readiness-1019.md. Recipient clean-install test,
   game-derived art/borrowed-code provenance, license/credits/docs remain.
   No public release/upload/push/contact authorized. Record actual results
   and close under ROADMAP7; keep rollback baselines.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| MOD-6 | vanilla/HPM/DoD short accepted; TGC2.5.2+V2DLL5.82 installed, runtime pending | instructions-mod-6.md; report-universal-tgc-1019.md |
| RELEASE-0 | assessed; public preparation/execution not selected | instructions-release-0.md; report-release-readiness-1019.md |
| MOD-3 / MOD-4 | vanilla/HPM short accepted including Universal; extended pending | instructions-mod-6.md |
| MOD-5 | DoD short accepted including Universal; TGC regression pending | instructions-mod-6.md |
| TPL-3 / TPL-4 | Ctrl/Alt and0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | user-deferred | respective instructions |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2+V2DLL5.82 passed; Universal pending | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted baselines retained | existing reports |
| LAW-3 / U-11 | passed0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; U-8 extended complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P59. Third-party art/decompiled outputs remain ignored work/.

---

# Session 53 - Three bases accepted; Universal installed for TGC/V2DLL (2026-10-19)

User confirms vanilla/HPM/DoD Universal work. Preserved available logs, prepared and installed the identical 37-file 0.10.4 payload into the isolated TGC2.5.2 + V2DLL5.82 copy after fresh normal-exit confirmation. Protected hashes unchanged; no game launched. TGC player test next. See report-universal-tgc-1019.md.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (53). **DoD Universal 0.10.4 short GUI/land-recruit check passed.
Public release readiness assessed; same-payload matrix and provenance next.**
Session 52 preserved in docs/archive/sessions.md. Next: Universal vanilla/HPM/TGC checks and extended cases under MOD-6.
Release preparation assessed, not publication-authorized.

## User decisions / boundaries

- Army compatibility first; culture filters/TPL-5 and Politics/LAW-4 wait.
- Same files-only universal package, no recipient installer/PS1/BAT.
- Conform recruits only within land reach of the selected army; reports
  shortages. Existing overseas units remain for manual transport (D17).
- Current Conform screenshot supersedes old mockup. DoD native folder
  overlap was reported; compact title-band entry needs final visual signoff.
- User alone launches/exits. Fresh normal-exit confirmation before game-copy
  writes, plus process gate. No commit/push/subagents unless asked.

## Current evidence / artifacts

- **DoD0.10.4 user confirms worked.** Full log preserved under
  work/universal-0104-dod-passed-1019: 4 pops excluded,5 builds,2/2detach;
  four join entries and fifth stack5628 still marching in snapshot. Short
  GUI/land filter accepted; not every final/extended case proved.
- Release assessment docs/report-release-readiness-1019.md: beta candidate
  technically plausible, public ZIP not ready as-is. Need same-payload
  matrix, recipient test and content rights/provenance.28 DDS in ZIP include
  game/base-derived composites; code includes marked V2DLL portions, no
  explicit license found. User chooses own license; no credits/licenses in
  current ZIP. No release/upload/push/contact performed or authorized.
  Future preparation instructions docs/instructions-release-0.md.
- **0.10.3 short DoD GUI check passed by user**, log confirms open/click,
  3/3 extra transfers. Land filter failed: 18/18 pops excluded, 0 of 4 built.
  Full log work/universal-0103-dod-route-failed-1019/tgcx.log.
  Earlier 0.10.1/0.10.2 invisible entry failures remain historical.
- **0.10.4 correction:** map province+0x2A is LAND when nonzero. Endpoint
  and path node checks fixed. Old fixture mirrored wrong polarity; added
  independent actual-native naval/coastal policy regression. Directional
  native route, cleanup and shared preview/recruit scope retained. Current
  report docs/report-universal-land-flag-1019.md. Short DoD correction check now accepted; extended cases pending.
- 172 signature/hash gates, x86 build, native/reference-window/GPU suite
  and 12 installer/Lua tests passed. ZIP manifest/CRC verified. GUI/art
  unchanged from 0.10.3; no repeated GUI tests without changes.
- Identical Universal package installed after fresh normal-exit confirmation
  in work/Victoria-II-DoDFF-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-clean-vanilla-Army090: 37/37 files each verified.
  Evidence work/universal-0104-installed.json; full previous package/log
  backup work/universal-land-flag-before-0104/<copy>/. Old 0.9.0 and root
  loaders unchanged. E:0.8.2 and TGC/V2DLL copy untouched. No game launched.
- ZIP work/Army-Templates-Conform-Universal-0.10.4.zip, 1,041,843 bytes,
  SHA256 2e1d8cf974f2a7eea92e82067c0f505c891d0b517f8e7c9b60fb9696c029dff1.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67.
- Launcher: **Army Templates and Conform (Universal)** plus DoD or HPM
  base; vanilla Universal alone. Untick old Army. Never tick two packages.
  HPM uses Historical Project Mod 0.4.5 (path fixed). Rollback selection to
  old 0.9.0 is available but retains its overseas issue.
- Templates local TXT loads once, edits save/update memory. Universal reads
  live engine definitions, not mod TXT files. Dynamic pop/route capacity
  refreshed for preview/actions, province results cached within snapshot.

## Next steps in order

1. **MOD-6 same-payload compatibility**, docs/instructions-mod-6.md:
   vanilla/HPM Universal short checks, then isolated TGC2.5.2 regression
   and Universal V2DLL route if advertised. Preserve exact hashes/logs.
   DoD0.10.4 short correction accepted; extended overseas/strait/shortage
   and target-moving cases remain evidence gaps.
2. Preserve fresh log before another launcher run. GUI access now accepted
   short; do not repeatedly redesign it without a new observed defect.
   Verify editor custom icons/pages and DoD overlap, Navy/fleet hiding,
   selection churn and save/reload library. D13 revised: reload forgets
   tracking; Conform builds missing units anew, never gathers idle stacks.
3. Release readiness: read report-release-readiness-1019.md and optional
   instructions-release-0.md. Resolve art/code permissions or alternatives
   before public release; user owns license/design/destination choices.
   TGC2.5.2/2.3.4 isolated
   regressions and Universal V2DLL route after these, under MOD-5/MOD-6
   instructions. 0.8.2 V2DLL acceptance is not Universal acceptance.
   HFM absent/unmeasured. Do not claim any/all mods solely from fixtures.
4. Record actual runtime results; close under ROADMAP7. Preserve baselines.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| MOD-6 | DoD0.10.4 short GUI/land/recruit check accepted; other Universal bases pending | instructions-mod-6.md; report-universal-land-flag-1019.md |
| RELEASE-0 | assessed; public preparation/execution not selected | instructions-release-0.md; report-release-readiness-1019.md |
| MOD-3 | vanilla0.9 short passed; Universal/extended pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM0.9 short passed; overseas defect; Universal pending | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD0.9 short passed; TGC regressions pending | instructions-mod-5.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | user-deferred | respective instructions |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2+V2DLL5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted baselines retained | existing reports |
| LAW-3 / U-11 | passed0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; U-8 extended complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P59. Third-party art/decompiled outputs remain ignored work/.

---

# Session 52 - DoD land correction accepted; release assessment (2026-10-19)

User confirms0.10.4 worked. Preserved log: four pops excluded, five built,2/2detach, four joins recorded and fifth still marching. Assessed public beta readiness: remaining Universal base/clean-user tests, provenance for game-derived art/V2DLL-derived code and release documentation/license decision. No public release or external actions.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (52). **DoD Universal 0.10.3 controls work. 0.10.4 corrects
inverted land flag; installed in all three Universal test copies.**
Session 51 preserved in docs/archive/sessions.md. Next: repeat failed
DoD mainland Conform case, then overseas exclusion and local shortage.

## User decisions / boundaries

- Army compatibility first; culture filters/TPL-5 and Politics/LAW-4 wait.
- Same files-only universal package, no recipient installer/PS1/BAT.
- Conform recruits only within land reach of the selected army; reports
  shortages. Existing overseas units remain for manual transport (D17).
- Current Conform screenshot supersedes old mockup. DoD native folder
  overlap was reported; compact title-band entry needs final visual signoff.
- User alone launches/exits. Fresh normal-exit confirmation before game-copy
  writes, plus process gate. No commit/push/subagents unless asked.

## Current evidence / artifacts

- **0.10.3 short DoD GUI check passed by user**, log confirms open/click,
  3/3 extra transfers. Land filter failed: 18/18 pops excluded, 0 of 4 built.
  Full log work/universal-0103-dod-route-failed-1019/tgcx.log.
  Earlier 0.10.1/0.10.2 invisible entry failures remain historical.
- **0.10.4 correction:** map province+0x2A is LAND when nonzero. Endpoint
  and path node checks fixed. Old fixture mirrored wrong polarity; added
  independent actual-native naval/coastal policy regression. Directional
  native route, cleanup and shared preview/recruit scope retained. Current
  report docs/report-universal-land-flag-1019.md. Runtime retry pending.
- 172 signature/hash gates, x86 build, native/reference-window/GPU suite
  and 12 installer/Lua tests passed. ZIP manifest/CRC verified. GUI/art
  unchanged from 0.10.3; no repeated GUI tests without changes.
- Identical Universal package installed after fresh normal-exit confirmation
  in work/Victoria-II-DoDFF-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-clean-vanilla-Army090: 37/37 files each verified.
  Evidence work/universal-0104-installed.json; full previous package/log
  backup work/universal-land-flag-before-0104/<copy>/. Old 0.9.0 and root
  loaders unchanged. E:0.8.2 and TGC/V2DLL copy untouched. No game launched.
- ZIP work/Army-Templates-Conform-Universal-0.10.4.zip, 1,041,843 bytes,
  SHA256 2e1d8cf974f2a7eea92e82067c0f505c891d0b517f8e7c9b60fb9696c029dff1.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67.
- Launcher: **Army Templates and Conform (Universal)** plus DoD or HPM
  base; vanilla Universal alone. Untick old Army. Never tick two packages.
  HPM uses Historical Project Mod 0.4.5 (path fixed). Rollback selection to
  old 0.9.0 is available but retains its overseas issue.
- Templates local TXT loads once, edits save/update memory. Universal reads
  live engine definitions, not mod TXT files. Dynamic pop/route capacity
  refreshed for preview/actions, province results cached within snapshot.

## Next steps in order

1. **MOD-6 DoD0.10.4 land retry**, docs/instructions-mod-6.md. Start with
   the exact mainland army wrongly excluded before. Preview must offer
   locally possible regiments; verify actual construction and march/merge.
   Then disconnected colonies excluded, reachable straits allowed, local
   shortage reported, moved target stops counting stranded reinforcements.
2. Preserve fresh log before another launcher run. GUI access now accepted
   short; do not repeatedly redesign it without a new observed defect.
   Verify editor custom icons/pages and DoD overlap, Navy/fleet hiding,
   selection churn and save/reload library. D13 revised: reload forgets
   tracking; Conform builds missing units anew, never gathers idle stacks.
3. Same payload vanilla/HPM Universal checks. TGC2.5.2/2.3.4 isolated
   regressions and Universal V2DLL route after these, under MOD-5/MOD-6
   instructions. 0.8.2 V2DLL acceptance is not Universal acceptance.
   HFM absent/unmeasured. Do not claim any/all mods solely from fixtures.
4. Record actual runtime results; close under ROADMAP7. Preserve baselines.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| MOD-6 | DoD0.10.3 GUI short accepted; 0.10.4 land fix installed, retry next | instructions-mod-6.md; report-universal-land-flag-1019.md |
| MOD-3 | vanilla0.9 short passed; Universal/extended pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM0.9 short passed; overseas defect; Universal pending | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD0.9 short passed; TGC regressions pending | instructions-mod-5.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | user-deferred | respective instructions |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2+V2DLL5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted baselines retained | existing reports |
| LAW-3 / U-11 | passed0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; U-8 extended complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P59. Third-party art/decompiled outputs remain ignored work/.


---

# Session 51 - DoD GUI accepted; inverted land flag corrected (2026-10-19)

User confirms Universal0.10.3 UI works but reachable land recruitment excluded. Full log shows 18/18 pops excluded, 0/4 built and 3/3 detach. Corrected map+2A polarity (nonzero land), independently checked with actual native naval/coastal policy bytes. 0.10.4 built/tested/packaged and installed in all three Universal copies after fresh normal-exit confirmation, 0.10.3 rollback retained. Land retry next; no game launch/commit. User caching question answered from current code.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (51). **Universal Army 0.10.3 prototype installed in vanilla,
HPM and DoD test copies. User game check next, DoD retry first.**
Session 50 is preserved in docs/archive/sessions.md.

## Current user decisions

- **0.10.2 DoD retry also failed.** Full log preserved; roots created and
  positioned, no open/click events. 0.10.3 subtree-activation candidate
  installed after fresh normal-exit confirmation, runtime pending.
  Root-only Show was insufficient in actual-native hidden-child fixture.
- **Universal 0.10.1 failed first DoD check:** game opened, controls missing.
  DLL/catalogue/root construction logged; visibility/entry gate failed.
  0.10.2 adds foreground-on-show, host hiding and placement diagnostics.
  Retry DoD first; no Universal runtime acceptance yet.
- **Conform: recruit only within land reach of the target; report shortages.**
  User reports overseas problem in all three 0.9.0 copies, not Universal.
  Shared planner fixed since 0.10.1; Universal display still failed at 0.10.1.
- Army compatibility first (D15); culture filtering/TPL-5 and Politics/LAW-4
  wait. Ctrl/Alt-click and base-specific vanilla/HPM/DoD short checks passed.
- Same files-only universal package: no recipient installer/PS1/BAT.
- DoD Templates overlaps added native unit tabs: candidate uses a compact
  entry above dense rows; user visual acceptance pending.
- Current Conform screenshot supersedes old mockup: visual-references.md.
- User controls game launch/exit. No commit/push/subagents unless asked.

## Installed candidate and evidence

- Current candidate **0.10.3**: report-universal-subtree-1019.md.
  Native secondary Show activates buttons/children on hidden-to-visible
  transitions; polling preserves hidden covers. Native byte regression
  reproduces root-only failure and exercises activation/reopen.
  Full 0.10.2 failed log: work/universal-dod-0102-failed-1019/tgcx.log.
  Retry expects entry activated/subtree=1 then visible/clickable controls.
- Prior 0.10.2 report: report-universal-visibility-1019.md. Evidence snapshot
  work/universal-dod-missing-1019. Initial log observed root creation; later
  read was overwritten with Lua-only line, so retain scope/excerpt limits.
  Retry should log entry placed pixels/foreground or exact refused stage.
- Land-reach correction: report-conform-land-reach-1019.md. Preview and
  recruitment share native route scope; sea paths excluded. Stranded builds
  stop counting as pending. Existing units/constructions/orders retained;
  old overseas troops need manual transport. Standalone Recruit unchanged.
- work/Victoria-II-clean-vanilla-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-DoDFF-Army090: **37 identical new package files each**,
  installed after fresh exit confirmation/process gate. Old 0.9.0 packages
  and root loaders unchanged. Per-copy UNIVERSAL_ARMY_TEST.txt and manifests;
  global evidence work/universal-0103-installed.json. No game launched.
  Only logic/version updated; full 0.10.2 backup work/universal-subtree-before-0103.
- Select **Army Templates and Conform (Universal)** and untick old Army.
  Vanilla: Universal alone. HPM: add Historical Project Mod 0.4.5 (path fixed).
  DoD: add Divergences of Darkness Fan Fork. Never tick two Army packages.
- Rollback: untick Universal and retick the accepted old Army package.
  E: accepted 0.8.2 and existing clean TGC/V2DLL copy untouched.
- ZIP work/Army-Templates-Conform-Universal-0.10.3.zip, 1,041,868 bytes,
  SHA256 df259a13eb92915f263ec8a90d40969f3bbf73e88502926500a3134ec26a4b4c.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  ea9453576cce5c2906847855011d8887ca2a0b341dfceed47d7370f94b9fcd45.
- Shared additive GUI, native factory/deferred release, live keys/icons/time;
  no original-GUI overrides or per-base metadata. 172 signatures, x86 build,
  native/GPU + 12 installer tests, 8 relevant Python tests and ZIP/dependency checks
  passed. **0.10.3 not yet tested in game.** report-universal-subtree-1019.md;
  report-conform-land-reach-1019.md describes retained recruitment scope;
  report-mod-6-1019.md retains the 0.10.0 architecture snapshot.
- HPM/DoD 0.9.0 acceptance: report-mod-4-5-passed-1019.md;
  logs work/mod4-mod5-passed-1019. HPM 9/9 extras, 5 built; DoD 1/1 extra,
  4 built, custom Regular live index 23. Snapshots do not show every final
  merge. Vanilla acceptance: report-mod-3-passed-1019.md. Extended
  save/reload, Navy/fleet and sustained turnover remain checks.
- Accepted per-base stages retained under work/army-090-conform-fix and
  own-loader stages. 0.9 baseline binaries work/mod6-baseline-090.
  Universal art/decompiled evidence stays ignored under work, outside git.

## Next session in order

1. **MOD-6 user check**, instructions-mod-6.md: DoD retry first, then vanilla
   (custom types and visual overlap), then HPM. Preserve fresh logs after
   normal exit; check real shared-root construction, recruit/Conform,
   selection turnover, Navy/fleet suppression and persistence. Also test a
   colonial country: mainland recruitment excludes unreachable colonies,
   local shortages show, overseas armies use their own reachable pool,
   native straits work, and a moved target releases stranded tracking.
2. If passed, isolated universal TGC 2.5.2/2.3.4 regression and V2DLL route
   under instructions-mod-6.md and instructions-mod-5.md. Same payload.
   HFM absent/unmeasured; do not claim all versions from fixtures.
3. If failed, preserve logs/selection and report exact named failure stage;
   follow MOD-6 instructions. No signature loosening or guessed offsets.
4. Close under ROADMAP section 7 with measured evidence and current next
   instructions. Culture/Politics remain deferred.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| MOD-6 | 0.10.2 DoD failed; 0.10.3 candidate installed, retry next | instructions-mod-6.md; report-mod-6-1019.md |
| MOD-3 | vanilla 0.9.0 short check passed; extended checks pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM 0.9.0 short check passed | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD 0.9.0 short check passed; TGC regressions pending | instructions-mod-5.md; report-mod-4-5-passed-1019.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | deferred by user | instructions-tpl-5.md; instructions-law-4.md |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2 + V2DLL 5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author request D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted 0.7.0 / 0.8.0 retained | existing reports |
| LAW-3 / U-11 | passed 0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P58. Preserve originals and paired rollback. Fresh normal-exit
confirmation is required before any new game-copy installation.


---

# Session 50 - DoD0.10.2 failed; subtree activation candidate installed (2026-10-19)

Full failed log preserved: created/positioned entries, no open events. Native reference reproduction shows root-only visibility leaves hidden children inactive. 0.10.3 uses checked secondary subtree Show on transitions; native/GPU/installer/Python tests pass. Installed identical packages in three copies after confirmed exit, prior version backed up. DoD retry next, no Universal acceptance or game launch.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (50). **Universal Army 0.10.2 prototype installed in vanilla,
HPM and DoD test copies. User game check next, DoD retry first.**
Session 49 is preserved in docs/archive/sessions.md.

## Current user decisions

- **Universal 0.10.1 failed first DoD check:** game opened, controls missing.
  DLL/catalogue/root construction logged; visibility/entry gate failed.
  0.10.2 adds foreground-on-show, host hiding and placement diagnostics.
  Retry DoD first; no Universal runtime acceptance yet.
- **Conform: recruit only within land reach of the target; report shortages.**
  User reports overseas problem in all three 0.9.0 copies, not Universal.
  Shared planner fixed since 0.10.1; Universal display still failed at 0.10.1.
- Army compatibility first (D15); culture filtering/TPL-5 and Politics/LAW-4
  wait. Ctrl/Alt-click and base-specific vanilla/HPM/DoD short checks passed.
- Same files-only universal package: no recipient installer/PS1/BAT.
- DoD Templates overlaps added native unit tabs: candidate uses a compact
  entry above dense rows; user visual acceptance pending.
- Current Conform screenshot supersedes old mockup: visual-references.md.
- User controls game launch/exit. No commit/push/subagents unless asked.

## Installed candidate and evidence

- Current report: report-universal-visibility-1019.md. Evidence snapshot
  work/universal-dod-missing-1019. Initial log observed root creation; later
  read was overwritten with Lua-only line, so retain scope/excerpt limits.
  Retry should log entry placed pixels/foreground or exact refused stage.
- Land-reach correction: report-conform-land-reach-1019.md. Preview and
  recruitment share native route scope; sea paths excluded. Stranded builds
  stop counting as pending. Existing units/constructions/orders retained;
  old overseas troops need manual transport. Standalone Recruit unchanged.
- work/Victoria-II-clean-vanilla-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-DoDFF-Army090: **37 identical new package files each**,
  installed after fresh exit confirmation/process gate. Old 0.9.0 packages
  and root loaders unchanged. Per-copy UNIVERSAL_ARMY_TEST.txt and manifests;
  global evidence work/universal-0102-installed.json. No game launched.
  Only logic/version updated; full 0.10.1 backup work/universal-visibility-before-0102.
- Select **Army Templates and Conform (Universal)** and untick old Army.
  Vanilla: Universal alone. HPM: add Historical Project Mod 0.4.5 (path fixed).
  DoD: add Divergences of Darkness Fan Fork. Never tick two Army packages.
- Rollback: untick Universal and retick the accepted old Army package.
  E: accepted 0.8.2 and existing clean TGC/V2DLL copy untouched.
- ZIP work/Army-Templates-Conform-Universal-0.10.2.zip, 1,041,372 bytes,
  SHA256 28314c6606fad1e2d23672d9826abf5b65882624bcb14975808b4ff640d542c6.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  d2f6a18490a1cd9c05fabd90f86e99cc66d917633b58798ac7fb656eaa6a6857.
- Shared additive GUI, native factory/deferred release, live keys/icons/time;
  no original-GUI overrides or per-base metadata. 169 signatures, x86 build,
  native/GPU + 12 installer tests, 8 relevant Python tests and ZIP/dependency checks
  passed. **0.10.2 not yet tested in game.** report-universal-visibility-1019.md;
  report-conform-land-reach-1019.md describes retained recruitment scope;
  report-mod-6-1019.md retains the 0.10.0 architecture snapshot.
- HPM/DoD 0.9.0 acceptance: report-mod-4-5-passed-1019.md;
  logs work/mod4-mod5-passed-1019. HPM 9/9 extras, 5 built; DoD 1/1 extra,
  4 built, custom Regular live index 23. Snapshots do not show every final
  merge. Vanilla acceptance: report-mod-3-passed-1019.md. Extended
  save/reload, Navy/fleet and sustained turnover remain checks.
- Accepted per-base stages retained under work/army-090-conform-fix and
  own-loader stages. 0.9 baseline binaries work/mod6-baseline-090.
  Universal art/decompiled evidence stays ignored under work, outside git.

## Next session in order

1. **MOD-6 user check**, instructions-mod-6.md: DoD retry first, then vanilla
   (custom types and visual overlap), then HPM. Preserve fresh logs after
   normal exit; check real shared-root construction, recruit/Conform,
   selection turnover, Navy/fleet suppression and persistence. Also test a
   colonial country: mainland recruitment excludes unreachable colonies,
   local shortages show, overseas armies use their own reachable pool,
   native straits work, and a moved target releases stranded tracking.
2. If passed, isolated universal TGC 2.5.2/2.3.4 regression and V2DLL route
   under instructions-mod-6.md and instructions-mod-5.md. Same payload.
   HFM absent/unmeasured; do not claim all versions from fixtures.
3. If failed, preserve logs/selection and report exact named failure stage;
   follow MOD-6 instructions. No signature loosening or guessed offsets.
4. Close under ROADMAP section 7 with measured evidence and current next
   instructions. Culture/Politics remain deferred.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| MOD-6 | 0.10.2 prototype installed; user game check next | instructions-mod-6.md; report-mod-6-1019.md |
| MOD-3 | vanilla 0.9.0 short check passed; extended checks pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM 0.9.0 short check passed | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD 0.9.0 short check passed; TGC regressions pending | instructions-mod-5.md; report-mod-4-5-passed-1019.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | deferred by user | instructions-tpl-5.md; instructions-law-4.md |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2 + V2DLL 5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author request D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted 0.7.0 / 0.8.0 retained | existing reports |
| LAW-3 / U-11 | passed 0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P57. Preserve originals and paired rollback. Fresh normal-exit
confirmation is required before any new game-copy installation.


---

# Session 49 - Universal DoD entry failure; visibility repair installed (2026-10-19)

User confirms game opened but controls missing. Log proved Universal loaded/created roots. Added first-show/reopen foreground promotion, independent host hiding and placement diagnostics. 0.10.2 built/tested; all three copies updated after fresh normal-exit confirmation with full 0.10.1 backup. DoD retry first, no runtime acceptance. No game launch/commit.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (49). **Universal Army 0.10.1 prototype installed in vanilla,
HPM and DoD test copies. User game check next, vanilla first.**
Session 48 is preserved in docs/archive/sessions.md.

## Current user decisions

- **Conform: recruit only within land reach of the target; report shortages.**
  User reports overseas problem in all three 0.9.0 copies, not Universal.
  Shared planner fixed in 0.10.1; Universal integration still untested.
- Army compatibility first (D15); culture filtering/TPL-5 and Politics/LAW-4
  wait. Ctrl/Alt-click and base-specific vanilla/HPM/DoD short checks passed.
- Same files-only universal package: no recipient installer/PS1/BAT.
- DoD Templates overlaps added native unit tabs: candidate uses a compact
  entry above dense rows; user visual acceptance pending.
- Current Conform screenshot supersedes old mockup: visual-references.md.
- User controls game launch/exit. No commit/push/subagents unless asked.

## Installed candidate and evidence

- Land-reach correction: report-conform-land-reach-1019.md. Preview and
  recruitment share native route scope; sea paths excluded. Stranded builds
  stop counting as pending. Existing units/constructions/orders retained;
  old overseas troops need manual transport. Standalone Recruit unchanged.
- work/Victoria-II-clean-vanilla-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-DoDFF-Army090: **37 identical new package files each**,
  installed after fresh exit confirmation/process gate. Old 0.9.0 packages
  and root loaders unchanged. Per-copy UNIVERSAL_ARMY_TEST.txt and manifests;
  global evidence work/universal-0101-installed.json. No game launched.
  Only logic/version updated; full 0.10.0 backup work/conform-land-reach-before-0101.
- Select **Army Templates and Conform (Universal)** and untick old Army.
  Vanilla: Universal alone. HPM: add Historical Project Mod 0.4.5 (path fixed).
  DoD: add Divergences of Darkness Fan Fork. Never tick two Army packages.
- Rollback: untick Universal and retick the accepted old Army package.
  E: accepted 0.8.2 and existing clean TGC/V2DLL copy untouched.
- ZIP work/Army-Templates-Conform-Universal-0.10.1.zip, 1,040,653 bytes,
  SHA256 5d1c370400e51870f8d9a51feb8dc7eaf04573d72dbc8d6bd8244a3936db183e.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  2363ef5615b9f20b5f7dd55bb9f89729f9cf1e721aba7f93d43b50bebff847bf.
- Shared additive GUI, native factory/deferred release, live keys/icons/time;
  no original-GUI overrides or per-base metadata. 169 signatures, x86 build,
  native/GPU + 12 installer tests, 13 relevant Python tests and ZIP/dependency checks
  passed. **Not yet tested in game.** report-conform-land-reach-1019.md;
  report-mod-6-1019.md retains the 0.10.0 architecture snapshot.
- HPM/DoD 0.9.0 acceptance: report-mod-4-5-passed-1019.md;
  logs work/mod4-mod5-passed-1019. HPM 9/9 extras, 5 built; DoD 1/1 extra,
  4 built, custom Regular live index 23. Snapshots do not show every final
  merge. Vanilla acceptance: report-mod-3-passed-1019.md. Extended
  save/reload, Navy/fleet and sustained turnover remain checks.
- Accepted per-base stages retained under work/army-090-conform-fix and
  own-loader stages. 0.9 baseline binaries work/mod6-baseline-090.
  Universal art/decompiled evidence stays ignored under work, outside git.

## Next session in order

1. **MOD-6 user check**, instructions-mod-6.md: vanilla first, then DoD
   (custom types and visual overlap), then HPM. Preserve fresh logs after
   normal exit; check real shared-root construction, recruit/Conform,
   selection turnover, Navy/fleet suppression and persistence. Also test a
   colonial country: mainland recruitment excludes unreachable colonies,
   local shortages show, overseas armies use their own reachable pool,
   native straits work, and a moved target releases stranded tracking.
2. If passed, isolated universal TGC 2.5.2/2.3.4 regression and V2DLL route
   under instructions-mod-6.md and instructions-mod-5.md. Same payload.
   HFM absent/unmeasured; do not claim all versions from fixtures.
3. If failed, preserve logs/selection and report exact named failure stage;
   follow MOD-6 instructions. No signature loosening or guessed offsets.
4. Close under ROADMAP section 7 with measured evidence and current next
   instructions. Culture/Politics remain deferred.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| MOD-6 | 0.10.1 prototype installed; user game check next | instructions-mod-6.md; report-mod-6-1019.md |
| MOD-3 | vanilla 0.9.0 short check passed; extended checks pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM 0.9.0 short check passed | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD 0.9.0 short check passed; TGC regressions pending | instructions-mod-5.md; report-mod-4-5-passed-1019.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | deferred by user | instructions-tpl-5.md; instructions-law-4.md |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2 + V2DLL 5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author request D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted 0.7.0 / 0.8.0 retained | existing reports |
| LAW-3 / U-11 | passed 0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P56. Preserve originals and paired rollback. Fresh normal-exit
confirmation is required before any new game-copy installation.


---

# Session 48 - Conform land-reach fix installed (2026-10-19)

User reports overseas failure in all three 0.9.0 copies; Universal still untested. Approved land-reachable recruitment only, shortages reported. Added guarded native route query, shared preview/recruit scope and stranded tracking correction. 0.10.1 built/tested/packaged; three Universal copies updated after fresh exit confirmation. Old 0.9.0 and 0.10.0 rollback retained. No game launched.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (48). **Universal Army 0.10.0 prototype installed in vanilla,
HPM and DoD test copies. User game check next, vanilla first.**
Session 47 is preserved in docs/archive/sessions.md.

## Current user decisions

- Army compatibility first (D15); culture filtering/TPL-5 and Politics/LAW-4
  wait. Ctrl/Alt-click and base-specific vanilla/HPM/DoD short checks passed.
- Same files-only universal package: no recipient installer/PS1/BAT.
- DoD Templates overlaps added native unit tabs: candidate uses a compact
  entry above dense rows; user visual acceptance pending.
- Current Conform screenshot supersedes old mockup: visual-references.md.
- User controls game launch/exit. No commit/push/subagents unless asked.

## Installed candidate and evidence

- work/Victoria-II-clean-vanilla-Army090, work/Victoria-II-HPM-Army090 and
  work/Victoria-II-DoDFF-Army090: **37 identical new package files each**,
  installed after fresh exit confirmation/process gate. Old 0.9.0 packages
  and root loaders unchanged. Per-copy UNIVERSAL_ARMY_TEST.txt and manifests;
  global evidence work/universal-010-installed.json. No game launched.
- Select **Army Templates and Conform (Universal)** and untick old Army.
  Vanilla: Universal alone. HPM: add Historical Project Mod 0.4.5 (path fixed).
  DoD: add Divergences of Darkness Fan Fork. Never tick two Army packages.
- Rollback: untick Universal and retick the accepted old Army package.
  E: accepted 0.8.2 and existing clean TGC/V2DLL copy untouched.
- ZIP work/Army-Templates-Conform-Universal-0.10.0.zip, 1,037,796 bytes,
  SHA256 60da0ea1fd9202cc859308479a832afc5a81318b7c48061384528e228045d6c8.
  Identical tgcx.dll/v2dll_mod.dll SHA256
  1c3c518f78cf5baa77ddbdb0f2f11eb5f39dfab87739636e426e1597c9cdf596.
- Shared additive GUI, native factory/deferred release, live keys/icons/time;
  no original-GUI overrides or per-base metadata. 164 signatures, x86 build,
  native/GPU + 12 installer tests, 34 Python tests and ZIP/dependency checks
  passed. **Not yet tested in game.** report-mod-6-1019.md.
- HPM/DoD 0.9.0 acceptance: report-mod-4-5-passed-1019.md;
  logs work/mod4-mod5-passed-1019. HPM 9/9 extras, 5 built; DoD 1/1 extra,
  4 built, custom Regular live index 23. Snapshots do not show every final
  merge. Vanilla acceptance: report-mod-3-passed-1019.md. Extended
  save/reload, Navy/fleet and sustained turnover remain checks.
- Accepted per-base stages retained under work/army-090-conform-fix and
  own-loader stages. 0.9 baseline binaries work/mod6-baseline-090.
  Universal art/decompiled evidence stays ignored under work, outside git.

## Next session in order

1. **MOD-6 user check**, instructions-mod-6.md: vanilla first, then DoD
   (custom types and visual overlap), then HPM. Preserve fresh logs after
   normal exit; check real shared-root construction, recruit/Conform,
   selection turnover, Navy/fleet suppression and persistence.
2. If passed, isolated universal TGC 2.5.2/2.3.4 regression and V2DLL route
   under instructions-mod-6.md and instructions-mod-5.md. Same payload.
   HFM absent/unmeasured; do not claim all versions from fixtures.
3. If failed, preserve logs/selection and report exact named failure stage;
   follow MOD-6 instructions. No signature loosening or guessed offsets.
4. Close under ROADMAP section 7 with measured evidence and current next
   instructions. Culture/Politics remain deferred.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| MOD-6 | 0.10.0 prototype installed; user game check next | instructions-mod-6.md; report-mod-6-1019.md |
| MOD-3 | vanilla 0.9.0 short check passed; extended checks pending | instructions-mod-3.md; report-mod-3-passed-1019.md |
| MOD-4 | HPM 0.9.0 short check passed | instructions-mod-4.md; report-mod-4-5-passed-1019.md |
| MOD-5 | DoD 0.9.0 short check passed; TGC regressions pending | instructions-mod-5.md; report-mod-4-5-passed-1019.md |
| TPL-3 / TPL-4 | Ctrl/Alt accepted; 0.8.2 detach accepted | report-tpl-4-runtime-1019.md |
| TPL-5 / LAW-4 | deferred by user | instructions-tpl-5.md; instructions-law-4.md |
| U-12 | Army parts accepted; Bill deferred | instructions-u-12.md |
| V2D-082 | 0.8.2 + V2DLL 5.82 passed | report-v2d-082-passed-1019.md |
| V2D-2 | waiting author request D16 | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 | accepted 0.7.0 / 0.8.0 retained | existing reports |
| LAW-3 / U-11 | passed 0.6.4; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P55. Preserve originals and paired rollback. Fresh normal-exit
confirmation is required before any new game-copy installation.


---

# Session 47 - Universal Army prototype installed (2026-10-19)

HPM/DoD short checks accepted; successful logs preserved. Implemented shared native GUI creation, live unit catalogue/icons and DoD compact entry. Built/tested/packaged 0.10.0 and added identical files to three copies after fresh exit confirmation. Working 0.9.0 packages/loaders retained. Universal user game check next; no game launch/commit/push.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (47). **HPM and DoD clean test copies ready with Army 0.9.0
and our loader. HPM user game check next.**
Session 46 is preserved in docs/archive/sessions.md.

## Current user decisions

- Finish Army across mod/base versions first (D15), on the reference 3.04
  executable. Politics/LAW-4 and primary+accepted/primary-only filtering
  (TPL-5) wait. Ctrl/Alt-click works: **TPL-3 accepted by the user**.
- Current Conform screenshot is authoritative; old mockup is outdated.
  Read docs/visual-references.md; preserve native army-style layout.
- Files-only distribution: no installer, PS1, BAT or launch scripts for
  recipients. Mod-only and pinned-V2DLL variants are staged.
- Previous clean TGC copy now has user-installed V2DLL. User requested
  another clean copy: use the separate vanilla folder below.

## Current artifacts and evidence

- **New test copies:** work/Victoria-II-HPM-Army090 and
  work/Victoria-II-DoDFF-Army090. Own loader, Army 0.9.0, no V2DLL.
  17,324 / 19,802 source files verified; both executable gates pass.
  Installed after fresh normal-exit confirmation. HPM select only its
  Army + Historical Project Mod 0.4.5 (path fixed); DoD select Army + DoD.
  See docs/report-mod-4-5-prepared-1019.md and per-copy README/manifests.
- **Universal direction approved:** after HPM/DoD validation, MOD-6 research
  and implementation (instructions-mod-6.md). Initial native factory
  evidence recorded in symbols.md; no universal build or new DLL hooks.
- **Latest acceptance:** docs/report-mod-3-passed-1019.md. User confirms
  success; log proves 7/7 and 3/3 extras detached, four recruits queued,
  all four new stacks marched and merged. Evidence work/mod3-090-passed-1019.
  Save/reload and explicit Navy/fleet checks remain extended coverage.
  One DLL already serves every base; a universal shared package needs
  runtime GUI/units work (instructions-mod-6.md), now selected after HPM/DoD.
- **Latest repair:** docs/report-mod-3-button-1019.md. Vanilla log confirms
  2 recruits queued, 0 omitted. Conform tab existed but used TGC y=68,
  under vanilla statistics. Generated anchor now gives y=42 for vanilla,
  HPM and DoD; TGC stays y=68. Exactly two GUI files updated in the fresh
  vanilla copy; DLL unchanged. Backup work/army-090-conform-fix-install-1019.
  Opening/missing/excess/march/merge now passed; save/reload remains extended.
- **New target:** work/Victoria-II-clean-vanilla-Army090. Vanilla copied
  from clean D:, 8,961 base files/851,338,278 bytes verified; our MOD-2 loader
  and Army 0.9.0 installed after normal-exit confirmation/process gate.
  No TGC/V2DLL. Open its victoria2.exe, tick only Army (vanilla).
  Manifest/README in that folder; docs/report-mod-3-fresh-1019.md.
- **0.9.0 DLL:** dll/tgcx/Release/tgcx.dll, SHA256
  ee56551e11b5f00537e7d685309f6f7ff848669329a8a405eb1bbc820f2e519b.
  Per-base runtime metadata, keys mapped to live engine indices; up to 256
  catalogue types/indices below 1024. Mismatches refuse operations.
- **Current prepared matrix:** work/army-090-conform-fix, vanilla/HPM0.4.5/TGC 2.3.4/
  TGC 2.5.2/DoDFF. Same DLL, effective base GUI/art/catalogue. Every ZIP entry
  and CRC verified; no scripts. HFM absent. See docs/report-mod-5-1019.md.
- Build/153 signatures/native+GPU/12 installer tests and 32 Python tests
  pass. TGC art (27 files) byte-identical to accepted 0.8.2. **Vanilla Templates has
  runtime acceptance; Conform short check now passed, extended coverage pending**.
- Earlier work/Victoria-II-clean-TGC252 is retained with V2DLL 5.82 and
  accepted Army 0.8.2; also has the additive vanilla 0.9.0 package, now
  superseded as test target. Canonical stable base mod/TGC (P52), profile
  TGC252_Fresh082_20261019. No files removed from this copy.
- E: remains accepted 0.8.2; package DLL SHA256 1ca013f5...a7e43, original
  MOD-2 loader. Stage E:/v2re/logs/tgcx-tpl4-1019/stage-082; backup
  before-082-submod. Accepted 0.8.2 sharing ZIPs remain in work/.

## Next session in order

1. **MOD-4 HPM user game check:** docs/instructions-mod-4.md. Copy ready,
   no installation needed. Inspect its fresh log after the user tests.
2. **MOD-5 DoD/custom units and TGC regression:** docs/instructions-mod-5.md.
   Extra types/icons/recruit/Conform; stable TGC0.9 regression and older
   TGC in its own copy. HFM remains unmeasured until available.
3. Extended vanilla coverage: save/reload and explicit Navy/fleet hiding
   (instructions-mod-3.md). Short recruitment/Conform check already passed.
4. **MOD-6 universal package approved after HPM/DoD validation**,
   instructions-mod-6.md. Follow the initial factory evidence in symbols.md;
   verify native construction, ownership/cleanup and dynamic unit icons.
5. Close under ROADMAP section 7; report actual per-base evidence before
   calling compatibility finished. No commit/push/subagents unless asked.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| TPL-3 | **user-confirmed passed**, Ctrl/Alt | report-mod-5-1019.md |
| TPL-4 | **0.8.2 passed in game**, own loader and V2DLL | report-tpl-4-runtime-1019.md; report-v2d-082-passed-1019.md |
| MOD-3 | **Short vanilla recruit/Conform check passed**; extended checks pending | instructions-mod-3.md; report-mod-3-fresh-1019.md |
| MOD-4 | own-loader test copy installed/verified; user game check next | instructions-mod-4.md |
| MOD-5 | DoD own-loader copy ready; custom-unit/TGC game checks pending | instructions-mod-5.md; report-mod-5-1019.md |
| TPL-5 | deferred by user until Army compatibility | instructions-tpl-5.md |
| LAW-4 | deferred while Army has priority | instructions-law-4.md |
| U-12 | Ctrl/Alt and detach passed; Bill portion deferred | instructions-u-12.md |
| V2D-082 | **passed** Army 0.8.2 + V2DLL 5.82 | report-v2d-082-passed-1019.md |
| V2D-2 | waiting on author request D16 (search all -mod folders) | report-v2dll-mod-slot-1018.md |
| MOD-2 / MOD-1 | accepted 0.8.0 /0.7.0 TGC game checks retained | report-mod-2-1018.md; report-mod-1-1018.md |
| LAW-3 / U-11 | passed0.6.4; G1/G3 open, deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0, Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P54. Preserve originals, use paired backups, do not mix loader
filenames, and do not install on a new test run without fresh exit confirmation.


---

# Session 46 - HPM/DoD copies ready; universal route selected (2026-10-19)

User approved HPM/DoD validation then universal package. Prepared and source-verified separate clean game/base copies, installed own loader and Army0.9.0 after fresh normal-exit confirmation, verified both executables. HPM user game check next. Read-only factory research produced initial MOD-6 evidence; no game launched or code changed.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (46). **Vanilla Templates + Conform short game check passed.**
HPM and DoD game checks next; universal-package research proposed.
Session 45 is preserved in docs/archive/sessions.md.

## Current user decisions

- Finish Army across mod/base versions first (D15), on the reference 3.04
  executable. Politics/LAW-4 and primary+accepted/primary-only filtering
  (TPL-5) wait. Ctrl/Alt-click works: **TPL-3 accepted by the user**.
- Current Conform screenshot is authoritative; old mockup is outdated.
  Read docs/visual-references.md; preserve native army-style layout.
- Files-only distribution: no installer, PS1, BAT or launch scripts for
  recipients. Mod-only and pinned-V2DLL variants are staged.
- Previous clean TGC copy now has user-installed V2DLL. User requested
  another clean copy: use the separate vanilla folder below.

## Current artifacts and evidence

- **Latest acceptance:** docs/report-mod-3-passed-1019.md. User confirms
  success; log proves 7/7 and 3/3 extras detached, four recruits queued,
  all four new stacks marched and merged. Evidence work/mod3-090-passed-1019.
  Save/reload and explicit Navy/fleet checks remain extended coverage.
  One DLL already serves every base; a universal shared package needs
  runtime GUI/units work (proposed instructions-mod-6.md), not yet selected.
- **Latest repair:** docs/report-mod-3-button-1019.md. Vanilla log confirms
  2 recruits queued, 0 omitted. Conform tab existed but used TGC y=68,
  under vanilla statistics. Generated anchor now gives y=42 for vanilla,
  HPM and DoD; TGC stays y=68. Exactly two GUI files updated in the fresh
  vanilla copy; DLL unchanged. Backup work/army-090-conform-fix-install-1019.
  Opening/missing/excess/march/merge now passed; save/reload remains extended.
- **New target:** work/Victoria-II-clean-vanilla-Army090. Vanilla copied
  from clean D:, 8,961 base files/851,338,278 bytes verified; our MOD-2 loader
  and Army 0.9.0 installed after normal-exit confirmation/process gate.
  No TGC/V2DLL. Open its victoria2.exe, tick only Army (vanilla).
  Manifest/README in that folder; docs/report-mod-3-fresh-1019.md.
- **0.9.0 DLL:** dll/tgcx/Release/tgcx.dll, SHA256
  ee56551e11b5f00537e7d685309f6f7ff848669329a8a405eb1bbc820f2e519b.
  Per-base runtime metadata, keys mapped to live engine indices; up to 256
  catalogue types/indices below 1024. Mismatches refuse operations.
- **Current prepared matrix:** work/army-090-conform-fix, vanilla/HPM0.4.5/TGC 2.3.4/
  TGC 2.5.2/DoDFF. Same DLL, effective base GUI/art/catalogue. Every ZIP entry
  and CRC verified; no scripts. HFM absent. See docs/report-mod-5-1019.md.
- Build/153 signatures/native+GPU/12 installer tests and 32 Python tests
  pass. TGC art (27 files) byte-identical to accepted 0.8.2. **Vanilla Templates has
  runtime acceptance; Conform short check now passed, extended coverage pending**.
- Earlier work/Victoria-II-clean-TGC252 is retained with V2DLL 5.82 and
  accepted Army 0.8.2; also has the additive vanilla 0.9.0 package, now
  superseded as test target. Canonical stable base mod/TGC (P52), profile
  TGC252_Fresh082_20261019. No files removed from this copy.
- E: remains accepted 0.8.2; package DLL SHA256 1ca013f5...a7e43, original
  MOD-2 loader. Stage E:/v2re/logs/tgcx-tpl4-1019/stage-082; backup
  before-082-submod. Accepted 0.8.2 sharing ZIPs remain in work/.

## Next session in order

1. **MOD-4 HPM:** docs/instructions-mod-4.md, isolated copy, our path-fixed
   descriptor. Fresh exit confirmation before installation.
2. **MOD-5 DoD/custom units and TGC regression:** docs/instructions-mod-5.md.
   Extra types/icons/recruit/Conform; stable TGC0.9 regression and older
   TGC in its own copy. HFM remains unmeasured until available.
3. Extended vanilla coverage: save/reload and explicit Navy/fleet hiding
   (instructions-mod-3.md). Short recruitment/Conform check already passed.
4. Proposed **MOD-6 universal package**, instructions-mod-6.md: same DLL
   already, but runtime GUI creation/attachment and unit icons need research.
   User asked about it; await architecture selection before implementing.
5. Close under ROADMAP section 7; report actual per-base evidence before
   calling compatibility finished. No commit/push/subagents unless asked.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| TPL-3 | **user-confirmed passed**, Ctrl/Alt | report-mod-5-1019.md |
| TPL-4 | **0.8.2 passed in game**, own loader and V2DLL | report-tpl-4-runtime-1019.md; report-v2d-082-passed-1019.md |
| MOD-3 | **Short vanilla recruit/Conform check passed**; extended checks pending | instructions-mod-3.md; report-mod-3-fresh-1019.md |
| MOD-4 | HPM package generated/verified; runtime pending | instructions-mod-4.md |
| MOD-5 | runtime catalogue implemented/fixture-tested; DoD/TGC game checks pending | instructions-mod-5.md; report-mod-5-1019.md |
| TPL-5 | deferred by user until Army compatibility | instructions-tpl-5.md |
| LAW-4 | deferred while Army has priority | instructions-law-4.md |
| U-12 | Ctrl/Alt and detach passed; Bill portion deferred | instructions-u-12.md |
| V2D-082 | **passed** Army 0.8.2 + V2DLL 5.82 | report-v2d-082-passed-1019.md |
| V2D-2 | waiting on author request D16 (search all -mod folders) | report-v2dll-mod-slot-1018.md |
| MOD-2 / MOD-1 | accepted 0.8.0 /0.7.0 TGC game checks retained | report-mod-2-1018.md; report-mod-1-1018.md |
| LAW-3 / U-11 | passed0.6.4; G1/G3 open, deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0, Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P54. Preserve originals, use paired backups, do not mix loader
filenames, and do not install on a new test run without fresh exit confirmation.


---

# Session 45 - Vanilla accepted, universal package discussion (2026-10-19)

User confirms Conform works. Logs show 7/7 and 3/3 detach, four recruits queued and four new stacks march/merge. Short vanilla check accepted; extended coverage remains unconfirmed. Reviewed single-DLL versus universal-package architecture; MOD-6 research proposal documented, not selected/implemented. No game changes.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (45). **Vanilla Templates works; Conform opening-button correction
installed after normal-exit confirmation. User retry next.**
Session 44 is preserved in docs/archive/sessions.md.

## Current user decisions

- Finish Army across mod/base versions first (D15), on the reference 3.04
  executable. Politics/LAW-4 and primary+accepted/primary-only filtering
  (TPL-5) wait. Ctrl/Alt-click works: **TPL-3 accepted by the user**.
- Current Conform screenshot is authoritative; old mockup is outdated.
  Read docs/visual-references.md; preserve native army-style layout.
- Files-only distribution: no installer, PS1, BAT or launch scripts for
  recipients. Mod-only and pinned-V2DLL variants are staged.
- Previous clean TGC copy now has user-installed V2DLL. User requested
  another clean copy: use the separate vanilla folder below.

## Current artifacts and evidence

- **Latest repair:** docs/report-mod-3-button-1019.md. Vanilla log confirms
  2 recruits queued, 0 omitted. Conform tab existed but used TGC y=68,
  under vanilla statistics. Generated anchor now gives y=42 for vanilla,
  HPM and DoD; TGC stays y=68. Exactly two GUI files updated in the fresh
  vanilla copy; DLL unchanged. Backup work/army-090-conform-fix-install-1019.
  User retry Conform opening, missing/excess operations and save/reload next.
- **New target:** work/Victoria-II-clean-vanilla-Army090. Vanilla copied
  from clean D:, 8,961 base files/851,338,278 bytes verified; our MOD-2 loader
  and Army 0.9.0 installed after normal-exit confirmation/process gate.
  No TGC/V2DLL. Open its victoria2.exe, tick only Army (vanilla).
  Manifest/README in that folder; docs/report-mod-3-fresh-1019.md.
- **0.9.0 DLL:** dll/tgcx/Release/tgcx.dll, SHA256
  ee56551e11b5f00537e7d685309f6f7ff848669329a8a405eb1bbc820f2e519b.
  Per-base runtime metadata, keys mapped to live engine indices; up to 256
  catalogue types/indices below 1024. Mismatches refuse operations.
- **Current prepared matrix:** work/army-090-conform-fix, vanilla/HPM0.4.5/TGC 2.3.4/
  TGC 2.5.2/DoDFF. Same DLL, effective base GUI/art/catalogue. Every ZIP entry
  and CRC verified; no scripts. HFM absent. See docs/report-mod-5-1019.md.
- Build/153 signatures/native+GPU/12 installer tests and 32 Python tests
  pass. TGC art (27 files) byte-identical to accepted 0.8.2. **Vanilla Templates has
  runtime acceptance; Conform retry/full game check remains pending**.
- Earlier work/Victoria-II-clean-TGC252 is retained with V2DLL 5.82 and
  accepted Army 0.8.2; also has the additive vanilla 0.9.0 package, now
  superseded as test target. Canonical stable base mod/TGC (P52), profile
  TGC252_Fresh082_20261019. No files removed from this copy.
- E: remains accepted 0.8.2; package DLL SHA256 1ca013f5...a7e43, original
  MOD-2 loader. Stage E:/v2re/logs/tgcx-tpl4-1019/stage-082; backup
  before-082-submod. Accepted 0.8.2 sharing ZIPs remain in work/.

## Next session in order

1. **MOD-3 vanilla user game check:** docs/instructions-mod-3.md. Inspect
   fresh log; editor, recruit, Conform missing/excess, Navy/fleet hiding,
   Ctrl/Alt, save/reload. Only the user launches/exits.
2. **MOD-4 HPM:** docs/instructions-mod-4.md, isolated copy, our path-fixed
   descriptor. Fresh exit confirmation before installation.
3. **MOD-5 DoD/custom units and TGC regression:** docs/instructions-mod-5.md.
   Extra types/icons/recruit/Conform; stable TGC0.9 regression and older
   TGC in its own copy. HFM remains unmeasured until available.
4. Close under ROADMAP section 7; report actual per-base evidence before
   calling compatibility finished. No commit/push/subagents unless asked.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| TPL-3 | **user-confirmed passed**, Ctrl/Alt | report-mod-5-1019.md |
| TPL-4 | **0.8.2 passed in game**, own loader and V2DLL | report-tpl-4-runtime-1019.md; report-v2d-082-passed-1019.md |
| MOD-3 | Templates passed; Conform button corrected, retry next | instructions-mod-3.md; report-mod-3-fresh-1019.md |
| MOD-4 | HPM package generated/verified; runtime pending | instructions-mod-4.md |
| MOD-5 | runtime catalogue implemented/fixture-tested; DoD/TGC game checks pending | instructions-mod-5.md; report-mod-5-1019.md |
| TPL-5 | deferred by user until Army compatibility | instructions-tpl-5.md |
| LAW-4 | deferred while Army has priority | instructions-law-4.md |
| U-12 | Ctrl/Alt and detach passed; Bill portion deferred | instructions-u-12.md |
| V2D-082 | **passed** Army 0.8.2 + V2DLL 5.82 | report-v2d-082-passed-1019.md |
| V2D-2 | waiting on author request D16 (search all -mod folders) | report-v2dll-mod-slot-1018.md |
| MOD-2 / MOD-1 | accepted 0.8.0 /0.7.0 TGC game checks retained | report-mod-2-1018.md; report-mod-1-1018.md |
| LAW-3 / U-11 | passed0.6.4; G1/G3 open, deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0, Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P54. Preserve originals, use paired backups, do not mix loader
filenames, and do not install on a new test run without fresh exit confirmation.


---

# Session 44 - Vanilla Conform opening button (2026-10-19)

User confirms Templates works, Conform does not open. Fresh log confirms two recruits queued. Generated button followed TGC header coordinates instead of vanilla; now follows effective native attrition icon. Tested and installed two generated GUI files after normal-exit confirmation, with backup. Runtime retry pending.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (44). **Army 0.9.0 compatibility candidate built and fixture-tested;
clean vanilla + own-loader test copy ready. Game checks pending.**
Session 43 is preserved in docs/archive/sessions.md.

## Current user decisions

- Finish Army across mod/base versions first (D15), on the reference 3.04
  executable. Politics/LAW-4 and primary+accepted/primary-only filtering
  (TPL-5) wait. Ctrl/Alt-click works: **TPL-3 accepted by the user**.
- Current Conform screenshot is authoritative; old mockup is outdated.
  Read docs/visual-references.md; preserve native army-style layout.
- Files-only distribution: no installer, PS1, BAT or launch scripts for
  recipients. Mod-only and pinned-V2DLL variants are staged.
- Previous clean TGC copy now has user-installed V2DLL. User requested
  another clean copy: use the separate vanilla folder below.

## Current artifacts and evidence

- **New target:** work/Victoria-II-clean-vanilla-Army090. Vanilla copied
  from clean D:, 8,961 base files/851,338,278 bytes verified; our MOD-2 loader
  and Army 0.9.0 installed after normal-exit confirmation/process gate.
  No TGC/V2DLL. Open its victoria2.exe, tick only Army (vanilla).
  Manifest/README in that folder; docs/report-mod-3-fresh-1019.md.
- **0.9.0 DLL:** dll/tgcx/Release/tgcx.dll, SHA256
  ee56551e11b5f00537e7d685309f6f7ff848669329a8a405eb1bbc820f2e519b.
  Per-base runtime metadata, keys mapped to live engine indices; up to 256
  catalogue types/indices below 1024. Mismatches refuse operations.
- **Prepared matrix:** work/army-090-candidate, vanilla/HPM0.4.5/TGC 2.3.4/
  TGC 2.5.2/DoDFF. Same DLL, effective base GUI/art/catalogue. Every ZIP entry
  and CRC verified; no scripts. HFM absent. See docs/report-mod-5-1019.md.
- Build/153 signatures/native+GPU/12 installer tests and 31 Python tests
  pass. TGC art (27 files) byte-identical to accepted 0.8.2. **0.9.0 has no
  integrated runtime acceptance yet**, regardless of generation results.
- Earlier work/Victoria-II-clean-TGC252 is retained with V2DLL 5.82 and
  accepted Army 0.8.2; also has the additive vanilla 0.9.0 package, now
  superseded as test target. Canonical stable base mod/TGC (P52), profile
  TGC252_Fresh082_20261019. No files removed from this copy.
- E: remains accepted 0.8.2; package DLL SHA256 1ca013f5...a7e43, original
  MOD-2 loader. Stage E:/v2re/logs/tgcx-tpl4-1019/stage-082; backup
  before-082-submod. Accepted 0.8.2 sharing ZIPs remain in work/.

## Next session in order

1. **MOD-3 vanilla user game check:** docs/instructions-mod-3.md. Inspect
   fresh log; editor, recruit, Conform missing/excess, Navy/fleet hiding,
   Ctrl/Alt, save/reload. Only the user launches/exits.
2. **MOD-4 HPM:** docs/instructions-mod-4.md, isolated copy, our path-fixed
   descriptor. Fresh exit confirmation before installation.
3. **MOD-5 DoD/custom units and TGC regression:** docs/instructions-mod-5.md.
   Extra types/icons/recruit/Conform; stable TGC0.9 regression and older
   TGC in its own copy. HFM remains unmeasured until available.
4. Close under ROADMAP section 7; report actual per-base evidence before
   calling compatibility finished. No commit/push/subagents unless asked.

## Cards

| Card | Current status | Instructions/evidence |
|---|---|---|
| TPL-3 | **user-confirmed passed**, Ctrl/Alt | report-mod-5-1019.md |
| TPL-4 | **0.8.2 passed in game**, own loader and V2DLL | report-tpl-4-runtime-1019.md; report-v2d-082-passed-1019.md |
| MOD-3 | 0.9.0 clean vanilla copy prepared; **game check next** | instructions-mod-3.md; report-mod-3-fresh-1019.md |
| MOD-4 | HPM package generated/verified; runtime pending | instructions-mod-4.md |
| MOD-5 | runtime catalogue implemented/fixture-tested; DoD/TGC game checks pending | instructions-mod-5.md; report-mod-5-1019.md |
| TPL-5 | deferred by user until Army compatibility | instructions-tpl-5.md |
| LAW-4 | deferred while Army has priority | instructions-law-4.md |
| U-12 | Ctrl/Alt and detach passed; Bill portion deferred | instructions-u-12.md |
| V2D-082 | **passed** Army 0.8.2 + V2DLL 5.82 | report-v2d-082-passed-1019.md |
| V2D-2 | waiting on author request D16 (search all -mod folders) | report-v2dll-mod-slot-1018.md |
| MOD-2 / MOD-1 | accepted 0.8.0 /0.7.0 TGC game checks retained | report-mod-2-1018.md; report-mod-1-1018.md |
| LAW-3 / U-11 | passed0.6.4; G1/G3 open, deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted; extended U-8 complete | existing reports |
| TPL-2 | on hold: Conform covers it | instructions-tpl-2.md |
| GOV-0, Economy | not selected | technical-plan.md |
| COMPAT-0 | Politics compatibility deferred D1 | instructions-compat-0.md |

Next pitfall P53. Preserve originals, use paired backups, do not mix loader
filenames, and do not install on a new test run without fresh exit confirmation.


---

# Session 43 - Army compatibility candidate and clean vanilla copy (2026-10-19)

User confirms Ctrl/Alt, prioritises Army across bases and defers culture filters; supplies current native Conform screenshot. Built and checked 0.9.0 with per-base metadata, five prepared base packages and files-only ZIPs. Prepared a separate clean vanilla + own-loader test copy after the user notes the previous TGC copy has V2DLL. Runtime checks remain pending.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (43). **Army 0.8.2 passed with our loader and V2DLL 5.82** in short
TGC v2.5.2 single-player checks. Read `docs/report-tpl-4-runtime-1019.md` and
`docs/report-v2d-082-passed-1019.md`. Session 42 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## This session: V2D-082 accepted

- User confirmed success after the fresh-copy map-path repair. Logs verify V2DLL plugin API v1, Army 0.8.2, **5/5 extras arrived**, none left behind, and **five regiments queued** with zero left out. No plugin faults/refusals/nonzero failure totals. Evidence: `work/v2d-082-passed-1019/`; `docs/report-v2d-082-passed-1019.md`.
- This covers Army only with V2DLL 5.82. Politics, separate Ctrl/Alt checks, alternate bases and multiplayer remain unconfirmed. V2DLL march/merge completion is not shown in this snapshot.
- Fresh test folder `work/Victoria-II-clean-TGC252/` now has the user's V2DLL/Army installed and canonical base `mod/TGC`; old nested base copy retained. Separate TGC profile `TGC252_Fresh082_20261019`.
- Shareable private test files: `work/Army-Templates-Conform-0.8.2-with-V2DLL.zip` (includes pinned V2DLL de66ee61) or `work/Army-Templates-Conform-0.8.2-mod-only.zip` (existing V2DLL users). No installers/scripts; normal launcher with Army + TGC only. Base TGC not included in ZIPs. Older 0.8.1 artifacts superseded.
- No installation, game launch, commit, push, subagents or external send during this log review.

## Current baseline

- **Installed on E: (2026-10-19): 0.8.2.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `1ca013f5…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **Remaining U-12:** `docs/instructions-u-12.md`, separate Ctrl/Alt editor counts and Bill screenshots after Party/Aye-Nay/Both. TPL-4 detach and V2D-082 have passed; no repeat repair needed.
2. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the screenshot.
3. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`; RE, mockup board 5, then code.
4. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then MOD-4 (HPM) and MOD-5 (runtime unit catalogue), `docs/report-modular-scope-1018.md` section 6.2.
5. **V2D-2** still waits on the author's D16 request (search every -mod=). V2D-082 worked with Army first, so it does not settle that broader loader request.
6. Small open items: G1 (arrow font), G3 (offer reset on new game), P19 recheck, MOD-0 run 3 cause.
7. Close under ROADMAP section 7. Next pitfall: **P53**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | detach **passed on 0.8.2**; separate Ctrl/Alt and LAW-4 checks unconfirmed | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | **passed in game on 0.8.2**, 2026-10-19; three detaches, no leftovers | report-tpl-4-runtime-1019.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-082 | **passed** 2026-10-19: V2DLL 5.82 loads Army 0.8.2; 5/5 extras moved, five recruited | report-v2d-082-passed-1019.md |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


## Superseded MOD-3 instructions

# MOD-3: the Army package on vanilla 3.04 in game

2026-10-18. D15 S1: Army works on any base. MOD-2 built the installer for any
base (`tools/tgcx_packages.py`, profile `vanilla`) and tested it only on TGC
v2.5.2. This card installs and checks it on vanilla.

## Steps

1. Dry run, then install after a normal exit, with a backup:

   ```
   python tools/install_submod.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod --base vanilla --packages army --source E:\v2re\logs\tgcx-mod2-1018\stage-080\source --dll <current tgcx.dll> --backup E:\v2re\logs\tgcx-mod3-MMDD\before-mod3 --dry-run
   ```

   The sprite and font check must pass: every sprite the Army files use must
   exist in vanilla. If it fails, the report lists the missing ones; replace
   them in the generated files with vanilla sprites (never copy TGC art, R8).
2. The `.mod` is `tgcx_army_vanilla.mod`, no `dependencies`, no `user_dir`.
3. Game check (the user): tick only **Army Templates and Conform
   (vanilla)**. The settings folder is the game root. Check:
   - `tgcx.log` says `Packages: army=… politics=none settings=`;
   - the Templates tab in Build Army, a template recruits;
   - Conform on a stack builds the missing regiments;
   - save, exit, reload: the templates are still there.
4. Vanilla unit types are fewer than TGC's: check the editor lists only units
   that exist. If not, note it: MOD-5 is the run-time catalogue.
5. Report `docs/report-mod-3-MMDD.md`. Then MOD-4 (HPM with
   `--fix-path mod/HPM`, S6).


---

# Session 42 - V2D-082 acceptance (2026-10-19)

User confirms success; logs verify Army 0.8.2 with V2DLL 5.82, 5/5 extras moved and five regiments queued.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (42). **0.8.2 installed on E:, TPL-4 passed in game (Army + our loader).** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-runtime-1019.md` and `docs/report-v2d-fresh-1019.md`. Session 41 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Current fresh-copy repair

- First V2DLL + Army launch initialized successfully (fresh logs show V2DLL 5.82, 0.8.2 plugin API v1, Army/Conform enabled), but game load failed on missing `mod/TGC/map/definition.csv`.
- Our copied TGC retained a hardcoded canonical map path. Repaired only the fresh layout after confirmed game exit: canonical `mod/TGC`, descriptor now points there, all 16,411 TGC files unchanged, old nested copy retained. Working E:/D: installs untouched. `docs/report-fresh-map-path-1019.md`; evidence/backup `work/fresh-TGC-path-fix/`.
- **Next: retry V2D-082 with the normal launcher.** No new ZIP/binary required; gameplay remains untested. The fresh folder now has the user's installed V2DLL/Army, so it is no longer the original pre-install baseline.

## This session: detach accepted, fresh V2DLL test prepared

- User says everything seems in order. Current 0.8.2 log confirms **3/3, 1/1 and 6/6 extras arrived**, none left behind; six recruited, four marches, three joins. No faults/refusals/nonzero failure totals. **TPL-4 passed**. Evidence: `docs/report-tpl-4-runtime-1019.md`, snapshot `work/tpl4-082-game-check-1019/`.
- This run used **our MOD-2 loader**, Army only. V2DLL 0.8.2, Politics/LAW-4 and separate Ctrl/Alt checks remain unconfirmed.
- User requested a fresh copy and chose **clean game + TGC v2.5.2**. Ready at `work/Victoria-II-clean-TGC252/`, original Lua DLL, no V2DLL/Army installed, no old settings/logs/saves. Separate TGC profile `TGC252_Fresh082_20261019`.
- Current handoff ZIP: `work/Army-Templates-Conform-0.8.2-with-V2DLL.zip`, Army + unchanged V2DLL_LATEST_INSTALL from pinned `de66ee61`; plugin loading enabled. No installer/scripts. User explicitly requested bundling V2DLL. `docs/report-v2d-bundle-1019.md`. The mod-only 0.8.2 ZIP is retained for users who already have V2DLL; older 0.8.1 ZIPs superseded.
- **Next: V2D-082**, user extracts the combined ZIP into the fresh folder and uses the normal launcher (`docs/instructions-v2d-082.md`). Report `docs/report-v2d-fresh-1019.md`. Nothing launched or sent externally.

## Current baseline

- **Installed on E: (2026-10-19): 0.8.2.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `1ca013f5…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **V2D-082, retry after map-path repair:** `docs/instructions-v2d-082.md`. V2DLL and Army are already installed in `work/Victoria-II-clean-TGC252`; user retries the normal launcher, selects Army + TGC in normal launcher, confirms fresh loaded_by marker and tests recruitment/Conform.
2. **Remaining U-12:** `docs/instructions-u-12.md`, Ctrl/Alt editor counts and Bill screenshot after clicking Party. Detach itself has passed; no repeat repair needed.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`; RE, mockup board 5, then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then MOD-4 (HPM) and MOD-5 (runtime unit catalogue), `docs/report-modular-scope-1018.md` section 6.2.
6. **V2D-2** remains waiting on the author's D16 request (search every -mod=). It is distinct from the new V2D-082 runtime check.
7. Small open items: G1 (arrow font), G3 (offer reset on new game), P19 recheck, MOD-0 run 3 cause.
8. Close under ROADMAP section 7. Next pitfall: **P53**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | detach **passed on 0.8.2**; separate Ctrl/Alt and LAW-4 checks unconfirmed | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | **passed in game on 0.8.2**, 2026-10-19; three detaches, no leftovers | report-tpl-4-runtime-1019.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-082 | startup/plugin init confirmed; fresh base map-path repaired; gameplay retry next | instructions-v2d-082.md; report-fresh-map-path-1019.md |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 41 - fresh TGC hardcoded map-path repair (2026-10-19)

V2DLL + Army initialization confirmed; first game load failed on missing canonical TGC definition.csv. Corrected fresh-copy layout after user closed game.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (41). **0.8.2 installed on E:, TPL-4 passed in game (Army + our loader).** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-runtime-1019.md` and `docs/report-v2d-fresh-1019.md`. Session 40 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## This session: detach accepted, fresh V2DLL test prepared

- User says everything seems in order. Current 0.8.2 log confirms **3/3, 1/1 and 6/6 extras arrived**, none left behind; six recruited, four marches, three joins. No faults/refusals/nonzero failure totals. **TPL-4 passed**. Evidence: `docs/report-tpl-4-runtime-1019.md`, snapshot `work/tpl4-082-game-check-1019/`.
- This run used **our MOD-2 loader**, Army only. V2DLL 0.8.2, Politics/LAW-4 and separate Ctrl/Alt checks remain unconfirmed.
- User requested a fresh copy and chose **clean game + TGC v2.5.2**. Ready at `work/Victoria-II-clean-TGC252/`, original Lua DLL, no V2DLL/Army installed, no old settings/logs/saves. Separate TGC profile `TGC252_Fresh082_20261019`.
- Current handoff ZIP: `work/Army-Templates-Conform-0.8.2-with-V2DLL.zip`, Army + unchanged V2DLL_LATEST_INSTALL from pinned `de66ee61`; plugin loading enabled. No installer/scripts. User explicitly requested bundling V2DLL. `docs/report-v2d-bundle-1019.md`. The mod-only 0.8.2 ZIP is retained for users who already have V2DLL; older 0.8.1 ZIPs superseded.
- **Next: V2D-082**, user extracts the combined ZIP into the fresh folder and uses the normal launcher (`docs/instructions-v2d-082.md`). Report `docs/report-v2d-fresh-1019.md`. Nothing launched or sent externally.

## Current baseline

- **Installed on E: (2026-10-19): 0.8.2.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `1ca013f5…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **V2D-082, fresh-user V2DLL + Army 0.8.2 check:** `docs/instructions-v2d-082.md`. User extracts the combined V2DLL+Army ZIP into `work/Victoria-II-clean-TGC252`, selects Army + TGC in normal launcher, confirms fresh loaded_by marker and tests recruitment/Conform.
2. **Remaining U-12:** `docs/instructions-u-12.md`, Ctrl/Alt editor counts and Bill screenshot after clicking Party. Detach itself has passed; no repeat repair needed.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`; RE, mockup board 5, then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then MOD-4 (HPM) and MOD-5 (runtime unit catalogue), `docs/report-modular-scope-1018.md` section 6.2.
6. **V2D-2** remains waiting on the author's D16 request (search every -mod=). It is distinct from the new V2D-082 runtime check.
7. Small open items: G1 (arrow font), G3 (offer reset on new game), P19 recheck, MOD-0 run 3 cause.
8. Close under ROADMAP section 7. Next pitfall: **P52**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | detach **passed on 0.8.2**; separate Ctrl/Alt and LAW-4 checks unconfirmed | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | **passed in game on 0.8.2**, 2026-10-19; three detaches, no leftovers | report-tpl-4-runtime-1019.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-082 | **next**: user installs/tests V2DLL + Army 0.8.2 in prepared clean folder | instructions-v2d-082.md; report-v2d-fresh-1019.md |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 40 - combined V2DLL ZIP (2026-10-19)

User requested the V2DLL distribution included with Army 0.8.2. Prepared combined ZIP; no game copy changed or launched.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (40). **0.8.2 installed on E:, TPL-4 passed in game (Army + our loader).** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-runtime-1019.md` and `docs/report-v2d-fresh-1019.md`. Session 39 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## This session: detach accepted, fresh V2DLL test prepared

- User says everything seems in order. Current 0.8.2 log confirms **3/3, 1/1 and 6/6 extras arrived**, none left behind; six recruited, four marches, three joins. No faults/refusals/nonzero failure totals. **TPL-4 passed**. Evidence: `docs/report-tpl-4-runtime-1019.md`, snapshot `work/tpl4-082-game-check-1019/`.
- This run used **our MOD-2 loader**, Army only. V2DLL 0.8.2, Politics/LAW-4 and separate Ctrl/Alt checks remain unconfirmed.
- User requested a fresh copy and chose **clean game + TGC v2.5.2**. Ready at `work/Victoria-II-clean-TGC252/`, original Lua DLL, no V2DLL/Army installed, no old settings/logs/saves. Separate TGC profile `TGC252_Fresh082_20261019`.
- Current handoff ZIP: `work/Army-Templates-Conform-0.8.2-mod-only.zip`, only mod/ + README; no installer/scripts. Older 0.8.1 ZIPs superseded.
- **Next: V2D-082**, user installs V2DLL then Army into the fresh folder and uses the normal launcher (`docs/instructions-v2d-082.md`). Report `docs/report-v2d-fresh-1019.md`. Nothing launched or sent externally.

## Current baseline

- **Installed on E: (2026-10-19): 0.8.2.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `1ca013f5…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **V2D-082, fresh-user V2DLL + Army 0.8.2 check:** `docs/instructions-v2d-082.md`. User installs into `work/Victoria-II-clean-TGC252`, selects Army + TGC in normal launcher, confirms fresh loaded_by marker and tests recruitment/Conform.
2. **Remaining U-12:** `docs/instructions-u-12.md`, Ctrl/Alt editor counts and Bill screenshot after clicking Party. Detach itself has passed; no repeat repair needed.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`; RE, mockup board 5, then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then MOD-4 (HPM) and MOD-5 (runtime unit catalogue), `docs/report-modular-scope-1018.md` section 6.2.
6. **V2D-2** remains waiting on the author's D16 request (search every -mod=). It is distinct from the new V2D-082 runtime check.
7. Small open items: G1 (arrow font), G3 (offer reset on new game), P19 recheck, MOD-0 run 3 cause.
8. Close under ROADMAP section 7. Next pitfall: **P52**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | detach **passed on 0.8.2**; separate Ctrl/Alt and LAW-4 checks unconfirmed | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | **passed in game on 0.8.2**, 2026-10-19; three detaches, no leftovers | report-tpl-4-runtime-1019.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-082 | **next**: user installs/tests V2DLL + Army 0.8.2 in prepared clean folder | instructions-v2d-082.md; report-v2d-fresh-1019.md |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 39 - accepted detach and fresh V2DLL test copy (2026-10-19)

Read logs: 0.8.2 detach passed all three cases. Prepared clean game + TGC and files-only 0.8.2 plugin for a new-user V2DLL test. Nothing launched.

## NEXT.md at session start

# NEXT - what comes next

2026-10-19 (39). **0.8.2 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1019.md`. Session 38 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## TPL-4 follow-up this session

- User confirmed detach still fails on 0.8.1. The logged location/movement fields pass; empty destination stacks disappear before next-day verification.
- 0.8.2 queues Create and all Transfers together, checks vanished destinations and logs first-extra type flags. This timing repair is fixture-tested, not yet confirmed in game.
- Installed after explicit normal-exit confirmation. Stage `E:/v2re/logs/tgcx-tpl4-1019/stage-082/tgcx.dll`; full rollback `E:/v2re/logs/tgcx-tpl4-1019/before-082-submod`. DLL `1ca013f5...`; loader stays 0.8.0. All non-DLL package files unchanged.
- **Next: `docs/instructions-tpl-4-082-check.md`.** Report `docs/report-tpl-4-1019.md`. No commit, push or subagents.

## Earlier private test handoff (0.8.1, known detach failure)


- User requested **no installer and no launch scripts**.
- Current artifact: `work/Army-Templates-Conform-0.8.1-mod-only.zip`: only `mod/` and README. Extract into the game folder; V2DLL plugin loading enabled; select Army + TGC v2.5.2 in the normal launcher. Nothing else selected for this test.
- Correction: prior MOD-2 evidence confirms the normal launcher puts Army first for this pair; scripts and base descriptor filename edits are unnecessary.
- Report: `docs/report-army-launcher-1018.md`. CRC/plugin identity checked; no scripts in ZIP. Earlier handoff ZIPs retained but superseded.
- The ZIP still contains 0.8.1 and its known detach failure. Wait for the 0.8.2 game check before replacing it. No external send by the agent.

## Current baseline

- **Installed on E: (2026-10-19): 0.8.2.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `1ca013f5…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **TPL-4, game check of 0.8.2:** `docs/instructions-tpl-4-082-check.md`. Then the remaining U-12 checks: `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if 0.8.2 still fails: inspect the batch, first-extra flags and disappeared/arrived result lines; do not assume a particular cause.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | detach **failed on 0.8.1**; other parts unconfirmed. Next: 0.8.2 detach recheck, then TPL-3/LAW-4 | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | 0.8.1 failed in game; 0.8.2 installed on E: 2026-10-19 (Create + Transfer batch), not tested in game | instructions-tpl-4-082-check.md; report-tpl-4-1019.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 38 - TPL-4 follow-up 0.8.2 (2026-10-19)

User confirmed 0.8.1 detach still fails. Built/tested/installed immediate command batch after normal exit confirmation.

## NEXT.md at session start

# NEXT - what comes next

2026-10-18 (38). **0.8.1 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1018.md`. Session 37 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Private test handoff this session

- User requested **no installer and no launch scripts**.
- Current artifact: `work/Army-Templates-Conform-0.8.1-mod-only.zip`: only `mod/` and README. Extract into the game folder; V2DLL plugin loading enabled; select Army + TGC v2.5.2 in the normal launcher. Nothing else selected for this test.
- Correction: prior MOD-2 evidence confirms the normal launcher puts Army first for this pair; scripts and base descriptor filename edits are unnecessary.
- Report: `docs/report-army-launcher-1018.md`. CRC/plugin identity checked; no scripts in ZIP. Earlier handoff ZIPs retained but superseded.
- Nothing installed, launched or sent externally. U-12 and 0.8.1 + V2DLL game testing remain pending. Existing next-card instructions remain current.

## Current baseline

- **Installed on E: (2026-10-18): 0.8.1.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `62fa7933…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **U-12, game check of 0.8.1:** `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if U-12 shows extras still refused: the
   `transfer check` line names the field; fix that field only.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | **next**: game check of 0.8.1 (TPL-3, TPL-4, LAW-4 screenshot) | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | installed 0.8.1 on E: 2026-10-18 (detach verified next day, retried once, refusal fields logged); not tested in game | report-tpl-4-1018.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 37 - normal-launcher handoff (2026-10-18)

Removed launch scripts from handoff; confirmed prior normal-launcher order.

## NEXT.md at session start

# NEXT - what comes next

2026-10-18 (37). **0.8.1 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1018.md`. Session 36 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Private test handoff this session

- User requested **files only, no installer**, with drop-location instructions.
- Current artifact: `work/Army-Templates-Conform-0.8.1-drop-in.zip`. Contains the generated Army folder and .mod, one 0.8.1 `v2dll_mod.dll`, a manual launcher pair and placement README. Extract beside `v2game.exe`; V2DLL and TGC v2.5.2 required.
- Supersedes the portable setup and mixed-package ZIPs for this handoff; old artifacts retained locally.
- Report: `docs/report-army-drop-in-1018.md`. CRC and plugin identity verified. Nothing installed or sent externally; no game launched.
- 0.8.1 + V2DLL remains untested in game. U-12 and the follow-up order below remain current; all listed instruction files already exist.

## Current baseline

- **Installed on E: (2026-10-18): 0.8.1.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `62fa7933…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **U-12, game check of 0.8.1:** `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if U-12 shows extras still refused: the
   `transfer check` line names the field; fix that field only.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | **next**: game check of 0.8.1 (TPL-3, TPL-4, LAW-4 screenshot) | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | installed 0.8.1 on E: 2026-10-18 (detach verified next day, retried once, refusal fields logged); not tested in game | report-tpl-4-1018.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 36 - drop-in Army-only handoff (2026-10-18)

User requested files without an installer; prepared the drop-in ZIP. No installation or launch.

## NEXT.md at session start

# NEXT - what comes next

2026-10-18 (36). **0.8.1 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1018.md`. Session 35 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Private test handoff this session

- User requested automatic setup and **only 0.8.1 Army Templates and Conform**.
- Current artifact: `work/Army-Templates-Conform-0.8.1-portable.zip`. Extract and run `Setup.cmd`; choose the game/base; setup generates and installs Army, retains backups, and creates a manual launcher. Restore is available in Setup. Python/Pillow included; V2DLL remains a prerequisite.
- Supersedes `work/TGCX-maxioten-private-test-2026-10-18.zip` (the old mixed-package bundle, retained locally).
- Report: `docs/report-army-portable-1018.md`. Five transaction tests, portable generation, content comparison, hidden Tk smoke and archive checks passed. Nothing installed on E: or sent externally; no game launched.
- 0.8.1 + V2DLL remains untested in game. U-12 and the follow-up order below remain current; all listed instruction files already exist.

## Current baseline

- **Installed on E: (2026-10-18): 0.8.1.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `62fa7933…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **U-12, game check of 0.8.1:** `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if U-12 shows extras still refused: the
   `transfer check` line names the field; fix that field only.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | **next**: game check of 0.8.1 (TPL-3, TPL-4, LAW-4 screenshot) | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | installed 0.8.1 on E: 2026-10-18 (detach verified next day, retried once, refusal fields logged); not tested in game | report-tpl-4-1018.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 35 - portable Army-only handoff (2026-10-18)

Prepared automated setup and the 0.8.1 Army-only portable ZIP. No live installation or launch.

## NEXT.md at session start

# NEXT - what comes next

2026-10-18 (35). **0.8.1 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1018.md`. Session 34 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Private test handoff this session

- Prepared `work/TGCX-maxioten-private-test-2026-10-18.zip` for the user to send to maxioten; nothing sent externally. Includes staged 0.8.0 and 0.8.1 as `v2dll_mod.dll`, generators and README. No bundled base files/art or loader.
- `tools/prepare_maxioten_test.py` regenerates both packages from the recipient's installed TGC; local preparation verified. See `docs/report-maxioten-test-1018.md`. V2DLL testing of 0.8.x remains pending.
- Original U-12 and follow-up order below remain current; all listed instruction files already exist.

## Current baseline

- **Installed on E: (2026-10-18): 0.8.1.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `62fa7933…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **U-12, game check of 0.8.1:** `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if U-12 shows extras still refused: the
   `transfer check` line names the field; fix that field only.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | **next**: game check of 0.8.1 (TPL-3, TPL-4, LAW-4 screenshot) | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | installed 0.8.1 on E: 2026-10-18 (detach verified next day, retried once, refusal fields logged); not tested in game | report-tpl-4-1018.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 34 - private testing handoff (2026-10-18)

Prepared the maxioten test bundle; no installation, game launch, commit, push or external message.

## NEXT.md at session start

# NEXT - what comes next

2026-10-18 (34). **0.8.1 installed on E:, not tested in game.** MOD-2 (0.8.0)
passed the game check. Read `docs/report-mod-2-1018.md` and
`docs/report-tpl-4-1018.md`. Session 33 is in `docs/archive/sessions.md`.

## User decisions last session

- **D15 S1-S8** (`AGENTS.md` §9): Army works on any base, as generic as
  possible (vanilla, TGC, HPM, HFM, others); Politics stays TGC v2.5.2 only.
  Packages "Army Templates and Conform" (no `user_dir`) and "TGCX Politics"
  (`user_dir` TGCX); Economy and Cosmetic later. One folder per base,
  `tgcx_<pkg>_<slug>`.
- **TPL-5 shape:** two buttons, "primary + accepted" and "primary only
  (strict)", for both Templates and Conform.
- **LAW-4 minimum:** the three parliament view buttons (Aye/Nay, Party,
  Both) must change the view and show which one is selected.
- The order below is the coordinator's recommendation; the user can reorder.

## Current baseline

- **Installed on E: (2026-10-18): 0.8.1.** Packages `mod\tgcx_army_tgc252`
  and `mod\tgcx_politics_tgc252` with their `.mod` files, `tgcx.dll`
  `62fa7933…` in each; loader `lua51.dll` `3aab0c21…` (0.8.0). The old
  game-folder `tgcx.dll` is set aside as `tgcx.dll.off.1`.
- Stages: `E:\v2re\logs\tgcx-mod2-1018\stage-080\` (loader, 0.8.0 `tgcx.dll`,
  `source\` = the submod source); `E:\v2re\logs\tgcx-tpl4-1018\stage-081\tgcx.dll`.
- Backups: `E:\v2re\logs\tgcx-tpl4-1018\before-081-submod` (0.8.0 packages);
  `E:\v2re\logs\tgcx-mod2-1018\before-080-dll` and `before-080-submod`
  (0.7.0 and the old `mod\TGCX`).
- **Uncommitted** since 938681b: all MOD-2, 0.8.1 and P50 work. Commit only
  when the user asks.

## Next session in order

1. **U-12, game check of 0.8.1:** `docs/instructions-u-12.md`. Ctrl/Alt-click
   in the editor (TPL-3), Conform detach with its log lines (TPL-4), and a
   screenshot of the Bill after clicking Party (LAW-4).
2. **TPL-4 follow-up** if U-12 shows extras still refused: the
   `transfer check` line names the field; fix that field only.
3. **LAW-4, Bill view buttons:** `docs/instructions-law-4.md`, from the U-12
   screenshot.
4. **TPL-5, culture filter:** `docs/instructions-tpl-5.md`. RE first, mockup
   board 5 next (R10), then code.
5. **MOD-3, Army on vanilla in game:** `docs/instructions-mod-3.md`. Then
   MOD-4 (HPM, our own descriptor) and MOD-5 (run-time unit catalogue),
   `docs/report-modular-scope-1018.md` §6.2.
6. **V2D-2** only when V2DLL answers the D16 request (search every `-mod=`).
7. Small open items: G1 (» arrow in the small font), G3 (reset the offer on a
   new game), P19 recheck, the MOD-0 run 3 cause.
8. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| U-12 | **next**: game check of 0.8.1 (TPL-3, TPL-4, LAW-4 screenshot) | instructions-u-12.md |
| TPL-3 | installed 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-4 | installed 0.8.1 on E: 2026-10-18 (detach verified next day, retried once, refusal fields logged); not tested in game | report-tpl-4-1018.md |
| LAW-4 | open: Bill view buttons do not stay ticked; needs the U-12 screenshot | instructions-law-4.md; report-tpl-4-1018.md §3 |
| TPL-5 | open: culture filter, two buttons; RE + mockup board 5 | instructions-tpl-5.md |
| MOD-3 | open: Army package on vanilla 3.04 in game | instructions-mod-3.md |
| MOD-4 / MOD-5 | open: HPM; run-time unit catalogue | report-modular-scope-1018.md §6.2 |
| V2D-2 | waiting on V2DLL (D16 request) | report-v2dll-mod-slot-1018.md §7 |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0): TGC + both, TGC + Army (templates carried over), TGC alone | instructions-mod-2.md; report-mod-2-1018.md |
| MOD-1 | 0.7.0 **passed the game check** 2026-10-18 | instructions-mod-1.md; report-mod-1-1018.md |
| V2D-1 | **passed** 2026-10-18 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-0 | **done** 2026-10-18 (P49, P50); open: Q3 units, P19 recheck, run 3 cause | instructions-mod-0.md; report-mod-0-1018.md |
| LAW-3 / U-11 | 0.6.4 **passed U-11** 2026-10-18; G1, G3 open | instructions-law-3.md; report-u-11-1018.md |
| LAW-2 / U-7 | partly passed; failures went to LAW-3 | instructions-law-2.md; report-u-7-1018.md |
| U-8 | **complete** | report-u-8-extended-1018.md |
| TPL-0 / TPL-1 | accepted in game | report-tpl-0-visual-1017.md; report-tpl-1-visual-1017.md |
| TPL-2 | on hold (user): Conform covers it | instructions-tpl-2.md |
| LAW-1 / U-6, HEMI-3 / U-5 | accepted; extended coverage unconfirmed | existing instructions/reports |
| GOV-0 | available, not selected | instructions-gov-0.md |
| COMPAT-0 | deferred D1 | instructions-compat-0.md |


# Session 2026-10-18 (33): MOD-0 runs 2-3, MOD-1, MOD-2, TPL-3 and TPL-4

- **MOD-0 done:** runs 2 and 3 recorded (P49: the launcher does not enforce
  `dependencies`; TGCX alone crashed on vanilla). `docs/report-mod-0-1018.md`.
- **V2D-1 passed:** V2DLL loaded `tgcx.dll` 0.7.0 as `v2dll_mod.dll` (`-> ok`);
  E: restored to 0.7.0. P48 recorded (V2DLL writes `common\defines_v2dll.txt`).
- **MOD-1 (0.7.0):** per-module signature gates, `tgcx.dll` behind a
  `lua51.dll` loader, `V2Mod_Init` export. Installed, game check passed.
- **D15 sub-decisions S1-S8 (user):** Army works on any base, as generic as
  possible; Politics stays TGC v2.5.2 only. Two packages now, "Army Templates
  and Conform" (no `user_dir`) and "TGCX Politics" (`user_dir` TGCX); Economy
  and Cosmetic later. One folder per base `tgcx_<pkg>_<slug>`, our own HPM
  descriptor, `tgcx.dll` in each package folder, the old `mod\TGCX` replaced.
- **MOD-2 (0.8.0):** `tools/tgcx_bases.py`, `tools/tgcx_packages.py`, the
  loader finds `tgcx.dll` through the `-mod=` list, `tgcx.dll` gates modules by
  ticked package, templates file per settings folder with carry-over.
  Installed and **tested in game**: TGC + both, TGC + Army, TGC alone all as
  expected. P50: the launcher sorts mods by display name.
  `docs/report-mod-2-1018.md`.
- **0.8.1 installed, not tested in game:** TPL-3 (Ctrl-click 5, Alt-click 2)
  and TPL-4 (Conform's detach never worked: Transfers refused silently; now
  verified next day, retried once, refusal fields logged). LAW-4 (Bill view
  buttons do not stay ticked) not fixed: a screenshot is needed.
  `docs/report-tpl-4-1018.md`.
- TPL-5 requested (culture filter: primary + accepted, primary strict).
- maxioten (V2DLL) exchange: `LoadV2Mods` reads only the first `-mod=`; the
  D16 request (search every `-mod=` or read a settings path) stands.
- Nothing committed since 938681b. Next pitfall P51.

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
5. Close under ROADMAP §7. The next pitfall is **P51**.

Only the user launches and exits the game. Install only after a normal
exit, with paired backups. No commit, push or subagents unless asked.

## Cards

| Card | Status | Instructions/evidence |
|---|---|---|
| V2D-1 | **passed** 2026-10-18: V2DLL `de66ee61` loaded tgcx.dll 0.7.0 (`-> ok`), 7 FOREIGN, all modules on; E: restored to 0.7.0 | instructions-v2d-1.md; report-v2d-1-1018.md |
| MOD-2 | **tested in game** 2026-10-18 (0.8.0 on E:): TGC + both packages, TGC + Army (templates carried over), TGC alone all as expected | instructions-mod-2.md; report-mod-2-1018.md |
| LAW-4 | bug: the Bill view buttons (Aye/Nay, Party, Both) do not stay ticked; code unchanged since 0.4.4. Needs a screenshot after clicking Party | report-tpl-4-1018.md §3 |
| TPL-3 | **installed** 0.8.1 on E: 2026-10-18 (Ctrl-click 5, Alt-click 2); not tested in game | report-tpl-4-1018.md |
| TPL-5 | request: culture filter for Templates and Conform, two buttons: primary + accepted, and primary only (strict). Needs RE of how the recruit command picks a soldier pop; mockup board 5 changes (R10) | — |
| TPL-4 | **installed** 0.8.1: Conform's detach never worked (Transfers refused silently); now verified the next day, retried once, refusal fields logged; **installed** on E: 2026-10-18, not tested in game | report-tpl-4-1018.md |
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

# Session 2026-10-18 (32): U-7 and U-8 game checks

Two side sessions guided the user through the game checks on the installed
0.6.3; nothing was built or installed.

- **U-8 extended: all five open cases passed** (one-regiment template,
  paused repeat-click block, partial recruit 22 queued / 81 left out,
  Military close/reopen, fresh-process reload `loaded=4`). U-8 is complete.
  `docs/report-u-8-extended-1018.md`.
- **U-7: partly passed.** Bill buttons, the bill from the Reforms tab,
  purchases, the Decisions tab, the monthly favour tick and the popup filter
  work. Failed: F1 the Reforms tab keeps every row lit with a bill open;
  F2 the top-bar decisions tooltip lists `tgcx_` decisions. Also F3 the
  SUB-0 load test is still installed, and long law names overflow the Bill
  text boxes. `docs/report-u-7-1018.md`.
- **User decision D14:** the Reforms tab offers only bills that pass their
  own decision's `allow`, and backward bills only where the game would allow
  a backward reform.
- P46 recorded (a native row builder keeps element state). Next pitfall P47.

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

# Session 2026-10-17 (30): 0.6.3 army-style candidate

Built and staged; automatic gates pass. Installation and player acceptance pending.

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


---

# Session 2026-10-17 (29) - TPL-1 Conform window, accepted in game (0.6.2)

The Conform button on the stack panel and the Conform window (board 6) were
built, installed and checked in game three times:

- 0.6.0: the button did nothing. GUI lookups search only direct children
  (P44); fixed with a top-level marker in 0.6.1.
- 0.6.1: the window opened and one Conform marched and merged, then the game
  crashed. The Merge allocation was 0x48 and the native size is 0x4C (P45).
  Hover also lit up the whole window.
- 0.6.2: Merge size fixed, background icon over the hover, and Conform only
  builds (user revised D13). **Accepted in game** ("it worked"): builds
  march and merge, extras detach, no crash, error logs empty.

Committed and pushed as `421bfc9`. Open: "Conform window disabled after
guarded GUI read fault" twice in the log. Next (user): make the Conform GUI
look more like Victoria II's own. See `report-tpl-1-window-1017.md`.

## NEXT as it stood at close (verbatim; it was updated during the session)

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


## Session 71 NEXT archived on 2026-10-22 before PR-2 integration close

# NEXT - what comes next

2026-10-21 (71). **TPL-8 design approved; implementation held at user's
wait instruction for fedacking's Rust work.** Province Rename0.2.6 ZIP
remains ready/installed and unpublished. Session70 starting NEXT archived
verbatim in docs/archive/sessions.md.

## ATCU current instruction: hold implementation for Rust coordination

- User selected easier features first and approved Non-core only as a third
  mutually exclusive checkbox in Templates AND Conform. None selected=All;
  clicking the selected box clears. Narrow Conform can use two lines.
- User then said wait because fedacking is porting to Rust. No ATCU source,
  generated header, art, package, build, game or settings change was made.
  work/noncore-0113 is only a scratch folder/executable gate, not a build.
- Verified GitHub PR#1 merged: Rust lua51 loader plus C++ build-compatibility
  fixes, including Templates click-body/SEH split. Merge1727804b; current
  local HEAD045bbfd5 is its base and has not incorporated it. Preserve all
  local uncommitted Province Rename work before any upstream integration.
- The non-core feature does not require a loader-interface change. A wider
  in-progress recruitment/UI port could still need the feature ported too;
  unpublished scope is unknown. Do not infer it from the merged loader PR.
- Before resuming implementation, clarify port ownership/scope and reconcile
  merged changes safely. No messages to fedacking, fetch/merge/rebase/push,
  source changes or installation were performed. No autonomous continuation
  of feature coding while this user hold applies.
- Report docs/report-tpl-8-rust-coordination-1021.md; current instructions
  docs/instructions-tpl-8.md. The older TPL-8 open-choice wording in the
  retained future queue below predates the approval above and is historical.


## Current scope and evidence

- User says0.2.5 works awesome. Screenshot/log confirms short dialog,
  typing/caret and repeated native renames with map/header updates.
- 0.2.6 raises24px ABC2px and changes name gap8px to10px. Native font/name
  anchor and clamping retained; accepted editor/caret/commands unchanged.
  Final2px visual check itself remains pending. No new addresses/pitfall.
- Initial request to release was superseded explicitly: dont release it
  yet, just give me the zip. No commit/push/tag/remote creation/publication.
  Public-release README/notes are prepared prospective material only.
- Package is Universal additive V2DLL, reference3.04/current V2DLL required.
  No dependencies/user_dir/replace_path/base GUI/data override. Native
  command0x707 only changes simulation. Other-base gameplay, ATCU
  coexistence, extended lifetime/save-reload and multiplayer remain untested.
- Current report docs/report-province-rename-package-1021.md. Current board
  docs/province-rename-layout.svg; previous025 board archived verbatim in
  docs/archive/province-rename-layout-025.svg. Native art stays ignored.

## Local deliverable and installed copy

- ZIP work/releases/province-rename/0.2.6/Province-Rename-Universal-0.2.6.zip:
  12files,264502bytes. SHA256
  fcc68a16c0987c5f947d8ada1fde7060c85a2e2b3f51da4d703f8ed05055ce9f.
  Adjacent .zip.sha256, SHA256.json, PROVENANCE.json and RELEASE.json.
  published=false; no destination or source commit selected. ZIP's11mod
  payload members byte-identical to checked0.2.6 candidate; README only differs.
- work/Victoria-II-Province-Rename-TGC252 updated after fresh user Yes,
  closed normally confirmation and process gate.11payload/25,388protected
  hashes pass; protected unchanged. Full0.2.5/guide rollback before-026 in
  work/province-rename/release-026-1021/, exact patch-026-installed.json.
  Earlier backups retained. No game launch/termination/main/other-copy change.
- Stock Lua, canonical TGC, V2DLL5.98/cdad55c7 and independent profile kept.
- 301guards/78Rename runtime, Win32 build,66native/4package/4actual-loader
  checks pass. Five-base checks are static, not gameplay. Native-art pixels
  and binaries remain in ignored work/. No memory update. Next pitfall P64.

## Next session in order

0. **TPL-8 held**: follow docs/instructions-tpl-8.md. Await clarification/
   resumption concerning Rust scope; preserve approved checkbox design.
   Inspect/reconcile upstream before implementation when work resumes.

1. Deliver local ZIP (and optional .sha256). No publication or new install.
   Current instructions docs/instructions-rename-026.md.
2. If user wants final appearance check, they launch dedicated copy with
   Province Rename (Universal)+TGC v2.5.2 (stable), inspect2px higher ABC/
   10px name gap and recheck opening/typing/rename. Expect0.2.6 Universal,
   78native signatures in Logs/tgcx_province_rename.log. Preserve fresh logs/
   unedited captures after normal exit. Final positioning unconfirmed yet.
3. Full limits/ownership/lifetime/save-reload per instructions-rename-020.md
   sections2-3; other-base gameplay/ATCU/MP only when selected later.
4. Publishing remains expressly deferred. If user asks again, confirm exact
   destination then finish source/release identity before publishing. Do not
   treat the old release request as standing authorization. Installation
   still requires fresh user exit/install instruction and process gate.
   Never launch/terminate the game autonomously; keep all rollbacks.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| RENAME-0/U/025 | short rename/editor/typing/caret accepted; full cases pending | instructions-rename-020.md |
| RENAME-026 | final ABC tweak installed, ZIP ready; publication deferred by user | instructions-rename-026.md |
| RENAME-MP | theoretical only; no host/client/OOS test | instructions-rename-020.md section4 |
| TPL-8 | design approved, implementation held for Rust coordination; no new source/build | instructions-tpl-8.md |
| TPL-5/6 |0.11.2 retained; extended checks unchanged | instructions-tpl-5.md; instructions-tpl-6.md |
| TPL-7 | short reload passed; pre-Conform-save case unconfirmed | report-tpl-7-orders-1019.md |
| RELEASE-0 | ATCU0.0.2 published previously; separate from Rename | report-release-002-1019.md |
| ATCU-MAIN | main E: retains ATCU0.0.1 | instructions-atcu-main.md |
| MOD-6/3/4/5 | accepted0.10.4 short matrix retained | instructions-mod-6.md |
| LAW-4 / U-12 Bill | deferred | existing instructions |

## Next up after 0.0.2 (user, 2026-10-20)

From the published v0.0.2 release notes, in the order listed there. Card
names are proposed; nothing is designed or implemented yet. Each needs its
mockup (R10) and the open [user] questions answered (R9) before work starts.

- **TPL-8 Non-core culture mode.** A third culture mode: recruit only from
  pops that are neither primary nor accepted (spare core pops). Smallest of
  the three: extends `tgcx_CultureMode` in `dll/tgcx/tgcx_army_filters.h`
  and the culture checkboxes. Open: how it sits with the two existing
  checkboxes (three exclusive options vs a fourth state), and whether
  Conform uses it too.
- **TPL-9 Autodivide.** Split a selected stack into N stacks by a rule.
  Conform's extras detach (Create + Transfers queued together, P51) is the
  base. Open: the rule (into single regiments, into N equal stacks, into
  copies of a template), where the button lives, and what happens to the
  leader.
- **TPL-10 Carpet siege.** Spread a stack over enemy provinces to siege
  them at once. Needs TPL-9's split plus one Move per new stack (safe path,
  as Conform). Open: which provinces (all enemy-controlled in a region or
  around the stack), how many regiments per province, and whether it keeps
  re-targeting when a siege ends (that would be a running order, not a
  one-off click).

- **TPL-11 Build Army gathers into one army** (user, 2026-10-20). Today
  Build Army posts one construction per province, so the regiments finish
  as separate stacks that go to the native rally point (or stay put).
  Instead, record a Build Army order like a Conform order (TPL-7 file):
  every finished regiment marches to one gathering province and merges
  into a single army; the first to arrive becomes the army. Reuses
  Conform's march-and-merge and its load resume. Makes several rally points
  irrelevant for template armies. Open: how the gathering province is
  chosen (picked by the player, the capital, the largest source province,
  or the native rally point), and what happens to regiments that cannot
  reach it by land (D17).
- **TPL-12 Replenish depleted regiments up to the template** (user,
  2026-10-21). Unlike Conform, which only counts regiments by type, this
  looks at strength: regiments that are depleted or close to being
  destroyed are removed from the army, and fresh ones of the same type are
  built and merged in (Conform's march-and-merge and TPL-7 orders), so the
  army ends at full template strength. Open: the threshold (a fixed
  strength %, or set per click), whether a depleted regiment is disbanded
  (losing its pops' soldiers back to the pool?) or detached into a
  reserve stack that reinforces normally, the order (build first and drop
  the old one only when its replacement arrives, so the army is never
  short), and whether it is a separate button or a Conform option. Needs
  research: the regiment strength field and the native disband command
  (R5: engine commands only).

Also asked by a player (2026-10-20), not selected: **mobilisation into a
template** (mobilised regiments join an army to fill its template). Needs
its own research: mobilised regiments are created by mobilisation, not
built, and taking them would partly reverse D13's "Conform always builds".
