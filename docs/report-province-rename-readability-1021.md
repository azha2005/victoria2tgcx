# Province Rename 0.2.5: caret, ABC size and input readability

2026-10-21. **Checked and installed in the dedicated TGC/V2DLL copy;
player verification pending.** User reports 0.2.4 works almost perfectly:
centering no longer has the previous failure, but the caret is displaced,
ABC is slightly too large and input text is too thin/hard to read. The
unaltered screenshot and 0.2.4 log are retained under
`work/province-rename/readability-025-1021/`. The log confirms 72 guards,
centering activation, a native rename/readback/header refresh for province
425 and reopen. This remains a short player check, not full persistence
or lifetime validation.

## Changes

- **Caret:** keep the native sprite and blink/input handling. The game's
  `cursor` image is 32x32 but its visible line occupies only
  (7,6)..(8,22), a 1x16 strip. The native caret pixel getter adds another
  3px gap. CEditBox's existing caret-only fields +0x184/+0x188 compensate
  for that transparent margin and gap: X = 1-7-3 = -9; Y =
  (font line height-16)/2-6. The visible line now sits one pixel after the
  measured text position and is centered within the font row. The shared
  text origin, working single-offset refresh and normal blink stay intact.
- **ABC:** reduce the native framed sprite from 28x28 to **24x24** with
  Lanczos resampling. Both the generated root/hit region and native anchor
  dimensions use 24px, preserving right-of-name placement and centering.
- **Input font:** replace thin, smoothed `vic_18_black` with the existing
  native **Main_16_black / Garamond 16 bold**, already used for captions.
  Its bitmap uses stronger strokes. Center using its actual native font
  metrics (16px line height); initial border Y becomes7. Captions, buttons,
  panel artwork and all simulation behavior otherwise remain unchanged.

No new runtime hook was added. Only this validated editor's caret-offset
fields change, during its existing native refresh/position path. Six new
relocation-aware guards cover native constructor caret offsets, exact
getter/gap and native caret update/ABI evidence: **78 Rename runtime
guards, 301 total**, Army179/research44 unchanged. The reference executable
was checked before address work. Details and confidence: symbols.md
RENAME-7. P63 records the visible-sprite-bound pitfall; next P64.

## Validation and the test error shown to the user

The first added native caret regression called `FUN_009B6160` as a method
without a stack argument. Its actual epilogue is **RET4**: it consumes a
frame argument even though the decompiler did not identify one. That
fixture call corrupted its own stack and produced the user's
`tgcx_rename_tests.exe - Application Error` dialog. This was the isolated
test executable, not Victoria II; the incomplete build was never installed.
Only the verified task-owned test process was closed. P31 already covers
this ABI class of mistake. The call now supplies the argument, and its
epilogue is guarded. Test-only exception reporting prevents a failed native
fixture from leaving another Windows error dialog awaiting dismissal.

The regression executes the actual native caret update and pixel getter
with a seeded native cursor-position cache, intercepting only the clock
and final positioning/GPU endpoint. It checks visible sprite ink rather
than the 32x32 container and verifies short/empty/proportional/accented
positions at both 16px and 18px font heights. This establishes the native
offset-consumer contract; it does not simulate all keyboard/cache updates.
The 0.2.4 actual refresh/texture/placement regression remains intact.

- **66 native checks, zero failures**, including actual command/registry/
  allocation/anchor/font/refresh/placement/caret contracts.
- MSVC Win32 build, **301 executable guards**, generated header pass.
- **4 package tests** pass, including schema, exact derived native art,
  static fonts/sprites and matching native cursor ink bounds for vanilla,
  TGC2.3.4, TGC2.5.2, path-fixed HPM and DoD.
- **4 actual current V2DLL loader probes** pass, including inactive/active,
  mod-folder/dll_Submods, load-once and unsupported-host refusal.
- Native-font preview inspected with the user's sample text. The stronger
  black input font remains centered inside the unchanged double braid.

Read-only live input inspection was attempted while the game was exiting;
no valid module snapshot was obtained. No live memory writes, injected
calls, game launch or game termination occurred. Current evidence is static
native analysis, isolated execution, asset checks and the user's capture.
Full caret/mouse/typing/scrolling and final visual acceptance remain for
the player. Other-base gameplay, ATCU coexistence and MP remain untested.

Evidence: `work/province-rename/readability-025-1021/`, including
`native-tests-025.txt`, `build-025.txt`, `package-loader-025.txt`,
`loader-probe-025.json`, native cursor PNG/bounds, `caret-fields-asm.txt`,
`caret-getter-abi.txt`, `caret-getter.txt`, user targets and 0.2.4 log.

## Package and installation

ZIP: `work/province-rename/candidate/Province-Rename-Universal-0.2.5-test.zip`.
**12 files, 263759 bytes**; SHA-256
`9ffd4a032687348ab88285d519350f47099f77e7bbb2a29994f7907acf0d3c24`.
Requirements and additive package structure unchanged. Current board:
`docs/province-rename-layout.svg`; previous 0.2.4 board archived verbatim
at `docs/archive/province-rename-layout-024.svg`. Native pixels remain
external under ignored versioned `work/` assets, never embedded in tracked
source. Preview `work/province-rename/candidate/layout-preview-025-native.png`
and editor-only `readability-025-1021/editor-preview.png` are generated
references, not gameplay captures. No remote artifact publication.

The user answered "done" to the fresh normal-exit confirmation for this
exact update. After the process gate, installed only in
`work/Victoria-II-Province-Rename-TGC252`. All **11 payload hashes** and
**25,388 protected hashes** pass; protected files unchanged. Full 0.2.4
payload and guide retained in `readability-025-1021/before-025/`; exact
record `readability-025-1021/patch-025-installed.json`. Previous backups
remain intact. Stock Lua, canonical TGC, V2DLL5.98/cdad55c7 and independent
profile retained. Main E:/D: and other copies untouched. No source commit,
push, release, publication or memory update.

Next: `instructions-rename-025.md`; expect 0.2.5 Universal, 78 signatures
and `single native text/caret origin` in the Rename log. The latest user
capture accepts short 0.2.4 operation; do not label the new 0.2.5 appearance
accepted before the player sees it.
