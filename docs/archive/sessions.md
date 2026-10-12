# Session log (archive)

> Every session, when closing, adds here **the `NEXT.md` it had** and
> **what was done** (rule: `ROADMAP.md` §7). Newest goes **on top**. What
> has been written is not edited.
>
> Translated to English on 2026-10-12 (D10). File and card names in the
> older entries were updated to the current English ones (`PROXIMO.md` →
> `NEXT.md`, `instrucciones-*` → `instructions-*`, LEY → LAW, GOB → GOV…)
> so the links still work; the content is otherwise unchanged.

---

## 2026-10-12 (2) — Decisions D4-D7 and staged law design

### The `NEXT.md` we had (verbatim)

### NEXT — what comes next

2026-10-12. **Bootstrap done (I-0).** TGC v2.5.2 (stable) cloned and
installed next to the user's TGC without touching it; Ghidra 12.1.4 with
`v2game.exe` analysed and the RE repo's symbols imported; V2DLL as a
reference; gates `tools/check_exe.py` and `tools/tgc_scan.py`; documents.
**At the end, a preview of GH-1 and GH-2:** RTTI inventory exported
(`docs/rtti-classes.md`, 1 500 vtables) and **the upper house found in the
code**: `CCountry+0x8B0`, `int64[]` by ideology at `+0x10`
(`docs/symbols.md`). Report: `docs/report-initial-1012.md`.

#### 1. User decisions

**Closed (2026-10-12):**

- **D1 — compatible with any TGC version.** Nothing copied from TGC;
  whatever has to be overridden is generated from the installed version
  (`docs/tgc-compatibility.md`).
- **D2 — TGC as the base**, instead of HPM. For testing: stable v2.5.2.

**Open (U-1), in order of urgency:**

1. **D7 — what "negotiating a law" means**: A (buy support), B
   (concessions) or C (staged negotiation); `docs/technical-plan.md` §2.
   Blocks LAW-0, the first visible mechanic.
2. **D6 — DLL base**: a V2DLL fork, our own chained with V2DLL, or our own
   without V2DLL. V2DLL has no published licence (P13).
3. **D4 — "leaders"**: new rulers, the generals that already exist, or
   both.
4. **D5 — multiplayer** from the start or single player first.
5. **D3 — the submod's name** (provisional `TGCX`, prefix `tgcx_`).

#### 2. The next session, in order

1. **U-1**: ask the §1 decisions (those the user has not answered).
2. **GH-1 step 5** — constructors and sizes of the mechanics' classes
   (steps 1-4 done on 2026-10-12).
3. **SUB-0** — submod skeleton and load test (with **U-2**: the user starts
   the game). It decides whether the compatibility design works in
   practice (P10).
4. **COMPAT-0** — version matrix.
5. Depending on D7: instructions for **LAW-0**. Depending on D6: for
   **DLL-0**.

| card | status / what to do | instructions |
|---|---|---|
| U-1 | decisions D3-D7 | §1 |
| GH-1 | steps 1-4 done; step 5 missing (constructors, sizes) | `docs/instructions-gh-1.md` |
| GH-2 | read in the code; confirm with the game running (the user's Cheat Engine or DLL-0's log) | `docs/symbols.md`, "Missing" |
| SUB-0 | minimal submod, reversible installer, who wins when a key is overridden | `docs/instructions-sub-0.md` |
| U-2 | the user tests SUB-0 with v2.3.4 and v2.5.2 | `docs/instructions-sub-0.md` §5 |
| COMPAT-0 | `master` + old versions, `tgc_scan.py --matrix` | `docs/instructions-compat-0.md` |
| ECO-A | charts from save files (no RE, no pending decisions): the fallback if the user cannot test in game | `docs/technical-plan.md` §3 |

#### 3. Safety net and continuity

