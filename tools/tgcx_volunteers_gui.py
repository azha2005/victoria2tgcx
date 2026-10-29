"""Generate the volunteer GUI from installed native art; derived pixels stay private."""
from pathlib import Path
import re
from PIL import Image
from tgcx_army_art import sprites, sprite
from tgcx_bases import Base

PREFIX='tgcx_vl_'
ROOT=Path(__file__).resolve().parent.parent
DEFAULT_GAME=Path('E:/Victoria-II-Volunteers-0004')
BUTTONS=['open','close','cancel','send','army_prev','army_next','recipient_prev','recipient_next',
         'page_prev','page_next','row0','row1','mission_prev','mission_next','recall']

def artwork(game=DEFAULT_GAME):
    game=Path(game);base=Base(game,game/'mod/TGC_v2.5.2.mod');table=sprites(game,base)
    def native(name,**kw):return sprite(game,table,name,profile=base,**kw)
    body=native('GFX_unit_bottom_bg');top=native('GFX_unit_toppanel_bg',frames=2)
    row=native('GFX_unit_entry_bg');footer=native('GFX_unit_gunboat_diplo_button')
    if body.size!=(332,408) or top.size!=(332,64) or row.size!=(317,30) or footer.size!=(148,28):
        raise ValueError('native army art dimensions changed; refuse guessed layout')
    def panel(height):
        out=Image.new('RGBA',(332,height));parchment=body.crop((0,80,332,360))
        for y in range(30,height-35,parchment.height):
            h=min(parchment.height,height-35-y);out.paste(parchment.crop((0,0,332,h)),(0,y))
        out.paste(body.crop((0,373,332,408)),(0,height-35))
        out.alpha_composite(top.crop((0,0,332,27)),(0,0))
        # Accepted Conform header preparation: native green strip and braid.
        out.alpha_composite(top.crop((51,5,324,27)).resize((317,22)),(7,5))
        out.alpha_composite(top.crop((0,58,332,64)),(0,27))
        return out
    plain=row.copy();plain.paste(row.crop((40,0,150,30)).resize((311,30)),(3,0))
    selector=plain.crop((0,0,263,24));selector.paste(plain.crop((311,0,317,24)),(257,0))
    chooser=panel(378);forces=panel(266)
    chooser.alpha_composite(selector,(39,83));chooser.alpha_composite(selector,(7,115))
    for y in (148,180):chooser.alpha_composite(plain,(7,y))
    for y in (42,117):
        card=Image.new('RGBA',(317,66));card.paste(plain.crop((0,0,317,10)),(0,0))
        card.paste(plain.crop((0,10,317,20)).resize((317,46)),(0,10));card.paste(plain.crop((0,20,317,30)),(0,56))
        forces.alpha_composite(card,(7,y))
    return {'chooser':chooser,'forces':forces,'button':footer,
        'entry':native('GFX_button_thin_160'),'row':Image.new('RGBA',(317,30)),
        'close':native('GFX_unit_close'),
        'left':native('GFX_button_arrow_left').resize((24,24),Image.Resampling.LANCZOS),
        'right':native('GFX_button_arrow_right').resize((24,24),Image.Resampling.LANCZOS)}

def layout():
    strings={};text_meta=[];button_meta=[]
    def text(name,value,x,y,w,h=18,font='Arial14_black',align='left',window='chooser'):
        key=PREFIX+name;strings[key]=value;text_meta.append((window,key,value,x,y,w,h,font,align))
        # Runtime fills every label: unloaded localisation must never expose keys.
        return f'instantTextBoxType={{name="{key}" position={{x={x} y={y}}} font="{font}" text="" maxWidth={w} maxHeight={h} fixedsize=yes format={align} allwaystransparent=yes}}'
    def btn(name,x,y,sprite='left',window='chooser'):
        key=PREFIX+name;button_meta.append((window,key,sprite,x,y))
        return f'guiButtonType={{name="{key}" position={{x={x} y={y}}} quadTextureSprite="{PREFIX}{sprite}" clicksound=click}}'
    def window(name,w,h,background,parts):
        return [f'windowType={{name="{PREFIX}{name}" position={{x=30000 y=30000}} size={{x={w} y={h}}} moveable=0 fullScreen=no orientation="UPPER_LEFT" backGround="{PREFIX}{name}_bg"',
            f'guiButtonType={{name="{PREFIX}{name}_bg" position={{x=0 y=0}} quadTextureSprite="{PREFIX}{background}"}}',*parts,'}']
    chooser=[text('title','Send volunteers',12,7,280,22,'vic_18'),btn('close',303,7,'close'),
        text('donor','From:',14,40,300,20,'vic_18_black'),text('recipient_label','Recipient',14,65,290,16,'Arial12_bold_black'),
        btn('recipient_prev',9,83),text('recipient','Choose recipient',43,87,250,18,'Arial14_black','centre'),btn('recipient_next',300,83,'right'),
        text('army','Selected army',14,120,245,18,'Arial14_black'),btn('army_prev',272,115),btn('army_next',300,115,'right'),
        btn('row0',7,148,'row'),btn('row1',7,180,'row'),text('row0_label','No brigades selected',16,155,295,18,'Arial12_bold_black'),text('row1_label','',16,187,295,18,'Arial12_bold_black'),
        text('selected','0 brigades selected - limit 5',14,222,242,18,'Arial12_black'),btn('page_prev',272,215),btn('page_next',300,215,'right'),
        text('rule_command','You own, command and supply the volunteers.',14,252,304,16,'Arial12_black'),
        text('rule_war','Fight for the recipient while you stay at peace.',14,271,304,16,'Arial12_black'),
        text('rule_flag','Your flag and military statistics are retained.',14,290,304,16,'Arial12_black'),
        text('status','Preview only - dispatch is not available.',14,311,304,20,'Arial12_black'),
        btn('cancel',12,342,'button'),text('cancel_label','Cancel',12,348,148,18,'vic_18_black','centre'),
        btn('send',172,342,'button'),text('send_label','Send volunteers',172,348,148,18,'vic_18_black','centre')]
    def ft(*args,**kw):return text(*args,**kw,window='forces')
    forces=[ft('forces_title','Volunteer forces',12,7,300,22,'vic_18'),
        ft('mission0','No active volunteers',16,52,299,48,'Arial12_black'),ft('mission1','',16,127,299,48,'Arial12_black'),
        btn('mission_prev',9,231,window='forces'),btn('mission_next',37,231,'right',window='forces'),ft('mission_page','1',67,235,60,18,'Arial12_black','centre'),
        btn('recall',172,230,'button',window='forces'),ft('recall_label','Recall selected',172,236,148,18,'vic_18_black','centre')]
    entry=[btn('open',0,0,'entry',window='entry'),text('open_label','Send volunteers',0,2,160,20,'vic_18_black','centre',window='entry')]
    gui=['guiTypes={',*window('chooser',332,378,'chooser',chooser),*window('forces',332,266,'forces',forces),
        f'windowType={{name="{PREFIX}entry" position={{x=30000 y=30000}} size={{x=160 y=24}} moveable=0 fullScreen=no orientation="UPPER_LEFT"',*entry,'}','}']
    gui='\n'.join(gui)+'\n';names=re.findall(r'\bname\s*=\s*"([^"]+)"',gui)
    if len(names)!=len(set(names)) or any(not name.startswith(PREFIX) for name in names):raise ValueError('P59: duplicate/unprefixed GUI name')
    if any(';' in v or '\n' in v or '\r' in v for v in strings.values()):raise ValueError('P68: unsafe localisation value')
    return gui,strings,text_meta,button_meta

