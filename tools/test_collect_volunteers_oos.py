"""Check current/legacy loader evidence on owned fixtures, never real peers."""
import json
from pathlib import Path
import tempfile
import unittest
import collect_volunteers_oos as collector


class CollectorTests(unittest.TestCase):
    def test_mp_receipts_and_first_unambiguous_mismatch_exclude_failed_or_old_loads(self):
        a,b='a'*64,'b'*64
        def sample(day,digest,extra=''):
            return f'mp_sync_sample date={day} journals=1 digest={digest} bootstrap_complete=true refused=0 missed=0 {extra}\n'
        receipt=f'mp_probe_execute date=Some(24) country=1 sequence=2 fingerprint={a} matches_local=true\n'
        old=sample(1,b)+'native_load_completed valid=true\n'
        host=collector.mp_trace(old+receipt+sample(24,a)+sample(48,a)+sample(48,b)+sample(72,a))
        client=collector.mp_trace(receipt+sample(24,a)+sample(48,a)+sample(72,b))
        result=collector.compare_mp({'mp_traces':{'one':host}}, {'mp_traces':{'two':client}})
        self.assertEqual(result['common_execution_receipts'],1)
        self.assertEqual(result['ambiguous_sample_dates'],[48])
        self.assertEqual(result['first_mirror_digest_mismatch'],72)
        self.assertEqual(result['comparable_sample_dates'],2)
        self.assertEqual(collector.compare_mp({}, {})['first_mirror_digest_mismatch'],None)
        bad=collector.mp_trace(receipt.replace('matches_local=true','matches_local=false')+
                               sample(24,a).replace('refused=0','refused=1'))
        result=collector.compare_mp({'mp_traces':{'x':bad}}, {'mp_traces':{'x':client}})
        self.assertEqual(result['comparable_sample_dates'],0)
        self.assertEqual(len(result['fingerprint_refusals']['host']),1)
    def test_current_core_plugins_and_logs_are_not_confused_with_legacy(self):
        collector.WORK.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(dir=collector.WORK) as temp:
            root=Path(temp);game=root/'game';out=root/'evidence'
            payload={'v2game.exe':b'fake exe',
                     'V2DLL/v2dll_core.dll':b'old core',
                     'z_CreatineCore/creatinecore.dll':b'new core',
                     'z_CreatineCore/creatinecore_settings.ini':b'ENABLE_STATE_PROBE=0\n',
                     'z_CreatineCore/dll_Submods/ccore_performance_patch.dll':b'new performance',
                     'z_CreatineCore/dll_Submods/ccore_performance_patch.ini':b'FIX_ARMY_WINDOW_LAG=0\n',
                     'V2DLL/Logs/v2dll.log':b'old log',
                     'z_CreatineCore/Logs/v2dll.log':b'new log'}
            for name,data in payload.items():
                p=game/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
            collector.collect(game,out)
            evidence=json.loads((out/'evidence.json').read_text())
            self.assertIn('game/z_CreatineCore/creatinecore.dll',evidence['files'])
            self.assertIn('game/z_CreatineCore/dll_Submods/ccore_performance_patch.ini',evidence['files'])
            self.assertEqual((out/'creatinecore-logs/v2dll.log').read_bytes(),b'new log')
            self.assertEqual((out/'loader-logs/v2dll.log').read_bytes(),b'old log')
            self.assertFalse((out/'game/z_CreatineCore/creatinecore.dll').exists())
            for name,data in payload.items():self.assertEqual((game/name).read_bytes(),data)


if __name__=='__main__':unittest.main()
