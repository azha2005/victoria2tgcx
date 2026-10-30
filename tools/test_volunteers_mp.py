"""Check the actual x86 MP diagnostic payload, without launching Victoria II."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import zipfile
import prepare_volunteers_mp as package


class MPProbePackageTests(unittest.TestCase):
    def test_gameplay_package_is_explicit_and_does_not_ship_scripts(self):
        package.WORK.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(dir=package.WORK) as temp:
            result=package.build(Path(temp)/'gameplay',gameplay=True)
            self.assertTrue(result['mp_volunteer_gameplay'])
            self.assertEqual(result['mode'],'mp')
            with zipfile.ZipFile(result['zip']) as z:
                self.assertIsNone(z.testzip())
                self.assertIn(b'mode=mp\n',z.read('mod/tgcx_volunteers/tgcx_volunteers.cfg'))
                self.assertIn(b'WHOLE EXISTING ARMIES ONLY',z.read('README.txt'))
                self.assertIn(b'No exact loader hash or private DLL RVA',z.read('README.txt'))
                self.assertIn(b'Existing\n  hooks are left in place',z.read('README.txt'))
                self.assertNotIn(b'NOT MP VOLUNTEER GAMEPLAY',z.read('README.txt'))
                self.assertFalse(any(Path(p).suffix.lower() in {'.exe','.bat','.cmd','.ps1','.py','.js','.vbs','.lnk'} for p in z.namelist()))

    def test_script_free_native_payload_and_reproducible_archive(self):
        package.WORK.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(dir=package.WORK) as temp:
            root=Path(temp)
            a=package.build(root/'a');b=package.build(root/'b')
            self.assertEqual(a['sha256'],b['sha256'])
            self.assertFalse(a['mp_volunteer_gameplay'])
            self.assertEqual(a['files'],19)
            with zipfile.ZipFile(a['zip']) as z:
                self.assertIsNone(z.testzip())
                self.assertFalse(any(Path(p).suffix.lower() in {'.exe','.bat','.cmd','.ps1','.py','.js','.vbs','.lnk'} for p in z.namelist()))
                manifest=json.loads(z.read('SHA256.json'))
                self.assertEqual(set(z.namelist()),set(manifest)|{'SHA256.json'})
                for name,digest in manifest.items():self.assertEqual(hashlib.sha256(z.read(name)).hexdigest(),digest)
                config=z.read('mod/tgcx_volunteers/tgcx_volunteers.cfg')
                self.assertIn(b'mode=mp_probe\n',config)
                self.assertNotIn(b'mode=prototype',config)
                self.assertEqual(sum(p.endswith('.dll') for p in z.namelist()),1)
                self.assertEqual(sum(p.endswith('.tga') for p in z.namelist()),8)
                for name in ('mod/tgcx_volunteers.mod','mod/tgcx_loader_data.mod'):
                    self.assertNotIn(b'replace_path',z.read(name))
                self.assertIn(b'NOT MP VOLUNTEER GAMEPLAY',z.read('README.txt'))
            with self.assertRaisesRegex(ValueError,'must be new'):package.build(root/'a')


if __name__=='__main__':unittest.main()
