"""Plain-package checks against the accepted local payload; no game launch."""
import json
from pathlib import Path
import tempfile
import unittest
import zipfile
import prepare_volunteers_share as share


class ShareTests(unittest.TestCase):
    def test_plain_zip_keeps_tested_payload_and_has_no_scripts(self):
        with tempfile.TemporaryDirectory() as temp:
            out=Path(temp)/'Volunteers'
            result=share.build(out)
            self.assertEqual(result['files'],20)
            with zipfile.ZipFile(result['zip']) as z:
                files={n:z.read(n) for n in z.namelist()}
            self.assertFalse(any(Path(n).suffix.lower() in {'.exe','.bat','.cmd','.ps1','.py','.vbs','.js','.lnk'} for n in files))
            self.assertEqual([n for n in files if n.endswith('.dll')],['mod/tgcx_volunteers/v2dll_mod.dll'])
            self.assertEqual(len([n for n in files if n.endswith('.tga')]),8)
            self.assertEqual(share.sha(files['mod/tgcx_volunteers/v2dll_mod.dll']),share.ACCEPTED_DLL)
            self.assertIn(b'mode=prototype\n',files['mod/tgcx_volunteers/tgcx_volunteers.cfg'])
            manifest=json.loads(files.pop('SHA256.json'))
            self.assertEqual(manifest,{n:share.sha(b) for n,b in files.items()})
            source=json.loads((share.ACCEPTED_PACKAGE/'MANIFEST.json').read_text())
            changed={'mod/tgcx_volunteers.mod','mod/tgcx_volunteers/tgcx_volunteers.cfg'}
            for name in source:
                if name.startswith('mod/') and name not in changed:
                    self.assertEqual(files[name],(share.ACCEPTED_PACKAGE/name).read_bytes())
            for descriptor in ['mod/tgcx_loader_data.mod','mod/tgcx_volunteers.mod']:
                self.assertNotIn(b'replace_path',files[descriptor])
                self.assertNotIn(b'user_dir',files[descriptor])
            self.assertIn('SINGLE PLAYER ONLY',files['README.txt'].decode('ascii'))
            with self.assertRaisesRegex(ValueError,'must be new'):share.build(out)

    def test_same_inputs_produce_identical_zip(self):
        with tempfile.TemporaryDirectory() as temp:
            a=share.build(Path(temp)/'a');b=share.build(Path(temp)/'b')
            self.assertEqual(Path(a['zip']).read_bytes(),Path(b['zip']).read_bytes())


if __name__=='__main__':unittest.main()
