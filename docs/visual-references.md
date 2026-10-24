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

## TPL-11 Gather at mockup (2026-10-24)

mockups/5-templates-gather-1024.png and 5-templates-gather-selected-1024.png:
the 0.11.3 Templates panel with `Gather at < Best supply >` on the
`Recruit from` line. Accepted by the user 2026-10-24; Arial, not a capture.
The "selected" board is out of date: 0.11.5/0.11.6 replaced the map selection
with the land rally points (report-tpl-11-rally-1024.md §4).
