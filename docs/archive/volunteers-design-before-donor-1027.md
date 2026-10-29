# Standalone volunteer submod: working design

2026-10-27. Selected by the user: a HoI4-like volunteer mechanic, packaged and
styled alongside PRU and ATCU. Provisional internal name/folder:
`tgcx_volunteers`; final public name remains a user decision (D3).

2026-10-27 supplied-reference review: work/volunteermaxioten/message (1).txt
requires donor ownership, donor payment/flag/score, no recipient control and
recipient vision until peace. That differs from the recipient-owned model
below. It has been cross-referenced, not silently adopted. Resolve this design
choice before gameplay implementation; see report-volunteers-maxioten-crossref-1027.md.

## Intended behavior

The donor selects land regiments and sends them as a dedicated volunteer stack
to a recipient at war. The donor keeps command while remaining outside that
war. The recipient's war relationships govern combat, and its army-owner goods
pool supplies the stack. Track original regiment/pop identities, preserve
losses, recall explicitly or request return when the recipient is at peace.

Use dedicated stacks only: an army containing unrelated recipient regiments
must not become donor-controlled merely because it contains one volunteer.
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
| VOL-1 | Prove native command-based ownership/transfer, army-specific donor control and pop maintenance | Open; commands not enabled |
| VOL-2 | Native chooser/active loans/Recall, stable registry, save/reload and identity verification | Proposed layout; runtime UI not built |
| VOL-3 | Recipient-peace return, refusal/loss handling, balancing and user acceptance | Pure lifecycle model built; native adapter open |
| VOL-MP | Sender authority, replicated registry, peer-compatible serialization and MP tests | Deferred under D5; no MP support claimed |

The user has authorized implementation. The earlier research note saying
"no implementation selected" is superseded by that request. Runtime behavior
still requires acceptance; its uncertainty is not a reason to fabricate offsets.

## Native implementation boundaries

- Every simulation action must go through a proven engine command or script
  (R5), including initial ownership and return. A direct call to the internal
  owner setter is not a command adapter.
- CArmy slot 2 has an army pointer and can apply a loan-specific permission.
  The country-only helper does not and cannot be the sole authorization patch.
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
It deliberately contains no Send or native command constructor entry point.

Tests prove the synthetic x86 wrapper preserves selected machine state and
calls the fixture; they are not game, loader-coexistence or MP acceptance.

## Packaging and visual style

One additive `.mod` folder, independent V2DLL plugin, no ATCU/PRU dependency,
no base GUI/data replacement, `replace_path`, `user_dir` or bundled loader.
The currently installed releases remain separate. All new runtime code is Rust.
Names/keys/files use `tgcx_`; generated/private art stays in ignored work/.

`docs/volunteers-layout.svg` is the working VOL-2 layout specification. It uses
the accepted 332-pixel army-panel width, green title, gold border, parchment
body, native-style rows and paired footer buttons from PRU/ATCU. A separate
active-loans pane shows donor/recipient, brigade count and recall status.
It is a proposal/mockup, not a screenshot of implemented controls. No existing
published Main/Bill/Ruler/Economy/Templates/Conform board has been changed.

## Runtime acceptance order

VOL-0 tick/selection/load observations first; then one brigade loan and control
in a disposable isolated SP campaign; paused and advancing, battle and a week
of pop maintenance; recall and peace; destruction; save/reload; coexistence
with PRU/ATCU. MP requires its own selected work and peer tests. The user
now authorizes desktop control if necessary for the agent's own test, using
the same Terra/high subagent and disposable game copy. The earlier separate
surface requirement is superseded. See instructions-volunteers-desktop.md.
