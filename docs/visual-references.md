# Current interface references

## Province Rename Universal (user, 2026-10-21)

User accepts0.2.5: native white/green/gold title, continuous parchment,
double input braids, black centered Garamond typing, aligned native caret,
blue/gold buttons and24px native ABC right of province name. Latest tweak
0.2.6 raises ABC2px and changes its name gap8px to10px; final position itself
has native geometry checks but no new player appearance confirmation yet.
Accepted editor otherwise unchanged. Report-province-rename-package-1021.md.

Current board docs/province-rename-layout.svg, native332x224 editor/296x30
input; private font preview work/province-rename/candidate/layout-preview-026-native.png.
Native external pixels remain versioned under ignored work/province-rename/
mockup-assets-0.2.6/, never embedded in tracked source. The host preview is
illustrative, not a gameplay capture. Prior025/024/023/022/021/0100 boards
retained verbatim under docs/archive/, with external paths relative to the
original docs/ location. Historical remote artifact not republished.
Accepted025 screenshot/log retained under work/province-rename/release-026-1021/.
Local ZIP only; publication explicitly deferred by the user.

## Conform (user correction, 2026-10-19)

The current accepted reference is `mockups/6-conform-current-1019.png`, the
unaltered screenshot the user supplied while reviewing the Army compatibility
work. It shows the 332px native army-style Conform panel flush to the right of
the selected army: green header, native close button, parchment body, pale
regiment rows, composition names/icons, Changes section and native bottom
Conform/Cancel buttons. Retain this appearance across generated base packages.

The user explicitly says the old mockup is outdated. `mockups/6-conform.png`
and the old Conform board in the published design artifact are historical
references, not the current visual target. They are preserved; this session
does not claim the remote artifact was republished. The native appearance was
already accepted in report-tpl-1-visual-1017.md; the supplied current screenshot
clarifies its continued authority. No mechanic or layout redesign is requested.

Templates retains board 5 for its layout, with native art from the selected
base. Compatibility adds the base's unit rows/pages/icons; it does not introduce
the deferred culture filter. The current screenshot is ignored by git (R8).


2026-10-19, user-approved Conform restriction (D17): preview/recruit only
within land reach of the selected army, with shortages reported. 0.10.1
keeps layout/art, changes summary/shortfall wording to "within land reach".
Existing overseas troops are not moved automatically. Runtime visual
acceptance of the universal controls and wording remains pending.


## Current main-copy screenshots and filter layout (2026-10-19)

The three newer user captures are retained unaltered as:

- mockups/6-conform-main-current-1019.png: current Conform preview.
- mockups/6-conform-main-queued-1019.png: current Conform after queueing.
- mockups/5-templates-main-current-1019.png: current Military Templates.

They are the baseline for TPL-5/TPL-6. Previous boards remain historical.
Accepted Universal0.10.4 already passed the four-base short matrix; that
acceptance does not apply to the new filter layout.

New local references are mockups/5-templates-filters-1019.{png,svg} and
mockups/6-conform-filters-1019.{png,svg}. They use staged native-style art
and actual generated coordinates, with sample labels in Arial. They are
layout previews, not gameplay captures. The historical remote artifact
was not republished. All game-derived images stay ignored by git.

Builder stays652x528, eight composition rows; culture controls at442 and
continent in the right footer468. Conform stays332x594; culture at202,
Changes moved32px down and reduced to seven visible rows. Existing overflow
summary preserves totals. These deviations reserve room for the requested
filters without expanding the current frames. Player visibility, native
font layout and hit-testing acceptance remains pending on installed0.11.0.

## 0.11.1 filter GUI previews (2026-10-19)

mockups/5-templates-filters-1019b.png and mockups/6-conform-filters-1019b.png
supersede the 0.11.0 filter previews: native GFX_checkbox_default boxes with a
"Culture" caption (builder row without background, at the user's request;
Conform row keeps its regiment-row band) and the compact builder footer
`Recruit from < Any continent >`. Generated from staged art; not game captures.

## 0.11.3 Non-core previews (2026-10-24)

