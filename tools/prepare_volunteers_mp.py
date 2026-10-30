"""Plain MP sync-probe archive, no installer/scripts or MP gameplay unlock."""
import argparse
import hashlib
import json
from pathlib import Path
import tempfile
import zipfile
import prepare_volunteers as development
from prepare_volunteers_share import ACCEPTED_CARRIER

ROOT = Path(__file__).resolve().parents[1]
WORK = ROOT / f"work/volunteers-mp-{development.VERSION.replace('.', '')}"
README = f'''VOLUNTEERS {development.VERSION} - MULTIPLAYER SYNC PROBE (NOT MP VOLUNTEER GAMEPLAY)

This build tests native network delivery and records matching volunteer-journal
digests. It does NOT enable multiplayer Send, Recall, combat or siege. It does
not fix the reported OOS yet. Use the separate SP beta for single-player testing.

This package includes the 0.0.15 button recovery fixes: deferred background-thread
hides, native layer/child visibility repair and expired-window reconstruction.
Checkpoint failures retain the validated UI while mission commands stay fenced.
The native button should remain visible and open its panels in MP, even though
MP Send/Recall are unavailable. Test background/restore and reopening Diplomacy.

Both players need the reference Victoria II 3.04 x86 executable, TGC 2.5.2 and
CreatineCore 6.36 installed separately, with matching mods/settings/DLLs. EXE SHA:
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648
Core: https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension/releases/tag/v6.36

INSTALL (close the game normally first)
1. Back up your existing Volunteers folder. Extract this ZIP into your separate
   Victoria II test copy, beside v2game.exe. Merge mod/. It does not replace
   v2game.exe, TGC files or your loader. No setup/launcher scripts are included.
2. In z_CreatineCore/creatinecore_settings.ini use LOCAL_MOD_CONFIG=0,
   LOAD_DLL_FROM_MOD_FOLDER=1, ENABLE_STATE_PROBE=0, ENABLE_RNG_PROBE=0,
   DEBUG_LOG=0. In dll_Submods/ccore_performance_patch.ini keep
   FIX_ARMY_WINDOW_LAG=0 and DEBUG_LOG=0. Keep the current performance plugin.
3. Select 00 - Volunteers loader data, your TGC 2.5.2, and
   ZZ - Volunteers (MP sync probe) in the normal launcher on BOTH machines.
   Do not select the old Volunteers package at the same time.

TWO-PLAYER TEST
1. Use disposable copies and a clean common save. Retain the exact pre-test save.
   Test a baseline without Volunteers first if reproducing an existing OOS.
2. Host an ordinary MP game. Join from the other machine as another country.
   This package already sets mode=mp_probe. No console command is required.
3. Unpause briefly. Each local country attempts ONE native CSetVariableCommand
   per load. It writes only a tgcx_vl_mp1_<fingerprint> marker, not troop state.
   This intentionally exercises the actual native host/client command queues.
4. Each Logs/tgcx_volunteers.log should contain mp_probe_post and execution
   receipts mp_probe_execute for BOTH countries. A queued=true line alone is
   not proof of delivery. Both receipts must have matches_local=true.
5. Let several game days pass, then save/exit normally. Preserve BOTH logs and
   the common pre-test save. Send those files back through the user. If OOS
   occurs, retain the first OOS date and the loader's error/OOS logs too.
6. Journal bootstrap scans up to 16 country slots per game day. Compare mirror
   digests only after bootstrap_complete=true, refused=0 and missed=0, at the
   same dates. Empty-journal hashes can match while no volunteers exist; this
   proves no volunteer combat behavior. Daily digests are NOT full engine CRCs.
7. Repeat with host/client countries switched, a save/reload and reconnect.
   No Create/Transfer/Move or pending volunteer intent is replayed by this probe.
   A disconnected player or missing receipt is not treated as consent/readiness.

WHAT THIS CAN ESTABLISH
- Whether this native variable command reaches and executes on both peers.
- Whether exact build/policy fingerprints match at the observed execution.
- Whether committed per-country volunteer journals diverge, and the first
  unambiguous comparable game date. The source evidence collector can compare
  the two logs; it is deliberately not shipped as a command script here.

WHAT IS STILL MISSING FOR MP VOLUNTEERS
- Verified network sender-to-country authority and complete peer admission.
- Shared registry activation at a deterministic simulation boundary, before
  native combat/access uses it, independently of local UI scheduling.
- One authority for automatic unit orders; no per-peer duplicate Recall/Move.
- Paired gameplay, reconnect/late-join and OOS reproduction tests.
Matching fingerprints/journals alone do not establish these properties. Merely
removing the MP guard would reintroduce a concrete local-policy desync risk.

ROLLBACK
Close normally, restore your backed-up Volunteers package and its configuration.
Do not overwrite the only copy of a campaign with a probe save. This build has
not been tested in an actual two-peer game by the agent. The local installed
0.0.12/CreatineCore copy was not changed while preparing this ZIP.

CREDITS
JC / azha2005: project. Native UI pixels are derived from installed game assets;
their original owners retain their rights. No art or game binary enters git.
V2DLL/CreatineCore by artyom-kuznetsov and contributors supplies the loader/API.
maxioten and Victoria2-Reverse-Engineering supplied targeted evidence/symbols.
Source: https://github.com/azha2005/victoria2tgcx
'''


