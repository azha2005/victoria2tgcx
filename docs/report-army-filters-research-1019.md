# TPL-5 / TPL-6: culture and continent feasibility

2026-10-19. User requests research into both next features. Research only:
no gameplay implementation, DLL rebuild, installation, game launch/exit,
commit/push or release change. ATCU0.0.1 and its accepted internal0.10.4
binaries remain unchanged. Release preparation is separate from this card.

## Finding

Both features are feasible within the existing Universal architecture.
The construction command already receives the exact soldier pop selected
by our planner. Read-only eligibility filters can restrict those choices;
no new simulation command, country mutation or per-mod package is needed.
Culture is the smaller implementation. Continent adds dynamic enumeration
and one more GUI preference, but its data is already loaded by the engine.

This is static executable and installed-data evidence, not a runtime pass
of new filters. The remaining work is snapshot/filter integration, settings,
UI layout and focused regression/game checks.

## Evidence collected

The reference executable SHA-256 and all prior guards pass. Added13
research-only signatures to tools/check_exe.py, separate from the172
runtime guards. All222 total runtime/research checks pass. Generated DLL
header remains current, verified by tools/gen_signatures.py --check.
TGC2.5.2 scan reports anchors/range OK. No native routine discovered here
is called or hooked, especially not AddAcceptedCulture.

Read-only Ghidra exports are in work/filters-research-1019/:
culture-construct.txt, filter-semantics.txt, continent-model.txt,
continent-catalogue.txt, filter-core-asm.txt and reference/vtable exports.
Only small research signatures and our descriptions enter source control;
the decompilation remains ignored. Ghidra used -readOnly -noanalysis.

The local maxioten reference notes, Engine/CEffectList_ParseEffect.md,
supplied culture leads. Their relevant fields were independently checked
against this executable. The native continent trigger, parser, name
lookup and accessors provide independent evidence for geographic identity.
Canonical upstream reference:
https://github.com/maxioten/Victoria2-Reverse-Engineering/blob/main/Engine/CEffectList_ParseEffect.md
The live page was unavailable through browsing; local notes and executable
inspection, rather than a presumed current upstream page, support this work.

## Culture: verified read path

| Data | Field | Confidence / evidence |
|---|---|---|
| Soldier pop culture | CPop+0x6C, Culture* | C static: native primary-culture trigger FUN_008F84D0 compares it to the country's primary |
| Player's primary culture | CCountry+0xE64, Culture* | C static: primary trigger and accepted-union effect FUN_00893490 |
| Accepted list | CCountry+0xE54 head, +0xE58 tail, +0xE5C count | C static: FUN_00522310 and accepted trigger FUN_008F8DB0 |
| Accepted list node | +0 Culture*, +4 previous, +8 next | C static: duplicate scan and native append code |
| Cached pop accepted status | CPop+0x4C, byte | C static: accepted trigger and native recruit eligibility FUN_005141D0 |

Recommended implementation reads primary and accepted cultures from the
current player country once per action/preview snapshot, bounds/validates
the list, and compares each pop's culture object. The country list is
the canonical choice for the requested policy, rather than trusting a
cached pop byte across accepted-culture changes. Runtime pointer values
must not be persisted or kept across world reloads.

Modes already requested by the user:

- Neither button selected: all normally eligible cultures.
- Primary + accepted: primary equality OR membership in the accepted list.
- Primary only: primary equality.

The two buttons are mutually exclusive. Store the mode, not a specific
culture name, so changing country or primary culture retains the intended
meaning. Native unit eligibility still applies: a culture choice cannot
override a unit's own restrictions. FUN_005141D0 already checks pop+0x4C
when a unit requires accepted culture; retain that native test.

Filters affect new recruitment and Conform's missing builds. Existing
regiments remain governed by composition counts; do not detach or disband
them merely because they have another culture. Queued regiments should
not be cancelled automatically when a preference or accepted culture changes.

## Recruitment control is sufficient

Current tgcx_TplPop stores pop pointer, province and free regiment capacity.
The planner retains the chosen pop index. tgcx_TplSendOrder passes that exact
pop pointer to FUN_0057D1B0, CConstructUnitCommand. Constructor field+0x64
keeps it; execute FUN_0057D2F0 copies the pop's +8/+0xC identity into
CMilitaryConstruction+0x90/+0x94. The command does not silently pick a
different province pop after our filter. Existing native command tests
and accepted recruitment are the baseline for this chain.

No temporary changes to accepted cultures, pop flags, province ownership,
unit definitions or game files are necessary. Simulation writes stay in
the existing engine command path.

## Important integration trap: per-pop versus per-province

tgcx_TplEligible currently receives only a province id. tgcx_TplFilterPops
caches that predicate once per province for land reach. It is suitable
for continent and land reach, but **cannot represent culture**: two soldier
pops in the same province may have different cultures.

Add a pop-aware read snapshot/filter stage. Keep geographic results cached
per province, culture decisions per pop, and use the same eligibility
result in preview/capacity/planning and action execution. Conform preview
currently scans/copies pops in tgcx_CfPreview; execution rescans through
tgcx_TplRecruitWith. Both need the selected mode. Updating only the GUI or
preview would create false capacity claims.

Recommended order is cheap culture and continent checks, then land reach
where applicable, with native unit eligibility/free capacity retained.
Selected restrictive modes must refuse unreadable eligibility data rather
than silently falling back to unrestricted recruitment.

