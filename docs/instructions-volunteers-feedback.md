# VOL-FEEDBACK: 0.0.13 focused player retest

2026-10-29. The user supplies tester feedback; specific many-send symptom is
unknown and distance complaint was retracted. Read report and P79..P82. User and
tester retain game/desktop input. No direct message or public upload authorized.

1. Use work/volunteers-0013/final-r2/Volunteers-0.0.13-SP-beta.zip. It is a plain
   20-file mod archive without EXE/setup/BAT/CMD/PowerShell/scripts, like ATCU/PRU.
   Back up the previous mod, exit normally, then replace the descriptor/package
   and carrier in a disposable game copy. Existing local accepted0.0.12 is unchanged.
2. Use the normal launcher: 00 carrier, TGC2.5.2, ZZ Volunteers. Core6.36 remains
   a separate prerequisite. Keep state/RNG/performance debug probes and the army
   window lag option off. Verify build0.0.13 and all admitted native hooks in logs.
3. Open a save made under0.0.12, including one with returned/lost/serving records.
   Verify nonzero saved journal loads and revalidates, active missions aren't
   replayed, old daily date does not block processing, and terminal rows don't
   consume brigade slots. Save, exit normally, restart and repeat. Capture exact
   specific unavailable message and corresponding journal/session/load log if it
   fails. Do not infer MP detection from generic unavailability.
4. Terminal missions should be hidden initially. Show/Hide history toggles them;
   Clear history can remove only completed records with no retained entitlement.
   Check terminal wartime sight remains, known peace prunes terminal entries and
   clears their sight, new missions use new IDs and can still be sent. Test recall
   selection after hidden/removed rows and save/load. Don't clear a live mission.
5. Choose a recipient at war. Idle valid donor armies should appear only after
   a native route succeeds. Search runs in small batches on existing updates and
   ticks; unpause briefly or interact while Checking reachable armies appears.
   Check pause responsiveness and many-army cost, change recipient/reopen, and
   ensure a long unreachable candidate doesn't block another reachable army.
   Test a recipient whose first province is unreachable but another is reachable.
   Send must recheck a changed route; preserve native access and land-only rules.
6. Retest recipient-controlled sight, recipient knowledge in foreign fronts and
   actual volunteer surroundings. Capture vision_runs/grants/refused/empty and
   vision_skipped_provinces. A skipped dummy must not cancel other valid sight;
   unknown data must not reveal the world. No simulation visibility/relations
   writes or alliance creation. Native display appearance still needs acceptance.
7. Reproduce repeated return/loss/send through several wars. On capacity refusal,
   existing missions must survive and the next intent/sequence must not be partly
   accepted. The concrete journal-limit regression is not proof of the unspecified
   tester failure; obtain actual symptom/date/save/log before claiming that fixed.
8. Retest normal command, battle, siege start/join/progress/completed credit and
   donor supply/ownership, recall/peace/donor-war/loss. Continue existing prototype
   instructions for uncovered acceptance. Keep the global brigade limit; no new
   one-war or country-distance policy was established by this continuation.
9. MP: current Send/Recall remain refused. Both peers may use mode=preview only
   for a read-only load/UI/OOS baseline, with identical EXE/plugin/settings and a
   disposable common save. Follow instructions-volunteers-oos.md for paired
   evidence and future synchronized transport. No force-enable-MP workaround.
10. Source checks: tools/build_volunteers.ps1, python tools/test_volunteers.py,
    python tools/test_volunteers_share.py, python tools/test_collect_volunteers_oos.py,
    python tools/check_exe.py. New share ZIP builds from the pinned candidate in
    prepare_volunteers_share.py. Keep built binaries/art/raw exports private.
11. Archive NEXT, update statuses, preserve immutable reports and write next exact
    steps. Source commit/push remains authorized; no public binary release implied.
