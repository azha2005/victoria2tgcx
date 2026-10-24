# TPL-11 check: Build Army gathering (0.11.6)

Candidate: `work/tpl11-0116/ATCU-gather-0.11.6-test.zip` (0.11.4 passed
steps 2-3 and Capital; 0.11.5 swaps the third option for the rally point).
Reports: `report-tpl-11-gather-1024.md`, `report-tpl-11-rally-1024.md`.

Install (agent, after you confirm the game is closed normally): back up
`mod/tgcx_army_universal` and its `.mod` from
`work/Victoria-II-TGC252-own-loader` into `work/tpl11-0114/before-0114/`,
then copy the candidate's `mod/` over it.

1. Launch with only ATCU and TGC v2.5.2 ticked. `Logs/tgcx.log` shows
   `TGCX TPL-11 0.11.6` and `Build Army gathering enabled`.
2. **Best supply:** recruit a template. The status names the province. When
   the first regiment finishes, it walks there; the others join it.
3. **Second army:** recruit again. It gathers in a province beside the first.
4. **Capital** and **rally points** (set two or more land rally points in
   your provinces; the arrows show each by name after Capital): recruit
   once with Capital and once with each rally point.
5. Save while regiments are being built, reload: they still gather.
6. Send screenshots and close the game normally.

Rollback: replace `mod/tgcx_army_universal` and its `.mod` with the copies in
`work/tpl11-0114/before-0114/mod/`.
