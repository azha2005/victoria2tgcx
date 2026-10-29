# Volunteers0.0.8: isolated installation after normal exit

2026-10-29. User confirmed the game was closed. A fresh process check found no
v2game.exe. Reference EXE/address checks passed, and source hashes matched the
staged0.0.8 build. Ran work/volunteers-0008/update_e_copy.py successfully.

- Installed only our15 mod payloads and test metadata in
  E:/Victoria-II-Volunteers-0004. Fresh readback matches every staged mod hash.
- Nine protected executable, loader and TGC files have identical before/after
  hashes. Main installations and save files were not edited.
-0.0.7 package/config/log backup: work/volunteers-0008/before-0008.
- Installed DLL SHA256: af4fc19a78cbeb10b7f905786113d85a793cfbeb9242076f5f439d7363081d11.
- ZIP SHA256: 3a74938684a3a628c9066c21a039b93bd56d40c7cb831c70c0e813e0b9c9a82f.
- Package marker confirms version0.0.8. Default remains observe; the existing
  tgcx_start_prototype.cmd/.ps1 explicitly sets prototype and launches the copied
  game with TGC then Volunteers descriptors. Launcher inspected, not executed.
- Install evidence: work/volunteers-0008/install-result.json and the identical
  E:/Victoria-II-Volunteers-0004/tgcx_update_0008_manifest.json.

The36 Rust/five package-loader checks belong to the unchanged built candidate.
This installation adds hash/readback verification only. No game launch, desktop
input, successful Send, measured performance result, battle/save-load acceptance
or multiplayer validation was performed. No new subagents, commits or publication.

Next: instructions-volunteers-send-0008.md steps4..10. Use a fresh campaign or
pre-failed-Send save, not the incomplete0.0.7 mission. First select every brigade
in a small army and Send; unpause and inspect reuse=true, complete receipt and
native movement. Then check five-brigade whole selection, partial splitting,
unchanged-day checkpoint stability, player control and remaining prototype cases.
