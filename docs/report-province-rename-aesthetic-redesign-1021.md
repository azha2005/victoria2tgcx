# Province Rename: Aesthetic Redesign & Native Asset Alignment

**Date:** 2026-10-21  
**Target:** Province Rename Universal (0.2.2 -> 0.2.3 candidate)  
**Status:** UI Redesign Completed, visual previews rendered, handoff prepared.

---

## 1. Context & User Feedback

Following initial in-game testing of Province Rename Universal 0.2.2, the user provided live feedback and screenshots identifying specific visual issues that broke immersion and aesthetic harmony with Victoria II's native Victorian UI:

1. **"weird bar on top of the buttons"**: A thin dark horizontal line (`GFX_unit_bg_28` composite slice at `y=170`) divided the panel awkwardly right above the action buttons.
2. **"the text input frame still looks odd - it lacks the braids at the bottom"**: The input box frame only featured gold braid along the top edge, leaving the bottom edge unfinished and unbraided.
3. **"the buttons still look bad with that background"**: The background panel had an abrupt, muddy dark-brown footer shelf pasted at `y=189` (`body.crop((0, 373, 332, 408))`), causing the buttons to sit awkwardly on a harsh two-tone band.
4. **"center the input text in the input bar"**: Request to center the user input string within the input box.
5. **Modern font clash**: Body and captions were using modern sans-serif fonts (`Arial12_black`, `Arial14_black`) instead of period serif typefaces.

---

## 2. Redesign & Native Asset Solutions

All visual assets and fonts used are strictly authentic in-game assets resolved from the reference game files without external art or unauthorized third-party pixels (R8):

### A. Background Panel (`panel_clean.png`)
- **Eliminated the dark footer shelf**: The panel body is now rendered from continuous, warm cream parchment (`GFX_unit_bottom_bg` cropped `y=80..304`) with only the bottom border (`y=400..408` pasted at `y=216`).
- **Eliminated the weird divider bar**: Removed `GFX_unit_bg_28` at `y=170`. The entire dialog interior is a unified, elegant parchment card.
- **Preserved native green header**: Retains the authentic green title bar with top and bottom gold braid and `vic_22` cream title ("Rename province").

### B. Double-Braided Input Frame (`field_double_braid.png`)
- Analysis of native `GFX_bp_textedit_bg.dds` revealed that the sprite file contains a top-braid half (`y=3..20`) and a bottom-braid half (`y=58..75`).
- Combined both halves into a 198×30 px frame source, then nine-sliced to `296×30 px`.
- **Result:** The text field now features authentic beaded gold braids along **both top and bottom edges**, with filigree corners and a recessed inner shadow.

### C. Authentic Period Typography
- Removed all modern Arial fonts.
- **Section Captions ("Current name", "New name"):** `Main_16_black` (native `garamond_16_bold`).
- **Current Province Name ("La Plata" / "Caen"):** `vic_18_black_bold` (centered).
- **Input Text:** `vic_18_black` (serif Victorian lettering).
- **Status / Limit text ("Up to 80 characters."):** `Main_14_black` (native `garamond_14_bold`).
- **Button Labels:** `vic_18_black`.
- All body fonts strictly end in `_black` and are verified available across vanilla, TGC 2.3.4, TGC 2.5.2, HPM 4.5, and DoD Fan Fork.

### D. Native Button Options (`work/btn_options_comparison.png`)
Generated a side-by-side comparison on the clean parchment:
1. **Option 1 (`GFX_button_standard_144`):** Authentic 144×34 px dialog button with gold corner filigree and beveled edges (matching Province View's `Build Army` / `Build Navy` buttons).
2. **Option 2 (`GFX_button_standard_111`):** 111×34 px compact variant providing extra breathing room on the margins.
3. **Option 3 (`button_type_1`):** Gold-brass plate with brass corner rivets, echoing the header and input box gold braids.
4. **Option 4 (`GFX_button_thin_120`):** 120×24 px thin variant from Province View's `Expand` control.

---

## 3. Preserved Evidence & Preview Files

- `work/field_double_braid.png`: Nine-sliced text input frame with double gold braids.
- `work/panel_clean.png`: Clean parchment panel with no footer shelf and no divider bar.
- `work/preview_clean_panel.png`: Complete redesigned 0.2.3 candidate layout.
- `work/btn_options_comparison.png`: Four-way button comparison matrix on clean parchment.

---

## 4. Next Session Handoff

1. **User Decision:** Confirm preferred button style from the comparison matrix (Option 1 `GFX_button_standard_144` vs Option 3 `button_type_1` gold plate).
2. **Input Alignment:** In Clausewitz GUI engine, `editBoxType` text origin is offset by `borderSize = { x=... y=... }`. Adjust `borderSize` or wrapper centering based on testing.
3. **Packaging & Installation:**
   - Run `tools/build_province_rename.ps1` to rebuild the 0.2.3 DLL with MSVC x86.
   - Run `python tools/test_province_rename.py` (verifies all schema, package, font, and base-matrix checks).
   - Execute native test binary `tgcx_rename_tests.exe`.
   - Gate installation behind a verified game exit.
