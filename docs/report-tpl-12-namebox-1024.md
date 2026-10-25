# TPL-12: unit name box stops at Disband depleted (0.11.16)

2026-10-24. **Done, not tested in game.** Installed in
`work/Victoria-II-TGC252-own-loader`; candidate
`work/tpl12-01116/ATCU-0.11.16-candidate.zip` (SHA-256 `5ad36357…0414`,
logic `tgcx.dll` `21a2db08…0d0f`). Rollback: `work/tpl12-01116/before-01116`
(0.11.15, 58 files hashed; record `installation-01116.json`).

## Request

User: "Have ATCU narrow the unit name box's clickable area so it stops at
Disband depleted while that button is shown." The problem is described in
`report-tpl-12-passed-1024.md`: TGC's `unitname` edit box (x 38, width 190)
covers the left 14px of `disbandtoosmallbutton` (x 214).

## What changed

- The unit name edit box's text entity (edit box `+0x110`, vtable RVA
  `0xA3BB58`) is hit-tested by `FUN_009DE0A0` (vtable slot 20, RVA
  `0xA3BBA8`). It is thiscall: (x, y, …) in screen pixels, `RET 0x20`, and it
  returns the hit in ST0. That is the same ABI as the HEMI-3 bounds hook.
- `templates_gui::namebox_thunk` replaces that slot. A point is refused
  (`fldz`) only when both of these hold:
  - the entity is a registered unit name box;
  - x is at or past Disband depleted's left screen edge.
  Every other text entity goes on to the original function.
- `univ_cf_tab` already reads Disband depleted's shown flag (`+0x4F`) and its
  position every frame, so it now also does the registration:
  - it finds `unitname` through GUI slot 16 (`0x5E4D20`) and checks the edit
    box vtable (`0xA39878`) and the entity vtable;
  - it records the limit, which is the root's screen x plus the button's
    local x;
  - when the button is hidden, the record is removed.
  A record older than 250 ms (a closed panel) is ignored.
- New signatures in `tools/check_exe.py` (5, all OK): the lookup, the edit
  box slot 15, the hit test's prologue and its `RET 0x20`, and the slot 20
  pointer. If any of them fails, the narrowing is not installed and the log
  says "Name box narrowing off". These signatures do not gate drawing or
  laws (`lib.rs` `drawing_signatures`).
- Only the hit test changed. The box still draws at its full width, and its
  text is not clipped.

## Checks

- `cargo build/clippy -D warnings/test --release`: 33 tests pass. That
  includes 2 new name-box tests (limit edge, hidden button, other entities,
  a stale panel, slot reuse).
- `check_exe` ALL OK (211 signatures). All `tools/test_*.py` suites pass
  except `test_province_rename.py`. That suite fails because the separate
  Province Rename V2DLL plugin build is missing ("requires the matching x86
  V2DLL plugin"). It is unrelated to this change.
- **Not verified in game:**
  - the click and the tooltip at the button's left half;
  - typing in the box to the left of the button;
  - the hidden-button case.

  The log should show `Name box hit test hooked` at start, and
  `Name box: stops at Disband depleted x=…` once a depleted army is selected.
  If it shows `unitname edit box or text entity not recognised`, the lookup
  assumption is wrong.
