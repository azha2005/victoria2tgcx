# NEXT - what comes next

2026-10-19 (55). **Universal0.10.4 short checks accepted on vanilla,
HPM, DoD and TGC2.5.2 (with V2DLL5.82). Culture buttons are next.**
Session54 preserved in docs/archive/sessions.md. User authorized this
source checkpoint commit/push; no public0.0.1 release/tag requested yet.

## User decisions / boundaries

- Next: primary+accepted and primary-only culture buttons for Templates
  and Conform (TPL-5). Politics/LAW-4 still wait.
- Then consider continent selection in the template builder (TPL-6).
  "Perhaps" is a proposal to assess; final design is not yet selected.
- Same files-only Universal package; no recipient installer/PS1/BAT.
- Conform recruits within land reach and reports shortage (D17).
- User says V2DLL-derived code use is no problem; author supports open
  source and has already been told of its use. Keep attribution. Do not
  treat earlier permission uncertainty as blocking this source checkpoint.
  Our own release license is still unselected; no invented upstream license.
- User alone launches/exits. Fresh normal-exit confirmation + process gate
  before game-copy writes. No new game changes needed for this checkpoint.
- Current Conform screenshot supersedes old mockup. Update visual references
  when adding controls; keep native styling and DoD compatibility.

## Evidence / accepted baseline

- report-universal-tgc-passed-1019.md: TGC user acceptance, V2DLL header,
  11 live land types, two missing builds queued and3/3extras detached.
 16 soldier pops excluded by land reach. New-build final joins absent in
  snapshot, so do not claim that log proves exhaustive completion.
- Logs/hash manifest: work/universal-tgc-0104-passed-1019/. Other accepted
  logs/user statement: work/universal-0104-confirmed-three-bases/.
  Vanilla latest log overwritten by launcher; acceptance is user-reported.
- Accepted ZIP work/Army-Templates-Conform-Universal-0.10.4.zip,
  SHA256 2e1d8cf974f2a7eea92e82067c0f505c891d0b517f8e7c9b60fb9696c029dff1.
  tgcx.dll/v2dll_mod.dll SHA256
  b53621609425f9c75135ba2fade947a6bf93197bcc14ec30182b105b9f7bba67.
- 172 signatures/native/reference GUI/actual D3D9 checks pass. Python
  discovery97 passed,2 probe/build-race errors resolved by complete12-test
  installer/Lua/plugin rerun. Do not rebuild probe concurrently with tests.
- Universal gets units from live engine definitions, template TXT cached
  in memory; capacity/routes refresh for preview/actions. Tracking remains
  session-only; after reload Conform builds anew, never gathers idle troops.
- All four short checks passed; extended straits/moving-target/selection/
  Navy/library cases, olderTGC2.3.4/HFM and MP are not fully validated.

## Next session in order

1. **TPL-5 culture buttons**, docs/instructions-tpl-5.md: verify native
   culture fields/command pop, update visuals, implement shared filtering
   and backwards compatible per-template setting, test/stage/game-check.
2. **TPL-6 continent proposal**, docs/instructions-tpl-6.md: after culture,
   inspect dynamic continent data and propose a builder selector/default/
   persistence behavior. Keep continent distinct from Conform land reach.
3. Preserve accepted0.10.4 rollback. No repeated four-base short test until
   changed behavior warrants it. Extended regression instructions remain
   docs/instructions-mod-6.md. Never launch or close the game for the user.
4. Public0.0.1 preparation when selected: docs/instructions-release-0.md.
   Credits/license text, bundled-art provenance, recipient install and
   release documentation remain. Source push is authorized this session;
   no release asset upload/tag/publication/external message authorized.
5. Record actual scope/results; close under ROADMAP7 with instructions.

## Cards

| Card | Status | Instructions / evidence |
|---|---|---|
| TPL-5 | selected next; not implemented | instructions-tpl-5.md |
| TPL-6 | prospective continent-selector design after TPL-5 | instructions-tpl-6.md |
| MOD-6 | four-base short checks passed including TGC/V2DLL; extended pending | instructions-mod-6.md; report-universal-tgc-passed-1019.md |
| MOD-3 / MOD-4 / MOD-5 | vanilla/HPM/DoD/TGC short accepted; extended pending | instructions-mod-6.md |
| RELEASE-0 | readiness assessed; public preparation not selected | instructions-release-0.md |
| TPL-3 / TPL-4 | Ctrl/Alt/detach accepted | report-tpl-4-runtime-1019.md |
| LAW-4 / U-12 Bill | user-deferred | respective instructions |
| V2D-082 | accepted; Universal route now also accepted | report-universal-tgc-passed-1019.md |
| V2D-2 | author-side enhancement D16, separate from working loader route | report-v2dll-mod-slot-1018.md |
| MOD-1 / MOD-2 / LAW-3 / U-11 | accepted baselines; G1/G3 deferred | existing reports |
| TPL-0 / TPL-1 / U-8 | accepted | existing reports |
| TPL-2 | on hold; Conform covers it | instructions-tpl-2.md |
| GOV-0 / Economy / COMPAT-0 | not selected / Politics compatibility deferred | technical-plan.md; instructions-compat-0.md |

Next pitfall P59. Third-party art, binaries and decompiled output stay ignored.
