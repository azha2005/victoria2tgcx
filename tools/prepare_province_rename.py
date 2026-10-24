"""Build the separate files-only Province Rename mod for V2DLL. Never installs it."""
import argparse
import hashlib
import json
import shutil
import struct
import zipfile
import re
from functools import lru_cache
from pathlib import Path
from PIL import Image, ImageDraw
from tgcx_pe import PE
from tgc_scan import parse
from tgcx_bases import Base
from tgcx_army_art import sprites as native_sprites, sprite as native_sprite
from tgcx_bill_gui import nine

ROOT = Path(__file__).resolve().parent.parent
VERSION = '0.2.5'
FOLDER = 'tgcx_province_rename'
STRINGS = {'title': 'Rename province', 'open': 'Rename', 'apply': 'Rename', 'cancel': 'Cancel',
           'current': '', 'current_caption': 'Current name', 'new_caption': 'New name',
           'status': 'Up to 80 characters.'}


def window(name, width, height, content):
    return (f'windowType={{name="tgcx_rename_{name}" position={{x=0 y=0}} '
            f'size={{x={width} y={height}}} backGround="" moveable=0 fullScreen=no\n'+content+'}\n')


def icon(name, sprite, x, y):
    return 'iconType={name="tgcx_rename_%s" spriteType="GFX_tgcx_rename_%s" position={x=%d y=%d}}\n' % (name,sprite,x,y)


def text(name, x, y, w, h=18, font='Arial14_black', align='left', key=None):
    return (f'instantTextBoxType={{name="tgcx_rename_{name}" position={{x={x} y={y}}} '
            f'font="{font}" text="tgcx_rename_{key or name}" maxWidth={w} maxHeight={h} '
            f'format={align} fixedsize=yes allwaystransparent=yes}}\n')


def button(name, sprite, x, y):
    return (f'guiButtonType={{name="tgcx_rename_{name}" quadTextureSprite="GFX_tgcx_rename_{sprite}" '
            f'position={{x={x} y={y}}}}}\n')


def layout():
    entry = button('open', 'entry', 0, 0)
    body = icon('background', 'panel', 0, 0)+text('title', 18, 4, 296, 27, 'vic_22', 'centre')
    body += text('current_caption', 18, 40, 296, 20, 'Main_16_black', 'centre')
    body += text('current', 18, 60, 296, 24, 'vic_18_black_bold', 'centre')
    body += text('new_caption', 18, 86, 296, 20, 'Main_16_black', 'centre')+icon('field', 'field', 18, 108)
    body += ('editBoxType={name="tgcx_rename_name" position={x=18 y=108} size={x=296 y=30} '
             'borderSize={x=8 y=7} font="Main_16_black" text="" orientation="UPPER_LEFT"}\n')
    body += text('status', 18, 145, 296, 22, 'Main_14_black', 'centre')
    body += button('cancel', 'button', 18, 178)+text('cancel_label', 18, 185, 144, 22, 'vic_18_black', 'centre', 'cancel')
    body += button('apply', 'primary', 170, 178)+text('apply_label', 170, 185, 144, 22, 'vic_18_black', 'centre', 'apply')
    return 'guiTypes={\n'+window('entry', 24, 24, entry)+window('window', 332, 224, body)+'}\n'


def validate_layout(gui):
    """Native construction resolves child names in a shared type registry."""
    seen = set()
    def visit(items):
        for kind, body in items:
            if not isinstance(body, list):
                continue
            attrs = dict(body)
            if 'name' in attrs:
                name = attrs['name']
                if name in seen:
                    raise ValueError(f'duplicate GUI definition: {name}')
                seen.add(name)
                if kind == 'instantTextBoxType' and attrs.get('allwaystransparent') != 'yes':
                    raise ValueError(f'text label must pass clicks through: {name}')
            visit(body)
    visit(parse(gui))


