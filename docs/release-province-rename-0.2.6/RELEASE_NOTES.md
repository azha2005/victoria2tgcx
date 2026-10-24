# Province Rename Universal 0.2.6 (beta)

Rename an owned province directly from its province panel. Click the native
ABC button beside its name, type a replacement and click Rename. Includes
a native green/gold/parchment editor, centered Garamond input, aligned
caret and compact 24px ABC button. This final pass raises ABC by 2px and
adds 2px of space after the province name.

Requires **Victoria II: Heart of Darkness 3.04** and current
[V2DLL with per-active-mod loading](https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension).
Tested with V2DLL 5.98. Enable `LOAD_DLL_FROM_MOD_FOLDER=1`, extract the ZIP
into the game folder and select **Province Rename (Universal)** alongside
your base mod. Full instructions and an alternative plugin location are
in `README.txt`.

The player confirmed the editor, centered typing/caret, repeated renames
and map/header updates on **TGC 2.5.2**. The final ABC position adjustment
passed native geometry checks; its final visual check remains pending.
Vanilla, TGC 2.3.4, HPM and DoD have static font/sprite/anchor checks only.
Other-base gameplay, ATCU coexistence, extended save/reload/lifetime cases
and **multiplayer remain untested**. All renames use the native engine
command; this is not a claim of multiplayer/OOS validation.

Supports 1-80 Windows-1252 bytes for owned land provinces. No base GUI/data
file, executable, V2DLL binary or replacement Lua DLL is bundled.
Unsupported executables/signatures refuse the hooks. The mod loads inertly
without V2DLL.

This is the first public Province Rename package, separate from ATCU.
The ZIP and its SHA-256 file are the downloadable mod. Source credits and
native-art provenance are retained; no project license has been selected.
