# Volunteers 0.0.14: native MP transport probe and replica diagnostics

2026-10-29. User explicitly requests multiplayer support and an OOS fix, or at
least an attempt. This checkpoint implements a runnable native-network probe
and deterministic journal mirror, not multiplayer volunteer gameplay. The
reported OOS has not been reproduced and no paired peer log/save was supplied.
No game or desktop input was used. No subagents or external messages were used.

## Finding and implementation boundary

The SP coordinator and published combat/access scope are local to the sending
player. Native Create/Transfer/Move commands alone cannot synchronize that
policy. Every peer needs the same policy activated at the same simulation
boundary, and automatic orders need one authority. P12/P69 remain applicable.
Donor ownership and ordinary native supply/maintenance remain required.

The new mode=mp_probe uses the native host/client queue to post one restricted
CSetVariableCommand per local country/load, marking a build/policy fingerprint
and monotonic sequence. Its native execute hook records the receipt and source
metadata on every peer that receives it. It writes only a tgcx_vl_mp1_ marker
through the engine, never unit fields, RNG, relations or simulation knowledge.
Posting a command is not a delivery acknowledgment. A failed or uncertain post
is not retried automatically; restart/load is an explicit new attempt.

This exercises a real native transport path in code, but that path has not been
tested over a live connection. It is not a custom registered command or an
authenticated volunteer request. The source slot is recorded, not interpreted
as proof of which country a peer may command. Fingerprints are consistency
diagnostics, not signatures, an all-peer handshake or permission to activate hooks.

## Native evidence (S = instructions inspected; no new R evidence)

RVA is relative to the loaded executable base; Ghidra preferred base is0x400000.
All addresses, offsets and RET cleanup sizes in the table are hexadecimal.
The exact reference executable gate passed before inspecting addresses.

| Entry | Ghidra VA / RVA | Evidence and ABI |
|---|---|---|
| Host/server queue table | E42728 / A42728 | S: slot6 = A62EA0, server.cpp path |
| Client queue table | E427C8 / A427C8 | S: slot6 = A66B30 |
| Host Post | A62EA0 / 662EA0 | S: ECX=queue, stack+4=command, void result; RET0x4 at A62F5E and A6318D. Assigns command+34 from manager+54 and command+38 local sequence on synchronous path |
| Client Post | A66B30 / 666B30 | S: ECX=queue, stack+4=command, void result; synchronous command+38 sequencing. Ghidra splits shared tails; follow A66CA5/A66D63 to RET0x4 at A66D79 |
| CSetVariableCommand ctor | 583900 / 183900 | S, previously established and re-exported: five stack parameters (allocation, country tag, country index, native string*, raw fixed-point value); EAX=command; RET0x14; allocation0x64 from native clone |
| Execute | 5839C0 / 1839C0 | S: ECX=command, RET; index+40, key+44, value+60; writes native CVariables through its existing implementation |
| Native ID / serialization / parsing | 583A20 /183A20; 583A30 /183A30; 583B60 /183B60 | S: ID49D; native save/network field serialization and parse, including country/key/value |
| Base command serialization | 9A2520 /5A2520 | S: serializes source short+34, asynchronous+36 and sequence+38 |

Raw .c/.asm.txt exports, table inspection, client-tail disassembly and logs stay
private under work/volunteers-mp-0014/research. NetSession_Service A61780, client
service A66D80 and related routines were also exported. Those broad decompilations
do not establish a complete authenticated peer/country map or all-peer membership
snapshot. No guessed field from them was added to the runtime.

Existing SP queue functions remain SP-only. The separate probe adapter checks
MP mode, exact native queue table and Post slot, explicit load epoch, local country,
one CSetVariableCommand, protocol key/value bounds and native validity. It checks
those properties again before transferring command ownership to the queue.
No SinglePlayerPermit is minted for MP and no MP troop command path is exposed.
The native wrappers retain existing CPU/SEH/last-error handling and byte guards.

