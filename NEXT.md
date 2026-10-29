# NEXT - what comes next

2026-10-29. **User requests MP support/OOS attempt. Native MP sync probe0.0.14
built; multiplayer volunteer gameplay remains unimplemented and refused.**
Do not describe this as an OOS fix or a tested two-peer transport. User/tester
retain game/desktop operation; no new subagents or external messages this task.

## Current verified state

- Plain MP probe ZIP: work/volunteers-mp-0014/final/Volunteers-0.0.14-MP-sync-probe.zip.
  454764 bytes,19 files; no EXE/setup/BAT/CMD/PowerShell/scripts. SHA256:
  f6cf2858f1d0a919c3f70ee9ab898f2484f36cb89d6009bf1cb30d2dbec181e5.
  DLL SHA256:0542c12d68ce086d23745c47002ad27560efd67719ba8974a0b08fddb5f61e60.
- mode=mp_probe posts one bounded fingerprint marker via native MP
  CSetVariableCommand per local country/load, observes execute receipts, and
  mirrors all-country committed journals read-only. No troop commands/scopes.
  Existing SinglePlayerPermit, combat/access and Send/Recall guards retained.
- Host/client Post ABI and native variable serialization inspected in targeted
  Ghidra exports. No complete sender-country authority/peer membership proved.
  Deterministic shared policy activation and one derived-order authority open.
- Explicit load resets the mirror; bounded country bootstrap, sequence/replay/
  overlap checks, same-date digest and missed/refused counters added. Collector
  compares last-load receipts and unambiguous complete samples, not native CRCs.
-57 Rust tests pass, two explicit timing benchmarks ignored. Strict Clippy,
  formatting, x86 release, six package, two collector, one MP ZIP and two retained
  SP ZIP fixtures pass. Reference executable and105 scoped signatures pass.
- No installation, game/desktop test, native network execution or paired MP
  logs/save supplied. Old OOS cause still unestablished. Installed E: copy stays
  accepted0.0.12/CreatineCore6.36, DLL08de457c...a00a1a3. SP0.0.13 ZIP retained
  under work/volunteers-0013/final-r2/ for the prior feedback retest.

## Remaining cards and exact next steps

| Card | Status | Instructions |
|---|---|---|
| VOL-MP-SYNC | MP delivery/mirror probe built; paired game test pending | docs/instructions-volunteers-mp-sync.md steps1..7 |
| VOL-MP | Sender/peer authority, deterministic activation and derived orders open | MP-sync steps8..10 |
| VOL-OOS | Specific divergence unreproduced; no fix claimed | MP-sync steps4..10; docs/instructions-volunteers-oos.md |
| VOL-FEEDBACK / VOL-SHARE | SP0.0.13 candidate remains available, not installed | docs/instructions-volunteers-feedback.md steps1..7 |
| VOL-CONTROLS / VOL-SIEGE / VOL-OPT / VOL-PROTOTYPE | Existing lifecycle/performance acceptance still open | Feedback steps4..8; prototype card |

Evidence: docs/report-volunteers-mp-sync-1029.md. Source commit/push remains
authorized. Preserve unrelated release-document changes and private binaries,
assets, raw RE, logs and saves. No memory writes. Next pitfall P85.
