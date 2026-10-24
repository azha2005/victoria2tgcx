"""Province Rename package checks and actual V2DLL loader handshake probe.

--v2dll-loader points to an unmodified V2DLL_ModLoader.cpp with V2ModAPI.h
beside it. Third-party source is copied only into ignored work/ fixtures.
The loader probe must refuse an unsupported host before calling patch APIs.
Native supported-image command execution is tested by tgcx_rename_tests.exe.
"""
import argparse
import hashlib
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path
from xml.sax.saxutils import escape
import zipfile
from PIL import Image
import prepare_province_rename as package
from tgcx_pe import PE
from tgcx_bases import Base, missing_fonts, missing_sprites
from tgc_scan import parse

ROOT = Path(__file__).resolve().parent.parent
WORK = ROOT/'work/province-rename'
DLL = ROOT/'dll/province_rename/Release/tgcx_province_rename.dll'
GAME = Path('E:/Victoria.II.v3.04.Inclu.ALL.DLC')


class PackageTests(unittest.TestCase):
    def test_schema_rejects_old_collision_and_opaque_labels(self):
        gui = package.layout()
        package.validate_layout(gui)
        for name in ('apply', 'cancel'):
            with self.assertRaisesRegex(ValueError, 'duplicate GUI definition'):
                package.validate_layout(gui.replace(f'name="tgcx_rename_{name}_label"',
                                                    f'name="tgcx_rename_{name}"'))
        with self.assertRaisesRegex(ValueError, 'pass clicks through'):
            package.validate_layout(gui.replace('allwaystransparent=yes', 'allwaystransparent=no', 1))
        self.assertNotIn('open_label',gui)

    def test_separate_package_and_zip(self):
        with tempfile.TemporaryDirectory(prefix='package-',dir=WORK) as temp:
            dest=Path(temp)/f'Province-Rename-{package.VERSION}'
            result=package.build(dest,DLL)
            self.assertEqual(Path(result['zip']).name,dest.name+'.zip')
            paths={p.relative_to(dest).as_posix() for p in dest.rglob('*') if p.is_file()}
            self.assertEqual(result['files'],12)
            self.assertEqual(len(paths),12)
            self.assertTrue(all(p=='README.txt' or p.startswith('mod/tgcx_province_rename') for p in paths))
            self.assertFalse(any(p.endswith('lua51.dll') or 'ATCU' in p or 'province_interface.gui' in p for p in paths))
            mod=(dest/'mod/tgcx_province_rename.mod').read_text()
            self.assertNotIn('replace_path',mod);self.assertNotIn('user_dir',mod);self.assertNotIn('dependencies',mod)
            self.assertIn('Province Rename (Universal)',mod)
            self.assertEqual((dest/'mod/tgcx_province_rename/v2dll_mod.dll').read_bytes(),DLL.read_bytes())
            gui=(dest/'mod/tgcx_province_rename/interface/tgcx_province_rename.gui').read_text()
            parsed=parse(gui)
            self.assertEqual(len(parsed),1)
            self.assertEqual(gui.count('editBoxType='),1)
            self.assertEqual(gui.count('guiButtonType='),3)
            gfx=(dest/'mod/tgcx_province_rename/interface/tgcx_province_rename.gfx').read_text()
            parse(gfx)
            for base in (Base(GAME,'vanilla'),Base(GAME,GAME/'mod/TGC.mod'),Base(GAME,GAME/'mod/TGC_v2.5.2.mod'),
                         Base(ROOT/'work/Victoria-II-HPM-Army090',ROOT/'work/Victoria-II-HPM-Army090/mod/tgcx_base_hpm4.5.mod'),
                         Base(ROOT/'work/Victoria-II-DoDFF-Army090',ROOT/'work/Victoria-II-DoDFF-Army090/mod/Divergences of Darkness Fan Fork.mod')):
                self.assertFalse(missing_fonts(base,gui))
                self.assertFalse(missing_sprites(base,gui,[f'GFX_tgcx_rename_{name}' for name in package.artwork()]))
                cursor=package.native_sprite(base.game,package.native_sprites(base.game,base),'cursor',profile=base)
                self.assertEqual((cursor.size,cursor.getbbox()),((32,32),(7,6,8,22)))
            loc=(dest/'mod/tgcx_province_rename/localisation/tgcx_province_rename.csv').read_bytes()
            self.assertFalse(loc.startswith(b'\xef\xbb\xbf'))
            lines=loc.decode('cp1252').splitlines()
            self.assertTrue(all(len(line.split(';'))==len(lines[0].split(';')) for line in lines))
            for name,img in package.artwork().items():
                with Image.open(dest/f'mod/tgcx_province_rename/gfx/interface/tgcx_rename_{name}.dds') as dds:
                    self.assertEqual(dds.convert('RGBA').tobytes(),img.tobytes())
            with zipfile.ZipFile(result['zip']) as archive:
                self.assertIsNone(archive.testzip())
                self.assertEqual(set(archive.namelist()),paths)
                for path in paths:self.assertEqual(archive.read(path),(dest/path).read_bytes())
            with self.assertRaises(ValueError):package.build(dest,DLL)

    def test_packaging_refuses_foreign_dll(self):
        with tempfile.TemporaryDirectory(prefix='foreign-',dir=WORK) as temp:
            with self.assertRaises(ValueError):package.build(Path(temp)/'out',ROOT/'dll/tgcx/Release/tgcx.dll')

    def test_export_and_geometry(self):
        self.assertEqual(PE(DLL).machine,0x14C)
        self.assertIn('V2Mod_Init',PE(DLL).exports())
        self.assertEqual(package.artwork()['entry'].size,(24,24))
        game=ROOT/'work/Victoria-II-Province-Rename-TGC252'
        base=Base(game,game/'mod/TGC_v2.5.2.mod')
        native=package.native_sprite(game,package.native_sprites(game,base),'GFX_bp_button_rename',profile=base)
        self.assertEqual(package.artwork()['entry'].tobytes(),native.resize((24,24),Image.Resampling.LANCZOS).tobytes())
        self.assertEqual(package.artwork()['panel'].size,(332,224))
        self.assertIn('size={x=296 y=30}',package.layout())
        import re
        gui=package.layout()
        title=re.search(r'instantTextBoxType=\{name="tgcx_rename_title"[^\n]+',gui).group()
        self.assertIn('font="vic_22"',title)
        fonts=re.findall(r'font="([^"]+)"',gui.replace(title,''))
        self.assertTrue(fonts and all(font.lower().endswith(('_black','_black_bold')) for font in fonts))


