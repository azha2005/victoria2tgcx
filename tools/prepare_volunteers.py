"""Stage the independent volunteer development plugin. Never installs it."""
from pathlib import Path
import argparse
import hashlib
import json
import shutil
import zipfile
from tgcx_pe import PE
from tgcx_volunteers_gui import generate

ROOT=Path(__file__).resolve().parent.parent
VERSION='0.0.11'
FOLDER='tgcx_volunteers'
SETTINGS='''# Conservative audit default: retain only the observer.
# mode=preview explicitly opts into the untested diplomacy UI.
# mode=prototype opts into the experimental SP path; all native gates apply.
mode=observe
# Policy limit; configuration cannot override a failed native gate.
max_brigades=5
# Log sampling cadence in executed tick commands, 1..30.
sample_every_days=1
'''
README='''Volunteers - experimental SP prototype 0.0.11 - GAME CHECK PENDING

STATUS: sending, combat and siege progress are confirmed by the user on0.0.10.
0.0.11 optimizes negative filters for unrelated units and closed-panel mission
snapshots. Fresh SP/load/ownership/receipt checks remain for relevant units.
Controlled Win32/x86 helper benchmarks improve; real game FPS/timing, completed
capture credit, save/reload and multiplayer acceptance are not established.
MP sending remains refused.

INSTALL:
Use the isolated E:/Victoria-II-Volunteers-0004 copy with TGC2.5.2 and
V2DLL5.98. The game must be closed normally before updating the package.
Keep the previous package backup. This ZIP itself does not install anything.
For another prepared copy, put mod/tgcx_volunteers.mod and mod/tgcx_volunteers/
under its mod folder. V2DLL LOAD_DLL_FROM_MOD_FOLDER=1 loads v2dll_mod.dll.
Do not put duplicate plugins in another loader folder or replace PRU/ATCU.
Native game-derived textures are private; redistribution rights are not granted.

MODES in mod/tgcx_volunteers/tgcx_volunteers.cfg:
mode=observe (default): existing command-tick observer only.
mode=preview: native diplomacy UI, explicit load fence and journal observer;
Send/Recall remain unavailable and no unit/variable commands are queued.
mode=prototype: opts into the experimental SP gameplay path. All exact EXE,
byte/vtable, session, load epoch, command queue and registry gates still apply.
An unknown, host, client or replay session cannot admit prototype sending.
No configuration switch overrides a failed native gate.

USE:
Open Diplomacy. The native thin 160x24 button captioned Send volunteers fills
the vacant action-grid cell beside Command Units in the supplied reference.
Choose a recipient at war, an idle donor army and up to five brigades.
The donor must be at peace and nonhostile to the recipient. Send first checks
a native land route. The prototype picks the first recipient-owned land
province; it refuses unreachable choices rather than teleporting troops.
Whole-army selection keeps the existing stack and leader. Partial selection
queues Create plus Transfers together as native commands. A complete
receipt is required before the dedicated stack borrows recipient war/access.
The player keeps selecting/moving it through the ordinary army controls.

IMPLEMENTED PROTOTYPE PATH:
Donor army/regiment ownership, command, flag, supply, reinforcements and rank
accounting remain native. Only registered land-stack combat eligibility and
access borrow recipient context. Both initial and existing-battle gates are
covered, including native unrelated checks and actual membership receipts.
The native battle side records the recipient once for war attribution.
Local map vision follows recipient knowledge, including foreign fronts, plus
receipt-validated volunteer surroundings. The
per-country simulation visibility array is untouched. Native visibility is
recomputed first. The entitlement survives return/loss until known peace.
Recall and recipient peace request physical return after combat/movement.
A native land-route check selects a reachable home province when possible.
A stranded or refused command goes to review; there is no sea/teleport fallback.
Confirmed absence of both stack and regiments across three different date
samples marks loss. Read faults, cycles and changed membership are not death.

PERSISTENCE:
Mission intent is queued through CSetVariableCommand before troop commands.
A bounded two-bank, checksummed country-variable journal commits its head last.
It is saved with the engine state. Only mission-state changes are checkpointed;
unchanged daily observations do not queue variable commands. Load uses
an explicit native finalization fence, restores suspended and validates real
stack receipts; it never automatically replays Create or Transfer. An
interrupted intent without a matching stack remains for review.
Commit logs include country, generation and SHA256 for OOS comparison.
No local orders file or unsynchronized maintenance-skip hook is used.

PERFORMANCE / DIAGNOSTICS:
The shared country-vector hook bypasses unrelated callers before Rust/FXSAVE.
Scalar reads use a tested SEH boundary without heap allocations/kernel calls.
Visibility bulk-reads the province pointer table and only writes local display data.
Daily scope_counters distinguish targeted/bypassed calls, permission grants,
visibility grants and wrong-thread refusal. These are diagnostics, not proof that
all lag or combat failures are solved. Existing exiled armies may need normal
return to accessible territory; use a pre-send save for a clean deployment check.

MULTIPLAYER / OOS:
MP sending is unsupported and refused. Matching DLLs or journal digests alone
are not multiplayer proof. Maxioten's source contains a local-only registry
that changes native maintenance execution, a concrete divergence mechanism.
Its screenshot's actual cause has not been reproduced or established.
Use tools/collect_volunteers_oos.py to collect/compare private peer evidence.
Do not deploy this experimental prototype on the only copy of a campaign.
Logs/tgcx_volunteers.log and V2DLL/Logs record native admission/failures.
'''