GAMEPLAY_README = f"""VOLUNTEERS {development.VERSION} - EXPERIMENTAL MULTIPLAYER PROTOTYPE

This package enables MP gameplay after fresh native execution receipts from ALL
human players using the same build/policy. Actual two-peer acceptance is pending;
it is not an OOS-fixed release. Use separate copies and a disposable common save.

REQUIRES
Victoria II 3.04 x86, TGC 2.5.2, CreatineCore 6.36 installed separately.
Reference EXE SHA-256:
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648
Core: https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension/releases/tag/v6.36

INSTALL ON EVERY PLAYER'S COPY (game closed normally)
1. Back up the older Volunteers folder. Extract beside v2game.exe, merging mod/.
   No game executable, loader or TGC file is replaced. No scripts are included.
2. In z_CreatineCore/creatinecore_settings.ini use LOCAL_MOD_CONFIG=0,
   LOAD_DLL_FROM_MOD_FOLDER=1, ENABLE_STATE_PROBE=0, ENABLE_RNG_PROBE=0,
   DEBUG_LOG=0. In dll_Submods/ccore_performance_patch.ini keep
   FIX_ARMY_WINDOW_LAG=0 and DEBUG_LOG=0. Match all mods/settings on all peers.
3. Normal launcher: select 00 - Volunteers loader data, TGC 2.5.2,
   ZZ - Volunteers (MP prototype). Untick older Volunteers descriptors.
   This package already sets mode=mp. mode=mp_probe cannot unlock gameplay.

USE / PAIRED TEST
1. Host/join a clean common save as distinct countries. Start with two players.
   Spectators, shared-country control, hotjoin and disconnect recovery are not
   validated. Unknown/changed rosters suspend new command authority.
2. Unpause briefly. All logs need mp_gameplay_hello receipts from BOTH countries,
   accepted=true and ready=true. If waiting, preserve both logs instead of
   disabling checks. queued=true alone does not prove execution/admission.
3. The donor must be at peace; the recipient must be at war and nonhostile.
   Open Diplomacy, choose the recipient and click Send volunteers beside
   Command Units. The native art/button recovery fixes are retained.
4. Select EVERY brigade of an idle army of at most five brigades. This MP build
   sends WHOLE EXISTING ARMIES ONLY. Split a larger army with the native game
   controls first. A verified native LAND ROUTE to recipient territory is needed;
   no teleport, sea deployment or country-count limit is added.
5. Send, then unpause. The donor keeps ownership, normal player command, flag,
   supply and reinforcements. Only that donor's client emits its mission orders.
   Both peers derive combat/access/siege policy from committed native journals,
   independently of whether either player's diplomacy window is open.
6. Compare BOTH logs at the same game date: matching journal_commit head/SHA256
   and mp_policy_active digest. Test movement, battles, siege progress, Recall
   and automatic return at peace. A policy digest is not the engine's full CRC.
7. Repeat with host/client countries swapped, two simultaneous donors, then
   save/exit/reload. Fresh admission is required after every load; saved marker
   values alone do not unlock gameplay. Interrupted work is not replayed.
8. Preserve the exact pre-test save, BOTH tgcx_volunteers.log files, loader logs
   and first OOS date. An ordinary baseline without Volunteers helps distinguish
   existing loader/mod desyncs. All peers must have identical game/mod builds.

LIMITS / TROUBLESHOOTING
- This is an experimental MP gameplay candidate; actual paired acceptance is pending.
- Whole-army reuse avoids unsynchronized new army-ID allocation. No partial Send.
- No local maintenance/RNG/CRC bypass is added. Original OOS is not yet proven fixed.
- Completed history is hidden/clearable and does not consume the active limit.
- Logs/tgcx_volunteers_<PID>.log must show {development.VERSION}, native_gui_installed,
  native_load_fence, native_journal_observer, scoped_hooks all true, plus
  mp_gameplay_requested=true. Startup now lists exact mismatching RVAs.
-0.0.17 recognizes transparent host-role probes by instruction/continuation
  contracts, including relinked modules, direct JMP veneers and stacked probes.
  No exact loader hash or private DLL RVA is required for this contract. Existing
  hooks are left in place; unrelated plugins do not need to be disabled merely
  because they are loaded. Unknown gameplay-changing hooks still refuse.
-0.0.18 checks the parallel WORLD country/controller vectors against each other;
  the separate global manager size is used as an index bound, not their length.
  Every roster field/identity/source check still applies. Accepted/refused roster
  stages are logged explicitly, so waiting does not hide a local read refusal.
- If the player roster cannot be established, Send stays refused and the panel
  identifies its stage. This is a correction/diagnostic candidate, not proof that
  the tester's silent roster refusal or original OOS is fixed.
- Each process now keeps a complete Logs/tgcx_volunteers_<PID>.log. Send BOTH
  per-process logs, beginning at startup. The old tgcx_volunteers.log is retained
  for convenience but can mix output when instances share a game folder.
- On conflict the log reports native_hook_chain_refused with reason/module path,
  and the UI says compatibility failed instead of falsely waiting for peers.
  Sharing a version label does not establish binary or configuration equality.
- For two local instances use SEPARATE game directories and save profiles. The
  earlier shared log interleaved two worlds/players and cannot prove paired order.
- Missing native gates can leave the button usable for diagnostics while Send
  stays unavailable. Do not remove signature/session/peer checks.

ROLLBACK
Close normally, restore the old Volunteers folder/descriptor/configuration and
use the untouched pre-test save. The previous SP package remains a rollback.

CREDITS
JC / azha2005: project. V2DLL/CreatineCore by artyom-kuznetsov and contributors:
loader/API and credited contracts. maxioten / Victoria2-Reverse-Engineering:
targeted evidence/symbols. Native UI art belongs to the original game owners;
no art/game binary enters git and no new rights are asserted.
Source: https://github.com/azha2005/victoria2tgcx
"""