PROBE = r'''
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>
#include <stdlib.h>
DWORD g_base=0;
static unsigned patches=0,refused=0,inactive=0;
void Log(const char* fmt,...) {
    char text[2048];va_list args;va_start(args,fmt);vsprintf_s(text,fmt,args);va_end(args);
    puts(text);if(strstr(text,"refused (returned 0)"))++refused;
    if(strstr(text,"package not selected; inactive"))++inactive;
}
bool PatchSlot(DWORD,int,void*,void**) {++patches;return false;}
bool StealToTrampoline(DWORD,unsigned,unsigned char*,int,const unsigned char*,void*,void**,const char*) {++patches;return false;}
const char* V2_OwnDir() {return ".";}
#include "V2DLL_ModLoader.cpp"
int main(int argc,char** argv) {
    if(argc<3)return 2;g_base=(DWORD)GetModuleHandleW(NULL);
    char path[MAX_PATH];sprintf_s(path,"%s\\v2dll_mod.dll",argv[1]);
    HMODULE module=LoadLibraryA(path);if(!module)return 3;
    auto init=(tV2ModInit)GetProcAddress(module,"V2Mod_Init");if(!init)return 4;
    V2ModAPI bad={};bad.size=sizeof(bad);bad.apiVersion=2;
    if(init(NULL)!=0 || init(&bad)!=0)return 5;
    if(argc>3 && !strcmp(argv[3],"submods")){LoadV2Submods();LoadV2Submods();}
    else {LoadV2Mods(argv[1]);LoadV2Mods(argv[1]);}
    bool active=!strcmp(argv[2],"active");
    if(patches || g_modPluginCount!=1 || (active?(refused!=1 || inactive!=0):(inactive!=1 || refused!=0)))return 6;
    printf("V2DLL actual loader handshake OK; patch calls=%u; load-once=%d\n",patches,g_modPluginCount);
    return 0;
}
'''
SHARED = '''#pragma once
#include <windows.h>
extern DWORD g_base;
void Log(const char*,...);
bool PatchSlot(DWORD,int,void*,void**);
bool StealToTrampoline(DWORD,unsigned,unsigned char*,int,const unsigned char*,void*,void**,const char*);
const char* V2_OwnDir();
'''


