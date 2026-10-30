# VOL-COEXIST: compatibility and MP acceptance

2026-10-30. User controls the desktop/game. No automatic installation or launch.
Read report-volunteers-coexistence-1030.md and pitfalls P87..P89.

1. Preserve0.0.16 ZIP and accepted0.0.12 copy.0.0.17 final ZIP lives under
   work/volunteers-mp-0017/final. It changes only our mod payload, not Core/TGC.
   Both peers need the same ZIP. Normal close before replacing files.
2. Keep the intended yearly cache, performance and Baltic plugins loaded on both
   peers with matching binaries/settings. Do not change MP_HOST_HUMAN_SYNC just
   to avoid a signature check; preserve its existing engine behavior. Independent
   game directories and save profiles are required for two local instances.
3. Normal launcher order: loader carrier, TGC2.5.2, Volunteers MP prototype.
   Check version0.0.17, mode=mp, matching native module gates and all scoped hooks.
   A native_hook_chain_verified line must verify the transparent role chain;
   native_signature_compatible then records admission. Unpatched original also
   remains valid. Other plugins' presence alone never refuses admission.
4. If still refused, collect BOTH full Volunteers and Core startup logs, exact
   plugin list/configurations and safe file hashes. New refusal includes owner
   path/RVA/hash and stage. Do not infer a version from the footer or bypass the
   failed guard. A controlled optional-plugin A/B run is diagnostic evidence only,
   not a permanent compatibility requirement. Inspect the actual failing contract.
5. For new contracts inspect targeted native/loader code first, after check_exe.
   Never accept any E9 as compatible. Existing matcher recognizes only complete
   PUSHFD/PUSHAD, private atomic marker, POPAD/POPFD and original trampoline
   contracts with bounded direct-JMP veneers. Arbitrary native behavior is unknown.
   Keep permissions/module bounds, exact original continuation and negative tests.
6. Host/join a clean common save with plugins enabled. Both logs must receive
   current-load HELLOs for every active human country and ready=true before Send.
   Then test whole army <=5 brigades with native land route. Record membership,
   donor ownership/control, client-originated Send and exact committed journal.
7. Close diplomacy on one peer; test two donors, battle, siege, occupation credit,
   Recall and automatic peace return. Compare native journal/policy digests and
   engine OOS traces at the same save/date. Validate the original failing case.
8. Save/reload, host/client swap, missing/different build and roster changes.
   Save markers alone never admit; no unit-command replay. Preserve uncertain
   partial work for review. Hotjoin/spectator/shared-country recovery unaccepted.
9. Measure same-save game-day speed with matching plugin/settings. This change
   runs the matcher only at startup, no new hot-callback scanning. Do not claim
   cache/performance coexistence or speed improvement from source fixtures.
10. Follow instructions-volunteers-mp-gameplay.md steps9..13 for paired evidence,
    collector limits and acceptance. Policy hash isn't engine CRC. Never mask CRC,
    disable native maintenance/RNG or claim the prior OOS fixed without reproduction.
11. After source changes: build_volunteers.ps1, check_exe.py, test_volunteers.py,
    test_volunteers_mp.py. New plain ZIP via prepare_volunteers_mp.py --gameplay;
    verify version/DLL/config/CRC/manifest and no scripts. No live swap.
12. New immutable report, archive NEXT verbatim, update next statuses/instructions,
    commit/push only owned source. No public binaries, peer messages or memory writes.
