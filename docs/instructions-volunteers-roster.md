# VOL-MP-ROSTER: resolve the silent pre-HELLO gate

2026-10-30. User controls game/desktop. Do not install/relaunch or send input.
Read report-volunteers-roster-1030.md and P90 before changing admission rules.

1. Preserve0.0.17/0.0.16 ZIPs and original uploaded logs. Both peers install the
   same0.0.18 plain ZIP only after normal close, using disposable saves/copies.
   Keep intended other Core plugins/settings matched. Do not disable their hooks
   or MP_HOST_HUMAN_SYNC just to bypass admission. No command scripts needed.
2. Launcher order: loader carrier, TGC2.5.2, Volunteers MP prototype. mode=mp.
   Collect EACH Logs/tgcx_volunteers_<PID>.log from STARTUP, not the mixed legacy
   tgcx_volunteers.log. Record PID, host/client role and local country separately.
   Separate local game directories and save profiles remain preferred.
3. Verify0.0.18, full native/module/load admission and mp_gameplay_initialized
   enabled=true. Unpause one or two days. Expect either mp_roster_accepted then
   HELLO post/execute, or mp_roster_refused with exact stage and raw values.
   No journal mirror/bootstrap hash substitutes for either of those observations.
4. If refused, capture the exact stage on BOTH peers. Stages distinguish native
   queue, manager/source range, controller/world vector bounds, human-list size/
   backlink/cycle/end, human/controller set, local source-country/local flag,
   country identity, duplicates and torn capture. Do not loosen the identified
   guard until instructions/runtime establish what its values mean.
5. Static layout: FUN_00680450 constructs world ADC/AE0 country pointer vector
   and AEC/AF0 human flags together. Global country-manager size is separate.
   FUN_0068B870 walks world countries and compares +20 country indices against
   human nodes. Capture requires matching active global/world pointers plus
   each country index/tag, not global/world length equality. Preserve every
   identity/complete-human-set/source and local control check.
6. If source/flag/list stages differ from expectations, read-only targeted Ghidra
   exports plus fresh user runtime trace are required. Check exact EXE first.
   Private exports under work/volunteers-mp-0018/research verify64A460 copies
   player fields,68EFC0 removes nodes and7297A0 constructs controller commands.
   These are S evidence, not proof of host/client runtime source mapping.
7. On valid roster, require native HELLO receipts for EVERY peer with matching
   fingerprint and ready=true on BOTH logs. Failed capture withdraws prior receipt
   authority; successful recapture needs fresh HELLOs. Wrong source/unknown peer/
   bad fingerprint never grants policy. Do not replay interrupted unit work.
8. Continue instructions-volunteers-coexistence.md steps6..10 with all plugins
   enabled, then MP-gameplay steps5..10. Whole idle army<=5, native land route,
   donor ownership/control, client Send, battle/siege/credit/Recall/peace return,
   two donors, closed UI, host/client swap and save/reload all need paired proof.
9. Compare the same save/load/date, journal commit and policy digests with native
   OOS evidence. Later SP deployments in a mixed log do not prove MP success.
   Preserve first OOS date and full Core logs. Never mask CRC or maintenance/RNG.
10. Measure game-day speed with intended plugin/settings. Keep probes off and
    retain ordinary OOS/crash logs. Roster diagnostics only log changed failures/
    accepted bindings; no new frame-by-frame world dump.
11. Collector recognizes PID logs, ignores legacy mirror for trace parsing when
    canonical files exist, and still copies both. Select the matching PID/run/load;
    it does not infer a valid paired test across many unrelated canonical files.
    Existing comparator handles diagnostic mirror, not actual gameplay admission.
12. After changes run build_volunteers.ps1, check_exe.py, test_volunteers.py,
    test_volunteers_mp.py and test_collect_volunteers_oos.py. Generate NEW plain
    ZIP via prepare_volunteers_mp.py --gameplay; verify DLL/config/manifest/CRC.
    New report, NEXT archive, statuses/next instructions; commit/push owned source.
    No public binaries, peer messages, new subagents or memory updates.
