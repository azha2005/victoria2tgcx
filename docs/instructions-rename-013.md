# RENAME-U: verify the measured-type correction in 0.1.3

2026-10-21. Follow NEXT.md and report-province-rename-button-type-1021.md.
The0.1.2 screenshot failed because the class guard refused valid native
standard buttons.0.1.3 is installed in the same dedicated copy.

1. User opens work/Victoria-II-Province-Rename-TGC252/victoria2.exe and
   selects Province Rename plus TGC v2.5.2 (stable). Use a disposable save.
2. Select an owned land province while paused. Inspect the top-left header
   between state and province names. Check Rename, Cancel, reopen, change
   province and host close/reopen. Do this before testing Apply.
3. Expected log:0.1.3,49native signatures, both constructed roots and entry
   placement; opening logs the province id. Logs/tgcx_province_rename.log
   is the plugin log; V2DLL/Logs/ contains core/crash logs. A refusal now
   reports actual type/table. Preserve that full log before another change.
4. If entry/open/close succeeds, complete instructions-rename-u.md sections
   3-4: typing, Apply/readback/header/map, Cancel, accents/length limits,
   selection/ownership invalidation and save/reload. Compare screenshots to
   docs/province-rename-layout.svg; record actual deviations. Later ATCU
   coexistence remains a separate check.
5. User exits normally. Verify the process actually exits. If it hangs,
   report the exact process and wait for explicit user authorization before
   any forced termination; the prior PID47976 exception is not recurring.
   Every installation needs fresh exit confirmation and a process gate.
6. Preserve fresh logs/screenshots under a new ignored work/ evidence
   folder, then record results in a new report. Archive/rewrite NEXT and
   update ROADMAP/SUBAGENTS/index with actual next-card instructions. No
   automatic release, commit/push or changes to other game copies.
