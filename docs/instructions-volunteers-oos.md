# VOL-OOS: diagnose the reported divergence

2026-10-29 current follow-up: the user explicitly requests MP/OOS work.
The0.0.14 native network probe is ready for paired testing under
instructions-volunteers-mp-sync.md. Only this restricted diagnostic command is
admitted in MP; volunteer Send/Recall/scopes remain refused. The original OOS
remains unreproduced. Historical steps below retain their investigative value.

2026-10-29. Source finding: report-volunteers-oos-1029.md; pitfall P69.

1. Preserve original Maxioten source/state/config. Do not change a live MP game,
   disable CRC verification or claim the notification's cause from the screenshot.
2. Resolve the pending human answer about both peers' log/save paths and whether
   OOS follows Send immediately or after daily maintenance. Independent SP coding
   can continue without that answer. The specific OOS needs paired evidence.
3. From stable files, run tools/collect_volunteers_oos.py collect for each peer,
   using distinct new outputs under work/. Include the exact same pre-Send save
   if available, game/profile/V2DLL logs, current DLL/settings and local state file.
   The script hashes DLL/EXE instead of redistributing them. Preserve both originals.
4. Compare the two evidence folders. Check actual executable/plugin/settings/
   descriptor differences and local-registry records; compare common journal
   country/generation hashes. A missing generation or digest mismatch is a lead,
   not proof of which earlier engine field diverged. Complete mod-tree checksum
   matching remains a separate check, not covered by this small collector.
5. Prepare a minimal two-copy/two-peer reproduction from one disposable save:
   baseline without plugin, observer/preview only, then the specific original
   volunteer action. Record exact date and first mismatch, host versus client
   initiation and first daily tick. Do not enable our unsupported MP Send.
6. Prove/refute the local-maintenance bypass mechanism by capturing FindByArmy
   and native maintenance execution for the same army on both peers. Removing
   the bypass avoids that mechanism, but recipient-owned pop reconciliation
   remains a separate gameplay bug. Keep our donor-owned native maintenance.
7. Implement MP only after establishing synchronized registry construction,
   issuer/sequence validation and a single synchronized authority for automatic
   commands. Matching DLLs, a sidecar, per-peer polling or a digest alone are
   insufficient. Verify save/load/late-join and rejection paths on both peers.
8. Record exact artifacts and validation strength in a new report. Do not label
   OOS fixed until the failing reproduction passes with matching simulation.
