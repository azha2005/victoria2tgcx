# NEXT - what comes next

2026-10-30. User requires coexistence with other Core plugins.0.0.17 is packaged,
not installed. User retains desktop/game control. No live-file or loader changes.

## Current verified state

- Exact game EXE gate and114 scoped signatures remain; all byte contracts outside
  native server Post are unchanged. MP permits/roster/source/epoch checks retained.
- Host-role compatibility now recognizes bounded, transparent instruction chains,
  independently of loader hash/version/private RVAs. Supports direct JMP veneers
  and up to four preserved role probes ending at exact original stolen bytes and
  a jump to the original continuation. Writes only recognized private DLL flag;
  executable/writable mapping checks, loops and traversal bounds enforced.
- Recognition is read-only. Other plugins stay loaded; no hook is overwritten,
  bypassed, disabled or executed for verification. Unknown contracts still refuse.
- Startup diagnostics identify owner DLL/path/RVA/hash and refusal stage. Local
  native-hook failure is shown before handshake waiting; no generic false wait.
-76 Rust tests pass,2 ignored benchmarks; fmt/strict Clippy/x86 release pass.
  New fixtures cover ASLR/link layout, veneers, stacked probes, mutated code/tail,
  loops/bounds/protections and wrong flag targets. Real x86 image/page fixture
  recognizes compiled probe without executing it. Six stage/two MP ZIP checks pass.
- Latest provided log:0.0.16 rejects patched RVA662EA0, gameplay/transport=false,
  scoped_hooks=false. No HELLO/Send executed. UI's waiting-for-peers was misleading.
  Two player/world streams are interleaved; this isn't paired OOS evidence.
- Official latest release checked2026-10-30 via GitHub API:6.36, archive digest
  f3d81c5bb03081a8b75ca9e0ca9537a6850eeb512e695188db9bc78d26cfc263.
  Screenshot says6.36 but doesn't prove same file or identify the rejected owner.
- No full compatibility acceptance for yearly cache/Baltic/performance/Volunteers
  combination. Do not ask users to disable plugins as the permanent solution.

ZIP: C:\Users\JC\Downloads\victoria2template\work\volunteers-mp-0017\final\Volunteers-0.0.17-MP-coexistence.zip
SHA256: 2010910f6ec5a6fde16bd59e4fa79c5a36f48445e5bc024362dbc029777b00a5
499732 bytes,19 files,mode=mp,no scripts/installers/EXEs.
DLL SHA256: 92bc4a34f307fb58252ab83530fdaad7e555ce1c968cbed5c8a3229d51074900

## Remaining cards

| Card | Status | Exact next instructions |
|---|---|---|
| VOL-COEXIST |0.0.17 source/ZIP checked; actual plugin combination pending | docs/instructions-volunteers-coexistence.md steps1..8 |
| VOL-MP / VOL-OOS | Original actual paired gameplay/OOS acceptance open | Coexistence steps6..10; docs/instructions-volunteers-mp-gameplay.md |
| VOL-ENTRY / VOL-FEEDBACK / VOL-CONTROLS / VOL-SIEGE / VOL-OPT | Full lifecycle/performance acceptance open | Coexistence steps6..9; MP-gameplay steps5..10 |

Installed copies remain E:/Victoria-II-Volunteers-0004 at0.0.12, workspace
work/volunteers-mp-0016/game-host at0.0.16. Both Core6.36. User tests them.
Preserve unrelated release-document edits, private binaries/art and raw RE.
Commit/push authorized. No new subagents, messages, public binaries or memory
writes. Next pitfall P90; read P89 and report-volunteers-coexistence-1030.md.
