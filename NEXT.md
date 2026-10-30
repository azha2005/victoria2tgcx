# NEXT - what comes next

2026-10-30. New tester0.0.17 log: hook coexistence accepted; MP handshake never
starts.0.0.18 roster correction/diagnostic candidate ready, not installed. User
retains game/desktop control; no automatic launch, input or live DLL changes.

## Current verified state

- Uploaded Downloads/tgcx_volunteers (2).log:75757 bytes. Third-party Core DLL
  5641e291d2384e956537bf8a8e9d5e1524aed2e43fd1b3417629ace968b82aa2,
  role thunk RVA12AD0 accepted by0.0.17. All native module/scoped/UI/load gates
  pass. MP section has no mp_gameplay_hello_post/hello lines. Exact pre-handshake
  refusal wasn't logged. Later native_load epoch2 and deployment are SinglePlayer,
  not MP success. File has interleaved/malformed streams; preserve it unchanged.
-0.0.18 uses parallel WORLD country/controller vectors (ADC/AE0, AEC/AF0) for
  their mutual length, with global manager size an independent index bound.
  Cross-checks full human set, source/country/tag, local flag, duplicate/cycle/
  count/queue and torn observations. Does not invent or bypass any peer.
- Structured capture refusal stages/values, accepted peer/source list and MP
  initialization logs remove silent returns. UI names roster refusal rather than
  saying missing peer build. A refused Post slot is diagnosed while actual
  posting/issuer capture still requires the checked queue.
- Failed capture withdraws cached receipts/roster; recapture needs fresh executed
  HELLOs with a new issuer round. No unit orders replayed. Coexistence retained.
- Every process has Logs/tgcx_volunteers_<PID>.log; legacy shared log remains only
  convenience. Collector parses PID logs and avoids duplicating legacy mirror.
  Pair exact PID/run/save/load; raw collector aggregates are still diagnostic.
-80 Rust tests,2 ignored benchmarks; fmt/strict Clippy/x86 release pass. Six stage,
  two MP-package and three collector tests pass. Reference EXE/115 guards pass.
- No runtime roster snapshot, actual0.0.18 game launch/install or paired MP test.
  Exact trigger of tester's silent pre-HELLO refusal still needs new-stage trace.

ZIP: C:\Users\JC\Downloads\victoria2template\work\volunteers-mp-0018\final\Volunteers-0.0.18-MP-roster.zip
SHA256: bb7c31d9bd34c29597fea989de12aa23634287142cdaad6b88245e8dde593783
503802 bytes,19 files,mode=mp,no installer/EXE/scripts.
DLL SHA256: 2dce5616fd48a1be225b02b127a3affc3298f0f4dfec7abb07a64b80d1fad1b0

## Remaining cards

| Card | Status | Exact next instructions |
|---|---|---|
| VOL-MP-ROSTER | Correction/diagnostic ZIP checked; exact tester refusal unknown | docs/instructions-volunteers-roster.md steps1..7 |
| VOL-COEXIST | Tester role hook accepted; full combination gameplay pending | Roster step8; docs/instructions-volunteers-coexistence.md |
| VOL-MP / VOL-OOS | Paired gameplay/original OOS acceptance open | Roster steps8..11; docs/instructions-volunteers-mp-gameplay.md |
| VOL-ENTRY / VOL-FEEDBACK / VOL-CONTROLS / VOL-SIEGE / VOL-OPT | Full lifecycle/performance acceptance open | Roster steps8..10; MP-gameplay steps5..10 |

Installed copies unchanged: E:/Victoria-II-Volunteers-0004 at0.0.12;
work/volunteers-mp-0016/game-host at0.0.16. Keep all rollback ZIPs. Preserve
unrelated release-document work and private art/raw RE/binaries. Commit/push
authorized. No subagents, external messages, public binaries or memory writes.
Next pitfall P91. Read P90 and report-volunteers-roster-1030.md.
