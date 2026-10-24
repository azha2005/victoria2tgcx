# TPL-11: Build Army gathers each template army in one province (0.11.4)

2026-10-24, session 74. Implements D20 as the user approved it on
2026-10-24 ("Mockup OK. 'Already there' means any army of ours; one army per
click. Build TPL-11 in Rust."). Rust only. Research and mockup:
`instructions-tpl-11.md`, `symbols.md` "TPL-11".

**Done, not tested in game.** Nothing was installed. The game was not
launched. Nothing was committed or pushed.

## 1. What it does

- One **Recruit Template** click makes **one army**. Each regiment is built
  in its own source province, as before. The first regiment to appear
  becomes the army and marches to the target by a safe land path (the
  rally-point Move). Every later regiment marches to that army and merges
  into it, with Conform's tracker (Move again only when the army changes
  province, at most one Merge per game day).
- **Gather at < … >** in the footer, right of Recruit from. The arrows
  cycle **Best supply** (default), **Capital** and the **province selected on
  the map** (the label shows its name while the province window is open,
  otherwise "Map selection"). Session only: not saved with the template,
  and reset to Best supply in a new game.
- **Best supply:** the dry plan (the same pop choice Recruit makes, nothing
  sent) lists the source province of every regiment. Of the land provinces
  the player owns *and* controls, ordered by supply limit (then id), the
  first one that every planned regiment reaches by land is the target. When
  none is reached by all, the one most of them reach. Only the best 12 are
  asked, because each answer costs route queries.
