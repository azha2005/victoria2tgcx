# TPL-8 check: Non-core only (0.11.3)

Installed in `work/Victoria-II-TGC252-own-loader` (2026-10-24).
Report: `report-tpl-8-noncore-1024.md`.

1. Launch with only ATCU and TGC v2.5.2 ticked. `Logs/tgcx.log` shows
   `TGCX TPL-8 0.11.3`.
2. **Templates:** three culture boxes; only one can be ticked; clicking the
   ticked one clears it. Recruit with Non-core only: only non-core pops are
   used, the rest is reported as left out, and nothing crashes.
3. **Conform:** culture on two lines, 6 Changes rows. Non-core only builds
   the missing regiments and they join the stack.
4. Save, reload: the choice is kept.
5. Send screenshots of both windows. Close the game normally.

Rollback: replace `mod/tgcx_army_universal` and its `.mod` with the copies in
`work/noncore-0113/before-0113/mod/`.
