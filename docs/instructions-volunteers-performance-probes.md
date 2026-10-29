# VOL-OPT: 0.0.12 game timing and diagnostic attribution

2026-10-29. Read the matching report and P72, P76, P77. The user retains game
input; no desktop takeover or automatic launch. Source commit/push is authorized.

1. Run `E:/Victoria-II-Volunteers-0004/tgcx_start_prototype.cmd`. Use the same
   campaign save, speed, camera and panel state as the slow 0.0.11 run. Main
   installations and saves remain outside this test. The package defaults to
   observe; the existing launcher explicitly enables prototype mode.
2. Check the newest V2DLL log's configuration lines: STATE_PROBE and RNG_PROBE
   must be 0. Preserve older logs; their existence does not mean a probe ran in
   the new session. Confirm no new daily state/RNG dumps are produced. Keep
   normal OOS/crash reporting enabled. Do not change simulation settings.
3. Distinguish paused map/button responsiveness from unpaused game-day speed.
   After warm-up time a fixed 30-day range several times; record wall seconds,
   campaign dates, speed, panel state and whether volunteers are active. Compare
   separate baseline/no-loan, serving and battle/siege states as available.
4. Read `Logs/tgcx_volunteers.log`: build 0.0.12, admission and plugin_timing at
   the first/each seventh tick. Compare cumulative deltas, not totals across
   different session lengths. Active=0 hooks intentionally bypass Rust and its
   counters. Hot callbacks sample 1/1024, GUI 1/64; daily/vision sample all.
   sample_us is sampled wall time, not total CPU time. Zero sub-microsecond
   samples do not establish zero cost; native originals and assembly are excluded.
5. If still slow, use those category costs/call counts to choose the next change.
   Compare V2DLL-only versus plugin in a disposable compatible save only if
   needed; do not unload it from a live process or use a serving mission as a
   supposedly equivalent no-plugin baseline. For a precise probe attribution,
   after normal exit compare the same DLL/settings with only the two diagnostics
   toggled. Preserve both configurations and logs. Do not claim all improvement
   belongs to one change when both DLL and configuration changed together.
6. Retest diplomacy entry persistence/reopen/position, Send, ordinary control,
   battles, siege start/join/progress, completed capture credit, recall/peace,
   donor war and save/reload. Parent-show must still activate native children.
   No performance shortcut may grant or retain stale native permission.
7. Rollback after normal exit: restore package/descriptor and the exact V2DLL
   settings from `work/volunteers-0012/before-0012`. Restoring the settings also
   restores the expensive probes; for a DLL-only comparison retain the light
   configuration deliberately and record that difference.
8. Source changes: `tools/build_volunteers.ps1`, `python tools/test_volunteers.py`,
   `python tools/check_exe.py`. The explicit full-wrapper benchmark runs from
   `rust/volunteers` as documented in the report. Keep private assets/logs/builds
   out of git; preserve unrelated release-document changes when committing.
9. Remaining functional/MP steps retain `instructions-volunteers-optimization.md`,
   `instructions-volunteers-prototype.md` and `instructions-volunteers-oos.md`.
   Archive NEXT, update current statuses and write the next concrete steps.
