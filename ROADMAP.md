# ROADMAP.md — Status, decisions and session close for the TGCX submod

> **What this file is.** The project's status (what is done, with which
> numbers), the decisions, the risks and the closing procedure. **What
> happens next is not here: it is in `NEXT.md`**, the only place that says
> it. `AGENTS.md` is the contract (hard rules, conventions, pitfalls); if
> anything here contradicts it, `AGENTS.md` wins.

---

## 0. Map of the documents and how to use them

| question | where |
|---|---|
| What do I do now? | **`NEXT.md`** |
| What is done, and with which numbers? | this file, §1 |
| Which cards exist and what is each one's status? | `SUBAGENTS.md` §4 |
| How is a mechanic built? | `docs/technical-plan.md` |
| How does it stay compatible with TGC? | `docs/tgc-compatibility.md` |
| Hard rules, executable map, pitfalls | `AGENTS.md` (pitfall text in `docs/pitfalls.md`) |
| Why was something decided? | §3, `AGENTS.md` §9, `docs/archive/` |

1. Read `NEXT.md`, then §1 of this file, then the card in `SUBAGENTS.md`
   and its `docs/instructions-<card>.md`, and `AGENTS.md` §2, §7, §8 and
   §11.
2. Work on **one card at a time**; the next one does not start while the
   previous one is red. New pitfall: `Pnn` in `docs/pitfalls.md` and in the
   index in `AGENTS.md` §8 (the next one is **P47**).
3. **When closing the session: §7** (required).

**Process rules (not negotiable):**

| # | Rule | Why |
|---|---|---|
| V1 | Before using or adding an executable address: `python tools/check_exe.py` green and the new address added to its signatures | P8: the docs may come from another build |
| V2 | Every submod change that depends on TGC: `tools/tgc_scan.py` green on TGC v2.5.2 | D1 (revised: v2.5.2 only) |
| V3 | A mechanic is not done until the user sees it in game; until then it is written "done, not tested in game" | `AGENTS.md` §11 |
| V4 | No third-party material in git (executable, Ghidra project, TGC, V2DLL) | R8 |
| V5 | **[user]** decisions are not taken by the agent: it prepares the data, recommends and asks | R9 |
| V6 | A new address or offset comes with its evidence (Ghidra function, bytes, or a dated Cheat Engine reading) and its `conf` | so the next person does not inherit an assumption as if it were data |

---

## 1. Status by stage (as of 2026-10-18)

MOD-1 update (2026-10-18): **0.7.0 installed and passed the game check (2026-10-18)** (`docs/report-mod-1-1018.md`). Signature gates are per module (stages; drawing+laws; Templates; Conform), V2DLL's sites are logged FOREIGN and block nothing, and the logic is `tgcx.dll` behind a `lua51.dll` loader, with a `V2Mod_Init` export for V2DLL (D16). Installed on E: 2026-10-18 (0.6.5 backed up to `before-070-dll`).

U-7/U-8 update (2026-10-18): **U-8 extended cases all passed on 0.6.3; U-8 complete** (`docs/report-u-8-extended-1018.md`). **U-7 partly passed** on the 0.3.5 law layer: buttons, bill flow, purchases, Decisions tab, monthly tick and popup filter work; the Reforms tab does not re-evaluate rows (F1), the top-bar tooltip lists `tgcx_` decisions (F2), the load test is still installed (F3), long names overflow the Bill (`docs/report-u-7-1018.md`). D14 decided; LAW-3 next.

TPL-1 update (2026-10-17): **0.6.3 army-style GUI installed on E: and accepted in game**. Automatic gates and installed hashes pass; paired 0.6.2 rollback retained. User reports "works"; current 0.6.3 log has no guarded GUI read fault. Extended coverage and new screenshot comparison remain unconfirmed. See `docs/report-tpl-1-visual-1017.md` and `docs/instructions-tpl-1-063-check.md`.