def loader_probe(source):
    source=Path(source).resolve()
    with tempfile.TemporaryDirectory(prefix='v2dll-loader-',dir=WORK) as temp:
        root=Path(temp)
        shutil.copy2(source,root/'V2DLL_ModLoader.cpp')
        shutil.copy2(source.with_name('V2ModAPI.h'),root/'V2ModAPI.h')
        (root/'V2DLL_Shared.h').write_text(SHARED)
        (root/'probe.cpp').write_text(PROBE)
        vcx=(ROOT/'dll/province_rename/tgcx_rename_tests.vcxproj').read_text()
        vcx=vcx.replace('$(MSBuildThisFileDirectory)Release\\tests\\',escape(str(root)+'\\'))
        vcx=vcx.replace('tgcx_rename_tests.cpp','probe.cpp').replace('tgcx_rename_tests','tgcx_rename_probe')
        (root/'probe.vcxproj').write_text(vcx)
        vswhere=Path('C:/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe')
        vs=subprocess.check_output([str(vswhere),'-latest','-products','*','-requires','Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
                                    '-version','[17.0,18.0)','-property','installationPath'],text=True).strip()
        run=subprocess.run([str(Path(vs)/'MSBuild/Current/Bin/MSBuild.exe'),str(root/'probe.vcxproj'),'/t:Rebuild',
                            '/p:Configuration=Release','/p:Platform=Win32','/nologo','/verbosity:minimal'],capture_output=True,text=True)
        if run.returncode:raise RuntimeError(run.stdout+run.stderr)
        mod=root/'mod/tgcx_province_rename';mod.mkdir(parents=True)
        shutil.copy2(DLL,mod/'v2dll_mod.dll')
        (mod/'tgcx_package.txt').write_text('package = rename\n')
        (root/'mod/tgcx_province_rename.mod').write_text('name="Province Rename"\npath="mod/tgcx_province_rename"\n')
        evidence=[]
        for submods in (False,True):
            folder=mod
            if submods:
                folder=root/'dll_Submods';folder.mkdir();shutil.move(mod/'v2dll_mod.dll',folder/'v2dll_mod.dll')
            for active in (False,True):
                logpath=root/'Logs/tgcx_province_rename.log'
                if logpath.exists():logpath.unlink()
                command=[str(root/'tgcx_rename_probe.exe'),str(folder),'active' if active else 'inactive']
                if submods:command+=['submods']
                if active:command+=['-mod=mod/tgcx_province_rename.mod']
                run=subprocess.run(command,cwd=root,capture_output=True,text=True,timeout=30)
                if run.returncode:raise RuntimeError(f'loader probe exit {run.returncode}\n'+run.stdout+run.stderr)
                if active:
                    log=logpath.read_text()
                    if 'BAD sha256' not in log or 'no game memory read or patched' not in log:raise AssertionError(log)
                elif logpath.exists():raise AssertionError('inactive DLL wrote log')
                evidence.append({'selected':active,'source':'dll_Submods' if submods else 'mod folder','output':run.stdout})
        return {'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'probes':evidence}


if __name__ == '__main__':
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--v2dll-loader',type=Path)
    args=ap.parse_args()
    WORK.mkdir(parents=True,exist_ok=True)
    result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(PackageTests))
    if not result.wasSuccessful():raise SystemExit(1)
    if args.v2dll_loader:
        evidence=loader_probe(args.v2dll_loader)
        (WORK/'loader-probe.json').write_text(json.dumps(evidence,indent=2)+'\n')
        print(json.dumps(evidence,indent=2))
