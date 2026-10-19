# Army filters implementation and isolated installation

2026-10-19. TPL-5 and TPL-6, internal candidate **0.11.0**. Implemented,
automatically checked and installed in the isolated TGC copy. **Not tested
in the game yet.** The accepted ATCU0.0.1 release candidate and main E:
installation remain on internal0.10.4. No commit, push or publication.

## Behavior and implementation

The user selected two mutually exclusive culture controls: Primary + accepted
and Primary only. Neither means All. The preference belongs to the selected
template and is shared between Build Army and Conform missing recruitment.
Existing regiments remain counted by type; their cultures do not cause detach.

The user chose a continent preference for the current recruitment action,
default Any. The builder's arrows cycle Any and the active game's catalogue.
It is session-only, shared across selections in that builder panel, and never
saved with a template. Conform ignores this selector and retains its separate
land-route restriction. Culture and continent restrictions intersect with the
engine's unit eligibility and free soldier capacity. Recruit what fits and
report shortages; invalid selected restrictions never silently become All/Any.

`tgcx_army_filters.h` validates a bounded country primary/accepted snapshot
and filters individual soldier pops, including mixed cultures in one province.
It cannot reuse the existing province-cached land predicate for culture.
The exact selected pop still goes to CConstructUnitCommand. There are no new
simulation mutators or direct country/pop writes. Both preview and recruitment
use the same filter and planner; Conform additionally applies land scope.
A template revision invalidates open Conform previews while paused.

The continent catalogue uses the loaded manager's objects, canonical keys and
display strings. It is cached by world/manager/vector identity, bounded to128
entries, validates each object vtable and refuses duplicate keys. Culture lists
are refreshed for each planning/action snapshot, bounded to1024accepted entries.
The unit catalogue keeps its existing cache. No mod TXT scan occurs per click.
These readers are static-evidence plus fixture verified, not new live-memory
measurements. Native recruitment checks can further reject queued commands.

Six research guards were promoted and a CContinent vtable guard added:179runtime
and44research checks,223total. The vtable at VA0x00DFC1DC, RVA0x9FC1DC, is checked
before object string reads. Existing field evidence is in symbols.md and
report-army-filters-research-1019.md. Guard failure leaves Army disabled safely.

## Settings and rollback compatibility

The candidate imports `tgcx_templates.txt` v1 only if the new file is absent.
All imported templates default to unrestricted culture. On editing, atomic
saves write `tgcx_templates_v2.txt` with version2 and exactly one `@culture=0`,
`1` or `2` per template. The original v1 file is never rewritten. Malformed v2
refuses editing instead of falling back to unrestricted v1. No continent or
runtime object pointer is persisted. Accepted0.0.1 reads the retained v1;
candidate-only edits stay in v2 and will not appear after old-build rollback.

## Visual references and intentional changes

User supplied three current main-copy screenshots. They are retained unaltered
in ignored mockups/ as 6-conform-main-current-1019.png,
6-conform-main-queued-1019.png and 5-templates-main-current-1019.png. They supersede
older boards as the baseline for this work. Generated layout previews use the
staged textures and actual control coordinates:
5-templates-filters-1019.{png,svg},6-conform-filters-1019.{png,svg}.
The preview raster uses Arial to illustrate geometry; it is not a native game
capture. Remote historical artifact was not republished.

Builder outer dimensions stay652x528, eight unit rows retained. Culture boxes
fit below the rows at y442, continent controls in the right footer at y468,
summary/status at494/510. Conform stays332x594; boxes at202, divider230,
Changes start264. It now shows seven change rows instead of eight to fit the
filter band; existing overflow text and totals retain omitted changes. Native
army style, bottom controls, compact DoD entry and unit pages are retained.
Checkbox frames are original Pillow art; other stage art retains its prior
private game-derived provenance. Player visual acceptance remains pending.

## Verification

- Supported executable SHA and223runtime/research byte checks pass; generated
  header matches179runtime entries. TGC2.5.2 scan anchors/range pass.
- Release/Win32 build passes `/W4 /WX`. Native suite passes, including real
  D3D9 pixels, hook state, metadata, routes and existing command fixtures.
  Restricted sandbox could not create a graphics device; final fixture was
  rerun successfully with graphics access, without launching the game.
- New independent engine-shaped fixtures cover mixed primary/accepted/foreign
  pops in one province, exact command pop identity, preview/action shortage
  agreement, native unit eligibility intersection, land-scope intersection,
  empty/changed/corrupt culture lists,6/8/14custom continent catalogues,
  duplicate translated names, missing keys and bounded vector refusal.
- v1 import, separate atomic v2 round trip, original byte preservation,
  invalid/missing/duplicate culture modes and no-fallback behavior pass.
- Full Python suite109tests passes; the subsequent additional old-DLL/new-GUI
  mismatch regression also passes (four focused layout/package tests).
  Distinct Python cases checked:110. No repeat full suite after a test-only add.
- Packaged loader and V2DLL API alias probes pass. They deliberately run in
  non-game processes: initialization refuses native hooks there. This is
  loader/API compatibility evidence, not integrated gameplay proof.
- ZIP CRC, inventory and every manifest hash verified:41files,39mod files
  plus root loader and one README.txt. No recipient scripts, GUI overrides
  from base files or extra documentation/manifests inside the ZIP.

Candidate: work/filters-0110/ATCU-filters-0.11.0-test.zip
Bytes: 1,052,283
ZIP SHA256: 04ca8a523016ce0d55099c049c76a6b81a7a75f3d3c92ec675541a0ce0ddf572
Logic SHA256: 340017f57cc953f91a68d21a824fe951c15f25e80fc251c3fa0170cadf933644
Loader SHA256: 3172e716e0d5e1bb7a4baf91bbab9828a9f9f57d2389744b72cb14b03ca3a5ca
Manifest stays alongside ZIP. Build/test/gate/probe logs and verification.json
are in work/filters-0110/. Accepted ATCU0.0.1 ZIP remains byte-identical:
9ee654e7b33fbe62444892a55388e24ecfc7246ace2fdef9bf1007585e4f799e.

## Installed target and player check

After fresh explicit normal-exit confirmation selecting the recommended isolated
copy and the process gate, installed39/39verified mod files into:
`work/Victoria-II-clean-TGC252`. Existing V2DLL5.82, original Lua, old packages
and protected base anchors remain unchanged. Existing test guide was backed up
and refreshed for0.11.0. Complete rollback: work/filters-0110/before-tgc/.
Install manifest: work/filters-0110/installed-tgc.json. Main E: copy remains
unchanged at ATCU0.0.1. No game launched or stopped. Other test copies remain0.10.4.

User opens that copy's victoria2.exe, selects ATCU - Army Template and Conform
Universal plus TGC v2.5.2 (stable), unticks old Army/Politics. Check mode changes
while paused, allowed source cultures, shortages, continent Any/Europe/overseas,
Conform land/culture intersection, save/reload, counts and Navy/fleet hiding.
Expected log marker: TGCX TPL-5+TPL-6 0.11.0, V2DLL plugin API v1.
Preserve Logs/tgcx.log and Logs/v2dll.log after normal exit. Test instructions
are instructions-tpl-5.md and instructions-tpl-6.md. If accepted, run focused
regressions on the other bases; no filter acceptance is inherited from0.10.4.

Rollback after normal exit: retain the candidate folder outside mod/, then
restore the complete previous Universal folder and descriptor from before-tgc.
Keep both template files. No loader replacement or base-file restoration needed.
