"""Plain, player-shareable Volunteers ZIP. No setup executable or launch scripts.

Reuses the checked 0.0.15 candidate; never installs, launches, downloads or publishes.
Game-derived UI pixels remain outside git, like the ATCU/PRU candidate packages.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parents[1]
VERSION = '0.0.15'
ACCEPTED_DLL = '974a306baf282a846bb8ff8d1648adf0223ae5030ff19c1ef1236647cb0dd4ef'
ACCEPTED_PACKAGE = ROOT / 'work/volunteers-entry-0015/candidate-r2/Volunteers-0.0.15-SP-entry-private'
ACCEPTED_CARRIER = ROOT / 'work/volunteers-loader-636/stage-636/mod/tgcx_loader_data/common/defines_ccore.txt'

README = '''VOLUNTEERS 0.0.15 - SINGLE-PLAYER BETA / ENTRY-RECOVERY CANDIDATE
========================================

Send up to five brigades to help a country at war while your own country stays
at peace. You retain ownership, command, flag and native supply/reinforcements.

REQUIRES
- Victoria II: Heart of Darkness 3.04, Windows x86.
- Reference v2game.exe SHA-256:
  62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648
- TGC 2.5.2. Other bases/versions are not validated for this prototype.
- CreatineCore Modding Toolkit 6.36 installed separately:
  https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension/releases/tag/v6.36

INSTALL (the game must be closed)
1. Extract this ZIP into your Victoria II folder, the one containing v2game.exe.
   Merge the mod folder. No executable, TGC file or loader is replaced by this ZIP.
   Back up an older Volunteers package before replacing it.
2. In z_CreatineCore/creatinecore_settings.ini set:
     LOCAL_MOD_CONFIG=0
     LOAD_DLL_FROM_MOD_FOLDER=1
     ENABLE_STATE_PROBE=0
     ENABLE_RNG_PROBE=0
     DEBUG_LOG=0
   In z_CreatineCore/dll_Submods/ccore_performance_patch.ini set:
     FIX_ARMY_WINDOW_LAG=0
     DEBUG_LOG=0
   Keep the performance plugin supplied with CreatineCore installed. These are
   the relevant loader/performance settings used in the accepted test copy;
   other loader settings, hardware and campaign state can affect results.
3. In the normal game launcher select these THREE mods:
     00 - Volunteers loader data
     your TGC 2.5.2
     ZZ - Volunteers (SP beta)
   The names arrange the carrier before TGC and Volunteers after TGC. Keep any
   other DLL submods unticked for the first test. Do not select Volunteers alone.
4. Start through the normal launcher. No BAT, CMD, PowerShell launcher or setup
   program is included. In the main menu CreatineCore should show version 6.36
   and the Volunteers plugin. Check the logs below if its button is absent.

WHY TWO VOLUNTEERS ENTRIES?
The tiny loader-data carrier gives CreatineCore its physical defines file without
letting its initializer rename/append files inside TGC. It has no gameplay DLL,
replace_path or save profile. Keep it FIRST. TGC remains before Volunteers.
The Volunteers package defaults to mode=prototype so the UI and Send are enabled
in an admitted single-player session. All executable/session/receipt gates apply.

USE
1. Use a disposable copy of a save for your first test.
2. Your country must be at peace. Open Diplomacy and choose a recipient at war.
3. Click Send volunteers beside the normal diplomatic action buttons.
4. The chooser lists idle donor armies with a verified land route. It searches
   in small batches; let game ticks pass while it says Checking reachable armies.
   Reopen or change recipient to refresh the search. Multiple recipient land
   destinations are tried, with current/nearby provinces prioritized.
   Pick an army and check up to five brigades, then Send volunteers.
   A native LAND ROUTE must exist to recipient territory. No teleport/sea
   deployment is implemented. Unreachable recipients are refused.
5. Unpause. Assembly verification and the movement order happen through native
   commands. Whole-army selection keeps the existing stack/leader; partial
   selection detaches the checked brigades into a dedicated stack.
6. Select and command the donor-owned army normally. The forces panel displays
   the mission. Recall selected requests a physical return when the army can move.
   Recipient peace or donor entry into war also requests return.
7. Completed missions are hidden by default. Show history / Hide history toggles
   them; Clear history removes completed records whose retained vision expired.
   Terminal records are automatically pruned after known recipient peace. They
   never consume the active brigade limit, including immediately after load.

0.0.15 BUTTON RECOVERY
- Deferred or off-thread diplomacy hides no longer permanently disable the UI.
- Restore an entry covered by the diplomacy window, preserving later dialogs.
- Restore hidden native button children and rebuild expired entry roots safely.
- Same native art, Send volunteers label and diplomacy action-grid placement.
  Test minimize/restore, reopening diplomacy, country changes and long sessions.
  These fixes pass source checks; the screenshot cause needs an in-game retest.

0.0.13 CHANGES RETAINED
- Saved-journal binding retries after transient read failure; restoring a journal
  resets the daily date/page state. Specific session messages replace the generic
  unavailable text. Exact cause of the tester's save/reload report is unconfirmed.
- New Send intents are rejected atomically if the save journal would overflow;
  existing missions remain intact. Cleared records do not reuse mission IDs.
- Vision now keeps independently valid province grants if another province is
  malformed, and includes native recipient-controller sight. It edits only local
  display data, never simulation knowledge. Full in-game sharing needs retesting.
- No artificial country-count/distance cap is added. Sea deployment is still absent.

KNOWN LIMITS
- SINGLE PLAYER ONLY. Multiplayer Send is deliberately refused. This is not an
  OOS-safe multiplayer release; matching DLL files alone do not make it one.
  For a read-only MP loading/UI baseline, both peers may set mode=preview in
  mod/tgcx_volunteers/tgcx_volunteers.cfg. Send/Recall stay unavailable. This does
  not test volunteer combat synchronization or establish multiplayer safety.
- Sending, player control, battles and siege progress were observed in earlier
  tests. The user accepted the CreatineCore 6.36 setup as working well enough.
  This 0.0.13 feedback candidate has passed automated checks, not a new game run.
  Completed occupation credit, full return/loss/load coverage and accounting
  acceptance are not established. Recipient-wide fog sharing is still incomplete.
- Existing exiled armies can need a normal return to accessible territory.
  Do not expect this mod to repair old incomplete send attempts automatically.
- Game speed varies. State/RNG probes must remain off for normal play; these
  diagnostic dumps previously produced hundreds of megabytes in short sessions.
- Save/reload restores the mission journal suspended for validation; interrupted
  commands are not automatically replayed. Keep a pre-mod save backup.

LOGS / TROUBLESHOOTING
- Logs/tgcx_volunteers.log: build 0.0.13 and native_gui_installed,
  native_load_fence, native_journal_observer, scoped_hooks all true.
- z_CreatineCore/Logs/v2dll.log: version 6.36, v2dll_mod.dll -> ok,
  ccore_performance_patch.dll -> ok, state/RNG probes disabled.
- Old V2DLL/Logs files are not the new loader's logs.
- If game arguments can be inspected, order must be carrier, TGC, Volunteers:
  -mod=mod/tgcx_loader_data.mod -mod=<your TGC descriptor> -mod=mod/tgcx_volunteers.mod
  Do not bypass a failed byte signature or use an unverified executable build.

REMOVE / ROLLBACK
Close the game normally. Untick both Volunteers entries and restore a pre-mod
save. Remove only mod/tgcx_volunteers.mod, mod/tgcx_volunteers/, the carrier
descriptor and mod/tgcx_loader_data/. Restore your backed-up loader settings if
desired. Do not remove CreatineCore if another mod needs it.

CONTENTS / CREDITS
This is a plain mod payload like ATCU/PRU. It contains our plugin, its native-style
UI and the tiny loader-data carrier. No loader distribution, game executable,
TGC distribution, saves, logs, Ghidra dumps, installer or command scripts.
The eight UI textures are derived from installed Victoria II/TGC assets; their
original rights remain with their owners. No new redistribution licence is
asserted. No third-party art or binary is committed to the source repository.
JC / azha2005: project. V2DLL/CreatineCore by artyom-kuznetsov and contributors:
plugin API and credited hook patterns. maxioten and Victoria2-Reverse-Engineering:
targeted native evidence and symbols. See the source repository for attribution:
https://github.com/azha2005/victoria2tgcx
'''

CHANGELOG = '''Volunteers 0.0.15 SP beta entry-recovery candidate - 2026-10-29
- Defer off-thread/reentrant host hides without permanently disabling the UI.
- Recover verified covered entry ordering, hidden controls and expired roots.
- Native art, label and action-grid layout retained; game retest pending.
- Rebind retry and reset runtime day/page state on loading a saved journal.
- Hidden/clearable completed history, post-peace pruning and monotonic IDs.
- Atomic journal-size refusal instead of partially accepting an overflowing Send.
- Batched reachable idle-army chooser with multiple destination checks.
- Independently validated province vision; controller sight; clearer session errors.
- New features require in-game retesting. No multiplayer Send unlock.
- Donor-owned/player-controlled volunteer prototype; native command journal.
- Scoped combat/access, initial/existing siege permission and war attribution.
- Native diplomacy entry and army-style chooser/forces windows.
- Reduced idle hook, observer and repeated GUI overhead; sampled timing logs.
- Packaged for the user-accepted CreatineCore 6.36 setup, installed separately.
- Plain mod ZIP, no installer or command/launcher scripts. Internal DLL version
  is 0.0.15, byte-identical to the regression-checked entry-recovery candidate.
- Full recipient vision, completed capture credit/lifecycle and MP remain open.
'''


def sha(data):
    return hashlib.sha256(data).hexdigest()


def build(output, candidate=ACCEPTED_PACKAGE, carrier=ACCEPTED_CARRIER):
    output, candidate, carrier = Path(output).resolve(), Path(candidate).resolve(), Path(carrier).resolve()
    archive = Path(str(output) + '.zip')
    if output.exists() or archive.exists():
        raise ValueError('output and ZIP must be new')
    manifest = json.loads((candidate / 'MANIFEST.json').read_text(encoding='ascii'))
    files = {}
    for name, digest in manifest.items():
        path = (candidate / name).resolve()
        if not path.is_relative_to(candidate) or sha(path.read_bytes()) != digest:
            raise ValueError('candidate integrity mismatch')
        if name.startswith('mod/'):
            files[name] = path.read_bytes()
    if len(files) != 15 or sha(files['mod/tgcx_volunteers/v2dll_mod.dll']) != ACCEPTED_DLL:
        raise ValueError('wrong accepted payload')
    files['mod/tgcx_volunteers.mod'] = b'name = "ZZ - Volunteers (SP beta)"\npath = "mod/tgcx_volunteers"\n'
    key = 'mod/tgcx_volunteers/tgcx_volunteers.cfg'
    config = files[key].decode('ascii')
    config, count = re.subn(r'(?m)^mode=observe\r?$', 'mode=prototype', config)
    if count != 1:
        raise ValueError('unexpected candidate mode')
    config = config.replace('# Conservative audit default: retain only the observer.',
                            '# SP beta: volunteer prototype enabled; all native admission gates apply.')
    files[key] = config.encode('ascii')
    files['mod/tgcx_loader_data.mod'] = b'name = "00 - Volunteers loader data"\npath = "mod/tgcx_loader_data"\n'
    files['mod/tgcx_loader_data/common/defines_ccore.txt'] = carrier.read_bytes()
    files['README.txt'] = README.encode('ascii')
    files['CHANGELOG.txt'] = CHANGELOG.encode('ascii')
    files['SHA256.json'] = (json.dumps({n:sha(b) for n,b in sorted(files.items())}, indent=2) + '\n').encode('ascii')
    for name in files:
        if Path(name).suffix.lower() in {'.exe','.bat','.cmd','.ps1','.py','.vbs','.js','.lnk'}:
            raise ValueError('forbidden setup/launcher content')
    output.mkdir(parents=True)
    for name, data in files.items():
        path=output/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(data)
    with zipfile.ZipFile(archive, 'x', zipfile.ZIP_DEFLATED) as z:
        for name,data in sorted(files.items()):
            entry=zipfile.ZipInfo(name,(2026,10,29,0,0,0));entry.compress_type=zipfile.ZIP_DEFLATED
            z.writestr(entry,data)
    with zipfile.ZipFile(archive) as z:
        if z.testzip() is not None or set(z.namelist()) != files.keys() or any(z.read(n) != b for n,b in files.items()):
            raise ValueError('ZIP integrity mismatch')
    result={'zip':str(archive),'bytes':archive.stat().st_size,'sha256':sha(archive.read_bytes()),'files':len(files),'plugin_sha256':ACCEPTED_DLL,'scripts':0,'installer':False,'loader_included':False,'mode':'prototype-SP-gated'}
    Path(str(archive)+'.sha256').write_text(result['sha256']+'  '+archive.name+'\n',encoding='ascii')
    Path(str(archive)+'.json').write_text(json.dumps(result,indent=2)+'\n',encoding='ascii')
    return result


if __name__ == '__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    args=parser.parse_args()
    print(json.dumps(build(args.output),indent=2))
