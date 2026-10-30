# Volunteers 0.0.15: MP package correction

2026-10-29. User clarifies the button correction should be packaged for MP.
The preceding delivered archive selected the SP prototype. A separate plain MP
archive now selects mode=mp_probe and uses the identical corrected0.0.15 DLL.
This is a configuration/packaging correction, not an MP volunteer gameplay unlock.

- ZIP: work/volunteers-mp-0015/final/Volunteers-0.0.15-MP-sync-probe.zip.
  460894 bytes,19 files, no EXE/installer/BAT/CMD/PowerShell/scripts.
  SHA256:2191551f2bf99fbb78af13348f377816205157a28d261167b5d5c54ad5232cc9.
- DLL SHA256:974a306baf282a846bb8ff8d1648adf0223ae5030ff19c1ef1236647cb0dd4ef,
  byte-identical to the0.0.15 SP entry-correction DLL. Native art/layout retained.
- Package fixture passes CRC/manifest/reproducibility, one DLL, eight native
  textures and explicit MP mode. Direct comparison verifies both archives contain
  the same DLL/button corrections. No Rust code changed or rebuild required;
  the preceding61 Rust/107 signature checks remain the DLL's validation scope.
- README explains MP entry/background recovery and paired probe testing. Both
  peers use this archive, select00 loader data, TGC2.5.2 and ZZ MP sync probe.
- MP Send/Recall/combat/siege remain refused. Native command delivery and journal
  comparison are diagnostic only; actual two-peer testing and the original OOS
  remain unverified. No game, desktop, installation or save changes occurred.

Next steps: instructions-volunteers-mp-sync.md. Keep SP0.0.15 and MP0.0.14 as
distinct rollbacks. No new pitfall or visual/mockup change is introduced.
