# TPL-8: non-core culture filter, currently held for Rust coordination

2026-10-24: **completed in Rust 0.11.3** (report-tpl-8-noncore-1024.md). The
next step is instructions-tpl-8-check.md. The text below is kept as written.

2026-10-22 update: PR #2 ports the full logic DLL and is merged/pulled at
09a361d6, together with PR #1. Upstream reconciliation is complete; the
implementation hold still applies. On explicit resumption assess both
C++ and Rust changes. The older loader-only scope below is historical.

2026-10-21. User approves third mutually exclusive Non-core only checkbox
in **both** Templates and Conform. No selected box=all, clicking selected
box=clear; Conform may use two lines. User then said wait because fedacking
is porting to Rust. **Do not implement yet.**

1. Read report-tpl-8-rust-coordination-1021.md and current NEXT.md. No source
   changes/build/install exist for TPL-8. Preserve the approved choices.
2. Clarify the scope of fedacking's next port before resuming feature work.
   Public merged PR#1 ports the loader and carries C++ compatibility fixes;
   it does not prove his current unpublished scope. No external messaging
   without explicit human authorization.
3. Before implementation resumes, reconcile merged upstream work with this
   checkout while preserving all uncommitted Province Rename work. Inspect
   exact diff, use an isolated checkout if appropriate, and preserve the
   Templates click-body/SEH split and other build fixes. Never reset local
   work or blindly overwrite upstream changes. No push implied.
4. Open the current filter mockups and update local mockups with the chosen
   design in the implementation session. Preserve native art/style,8builder
   composition rows, current continent behavior and Conform footer. Record
   any two-line/Changes-list layout tradeoff explicitly.
5. Add exact non-primary-and-non-accepted pop filtering, shared preview/
   recruitment semantics, both checkbox handlers, paused refresh and safe
   persistence migration. Keep old preference files for rollback; invalid
   restrictions refuse, never silently become All. GUI/DLL versions must fit.
6. Add meaningful mixed-pop/native eligibility/land reach and preference
   round-trip/invalid/rollback tests. Run the supported-executable gate,
   relevant native/GUI/package/loader checks, TGC scan and render previews.
   These are not gameplay acceptance. Do not broaden into another feature.
7. Stage a separate local candidate. Installation requires fresh user
   exit/install instruction and no running game. Main E: stays unchanged;
   never launch/terminate the game. No release/push without authorization.
8. Preserve evidence and write the required session close with the actual
   remaining player checks. Province Rename publication remains deferred.
