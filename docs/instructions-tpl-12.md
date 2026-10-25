# TPL-12: replace depleted regiments

Current completion (2026-10-24, session 75): **native-Y adaptive header and single-line titles implemented in Rust
0.11.12**. Local ZIP with README/changelog is ready; installed and verified
in the own-loader TGC test copy, not tested in game.
Current build/install: `report-tpl-12-title-alignment-1024.md`.
Current visual pass/log audit: `report-tpl-12-visual-check-1024.md`. `report-tpl-12-header-1024.md`; next steps:
`instructions-tpl-12-check.md`. The 0.11.7 short gameplay pass is recorded
in `report-tpl-12-replace-1024.md` section 9. Older research/layout steps
below are retained as history; the Second layout section is the current
accepted design. The old disband proposal is superseded by the user's
detach answer.

2026-10-24, session 74. Card text: `archive/sessions.md`, "TPL-12 Replenish
depleted regiments up to the template". User, same day: "lets do tpl-12
next (depleted provs)"; asked what depleted means: **"both, but you get to
choose thru a button"** (low strength, and a pop too small).

Rust only (all new DLL code). **Built 2026-10-24 as 0.11.7, not tested in
game: `docs/report-tpl-12-replace-1024.md`.** User: "Keep it as a separate
window, as in the accepted mockup. Build TPL-12 in Rust." (A variant docked
inside Conform, `6-conform-replace-docked-1024.png`, was rejected.)

## Research (2026-10-24, read-only Ghidra on 62d48c20...6648; C static)

| What | Ghidra VA | RVA | Evidence |
|---|---|---|---|
| `CDisbandRegimentCommand` vtable | 0x00E034D4 | 0xA034D4 | RTTI; command id **0x40F**; clone allocates **100 (0x64) bytes** (`FUN_005E11B0`, `push 100; call new`, P45) |
| ctor (empty) / ctor (id) | 0x005E0E90 / 0x005E1000 | 0x1E0E90 / 0x1E1000 | `+4` = 0x18D, `+0x2C/+0x30` = 0x40F; `+0x3C` resolved object, `+0x40/+0x44` the regiment's id pair (type, id; copied by `FUN_0072D4E0`) |
| execute (slot 6) | 0x005E10A0 | 0x1E10A0 | registry lookup of `+0x40/+0x44` (high/low registry by type < 0x1269), and only if the regiment's `+0x74` (its stack) is set: `FUN_005CB230(regiment)` (detach) then `FUN_0068A900(regiment)` (delete) |
| `CDisbandUndermannedCommand` | 0x00E03494 | 0xA03494 | id **0xA48**; execute `FUN_005E0BF0` takes a **stack**: every regiment whose pop (`+0x30`) has size (`pop+0x58`) **below** `[[g_economyDefines]+0x20]+0x10 / 1000` (the minimum pop for a regiment) is detached and deleted; an emptied stack is deleted. This is the unit panel's `disbandtoosmallbutton` |
| Pop colour in the regiment row | 0x00794052 | 0x394052 | `FUN_00965870(pop)` (regiments the pop supports, `symbols.md`) vs `FUN_00965930` (regiments it already has): **red** when it supports none, **yellow** when it supports fewer than it has |
| Regiment strength | | | `regiment+0x3C` (int). Native reserve pick `FUN_0059AE90`: `random(100.5) <= +0x3C` and organisation `+0x40 > 999`; V2DLL's copy uses `random(96)`. **Scale not confirmed** (0..100 percent or thousandths of the type's `max_strength`): the first candidate logs it beside the panel's number |

So: **pop too small** has a native definition (red: below the minimum;
yellow: the pop backs fewer regiments than it has). **Low strength** needs
the scale confirmed in game.

Disbanding is a native command (R5). The Vic2 disband returns no men: the
regiment is deleted, and its pop stays where it is.

## Mockup (awaiting the user's OK)

Third draft. User: "shouldnt replace be a button? and then you choose what
you want". `mockups/6-conform-replace-1024.png` and
`6-conform-replace-choose-1024.png`, drawn by `work/tpl12/render_replace.py`:
staged 0.11.3 Conform art, the native budget slider sprites
(`bg_horizontal_slider_180`, `scrollbar_left/rightbutton`,
`scrollbar_slider`), Arial; not captures.

- **Conform is unchanged** except a third footer button, **Replace
  depleted**, between Conform and Cancel (92 / 116 / 92 px).