@lru_cache(maxsize=1)
def artwork():
    # Same private native-art preparation approach as ATCU. Never write these
    # game-derived pixels into tracked files; packages/previews stay in work/.
    game=ROOT/'work/Victoria-II-Province-Rename-TGC252'
    base=Base(game,game/'mod/TGC_v2.5.2.mod');table=native_sprites(game,base)
    def native(name,**kw):return native_sprite(game,table,name,profile=base,**kw)
    body=native('GFX_unit_bottom_bg');top=native('GFX_unit_toppanel_bg',frames=2)
    if body.size!=(332,408) or top.size!=(332,64):raise ValueError('native army-panel dimensions changed')
    panel=body.crop((0,80,332,304))
    panel.paste(body.crop((0,400,332,408)),(0,216))
    panel.alpha_composite(top.crop((0,0,332,27)),(0,0))
    panel.alpha_composite(top.crop((51,5,324,27)).resize((317,22)),(7,5))
    panel.alpha_composite(top.crop((0,58,332,64)),(0,27))
    # User's current reference: white title directly on native green, with
    # the original gold braid. No parchment title plate.
    source=native('GFX_bp_textedit_bg')
    if source.size!=(208,80):raise ValueError(f'unexpected native input frame size: {source.size}')
    frame=Image.new('RGBA',(198,30))
    frame.paste(source.crop((5,3,203,20)),(0,0))
    frame.paste(source.crop((5,58,203,75)),(0,13))
    field=nine(frame,6,296,30)
    button=native('GFX_button_standard_144')
    if button.size!=(144,34):raise ValueError('native standard144 button dimensions changed')
    entry=native('GFX_bp_button_rename')
    if entry.size!=(28,28):raise ValueError('native ABC rename sprite must be28x28')
    entry=entry.resize((24,24),Image.Resampling.LANCZOS)
    cursor=native('cursor')
    if cursor.size!=(32,32) or cursor.getbbox()!=(7,6,8,22):
        raise ValueError('native caret ink bounds changed; revise caret offsets')
    return {'entry':entry,'button':button,'primary':button.copy(),'field':field,'panel':panel}


def dds(image, path):
    w,h = image.size
    header = struct.pack('<4s7I44x2I4s5I5I',b'DDS ',124,0x100F,h,w,w*4,0,1,
                         32,0x41,b'\0'*4,32,0xFF0000,0xFF00,0xFF,0xFF000000,0x1000,0,0,0,0)
    r,g,b,a = image.split();path.write_bytes(header+Image.merge('RGBA',(b,g,r,a)).tobytes())


@lru_cache(maxsize=8)
def native_font(name):
    """Private preview only: use actual BMFont metrics and atlas, not substitutes."""
    game=ROOT/'work/Victoria-II-Province-Rename-TGC252';base=Base(game,game/'mod/TGC_v2.5.2.mod')
    fontname={'Main_16_black':'garamond_16_bold','Main_14_black':'garamond_14_bold',
              'vic_18_black_bold':'vic_18_bold','vic_18_black':'vic_18'}.get(name,name)
    fnt=base.find(f'gfx/fonts/{fontname}.fnt');atlas=base.find(f'gfx/fonts/{fontname}.tga')
    if not fnt or not atlas:raise ValueError(f'missing native preview font {name}')
    chars={};pairs={}
    for line in fnt.read_text(encoding='cp1252').splitlines():
        values={k:int(v) for k,v in re.findall(r'(\w+)=(-?\d+)',line)}
        if line.startswith('char '):chars[values['id']]=values
        elif line.startswith('kerning '):pairs[(values['first'],values['second'])]=values['amount']
    with Image.open(atlas) as image:bitmap=image.convert('RGBA')
    return chars,pairs,bitmap


