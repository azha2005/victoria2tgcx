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


def build(output, dll=None, carrier=ACCEPTED_CARRIER):
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
    files['README.txt']=README.encode('ascii')
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
            'bytes':archive.stat().st_size,'files':len(files),'mode':'mp_probe',
            'mp_volunteer_gameplay':False,'actual_two_peer_test':'not run'}


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    args=parser.parse_args()
    print(json.dumps(build(args.output),indent=2))
