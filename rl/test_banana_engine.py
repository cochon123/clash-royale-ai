"""Recovery must reproduce the same request prefix or fail closed."""
from pathlib import Path
import unittest
from unittest.mock import patch
from rl.banana_engine import ReplayFallbackEngine


class EngineRecoveryTests(unittest.TestCase):
    def factory(self, mismatch=False):
        created=[]
        class Engine:
            def __init__(self, _checkpoint, binary):
                self.reference=binary.name=='reference'
                self.tick=0; self.requests=[];self.closed=False
                created.append(self)
            def request(self, request):
                self.requests.append(request)
                if request['op']=='reset':
                    self.tick=0
                    return {'observation':{'tick':0},'reset_seconds':4 if self.reference else 1}
                if request['op']=='invalid':raise RuntimeError('invalid side')
                if not self.reference and self.tick==20:raise RuntimeError('untranslated function 0x1083476c')
                self.tick+=request['ticks']
                return {'observation':{'tick':self.tick+(1 if self.reference and mismatch else 0)},'step_seconds':2 if self.reference else 1}
            def close(self):self.closed=True
        return Engine,created

    def test_recovers_exact_seed_action_prefix_and_retries_compiled_after_reset(self):
        factory,created=self.factory()
        with patch('rl.banana_engine.JsonlEngine',factory):
            engine=ReplayFallbackEngine(Path('checkpoint'),Path('compiled'),Path('reference'))
            reset={'op':'reset','seed':47,'deck_preset':'hog26'}
            step={'op':'step','ticks':20,'actions':[{'side':0,'slot':2,'x':3500,'y':9000}]}
            engine.request(reset);engine.request(step)
            result=engine.request(step)
            self.assertEqual(result['observation']['tick'],40)
            self.assertEqual(created[1].requests,[reset,step,step])
            self.assertTrue(created[0].closed)
            self.assertEqual(engine.fallback_count,1)
            engine.request(reset)
            self.assertFalse(engine.in_reference)
            self.assertEqual(len(created),3)
            engine.close()

    def test_disagreement_poisoned_until_a_fresh_reset(self):
        factory,_=self.factory(mismatch=True)
        with patch('rl.banana_engine.JsonlEngine',factory):
            engine=ReplayFallbackEngine(Path('checkpoint'),Path('compiled'),Path('reference'))
            engine.request({'op':'reset'});engine.request({'op':'step','ticks':20})
            with self.assertRaisesRegex(RuntimeError,'disagrees'):
                engine.request({'op':'step','ticks':20})
            self.assertEqual(engine.fallback_count,0)
            with self.assertRaisesRegex(RuntimeError,'reset required'):
                engine.request({'op':'step','ticks':20})
            engine.request({'op':'reset'})
            self.assertFalse(engine.poisoned)
            engine.close()

    def test_request_errors_do_not_enable_reference_recovery(self):
        factory,created=self.factory()
        with patch('rl.banana_engine.JsonlEngine',factory):
            engine=ReplayFallbackEngine(Path('checkpoint'),Path('compiled'),Path('reference'))
            engine.request({'op':'reset'})
            with self.assertRaisesRegex(RuntimeError,'invalid side'):
                engine.request({'op':'invalid'})
            self.assertEqual(len(created),1)
            self.assertEqual(engine.fallback_count,0)
            engine.close()

if __name__=='__main__':unittest.main()
