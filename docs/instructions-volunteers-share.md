# VOL-SHARE: player handoff and next acceptance checks

2026-10-29. Plain ZIP requested like ATCU/PRU, explicitly no command files or
installer. The artifact is ready; send the ZIP linked in the report. No publication
or messages to third parties were authorized. User handles sharing and game input.

1. Keep the final ZIP and its adjacent SHA-256/JSON sidecars under
   work/volunteers-share-0012/final. Do not add DLLs or derived artwork to git.
2. Recipient reads README and installs CreatineCore 6.36 separately. Extract mod
   payload into their prepared Victoria II 3.04/TGC 2.5.2 copy after normal exit.
   No BAT, CMD, PowerShell, setup or new external launcher is required.
3. Use ordinary launcher selections 00 carrier, TGC, ZZ Volunteers. Verify actual
   arguments/order and startup logs; display-name sorting is the established
   policy, but do not assume a different launcher enforces it. Confirm unchanged
   TGC profile and physical defines target in the carrier.
4. Confirm native GUI/load/journal/scoped hooks and core/performance plugin all
   admitted, probes off. Test neutral donor, wartime recipient and reachable idle
   land army. Verify whole/partial selection and ordinary player command.
5. Continue loader instructions steps 4..7 for battles, siege progress/completed
   capture attribution, recall, recipient peace, donor war, loss and save/load.
   Recipient-wide custom vision remains incomplete; multiplayer Send is refused.
6. Do not claim all bases, multiplayer, OOS safety or complete prototype acceptance
   from the user saying it worked well enough. Same-save game-day timings remain
   the way to measure performance. Preserve screenshots/logs privately.
7. Regenerate only from the accepted source candidate after changes are tested:
   python tools/test_volunteers_share.py
   python tools/prepare_volunteers_share.py <new output folder>
   Read ZIP CRC/manifest and compare DLL/art to the actually accepted build.
8. Preserve immutable report and prior handoffs, archive NEXT and update status
   with the next exact steps. Source-only commit/push remains authorized.
