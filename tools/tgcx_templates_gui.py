"""TPL-0 native panel, generated from installed TGC/vanilla art and unit data.

Only our fragment enters git. The installer inserts it at the named large
build window in TGC's effective buildunit.gui (vanilla when TGC inherits it).
"""
import argparse
import re
from pathlib import Path
from PIL import Image, ImageDraw
from gen_laws import DEFAULT_TGC, ROOT
from tgc_scan import parse, read
from tgcx_bill_gui import sprites, texture, sprite, section, write_dds, game_root

LAYOUT = "tgcx_templates_layout.txt"
ORDER = "infantry guard irregular artillery engineer cuirassier dragoon hussar cavalry tank plane".split()
ABBR = "Inf Gds Irr Art Eng Cui Dra Hus Cav Tnk Air".split()
FOLDERS = [17, 1, 2, 3, 16, 18, 14, 13, 15, 20, 19]
ROWS, SLOTS = 8, 12


def units(tgc=DEFAULT_TGC):
    found = {}
    for path in sorted((Path(tgc) / "units").glob("*.txt")):
        for key, body in parse(read(path)):
            data = dict(body)
            if data.get("type") == "land":
                found[key] = (int(data["build_time"]), int(data["icon"]))
    if set(found) != set(ORDER):
        raise ValueError("TPL-0 requires the reviewed TGC v2.5.2 land unit catalogue")
    return [(key, key.replace("_", " ").title(), abbr, *found[key]) for key, abbr in zip(ORDER, ABBR)]


# Panel coordinates are build_unit_view_large's minus (5,69); board 5's
# source (Templates.dc.html) gives the native sprites and positions.
MINIS = 4


def buttons():
    result = [("open", "tab", 371, 31, 34, 36)]
    result += [(f"slot_{i}", "GFX_province_recruit_group_button", 2, 2+i*32, 366, 30) for i in range(4)]
    result += [("list_prev", "left", 296, 132, 24, 24), ("list_next", "right", 364, 132, 24, 24),
               ("new", "GFX_bp_button_add", 4, 130, 28, 28), ("delete", "GFX_bp_button_remove", 193, 485, 28, 28),
               ("unit_prev", "left", 314, 160, 24, 24), ("unit_next", "right", 362, 160, 24, 24)]
    for i in range(ROWS):
        result += [(f"minus_{i}", "left", 231, 189+i*31, 24, 24), (f"plus_{i}", "right", 301, 189+i*31, 24, 24)]
    # GFX_button_standard_177 is a textSpriteType: as a bare button (no
    # buttonText) it neither draws nor binds, so a spriteType copy is used.
    result += [("recruit", "recruit", 7, 482, 177, 34)]
    result += [(f"native_{n}", "", 0, 0, 0, 0) for n in FOLDERS]
    return result


def texts():
    result = [("title", "Build Army: Templates", 4, -60, 360, 22, "vic_18", "centre")]
    for i in range(4):
        result += [(f"slot_{i}", "", 42, 4+i*32, 150, 14, "Arial12_bold_black", "left"),
                   (f"detail_{i}", "", 42, 18+i*32, 90, 14, "Arial12_black", "left"),
                   (f"sdays_{i}", "", 124, 18+i*32, 66, 14, "Arial12_black", "right")]
        result += [(f"mini_{i}_{k}", "", 223+k*42, 11+i*32, 20, 14, "Arial12_bold", "left") for k in range(MINIS)]
    result += [("new", "New Template", 39, 135, 190, 20, "vic_18_black", "left"),
               ("list_page", "", 320, 136, 44, 16, "Arial12_black", "centre"),
               ("editor", "Composition", 4, 162, 300, 20, "vic_18_black", "centre"),
               ("unit_page", "", 338, 164, 24, 16, "Arial12_black", "centre")]
    for i in range(ROWS):
        result += [(f"unit_{i}", "", 42, 188+i*31, 150, 16, "Arial12_bold_black", "left"),
                   (f"days_{i}", "", 42, 202+i*31, 150, 14, "Arial12_black", "left"),
                   (f"count_{i}", "", 257, 192+i*31, 40, 18, "vic_18", "centre")]
    result += [("summary", "", 231, 487, 420, 16, "Arial12_bold_black", "left"),
               ("status", "One build order per regiment, from provinces with free soldier pops.", 231, 503, 420, 22, "Arial12_black", "left"),
               ("recruit", "Recruit Template", 7, 490, 177, 20, "vic_18_black", "centre")]
    return result


