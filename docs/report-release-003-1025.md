# RELEASE: ATCU 0.0.3 candidate, 2026-10-25

Status: **prepared and validated, final build not written yet.** The tool
refuses to build until `dll/`, `rust/` and `tools/` match a commit, because
the release names that commit. Nothing is published. Publishing needs the
user's explicit word, as for 0.0.1 and 0.0.2.

## What it is

ATCU 0.0.3 is internal build 0.11.16 (log marker `TGCX TPL-12 0.11.16`).
The user passed it in game on 2026-10-25 ("worked"), after 0.11.15 passed
Replace depleted. New since 0.0.2 (0.11.2):

- Non-core only (TPL-8);
- Build Army gathering (TPL-11);
- Mobo stacks (fedacking's contribution);
- Replace depleted (TPL-12);
- the Rust DLL;
- orders resumed after main-menu loads (P64/P65);
- the name box narrowed beside Disband depleted.

## Inputs

- Payload: `work/tpl12-01116/ATCU-0.11.16-candidate.zip`, SHA-256
  `5ad363579e8c450eb37e8662d473d2554a24b9766036efe14de5f238795d0414`
  (installed and passed in game; its manifest sits beside it).
- Logic `tgcx.dll` (also `v2dll_mod.dll`): `21a2db08…960d0f`.
- Loader `lua51.dll`: `ab8e3653…4b36dd`, the same as 0.0.1 and 0.0.2.
- Texture inventory: 38 textures. The 6 new ones are the Replace button and
  the chooser's slider parts. Their origins are recorded in
  `prepare_atcu_release.art_origin`.

## Tool changes (`tools/prepare_atcu_release.py`)

- Version 0.0.3, build 0.11.16, with the new input hashes.
- The ZIP now also holds `CHANGELOG.txt` at its top level. It is ASCII and
  converted to CRLF, like README.txt.
- The source gate now covers `rust/` too. The V2DLL attribution scan also
  reads `rust/**/*.rs` and skips `target`; it finds 42 markers.
- The runtime checks and the pending list are updated.
- `tools/test_atcu_release.py`: OK (7 tests).

## Validation build (scratch, 2026-10-25)

`build(..., commit='validation-uncommitted')`:

- 50 files: the 48 payload files, plus README.txt and CHANGELOG.txt.
- `verify_release` passed on the written archive.
- Only the launcher descriptor changed from the tested payload (LF line
  endings).

The real build goes to `work/releases/final/ATCU-0.0.3` once the sources are
committed:

```
python tools/prepare_atcu_release.py --source work/tpl12-01116/ATCU-0.11.16-candidate.zip --output work/releases/final/ATCU-0.0.3
```

## Not covered

- vanilla, HPM and DoD (0.0.3 features not rechecked there);
- V2DLL 5.82 with 0.0.3;
- Build Army gathering save/reload;
- multiplayer;
- a second-PC recipient test;
- the licence (undecided).
