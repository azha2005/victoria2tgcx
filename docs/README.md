# Index of `docs/`

> What is in each document and whether it is current. **What happens next
> is only in `../NEXT.md`**; the current status and numbers, in
> `../ROADMAP.md` §1-§2. No document in this folder is used to learn "what
> comes next".
>
> Three kinds, with different rules:
>
> - **Reference**: kept up to date; if what it describes changes, it is
>   edited.
> - **Report**: the snapshot of a measurement or a card on a date. **Not
>   rewritten**; if a new measurement supersedes it, write another report
>   and update `ROADMAP.md` §1-§2.
> - **Archive** (`archive/`): superseded material, verbatim, with the date
>   and the reason in its header. Not edited.
>
> Everything is in English (D10); the whole folder, archive included, was
> translated on 2026-10-12.
>
> When adding a document, add it here.

## Reference (current)

| document | what |
|---|---|
| `technical-plan.md` | how each mechanic is built: the shared DLL (§1), laws (§2), charts (§3), hemicycle (§4), rulers (§5), templates (§6), overall order (§7), feasibility of the mockups and the interface route, D9: hybrid case by case (§9) |
| `law-design.md` | laws with political favours and ideological affinity (D7 revised): each bloc's stance, projected vote, favours, promises, Crown, cycle, what the DLL adds; v3 builds it on TGC's idioms (timed modifiers, step-chains) and maps the Bill mockup element by element |
| `tgc-compatibility.md` | levels N0/N1/N2, the generator for fixed-name files, event range; the any-version matrix is on hold (D1 revised: TGC v2.5.2 only) |
| `pitfalls.md` | full text of pitfalls P1-P34 (index in `AGENTS.md` §8) |
| `symbols.md` | what this project has named or discovered in `v2game.exe`, with evidence (today: the upper house at `CCountry+0x8B0`) |
| `rtti-classes.md` | **generated** by `ghidra/ExportRtti.java`: the 1 500 vtables with RVA and slots, and the detail of 22 mechanics classes. Not edited by hand |
| `instructions-gh-1.md` | GH-1: review the Ghidra analysis, export the RTTI inventory, create `symbols.md` |
| `instructions-sub-0.md` | SUB-0: minimal submod, reversible installer, load test in game (U-2) |
| `instructions-compat-0.md` | COMPAT-0: clone more TGC versions and `tgc_scan.py --matrix` |
| `instructions-law-1.md` | LAW-1 short smoke accepted; script baseline, later fixes and release gates |
| `instructions-u-6.md` | accepted short LAW-1 smoke, retained evidence and limitations |
| `instructions-law-2.md` | installed native Bill prototype; remaining full controls and automatic favours |
| `instructions-u-7.md` | brief Bill visual/Count check and complete script/DLL rollback |
| `law-1-affinity-review.md` | generated negotiation preferences: all 23 base groups approved; later user corrections welcome |
| `instructions-law-0.md` | LAW-0: one bill with favours and affinity (`press_rights`, United Kingdom) end to end, and its test in game (U-3); v3 adds the generated `event_modifiers.txt` and `tools/gen_laws.py` |
| `instructions-dll-0.md` | DLL-0: our own `lua51.dll` proxy that logs signatures and the upper house (confirms GH-2; U-4) |
| `instructions-u-4.md` | user test: install DLL-0, check main-menu behavior, compare upper-house log, save/load and removal |
| `instructions-dll-1.md` | 0.2.4 working, game-tested renderer/cadence/visibility; extended integration coverage outstanding |
| `instructions-dll-2.md` | DLL-2 user alias for HEMI-1; working, game-tested 100-seat prototype; visual polish outstanding |
| `instructions-u-5.md` | functional hover checkpoint passed; 0.2.8 header accepted; remaining integration checks |
| `instructions-hemi-3.md` | 0.2.6 hover accepted; 0.2.8 hover/header accepted in game |
| `instructions-hemi-2.md` | 0.2.5 works in game; saved evidence and known hover mismatch |
| `rea-for-templates.md` | user-suggested REA tooling: candidate evaluation for GH-4 and army templates; Windows x86 release boundary |
| `instructions-gov-0.md` | GOV-0: data for 1836 rulers and generals' portraits (Wikipedia, Wikidata, Commons) |

## Reports (dated snapshots; not rewritten)

Statuses in these summaries reflect when each report was written. Current
0.2.4 functional acceptance is in `report-u-5-1012.md`; earlier pending
statuses remain historical evidence. HEMI-2 0.2.5 works in game and is retained. HEMI-3 0.2.6 hover is accepted in game; 0.2.8 hover/header is accepted in game. See `report-law-1-start-1012.md` section 1.

