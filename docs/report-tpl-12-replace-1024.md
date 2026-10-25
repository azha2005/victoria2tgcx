# TPL-12: Replace depleted, built in Rust (0.11.7)

2026-10-24, session 74. Card and research: `docs/instructions-tpl-12.md`.
Mockups (accepted): `mockups/6-conform-replace-1024.png` (Conform footer)
and `mockups/6-conform-replace-choose-1024.png` (the chooser). User, last
instruction: "Keep it as a separate window, as in the accepted mockup.
Build TPL-12 in Rust."

**Status: done, not tested in game.** Local candidate only; not installed.

## 1. What it does

- Conform's footer has three buttons: **Conform** (92 px), **Replace
  depleted** (116 px), **Cancel** (92 px).
- Replace depleted opens a separate **Replace depleted** window
  (332x210), placed right of Conform with the bottoms aligned. When the
  screen has no room on the right, it goes left of Conform. Clicking the
  button again, or Cancel, closes it. Closing Conform closes it too.
- **[x] Low strength, below [slider] 50%.** The slider has nine steps,
  10% to 90%. The arrows move one step; a click on the track jumps to that
  step.
- **[x] Pop too small.** Red: the pop supports no regiment, so all its
  regiments in this army count. Yellow: the pop backs more regiments than
  it supports, so only the difference counts, taken from its last regiments
  in this army. A regiment that is already chosen for low strength counts
  toward that difference.
- Either box, both, or neither may be ticked. The choice is kept in
  `tgcx_replace_settings.txt`, in the same settings folder as the templates.
- **Preview.** "N regiments to replace", then a list such as
  "Infantry (38%), Artillery (pop)."
