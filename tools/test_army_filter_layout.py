"""Filter controls preserve the accepted panel bounds and avoid native rows."""
import unittest
from pathlib import Path
import tempfile

from tgc_scan import parse
import tgcx_templates_gui as tpl
import tgcx_conform_gui as cf
import build_universal_army as package


def root_size(fragment, name):
    roots = parse('guiTypes={' + fragment + '}')[0][1]
    window = next(dict(v) for k, v in roots
                  if k == 'windowType' and dict(v).get('name') == name)
    return tuple(int(dict(window['size'])[axis]) for axis in ('x', 'y'))


class FilterLayoutTests(unittest.TestCase):
    def test_old_logic_cannot_ship_with_new_filter_gui(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            dll = folder / 'old.dll'
            dll.write_bytes(b'TGCX MOD-6 0.10.4 tgcx.dll')
            with self.assertRaisesRegex(ValueError, 'matching 0.11.2'):
                package.build(folder / 'output', folder / 'art', dll, folder / 'loader')
            self.assertFalse((folder / 'output').exists())

    def test_builder_filter_band_and_footer_fit_existing_frame(self):
        fragment, _ = tpl.layout(universal=True)
        self.assertEqual(root_size(fragment, 'tgcx_tpl_window'), (652, 528))
        controls = {name: (x, y, w, h) for name, _, x, y, w, h in tpl.buttons()}
        row_bottom = 186 + (tpl.ROWS - 1) * 31 + 30
        for name in ('culture_accepted', 'culture_primary'):
            x, y, w, h = controls[name]
            self.assertGreaterEqual(y, row_bottom)
            self.assertLessEqual(y + h, 466)
            self.assertLessEqual(x + w, 405)
        for name in ('continent_prev', 'continent_next'):
            x, y, w, h = controls[name]
            self.assertGreaterEqual(x, 231)
            self.assertGreaterEqual(y, 466)
            self.assertLessEqual(x + w, 652)
            self.assertLessEqual(y + h, 494)

    def test_conform_culture_band_keeps_changes_and_footer_clear(self):
        fragment, _ = cf.layout(universal=True)
        self.assertEqual(root_size(fragment, 'tgcx_cf_window'), (332, 594))
        self.assertEqual(cf.CHANGES, 7)
        self.assertLessEqual(cf.SLOT_Y + (cf.SLOTS - 1) * 32 + cf.ROW_H, 202)
        for name, _, x, y, w, h in cf.buttons():
            if name.startswith('culture_'):
                self.assertGreaterEqual(y, 198)
                self.assertLessEqual(y + h, 230)
                self.assertLessEqual(x + w, 332)
        self.assertLessEqual(cf.CHANGE_Y + (cf.CHANGES - 1) * 32 + cf.ROW_H, 492)

    def test_generated_names_and_text_bounds(self):
        for module, width, height in ((tpl, 652, 528), (cf, 332, 594)):
            fragment, loc = module.layout(universal=True)
            seen = []
            def visit(items):
                for _, value in items:
                    if isinstance(value, list):
                        name = dict(value).get('name')
                        if name:
                            seen.append(name)
                        visit(value)
            visit(parse(fragment))
            self.assertEqual(len(seen), len(set(seen)))
            self.assertTrue(all(key.startswith('tgcx_') for key in loc))
            for name, _, x, y, w, h, _, _ in module.texts():
                if name == 'title':
                    continue
                self.assertTrue(0 <= x and x + w <= width and 0 <= y and y + h <= height, name)


if __name__ == '__main__':
    unittest.main()
