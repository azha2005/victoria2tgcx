"""Collect private host/client OOS evidence read-only; never fixes a running save."""
from pathlib import Path
import argparse
import hashlib
import json
import re
import shutil

ROOT = Path(__file__).resolve().parent.parent
WORK = (ROOT / 'work').resolve()
MAX_FILE = 32 * 1024 * 1024


def mp_trace(text):
    """Last explicit load only. Raw receipts are evidence, not peer admission."""
    text = text.rsplit('native_load_completed ', 1)[-1]
    probes = [{'date': int(d), 'country': int(c), 'sequence': int(s),
               'fingerprint': h, 'matches_local': same == 'true'}
              for d,c,s,h,same in re.findall(
                  r'mp_probe_execute date=Some\((\d+)\) country=(\d+) sequence=(\d+) '
                  r'fingerprint=([0-9a-f]{64}) matches_local=(true|false)', text)]
    samples = [{'date': int(d), 'journals': int(n), 'digest': h,
                'bootstrap_complete': complete == 'true',
                'refused': int(refused), 'missed': int(missed)}
               for d,n,h,complete,refused,missed in re.findall(
                   r'mp_sync_sample date=(\d+) journals=(\d+) digest=([0-9a-f]{64}) '
                   r'bootstrap_complete=(true|false) refused=(\d+) missed=(\d+)', text)]
    return {'probes': probes, 'samples': samples}


def compare_mp(a, b):
    def receipts(report):
        return {(r['country'],r['sequence'],r['fingerprint']) for trace in report.get('mp_traces', {}).values()
                for r in trace['probes']}
    def samples(report):
        result = {}
        for trace in report.get('mp_traces', {}).values():
            for row in trace['samples']:
                if row['bootstrap_complete'] and row['refused'] == row['missed'] == 0:
                    result.setdefault(row['date'], set()).add((row['journals'], row['digest']))
        return result
    left,right = receipts(a),receipts(b)
    sa,sb = samples(a),samples(b)
    common = sorted(set(sa) & set(sb))
    unambiguous = [day for day in common if len(sa[day]) == len(sb[day]) == 1]
    mismatch = [day for day in unambiguous if sa[day] != sb[day]]
    return {'common_execution_receipts': len(left & right),
            'receipts_only_host': [list(row) for row in sorted(left-right)],
            'receipts_only_client': [list(row) for row in sorted(right-left)],
            'fingerprint_refusals': {label: [r for trace in report.get('mp_traces', {}).values()
                                             for r in trace['probes'] if not r['matches_local']]
                                     for label,report in [('host',a),('client',b)]},
            'comparable_sample_dates': len(unambiguous),
            'ambiguous_sample_dates': [d for d in common if d not in unambiguous],
            'first_mirror_digest_mismatch': mismatch[0] if mismatch else None,
            'mirror_digest_mismatch_dates': mismatch,
            'limits': 'Compare the same save/run and exact dates. These are plugin journal digests, not full engine CRCs or multiplayer gameplay approval.'}


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def collect(game, output, profile=None, save=None):
    game = game.resolve()
    output = output.resolve()
    if not output.is_relative_to(WORK) or output == WORK:
        raise ValueError('private output must be a new folder beneath this repository work/')
    if output.exists() or not (game / 'v2game.exe').is_file():
        raise ValueError('requires an existing game and a new output folder')
    if output.is_relative_to(game) or game.is_relative_to(output):
        raise ValueError('evidence output must be separate from the game')
    artifacts = set()
    for name in ('v2game.exe', 'lua51.dll', 'lua5.1.dll', 'dinput8.dll',
                 'V2DLL/v2dll_core.dll', 'V2DLL/v2dll_settings.ini',
                 'z_CreatineCore/creatinecore.dll', 'z_CreatineCore/creatinecore_settings.ini'):
        if (game / name).is_file():
            artifacts.add(game / name)
    for pattern in ('mod/*.mod', 'mod/*/*.dll', 'V2DLL/Mods/*.dll',
                    'mod/*/tgcx_package.txt', 'mod/*/tgcx_volunteers.cfg',
                    'mod/*/v2dll_mod_volunteers.ini', 'mod/*/v2dll_mod_volunteers.state',
                    'z_CreatineCore/dll_Submods/*.dll', 'z_CreatineCore/dll_Submods/*.ini'):
        artifacts.update(p for p in game.glob(pattern) if p.is_file())
    if len(artifacts) > 1024:
        raise ValueError('artifact inventory exceeds the bounded collector')
    sources = [(p, 'game/' + p.relative_to(game).as_posix()) for p in artifacts]
    for folder, label in ((game / 'Logs', 'game-logs'),
                          (game / 'V2DLL/Logs', 'loader-logs'),
                          (game / 'z_CreatineCore/Logs', 'creatinecore-logs'),
                          ((profile / 'logs') if profile else None, 'profile-logs')):
        if folder and folder.is_dir():
            logs = sorted(p for p in folder.iterdir() if p.is_file() and p.suffix.lower() in ('.log', '.txt'))
            if len(logs) > 256:
                raise ValueError('log inventory exceeds the bounded collector')
            sources.extend((p, label + '/' + p.name) for p in logs)
    if save:
        if not save.is_file():
            raise ValueError('save path does not exist')
        sources.append((save, 'save/' + save.name))
    output.mkdir(parents=True)
    report = {'game': str(game), 'coverage': 'listed binaries/descriptors/settings/logs only, not a full mod checksum',
              'private': True, 'files': {}, 'skipped': [], 'registry_records': {},
              'journal_commits': [], 'mp_traces': {}}
    for source, name in sorted(sources, key=lambda item: item[1]):
        size = source.stat().st_size
        if size > MAX_FILE:
            report['skipped'].append({'name': name, 'reason': 'over 32 MiB', 'size': size})
            continue
        before = sha(source)
        # Copy only diagnostic text and optional saves. Game binaries remain
        # hash-only; their presence is not permission to redistribute them.
        copied = source.suffix.lower() not in ('.dll', '.exe', '.mod')
        if copied:
            dest = output / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, dest)
            if sha(dest) != before:
                raise ValueError('source changed during capture: ' + name)
        if sha(source) != before:
            raise ValueError('source changed during capture: ' + name)
        report['files'][name] = {'sha256': before, 'size': size, 'copied': copied}
        if source.name == 'v2dll_mod_volunteers.state':
            rows = [line.split() for line in source.read_text(encoding='cp1252').splitlines()
                    if line.strip() and not line.lstrip().startswith(';')]
            report['registry_records'][name] = sorted(rows)
        if source.name.lower() == 'tgcx_volunteers.log':
            text = source.read_text(encoding='utf-8', errors='replace')
            report['mp_traces'][name] = mp_trace(text)
            text = text.rsplit('native_load_completed ', 1)[-1]
            report['journal_commits'].extend(
                {'country': int(c), 'head': int(h), 'sha256': digest}
                for c, h, digest in re.findall(
                    r'journal_commit country=(\d+) head=(\d+) sha256=([0-9a-f]{64})', text))
    (output / 'evidence.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    return {'output': str(output), 'files': len(report['files']), 'skipped': len(report['skipped']),
            'scope': report['coverage']}


