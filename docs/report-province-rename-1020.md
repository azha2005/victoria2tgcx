# Province Rename 0.1.0: separate V2DLL plugin

2026-10-20. **Implemented and automatically checked; not installed or tested
in game.** User selected free-text renaming in the province window and then
explicitly requested V2DLL integration. No subagents were used.

## Result

Standalone Province Rename package, independent of Army/Politics. It adds a
Rename control to the standard province header and a native editor beside
the panel, with the current name, text field, Apply and Cancel. Only land
provinces owned by the player qualify. Names use 1-80 Windows-1252 bytes;
surrounding spaces are trimmed, and quotes, backslashes, formatting/control
bytes and undefined CP1252 bytes are refused. Native input is not Unicode.

Apply rechecks the world, selected province, owner and player before queuing
the existing CChangeProvinceNameCommand. The engine applies the name and
updates both map label paths. The plugin observes the applied name before
refreshing the cached province header. It does not write province names,
map caches or simulation data directly, and does not generate script from
typed text. Cancel and selection/ownership/country/world changes discard
the pending edit. There is no persistent local names sidecar.

The original research-province-rename-1019.md remains unchanged. Its main
unknown is resolved: the native command already exists. It is registered by
FUN_0081B290 and has serialization, parsing, cloning and execution slots.
The province writer/parser also use the native name getter/storage. This is
static persistence evidence; a real save/reload check is still required.

## V2DLL integration

The x86 DLL exports V2Mod_Init, API v1. DllMain installs nothing. The package
ships it as mod/tgcx_province_rename/v2dll_mod.dll. Keep the user's V2DLL Lua
loader and enable LOAD_DLL_FROM_MOD_FOLDER. No Lua proxy or V2DLL binary is
included, and no existing ATCU binary or loader was replaced.

Current upstream was inspected at cdad55c70c793144ce4e157d3a6e1ffa247bcc69:

- Source/V2/V2/V2DLL_ModLoader.cpp loads one plugin per active mod and also
  supports V2DLL/dll_Submods/*.dll. Its per-plugin API storage and load-once
  checks are present; the API remains version 1.
- The global-loader alternative is to MOVE the same DLL to
  V2DLL/dll_Submods/tgcx_province_rename.dll, leaving the selected GUI mod
  in mod/. Do not keep two copies under different paths. The plugin checks
  the selected package marker and stays inactive when it is unticked.
- The older local 5.82 loader only loads the first -mod= plugin. For that
  version Rename must be first. Combining plugin mods such as ATCU needs
  the current loader. No claim that 5.82 loads every selected plugin.

Public source and current instructions:
[V2DLL](https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension).
The three downloaded reference files remain ignored in
work/province-rename/. Loader source SHA-256:
03e2100285f466e336702ebe91ae59864095e16cfae6b29fce464dfdb538094b.

Initialization validates API size/version/base, the executable file hash
and 36 relocation-aware signatures before installing anything. It uses
V2DLL's PatchSlot and StealToTrampoline helpers only after those checks.
The only hooks are the province refresh entry and the province view
destructor. They preserve registers, flags and FP/SSE state. A main-thread
100ms Windows timer creates/ticks GUI roots after native selection updates,
including while paused; its actual dispatch still needs player verification.
Destructor handling stops the timer and requests native deferred GUI
collection (P54). No renderer, input pump or Army hook is replaced.

## Interface reference and limits

Local specification: province-rename-layout.svg, created and inspected
before interface implementation. Package generation produces the same
84x18 entry, 340x180 editor and 308x28 native edit field. Preview:
work/province-rename/candidate/layout-preview.png.

All GUI definitions, localisation and geometric textures are new, prefixed
files. Native factories create independent roots, which follow the host's
actual render-owner chain (P40), show their complete subtree (P57), promote
on opening/reopening/native refresh (P56), and hide with the host. No base
province_interface.gui is copied or overridden. Vanilla and installed
TGC2.5.2 fonts/sprites were checked; other bases have not been run.

Recorded deviations/limits:

- There was no prior Province Rename board in the historical mockup set.
  This new local board is the implementation reference; the remote artifact
  was not republished. It is not a claimed user-approved game screenshot.
- Own flat parchment/green artwork avoids redistributing game/TGC art;
  native font rasterization, hover and hit testing are unverified.
- The standard header gap at x17/y27 is used. Mods that restructure the
  province header may need a placement adaptation. The window moves inside
  the screen on narrow displays; the nominal board places it beside the host.
- Apply/Cancel are mouse controls. No new keyboard shortcut is installed.
- Single-player candidate. Using the native synchronized command preserves
  R5, but does not constitute a multiplayer test or acceptance.

## Verification

- Reference executable SHA-256 62d48c20...6648 and **259 guards pass**:
  179 existing runtime, 44 prior research, 36 separate Rename runtime guards.
  Existing Army/Politics generated header is unchanged and checks current.
- Win32 Release plugin and fixture builds pass with warnings as errors.
- **30 native fixture checks pass.** The reference image is mapped and
  relocated in a test process without executing its entry point. Actual
  constructor, execute, province setter/getter, destructor and edit-buffer
  SetText/GetText code run with explicitly stubbed allocation/string,
  font-dependent insertion, queue and map-render boundaries. Tests check
  0x5C allocation canaries, command id0x707, exact long accented name/id,
  one queue handoff, native province update and both map label calls, stale
  selection/world/owner rejection, input boundaries, altered-byte guard
  refusal and one header refresh only after the applied name is observed.
  This is isolated native execution, not an integrated game run.
- **Three Python tests pass:** separate package boundaries, valid GUI/GFX
  syntax, native fonts/sprites on vanilla/TGC, localisation columns/encoding,
  exact DDS pixels, ZIP content/CRC, refusal of old output and a foreign DLL,
  x86 export/geometry and complete versioned archive filename preservation.
- **Four probes through the unmodified current V2DLL loader source pass:**
  mod-folder and dll_Submods routes, each with the package selected/unticked.
  The real plugin export checks API failures and selected-package behavior;
  unsupported probe executables correctly refuse before patch calls.
  Load-once is verified. Core patch helper implementations are stubbed and
  never called; these probes do not claim successful game initialization.
- TGC2.5.2 scan: anchors and reserved event range OK. No new script events.
- Generated preview inspected. Native appearance/game/save/MP checks remain.

Evidence: work/province-rename/{exe-check.txt,build.log,package-tests.txt,
loader-probe.json,tgc-scan.txt} and focused ignored Ghidra exports.

Reproduce:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/build_province_rename.ps1
# Build dll/province_rename/tgcx_rename_tests.vcxproj, Release|Win32, then:
dll/province_rename/Release/tests/tgcx_rename_tests.exe E:/Victoria.II.v3.04.Inclu.ALL.DLC/v2game.exe
python tools/test_province_rename.py --v2dll-loader work/province-rename/V2DLL_ModLoader.cpp
python tools/prepare_province_rename.py <new output folder> --preview <preview.png>
```

## Deliverable and next check

work/province-rename/candidate/Province-Rename-0.1.0-test.zip:
**11 files, 178193 bytes**. SHA-256:
dfd67d998aa2c62d15b539b383a638c48a8766ddec81f22c21ce6561e4e8f082.
The adjacent .sha256 and -manifest.json record the archive and payload.

Source: dll/province_rename/. Generation/build/test tools have province_rename
or rename in their names. The ZIP contains only mod/ and README.txt.

RENAME-U: instructions-rename-u.md. Choose the target copy and loader first;
obtain fresh normal-exit confirmation before copying into it. User launches
and exits. No installation, game controls, commit/push or publication occurred.
