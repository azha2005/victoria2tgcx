# TPL-8: approved non-core design, implementation held for Rust coordination

2026-10-21. User selected easier ATCU features first, then explicitly
approved a third **Non-core only** mutually exclusive checkbox in both
Templates and Conform. No selected box means all cultures; clicking the
selected box clears it. Non-core means a soldier pop's culture is neither
the country's primary nor an accepted culture. It does not refer to the
province's territorial cores. Conform's narrow panel may use two lines.

Before implementation, user said to wait because fedacking is porting the
project to Rust. **No ATCU source, generated header, art, package, DLL,
game files or settings were changed in this attempt.** Work consists of
read-only inspection, a scratch work/noncore-0113 folder and executable
gate output. Do not mistake the scratch folder name for a built version.

## Verified upstream evidence

Read GitHub PR metadata, diff and comments on2026-10-21. [PR#1, Rust port
of the lua51.dll loader](https://github.com/azha2005/victoria2tgcx/pull/1)
is merged. PR base045bbfd5bcd54db25be3564d3461814b97a2d15c,
head49eda091da1209ec29653ff35d7e88b32d65577f,
merge1727804b76dbce039402505f92c50ab71a8b09f7.

The PR describes/implements the Rust loader under rust/loader, retaining
the tgcx_Init interface and leaving feature logic in the C++ tgcx.dll. It
also carries C++ build-compatibility edits in tgcx_templates_gui.h,
tgcx_templates.h and tgcx_conform.h, plus build-tool environment changes.
In particular it splits the Templates click body from its SEH wrapper;
TPL-8 would edit that click handling. These edits must be preserved.
There were no returned PR discussion comments at inspection time.

The local checkout is still at the PR's base045bbfd5. It has substantial
uncommitted Province Rename work from the previous sessions. Do not reset,
discard it or blindly pull across it. No fetch/merge/rebase/push was done.

No direct Rust loader interface change is needed for the non-core filter.
However, a subsequent port of recruitment/UI/preferences would need the
new feature ported too. Public PR evidence does not establish the scope
of fedacking's unpublished or current next work. No message was sent to him.
User's wait instruction holds feature implementation pending coordination.

## Design/implementation notes for a later resume

Current references opened: mockups/5-templates-filters-1019b.png and
6-conform-filters-1019b.png, plus current GUI generators/filter/storage.
Relevant pitfalls reviewed: P41/P42 native checkbox sprites/show state,
P44 direct-child lookup, P47 lifetime/visibility-cache bounds, P53/P56/P57
base-aware anchors/root activation, P55 land reach, P58 independent field
semantics, P59 cross-type unique names and P31 ABI verification.

Proposed implementation, not yet built:

- Append culture mode3; leave existing0/1/2 meanings unchanged. Filter exact
  soldier pops before both preview and command planning. Non-core excludes
  primary and every accepted culture, including mixed-culture provinces.
- Reuse native checkbox art and current backgrounds. Templates has room
  for a compact third choice within its left404px region (actual native
  Arial12 advances:93px accepted label,59px primary label,68px non-core).
  Conform needs a second line; retain outer size/footer, document any
  reduction of visible Changes rows and keep complete overflow totals.
- Update checkbox visibility/cache bounds and both click dispatches,
  preserving upstream's SEH split. Preview refreshes while paused.
- Consider a separate version3 preferences file importing v2 then v1,
  retaining both originals for rollback; old v2 parsers reject mode3.
  Do not silently widen invalid/missing culture data to unrestricted.
- Bump the shared GUI schema marker if control/layout expectations change;
  reject old-DLL/new-GUI mixtures. Continent remains session-only/defaultAny.
  Conform retains land reach and count-based existing regiments.
- Native commands remain the only simulation mutation path. No address
  research is required for the additional boolean culture policy itself.

This is a design/coordination handoff, not implementation or gameplay
validation. No new pitfall (nextP64), test suite/build, install, release,
source commit, push, game controls or memory update. Province Rename0.2.6
ZIP and installed copy remain unchanged and unpublished as requested.

Next: instructions-tpl-8.md; reconcile verified upstream Rust/build work
and clarify port ownership before implementing the approved feature.
