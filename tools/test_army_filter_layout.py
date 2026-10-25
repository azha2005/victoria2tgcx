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
    def test_replace_is_independent_and_buttons_sit_beside_native_actions(self):
        fragment, _ = cf.layout(universal=True)
        roots = {dict(v)['name']: v for k, v in parse('guiTypes={'+fragment+'}')[0][1]}
        direct = lambda name: {dict(v).get('name') for k, v in roots[name] if isinstance(v, list)}
        self.assertTrue({'tgcx_cf_open', 'tgcx_cf_replace'} <= direct('tgcx_cf_tab'))
        self.assertNotIn('tgcx_cf_replace', direct('tgcx_cf_window'))
        self.assertEqual(root_size(fragment, 'tgcx_cf_rp_window'), (332, 270))
        controls = {n: (x,y,w,h) for n,_,x,y,w,h in cf.buttons()}
        self.assertEqual(controls['open'], (21,0,20,20))
        self.assertEqual(controls['replace'], (0,0,20,20))
        # TGC: native Disband y=32; our pair shares that row, ending before
        # the first native action (red disband-too-small at x=214).
        for name in ('open', 'replace'):
            self.assertEqual(cf.OPEN[1],32)
            self.assertLessEqual(cf.OPEN[0]+controls[name][0]+controls[name][2],213)
        self.assertEqual(controls['conform'],(12,558,148,28))
        self.assertEqual(controls['cancel'],(172,558,148,28))
        for name in ('accepted','primary','noncore'):
            self.assertIn(f'tgcx_cf_rp_culture_{name}',direct('tgcx_cf_rp_window'))
            x,y,w,h=controls[f'rp_culture_{name}']
            self.assertTrue(97 <= y and y+h <= 151)

    def test_old_logic_cannot_ship_with_new_filter_gui(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            dll = folder / 'old.dll'
            dll.write_bytes(b'TGCX MOD-6 0.10.4 tgcx.dll')
            with self.assertRaisesRegex(ValueError, 'matching 0.11.16'):
                package.build(folder / 'output', folder / 'art', dll, folder / 'loader')
            self.assertFalse((folder / 'output').exists())

    def test_builder_filter_band_and_footer_fit_existing_frame(self):
        fragment, _ = tpl.layout(universal=True)
        self.assertEqual(root_size(fragment, 'tgcx_tpl_window'), (652, 528))
        controls = {name: (x, y, w, h) for name, _, x, y, w, h in tpl.buttons()}
        row_bottom = 186 + (tpl.ROWS - 1) * 31 + 30
        for name in ('culture_accepted', 'culture_primary', 'culture_noncore'):
            x, y, w, h = controls[name]
            self.assertGreaterEqual(y, row_bottom)
            self.assertLessEqual(y + h, 466)
            self.assertLessEqual(x + w, 405)
        # TPL-8: the three labels stay on one row, clear of the next box and
        # of the left column's edge.
        labels = {name: (x, w) for name, _, x, _, w, *_ in tpl.texts()}
        boxes = [controls[f'culture_{k}'][0] for k in ('accepted', 'primary', 'noncore')]
        for k, nxt in (('accepted', boxes[1]), ('primary', boxes[2]), ('noncore', 405)):
            x, w = labels[f'culture_{k}']
            self.assertLessEqual(x + w, nxt, k)
        for name in ('continent_prev', 'continent_next', 'gather_prev', 'gather_next'):
            x, y, w, h = controls[name]
            self.assertGreaterEqual(x, 231)
            self.assertGreaterEqual(y, 466)
            self.assertLessEqual(x + w, 652)
            self.assertLessEqual(y + h, 494)
        # TPL-11: Gather at sits right of Recruit from, clear of its next arrow.
        x, w = labels['gather_caption']
        self.assertGreaterEqual(x, controls['continent_next'][0] + 24)
        self.assertLessEqual(x + w, controls['gather_prev'][0])
        x, w = labels['gather']
        self.assertEqual(x, controls['gather_prev'][0] + 24)
        self.assertLessEqual(x + w, controls['gather_next'][0])

    def test_conform_culture_band_keeps_changes_and_footer_clear(self):
        fragment, _ = cf.layout(universal=True)
        self.assertEqual(root_size(fragment, 'tgcx_cf_window'), (332, 594))
        # TPL-8: two culture lines; Changes keeps its divider and six rows.
        self.assertEqual(cf.CHANGES, 6)
        self.assertLessEqual(cf.SLOT_Y + (cf.SLOTS - 1) * 32 + cf.ROW_H, cf.BAND_Y)
        lines = set()
        for name, _, x, y, w, h in cf.buttons():
            if name.startswith('culture_'):
                self.assertGreaterEqual(y, cf.BAND_Y)
                self.assertLessEqual(y + h, cf.BAND_Y + cf.BAND_H)
                self.assertLessEqual(x + w, 332)
                lines.add(y)
        self.assertEqual(len(lines), 2)
        self.assertLessEqual(cf.BAND_Y + cf.BAND_H, cf.CHANGES_Y)
        self.assertLessEqual(cf.CHANGES_Y + 28, cf.CHANGE_Y)
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
