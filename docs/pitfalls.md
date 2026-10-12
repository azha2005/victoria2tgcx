# Known pitfalls (full text)

> The index, one line per pitfall, is in `AGENTS.md` §8. Here is the full
> text: cause, symptom, fix and how to check it. **Read the ones for the
> area you are touching before changing anything.** A new pitfall goes here
> and in the index, with the next number; never renumber.

---

### P1 — `git clone` with `core.autocrlf=true` converts TGC to CRLF

**Found:** 2026-10-12, when cloning v2.5.2.

**Symptom:** every `.gui` in v2.5.2 "differs" from v2.3.4's, but the diff
marks all 2 649 lines of `country_politics.gui`: it is only the line
ending. Worse: git decides what is binary by content, and a `.tga` or
`.bmp` with no null bytes at the start can be taken for text and get
corrupted with added `\r`.

**Cause:** the user's git has `core.autocrlf=true` globally.

**Fix:** in every TGC clone,

```bash
git config core.autocrlf false
git rm --cached -r -q . && git reset -q --hard
```

or clone directly with `git -c core.autocrlf=false clone …`. Check with
`git ls-files --eol` (the `w/` column must match `i/`). `tgc_scan.py` also
ignores `\r` when hashing the interface files.

---

### P2 — `TGC.mod` does not state the real version

**Found:** 2026-10-12. Tag `v2.5.2` ships `current_release = "v2.5.1"` in
`TGC.mod`.

**Fix:** take the version from git (`git describe --tags`) or, in a folder
without git, from the hash of the files the submod uses (`tgc_scan.py`).
Never from the `.mod`.

---

### P3 — The Windows console is not UTF-8

**Symptom:** a tool prints `sincron�a` instead of `sincronía` from bash or
PowerShell (seen with `check_exe.py` when its messages were in Spanish).

**Fix:** every tool in `tools/` starts `main()` with
`sys.stdout.reconfigure(encoding="utf-8")`. Since D10 the messages are in
English, but TGC names and paths can still carry non-ASCII characters.

---

### P4 — `HPM4.5.mod` points to a folder that does not exist

