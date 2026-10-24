# TPL-11: Build Army gathers around the best-supply province

Decided by the user on 2026-10-24 (D20, corrected the same day: one merged army per template). Rust only, in fedacking's style.

## What it does

- One Build Army click makes **one template army**. Each regiment is built
  in its own source province; when it finishes it marches by a safe land
  path to the target and **merges** into that template's army (Conform's
  march-and-merge, with the first arrival becoming the army).
- **Target:** chosen in a new footer row `Gather at < Best supply >`, next to
  `Recruit from`. Choices: Best supply (default), Capital, the province
  selected on the map. Session only, not saved with the template (like the
  continent).
- **Best supply:** of the provinces the country owns that every source of
  this click can reach by land, the one with the highest supply limit.
- **More than one army:** if one of our armies already stands at the target,
  or a second template army is recruited, that army gathers in the next
  adjacent owned, reachable province by supply instead, and so on around the
  first. Armies never merge with each other.
- A regiment whose source has no land route to its target is **not built**;
  it is reported as left out, as Conform does (D17).
- Orders persist like Conform orders (TPL-7 file): resumed after a load.

## Research done (2026-10-24)

`docs/symbols.md` "TPL-11"; 8 signatures in `check_exe.py`, ALL OK.

- **Supply limit:** native `FUN_004B3C40(province, ref)`, RET 8, result in
  thousandths through ESI, reads only. Called for owned and controlled
  provinces with `ref = {0, country index}`.
- **Adjacency:** `[province+0x5C]` vector `+0xC8/+0xCC`, 0x24-byte entries,
  `+0` type (4 = impassable, skip), `+4` neighbour province id.
- **Map-selected province:** `[[g_world+0xB24]+0x1D08]` is CProvinceView
  (vtable 0xE13768); `+0x24` province; root `+0x20`, shown byte `+0x67`.
  Read only, no detour, so no clash with Province Rename.

## Mockup (2026-10-24, accepted by the user the same day)

`mockups/5-templates-gather-1024.png` (Best supply) and
`5-templates-gather-selected-1024.png` (a selected province). `Gather at
< Best supply >` sits on the `Recruit from` line (caption x 462, arrows 520
and 626, label 82px); nothing else moves. The summary line names the
target. Renderer: `work/noncore-0113/render_gather.py` (Arial, not a capture).

## Steps

1. Research: done (above).
2. Mockup: accepted (above), with the answers below.
3. Done 2026-10-24 (`report-tpl-11-gather-1024.md`). Rust: reuse Conform's reachability (`conform_native::reachable`), safe-path
   Move and the TPL-7 order file (a new order kind, old files still load).
   Plan the targets when Recruit is clicked, so the preview's left-out count
   includes unreachable sources.
4. Done. Tests: Rust unit tests for target choice, supply filling order and
   left-out counting; layout test for the new row; check_exe, full Python
   suite.
5. Candidate 0.11.4 built; install only after a normal-exit
   confirmation; user checks in game: `instructions-tpl-11-check.md`.

Answered by the user (2026-10-24): "already there" means **any army of
ours** at the target; **one army per click**. The saved orders went to their
own file (`tgcx_gather_orders.txt`) rather than a new kind of line in the
TPL-7 file, so a 0.11.3 rollback still reads its Conform orders.
