# VOL-MP: actual0.0.16 prototype acceptance

2026-10-30. User has taken over game operation. Read the new report and P87/P88.
Do not relaunch, send desktop input or replace a live DLL while user is testing.

1. Preserve0.0.12 installed rollback and all earlier ZIPs. Current interactive
   test root is work/volunteers-mp-0016/game-host, profile
   TGC252_Volunteers_MP0016_Host. It already contains the final0.0.16 DLL.
2. Both players use the same plain final ZIP, reference EXE, TGC2.5.2 and
   Core6.36/settings. Close normally before installing on another copy. Read
   README.txt; no scripts/installer needed. Select loader carrier, TGC, then
   Volunteers (MP prototype), and no older Volunteers descriptor. Mode=mp.
3. Host/join a disposable common save as distinct countries. Start with two
   players. Retain original pre-Send save. Do not claim hotjoin/spectators or
   shared-country control accepted. Unpause for native HELLO execution.
4. BOTH logs require mp_gameplay_hello for BOTH roster countries with accepted=true
   and ready=true. queued=true alone is not proof. If missing, preserve logs and
   inspect exact human-list +18/+1C/+20, nodes+1C/+20/+24/+38/+3C, controller table
   +AEC/+AF0, manager+54 and local country+B60. Never loosen guessed mapping.
   Treat fields as S (static) until runtime cross-checked on host and client.
5. Keep donor neutral, recipient at war and nonhostile. Open Diplomacy and check
   Send volunteers visibility/placement after reopen, background/restore and long
   sessions. UI signature admission can survive unrelated gameplay refusal.
6. Prepare an idle native army with <=5 brigades and a native land route. Select
   EVERY member. Partial Send is refused in MP; use native Split first. Verify
   same army ID/owner/leader, normal player command, supply and reinforcement.
7. Send from CLIENT first, then host in a separate run. At matching dates require
   exact journal heads/hashes and mp_policy_active digests on both peers. Keep
   windows closed on one peer to test UI-independent policy. Follow donor-owned
   movement into enemy territory, battles, siege progress and occupation credit.
8. Test Recall, recipient peace, donor entry into war, loss, return/history cleanup,
   two simultaneous donors and swapped host/client countries. Confirm only one
   donor client emits each derived movement; remote peers must never duplicate it.
9. Save/exit/reload. Fresh current-load HELLOs must be required; stored markers
   cannot unlock. Exact receipts revalidate saved policy; incomplete/foreign
   membership refuses. No new army-ID allocation or Create/Transfer replay in MP.
   Test missing plugin/different fingerprint, changed roster and disconnect.
   These are negative tests, not validated recovery modes.
10. Compare same-save game-day timings, idle versus active and closed versus open
    UI. Keep state/RNG probes off. Native startup crash/OOS logs remain on. Use
    first OOS date, exact common save and BOTH logs; policy digest isn't engine CRC.
11. Collect stable evidence via tools/collect_volunteers_oos.py. Existing mirror
    comparator is diagnostic only; inspect mp_gameplay_hello and mp_policy_active
    lines separately. Reproduce a baseline without Volunteers and locate first
    divergent command/native state. Do not mask CRC or alter maintenance/RNG.
12. After any new code, run tools/build_volunteers.ps1, tools/check_exe.py,
    tools/test_volunteers.py, tools/test_volunteers_mp.py and collector tests.
    Build a NEW output with prepare_volunteers_mp.py --gameplay; verify its DLL
    hash, mode, CRC/manifest and no script payload. No live install/rebuild swap.
13. New report for actual results; never rewrite immutable reports as passed.
    Archive NEXT verbatim, update next steps/statuses and commit/push owned source.
    No public binary upload, third-party message, new subagent or memory update.
