# Province Rename 0.2.4: correct the doubled input offset

2026-10-21. **Checked correction installed in the dedicated copy; player
retry pending.** The user tested 0.2.3 and reported that it works, but text
recentering flickers while typing and is off center. The screenshot and
0.2.3 log are preserved unedited under
`work/province-rename/centering-024-1021/`. The log confirms 66 guards,
centering activation, one successful Jijolandia command/readback/header
refresh for province 425 and editor reopen. It does not prove centering.

## Cause and correction

The 0.2.3 correction traced the shared text/caret border fields, but missed
a second native consumer. `FUN_009B5A40` adds edit+0x150/+0x154 to the
control origin when positioning the text entity. Separately,
`FUN_009B6400` passes those same insets into the native text texture setter
`FUN_009DDCF0`, which forwards them to the rasterizer. Thus the centered
offset was baked into the texture and applied again to its position.
Parent positioning repeats through the dialog's native placement path,
explaining why an inset-only fixture did not model the reported movement.
The specific visual flicker still needs a player retry after correction.

0.2.4 keeps the same single refresh detour. For this exact, validated edit
control on its UI thread, it measures the name, temporarily supplies zero
texture padding during the original native display refresh, restores the
centered shared text/caret insets, then calls the native packed position
method with the control's stored origin. The texture has zero padding and
the entity/caret use one centered origin. Repeated native parent placement
keeps that origin. The synchronous operation exposes no intermediate
frame. Other edit controls forward unchanged.

Registers, flags, FP/SSE and last-error state remain preserved. Input
buffer dimensions, cursor, selection, hit region and keyboard handling are
unchanged. Long names retain minimum 8px horizontal padding. Simulation
still changes only through native command 0x707. The native redesign,
ABC entry, fonts, art, dimensions and proven province anchor are unchanged.

R2 reference executable gate passed before new address work. Six new
relocation-aware guards cover the concrete text texture table, position/
raster slots, native position entry/dispatch/RET4 and texture setter:
**72 Rename runtime guards, 295 total** (Army179/research44 unchanged).
VA/RVA contracts and confidence are recorded in symbols.md RENAME-5/6;
read-only exports remain in the ignored evidence folder. P62 records this
failure; next pitfall P63. No live memory writes or injected calls occurred.

## Independent regression and validation

The old fixture checked only the insets at an intercepted refresh entry.
The new regression executes the **actual native display refresh**, active
line/cursor getters, text texture setter and packed position method from
an independently relocated reference executable. Only raster/GPU endpoint
calls are intercepted. The fixture supplies a cached native line vector;
it does not claim to exercise the complete keyboard/layout system.

With origin X50 and a 32px name, 0.2.3's 132px inset becomes texture padding
132 plus entity position182, yielding visible X314 instead of182. The
test reproduces that double application. Through the corrected wrapper,
texture padding is0 and visible X182. Empty, short, proportional and
accented names keep the expected origin across typing/deletion and four
repeated parent position calls per name.

- MSVC Win32 build and all 295 executable guards pass; generated header current.
- **65 native checks, zero failures**, including the above regression and
  retained command, allocation, registry, anchor, native font and input tests.
- **4 package tests** pass, including static fonts/sprites for vanilla,
  TGC2.3.4, TGC2.5.2, path-fixed HPM and DoD.
- **4 current V2DLL loader probes** pass, including selected/unselected,
  mod-folder/dll_Submods, load-once and unsupported-host refusal.

Evidence includes `native-tests-024.txt`, `build-024.txt`,
`package-loader-024.txt`, `loader-probe-024.json` and native `text-*.txt`,
`input-position-asm.txt`, `input-texture.txt`, `display-lines.txt` exports.
The previous 0.2.3 redesign report and user-owned aesthetic handoff are
retained unchanged. A read-only live check was offered after observing no
game process; the user instead directed "ok. install it and ill check".
No fresh live input-memory snapshot was obtained.

## Package and installation

`work/province-rename/candidate/Province-Rename-Universal-0.2.4-test.zip`:
12 files; SHA-256
`829974db16c5d58b16b6bdd0619618347642fa284ba1bdaad2e8e75d07f70025`.
One additive Universal V2DLL package; requirements unchanged. Private
native-font preview: `work/province-rename/candidate/layout-preview-024-native.png`.
Current board `docs/province-rename-layout.svg`; previous board archived
verbatim at `docs/archive/province-rename-layout-023.svg`. External native
pixels stay versioned under ignored `work/`; no remote publication.

The user's fresh explicit installation instruction authorized this update
after the game was observed exited. This is **not recorded as a literal new
normal-exit confirmation**. The process gate remained clear throughout
installation. Only `work/Victoria-II-Province-Rename-TGC252` was updated;
all **11 payload** and **25,388 protected** hashes pass, protected files
unchanged. Full 0.2.3 payload and guide retained in
`centering-024-1021/before-024/`; exact record
`centering-024-1021/patch-024-installed.json`. All prior rollbacks retained.
No game launch/termination, other-copy/main E:/D: update, publication,
source commit, push or memory update occurred.

## Retry and remaining limits

Follow `instructions-rename-024.md`. Expect 0.2.4 Universal, 72 guards and
`Rename centered input enabled; single native text/caret origin`.
Check centering/flicker while typing, deletion, mouse caret and Home/End,
idle positioning, Cancel/reopen and short Rename/map/header update. Real
GUI/keyboard/raster timing acceptance is still pending. Extended limits,
lifetime, save/reload, other-base gameplay, ATCU coexistence and multiplayer
remain unproven. Earlier fixture passes must not be presented as runtime
acceptance.
