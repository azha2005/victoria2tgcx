# NEXT - what comes next

2026-10-29. **Plain Volunteers 0.0.12 SP beta ZIP ready for the user to send.**
User accepts CreatineCore 6.36 setup as working well enough, requests ZIP and
explicitly says no BAT/command files or similar, like ATCU/PRU. No installer,
external launcher or scripts are included. No public upload requested.

## Current verified state

- Final ZIP: work/volunteers-share-0012/final/Volunteers-0.0.12-SP-beta.zip.
  421453 bytes, 20 files. SHA256:
  9481348cc4aeef0a09b5696f4a8363f4a9c8f6a037a46e9afb1a3f71e0634ae8.
- DLL and 12 supporting UI/marker files match the accepted 0.0.12 copy. Only
  display name and explicit prototype-mode config differ in the original payload.
  Eight native-style derived UI assets remain outside git. SP only; MP refused.
- Normal launcher: 00 - Volunteers loader data, recipient's TGC 2.5.2,
  ZZ - Volunteers (SP beta). Carrier is additive, no profile/replace override.
  Core 6.36 is a separate README-linked prerequisite, not included in the ZIP.
- Two package tests pass, deterministic bytes/CRC/manifest verified. Final 13
  plugin/UI files reread against the installed copy. No runtime code changed.
- Actual latest installed logs: core 6.36 and performance/Volunteers admitted,
  all volunteer install gates true, probes off; 140 ticks with active mission.
  User accepts general operation/performance, not full lifecycle/MP acceptance.
- Existing loader backup: work/volunteers-loader-636/before-636. No game launch,
  input or installed-file changes during this packaging task.

## Remaining cards

| Card | Status | Step-by-step instructions |
|---|---|---|
| VOL-SHARE | ZIP ready; recipient handoff/ordinary-launcher check next | docs/instructions-volunteers-share.md |
| VOL-LOADER / VOL-OPT | 6.36 startup/user general result accepted; quantitative timing open | docs/instructions-volunteers-loader-636.md |
| VOL-SIEGE / VOL-CONTROLS | Earlier progress accepted; capture credit/full vision incomplete | Share steps 4..6 and docs/instructions-volunteers-siege.md |
| VOL-PROTOTYPE | Return/loss/load/supply acceptance incomplete | docs/instructions-volunteers-prototype.md |
| VOL-OOS / VOL-MP | Send refused; synchronized transport absent | docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-share-1029.md. User handles desktop testing and
sharing. Source-only commit/push remains authorized; preserve unrelated release
documents and exclude private art, binaries, source exports and logs. No subagents
or memory edits. Next pitfall P79.
