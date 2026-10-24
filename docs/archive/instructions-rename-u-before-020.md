# RENAME-U - install and check the separate Province Rename candidate

**Current update:** 0.1.2 had no entry because the concrete type guard was
wrong. Measured-type correction0.1.3 is installed in the dedicated copy.
Start with instructions-rename-013.md, then return here for full checks.
Expected0.1.3 and49native signatures; V2DLL logs are V2DLL/Logs/. Older
candidate details below are historical.

2026-10-20. RENAME-0 is implemented and automatically checked, **not tested
in game**. User selected free-text renaming and V2DLL. Follow the report:
report-province-rename-1020.md. No additional feature design is required.

**Installation update2026-10-20:** a NEW dedicated copy is ready at
work/Victoria-II-Province-Rename-TGC252, with TGC2.5.2, current V2DLL and
Rename installed/verified. User opens victoria2.exe there and selects
Province Rename plus TGC v2.5.2 (stable). Profile:
TGC252_ProvinceRename_20261020. Proceed to section2; section1 is for future
installations/updates. See report-province-rename-fresh-tgc-1020.md. The
initial addition to the older Army test copy was reverted.

## 1. Before installation

1. Read NEXT.md, AGENTS.md, the report and the candidate README.txt. Candidate:
   work/province-rename/candidate/Province-Rename-0.1.0-test.zip.
2. Select an isolated game copy. Main E: and all existing ATCU/Politics
   installations stay as they are unless the user explicitly selects them.
3. Check that copy's V2DLL loader. Current per-active-mod loading is the
   preferred route. Older 5.82 only loads the first -mod= plugin, so do not
   assume that selecting both ATCU and Rename loads both. The report and
   README describe the current dll_Submods alternative; use one DLL copy only.
4. Before any game-copy write obtain a fresh user confirmation of normal
   exit and check Get-Process v2game. Do not launch or terminate the game.
5. Run python tools/check_exe.py <copy/v2game.exe> and compare candidate
   SHA-256 with the adjacent .sha256. Preserve hashes of the copy's Lua/V2DLL,
   ATCU/Politics and relevant base files. Preserve existing conflicting target
   files outside mod/ before replacing anything; refuse unknown files.
6. Copy only the candidate's mod/ into the selected game folder. For the
   normal mod-folder route enable LOAD_DLL_FROM_MOD_FOLDER=1 in its V2DLL
   settings if needed, preserving the original settings. Do not install a
   different lua51.dll from another project or change base data.

## 2. User launch and first selection

1. User opens the normal launcher and selects Province Rename plus the base.
   For the first check, test the separate mod. Check ATCU combination later
   with a loader that supports several selected plugins.
2. Load a disposable save or start a country with an owned land province.
   Logs/tgcx_province_rename.log must say TGCX Province Rename0.1.0,
   V2DLL plugin API v1,36native signatures OK and Rename hooks installed.
   V2DLL's log must show this plugin -> ok. A host/hash refusal in an isolated
   loader fixture is expected; a game-start refusal needs diagnosis.
3. Select an owned province while paused. A Rename control should appear
   between the state and province names. Open/close/reopen it; the editor
   should be visible, accept clicks, and not cover native header controls.
4. Compare screenshots to docs/province-rename-layout.svg and the generated
   layout-preview.png. Fix deviations or record the engine reason. Preview
   pixels and successfully constructed roots are not visual acceptance.

## 3. Rename and invalidation checks

1. Open Rename, click the field, replace the text with an ordinary new name,
   then Apply. Confirm the province header, map tooltip and displayed map
   label update. Inspect both Rename queued and Rename observed applied in
   the log. Record whether the command is processed while paused; if it
   waits, record what changes after a normal unpause/day advance.
2. Repeat with San Martín de los Andes and a longer valid name. Check that
   the stored/displayed text matches, including accents; test up to80bytes.
3. Cancel an edit and confirm no name changes. Open an editor, select another
   province or close the host, then reopen: stale input must not rename the
   old or new province. Repeat after changing the player/owner where possible.
4. Foreign and sea provinces must have no usable Rename entry. Blank,
   quote/backslash/control-containing and over80-byte names must not queue.
5. Check foreground and hit testing after closing/reopening the province
   panel, selecting multiple provinces, changing tabs and changing resolution.
   No stale roots or frozen controls after a load or return to menu.

## 4. Persistence and coexistence

1. Save after a successful rename; reload the save. Check exact name in the
   header and map. No names sidecar should be needed. Load an earlier save:
   its earlier names must remain intact.
2. After normal exit, untick only Province Rename and reload the renamed
   disposable save. Check the renamed name persists through the native save
   path. Do not modify real saves to manufacture evidence.
3. With current multi-plugin V2DLL, select Rename plus ATCU and the base.
   Check both plugins' logs, rename, open Templates/Conform and perform one
   ordinary Army action. Report the exact V2DLL build and acceptance scope.
4. Do not claim MP or other-base acceptance from this single-player check.

## 5. Preserve evidence and close

1. After the user exits normally, preserve full rename/V2DLL logs and relevant
   game error/system logs in a new ignored work/ folder; retain originals.
2. Record observations/screenshots against every applicable item. If a guard,
   timer, selection, save or visual check fails, diagnose before another
   installation. Every update needs fresh normal-exit confirmation.
3. Write a new dated report; never rewrite the implementation report. Update
   ROADMAP, SUBAGENTS, archive/rewrite NEXT and prepare instructions for the
   actual next card. No automatic commit/push, release or base changes.
