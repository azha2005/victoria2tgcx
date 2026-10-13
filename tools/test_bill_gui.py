"""Verify native Bill injection/metadata stays bounded and preserves the TGC base."""
from pathlib import Path
import sys
import unittest

from gen_laws import DEFAULT_TGC
from tgc_scan import parse, read
from tgcx_bill_gui import inject, layout, catalogue_header


class BillGuiTests(unittest.TestCase):
    def test_real_base_is_preserved_byte_for_byte_outside_insertion(self):
        original = read(Path(DEFAULT_TGC) / "interface/country_politics.gui")
        fragment, _ = layout()
        generated = inject(original, fragment)
        start = generated.index("\n# >>> tgcx_bill\n")
        end = generated.index("\n# <<< tgcx_bill\n") + len("\n# <<< tgcx_bill\n")
        self.assertEqual(generated[:start]+generated[end:], original)
        root = dict(parse(generated))["guiTypes"]
        country = next(v for k, v in root if k == "windowType" and dict(v).get("name") == "country_politics")
        self.assertEqual(sum(k == "windowType" and dict(v).get("name") == "tgcx_bill_window" for k, v in country), 1)

    def test_reject_missing_duplicate_and_already_modified_anchors(self):
        fragment, _ = layout()
        base = 'guiTypes={windowType={name="country_politics"}}'
        for bad in ("guiTypes={}", base+base, inject(base, fragment)):
            with self.assertRaises(ValueError): inject(bad, fragment)
        # Comments/strings containing braces do not affect insertion depth.
        original = 'guiTypes={windowType={name="country_politics" text="} {" # }\n }}'
        self.assertIn("# >>> tgcx_bill", inject(original, fragment))

    def test_read_only_fallback_layout_and_catalogue(self):
        fragment, loc = layout()
        self.assertIn("x=30000 y=30000", fragment)
        self.assertEqual(len([k for k in loc if k.startswith("tgcx_bill_bloc_") and k[-1].isdigit()]), 9)
        header = catalogue_header()
        self.assertEqual(header.count("    {"), 144)
        self.assertTrue(all(k.startswith("tgcx_") for k in loc))
        self.assertTrue(all(";" not in text for text in loc.values()))
        self.assertIn("undecided Aye 2", loc["tgcx_bill_prices"])
        self.assertNotIn("replace_path", fragment)


if __name__ == "__main__":
    unittest.main(testRunner=unittest.TextTestRunner(stream=sys.stdout, verbosity=2))
