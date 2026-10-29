# Volunteers 0.0.12: idle hooks and expensive diagnostics

2026-10-29. The user reports 0.0.11 is slightly better but still runs poorly.
This is a failed performance acceptance result. No quantified game timing or
distinction between map responsiveness and game-day speed was supplied.

## Live-file evidence

The captured 0.0.11 volunteer log has 27 executed daily commands, no deployment
or movement posts, and zero exile/vision/siege grants. Its country-vector hook
records 144853 unrelated bypasses and 48 targeted calls at the last sample.
This captured run does not establish active-volunteer costs.

The isolated copy's actual V2DLL settings contain `ENABLE_STATE_PROBE=1` and
`ENABLE_RNG_PROBE=1`. Its V2DLL log explicitly confirms both probes installed:
state hashes run at daily tick entry; RNG and event-roll wrappers are installed.
The latest state log is 25351284 bytes, spanning January 1 through January 27,
1836; it includes country/province/pop/state/building inventories and detailed
differences. The latest RNG log is 189650 bytes. Four retained earlier state logs
range from 270489902 to 395747444 bytes. Raw logs remain private in
`work/volunteers-0012/before-diagnostics`; hash/size/date evidence is in its
`metrics.json`. These are concrete avoidable workloads, not a measured
attribution of all lag or proof that disabling them fixes it.

## Changes

- **Test-copy diagnostics:** only `ENABLE_STATE_PROBE` and `ENABLE_RNG_PROBE`
  change from 1 to 0 in `E:/Victoria-II-Volunteers-0004/V2DLL/v2dll_settings.ini`.
  Normal logging, OOS watch, crash logging, simulation patches and loader options
  remain unchanged. Existing diagnostic files are preserved. This settings change
  belongs to the isolated installation, not the private Volunteers ZIP. The pure
  `tools/volunteers_test_diagnostics.py` transformer rejects duplicate/missing or
  invalid keys, preserves other bytes, and is idempotent.
- **Inactive hooks:** seven unit/vision wrappers tail-call the original before
  FXSAVE/Rust when no native scope is published. The aligned atomic switch is a
  negative filter; an active switch still takes fresh thread, mode, load epoch,
  ownership and receipt gates. The explicit load fence clears published scope.
  Terminal missions retaining vision still count as scope. No stale positive
  permission, native pointer cache or simulation writes were introduced.
- **GUI:** skip repeated button name searches for bindings in a live immutable
  owned subtree. Retiring clears those bindings as before. Hidden windows no
  longer receive repeated native Hide calls; already positioned windows skip
  their native setter after a fresh position read. Show uses the real native
  visibility byte, preserving P42 and P57 across parent show/hide transitions.
- **Observer:** full country/selection inventories are sampled every seven ticks
  by default, using bounded local SEH reads instead of one kernel read per field.
  Tiny fresh world/date checks still notice transitions between samples. Gameplay
  updates remain daily; changing diagnostics does not defer recalls or receipts.
  `sample_every_days=1` remains available for daily evidence.
- **Timing:** cumulative `plugin_timing` lines accompany sampled diagnostics.
  Hot Rust callbacks sample 1/1024 entries, GUI 1/64, daily/observer/vision every
  call. Logs contain calls, samples, sampled microseconds and largest sample.
  They exclude native originals and assembly save/restore, use microsecond
  granularity, and cannot quantify sub-microsecond callbacks from zero durations.
  Counters/owned wrapper benchmarks and same-save wall timing complement them.
  Timing totals saturate instead of silently wrapping.

## Verification and measurements

45 Rust tests pass; two explicit benchmarks are ignored normally. Strict Clippy,
format, x86 release compilation, six package/actual-loader/settings checks and 96
reference native signatures pass. Inactive-wrapper tests pass invalid synthetic
arguments without entering Rust, preserve original results/flags and RET4 across
all seven routes, then verify active-to-inactive switching and callback entry.
Existing active exile/siege/join wrapper checks also pass.

One million full pair-hook invocations against a fake native RET4 function:

| Version | Three release runs, nanoseconds | Median |
|---|---|---|
| 0.0.11 | 120645400, 114884700, 116484800 | 116484800 |
| 0.0.12 inactive | 5904700, 5863600, 5804500 | 5863600 |

This is about 19.9 times less elapsed wrapper cost in a controlled owned-process
fixture, not game FPS. Reproduce from `rust/volunteers` with
`cargo test --release --offline idle_pair_wrapper_benchmark -- --ignored --nocapture --test-threads=1`.
Raw measurements: `work/volunteers-0012/before-wrapper-*.txt`,
`after-wrapper-*.txt`; initial runs also appeared in tool output.

Installed 0.0.12 after checks found no game process. Fifteen payload readbacks
match the staged manifest; eight protected executable/loader/TGC hashes remain
unchanged. The ninth previously protected file is the intentionally changed
V2DLL diagnostics configuration, verified to differ in precisely two values.
Backup of the complete 0.0.11 package and previous settings:
`work/volunteers-0012/before-0012`.

Private ZIP SHA-256:
`7fb12b7d67dd645789ebf0096769ddae05c4cc771111ea8f0bdb6d929d85df99`.
Installed DLL SHA-256:
`08de457c5b49c12ce990d562a3c9ab1dc87be6ee766f7aa253702c768a00a1a3`.
Settings SHA-256 before/after:
`612b5393338233909d41b9de0b6d89cdbc38127c6a035ad424792747e5de3d82` /
`0976bce112d2d02dcdf288d8fabad0ef4843f271c01119e418fea5b333071cd1`.

No game launch/input or new save edits were performed. Game performance remains
unverified. Combat/siege progress acceptance refers to the earlier user tests;
capture credit, full recipient vision, lifecycle acceptance and MP transport are
still open. Follow `instructions-volunteers-performance-probes.md` next.
