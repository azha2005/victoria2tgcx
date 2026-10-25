# TPL-12: new regiment did not march; false load detection fixed (0.11.14)

2026-10-24, session 81. User, with four 0.11.13 screenshots: **"gui looks
fine. but the new unit didnt go to the stack"**. GUI accepted as shown.
**Rust 0.11.14 built, packaged and installed; not tested in game.**

## Evidence

Logs (preserved in `work/tpl12-01114/before-01114/Logs/tgcx.log`, and the
profile's engine logs):

- `TGCX TPL-12 0.11.13`, all guards OK, Replace enabled.
- Replace on stack 40/415: regiment 410 (pop size 350, supports 0) found
  depleted; one infantry queued at province 963 (Lubny);
  `Replace stack 415: 1 depleted, 1 replacements built`.
  Order saved: created 59883768 (= **18 January 1836**), stack 4713/8768.
- Next driver line: **`Conform: 1 pending orders forgotten (another game
  or a load)`**, with **no** "saved orders resumed" line. No march, merge
  or detach afterwards; stack 4713/8768 later appears at province 963
  with its one regiment.
- `game.log` runs day by day from 1 January to 1 June 1836 with **no
  load**. A save `Russian_Empire1836_02_09.v2` was made during
  construction (instruction step 5).
- Same pattern earlier: TPL-11 0.11.5 log, "3 pending orders forgotten"
  right after a Build Army click, no resume line.

## Cause (inferred)

The tracker treated any single frame where the world, player or date
differed from the previous frame (date going back, or a large jump) as a
load: it dropped all tasks and resumed only saved orders applying to the
date read on that frame. The resume skipped the order, which fits a date
read once as earlier than 18 January (e.g. around the save); the next
normal frame was not a load again, so nothing restored the task. The
exact transient value was not logged, so this is inferred from the code
path and the absent resume line, not observed. Recorded as **P64**.

## Fix

- `conform::confirm_load`: a load must be seen on **three frames in a
  row** before tasks are dropped; until then the driver does nothing
  that frame. Real loads stay stable (paused after loading) and are
  confirmed within three frames. New test (30 Rust tests).
- World dates before year 0 (`5000 * 8760` hours) are ignored.
- A confirmed load logs `Conform: load seen: world, player, date, step`
  (old -> new), so another occurrence can be diagnosed.
- Covers Conform, Replace, Build Army gathering and Mobo, which share
  this driver. No new address, read or command (R5 unchanged).

Recovery of the stranded regiment: the saved orders still have it open
(`done=0`, Conform order and replacement record). Loading the 9 February
save resumes its march and merge, then the old regiment's detach.

## Validation and installation

`check_exe` ALL OK; Rust fmt, host and x86 Clippy clean, 30 tests;
Python GUI/Army/filter/catalogue pass. Package diff to 0.11.13: DLLs,
package version, README version line, CHANGELOG only.

`work/tpl12-01114/ATCU-0.11.14-candidate.zip`, 1,142,524 bytes,
SHA-256 `24a6b960adba6914077e66213408888ac32183e2073831a5e675aea86167ddc6`;
logic DLL `a18371eae849c532a2af8462598a1bcbd3185866fad50d8b05deb9efa9bb20ca`.
No game process. 0.11.13 backup with log and settings:
`work/tpl12-01114/before-01114` (58 files); 48 payload hashes verified;
settings untouched. Record `installation-01114.json`. No game
launch/termination, commit, push or publication.

Next: `instructions-tpl-12-check.md`, starting with the 9 February load.
