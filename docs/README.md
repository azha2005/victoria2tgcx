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
| `pitfalls.md` | full text of pitfalls P1-P18 (index in `AGENTS.md` §8) |
| `symbols.md` | what this project has named or discovered in `v2game.exe`, with evidence (today: the upper house at `CCountry+0x8B0`) |
| `rtti-classes.md` | **generated** by `ghidra/ExportRtti.java`: the 1 500 vtables with RVA and slots, and the detail of 22 mechanics classes. Not edited by hand |
| `instructions-gh-1.md` | GH-1: review the Ghidra analysis, export the RTTI inventory, create `symbols.md` |
| `instructions-sub-0.md` | SUB-0: minimal submod, reversible installer, load test in game (U-2) |
| `instructions-compat-0.md` | COMPAT-0: clone more TGC versions and `tgc_scan.py --matrix` |
| `instructions-law-0.md` | LAW-0: one bill with favours and affinity (`press_rights`, United Kingdom) end to end, and its test in game (U-3); v3 adds the generated `event_modifiers.txt` and `tools/gen_laws.py` |
| `instructions-dll-0.md` | DLL-0: our own `lua51.dll` proxy that logs signatures and the upper house (confirms GH-2; U-4) |
| `instructions-gov-0.md` | GOV-0: data for 1836 rulers and generals' portraits (Wikipedia, Wikidata, Commons) |

## Reports (dated snapshots; not rewritten)

| document | date | what |
|---|---|---|
| `report-initial-1012.md` | 2026-10-12 | feasibility, downloads, executable signatures, RTTI, TGC v2.3.4 versus v2.5.2, Ghidra analysis |
| `report-sub-0-1012.md` | 2026-10-12 | SUB-0: submod layout, installer, gates, the U-2 checklist (U-2 passed: results in `ROADMAP.md` §1) |
| `report-law-0-1012.md` | 2026-10-12 | LAW-0: what was generated, gates, the simulation, decisions taken while building, deviations from the Bill mockup, what U-3 must check |
| `report-research-1012.md` | 2026-10-12 | modding docs and prior art: how `dependencies`, `replace_path`, `user_dir` and localisation order work (P19, P20); what exists for each mechanic |
| `scripting-reference.md` | living | where to look before writing script: wiki pages, TGC idioms, facts measured on this install (P19-P23) |
| `tgc-howto.md` | living | TGC's systems each mechanic builds on: upper house read/shift, law changes, vote tallies in variables, political leaders, on_actions |

## Archive (`archive/`)

| document | what |
|---|---|
| `archive/sessions.md` | **session log**: every close adds the completed `NEXT.md` and what was done (newest first) |
| `archive/law-design-v1-stages.md` | staged law design (D7 v1, like Better Politics Mod); superseded on 2026-10-12 by favours and affinity |
| `archive/instructions-law-0-v1-stages.md` | LAW-0 instructions for the staged design; superseded on 2026-10-12 |
| `archive/law-design-v2-1012.md` | law design v2 (favours and affinity) before it was rebuilt on TGC's idioms; superseded by v3 on 2026-10-12 (same mechanic, different plumbing) |
| `archive/instructions-law-0-v2.md` | LAW-0 instructions v2; superseded by v3 on 2026-10-12 |

## Outside the repository

| what | where |
|---|---|
| Interface mockups (politics, bill, head of state and generals, economic history, templates), built with the game's and TGC's sprites and bitmap fonts | <https://claude.ai/artifact/Wt2YTaaM6xV8JXsUwapRV4>. **Private**: it contains game and TGC art, not to be shared (R8, `AGENTS.md` §13). PNG exports in `../mockups/`. The scripts that extract the sprites and fonts are not in git. Translated to English on 2026-10-12 (boards: Main, Bill, Ruler, Economy, Templates). PNGs re-exported on 2026-10-12 with `tools/mockups/export.py` (boards and assets in `E:\v2re\mockups`, outside git; after republishing the artifact, refresh the boards there first) |