- **Taken targets:** a province is taken if **any land army of ours** stands
  there (user's answer), or an earlier click is still gathering there. Then
  the army gathers in the free province around it, ring by ring through our
  own land neighbours (impassable edges skipped), highest supply first,
  up to 6 rings. Armies never merge with each other.
- **Left out:** pops with no safe land route to the target are excluded
  before the plan (Conform's scope, D17). Their regiments are built from
  other pops if possible, otherwise reported as left out.
- **After a load:** each regiment is written to `tgcx_gather_orders.txt` in
  the settings folder (tag, date window, target, click, army once known,
  stack and construction ids). Loading a save of the same country dated in
  an order's window resumes it, as TPL-7 does for Conform. If the recorded
  army is gone (or not in that save), the next regiment to appear leads.
  A separate file keeps the 0.11.3 Conform orders parser untouched, so a
  rollback still reads its own file.
- **Needs one land army of ours** to ask the native route query (it plans as
  that army's owner would march). Without one, nothing is queued and the
  status says so.
- If the TPL-11 guards fail, the footer shows "Unavailable" and Recruit
  works exactly as 0.11.3 (regiments go to the rally point).

## 2. Changes

| Area | File | Change |
|---|---|---|
| Pure logic | `rust/tgcx/src/gather.rs` (new) | Modes, Best supply choice, ring search around a taken target, the gather orders file (strict round trip, eviction, group ids). 7 tests |
| Native layer | `rust/tgcx/src/conform_native.rs` | Supply limit call, adjacency, map selection, capital, province name, the player's land armies; the click (`gather_recruit`); election of the army, resume after a load, and the orders file in the daily driver. Route query generalised to `route_between(army, from, to)` |
| Tracker | `rust/tgcx/src/conform.rs` | `CfTask.group` (0 for Conform); `CfId: Debug`; `id_pair` shared |
| Dry plan | `rust/tgcx/src/templates.rs` | `plan_sources`: the source province of each regiment Recruit would queue |
| GUI | `tools/tgcx_templates_gui.py`, `rust/tgcx/src/templates_gui.rs` | Two arrows and two texts at x 462..650, y 468 (mockup position); Recruit goes through `gather_recruit` |
| Guards | `tools/check_exe.py`, `dll/tgcx/tgcx_signatures.h` | Research gates promoted to DLL guards, plus capital (`+0xBE4`, RVA 0x4CF4A9) and province name (map province `+0xE4`, RVA 0xAAF90/0xC4558). 195 guards |
| Headers | `dll/tgcx/tgcx_templates_catalog.h` | Regenerated: 2 buttons, 2 texts |
| Version | `Cargo.toml`, `lib.rs`, `build_universal_army.py`, `install_dll.py` | `TGCX TPL-11 0.11.4`; the builder refuses any other DLL |

## 3. Native calls (R5, R7)

All reads; no field is written. The only commands are the existing Move
and Merge built as before.

- `FUN_004B3C40` supply limit (RVA 0xB3C40): `push ref, push province`, ESI
  the output, RET 8. Called only for provinces we own and control, with
  `ref = {0, country index}`; on that path it reads only `ref+4`. Checked
  again in the disassembly on 2026-10-24 (`work/tpl11-0114/supply-abi.txt`,
  not in git): EBX/EDI are pushed and restored, ESI is used only as the
  output pointer.
- Route query `FUN_005B8500` with the destination province given
  explicitly instead of the army's own location; the army is any land army
  of ours (owner's access).
- Adjacency, map selection, capital, province name: offsets in
  `symbols.md` "TPL-11". The name at map province `+0xE4` is what the
  native getter returns and what Province Rename writes; that it is the
  displayed (localised) name is **inferred**, not seen in game.

## 4. Deviations from the mockup (R10)

Mockup: `mockups/5-templates-gather-1024.png` and
`5-templates-gather-selected-1024.png`.

- The result goes on the status line ("10 queued, 1 left out; the army
  gathers at Pest (supply 20).", or "... at Buda, beside Pest."). The
  mockup also put the counts on the summary line. The summary line keeps
  the template's composition summary, as before, because it updates on
  every selection, not only after a click.
- With no province window open, the Selected mode label reads
  "Map selection" (the mockup only showed a selected province).
- The remote mockup artifact was not republished (the boards are local
  previews, as for TPL-8).

## 5. Verification

- `cargo build --release` (i686), `cargo fmt --check`,
  `cargo clippy --release --all-targets -- -D warnings`: clean.
- `cargo test --release`: 12 pass (7 new):
  - mode cycling both ways;
  - Best supply: a supply tie asks the lower id first; an island only 2 of
    5 regiments reach loses to the next one all reach; the fallback to the
    most reached; `tries` limits the questions;
  - ring search: a free base is kept; ring one by supply; ring two when ring
    one is full; never through a foreign province; None when all is taken;
  - orders file: round trip, refusals (padded numbers, group 0, unit out of
    range, a Conform orders file), size limit, date window, eviction of the
    oldest ended order, new group ids.
- `python tools/check_exe.py`: ALL OK. `gen_signatures.py`: 195 guards.
  `tgc_scan.py` on TGC v2.5.2: anchors and range OK.
- All `tools/test_*.py` pass except `test_province_rename` (pre-existing:
  "requires the matching x86 V2DLL plugin"). The layout test now checks the
  Gather at row: inside the footer, clear of Recruit from's arrow, label
  between its arrows.
- The DLL exports `tgcx_Init` and `V2Mod_Init` and contains the 0.11.4
  marker.

None of this is gameplay acceptance: the supply call, the route query with
an explicit destination, the map selection read, the election and merge,
and the ring placement have not run in the game.

## 6. Candidate (local only)

`work/tpl11-0114/ATCU-gather-0.11.4-test.zip`, 43 files, SHA-256
`db60e0e3e32f9ad056015226de67b87d115f81ab451bdbb566ca915a600f40f2`.
Rust logic DLL `a74f2b1862a43cfaa6e4028497ebfb5518b29cb5559dc7451ed88b9867e0feb2`
(as `tgcx.dll` and `v2dll_mod.dll`); loader unchanged (`ab8e3653...`). Art
regenerated from installed TGC v2.5.2 into `work/tpl11-0114/art/`
(ignored): only the layout and localisation changed, no sprite. Expected
log lines: `TGCX TPL-11 0.11.4 tgcx.dll` and `Build Army gathering enabled`.

## 7. What is not done

- In-game check: `instructions-tpl-11-check.md`. Installing needs a fresh
  normal-exit confirmation.
- The C++ DLL was not rebuilt; built from these headers its Gather arrows
  would do nothing. The release is still C++ v0.0.2.
- Vanilla/HPM/other bases; multiplayer (D5).
- No new pitfall. The next one is still P64.

## 8. Installation into the test copy (2026-10-24)

After the user's normal-exit confirmation: no `v2game.exe` process;
`check_exe.py` on `work/Victoria-II-TGC252-own-loader/v2game.exe` ALL OK.
The active package was Universal ATCU 0.11.3 (`tgcx.log`: TPL-8 0.11.3,
settings `TGC252_Fresh082_20261019`). Backup: `work/tpl11-0114/before-0114/`
(package, descriptor, loader, `tgcx.log`, the five settings files; 48 hashes
in `before-0114.SHA256.txt`). The package folder was replaced and the
descriptor copied; all 41 mod files match the candidate manifest, with no
extra file. The root `lua51.dll` was already identical to the candidate's
loader. The main E: install is unchanged. The game was not launched.

Rollback (after a normal exit): delete `mod/tgcx_army_universal`, then copy
back `before-0114/mod/*`. Settings need nothing restored; delete
`tgcx_gather_orders.txt` to forget 0.11.4 gather orders.
