# Volunteers test copy: CreatineCore 6.36 loader migration

2026-10-29. The user reports continued game-day lag, asks whether V2DLL has a
newer version, then explicitly requests replacement of the loader. Replaced only
the loader in `E:/Victoria-II-Volunteers-0004`. No game launch, desktop input,
process termination, save edit or main installation change was performed.

## Upstream and migration

- Installed baseline was V2DLL 5.98, core SHA-256
  `c6816a2118842fc635aaa2b24f067418359ae9462b1f1952d0442a68cb55b699`.
  Its startup log confirms the version. New official release is CreatineCore
  Modding Toolkit 6.36, published 2026-10-26. The project renamed V2DLL and moved
  its core/settings into `z_CreatineCore`.
- Official release:
  <https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension/releases/tag/v6.36>.
  Downloaded release ZIP matches GitHub's published SHA-256
  `f3d81c5bb03081a8b75ca9e0ca9537a6850eeb512e695188db9bc78d26cfc263`.
  Extraction CRCs/path bounds and each staged DLL's x86 PE format were checked.
- New root `dinput8.dll` loads only `z_CreatineCore/creatinecore.dll`.
  Old `V2DLL` files/logs remain intact and inactive through this proxy. Original
  Lua DLLs are unchanged. Existing Volunteers 0.0.12 is unchanged.
- The included `ccore_performance_patch.dll` is installed in
  `z_CreatineCore/dll_Submods`. Previous performance options, including worker
  count 0 and pop quantization 12 bits, migrate into its separate INI.
  `FIX_ARMY_WINDOW_LAG=0` follows upstream's 6.28 default change because its
  stutter fix remained incomplete. Performance `DEBUG_LOG=0` overrides the
  distributed verbose default. State/RNG probes stay off; crash/OOS logs stay on.
- Core options migrate by key. New gameplay/script/UI extensions absent from the
  old configuration are disabled by the explicit policy in
  `tools/tgcx_creatinecore_migration.py`; they are not silently imported from the
  distributed defaults. Local mod configuration stays off; per-active-mod DLL
  loading stays on. Retired, disabled `PATCH_GRAPH_POINT_CLAMP=0` is dropped;
  unknown old settings or enabled retired settings cause refusal.
- No bundled base GUI, topbar, menu, localization, graphics or dgVoodoo settings
  overwrite is performed. The optional personal gameplay-patches DLL is omitted.
  The installed supporting core eventmusic DLL follows the new core path; the
  prior `ENABLE_EVENT_SOUNDS=0` setting is preserved.

## Generated data and launch order

Pinned 6.36 source `V2TechButton.cpp` shows `ResolveModFolder` selects the FIRST
`-mod=` descriptor for `EnsureCCoreDefines`. It can rename old
`common/defines_v2dll.txt` and append missing keys. A naive loader swap would
therefore alter the generated file inside the installed TGC tree on next launch.

The three existing PowerShell launchers now explicitly select:

1. `mod/tgcx_loader_data.mod`: a new additive carrier containing only the required
   `common/defines_ccore.txt` physical configuration.
2. `mod/TGC_v2.5.2.mod`: the same TGC descriptor, profile and load order relative
   to Volunteers.
3. `mod/tgcx_volunteers.mod`: the unchanged Volunteers package.

The carrier has no DLL, `user_dir`, dependencies or `replace_path`. Its defines
file is generated from an allowlist of prior loader-generated values, with own
comments and the new disabled feature's data default; no whole TGC source file
is copied. All keys the upstream initializer would append are present. TGC's
virtual `common` replacement does not affect the loader's direct physical file
read. Normal TGC-before-Volunteers order remains explicit. The upstream-required
`defines_ccore.txt` filename is the only non-prefixed file in this own carrier.

## Validation and exact installation

- Four pure migration tests cover preserved options, diagnostic/stutter gates,
  separation of performance options, duplicate/unknown/enabled-retired refusal,
  empty values/line boundaries, and safe extraction of generated define values.
- Own Win32/x86 non-game probe loads the actual new proxy and finds all six
  DirectInput exports. Its non-game host guard leaves both core DLLs unloaded.
  Compiling against the pinned 6.36 `V2ModAPI.h` establishes API version 1,
  28-byte layout and the expected hook-function offsets. The unchanged Volunteers
  DLL accepts that API while unticked and stays inactive. This is a loader/ABI
  fixture, not an actual game/plugin initialization test.
- All three changed launcher scripts parse. No game launch was used as a test.
- Reference EXE and all 96 Volunteers native signature records pass. Runtime
  hooks still have their exact admission/signature gates; none were relaxed.
- 15 installed payload hashes match stage. Full-tree comparisons confirm all
  16412 TGC files and 15 Volunteers files unchanged; ten further protected
  executable/Lua/old-loader/descriptor/shortcut files match before and after.
  The root proxy is intentionally replaced; other new payloads live in own paths.
- New dinput8 SHA-256:
  `f6ec57795022e9bab3fcdd52c5d39ec8884ceaea0454317746360d6293480f47`.
- New core SHA-256:
  `a36cc0339e83db2cab2d6c60047d84c1abf5e90b9fb267d54cbcee7d7191c5a1`.
- Performance plugin SHA-256:
  `895d21d1c7beea8a7ee43a75f3cf7cffe4837e208cf6bd9e4f449660e1a43dc3`.

Private evidence: `work/volunteers-loader-636/release-manifest.json`,
`migration.json`, `stage-manifest.json`, `protected-hashes.json`,
`install-result.json`, pinned upstream sources and the owned x86 probe.
No upstream binaries, art, full source files or archives enter git.

## Rollback and acceptance limits

Backup: `work/volunteers-loader-636/before-636`. Its `rollback.json` distinguishes
replaced files from newly created paths. Old V2DLL tree remains intact, with
additional core/settings copies in the backup. After normal exit, restore the
backed-up dinput8 proxy, three launch scripts and test README. Move the newly
created core/carrier paths out of the game directory for a complete rollback;
never remove or overwrite them while a game is running. Do not restore the older
pre-0.0.12 diagnostics configuration accidentally.

The latest pre-upgrade 0.0.12 run confirms probes disabled and 56 executed ticks;
its measured daily-plugin work totals 5814 microseconds and vision 3781
microseconds across 21 calls. Those timings exclude native originals and
assembly; hot callbacks are sampled. They do not prove that Volunteers has no
performance cost or that a new loader cures the lag.

CreatineCore startup, actual Volunteers hook admission, save/profile visibility,
combat/siege behavior and game-day performance remain unverified on 6.36. User
testing follows `instructions-volunteers-loader-636.md`. Multiplayer sending
remains unsupported/refused.