| document | date | what |
|---|---|---|
| `report-law-2-bill-1013.md` | 2026-10-13 | LAW-1 short smoke accepted; Bill native GUI/command prototype, gates, deviations and rollback; 0.3.0 installed, not tested in game |
| `report-u-6-test-workflow-1013.md` | 2026-10-13 | short player smoke test, coordinator coverage and verified one-command automatic/log bundle; no new live acceptance |
| `report-law-1-report-fix-1013.md` | 2026-10-13 | U-6 screenshot exposes every conditional option; restored one-event report dispatch, 24 catalogue tests; fix installed, retest pending |
| `report-law-1-native-1012.md` | 2026-10-12 | native trace/import, all base affinities, balanced conserved counts, 23 catalogue/7 installer tests; installed candidate and rollback, U-6 pending at that date; report UI superseded on 2026-10-13 |
| `report-law-1-start-1012.md` | 2026-10-12 | HEMI-3 0.2.8 acceptance; catalogue preview/tests, ten affinity approvals, approved native vote model, bounded research and handoff |
| `report-law-1-tgc-support-1012.md` | 2026-10-12 | installed TGC versus vanilla reform rules; recommendation before user model approval |
| `report-initial-1012.md` | 2026-10-12 | feasibility, downloads, executable signatures, RTTI, TGC v2.3.4 versus v2.5.2, Ghidra analysis |
| `report-sub-0-1012.md` | 2026-10-12 | SUB-0: submod layout, installer, gates, the U-2 checklist (U-2 passed: results in `ROADMAP.md` §1) |
| `report-law-0-1012.md` | 2026-10-12 | LAW-0: what was generated, gates, the simulation, decisions taken while building, deviations from the Bill mockup, what U-3 must check |
| `report-dll-0-1012.md` | 2026-10-12 | DLL-0 Release/Win32 build, provenance, relocated signatures, hook/Lua/installer tests; not tested in game |
| `report-dll-0-install-1012.md` | 2026-10-12 | authorized DLL installation on E:\; active/original hashes verified; U-4 gameplay still pending |
| `report-hemi-3-seam-1012.md` | 2026-10-12 | 0.2.7 seam feedback; native artwork border measured, cover aligned at y=101; 0.2.8 built/checked/installed, visual confirmation pending |
| `report-hemi-3-header-1012.md` | 2026-10-12 | user accepts 0.2.6 hover; 27 runtime guards/three UK mappings/empty errors; cover clipped below header, 0.2.7 built/checked/installed, visual confirmation pending |
| `report-hemi-3-1012.md` | 2026-10-12 | verified native scalar bounds/slice hooks, shared seat hover and safe fallback; 27 guards/native/GPU/installer gates pass; 0.2.6 installed, not tested in game |
| `report-hemi-2-handoff-1012.md` | 2026-10-12 | user confirms 0.2.5 works, reports tooltip mismatch; original screenshot and accepted log saved; HEMI-3 next |
| `report-hemi-2-startup-1012.md` | 2026-10-12 | live 0.2.5 startup, 5945 chart draws, no guarded failures, empty errors; user visual/integration acceptance pending |
| `report-hemi-2-build-1012.md` | 2026-10-12 | recovered polish, redirected check-runner/metadata fixes, GPU feather coverage; 0.2.5 DLL/submod installed, game acceptance pending |
| `report-u-5-1012.md` | 2026-10-12 | user confirms 0.2.4 works but looks clunky; 2687 ordered draws/paused samples/empty errors; functional checkpoint accepted, polish/extended coverage remain |
| `report-dll-12-layering-fix-1012.md` | 2026-10-12 | 0.2.3 stable/drawing but covers native UI; live ordered-list diagnosis, 23 guards, real GPU occlusion/native state tests; 0.2.4 installed, game acceptance pending |
| `report-dll-12-tooltip-fix-1012.md` | 2026-10-12 | chart appeared then crashed; dump identifies structured tooltip copying, invalid return hook removed; 0.2.3 installed, stability pending |
| `report-dll-12-live-fix-1012.md` | 2026-10-12 | live CEU3Graphics/absolute-position diagnosis, complete ideology mapping, 0.2.2 built/tested/installed; chart acceptance pending |
| `report-dll-12-fix-1012.md` | 2026-10-12 | U-5 missing chart, final native Present/viewport repair, 18 guards and tests passed; 0.2.1 installed, not tested in game |
| `report-dll-12-1012.md` | 2026-10-12 | combined DLL-1/HEMI-1, scoped symbols, real D3D9 checks, installed 0.2.0; not tested in game |
| `report-u-4-1012.md` | 2026-10-12 | in-game proxy/upper-house test passed; indices 1/2/6 confirmed, rounding sum 32767, save/load evidence; callback cadence and full mapping remain unresolved |
| `report-research-1012.md` | 2026-10-12 | modding docs and prior art: how `dependencies`, `replace_path`, `user_dir` and localisation order work (P19, P20); what exists for each mechanic |
| `scripting-reference.md` | living | where to look before writing script: wiki pages, TGC idioms, facts measured on this install (P19-P23) |
| `tgc-howto.md` | living | TGC's systems each mechanic builds on: upper house read/shift, law changes, vote tallies in variables, political leaders, on_actions |

