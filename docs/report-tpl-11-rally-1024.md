# TPL-11: Gather at uses the land rally point (0.11.5)

2026-10-24, session 74. Follows `report-tpl-11-gather-1024.md` (0.11.4),
which stays as written.

**0.11.6 passed in game** (user, 2026-10-24; §9). Installed in the test copy
(§7, §8).
Nothing was committed or pushed.

## 1. Why

0.11.4 passed its in-game run for Best supply and Capital (`tgcx.log`:
four armies led, marched and merged at Raab, Stuhlweissenburg beside it,
Egersee, and Wien as Capital). The user then asked how to select a province:
opening the Military screen closes the map's province window, so the third
option ("the province selected on the map") could never be used from the
Templates tab. The user's answer: "leave as is and just use the gathering
point chosen as that option". Read as **the game's own rally point**; the
rest of TPL-11 is unchanged.

## 2. What changed

- The third **Gather at** option is **Rally point**. The label shows the
  rally point's province name, or "Rally point" when none is set.
- Recruit in that mode gathers the army at the country's **first land rally
  point** (in the order the game keeps them). The rest is as before: taken
  by any army of ours, the free province beside it by supply; pops with no
  land route left out.
- Refusals: "Set a land rally point first; nothing queued." and "The rally
  point is not in a province you own and control; nothing queued."
- The map selection reads are no longer used. Their gates moved to
  `RESEARCH_SIGNATURES`; three rally point gates took their place in the
  DLL table (195 guards still).

## 3. Native reads (R5, R7)

Read only; no command is new. Evidence in `symbols.md` "TPL-11", rally
point rows:

- `CCountry+0x1570/+0x1574`: a vector of pointers to rally points
  (FUN_0053F3B0, RVA 0x13F3B0, looks one up by province and kind).
- Rally point `+8` province id, `+0x20` kind byte.
- Kind **0 = land, 1 = naval**: the province view's land rally checkbox
  calls FUN_0073C930 with `EBX` (zero there), the naval checkbox with
  `push 1` (RVA 0x33F03C); FUN_0073C930 passes that argument to
  FUN_0053F3B0 as the kind. **C static.**

## 4. Mockup (R10)

The mechanic changed for one option, so the mockup is out of date there:
`mockups/5-templates-gather-selected-1024.png` shows the map selection. The
board was a local preview and was not republished for 0.11.4 either;
the label text is the only difference ("Rally point" / the rally province
name instead of the selected province name). Not redrawn this session.

## 5. Verification

- `cargo build --release`, `cargo fmt`, `cargo clippy --release
  --all-targets -- -D warnings`: clean. `cargo test --release`: 12 pass
  (mode tests use `MODE_RALLY`).
- `python tools/check_exe.py`: ALL OK. `gen_signatures.py`: 195 guards.
- `tools/test_army_filter_layout.py`: OK (layout unchanged).

Not checked in game: the rally point read, the label, the refusals.

## 6. Candidate (local only)

`work/tpl11-0115/ATCU-gather-0.11.5-test.zip`, 43 files, SHA-256
`9cb507d57d28342b71fae21b85429fd04ee112e5f513732aacc4906178cf565c`.
Rust logic DLL `4f8c2037df56b16e115c73fd69cc58dd96eb84c06a6415f63d0ffa777881e948`;
loader unchanged (`ab8e3653...`); art as 0.11.4. Expected log lines:
`TGCX TPL-11 0.11.5 tgcx.dll` and `Build Army gathering enabled`.

Install over 0.11.4 in `work/Victoria-II-TGC252-own-loader` only after a
normal-exit confirmation; `tgcx_gather_orders.txt` keeps its format.

## 7. Installation into the test copy (2026-10-24)

After the user's normal-exit confirmation: no `v2game.exe` process;
`check_exe.py` on `work/Victoria-II-TGC252-own-loader/v2game.exe` ALL OK.
Active package was 0.11.4 (`tgcx.log`: five gather armies, the last at
Ödenburg beside a taken target). Backup: `work/tpl11-0115/before-0115/`
(package, descriptor, loader, `tgcx.log`, the six `tgcx_*` settings files of
`TGC252_Fresh082_20261019`; 49 hashes in `before-0115.SHA256.txt`). The
package folder was replaced and the descriptor copied; all 41 mod files and
the root `lua51.dll` match the candidate manifest, with no extra file. The
main E: install is unchanged. The game was not launched.

Rollback (after a normal exit): delete `mod/tgcx_army_universal`, then copy
back `before-0115/mod/*`.

## 8. 0.11.6: every land rally point (2026-10-24)

The user, after the 0.11.5 install: "when there's more than 1 rally point
it only shows the first and only one". 0.11.5 took only the first land
rally point by design (§2). The 0.11.5 log loaded cleanly with every
gate OK, but has no Recruit line and did not list the rally points.

- The arrows now cycle **Best supply, Capital, then each land rally point**
  in the game's order (mode `MODE_RALLY + k`). The label shows that rally
  point's province name. With no rally point there is one "Rally point"
  entry, as before.
- Each arrow click logs `Gather: N land rally points: <id> <name>; ...`, so
  the log shows what the game holds.
- A rally point removed since it was chosen: the label falls back to
  "Rally point". Recruit refuses with "No land rally point at this choice;
  set one or click the arrows. Nothing queued." The next arrow click treats
  the stale choice as the last one.
- Gates unchanged (195). `cargo test`: 12 pass (the mode test now covers
  three rally points and a removed one). Clippy, fmt, `check_exe.py`, layout
  test: clean.

Candidate `work/tpl11-0116/ATCU-gather-0.11.6-test.zip`, SHA-256
`402c2ca280e62f815cd420b4f1e5f30c42339ebb9f5fe3dd67fc9fd088443970`. Rust DLL
`84737b5ecdf815c0ccfbc40a3c578cde4973a8ce2e23dd39700ad754cee080de`; loader and
art as 0.11.5. Not tested in game.

**Installed** (2026-10-24, after the user's normal-exit confirmation): no
`v2game.exe` process; `check_exe.py` on the test copy's `v2game.exe` ALL OK.
The 0.11.5 install (package, descriptor, loader, `tgcx.log`, six settings
files) is backed up in `work/tpl11-0116/before-0116/`, 49 hashes in
`before-0116.SHA256.txt`. All 41 mod files and the root `lua51.dll` match
the candidate manifest, no extra file. Rollback: delete
`mod/tgcx_army_universal`, copy back `before-0116/mod/*`.

## 9. 0.11.6 passed in game (2026-10-24)

User: "done. wonderful". `tgcx.log` of the run (`TGCX TPL-11 0.11.6`):

- every arrow click listed `2 land rally points: 736 Triest; 642 Raab;`;
- army 6 gathered at Raab (642, supply 52), army 7 at Triest (736, supply
  52), and army 8, with Triest taken, at Görz (737) beside it; 3 queued and
  0 left out each time.

This run's log has no march or merge lines, so the march and merge come from
the user's report and from the 0.11.4 run, which logged them. Not checked:
save/reload of a rally gather, other bases, multiplayer (D5). The mockup
board for the third option still shows the map selection (§4).
