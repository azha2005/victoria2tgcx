"""Build the files-only ATCU filter candidate for supported 3.04 bases."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import zipfile

from PIL import Image
import tgcx_templates_gui as tpl
import tgcx_conform_gui as cf

VERSION = '0.11.2'
MARKER = b'TGCX TPL-7 0.11.2 tgcx.dll'


def build(output, art, dll, loader):
    output, art, dll, loader = map(lambda p: Path(p).resolve(), (output, art, dll, loader))
    if output.exists() or Path(str(output)+'.zip').exists():
        raise ValueError('output folder/ZIP must be new')
    if MARKER not in dll.read_bytes():
        raise ValueError('filter package requires the matching 0.11.2 DLL and GUI')
    package = output/'mod/tgcx_army_universal'
    (package/'interface').mkdir(parents=True)
    (package/'gfx/interface').mkdir(parents=True)
    (package/'localisation').mkdir()
    for p in (art/'gfx/interface').glob('tgcx_*.dds'):
        shutil.copy2(p, package/'gfx/interface'/p.name)
    for name in ('tgcx_templates.gfx', 'tgcx_conform.gfx'):
        shutil.copy2(art/'interface'/name, package/'interface'/name)
    with Image.open(package/'gfx/interface/tgcx_tpl_tab.dds') as image:
        tpl.write_dds(image.convert('RGBA').resize((20,20)), package/'gfx/interface/tgcx_tpl_compact.dds')
    gfx = package/'interface/tgcx_templates.gfx'
    text = gfx.read_text(encoding='cp1252')
    text = text.rstrip()[:-1] + '\nspriteType={name="tgcx_tpl_compact" texturefile="gfx/interface/tgcx_tpl_compact.dds" noOfFrames=1 loadType="INGAME"}\n}\n'
    gfx.write_text(text, encoding='cp1252')
    templates, tloc = tpl.layout(universal=True)
    conform, cloc = cf.layout(universal=True)
    (package/'interface/tgcx_army.gui').write_text('guiTypes={\n'+templates+conform+'}\n', encoding='cp1252')
    loc = ['CODE;ENGLISH;FRENCH;GERMAN;POLISH;SPANISH;ITALIAN;SWEDISH;CZECH;HUNGARIAN;DUTCH;PORTUGUESE;RUSSIAN;FINNISH;x']
    loc += [k+';'+v+';'*13+'x' for k,v in (tloc|cloc).items()]
    (package/'localisation/tgcx_army.csv').write_bytes(('\n'.join(loc)+'\n').encode('cp1252'))
    (package/'tgcx_package.txt').write_text(f'package = army\nbase = universal\nmodules = "templates conform"\nversion = {VERSION}\n')
    (package/'tgcx_universal.txt').write_bytes(b'army_gui 1\n')
    shutil.copy2(dll, package/'tgcx.dll')
    shutil.copy2(dll, package/'v2dll_mod.dll')
    (output/'mod/tgcx_army_universal.mod').write_text('name = "ATCU - Army Template and Conform Universal"\npath = "mod/tgcx_army_universal"\n', encoding='cp1252')
    shutil.copy2(loader, output/'lua51.dll')
    (output/'README.txt').write_text(
        'ATCU - Army Template and Conform Universal\n'
        'Saved Conform orders test candidate, internal version 0.11.2\n\n'
        'The SAME mod folder/descriptor and DLL work with the active supported 3.04 base.\n'
        'No installer, scripts, per-base preparation or original GUI overrides.\n'
        'Close the game normally before copying. Keep backups of existing files.\n'
        'With our existing loader: copy mod/ into the game folder. In the normal\n'
        'launcher select ONLY ATCU plus your base (if any). Untick old Army\n'
        'and TGCX Politics packages. Keep the complete previous ATCU folder\n'
        'and descriptor outside mod/ before replacing an earlier ATCU build.\n'
        'Clean game: retain original lua51.dll as lua51_real.dll, then copy the\n'
        'included lua51.dll loader. Do not replace a V2DLL installation with it.\n'
        'V2DLL users: the identical logic is included as v2dll_mod.dll in the Army folder,\n'
        'with LOAD_DLL_FROM_MOD_FOLDER=1 and Army first in the mod list.\n\n'
        'CULTURE: each template saves either Primary + accepted, Primary only,\n'
        'or unrestricted (neither box ticked). Clicking the selected box clears it.\n'
        'Both Build Army and Conform missing builds use this preference.\n'
        'Existing regiments are counted by type and are not removed for culture.\n'
        'CONTINENT: builder arrows select Any or a continent where you own provinces.\n'
        'This session-only recruitment choice is not saved with the template.\n'
        'Conform keeps its separate land-reach restriction and ignores this selector.\n'
        'Filters combine with normal unit eligibility and free soldier capacity.\n'
        'Recruitment queues what fits and reports what is left out.\n\n'
        'CONFORM ORDERS: regiments being built for a Conform are also listed in\n'
        'tgcx_conform_orders.txt in the same settings folder (country tag, date,\n'
        'stack and construction ids). Loading a save of the same country dated\n'
        'within an order resumes it: the regiment still joins its conformed\n'
        'stack, even from a rally point. Not part of the save; deleting the file\n'
        'only stops resuming after a load.\n\n'
        'MOBO STACK: the box in the Conform window header marks the open stack with\n'
        'the infantry its selected template asks for; Conform then keeps infantry out\n'
        'of that stack. When the country mobilizes, each new all-infantry stack that\n'
        'appears marches to the nearest marked stack still short of infantry, over a\n'
        'safe land route, and joins it (only if all its regiments fit). Marks are kept\n'
        'in tgcx_mobo_marks.txt in the same settings folder, resumed after a load\n'
        'like the conform orders. Regiments raised into a province that already\n'
        'holds an army join that army and are not moved.\n\n'
        'SETTINGS: imports the old tgcx_templates.txt without rewriting it.\n'
        'Edits save atomically to tgcx_templates_v2.txt in the same Victoria II\n'
        'user settings folder. Continent membership is cached from engine data;\n'
        'the mod TXT files are not parsed on each click.\n'
        'ROLLBACK: restore the previous complete ATCU folder and descriptor.\n'
        'The old build reads the retained v1 file; edits made in this candidate\n'
        'remain in v2 and will not appear in the old build. Keep both files.\n\n'
        'TEST CANDIDATE: automated checks pass; filter gameplay and visuals\n'
        'still need player verification. Check allowed cultures, continents,\n'
        'Conform shortages, paused switching, save/reload and native Navy tabs.\n'
        'Preserve Logs/tgcx.log. This ZIP is for local testing, not publication.\n', encoding='utf-8')
    manifest={p.relative_to(output).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()
              for p in sorted(output.rglob('*')) if p.is_file()}
    Path(str(output)+'.SHA256.json').write_text(json.dumps(manifest,indent=2))
    archive=Path(str(output)+'.zip')
    with zipfile.ZipFile(archive,'x',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(output.rglob('*')):
            if p.is_file():z.write(p,p.relative_to(output))
    with zipfile.ZipFile(archive) as z:
        if z.testzip() is not None:raise ValueError('ZIP failed CRC')
        for name,digest in manifest.items():
            if hashlib.sha256(z.read(name)).hexdigest()!=digest:raise ValueError('ZIP entry mismatch')
    return archive


if __name__=='__main__':
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--output',required=True,type=Path)
    ap.add_argument('--art',required=True,type=Path,help='private accepted staged Army art folder')
    ap.add_argument('--dll',type=Path,default=Path('dll/tgcx/Release/tgcx.dll'))
    ap.add_argument('--loader',type=Path,default=Path('dll/tgcx/Release/lua51.dll'))
    a=ap.parse_args();print(build(a.output,a.art,a.dll,a.loader))
