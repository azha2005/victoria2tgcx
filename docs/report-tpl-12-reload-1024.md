# TPL-12: Replace reload fixes and README (0.11.13)

2026-10-24, session 81. User: review the latest work and the candidate,
then **"fix whatevers broken and the readmes and i'll run tests"**.
**Rust 0.11.13 built, packaged and installed in the own-loader test copy;
the new reload paths are not tested in game.**

## Review findings (0.11.12)

All automated checks were green and the ZIP matched its report and the
current release build. The review found three defects, none visual:

1. **Merged but not yet detached, then saved.** `replace_done` queued the
   old regiment in the in-memory `retire` list and the record closed the
   same day. A save taken before the detach (for example while the
   army's extras were still leaving, which defers it) lost that state:
   after loading, the old regiment stayed in the army.
2. **Open records that nothing resumed stayed open forever.** On a load,
   a record whose JOIN was not resumed was only made inactive, never
   closed, so it was retried on every load until the 512-record table
   dropped it. The test profile has one (`done=0`, stack 9322).
3. **README wording:** "the maroon **top** button" dated from 0.11.8,
   when the buttons sat above the header; the order-file note also said
   the format was unchanged.

## Changes

- `replace.rs`: `RpOrder.merged`; `tgcx_replace_orders.txt` is now
  version 2 (each line ends `merged=0|1`). Version 1 is imported as not
  merged; strict parsing as before (a v1 line with `merged`, or any other
  value, refuses the file). Pure decisions `on_load` (Skip / Resume /
  Retire / Close) and `finished`, with three new tests (29 in total).
- `conform_native.rs`: `replace_done` marks the record merged. Daily
  closing keeps a merged record open while its old regiment is still in
  the target (read by id: `old_in_target`), so retries across saves work.
  On a load, a merged record whose old regiment is still there is sent
  out again through the existing `replace_retire` (Create + Transfers,
  P51); a record that applies but cannot be resumed is closed at the
  save's date. The stack class needed by `replace_retire` is taken from
  one of the player's armies when nothing else set it after a load, as
  TPL-11 already does. Resume is skipped when Replace's guards failed.
  No new address, read or command: R5 unchanged.
- README: Replace described beside the native header actions; order
  file v2 and rollback consequence documented. CHANGELOG 0.11.13 added.

A merged record whose detach the engine refused stays open until the
old regiment leaves or the next load retries it; nothing is disbanded.

## Validation

- `check_exe.py`: ALL OK (reference SHA-256).
- Rust: `cargo fmt`, host and x86 release Clippy (`-D warnings`, all
  targets) clean; **29 tests** pass.
- Python: Conform GUI, Universal Army, filter layout, catalogue pass.
- Package diff against 0.11.12: only `tgcx.dll`, `v2dll_mod.dll`,
  `tgcx_package.txt`, README and CHANGELOG differ. GUI, art, localisation
  and loader are byte-identical.

## Package and installation

`work/tpl12-01113/ATCU-0.11.13-candidate.zip`, **1,141,046 bytes**.

| Item | SHA-256 |
|---|---|
| ZIP | `76f2643e08865f42e9e7f6015b72290e99651a6787d3cd2559121babd9311264` |
| Logic DLL | `564fa68ec533a98643d034bbd819d879a1139715883478c945ddb8d59974717a` |

No `v2game.exe` process existed. Backup of the 0.11.12 package,
descriptor, loader, `Logs/tgcx.log`, installed manifest and the eight
`tgcx_*` settings: `work/tpl12-01113/before-01113/` (**58** files,
hashes in its sidecar). **48 payload hashes** verified after copying;
no obsolete package files; settings untouched by the install. Record:
`work/tpl12-01113/installation-01113.json`. Main E: unchanged. No game
launch/termination, commit, push or publication.

Rollback: restore `before-01113`, **including `tgcx_replace_orders.txt`**:
0.11.12 leaves a v2 file untouched and does not resume from it.

Next: `instructions-tpl-12-check.md` steps 1-6, with the new step 6 case
(save between merge and detach).
