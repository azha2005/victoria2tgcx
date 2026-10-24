# New Province Rename / TGC / V2DLL game copy

2026-10-20. **Staged and verified; not launched by the agent or accepted in
game.** User explicitly requested a NEW copy with TGC and V2DLL after
correcting the initial interpretation to reuse an existing test copy.

## Target and contents

`C:/Users/JC/Downloads/victoria2template/work/Victoria-II-Province-Rename-TGC252`.

- 8,967 stock game files copied from D:/Victoria.II.v3.04.Inclu.ALL.DLC,
  checked against the prior clean-copy manifest. Stock lua51.dll hash is
  b2ba19343691f22d1af1417f9a7cbaeae74ba820a2a412f5a6cdea271cc1a7b0.
- 16,411 TGC2.5.2 files copied from the installed stable source at
  E:/Victoria.II.v3.04.Inclu.ALL.DLC/mod/TGC_v2.5.2/TGC, checked against
  that same clean-copy inventory. Canonical target mod/TGC satisfies the
  unmodified hardcoded definition.csv path (P52).
- Own stable descriptor points to mod/TGC with independent user_dir
  TGC252_ProvinceRename_20261020. Original descriptors and saves untouched.
- Current upstream V2DLL distribution pinned at
  cdad55c70c793144ce4e157d3a6e1ffa247bcc69: nine files downloaded, each
  checked against its Git blob SHA-1 and recorded with SHA-256. This uses
  dinput8.dll and V2DLL/v2dll_core.dll, leaving Lua original. Both binaries
  are PE32/x86; expected DirectInput exports and V2DLL_CoreVersion present.
- V2DLL's new-format settings preserve the existing TGC test copy's explicit
  preferences. LOAD_DLL_FROM_MOD_FOLDER=1 and LOCAL_MOD_CONFIG=0. Newly
  introduced feature toggles are explicitly off, preserving the earlier
  baseline: satellite-command fix, vassal colours, pop salary multiplier,
  move-pop extension and host-human sync. Vassal colour percentage remains65
  but colours are disabled. Upstream INI and exact changes retained in evidence.
- Province Rename0.1.0:10mod files from the verified candidate ZIP;
  plugin at mod/tgcx_province_rename/v2dll_mod.dll. No ATCU/Politics package
  or old Lua proxy is included in this new copy.

The source game/TGC payload totals1,211,130,495bytes. Full stage contains
25,400files:25,399files in the final verified payload manifest plus its own
summary manifest. Files are independent copies, not hard links. Executable
and TGC map definition link counts checked. Free-space margin was checked
before copying; no bulk secondary staging of the game was needed.

## Correction and process gates

Initially added10Rename files plus a new guide to the existing
work/Victoria-II-clean-TGC252 copy. On the user's correction, verified every
added file against its installed hash, removed only those files/empty owned
directories, and confirmed all142protected original hashes unchanged.
The original test copy's Lua/V2DLL/settings/TGC/ATCU files were not replaced.
Evidence: work/province-rename/install-tgc252-1020/{plan,installed,reverted}.json.

The user had confirmed normal exit. During preparation they started that
old game copy again; process gate detected PID44452 and stopped before any
new-copy creation. User then confirmed normal exit again. No process was
launched or terminated by the agent. New copy staging proceeded only after
the fresh process gate passed.

## Verification and evidence

- Final25,399payload hashes match, including the configured V2DLL INI and
  all10Rename files. Complete target inventory matches exactly, with the
  additional summary manifest only. No unaccounted payload files.
- Reference executable hash and all259guards pass on this new copy.
- TGC scan of the new mod/TGC: anchors and reserved range OK.
- Only two mod descriptors exist: TGC_v2.5.2.mod and
  tgcx_province_rename.mod. No dependency on the prior Army test installation.
- Current V2DLL loader source/API checks and the Rename native/package
  fixture evidence remain in report-province-rename-1020.md. The new upstream
  runtime binaries have not been executed in this copy; source/API probes
  are not a runtime compatibility claim.

Evidence: work/province-rename/fresh-tgc252-1020/{plan,installed,verification}.json,
exe-check.txt, tgc-scan.txt, upstream-v2dll-settings.ini and
v2dll-settings-changes.json. Distribution inventory/download evidence is in
work/province-rename/v2dll-current-tree.json and
work/province-rename/v2dll-cdad55c7/upstream-manifest.json. The implementation
candidate ZIP itself remains unchanged.

## Next action

User opens **victoria2.exe in the new folder** and selects **Province Rename**
plus **TGC v2.5.2 (stable)**. Test guide is tgcx_province_rename_test.txt in
that game root; full steps remain docs/instructions-rename-u.md from section2.
Verify native editor, Apply/Cancel, ownership, labels, accents, save/reload and
foreground/hit testing. Preserve full rename and V2DLL logs after normal exit.
No automatic game launch, commit/push or publication.
