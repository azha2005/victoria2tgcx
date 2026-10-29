# Volunteers 0.0.12: plain shareable SP beta ZIP

2026-10-29. The user accepts the CreatineCore setup as working well enough and
asks for a ZIP to send. They explicitly require no BAT/command files or similar,
matching ATCU/PRU packaging. The final archive is a plain mod payload; no setup
program, EXE, script, shortcut or external launcher is included. An initially
prepared installer-build environment was abandoned before creating any installer.
It stays private under work and is absent from the archive.

Output:
`work/volunteers-share-0012/final/Volunteers-0.0.12-SP-beta.zip`.
Size: 421453 bytes. SHA-256:
`9481348cc4aeef0a09b5696f4a8363f4a9c8f6a037a46e9afb1a3f71e0634ae8`.

The archive has 20 files: README, changelog, SHA-256 manifest, the existing
Volunteers mod payload and the small generated-data carrier. The DLL SHA-256
remains `08de457c5b49c12ce990d562a3c9ab1dc87be6ee766f7aa253702c768a00a1a3`.
All 13 existing plugin/UI/localization/marker files match both the accepted
candidate and the actual installed copy byte for byte. The only changes within
the original package are the descriptor's sortable display name and explicit
`mode=prototype` configuration with an updated default comment. Maximum brigades
5 and report cadence 7 remain unchanged. No Rust/native behavior was changed.

Normal launcher selection is explicitly documented as:

1. `00 - Volunteers loader data`
2. The recipient's TGC 2.5.2
3. `ZZ - Volunteers (SP beta)`

Names preserve the launcher's known display-name ordering. No dependencies,
replace_path or profile override is added by our descriptors. The carrier keeps
CreatineCore's physical generated defines outside TGC. The documented order
matches the accepted command-line sequence, but using these new display names
through another person's launcher has not been observed here.

CreatineCore 6.36 is a separate linked prerequisite. No upstream loader binary,
TGC distribution, game executable, original Lua library, private log/save or
Ghidra material is included. Eight game-derived UI textures are included in this
requested private mod handoff, following the requested ATCU/PRU form; they and
compiled binaries stay under ignored work and are not committed. The README
retains original rights/attributions and grants no new third-party licence.

The newest local logs confirm core 6.36, Volunteers and performance plugin
accepted, state/RNG probes disabled, all volunteer installation gates true and
140 executed ticks with an active mission. User feedback accepts general
operation/performance; this is not quantified speed acceptance, complete
lifecycle/capture attribution, recipient-wide vision or multiplayer acceptance.
Those limits are explicit in README. Multiplayer Send remains refused.

Two package tests pass: exact accepted payloads, prototype mode, manifest/CRC,
20-file inventory, no executable/scripts/installer, only our one DLL, eight
textures, additive descriptors, reproducible ZIP bytes and output-overwrite
refusal. Final archive was reread and matched against installed files. No game
launch, desktop input or installed-file changes were performed while packaging.

Owned packager: `tools/prepare_volunteers_share.py`. Checks:
`python tools/test_volunteers_share.py`. Sharing is authorized; public release
upload/ModDB publication was not requested. Remaining acceptance instructions:
`instructions-volunteers-share.md` and the existing prototype/loader/OOS cards.
