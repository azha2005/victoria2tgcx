2026-10-29 follow-up:0.0.7 gameplay test found a five-to-four split and lag.
Use instructions-volunteers-send-0008.md for the built/staged correction and
normal-exit install first. The older implementation status below is historical;
full gameplay acceptance steps remain required.

# VOL-PROTOTYPE: next acceptance and implementation steps

2026-10-29. Current implementation/report: report-volunteers-prototype-1029.md.
The user requested coding first; no new game/behavior suite has run. Keep
implementation acceptance distinct from compiler/static/packaging evidence.

1. Read NEXT.md, this card, the report and pitfalls.md P7/P8/P12/P24/P31/P45/
   P47/P51/P54/P59/P60/P64..P69. Run python tools/check_exe.py before address work.
2. Keep the isolated E:/Victoria-II-Volunteers-0004 copy.0.0.7 is installed;
   rollback0.0.6 is work/volunteers-0007/before-0007. Do not alter main/TGC files.
   The prototype launcher is explicit; observe remains default configuration.
3. Complete meaningful fixtures before calling the prototype verified. Use
   tools/build_volunteers.ps1 for Rust tests/lint/release, then tools/test_volunteers.py
   for package and unsupported-loader probes. Tests were deliberately deferred
   during this pass. Add independent x86 ABI fixtures for all five new inline
   wrappers, load fence and journal observer, including RET cleanup, flags,
   x87/SSE, changed signatures, partial SDK/protection failure and original paths.
4. When runtime testing is undertaken, launch tgcx_start_prototype.cmd, never the
   main installation. It changes only mode and invokes the copied v2game.exe with
   explicit TGC then Volunteers descriptors and its own working directory/profile.
   Preview opens UI without volunteer commands; observe avoids optional UI hooks.
5. Start a disposable SP campaign. Keep donor at peace and choose a recipient at
   war. Prefer a donor army already near/in recipient territory so the land route
   is short. Select one idle brigade and Send. Check log local SP classification,
   valid load epoch, every hook admitted, intent journal commit before Create,
   complete transfer receipt, unchanged donor ownership and ordinary player orders.
   A refused gate is evidence to investigate, never a reason to force capabilities.
6. Confirm initial battle creation and joining an existing battle against a
   recipient enemy. Verify native non-volunteer gates still reject inappropriate
   units. Recipient should appear once in the combat-side country list, and the
   native result should resolve the recipient's war. Donor stays outside the war.
   Check recipient AI cannot command the stack and ordinary donor stacks stay neutral.
7. Inspect donor supply, reinforcement origin, flag and military rank accounting.
   Check vision includes recipient-owned territory, survives recall/destruction,
   expires only at known peace and preserves ordinary sight. Test pause/switching/
   return/loss; a read fault must not be treated as destruction. No owner swaps or
   native maintenance skips are acceptable repairs.
8. Save/reload in service, during assembly/return, and after troops return/die.
   The engine journal must load suspended under a new explicit epoch and revalidate
   receipts. No Create/Transfer replay, duplicate registry, stale scope or revived
   dead army. Terminal vision must restore without needing an army object.
9. Recall while moving/in combat and after recipient peace. Defer busy stacks.
   A land route must be established; record stranded/no-route cases and improve
   destination selection/return policy without teleporting or changing ownership.
   Provisional destination is the first recipient-owned land province; return
   searches up to128 native routes for donor-owned land. Journal is bounded16KiB.
10. Close normally and preserve screenshots/logs/save artifacts. Use supported
    computer-use skill and the previously authorized Terra/high route only for
    an actual agent-operated desktop test. F11 capture fallback requires fresh
    observations; never send blind input. MP remains refused; follow its separate
    OOS card. Update NEXT/ROADMAP/SUBAGENTS/index and immutable new reports.