def build(output:Path,dll:Path):
    output=output.resolve();archive=Path(str(output)+'.zip')
    if output.exists() or archive.exists():raise ValueError('output and ZIP must be new')
    pe=PE(dll)
    if pe.machine!=0x14C or 'V2Mod_Init' not in pe.exports() or f'TGCX Volunteers {VERSION} development'.encode('ascii') not in dll.read_bytes():
        raise ValueError('requires matching x86 volunteer DLL')
    package=output/'mod'/FOLDER;package.mkdir(parents=True)
    (output/'mod'/f'{FOLDER}.mod').write_text('name = "Volunteers - development preview"\npath = "mod/tgcx_volunteers"\n',encoding='cp1252')
    (package/'tgcx_package.txt').write_text(f'package=volunteers\nversion={VERSION}\n',encoding='ascii')
    (package/'tgcx_volunteers.cfg').write_text(SETTINGS,encoding='ascii')
    shutil.copyfile(dll,package/'v2dll_mod.dll')
    generate(package)
    (output/'README.txt').write_text(README,encoding='ascii')
    paths=sorted(p for p in output.rglob('*') if p.is_file())
    manifest={p.relative_to(output).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
    (output/'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='ascii')
    paths.append(output/'MANIFEST.json')
    with zipfile.ZipFile(archive,'x',zipfile.ZIP_DEFLATED) as z:
        for p in paths:
            entry=zipfile.ZipInfo(p.relative_to(output).as_posix(),(2026,10,27,0,0,0));entry.compress_type=zipfile.ZIP_DEFLATED
            z.writestr(entry,p.read_bytes())
    with zipfile.ZipFile(archive) as z:
        if z.testzip():raise ValueError('ZIP CRC failed')
        for p in paths:
            if z.read(p.relative_to(output).as_posix())!=p.read_bytes():raise ValueError('ZIP payload mismatch')
    digest=hashlib.sha256(archive.read_bytes()).hexdigest()
    Path(str(archive)+'.sha256').write_text(f'{digest}  {archive.name}\n',encoding='ascii')
    return {'zip':str(archive),'sha256':digest,'files':len(paths),'mode':'observe-default-preview-or-prototype-opt-in-SP-gated','validation':'regression-loader-package-checks-game-check-pending'}

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    parser.add_argument('--dll',type=Path,default=ROOT/'rust/volunteers/target/i686-pc-windows-msvc/release/tgcx_volunteers.dll')
    args=parser.parse_args();print(json.dumps(build(args.output,args.dll),indent=2))
