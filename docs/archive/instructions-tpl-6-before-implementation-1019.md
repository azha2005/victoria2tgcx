# TPL-6 - Consider continent selection in the template builder

2026-10-19 research update: read report-army-filters-research-1019.md
and docs/symbols.md final section before implementation. Static feasibility
is established; no filter is implemented or runtime-accepted yet. Research
only guards are in tools/check_exe.py, separate from172 runtime guards.

2026-10-19. User suggests "perhaps specifying which continent" to recruit
in from the template builder. Prospective follow-up after TPL-5; design
and implementation are not yet settled. Do not block culture buttons on it.

1. Read NEXT.md and inspect accepted Universal builder, culture filtering
   and live recruitment snapshot. Preserve the same package across mods.
2. Use statically verified province+0x16C CContinent*, loaded catalogue
   [g_provinceSource]+0x2A78..+0x2A7C, key+0x40 and display+0x5C. Add
   bounded readers/world-lifetime cache and runtime validation;
   installed bases define6/8/6/14continents (vanilla/HPM/DoD/TGC). Verify new
   fields/signatures before use, record evidence and use cached engine data.
   No fixed
   vanilla continent list and no per-click parsing of all mod TXT files.
3. Prepare a concrete native selector proposal in the template builder:
   default unrestricted geography, named continent choices drawn from the
   active game, clear eligibility/shortage summary. Decide whether preference
   belongs per template or per recruitment action; present that choice if
   it materially affects behavior. Keep culture and unit page controls clear.
4. Geographic continent membership is not land connectivity. Preserve
   D17 Conform land reach. Explain any proposed Conform interaction before
   extending a builder-only selector to it; never route armies across sea
   automatically or silently override the selected recruitment restriction.
5. If selected for implementation, update visual references, filter shared
   preview/action eligibility and add dynamic-mod/empty-continent/shortage/
   persistence/culture-intersection cases. Use engine commands only.
6. Build/check/stage, preserve rollback, install only after fresh confirmed
   normal exit and process gate, then user runtime acceptance and report.
   No release, game launch/exit or external contact on user's behalf.
