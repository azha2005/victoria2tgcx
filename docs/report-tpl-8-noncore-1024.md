# TPL-8: Non-core only culture mode, built in Rust (0.11.3)

2026-10-24, session 74. Implements D19 as approved on 2026-10-21: a third
**Non-core only** checkbox in both Templates and Conform. The three culture
boxes exclude each other; no box ticked means all cultures; clicking the
ticked box clears it. Non-core means a soldier pop whose culture is neither
the country's primary nor one of its accepted cultures. It has nothing to do
with the province's territorial cores. The Rust hold ended in session 73
(NEXT.md: all new DLL code is Rust).

**Done, not tested in game.** Nothing was installed. The game was not
launched. Nothing was committed or pushed.

## 1. What changed

| Area | File | Change |
|---|---|---|
| Filter | `rust/tgcx/src/templates.rs` | `CULTURE_NONCORE = 3`. `army_apply` keeps a pop only if its culture is neither the primary nor accepted. Build Army and Conform missing builds share this one path, so preview and recruitment agree. Unknown modes, an unreadable culture list and pops without a culture all **refuse**; nothing silently becomes All |
| Preferences | `templates.rs` | New `tgcx_templates_v3.txt` (header `tgcx_templates 3`, `@culture=0..3`). Load order: v3, then v2, then v1. An older file is imported only when every newer one is absent, and an invalid file never falls back to an older one. v2 and v1 are never rewritten, so a rollback build still reads its own file. The v2 parser still rejects `@culture=3` |
| Templates GUI | `tools/tgcx_templates_gui.py`, `rust/tgcx/src/templates_gui.rs` | Third box and label on the existing culture row (y 439), inside the 405px left column |
| Conform GUI | `tools/tgcx_conform_gui.py`, `rust/tgcx/src/conform_gui.rs` | Culture band grows to two lines (198..252). Non-core only sits on line 2 under Primary + accepted. The Changes divider moves to 254 and rows start at 288: **6 visible Changes rows instead of 7**. Overflow is still counted in the summary ("N more rows") |
| Headers | `dll/tgcx/tgcx_templates_catalog.h`, `tgcx_conform_layout.h` | Regenerated: one new button and one new text each. Conform's `tgcx_cf_changes` is now 6 |
| Version | `rust/tgcx/Cargo.toml`, `lib.rs`, `tools/build_universal_army.py`, `install_dll.py` | `TGCX TPL-8 0.11.3`. The Universal builder refuses any other DLL (a 0.11.2 DLL would leave the new box dead) |
| Tests | `templates.rs` `#[cfg(test)]`, `tools/test_army_filter_layout.py` | See §3 |

The ticked state is the native `check_on` frame in its own window, as for
the other boxes (P41/P42). Each box has its own visibility-cache slot:
Templates `SHOW_END` +1, Conform +1 after the mobo box. Clicking a box
saves first and refreshes the preview. This works while paused, as before.

## 2. Deviations from the mockup (R10)

The baseline was `mockups/5-templates-filters-1019b.png` and
`6-conform-filters-1019b.png`. The new previews are
`mockups/5-templates-noncore-1024.png` and `mockups/6-conform-noncore-1024.png`.
Like the earlier previews, they are rendered from the staged art and the
generated coordinates in Arial. They are layout previews, not game captures.
Renderer: `work/noncore-0113/render_previews.py`.

- **Templates:** Primary only moved from x 204 to 196 to make room for the
  third box (x 296). The label boxes shrank to 104/64/84px. The native Arial12
  text is 93/59/68px wide, so 13-17px gaps remain in game. The preview
  font is wider than the game's and looks tighter than the game will.
- **Conform:** a second line, as the user allowed on 2026-10-21. One Changes
  row is lost (7 to 6). The outer size (332x594), the footer, the summary
  and the overflow totals are unchanged.
- The remote mockup artifact was not republished. The 1019 filter boards
  were also local previews.

## 3. Verification

- `cargo build --release` (i686), `cargo fmt --check`,
  `cargo clippy --release --all-targets -- -D warnings`: clean.
- `cargo test --release`: 5 new tests pass, 6 runs in a row:
  - culture modes on a mixed province, with a fake country laid out like
    the engine's (+0xE54/+0xE58/+0xE5C list, +0xE64 primary): All keeps 5,
    Primary + accepted 3, Primary only 1, Non-core only the 2 others;
  - refusals: mode 4, country 0, a pop with no culture, filters disabled;
  - parser: v3 reads mode 3; v2 with mode 3, v1 with a culture line,
    mode 4 and a v3 template without a culture line are all rejected;
  - save round-trip writes `tgcx_templates 3` with modes 3 and 2. An
    unknown mode is never written and the existing file is kept;
  - load order: v2 imported when v3 is absent, v1 when v2 is also absent,
    an invalid v3 never falls back.
  The tests share one lock: the unit catalogue and the filter switch are
  single-threaded globals, as in the game. Running them in parallel at
  first showed a race in the test harness only.
- `python tools/check_exe.py`: ALL OK. `tgc_scan.py` on TGC v2.5.2:
  anchors and range OK.
- All `tools/test_*.py` pass except `test_province_rename` (1 error:
  "requires the matching x86 V2DLL plugin"). That test needs the Rename
  DLL, which is not built here. TPL-8 does not touch Rename.