Before touching addresses: `python tools/check_exe.py` (it must print
`ALL OK`). Before touching anything that names TGC things:
`python tools/tgc_scan.py --diff E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC E:\Victoria.II.v3.04.Inclu.ALL.DLC\mod\TGC_v2.5.2\TGC`
(today: only `core.gfx`, `news.gui`, `topbar.gui` differ; anchors and
range OK). `check_exe.py` has 7 checks (hash + 6 signatures, 2 of them for
the upper house).

Ghidra: one writer at a time in `E:\v2re\ghidra`. Nothing from `E:\v2re`,
the game or TGC enters git (R8). The repository has no commits yet: the
first one when the user asks.

On close: archive this file in `docs/archive/sessions.md` and write the
instructions for the next session's cards (`ROADMAP.md` §7).

### What was asked

"Staged negotiation like in Victoria 3 mods such as Better Politics Mod;
our own DLL, but since it is for my own use you can use the other one too;
both [rulers and generals], but focusing on the ones there were in 1836;
MP later."

### What was done

- D4, D5, D6 and D7 closed in `AGENTS.md` §9, `ROADMAP.md` §3,
  `docs/technical-plan.md`. R8 allows marked V2DLL code; P13 resolved for
  personal use.
- Research: Vic3 dev diary 80 (enactment phases and outcomes) and Better
  Politics Mod (projected vote, the ruler's veto, manoeuvres). Nothing
  copied.
- `docs/law-design.md`: the design translated into what Vic2 script can do
  (no `if`, fixed weights in `random_list`, generated support steps).
- Instructions for LAW-0, DLL-0 and GOV-0.
- Measured in TGC v2.5.2: 297 historical leaders, 75 with `picture` (44 of
  their own); 223 portraits (45 of its own); no rulers.
- New pitfalls: P16 (there is no `if`), P17 (V2DLL asks for toolset
  `v145`; here there is `v143`).

---

## 2026-10-12 — Bootstrap: feasibility, downloads, Ghidra and documents

There was no `NEXT.md`: it is the first session.

### What was asked

1. Can rulers with portraits, a hemicycle, law negotiation, EU4-style army
   templates and more economy charts be added to Victoria II, starting
   from `maxioten/Victoria2-Reverse-Engineering`, on top of HPM? Or is a
   more extensive decompilation needed?
2. Would it be easier to decompile the game with Ghidra first? Integrate it
   with the latest TGC version.
3. Download stable TGC and everything needed; a submod compatible with any
   version; documents following the logic of the SMW-to-Amiga port's
   (`C:\Users\JC\Downloads\sma\port-amiga`).

### What was answered and done

- Feasibility: the RE repo covers economy, AI, events, technology; **not**
  GUI, upper house, reforms, leaders or recruitment. A full decompilation
  is not needed, but RE per subsystem is. The practical base is the V2DLL
  technique (a `lua51.dll` proxy). Five V2DLL signatures match the user's
  `v2game.exe`.
- Ghidra does help: 1 165 named RTTI classes, among them each mechanic's
  (`CConstructUnitCommand`, `CLeader`, `CIssueChange`, `CGuiObject`…).
- TGC: the user's is v2.3.4; latest stable v2.5.2; `master` is still
  active. The user chose TGC as the base (D2) and compatibility with any
  version (D1).
- Downloaded and installed: TGC v2.5.2 (`mod\TGC_v2.5.2`, its own `.mod`),
  Ghidra 12.1.4, V2DLL. Ghidra analysis with the repo's symbols.
- Tools: `check_exe.py`, `tgc_scan.py`, `ImportV2Symbols.java`,
  `ExportRtti.java`.
- Documents: `AGENTS.md`, `NEXT.md`, `ROADMAP.md`, `SUBAGENTS.md`, `docs/`
  (index, technical plan, compatibility, pitfalls, three instruction files,
  report, symbols, RTTI inventory). No `CLAUDE.md`: the user asked for
  `AGENTS.md` to replace it.
- Ghidra: analysis 752 s; 260/260 symbols; 1 500 vtables. Preview of GH-2:
  upper house at `CCountry+0x8B0` (read in the code, not run).

Report: `docs/report-initial-1012.md`.
