# Province Rename 0.1.2: final preflight scope audit

2026-10-21. **0.1.2 is the current installed candidate. Player retry remains
pending.** Supersedes the interim 0.1.1 status in report-province-rename-crash-1020.md;
the original failure evidence and collision correction described there remain valid.

## Additional finding before handoff

The final audit caught an error in the new 0.1.1 defensive preflight. It used
CEU3Gui+0x34, the registry for root window definitions, to look up child
buttons. That could refuse valid GUI definitions. The initial schema fixture
did not distinguish those registries. No player run of 0.1.1 was reported.

Native FUN_009CFF20 uses ESI=owning CWindowType definition and searches its
button registry at+0x150. The native window constructor stores that definition
at window+0x94; FUN_009CDE70 passes it to FUN_009E16C0, and the latter reloads
it before calling the builder. Read-only evidence is in the first crash's
native-stack.txt and dispatch-asm.txt. These offsets refer to the definition,
not the runtime window or CEU3Gui object.

0.1.2 first resolves tgcx_rename_entry/window through CEU3Gui+0x34, then
resolves each button through that definition's+0x150 registry, matching
the native construction path. The runtime CEU3ButtonType check remains.
An added signature guards the actual native registry lookup instructions.
The fixture now supplies distinct window and button registries and rejects
global button lookup, while retaining actual native builder/thunk execution.

The unique transparent labels, native allocation-size evidence, aligned
observer storage, click-time root check, deferred cleanup and diagnostics
from 0.1.1 remain. Layout and mechanics are unchanged; the original local
SVG specification still applies. No ATCU implementation changed.

## Checked candidate and installation

- Win32 Release build with warnings as errors passes; **265 executable
  guards**, **38 native fixture checks**, **4 package tests**, **4 loader
  probes** pass. TGC scan retained from the same unchanged base.
- ZIP work/province-rename/candidate/Province-Rename-0.1.2-test.zip:
  11 files, 179059 bytes; SHA-256
  101bd8f220a4ba51f6cce3153cea247fe9d55d0531146030a2da46303956ded1.
- Plugin SHA-256:
  8fe892c488412931bb96bf8335ed85454ef9e7b8b50c6cb4c50878daba7da247.

Within the same user-confirmed closed-game update, another ToolHelp process
gate passed. The plugin/version marker/test guide were updated from0.1.1
to0.1.2 in work/Victoria-II-Province-Rename-TGC252. The corrected GUI already
installed by 0.1.1 was retained. All 10 package files match 0.1.2; all 25,388
protected files still match the original staging baseline. No game launch
or exit was performed. The original crash and both interim versions remain
preserved outside the game. Evidence: crash-0100-1020/patch-012-installed.json,
before-012/, native-012-tests.txt, correction-012-build.txt and
package-and-loader-012-tests.txt. Main E:, other copies and ATCU unchanged.

## Next gate

Follow instructions-rename-012.md: select an owned land province, open,
Cancel, reopen, change selection and close/reopen the host. Then perform
the full input/Apply/map/header/save checks in instructions-rename-u.md.
Native fixture coverage does not establish full engine GUI integration,
visual placement, text focus, sustained collection, persistence or MP.
Do not claim the crash fixed in game before the player retry succeeds.
