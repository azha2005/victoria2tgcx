# TPL-12 check: name box and Disband depleted (0.11.16)

0.11.16 is installed in `work/Victoria-II-TGC252-own-loader` (report:
`report-tpl-12-namebox-1024.md`). 0.11.15 passed the Replace depleted check
(`report-tpl-12-passed-1024.md`). The only change since then: the unit
name box no longer covers the left part of Disband depleted.

## In game

1. **Button shown.** Select an army that has a depleted regiment, so
   Disband depleted is shown. Hover the button from its right edge to its
   left edge. You see the button's own tooltip everywhere, never "Name of
   Unit". Click its left half: it acts like the right half.
2. **Name box still works.** Click the name box left of the button. Rename
   the army and press Enter. The new name is kept.
3. **Button hidden.** Select an army with no depleted regiment. The name box
   reacts across its full width, as in 0.11.15.

## Afterwards

Keep `Logs/tgcx.log`. It should say `TGCX TPL-12 0.11.16` and
`Name box hit test hooked`. After step 1 it should also say
`Name box: stops at Disband depleted`. A line saying `not recognised` means
the fix did not engage.

## Rollback

Close the game normally. Then restore `work/tpl12-01116/before-01116`
(0.11.15) or `work/tpl12-01115/before-01115` (0.11.14).
