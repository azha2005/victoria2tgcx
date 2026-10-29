# VOL-AUDIT continuation and deferred verification

> 2026-10-28: the user now authorized an isolated E: test installation.
> Completed at E:\Victoria-II-Volunteers-0004; no game launched. The older
> no-install phase below is superseded for this copy. Current entry point:
> instructions-volunteers-e-copy.md and report-volunteers-install-e-1028.md.

2026-10-27. Read report-volunteers-pitfalls-1027.md and pitfalls P31/P45/P47/
P54/P59/P60/P64-P68. User ordering remains code/audit first, tests later.

1. Use the0.0.4-audit-untested-r2 candidate; do not install interim stages.
   Default observation is deliberate. The new UI is explicit mode=preview;
   neither configuration enables gameplay or MP. No test/game launch now.
2. Preserve all current hardening when continuing the native card. Use
   instructions-volunteers-native.md for SP/session/save/combat/access/vision;
   the0.0.3 implementation report is historical. Do not bypass preflight or
   accept a guessed type/font because a window refuses to construct.
3. Before any new RVA use run check_exe. Keep old reports and ZIPs unchanged.
   Existing read-only raw exports establish allocation pushes at RVA39DBAB
   (Create0x60),3944E1 (Transfer0x6C),092322 (Move0x90); scope byte guards with
   relocation offsets. Do not replace them with ctor-field-based size guesses.
4. Coding builds use cargo fmt then cargo build --release --offline or
   tools/build_volunteers.ps1 -CompileOnly. Staging validates generated names,
   localisation and exact files; font names were separately checked against
   installed definitions. This is not gameplay acceptance.
5. When the user selects tests, first exercise actual x86 wrappers: CPU flags,
   GPR/x87/SSE/last-error/stack, API size/unaligned/unreadable input, narrowly
   caught AV versus passed-through exceptions, permanent disable and SafeSEH.
   Keep native state assertions independent of the implementation assumptions.
6. Add patch failure injection: successful/failed CAS, changed owner, failure
   restoring protection, rollback/retry failures, partial multi-slot install,
   preserved original protection and pinned residual callbacks. No blind writes.
7. Exercise actual registry/factory paths: root/button/text cross-type name
   collisions, missing/wrong definitions, backgrounds, wrong concrete factories,
   standard fonts, stale definition/context/renderer, foreign-thread callbacks,
   incomplete native allocation, deferred collection and observer capacity.
8. Check transient/backward reads preserve work; repeated unknown stack reads
   lead to review rather than false loss. Check failed create_new preserves an
   existing temp file and last good checkpoint; failed loads invalidate binding;
   malformed/oversized/trailing/wrong-save/duplicate checkpoint data refuses.
9. Only then use the authorized disposable game and same Terra/high agent for
   menu/load/show-hide/reopen/resize/selection/long-session/campaign-switch/
   save-reload/exit checks. F11 remains the capture fallback. Preserve both game
   and plugin logs; unsupported main-menu state must stay inactive.
10. Gameplay testing waits for all native SP prerequisites, and MP for its own
    command/registry replication work. Do not advertise an OOS fix from local
    deterministic data or matching hashes alone.

Record exact validation strength and remaining cases. No commit/publication,
installation, desktop control or memory write is implied by this audit card.