## Archive (`archive/`)

| document | what |
|---|---|
| `archive/instructions-dll-1-rectangle-1012.md` | rectangle-only scope before the user requested a combined hemicycle build; preserved verbatim |
| `archive/instructions-*-before-fix-1012.md` | DLL-1/DLL-2/U-5 instructions before the missing-chart repair, preserved verbatim |
| `archive/instructions-*-before-layering-fix-1012.md` | DLL-1/DLL-2/U-5 instructions before native layering repair, preserved verbatim |
| `archive/instructions-*-before-u5-acceptance-1012.md` | DLL-1/DLL-2/U-5 instructions before functional acceptance/status update; preserved verbatim |
| `archive/instructions-*-before-hemi2-build-1012.md` | pre-0.2.5 DLL-1/2, HEMI-2 and U-5 instructions, preserved verbatim |
| `archive/instructions-*-before-hemi2-handoff-1012.md` | HEMI-2/U-5 instructions before user functional acceptance, verbatim |
| `archive/instructions-*-before-hemi3-build-1012.md` | HEMI-3/U-5 instructions before the 0.2.6 native scalar-hover implementation, preserved verbatim |
| `archive/instructions-*-before-header-fix-1012.md` | DLL-1/2 and HEMI-3/U-5 instructions before 0.2.6 acceptance and 0.2.7 header correction, verbatim |
| `archive/instructions-*-before-seam-fix-1012.md` | DLL-1/2 and HEMI-3/U-5 instructions before correcting 0.2.7 seam from native artwork, verbatim |
| `archive/sessions.md` | **session log**: every close adds the completed `NEXT.md` and what was done (newest first) |
| `archive/instructions-law-1-before-native-1012.md` | pre-native-implementation instructions, preserved verbatim |
| `archive/instructions-*-before-report-fix-1013.md` | LAW-1/U-6 instructions before the conditional-option failure, preserved verbatim |
| `archive/instructions-u-6-before-short-test-1013.md` | full U-6 manual checklist before the player/coordinator split, preserved verbatim |
| `archive/law-design-v4-before-native-1012.md` | pending native-vote reference before implementation, preserved verbatim |
| `archive/law-design-v3-1012.md` | affinity-only projected-vote design before user approved TGC starting votes |
| `archive/instructions-*-before-law1-handoff-1012.md` | DLL/HEMI/U-5 instructions before 0.2.8 acceptance and LAW-1 handoff, verbatim |
| `archive/law-design-v1-stages.md` | staged law design (D7 v1, like Better Politics Mod); superseded on 2026-10-12 by favours and affinity |
| `archive/instructions-law-0-v1-stages.md` | LAW-0 instructions for the staged design; superseded on 2026-10-12 |
| `archive/law-design-v2-1012.md` | law design v2 (favours and affinity) before it was rebuilt on TGC's idioms; superseded by v3 on 2026-10-12 (same mechanic, different plumbing) |
| `archive/instructions-law-0-v2.md` | LAW-0 instructions v2; superseded by v3 on 2026-10-12 |
| `archive/instructions-dll-0-research-1012.md` | DLL-0 instructions and research handoff before implementation; archived verbatim on 2026-10-12 |
| `archive/instructions-dll-0-pre-u4-1012.md` | implementation handoff before U-4, preserved verbatim; superseded by tested status |

## Outside the repository

| what | where |
|---|---|
| Interface mockups (politics, bill, head of state and generals, economic history, templates), built with the game's and TGC's sprites and bitmap fonts | <https://claude.ai/artifact/Wt2YTaaM6xV8JXsUwapRV4>. **Private**: it contains game and TGC art, not to be shared (R8, `AGENTS.md` §13). PNG exports in `../mockups/`. The scripts that extract the sprites and fonts are not in git. Translated to English on 2026-10-12 (boards: Main, Bill, Ruler, Economy, Templates). PNGs re-exported on 2026-10-12 with `tools/mockups/export.py` (boards and assets in `E:\v2re\mockups`, outside git; after republishing the artifact, refresh the boards there first) |

LAW-2 superseded instructions retained verbatim: `archive/instructions-law-1-before-law2-1013.md`, `archive/instructions-u-6-before-law2-1013.md`.
