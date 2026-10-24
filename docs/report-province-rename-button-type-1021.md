# Province Rename 0.1.3: measured button type and missing entry

2026-10-21. User screenshot shows the France-owned Evreux province panel,
with Normandie/Evreux header and no Rename entry. **0.1.3 is now installed;
its in-game retry is pending.** Supersedes the0.1.2 candidate status in
report-province-rename-preflight-1021.md. Prior reports remain historical.

## Diagnosis from the running game

0.1.2's log records all42guards and hook installation, then refuses
tgcx_rename_entry/tgcx_rename_open as not CEU3ButtonType. The failure is
before construction, so this screenshot is not a placement/foreground
failure. The ownership snapshot already passed.

Read-only inspection of the running test process PID47976, without
injecting code or writing game memory, found the actual CProvinceView,
selected province438, root and GUI context. The view was above the2GB
address boundary; inspection covered the32-bit address space. The native
registry's ternary lookup was inspected and reproduced in the reader.
Both root definitions resolve correctly. All three buttons resolve to
table **RVA0xA3CFBC**, whose slot11 is the native button factory
**RVA0x5F0310**. The0.1.2 guard instead required **RVA0xA08D2C**, the related
CEU3ButtonType table. This was an incorrect concrete-type assumption.

The live root/button registries use tableRVA0xA41E84. Native string wrapper
FUN_00641BA0/RVA0x241BA0 dispatches C-string lookup FUN_009E6B10/RVA0x5E6B10,
which traverses nodes through FUN_009E6C10/RVA0x5E6C10. Native standard
button factory FUN_009F0310 is statically verified as constructing a button;
it is not the destructive wrong-type thunk involved in0.1.0.

Evidence is retained in work/province-rename/missing-012-1021/: screenshot,
original log, live.json, resolved-live.json, modules.json and bounded native
exports. No process memory was modified for inspection. No arbitrary calls
were executed inside the game. The root/button addresses are observations
of this run, not fixed addresses used in the plugin.

## Correction and independent coverage

0.1.3 requires the measured standard-button table and its factory slot,
preserving collision refusal. Failure diagnostics now include the actual
resolved object/table addresses. The distinct root-definition/child-button
registry scopes remain as corrected in0.1.2. Generated GUI/layout unchanged.

The native regression now uses the measured standard-button table for
actual native builder dispatch. Additional fixtures execute the engine's
actual string wrapper, C-string lookup and ternary traversal with separate
window/entry/body registries. Only CRT character conversion is substituted.
They cover valid standard buttons, case-insensitive names, a wrong-type
Apply definition and a modified destructive factory pointer. This removes
the earlier lookup stub as the only evidence for schema acceptance.

P60 records the mistaken type assumption and the fixture limitation. The
ATCU lessons about native construction, concrete classes and independent
semantic evidence apply here. No ATCU feature source changed.

## Validation and installation

- Reference executable and **272 guards pass**: unchanged179Army runtime,
 44prior research and49Rename guards. Win32 Release build passes.
- **42 native checks**, **4 package tests**, **4 current-V2DLL loader
 probes** pass. The same unchanged TGC source retains its passed scan.
- Candidate: work/province-rename/candidate/Province-Rename-0.1.3-test.zip.
 SHA256774d6fbb2dc446efd431a380005d6ac8ff6db9724cc901bf35bfb30859f2c89a.

User confirmed normal exit, but the windowless process remained alive
with73threads. Installation correctly refused; no game payload was changed.
After reporting that it was stuck, the user explicitly said **"close it
forcefully"**. The agent verified PID47976 still belonged to the exact test
copy, force-closed only that process and verified no Victoria II process
remained. This is a one-time explicit exception, not authorization to launch
or terminate future games automatically.

Installed0.1.3 in work/Victoria-II-Province-Rename-TGC252: plugin/version
marker and test guide updated; all10package hashes match. All25,388protected
files match the original baseline before/after the patch. Evidence:
missing-012-1021/patch-013-installed.json; rollback before-013/. The source
copy, main E:, other test copies, base data, V2DLL and stock Lua unchanged.

Next: instructions-rename-013.md. Full GUI construction/display/input,
applied command behavior, save/reload and extended cleanup still need
in-game evidence. No game launch, commit/push or publication performed.
