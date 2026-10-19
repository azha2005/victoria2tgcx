"""Prepare the release ZIP from the immutable, gameplay-tested Army ZIP.

Maintainer tool only. Never installs, launches the game, downloads or publishes.
The ZIP holds README.txt, the root loader and the mod folder; nothing else.
The hash manifest, release identity and art provenance are written beside it.
"""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path, PurePosixPath
import zipfile

ROOT = Path(__file__).resolve().parents[1]
VERSION = '0.0.2'
BUILD = '0.11.2'
NAME = 'ATCU - Army Template and Conform Universal'
# work/orders-0112/ATCU-orders-0.11.2-test.zip, the build installed and checked in game.
ACCEPTED_ZIP = '42cb2b3bb07f4a8b81e2c3eb906590b0270660d3fa5ebc10c05037a1f6096fae'
LOGIC = '7afefc61427be3c6dae12bd8004cdadba2b8c8866fd3cddb9c17dab76eb2c49e'
LOADER = 'ab8e3653a8ea697fa29e05907659b13a69af43c223d0f50116fa58865d4b36dd'
ART_COUNT = 32
DOCS = 'docs/release-' + VERSION
# The DLL and package sources must be committed: the release names that commit.
SOURCE_PATHS = ('dll', 'tools')
PACKAGE = 'mod/tgcx_army_universal/'
DESCRIPTOR = 'mod/tgcx_army_universal.mod'
README = 'README.txt'
SIDECARS = ('SHA256.json', 'RELEASE.json', 'PROVENANCE.json')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def json_bytes(value):
    return (json.dumps(value, indent=2, ensure_ascii=True) + '\n').encode()


def archive_path(output):
    return Path(str(output) + '.zip')


def read_archive(path):
    with zipfile.ZipFile(path) as z:
        if z.testzip() is not None:
            raise ValueError('ZIP CRC failed')
        entries = [i for i in z.infolist() if not i.is_dir()]
        names = [i.filename for i in entries]
        if len(names) != len(set(names)):
            raise ValueError('duplicate ZIP entries')
        for entry in entries:
            name = entry.orig_filename
            parts = PurePosixPath(name)
            if chr(92) in name or ':' in name or parts.is_absolute() or '..' in parts.parts:
                raise ValueError('unsafe ZIP path')
        return {name: z.read(name) for name in names}


def check_manifest(files, manifest=None):
    if manifest is None:
        manifest = json.loads(files['SHA256.json'])
        files = {name: data for name, data in files.items() if name != 'SHA256.json'}
    if set(manifest) != set(files):
        raise ValueError('manifest inventory mismatch')
    if any(sha(files[name]) != digest for name, digest in manifest.items()):
        raise ValueError('manifest digest mismatch')


def art_origin(name):
    key = Path(name).stem
    if key.startswith('tgcx_cf_'):
        leaf = key[len('tgcx_cf_'):]
        code = 'tools/tgcx_conform_gui.py:generate'
        if leaf == 'ring':
            return 'original drawn UI', ['Pillow rectangle drawing'], code
        if leaf in ('check', 'check_on'):
            return ('game/effective-base derived UI', ['gfx/interface/checkbox_default.dds'],
                    'tools/tgcx_army_art.py:filter_checkbox')
        if leaf in ('slot', 'change'):
            sprites = ['GFX_unit_entry_bg']
        elif leaf == 'button':
            sprites = ['GFX_unit_gunboat_diplo_button']
        elif leaf in ('left', 'right'):
            sprites = ['GFX_button_arrow_' + leaf]
        elif leaf == 'open':
            sprites = ['GFX_unit_disband', 'GFX_unit_strip']
        else:
            sprites = ['GFX_unit_bottom_bg', 'GFX_unit_toppanel_bg', 'GFX_unit_bg_28']
        return 'game/effective-base derived UI', sprites, code
    leaf = key[len('tgcx_tpl_'):]
    code = 'tools/tgcx_templates_gui.py:generate'
    if leaf in ('check', 'check_on'):
        return ('game/effective-base derived UI', ['gfx/interface/checkbox_default.dds'],
                'tools/tgcx_army_art.py:filter_checkbox')
    if leaf in ('bg', 'pager', 'ring', 'tabsel') or leaf.startswith('blank_'):
        return 'original drawn UI', ['Pillow drawing; tools/tgcx_bill_gui.py:section'], code
    if leaf in ('bar', 'header'):
        sprites = ['GFX_military_build_unit_bg']
    elif leaf in ('left', 'right'):
        sprites = ['GFX_button_arrow_' + leaf]
    elif leaf == 'recruit':
        sprites = ['GFX_button_standard_177']
    else:
        sprites = ['GFX_unit_strip', 'original drawn folder frame']
        if leaf == 'compact':
            code += '; tools/build_universal_army.py:build resize'
    return 'game/effective-base derived UI', sprites, code


