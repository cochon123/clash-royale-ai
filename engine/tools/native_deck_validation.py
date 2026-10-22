#!/usr/bin/env python3
"""Record/replay exact original-native deck episodes, with optional source coverage.

Performance is measured on replay without coverage hooks. Each trace pins the
source binary and all requests, including repeated fresh-deck resets/seeds.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import subprocess
import time

ROOT=Path(__file__).resolve().parents[1]
CHECKPOINT=ROOT/'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state'


def sha256(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()


class Engine:
    def __init__(self,binary,profile=None):
        env=dict(os.environ)
        if profile:env['CLASH_DECK_PROFILE_DIR']=str(profile.resolve())
        self.p=subprocess.Popen([str(binary.resolve()),'--checkpoint',str(CHECKPOINT)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True,env=env,cwd=ROOT)
        self.closed=False
    def request(self,r):
        self.p.stdin.write(json.dumps(r)+'\n');self.p.stdin.flush()
        line=self.p.stdout.readline()
        if not line:raise RuntimeError(f'engine exited {self.p.poll()} on {r}')
        x=json.loads(line)
        if not x.get('ok'):raise RuntimeError(f'{r}: {x}')
        if r.get('op')=='close':self.closed=True
        return x['result']
    def close(self):
        if self.p.poll() is None:
            try:
                if not self.closed:self.request({'op':'close'})
            finally:self.p.stdin.close();self.p.wait(timeout=15)


def semantic(response):
    return {k:v for k,v in response.items() if k not in ('reset_seconds','step_seconds')}


def record(args):
    engine=Engine(args.binary,args.profile_dir); records=[]; games=[]
    args.output.parent.mkdir(parents=True,exist_ok=True)
    try:
        for deck in args.decks:
            for seed in args.seeds:
                rng=random.Random(seed+1_000_000)
                def capture(r):
                    response=engine.request(r)
                    records.append({'request':r,'expected':semantic(response)})
                    return response
                reset=capture({'op':'reset','deck_preset':deck,'seed':seed,'max_ticks':7300})
                result=capture({'op':'step','actions':[],'ticks':100});obs=result['observation']
                seen=set();t0=time.monotonic()
                for _ in range(args.steps):
                    actions=[]
                    for side in (0,1):
                        hand=[c for c in obs['hands'][side] if c['available']]
                        unseen_hand=[c for c in hand if (side,c['name']) not in seen]
                        target=max(unseen_hand,key=lambda c:c['cost']) if unseen_hand else None
                        affordable=([target] if target and target['cost']<=obs['elixir'][side] else [] if target else [c for c in hand if c['cost']<=obs['elixir'][side]])
                        if affordable and rng.random()>.15:
                            # Prefer each still-unplayed card to cover both deck families.
                            unseen=[c for c in affordable if (side,c['name']) not in seen]
                            c=rng.choice(unseen or affordable);seen.add((side,c['name']))
                            y=rng.randint(2000,14500) if side==0 else rng.randint(17500,30000)
                            if c['name'] in ('Rocket','Fireball','Arrows','Log','BarbLog'):
                                y=rng.randint(18000,29000) if side==0 else rng.randint(3000,14000)
                            actions.append({'side':side,'slot':c['slot'],'x':rng.choice([3500,9000,14500]),'y':y})
                    result=capture({'op':'step','actions':actions,'ticks':20});obs=result['observation']
                    if result['terminated'] or result['truncated']:break
                row={'deck':deck,'seed':seed,'tick':obs['tick'],'terminated':result['terminated'],'truncated':result['truncated'],'winner_side':obs['winner_side'],'attempted_card_names_by_side':[sorted(n for s,n in seen if s==side) for side in (0,1)],'wall_seconds_with_optional_profile':time.monotonic()-t0}
                if args.audit:
                    out=args.output.parent/f'{args.output.stem}-{deck}-{seed}-reference-audit'
                    engine.request({'op':'audit','out':str(out.resolve())});row['reference_audit']=str(out.resolve())
                if args.profile_dir:engine.request({'op':'profile'})
                games.append(row)
                trace={'format':'native-deck-trace-v1','checkpoint_sha256':sha256(CHECKPOINT/'checkpoint.json'),'reference_binary':str(args.binary.resolve()),'reference_binary_sha256':sha256(args.binary),'requests':records,'games':games}
                args.output.write_text(json.dumps(trace,indent=2)+'\n')
                print(json.dumps(row),flush=True)
    finally:engine.close()


def replay(args):
    trace=json.loads(args.trace.read_text());engine=Engine(args.binary)
    compared=0;difference=None;timings={'reset':[],'step':[]};started=time.monotonic();game=0
    try:
        for index,item in enumerate(trace['requests']):
            request=item['request'];t0=time.monotonic()
            try:actual=engine.request(request)
            except Exception as exc:
                difference={'index':index,'request':request,'error':str(exc)};break
            timings.setdefault(request['op'],[]).append(time.monotonic()-t0)
            if semantic(actual)!=item['expected']:
                difference={'index':index,'request':request,'expected':item['expected'],'actual':semantic(actual)};break
            compared+=1
            if request['op']=='step' and (actual.get('terminated') or actual.get('truncated')):
                if args.audit:
                    row=trace['games'][game]
                    out=args.output.parent/f'{args.output.stem}-{row["deck"]}-{row["seed"]}-candidate-audit'
                    engine.request({'op':'audit','out':str(out.resolve())})
                game+=1
        elapsed=time.monotonic()-started
    finally:engine.close()
    report={'matched':difference is None and compared==len(trace['requests']),'requests_compared':compared,'request_count':len(trace['requests']),'difference':difference,'wall_seconds':elapsed,'binary_sha256':sha256(args.binary),'trace_sha256':sha256(args.trace),'timings':{k:{'calls':len(v),'seconds':sum(v),'values':v} for k,v in timings.items()}}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k!='timings'},indent=2))
    return 0 if report['matched'] else 1


def main():
    p=argparse.ArgumentParser(description=__doc__);mode=p.add_mutually_exclusive_group(required=True)
    mode.add_argument('--record',action='store_true');mode.add_argument('--trace',type=Path)
    p.add_argument('--binary',type=Path,required=True);p.add_argument('--output',type=Path,required=True)
    p.add_argument('--decks',nargs='+',default=['hog26','pixel9_mortar']);p.add_argument('--seeds',type=int,nargs='+',default=[1,2]);p.add_argument('--steps',type=int,default=360)
    p.add_argument('--profile-dir',type=Path);p.add_argument('--audit',action='store_true');a=p.parse_args()
    return record(a) if a.record else replay(a)
if __name__=='__main__':raise SystemExit(main())
