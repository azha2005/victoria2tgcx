# NEXT - what comes next

2026-10-29. **User tests Volunteers 0.0.12 performance candidate.**
0.0.11 improved slightly but still ran poorly. Its latest captured run had no
volunteer operations; the copied V2DLL configuration enabled state/RNG probes.
The state log wrote 25351284 bytes across January 1..27, 1836. Both expensive
probes are now off in the isolated copy; normal crash/OOS logs remain on.

## Current verified state

- Installed E:/Victoria-II-Volunteers-0004, 0.0.12: 15 payload hashes match,
  eight protected executable/loader/TGC files unchanged. Precisely two V2DLL
  diagnostic values changed; no simulation setting changed. No game launch/input.
- Rollback package AND prior settings: work/volunteers-0012/before-0012.
  Restoring those settings re-enables the expensive probes; record this in comparisons.
- Seven inactive hooks now tail-call native code before FXSAVE/Rust. Active
  paths retain fresh session/thread/load/owner/member gates. Load clears scope.
- Repeated native UI calls are skipped using current visibility/position and
  immutable owned bindings. Observer reports default to every seven ticks;
  gameplay remains daily. Sampled plugin_timing reports categories/call counts.
- 45 Rust tests, six package/actual-loader/settings checks, strict Clippy,
  format/release and 96 native signatures pass. Two benchmarks ignored normally.
  Controlled full idle pair wrapper median 116484800 -> 5863600 ns per million
  fake-native invocations (~19.9x). This is NOT measured game FPS or an OOS fix.
- 0.0.12 game speed/gameplay remain unverified. Earlier sending/battle/siege
  progress accepted; completed capture credit, full vision and lifecycle open.
- Private ZIP: work/volunteers-0012/Volunteers-0.0.12-SP-light-diagnostics-private.zip.
  SHA256 7fb12b7d67dd645789ebf0096769ddae05c4cc771111ea8f0bdb6d929d85df99.

## Remaining cards

| Card | Status | Step-by-step instructions |
|---|---|---|
| VOL-OPT | Installed; actual same-save timing pending | docs/instructions-volunteers-performance-probes.md |
| VOL-SIEGE / VOL-CONTROLS | Progress accepted; capture credit/full vision incomplete | Performance steps 3..6 and docs/instructions-volunteers-siege.md |
| VOL-PROTOTYPE | Return/loss/load/supply acceptance incomplete | docs/instructions-volunteers-prototype.md |
| VOL-OOS / VOL-MP | Send refused; synchronized transport absent | docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-performance-probes-1029.md; private logs/metrics,
benchmarks and install-result.json in work/volunteers-0012. User retains desktop
testing. Source commit/push authorized; preserve unrelated release-document work
and keep private game art/logs/builds out of git. No new subagents or memory edits.
