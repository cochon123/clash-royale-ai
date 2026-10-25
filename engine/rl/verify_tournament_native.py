"""Compare opt-in compiled placement resolution with the original emulator."""
import argparse
import json
from pathlib import Path
import sys

from .banana_train import ROOT, atomic_json, utc_now
sys.path.insert(0,str(ROOT.parent/'src'))
from cr_replay_pipeline.native_enrichment import Engine


def main():
    p=argparse.ArgumentParser()
    p.add_argument('--binary',type=Path,required=True)
    p.add_argument('--out',type=Path,required=True)
    a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
    checkpoint=ROOT/'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state'
    fast=Engine(a.binary.resolve(),checkpoint,a.out/'compiled.log')
    reference=Engine(ROOT/'native-engine/target/release/native-watch',checkpoint,a.out/'reference.log')
    checks=0;changed=0
    try:
        decks=json.loads((ROOT/'data/fast_league_decks.json').read_text())['decks']
        for deck in decks:
            req=dict(op='reset',deck0=deck['native_cards'],deck1=deck['native_cards'],seed=3824099911,max_ticks=7300,reward_state=True)
            f,r=fast.request(req),reference.request(req)
            assert f['observation']==r['observation'],deck['id']+' reset'
            step=dict(op='step',actions=[],ticks=100,resolve_actions=True)
            f,r=fast.request(step),reference.request(step)
            assert f['observation']==r['observation'],deck['id']+' warmup'
            for side in (0,1):
                for slot in range(4):
                    for x,y in [(3500,6500),(9000,16000),(14500,25500)]:
                        request=dict(op='resolve_target',side=side,slot=slot,x=x,y=y)
                        left,right=fast.request(request),reference.request(request)
                        assert left==right,(deck['id'],request,left,right)
                        checks+=1;changed+=left.get('valid') and (left['x'],left['y'])!=(x,y)
            for side in (0,1):
                obs=f['observation'];hand=obs['hands'][side]
                card=min(hand,key=lambda c:c['cost'])
                action=dict(side=side,slot=card['slot'],x=9000,y=25000 if side==0 else 7000)
                step=dict(op='step',actions=[action],ticks=20,resolve_actions=True)
                f,r=fast.request(step),reference.request(step)
                assert f['actions']==r['actions'],(deck['id'],'actions',f['actions'],r['actions'])
                assert f['observation']==r['observation'],(deck['id'],'after action')
                checks+=1
            print(deck['id'],'passed',checks,'checks',flush=True)
        atomic_json(a.out/'verification.json',dict(at=utc_now(),passed=True,checks=checks,targets_moved=changed,
                    source='Original native-watch emulator vs compiled native placement resolver and action replies.'))
    finally:fast.close();reference.close()


if __name__=='__main__':main()
