# Volunteers0.0.11: measured helper optimizations

2026-10-29. User confirms0.0.10 works and supplies siege-progress screenshot
(Sao Jose dos Campos45%), requests optimization, then explicitly requests commit
and push after completion. Preserve0.0.10 as the working gameplay baseline.
This confirms siege start/progress, not completed capture attribution or MP.
No desktop input, launch or save edits were used during optimization.

## Changes and safety

- Ordinary pair, access, exile, battle-country and siege callbacks now check whether
  any relevant army identity belongs to a live loan BEFORE fresh session/receipt work.
  Existing-side checks return the original result for entirely unrelated units.
  This is a negative filter only: it never grants permission, caches a result or
  substitutes identity for ownership/membership. Relevant stacks still reread mode,
  actual load context/epoch, donor/recipient conditions and full native receipts.
- A cheap atomic load barrier rejects scope during/after load before reading callback
  units. Positive admission still checks the actual current native context/session.
  Wrong-thread, unknown and multiplayer behavior remains native/refused.
- Closed volunteer panels use a fresh mission-country snapshot without rebuilding
  the full chooser's country list, army names and brigade rows. All mission donors,
  recipients and the current player are included. Real unit receipts remain direct.
  Open/click actions retain the full fresh chooser snapshot.
- Compact polling is allowed only for the same player and explicit native load
  epoch with a bound registry. A load/player change or failed compact capture takes
  a full snapshot, including recipients in the newly loaded journal. No date-jump
  heuristics, stale pointer cache or omitted command-state update was introduced.
- Regression compares full/narrow world inputs under service, recipient peace and
  donor entry into war: orders, mission state and journal bytes remain equal.
  An identity match with wrong ownership/membership never grants authority; a
  subsequent transport change to MP still invokes fresh validation and refuses.

## Measurements

Owned synthetic memory in an actual Win32/x86 test process. It contains1000
countries and an unrelated active mission, native session metadata and two ordinary
armies. No real game process/memory/UI is used. Three release runs per version;
raw logs/source hashes: work/volunteers-0011/before-bench*.txt and after-bench*.txt,
before-bench.json and after-bench.json. Measurements are helper costs, NOT FPS.

| Workload | Before | After |
|---|---:|---:|
| Ordinary-pair memory reads |62|4 (93.5% fewer)|
|1000000 ordinary pair calls, median|485.095ms|45.475ms (10.7x faster)|
| Closed-panel country views in fixture |999|2 (99.8% fewer)|
|2000 full vs compact snapshots, optimized-build median|393.669ms|2.231ms (176.5x faster)|

Full snapshots remain available for the UI. Timing varies with host scheduling:
old pair runs403.741..613.086ms, new42.055..58.586ms; same-build full snapshot runs
279.790..445.525ms, compact1.424..3.002ms. Read counts/included-country sets are exact
fixture outputs. No inference that the whole game is10x/176x faster is warranted.
Real improvement requires a same-save/speed/visibility comparison over game days.

Reproduce the current helper measurement from rust/volunteers so its i686
.cargo config is effective:
`cargo test --release --offline perf_tests::helper_benchmark -- --ignored --nocapture --test-threads=1`.
The benchmark is deliberately ignored by the normal fast suite, then executed
explicitly for this report. Own source fixture: rust/volunteers/src/perf_tests.rs.

## Verification and installed candidate

43 normal Rust tests pass (one explicit benchmark ignored there), plus three
explicit release benchmark runs and five package/actual-loader tests. Strict
all-target Clippy, formatting/release and96 reference-native guards pass. No
additional game address/patch or changed ABI. Siege/combat wrappers remain covered.

Installed0.0.11 in E:/Victoria-II-Volunteers-0004 after process checks:15 mod payload
readbacks match; nine protected game/loader/TGC files unchanged. Backup of working
0.0.10: work/volunteers-0011/before-0011. Main installs and saved games untouched.
ZIP: work/volunteers-0011/Volunteers-0.0.11-SP-optimized-private.zip.
ZIP SHA256: 230872931c9d4ea4188c88f9167018d64883e16bbbd7c2dc0648f2e4975d270e.
DLL SHA256: 0796dca37278af9c3ade497d3b01a555dcb1bce9d94d7bc39135cf6101ea2997.
The prototype launcher opts in; package default stays observe. No0.0.11 game
launch/acceptance is claimed.0.0.10 user/log evidence retains siege_granted2 and
siege_join_granted1; custom vision still shows zero grants with planner refusals,
so recipient-wide sharing remains an open acceptance issue. This pass does not
change visibility algorithms or force simulation state. MP remains refused.

The user authorized a source commit/push. Game art, binaries, raw Ghidra exports,
ZIPs and private peer/log/save evidence stay ignored. Source validation, exact
staged paths and remote state are reviewed before a normal non-force push.
See instructions-volunteers-optimization.md for remaining player/MP work.
