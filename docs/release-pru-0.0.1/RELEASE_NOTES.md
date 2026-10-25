# PRU - Province Rename Universal 0.0.1 (beta)

Rename any province you own, with free text, straight from its province
panel. Click the ABC button beside the name, type the new name and click
Rename. The editor is built from native Victoria II artwork, with centered
Garamond text and the native caret. The engine's own rename command changes
the name, so the header and map label update and the name is stored in
the save.

![Rename editor open on Troyes](ss1.png)
![Troyes renamed: header and map label updated](ss2.png)

First public release (internal build 0.2.8). Full history: `CHANGELOG.txt`.

## Requirements

**Victoria II: Heart of Darkness 3.04** and current
[V2DLL with per-active-mod loading](https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension)
(tested with 5.98). Enable `LOAD_DLL_FROM_MOD_FOLDER=1`, extract the ZIP into
the game folder and select **PRU - Province Rename Universal** alongside your
base mod. Full instructions are in `README.txt`.

## Tested

Played on **TGC 2.5.2**: editor, typing, repeated renames, map and header
updates, ABC placement. Vanilla, TGC 2.3.4, HPM and DoD Fan Fork pass
static font, sprite and anchor checks only. ATCU coexistence, other bases in
play, extended save/reload cases and **multiplayer are untested**.

No base GUI or data file, executable, V2DLL binary or replacement Lua DLL
is bundled. Without V2DLL the mod loads inertly.
