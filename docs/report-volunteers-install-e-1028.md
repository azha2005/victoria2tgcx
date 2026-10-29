# VOL-INSTALL-E: isolated Volunteers0.0.4 test copy

2026-10-28. The user requested installation in a copy of Victoria II with
V2DLL, preferably on E:, then asked to continue. Installed at
**E:\Victoria-II-Volunteers-0004**, a previously absent directory.
No game launched, input sent, process terminated or main install changed.

## Contents and isolation

- Source: the earlier work/Victoria-II-Volunteers-0001 disposable copy, whose
  observer/V2DLL route has prior campaign evidence. Source files copied,
  hashed and rehashed; no source mutation. The main E: game was not a target.
- 25,385 base/TGC/loader files,1212319718 bytes, copied with independent
  source-before/source-after/destination SHA-256 comparisons. Normal files,
  no hardlinks/junctions.25,385 comparisons all completed successfully.
- TGC2.5.2 at canonical mod/TGC, retaining its explicit definition.csv path
  (P52). Separate generated descriptor names the test and uses a new user_dir.
- V2DLL5.98 from the previously exercised local copy. dinput8.dll loads
  V2DLL/v2dll_core.dll; original Lua companion DLLs retained unchanged. No
  upstream download/update or unrelated loader swap. Existing V2DLL settings
  preserved, including LOAD_DLL_FROM_MOD_FOLDER=1 and previous feature gates.
- 13 volunteer mod files copied from the final0.0.4 audit-r2 stage and checked
  against its manifest; package README copied separately. No PRU/ATCU plugin,
  global DLL subplugin, source game logs/screenshots or old test manifests.
- Default plugin config remains mode=observe. Preview launcher explicitly
  changes it to mode=preview in this copy only. No setting enables Send/Recall.
- Save/settings profile: TGC252_Volunteers_0004_20261028 under the usual
  Documents/Paradox Interactive/Victoria II folder. It will be created by the
  game on launch; it was not copied from the user's ordinary campaign profile.
- Fresh empty Logs and V2DLL/Logs directories. Prior crash/OOS logs were not
  copied, so they cannot be mistaken for evidence from a new test.

## Launch files

In E:\Victoria-II-Volunteers-0004:

1. **tgcx_start_preview.cmd** opens the optional native UI preview.
2. **tgcx_start_observe.cmd** selects observation only.

Both invoke their matching PowerShell script, refuse if another v2game is
running, change only this copy's mode, then use the actual v2game.exe with this
folder as working directory and explicit arguments:

```
-mod=mod/TGC_v2.5.2.mod -mod=mod/tgcx_volunteers.mod
```

The base is explicit; no launcher dependency/order assumption (P49/P50).
tgcx_README_TEST.txt describes the scope, log locations and disposable saves.
These launchers were parsed and inspected but not executed to start the game.

## Hashes and recorded evidence

EXE SHA-256:
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648.
Volunteers DLL SHA-256:
9813de11a98c5518faaf392ec5f92a2ae7419432baedf8679061a048dc44d49e.
V2DLL core SHA-256:
c6816a2118842fc635aaa2b24f067418359ae9462b1f1952d0442a68cb55b699.
dinput8 SHA-256:
0f3d009ecbebb9e0c2fcc625e1254c6abddcd2516639be19bd7c58ec3354019a.
Exact candidate ZIP SHA-256:
4a644dcfb526741162d52a639a4973aa3b14ab9fd7e696f27cd6c0837fc73df2.

Complete file evidence is in E:\Victoria-II-Volunteers-0004\tgcx_install_manifest.json
and work/volunteers-0004/install-e-result.json. The installer is preserved at
work/volunteers-0004/install_e_copy.py. It required a new target, refused
reparse sources/unexpected global subplugins and never deleted/overwrote a
pre-existing destination. The E: write used the user-authorized sandbox approval.

Post-install reference gate ALL OK. Installed descriptor/canonical map/plugin
marker/config, single plugin inventory and launch-script syntax checked. No
game process was running at preflight or postflight. No Rust/Python behavior
tests, native fixture or game/UI test performed as part of installation.

## Test limits

This is ready for a disposable **UI/observer** test, not completed volunteer
gameplay. Send/Recall remain blocked by native session/campaign/combat/access/
vision/save prerequisites. UI callbacks, SEH/patch hardening, collection,
save/reload, coexistence and long sessions remain unverified. Not an OOS fix;
no multiplayer support. V2DLL's existing simulation patches/configuration are
retained, so this is not a clean vanilla-engine comparison.

The previous restricted game launch failed on a native write path. Preparation
on E: does not prove that problem fixed; no launch was attempted this time.
P48: V2DLL may generate defines_v2dll.txt in this disposable base's common
folder after launch; do not interpret that as a change to the main installation.

Next: instructions-volunteers-e-copy.md, with audit/native instructions retained.
User-authorized installation supersedes the older blanket no-install phase
only for this copy. A later game run should preserve logs and use disposable saves.
