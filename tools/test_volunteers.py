"""Stage integrity and actual x86 DLL loader probes; no game launch/install."""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess
import tempfile
import unittest
import zipfile
import prepare_volunteers as package
from volunteers_test_diagnostics import light_settings
from tgcx_pe import PE

ROOT=Path(__file__).resolve().parent.parent
DLL=ROOT/'rust/volunteers/target/i686-pc-windows-msvc/release/tgcx_volunteers.dll'
PROBE=DLL.parent/'tgcx_volunteers_probe.exe'
WORK=ROOT/'work/volunteers-0013'

class VolunteersTests(unittest.TestCase):
    def test_light_diagnostics_changes_only_two_probe_values(self):
        source=(b'LOCAL_MOD_CONFIG=0\r\nENABLE_LOG=1\r\nDEBUG_LOG=0\r\n'
                b'ENABLE_OOS_LOG=1\r\nENABLE_STATE_PROBE=1\r\n'
                b'ENABLE_RNG_PROBE=1\r\nENABLE_CRASH_LOG=1\r\n'
                b'PATCH_STATE_SAVINGS_CAP=1\r\nSTATE_SAVINGS_CAP_POUNDS=60000\r\n')
        expected=source.replace(b'ENABLE_STATE_PROBE=1',b'ENABLE_STATE_PROBE=0').replace(b'ENABLE_RNG_PROBE=1',b'ENABLE_RNG_PROBE=0')
        self.assertEqual(light_settings(source),expected)
        self.assertEqual(light_settings(expected),expected)
        with self.assertRaises(ValueError):light_settings(source+b'ENABLE_STATE_PROBE=1\r\n')
        with self.assertRaises(ValueError):light_settings(source.replace(b'ENABLE_RNG_PROBE=1',b'ENABLE_RNG_PROBE=2'))
    def test_additive_package_and_exact_manifest(self):
        with tempfile.TemporaryDirectory(dir=WORK) as temp:
            dest=Path(temp)/'candidate';result=package.build(dest,DLL)
            manifest=json.loads((dest/'MANIFEST.json').read_text())
            self.assertEqual(result['files'],17)
            for name,digest in manifest.items():self.assertEqual(hashlib.sha256((dest/name).read_bytes()).hexdigest(),digest)
            descriptor=(dest/'mod/tgcx_volunteers.mod').read_text()
            for forbidden in ('replace_path','user_dir','dependencies'):self.assertNotIn(forbidden,descriptor)
            self.assertFalse(any(p.name=='lua51.dll' for p in dest.rglob('*')))
            self.assertEqual([p.name for p in dest.rglob('*.gui')],['tgcx_vl_windows.gui'])
            self.assertEqual(len(list(dest.rglob('tgcx_vl_*.tga'))),8)
            self.assertEqual((dest/'mod/tgcx_volunteers/v2dll_mod.dll').read_bytes(),DLL.read_bytes())
            with zipfile.ZipFile(result['zip']) as z:
                self.assertIsNone(z.testzip())
                self.assertEqual(len(z.namelist()),result['files'])
                for name in z.namelist():self.assertEqual(z.read(name),(dest/name).read_bytes())
            with self.assertRaisesRegex(ValueError,'must be new'):package.build(dest,DLL)

    def test_identical_inputs_produce_identical_zip_bytes(self):
        with tempfile.TemporaryDirectory(dir=WORK) as temp:
            a=package.build(Path(temp)/'a',DLL);b=package.build(Path(temp)/'b',DLL)
            self.assertEqual(Path(a['zip']).read_bytes(),Path(b['zip']).read_bytes())

    def test_x86_exports_and_wrong_dll_refusal(self):
        image=PE(DLL)
        self.assertEqual(image.machine,0x14C)
        self.assertEqual(set(image.exports()),{'V2Mod_Init','tgcx_VolunteersVersion'})
        with tempfile.TemporaryDirectory(dir=WORK) as temp:
            with self.assertRaisesRegex(ValueError,'matching x86'):package.build(Path(temp)/'bad',PROBE)

    def probe(self,active):
        with tempfile.TemporaryDirectory(dir=WORK) as temp:
            fixture=Path(temp);shutil.copyfile(PROBE,fixture/'probe.exe')
            built=package.build(fixture/'stage',DLL)
            shutil.copytree(fixture/'stage/mod',fixture/'mod')
            command=[str(fixture/'probe.exe'),str(DLL)]
            if active:command+=['--active','-mod=mod/tgcx_volunteers.mod']
            result=subprocess.run(command,cwd=fixture,capture_output=True,text=True)
            self.assertEqual(result.returncode,0,result.stdout+result.stderr)
            log=fixture/'Logs/tgcx_volunteers.log'
            if active:
                text=log.read_text();self.assertIn('Unsupported executable',text)
                self.assertNotIn('observer_installed=true',text)
            else:self.assertFalse(log.exists())
            self.assertTrue(Path(built['zip']).exists())

    def test_actual_dll_refuses_selected_unsupported_host(self):self.probe(True)
    def test_actual_dll_inactive_when_unticked(self):self.probe(False)

if __name__=='__main__':
    WORK.mkdir(parents=True,exist_ok=True)
    unittest.main()
