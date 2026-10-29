# VOL-SEND-0008: install and verify the Send/lag correction

2026-10-29. Read NEXT.md, report-volunteers-send-assembly-1029.md and pitfalls
P31/P45/P51/P64..P71.0.0.8 was installed after confirmed normal exit. All15
mod payloads and nine protected-file checks pass;0.0.7 backup is preserved.
Installation steps1..3 are complete; continue from step4. For an existing successful
installation, do not rerun the one-use updater or overwrite its backup.
See report-volunteers-install-0008-1029.md.

1. Preserve the running test/save/log. Do not kill it or overwrite its loaded DLL.
   User exits normally. Confirm Get-Process v2game has no result before installing.
2. Run python tools/check_exe.py. Use the exact stage
   work/volunteers-0008/Volunteers-0.0.8-SP-send-fix-private and its manifest.
   If source changed, rerun tools/build_volunteers.ps1 and tools/test_volunteers.py,
   stage to a NEW path, and update the updater's STAGE; do not silently overwrite ZIPs.
3. Run python work/volunteers-0008/update_e_copy.py with authorized filesystem access.
   It refuses a running game, backs up our0.0.7 package/log, installs our payload only,
   checks installed hashes and nine protected files, and records install-result.json.
   Do not modify TGC data or main installations. No duplicate plugin loaders.
4. Launch E:/Victoria-II-Volunteers-0004/tgcx_start_prototype.cmd. For a clean test use
   a fresh campaign or pre-failed-Send save. Existing incomplete0.0.7 mission records
   are review-only; do not replay Create/Transfer or erase records to force admission.
5. Peaceful donor, nonhostile at-war recipient, idle land army near a verified route.
   Select all members of a one-brigade army. Send and unpause. Verify log reuse=true,
   same army identity and owner, complete receipt, then native movement to recipient.
6. Repeat with a whole five-brigade army. No four-plus-one split; leader and source
   identity survive. Then select fewer than all from a larger army: new donor stack,
   exact selected membership and nonempty source. Inspect queue/receipt failures.
7. Advance at least30 days with unchanged mission state. Journal head should remain
   stable after its initial intent/assembly/movement transitions, rather than advance
   daily. Compare actual game responsiveness; fixtures alone do not establish speed.
8. Save/reload a serving or just-assembling mission. Complete receipt restores with
   no Create/Transfer replay or repeated assembly rejection. Verify normal player
   selection/orders, battle creation/joining and recipient AI exclusion. Follow full
   instructions-volunteers-prototype.md for return/loss/vision/supply/exit coverage.
9. MP is unsupported. Continue instructions-volunteers-oos.md only with paired peer
   logs/saves; never suppress OOS checks or claim SP refusal implements MP.
10. Record actual outcomes, update NEXT/ROADMAP/SUBAGENTS/docs index, archive NEXT,
    and write steps for remaining cards. No successful game/lag result is presumed.
