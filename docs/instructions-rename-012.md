# RENAME-U: retry the installed 0.1.2 candidate

**Superseded by0.1.3. Use instructions-rename-013.md.** The0.1.2 player
run had no entry because its concrete button-class guard was wrong. P60
and report-province-rename-button-type-1021.md record live evidence.

2026-10-21. Read NEXT.md, report-province-rename-crash-1020.md and
report-province-rename-preflight-1021.md. 0.1.2 supersedes interim 0.1.1.

1. User opens work/Victoria-II-Province-Rename-TGC252/victoria2.exe;
   tick Province Rename plus TGC v2.5.2 (stable), then use a disposable save.
   No further installation is needed. The agent never launches/terminates it.
2. Select an owned land province while paused. Verify stability, then click
   the added Rename control between state and province names. Test Cancel,
   reopen, another province, host close and host reopen before testing Apply.
3. Logs/tgcx_province_rename.log must identify 0.1.2, 42 native signatures,
   both constructed roots and entry placement. Opening logs the province id.
   A schema refusal names the owning window and button; preserve the full
   log. V2DLL core/crash logs are in V2DLL/Logs/.
4. If stable, follow instructions-rename-u.md sections 3-4: real typing,
   Apply/readback/header/map, accents/length limits, stale selection/owner,
   Cancel and save/reload. Compare screenshots to the local SVG layout.
   Later ATCU coexistence is a separate check with a multi-plugin loader.
5. On failure preserve fresh full logs in a NEW ignored work/ evidence
   folder. Do not overwrite crash-0100-1020. Read relevant ATCU pitfalls and
   diagnose the exact stage before another patch. Do research/build work
   first; obtain fresh exit confirmation and process gate before installation.
6. Record actual observations in a new report. Archive/rewrite NEXT, update
   ROADMAP/SUBAGENTS/index and prepare next-card instructions. No automatic
   release, commit/push or modification of other game copies.
