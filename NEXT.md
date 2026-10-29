# NEXT - what comes next

2026-10-29. **User tests CreatineCore 6.36 with unchanged Volunteers 0.0.12.**
User reports continued game-day lag, asks for a newer V2DLL, then explicitly
requests loader replacement. The isolated E: copy now has official CreatineCore
6.36 (the renamed V2DLL). No game launch/input or save edits were performed.

## Current verified state

- Target E:/Victoria-II-Volunteers-0004 only. Official release ZIP digest matches
  GitHub. New dinput8 proxy loads z_CreatineCore/creatinecore.dll; the separate
  ccore_performance_patch.dll is installed with migrated performance options.
- State/RNG probes and verbose performance logging off; FIX_ARMY_WINDOW_LAG off
  per upstream's incomplete-stutter-fix note. Old gameplay options retained;
  new opt-in features disabled. Optional personal-patches plugin omitted.
- All 16412 TGC files, 15 Volunteers files and ten further protected files hash
  unchanged. Root dinput8 intentionally replaced. Old V2DLL tree remains inactive.
- Same three launch shortcuts; scripts now select the additive loader-data
  carrier FIRST, then TGC, then Volunteers. The physical defines_ccore file goes
  in our carrier, preventing upstream rename/append inside TGC. Profile descriptor
  unchanged; actual save-list/profile behavior still needs the player check.
- Four migration tests pass; three launch scripts parse. Own x86 probe passes
  six proxy exports, non-game host guard, pinned upstream 28-byte API1 layout and
  inactive Volunteers admission. EXE and 96 native signatures pass. This is NOT
  actual game hook initialization or a proven speed improvement.
- Rollback: work/volunteers-loader-636/before-636; rollback.json lists replaced
  and new paths. Restore only after normal exit. Do not re-enable older probes.
- Latest pre-upgrade 0.0.12 log: 56 ticks, probes confirmed off, daily-plugin
  work 5814 us total; vision 3781 us/21 calls. Timings exclude native originals
  and assembly; hot callbacks sampled. Continued lag is still unresolved.

## Remaining cards

| Card | Status | Step-by-step instructions |
|---|---|---|
| VOL-LOADER / VOL-OPT | 6.36 installed; startup/hooks/game-day timing pending | docs/instructions-volunteers-loader-636.md |
| VOL-SIEGE / VOL-CONTROLS | Earlier progress accepted; capture credit/full vision incomplete | Loader steps 4..7 and docs/instructions-volunteers-siege.md |
| VOL-PROTOTYPE | Return/loss/load/supply acceptance incomplete | docs/instructions-volunteers-prototype.md |
| VOL-OOS / VOL-MP | Send refused; synchronized transport absent | docs/instructions-volunteers-oos.md |

Evidence: docs/report-volunteers-loader-636-1029.md and private manifests/probe/
release sources in work/volunteers-loader-636. User retains desktop testing.
Source commit/push remains authorized; exclude upstream code/art/binaries and
preserve unrelated release-document changes. No subagents or memory edits.