mockups/5-templates-noncore-1024.png and mockups/6-conform-noncore-1024.png
supersede the 1019b filter previews for TPL-8. Templates: three boxes on the
culture row (Primary only moved 204 to 196). Conform: the culture band has two
lines with Non-core only on line 2, and Changes drops from 7 to 6 visible rows.
Rendered by work/noncore-0113/render_previews.py from staged art (Arial); these
are not game captures.

## TPL-12 Replace depleted mockup (2026-10-24)

mockups/6-conform-replace-1024.png (Conform footer: Conform 92 / Replace
depleted 116 / Cancel 92) and 6-conform-replace-choose-1024.png (the
separate chooser window) are accepted. 6-conform-replace-docked-1024.png was
rejected. Drawn by work/tpl12/render_replace.py from staged art (Arial), not
captures. Built in 0.11.7; deviations (stepped slider, list wording) in
report-tpl-12-replace-1024.md §4.

## TPL-11 Gather at mockup (2026-10-24)

mockups/5-templates-gather-1024.png and 5-templates-gather-selected-1024.png:
the 0.11.3 Templates panel with `Gather at < Best supply >` on the
`Recruit from` line. Accepted by the user 2026-10-24; Arial, not a capture.
The "selected" board is out of date: 0.11.5/0.11.6 replaced the map selection
with the land rally points (report-tpl-11-rally-1024.md §4).

## TPL-12 option A, current (2026-10-24)

User selected A: `mockups/6-replace-button-1024.png` and
`6-replace-button-header-1024.png` supersede the footer-button mockup.
Two 16px header buttons, 3px gap: maroon Replace left, green Conform right;
both above the braid. Independent 332x270 chooser at Conform's normal
position, with its own exclusive culture boxes. Conform has its original
two footer buttons. The previous chooser beside Conform is historical.

0.11.8 actual packaged-art previews: `mockups/6-replace-whole-candidate-0118.png`,
`6-replace-header-candidate-0118.png`, `6-replace-candidate-0118.png`.
Rendered from generated coordinates with Arial, visually inspected; not
game captures. Header matches A. Report: `report-tpl-12-header-1024.md`.
Historical remote artifact not republished; live appearance remains untested.

## TPL-12 native-sized buttons beside native actions, current (2026-10-24)

User accepts 0.11.8 short check; asks for native Disband size and clarifies
**not above, right beside**. Current reference is
`mockups/6-replace-beside-20px-1024.png` and its detail variant: both 20x20,
3px gaps, same header row, immediately left of native actions. TGC Replace
(168,32), Conform (191,32), anchored to native disbandbutton. Native name
width is unchanged; long names have less room. The chooser is unchanged.

The unmodified 0.11.8 capture is private in `work/tpl12-0119`; these new
references are actual 0.11.9 DDS composites, not live captures. The earlier
above-header draft is superseded and retained, as are the 16px option A
references. New placement awaits game check. `report-tpl-12-beside-1024.md`.

## TPL-12 native spacing and adaptive slot, current (2026-10-24)

The user supplies healthy/depleted 0.11.9 captures, asks for the native gap
and a shift into the hidden Disband depleted slot, then a 1px upward move.
Current `mockups/6-replace-spacing-aligned-states-1024.png` and aligned
healthy/depleted variants show both final 0.11.11 states. 20px textures,
21px pitch, 2px painted gaps; TGC y=31. Shown-depleted x=172/193; hidden
x=193/214. Unchanged native actions and unchanged chooser.

Actual-art composites, with cleaned header pixels from an earlier capture;
not new game screenshots. Unmodified user captures remain under
work/tpl12-01110. Pre-lift previews are retained under 6-replace-spacing-*.
Live alignment/visibility movement pending. report-tpl-12-spacing-1024.md.

## TPL-12 live alignment and Conform one-line titles, current (2026-10-24)

0.11.11 live pixels supersede the y31 composite: ours at y184, native y185.
Current 0.11.12 uses native Y (TGC y32); 21px pitch and visibility shifts
are unchanged. `mockups/6-replace-native-y-01112.png` composites the actual
unchanged frames at corrected coordinates. Raw capture and measurements:
work/tpl12-01112/01111-capture.png and pixel-evidence.json.

