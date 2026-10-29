# NEXT - what comes next

2026-10-29. **User/tester retests Volunteers0.0.13 SP feedback candidate.**
Tester requests recipient sight, cleanup of returned/lost history, valid-army
selection and load/session errors; distance complaint later retracted. Specific
many-send symptom unknown (user answered idk). Tester also asks for MP, but no
synchronized volunteer transport exists. Keep Send refused; preview is read-only.

## Current verified state

- Plain ZIP: work/volunteers-0013/final-r2/Volunteers-0.0.13-SP-beta.zip.
  443954 bytes,20 files; no setup/EXE/BAT/CMD/PowerShell/scripts. SHA256:
  649454bb0a9d630b6f68d621194e6e39c160c607edd147e6a2d42a1a513d1f1d.
- DLL SHA256 156a02d2ff2551bf90d4acaae0dd2d7c87d73149ee1567e01bc611e068b9721b.
  Actual local game stays accepted0.0.12/CreatineCore6.36. No installation, game
  launch, desktop input or save edits in this task. New candidate untested in game.
- Source fixes: retry failed journal binding, reset restored daily/page state;
  hidden/clearable terminal history, prune expired records after peace, saved
  terminal phase excluded from cap; sequence-based serial high-water and atomic
  journal-size refusal preserve existing missions after repeated sends.
- Route-filtered idle-army chooser uses bounded/fair native queries and multiple
  recipient land targets. Cached answers are UI-only; fresh Send/session/epoch/
  membership/owner/route gates retained. Search progresses on updates/ticks/clicks;
  reopening refreshes newly available access. No country/distance cap or teleport.
- Sight keeps independent valid grants after a malformed province, reads native
  recipient-controller/rebel-owner sight and reports skipped provinces. No native
  simulation visibility/alliance writes. Full runtime sharing still unverified.
-51 Rust tests pass, two explicit timing benchmarks ignored; strict Clippy,
  format/release, six package tests, two share tests, one collector test and96
  native signatures pass. Updated native-art preview inspected, own SVG updated.
- Specific unavailable reasons distinguish actual MP from unknown session,
  unbound campaign and queue/hook/persistence gates. Tester root cause remains
  unconfirmed without log/save. Do not claim all reload errors or OOS fixed.

## Remaining cards

| Card | Status | Step-by-step instructions |
|---|---|---|
| VOL-FEEDBACK / VOL-SHARE |0.0.13 ZIP ready; tester retest required | docs/instructions-volunteers-feedback.md steps1..7 |
| VOL-CONTROLS / VOL-SIEGE / VOL-OPT | New route/history/sight controls need game/performance checks | Feedback steps4..8; docs/instructions-volunteers-prototype.md |
| VOL-PROTOTYPE | Save/load, return/loss, capture/accounting acceptance incomplete | Feedback steps3..8 and existing prototype card |
| VOL-OOS / VOL-MP | MP Send refused; preview baseline/paired evidence only | Feedback step9 and docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-feedback-1029.md. User retains testing/sharing.
Source commit/push remains authorized. Preserve unrelated release-document work,
private art/builds/raw exports/logs and immutable reports. No new subagents or
memory writes. Next pitfall P83.
