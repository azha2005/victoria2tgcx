# Province Rename 0.2.2: white title on the native green header

2026-10-21. User accepts 0.2.1 short behavior and supplies a live screenshot
with ABC correctly beside Jijolandia, its map label updated and editor open.
The preserved log confirms the native queued rename/readback/header refresh
and editor reopening. This is short functional acceptance; persistence,
extended lifetime, other bases, ATCU coexistence and MP remain untested.

User asks for further polish, then specifies a **white title and no beige
title background**. 0.2.2 implements that focused refinement:

- Remove the parchment title plate, exposing genuine native green across
  the header inside its gold braid.
- Title uses native `vic_22`, white/cream like the province header, centred
  at x18/y4 in a296x27 box. Previously it used black vic_18 at y7.
- Field, captions, current name, status and buttons retain black fonts.
  The332x224 editor, native28x28 ABC, proven placement and rename command
  path remain unchanged. Runtime source changes only its version text.

Current board `docs/province-rename-layout.svg` updated in the same session;
previous board retained verbatim at `docs/archive/province-rename-layout-021.svg`.
Native bitmap-font preview: `work/province-rename/candidate/layout-preview-022-native.png`.
Versioned private SVG artwork under `work/province-rename/mockup-assets-0.2.2/`
keeps prior prepared pixels intact. No game artwork embedded into tracked
source, no remote artifact publication. The white-title request supersedes
the earlier all-black title specification and its parchment contrast plate.

Validation:282reference guards/59Rename guards, Win32 build,53native
checks,4package tests and4current-V2DLL loader probes pass. Existing colour
check now requires vic_22 for the title and black fonts for every other
added text/input control. Same five-base static font/sprite checks pass.
Visual preview inspected; 0.2.2 appearance still needs a player check.

Candidate: `work/province-rename/candidate/Province-Rename-Universal-0.2.2-test.zip`,
12files,255324bytes. SHA-256:
`f02656a991dcef5db7e9b5556d4ed8e40c12e19678117ee5d1481d55288294fa`.

User confirmed Closed normally now. No running game before/through update.
Installed in `work/Victoria-II-Province-Rename-TGC252`; all11payload and
25,388protected baseline hashes pass. Full0.2.1/guide backup:
`work/province-rename/polish-022-1021/before-022/`. Installed record,
accepted capture/log, native-source preview, build/tests and loader evidence
are retained beside it. Target guide updated; main E:, D: and other copies
untouched. No game launch/termination, new addresses, pitfall, commit,
push, tag or publication. Next pitfall stays P62.

Next: instructions-rename-022.md. Player selects Province Rename (Universal)
plus TGC v2.5.2 (stable), opens owned land and checks the white title on
green, no plate, black body text and existing rename/Cancel behavior.
Expected plugin0.2.2 Universal with59native signatures.
