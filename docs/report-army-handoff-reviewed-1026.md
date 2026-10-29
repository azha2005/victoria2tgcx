# ARMY-HANDOFF revision 2: analysed layouts and clearer package

2026-10-26. User requested analysis of the existing classes and a clearer ZIP.
Both CArmy and CRegiment exist; no named structure data types are defined in
the Ghidra database. Native construction, vtable assignments and callers now
support expanded partial byte maps, not invented complete class definitions.

## Deliverable

`work/army-handoff-1026.zip`, also `work/army-handoff-1026-v2.zip`.
103 files, 132758 bytes.
SHA-256: `e2955f0769e0c7134fab2b7e939aa447b1062b5361524693f6bbe9aca447403d`.
Previous ZIP preserved as `work/army-handoff-1026-v1.zip` with original hash.
Curated source folder: `work/army-handoff-1026-v2` (ignored by Git).

Entry point README; separate layout, command and evidence guides; partial C++
byte views with static_assert checks; 40 indexed individual pseudo-C/instruction
pairs; vtables; imported build identity and function prefix evidence; selected
existing wrapper excerpts and both TPL-4 implementation/runtime reports;
self-contained verification and ZIP regeneration scripts. No executable,
Ghidra database, third-party assets or misleading full chronological symbol dump.

## Findings

- CArmy allocation 0x1B8, ESI constructor, separate identity at +8;
  CRegiment allocation 0x8C, shared CSubUnit construction then inline vtable store.
- Regiment +0x30 supporting pop, +0x38 unit type, +0x3C strength, +0x40
  organisation, +0x44 name string, +0x60/+0x64 owner tag/index, +0x74 parent.
- Country list +0x7B4/+0x7B8/+0x7BC is a unit list including other classes;
  guides explain filtering, node flags, registry conversion and pointer lifetime.
- Transfer validity resolves the regiment reference, not the destination army.
  Execute performs destination lookup and native eligibility checks.
- Tables contain 40 army primary, eight army identity and 17 regiment code
  entries before non-code. Earlier symbol-delimited inventory counts truncated.
- Shared true/false helper names are misleading; raw code establishes behavior.

## Validation and scope

Fresh Ghidra 12.1.4 read-only/noanalysis export on a workspace database copy.
Imported database SHA matches reference EXE; check_exe SHA/all signatures pass.
All 40 exports successful. All 40 16-byte function prefixes and 65 vtable entries
match the installed reference EXE. C++17 GCC -Wall -Wextra -Werror syntax check
passes all byte-view size/offset assertions. ZIP CRC and 103 payload comparisons,
102-entry SHA manifest verified. Main ZIP equals verified v2 byte for byte.

These are static analysis/packaging checks, not new runtime evidence or new
runtime signature guards. Unknown bytes, army display name handling, exact
organisation display scale, other builds and new MP coverage remain unresolved.
No feature implementation, install, game control, external send, release,
commit/push or subagents. Previous reports and unrelated dirty work preserved.
Next: no new implementation selected. Optional follow-up remains in
`docs/instructions-army-handoff.md`, now pointing at revision 2.
