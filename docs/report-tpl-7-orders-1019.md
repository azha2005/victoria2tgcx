# TPL-7: Conform orders survive a load (0.11.2), 2026-10-19

Status: **passed in game 2026-10-19** (user: "i believe it worked"; tgcx.log, �8). Installed in the isolated TGC copy on
2026-10-19 after the user confirmed a normal game exit: 41/41 package files
verified, 0 stale, protected anchors and the loader unchanged. Backup:
`work/orders-0112/before-tgc/`; record `work/orders-0112/installed-tgc.json`.

## 1. Trigger

The user's 0.11.1 demo needed a reload. After it, a regiment that had been
queued by Conform for a stack went to the native rally point instead of
joining that stack. This was the expected behaviour under D13 (session-only
tracking: `tgcx_CfForget` dropped every task when the date went back).

The user chose option 2: "file matching date, stack and construction id".
That revises D13 (AGENTS.md §9).

## 2. Evidence the ids survive a save

In `autosave.v2`, each `military_construction` block holds its own
`id={id=… type=4713}` and `unit={id=… type=4713}`. `unit` is the id of the
future stack (construction `+0x58`; see `docs/symbols.md`). Both are written
to the save and read back, so the ids the DLL recorded when it queued the
regiment are still valid after a load. Type 4713 is 0x1269, the high
registry table, which is the same one `tgcx_CfLookup` already walks.

## 3. Design

- **Record.** `tgcx_CfOnQueued` keeps doing what it did (one JOIN task per
  regiment Conform builds). It now also appends a record:
  `tag` (target stack +0xC0), `created` (game date), `done` (0 while open),
  `target`, `stack`, `construction`, `unit`. The task holds the record's
  1-based index.
- **File.** `tgcx_conform_orders.txt` lives in the same Victoria II settings
  folder as `tgcx_templates_v2.txt`. The first line is
  `tgcx_conform_orders 1`, then one `order tag=%08X created=%u done=%u
  target=%u/%u stack=%u/%u construction=%u/%u unit=%d` line per order (CRLF).
  The file is written whole to `.pending`, flushed, then moved over the old
  one with `MoveFileExW(REPLACE_EXISTING|WRITE_THROUGH)`, the same pattern as
  the templates. It holds at most 512 orders. When it is full, the oldest
  finished order is dropped first, then the oldest open one.
- **Strict parse.** Each line has to print back byte for byte. A file that
  fails this, or has an unknown header, is logged and left untouched. Nothing
  is resumed from it and it is never overwritten.
- **Closing.** After each game day, an open record whose task has ended
  (merged, cancelled, target gone, unreachable) gets `done = that day`.
- **Load detection.** Any of these counts as a load:
  - a different world pointer;
  - a different player country index (`+0xB60`);
  - the date going backwards;
  - a forward jump of more than 45 times the smallest day step seen.

  On a load, every session task is forgotten, as before. Then each record is
  resumed if all of these hold:
  - the target's tag equals the record's;
  - `created <= loaded date < done` (an open order has no end);
  - the target is again the player's land stack.

  The land-stack class is taken from the target when no Conform ran yet this
  session (`n.land`), and must match otherwise.
- **Resume.** The task is re-added as JOIN with `construction_seen=true`:
  - a regiment still being built waits for its construction;
  - a finished one, even one at a rally point, gets the usual Move to the
    target's province and a Merge there;
  - a construction and stack that are both gone end after the usual two
    days and close their record.

  Only the existing Move/Merge engine commands are sent (R5).
- **Failure.** If the file cannot be read or written, the log says so once.
  Session tracking carries on as in 0.11.1.

A save made before an order was created, or after it finished, never resumes
it. The date window gives that, so no id of the save itself is needed.

## 4. Files

- `dll/tgcx/tgcx_conform.h`: `tgcx_CfTask::record`; the pure `tgcx_CfOrder`
  section (applies, add with drop index, line, write, strict parse).
- `dll/tgcx/tgcx_conform_native.h`:
  - load, save and new record;
  - resume and close;
  - load detection in `tgcx_CfDaily`;
  - recording in `tgcx_CfOnQueued`;
  - a save after the recruit in `tgcx_CfConformWith`.