def provenance(files, root):
    origins = []
    for name in sorted(files):
        if name.endswith('.dds'):
            category, inputs, generator = art_origin(name)
            origins.append({'path': name, 'sha256': sha(files[name]),
                            'category': category, 'generator': generator,
                            'source_sprite_or_method': inputs,
                            'exact_historical_base_ancestry': 'not fully recorded',
                            'publication_status': ('project license undecided' if category == 'original drawn UI'
                                                   else 'redistribution permission not recorded')})
    markers = []
    for path in sorted((root / 'dll/tgcx').rglob('*')):
        if path.suffix not in ('.h', '.cpp'):
            continue
        for line, text in enumerate(path.read_text(encoding='utf-8').splitlines(), 1):
            if 'from V2DLL:' in text:
                markers.append({'path': path.relative_to(root).as_posix(), 'line': line,
                                'attribution': text.strip()})
    return {'artwork': origins, 'v2dll_source_attribution': markers,
            'v2dll_provenance': 'Owner reports author supports open source and has been informed of use; retain attribution.',
            'license': 'undecided',
            'excluded': ['game executable', 'original Lua library', 'base mod distributions',
                         'V2DLL distribution', 'game logs/saves', 'Ghidra/decompiled dumps']}


def readme_bytes(root):
    # CRLF for Notepad, and independent of how git checked the file out.
    text = (root / DOCS / README).read_text(encoding='ascii')
    return text.replace('\r\n', '\n').replace('\n', '\r\n').encode('ascii')


def verify_release(files, manifest, identity):
    check_manifest(files, manifest)
    if {name.split('/')[0] for name in files} != {README, 'lua51.dll', 'mod'}:
        raise ValueError('unexpected top-level content')
    if sha(files[PACKAGE + 'tgcx.dll']) != LOGIC or files[PACKAGE + 'tgcx.dll'] != files[PACKAGE + 'v2dll_mod.dll']:
        raise ValueError('accepted logic changed')
    if sha(files['lua51.dll']) != LOADER:
        raise ValueError('accepted loader changed')
    expected = f'name = "{NAME}"\npath = "mod/tgcx_army_universal"\n'.encode('cp1252')
    if files[DESCRIPTOR] != expected:
        raise ValueError('ATCU descriptor mismatch')
    allowed_dll = {'lua51.dll', PACKAGE + 'tgcx.dll', PACKAGE + 'v2dll_mod.dll'}
    if {name for name in files if name.endswith('.dll')} != allowed_dll:
        raise ValueError('unexpected DLL')
    for name, data in files.items():
        if name.endswith(('.exe', '.py', '.ps1', '.bat', '.cmd', '.pdb', '.log', '.v2')) or 'lua51_real' in name:
            raise ValueError('unexpected recipient content')
    try:
        files[README].decode('ascii')
    except UnicodeDecodeError:
        raise ValueError('README.txt must be plain ASCII (no em dashes)')
    if any(PACKAGE + 'interface/' + name in files for name in ('buildunit.gui', 'unitpanel.gui', 'country_politics.gui')):
        raise ValueError('base GUI override present')
    if len([n for n in files if n.endswith('.dds')]) != ART_COUNT:
        raise ValueError('accepted art inventory changed')
    if (identity['version'], identity['internal_build'], identity['license']) != (VERSION, BUILD, 'undecided'):
        raise ValueError('release identity mismatch')


