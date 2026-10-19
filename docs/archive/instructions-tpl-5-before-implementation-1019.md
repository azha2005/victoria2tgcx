# TPL-5 - Culture buttons for Templates and Conform

2026-10-19 research update: read report-army-filters-research-1019.md
and docs/symbols.md final section before implementation. Static feasibility
is established; no filter is implemented or runtime-accepted yet. Research
only guards are in tools/check_exe.py, separate from172 runtime guards.

2026-10-19. Selected by user after Universal0.10.4 worked on vanilla,
HPM, DoD and TGC2.5.2/V2DLL. Start from that accepted baseline. No code
for this card is implemented yet. Politics remains deferred.

1. Read NEXT.md, AGENTS.md and report-universal-tgc-passed-1019.md. Run
   tools/check_exe.py before address-dependent work. Inspect existing
   recruitment snapshots/metadata so filtering uses the actual pop chosen
   by CConstructUnitCommand, retaining engine-command-only mutation.
2. Use statically verified CPop+0x6C culture, CCountry+0xE64 primary and
   accepted list+0xE54/+0xE58/+0xE5C (node value+0,next+8). Add guarded
   snapshot reads and isolated-layout fixtures; live verification remains.
   Record new offsets/signatures and confidence in docs/symbols.md and
   check_exe.py. No whole-game RE.
3. Keep the previously requested two mutually exclusive controls:
   **Primary + accepted** and **Primary only**. Neither selected means
   unrestricted culture, preserving old templates. Apply to Templates
   recruitment and Conform missing builds; do not detach existing troops
   just because their culture is outside the recruitment preference.
4. Update current local Templates and Conform visual references before
   implementation, following R10/current screenshot. Keep native style,
   DoD compact entry, unit pages and available space. Preserve superseded
   references. Document layout deviations and refresh exports/artifact
   where available; do not silently claim remote publication.
5. Store mode per template as an interface preference, not culture ids.
   Current v1 parser rejects unknown/non-numeric lines. Plan a versioned
   migration or separate settings format with old-file preservation and
   explicit rollback, backwards compatible loading/atomic saves. Old files
   default to unrestricted culture; invalid selected restrictions refuse
   recruitment until corrected. No country/game-state writes or save dependency.
6. Add pop-aware filtering: current tgcx_TplEligible caches per province
   and cannot distinguish mixed-culture pops in one province. Read the
   bounded accepted set once per snapshot; share eligibility across
   tgcx_CfPreview and tgcx_TplRecruitWith, applying
   culture AND existing Conform land reach. Recruit what fits; report
   shortage accurately. Preserve unit-specific eligibility/queue capacity,
   ctrl/alt amounts, safe routing and session tracking.
7. Add meaningful cases for primary/accepted/other cultures, empty accepted
   set, mixed pops, shortage, preview/build agreement, old/new persistence,
   unit eligibility and land-scope interaction. Run appropriate native and
   Python checks sequentially where probe binaries are rebuilt.
8. Build/stage a new candidate and files-only Universal ZIP. Keep0.10.4
   rollback and exact hashes. Never edit base files or launch/close game.
   Before any installation get fresh normal-exit confirmation/process gate.
9. User checks controls, recruitment source cultures and Conform shortages
   across accepted bases; preserve logs, report tested scope and visuals.
   Then assess TPL-6 continent-selection proposal; no automatic expansion
   of this card or prerequisite continent implementation.
10. Immutable report and mandatory NEXT/archive/status close. No new
    commit/push/release/subagents without authorization for that future work.
