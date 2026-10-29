# Volunteers0.0.7: experimental donor-controlled SP prototype

2026-10-29. The user reported the existing native UI working, requested continued
coding until a prototype, specified the vacant diplomacy action slot beside
Command Units and asked to try to solve the reported OOS. This pass implements
the first connected SP gameplay candidate. **No campaign or two-peer acceptance
has been run.** It is an experimental implementation, not a verified playable
release or a reproduced OOS fix.

## Delivered and installed

- Native160x24 GFX_button_thin_160 entry with vic_18_black and caption
  **Send volunteers**, matching the supplied reference. Placement reads the
  actual action-grid children; no guessed screen coordinate or whole copied GUI.
-0.0.6 button-only build was installed with0.0.5 rollback; then0.0.7 installed
  into the same authorized E:/Victoria-II-Volunteers-0004 copy.0.0.6 rollback is
  work/volunteers-0007/before-0007. Fifteen package files hash-verified; nine
  protected executable/base/loader files unchanged. No game launched.
- tgcx_start_prototype.cmd is the explicit experimental SP entry point.
  Preview and observe launchers retain their narrower roles. Default config
  remains observe. The profile is still TGC252_Volunteers_0004_20261028.
- Native chooser/forces panels, donor ownership and player command retained.
  Generated design detail: work/volunteers-0007/diplomacy-action-preview.png.
  Source screenshot: work/volunteers-0006/user-button-position-reference.png.

## Connected implementation

1. Exact local transport/configuration classification, rather than inferring SP
   from player count or loopback networking. Host/client/replay/unknown refuse
   simulation work. A native load-finalization fence supplies the runtime epoch;
   a date jump does not create a campaign identity.
2. Opt-in prototype installs guarded initial-pair, existing-side, land-access,
   local-vision and battle-country hooks. Until the whole hook set, session,
   queue, epoch and registry are admitted, callbacks preserve the native result.
   Ordinary armies retain native eligibility. Other threads do not borrow scope.
3. Send validates an idle donor land stack, neutral donor, nonhostile recipient
   at war, unique selected brigades, limit and a native land route. Army identity
   uses the proved native client allocation lock/counter path. Create plus all
   Transfers are queued together. Real ownership and affiliation remain donor.
4. Complete stack receipts activate scope. Both combat entry points retain
   unrelated land/unit/transport/exclusion checks. Duplicate hostility tests in
   the original functions are handled by their respective enclosing hooks.
   Recipient country context is added once to the native combat side, whose
   country vector is read by battle finalization to find the war. Actual army
   and regiment owners are never temporarily swapped.
5. Land access evaluates the original permission predicate against a private
   full-size input image. The original caller retains other movement gates.
   Native route output is bounded, checked for land and freed by the native
   cleanup. No teleport, naval routing or global military-access write.
6. Vision runs after native visibility recomputation. It raises only the local
   map-province display level for recipient-owned provinces. The province+174
   per-country simulation array is untouched. The native map texture is marked
   dirty. Vision survives return/loss until fresh recipient peace; native sight
   from all other sources is recomputed first.
7. Daily coordination waits through combat/movement, recalls on recipient peace
   or donor war, and uses native Move commands. Return tries a reachable donor
   land province, with a128-query bound. No route or refused work goes to review.
   Complete readable registry absence of army and all regiments on three distinct
   date samples marks loss. Read faults/cycles/changed membership are not death.
8. Mission intent is journaled before troop commands through CSetVariableCommand.
   Two bounded banks, SHA256 and a commit-last head protect interrupted writes;
   only changed variable words are queued. The engine saves these variables.
   Native commit observation validates/digests the journal. Load restores suspended,
   revalidates receipts and never replays Create/Transfer. A pending intent with
   no matching stack remains for review. Terminal vision restores independently
   of whether a lost army object still exists.

## Static native evidence

All entries below are S (inspected instructions), not R for this integration.
Ghidra preferred base0x400000; runtime uses loaded base + RVA. New raw pairs are
in ignored work/volunteers-0006/research; prior combat pairs remain in
work/volunteer-handoff-1026/research/functions. A timed-out decompile is not a C
specification:005A0180 and00662010 were reviewed from their complete assembly.

