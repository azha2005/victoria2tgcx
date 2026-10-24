# RENAME-024: retry centered typing after the double-offset correction

2026-10-21. Checked 0.2.4 is installed in the dedicated TGC/V2DLL copy on
the user's fresh installation instruction and clear process gate. Full
0.2.3 backup retained. No additional installation needed.
Report: `report-province-rename-centering-1021.md`.

1. User opens `victoria2.exe` in
   `work/Victoria-II-Province-Rename-TGC252`, selecting **Province Rename
   (Universal)** plus **TGC v2.5.2 (stable)**. Agent never launches it.
2. Select owned land and click ABC. The accepted short 0.2.3 redesign is
   retained. Current board is `province-rename-layout.svg`; native-font
   preview is `work/province-rename/candidate/layout-preview-024-native.png`.
3. Type a short name, add/delete letters and pause between edits. The name
   should remain horizontally centered, with no offset jumps as the dialog
   is repositioned. Check mouse caret placement, insertion in the middle,
   Backspace/Delete, Home/End, empty text, CP1252 accents, long names and
   paste. The caret's normal blinking is separate from text movement.
   Long names retain minimum padding and must remain editable.
4. Check Cancel/reopen, Rename, map/header update and province/host changes.
   Other native inputs should retain their ordinary appearance. Follow
   `instructions-rename-020.md` sections2-3 for remaining limits/ownership/
   stale selection and save/reload checks. Prior short success is not full
   persistence/lifetime validation.
5. `Logs/tgcx_province_rename.log` should show **0.2.4 Universal**,
   **72 native signatures OK**, and after opening:
   `Rename centered input enabled; single native text/caret origin`.
   A centering hook/placement refusal means centering is not accepted.
   V2DLL logs remain in `V2DLL/Logs/`.
6. Preserve fresh logs and unedited captures after normal exit. Any further
   installation needs a fresh user exit/install instruction and clear
   process gate. Never launch/terminate autonomously. Keep
   `centering-024-1021/before-024` and earlier backups.
7. Other-base gameplay, ATCU coexistence and MP remain later selected work.
   No extra-copy update, release, commit or push is implied.
