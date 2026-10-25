# TPL-12: independent Replace button, option A (0.11.8)

2026-10-24, session 75. Supersedes the interface in
`report-tpl-12-replace-1024.md`; that report's section 9 records the
0.11.7 short gameplay pass. Current request: finish
`instructions-tpl-12.md`, with **option A** selected. Rust only.

**Status: built and packaged; not installed or tested in game.**

## Behavior

- Two header buttons share one entry root and its visibility: **Replace**
  on the left, **Conform** on the right. Both are 16x16 with a 3px gap.
  Conform's right edge is retained. The entry follows the effective native
  attrition icon at x minus 38, preserving vanilla/TGC's different heights.
- Art matches option A: native frame corners; green Conform and infantry;
  maroon Replace, infantry and native low-strength bar (frame 2 of 11).
- Conform's footer returns to **Conform / Cancel**, 148px each. The
  template's culture settings and Conform planning are unchanged.
- Replace opens without opening Conform or selecting a template. Its
  **332x270** chooser occupies Conform's usual position (unit panel +331,
  +26, clamped to the screen using the chooser's own dimensions). Opening
  either window closes the other. Repeating the header click toggles it.
- The chooser retains the strength/pop checkboxes and 10-90% stepped
  slider. It adds its own three exclusive culture boxes, with no box
  meaning all cultures. Clicking the selected box clears it. This choice
  is passed to the existing population-level culture filter and recruiter.
- Closing the host, changing armies, or selecting a fleet closes the
  chooser. First show/reopen clears child visibility and text caches (P42).
- Replacement tracking, build-first, safe land reach and detach after
  confirmed merge retain the 0.11.7 command path. Nothing is disbanded.

## Settings and rollback

`tgcx_replace_settings.txt` now writes canonical **version 2**, adding
`culture=0..3`. Version 1 imports the strength/pop/threshold fields with
unrestricted culture. Loading alone does not rewrite the file; a user
change saves version 2 atomically. Malformed data is refused.

The replacement orders format is unchanged. Before installing, back up
the complete old package, descriptor, loader, log and settings. To return
to 0.11.7, restore the package and its version 1 Replace settings backup;
otherwise 0.11.7 defaults its chooser because it cannot read version 2.

## Visual comparison (R10)

References opened: `mockups/6-replace-button-1024.png` and
`6-replace-button-header-1024.png`, plus the user-supplied matching images.
Actual packaged DDS assets and generated coordinates were rendered by
`work/tpl12-0118/render_candidate.py` and inspected:

- `mockups/6-replace-whole-candidate-0118.png`;
- `mockups/6-replace-header-candidate-0118.png`;
- `mockups/6-replace-candidate-0118.png` (chooser only).

The header matches option A, including its braid clearance. Unchanged
Conform textures are byte-identical to 0.11.7. Native arrows are visible
beside the slider; unticked culture boxes use the actual native off frame.
The note uses its generated text field at y=200 (the illustrative board
used y=204). Arial previews cannot confirm native font wrapping or live
hit-testing. These images are layout previews, not game captures. The
historical remote artifact was not republished.

Long location names can still extend under the buttons, as noted in the
accepted instructions. No new truncation or location-row redesign.

## Verification

- Reference executable: all **328** checks pass (runtime, research and
  Rename guards); generated runtime header remains **202** signatures.
- TGC v2.5.2 scan: anchors and reserved range OK.
- Rust x86 offline release build, format check and strict Clippy
  (`--all-targets -- -D warnings`) pass; **21 Rust tests** pass. New cases
  cover culture exclusivity/clearing and v1 import/v2 validation.
- Python GUI checks: **20** pass. Army checks: **17** pass, including the
  new header-root, braid-clearance, footer and chooser-culture regression.
  DLL installer/Lua forwarding checks: **12** pass against their existing
  fixtures. The separate plugin probe loads the **actual packaged Rust
  DLL** and passes; it does not start Victoria II or exercise its GUI.
- All **50** package files match the manifest and ZIP hashes; CRC OK.
  Both logic DLL filenames are identical to the Rust build. Every DDS
  decodes, all declared texture paths exist, and **415** GUI object names
  are unique with balanced braces.
- `git diff --check` passes. No C++ logic extension or C++ release rebuild.

Gameplay, native font layout, save/reload and multiplayer are not proven
by these checks. The current player checklist is
`instructions-tpl-12-check.md`.

## Local ZIP

`work/tpl12-0118/ATCU-0.11.8-candidate.zip` (**1,132,252 bytes**), including
`README.txt`, `CHANGELOG.txt`, the complete additive Universal package,
Rust logic DLL under both loader names, and the unchanged accepted loader.
Sidecar: `ATCU-0.11.8-candidate.SHA256.json`.

| Item | SHA-256 |
|---|---|
| ZIP | `3bf189653b682b3db2f1a9cde202cbff0a158da9e7cbd0a2ad14e34f4b67cc3a` |
| Rust logic DLL | `056e56b5785166a8e465524743348d9fbfef75512537e1e9b9e6a1d205439839` |
| Loader | `ab8e3653a8ea697fa29e05907659b13a69af43c223d0f50116fa58865d4b36dd` |

The previous 0.11.7 test install and its rollback remain intact. No
installation, launch, termination, commit, push or public release.
