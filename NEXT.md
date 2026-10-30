# NEXT - what comes next

2026-10-29. **User clarifies the0.0.15 button fixes are wanted in the MP build.**
Separate MP probe ZIP prepared with the same corrected DLL and mode=mp_probe.
Full MP volunteer gameplay remains unfinished; no OOS fix or two-peer test claimed.
The screenshot trigger remains unconfirmed; user/tester retain game operation.

## Current verified state

- Correct MP archive: work/volunteers-mp-0015/final/Volunteers-0.0.15-MP-sync-probe.zip.
  460894 bytes,19 files, no installer/scripts. SHA256:
  2191551f2bf99fbb78af13348f377816205157a28d261167b5d5c54ad5232cc9.
  DLL is byte-identical to SP0.0.15 below; explicit mode=mp_probe and native art
  verified. One package regression/CRC/reproducibility check passes. Both players
  select this ZIP for diagnostic MP testing; Send/Recall/scopes remain refused.
- Plain ZIP: work/volunteers-entry-0015/final-r2/Volunteers-0.0.15-SP-beta.zip.
  463228 bytes,20 files; no installer/EXE/BAT/CMD/PowerShell/scripts. SHA256:
  c3df063e8bf71500cc1e1bb265c69ce615c880259a0100af7229fc53ed311380.
  DLL SHA256:974a306baf282a846bb8ff8d1648adf0223ae5030ff19c1ef1236647cb0dd4ef.
- Worker/reentrant hides defer to the UI thread rather than permanently disabling
  the UI. Read-only scene-order checks recover an entry covered by Diplomacy,
  preserve later visible dialogs and handle recognized closed-dialog renderers.
  Hidden child controls get native subtree activation; expired/pending-release
  owned windows are rebuilt with old observer boxes retained under existing bounds.
- Checkpoint failures stop mission commands/scope until an explicit new load
  epoch while leaving the validated button/panels available. No automatic partial
  retry or replay; actual native faults/type/capacity failures remain hard gates.
-61 Rust tests pass, two benchmarks ignored. Format/strict Clippy/x86 release,
  six package, two share, one MP-package, two collector checks and107 scoped
  signatures pass.11 native art/GUI/localization files are byte-identical to0.0.13.
- No game, desktop input, install, loader/TGC/save edit or live disappearance
  reproduction. Installed E: copy stays0.0.12/CreatineCore6.36 (DLL08de457c...a00a1a3).
  All prior SP/MP archives retained. New candidate needs actual click/reopen/
  background/long-session and performance retest.

## Remaining cards

| Card | Status | Exact next instructions |
|---|---|---|
| VOL-ENTRY |0.0.15 SP ZIP ready; actual recovery untested | docs/instructions-volunteers-entry.md steps1..7 |
| VOL-FEEDBACK / VOL-CONTROLS / VOL-SIEGE / VOL-OPT / VOL-PROTOTYPE | Prior lifecycle, vision and performance acceptance open | Entry step9; docs/instructions-volunteers-feedback.md |
| VOL-MP-SYNC |0.0.15 MP probe includes entry fixes; paired test not run | docs/instructions-volunteers-mp-sync.md steps1..7 |
| VOL-MP / VOL-OOS | Full volunteer multiplayer, sender/peer authority, deterministic activation/order authority and original OOS remain unresolved | MP-sync steps8..10; docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-mp-package-1029.md and docs/report-volunteers-entry-1029.md. Commit/push remains authorized.
Preserve unrelated release-document edits and private art/builds/raw exports.
No new subagents, peer messages or memory writes. Next pitfall P87.
