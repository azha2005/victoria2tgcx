# RENAME-020: install and check the Universal native UI

**Superseded placement, 2026-10-21:**0.2.0 loaded but hid ABC due to wrong
geometry assumptions.0.2.1 correction is installed. Start instructions-rename-021.md
and use0.2.1/59 for log checks. Sections2-3 here retain the full behavior checklist.

**Installation update, 2026-10-21:** 0.2.0 r3 is installed and verified in
the dedicated copy. User explicitly instructed installation after the
pending exit gate was explained; no game process was present. Full backup
retained,11payload/25,388protected hashes pass. No literal normal-exit
confirmation is asserted. Report-province-rename-020-installed-1021.md.
**Start at section2. Do not rerun section1 against installed0.2.0.** The
pre-install description and section1 below are retained historical procedure.

2026-10-21. 0.1.3 short rename behavior is accepted. The final ABC/native-art
0.2.0 r3 candidate passes automatic checks but is not installed or checked
in game. Read NEXT.md and report-province-rename-universal-ui-1021.md.

## 1. Finish the dedicated-copy installation

1. The fresh normal-exit question is pending. Obtain the user's answer;
   absence of a process alone does not satisfy NEXT.md's installation gate.
   Do not launch or terminate the game. The old one-time force-close
   authorization for PID 47976 does not apply to any future process.
2. Target only `work/Victoria-II-Province-Rename-TGC252`. Preserve main E:,
   D: and other test copies. Run the default read-only preflight:
   `python work/province-rename/ui-universal-020-1021/update_020.py`.
3. After fresh confirmation and a clear process gate, run the same script
   with `--apply-confirmed-normal-exit`. It verifies the exact r3 package,
   old 0.1.3 payload/guide, reference executable and 25,388 protected hashes.
   It creates `before-020/` once, backs up all replaced files, installs 11
   payload files and updates only the test guide. Unknown conflicts refuse.
4. Check `patch-020-installed.json` and all payload/protected hashes. A
   partial failure needs inspection against the retained backup before
   launching. Do not blindly rerun an interrupted update or discard backup.
5. Record installation in a dated addendum report and update NEXT.md/status
   docs. The current report records the pre-install state and is historical.

## 2. Native UI player check

1. User opens `victoria2.exe` in that copy and selects **Province Rename
   (Universal)** plus **TGC v2.5.2 (stable)**. Profile remains
   `TGC252_ProvinceRename_20261020`; current V2DLL stays in place.
2. Logs/tgcx_province_rename.log should show **0.2.0 Universal**, V2DLL API1,
   **56 native signatures OK**, both native roots and **Universal Rename
   ABC icon** with `measured font width`. V2DLL logs are in V2DLL/Logs/.
3. Select owned land while paused. Expect one native **28x28 ABC button**
   immediately right of the province name, no extra Rename label. The
   actual province name must remain visible. Check short, long, narrow and
   accented names; note any right-edge clamp or unsupported-font fallback.
4. Click ABC. Compare to docs/province-rename-layout.svg and
   work/province-rename/candidate/layout-preview-020-native.png: 332x224
   editor, native gold braid/green/parchment/buttons and black text/input.
   The title's parchment plate is deliberate for black-text contrast.
   Preview pixels do not establish live alignment or hit testing.
5. Check both buttons and the field can be clicked; test Cancel/reopen,
   another selected province and host close/reopen. Native state/province
   controls must remain usable. Where practical, test another resolution
   and the editor's narrow-viewport fallback without hiding controls.

## 3. Full behavior and persistence

1. Rename an owned province, check header/map label/tooltip and the queued
   and observed-applied log entries. Record paused versus unpaused behavior.
2. Test `San Martín de los Andes`, a long valid name and the 80-byte limit.
   Invalid empty, over-limit and control/format input must be refused.
3. Cancel an edit; change selection, ownership/player or close the province
   panel with an editor open. Stale input must not rename a different target.
   Foreign land and sea provinces must not offer a usable rename action.
4. Save under a new test name, exit normally and reload that save. Check
   all changed province names and map labels, then load a different save
   to check stale GUI/state cleanup. Do not assume the short 0.1.3 success
   already tested persistence or extended lifetime.
5. Preserve fresh logs and unedited captures in a new dated work/ folder.
   If anything fails, identify the failing phase before changing code and
   consult the ATCU pitfall audit/P59/P60. A successful fixture is not live
   GUI acceptance. New installation again requires fresh exit confirmation.

## 4. Universal and multiplayer follow-ups

The same package statically resolves anchors/fonts/sprites on vanilla,
TGC 2.3.4/2.5.2, HPM and DoD; no other-base gameplay has been performed.
Do not install into more copies automatically. Once the user selects a
base check, stage/reuse an appropriate isolated copy and follow the same
installation and player sequence. Test ATCU coexistence separately with a
V2DLL loader that can load several selected plugins.

MP is theoretically supported by native command 0x707 but untested. If the
user selects an actual MP test, use matching builds/settings on both peers;
rename from both host/client, inspect names/header after reselection on
both, advance at least 30 days while watching OOS, and save/reload. No MP
acceptance or game-launch authorization follows from the theoretical query.
