# SUBAGENTS.md — The project, in cards

> How the work is split between the coordinator (the main session) and the
> subagents, and the status of each card. **What is done now: only
> `NEXT.md`.** Hard rules: `AGENTS.md` §2.

---

## 1. Roles

| Role | Does | Does not |
|---|---|---|
| **Coordinator** | picks the card, builds the prompt (§2.2), reviews and integrates, updates `ROADMAP.md`, `NEXT.md` and this file, talks to the user | does not delegate [user] decisions or U cards |
| **Subagent** | one card, in its own folder or worktree, up to the gate | does not edit `AGENTS.md`, `ROADMAP.md`, `NEXT.md` or this file |
| **User** | decides (D3-D10), starts the game and says what they see, plays test games | — |

Much of this project cannot be verified without the game running: every
card that changes what is seen ends with a **U** card so the user can test
it.

---

## 2. Protocol

### 2.1 Before launching (coordinator)

```powershell
python tools/check_exe.py          # the base must print ALL OK
python tools/tgc_scan.py --diff E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC
```

**Ghidra:** a Ghidra project does not allow two writers. Only one subagent
at a time works in `E:\v2re\ghidra`; the others read its exports in
`docs/`.

### 2.2 The card (prompt template)

```
Task <ID> — <title>
Read (only this): <exact files and sections>
Context: <2-5 lines: why, what is done, starting numbers>
Do: <numbered, concrete steps>
Gate (done when): <exact commands and what they must print>
Do not touch: <other cards' files>
Deliver: <what goes in the report>
```

### 2.3 Fixed rules (copied into every prompt)

1. Work **only** on what the card says. Do not push.
2. **Do not edit** `AGENTS.md`, `ROADMAP.md`, `NEXT.md` or `SUBAGENTS.md`.
   New pitfalls and numbers go in the report; the coordinator integrates
   them.
3. Never touch the game's, TGC's or HPM's files in place (R1).
4. Every executable address you use: with its evidence and added to the
   signatures in `tools/check_exe.py` (V1, V6).
5. Never loosen a gate to reach it.
6. **Stop and report** if the gate is not met after 3 different attempts,
   if a "Do not touch" file needs touching, or if something contradicts the
   card or `AGENTS.md`. Report with evidence (tool output, addresses,
   bytes), not assumptions.
7. Final report: what you did, the gate's output, numbers with their tool,
   what you could not test (above all: "not tested in game"), new pitfalls
   written as `Pnn`.
