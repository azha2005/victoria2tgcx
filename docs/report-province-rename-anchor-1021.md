# Province Rename 0.2.0 missing ABC: live geometry correction in 0.2.1

2026-10-21. User reports no ABC entry after 0.2.0 installation. The log
confirms the right plugin, 56 guards, both constructed roots, then
`Rename native province-name anchor unavailable; placement refused`.
**0.2.1 is now installed and checked; player retry is next.** This report
supersedes 0.2.0's placement assumptions and earlier pending-install status.

## Pitfall review and read-only evidence

Reviewed P37/P40 (owner-relative matrices), P44 (direct named children),
P53 (base-specific anchors), P54/P56/P57 (native lifetime/foreground/subtree),
P58 (fixtures repeating assumptions), and P59/P60 (registry/concrete types).
The failure is before entry activation, not a missing DDS or failed loader.
The live sample also exposes exactly the fixture-assumption problem in P58.

User reopened the dedicated copy, selected owned land and left it open for
read-only inspection. Verified PID 16424's executable and plugin paths;
reference game base 0x005C0000. Selected province 402, Amiens; viewport
2560x1440. No injected calls, memory writes, patching or game control during
inspection. Private evidence: `work/province-rename/missing-020-1021/`.

- Native header and province_name lookups are valid. Their native runtime
  arrays and text font resolve correctly. Font width/height methods match.
- Host origin is (0,841). Runtime root+0x94 is the correct CWindowType
  definition, but definition+0xA0 is **X=0**, not width. Definition+0xA4
  is Y=-599; +0xA8/+0xAC are width380/height525. Reading the earlier
  constructor's ESI-relative width word as this object's +0xA0 was wrong.
- The actual CFixedWindow constructor stores runtime width/height at
  root+0x4A0/+0x4A4; those are380/525 in the live window. 0.2.1 uses runtime
  width directly, avoiding the mistaken definition interpretation.
- Text inner matrix is identity, with the expected owner chain through the
  header to the host. Local name position is separately stored as17,45 at
  text inner+0x188/+0x18C. Native Draw uses those offsets for the font call.
  0.2.0 would have omitted them even after correcting its width gate.

Thus the measured text origin is (17,886), not (0,841). 0.2.1 combines the
owner chain, actual text-local origin and native proportional font metrics.
The existing 28x28 ABC gap/clamp, modal geometry and native artwork are
unchanged. Placement refusal now reports its precise stage/root/runtime
width. No guard is weakened to accept an arbitrary text class.

## Independent native validation

Original 0.2.0 fixture wrote a guessed width into definition+0xA0 and put
text coordinates directly into the matrix. Both agreed with the incorrect
implementation, so passing it did not establish the real field layout.

Corrected fixture preserves the live distinction: definition X=0, runtime
width480/380, identity text matrix and separate local offsets. It retains
translated header owners, mod-specific coordinates, font fallback and
viewport limits. The live TGC placement case with synthetic text width32
places ABC at (57,881) and editor at (400,841); the actual name's native
font width determines its own X in game.

A new fixture executes **actual native Draw**, intercepting only its three
font/GPU boundary calls. It observes Draw passing local17,45 to font drawing
despite the identity matrix. Actual native font width/height still execute
in the separate metrics fixture. This supplies semantic evidence beyond
another copy of the implementation's offset assumptions.

- Reference SHA-256 and **282 total guards** pass: **59 Rename**, unchanged
  Army179 and prior research44. Removed the misleading definition-width
  guard; four guards cover actual size stores, both text offsets and Draw
  slot10. New/changed addresses are documented in symbols.md.
- Win32 Release build and **53 native checks** pass, zero failures.
- **4 package tests** and **4 current-V2DLL loader probes** pass. Same five
  static base profiles and fonts; native ABC pixels and GUI unchanged.
- No new TGC script or data dependency; its prior passed scan still applies.
  No other-base gameplay, MP or 0.2.1 live appearance acceptance is claimed.

Candidate: `work/province-rename/candidate/Province-Rename-Universal-0.2.1-test.zip`.
12 files,250713 bytes; SHA-256
`d3f9ba6881a21780149cdc91a2b9a0a8f1cd2de42b6ce7b556f01b9f8ebdca47`.
Native exports, live dumps, failed log, final build/check records and loader
probe remain private in the evidence folder. New pitfall **P61** records the
field/fixture correction; next pitfall P62. Earlier reports are historical.

## Installation and next player check

User confirmed **Closed normally now** after the diagnostic session. No
v2game.exe was running before/through update. Exact previous 0.2.0 payload
and guide were verified and backed up as `missing-020-1021/before-021/`.
All **11 candidate payload hashes** and **25,388 protected baseline hashes**
pass after installation. Record: `patch-021-installed.json` in that folder's
parent. Only plugin/version marker and the test guide change from 0.2.0;
all native art, GUI and localisation bytes remain identical.

Target remains `work/Victoria-II-Province-Rename-TGC252`, current V2DLL and
independent TGC252_ProvinceRename_20261020 profile. Main E:, D: and other
copies untouched. No launch, termination, commit, push or publication.

User selects Province Rename (Universal) plus TGC v2.5.2 (stable), then owned
land. Expect **0.2.1 Universal**, **59 native signatures OK** and **Universal
Rename ABC icon** with placement pixels. Follow instructions-rename-021.md.
Full persistence/lifetime and MP remain separate from this entry retry.