- Layout tests: three boxes on one row within 405px with each label clear
  of the next box; two Conform lines inside the band; divider and six rows
  clear of the summary at 492.
- The Rust DLL exports `tgcx_Init` and `V2Mod_Init` and contains the 0.11.3 marker.

None of this is gameplay acceptance: not visibility, hit-testing, font fit
or a real mixed-culture recruit.

## 4. Candidate (local only)

`work/noncore-0113/ATCU-noncore-0.11.3-test.zip`, 43 files,
SHA-256 `9801d0b1b59dbed8b8cd844e20e6a23e6413b0fa1a1417a4f141f376019d101a`.
Rust logic DLL `4960aa2f0b138f6ab135dfca510c96c2e7e8e01b5c20d58fdcd945548d622e06`
(as `tgcx.dll` and `v2dll_mod.dll`). Loader: the accepted C++ `lua51.dll`
`ab8e3653...` (the Rust loader is RUST-CHECK's business). The art was
regenerated from installed TGC v2.5.2 into `work/noncore-0113/art/`
(ignored, game-derived). Expected log line: `TGCX TPL-8 0.11.3 tgcx.dll`.

**This is also the first Rust logic DLL in a package.** Its in-game check
therefore covers part of RUST-CHECK as well: Templates and Conform in
general, not only the new box.

## 5. What is not done

- In-game check: `docs/instructions-tpl-8-check.md`. The user installs and
  launches. Installation needs a fresh normal-exit confirmation.
- The C++ DLL was **not** rebuilt. If it were rebuilt from the new headers,
  it would show a Non-core box that does nothing. The C++ 0.11.2 release
  and its package tools stay valid only at their committed revision. The
  release default is still C++ 0.0.2 (`prepare_atcu_release.py` unchanged).
- Vanilla/HPM/other-base visual check; multiplayer (D5).
- No new pitfall. The next one is still P64.

## 6. Installation into the test copy (2026-10-24)

After the user's fresh normal-exit confirmation: no `v2game.exe` process;
`check_exe.py` on `work/Victoria-II-TGC252-own-loader/v2game.exe` ALL OK.
The active package there was Universal ATCU 0.11.2 (`tgcx.log`: TPL-7
0.11.2, settings `TGC252_Fresh082_20261019`, `tgcx_templates_v2.txt` with 2
templates). Backup: `work/noncore-0113/before-0113/` (package, descriptor,
loader, the three settings files, the old `tgcx.log`; 46 hashes in
`before-0113.SHA256.txt`). The old and new packages have the same file set.
The package folder was replaced and the descriptor copied. All 41 mod files
were verified against the candidate manifest. The root `lua51.dll` was
already byte-identical to the candidate's loader. The main E: install is
unchanged. The game was not launched.

Rollback (after a normal exit): delete `mod/tgcx_army_universal`, then copy
back `before-0113/mod/*`. Settings need nothing restored: v2/v1 are never
written by 0.11.3; delete `tgcx_templates_v3.txt` to forget 0.11.3 edits.

## 7. In-game result (user, 2026-10-24)

The user says "it works", and sent two screenshots (Templates and Conform,
Austria 1836, TGC v2.5.2). `tgcx.log` of that run:

- `TGCX TPL-8 0.11.3`, signature gates all OK, no fault or failure lines.
  `Templates v2 import ... original retained`, then the v3 file loaded
  with 2 templates. After the edits, `tgcx_templates_v3.txt` holds both
  templates with `@culture=3`. The hash of `tgcx_templates_v2.txt` is
  unchanged (`666a74fe...`).
- Templates, Non-core only: `12 queued, 0 left out` and `10 queued, 1 left
  out` (the engineer). Every queued pop is in a Hungarian, Czech or Slovak
  province (617-639). The native queue names the regiments Hungarian/Czech/Slovak, so
  no primary-culture pop was used.
- Conform on V. Armee (8 regiments): 3 extra cuirassiers detached
  (`3 of 3 extras arrived`), 6 built, 1 left out (engineer: "no eligible
  pop in land reach"). The screenshots show the three boxes on one row,
  the two-line Conform band, the 6 Changes rows and the mobo box.

Not shown by this run: save/reload of the choice, the march and merge
after the builds finish, and switching while paused. Seen in the
screenshots, from before TPL-8 (unchanged layout): a long template name in
Conform's slot list wraps over its "N regiments" line, and the left-out
Changes title is pale yellow on parchment, hard to read.

TPL-8: **passed in game** for recruitment and the GUI. Save/reload and the
merge remain unconfirmed.

## 8. Save/reload and merge (user, 2026-10-24)

Same session, continued `tgcx.log`: after a load,
`6 saved orders resumed after a load (6 still being built), 0 refused`. Each
of the six regiments built for the Non-core Conform then
`marches to province 641 (safe path)` and `joins 756`. The mobo box
toggled on and off (`marked for 5 infantry` / `no longer marked`). The
template choice is in `tgcx_templates_v3.txt`, which is outside the save,
so a reload cannot change it. v2 is still `666a74fe...`. The game was closed
normally (no process left).

TPL-8: **passed in game** (recruit, GUI, Conform detach/build/resume/merge).
Switching while paused was not separately shown. Release still C++ v0.0.2.
