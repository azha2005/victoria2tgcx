# Volunteers0.0.10: battle confirmed, siege admission and visibility follow-up

2026-10-29. User reports no siege, but confirms combat and supplies the Battle
of Sao Paulo screenshot. Read the preserved0.0.9 log as requested. No desktop
input or launch was performed during this follow-up; user retains testing.

## Observed0.0.9 result

- Army40/2183 stays owner/affiliation625, five regiments, supporting-pop owner625.
  Combat=true samples appear in province2446, then combat=false. User's screenshot
  shows fighting and donor Argentina's top bar still At peace. Combat is now
  runtime-confirmed for this scenario; full prototype acceptance remains open.
- Last counters: country_bypass1512820, country_target173, exile_granted288,
  vision_runs389, vision_grants0, thread_refusals0. Unrelated shared-vector work
  is being bypassed; no wrong-thread fallback in the sample. This is not measured
  proof that all lag is fixed. Exile permission overrides execute in the game.
- Zero custom vision grants require further correction; the screenshot's local
  light area alone does not prove recipient-wide sharing. Native own-army vision
  can produce it. Captured log: work/volunteers-0010/before-tgcx_volunteers.log.
- Game had already exited when the log was inspected. No live siege/exile fields
  were read and no game process was interrupted.

## Native siege paths and correction (S)

Raw exports .c/.asm.txt and map-refs.txt in work/volunteers-0010/research and its
parent. Address gate passed before use and after96 signature records regenerated.

| Ghidra VA / RVA | ABI / evidence |
|---|---|
|0x005B5710 /0x1B5710| EAXprovince, EDIarmy, RET, result3=siege; preserves native no battle, land, exile/retreat/movement/strength and controller checks |
|0x005CC200 /0x1CC200| ESIprovince, stackarmy, ALresult, RET4; initial-siege hostility uses army+C4 country against province+134 controller, with D4/rebel gates |
|0x005A9E80 /0x1A9E80| ECXfortress side, stackarmy, ALresult, RET4; CFortressCombatant slot16, separate owner-based hostility gate when joining a siege |
|0x005AB650 /0x1AB650| CSiegeCombat slot17 join; checks friendliness to attacker lead, fortress admission, no current siege and no movement |
|0x005A9FA0 /0x1A9FA0| creates CLandCombatant attacker and CFortressCombatant defender; attacker type confirmed by FUN_00599640 |
|0x005AA190 /0x1AA190| native siege update: attacker side+34's first country drives hostility and completion; native control change uses its country reference |

Initial and existing-siege hostility now evaluate the original read-only
predicate with a private recipient army image. Exact SP epoch, donor neutrality,
recipient war, real current province and complete registered receipt are required.
Joining also checks recipient friendliness with the existing siege's lead country;
it cannot join a hostile recipient's siege merely because the real donor is neutral.
Existing movement/retreat/exile/strength checks remain. Both new hooks must install
before gameplay admission; uncertain/partial installs preserve native paths.

The existing AddArmy country-reference hook already covers CLandCombatant attacker
sides created by sieges. It remaps only a validated registered volunteer's country
reference to the recipient, preserves a single recipient entry, and leaves ordinary
armies unchanged. That feeds native hostility/progress and final control attribution.
No direct controller/owner, army membership, siege progress or state writes were
added. Creation, ticking and capture remain in the engine's own command path.
Actual siege creation, progress, joining and recipient capture credit need runtime
verification; static tracing and fixtures do not prove them.

## Visibility correction and diagnostics

Native world visibility permits per-country arrays of length0. Our planner instead
refused the entire world on any empty array, potentially discarding every valid
grant.0.0.10 tolerates empty entries, still rejects invalid nonempty arrays and
out-of-range levels, and retains native/local maximum visibility. A regression
reproduces a non-null empty dummy among otherwise valid provinces. Known primary
and secondary CMapProvince tables (RVAs9FD310/9FD338; constructor places secondary
at+8) are guarded and accepted; this is a type/layout safeguard, not a claim the
old runtime refusal was proved to occur at that exact check.

Added siege_granted/siege_join_granted and vision_refused/vision_empty counters
separate successful overrides, failed plans and no-new-grant results. Per-country
province+174 remains read-only. Recipient peace still stops grants, and ordinary
native vision is retained. No fog/checksum disabling.

## Delivered and verified

41 Rust tests pass, including initial-siege ESI/stack/RET4 and fortress-join
ECX/stack/RET4 native wrapper fixtures. Strict Clippy/format/release,96 native
signature gates and five package/actual-loader checks pass. Installation to the
same isolated E:/Victoria-II-Volunteers-0004 succeeded with no running game:
15 payload hashes match; nine protected base/loader/TGC files unchanged.
0.0.9 rollback: work/volunteers-0010/before-0010.

Private ZIP: work/volunteers-0010/Volunteers-0.0.10-SP-siege-private.zip.
ZIP SHA256: e951a1798902bc829898110f5d8190de495c4abf6258db8896deb854f0fcba86.
DLL SHA256: 4d58c2f62c321b1e04e73e89bc5996c947c6da39e55a7b027ea6d565f8a1f4e3.
No0.0.10 launch/game test, new subagent, commit, publication or memory edit.
MP remains refused; no paired OOS fix claim. Follow instructions-volunteers-siege.md.
