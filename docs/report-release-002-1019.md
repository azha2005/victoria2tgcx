# RELEASE: ATCU 0.0.2 prepared, 2026-10-19

Status: **published 2026-10-19** as a pre-release, like 0.0.1:
<https://github.com/azha2005/victoria2tgcx/releases/tag/v0.0.2>. Tag
`v0.0.2` points at `cae2a68`. The assets are `ATCU-0.0.2.zip` (1 108 851
bytes; the GitHub digest and a re-download both match the SHA-256 below) and
`ATCU-0.0.2.zip.sha256`. The body is `docs/release-0.0.2/RELEASE_NOTES.md`.
It was created through the user's signed-in Chrome on the user's request.

## What it is

ATCU 0.0.2 is internal build 0.11.2, with the log marker `TGCX TPL-7 0.11.2`:

- culture and continent filters (TPL-5/TPL-6, 0.11.0 and 0.11.1;
  `docs/report-army-filters-gui-1019.md`);
- Conform orders that survive a load (TPL-7;
  `docs/report-tpl-7-orders-1019.md`).

## Inputs

- Payload: `work/orders-0112/ATCU-orders-0.11.2-test.zip`, SHA-256
  `42cb2b3bb07f4a8b81e2c3eb906590b0270660d3fa5ebc10c05037a1f6096fae`. This
  build was installed and passed in game; its manifest sits beside it.
- Logic `tgcx.dll` (also shipped as `v2dll_mod.dll`):
  `7afefc61427be3c6dae12bd8004cdadba2b8c8866fd3cddb9c17dab76eb2c49e`.
- Loader `lua51.dll`:
  `ab8e3653a8ea697fa29e05907659b13a69af43c223d0f50116fa58865d4b36dd`. Its
  source is unchanged since MOD-2 0.8.0; this binary is the one rebuilt for
  0.11.2 and used in the own-loader run.
- Source commit: `a90050c06b4cea8211e4124f7c8f782b6860e8f2`. The tool now
  refuses to build while `dll/` or `tools/` differ from HEAD.

## Output (`work/releases/final/`, ignored by git)

- `ATCU-0.0.2.zip`: 43 files, 1 108 851 bytes, SHA-256
  `e656c58dd4cc040f5fc98ced40e3f5b28819a97de66df257e4c722644becb4a6`.
  CRC is OK and `verify_release` passed on the written archive.
- Beside it: `.SHA256.json`, `.RELEASE.json`, `.PROVENANCE.json` (32
  textures, 4 of them new: the checkboxes derived from
  `checkbox_default.dds`) and `.zip.sha256`.
- Compared with the tested payload, only the launcher descriptor changed
  (LF line endings).
- README.txt is `docs/release-0.0.2/README.txt` with CRLF line endings. The
  GitHub release body is `docs/release-0.0.2/RELEASE_NOTES.md`.

## Gates

- `tools/test_atcu_release.py`: OK (7 tests).
- The 0.11.2 DLL, Python and `check_exe.py` gates passed for TPL-7 (same
  binaries).

## Test coverage (stated in README and notes)

- 0.0.2 was checked on TGC 2.5.2 with V2DLL 5.82 (filters; Conform orders
  resumed across a load) and with the ATCU loader (templates and Conform).
- Vanilla, HPM 0.4.5 and DoD Fan Fork were checked with 0.0.1 only. The
  0.11.x filters were not rechecked there; this is listed as pending in
  RELEASE.json.
- Still open, as for 0.0.1: the project licence (left undecided by the
  user's choice), the publication status of the bundled derived artwork,
  and a test on a second PC.

## Next

Still open: rechecking the 0.11.x filters on vanilla, HPM and DoD; a
second-PC recipient test; the licence; the artwork publication status.
