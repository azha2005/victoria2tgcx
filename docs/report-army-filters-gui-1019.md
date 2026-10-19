# Army filters GUI refinement (0.11.1)

2026-10-19. Internal candidate **0.11.1**, built, automatically checked,
installed in the isolated TGC copy, **not tested in the game**. Supersedes the 0.11.0 filter
controls described in `report-army-filters-implementation-1019.md`; that
report is retained.

## 0.11.0 check (user's run, before the change)

The isolated TGC copy's `Logs/tgcx.log` (2026-10-19 04:53) shows the
0.11.0 marker, every runtime and TPL-5/TPL-6 guard OK, the v1 settings
import with the original kept, and three Build Army recruitments (1, 10 and
10 orders queued, none left out). The log does not record which culture or
continent choices were active, so it does not prove the filters themselves.
No errors were logged.

## User requests

- The filter GUI looked plain: refine it.
- Offer only the continents the country is present in.
- Builder culture row: no background, text and boxes only.

## Changes

- **Native checkboxes.** Both windows use `GFX_checkbox_default` (vanilla
  `gfx/interface/checkbox_default.dds`, 40x20, two frames; the same art as
  "Auto-Create Leaders" in the Military window) instead of the Pillow box
  with a text "x". The off frame is painted into the background; the "on"
  frame is an icon in a window beneath the button, shown by the DLL; the
  button carries only the pixels both frames share, so it stays on top for
  clicks. Fallback art only if a base lacks the texture. The check text
  controls are gone.
- **Culture caption.** A bold "Culture" label leads the row in both windows.
  Builder: row at y439 under the eight unit rows, no band (user). Conform:
  a regiment-row band at y198 above Changes (kept; seven change rows).
- **Continent selector.** Builder footer reads `Recruit from < Any continent >`
  with the arrows 126px apart (was 395px). The label shows "Any continent",
  the continent's display name, or "Unavailable".
- **Present continents only.** The arrows skip continents with no province
  the country owns (country +0x9D8/+0x9DC ids through g_world+0xACC, the
  scan's existing reads, and province +0x16C, the TPL-6 continent field).
  If that read fails every continent is offered. A selection made earlier
  stays shown after its last province is lost; recruiting then reports the
  shortage. The filter itself is unchanged.
- **Guard failure.** If the TPL-5/TPL-6 guards fail, Army now stays on with
  filters disabled: unrestricted recruitment works, a restricted template
  or continent refuses (never widens). 0.11.0 turned Army off entirely.

## Verification

- `check_exe.py` ALL OK; `build_dll.ps1` (/W4 /WX) passes; `test_dll.ps1`
  ALL OK including new fixtures: presence from owned provinces only,
  cycling both directions over Any + present, unknown presence offers all,
  misaligned owned list refused, no presence leaves only Any.
- Python suite: 110 tests OK.
- Previews from staged art and control geometry (Arial, not a game capture):
  `mockups/5-templates-filters-1019b.png`, `mockups/6-conform-filters-1019b.png`.

Candidate: `work/filters-0111/ATCU-filters-0.11.1-test.zip`, 43 files,
SHA-256 `5b773cb0d870bbc2618faa04bad1e22fab8aa67b84632635aadd27d824f33a17`.
Logic DLL SHA-256 `10accdec19bf1ae051e408046d066bb9b5a152049164e08e1f8245aa1f1d421b`.
Expected log marker: `TGCX TPL-5+TPL-6 0.11.1`.

## Installation (2026-10-19)

After the user's fresh normal-exit confirmation and the process gate,
`work/filters-0111/install.py` checked the target still matched the 0.11.0
install record (package plus 88 protected anchors), backed up the complete
installed package to `work/filters-0111/before-tgc/`, copied 41 files and
removed the two unused 0.11.0 `*_filter_box.dds` sprites. All 41 hashes and
the protected anchors verified; record `work/filters-0111/installed-tgc.json`.
V2DLL 5.82, Lua, old packages and the main E: install are unchanged. No game
launched. Rollback after normal exit: restore `before-tgc/` over the package
and descriptor and delete the two `*_check*.dds` files it lacks.

Launch that copy's victoria2.exe with **ATCU - Army Template and Conform
Universal** plus **TGC v2.5.2 (stable)** only. Expected marker
`TGCX TPL-5+TPL-6 0.11.1`. Then run instructions-tpl-5.md and
instructions-tpl-6.md; additionally check the boxes tick/untick by clicking,
and that the continent arrows offer only Any plus continents with owned
provinces.

## Not done

- In-game look, hit-testing of the overlaid
  checkboxes and the continent skipping are unverified.
- Remote mockup artifact not republished (as for 0.11.0).

## Player check (2026-10-19)

User: "i think it works great" on 0.11.1 in the isolated TGC copy. The
partial `Logs/tgcx.log` (game still running at 15:14) has the 0.11.1 marker,
no BAD/refused/disabled lines, playing AUS: Conform windows opened on two
stacks, and one Conform queued 24 builds with 54 left out and 0 failed. The
user gave no per-item results for the TPL-5/TPL-6 checklists, and the log
does not show which culture/continent choices were used, so that coverage
stays unconfirmed. Other bases are not covered.
