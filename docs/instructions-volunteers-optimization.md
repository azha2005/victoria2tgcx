# VOL-OPT:0.0.11 comparison and follow-up

2026-10-29.0.0.11 installed;0.0.10 siege progress accepted by user. Source commit
and push explicitly authorized. Read report-volunteers-optimization-1029.md and
pitfalls P70..P76. Keep working0.0.10 backup work/volunteers-0011/before-0011.

1. Use E:/Victoria-II-Volunteers-0004/tgcx_start_prototype.cmd for the player check.
   One game only; don't replace its loaded DLL or alter main/TGC files. No desktop
   input on the user's behalf. Package/default observe differs from prototype launcher.
2. Compare same saved campaign/date, speed, camera/fog and panel state against the
   preserved0.0.10. Measure wall time for a fixed30-day range after warm-up. Repeat
   several times and retain logs. Benchmark helper ratios do not predict game FPS.
3. Check closed and open panels. The chooser must still refresh all recipients/
   armies and show names; opening/clicking is full snapshot, closed polling compact.
4. Retest sending, ordinary donor movement, battle, siege start/join/progress and
   completed occupation credit to recipient. Donor stays neutral and commands its
   troops; native supply/reinforcement/flag/rank accounting unchanged. Do not mark
   capture credit accepted from a progress bar alone.
5. Save/reload a serving/sieging mission and switch player if supported. Verify full
   rebind discovers newly loaded recipients; no command replay or stale membership.
   Recall/recipient peace/donor war must still queue the same native return orders.
6. For missing vision use vision_refused/vision_empty/grants and a fresh same-date
   save/log, following instructions-volunteers-controls.md. Zero grants remain open;
   this optimization must not hide/force the renderer or province+174 simulation array.
7. Continue instructions-volunteers-prototype.md for the remaining lifecycle cases.
   MP/OOS remains instructions-volunteers-oos.md: no MP transport, paired evidence
   required. Never weaken mode, thread, epoch, receipt or native siege gates.
8. For source changes run tools/build_volunteers.ps1 and tools/test_volunteers.py.
   Explicit benchmark from rust/volunteers:
   cargo test --release --offline perf_tests::helper_benchmark -- --ignored --nocapture --test-threads=1
   Fixtures own their memory; do not turn them into external game mutation tools.
9. Preserve immutable reports/rollbacks, archive NEXT, update statuses and next-card
   steps. User authorized source commit/push; no game assets or raw decompiler output.
