# Volunteers 0.0.13: tester feedback candidate

2026-10-29. User supplies Maxioten's screenshots: missing recipient sight,
intermittent invalid/unavailable volunteers, terminal history clutter, awkward
army selection and apparent distance limits. A later screenshot retracts the
distance complaint and repeats the vision/history/valid-army/load concerns,
asking about an MP test. The user does not know the specific symptom of the
earlier "many volunteers" report. No tester save/log or crash reproduction was
supplied. These screenshots are evidence of symptoms and requests, not native
offset, session-mode or OOS proof.

## Changes and evidence strength

| Report | Implemented correction | Remaining validation |
|---|---|---|
| Save/reload unavailable | Retry journal binding when no registry was obtained, even if player/epoch already match; bind through App's reset path so old daily/page state is cleared | Exact tester failure cause unestablished; new session-specific messages/log needed |
| Returned/lost consume limit after load | Count saved terminal phase, rather than temporary NeedsReview state, when computing deployed brigades | Same-save return/loss reload check |
| History clutter and journal growth | Completed rows hidden by default; Show/Hide history and Clear history native buttons; auto-prune terminal records after entitlement expires at known peace | Native click/reopen/peace gameplay check |
| Many sends | Atomic candidate acceptance refuses a full 16-KiB journal without changing existing missions; IDs remain monotonic after cleanup because retained sequence high-water survives | This reproduces one concrete failure mechanism, not the unknown tester symptom |
| Only selectable valid armies | Native route shortlist excludes busy/already-loaned/foreign or empty stacks; searches multiple recipient land destinations in bounded round-robin batches | Runtime responsiveness, paused search progress, actual access/route behavior |
| Missing recipient sight | Per-province planning isolates malformed/dummy data; valid grants elsewhere survive; native recipient-controller/rebel-owner sight included | Actual grants/ally-like behavior unverified; prior log had zero grants |

The native army ownership/control model, global five-brigade limit and SP-only
admission remain. There is no explicit single-war gate in the current source;
the global brigade cap can explain an apparent one-war restriction. No new
product restriction or sea/teleport deployment is introduced. The retracted
distance complaint is not treated as evidence for changing that policy.

## Load, history and persistence safety

Previously observe_world stored bound epoch/player before a journal read. If the
read failed while registry stayed None, the same binding would not retry. The
restored registry was assigned directly instead of using bind_registry, leaving
last_day from an earlier, later-dated run. Both are independently inspectable
source defects. These fixes do not assume the screenshot really entered MP or
weaken local transport/load checks. A malformed nonzero journal still refuses;
it never becomes a new empty campaign or replays Create/Transfer.

History clear is local coordinator state checkpointed through the existing
engine-variable command journal. It does not delete troops, change relations,
alter engine lists or write simulation memory directly. Terminal records with
active/suspended vision remain until recipient peace; the UI can hide them
without revoking that entitlement. Cleared records never reuse a serial; country
sequence totals form a conservative monotonic high-water. Suspended terminal
records do not count as active brigades. Page/index clamps follow cleanup.

The native forces window retains 332x266 bounds and existing card/recall layout.
Two native 148x28 buttons occupy the existing space above the footer at y194:
Show/Hide history and Clear history. Own generated mockup updated in
volunteers-layout.svg; previous SVG archived verbatim. Native PNG preview in
work/volunteers-0013 was inspected. It is a generated layout check, not a game
screenshot. No external hosted mockup was republished.

## Route and sight safety

The old Send preview tried only the country's first owned land province. The
new chooser enumerates the recipient's owned land IDs, prioritizes current/nearby
provinces and probes the native pathfinder in batches of at most eight per
advance. Round-robin scheduling stops an unreachable first army from starving
another. There is no province-ID, country-count or distance cutoff; native land
access remains decisive. Per-callback work is bounded; action callbacks can
advance more than once, while no new per-frame hook was installed. The search
advances on existing GUI updates/clicks/daily ticks; paused progress and maximum
callback cost remain runtime acceptance items. Reopen/change recipient to refresh.

Cached targets are display-only and retain no native pointers. Fresh session,
explicit load epoch, donor/recipient and source gates precede route work. Send
checks that the destination is still recipient-owned land and reruns the native
route before accepting; engine command preparation retains its own gates. Load,
player, recipient and army availability changes rebuild the shortlist. A route
becoming newly available without such a change can require reopening the panel.

Targeted read-only Ghidra export of FUN_0068C040 (VA0x68C040/RVA0x28C040) confirms
province+174 is an int-array pointer and +178 its length; map+360 is local sight.
VA68C1EA..68C2B9 establishes controller+134, tag+130 with REB owner+12C fallback,
and alliance-propagated sight. The candidate reads these same established fields
and edits only local map display via the existing adapter. Invalid per-province
data gets no grant; other independent grants survive. Global torn-session/world
observations still refuse. vision_skipped_provinces reports partial refusals.
No simulation knowledge at province+174 or relation/alliance field is written.
Raw .c/.asm files remain private under work/volunteers-0013/research.

## Multiplayer request

The screenshot asks whether the current prototype can test MP/OOS. It cannot
send volunteers in MP: synchronized registry construction/command authority and
late-join/load behavior are not implemented. Merely removing the MP guard would
repeat the local-state divergence risk. Mode=preview on both peers can test a
read-only load/UI baseline, with Send/Recall still refused; it cannot test actual
volunteer combat synchronization. Matching DLLs/digests are not OOS proof.

The owned evidence collector now includes z_CreatineCore core/settings/performance
plugin hashes and its current logs alongside the preserved legacy V2DLL tree.
One synthetic fixture checks those paths remain distinct and binaries stay
hash-only. No paired peer evidence or multiplayer game was collected/run.

## Validation and handoff

- 51 Rust tests pass, two explicit timing benchmarks ignored normally. New
  regressions exercise history/vision lifetime, post-cleanup IDs across load,
  loaded-terminal brigade accounting, backward-date reset, full journal atomic
  refusal, bounded/fair multi-destination route search and distinct MP/unknown
  errors without outbox work. Expanded vision fixture checks independent grants,
  short/bad arrays and controller sight with simulation knowledge unchanged.
- Strict Clippy, formatting and release compilation pass. Six package/actual
  loader/settings tests, two share-package tests and one collector test pass.
- Reference executable and 96 native signature records pass; no new hook site
  or ABI was introduced. All Rust logic changes retain native admission gates.
- Final plain ZIP has 20 files, 443954 bytes, no installer or command scripts:
  work/volunteers-0013/final-r2/Volunteers-0.0.13-SP-beta.zip.
  SHA-256: 649454bb0a9d630b6f68d621194e6e39c160c607edd147e6a2d42a1a513d1f1d.
- DLL SHA-256: 156a02d2ff2551bf90d4acaae0dd2d7c87d73149ee1567e01bc611e068b9721b.
  Manifest, ZIP CRC and deterministic rebuild tests pass. The descriptor/config
  retain script-free normal-launcher selection and prototype SP mode.
- No game launched, desktop input sent, peer messaged, save edited or installed
  payload changed. The accepted local 0.0.12/CreatineCore6.36 copy stays intact.
  New candidate fixes are done but not tested in game. Instructions are in
  instructions-volunteers-feedback.md; no complete prototype/MP claim is made.
