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

VERSION = '0.11.15'
# TPL-12: the Replace depleted chooser needs the Rust DLL that handles it.
MARKER = b'TGCX TPL-12 0.11.15 tgcx.dll'


def build(output, art, dll, loader):
    output, art, dll, loader = map(lambda p: Path(p).resolve(), (output, art, dll, loader))
    if output.exists() or Path(str(output)+'.zip').exists():
        raise ValueError('output folder/ZIP must be new')
    if MARKER not in dll.read_bytes():
        raise ValueError('filter package requires the matching 0.11.15 DLL and GUI')
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
        'Replace depleted adaptive header, internal version 0.11.15, local candidate\n\n'
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
        'CULTURE: each template saves Primary + accepted, Primary only,\n'
        'Non-core only, or unrestricted (no box ticked). Clicking the selected box\n'
        'clears it. Non-core means a soldier pop whose culture is neither the\n'
        'country\'s primary nor an accepted culture (not the province\'s cores).\n'
        'Both Build Army and Conform missing builds use this preference.\n'
        'Existing regiments are counted by type and are not removed for culture.\n'
        'CONTINENT: builder arrows select Any or a continent where you own provinces.\n'
        'This session-only recruitment choice is not saved with the template.\n'
        'Conform keeps its separate land-reach restriction and ignores this selector.\n'
        'Filters combine with normal unit eligibility and free soldier capacity.\n'
        'Recruitment queues what fits and reports what is left out.\n\n'
        'GATHER AT: one Recruit Template click makes one army. Each regiment is\n'
        'built in its own province, then marches by a safe land route to the\n'
        'target and merges; the first one to finish becomes the army. Arrows choose\n'
        'Best supply (your province with the highest supply limit that the\n'
        'regiments reach by land), Capital, or each of your land rally points.\n'
        'If any army of yours already stands there, or an earlier click still\n'
        'gathers there, the army gathers in the free province beside it with the\n'
        'highest supply. Pops with no land route to the target are left out.\n'
        'Needs at least one army of yours to plan the routes. Session-only choice.\n'
        'Orders are kept in tgcx_gather_orders.txt and resumed after a load.\n\n'
        'CONFORM LIST: long template names are shortened with an ellipsis to fit\n'
        'one line; composition and the regiment count are unchanged.\n\n'
        'CONFORM ORDERS: regiments being built for a Conform are also listed in\n'
        'tgcx_conform_orders.txt in the same settings folder (country tag, date,\n'
        'stack and construction ids). Loading a save of the same country dated\n'
        'within an order resumes it: the regiment still joins its conformed\n'
        'stack, even from a rally point. Not part of the save; deleting the file\n'
        'only stops resuming after a load.\n\n'
        'REPLACE DEPLETED: the maroon button, left of the green Conform button\n'
        'in the army header, opens its own chooser. Both are 20x20 (native\n'
        'Disband size), in the same row as the native army-header actions, with\n'
        'the native 21px pitch (2px painted gap). Opening either closes the other.\n'
        'The pair shifts right when native Disband depleted is hidden, closing\n'
        'the empty slot, and moves back when it is shown. No native actions are\n'
        'moved or hidden. Conform retains its two original footer buttons.\n'
        'Tick Low strength (below the slider, 10% to 90%) and/or Pop too small\n'
        '(the unit panel shows the pop red or yellow). Replace builds one\n'
        'regiment of the same type for each depleted one, within land reach and\n'
        'with the chooser\'s own Culture boxes: Primary + accepted, Primary only,\n'
        'Non-core only, or any culture (no box ticked, the default). Clicking a\n'
        'selected box clears it. No selected template is required. It marches like a\n'
        'Conform build. When it has merged, the old regiment is detached into its\n'
        'own stack beside the army; nothing is disbanded (use the native button).\n'
        'The choice is kept in tgcx_replace_settings.txt (v2, imports v1); rolling\n'
        'back to 0.11.7 resets its choice unless you restore that settings backup.\n'
        'Replacements are kept in tgcx_replace_orders.txt (v2, imports v1) and\n'
        'resumed after a load, including one saved after its merge but before\n'
        'the old regiment left: the old regiment is detached again. A record\n'
        'that can no longer be resumed is closed at that load.\n\n'
        'MOBO STACK: the box in the Conform window header marks the open stack with\n'
        'the infantry its selected template asks for; Conform then keeps infantry out\n'
        'of that stack. When the country mobilizes, each new all-infantry stack that\n'
        'appears marches to the nearest marked stack still short of infantry, over a\n'
        'safe land route, and joins it (only if all its regiments fit). Marks are kept\n'
        'in tgcx_mobo_marks.txt in the same settings folder, resumed after a load\n'
        'like the conform orders. Regiments raised into a province that already\n'
        'holds an army join that army and are not moved.\n\n'
        'SETTINGS: imports tgcx_templates_v2.txt (or the older tgcx_templates.txt)\n'
        'without rewriting it. Edits save atomically to tgcx_templates_v3.txt in the\n'
        'same Victoria II\n'
        'user settings folder. Continent membership is cached from engine data;\n'
        'the mod TXT files are not parsed on each click.\n'
        'ROLLBACK: restore the previous complete ATCU folder and descriptor.\n'
        'The old build reads its retained v2 or v1 file; edits made in this\n'
        'candidate remain in v3 and will not appear in the old build. Keep all files.\n'
        '0.11.12 and older refuse a v2 tgcx_replace_orders.txt (left untouched):\n'
        'replacements then do not resume after a load in the old build.\n\n'
        'TEST CANDIDATE: automated checks pass; Replace depleted gameplay and\n'
        'visuals still need player verification. Check the chooser and slider,\n'
        'the preview, replacements merging, old regiments detached, save/reload.\n'
        'Preserve Logs/tgcx.log. This ZIP is for local testing, not publication.\n', encoding='utf-8')
    (output/'CHANGELOG.txt').write_text(
        'ATCU 0.11.15 - 2026-10-24 (local candidate)\n\n'
        '- Fix: a save loaded from the main menu (dated after the menu\'s\n'
        '  1 January 1836) was never seen as a load, so its regiments being\n'
        '  built for Replace, Conform or Build Army were not resumed and stayed\n'
        '  where they were built. Before a day step is known, any jump of more\n'
        '  than two days is now a load. Loading from the in-game menu was\n'
        '  already handled when the date goes back.\n'
        '- No visual, GUI, texture, chooser or settings change from 0.11.14.\n\n'
        'ATCU 0.11.14 - 2026-10-24 (local candidate)\n\n'
        '- Fix: a regiment built by Replace, Conform or Build Army could stay\n'
        '  where it was built instead of marching to its army. One odd frame\n'
        '  (seen around a save during construction) was taken for a load, the\n'
        '  order was dropped and not resumed. A load must now be seen on three\n'
        '  frames in a row, and dates before year 0 are ignored.\n'
        '- The log now records what a detected load changed (world, player, date).\n'
        '- A regiment already left behind this way resumes when you load a save\n'
        '  taken while it was being built.\n'
        '- No visual, GUI, texture, chooser or settings change from 0.11.13.\n\n'
        'ATCU 0.11.13 - 2026-10-24 (local candidate)\n\n'
        '- Replace: a save taken after a replacement merged but before the old\n'
        '  regiment left now detaches the old regiment again after loading.\n'
        '  Its record stays open until the old regiment has left the army.\n'
        '- Replace: saved records that a load cannot resume are closed instead\n'
        '  of staying open and being retried on every load.\n'
        '- tgcx_replace_orders.txt is now v2 (adds merged=0/1); v1 is imported.\n'
        '  0.11.12 and older leave a v2 file untouched and do not resume from it.\n'
        '- README: Replace button described in its current header position.\n'
        '- No visual, GUI, texture, chooser or settings change from 0.11.12.\n'
        '- The new reload paths still need a player check.\n\n'
        'ATCU 0.11.12 - 2026-10-24 (local candidate)\n\n'
        '- Header controls match native 21px pitch (2px gap between painted frames).\n'
        '- Use the native header Y directly: live borders proved the 1px lift wrong.\n'
        '- Long Conform template names fit on one line, using native font widths\n'
        '  and an ellipsis; the regiment count stays on its own line.\n'
        '- Replace and Conform shift right one slot when native Disband depleted\n'
        '  is hidden; move back when shown, following its actual visibility.\n'
        '- Button visibility uses CEU3Button +0x4F with Show/Hide signature guards.\n'
        '- Textures, chooser, culture, orders and settings formats unchanged.\n'
        '- Both visibility transitions still need a player check.\n\n'
        'ATCU 0.11.9 - 2026-10-24 (local candidate)\n\n'
        '- Conform and Replace now match native Disband size (20x20), retaining\n'
        '  the green/maroon infantry and low-strength art.\n'
        '- The pair sits beside the native header actions, 3px apart, aligned from the\n'
        '  native Disband button. The location row is clear of our controls.\n'
        '- Long army names have less room in the header and may run under the\n'
        '  new controls, as long location names did with the previous position.\n'
        '- 0.11.8 passed the user\'s short check; chooser, filters, orders and\n'
        '  replacement/detach behavior are unchanged. Settings formats unchanged.\n'
        '- New placement still needs a player appearance/hit-testing check.\n\n'
        'ATCU 0.11.8 - 2026-10-24 (local candidate)\n\n'
        '- Replace depleted opens from its own army-header button.\n'
        '- Option A: two 16px buttons above the braid, 3px apart; green Conform,\n'
        '  maroon Replace with infantry and low-strength bar.\n'
        '- Conform restored to its Conform / Cancel footer.\n'
        '- Independent 332x270 Replace chooser with saved, exclusive Culture boxes.\n'
        '- Low-strength slider, red/yellow pop detection and detach-after-merge\n'
        '  behavior retained from 0.11.7; no regiment is disbanded.\n'
        '- Settings v2 imports v1 with unrestricted culture; order format unchanged.\n'
        '- 0.11.7 replacement/detach passed a short player check. 0.11.8 visuals,\n'
        '  culture selection and reload still require a player check.\n'
        '- Long location names may still extend beneath the header buttons.\n'
        '- No public release, multiplayer certification or installation implied.\n', encoding='utf-8')
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
