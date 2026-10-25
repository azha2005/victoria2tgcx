"""Package the checked Province Rename payload with its release README.

Never installs, starts the game, commits, pushes or publishes. Release pixels
and binaries remain under ignored work/. The native payload is byte-identical
to the supplied candidate ZIP; only README.txt changes.
"""
import argparse
import hashlib
import shutil
import json
from pathlib import Path, PurePosixPath
import subprocess
import zipfile

import prepare_province_rename as package
from tgcx_pe import PE

ROOT = Path(__file__).resolve().parents[1]
# Public PRU version; package.VERSION stays the internal build (like ATCU).
RELEASE = '0.0.1'
DOCS = ROOT / f'docs/release-pru-{RELEASE}'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def build(candidate, output, images=()):
    candidate, output = Path(candidate).resolve(), Path(output).resolve()
    if output.exists():
        raise ValueError('release output directory must be new')
    with zipfile.ZipFile(candidate) as source:
        names = source.namelist()
        if source.testzip() is not None or len(names) != len(set(names)):
            raise ValueError('invalid candidate ZIP')
        for name in names:
            path = PurePosixPath(name)
            if path.is_absolute() or '..' in path.parts or ':' in name or '\\' in name:
                raise ValueError('unsafe candidate member')
        files = {name: source.read(name) for name in names}
    manifest_path = candidate.with_name(candidate.stem + '-manifest.json')
    manifest = json.loads(manifest_path.read_text())
    if {name: sha(data) for name, data in files.items()} != manifest:
        raise ValueError('candidate manifest mismatch')
    prefix = 'mod/tgcx_province_rename/'
    if len(files) != 12 or any(name not in ('README.txt', 'mod/tgcx_province_rename.mod')
                              and not name.startswith(prefix) for name in files):
        raise ValueError('unexpected candidate inventory')
    if files[prefix + 'tgcx_package.txt'].decode().splitlines() != [
            'package = rename', f'version = {package.VERSION}']:
        raise ValueError('candidate version mismatch')
    dll = ROOT / 'dll/province_rename/Release/tgcx_province_rename.dll'
    if PE(dll).machine != 0x14C or files[prefix + 'v2dll_mod.dll'] != dll.read_bytes():
        raise ValueError('candidate does not contain current Win32 build')
    for name in ('README.txt', 'CHANGELOG.txt'):
        files[name] = DOCS.joinpath(name).read_text(encoding='ascii').replace(
            '\r\n', '\n').replace('\n', '\r\n').encode('ascii')
    output.mkdir(parents=True)
    archive = output / f'PRU-{RELEASE}.zip'
    with zipfile.ZipFile(archive, 'x', zipfile.ZIP_DEFLATED) as dest:
        for name, data in sorted(files.items()):
            entry = zipfile.ZipInfo(name, (2026, 10, 21, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            dest.writestr(entry, data)
    with zipfile.ZipFile(archive) as check:
        if check.testzip() is not None or {n: check.read(n) for n in check.namelist()} != files:
            raise ValueError('written release ZIP verification failed')
    digest = sha(archive.read_bytes())
    (output / (archive.name + '.sha256')).write_text(f'{digest}  {archive.name}\n', encoding='ascii')
    hashes = {name: sha(data) for name, data in files.items()}
    (output / 'SHA256.json').write_text(json.dumps(hashes, indent=2) + '\n')
    provenance = {
        'entry': ['GFX_bp_button_rename: native28x28 resized20x20'],
        'button': ['GFX_button_standard_144: native144x34'],
        'primary': ['GFX_button_standard_144: native144x34'],
        'field': ['GFX_bp_textedit_bg: top/bottom braid crops, nine-slice296x30'],
        'panel': ['GFX_unit_bottom_bg: parchment/bottom border crops',
                  'GFX_unit_toppanel_bg: green/gold header crops'],
    }
    (output / 'PROVENANCE.json').write_text(json.dumps({
        'art_preparer': 'tools/prepare_province_rename.py:artwork',
        'derived_game_assets': provenance,
        'credits': 'README.txt', 'project_license': 'not selected',
        'v2dll_binary_included': False, 'base_gui_or_data_included': False,
    }, indent=2) + '\n')
    sources = [ROOT / 'tools/prepare_province_rename.py',
               ROOT / 'tools/prepare_province_rename_release.py',
               ROOT / 'tools/tgcx_rename_signatures.py']
    sources += sorted((ROOT / 'dll/province_rename').glob('*.cpp'))
    sources += sorted((ROOT / 'dll/province_rename').glob('*.h'))
    identity = {
        'version': RELEASE, 'build': package.VERSION, 'zip': str(archive), 'sha256': digest,
        'bytes': archive.stat().st_size, 'files': len(files),
        'candidate_sha256': sha(candidate.read_bytes()),
        'payload_identical_to_candidate': True, 'changed_members': ['README.txt', 'CHANGELOG.txt'],
        'checkout_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
        'source_hashes': {p.relative_to(ROOT).as_posix(): sha(p.read_bytes()) for p in sources},
        'source_commit': None, 'publication_destination': None, 'published': False,
        'gameplay': 'TGC2.5.2/V2DLL5.98 short0.2.5 accepted; 0.2.8 ABC placement confirmed in game 2026-10-25',
        'multiplayer': 'untested', 'atcu_coexistence': 'untested',
    }
    # Release-page assets beside the ZIP: notes, readme, changelog and the
    # player's screenshots, copied unchanged.
    for name in ('RELEASE_NOTES.md', 'README.txt', 'CHANGELOG.txt'):
        shutil.copyfile(DOCS / name, output / name)
    for image in images:
        shutil.copyfile(image, output / Path(image).name)
    identity['assets'] = {path.name: sha(path.read_bytes()) for path in sorted(output.iterdir())
                          if path.name != 'RELEASE.json'}
    (output / 'RELEASE.json').write_text(json.dumps(identity, indent=2) + '\n')
    return identity


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('candidate', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--image', type=Path, action='append', default=[])
    args = parser.parse_args()
    print(json.dumps(build(args.candidate, args.output, args.image), indent=2))
