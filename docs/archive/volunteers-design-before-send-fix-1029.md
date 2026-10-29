# Standalone volunteer submod: working design

2026-10-29 implementation: experimental SP0.0.7 connects donor-owned gameplay,
engine-command journal, scoped combat/access/vision and explicit load handling.
The user specifies Send volunteers in the native empty diplomacy action cell
beside Command Units, using the same thin button/font. Existing two panels stay.
See report-volunteers-prototype-1029.md for S evidence and untested limitations.
The generated design detail is not a runtime screenshot. MP remains refused.

2026-10-28 user correction (D23): actual in-game assets and a flush native
look are required; the flat placeholder art is rejected.0.0.5 uses locally
derived native army-panel/button/row/arrow textures and native bitmap fonts,
with runtime caption textboxes and side docking. Generated preview inspected;
new game appearance unverified. report-volunteers-native-art-1028.md.

2026-10-27 pitfall audit: current Rust0.0.4 retains this donor-owned design.
Default mode is observation; native UI is opt-in and untested. Missing native
session/campaign/combat/access/vision/save integration still blocks Send.
See report-volunteers-pitfalls-1027.md for source hardening and residual risks.

2026-10-27. Selected by the user: a HoI4-like volunteer mechanic, packaged and
styled alongside PRU and ATCU. Provisional internal name/folder:
`tgcx_volunteers`; final public name remains a user decision (D3).

2026-10-27 user decision: **donor ownership, with the sending player retaining
control**, while keeping our existing volunteer layout and reusing proved code.
This replaces the earlier recipient-owned model. Its prior text/layout remain
in docs/archive/volunteers-design-before-donor-1027.md and
volunteers-layout-before-donor-1027.svg. Maxioten's screenshot is a visual
reference for foreign selection, not proof of costs/control/combat/MP behavior.

## Intended behavior

The donor selects land regiments and sends them as a dedicated volunteer stack
to a recipient at war. The donor keeps command while remaining outside that
war. Army and regiment ownership, affiliation and supporting pops remain with
the donor. The sending player selects and orders them through the normal army
interface; the recipient and its AI get no command permission. Preserve donor
goods/accounting, donor flags and donor military statistics without adding a
second charge or score contribution. These native consequences still require
end-to-end validation.

Only the registered dedicated stack's combat/access context can use the
recipient's war relationships. Never grant the donor's whole military those
relationships or add the donor country to the recipient's wars. A native
Create+Transfer may detach chosen donor regiments into a donor-owned stack,
initially at their source province. Physical travel/arrival is a separate
command path; this is not a teleport or a transfer of ownership.

Track original regiment/pop identities and losses. Recall or recipient peace
requests physical return with donor ownership unchanged. Grant the donor vision
of recipient-owned territory for the mission; a granted entitlement persists
after recall, destruction or departure until the recipient has no wars.
Preserve every native source of visibility. Suspend unverified grants on load
rather than applying a previous campaign's data.

Use dedicated stacks only: an army containing unrelated recipient regiments
must not enter the loan registry merely because it contains one volunteer.
Do not grant blanket control of the recipient country's military. Keep native
overlord control intact. No disband/rebuild substitute or free replacements.

Initial proposed SP policy: donor neutral, recipient at war, both distinct and
non-hostile; land troops, no active combat or movement at dispatch. Default
limit is five brigades across all deployments from that donor, configurable
1..64. These are working defaults, not claimed exact HoI4 balance rules.
Travel/arrival delay, consent/AI acceptance, access/transport rules and a final
public title remain open product choices. The first probe does not implement
or enforce game-changing parts of this policy.

## Delivery stages

| Card | Deliverable | Current state |
|---|---|---|
| VOL-0 | Independent Rust plugin, read-only runtime probe, byte gates and package | 0.0.1: nine daily samples/two armies observed; remaining acceptance open |
| VOL-1 | Donor detachment, selective recipient combat/access, player control and accounting | 0.0.3 native donor command wrappers coded; scoped native adapters open |
| VOL-2 | Native chooser/active loans/Recall, stable registry, save/reload and identity verification | 0.0.3 optional native UI/registry/checkpoint endpoints coded; untested |
| VOL-3 | Recipient-peace return, refusal/loss handling, balancing and user acceptance | 0.0.3 daily receipt/recall coordination coded; native gates still closed |
| VOL-MP | Sender authority, replicated registry, peer-compatible serialization and MP tests | Deferred under D5; no MP support claimed |

The user has authorized implementation. The earlier research note saying
"no implementation selected" is superseded by that request. Runtime behavior
still requires acceptance; its uncertainty is not a reason to fabricate offsets.

