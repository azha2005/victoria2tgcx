# TPL-1 visual candidate 0.6.3 - 2026-10-17

Status: implemented and staged, not installed, not tested in game. The
accepted installed baseline remains 0.6.2. No commit or push.

## Lifetime repair

The baseline log is preserved at `E:\v2re\logs\tgcx-tpl1-1017\baseline-before-063.log`
and `work/tpl-063/baseline-062.log`. Faults appear at lines 314 and 553;
the latter follows a new native window on view B8C62698, root BBBC3B70.
That view address appears with different roots earlier in the run.
The old log cannot establish the exact faulting operation.

The root-change branch used to hide the cached previous window before
resetting it. It now discards that pointer without invoking a potentially
destroyed child. Stack identity reads are separately guarded and must round
trip through the registry before ownership or virtual land tests. This check
runs each tick, including when the raw stack pointer has not changed, and
on Open clicks. Disabled panels can recover on root or selection changes;
clicks on a disabled panel are rejected. Tick runs on the captured GUI thread.
The last-resort SEH guard now reports the failing named stage and pointers.
Synthetic tests cover registry removal, freed memory and root replacement
with an invalid previous child. They pass. These are demonstrated fixes to
unsafe paths, not proof that either caused both historical faults. Runtime
reproduction remains required before calling the original fault resolved.

## Visual reference and deviations (R10)

The user supplied an army-composition screenshot during this session and
asked for that look. Preserved privately in `work/tpl-063/user-army-reference.png`.
Board 6 was opened and compared; the mechanic is unchanged, so no mechanic
mockup republish is needed. The user's screenshot supplies the styling target.

| Element | Candidate and difference from board 6 |
|---|---|
| Frame/background | Native army parchment and gold border from GFX_unit_bottom_bg; green header from the first of two GFX_unit_toppanel_bg frames. Replaces the flat board background at the user's request. |
| Title/close | Left-aligned Arial14 and native 20x20 GFX_unit_close, like the army panel; shorter generic title because army names are not read by the verified interface. |
| Template/change rows | Native GFX_unit_entry_bg in both lists, unit-strip icons and selection border. Pale army rows replace the board's blue template rows. |
| Size | 400x624, narrower than 460x624 baseline; still allows eight change rows and four templates. Empty space reserves the remaining rows. |
| Actions | Retain readable native 177x34 text buttons for Conform/Cancel, rather than ambiguous tiny army action icons. |
| Open button | Existing 20x20 infantry button in the stack panel free cell; the board's wide button cannot fit the native stack header. |
| Content | Composition names per D12, generic build/detach locations, four templates per page. Existing mechanic limitations retained. |

`work/tpl-063/army-preview.png` is a composited asset/layout preview with
example data and approximate Windows fonts; it is not an engine screenshot
and omits the miniature composition icons. Real fonts, clipping, layering,
hover and placement require the player screenshot beside board 6/reference.

## Gates and artifacts

- check_exe ALL OK; 144 runtime signatures unchanged.
- TGC v2.5.2 scan: anchors and event range OK.
- Release Win32 build, full native/reference-window/real D3D9 suite ALL OK.
  Sandbox D3D9 creation initially failed with 8876086A; the unmodified suite
  passed outside the sandbox, including the final rebuilt candidate.
- 18 GUI tests passed; five Conform tests passed again after army styling.
- LAW-0 scripted scenario ALL OK; law catalogue, ten submod installer and
  eight DLL installer/Lua tests passed.
- Both final installation dry runs passed. No live files changed.
- Stage preserves stage-062's law scripts, modifiers and other assets.
  Generated buildunit.gui and country_politics.gui hashes are unchanged.

Stage: `E:\v2re\logs\tgcx-tpl1-1017\stage-063`.
DLL retained beside it as `lua51-063.dll`.
DLL SHA256: `71cd19d540bd036674805ee3f0d702640bdab52dc6932ce72f84d9cbd968065e`.
Generated unitpanel.gui SHA256: `5b826305a4aa5c6b1a424af77a6bc7f50ed00e45bb7821cfb4c43e3f26ecf265`.

Next: `instructions-tpl-1-063-check.md`. Do not mark runtime or visual
acceptance complete until the player tests. No new numbered pitfall is
assigned while the original fault remains unlocalized; next remains P46.

## Installation - 2026-10-17

User authorized installation and confirmed exit. Earlier attempts stopped
without changes because v2game.exe PID 37236 was still running. After the
final confirmation, both process/dry-run gates passed. Installed 0.6.3 on E:.
Paired 0.6.2 backups retained at
`E:\v2re\logs\tgcx-tpl-place-1017\before-063-dll` (DLL and install record,
copy hashes verified) and `before-063-submod` (installer rollback source).
Installed DLL and all three GUI hashes match the candidate values; original
Lua hash matches the installation record. No game launch, commit or push.
Player visual and lifetime/Conform checks remain pending.

## Player acceptance - 2026-10-17

User: "works. commit and push" after installing 0.6.3. The army-style
Conform candidate is accepted as working in game. Current tgcx.log identifies
0.6.3 and contains no "guarded GUI read fault" entry. This short acceptance
does not establish exhaustive view-turnover coverage or a new side-by-side
engine screenshot; those evidence limits remain explicit. Commit and push
are authorized. No additional mechanic selected or started.
