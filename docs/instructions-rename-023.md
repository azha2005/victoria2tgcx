# RENAME-023: check the installed redesign and centered typing

2026-10-21. Version 0.2.3 is installed in the dedicated TGC/V2DLL copy after
fresh normal-exit confirmation, with all payload/protected hashes checked.
No additional installation is needed. Implementation report:
`report-province-rename-redesign-implemented-1021.md`.

1. The user opens `victoria2.exe` in
   `work/Victoria-II-Province-Rename-TGC252`, selecting **Province Rename
   (Universal)** and **TGC v2.5.2 (stable)**. The agent never launches it.
2. Select owned land and click ABC right of the province name. Compare
   with `province-rename-layout.svg` and the native-font preview
   `work/province-rename/candidate/layout-preview-023-native.png`.
   Expect a white title on native green/gold, continuous parchment with
   no divider or dark footer shelf, both gold input braids, centered black
   serif captions/current name/status and blue buttons with gold corners.
3. Click in the input and replace the name with a short one. The **typed
   text must center horizontally as it changes**, with coherent caret
   positioning. Try inserting in the middle, deleting/backspacing,
   Home/End and accented CP1252 letters. Check empty input, long names and
   paste. Long text uses minimum padding; editing must remain usable.
   Valid limit remains 1-80 CP1252 bytes, subject to existing validation.
   Blank/over-limit input must not queue an invalid rename.
4. Check Cancel/reopen, Rename and header/map update, clicking ABC again,
   province-selection/ownership changes and closing/reopening the host.
   Other native inputs must keep their ordinary layout. Complete remaining
   behavior and save/reload cases in `instructions-rename-020.md` sections
   2-3; earlier short 0.2.1 success does not prove these extended cases.
5. Log `Logs/tgcx_province_rename.log` must show **0.2.3 Universal**,
   **66 native signatures OK**, and after opening the editor:
   `Rename centered input enabled; shared native text/caret inset`.
   `Centered-input hook refused` means the new centering did not install.
   Check for other refusal/fault messages. V2DLL logs are in `V2DLL/Logs/`.
6. Preserve fresh logs and unedited appearance/caret captures after the
   user's normal exit. Preview/fixture checks are not game acceptance.
   Any correction requiring installation again needs fresh exit/process
   gates. Keep `redesign-023-1021/before-023` and all earlier rollbacks.
7. Other-base gameplay, ATCU coexistence and MP are later selected work;
   no extra copy, publication, commit or push is implied by this check.
