# NEXT - what comes next

2026-10-29. **User reports the Send volunteers button disappearing again.**
Source-backed entry recovery0.0.15 built and packaged for SP retesting. The exact
screenshot trigger is unconfirmed. User/tester retain desktop/game operation.

## Current verified state

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
| VOL-MP-SYNC |0.0.14 native transport probe ready; paired test not run | docs/instructions-volunteers-mp-sync.md steps1..7 |
| VOL-MP / VOL-OOS | Full volunteer multiplayer, sender/peer authority, deterministic activation/order authority and original OOS remain unresolved | MP-sync steps8..10; docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-entry-1029.md. Commit/push remains authorized.
Preserve unrelated release-document edits and private art/builds/raw exports.
No new subagents, peer messages or memory writes. Next pitfall P87.