## Native implementation boundaries

- Every simulation action must go through a proven engine command or script
  (R5), including detachment/travel/return. Create uses the donor's country
  reference; Transfer stays within donor ownership. Do not call SetOwner or
  modify army/regiment/pop fields to impersonate the recipient.
- Native donor player control is retained. A policy filter may deny loan
  control to other countries, but must never grant recipient control or
  globally alter native overlord behavior for ordinary armies.
- Country list aggregation, map selection, per-unit actions and native queue
  validation must all be checked. Do not claim Move's lack of a local owner
  comparison establishes complete command authorization.
- Foreign-pop maintenance compares army+0xCC with pop-province ownership. Do
  not label +0xCC "controller" or skip the entire maintenance routine based on
  a guessed flag. It also handles pop validity/death/support transitions.
- A queued command is an attempt. Require observed, exact identity membership
  before marking serving/returned. A destroyed unit is lost, never returned.
- Reload/campaign changes suspend unverified records. No active-loan sidecar
  is written by VOL-0. Persistence must distinguish campaigns, not just tag/date.

Combat helper RVA0x1CC160 is followed by repeated direct owner/hostility
comparisons in RVA0x1CBF60 and0x1DBB50. A single true-return patch is insufficient.
Preserve unrelated eligibility gates; see the donor combat report.

`rust/volunteers/src/mission.rs` implements the offline donor-owned contract,
verified membership, separate combat/access context and independent vision
lifetime. It reuses the existing lifecycle and native Create argument order.
0.0.3 adds optional native UI, command wrappers and registry/lifecycle adapters.
The live world still reports an unknown session and no campaign, and capabilities
remain false. Send/Recall cannot post commands. No native combat/access/vision
hooks or actual save/load binding are enabled. Current implementation and OOS
limits: report-volunteers-implementation-1027.md. Tests are deferred by the user.

## Why the first build is a read-only probe

Fresh review of the exported references shows DayTick also called from
Game_FinalizeLoadAndEnterGameplay (VA 0x00662010) at VA 0x00663234 and
0x0066339D. A detour there is not equivalent to one gameplay-day callback.
VOL-0 instead wraps CHourlyTickCommand execute slot 6:

- table VA 0x00E12414 / RVA 0xA12414;
- slot VA 0x00E1242C / RVA 0xA1242C;
- original VA 0x0072ECF0 / RVA 0x32ECF0, ECX command, plain RET;
- original loads world and calls world update with world and command+0x3C.

Only a pointer is replaced; no instructions or simulation fields are changed.
The full executable hash, relocated method bytes, original slot and API base
must match. A different hook owner causes refusal rather than replacement.
The observer records after the original returns, using bounded ReadProcessMemory
reads, thread IDs, depth/date transitions and exact CArmy selection filtering.
That description applies to the original 0.0.1 observer. The 0.0.3 module also
contains optional native UI and command constructors, with gameplay admission
closed until native prerequisites are established.

Tests prove the synthetic x86 wrapper preserves selected machine state and
calls the fixture; they are not game, loader-coexistence or MP acceptance.

## Packaging and visual style

One additive `.mod` folder, independent V2DLL plugin, no ATCU/PRU dependency,
no base GUI/data replacement, `replace_path`, `user_dir` or bundled loader.
The currently installed releases remain separate. All new runtime code is Rust.
Names/keys/files use `tgcx_`; base/private reference art stays in ignored work/.
Our procedural textures are generated from tools/tgcx_volunteers_gui.py.

`docs/volunteers-layout.svg` is the working VOL-2 layout specification. It uses
the accepted 332-pixel army-panel width, green title, gold border, parchment
body, native-style rows and paired footer buttons from PRU/ATCU. A separate
active-loans pane shows donor/recipient, brigade count and recall status.
It is a design mockup, not a game screenshot. 0.0.3 codes these controls with
paging arrows; visual/native behavior remains untested. See the current report
for deviations. No existing
published Main/Bill/Ruler/Economy/Templates/Conform board has been changed.

## Runtime acceptance order

VOL-0 tick/selection/load observations first; then one brigade loan and control
in a disposable isolated SP campaign; paused and advancing, battle and a week
of pop maintenance; recall and peace; destruction; save/reload; coexistence
with PRU/ATCU. MP requires its own selected work and peer tests. The user
now authorizes desktop control if necessary for the agent's own test, using
the same Terra/high subagent and disposable game copy. The earlier separate
surface requirement is superseded. See instructions-volunteers-desktop.md.