def generate(package:Path,game=DEFAULT_GAME):
    interface=package/'interface';gfx=package/'gfx/interface';loc=package/'localisation'
    for folder in (interface,gfx,loc):folder.mkdir(parents=True,exist_ok=True)
    assets=artwork(game)
    for name,image in assets.items():image.save(gfx/f'{PREFIX}{name}.tga')
    lines=['spriteTypes={']+[f'spriteType={{name="{PREFIX}{name}" texturefile="gfx/interface/{PREFIX}{name}.tga" noOfFrames=1 loadType="INGAME"}}' for name in assets]+['}']
    (interface/f'{PREFIX}sprites.gfx').write_bytes(('\n'.join(lines)+'\n').encode('cp1252'))
    gui,strings,_,_=layout();(interface/f'{PREFIX}windows.gui').write_bytes(gui.encode('cp1252'))
    header='CODE;ENGLISH;FRENCH;GERMAN;POLISH;SPANISH;ITALIAN;SWEDISH;CZECH;HUNGARIAN;DUTCH;PORTUGUESE;RUSSIAN;FINNISH;x\n'
    (loc/f'{PREFIX}text.csv').write_bytes((header+''.join(k+';'+v+';'*13+'x\n' for k,v in strings.items())).encode('cp1252'))
    return assets

def preview(path,game=DEFAULT_GAME):
    from prepare_province_rename import bitmap_label
    assets=artwork(game);_,_,labels,buttons=layout();canvas=Image.new('RGBA',(664,378))
    for window,ox in [('chooser',0),('forces',332)]:
        canvas.alpha_composite(assets[window],(ox,0))
        for host,key,art,x,y in buttons:
            if host==window:canvas.alpha_composite(assets[art],(ox+x,y))
        for host,key,value,x,y,w,h,font,align in labels:
            if host!=window:continue
            sample={'donor':'From: FRA','recipient':'AUS - at peace','army':'1ere Division d\'Hussard','row0_label':'[x] 1ere Division d\'Hussard',
                'row1_label':'[ ] 2ere Division d\'Hussard','selected':'1 brigade selected - limit 5'}.get(key.removeprefix(PREFIX),value)
            atlas_font={'Arial12_black':'Arial12','Arial12_bold_black':'Arial12_bold','Arial14_black':'Arial14'}.get(font,font)
            bitmap_label(canvas,ox+x,y,sample,atlas_font,'#000000' if font.endswith('_black') else '#f8f1d7',w if align=='centre' else None)
    canvas.save(path)

def preview_entry(path,game=DEFAULT_GAME):
    """Generated native-art design detail, not an in-game screenshot."""
    from prepare_province_rename import bitmap_label
    asset=artwork(game)['entry']
    if asset.size!=(160,24):raise ValueError('native diplomacy button dimensions changed')
    canvas=Image.new('RGBA',(320,72))
    captions=['War Subsidies','Remove from Sphere','Declare War','Justify war',
              'Command Units','Send volunteers']
    for index,caption in enumerate(captions):
        x=(index%2)*160;y=(index//2)*24
        canvas.alpha_composite(asset,(x,y))
        bitmap_label(canvas,x,y+2,caption,'vic_18_black','#000000',160)
    canvas.save(path)

if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('package',type=Path);parser.add_argument('--game',type=Path,default=DEFAULT_GAME)
    args=parser.parse_args();generate(args.package,args.game)
