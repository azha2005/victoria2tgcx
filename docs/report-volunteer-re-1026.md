# VOLUNTEER-RE: scoped native research handoff

2026-10-26. User requested five areas for a volunteer-brigade mechanic and
then asked to continue. Local research addendum prepared; no feature implemented.

Package: `work/volunteer-handoff-1026.zip`.
435 files; 1,083,291 bytes; SHA-256 `9a5ad32bbb066dbbf70ead4b9bef692ef0586cc4703a0d796e144657097fdcca`.
Guides/evidence source: `work/volunteer-handoff-1026/README.md`.
Army handoff v2 retained unchanged.

## Findings by requested priority

1. All eight control calls in six caller functions exported and inspected.
   FUN_005CC2E0 takes country refs, not an army. Global modification would
   authorize all recipient armies. CArmy slot 2 FUN_005CC4D0 is the smallest
   army-aware entry candidate; list aggregation and map/UI paths still matter.
   Native Move validity/execute do not directly call the subject-control helper;
   complete network sender authorization remains unestablished.
2. CCountry+0x67C is the at-war cache; bilateral relation+0x34 drives hostility.
   FUN_00517AF0 rebuilds opponent vector +0xDA8/+0xDAC/+0xDB0 (not vassals).
   Combat eligibility uses army owners and other state/class/transport gates;
   merely writing an owner index is not a complete implementation.
3. DayTick VA 0x006859C0/RVA 0x2859C0: pushed world, RET4; first five complete
   instructions/bytes documented as a trampoline candidate. Native command
   chain traced. OS thread, reentrancy and actual MP peer coverage not probed.
4. Army supply routine resolves army+0xC4 and consumes owner goods pool+0x1184.
   Final treasury debit not established. Reinforcement uses regiment+0x30;
   missing-pop fallback selects by reg+0x64. Native daily reconciliation can
   move/remove foreign-pop regiments, so a simple loan needs further handling.
5. Selected-object list at in-game context+0x1600; UI action row builder and
   fixed 17-pair table traced. Actual action slots differ from prompt's proposed
   map: Give execution slot32, availability36; slot1 persistent writer.
   Give action 0x28 layout and nested command/factory persistence traced.
   Complete custom registration API, all subclass layouts and network framing
   remain unestablished. No fabricated new action ID or complete recipe.

## Verification

Fresh Ghidra12.1.4 readOnly/noanalysis on workspace project copy; imported SHA
matches reference. 201 successful pseudo-C/assembly pairs; three failed address
probes explicitly recorded and not used as function evidence. One non-mutating
linear supplement recovered the 0x00644B70 RET0xC epilogue after a helper falsely
marked noreturn. All 201 function prefixes,176 vtable entries,17 action pairs
and both hook prefixes match the reference file. Full check_exe passes.
435 ZIP payloads/CRC and434 SHA manifest hashes verified. Raw keyword-factory
exports from an earlier exploratory pass are excluded from the package.

Exact TGC252 diplomacy GUI/localization files and a six-row Command Units
excerpt included locally with source hashes. No Army diplomacy override exists;
no screenshot match asserted. Third-party/EXE-derived material stays ignored.
No game launch/termination, install, hooks, MP test, external send/publication,
commit/push or subagents. Existing dirty work preserved.

## Remaining work and next session

The static research delivery is complete with explicit limits. The volunteer
mechanic is not ready to claim a one-hook or MP solution. No implementation
selected. `docs/instructions-volunteer-re.md` lists exact further research and
runtime probe order if selected. Do not infer full ABI from raw pseudo-C.