## Mirroring and OOS evidence

mp_sync::Mirror consumes only engine-loaded, digest-validated journals. It stores
per-country generation/campaign/hash, sequence high-water and live army/regiment
IDs. Wrong donor/campaign, generation replay/conflict, sequence regression and
cross-country duplicate live objects refuse atomically. Identical repeated heads
are idempotent. Hashes have deterministic country order and omit local ASLR,
player, UI, load epoch and thread identifiers. This is read-only diagnostic state;
it never publishes native combat/access authority or emits automatic unit orders.

The native journal execute observer mirrors commits for any country in MP.
Load/late-join bootstrap scans up to16 country slots per game day independently
of local diplomacy UI and never replays a saved intent. Explicit load invalidates
the mirror. Samples report bootstrap completeness, refused/missed observations,
journal count and digest; incomplete/refused samples are not comparison proof.
The normal SP coordinator remains unchanged and SP0.0.13 ZIP remains retained.

The collector now extracts probe execution receipts and mirror samples from the
last explicit load segment. Comparison reports missing receipts, fingerprint
disagreements, ambiguous same-date samples and the first unambiguous journal
digest mismatch after complete bootstrap with zero refusals/missed observations.
It does not call an absent sample a match, compare local epochs as shared IDs,
equate mirror hashes to native CRCs, or establish the cause of the old screenshot.
Use matching common saves/runs and native OOS logs for further diagnosis.

## Checks and artifact

-57 Rust tests pass, two explicit timing benchmarks remain ignored. Fixtures
  exercise host/client table-slot mismatch, bounded marker decoding, independent
  commit order/epoch, stale/conflicting generations, foreign donor/duplicate
  objects, sequence regression and preserved MP Send refusal.
- Formatting, strict all-target Clippy and x86 release compilation pass.
- Six existing package/actual-loader fixtures pass. Two collector fixtures pass,
  including last-load slicing, missing evidence, ambiguity and mismatch detection.
- One new plain MP-package fixture checks reproducible ZIP bytes, CRC, manifest,
  one owned DLL, eight native-derived textures, additive descriptors and no
  EXE/BAT/CMD/PowerShell/scripts. Two retained SP-share fixtures also pass.
- Exact executable and105 scoped signature records pass (previously96).
- Plain artifact: work/volunteers-mp-0014/final/Volunteers-0.0.14-MP-sync-probe.zip.
  454764 bytes,19 files. SHA256:
  f6cf2858f1d0a919c3f70ee9ab898f2484f36cb89d6009bf1cb30d2dbec181e5.
- DLL SHA256:0542c12d68ce086d23745c47002ad27560efd67719ba8974a0b08fddb5f61e60.
- Installed E:/Victoria-II-Volunteers-0004 remains0.0.12; DLL hash remains
  08de457c5b49c12ce990d562a3c9ab1dc87be6ee766f7aa253702c768a00a1a3.
  No installation, loader/TGC/save change, game launch or two-peer test occurred.
  The new mode has no UI layout change; existing native art/layout is reused.

## Still required for MP volunteer gameplay

1. Establish peer membership and sender-to-country authority at native receive,
   including late joins, disconnects, shared countries, AI and country switching.
2. Implement deterministic mission replication and activation on the simulation
   thread, independently of local UI. A shadow mirror is not that activation.
3. Establish one synchronized authority for derived Create/Transfer/Move/Recall,
   ID reservation, save checkpoints and failure handling. Avoid per-peer polling
   that posts duplicate orders or resumes a partially queued split after load.
4. Prove the all-peer combat/access/siege scopes, save/reload and reconnect using
   paired traces. Then reproduce the old OOS and demonstrate matching simulation.

No CRC masking, owner swap, supply/maintenance bypass or force-enable-MP option
was added. The next concrete test is native probe delivery on two disposable
copies; instructions-volunteers-mp-sync.md contains all player/source steps.
