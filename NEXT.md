# NEXT - what comes next

2026-10-19 (59). **ATCU culture and continent filters implemented as
internal0.11.0 and installed in the isolated TGC copy. Player test next.**
Session58 archived in docs/archive/sessions.md.

**Update 2026-10-19 (release): ATCU 0.0.2 prepared from internal 0.11.2.** work/releases/final/ATCU-0.0.2.zip, SHA-256 e656c58d...becb4a6, source a90050c, pushed to origin/master. Not tagged or published: that needs the user's go-ahead. docs/report-release-002-1019.md. The 0.11.x filters were not rechecked on vanilla, HPM or DoD.

**Update 2026-10-19 (later): TPL-7, 0.11.2, Conform orders survive a load; PASSED in game (2 orders resumed and merged after a load; pre-Conform save case unconfirmed); installed in the isolated TGC copy 2026-10-19 (41/41 verified, backup work/orders-0112/before-tgc/).** A reload had sent a Conform regiment to the rally point (D13 forgot it). The user chose option 2 (D13 revised): tgcx_conform_orders.txt beside the templates records tag, date, stack and construction ids; a load of that tag dated within an order resumes the march and merge. docs/report-tpl-7-orders-1019.md. Candidate work/orders-0112/ATCU-orders-0.11.2-test.zip; Run the report's section 7 check (marker TGCX TPL-7 0.11.2).

**Update 2026-10-19: 0.11.1 GUI refinement installed in the isolated TGC copy; user reports it works great (checklist items not individually confirmed).** Native
checkboxes with a "Culture" caption (no band in the builder), compact
`Recruit from < Any continent >`, continents limited to those with owned
provinces, and filter-guard failure keeps unrestricted Army on.
docs/report-army-filters-gui-1019.md. Candidate
work/filters-0111/ATCU-filters-0.11.1-test.zip; backup work/filters-0111/before-tgc/.
Run steps 1-2 below on 0.11.1 (marker TGCX TPL-5+TPL-6 0.11.1).

## User decisions and boundaries

- Culture modes per template: Primary + accepted / Primary only; neither
  means All. Used by Build Army and Conform missing recruitment. Existing
  regiments remain counted by type, no culture-based purge.
- Continent choice: current recruitment action, default Any. Session-only
  builder selector, never saved per template. Conform retains independent
  D17 land reach and ignores this continent selector.
- User supplied current GUI screenshots, retained as visual references.
  Keep652x528builder/eight unit rows and332x594Conform. Filter band reduces
  visible Conform changes to seven rows, with overflow/totals retained.
- Name ATCU - Army Template and Conform Universal. No em dashes. Files-only
  ZIP, one README.txt, no recipient scripts. License remains undecided.
- User alone launches/exits. Fresh normal-exit confirmation/process gate
  before any further game-copy writes. No new commit/push/publication or
  Politics expansion authorized.

## Installed candidate

User confirmed normal exit and chose isolated TGC installation.39/39mod
files verified in work/Victoria-II-clean-TGC252. Existing V2DLL5.82/Lua,
old packages and protected base anchors unchanged. No game launched.

Open that copy's victoria2.exe, select **ATCU - Army Template and Conform
Universal** plus **TGC v2.5.2 (stable)** only. Untick old Army/Politics.
Expected log: TGCX TPL-5+TPL-6 0.11.0, V2DLL plugin API v1.
Backup: work/filters-0110/before-tgc/. Installation and test evidence:
work/filters-0110/{installed-tgc,verification}.json. Test guide in game root
was refreshed with its original retained in the backup.

Main E:/Victoria.II.v3.04.Inclu.ALL.DLC remains ATCU0.0.1/internal0.10.4;
vanilla/HPM/DoD test copies remain0.10.4. Four-base acceptance applies to
0.10.4, not these filters. Main install instructions remain separate.

## Implementation and checks

- report-army-filters-implementation-1019.md: per-pop culture snapshot,
  cached loaded continent catalogue, exact engine-command pop selection,
  shared preview/action filtering, paused preview revision invalidation.
- Imports old tgcx_templates.txt when v2 absent. Edits save atomically to
  tgcx_templates_v2.txt. Original v1 stays intact; malformed v2 refuses,
  never falls back to unrestricted. Old build reads v1, so v2-only edits
  remain unavailable there. No continents or native pointers persisted.
-223executable checks pass:179runtime +44research, generated header matches;
  TGC scan, Win32 build, native/real-D3D9 fixtures pass. Python109full-suite
  cases plus added package-mismatch guard pass,110distinct cases checked.
  Packaged loader and V2DLL API probes pass. No filter game acceptance yet.
- Visual previews use actual staged art/control geometry. Current supplied
  captures and new PNG/SVG references are indexed in visual-references.md;
  native font/visibility/hit-testing still require player verification.
  Remote historical artifact was not republished.
- Files-only candidate: work/filters-0110/ATCU-filters-0.11.0-test.zip,
  41files,1,052,283bytes; SHA256
  04ca8a523016ce0d55099c049c76a6b81a7a75f3d3c92ec675541a0ce0ddf572.
  Accepted slim0.0.1 ZIP and main installation retained unchanged.

## Next session in order

1. **TPL-5 player check**, docs/instructions-tpl-5.md. User launches isolated
   TGC copy; compare All / Primary + accepted / Primary only from mixed
   culture soldier pops, actual source pops and shortage. Toggle while
   paused, save/reload; Conform existing units unchanged by culture.
2. **TPL-6 player check**, docs/instructions-tpl-6.md. Same template with
   Any / Europe / another owned continent; compare actual source continent
   and shortage. Check combinations with culture, and Conform land reach
   independently. No sea routing implied by continent membership.
3. After normal exit preserve full Logs/tgcx.log and Logs/v2dll.log. Verify
   marker/catalogue/commands and user visuals. Diagnose failures before
   any additional installation; fresh exit confirmation for every update.
4. Once TGC filters accepted, stage focused vanilla/HPM/DoD checks of the
   changed controls/source selection. Keep previous packages as rollback;
   do not claim acceptance from fixtures or previous0.10.4matrix.
5. Record actual test scope and close under ROADMAP7. Public release work
   follows instructions-release-0.md separately; no automatic upload/push.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| TPL-5 |0.11.0implemented, installed isolated TGC, player verification next | instructions-tpl-5.md; report-army-filters-implementation-1019.md |
| TPL-6 |0.11.0implemented, session-only selector, player verification next | instructions-tpl-6.md; same report |
| ATCU-MAIN |0.0.1retained on main E: | instructions-atcu-main.md; report-atcu-main-install-1019.md |
| RELEASE-0 | slim local0.0.1 retained; publication separate | instructions-release-0.md |
| MOD-6 / MOD-3 / MOD-4 / MOD-5 |0.10.4four-base short accepted; filters unaccepted | instructions-mod-6.md |
| TPL-3 / TPL-4 / V2D-082 | accepted baselines | existing reports |
| LAW-4 / U-12 Bill | deferred | respective instructions |
| V2D-2 | separate author-side enhancement | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Native exports, art and binaries stay ignored work/.
