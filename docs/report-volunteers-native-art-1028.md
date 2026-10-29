# VOL-UI: native assets, readable labels and flush docking

2026-10-28. The user supplied a live0.0.4 screenshot, rejected the flat
placeholder appearance, requested in-game assets and a flush native look, then
confirmed the game closed. Rust0.0.5 is built and installed into the same
E:\Victoria-II-Volunteers-0004 test copy, with a complete prior-package backup.
No new game launched or desktop input sent during this update.

## What the screenshot establishes

The native Diplomacy entry and both independent volunteer roots were displayed
in the user's campaign. Army/recipient names and selected brigade rows were
present; two rows showed selected. This supports a short native UI/display
observation for0.0.4. It does not establish full controls/lifetime/crash safety,
save/reload, sending, combat, costs, AI exclusion or multiplayer.

Visible failures: flat generated textures, raw tgcx_vl_* localisation keys,
blank button captions and large windows over the centre of Diplomacy. Raw
capture preserved unchanged at work/volunteers-0005/user-0004-placeholder-reference.png.
The precise cause of missing static localisation in this run was not established;
the corrected UI does not depend on that lookup for visible captions.

## Changes

- Reused the accepted PRU/Conform private art-preparation approach. The chooser
  and forces backgrounds now use native332px unit-panel gold braid, green
  header, textured parchment and recessed footer. Native regiment bands frame
  selectors, brigade rows and mission entries.
- Native unit close icon, native arrow glyphs and native148x28 footer buttons
  replace the flat squares. The closed entry uses the actual thin160px
  diplomacy button texture. No procedural flat panel/button palette remains.
- Titles use vic_18, with native Arial12/14 and vic_18_black labels matching
  the compact army UI. All font names resolve in the inspected effective base.
- Plain native buttons have separate, uniquely named click-through textboxes,
  following the accepted Conform/PRU pattern (P41/P59). ButtonText was blank in
  the user's screenshot; runtime now explicitly sets the visible captions.
- All static labels and dynamic content are set through the native text path;
  GUI fallback text is empty, so unresolved localisation keys are not painted.
  A per-window text cache avoids rerasterizing unchanged strings every frame
  and clears on retirement/recreation. Localisation remains additive/cp1252.
- Dock both panels flush against each other and beside Diplomacy, aligned to
  the host's top. Right side is preferred when it fits; left side otherwise.
  On a narrow display with no full side space, the pair uses a bounded fallback
  over the host. No off-screen placement or universal side-fit is promised.
- Read actual host width at runtime+4A0 (P61), retaining the validated owner-chain
  screen coordinates (P40). Removed the old fixed+700/+500 entry placement and
 22px gap between panels. Entry docks beside the host and hides while open.
- Native type preflight, observer lifetime, deferred release, thread/SEH/FPU/
  patch guards and closed gameplay/session gates retained. No simulation or
  native ownership change. No new RVA was introduced.

## Native source assets

Generated privately from the effective TGC2.5.2/vanilla file set in the test copy:

| Native declaration | Inspected frame size | Use |
|---|---|---|
| GFX_unit_bottom_bg |332x408| Parchment, side braid, footer |
| GFX_unit_toppanel_bg |332x64, two-frame664px strip| Green title and gold header |
| GFX_unit_entry_bg |317x30| Brigade, selector and mission bands |
| GFX_unit_gunboat_diplo_button |148x28| Footer actions |
| GFX_unit_close |20x20| Close |
| GFX_button_arrow_left/right |32x32, scaled24x24 like Conform| Selectors/paging |
| GFX_button_thin_160 |160x24| Diplomacy entry |

tools/tgcx_volunteers_gui.py generates our own .gui/.gfx/localisation plus eight
private textures. No whole base GUI is copied. No base art or executable was
added to tracked source; derived pixels/package stay in ignored work/. The ZIP
is marked private; native art is not a new redistribution licence (R8).

The underlying layout retains two brigade rows and two mission rows,332x378 /
332x266 roots. Text selection markers remain; type/strength art, a native
checkbox-state control, country full-name lookup and disabled-state styling
are still not implemented. These gaps are not claims of exact native parity.

## Preview and checks

Updated the mockup in the same session. Old SVG/design archived verbatim under
docs/archive/*before-native-art-1028*. Current docs/volunteers-layout.svg embeds
the privately generated work/volunteers-0005/native-panels-preview.png.
Rendered using native bitmap-font atlases, not desktop substitute fonts;
visually inspected: readable labels, matching textures, native footer sizes,
aligned headers/edges and no overlapping labels. It is a generated design
preview, not a game screenshot or runtime docking proof.

- i686-pc-windows-msvc Rust0.0.5 release compilation/link succeeds; fmt applied.
- Reference executable/address gate ALL OK; existing40 scoped guards retained.
- Effective-base font lookup reports no missing fonts. Generated-name collision
  and localisation delimiter checks pass during staging.
- Exact17-file private ZIP CRC/payload manifest verified by packager.
- No new test suite, fixture, game launch or multiplayer check. Previous audit
  limitations remain. Source changes are not an in-game acceptance pass.

## Installation

User confirmed normal close. No v2game process was found at update preflight.
Only our package plus generated test descriptor label/README were updated in
E:\Victoria-II-Volunteers-0004. Same save profile and launch scripts retained.
Run **tgcx_start_preview.cmd** again to inspect0.0.5.

Rollback: work/volunteers-0005/before-0005 contains the complete0.0.4 volunteer
folder, generated descriptors/README and plugin-run Logs. Existing log originals
were preserved. The obsolete small.tga is retained inert in the installed folder;
the new .gfx does not reference it. No loaded file was changed while the game ran.

Fifteen new mod files match the staged manifest. Nine protected EXE/Lua/V2DLL/
settings/TGC GUI/map files were hashed before and after and are unchanged.
The generated test descriptor's display name now says0.0.5; canonical mod/TGC
and profile TGC252_Volunteers_0004_20261028 remain unchanged. The main/source
game installations were not targeted.

Installation evidence: work/volunteers-0005/install-result.json and
E:\Victoria-II-Volunteers-0004\tgcx_update_0005_manifest.json.
Exact private candidate: work/volunteers-0005/Volunteers-0.0.5-native-ui-private.zip.
ZIP SHA-256 6c758e6ca6f64c159dab318dac6bac3f82142ff29e68f2e04772d0893b126972.
DLL SHA-256 d431dc5fddea3e1e646d190b36f11f6dddaa346e9bf0e72318822fd12aeabe0a.

New native-art/docking/caption behavior awaits the user's in-game check.
Send/Recall remain disabled; no crash-free or multiplayer/OOS fix claim.
Current next steps: instructions-volunteers-native-art.md.
