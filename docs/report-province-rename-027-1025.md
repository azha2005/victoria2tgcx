# Province Rename 0.2.7: smaller, level ABC and local ZIP

2026-10-25. The user reports the 0.2.6 ABC as "a bit too low and big"
(screenshot: TGC, Normandie / province name row). 0.2.7 changes only the
entry button:

- **Size 24x24 -> 20x20**, the native Disband size ATCU already uses (D21).
  Same native `GFX_bp_button_rename` 28x28 source, LANCZOS resize.
- **Vertical:** `ey = name_y + (name_h - 20)/2 - 5` (0.2.6: `(h-24)/2 - 2`).
  The top edge is 1px higher and the centre 3px higher than 0.2.6.
- Gap after the measured name stays 10px; right-edge clamp, fallback,
  editor placement, typing/caret and the native command are unchanged.

## Validation

- `check_exe.py` ALL OK; 78 Rename signatures OK.
- MSVC Win32 build; `tgcx_rename_tests.exe` 66 checks, 0 failures (anchor
  expectations moved 1px up; fallback x +4 for the narrower button).
- `test_province_rename.py --v2dll-loader`: 4 tests OK, loader probes OK.
- Preview `work/province-rename/candidate/layout-preview-027-native.png`
  (generated, not a gameplay capture); board `docs/province-rename-layout.svg`
  regenerated, 0.2.6 board kept in `docs/archive/province-rename-layout-026.svg`.
- **Not tested in game.** Nothing was launched or published.

## Installation

User confirmed "the game is closed normally" and asked to install 0.2.7 in
the dedicated copy with a 0.2.6 backup. Process gate clear;
`work/province-rename/release-027-1025/update_027.py` (from `update_026.py`)
verified the installed 0.2.6 payload and guide, 25,388 protected hashes and
the target executable gate, backed up the 12 files to `before-027/`, then
installed the 11-file payload and the 0.2.7 guide. Payload and protected
hashes re-verified; record `patch-027-installed.json`. Earlier backups kept.
In game, expect `0.2.7 Universal` and 78 signatures in
`Logs/tgcx_province_rename.log`.

## 0.2.8: 2px back down

User's 0.2.7 screenshot (Orleanais / Chartres): "slightly too up". 0.2.8
keeps 20x20 and changes the offset to `(name_h - 20)/2 - 3`: 2px lower than
0.2.7, centre 1px above 0.2.6. Anchor expectations moved 2px; check_exe ALL
OK, 78 signatures, 66 native checks, 4 package tests and loader probes pass.
Board regenerated, 0.2.7 board in `docs/archive/province-rename-layout-027.svg`.

`work/releases/province-rename/0.2.8/Province-Rename-Universal-0.2.8.zip`:
12 files, 264141 bytes; SHA-256
`c5c4417206cc46d20cf39c0e0144c3cfa6638c89bf51c6c72625caf300d0dd92`
(candidate `62b0ba51…5dfe`). Notes `docs/release-province-rename-0.2.8/`.
**Installed** in the dedicated copy 2026-10-25 after the user's fresh "the
game is closed normally" confirmation: `release-028-1025/update_028.py`
verified the 0.2.7 payload/guide, 25,388 protected hashes and the executable
gate, backed up 12 files to `before-028/`, installed and re-verified;
record `patch-028-installed.json`. Not tested in game; not published.

## Deliverable

`work/releases/province-rename/0.2.7/Province-Rename-Universal-0.2.7.zip`:
12 files, 264141 bytes; SHA-256
`f266090acb07e51cd2740a815f8b283618c8331a01af7ce101d7ce2c54c757c4`.
Candidate `work/province-rename/candidate/Province-Rename-Universal-0.2.7-test.zip`,
SHA-256 `fecdcac3ca7ed70f43e91888b4a1a7cbb3a39b4a741fbc4efc21f75b53508fe3`.
README/notes: `docs/release-province-rename-0.2.7/`. `RELEASE.json` records
`published=false`.
