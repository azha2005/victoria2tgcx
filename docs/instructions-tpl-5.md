# TPL-5 - Verify culture filtering in the isolated TGC copy

2026-10-19. Implemented internal0.11.0, installed after explicit normal-exit
confirmation. Not tested in game yet. Read NEXT.md and
report-army-filters-implementation-1019.md. No reinstall required.

1. User opens work/Victoria-II-clean-TGC252/victoria2.exe. Select ATCU - Army
   Template and Conform Universal plus TGC v2.5.2 (stable). Untick old Army
   and Politics. Never launch or stop the game on the user's behalf.
2. Use a country with primary, accepted and foreign soldier cultures.
   Open Build Army / Templates. Existing templates import as All. Test
   neither box, Primary + accepted, Primary only. Boxes are mutually
   exclusive; clicking the selected box returns to All. Check placement,
   click targets and eight composition rows against current screenshots.
3. Recruit the same small composition with each mode, unpausing between
   batches. Inspect the actual queued regiments' source pops, including a
   mixed-culture province where possible. Native unit restrictions still
   apply. Shortage is expected when allowed pops cannot fill the template;
   forbidden cultures must never be used to fill it.
4. Open Conform on an army missing some of the template. Toggle culture
   while paused and verify immediate preview/shortage refresh. Existing
   regiments stay count-based; only missing builds are culture restricted.
   Existing land reach must still exclude overseas inaccessible sources.
   Verify detach/recruit/march/merge still works. Seven Changes rows plus
   overflow summary are intentional; current outer panel stays332x594.
5. Save/reload and verify the mode remains per template, shared across
   builder/Conform. Edits use tgcx_templates_v2.txt in the normal settings
   profile; old tgcx_templates.txt remains byte-identical. Old0.0.1rollback
   sees its retained v1 and cannot see candidate-only v2 edits.
6. User exits normally; preserve full Logs/tgcx.log and Logs/v2dll.log in
   a new ignored evidence folder. Confirm internal0.11.0 and11TGC land
   types. Report observed counts/source cultures and visual acceptance
   separately from automated tests. Do not infer acceptance from fixtures.
7. Diagnose any failure before changing another game copy. Before further
   writes require fresh normal-exit confirmation and process gate. Keep
   before-tgc complete Universal folder/descriptor as0.10.4rollback; no
   base files or loaders need replacement. Main E: stays0.0.1.
8. TPL-6 continent checks follow in the same isolated session if useful.
   After TGC acceptance, stage focused vanilla/HPM/DoD regressions with the
   identical package, preserving rollback and confirming normal exit.
9. Write immutable follow-up report and mandatory NEXT/archive/status close.
   No commit, push, release or external messaging is authorized by testing.