def _window(name, x, y, w, h, inner):
    return [f'windowType={{name="{name}" position={{x={x} y={y}}} size={{x={w} y={h}}} backGround="" moveable=0 fullScreen=no'] + inner + ['}']


def _strip(name, x, y, frame, scale=None):
    extra = f' scale={scale}' if scale else ''
    return f'iconType={{name="{name}" spriteType="GFX_unit_strip" frame={frame} position={{x={x} y={y}}}{extra}}}'


def layout():
    icons = {key: icon for key, _, _, _, icon in units()}
    # Build Navy shares this view and has more unit folders: the tab sits in a
    # window so the DLL can hide it there. Button lookups are flat, so an
    # off-screen marker button tells the DLL silently that this is the large
    # build view before it looks up any window (the province view has none).
    parts = ['guiButtonType={ name="tgcx_tpl_marker" quadTextureSprite="tgcx_tpl_tab" position={x=30000 y=30000} }',
             'windowType={ name="tgcx_tpl_tab" position={x=371 y=31} size={x=34 y=36} moveable=0 fullScreen=no backGround="" orientation="UPPER_LEFT"',
             'guiButtonType={ name="tgcx_tpl_open" quadTextureSprite="tgcx_tpl_tab" position={x=0 y=0} clicksound=click }',
             '}',
             'windowType={ name="tgcx_tpl_window" position={x=30000 y=30000} size={x=652 y=528} moveable=0 fullScreen=no backGround="" orientation="UPPER_LEFT"',
             'iconType={name="tgcx_tpl_header" spriteType="tgcx_tpl_header" position={x=0 y=-64}}',
             # The Templates folder shows as selected while the panel is open.
             'iconType={name="tgcx_tpl_tabsel" spriteType="tgcx_tpl_tabsel" position={x=366 y=-38}}',
             'iconType={name="tgcx_tpl_bg" spriteType="tgcx_tpl_bg" position={x=0 y=0}}',
             'iconType={name="tgcx_tpl_bar" spriteType="tgcx_tpl_bar" position={x=0 y=466}}']
    parts += [f'iconType={{name="tgcx_tpl_erow_{i}" spriteType="GFX_province_recruit_button" position={{x=2 y={186+i*31}}}}}' for i in range(ROWS)]
    for name, art, x, y, _, _ in buttons()[1:]:
        if not art:
            continue
        sprite_name = art if art.startswith("GFX_") else f"tgcx_tpl_{art}"
        parts.append(f'guiButtonType={{name="tgcx_tpl_{name}" quadTextureSprite="{sprite_name}" position={{x={x} y={y}}} clicksound=click}}')
    # Native buttons cannot be hidden by the DLL's verified window methods:
    # the selection ring and the covers for empty rows are windows above them.
    for row in range(4):
        parts += _window(f"tgcx_tpl_slot_sel_{row}", 2, 2+row*32, 366, 30,
                         [f'iconType={{name="tgcx_tpl_ring_{row}" spriteType="tgcx_tpl_ring" position={{x=0 y=0}}}}'])
        parts += _window(f"tgcx_tpl_slot_blank_{row}", 2, 2+row*32, 366, 30,
                         [f'iconType={{name="tgcx_tpl_blank_icon_{row}" spriteType="tgcx_tpl_blank_{row}" position={{x=0 y=0}}}}'])
    parts += _window("tgcx_tpl_pager_blank", 296, 130, 92, 28,
                     ['iconType={name="tgcx_tpl_pager_icon" spriteType="tgcx_tpl_pager" position={x=0 y=0}}'])
    # Each slot and miniature has its own fixed unit icon; the DLL only shows
    # one per place. No unverified dynamic sprite setter is required.
    for row in range(4):
        for u in ORDER:
            parts += _window(f"tgcx_tpl_slot_icon_{row}_{u}", 3, row*32, 36, 36, [_strip(f"tgcx_tpl_slot_sprite_{row}_{u}", 0, 0, icons[u])])
            for k in range(MINIS):
                parts += _window(f"tgcx_tpl_mini_{row}_{k}_{u}", 199+k*42, 5+row*32, 24, 24,
                                 [_strip(f"tgcx_tpl_mini_sprite_{row}_{k}_{u}", 0, 0, icons[u], 0.66)])
    for page in range(2):
        parts.append(f'windowType={{name="tgcx_tpl_icons_{page}" position={{x=0 y=0}} size={{x=40 y=466}} backGround="" moveable=0 fullScreen=no')
        for row, u in enumerate(ORDER[page*ROWS:(page+1)*ROWS]):
            parts.append(_strip(f"tgcx_tpl_icon_{u}", 3, 184+row*31, icons[u]))
        parts.append('}')
    loc = {}
    for name, text, x, y, w, h, font, fmt in texts():
        key = f"tgcx_tpl_text_{name}"
        loc[key] = text
        parts.append(f'instantTextBoxType={{name="{key}" position={{x={x} y={y}}} font="{font}" text="{key}" maxWidth={w} maxHeight={h} format={fmt} allwaystransparent=yes}}')
    parts.append('}')
    return "\n".join(parts)+"\n", loc