def bitmap_label(canvas,x,y,value,font='Arial14',fill='#000000',center=None):
    chars,pairs,atlas=native_font(font);codes=list(value.encode('cp1252'))
    width=sum(chars.get(c,{}).get('xadvance',0)+pairs.get((c,codes[i+1]),0) if i+1<len(codes)
              else chars.get(c,{}).get('xadvance',0) for i,c in enumerate(codes))
    if center is not None:x+=int((center-width)/2)
    for i,c in enumerate(codes):
        info=chars.get(c)
        if info:
            if info['width'] and info['height']:
                mask=atlas.crop((info['x'],info['y'],info['x']+info['width'],info['y']+info['height'])).getchannel('A')
                canvas.paste(fill,(x+info['xoffset'],y+info['yoffset']),mask)
            x+=info['xadvance']+pairs.get((c,codes[i+1]),0) if i+1<len(codes) else info['xadvance']
    return width


def editor_preview(current='La Plata', proposed='New La Plata'):
    """Render the actual generated editor definitions with native font atlases."""
    assets=artwork();canvas=assets['panel'].convert('RGB')
    root=next(body for kind,body in parse(layout())[0][1] if dict(body).get('name')=='tgcx_rename_window')
    for kind,body in root:
        if not isinstance(body,list):continue
        attrs=dict(body);pos=dict(attrs.get('position',[]));x=int(pos.get('x',0));y=int(pos.get('y',0))
        if kind in ('iconType','guiButtonType'):
            key=attrs.get('spriteType',attrs.get('quadTextureSprite','')).removeprefix('GFX_tgcx_rename_')
            if key in assets:canvas.paste(assets[key],(x,y),assets[key])
        elif kind=='instantTextBoxType':
            key=attrs['text'].removeprefix('tgcx_rename_');value=current if key=='current' else STRINGS[key]
            bitmap_label(canvas,x,y,value,attrs['font'],'#000000' if attrs['font'].lower().endswith(('_black','_black_bold')) else '#f8f1d7',
                         int(attrs['maxWidth']) if attrs.get('format')=='centre' else None)
        elif kind=='editBoxType':
            # The runtime shares this centering inset with its native caret.
            border=dict(attrs['borderSize']);width=int(dict(attrs['size'])['x'])
            bitmap_label(canvas,x,y+int(border['y']),proposed,attrs['font'],center=width)
    return canvas


def preview(path):
    assets=artwork();canvas=Image.new('RGB',(780,350),'#303d42');draw=ImageDraw.Draw(canvas)
    bitmap_label(canvas,20,14,f'Province Rename {VERSION} - generated layout preview','vic_18','#f8f1d7')
    draw.rectangle((15,65,395,325),fill='#d9d2ba',outline='#a18e58',width=3)
    draw.rectangle((21,71,389,132),fill='#38563f')
    bitmap_label(canvas,32,72,'Buenos Aires','vic_22','#f8f1d7')
    name_width=bitmap_label(canvas,32,110,'La Plata','vic_18','#f8f1d7')
    canvas.paste(assets['entry'],(32+name_width+8,107),assets['entry'])
    bitmap_label(canvas,32,165,'Existing province controls stay in place.','Main_14_black')
    canvas.paste(editor_preview(),(415,65));canvas.save(path)


def mockup(path):
    """Own SVG layout with private external native-art previews (R8)."""
    assets=artwork();folder=ROOT/f'work/province-rename/mockup-assets-{VERSION}';folder.mkdir(parents=True,exist_ok=True)
    for name,img in assets.items():img.save(folder/f'tgcx_rename_{name}.png')
    editor_preview().save(folder/'tgcx_rename_editor_preview.png')
    def asset(name,x,y,w,h):
        rel=Path('../work/province-rename')/folder.name/f'tgcx_rename_{name}.png'
        return f'<image x="{x}" y="{y}" width="{w}" height="{h}" href="{rel.as_posix()}"/>'
    body='<svg xmlns="http://www.w3.org/2000/svg" width="780" height="350" viewBox="0 0 780 350">'
    body+='<style>text{font-family:serif;fill:#000}</style><rect width="780" height="350" fill="#d4d0c2"/>'
    body+=f'<text x="18" y="28">Province Rename Universal {VERSION} - centred input, native redesign, 2026-10-21</text>'
    body+='<rect x="15" y="65" width="380" height="260" fill="#d9d2ba" stroke="#a18e58" stroke-width="3"/>'
    body+='<text x="32" y="91" font-size="20">Buenos Aires</text><text x="32" y="127" font-size="18">La Plata</text>'
    body+=asset('entry',110,108,24,24)
    body+='<text x="32" y="177" font-size="12">Native province controls remain below.</text>'
    body+=asset('editor_preview',415,65,332,224)
    Path(path).write_text(body+'</svg>\n',encoding='utf-8')