def source_commit(root):
    """HEAD, refused while the DLL or package sources differ from it."""
    git = ['git', '-C', str(root)]
    dirty = subprocess.run(git + ['status', '--porcelain', '--', *SOURCE_PATHS],
                           capture_output=True, text=True, check=True).stdout.strip()
    if dirty:
        raise ValueError('commit the DLL and tool sources first: ' + ' '.join(dirty.split()))
    return subprocess.run(git + ['rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()


def build(source, output, root=ROOT, commit=None):
    source, output, root = Path(source).resolve(), Path(output).resolve(), Path(root).resolve()
    if output.exists() or archive_path(output).exists():
        raise ValueError('output folder/ZIP must be new')
    if sha(source.read_bytes()) != ACCEPTED_ZIP:
        raise ValueError('input is not the accepted ' + BUILD + ' archive')
    accepted = read_archive(source)
    if 'SHA256.json' in accepted:
        check_manifest(accepted)
    else:   # test builds keep their manifest beside the ZIP
        check_manifest(accepted, json.loads(source.with_suffix('.SHA256.json').read_text()))
    files = {name: data for name, data in accepted.items() if name.startswith('mod/') or name == 'lua51.dll'}
    files[DESCRIPTOR] = f'name = "{NAME}"\npath = "mod/tgcx_army_universal"\n'.encode('cp1252')
    files[README] = readme_bytes(root)
    # The accepted ZIP's own README.txt is replaced, not payload.
    changed = [name for name in files if name != README and name in accepted and files[name] != accepted[name]]
    if set(changed) - {DESCRIPTOR}:
        raise ValueError('only launcher branding may change in accepted payload')
    identity = {
        'name': NAME, 'version': VERSION, 'internal_build': BUILD,
        'status': 'beta release', 'license': 'undecided',
        'gameplay_source_commit': commit or source_commit(root), 'accepted_archive_sha256': ACCEPTED_ZIP,
        'logic_sha256': LOGIC, 'loader_sha256': LOADER,
        'tested_executable_sha256': '62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648',
        'runtime_checks': ['TGC 2.5.2 + V2DLL 5.82 (filters, saved Conform orders across a load)',
                           'TGC 2.5.2 + ATCU loader (templates, Conform)'],
        'earlier_runtime_checks_0_10_4': ['vanilla', 'HPM 0.4.5', 'DoD Fan Fork', 'TGC 2.5.2 + V2DLL 5.82'],
        'changed_accepted_payload_files': changed,
        'pending': ['own license undecided by user choice', 'bundled artwork publication status',
                    'second-PC recipient test', '0.11.x filters not rechecked on vanilla, HPM or DoD'],
    }
    manifest = {name: sha(data) for name, data in sorted(files.items())}
    verify_release(files, manifest, identity)
    output.mkdir(parents=True)
    for name, data in files.items():
        path = output / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    archive = archive_path(output)
    with zipfile.ZipFile(archive, 'x', compression=zipfile.ZIP_DEFLATED) as z:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 10, 19, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            z.writestr(info, data)
    verify_release(read_archive(archive), manifest, identity)
    sidecars = {'SHA256.json': manifest, 'RELEASE.json': identity, 'PROVENANCE.json': provenance(files, root)}
    for name in SIDECARS:
        Path(str(output) + '.' + name).write_bytes(json_bytes(sidecars[name]))
    archive.with_suffix('.zip.sha256').write_text(sha(archive.read_bytes()) + '  ' + archive.name + '\n', encoding='ascii')
    return archive


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    print(build(args.source, args.output))