- `dll/tgcx/tests/tgcx_conform_native_tests.h`: the D13 forget expectation is
  reworded, and 14 new checks are added:
  - round trip, damaged line, header, date and tag window;
  - full-file drop order (finished first, then oldest open);
  - resume of an order still under construction and of one at a rally point
    (the march is sent);
  - closing on the day the task ends;
  - the 46-day jump counts as a load;
  - another tag is not resumed;
  - Conform records one linked order per built regiment.
- Version: `tgcx_dllmain.cpp` marker `TGCX TPL-7 0.11.2`;
  `build_universal_army.py`, `install_dll.py`, `test_install_dll.py` and
  `test_army_filter_layout.py`. The README has a new CONFORM ORDERS paragraph.

## 5. Gates (2026-10-19)

- `tools/build_dll.ps1`: built, gen_signatures ALL OK (179).
- `tools/test_dll.ps1`: `tgcx_tests: ALL OK`, installer tests OK.
- Every `tools/test_*.py`: OK.
- `python tools/check_exe.py`: ALL OK.

## 6. Package

`work/orders-0112/ATCU-orders-0.11.2-test.zip` has 43 files, SHA-256
`42cb2b3bb07f4a8b81e2c3eb906590b0270660d3fa5ebc10c05037a1f6096fae`.
Per-file hashes are in `ATCU-orders-0.11.2-test.SHA256.json`. Compared with
0.11.1, only these differ:

- `tgcx.dll` and `v2dll_mod.dll`;
- `lua51.dll`: rebuilt, but its source is unchanged;
- `tgcx_package.txt` (version);
- `README.txt`.

The art and GUI are those of 0.11.1.

Install (after a fresh normal-exit confirmation):
`python work/orders-0112/install.py --normal-exit-confirmed`. It backs up into
`work/orders-0112/before-tgc/`.

## 7. Player check (pending)

1. Conform a stack so that some regiments are built. Save during the build.
2. Let some finish and merge, then load that save.
3. `tgcx.log` should show `Conform: N saved orders resumed after a load`.
   The finished regiments should march to and merge into the conformed stack
   instead of staying at the rally point.
4. Load a save from before that Conform: nothing should resume.

Not covered: multiplayer (D5), and a target stack that was renumbered or
merged into another stack before the save (that order is skipped).

## 8. In-game result (2026-10-19)

The user ran the check on the isolated TGC copy and said "i believe it worked".
`work/Victoria-II-clean-TGC252/Logs/tgcx.log`:

```
TGCX TPL-7 0.11.2 tgcx.dll loaded_by=V2DLL plugin API v1 base=00E10000 ...
Conform stack 415: 2 detach transfers queued, 0 gathered, 3 built, 0 left out, 0 failed
Conform: stack 9622 marches to province 364 (safe path)
Conform: stack 9622 joins 415
Conform: 2 pending orders forgotten (another game or a load)
Conform: 2 saved orders resumed after a load (2 still being built), 0 refused
Conform: stack 9619 marches to province 364 (safe path)
Conform: stack 9616 marches to province 364 (safe path)
Conform: stack 9619 joins 415
Conform: stack 9616 joins 415
```

One of the three regiments joined before the save. After the load the other
two were resumed while still being built, then marched and merged into stack
415 instead of staying at the rally point. Step 4 of �7 (a save from before
the Conform resumes nothing) is not shown in the log and stays unconfirmed.
Loaded through V2DLL; the own-loader path was not part of this run.

## 9. Own-loader run (2026-10-19)

New copy `work/Victoria-II-TGC252-own-loader`: a copy of the V2DLL test copy
without V2DLL (`lua51.dll` is the 0.11.2 package loader, `lua51_real.dll`
the original), with ATCU 0.11.2 files hash-verified (0 mismatches) and the
reference `v2game.exe`. Recorded as `C:\Users\JC\Videos\atcu-demo-own-loader-1019.mkv` (4:52).
`tgcx.log` there:

- `TGCX TPL-7 0.11.2 tgcx.dll loaded_by=TGCX loader MOD-2 0.8.0`; templates=1 conform=1.
- `Conform orders file: 3 saved orders` (shared TGC profile); no order
  resumed when the game was entered, which is consistent with a game dated
  outside every order's window. What was loaded is not in the log, so §7
  step 4 stays unconfirmed.
- Two template recruits (10 and 3 queued); Conform stack 415 built 3
  regiments and all three marched and joined 415. No reload in this run.