def inject(base, fragment):
    # Reuse the byte-preserving named-window parser, with our window/marker.
    from tgcx_bill_gui import inject as insert
    if "tgcx_tpl_window" in base or "country_politics" in base:
        raise ValueError("require an unmodified buildunit GUI")
    if len(re.findall(r'name\s*=\s*"build_unit_view_large"', base)) != 1:
        raise ValueError("require exactly one build_unit_view_large")
    renamed = re.sub(r'(name\s*=\s*)"build_unit_view_large"', r'\1"country_politics"', base)
    result = insert(renamed, fragment)
    result = result.replace('"country_politics"', '"build_unit_view_large"')
    return result.replace("# >>> tgcx_bill", "# >>> tgcx_templates").replace("# <<< tgcx_bill", "# <<< tgcx_templates")


def header(tgc=DEFAULT_TGC):
    rows = units(tgc)
    q = lambda s: '"'+s+'"'
    lines = ['// Generated by tools/tgcx_templates_gui.py --header; do not edit.', '#pragma once',
             'struct tgcx_TplUnitInfo { const char* key; const char* label; const char* abbr; int days; };',
             'static constexpr tgcx_TplUnitInfo tgcx_tpl_catalog[]={']
    lines += ['    {'+','.join([q(key), q(label), q(abbr), str(days)])+'},' for key, label, abbr, days, _ in rows]
    lines += ['};', 'enum tgcx_TplButtonId {']
    lines += [f'    TGCX_TPL_{name.upper()},' for name, *_ in buttons()]
    lines += ['    TGCX_TPL_BUTTONS', '};', 'static constexpr const char* tgcx_tpl_button_names[]={']
    lines += [f'    "{"unit_folder_"+name[7:] if name.startswith("native_") else "tgcx_tpl_"+name}",' for name, *_ in buttons()]
    lines += ['};', 'enum tgcx_TplTextId {']
    lines += [f'    TGCX_TPL_TEXT_{name.upper()},' for name, *_ in texts()]
    lines += ['    TGCX_TPL_TEXTS', '};', 'static constexpr const char* tgcx_tpl_text_names[]={']
    lines += [f'    "tgcx_tpl_text_{name}",' for name, *_ in texts()]
    lines += ['};', '#define TGCX_TPL_THUNKS(M) '+ ' '.join(f'M({i})' for i in range(len(buttons()))), '']
    return '\n'.join(lines)


