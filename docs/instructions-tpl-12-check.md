# TPL-12 check: Replace depleted (0.11.15)

0.11.15 is installed in `work/Victoria-II-TGC252-own-loader` (report:
`report-tpl-12-menu-load-1024.md`). The GUI passed. The Lubny regiment
did not march in 0.11.13 (P64) or 0.11.14 (save loaded from the main menu
not seen as a load, P65); 0.11.15 fixes both.

## In game

1. **Stranded regiment.** Start the game, load `Russian_Empire1836_02_09`
   from the main menu. Unpause. The
   infantry built at Lubny marches to the 3-i Armeiskii Korpus and merges.
   Then the old regiment (the red pop) leaves into its own stack.
2. **New replacement.** Press Replace on another army with a depleted
   regiment. Let it run past the build. It marches and merges, then the
   old regiment leaves. Nothing is disbanded.
3. **Save during construction.** Save, keep playing: it still marches.
   Load that save: it still marches.
4. **Save after the merge.** Save on the day the new regiment has merged,
   while the old one is still in the army. Load it. The old regiment
   still leaves.

## Afterwards

Keep `Logs/tgcx.log` and your screenshots. It should say
`TGCX TPL-12 0.11.15`. Loading the save shows `Conform: load seen` with
the save's date, then `saved orders resumed after a load`.
A `pending orders forgotten` line without a load from you is the bug
again. This check does not cover multiplayer or other bases.

## Rollback

Close the game normally, then restore `work/tpl12-01115/before-01115`
(0.11.14), `work/tpl12-01114/before-01114` (0.11.13) or `work/tpl12-01113/before-01113` (0.11.12; also restore its
`tgcx_replace_orders.txt`).