| # | Stage | Status | Numbers / evidence |
|---|---|---|---|
| 0 | **Infrastructure** | **done** | TGC v2.5.2 cloned (tag, `e06ade2`, 589 MB, no line-ending conversion); Ghidra 12.1.4 (zip SHA-256 equal to the published one); refs: RE repo `e299a02`, V2DLL `5d6f894`; initial `check_exe.py` 7/7 OK on `D:\` and `E:\`; TPL-0 gate now SHA-256 + 91 runtime signatures OK (`check_exe.py` ALL OK, 2026-10-17) on `E:\`; `tgc_scan.py` OK on v2.3.4, v2.5.2 and HPM; Ghidra analysis 752 s, 260/260 symbols imported, 1 500 vtables (`docs/report-initial-1012.md` §4) |
| 1 | Submod skeleton + loading | **done** (SUB-0 + U-2, 2026-10-12) | loads on v2.5.2 and v2.3.4 (settings folders `TGC252_TGCX`, `TGC_TGCX`; `setup.log` lists our 2 files; `error.log` empty); `000_tgcx_` override wins; one event per click; install + remove leaves `mod\` identical (211 entries); P19-P21 found on the way |
| 1b | TGC version matrix | **deferred** (D1 revised: v2.5.2 only) | v2.3.4 ↔ v2.5.2: what the submod uses is the same except `news.gui`, `topbar.gui`, `core.gfx` (the trees differ in 223 files, P18) |
| 2 | Law negotiation (script) | LAW-1 short smoke accepted; LAW-2 0.3.3 placement/layering accepted; 0.3.4 board layout, controls and automatic favours: look accepted by the user ("splendid"); 0.3.5 visible buttons, Reforms tab, Decisions filter and no DLL popups: **U-7 partly passed in game (2026-10-18)**; F1 row refresh, F2 top-bar tooltip, F3 load test, D14 offer rule and Bill text overflow go to LAW-3, **0.6.4 installed, U-11 passed 2026-10-18** (G1 open) (`docs/report-law-3-1018.md`); Bill board 2 matches apart from listed deviations | 23 approved groups, 144 transitions; 27 catalogue / 8 GUI / 10 script installer tests; native controls/window/GPU fixtures and 8 DLL tests pass; `docs/report-law-2-controls-1017.md`; `docs/report-law-2-reforms-1017.md` |
| 3 | Charts outside the game (ECO-A) | pending | — |
| 4 | Ghidra: RTTI and subsystems | GH-1 steps 1-4 done; GH-2 hemicycle data/mapping/colours/coordinates and native ordered draw runtime-confirmed | 1 165 RTTI classes, 1 500 vtables; `docs/symbols.md`; `docs/report-u-5-1012.md`; 2026-10-17 external cross-check (Discord CE tables, smedley_kernel): 59 smedley RVAs on function starts, 10 decompiled, `docs/report-research-discord-1017.md` |
| 5 | DLL base | DLL-0/U-4 done; DLL-1 **working prototype, tested in game on 0.2.4**; extended integration coverage still open | 23 guards, Win32 /MT, 128 forwards and native/GPU/installer gates passed; U-5 log: 11133 frames, 2687 ordered chart draws, paused cadence, show/hide, empty errors |
| 6 | Hemicycle | HEMI-1 (DLL-2) **working prototype, tested in game**; user accepts function but says it looks clunky; HEMI-2 0.2.5 works in game; HEMI-3 0.2.6 hover **accepted in game**; 0.2.8 hover/header correction **accepted in game** | 100 seats/seven rows, native ordered GUI pass, native tooltip retained; `docs/report-u-5-1012.md`; 27 runtime guards, 5555 ordered draws, three UK native hover mappings; extended U-5 coverage remains; `docs/report-hemi-3-seam-1012.md` |
| 7 | Charts in game (ECO-B) | renderer prerequisite available as a working prototype; GH-5/save-history work still needed; not selected | — |
| 8 | Rulers and generals | GOV-0 data ready (D4); DLL-1 prototype available for future UI; not selected | TGC v2.5.2: 297 historical leaders, 44 own portraits; rulers: none |
| 9 | Army templates | **TPL-0 0.4.0 functional prototype accepted in game; 0.4.1 placement (owner-chain anchor, P40) confirmed in game; 0.4.2-0.4.4 board-5 details, Recruit (P41), reopen (P42) and navy tab accepted in game (0.4.4)**; TPL-1 Conform: research, native layer (0.5.0/0.5.1, U-9 passed), the Conform button and window (**0.6.2 accepted in game**, `docs/report-tpl-1-window-1017.md` §8), and the **0.6.3 332x594 native army-panel visual pass and flush placement tested and accepted in game** (`docs/report-tpl-1-visual-1017.md`); **U-8 extended cases all passed on 0.6.3 (2026-10-18)**; native Military build panel, local composition-named library, engine-command recruit; automatic replacement of regiments lost to rebellion (TPL-2) is on hold: Conform covers it | user screenshots/logs confirm 4 infantry + 4 artillery + 1 hussar and save/reload; `docs/report-tpl-0-runtime-1017.md`; 0.4.1: translated-parent fixture, `docs/report-tpl-0-placement-1017.md`; 0.4.1 anchor log (1104,490) and 0.4.2, `docs/report-tpl-0-visual-1017.md`; 144 runtime guards; native/reference-window/GPU and 8 DLL installer/Lua tests; 5 Conform GUI/rollback tests; `docs/report-tpl-1-visual-1017.md` |

---

## 2. The version matrix

2026-10-18 refresh: TGC v2.5.2 scan and 0.6.3 generator gates pass; 0.6.3 is tested and accepted in game.

| version | `tgc_scan.py` | generator | loads in game |
|---|---|---|---|
| **v2.5.2 (stable) — the target (D1)** | OK (2026-10-18, 0.6.3 gates) | LAW-0/LAW-1 gates pass; short U-6 smoke accepted; LAW-2 0.3.4 look accepted, 0.3.5 U-7 partly passed; LAW-3 0.6.4 installed, U-11 passed (G1 open); TPL-0 0.4.4 accepted, U-8 complete; TPL-1 0.6.3 accepted in game | yes (U-2/U-3/U-4); DLL-1/HEMI-1 0.2.4 functional checkpoint accepted in U-5, 0.2.5 accepted and retained; 0.2.6 scalar hover accepted; 0.2.8 hover/header accepted; LAW-1 corrected report/Count/save-reload smoke accepted; LAW-2 0.3.3 placement/layering and 0.3.4 look accepted; TPL-0 0.4.4 and TPL-1 0.6.3 tested and accepted in game |
| v2.3.4 (the user's old install) | OK (2026-10-12) | — | yes, skeleton only (U-2); no longer tested |
| `master` | not cloned | — | — (deferred) |

---

## 3. Decisions

Full table in `AGENTS.md` §9. Closed on 2026-10-12: D1, D2 and, in the
user's fourth answer, D4-D7; after the mockups, D9; at the end of the day,
D10:

| ID | Decision | Where it applies |
|---|---|---|
| D4 | rulers **and** generals, starting with the 1836 ones | `docs/instructions-gov-0.md` |
| D5 | multiplayer **later**; the script is still written compatible (R5) | `AGENTS.md` R5 |
| D6 | our own DLL; marked V2DLL code may be used (personal use) | `AGENTS.md` R8, P13; `docs/instructions-dll-0.md` |
| D7 | **political favours and ideological affinity**, deterministic; LAW-1 uses TGC conditional starting votes, affinity for bargaining/promises (user approved 2026-10-12) | `docs/law-design.md`; `docs/instructions-law-0.md` |
| D9 | **hybrid interface, case by case**: each element chooses drawing on top (A) or a real game element (B) | `docs/technical-plan.md` §9.1; every interface card records its choice |
| D10 | **everything in English**: code, docs, file names, in-game text, chat | `AGENTS.md` §7; the whole repository was translated on 2026-10-12 |
| D1 (revised) | **TGC v2.5.2 only**; other versions deferred | `AGENTS.md` §1; `docs/tgc-compatibility.md` (header) |
| D11 | favours: **virtual purchases per bill; automatic accrual like diplomatic points; the rate from prestige, stability, militancy, consciousness, voting pop, monarch/leader support** | `docs/law-design.md` §5, §11; mockup Bill board |
| D12 | army templates (user, 2026-10-17): **stored in a local file** written by the DLL (not in the save); **names built from the composition**; **native windows** generated from TGC's `.gui` (route B); when soldier pops run short, **recruit what fits** and report the rest. Added the same day: a **"Conform to template" button on stacks in the field**; the template is **picked on each click**; missing regiments are **built, then marched to the stack and merged** by the DLL (engine move/merge commands); extra regiments are **detached into a new stack** | `docs/report-tpl-0-research-1017.md` §4; `docs/instructions-tpl-0.md` |

| D13 | Conform: **session tracking; re-Conform after a reload** gathers finished unattached regiments (user, 2026-10-17) | `docs/report-tpl-1-research-1017.md` §4; `docs/instructions-tpl-1.md` |
| D15 | **Templates + Conform: their own package that works almost anywhere**; D1 still holds for laws; LAW-3 first, then MOD-0..MOD-5 (user, 2026-10-18). Sub-decisions S1-S8 the same day: Army on any base (generic), packages Army ("Army Templates and Conform", no `user_dir`) and Politics (TGC v2.5.2, `user_dir` TGCX), later Economy and Cosmetic; one folder per base; own HPM descriptor; DLL in package folders; `mod\TGCX` replaced | `docs/report-modular-scope-1018.md` §6; `docs/instructions-mod-2.md` §2 |
| D16 | V2DLL: **(a) keep our own `lua51.dll` proxy now; (b) after MOD-1, one logic DLL `tgcx.dll` behind two thin loaders** (our proxy, and V2DLL by its mod slot or a setting) once the signature gate is per mechanic (user, 2026-10-18) | `docs/report-v2dll-mod-slot-1018.md` §5-6 |
| D14 | Reforms tab offer: **only bills whose decision passes its own `allow`, and backward steps only where the game allows a backward reform** (user, 2026-10-18) | `docs/report-u-7-1018.md` F4; `docs/instructions-law-3.md` §4 |

Open:

| ID | Question | What the user is shown |
|---|---|---|
| D3 | Submod name and prefix | provisional `TGCX` / `tgcx_` |
| — | V2DLL `v2dll_mod.dll` slot | `docs/report-v2dll-mod-slot-1018.md`: options (a)/(b)/(c), recommends (a) own proxy |

---

## 4. Sessions left (estimate)

See `docs/technical-plan.md` §7. Until the first mechanic visible in game
(laws by script or charts outside the game): **2 sessions**. Until the
hemicycle: **4-6**. Templates: not estimated until GH-4 is finished.

---

## 5. Main risks

| Risk | Effect | What is done about it |
|---|---|---|
| The launcher does not respect the submod's load order (P10) | the submod cannot see TGC's content, or TGC hides the submod's | **resolved** in U-2: `dependencies` with TGC's exact `name` (P19) |
| A TGC version renames a law or a GUI element | the mechanic silently stops having an effect | D1 revised: only v2.5.2 is supported, so this only bites if the user upgrades TGC; `tgc_scan.py` + anchors still fail loudly |
| Creating GUI on the fly turns out too expensive to reverse | route A custom drawing still needs native ordering and hover ownership | D9 case by case: 0.2.4 hemicycle draws inside the ordered GUI pass (P28); native tooltip construction left intact (P27) |
| V2DLL has no licence (P13) | anything using its code could not be published | D6: personal use; everything taken is marked (R8) so it can be removed if it is ever published |
| Desync in MP (P12) | games cut short | R5; D5: MP later, but nothing written that would prevent it |
| The law clock (debate, favours, debt deadlines) stops on save/load | bills never come to a vote, debts never expire | law design v3: TGC's state-of-emergency idiom (a timed modifier, saved with the country, plus a polled event); U-3 saved and loaded mid-debate: the modifier held (2026-10-12); polled events can lag a month (P23) |
| A different executable on another machine | no address is valid | `check_exe.py`; the DLL installs nothing if the signatures do not match (R6) |

---

## 6. Environments and startup

```powershell
cd C:\Users\JC\Downloads\victoria2template
python tools/check_exe.py                       # reference executable
python tools/tgc_scan.py --diff E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC
python tools/install_submod.py E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod   # install the submod (--remove to remove it)
E:\tools\ghidra_12.1.4_PUBLIC\ghidraRun.bat     # project E:\v2re\ghidra\v2game.gpr
```

The game is started from the launcher (`victoria2.exe`) by choosing the
mod; the stable version's `.mod` is called **"TGC v2.5.2 (stable)"**.

---

## 7. Session close (required)

When closing, in this order:

1. Add to the top of `docs/archive/sessions.md` the `NEXT.md` you had at
   the start (full text) and what was done.
2. Rewrite `NEXT.md`: the user's decisions from this session, the next
   session in order, the table of cards with status and instructions.
3. **Write `docs/instructions-<card>.md` for every card of the next
   session** that lacks a current one, and add them to `docs/README.md`.
4. Update §1-§2 of this file and the status in `SUBAGENTS.md` §4.
5. One line below with the date and what was done (newest first).

2026-10-18 (MOD-0 runs 2-3): three mods load with no warning (`-mod=` OrganizedStacks, TGC, TGCX: a mod with an unticked dependency goes first); TGCX alone gets only `-mod=mod/TGCX.mod`, runs on vanilla and crashed after loading (likely the TGC-generated `country_politics.gui`'s 14 TGC-only sprites; inferred). `mod\` unchanged. P49. MOD-0 done; P19 needs a recheck (V2D-1 contradicts it). `docs/report-mod-0-1018.md`.

2026-10-18 (V2D-1): with V2DLL `de66ee61` installed and TGCX first on the command line, V2DLL loaded tgcx.dll 0.7.0 (`ModLoader … -> ok`); 146 OK + 7 FOREIGN (exactly V2DLL's sites), Direct3DCreate9 chained through V2DLL, slot 10 left alone, all modules on through ~11 months of play. TGCX script also worked with TGCX listed first (narrows P19). E: restored byte-identical to 0.7.0. Left: ask V2DLL for a load setting or a search of every `-mod=` (D16). `docs/report-v2d-1-1018.md`.

2026-10-18 (MOD-1 game check): the user played England on 0.7.0, "everything works rather normally"; log clean: 153 OK, all modules on, Bill/Reforms/hemicycle/Templates/Conform active, four P47 slot frees (first live confirmation). MOD-1 done; V2DLL path untested.

2026-10-18 (MOD-1 install): after a normal exit, backed up 0.6.5 (`lua51.dll` `9528c982…` and its record) to `before-070-dll` and installed 0.7.0; installed hashes equal the stage, record card `MOD-1`. Game check next.

2026-10-18 (MOD-1): built 0.7.0 (`docs/report-mod-1-1018.md`): per-module gates, Direct3DCreate9 chaining, slot 10 no longer asserted, `tgcx.dll` + `lua51.dll` loader, installer handles both and the 0.6.5 rollback. Gates green (153 signatures, tgcx_tests ALL OK, installer 11/11, check_exe). Staged in `E:\v2re\logs\tgcx-mod1-1018\stage-070\`; not installed. Repaired four control-character paths in AGENTS/NEXT/symbols.

2026-10-18 (MOD-0 run 1 / P47): run 1 played a year as France; the command line is `-mod=mod/TGC_v2.5.2.mod -mod=mod/TGCX.mod`. Conform stopped: unit-panel slots leaked (P47). 0.6.5 frees them and logs a full table; built, tests pass, installed after a normal exit (backup `before-065-dll`), not yet tested in game (`docs/report-tpl-1-slots-1018.md`).

2026-10-18 (D16): committed LAW-3/U-11 (75e2077). The user chose V2DLL option (a) now and (b) after MOD-1. Next: MOD-0.

2026-10-18 (U-11): installed 0.6.4 after a normal exit (backups `before-064-*`); the user's check passed: D14 offer (15 rows lit, against about 38 in U-7), F1 target-only rows, F2 tooltip, F3 load test gone, clean logs. G1: the » arrow is missing in the small Bill font. Next: 0.6.5 for G1/G3 (`docs/report-u-11-1018.md`).

2026-10-18 (LAW-3): built and staged 0.6.4 (`docs/report-law-3-1018.md`): F1 Reforms rows rebuilt through `CPoliticsView` slot 11 and darkened while a bill is open; F2 `tgcx_` decisions skipped by the top-bar alert list (icon not covered); F3 load test only with `--load-test`; F4/D14 offer from TGC `allow` plus the native reform delay, no backward negotiation; Bill text fitted. Mockup board 1 republished. All automatic gates pass; not installed, not tested in game. Next: U-11.

2026-10-18 (U-7/U-8 checks): two side sessions ran the game checks on 0.6.3; nothing built or installed. U-8 extended cases all passed (complete). U-7 partly passed: F1 rows not re-evaluated with a bill open, F2 top-bar tooltip lists `tgcx_` decisions, F3 load test still installed, Bill text overflow. User decided D14 (offer only what the game allows, A+B). P46. Next: LAW-3 then U-11 (`docs/instructions-law-3.md`, `docs/instructions-u-11.md`).

2026-10-17 (TPL-1 visual): staged 0.6.3 with army-panel styling per user screenshot, lifetime guards/recovery and diagnostics. All automatic gates pass; not installed or tested in game. Next: normal exit confirmation, paired backup, installation and player check (`docs/instructions-tpl-1-063-check.md`).

2026-10-17 (TPL-1 window): Conform button and window, 0.6.0 to 0.6.2 (P44
marker lookup, P45 Merge size 0x4C, hover, D13 revised: build only).
**0.6.2 accepted in game**: builds march and merge, extras detach, no
crash, error logs empty. Pushed as `421bfc9`. Open: a guarded GUI read
fault. Next (user): the V2-style visual pass,
`docs/instructions-tpl-1-visual.md`. `docs/report-tpl-1-window-1017.md`.

2026-10-17 (TPL-0 runtime): user accepts nine-regiment recruitment and
save/reload; original screenshots, proxy/game logs and preferences preserved.
Both error logs empty. Detached panel/top-bar overlap remain visual repair
work; full U-8 and TPL-1 remain pending. `docs/report-tpl-0-runtime-1017.md`.

2026-10-17 (TPL-0 panel): native Templates tab/list/editor/Recruit, atomic
local preference file and guarded engine-command recruitment; 91 runtime
signatures, native/reference-window/GPU, 4 Templates GUI/rollback tests and
existing law gates pass. 0.4.0 installed; done, not tested in game (U-8).
All pre-existing submod files unchanged, paired 0.3.5 rollback retained.
Next: U-8, repairs/acceptance, then TPL-1 scoped research.
`docs/report-tpl-0-panel-1017.md`.

2026-10-17 (LAW-2 reforms): visible Bill buttons; Reforms tab rows lit per
board 1 and clicks present bills; `tgcx_` decisions hidden by the DLL list
filter (P39) and DLL-sent decision popups skipped (14 new guards, 62 total).
Gates pass; 0.3.5 installed, not tested in game. User selected army
templates: TPL-0 research started. `docs/report-law-2-reforms-1017.md`.

2026-10-17 (LAW-2 controls): Bill rebuilt on the 900 x 772 board with composed
installed art; every control through guarded native decision dispatch; Crown,
debts and timers from the timed-modifier list (6 new guards, 48 total);
automatic favour collection. Gates pass; 0.3.4 installed, not tested in game.
`docs/report-law-2-controls-1017.md`.

2026-10-16 (mockup scope clarification): user requires the Bill to match
its agreed mockup. 0.3.3 acceptance covers position/chart placement/layering
only; prototype appearance is not final or R10-accepted. Fidelity is required
implementation alongside full controls, not optional polish. NEXT and card
instructions corrected. `docs/report-law-2-mockup-scope-1016.md`.

2026-10-16 (LAW-2 visual acceptance): user accepted the 0.3.3 screenshot.
Bill centred, projected chart in left panel, Bill above Politics, no native
labels across lower controls. Original screenshot saved with verified hash;
current logs retained. Layout defect closed; full law mechanic unfinished.
`docs/report-law-2-visual-1016.md`.

2026-10-16 (LAW-2 live correction): user left 0.3.2 running. Read-only
snapshot confirmed raw render inputs differ from pixel matrix translation
and Bill overlay entry 170 precedes Politics backgrounds 256/257. Use scene
append and packed setter with positive screen coordinates; chart follows
actual matrix pixels. 0.3.3 native/GPU/installer/executable/TGC gates pass,
installed after user's normal exit, game check pending. P37; next P38.
Computer Use capture/recovery timed out; no GUI inputs sent.
`docs/report-law-2-live-1016.md`.

2026-10-16 (LAW-2 position/drag correction): user tested 0.3.1 while
work continued; Bill was off-screen and not draggable. Replaced centred
packed setter with native absolute x/y setter; checked persistent position
and render translation at three resolutions. Enabled native named-background
dragging. 0.3.2 DLL and regenerated GUI installed; 74 file/42 runtime gates,
seven law gates (25 catalogue/4 GUI/10 installer), native/GPU and eight
DLL installer/Lua tests pass. Game repair/drag check pending; P36; next P37.
`docs/report-law-2-position-1016.md`.

2026-10-16 (LAW-2 layout repair): user tested 0.3.0 and rejected its
layout; original screenshot/logs retained. Native foreground promotion on
opening and actual Bill render translation replace the parent-relative/
independent-chart mismatch. 0.3.1 built, 73 file/41 runtime gates, seven
law checks, reference-engine window fixtures, real-GPU pixel/layer/state
and eight installer/Lua tests passed; installed reversibly on E:. Repair
is done, not tested in game; full LAW-2 unfinished. P35; U-7 repair check
next. `docs/report-law-2-layout-1016.md`.

2026-10-13 (LAW-2 Bill prototype): user confirmed LAW-1 smoke and selected
finishing laws before another mechanic. Native child/labels/Open/Close/Count,
guarded saved-variable model and ordered projected chart implemented;
0.3.0 built and installed with complete prior source/DLL rollback. 70 file
gates / 38 runtime guards, 25 catalogue / 3 GUI / 10 script installer /
8 DLL installer tests plus native/GPU pass. Prototype is not tested in
game; full actions and automatic favour accrual remain unfinished.
`docs/report-law-2-bill-1013.md`; LAW-2/U-7 selected, next P35.

2026-10-13 (U-6 simpler testing): replaced the full player assignment with
a short Present/Count/save-reload smoke test. Extended scenarios remain
coordinator coverage; added `tools/tgcx_check_laws.py` to run existing gates
and copy current logs into a new private bundle. No installation or game
launch; acceptance remains pending. Smoke evidence cannot substitute for
remaining live mechanics/native comparisons.

2026-10-13 (LAW-1 report correction): user screenshot showed every report
option, despite empty error/system_error logs. Preserved evidence and
restored conditional firing of one ordinary report after decision counting;
removed the interpreter's unsupported UI assumption. 24 catalogue tests
(including 231 valid tally combinations), seven installer tests, LAW-0 and
TGC scan pass. Corrected candidate installed on E:, pre-fix rollback retained;
DLL 0.2.8 unchanged. Generated decisions now 35,319,087 bytes; retest load
and presentation first. U-6 in progress, P34, next P35.
`docs/report-law-1-report-fix-1013.md`; online artifact remains pending.

2026-10-12 (LAW-1 native implementation): all base affinities approved;
imported TGC conditional starting support, balanced 100-seat counts and
fresh spending/target gates. 23 catalogue + 7 reversible installer tests,
LAW-0 byte identity, TGC scan and 45 file gates pass (27 runtime unchanged).
Installed game-test candidate on E:; complete LAW-0 rollback retained;
DLL 0.2.8 unchanged. No game run yet. Local low/high-pressure mockups
updated/exported; online browser still unavailable. U-6 next; P33, next P34.
`docs/report-law-1-native-1012.md`; `docs/instructions-u-6.md`.

2026-10-12 (LAW-1 start/handoff): user accepts HEMI-3 0.2.8; preserved
27-guard/1451-draw runtime and empty errors. LAW-1 preview: 23 groups,
46 entries, 144 transitions, nine blocs; 12 tests pass, LAW-0 byte identity
preserved. Ten affinity groups approved, thirteen pending. User approves
TGC starting votes + affinity bargaining after installed-script comparison.
Native vote migration and 100-seat conservation remain; no LAW-1 install.
Local mockup updated/exported; online artifact inaccessible (no browser).
`docs/report-law-1-start-1012.md`; instructions-law-1.md. User requested
handoff; no further implementation begun. Next P33.

2026-10-12 (HEMI-3 seam correction): user reports 0.2.7 cover short by
about two pixels. Measured actual native header artwork; 0.2.8 aligns cover
at first background row y=101, replacing text-maximum-derived y=104. GPU
checks preserve the border and cover the seam. All automatic gates pass;
installed with game closed, visual confirmation pending. 0.2.6 baseline
retained; P32 refined, next P33. `docs/report-hemi-3-seam-1012.md`.

2026-10-12 (HEMI-3 acceptance/header correction): user confirms 0.2.6 hover
works, reports header overlap. Preserved accepted runtime log/DLL: 27 guards,
5555 ordered draws, three UK hover mappings, empty errors. 0.2.7 clips the
cover below the header; all automatic gates pass, installed with game closed.
Visual correction not tested in game; 0.2.6 retained. P32, next P33.
`docs/report-hemi-3-header-1012.md`. LAW-1 remains for later discussion.

2026-10-12 (HEMI-3 implementation): bounded native scalar hover research,
shared seat geometry, guarded bounds/slice hooks and fallback. 0.2.6 built/
installed on E: with game closed; 27 guards, native/GPU/Lua/eight installer
checks and law checks pass. **Done, not tested in game**; one combined U-5
run next. 0.2.5 retained. User will consider LAW-1 after acceptance.
P31, next P32. `docs/report-hemi-3-1012.md`.

2026-10-12 (HEMI-2 acceptance/handoff): user confirms 0.2.5 works,
reports chart tooltip mismatch. Saved original 2560x1440 PNG and accepted
10032-draw log outside git, 23 guards OK, empty errors. HEMI-3 hover
integration next; extended U-5 coverage remains. P30, next P31.
`docs/report-hemi-2-handoff-1012.md`. No new build or replacement.

2026-10-12 (HEMI-2 build/check repair): recovered prior source polish, fixed
PowerShell redirected unittest output and HEMI-2 install metadata. New
GPU feather check and all automatic gates pass; 0.2.5 DLL/submod installed.
Game acceptance pending; combined HEMI-2/U-5 test next. P29, next P30.
`docs/report-hemi-2-build-1012.md`.

2026-10-12 (U-5 functional acceptance/docs): user confirms 0.2.4 works,
requests project/Markdown updates and notes clunky appearance. Preserved
11133-frame/2687-draw log, paused samples, show/hide and empty errors.
Working prototype now game-tested; do not claim all integration checks
passed. HEMI-2 polish queued with instructions; remaining U-5 coverage
bundled with that future build. No code/build/install changes.
`docs/report-u-5-1012.md`.

2026-10-12 (U-5 layering repair): user confirmed 0.2.3 works but draws
above native tooltips. 147442 frames/85103 chart draws and normal exit;
error logs empty. Native named overlay/ordered pass identified live.
0.2.4 built/tested/installed, 23 guards; real GPU layering and native state
checks passed. Game acceptance pending. P28 added, next P29.
`docs/report-dll-12-layering-fix-1012.md`.

2026-10-12 (U-5 crash repair): 0.2.2 chart displayed, then crashed. Dump
shows engine tooltip copying; removed guessed single-string suppression.
0.2.3 built/tested/installed; native tooltip left untouched. Stability/game
acceptance pending. `docs/report-dll-12-tooltip-fix-1012.md`.

2026-10-12 (U-5 live correction): read-only inspection confirms complete
ideology data and paused cadence. CEU3Graphics identity and absolute root
coordinates corrected; 0.2.2 built/tested/installed, 19 signatures green.
Chart game acceptance still pending. `docs/report-dll-12-live-fix-1012.md`.

2026-10-12 (U-5 repair): 0.2.0 chart absent; failed log preserved. Scoped
final-Present hook and viewport restoration built/tested/installed as 0.2.1.
Eighteen signatures, real D3D9 and eight installer/Lua tests passed. Exact
live blocker remains unconfirmed; U-5 retry with game kept open is next.
`docs/report-dll-12-fix-1012.md`.

2026-10-12 (DLL-1 + HEMI-1): combined at user request; renderer/100-seat
hemicycle, engine-name mapping, geometry/lifetime/tooltips built and installed.
Sixteen gates, real D3D9 and eight installer/Lua checks green. Both cards
**done, not tested in game**; one U-5 batch next. `docs/report-dll-12-1012.md`.

2026-10-12 (U-4 complete): user reports all steps worked; screenshots
match logged shares (83/5/12%, raw sum 32767), actual base 0x007D0000,
eight signatures OK. Error logs empty, January 6 reload in game.log.
DLL-0 tested in game; GH-2 core layout/data R, indices 1/2/6 confirmed.
Only one snapshot retained: cadence and full mapping/length remain
follow-up work. DLL-1 instructions written; see `docs/report-u-4-1012.md`.

2026-10-12 (DLL-0 installation): at the user's explicit request, ran the
installer on E:\ and verified active/original hashes and installation
record. U-4 gameplay pending; instructions start at §2. Evidence:
`docs/report-dll-0-install-1012.md`.

2026-10-12 (DLL-0 implementation): logging proxy built and automatically
verified (native hook fixtures, 128 forwards, real Lua execution and eight
installer tests); installer dry run passed. **Done, not tested in game**;
not installed, U-4 next (`docs/instructions-u-4.md`). P24 (ASLR-relocated
signature operands). REA reviewed and recorded for GH-4/templates; not
installed. Starting NEXT.md and research instructions archived.

2026-10-12 (U-3 complete): points 6-7 with a 60-day test build: a broken
promise (+2 militancy, the double-price modifier for 10 years), the 25-seat
lots, screenshots compared with the Bill board. P23 (a polled event fired a
month late). **LAW-0 done**; the normal build reinstalled.

2026-10-12 (LAW-0 and U-3 points 1-5): LAW-0 generated and installed;
the user played it: favours, bill, purchases, promise, save/load, veto
and pass all as designed, `error.log` empty. P22 (no hidden events: the
user's run of the `hidden = yes` test opened a window). Feedback "more
clicks than thought" -> 25-seat purchase lots (data `lots`), the Bill
board's lot selector (artifact v29, PNGs re-exported). Test build with
`debt_days` 60 installed for points 6-7.

2026-10-12 (U-2 and law design v3): U-2 passed on TGC v2.5.2 and v2.3.4
(after P19-P21: `dependencies` naming the TGC in use, `000_tgcx_`
localisation overrides, branching in the firing effect). The user revised
D1: **TGC v2.5.2 only**, other versions deferred. Docs research
(`docs/scripting-reference.md`, `docs/tgc-howto.md`) and R10 (the mockups
are the spec). Law design v3 and LAW-0 v3 rebuilt on TGC's idioms: timed
modifiers + polled events (the state of emergency), step-chains, the count
inside decision effects (Vic2 has no hidden events, P22), `scaled_militancy`;
the Bill board mapped element by element; three options left to the user
(`docs/law-design.md` §11). Mockup PNGs re-exported (new favours medallion).

2026-10-12 (SUB-0 and D10): SUB-0 done, not tested in game: `submod\TGCX`
with a decision and two events that say which TGC is loaded (the
`great_gamer` flag that v2.5.2 gives the UK), a `DEN` key overridden on
purpose, and `tools/install_submod.py` (install + remove leaves `mod\`
identical). P18: `tgc_scan.py --diff` compares the inventory, not the
files. The user decided D10: everything in English; the whole repository
was translated (file names included: `NEXT.md`, `SUBAGENTS.md`,
`docs/archive/`, `instructions-*`, `report-*`; cards LEY → LAW, GOB → GOV).

2026-10-12 (D7 revised and mockups): the user disliked BPM's stalling; D7
becomes **political favours and ideological affinity**, deterministic
(`docs/law-design.md` v2, LAW-0 rewritten; the earlier version, verbatim,
in `docs/archive/`). Interface mockups using the game's and TGC's sprites
and bitmap fonts (private artifact, link in `docs/README.md`): politics
with ruler, hemicycle and negotiation badges; bill; head of state and
generals; economic history; army templates.

2026-10-12 (decisions D4-D7): the user chose staged negotiation (D7), our
own DLL with marked V2DLL code (D6), 1836 rulers and generals (D4) and MP
later (D5). Law design in `docs/law-design.md` (sources: Vic3 dev diary 80
and Better Politics Mod); instructions for LAW-0, DLL-0 and GOV-0; P16 (no
`if`) and P17 (toolset `v145`).

2026-10-12 (bootstrap): feasibility analysis (for each mechanic: what
script gives, what needs the DLL, what is left to reverse); the user chose
TGC as the base (D2) and compatibility with any version (D1). Downloaded
TGC v2.5.2, Ghidra 12.1.4, V2DLL; Ghidra analysis with the repo's symbols;
`check_exe.py` and `tgc_scan.py` gates; docs with the SMW port's layout.
At the end: RTTI inventory exported and the upper house found in the code
(`CCountry+0x8B0`). `docs/report-initial-1012.md`.

---

## 8. Checks: the scripts

| script | what it checks | exits with 1 if |
|---|---|---|
| `tools/gen_signatures.py [--check]` | shares the reference hash, 91 runtime signatures and relocation offsets with the DLL | reference mismatch, unsupported relocation or stale generated header |
| `tools/build_dll.ps1` | Release/Win32 proxy, shared signature header | generation or `/W4 /WX` build fails |
| `tools/test_dll.ps1` | native/real-D3D9 fixtures and eight installer/Lua tests, outside the game | fixture, forwarding, safety or round-trip test fails |
| `tools/install_dll.py <game> [--dry-run] [--remove]` | reference executable, original Lua identity, proxy exports, running-game gate; reversible rename installation/removal | foreign files, mismatch or active game; refuses mutation |
| `tools/check_exe.py [exe]` | SHA-256 and byte signatures of the reference executable | the hash or a signature does not match |
| `tools/tgc_scan.py <TGC> [--out x.json]` | inventory of what the submod uses; GUI anchors; event range | an anchor is missing or there are events in the reserved range |
| `tools/tgc_scan.py --diff <A> <B>` | what changes between two versions (inventory only, P18) | same, in either of the two |
| `tools/mockups/export.py [--src] [--out]` | re-exports the mockup PNGs and the overview from the boards in `E:\v2re\mockups` (R10) | Chrome fails or a board is missing |
| `tools/gen_laws.py [--tgc]` | generates the law mechanic from `data/tgcx_*.txt` + TGC's `issues.txt`; braces, unique keys | an option is missing in TGC, a key repeats, braces do not balance |
| `tools/test_gen_laws.py` | runs the generated decisions for the 1836 United Kingdom against the design's numbers | any number differs |
| `tools/test_law_catalog.py` | native support, allocation, negotiation, catalogue/migration and size behavior | any expected behavior fails |
| `tools/tgcx_check_laws.py` | one-command existing law gates and private log snapshot, including Bill GUI gates, without game control or installation | any automatic gate fails; live acceptance remains separate |
| `tools/tgcx_bill_gui.py --stage <private source> [--header]` | own Bill fragment/texture/localisation and catalogue header | protected staging, missing/duplicate named root or invalid layout |
| `tools/tgcx_templates_gui.py --stage <private source> [--header]` | native Templates fragment, installed unit metadata and private art | missing reviewed land types, bad stage or assets |
| `tools/test_templates_gui.py` | byte-preserving inherited/TGC GUI insertion, catalogue/asset/layout and staged rollback | a named anchor, real asset or rollback differs |
| `tools/test_bill_gui.py` | preserves installed GUI bytes outside insertion, fallback and metadata bounds | anchor, CSV or catalogue/layout check fails |
| `tools/test_install_submod.py` | staged installation, backup round-trip and failure safety | live state changes on a rejected operation or rollback differs |
| `tools/install_submod.py <mod> [--remove] [--tgc] [--source] [--dry-run] [--backup]` | stages generation, retains rollback, installs/removes `TGCX\` + `TGCX.mod`; refuses to delete a folder with files lacking `tgcx_` | the target is not ours or not a `mod\` folder |
| `ghidra/ImportV2Symbols.java` | imports `Symbols/*.csv` into the project | — (prints OK and failed rows; without that line it did not run, P15) |
| `ghidra/ExportRtti.java <output.md> <classes>` | regenerates `docs/rtti-classes.md` | — |
| `ghidra/DecompileAt.java <output> <VA>…` | pseudo-C of individual functions, to `E:\v2re\work\` (R8) | — |

### 8.1 What to run depending on what changes

| change | run |
|---|---|
| anything in the submod that names TGC things | `tgc_scan.py` on each version in §2 |
| the DLL or an address | `check_exe.py` |
| a generator operation (COMPAT-1) | the generator on each version in §2, twice (idempotent) |
| the submod's files | `install_submod.py` (reinstall) before U-tests |
