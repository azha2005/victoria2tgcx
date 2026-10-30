# Volunteers0.0.16: native MP gameplay candidate

2026-10-30. Implementation and startup validated; actual paired gameplay/OOS
acceptance remains open. User requests to test the game and retains control.

## Uploaded log and entry failure

The supplied Downloads/tgcx_volunteers.log is618 bytes:0.0.15, native_gui/load/
journal=false, probe_requested=true, prototype_requested=false. Its fingerprint
matches the correctly packaged0.0.15 DLL, so it does not establish a stale binary
or wrong mode. The earlier screenshot's different flags are separate evidence.

The full Native::new gate expected untouched RVA662EA0. Official Core6.36 installs
its own eight-byte host-role probe there (MP_HOST_HUMAN_SYNC=1). That concrete
source conflict rejected the whole GUI. The uploaded log did not identify its
first failing RVA, so the exact tester-side trigger is not directly recorded.

0.0.16 logs every mismatching signature and checks UI/lifecycle/journal/transport
separately. It accepts only the official Core6.36 DLL hash
 a36cc0339e83db2cab2d6c60047d84c1abf5e90b9fb267d54cbcee7d7191c5a1,
its exact role thunk/marker, relocated pointer, original eight-byte trampoline
and return jump plus unchanged native tail. No unknown inline patch is accepted.

## Static evidence (S; new fields have no host/client R acceptance yet)

All addresses are RVAs; Ghidra VA =00400000+RVA; runtime=loaded base+RVA.
Reference EXE SHA25662d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648.

- RVA28B470 (VA68B470) allocates44-byte human nodes and appends world+18/head,
  +1C/tail,+20/count; node+38 prev,+3C next. Constructors RVA24A460/1A100
  (VA64A460/41A100) establish node+1C player/source ID,+20 tag,+24 country index.
- RVA28B6C0/28B870 (VA68B6C0/68B870) maintain/check world+AEC/+AF0 indexed human
  controller table; nonzero means human,1 local. Local country world+B60 and
  manager+54 source must match exactly one list node and the country-manager tag/
  index. Cycles, duplicates, extra/missing humans, spectators or shared country
  fail. Initial limits2..32 active distinct-country players, country0 excluded.
- Native server Post RVA662EA0 / VAA62EA0 and client Post RVA666B30 / VAA66B30:
  ECX=queue, one command pointer stack word, RET4. Existing typed queue/vtables and
  native load fence retained. Source short+34, flags+36, sequence+38 are metadata.
- CSetVariableCommand Clone RVA183C00 / VA583C00: ECX=this, seven opaque by-value
  stack words, EAX=new command, RET1C at RVA183C7F / VA583C7F. Its constructor
  resets source+34 toFFFF; native network service copies sequence+38 but does not
  preserve source. Wrapper forwards the opaque blob once and preserves source
  ONLY between matching newly allocated tgcx_vl_ variable commands. Flags/sequence
  remain native. Both vtable-slot replacements must complete; rollback is logged.
  Do not infer the seven words' semantics from the decompiler signature.
- Official Core DLL preferred base10000000, image sizeD8D000, role thunk RVA12A80,
  marker RVA12A70, trampoline pointer RVA82EE50, private role flag RVA9931FC.
  These are exact-version compatibility checks, not new gameplay offsets.

Private raw C/ASM exports: work/volunteers-mp-0016/research. They remain outside
git with the game binary/Ghidra project/art, per R8. New signatures are relocated.

## MP design implemented

mode=mp is an explicit gameplay candidate. mode=mp_probe remains diagnostic only.
Each process reads the complete native human roster. Fresh executed mp2 fingerprint
markers from all roster members admit this load; saved markers alone do not. Source
metadata is cross-checked against the current source-country list. Probe mp1 and
SP mode cannot mint MP authority. These checks are not a claim of hostile-client
security; the native network remains responsible for transport authentication.

An issuer permit is local-country/current-load/current-roster-round only. It
cannot authorize another donor or a merged-policy write. Each donor's local client
alone runs its mission coordinator and sends journal/Move commands. Whole existing
armies only avoid a racing global ID allocator; partial MP Send is refused and
native allocation explicitly denied. Native ownership, supply/pops stay unchanged.

HEAD executes after journal words/length in the native command stream. All peers
load that committed bank; before every native hourly command they also revalidate
current membership/war into a shared policy. GUI state never publishes/withdraws MP
policy. Per-donor campaign IDs survive merging, even identical mission serials;
the merged view cannot be serialized or used for issuer actions. It is immutable
and shared by Arc; callbacks release the publication lock before native calls.
MP callbacks wait for publication rather than varying rules with try_read/thread
contention. New commands still require fresh full roster and native queue checks.

Uncommitted banks, foreign membership, missing receipts/roster or fingerprint
mismatch never authorize new scopes. No native maintenance/RNG/CRC bypass and no
Create/Transfer replay added. Disconnect/hotjoin behavior and actual native-order
source propagation remain unaccepted until paired execution is observed.

## Checks and actual startup

-70 Rust tests passed;2 ignored benchmarks. Format/strict all-target Clippy and
  x86 release passed. New fixtures: RET1C opaque ABI/canaries/CF/EAX/sender scope;
  host/client roster and malformed membership; two replica native-word journals,
  uncommitted HEAD exclusion, two donors/campaigns/serial collision, policy digest
  equality across local epochs/order, issuer/subset refusal and reload validation.
- Six stage/x86-loader tests, two MP-package and two OOS-collector tests pass.
 114 scoped signatures pass exact EXE gate. Final archive CRC/manifest verified;
  repeated builds produced the same SHA256. Eight native art textures retained.
- Copied25390 files /1.101GiB to a NEW workspace game-host copy, excluding old
  logs/Volunteers and command scripts. Accepted E: copy unchanged. New profile
  TGC252_Volunteers_MP0016_Host, Core role probe explicitly enabled only here.
- First restricted launch raised a native C++ file-open exception (RVA592A79,
  VA992A79 in stream opening); exact path not established. A subsequent launch
  with profile access created its unique profile and reached done frontend.
  The hidden idle menu instance was stopped before handing over the visible one;
  no save/campaign was loaded by the agent.
- Real startup log verifies the official existing role probe twice and reports
  gameplay/lifecycle/journal/transport=true, native_gui_installed=true,
  native_load_fence=true, native_journal_observer=true, all eight scoped hooks=true.
  This proves startup admission, not gameplay or pixel/button recovery acceptance.
- No desktop input sent. Game capture timed out; no stale/blind input followed.
  User then took over. Do not change the live test copy. No two-player session,
  Send/battle/siege/return/OOS test completed by the agent.

## Handoff

Final: C:\Users\JC\Downloads\victoria2template\work\volunteers-mp-0016\final\Volunteers-0.0.16-MP-prototype.zip
SHA256: f249acbde424144f52f013df16fbb1a2ff00e250851dc77513661d34c8318cd5
498115 bytes,19 files. mode=mp. No installer/EXE/BAT/CMD/PS1/scripts.
DLL SHA256: 801bf93763eaede58acd0b204584744018611d4b00db03eb1d6c5e40eadc6041
Running test copy and final ZIP DLLs are byte-identical. No loader replacement,
TGC distribution or third-party art/raw RE enters git. Source commit/push authorized;
no public binary release or messages to another person. Read the new exact paired
instructions. Original OOS remains unresolved until its reproduction passes.
