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
VERSION='0.0.18'
FOLDER='tgcx_volunteers'
SETTINGS='''# Conservative audit default: retain only the observer.
# mode=preview explicitly opts into the untested diplomacy UI.
# mode=prototype opts into the experimental SP path; all native gates apply.
mode=observe
# Policy limit; configuration cannot override a failed native gate.
max_brigades=5
# Log sampling cadence in executed tick commands, 1..30.
sample_every_days=7
'''
README='''Volunteers 0.0.18 development staging payload.

This private staging output defaults to observe; it is not the MP handoff.
Use tools/prepare_volunteers_mp.py --gameplay for the plain MP prototype ZIP
with the exact multiplayer installation and paired-test instructions.

Modes: observe, preview, prototype (SP), mp_probe (diagnostic only), mp
(experimental whole-army multiplayer gameplay). Every native/session/load gate
still applies. MP also requires fresh execution receipts from the complete human
roster, matching build/policy fingerprints and one local command issuer per donor.
The plugin uses native commands and saved country-variable journals; no local
sidecar grants simulation authority and no unit commands are replayed on load.

0.0.18 separates startup UI/gameplay signature groups, diagnoses mismatching RVAs,
recognizes transparent role probes by their bounded code/continuation contract, preserves our variable-command
clone sender metadata, and derives MP combat/access/siege policy at native command
boundaries. SP partial-army sending remains; MP reuses whole existing armies only.

The exact EXE remains mandatory:
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648
Requires TGC2.5.2 and CreatineCore6.36, installed separately. Never install over a
running game or the only campaign copy. No game/TGC/loader binary is supplied.
Native art remains private and outside git. Actual paired MP/OOS acceptance is
still pending. Matching policy digests are not the native engine CRC.
Logs/tgcx_volunteers.log records gates, peer receipts, commits and policy digests.
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
