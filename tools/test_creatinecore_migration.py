"""Migration policy tests. No game, upstream files or filesystem writes."""
import unittest
from tgcx_creatinecore_migration import defines, migrate, render, settings


class MigrationTests(unittest.TestCase):
    def test_empty_values_keep_following_lines_and_comments(self):
        template='EMPTY=\r\nNEXT=1\r\nLIST=lumber; inline comment\r\nLAST=\r\n'
        result=render(template, {'EMPTY':'','NEXT':'0','LIST':'fishery','LAST':''})
        self.assertEqual(result,'EMPTY=\r\nNEXT=0\r\nLIST=fishery; inline comment\r\nLAST=\r\n')
        self.assertEqual(settings(result),{'EMPTY':'','NEXT':'0','LIST':'fishery','LAST':''})
    def test_preserves_options_and_separates_performance(self):
        old = 'ALWAYS_ADD_WARGOALS=1\nPATCH_POP_QUANTIZE=1\nFIX_ARMY_WINDOW_LAG=1\nDEBUG_LOG=0\nENABLE_STATE_PROBE=0\nENABLE_RNG_PROBE=0\nPATCH_GRAPH_POINT_CLAMP=0\n'
        core = 'ALWAYS_ADD_WARGOALS=0\nDEBUG_LOG=1\nENABLE_STATE_PROBE=1\nENABLE_RNG_PROBE=1\nLOCAL_MOD_CONFIG=1\nLOAD_DLL_FROM_MOD_FOLDER=0\nFACTORY_DELETE_CLOSED=1\n'
        perf = 'PATCH_POP_QUANTIZE=0\nFIX_ARMY_WINDOW_LAG=1\nDEBUG_LOG=1\n'
        a, b, report = migrate(old, core, perf)
        self.assertEqual(settings(a)['ALWAYS_ADD_WARGOALS'], '1')
        self.assertEqual(settings(a)['FACTORY_DELETE_CLOSED'], '0')
        self.assertEqual(settings(a)['ENABLE_STATE_PROBE'], '0')
        self.assertEqual(settings(a)['ENABLE_RNG_PROBE'], '0')
        self.assertEqual(settings(a)['LOCAL_MOD_CONFIG'], '0')
        self.assertEqual(settings(a)['LOAD_DLL_FROM_MOD_FOLDER'], '1')
        self.assertEqual(settings(b), {'PATCH_POP_QUANTIZE':'1','FIX_ARMY_WINDOW_LAG':'0','DEBUG_LOG':'0'})
        self.assertEqual(report['retired_disabled_keys'], ['PATCH_GRAPH_POINT_CLAMP'])

    def test_rejects_unknown_enabled_removed_or_duplicate_options(self):
        for old in ['UNKNOWN=1\n', 'PATCH_GRAPH_POINT_CLAMP=1\n', 'KNOWN=1\nKNOWN=0\n']:
            with self.assertRaises(ValueError):migrate(old, 'KNOWN=0\n', 'FIX_ARMY_WINDOW_LAG=0\nDEBUG_LOG=0\n')

    def test_define_values_and_empty_formula_preserved_without_source_comments(self):
        text = defines('# Upstream generated comment\nfactory_unprofitable_close_days=47\nminting_formula=\nfactory_min_wage_per_10000=7.5\n')
        self.assertIn('factory_unprofitable_close_days = 47\n', text)
        self.assertIn('factory_min_wage_per_10000 = 7.5\n', text)
        self.assertIn('minting_formula = \n', text)
        self.assertIn('factory_closed_delete_days = 90\n', text)
        self.assertNotIn('Upstream generated comment', text)
        with self.assertRaises(ValueError):defines('unknown_define=1\n')


if __name__ == '__main__':unittest.main()
