# VOL-NATIVE: remaining donor-owned native integration

> 2026-10-29: superseded as the current entry point by
> instructions-volunteers-prototype.md and report-volunteers-prototype-1029.md.
>0.0.7 is installed in the same isolated E: copy; native-art requirements remain.
> This older card is preserved below as historical instructions.

> 2026-10-28: the user now authorized an isolated E: test installation.
> Completed at E:\Victoria-II-Volunteers-0004; no game launched. The older
> no-install phase below is superseded for this copy. Current entry point:
> instructions-volunteers-e-copy.md and report-volunteers-install-e-1028.md.

> 2026-10-27 update: current candidate is Rust0.0.4 after the pitfall audit.
> Read instructions-volunteers-audit.md and report-volunteers-pitfalls-1027.md.
> Default observation, explicit untested UI opt-in; new native preflight,
> lifetime/CPU/exception/patch and persistence guards must be retained.
> The0.0.3 references below describe earlier coding, not current validation.

2026-10-27. Current authority: the user wants donor ownership, sending-player
control, our layout and as much code as possible; **tests come later**.
Do not launch a game, resume Terra, install or execute test suites during this
coding phase. Prior desktop permission does not override that latest ordering.

1. Read report-volunteers-implementation-1027.md, volunteers-design.md,
   session.rs, app.rs, native.rs/native_gui.rs and the current registry/mission
   interfaces. The 0.0.3 candidate is compiled, untested and gameplay-blocked.
   Reuse old raw exports; do not repeat broad decompilation or reopen ownership.
2. Run tools/check_exe.py before address work. Preserve R1/R5/R7: immutable game
   files, base plus RVA, command-based mutation. Add new evidence/guards to
   docs/symbols.md and the scoped generator; unknown ABI stays unestablished.
3. Trace actual network/session mode and client authority from native command
   submission/deserialization. Implement verified SP admission only; leave MP
   and transitions unknown until positively identified. Do not add a config
   override, assume that one connected player means SP, or activate a registry
   using process addresses, tag/date or a manually entered campaign ID.
4. Establish actual engine save-complete/load-complete/new-campaign signals and
   stable campaign association. Connect lifecycle.rs and bind_registry with a
   fresh epoch. Suspend before load/country/campaign transitions. Require exact
   save digest and membership before resuming active records; terminal vision
   uses its separate verified restoration path. Never replay pending commands
   merely because a checkpoint exists. Define orphan/missing-checkpoint handling.
5. Finish scoped combat from RVA0x1CC160, 0x1CBF60 and 0x1DBB50. Inspect repeated
   hostility comparisons, army eligibility, province rules, side assignment,
   result/war attribution and recipients' allies. combat.rs supplies policy,
   not proof that a native boundary has passed unrelated checks. Do not return
   true for every registered army or temporarily swap owner fields.
6. Trace per-stack path/access under the recipient context and implement its
   native adapter. Preserve unrelated native gates, continent/transport rules
   and physical return. The current first-owned-land destination is provisional;
   establish reachability/choice rather than promising that every target works.
7. Implement recipient-owned-territory vision as an OR of native visibility
   plus verified donor entitlement. Preserve it after return/loss until known
   recipient peace. Unknown war state suspends/restores conservatively. Do not
   globally grant map vision or change native sight for other players.
8. Complete coordinator edge cases before admission: partial command outcomes,
   rejected/interrupted paths, construction waits, losses during return,
   country switching, terminal records/capacity, bounded history and explicit
   review of pending work. Native busy state must defer recall orders.
9. Refine UI against volunteers-layout.svg: exact diplomacy anchor, native
   availability styling, stale rows/selection, page labels, frame refresh,
   callback fault containment and lifetime/capacity behavior. Keep original
   generated assets/additive independent windows. Do not copy Maxioten's whole
   GUI or auto-send loop. Record each remaining layout deviation in a new report.
10. During the coding phase use cargo fmt and cargo build --release --offline
    (or tools/build_volunteers.ps1 -CompileOnly). These are compile checks only.
    Stage a new, clearly untested package; never overwrite the final 0.0.3 ZIP.
    Keep Send refused until all native SP prerequisites have evidence.
11. When the user selects testing later, execute meaningful Rust/native/payload/
    checkpoint/UI and package checks, then disposable SP control/combat/cost/
    recall/loss/vision/save-reload checks. Use the same authorized Terra/high
    agent and native F11 fallback if needed. Do not call old observer samples
    acceptance for any new native hook or gameplay.
12. MP remains deferred: the screenshot establishes a desync, not its cause.
    Diagnose from both peers' exact artifacts/logs. Establish command factory,
    serializer/deserializer, issuer and deterministic execute path before
    implementing EngineTransport. Verify registry/sequence execution on all
    peers, late joins/load and peer fingerprint refusal before any MP claim.

Close by archiving NEXT verbatim, recording actual validation strength and
rewriting NEXT/ROADMAP/SUBAGENTS/index/instructions. Do not commit, publish,
install, control the desktop or write memories without a current request.