- **Replace.** One regiment of the same type is built for each depleted
  regiment. The build uses Conform's recruit: the selected template's
  culture rule (unrestricted when no template is selected), and only
  provinces with a land route to the army (D17).
  - Each new regiment is tracked like a Conform build: it marches by a safe
    path and merges. It is also a TPL-7 order, so a reload resumes it.
  - When it has merged, and its regiment is seen in the army, the old
    regiment is **detached** into a new stack beside the army. This uses
    Conform's detach (Create + Transfers together, P51), with its check and
    one retry. Nothing is disbanded (user: "detach it, user can choose to
    disband").
- **Never chosen twice.** A regiment is skipped while it is being replaced
  or already leaving. This covers Conform's extras too.
- **Conform is unaffected.** Its plan does not count a replacement as a
  regiment on its way: the old regiment leaves when the new one arrives.
- **Reload.** Replacements being built are kept in
  `tgcx_replace_orders.txt`, one line per replacement: tag, created, done,
  target, joining stack, old regiment. The format is strict, as the other
  order files. After a load, the resumed Conform order carries its old
  regiment again.

## 2. Native reads (R5: reads only; commands are Conform's)

Seven new guards in `check_exe.py` SIGNATURES; `gen_signatures.py` now gives
202. The DLL checks them as `REPLACE_RVAS`. If any guard fails, the chooser
stays closed and the Conform window says why.

| Read | Evidence |
|---|---|
| Strength `regiment+0x3C`, full strength `[regiment+0x38]+0xB0` | CRegiment slot 12, FUN_005BFF80 (RVA 0x1BFF80) |
| Pop `regiment+0x30`, its size `pop+0x58` | FUN_005E0BF0, the native disband-undermanned (RVA 0x1E0C82) |
| How many regiments the pop supports: FUN_00965870(pop, 0), stdcall, RET 8 | RVA 0x565878/0x565921 |
| How many regiments the pop backs: FUN_00965930, EAX = pop, a pushed 0, RET 4 | RVA 0x565930/0x56598A |
| The unit panel colours the pop row from these two counts, both with flag 0 (EBX = 0) | RVA 0x394048 |

Strength percent is `+0x3C * 100 / +0xB0`, both in the same unit.
**Inferred: the scale.** The first eight regiments read log a line like
`Replace probe: regiment N strength +3C a of type +B0 b = p%, pop ...
size s supports x has y`. Compare it with the unit panel to confirm.

## 3. Code

- `rust/tgcx/src/replace.rs` (new, pure):
  - `Choice`, the slider steps;
  - `depleted()`, the rule;
  - the settings file;
  - the replacement-order file;
  - 7 tests.
- `conform.rs`:
  - `CfTask` gains `replaces`, `fresh` and `rp_record`;
  - `pending()` leaves replacements out.
- `conform_native.rs`:
  - the Create + Transfers detach is now the helper `queue_detach`, and
    the build tracking is now `track_build`;
  - Conform behaves as before;
  - new section "Replace depleted (TPL-12)": `depleted_of`,
    `replace_with`, the merge check (`replace_done`), the daily retirement
    (`replace_retire`), the order file, resume and close.
- `conform_gui.rs`:
  - the chooser window: creation, placement, the cached show state of its
    children (P42), labels and clicks;
  - in Universal mode the window is built with `univ_make`, which is now
    `pub`.
- `tools/tgcx_conform_gui.py`:
  - the footer buttons and the `rp_` buttons and texts;
  - the window `tgcx_cf_rp_window` (bg button and icon, two check-on
    windows, nine handle windows, the arrows, nine transparent step
    buttons, Replace/Cancel);
  - the art: `button_s`/`button_m` (the native button's two ends joined),
    `rp_bg` (Conform's braid, parchment, two-line band and footer tray,
    with the native track `GFX_bg_horizontal_slider_220wide`),
    `rp_left`/`rp_right` (`yearslider_left/rightbutton`), `rp_handle`
    (`yearslider_slider`), and a transparent `rp_step`;
  - the header regenerated (30 buttons, 58 texts);
  - every Conform sprite of 0.11.6 is byte-identical.

## 4. Deviations from the mockup (R10)

- **Slider.** The handle moves in nine fixed steps, not continuously:
  one handle window per step, shown by the DLL, with transparent buttons
  over the track. A native scrollbar was not researched. The art is the
  native slider's art, as in the mockup.
- **List text.** The list reads "Infantry (38%), Artillery (pop)" rather
  than the mockup's "Infantry 3 (strength 38%), Artillery 1 (pop 820)".
  The game's regiment names are not read, and the pop size would not fit
  in two lines.
- **Note line.** It is the status line, and shows the outcome after
  Replace. Its default text is the mockup's "Built first within land
  reach; old ones are detached as they merge."

The mechanic itself is unchanged since the accepted mockup, so the
mockup was not redrawn.

## 5. Verification

- `cargo build --release`, `cargo fmt`, `cargo clippy --release
  --all-targets -- -D warnings`: clean. `cargo test --release`: 19 pass
  (7 new).
- `python tools/check_exe.py`: ALL OK. `gen_signatures.py`: 202 guards.
- Python tests:
  - `test_conform_gui.py` (6), `test_universal_army.py` (2, now expecting
    `tgcx_cf_rp_window`) and `test_army_filter_layout.py`: OK;
  - GUI names: 403, no duplicates (P59); braces balanced.

**Not checked in game:**

- the chooser's look and placement;
- the strength scale;
- the pop counts against the panel's colours;
- replacements merging;
- the old regiment leaving;
- a reload.

## 6. Candidate (local only)

`work/tpl12-0117/ATCU-replace-0.11.7-test.zip`, 50 files.

| Item | SHA-256 |
|---|---|
| ZIP | `71ed723877e2b9a4a7ff60fb7be9444ed03ccaf0d4c7bc3c87373db84647e216` |
| Rust logic DLL | `6ff238f79f2537af3ff701019cfaf8595de102ebe54eb44a1cc987de396e2e18` |
| Loader (unchanged) | `ab8e3653...` |

The art is 0.11.6's plus the seven new sprites, from
`work/tpl12-0117/art/TGCX`.

Expected log lines:

- `TGCX TPL-12 0.11.7 tgcx.dll`;
- `Replace depleted enabled: strength and pop support read only`.

**Install** over 0.11.6 in `work/Victoria-II-TGC252-own-loader`, only after
the user confirms a normal exit. Back up first, as for 0.11.6. The existing
settings files keep their formats; the two new files are created on first
use.

## 7. Check in game

1. Open Conform on an army that has worn regiments. Click **Replace
   depleted**: the window opens beside Conform, both boxes ticked, at 50%.
2. Compare the preview with the unit panel's strength and pop colours.
   Read the `Replace probe` lines in `tgcx.log`.
3. Move the slider with the arrows and by clicking the track. Untick each
   box in turn. Reopen the window: the choice is kept.
4. Click **Replace**, then unpause. The new regiments march and merge, then
   the old ones appear in a new stack beside the army.
5. Save while a replacement is being built, then load: it still merges,
   and its old regiment still leaves.

## 8. Installation into the test copy (2026-10-24)

After the user's normal-exit confirmation:

- no `v2game.exe` process was running;
- `check_exe.py` on `work/Victoria-II-TGC252-own-loader/v2game.exe`: ALL OK;
- the active package was 0.11.6 (its DLL carries `TGCX TPL-11 0.11.6`).

Backup: `work/tpl12-0117/before-0117/`, with 49 hashes in
`before-0117.SHA256.txt`. It holds:

- the package folder and its descriptor;
- the loader;
- `Logs/tgcx.log`;
- the six `tgcx_*` settings files of `TGC252_Fresh082_20261019`.

The package folder was replaced and the descriptor copied. All 48 mod files
and the root `lua51.dll` (unchanged) match the candidate manifest, with no
extra or missing file. The main E: install is unchanged. The game was not
launched. **Not tested in game.**

Rollback, after a normal exit: delete `mod/tgcx_army_universal`, then copy
back `before-0117/mod/*`. The two new settings files
(`tgcx_replace_settings.txt`, `tgcx_replace_orders.txt`) can stay; 0.11.6
ignores them.

## 9. 0.11.7 passed in game (2026-10-24)

User: "it worked". No screenshots were taken. Evidence is `tgcx.log` of the
run (`TGCX TPL-12 0.11.7`, every gate OK, `Replace depleted enabled`):

- **Strength scale confirmed:** `+3C 3000 of type +B0 3000 = 100%`. Strength
  is in thousandths of the type's full strength, so the percent rule is right.
- **Pop counts match the panel's rule:** regiment 410's pop (size 350)
  supports 0 regiments (red) and was the one chosen. Pop 5C22A758 supports 2
  and has 2, so it was not chosen.
- **Second Replace:** stack 10281 marched, joined 415, and regiment 410 was
  detached (`1 of 1 extras arrived in stack 11444`, 0 failed).
- **First Replace:** stack 6888 marched and the next day was gone without
  joining. The log says "did not join 415; regiment 410 stays", so the guard
  worked as designed. Why the stack disappeared is not in the log.

User follow-up: make Replace depleted its own button beside the Conform
opening button, and leave Conform to template as it was. This is a mechanic
change, so the mockup changes first (R10).