| Role | Ghidra VA | RVA | Exact contract/evidence |
|---|---|---|---|
| Diplomacy action builder |0x00622270|0x222270| action0..N direct native160x24 subwindows; grid position from parsed effective base |
| Native optional-window lookup |0x009E4A20|0x5E4A20| ECXwindow, stack string*, RET4; missing names log warnings |
| Subwindow vector readers |0x009E68A0 /0x009E6960|0x5E68A0 /0x5E6960| two stack args RET8; window+424 begin/+428 end,32-byte entries, value+1C. Bounded read-only enumeration avoids missing-name calls |
| Current manager getter |0x00651D10|0x251D10| ECXcontext -> [[context+17E8]+12C], EAX result, RET |
| Configuration copy |0x00648AF0|0x248AF0| config+6C transport kind copied; manager embedded config+114 -> kind at manager+180 |
| Manager construction |0x009AE5F0|0x5AE5F0| kind2 creates local queue via00A682C0, kind0/1 creates network variants; local ready state4; manager vtable RVA A390D0 |
| Local transport |0x00A682C0 /0x00A68560|0x6682C0 /0x668560| table RVA A42840, manager backlink+1C, null ENet host+18; post slot6 ECXqueue, stack command, RET4 |
| Load finalize |0x00662010|0x262010| CInGameIdler slot3, ECXcontext, no stack args, RET; first instruction MOV EBX,ECX; complete asm return inspected |
| Variable constructor |0x00583900|0x183900| stdcall (allocation,tag,index,string*,raw value), RET14; native clone caller00583C1E pushes allocation64 |
| Variable execute |0x005839C0|0x1839C0| ECXcommand, RET; country index+40; key+44; value+60 -> native CVariables country+1DC, node+1C |
| Variable read/write/valid |0x00583A30 /0x00583B60 /0x0072A5F0|0x183A30 /0x183B60 /0x32A5F0| native serialization and validity; command id49D/table RVA A0121C; execute slot6 |
| Initial combat pair |0x005CBF60|0x1CBF60| ECXleft,EAXright,stack province,AL result,RET4 |
| Existing battle side |0x005DBB50|0x1DBB50| EAXlist header,stack candidate,AL result,RET4 |
| Land slot33 |0x005D5E30|0x1D5E30| ECXarmy,stack province,AL result,RET4; land branch invokes005CD2F0 |
| Land permission |0x005CD2F0|0x1CD2F0| ECXarmy,stack province,RET4; only army+DC,+C0/+C4 read; delegates to0051B040 with EAXprovince/stack country |
| Land route |0x005B8500|0x1B8500| EAXstart,stack finder*,army,destination,path;RET10; CVerySafePathFind RVA A03794; cleanup00849E90/RVA449E90 |
| Battle country append |0x004D61D0|0xD61D0| EAXcountry-ref*,EDIvector,RET; remap only caller0059456E. Eight-byte private input buffer; duplicate recipient insertion skipped |
| Battle war attribution |0x005A0180|0x1A0180|005A0212..005A0238 read side+34 country pairs and relation+34; no full decompile claim |
| Visibility rebuild |0x0068C040|0x28C040| stack world,RET4; per-country province+174 vs selected-viewer map+360; native dirty path0068C51E..0068C53B |

The inline hooks steal only six complete address-free prologue bytes. A valid
fallback trampoline is populated/flushed before SDK redirection. DLL/originals
remain pinned if installation is partial; policy admission stays off. Pointer
patches retain P66 ownership/protection handling. No native maintenance skip,
owner swap, global fog toggle or direct simulation-variable write was added.

## OOS finding and actual limit

See report-volunteers-oos-1029.md. Maxioten's supplied source changes a local
registry in Send, then uses that local registry to skip native regiment/pop
maintenance. Native Create/Transfer commands do not replicate that registry.
This is a concrete peer-divergence mechanism. The particular screenshot has no
paired artifacts/logs/reproduction, so its actual cause is not established.
The new donor-owned prototype does not use that bypass and refuses MP sending.
It is not multiplayer support and does not claim to repair an existing save.

## Validation and remaining acceptance

Reference EXE SHA256 gate and80 scoped signature guards pass. cargo fmt,
i686 cargo check --all-targets, strict Clippy --all-targets -D warnings and
release compilation pass. Package ZIP CRC, payload/manifest and installed
hashes checked. Rust behavioral/native fixture/game/MP suites were not run;
the earlier request to code first and test later is retained. New session and
journal tests compile but are not behavioral evidence.

The first SP implementation is connected, but **full playable-prototype
acceptance is outstanding**: native callback ABI execution, actual SP detection,
Send receipt/command, both battle entry routes, war result, donor costs/control,
vision restoration, recall/loss, save/reload and clean exit. Static evidence
cannot rule out crashes or prove OOS safety. Only disposable saves are suitable.
Known limits: land-only deployment, provisional first-owned-land destination,
no naval transport automation, bounded journal/history/return search, interrupted
unmatched intent stays in review, MP unsupported. No external publication,
commit, memory update or desktop manipulation was performed.

Plugin SHA256: `43a4b944984b2f9690556937a5fdf46b82ed30be7ea1b6b0cd2beb8df4fa77b6`.

Private ZIP SHA256: `5d0ed71fe8451b106816f5aa43edef2f01609db56d786baee6bf82605e37a863`.
