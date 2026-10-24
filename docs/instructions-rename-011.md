# RENAME-U: retry the 0.1.1 construction correction

**Superseded before handoff by0.1.2. Use instructions-rename-012.md.** The
new preflight needed the owning window definition's button registry;
report-province-rename-preflight-1021.md records the correction.

2026-10-20. Read NEXT.md and report-province-rename-crash-1020.md. The
0.1.0 run failed during native GUI construction; it is not an accepted
baseline. 0.1.1 is installed in the dedicated test copy and needs a retry.

1. User launches work/Victoria-II-Province-Rename-TGC252/victoria2.exe;
   select Province Rename plus TGC v2.5.2 (stable). Use a disposable save.
   No further installation is needed. The agent never launches/terminates it.
2. Select an owned land province while paused. Verify selection itself is
   stable. Click **Rename**, the added control between state and province
   names. Check open, Cancel, reopen, another province and host close/reopen.
3. Logs/tgcx_province_rename.log must identify0.1.1,41native signatures and
   both constructed roots, followed by entry placement. Opening logs the
   province id. A schema refusal names a non-button definition and must be
   diagnosed before another patch. V2DLL's actual logs are V2DLL/Logs/.
4. If stable, follow instructions-rename-u.md sections3-4: real typing,
   Apply, read-back/header/map, accents/length, stale selection/ownership,
   Cancel, save/reload and later ATCU coexistence. Real input/display/save
   acceptance is separate from the native dispatcher regression fixture.
5. On another failure preserve full Rename and V2DLL logs into a NEW ignored
   work/ evidence folder. Do not overwrite crash-0100-1020. Ask only missing
   trigger details. Read the corresponding ATCU pitfalls before changing
   the implementation. Any subsequent installation requires fresh exit
   confirmation and process gate; do all research/build work first.
6. Record observed behavior and screenshot/layout deviations in a new
   report. Archive/rewrite NEXT and update ROADMAP/SUBAGENTS/index. No
   automatic release, commit/push or changes to other game copies.
