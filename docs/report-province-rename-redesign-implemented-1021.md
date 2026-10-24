# Province Rename 0.2.3: native redesign implemented and installed

2026-10-21. **Implemented, checked and installed; the new appearance and
interactive centering still require player verification.** The user's
`report-province-rename-aesthetic-redesign-1021.md` handoff is retained
unchanged. Its previews were starting material, not the installed package.
The latest explicit instruction, "typed text should be center aligned",
supersedes the left-aligned typing shown in the supplied reference image.

## Appearance and reference

The editor remains 332x224 and the independent ABC entry remains 28x28,
right of the measured province name. The native command and proven province
anchor are unchanged. Changes follow the supplied La Plata reference:

- White native `vic_22` title directly on green, with native gold braid.
- Continuous native parchment; remove the divider above the buttons and
  the dark footer shelf. Retain only the bottom border at y216..224.
- Double-braided 296x30 input frame at (18,108).
- Centered black period serif captions, bold current name and status.
  Fonts: `Main_16_black`, `vic_18_black_bold`, `Main_14_black` and
  `vic_18_black`; button captions use the latter.
- Actual native blue `GFX_button_standard_144` buttons with gold corners,
  144x34 at (18,178) and (170,178). This is the handoff's option 1 and
  matches the user's supplied screenshot.
- Typed names center horizontally and vertically as they change. Names
  wider than the available space retain minimum native padding instead.

Native input art resolves to an actual 208x80 source. The handoff's 198px
width is the cropped content width: crop (5,3,203,20) and (5,58,203,75),
overlap the two 198x17 pieces into 198x30, then nine-slice to 296x30 with
6px borders. No generated artwork replaces the game's original assets.
All extracted/generated game pixels remain under ignored `work/`.

`tools/prepare_province_rename.py` generates package art, GUI, previews and
the local SVG board. The preview reads the generated GUI coordinates and
actual native bitmap-font metrics. Current board:
`docs/province-rename-layout.svg`; old 0.2.2 board archived verbatim at
`docs/archive/province-rename-layout-022.svg`. Native exports are under
`work/province-rename/mockup-assets-0.2.3/`. The full preview is
`work/province-rename/candidate/layout-preview-023-native.png`; the editor
alone is `work/province-rename/redesign-023-1021/editor-preview-final.png`.
The host panel in the full preview is illustrative. Neither preview is a
gameplay capture, and the historical remote artifact was not republished.

## Native input centering and evidence

A native `editBoxType` has no established center-alignment setting here.
The reference executable was checked before address work. Read-only Ghidra
exports trace constructor, input buffer, display refresh and caret paths.
New contracts are recorded in `docs/symbols.md`, RENAME-5.

`FUN_009B6400` (RVA 0x5B6400) is `stdcall(edit)`, with RET4. It passes the
edit control's +0x150/+0x154 border insets to the native text renderer.
The caret path in `FUN_009B5C70` uses those same insets. The embedded buffer
is edit+0x54; its native constructor stores input context at buffer+0x74
and font at buffer+0x78. These are distinct fields, independently checked
by executing the original constructor in the native fixture.

The new refresh detour measures this editor's current string with the
actual native proportional font width/kerning and line-height methods.
It sets X to max(8,(296-width)/2), and Y to (30-height)/2 bounded to 2..10,
before forwarding to the original refresh. It preserves registers, flags,
FP/SSE state and last-error state. It accepts only our exact edit pointer,
UI thread, expected native class/buffer tables and 296x30 dimensions.
Other inputs pass through unchanged. Cursor, selection, usable buffer
dimensions and keyboard editing stay under native control.

This changes GUI-only insets, not simulation state. Rename still queues
the engine's `CChangeProvinceNameCommand`, command 0x707. Seven additional
relocation-aware signatures cover the new field/table/ABI evidence:
**66 Rename runtime guards, 289 total**, including the unchanged 179 Army
and 44 earlier research guards. A refused centering detour logs its refusal
and leaves native input available. Do not treat that fallback as centered
input acceptance.

## Validation and its limits

- Reference executable and all 289 guards pass; generated header current.
- MSVC Win32 build passes. **63 native checks, zero failures**. Ten new
  checks cover the actual buffer constructor, font metrics, short/empty/
  accented/overflow strings, unchanged cursor/dimensions, unrelated fields,
  missing-font refusal, and the naked wrapper forwarding after centering.
- **4 package tests** pass, including unique names and static font/sprite
  resolution for vanilla, TGC 2.3.4, TGC 2.5.2, path-fixed HPM and DoD.
- **4 current V2DLL loader probes** pass: selected/unselected package,
  mod-folder/dll_Submods paths, load-once and unsupported-host refusal.
  These prove loader behavior, not the complete game GUI.
- Native asset/font preview inspected against the reference. Input shows
  the requested horizontal centering instead of the reference's left inset.

Native code inspection and isolated execution support the shared text/caret
inset contract; actual keyboard, mouse caret positioning, scrolling, focus
and appearance with the complete game remain unverified for 0.2.3. Earlier
0.2.1 short rename success is retained but does not establish extended
limits, lifetime, save/reload, other-base gameplay, ATCU coexistence or MP.
No new observed pitfall was added; P59-P61 and the ATCU lifetime/coordinates/
factory/signature pitfalls were reviewed. Next pitfall remains P62.

Evidence folder: `work/province-rename/redesign-023-1021/`, including
`native-tests-023.txt`, `loader-probe-023.json`, `build-023.txt`, the original
user target and read-only `edit-*.txt` native exports.

## Candidate and installation

ZIP: `work/province-rename/candidate/Province-Rename-Universal-0.2.3-test.zip`.
12 files, 263045 bytes; SHA-256:
`7c0dd02cc59a646bdcb8932eeea843030a0d968a6a09d0613496ab6cb32540ea`.
The 11-file additive mod payload has no dependencies, user_dir,
replace_path or base GUI override. V2DLL/reference 3.04 remain required.

After the user freshly confirmed "Closed normally now", installation
checked the process gate and updated only
`work/Victoria-II-Province-Rename-TGC252`. All **11 payload hashes** and
**25,388 protected hashes** pass; protected files unchanged before/after.
Current V2DLL 5.98/cdad55c7, stock Lua, canonical mod/TGC and independent
profile `TGC252_ProvinceRename_20261020` remain in place.

Record: `redesign-023-1021/patch-023-installed.json` beneath the evidence
root. Full previous 0.2.2 payload and guide retained in `before-023/`.
Earlier backups remain intact. No game launch/termination, main E:/D:
installation, other-copy update, release, publication, source commit, push
or memory update occurred.

## Next player check

Follow `docs/instructions-rename-023.md`. Expect `0.2.3 Universal`,
`66 native signatures OK` and, after opening the editor,
`Rename centered input enabled; shared native text/caret inset` in
`Logs/tgcx_province_rename.log`. A centering-hook refusal is a failed
centering check. Preserve unedited screenshots and logs after normal exit.
