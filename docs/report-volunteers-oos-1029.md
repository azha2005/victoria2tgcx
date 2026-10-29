# Volunteers OOS investigation

2026-10-29. Source inspection, not a reproduced network diagnosis.

## Concrete divergence mechanism

Original reference preserved at work/volunteermaxioten/V2Mod_Template.cpp:

- Lines13 and16 describe local/SP limitations, but the runtime does not enforce
  a proved session gate before Send/DailyWork.
- Send at449..475 posts native Create/Transfer, then writes sender-local g_recs.
  SaveState at255..270 persists that registry in a local text file.
- HookReconcile at521..530 returns early for a local FindByArmy match, bypassing
  the native FUN_005C8330/RVA1C8330 maintenance routine. Another peer without
  that local record runs the original. This can diverge pop/regiment handling.
- DailyWork at904..945 increments local record ages/peace days, may issue Recall
  and reads local orders. Calling this on every peer can also duplicate commands
  when local files happen to match; calling it only on the sender does not make
  its local hooks deterministic on the other peers.
- HookArmyControl at507..515 also depends on local records/player identity. It
  does not establish native command issuer validation or recipient-AI exclusion.
- Local ID counter allocation at401..410 occurs before network execution. A
  synchronized membership command is insufficient to synchronize these policies.

The reported screenshot establishes an OOS notification, not which state first
diverged. No same-save host/client artifacts or logs were supplied in this pass.
No claim is made that changing a maintenance setting repairs the specific game.

## Implemented response

Our donor-owned design retains ordinary native maintenance and command ownership.
No HookReconcile bypass or local orders-file transport is used. A strict local-
transport/session/epoch gate refuses MP/unknown sending. Registry checkpoints use
CSetVariableCommand; commit-last heads are observed at native execution and
logged with country/generation/SHA256. The collector compares binaries/settings,
local records and journal digests. These diagnostics do not implement MP.

To actually support MP, every simulation peer must construct the same scoped
registry from synchronized native execution, validate issuer/sequence against
the same campaign, preserve identical policy/config/code, and avoid each peer
independently emitting automatic return commands. Owner-state and CRC masking,
disabling checksum verification or resynchronizing only the visible army cannot
solve an unknown underlying divergence. No such workaround was added.

## Evidence tool

`tools/collect_volunteers_oos.py collect <game> <new work/output> --profile <profile>
[--save <save>]` gathers private logs/settings, hashes binaries/descriptors and
optionally copies one save. Binary executables/DLLs are hash-only. Read-only
source capture verifies stable hashes, caps each copied file at32MiB and refuses
outputs outside work/. A file that changes during capture is reported as failure.

`python tools/collect_volunteers_oos.py compare <host output> <client output>`
reports differing artifacts, unmatched/conflicting journal generations and digest
mismatches for common commits. Matching output does not prove deterministic
simulation; the inventory explicitly does not hash the complete base mod tree.

One read-only collection from the isolated E: copy completed:16 files, no skipped
files, under work/volunteers-0006/oos-e-copy-evidence. It is a local installation
snapshot, not host/client OOS evidence. An asynchronous question requested paired
log/save locations and whether the OOS was immediate or delayed; no answer was
available at this checkpoint. Continue implementation independently while that
information remains missing. See instructions-volunteers-oos.md.
