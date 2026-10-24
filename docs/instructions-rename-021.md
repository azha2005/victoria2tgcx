# RENAME-021: retry the corrected native ABC placement

2026-10-21. 0.2.0 loaded/constructed roots but refused placement. Read-only
live inspection established the real runtime size and separate text-local
offsets. 0.2.1 is installed after fresh normal-exit confirmation, with full
backup and verified payload/protected hashes. No additional install needed.
Report: report-province-rename-anchor-1021.md; pitfall P61.

1. User opens `victoria2.exe` inside
   `work/Victoria-II-Province-Rename-TGC252`. Select **Province Rename
   (Universal)** plus **TGC v2.5.2 (stable)**. Keep the existing V2DLL and
   TGC252_ProvinceRename_20261020 profile; no main-copy changes.
2. Select owned land while paused. Expect **one 28x28 native ABC button
   immediately to the RIGHT of the province name**. Actual province name
   remains visible; there is no extra Rename label. Check a short and a
   long/accented name. The entry may clamp at the native header's right edge.
3. Inspect Logs/tgcx_province_rename.log: **0.2.1 Universal**, **59 native
   signatures OK**, both roots, then **Universal Rename ABC icon** with
   screen coordinates, editor coordinates and measured font width/fallback.
   Any refusal now specifies its stage/root/runtime width. V2DLL logs are
   in V2DLL/Logs/. Preserve fresh logs/captures before another run overwrites
   them; do not assume a log from an earlier launch is current.
4. Click ABC. Check black text/input, native green/braid/parchment/buttons,
   editing, Rename, Cancel/reopen and host/selection changes. Compare to
   docs/province-rename-layout.svg and
   work/province-rename/candidate/layout-preview-021-native.png. Native art
   and layout are identical to0.2.0; only placement/diagnostics changed.
5. If the entry works, continue instructions-rename-020.md sections2-3 for
   full behavior/limits/accents, map/header/tooltip, stale selection/owner,
   save/reload and different-save cleanup. Use0.2.1/59 in all log checks.
   The earlier accepted0.1.3 short check did not prove those extended cases.
6. User exits normally and preserves logs/unedited screenshots. On failure,
   identify the actual refused stage before patching. Read P37/P40/P44/
   P53-P61, match concrete object fields to live/native semantic evidence,
   and add a regression that the previous implementation actually fails.
   Do not reuse a fixture that merely repeats the field interpretation.
7. Further installation requires fresh exit confirmation and no running
   v2game.exe. Do not launch or terminate the game autonomously. Retain
   before-021 (0.2.0) and before-020 (working0.1.3) rollbacks. Other bases,
   ATCU coexistence, MP, release and publication remain later selected work.