Second raw capture shows Hus/count overlap in Conform. The accepted row
retains a one-line title above its count. Native loaded-font measurement
and ellipsis now enforce that layout. Preview:
`mockups/6-conform-title-single-line-01112.png`, Arial stand-in in the
captured row; not new gameplay proof. Stored composition is unchanged.
Current report-tpl-12-title-alignment-1024.md; player retry pending.

## 0.11.12 alignment/title visual acceptance (2026-10-24)

User confirms "it worked", explicitly retaining the need for more testing.
Accept the native-Y header and single-line title correction as a short
visual check. No new final screenshot supplied. Latest log contains no
GUI/font fault; it cannot independently prove every alignment/visibility
case. Functional/reload/other-base/MP checks remain open.
report-tpl-12-visual-check-1024.md; logs under work/tpl12-01112/check-1024.


## Volunteers (working design, 2026-10-27)

The user selected a separate volunteer submod in PRU/ATCU style.
`docs/volunteers-layout.svg` proposes332px green/gold/parchment panels;
`work/volunteers-0001/volunteers-layout.png` is its rendered preview. The accepted
Conform screenshot was opened as a visual reference. This is a new local
proposal, not a gameplay capture or user-accepted volunteer UI. The0.0.1 DLL
is observe-only and adds no native window. No existing remote board changed.

2026-10-27 donor decision: retain the same chooser/active-forces layout, update
text for donor ownership, supply, flag/statistics and exclusive sending-player
command. Current preview work/volunteers-0002/volunteers-layout.png was rendered
and visually checked. Old SVG archived verbatim. Maxioten's screenshot is kept
privately at work/volunteers-0002/maxioten-reference.png; it shows foreign-unit
selection, not proof of billing/battle/AI/MP semantics. Runtime0.0.2 still has no
Send or loan UI enabled. See report-volunteers-donor-1027.md.

2026-10-27 native coding: same panel dimensions/style retained in 0.0.3,
with recipient/brigade/mission paging arrows and current development labels.
Own procedural assets/additive GUI are generated; this is compiled code and a
mockup update, not an in-game visual pass. Entry anchor, availability styling,
native wrapping and callback/lifetime behavior remain unverified. Tests/game
checks are deferred. report-volunteers-implementation-1027.md.

2026-10-27 pitfall audit0.0.4: panel structure retained. Native small-text font
changed from absent vic_14_black to reference-declared Arial14_black; appearance
is not runtime-verified. SVG labels updated for observation default/opt-in UI;
export work/volunteers-0004/volunteers-layout.png inspected. Optional Inkscape
AppData log/recent-file writes failed, but PNG export completed. Old SVG/design
archived verbatim before the update. No game screenshot or UI acceptance.

2026-10-28 native-art correction D23: user0.0.4 screenshot rejects flat panels,
raw localisation keys and blank buttons. Source screenshot retained unchanged
under work/volunteers-0005. Current0.0.5 native-panel preview uses actual
installed game textures and BMFont atlases; inspected at native332px widths.
docs/volunteers-layout.svg now embeds that private generated image. Old SVG/
design archived verbatim. Runtime panes dock flush beside Diplomacy; no new
game screenshot or native positioning acceptance yet. The E: test copy is
updated after confirmed close, with0.0.4 rollback. See report-volunteers-native-art-1028.md.

2026-10-29 Volunteers entry correction: the user's cropped reference places Send
volunteers in the vacant bottom action cell beside Command Units. Generator uses
GFX_button_thin_160160x24/vic_18_black, not custom placeholder art. Runtime follows
native action0..N rows. Current SVG embeds the new native-art detail plus retained
two panels from work/volunteers-0007. Prior SVG/design archived verbatim. The user
reported earlier UI working; new position/gameplay were not tested by this agent.

2026-10-29: user accepts Volunteers native appearance.0.0.8 preserves these panels/assets; source mockup volunteers-layout.svg records whole-stack retention versus partial split. No new visual layout or art is introduced.
