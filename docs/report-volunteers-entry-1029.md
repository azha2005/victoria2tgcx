# Volunteers 0.0.15: recover the native diplomacy entry

2026-10-29. User supplies Maxioten's screenshot with the Send volunteers entry
missing again. The surrounding conversation mentions leaving work in the
background; it does not prove a particular native event caused the disappearance.
No friend log/save, direct runtime reproduction, game launch or desktop input
was supplied/performed. This is a source-backed correction awaiting game retest.

## Concrete failure paths and fixes

- hidden() set Ui.disabled=true when invoked off the captured UI thread. That
  disabled the UI permanently while the view/root stayed the same. A pending
  atomic hide now records worker/reentrant events; only the captured UI thread
  hides native objects. The next accepted update can restore the entry. Actual
  native faults, type/signature refusals and capacity fences remain hard failures.
- position() promoted only hidden roots. Native diplomacy could move above an
  already-visible independent entry, leaving it covered forever. A bounded,
  read-only renderer-link check detects that specific ordering and invokes the
  existing native foreground method. Regular visible polls preserve later visible
  dialogs. Recognized hidden window renderers do not prevent eventual repair;
  unknown nodes, torn links, owner mismatch and cycles never authorize promotion.
- Window+0x67 alone does not prove its button child is active (P42/P57). With
  exact CEU3Button Show/Hide slots checked, child+0x4F=0 triggers native subtree
  Show even when the entry root still reports visible. It restores children/text
  without setting simulation flags or showing the whole subtree every frame.
  Native Show's +0x6C gate is respected, avoiding repeated refused calls/log flushes.
- An expired owned-root lease previously left a nonzero pointer that was never
  rebuilt. Unknown/mismatched or pending-release (+0x52) roots are forgotten on
  the UI thread and recreated through the normal factory. Old observer boxes
  remain retained for deferred collection; no stale root is deleted or reused by
  hand. The existing bounded observer capacity still applies. Panel leases use
  the same check, with their show/placement state reset for reconstruction.
- Checkpoint preparation/post failures used the same fail() as unsafe UI types,
  hiding the entry. They now stop mission work for that explicit load epoch,
  suspend/unpublish scope and keep the independently validated UI available.
  Send/Recall/history writes and subsequent checkpoints remain refused. The
  panel explains that reload is required; no partial queue is retried. Only a
  new recognized load epoch can clear this command fence.

Host hide/recovery invalidates the action-grid position cache. Current native
anchors are read on reopen. Native button art, label, dimensions and placement
are unchanged. No entry is hidden merely because sending is unavailable.
There is no MP gameplay unlock or OOS fix in this UI correction.

## Native evidence and validation

The exact reference EXE gate passed before address work. Existing renderer
links+0xD4/+0xD8/+0xE4, window renderer+0x494, renderer scene+0x6B9E0 and native
window-renderer visibility+0x124 follow the existing real-engine GUI fixtures
(P35/P37/P56/P57). The new planner writes no links; it calls native promotion.

Fresh read-only raw exports under work/volunteers-entry-0015/research confirm:
FUN_009E3D60/RVA0x5E3D60 sets window+0x52=1 at VA0x009E3D72;
FUN_009EFEF0/RVA0x5EFEF0 sets button+0x4F=1 at VA0x009EFEFF;
FUN_009EFEB0/RVA0x5EFEB0 is the paired Hide implementation. These are the
established native no-stack-argument ECX=this, plain RET methods. The existing
window subtree Show is RVA0x5E4490 and foreground is RVA0x5E2FC0. All calls retain
expected-slot/signature, UI-thread, lifetime and existing CPU/SEH safeguards.

-61 Rust tests pass, two explicit timing benchmarks ignored. New fixtures cover
  covered/normal/later-dialog/closed-dialog ordering, bad links/cycles/renderers,
  and execute the actual hidden callback on a worker then the original UI thread.
  The latter also checks checkpoint failure retains UI while command capability
  is removed. No actual game GUI is invoked by that empty-root fixture.
- Formatting, strict all-target Clippy, x86 release build and107 scoped native
  signature records pass. Six package/loader, two SP-share, one MP-package and
  two collector fixtures pass. Native art/GUI/localization match0.0.13 in11 files.
- Plain SP ZIP: work/volunteers-entry-0015/final-r2/Volunteers-0.0.15-SP-beta.zip.
  463228 bytes,20 files, no EXE/BAT/CMD/PowerShell/scripts/installer.
  SHA256:c3df063e8bf71500cc1e1bb265c69ce615c880259a0100af7229fc53ed311380.
- DLL SHA256:974a306baf282a846bb8ff8d1648adf0223ae5030ff19c1ef1236647cb0dd4ef.
- Installed E: copy remains0.0.12/CreatineCore6.36, DLL08de457c...a00a1a3.
  No game/TGC/loader/save installation changes were made.0.0.13 SP and0.0.14
  MP-probe archives remain as rollbacks. The screenshot's exact trigger and
  live recovery, clickability and performance remain unverified.

Next test: instructions-volunteers-entry.md. Keep the native visual reference;
no layout/mockup change is required because all11 visual payloads are unchanged.