def compare(host, client):
    a = json.loads((host / 'evidence.json').read_text(encoding='utf-8'))
    b = json.loads((client / 'evidence.json').read_text(encoding='utf-8'))
    differences = []
    for name in sorted(set(a['files']) | set(b['files'])):
        if not name.startswith('game/'):
            continue
        x, y = a['files'].get(name), b['files'].get(name)
        if x is None or y is None or x['sha256'] != y['sha256']:
            differences.append(name)
    registry_available = bool(a['registry_records']) and bool(b['registry_records'])
    # Ignore installation folder names, but preserve all record values.
    left = sorted(a['registry_records'].values())
    right = sorted(b['registry_records'].values())
    def journal(report):
        result, conflicts = {}, []
        for record in report.get('journal_commits', []):
            key = (record['country'], record['head'])
            if key in result and result[key] != record['sha256']:
                conflicts.append(list(key))
            result[key] = record['sha256']
        return result, conflicts
    ja, ca = journal(a)
    jb, cb = journal(b)
    common = sorted(set(ja) & set(jb))
    return {'artifact_differences': differences,
            'mp_sync': compare_mp(a,b),
            'both_have_local_registry_evidence': registry_available,
            'local_registry_records_match': (left == right) if registry_available else None,
            'common_journal_commits': len(common),
            'journal_digest_mismatches': [list(key) for key in common if ja[key] != jb[key]],
            'journal_commits_only_host': [list(key) for key in sorted(set(ja) - set(jb))],
            'journal_commits_only_client': [list(key) for key in sorted(set(jb) - set(ja))],
            'conflicting_host_generations': ca, 'conflicting_client_generations': cb,
            'limits': 'Differences identify investigation targets. Matching files do not prove deterministic execution or establish the screenshot cause.'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_subparsers(dest='action', required=True)
    capture = actions.add_parser('collect')
    capture.add_argument('game', type=Path)
    capture.add_argument('output', type=Path)
    capture.add_argument('--profile', type=Path)
    capture.add_argument('--save', type=Path)
    diff = actions.add_parser('compare')
    diff.add_argument('host', type=Path)
    diff.add_argument('client', type=Path)
    args = parser.parse_args()
    result = collect(args.game, args.output, args.profile, args.save) if args.action == 'collect' else compare(args.host, args.client)
    print(json.dumps(result, indent=2))