## Continent: verified read path

| Data | Field | Confidence / evidence |
|---|---|---|
| Province continent | CProvince+0x16C, CContinent* | C static: FUN_008CF460 compares it with named continent lookup result |
| Continent catalogue | [g_provinceSource]+0x2A78 begin, +0x2A7C end, +0x2A80 capacity | C static: FUN_0049C710 appends pointers; FUN_0049D040 iterates begin/end |
| Global province source | VA0x0125870C / RVA0xE5870C | Existing project symbol, also used by continent-trigger name lookup |
| Continent stable key | CContinent+0x40, game string | C static: native slot11 FUN_00494550 copies it; name lookup compares it |
| Continent display string | CContinent+0x5C, game string | C static: slot6 FUN_00494520 copies it; constructor resolves display text |

Use the current loaded catalogue and its native strings. Bounds, pointer
and string validation should follow existing guarded readers. Cache
catalogue keys/labels for the world lifetime; invalidate on world/catalogue
change. Read province membership into recruitment snapshots. No need to
parse continent.txt or localisation files on each click.

The ordinary Build Army queue already binds a continent **label** in
FUN_00732720. This is display evidence, not an existing recruitment-filter
widget to reuse. The safest initial selector uses our already-working
native button/text controls, for example previous/value/next, rather than
introducing an unverified dropdown ABI.

## Installed-base inventory

Read effective map/continent.txt with the project's base profile resolver.
Full keys/counts and matched English labels are saved in ignored
base-continents.json. Static file labels are leads only; the implementation
should use native loaded/localised strings.

| Base | Defined continents | Extra keys beyond vanilla |
|---|---:|---|
| Vanilla | 6 | none |
| HPM0.4.5 | 8 | australia_new_zealand, polynesia |
| DoD Fan Fork | 6 | none |
| TGC2.5.2 | 14 | indochina, east_indies, central_africa, west_africa, east_africa, south_africa, australia_new_zealand, polynesia |

A fixed six-continent list would fail the existing tested matrix. Enumerate
whatever the active game defines, including future custom keys. Native
key identity also avoids treating two translated labels as the same value.
An unavailable saved key should be visible and refuse recruitment until
changed; it must not silently become Any.

Continent identity is **not land connectivity**. Separate islands can share
a continent and connected provinces can have different continent labels.
The existing Conform land-route policy remains necessary.

## Recommended feature scope and settings

1. Culture mode per template, available in builder and Conform, as already
   requested. Default for an old/no-mode template is unrestricted culture.
2. Continent selector initially in the template builder, default Any.
   Recommended initial semantics: recruitment-action preference, so one
   composition can be built in Europe today and Asia later. Per-template
   storage is an alternative for user choice before implementation. Do not
   silently apply a builder selector to Conform.
3. If continent restrictions are later added to Conform, combine them with
   land reach using AND. No naval transport or automatic broadening.

Current tgcx_templates 1 parser accepts only numeric unit-count lines.
Adding culture=primary or a continent key would currently reject the
library, not ignore the extra field. Design a versioned migration or a
separate setting format before saving new data. New code must import old
templates as unrestricted, preserve original files/atomic replacement,
and define rollback behavior. A v2 file with a retained v1 backup is one
option; old0.10.4 will not read v2 automatically. Persist keys/modes only.

## UI and cost assessment

Viewed current mockups/5-templates.png and the accepted
mockups/6-conform-current-1019.png. No visual spec was changed by research.
Current builder root is652x528, eight unit rows begin at186 with31px
steps, footer starts466. There is only about one control row between the
editor and footer. Two culture controls plus a continent selector require
a planned filter band, footer adjustment or row/page tradeoff. Do not
crowd them into the native DoD unit tabs. Conform has little spare room
after its Changes rows; a clear culture control band needs a mockup update.

Data access is inexpensive in principle: one bounded culture-set read per
snapshot, one culture comparison per pop, one continent membership read
per province. A small sorted culture set avoids repeatedly walking the
linked list for every pop. Continent catalogue can be cached in memory.
Existing native eligibility/path calls are the heavier operations; cheap
filters can reduce those calls. No measured runtime speedup is claimed.

Paused toggles must invalidate preview immediately, rather than waiting
for the date to change. Selection/library edits and world/player changes
also invalidate relevant snapshots. Existing subtree-show/lifetime guards
and count modifiers must remain intact.

## Suggested implementation and acceptance order

1. Add read-only snapshot fields and bounded canonical culture-set reader.
   Verify memory layouts in isolated fixtures before new runtime use.
2. Add shared pop-aware eligibility and preview/action agreement, preserving
   native unit checks, shortages and Conform land reach.
3. Culture GUI/mode persistence and a visual-reference update; test mixed
   culture pops in one province, empty accepted list, primary changes,
   restricted unit types, shortages and old-library migration/rollback.
4. Dynamic continent catalogue/selector after its action-versus-template
   preference choice. Test6/8/14catalogues, custom/absent keys, translated
   labels, zero eligible capacity and culture/continent/route intersections.
5. New candidate build, normal-exit/process gate before installation, user
   gameplay checks and logs. No installer/scripts added for recipients.

Culture feasibility: high confidence, low engine risk, moderate shared
filter/settings/UI work. Continent feasibility: high confidence, moderate
catalogue/UI/settings work. Both remain **not implemented or tested in
game**. Start implementation with culture; keep ATCU0.0.1 as rollback.
