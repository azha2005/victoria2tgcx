# TPL-12: save loaded from the main menu not seen as a load (0.11.15)

2026-10-24, session 81. User on 0.11.14: **"the lubny regiment still didnt
march"**. **Rust 0.11.15 built, packaged and installed; not tested in game.**

## Evidence

`work/tpl12-01115/before-01115/Logs/tgcx.log` (0.11.14 run):

- One load only: `Conform: load seen: world 0479C550->0479C550 player 0->65
  date 59883360->59883360 step 0`. That date is **1 January 1836**: the
  main menu's world, when Russia was picked. No resume could apply.
- The save then loaded, dated later, with no second "load seen" and no
  "saved orders resumed". Stack 4713/8768 (the Lubny regiment, one
  infantry) is probed in that world; nothing moves it.
- Both saved records for it are still open (`done=0`).

## Cause

A forward date jump counted as a load only once a day step was known
(`step != 0`). Right after a load (and in the menu) the step is 0, so a
save dated after the menu's bookmark date was never a load: it looked
like time passing. Loads from the in-game menu go back in time and were
detected. Recorded as **P65**.

## Fix

`conform::date_jump`: with no step known, a forward change of more than
**two days** is a load; with a step, 45 steps as before. Then P64's
three-frame confirmation applies. New test (31 Rust tests). No new
address, read or command.

Remaining gap: a reload to the same date, or forward by less than 45 days
while playing, is still not seen as a load (orders already in memory keep
running).

## Validation and installation

`check_exe` ALL OK; Rust fmt, host and x86 Clippy clean; Python
GUI/Army/filter/catalogue pass. `work/tpl12-01115/ATCU-0.11.15-candidate.zip`,
1,141,614 bytes, SHA-256
`1c95c31fe332644f73b715937ec14a4a31a79c98366d6fe4a7c245aa87c41415`;
logic DLL `04327d5aff13d6527fa18c5aacbd1ec4c3b0c9e95b305d0cd8bed7d7d2a2b546`.
No game process; 0.11.14 backup with log and settings
`work/tpl12-01115/before-01115` (58 files); 48 payload hashes verified;
settings untouched. Not committed.
