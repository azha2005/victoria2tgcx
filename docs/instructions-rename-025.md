# RENAME-025: check caret, compact ABC and readable typing

2026-10-21. Checked 0.2.5 is installed in the dedicated TGC/V2DLL copy
after fresh normal-exit confirmation and clear process gate. No additional
installation needed. Report: `report-province-rename-readability-1021.md`.

1. User opens `victoria2.exe` in
   `work/Victoria-II-Province-Rename-TGC252` with **Province Rename
   (Universal)** and **TGC v2.5.2 (stable)**. Agent never launches it.
2. Select owned land. ABC should be **24x24**, still framed, clickable and
   right of the province name. Open it; native green/gold/parchment dialog
   remains unchanged. Current board `province-rename-layout.svg`; private
   native-font preview `work/province-rename/candidate/layout-preview-025-native.png`.
3. Type a short name, edit in the middle, delete/backspace and use Home/End.
   Text should remain centered without flicker; input now uses stronger
   native Garamond. The blinking **visible caret line** should be centered
   vertically in the field and sit just after the insertion position,
   without the old transparent-margin gap. Check empty input and accents,
   mouse caret positioning, long names and paste. Long names retain padding.
4. Check Cancel/reopen, Rename, map/header refresh and province/host changes.
   Complete remaining limits/stale selection/ownership/lifetime/save-reload
   cases per `instructions-rename-020.md` sections2-3 using 0.2.5/78 log
   expectations. Short previous success does not prove all these cases.
5. `Logs/tgcx_province_rename.log` should show **0.2.5 Universal**,
   **78 native signatures OK**, and after opening:
   `Rename centered input enabled; single native text/caret origin`.
   Check for hook/placement/expired-control refusals. V2DLL logs remain
   `V2DLL/Logs/`. Preserve fresh logs and unedited screenshots after exit.
6. Any later installation needs fresh user exit/install instruction and
   clear process gate. Never launch/terminate autonomously. Keep
   `readability-025-1021/before-025` and all earlier rollbacks.
7. Other-base gameplay, ATCU coexistence and MP remain later selected work.
   No extra-copy update, release, commit or push is implied.
