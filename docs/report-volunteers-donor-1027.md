# Volunteers: donor ownership and sending-player control

2026-10-27. The user chooses donor ownership, retains our existing volunteer
layout, authorizes reuse of the proved code, and explicitly reiterates that
the **player must control the volunteers**. This resolves the ownership-model
question raised by the Maxioten cross-reference.

## Implemented this session

Rust0.0.2 adds `rust/volunteers/src/mission.rs` on top of the existing policy,
lifecycle and observer. This is an offline contract, not enabled gameplay:

- Create argument planning reuses the native ATCU/army-handoff contract at
  RVA0x17DB30, keeping both army and regiment ownership with the donor. It
  selects only the requested regiments; no automatic scan of all donor armies.
  Native creation remains at the source province, with no teleport assumed.
- A serving loan needs an exact observed destination army, donor owner and
  affiliation, complete selected-regiment identity set and donor regimental
  ownership. Recipient/mixed ownership, wrong campaign and changed membership
  cannot produce a validated loan context.
- Command policy admits the donor player and denies the recipient/other
  countries for this verified loan only. Ordinary armies keep their native
  control path. This policy does not install a control hook or prove AI filtering.
- Accounting/ownership/flags/statistics remain donor context; only combat and
  access may use the recipient context, and only while it is known to be at war.
- Queue posting cannot forge arrival or return. A return receipt requires the
  verified donor-owned stack physically in donor-owned territory. Ownership
  is never transferred back because it never leaves the donor.
- A separately granted vision entitlement survives recall, destruction and
  return until recipient peace, preserves native visibility and is restricted
  to the donor viewing recipient-owned provinces. Unknown war state does not
  falsely report peace. Load/campaign changes suspend unverified grants.

`snapshot.rs` adds a bounded full-stack receipt reader using existing proved
fields. It checks exact native army/regiment types, parent pointers, all member
identities, count, cycle/deferred-removal state and maximum64 regiments. It does
not read a partial list and silently declare it a complete deployment.

The actual runtime still only installs the existing guarded tick observer;
there is no Send constructor dispatch, loan registry, visibility grant, player
permission patch, AI filter or save writer enabled by this candidate. The new
policy is reusable tested code for those adapters, not a shipped mechanic.

## Native combat trace: why one permission patch will not suffice

All addresses below apply only to the reference EXE; check_exe passes.
S = inspected existing raw instructions, not new runtime battle evidence.

| Native path | ABI / sites | Consequence for donor-owned troops |
|---|---|---|
| Pair hostility helper FUN_005CC160 | VA0x005CC160 / RVA0x1CC160; ECX first army, EAX other army; no stack args, AL, plain RET | Checks both +0xD4 exclusions, resolves owner +0xC4, tests relation +0x34 and REB cases. Its custom EAX/ECX ABI is not an ordinary two-argument fastcall. |
| Initial admission FUN_005CBF60 | VA0x005CBF60 / RVA0x1CBF60; ECX first army, EAX other army, stack province, AL, RET4 | Calls helper at VA0x005CC08F / RVA0x1CC08F, then repeats direct owner-country hostility at VA0x005CC0EC / RVA0x1CC0EC. Returning true from the helper alone still fails the later gate. |
| Existing-side eligibility FUN_005DBB50 | VA0x005DBB50 / RVA0x1DBB50; EAX army-list header, stack candidate army, AL, RET4 | Calls helper at VA0x005DBCAA / RVA0x1DBCAA, then repeats owner hostility at VA0x005DBD04 / RVA0x1DBD04 for existing armies. |
| Native combatant predicate FUN_00594690 | VA0x00594690 / RVA0x194690; ECX combatant, stack candidate | Builds EAX=this+0x20 then tail-jumps to FUN_005DBB50. CLandCombatant slot16/+0x40 uses it. |
| Join existing combat FUN_0059C320 | VA0x0059C320 / RVA0x19C320; ECX combat, stack army, RET4 | Tests both side predicates through +0x40, joins only one eligible side and stores army combat pointer. Both sides need a consistent scoped context. |
| Create combat FUN_005B5840 | VA0x005B5840 / RVA0x1B5840; EAX kind and four stack args, RET16 | Creates sides, adds both seed armies, then tests the remaining province armies. Changing only movement/UI would not handle this admission path. |

There are also native category, activity, same-owner, retreat/other state and
province gates. Preserve them. Do not replace the entire predicate with a
single "registered volunteer => true" result, mutate owner fields temporarily,
or change the country's diplomacy globally. Battle resolution/war attribution,
recipient-side ally handling and native access predicates still need tracing
before installing a donor-owned combat adapter.

Raw evidence is reused from work/volunteer-handoff-1026/research/functions/;
no repeated 201-function export or whole-executable decompilation was needed.

## Layout and supplied screenshot

Retained the two332px green/gold/parchment panels, army/regiment selection,
Send footer and active-forces/Recall pane. Updated explanatory text for donor
ownership/payment/flag/statistics, exclusive player control and vision lifetime.
Previous design/SVG archived verbatim before editing. Current SVG rendered to
work/volunteers-0002/volunteers-layout.png and visually inspected; controls do
not overlap. Inkscape's optional AppData log/recent-file writes were denied;
PNG export succeeded. No remote historical board was changed.

Maxioten's provided screenshot was inspected and retained unchanged privately
as work/volunteers-0002/maxioten-reference.png. It shows Austria's view and
selected Ottoman-labelled armies, Ottoman flags and visible unit panels. It
does not establish who issued orders, paid maintenance, counted statistics,
entered combat or synchronized multiplayer. We retain our chooser layout.

## Verification and candidate

- x86 build, formatting and strict Clippy all-targets pass.
- 24 Rust tests pass, including seven new donor/vision/receipt scenarios and
  one full native-layout receipt-reader scenario. Existing x86 ABI wrapper,
  signature/refusal and bounded-selection checks remain green.
- Five Python package/actual-DLL unsupported-host tests pass. They verify API
  shape/refusal and package integrity, not upstream gameplay or multiplayer.
- Candidate: work/volunteers-0002/Volunteers-technical-preview-0.0.2-donor-checked.zip.
  SHA256 af4e43964fa2473ad8b4c5c4c2969f4abbc429d198b61571df6a0e0fa078ee08.
  DLL SHA256 3c96bcad5e1cd9ba998bb51aa70a57e5a94c240f9a1095656ca69846a4a7fa43.
  Six additive files; observe-only. Earlier0.0.2 staging without -donor-checked
  is interim; earlier0.0.1 checked ZIP and installed disposable copy preserved.

No game launch/input or installation this session. The previous0.0.1 observer
run's nine samples/two armies do not test0.0.2 or donor-owned loans. No full
gameplay, save/reload, cost/flag/score, AI-exclusion or MP acceptance is claimed.
Next card: instructions-volunteers-donor.md. Native integration remains open.