`path = "mod/HPM4.5"` but the folder is `mod\HPM` (on `D:\` and on `E:\`).
If HPM does not load, this is why. Not fixed (R1): HPM is no longer the
base (D2).

---

### P5 — `victoria2.exe` is not the game

`victoria2.exe` (26 KB) is the launcher. The game, what is analysed and
what loads the DLL, is `v2game.exe` (12 MB).

---

### P6 — The user's `mod\TGC` has junk in its root

The installed v2.3.4 has the save analyser's scripts in the root of
`mod\TGC` (`Artisans.py`, `Country.py`, … `startscript.bat`, `outputs\`),
besides its copy in `Save_Game_Analyzer\`. It is harmless to the game, but
that folder is not a clean reference for v2.3.4: to compare versions, a
clone of the tag is better.

---

### P7 — ASLR: the executable does not load at `0x400000`

`v2game.exe` has a relocation table and `DYNAMIC_BASE`. Ghidra shows VAs
with base `0x400000`; at run time it was seen at `0x00080000`. In the DLL:
`g_base = GetModuleHandle(NULL)` and `address = g_base + RVA`, with
`RVA = VA - 0x400000`.

---

### P8 — Addresses from V2DLL and from the RE repo may come from different builds

The RE repo's `PATCHES.md` warns about it: "These projects may target
different Victoria II executable builds". On 2026-10-12 five signatures
were checked against the user's `v2game.exe` and they match
(`tools/check_exe.py`), but that does not guarantee every address. **Every
new address is added to `check_exe.py` with its expected bytes before it is
used.**

---

### P9 — TGC's official submods override whole files

`TGCDirectImmigration` ships its own `common/buildings.txt`,
`common/pop_types.txt` and `interface/province_interface.gui`, copied from
one TGC version and edited. With another TGC version, those files hide the
new ones: the submod "works" but undoes TGC's changes in those files,
without any error. It is exactly what R3 forbids. Fix:
`docs/tgc-compatibility.md` §3.

---

### P10 — Load order and `dependencies`: unverified

TGC's submods declare `dependencies = { "TGC - The Grand Combination" }`.
It is not verified in this game (3.04, the launcher in `launcher\`) whether
the launcher uses that for the load order, nor what happens if the
installed TGC has a different name (like `TGC v2.5.2 (stable)`, created on
2026-10-12). Until SUB-0 verifies it (U-2), do not assume the submod loads
afterwards.

---

### P11 — The Vic2 GUI is static

A `.gui` defines named elements; the engine fills them according to their
name (`chart_upper_house`, `upper_house`…). There is no way, through
script, to show an image according to a variable or a flag. Consequence: a
ruler's portrait or a hemicycle **need the DLL**. Through script there are
only alternatives: an event's picture, a national modifier's icon
(`icon = N`), or a decision's text.

---

### P12 — Changing the simulation from the DLL desyncs multiplayer

In MP each client simulates on its own and compares a checksum per day
(`FUN_00682EC0`). If one client's DLL writes a field that does not change
on the other, the game is cut with "Games out of synch". Also, every player
needs the same DLL and the same configuration (V2DLL says so for its own
patches). Hence R5.

---

### P13 — V2DLL has no published licence

The repo `artyom-kuznetsov/V2DLL_Victoria-2-Extension` has no licence file;
the README invites people to "investigate v2dll under the hood to use for
your own work", but that is not a licence. Resolved for this project by D6:
it is for personal use, so V2DLL code may be used, marked
`// from V2DLL: <file>` (R8) so it can be removed if the submod is ever
published.

---

### P14 — `cmd` splits `analyzeHeadless.bat` arguments at commas

**Found:** 2026-10-12, when exporting the RTTI.

**Symptom:** `-postScript ExportRtti.java output.md 'CPoliticsView,CBudgetView,…'`
exports the detail of **only one** class. The log shows the arguments
already split (`'CPoliticsView' 'CBudgetView' …`).

**Cause:** `analyzeHeadless.bat` goes through `cmd.exe`, which treats the
comma as an argument separator even though PowerShell sent it in quotes.

**Fix:** the scripts in `ghidra/` accept the list in one argument or in
several (`ExportRtti.java`, `DecompileAt.java` join `args[1..]`).

---

### P15 — A Ghidra script cannot redeclare a `GhidraScript` method

**Found:** 2026-10-12. `ImportV2Symbols.java` declared its own
`setPlateComment(Address, String)`; `GhidraScript` already has one with
another return type. The script **does not compile**, and in headless mode
that does not stop the analysis: the log says `skipping …
ImportV2Symbols.java` and the analysis finishes "fine" without importing
anything.

**Fix:** use `GhidraScript`'s methods and, after any headless run with
scripts, search the console for `REPORT SCRIPT ERROR` and for the script's
summary line (`ImportV2Symbols: N rows OK`). Without that line, the script
did not run.

---

### P16 — Victoria II script has no `if`

**Measured:** 2026-10-12, in TGC v2.5.2's events and decisions: 0 uses of
`if`/`else_if`, against 5 304 of `random_owned = { limit = { owner = { … } }
owner = { … } }`, the usual trick for a conditional effect.

**Consequence:** every conditional branch of an effect is (a) a different
event or option with its own `trigger`/`allow`, or (b) the `random_owned`
trick (it only works if the country has at least one province).
Probabilities that depend on state are done in steps: one event per step
with its `random_list` of fixed weights (`docs/law-design.md` §2).

SUB-0 found that TGC's event options never use `trigger` (0 uses in
v2.3.4's `events\`); whether the engine evaluates the `trigger` of an
`is_triggered_only` event fired by an effect is measured in U-2.

---

### P17 — The V2DLL project uses the `v145` toolset

`V2.vcxproj` asks for `PlatformToolset` `v145` (Visual Studio 2026). This
machine has Visual Studio 2022 (MSVC 14.43, toolset `v143`) and Build Tools
2019 (`v142`). When building V2DLL code: retarget to `v143`, **Win32**
platform. If something uses C++ from a version `v143` lacks, note it.

---

### P18 — `tgc_scan.py --diff` is not a file diff

**Measured:** 2026-10-12 (SUB-0). Between `mod\TGC` (v2.3.4) and
`TGC_v2.5.2\TGC`, `tgc_scan.py --diff` lists only 3 interface files;
`diff -rq` gives **223** differing files (decisions, events, history,
`countries.txt` with `dynamic_tags = yes` and the new tag `JLS`, etc.).

**Cause:** the tool compares the **inventory** (ideologies, laws,
governments, event range, hashes of the `.gui`/`.gfx` files, anchors), not
the tree.

**Consequence:** "no differences" from `--diff` means "nothing the submod
takes for granted changed", not "the versions are the same". To look for
something that tells versions apart (or a name the submod will use outside
the inventory), run `diff -rq --strip-trailing-cr` on the two folders. If
the submod starts depending on something new, add it to the inventory.

### P19 — A submod loaded before TGC is wiped by TGC's `replace_path`

**Measured:** 2026-10-12 (U-2, first run, TGCX + "TGC v2.5.2 (stable)").
The game made the settings folder `TGCX_TGC252` (TGCX first), and
`setup.log` lists the 105 decision files of TGC and not
`tgcx_loadtest.txt`. Denmark kept TGC's name; the load-test decision did not
exist.

**Cause:** `TGCX.mod` named only `"TGC - The Grand Combination"` in
`dependencies`. The v2.5.2 test copy is called `"TGC v2.5.2 (stable)"`, so
the launcher did not order it before TGCX. TGC loaded second, and its
`replace_path` on `decisions`, `events`, `localisation` (and the rest)
discarded everything loaded before it, TGCX's files included.

**Fix:** `dependencies` lists every name TGC can have here:
`{ "TGC - The Grand Combination" "TGC v2.5.2 (stable)" }`. To check a run,
the settings folder must be named `TGC…_TGCX` (TGC first) and `setup.log`
must list `decisions/tgcx_*.txt`. A TGC installed under another `name`
needs that name added.

### P20 — Localisation: the file that sorts first wins

**Measured:** 2026-10-12 (U-2, first run) and documented on the Vic2 wiki,
`Localisation` page (`docs/report-research-1012.md` §1).

**Cause:** when a key is defined in several `.csv` files, the game keeps the
one from the file whose name sorts **first** (inside one file, the last line
wins). TGC defines country names in `00_countries-names.csv`; a
`tgcx_*.csv` sorts after every `00_*.csv`, so its value for an existing key
is ignored whatever the mod load order.

**Fix:** keys that override TGC or vanilla go in a file named
`000_tgcx_<mechanic>.csv` (`0` sorts before `_`). Keys that are ours alone
stay in `tgcx_*.csv`. `tools/install_submod.py` accepts the `000_` prefix.

### P21 — An event's `trigger` is not checked when an effect fires it

**Measured:** 2026-10-12 (U-2, run 2, TGC v2.5.2). The load-test decision
fired 420000001 and 420000002 with `country_event`; their triggers are
opposite (`ENG` has / has not `great_gamer`), and **both** appeared.

**Cause:** an `is_triggered_only` event fired by an effect skips its
`trigger`. (A search summary attributed this to the Vic2 wiki, with events
the game fires itself as the exception; the wiki pages read did not say it.
The measurement is the evidence.)

**Fix:** decide in the effect that fires the event, with the IF emulation
(`random_owned = { limit = { … } owner = { country_event = … } }`), as TGC
does. Keep the event's `trigger` only as documentation or for other ways of
firing it. `docs/scripting-reference.md`.

### P22 — Victoria II has no hidden events: every event opens a window

**Measured:** 2026-10-12. The user, from play: every event pops up. Neither
the wiki's [Event modding](https://vic2.paradoxwikis.com/Event_modding) nor
[How to make an event](https://vic2.paradoxwikis.com/How_to_make_an_event)
lists a `hidden` (or `hide_window`, `is_hidden`) key, and every event needs
an `option`. TGC v2.5.2 has 0 uses. (Later Paradox games have
`hidden = yes`; Vic2 does not.) The user also fired the LAW-0 test event
(`hidden = yes`) before it was removed: `game.log` records "Event: 'Hidden
Event Test' for player" and its window opened; `error.log` stayed empty, so
the unknown key is silently ignored.

**Cause:** the engine predates hidden events; an event is always a window
with at least one button.

**Fix:** silent work goes into **decision effects** (a decision opens no
window), with the IF emulation (P16, P21). Events only for news the player
should see anyway. Something that must run on its own (the favours tick) is
a decision the DLL sends (`CExecuteDecisionCommand`, R5), clicked by hand
without the DLL (R6). The `hidden = yes` test was then dropped from the
generator.

### P23 — A polled event with `mean_time_to_happen = { days = 1 }` can fire a month late

**Measured:** 2026-10-12 (U-3, two runs, TGC v2.5.2, test build with
`debt_days` 60). Promise made on 1 January 1836, so the
`tgcx_debt_deadline_reactionary` modifier ended around 2 March; the broken-
promise event (trigger: the debt flag set and that modifier gone, MTTH 1
day) fired on **2 April 1836** in both runs (`game.log`). The modifier
durations themselves are exact: the broken-debt modifier added on 2 April
1836 with 3 650 days says "Expires on 2 April, 1846".

**Cause:** not documented. The engine evaluates MTTH country events less
often than daily (about monthly here); a search found no Vic2 source for
the interval.

**Fix:** a polled event is "soon after", not "on the day". What must
happen on the day goes into a decision's `allow` (it reads the modifier
live): the vote already checks `tgcx_debate` that way. Deadlines measured in
years (the debts) can absorb the month; show the deadline from the
modifier's own date, not from the event.
