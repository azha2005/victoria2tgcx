# ATCU 0.0.1 installed in the main game

2026-10-19. User requests ATCU in the main installation, clarifies0.0.1,
then confirms normal exit. Installed after the process gate. No game
launched or terminated; no feature implementation or release changes.

## Target and source

Target: `E:/Victoria.II.v3.04.Inclu.ALL.DLC`.
Source: `work/releases/final/ATCU-0.0.1.zip`, SHA-256
`9ee654e7b33fbe62444892a55388e24ecfc7246ace2fdef9bf1007585e4f799e`.
This is distribution0.0.1 with accepted internal build0.10.4. Culture and
continent filters are not included.

Added only `mod/tgcx_army_universal/` and `mod/tgcx_army_universal.mod`:
37/37 payload hashes verified. The package was absent before installation.
DLL SHA-256:
`b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67`.

## Verification and rollback

- Supported executable hash and all222runtime/research byte checks pass.
- Existing MOD-2 loader's128 Lua forward exports and original Lua verified.
  Main loader SHA-256 remains
  `3aab0c2133b516521f2ea531d9b59e755c4797bd53c657d3d011d32b220503ba`.
  It differs from the ZIP loader but passed an isolated probe loading the
  actual ATCU payload through the renamed descriptor. No replacement needed.
- Original Lua SHA-256 remains
  `b2ba19343691f22d1af1417f9a7cbaeae74ba820a2a412f5a6cdea271cc1a7b0`.
- Protected executable/loader/Lua/install record, old Army and Politics
  files/descriptors, TGC descriptor/map anchors unchanged after installation.
  No writes to base mods, settings profiles or template preferences.
- Backups and preserved previous log:
  `work/main-atcu-001-1019/before/` and `previous-logs/`.
  Plan, complete protected/payload hashes and installed confirmation:
  `work/main-atcu-001-1019/{plan,installed}.json`.
- Installation used exclusive file creation with rollback limited to paths
  created by this transaction; no original files were overwritten.

## Launcher and user check

Open the normal launcher from the main E: game. Tick **ATCU - Army
Template and Conform Universal** and the chosen base, for example
**TGC v2.5.2 (stable)**. Untick the older Army and TGCX Politics entries.
The old Politics package carries an older logic DLL and can take
precedence through our loader's reverse mod-order search; it is retained
as rollback, not an ATCU0.0.1 coexistence claim. ATCU alone works for vanilla.

Expected log: MOD-6 0.10.4 loaded_by=TGCX loader MOD-2 0.8.0. Test
Templates and Conform during ordinary use; preserve Logs/tgcx.log after
normal exit if anything fails. Main-copy gameplay is not yet confirmed;
the same payload has already passed the four-base short matrix elsewhere.

Rollback: after normal exit, untick ATCU and reselect the prior Army /
Politics combination. Their original files and loader are still present.
There is no need to reinstall or restore the game. Research implementation
continues with TPL-5; no commit/push/release performed this session.