8. Everything in English (D10): code, comments, docs, in-game text.
9. **Look it up before writing it:** start from `docs/scripting-reference.md`
   (the wiki) and `docs/tgc-howto.md` (TGC's own system for the same thing:
   extend it, don't build a parallel one), plus a working TGC example.
10. **The mockup is the interface spec (R10):** replicate the card's mockup
    board as closely as the engine allows (PNG in `mockups\`; the
    coordinator has the live artifact). List every deviation with its reason
    in the report. If the card changes the mechanic, say so: the mockup must
    be updated in the same session.

### 2.4 Review and integration (coordinator)

For every delivery: re-run the gate, read the whole diff, check that
nothing in "Do not touch" was touched, integrate pitfalls and numbers,
update §4.

---

## 3. Conflict map

| Resource | Who uses it | Rule |
|---|---|---|
| `E:\v2re\ghidra` | GH-* | one at a time |
| `submod\` | SUB-*, LAW-*, COMPAT-1 | one card at a time writes `tgcx_*` files; each mechanic in its own file |
| `dll\` | DLL-*, HEMI-*, ECO-B, GOV-*, TPL-* | one card at a time until DLL-0 sets up the structure |
| The installed game | U-* | only the user starts it |

---

## 4. Cards

### Card status (as of 2026-10-17)

TPL-1 update (2026-10-17): **0.6.3 army-style GUI installed on E: and accepted in game**. Automatic gates and installed hashes pass; paired 0.6.2 rollback retained. User reports "works"; current 0.6.3 log has no guarded GUI read fault. Extended coverage and new screenshot comparison remain unconfirmed. See `docs/report-tpl-1-visual-1017.md` and `docs/instructions-tpl-1-063-check.md`.

| ID | Title | Status | Instructions |
|---|---|---|---|
| I-0 | Downloads, Ghidra, gates, docs | **done** | `docs/report-initial-1012.md` |
| U-1 | Decisions D3-D7 | D4-D7 **closed** (2026-10-12); D3 (name) missing | `NEXT.md` §1 |
| GH-1 | Review the Ghidra analysis and export the RTTI inventory | **steps 1-4 done** (2026-10-12); step 5 missing | `docs/instructions-gh-1.md` |
| SUB-0 | Submod skeleton and load test | **done** (2026-10-12; tested in U-2) | `docs/instructions-sub-0.md`; `docs/report-sub-0-1012.md` |
| COMPAT-0 | Version matrix (`master` + `--matrix`) | **deferred** (D1 revised: v2.5.2 only) | `docs/instructions-compat-0.md` |
| ECO-A | Economy charts outside the game | ready to start | `docs/technical-plan.md` §3 (instructions when chosen) |
| GH-2 | Upper house: where it lives in `CCountry` | hemicycle data/mapping/colours/coordinates and native ordered draw runtime-confirmed | `docs/symbols.md`; `docs/report-u-5-1012.md` |
| U-2 | Start the skeleton submod with v2.3.4 and v2.5.2 | **done** (2026-10-12): loads on both, override works, one event per click, logs clean | `docs/report-sub-0-1012.md` §4 |
| COMPAT-1 | Generator (N1), v2.5.2 only: build the overrides from the installed TGC at install time | when the first card needs a fixed-name file (LAW-0 needs `event_modifiers.txt`) | `docs/instructions-law-0.md` step 0 (the seed) |
| LAW-0 | One bill with favours and affinity, end to end (`press_rights`) | **done** (2026-10-12, U-3 passed) | `docs/instructions-law-0.md`; `docs/report-law-0-1012.md` |
| U-3 | Play the LAW-0 bill (with a save/load halfway) | **done** (2026-10-12) | `docs/instructions-law-0.md` step 6 |
| LAW-1 | Catalogue for 23 groups / 46 direction entries | **short script smoke accepted**; broader engine coverage unconfirmed; 25 catalogue/10 installer tests pass | `docs/instructions-law-1.md`; `docs/report-law-1-report-fix-1013.md` |
| LAW-2 | Bill GUI and command bridge | 0.3.4 look accepted; **0.3.5: visible buttons, Reforms tab, Decisions filter, no DLL popups; done, not tested in game** (U-7 pending) | `docs/instructions-law-2.md`; `docs/report-law-2-reforms-1017.md` |
| U-7 | Bill prototype visual/Count evidence | 0.3.3 placement and 0.3.4 look accepted; **0.3.5 check pending**, brief check only | `docs/instructions-u-7.md` |
| U-8 | First live template recruit and visual check | recruit/save-reload (0.4.0) and the 0.4.4 short check accepted; extended checks open | `docs/instructions-u-8.md` |
| U-9 | Conform probe on 0.5.0 (log only) | **passed on 0.5.0** 2026-10-17 (P43 found, fixed in 0.5.1, installed; 0.5.1 confirmed) | `docs/instructions-u-9.md` |
| U-10 | Conform in game: button, window, one Conform, march and merge (0.6.0) | **passed on 0.6.2** 2026-10-17 (user: "it worked"; 0.6.0 P44 and 0.6.1 P45 fixed); visual pass next | `docs/instructions-u-10.md` |
| U-6 | LAW-1 catalogue game evidence | **short smoke passed**; user requests moving on, extended coverage unconfirmed | `docs/instructions-u-6.md` |
| DLL-0 | Our own `lua51.dll` proxy that logs signatures and the upper house | **done, tested in game** (U-4, 2026-10-12); installed on E:\; cadence coverage remains to verify in DLL-1 | `docs/instructions-dll-0.md`; `docs/report-dll-0-1012.md` |
| U-4 | Install DLL-0, play a few days, send the log | **done** (2026-10-12): screenshots/log match, clean errors, save/load | `docs/instructions-u-4.md`; `docs/report-u-4-1012.md` |
| DLL-1 | Renderer, cadence and Politics visibility | **working prototype, tested in game (0.2.4)**; extended coverage outstanding | `docs/instructions-dll-1.md`; U-5 report |
| U-5 | Combined renderer/hemicycle acceptance | **functional hover checkpoint passed**; 0.2.8 header accepted; extended integration coverage remains | `docs/instructions-u-5.md`; `docs/report-u-5-1012.md` |
| GH-3 | GUI: element positions and creation on the fly | after GH-1 | to be written |
| GH-4 | Engine commands: CConstructUnitCommand and posting | **TPL-0 static research/guards and recruit runtime-confirmed**; extended U-8 cases open | `docs/symbols.md`; TPL-0 reports |
| GH-5 | Save/load: the save's name, the moment | after GH-1 | to be written |
| HEMI-1 | Hemicycle (user DLL-2 alias) | **working prototype, tested in game (0.2.4)**; visual polish outstanding | `docs/instructions-dll-2.md`; U-5 report |
| HEMI-3 | Match hover with rendered ideology | **0.2.8 hover/header accepted in game** | `docs/instructions-hemi-3.md`; `docs/report-hemi-3-seam-1012.md` |
| HEMI-2 | Visual polish of the working hemicycle | **0.2.5 works in game**, user-confirmed; tooltip mismatch remains; softer dots/background and bloc sweep | `docs/instructions-hemi-2.md` |
| ECO-B | Charts in game | DLL-1 prototype available; GH-5 still needed; not selected | — |
| GOV-0 | Data: 1836 rulers and generals' portraits | ready to start (no RE, in parallel) | `docs/instructions-gov-0.md` |
| GOV-1 | Rulers by script | waits for GOV-0, SUB-0 | — |
| GOV-2 | Ruler portrait/name (DLL) | DLL-1 prototype available; waits for GOV-1; not selected | — |
| TPL-0 | Native Templates panel, local library and first recruit | **0.4.4 accepted in game** (recruit, reopen, navy tab, queue clicks); extended U-8 cases open | `docs/instructions-tpl-0.md`; `docs/report-tpl-0-visual-1017.md` |
| TPL-1 | Conform an existing army to a template | **research started 2026-10-17**: research done (commands, recruit stack id, Move, CUnitView); D13 = A; signatures, planner, native wrappers and unit-panel hooks: U-9 passed on 0.5.0; 0.5.1 province fix installed and confirmed; Conform button and window: **0.6.2 accepted in game** (U-10 passed; report-tpl-1-window-1017.md §8); next: visual pass and the guarded read fault | `docs/instructions-tpl-1.md`; `docs/instructions-tpl-1-visual.md` |
| TPL-2 | Replace regiments lost to rebellion automatically | **on hold** (user, 2026-10-17): TPL-1 Conform already restores the lost regiments | `docs/instructions-tpl-2.md` |

### I — Infrastructure

**I-0 (done, 2026-10-12).** See `docs/report-initial-1012.md`.

### GH — Ghidra

Always: the project is `E:\v2re\ghidra\v2game.gpr`; exports go to `docs/`
as a report or a table. What gets named in Ghidra is recorded in
`docs/symbols.md` with VA, name, `conf` and evidence.

- **GH-1** — review the analysis log and the `ImportV2Symbols` log, export
  the 1 165 RTTI classes with their vtable and slot count
  (`docs/rtti-classes.md`, generated), and the mechanics' classes with
  their slots resolved.
- **GH-2** — find the upper house's composition in `CCountry` (offset, type,
  how it is indexed by ideology). Three routes in `docs/technical-plan.md`
  §4.1.
- **GH-3** — `CGuiObject`/`CEU3Gui`: how to find an element's on-screen
  position by its name, and whether a new one can be created from a
  `windowType` in a `.gui` of our own.
- **GH-4** — `CConstructUnitCommand`: fields, constructor, who queues it and
  how it is sent in MP.
- **GH-5** — loading and saving a game: the file name and the moment, for
  our own per-save file.

### SUB / COMPAT — The submod and compatibility

- **SUB-0** — minimal `submod\TGCX\` + `TGCX.mod`: one test decision and
  event, our own localisation, installation by copy (not link) into
  `mod\`. Check the load order (P10).
- **COMPAT-0** — clone `master` separately, `tgc_scan.py --matrix` (several
  folders, one table), measure 2-3 old versions to know how much what we
  use moves.
- **COMPAT-1** — the generator of `docs/tgc-compatibility.md` §3.

### LAW, ECO, HEMI, GOV, TPL — The mechanics

Each is detailed in `docs/technical-plan.md` §2-§6 (laws: in
`docs/law-design.md`); their instructions are written when their turn
comes.

- **LAW-0** — one bill (`press_rights` → `free_press`), generated from our data:
  affinity, projected vote, purchases, promise, hearing, Crown; save/load
  test (U-3).
- **LAW-1** — the generator of every bill from the installed TGC version
  (anchors: the option names and their `allow`) and the full affinity
  table, reviewed group by group with the user.
- **GOV-0** — data only: `data\rulers_1836.csv`, `data\generals_1836.csv`,
  Commons portraits outside git.

### DLL

- **DLL-0** — D6: our own DLL (V2DLL code marked, R8), Win32 `v143`; a
  `lua51.dll` proxy that forwards everything and writes `Logs\tgcx.log`
  with the load base, the `check_exe` signatures and the player's upper
  house (confirms GH-2). Reversible install and uninstall (rename, never
  delete).
- **DLL-1** — guarded native GUI ordered drawing/final Present cadence and Politics
  lifetime/geometry; combined with HEMI-1 at user request.
- **DLL-2** — user alias for HEMI-1: 100 seats, seven rows and tooltips.
  0.2.4, HEMI-2 0.2.5 and HEMI-3 0.2.6 work in game. 0.2.8 header
  correction is accepted in game; extended U-5 coverage remains.

- **HEMI-2** — mockup-based visual polish of the working 0.2.4 hemicycle;
  consolidate visual changes and remaining integration tests in one run.

### U — User

- **U-1** — decisions D3-D7 (`ROADMAP.md` §3); D4-D7 closed, D3 missing.
- **U-2** — start the game with the skeleton submod on v2.3.4 and on
  v2.5.2 and report what is seen.
- **U-3** — play the LAW-0 bill with the United Kingdom, saving and loading
  with the debate halfway.
- **U-4** — install DLL-0, play a few days with the United Kingdom, send
  `Logs\tgcx.log` and the percentages of the upper-house pie chart.
