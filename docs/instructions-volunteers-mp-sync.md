# VOL-MP-SYNC: paired transport test and next implementation

2026-10-29. Read report-volunteers-mp-sync-1029.md and pitfalls P12/P69/P83/P84.
The user asks to attempt MP/OOS support.0.0.15 is a diagnostic transport build,
not MP volunteers or an OOS fix. The user/tester retain game/desktop operation.

1. Keep the prior SP ZIP and accepted0.0.12 installation as rollbacks. Use separate
   disposable game copies, reference EXE, TGC2.5.2 and CreatineCore6.36. Match full
   mod/loader/settings on both peers. Do not replace the only campaign save.
2. With games closed normally, extract work/volunteers-mp-0015/final/Volunteers-0.0.15-MP-sync-probe.zip beside
   v2game.exe. Normal launcher selects00 loader data, TGC, ZZ MP sync probe;
   never both old and new Volunteers descriptors. No BAT/CMD/scripts or external
   launcher needed. README includes exact loader/performance settings.
3. Preserve a common pre-test save and run a baseline without Volunteers first.
   Host/join a two-player MP session, each controlling a different country.
   mode=mp_probe is already set. Verify0.0.15, signature admission, load fence
   and native journal observer in Logs/tgcx_volunteers.log. Offline check_exe
   covers107 scoped records; no failed gate may be overridden.
4. Test the Send volunteers entry after opening/closing Diplomacy, switching
   countries and background/restore. The0.0.15 native entry corrections are present
   even though MP Send/Recall remain unavailable. Then unpause briefly. Expect one mp_probe_post attempt per local country/load;
   queued=true alone is insufficient. BOTH logs need mp_probe_execute receipts
   for BOTH countries with the same fingerprint/sequence and matches_local=true.
   Capture exact native source metadata; do not interpret it as country authority.
5. Let bootstrap complete (up to16 country slots/day). Preserve logs with
   mp_sync_sample fields at matching dates: bootstrap_complete=true, refused=0,
   missed=0. Empty-journal agreement does not test volunteer policy or engine CRC.
6. Save/exit normally. User provides both logs, native loader OOS/error logs,
   exact pre-test save and first OOS date if applicable. Repeat host/client
   country swap, save/reload, reconnect and missing-plugin/fingerprint negative
   cases on disposable copies. No live-game DLL replacement or guard removal.
7. Source-side evidence collection: use tools/collect_volunteers_oos.py collect
   once per stable peer capture under distinct work/ outputs, then compare.
   It hashes binaries and copies bounded logs; never send messages or public
   uploads. Preserve receipt differences, ambiguous dates and the first clean
   mirror mismatch. Missing evidence remains unknown, not success.
8. Native RE next: inspect server/client receive from exported A61780/A66D80,
   prove sender-source to country/peer membership and ordered command execution
   boundary. Verify each field in instructions before adding RVAs; run check_exe.
   Use read-only targeted exports under work/, never commit raw game material.
9. Implement shared mission activation from committed native execution on all
   peers, plus one authority for derived orders. Keep UI out of policy activation;
   cover replay/conflict, overlapping donors, ID allocation, disconnect, late join,
   save/load and partial queues with deterministic two-replica fixtures first.
   Do not turn a diagnostic mirror/fingerprint into a SimulationPermit.
10. Only after those gates are established expose an explicit MP gameplay test
    mode. Reproduce Send/control/battle/siege/return against paired traces, then
    validate the original OOS trigger. Retain native maintenance, donor ownership
    and native command writes. No CRC masking or per-peer automatic duplicate orders.
11. Source checks: tools/build_volunteers.ps1; tools/test_volunteers.py;
    tools/test_volunteers_mp.py; tools/test_collect_volunteers_oos.py;
    tools/test_volunteers_share.py; tools/check_exe.py. Keep binaries/art/raw exports
    private. Rebuild the plain MP ZIP with tools/prepare_volunteers_mp.py.
12. Archive the current NEXT verbatim, update status/index and next card steps.
    Source commit/push remains authorized; no public binary release or peer message.
