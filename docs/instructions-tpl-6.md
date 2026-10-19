# TPL-6 - Verify the session-only continent selector

2026-10-19. Implemented internal0.11.0 and installed in isolated TGC copy.
User chose current recruitment action, default Any. Read NEXT.md,
instructions-tpl-5.md and report-army-filters-implementation-1019.md.

1. User launches the already prepared isolated TGC copy with ATCU+TGC only.
   Do not reinstall, launch or terminate the game. First verify0.11.0log.
2. Open Templates builder. Continent controls appear in the right footer;
   default Any, arrows cycle active-mod continent labels. TGC catalogue
   has14entries, not a fixed vanilla-six list. Verify readable names,
   click targets and no overlap with composition/recruit/summary controls.
3. Use a country owning eligible soldier pops on multiple continents.
   With the same composition, compare Any, Europe and an overseas continent.
   Inspect actual queued regiment source provinces and the native queue's
   continent labels. Unpause between batches. Shortage should leave units
   out rather than recruit elsewhere or silently switch back to Any.
4. Combine continent selection with All / Primary + accepted / Primary
   only. Both restrictions and native unit eligibility must hold. A
   continent with no eligible soldier capacity queues none and reports
   shortage; it does not fall back to other continents.
5. Continent choice is session-only in the builder panel, not attached to
   the selected template and not written to the library. After restarting
   the game it defaults to Any. Culture mode persists separately in v2.
6. Verify Conform ignores builder continent choice and retains independent
   land reach plus selected template culture. Geographic membership does
   not imply land connection or enable transport/marching across sea.
7. After user normal exit preserve logs and actual observations with TPL-5
   evidence. Any further installation needs fresh exit confirmation and
   process gate. Keep accepted0.10.4rollback and main0.0.1 unchanged.
8. After TGC acceptance, perform focused source-selection/layout regressions
   on vanilla/HPM/DoD. Dynamic6/8/14catalogue fixtures and previous four-base
   acceptance do not prove these new filters in those games.
9. Record tested scope, failures and reference deviations in a new report;
   archive/rewrite NEXT and update statuses. Publication remains separate.
