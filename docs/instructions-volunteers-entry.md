# VOL-ENTRY: 0.0.15 disappearance retest

2026-10-29. Read report-volunteers-entry-1029.md and P42/P54/P56/P57/P85/P86.
User/tester retain desktop/game operation. This build does not enable MP sending.

1. Close normally. Back up the previous Volunteers folder/configuration. Extract
   work/volunteers-entry-0015/final-r2/Volunteers-0.0.15-SP-beta.zip into a disposable
   prepared game copy. Normal launcher selects00 loader data, TGC2.5.2, ZZ SP beta.
   Keep CreatineCore6.36/settings from the included README. No scripts needed.
2. Verify version0.0.15 and admitted GUI/load hooks. Test a normal new SP game
   and an existing0.0.13 save separately. The installed local0.0.12 copy was not
   changed by this task. Preserve prior saves/logs and the0.0.13 rollback ZIP.
3. Open/close Diplomacy repeatedly, switch recipient countries, send/recall/open
   panels, change tabs, minimize/restore and leave the game in the background.
   The same native Send volunteers button should remain/reappear in its grid cell.
4. Open another native dialog/tooltip. Ordinary polls must not bring the entry
   above that dialog. After closing it, the entry should recover if Diplomacy
   had covered it. Check the caption and click response, not just construction logs.
5. Leave a campaign running and reopen Diplomacy. If native roots expire, look
   for window lease expired and a replacement entry. Check no duplicated button,
   duplicate click callback or creeping UI pauses. Unknown native data still
   refuses; do not force signatures or free retained observer boxes manually.
6. A checkpoint failure should retain the entry/panels with a reload requirement
   while refusing mission writes/commands. Do not deliberately corrupt the only
   save. Preserve the first failure log, then reload a disposable save normally;
   no Create/Transfer/Move or partial checkpoint may be replayed automatically.
7. If it disappears again, retain Logs/tgcx_volunteers.log and loader error logs,
   exact version/country/date, whether the game was backgrounded, and whether
   closing/reopening Diplomacy or reloading restores it. Include the screenshot.
   Inspect volunteer_entry_repaired, off-thread hide, lease expiry, native fault,
   capacity and checkpoint messages before assigning a cause.
8. Source verification: tools/build_volunteers.ps1, test_volunteers.py,
   test_volunteers_share.py, test_volunteers_mp.py, test_collect_volunteers_oos.py,
   and check_exe.py under tools/. Keep binary/art/raw exports outside git.
9. Continue prior SP lifecycle/performance/vision acceptance under
   instructions-volunteers-feedback.md. MP transport/OOS work remains under
   instructions-volunteers-mp-sync.md; sending/Recall/scopes are still refused.
10. Archive NEXT verbatim, update report/index/statuses and next steps. Source
    commit/push remains authorized; no public binary upload or peer message.