def build(output, dll=None, carrier=ACCEPTED_CARRIER, gameplay=False):
    output = Path(output).resolve()
    archive = Path(str(output)+'.zip')
    if output.exists() or archive.exists():
        raise ValueError('output and ZIP must be new')
    dll = Path(dll or ROOT/'rust/volunteers/target/i686-pc-windows-msvc/release/tgcx_volunteers.dll')
    WORK.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=WORK) as temporary:
        candidate=Path(temporary)/'candidate'
        development.build(candidate,dll)
        files={p.relative_to(candidate).as_posix():p.read_bytes()
               for p in (candidate/'mod').rglob('*') if p.is_file()}
    files['mod/tgcx_volunteers.mod']=b'name = "ZZ - Volunteers (MP sync probe)"\npath = "mod/tgcx_volunteers"\n'
    files['mod/tgcx_volunteers/tgcx_volunteers.cfg']=(
        '# Opt-in MP command-delivery diagnostic; troop gameplay stays refused.\n'
        'mode=mp_probe\nmax_brigades=5\nsample_every_days=7\n').encode('ascii')
    files['mod/tgcx_loader_data.mod']=b'name = "00 - Volunteers loader data"\npath = "mod/tgcx_loader_data"\n'
    files['mod/tgcx_loader_data/common/defines_ccore.txt']=Path(carrier).read_bytes()
    if gameplay:
        files['mod/tgcx_volunteers.mod']=b'name = "ZZ - Volunteers (MP prototype)"\npath = "mod/tgcx_volunteers"\n'
        files['mod/tgcx_volunteers/tgcx_volunteers.cfg']=b'# Experimental multiplayer gameplay; every peer needs this exact build.\nmode=mp\nmax_brigades=5\nsample_every_days=7\n'
    files['README.txt']=(GAMEPLAY_README if gameplay else README).encode('ascii')
    files['SHA256.json']=(json.dumps({k:hashlib.sha256(v).hexdigest() for k,v in sorted(files.items())},indent=2)+'\n').encode('ascii')
    output.mkdir(parents=True)
    with zipfile.ZipFile(archive,'x',zipfile.ZIP_DEFLATED) as z:
        for name,data in sorted(files.items()):
            target=output/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
            entry=zipfile.ZipInfo(name,(2026,10,29,0,0,0));entry.compress_type=zipfile.ZIP_DEFLATED
            z.writestr(entry,data)
    with zipfile.ZipFile(archive) as z:
        if z.testzip():raise ValueError('ZIP CRC failed')
    return {'zip':str(archive),'sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),
            'bytes':archive.stat().st_size,'files':len(files),'mode':'mp' if gameplay else 'mp_probe',
            'mp_volunteer_gameplay':gameplay,'actual_two_peer_test':'not run'}


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    parser.add_argument('--gameplay',action='store_true',help='Explicit experimental MP gameplay package')
    args=parser.parse_args()
    print(json.dumps(build(args.output,gameplay=args.gameplay),indent=2))