def build(output, dll):
    output,dll=Path(output).resolve(),Path(dll).resolve()
    if output.exists() or Path(str(output)+'.zip').exists():
        raise ValueError('output folder and ZIP must be new')
    image = PE(dll)
    if image.machine != 0x14C or 'V2Mod_Init' not in image.exports() or f'TGCX Province Rename {VERSION}'.encode() not in dll.read_bytes():
        raise ValueError('requires the matching x86 V2DLL plugin')
    gui = layout()
    validate_layout(gui)
    package=output/'mod'/FOLDER
    for rel in ['interface','gfx/interface','localisation']:
        (package/rel).mkdir(parents=True,exist_ok=True)
    (output/'mod'/f'{FOLDER}.mod').write_text('name = "Province Rename (Universal)"\npath = "mod/tgcx_province_rename"\n',encoding='cp1252')
    (package/'tgcx_package.txt').write_text(f'package = rename\nversion = {VERSION}\n',encoding='ascii')
    (package/'interface/tgcx_province_rename.gui').write_text(gui,encoding='cp1252')
    definitions=['spriteTypes={']
    for name,img in artwork().items():
        filename=f'tgcx_rename_{name}.dds';dds(img,package/'gfx/interface'/filename)
        definitions.append(f'spriteType={{name="GFX_tgcx_rename_{name}" texturefile="gfx/interface/{filename}" '
                           'noOfFrames=1 loadType="INGAME"'+(' effectFile="gfx/FX/buttonstate.fx"' if name in ('entry','button','primary') else '')+'}')
    (package/'interface/tgcx_province_rename.gfx').write_text('\n'.join(definitions+['}','']),encoding='cp1252')
    loc=['CODE;ENGLISH;FRENCH;GERMAN;POLISH;SPANISH;ITALIAN;SWEDISH;CZECH;HUNGARIAN;DUTCH;PORTUGUESE;RUSSIAN;FINNISH;x']
    loc += [f'tgcx_rename_{key};{value}'+ ';'*13+'x' for key,value in STRINGS.items()]
    (package/'localisation/tgcx_province_rename.csv').write_bytes(('\n'.join(loc)+'\n').encode('cp1252'))
    shutil.copy2(dll,package/'v2dll_mod.dll')
    (output/'README.txt').write_text(
        f'Province Rename (Universal) {VERSION} - separate V2DLL mod\n\n'
        'REQUIREMENTS: reference Victoria II 3.04 and V2DLL with per-active-mod\n'
        'plugin loading (verified source cdad55c7, 2026-10-20). Enable\n'
        'LOAD_DLL_FROM_MOD_FOLDER=1 in V2DLL/v2dll_settings.ini.\n'
        'Older 5.82 only loads a plugin from the FIRST -mod= folder; Province\n'
        'Rename must be first with that version. Use current V2DLL to combine\n'
        'plugins such as ATCU. No V2DLL binary is bundled.\n'
        'Upstream: https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension\n\n'
        'INSTALL: close the game normally. Copy mod/ into your game folder.\n'
        'Select Province Rename (Universal), plus your base mod if any.\n'
        'One additive package for vanilla and ordinary 3.04 mods. No TGC\n'
        'dependency, user_dir, base data or GUI replacement is included.\n'
        'Keep your V2DLL loader. No replacement lua51.dll is supplied.\n'
        'The plugin is mod/tgcx_province_rename/v2dll_mod.dll and exports V2Mod_Init.\n'
        'The separate GUI files add definitions; no base GUI file is overridden.\n'
        'ATCU and Politics files are not part of this package.\n\n'
        'ALTERNATIVE (current V2DLL): MOVE the plugin from the mod folder to\n'
        'V2DLL/dll_Submods/tgcx_province_rename.dll, leaving the GUI mod in mod/.\n'
        'This needs no loader setting. Keep only ONE copy of the plugin.\n'
        'The plugin stays inactive unless Province Rename is selected.\n\n'
        'USE: select a land province you own. Click the native ABC icon to the\n'
        'right of its name, type a new name in the editor, then click Rename.\n'
        'Typed names are centred as you edit; long names retain normal padding.\n'
        'Cancel discards the edit. Changing selection, owner, country or world\n'
        'cancels the edit. Foreign and sea provinces cannot be renamed.\n'
        'Names support 1-80 Windows-1252 characters, trimmed at either end.\n'
        'Quotes, backslashes, format/control characters are refused.\n'
        'Apply queues the native CChangeProvinceNameCommand (0x707); the engine\n'
        'updates the province name and map label. No local names sidecar.\n'
        'Province names use the engine save/load path. Save/reload and visual\n'
        'behavior still require an in-game check of this candidate.\n\n'
        'VERIFY: Logs/tgcx_province_rename.log should say V2DLL plugin API v1,\n'
        '78 native signatures OK and Rename hooks installed. Queue actions\n'
        'include province id and the name. V2DLL log must show plugin -> ok.\n'
        'Unsupported executables/signatures refuse the rename hooks.\n'
        'Without V2DLL this mod loads inertly; it cannot provide free text.\n'
        'Multiplayer has not been tested. All renames use engine commands.\n\n'
        'REMOVE: untick Province Rename; after exiting normally, remove only\n'
        'its folder and .mod descriptor. Names already saved stay in the save.\n'
        'Private personal-use candidate: native game/TGC artwork crops are\n'
        'generated during packaging, like ATCU. They stay outside git.\n'
        'No base GUI/data file is bundled or overridden.\n'
        'V2DLL API and observer/native text ABI patterns are credited in source.\n'
        'TEST CANDIDATE: automated evidence is separate from game acceptance.\n',encoding='ascii')
    paths=sorted(p for p in output.rglob('*') if p.is_file())
    manifest={p.relative_to(output).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
    archive=Path(str(output)+'.zip')
    with zipfile.ZipFile(archive,'x',zipfile.ZIP_DEFLATED) as z:
        for p in paths:
            entry=zipfile.ZipInfo(p.relative_to(output).as_posix(),(2026,10,20,0,0,0));entry.compress_type=zipfile.ZIP_DEFLATED
            z.writestr(entry,p.read_bytes())
    manifest_path=Path(str(output)+'-manifest.json');manifest_path.write_text(json.dumps(manifest,indent=2)+'\n')
    digest=hashlib.sha256(archive.read_bytes()).hexdigest()
    Path(str(archive)+'.sha256').write_text(f'{digest}  {archive.name}\n',encoding='ascii')
    return {'zip':str(archive),'sha256':digest,'files':len(paths)}


if __name__ == '__main__':
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('output',type=Path)
    ap.add_argument('--dll',type=Path,default=ROOT/'dll/province_rename/Release/tgcx_province_rename.dll')
    ap.add_argument('--preview',type=Path)
    ap.add_argument('--mockup',type=Path)
    args=ap.parse_args()
    print(json.dumps(build(args.output,args.dll),indent=2))
    if args.preview:preview(args.preview)
    if args.mockup:mockup(args.mockup)