def generate(output, tgc=DEFAULT_TGC):
    output = Path(output)
    if not (output/'TGCX.mod').is_file():
        raise ValueError("Templates GUI needs a complete staged source")
    fragment, loc = layout()
    (output/'TGCX'/LAYOUT).write_bytes(fragment.encode('cp1252'))
    gfx = output/'TGCX/gfx/interface'
    gfx.mkdir(parents=True, exist_ok=True)
    (output/'TGCX/interface').mkdir(parents=True, exist_ok=True)
    (output/'TGCX/localisation').mkdir(parents=True, exist_ok=True)
    table = sprites(tgc)
    def native(name):
        # Frames are metadata, not inferred from a texture's aspect ratio.
        for root in (Path(tgc), game_root(tgc)):
            for path in sorted((root/'interface').glob('*.gfx')):
                for _, body in parse(read(path)):
                    if not isinstance(body,list):
                        continue
                    for _, spec in body:
                        if isinstance(spec,list) and dict(spec).get('name','').strip('"')==name:
                            return sprite(tgc,table,name,frames=int(dict(spec).get('noOfFrames',1)))
        raise ValueError('missing sprite '+name)
    images = {}
    # The left column, wide enough to cover the native list's scrollbar.
    images['bg'] = section(405,466)
    native_bg = texture(tgc,table['GFX_military_build_unit_bg'])
    images['header'] = native_bg.crop((6,8,401,31))
    images['bar'] = native_bg.crop((6,538,658,600))
    for row in range(4):
        images[f'blank_{row}'] = images['bg'].crop((2,2+row*32,368,32+row*32))
    images['pager'] = images['bg'].crop((296,130,388,158))
    ring = Image.new('RGBA',(366,30))
    d = ImageDraw.Draw(ring)
    d.rectangle((0,0,365,29), outline=(59,46,16,255))
    d.rectangle((1,1,364,28), outline=(243,215,124,255)); d.rectangle((2,2,363,27), outline=(243,215,124,255))
    images['ring'] = ring
    # Board 5's Templates folder, with the infantry figure from the unit strip.
    strip = texture(tgc,table['GFX_unit_strip'])
    w = strip.width//21
    tab = Image.new('RGBA',(34,36))
    d = ImageDraw.Draw(tab)
    d.rectangle((0,0,33,35), fill=(220,213,190,255), outline=(59,49,32,255))
    d.rectangle((2,2,31,33), outline=(168,137,63,255))
    d.rectangle((4,4,25,27), fill=(201,191,155,255), outline=(91,76,44,255))
    d.rectangle((8,8,29,31), fill=(241,236,217,255), outline=(59,49,32,255))
    tab.alpha_composite(strip.crop((0,0,w,strip.height)).resize((24,24)),(8,9))
    images['tab'] = tab
    sel = Image.new('RGBA',(34,36))
    d = ImageDraw.Draw(sel)
    d.rectangle((0,0,33,35), outline=(243,215,124,255)); d.rectangle((1,1,32,34), outline=(243,215,124,255))
    images['tabsel'] = sel
    for key, name in [('left','GFX_button_arrow_left'),('right','GFX_button_arrow_right')]:
        images[key] = native(name).resize((24,24))
    images['recruit'] = native('GFX_button_standard_177')
    declarations=[]
    for key, image in images.items():
        filename=f'tgcx_tpl_{key}.dds'
        write_dds(image,gfx/filename)
        declarations.append(f'spriteType={{name="tgcx_tpl_{key}" texturefile="gfx/interface/{filename}" noOfFrames=1 loadType="INGAME"}}')
    (output/'TGCX/interface/tgcx_templates.gfx').write_text('spriteTypes={\n'+'\n'.join(declarations)+'\n}\n',encoding='cp1252')
    rows=['CODE;ENGLISH;FRENCH;GERMAN;POLISH;SPANISH;ITALIAN;SWEDISH;CZECH;HUNGARIAN;DUTCH;PORTUGUESE;RUSSIAN;FINNISH;x']
    rows += [k+';'+v+';'*13+'x' for k,v in loc.items()]
    (output/'TGCX/localisation/tgcx_templates.csv').write_bytes(('\n'.join(rows)+'\n').encode('cp1252'))
    return images


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--header',action='store_true')
    ap.add_argument('--stage',type=Path)
    ap.add_argument('--tgc',type=Path,default=Path(DEFAULT_TGC))
    args=ap.parse_args()
    if args.header:
        (Path(ROOT)/'dll/tgcx/tgcx_templates_catalog.h').write_text(header(args.tgc),encoding='utf-8')
    if args.stage:
        stage=args.stage.resolve()
        if stage==Path(ROOT) or stage.is_relative_to(Path(ROOT)) or any((p/'v2game.exe').exists() for p in (stage,*stage.parents)):
            ap.error('use staging outside the repository and game')
        generate(stage,args.tgc)
    if not args.header and not args.stage:
        ap.error('use --header or --stage')


if __name__=='__main__':
    main()
