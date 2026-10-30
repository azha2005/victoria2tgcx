# Volunteers0.0.18: roster gate and unambiguous traces

2026-10-30. New tester log confirms0.0.17's hook coexistence correction. Actual
MP handshake still never starts. This source correction/diagnostic candidate is
not a claim that the tester's exact roster refusal or original OOS is fixed.
User controls the game; no launch, input, installation or live files changed.

## New evidence

Downloads/tgcx_volunteers (2).log is75757 bytes.0.0.17 reports reference EXE,
mp_play=true and all native/module/scoped/UI/load gates passing. The third-party
Core binary SHA2565641e291d2384e956537bf8a8e9d5e1524aed2e43fd1b3417629ace968b82aa2
has its role thunk at private RVA12AD0;0.0.17 verifies its transparent contract.
Thus the prior fixed-binary/layout conflict is resolved in this actual startup.
It does not establish the behavior of the full third-party plugin combination.

The MP segment contains no mp_gameplay_hello_post, HELLO receipt or sender refusal.
It records diagnostic empty-journal mirrors only, which never authorize gameplay.
No exact native roster/queue failure stage was logged. Source has silent Option
returns before HELLO, so missing-plugin/mismatched-peer claims aren't established.
Later native_load_completed epoch2 and the four-brigade deployment/journal are
explicit SinglePlayer. They do not prove MP Send. Streams for different worlds/
threads and malformed interleaved lines are present; exact file-sharing/merge
history is unknown. Preserve both old logs, don't treat them as clean paired runs.

## Source correction and evidence

Addresses remain RVAs; Ghidra VA=00400000+RVA; runtime=loaded base+RVA.
Exact reference EXE62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648.

Old roster capture equated (world+AF0 minus world+AEC)/4 with the independent
global country-manager size. FUN_00680450 (RVA280450) constructs TWO parallel
WORLD vectors from its input: country pointers ADC/AE0 and human flags AEC/AF0.
FUN_0068B870 (RVA28B870) walks the world country vector, reads country+20 and
checks human nodes. A global table size isn't evidence of the parallel vectors'
length.0.0.18 validates the two world lengths together; the global size remains
an independent bound. Every human country must resolve to the SAME nonzero pointer
in global and world tables and match its country index/tag. Full controller/human
set equality, source mapping, local flag1, duplicate/cycle/list count/end and torn
world/manager/local/vector checks remain. No guessed controller/source fallback.
This removes a concrete unjustified invariant; the uploaded log cannot establish
whether that invariant was the tester's first actual refusal.

Fresh read-only exports also verify FUN_0064A460 (RVA24A460) copies node fields
+1C/+20/+24 from EDI source to ESI destination, FUN_0068EFC0 (RVA28EFC0) handles
native node removal and FUN_007297A0 (RVA3297A0) constructs controller commands.
No new native function calls/hooks are added. Raw C/ASM stays private in
work/volunteers-mp-0018/research. These are S evidence; live roster/source values
on host/client remain unestablished until fresh per-process traces are returned.

## Diagnostic/lifecycle corrections

Capture now returns structured refusal stages and bounded raw values instead of
silent None. Startup logs mp_gameplay_initialized. Native barrier logs the accepted
peer/source/local list or changed mp_roster_refused stage. A failed Post slot can
be diagnosed after recognized MP classification, but posting/issuer proof still
requires the exact queue. No commands are emitted with an invalid roster.

The panel replaces generic peer-wait text with the exact local roster refusal.
Native hook compatibility refusal still has higher priority. Failed capture
withdraws cached roster/receipt authority. Successful recapture needs fresh native
HELLOs, and a new admission round invalidates old issuer permits. No native orders
are replayed, no registry ownership is changed and no safety gate is bypassed.

Each process now writes canonical Logs/tgcx_volunteers_<PID>.log alongside the
legacy convenience file. The latter can still mix shared-directory writers; the
PID file is the pairing artifact. Collector parses PID logs and excludes the
legacy mirror from trace parsing when canonical logs exist, while retaining its
raw copy. Users still select matching PID/save/load; many unrelated traces aren't
a valid paired comparison. No external messages or uploads performed.

## Validation and limits

-80 Rust tests passed,2 ignored benchmarks. Format/strict all-target Clippy/x86
  release pass. New fixtures cover independent global capacity, mismatched world
  vector lengths/pointers, source/local-flag/missing-human refusal stages, receipt
  withdrawal/fresh-round proof invalidation and two writers' intact PID traces.
- Six stage/x86 loader tests, two MP ZIP tests and three collector tests pass.
  Exact reference EXE and115 scoped signatures pass; new280450 constructor record
  anchors world-vector layout evidence. ZIP CRC/manifest/DLL verified.
- No actual0.0.18 game or paired test. No runtime memory roster captured. Actual
  first pre-HELLO refusal, real HELLO delivery, MP Send/control/battle/siege/return
  and the original OOS still need paired evidence. No claim of a full MP fix.
- Installed0.0.12 E: copy and0.0.16 workspace game-host remain unchanged. No
  loader/TGC/executable/save modification, launch or desktop interaction.

## Artifact

C:\Users\JC\Downloads\victoria2template\work\volunteers-mp-0018\final\Volunteers-0.0.18-MP-roster.zip
SHA256: bb7c31d9bd34c29597fea989de12aa23634287142cdaad6b88245e8dde593783
503802 bytes,19 files,mode=mp,8 native textures. No installer,EXE,BAT,CMD,
PowerShell or other scripts. DLL SHA256: 2dce5616fd48a1be225b02b127a3affc3298f0f4dfec7abb07a64b80d1fad1b0
All prior archives retained. Binaries/native art/raw exports stay private and
outside git. Read instructions-volunteers-roster.md. Commit/push authorized;
no public binaries, third-party messages, subagents or memory updates.
