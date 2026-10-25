PRU - PROVINCE RENAME UNIVERSAL 0.0.1 (BETA)
Internal build 0.2.8. First public release.

Rename a province you own directly from its province panel. Click the ABC
button beside the name, type a replacement and click Rename. The editor
uses native Victoria II artwork, centered Garamond text and the native
caret. Cancel discards the edit. This is a separate mod from ATCU.
See CHANGELOG.txt for the version history.

REQUIREMENTS

- Victoria II: Heart of Darkness 3.04, supported reference executable
  SHA-256 62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648.
- Current V2DLL with per-active-mod plugin loading. Tested with V2DLL 5.98,
  source cdad55c7. Get V2DLL from its author:
  https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension
- Enable LOAD_DLL_FROM_MOD_FOLDER=1 in V2DLL/v2dll_settings.ini.

No V2DLL binary, replacement lua51.dll, executable or base mod is included.
Older V2DLL 5.82 only loads from the first -mod= folder; use a current
version when combining plugins. Without V2DLL, this GUI mod loads inertly
and cannot provide free-text renaming.

INSTALL

1. Close Victoria II normally.
2. Extract this ZIP into the game directory containing v2game.exe. It adds
   mod/tgcx_province_rename.mod and mod/tgcx_province_rename/.
3. In the launcher select PRU - Province Rename Universal, plus your base mod
   if you use one. Keep V2DLL installed and enable its setting above.
4. Start the game yourself. Select owned land and click ABC beside its name.

No base GUI or data file is replaced. The package has no TGC dependency,
user_dir or replace_path. It does not contain Army, Conform or Politics.

Alternative with current V2DLL: MOVE mod/tgcx_province_rename/v2dll_mod.dll
to V2DLL/dll_Submods/tgcx_province_rename.dll, retaining the GUI mod. That
location needs no loader setting. Keep only ONE copy of the plugin and
keep PRU selected in the launcher.

USE AND LIMITS

- Owned land provinces only. Selection, ownership, country or world changes
  cancel the active edit. The target is checked again before queuing.
- Names use 1-80 Windows-1252 bytes, with surrounding spaces trimmed.
  Quotes, backslashes and formatting/control characters are refused.
  Cyrillic/CJK/Unicode input is not supported by this version.
- Typed names center while editing; long names retain minimum padding.
- Rename queues the native CChangeProvinceNameCommand (0x707). The engine
  changes the name and refreshes the map. No local names sidecar is used.
- Names use the engine's save/load path. Extended save/reload, ownership
  and GUI-lifetime cases have not all been gameplay-tested.

TEST COVERAGE

The player confirmed the editor, typing/caret, repeated renames, native
map/header updates and the final 20x20 ABC placement in game with
TGC 2.5.2 and current V2DLL 5.98 (2026-10-25).

The package is designed for vanilla and ordinary reference-3.04 base mods.
Static font/sprite/anchor checks passed for vanilla, TGC 2.3.4, TGC 2.5.2,
path-fixed HPM and DoD Fan Fork. Only the TGC 2.5.2 gameplay check above is
claimed. ATCU coexistence, other-base gameplay and multiplayer are untested.
Native command serialization is not a multiplayer/OOS test.

LOGS AND REMOVAL

Logs/tgcx_province_rename.log should show 0.2.8 Universal (the
internal build of PRU 0.0.1), 78 native signatures OK and Rename hooks
installed. After opening the editor expect
single native text/caret origin. V2DLL logs are under V2DLL/Logs/.
Unsupported executables/signatures refuse the hooks and log the reason.

For a report include the version, base mod and every selected mod, V2DLL
version, what you clicked, what happened, a screenshot and the logs.
Logs may contain your Windows user name in paths.

To remove: untick PRU. After exiting normally remove its .mod
and folder (and its optional dll_Submods copy, if used). Renames already
stored in a save remain part of that save. Keep your saves backed up.

CREDITS

azha2005 / JC: PRU - Province Rename Universal.
artyom-kuznetsov and V2DLL contributors: plugin API and native hook/GUI
patterns, credited in source. maxioten and Victoria2-Reverse-Engineering
contributors: research notes and symbols. Paradox Interactive: Victoria II
and the native interface artwork used for the derived UI textures.
TGC, HPM and DoD Fan Fork teams: the base mods inspected during preparation.
No endorsement is implied. No project license has been chosen yet.
