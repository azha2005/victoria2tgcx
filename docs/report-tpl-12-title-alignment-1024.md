# TPL-12: live alignment correction and single-line titles (0.11.12)

2026-10-24, session 79. User reports 0.11.11 remains 1px misaligned and
supplies a second capture where **Hus wraps over the regiment count**.
**Rust 0.11.12 built, packaged and installed in the same test copy;
both corrections await the next player check.**

## Alignment: live pixels supersede the preview lift

Unmodified capture: `work/tpl12-01112/01111-capture.png` (2560x1440).
Original-resolution measurements in `pixel-evidence.json` show the first
continuous gold top-border row:

| Button | Capture Y |
|---|---|
| Replace | 184 |
| Conform | 184 |
| Native Disband depleted | 185 |
| Native Disband | 185 |

The requested lift was applied, but the earlier illustrative preview was
misleading. The live result is 1px too high. Remove y minus 1 and use the
native button's Y directly. TGC with depleted shown: Replace (172,32),
Conform (193,32); hidden: Replace (193,32), Conform (214,32). The existing
21px pitch, 2px painted gap and native-visibility-based shift are retained.

Current visual target: `mockups/6-replace-native-y-01112.png`. It composites
the unchanged actual DDS frames at the corrected position in the capture;
not a new gameplay screenshot. No new art or mechanic, no remote artifact
republishing. Earlier y=31 references remain historical.

## Conform titles: fit the loaded font to one line

The row title's 132px box let a long composition wrap past its 14px height,
over its separate count line. `title_label()` now reads that control's
actual loaded bitmap font and measures the complete displayed name.
If it exceeds 128px (4px allowance inside the 132px field), `fit_line()`
measures successively shorter prefixes plus **...** until one fits.
Short names stay complete; the count line keeps its current position.

This uses proportional glyph widths and kerning, not a character-count
limit. CP1252 bytes are preserved. Unexpected line breaks cannot leak a
second line. Font calls run under the existing guarded UI-thread path;
unknown methods/failed signatures do not call the font routine. Guard
failure logs the fallback; measurement failure suppresses the overflowing
title. The full composition, miniature unit counts, selection and stored
template data are unchanged. Only the Conform list is adjusted.

Three existing Rename evidence sites are promoted into the Army runtime
table: native width RVA 0x624E20 (VA 0x00A24E20), RET8 RVA 0x624F56,
and rendered-text font load RVA 0x65ED44 (inner+0x1BC). The loaded font's
slot10 must also point to the guarded width function. Existing evidence:
`docs/symbols.md` RENAME-UI; no new ABI or data-layout guess.

User capture: `work/tpl12-01112/01111-conform-wrap.png`; enlarged crop:
`conform-wrap-3x.png`. Illustrative corrected title layout:
`mockups/6-conform-title-single-line-01112.png` (Arial stand-in). Exact
ellipsis position is decided by the loaded game font at runtime. This
restores the accepted two-line row layout; it does not change the mechanic.

## Validation

- Executable: **332 check entries** pass, including the promoted sites
  also covered by Rename; Army runtime header: **206 signatures**.
- Rust x86 offline release build, format and strict all-target Clippy
  pass; **26 tests** pass. New cases cover long composition names,
  proportional widths, CP1252, newline removal and failed measurement.
  These are pure formatting tests with a width callback; the new Conform
  integration still requires gameplay verification.
- Python GUI **20**, Army **17**, installer/Lua **12** pass. The actual
  packaged Rust DLL passes the plugin probe. A test-module ordering lint
  was corrected before the final clean Clippy run.
- All **50** manifest/ZIP files match, CRC OK; **415** GUI names unique,
  declared textures resolve. Art and loader are byte-identical to 0.11.11.
  No C++ release build or logic extension.

## Package and installation

`work/tpl12-01112/ATCU-0.11.12-candidate.zip`, **1,136,400 bytes**, includes
README and changelog.

| Item | SHA-256 |
|---|---|
| ZIP | `acfc5f8a7bf69a1e988b0d874f6c7ec43f79c7bbf749278f6359c0cddefc88ca` |
| Logic DLL | `40ac14c3e7690c873b3821fc47ab2448066fea36ff72f68bcc9b26ed5dcb2645` |

Installed in `work/Victoria-II-TGC252-own-loader`. The game was initially
running, so implementation/build work proceeded first. Later process
checks found it closed, including immediately before the swap. No game
was launched or terminated; no separate normal-exit statement is claimed.

**48 payload hashes pass**; **25,466 protected files/eight settings
unchanged**. 0.11.11 rollback: `work/tpl12-01112/before-01112/`, **58**
verified hashes in its sidecar. Evidence: `installation-01112.json`.
Installed manifest updated. Main E: installation unchanged.

Next: `instructions-tpl-12-check.md` step 4. Check native border alignment,
both visibility states, long/short list titles and their separate counts,
page/selection changes and reopening. Replacement reload and multiplayer
remain open. No commit, push, public release or game control.
