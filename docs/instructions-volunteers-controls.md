# VOL-CONTROLS:0.0.9 player check and follow-up

2026-10-29.0.0.9 installed; game launched for a short check, user takes over.
Read report-volunteers-controls-1029.md and pitfalls P31/P64..P74. No game result
is established merely by successful hooks/fixtures. Preserve donor ownership.

1. Use E:/Victoria-II-Volunteers-0004/tgcx_start_prototype.cmd if the game is closed;
   one game only. Current0.0.9 startup log confirms all six scoped hooks admitted.
   Rollback0.0.8 is work/volunteers-0009/before-0009. Do not install over a running DLL.
2. Fresh campaign or pre-send save: neutral donor, at-war nonhostile recipient,
   idle funded army with organization recorded before Send and a verified land route.
   Old exiled troops may need normal return to accessible territory; never force flags.
3. Open/close/reopen volunteer panes. Send entry must remain in Diplomacy's native
   action slot while panes are open; it must hide with Diplomacy, then reopen correctly.
4. Send a complete army, unpause, verify reuse=true and serving. Observe recipient's
   foreign fronts and the volunteer's current/adjacent provinces. Select and issue
   native movement orders as donor, including after closing Diplomacy. Preserve
   unrelated countries' fog and do not write province+174 to make selection work.
5. Compare organization, supply and reinforcement before/after entering foreign land;
   record exile byte+164, retreat+104 and combat+74 READ ONLY if diagnosis is needed.
   Confirm initial combat and joining an existing battle against the recipient's
   enemy, donor stays neutral, recipient AI cannot command, native accounting holds.
6. Capture Logs/tgcx_volunteers.log plus same-date save/screenshots. scope_counters
   should show country_bypass high versus country_target, exile_granted on relevant
   permission refusals and vision grants. If thread_refusals rises, establish actual
   call/thread ownership before widening admission. Never assume another thread safe.
7. Measure time for identical game-day ranges with same base/save/speed, visibility
   and mod mode. Stable checkpoint head was verified in0.0.8; persistent lag must not
   be blamed on date-only journaling again without evidence. Investigate remaining
   scoped pair/access work using counters, not broad bypasses of native validation.
8. Save/reload, return/peace, loss and vision lifetime: continue
   instructions-volunteers-prototype.md. MP/OOS: instructions-volunteers-oos.md,
   paired same-save evidence only. No MP transport exists; sending remains refused.
9. Fix concrete observed failures, run meaningful regressions, preserve immutable
   reports and rollbacks, update/archive NEXT and write remaining card instructions.
   User is testing; do not take desktop input back without their request.
