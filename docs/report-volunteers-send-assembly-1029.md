# Volunteers0.0.8: Send assembly failure and avoidable daily work

2026-10-29. User reports Send never completes, supplies ARC -> RGS five-brigade
"awaiting verification" screenshot, and reports lag. This supersedes the earlier
France-at-war refusal diagnosis for this later attempt. Current ARC is at peace,
RGS is at war and nonhostile. Native appearance remains accepted.

## Evidence and diagnosis

Reference EXE SHA256 and existing address gates pass. Read-only process inspection
of PID29728, E:/Victoria-II-Volunteers-0004/v2game.exe, loaded base0x00EB0000:
ARC625 owns source army(40,2183), one brigade(40,2182), and new donor-owned
army(4713,7438), four brigades(40,2178..2181). Both remain idle in province2348.
Owner/affiliation remain625. Therefore Create and four Transfers actually executed;
the request did not merely fail its policy gate. Raw read-only probes and snapshots
are retained under work/volunteers-0008 and work/volunteers-0007/send-check-1029.
No remote memory writes, desktop input, game launch or forced exit was used.

S: native can-transfer FUN_00513F30 / RVA0x113F30, raw .c/.asm.txt in
work/army-handoff-1026-v2/research/functions, checks the destination's land slot14,
then source leader slot8; when true, source regiment count+0x40 below2 refuses.
Instructions VA0x00513FF3..0x0051401B show the final guard. R: source leader slot8
points to loaded base+0x631490, the guarded return-true helper, and source count1.
This explains the last selected brigade staying behind. Prequeue command validity
was insufficient: execution separately calls this can-transfer gate.

R: preserved log contains325 ARC journal commits through head325/date59891184,
matching repeated daily ticks while the mission stays unresolved. S: Registry
serialized last_processed_date, which changed every day, forcing another native
variable transaction/checksum/lookup sequence. This is a concrete source of
avoidable work consistent with the lag report; no frame-time attribution or full
performance fix is claimed. The visibility callback also scanned the entire world
when invoked; no usable mission still published a scope and entered its checks.

## Corrections

- Whole-army selection retains the existing donor army and leader. It allocates no
  new identity and queues no Create/Transfer. Native preparation rereads every
  selected regiment's owner/parent and proves exact complete source membership.
  Partial selections keep the native Create+Transfer batch and leave source members.
- A complete exact receipt is still required before war/access/vision scope. An
  incomplete group remains refused. Travel and return remain native Move commands;
  ownership and ordinary maintenance stay unchanged. No transfer predicate bypass.
- Daily sampling watermark stays runtime-only. The existing wire word is reserved0,
  retaining old journal decoding. Genuine state/vision/membership changes still
  checkpoint.300 unchanged-day regression confirms identical bytes; peace changes them.
- No native scope is published for empty/unverified missions without vision.
  Visibility iterates only entitled recipients' owned-province ID vectors, validates
  current ownership, bounds, map type and vector stability, then applies local display
  grants as before. A sparse40000-province fixture needs fewer than100 reads for one
  entitled province; this is bounded-read evidence, not a game timing benchmark.
- Interrupted complete assembly on load also sets assembly_verified, preventing a
  second assembly check from rejecting an already-serving record. No command replay.
- Returned whole armies can be sent again: terminal history no longer shadows the
  current live mission or rejects its army ID; terminal vision history is retained.
- Specific policy refusals, nonblank Army<ID> fallback, partial assembly count, and
  per-deployment/movement queue diagnostics make subsequent failures distinguishable.

## Verification and remaining work

36 Rust tests pass, including seven new regression scenarios; strict all-target
Clippy, format and release build pass. Five Python package/actual x86 loader checks
pass, including unsupported-host refusal, unticked inactivity, manifest and ZIP
integrity/determinism. These are not a successful in-game Send, native combat proof,
measured lag improvement or multiplayer acceptance. MP remains refused (P69).

0.0.8 is staged, not installed: the game still runs and its DLL is not replaced.
Private ZIP: work/volunteers-0008/Volunteers-0.0.8-SP-send-fix-private.zip.
ZIP SHA256: 3a74938684a3a628c9066c21a039b93bd56d40c7cb831c70c0e813e0b9c9a82f.
DLL SHA256: af4fc19a78cbeb10b7f905786113d85a793cfbeb9242076f5f439d7363081d11.
The earlier0.0.8 prototype-private stage is an intermediate build; use send-fix-private.
The exact-target updater work/volunteers-0008/update_e_copy.py checks normal exit,
manifest, EXE and allowed destinations; it backs up0.0.7 and verifies nine protected
files before/after. It changes only our package and test metadata, never TGC data.

For a clean check, use a fresh campaign or a save from before the failed0.0.7 Send.
Existing partial missions are retained for review and never automatically replayed.
Follow instructions-volunteers-send-0008.md, then the prototype/OOS acceptance cards.
