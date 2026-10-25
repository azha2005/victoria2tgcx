# TPL-12: 0.11.8 installed in the own-loader test copy

2026-10-24, session 76. User explicitly requested installation in
`C:/Users/JC/Downloads/victoria2template/work/Victoria-II-TGC252-own-loader`.
Build, option A and ZIP: `report-tpl-12-header-1024.md`.

**Installed and hash-verified; not tested in game.**

## Gates and installation

- The Windows Toolhelp process check found no `v2game.exe`, both before
  preparation and immediately before the swap. WMI process inspection was
  unavailable; the existing installer's Toolhelp check succeeded instead.
- This was an explicit fresh install request. No separate normal-exit
  confirmation is claimed.
- `check_exe.py` on the requested test copy: all checks pass.
- Candidate directory, ZIP CRC and every manifest entry verified before
  mutation. Previous logic DLL identified as TPL-12 0.11.7.
- The complete replacement package was staged and checked, then the old
  package moved into the validated backup path and the staged package put
  in its place. No obsolete package files remain.
- **48 payload files verified**: 46 package files, descriptor and root
  loader. The loader already matched and was left unchanged.
- **25,466 other test-copy files** retained their SHA-256 hashes, including
  the executable, TGC data and original Lua DLLs. All **eight** current
  `tgcx_*` settings files are unchanged.
- The installed-payload manifest is now current:
  `work/Victoria-II-TGC252-own-loader/universal-army-installed-sha256.json`.

## Backup and evidence

`work/tpl12-0118/before-0118/` contains the complete 0.11.7 package,
descriptor, loader, previous installed manifest, `Logs/tgcx.log` and eight
settings files from `TGC252_Fresh082_20261019`. **59 hashes verified** in
`work/tpl12-0118/before-0118.SHA256.json`.

Installation evidence and the installed payload hashes:
`work/tpl12-0118/installation-0118.json`.

Rollback after the game is closed: preserve the current files, restore the
backup's ATCU package and descriptor, and restore its Replace settings if
the game has since written version 2. Keep replacement-order backups. The
old 0.11.7 chooser reads settings version 1.

## Next check

The user starts the game with TGC v2.5.2 and ATCU selected, old Army/Politics
packages unticked. Continue at **step 4** of `instructions-tpl-12-check.md`:
two small header buttons, independent chooser, saved exclusive culture
choices, restored Conform footer, replacement merge/detach and save/reload.
The next log should identify `TGCX TPL-12 0.11.8`.

No game launched or terminated. Main E: install unchanged. No commit,
push or public release.