- The button opens a small **Replace depleted** window beside Conform, in
  the same style:
  - **[x] Low strength, below [slider] 50%**: the threshold;
  - **[x] Pop too small**;
  - either or both may be ticked; the choice is saved with the Conform
    settings;
  - a preview ("2 regiments to replace / Infantry 3 (strength 38%),
    Artillery 1 (pop 820)");
  - **Replace** and **Cancel**.
- Replace builds one regiment of the same type for each depleted one
  (Conform's culture rules and land reach). When the new one merges, the old
  one is disbanded with `CDisbandRegimentCommand`, so the army is never
  short.

A native slider (route B) needs research: the engine's scrollbar type and
how to create one from our GUI definitions. Fallback if that fails: the same
sprites, with the arrows and clicks on the track moving the handle in
steps (drawn by us, route A).

## Answers (user, 2026-10-24)

"1. yes 2. yellow too 3. detach it, user can choose to disband."

1. Slider **10% to 90% in steps of 10**, default 50%.
2. **Pop too small** means red **and yellow**: the pop supports no regiment
   (below the minimum), or fewer regiments than it has.
3. The old regiment is **detached** into its own stack (Conform's detach,
   Create + Transfers, P51) when its replacement merges; it is not
   disbanded. The player can disband it with the native button. So no
   disband command is needed; `CDisbandRegimentCommand` stays research.

The third mockup draft is taken as accepted with these answers. Its
"old ones disband as they merge" line now reads "old ones are detached as
they merge".

## Steps

1. Mockup accepted, questions answered.
2. Research the native slider (scrollbar type, creation, value read).
3. Promote the gates to `check_exe.py` SIGNATURES: the disband command
   (vtable, ctor, size 100, execute), the minimum pop define, regiment
   `+0x30/+0x3C/+0x40`, `FUN_00965870/00965930`.
4. Rust: `conform.rs` plan gains replacements (a new `CfTask` kind holding
   the old regiment's id); `conform_native.rs` builds them like missing
   regiments; on merge it posts `CDisbandRegimentCommand` for the old one.
   TPL-7 orders keep the old id so a load can still finish it.
5. Tests (plan, orders file round trip), gates, candidate, in-game check.

## Second layout (user, 2026-10-24, after 0.11.7 passed in game)

User: "perhaps itd be worth it to make the depleted regiments thing as
another button besides the conform to template one, and leave conform to
template as is". Culture, when asked: **boxes in the chooser**.

Mockups `mockups/6-replace-button-1024.png` and
`6-replace-button-header-1024.png`, drawn by `work/tpl12/render_button.py`.
They are drawn on the accepted 2026-10-19 capture, with staged 0.11.7 art
and Arial; they are not captures.

**Header buttons, option A (user, 2026-10-24: "i like A").** The user saw
both 20 px buttons sit on the braid and Replace look too much like Conform.
`work/tpl12/render_icons.py` drew four options
(`mockups/6-replace-button-options-1024.png`); A was chosen:

- **Both buttons 16x16 at the same top** (capture y 219), so they end at y
  234, above the braid at y 235. Conform's keeps its right edge; Replace sits
  3 px left of it.
- **Conform's art** is its 0.11.7 art scaled to 16 px: the native
  `unit_disband` frame (4 px corners kept, edges stretched), TGC's green
  centre, the infantry figure.
- **Replace's art** is the same frame with vanilla's maroon centre
  (88,22,20), the unit strip's infantry figure and the native low-strength
  bar (`tooltip_strength`, frame 2).
- The two show and hide together.
- **Conform is back to 0.11.6.** Its footer has Conform and Cancel, each
  148 px.
- **The chooser** opens where Conform opens (unit panel +331, +26).
  Opening either window closes the other. It is 332x270:
  - the strength and pop band, as in 0.11.7;
  - a **Culture** band with Conform's three exclusive boxes (Primary +
    accepted, Primary only, Non-core only; none ticked means any culture),
    saved with the rest of the choice in `tgcx_replace_settings.txt`;
  - the preview and the status line;
  - Replace and Cancel.
- **Long location names.** A long province name in the location row may run
  under the new button; it already runs under Conform's.

## Current layout correction: beside native actions (2026-10-24)

User accepts 0.11.8: "works great", but asks for buttons as large as native
Disband regiments, at the top; clarifies **"not above, right beside"**.
Current reference: `mockups/6-replace-beside-20px-1024.png` and
`6-replace-beside-20px-detail-1024.png`. Both buttons 20x20, same native
header row, 3px gaps. TGC Replace (168,32), Conform (191,32), immediately
left of native actions; anchor is native disbandbutton. Long army names
have less room and may run under the buttons. Chooser and mechanics unchanged.

Built/installed in Rust 0.11.9; `report-tpl-12-beside-1024.md`. The above-header
draft was superseded before installation and retained privately. Prior option A
16px layout is historical. Next: `instructions-tpl-12-check.md` step 4.

## Current spacing/visibility correction and 1px lift (2026-10-24)

User supplies 0.11.9 screenshots: use native Disband/Disband depleted gap,
and move our pair into the slot when the latter is hidden. Final steering:
**move both up 1px**. Native 21px pitch gives a 2px painted gap. TGC positions:
shown native depleted: Replace (172,31), Conform (193,31); hidden: Replace
(193,31), Conform (214,31). Follow its actual +0x4F control visibility.

Current Rust 0.11.11 installed; report-tpl-12-spacing-1024.md. Mockup:
`mockups/6-replace-spacing-aligned-states-1024.png` (actual-art composite).
Prior 0.11.9 and pre-lift layouts remain historical. New behavior awaits
player check, instructions-tpl-12-check.md step 4.

## Live alignment supersedes the lift; long-title fix (2026-10-24)

User's actual 0.11.11 capture proves the lifted borders are 1px too high
(y184 vs native y185), and another capture shows Hus wrapping over the
regiment count. Current Rust **0.11.12** restores native Y (TGC y32),
retains 21px pitch/visibility shifts, and fits long Conform list titles to
one line using the actual loaded font plus ellipsis. Counts and complete
composition data are unchanged. Prior y31 previews are superseded.

Installed and verified; next player check in instructions-tpl-12-check.md.
Report: report-tpl-12-title-alignment-1024.md. Pixel evidence and raw captures
are retained under work/tpl12-01112; visual references in that report.

## 0.11.12 visual short pass, functional testing open (2026-10-24)

User: "it worked. still lacks testing. check logs". Alignment/title fixes
accepted visually. Latest run: 206 guards OK, custom diagnostics clean,
profile error logs empty; Conform/Replace preview read. No queue/resume/
join/detach result recorded. Full testing remains open; prioritize a
replacement saved during construction, reloaded and followed to merge/
old-regiment detach, with choice/culture/yellow-pop cases. Preserved logs:
work/tpl12-01112/check-1024. Report: report-tpl-12-visual-check-1024.md.
