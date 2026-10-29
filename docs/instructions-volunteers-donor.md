# VOL-DONOR: native donor-owned volunteers

> 2026-10-27: current continuation is instructions-volunteers-native.md.
> Rust 0.0.3 implements UI/command/registry/checkpoint adapters, with native
> prerequisites still open. The user now defers tests/game checks; steps 8-9
> below describe later validation, not permission to run it during coding.

2026-10-27. User decision is resolved: donor owns and the sending player
selects/commands volunteers; recipient/AI does not. Keep our current layout.
All new runtime code stays Rust. Follow report-volunteers-donor-1027.md.

1. Read mission.rs, snapshot.rs, volunteers-design.md and the report's exact
   native ABI/gate table. Run check_exe before address work. Reuse existing
   raw exports; do not restart broad decompilation or the ownership decision.
2. Preserve native donor control/ownership, affiliation, supporting pops and
   donor country-list membership. Reuse same-owner native Create+Transfer
   detachment for selected brigades only, with the donor reference. Use receipt
   verification; no SetOwner calls or recipient ownership workaround.
3. Trace combat-result war attribution and both side predicates. The pair
   helper and repeated direct hostility gates are separate. Inspect native
   eligibility and result consumers before selecting patch boundaries. Build
   targeted fixtures for eligible enemies, recipient allies, two different
   missions, unregistered troops, same-owner troops, retreat/busy state and
   unavailable campaign records. Never override all false eligibility results.
4. Trace native path/access predicates under the recipient context for just
   that verified stack. Resolve departure, travel, arrival, continent/transport
   and return through engine commands. Keep losses/support/attrition native;
   no whole maintenance skip, teleport or global diplomatic access grant.
5. Reuse our native button/observer helpers, with the proved safe template and
   lifetime handling, to implement volunteers-layout.svg. Keep the selector
   and active-forces pane. Do not use Maxioten's auto-send-from-all-armies loop.
   Generate any inherited GUI insertion by named anchors; no full copied GUI.
6. Wire scoped recipient vision to a separately persisted entitlement, keeping
   native visibility as an OR input. Recall/loss/return must not revoke vision;
   known no-war state does. Unknown war state is not peace. Save/load needs a
   real campaign identity and verified reconstruction before reactivating it.
7. Verify native goods debit, source-pop reinforcement, flags and military
   statistics with donor ownership intact. Trace the actual brigade/capacity
   UI separately from military score; do not add extra costs or score manually.
8. Build with tools/build_volunteers.ps1 and tools/test_volunteers.py. Stage
   new versions without overwriting prior checked packages. Do not install
   until the native gates are proved; mode=observe cannot enable Send.
9. On the authorized disposable computer-use test path, verify player movement
   orders, recipient AI exclusion, actual recipient-enemy battle, losses, a
   week of maintenance, physical recall, multi-war peace/vision and save/reload.
   Use native F11 images if Windows capture times out; preserve logs/images.
   Keep MP refused until its synchronized registry/authority card is complete.
