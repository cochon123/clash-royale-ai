"""Frozen three-way Banana round robin through the recovered original engine.

Both policies play identical decks, both arena sides, and identical native seed
banks. Only native terminal outcomes count. No optimizer or real game is used.
"""
from __future__ import annotations

import argparse
import atexit
from collections import Counter
from concurrent.futures import ProcessPoolExecutor, wait, FIRST_COMPLETED
import fcntl
import gzip
import hashlib
from itertools import combinations
import json
import multiprocessing as mp
import os
from pathlib import Path
import random
import signal
import socket
import sys
import time

import torch

from .banana_policy import BananaEpisode, _native_card
from .banana_placement import resolved_play_target
from .banana_timed_policy import PhoneStyleScheduler
from .banana_train import ROOT, atomic_json, sha256, utc_now
from .banana_watch import load_watched_policy

sys.path.insert(0,str(ROOT.parent/'src'))
from cr_replay_pipeline.native_enrichment import Engine

CONFIG = POLICIES = ENGINE = None


def initialize(config):
    global CONFIG,POLICIES,ENGINE
    CONFIG=config
    torch.set_num_threads(1)
    torch.manual_seed(20261024)
    POLICIES={name:load_watched_policy(Path(path),'cpu',20261024,name)
              for name,path in config['checkpoints'].items()}
    ENGINE=None
    atexit.register(close_engine)


def close_engine():
    global ENGINE
    if ENGINE is not None: ENGINE.close();ENGINE=None


def engine():
    global ENGINE
    if ENGINE is None or ENGINE.proc.poll() is not None:
        close_engine()
        ENGINE=Engine(Path(CONFIG['binary']),Path(CONFIG['checkpoint']),
                      Path(CONFIG['run_dir'])/f'native-{os.getpid()}.log')
    return ENGINE


def make_jobs(models,decks,seeds):
    return [dict(id=f'{a}__{b}__{deck}__{seed}__{seat}',a=a,b=b,deck=deck,seed=seed,seat=seat)
            for seed in seeds for deck in decks for a,b in combinations(models,2) for seat in (0,1)]


def game(job):
    started=time.monotonic();row={**job,'at':utc_now(),'result':'VOID'}
    trace=[];plays=[Counter(),Counter()];rejected=Counter()
    def timeout(*_): raise TimeoutError('Game exceeded 240-second wall budget')
    signal.signal(signal.SIGALRM,timeout);signal.setitimer(signal.ITIMER_REAL,240)
    try:
        client=engine();deck=CONFIG['decks'][job['deck']]
        response=client.request(dict(op='reset',deck0=deck['native_cards'],deck1=deck['native_cards'],
                                     seed=job['seed'],max_ticks=7300,reward_state=True))
        for names in response['configured_decks']:
            names=names.get('cards') if isinstance(names,dict) else names
            normalized=[_native_card(n,deck['cards']) for n in names]
            if Counter(normalized)!=Counter(deck['cards']): raise ValueError('Wrong configured deck')
        response=client.request(dict(op='step',actions=[],ticks=100,resolve_actions=True))
        if response['ticks_advanced']!=100: raise RuntimeError('Native warmup failed')
        observation=response['observation'];base=observation['tick']
        observation['tower_state']=response['reward_state']
        names=[job['a'],job['b']] if job['seat']==0 else [job['b'],job['a']]
        policies=[POLICIES[n] for n in names]
        episode=BananaEpisode((tuple(deck['cards']),tuple(deck['cards'])),battle_id=job['id'])
        episode.policy_seed=job['seed']
        mask=lambda request,slot:client.request(dict(op='placement_mask',side=request[2],slot=slot))['mask']
        scheduler=PhoneStyleScheduler(policies,placement_mask_provider=mask)
        last_tick=observation['tick']
        while not (response.get('terminated') or response.get('truncated')):
            now=(observation['tick']-base)/20.
            actions=scheduler.choose(episode,observation,now)
            response=client.request(dict(op='step',actions=[a.native_request() for a in actions],
                                         ticks=1,resolve_actions=True))
            if len(response['actions'])!=len(actions): raise RuntimeError('Action result mismatch')
            for action,result in zip(actions,response['actions']):
                scheduler.observe_result(action,result)
                trace.append(dict(tick=observation['tick'],card=action.card,side=action.side,
                                  requested=action.native_request(),result=result))
                if result.get('accepted'):
                    x,y=resolved_play_target(result)
                    episode.record_action(action.side,action.card,x,y,now,True)
                    plays[action.side][action.card]+=1
                else: rejected[result.get('rejection','unknown')]+=1
            observation=response['observation'];observation['tower_state']=response['reward_state']
            if observation['tick']<=last_tick and not (response.get('terminated') or response.get('truncated')):
                raise RuntimeError('Native world did not advance')
            last_tick=observation['tick']
        winner=observation.get('winner_side');terminal=bool(response.get('terminated'))
        if terminal and winner not in (None,-1,0,1): raise ValueError('Unknown native winner')
        row.update(result='VOID' if not terminal else 'D' if winner in (None,-1) else 'W' if winner==job['seat'] else 'L',
                   winner_model=names[winner] if terminal and winner in (0,1) else None,
                   winner_side=winner,terminated=terminal,truncated=bool(response.get('truncated')),
                   native_seconds=(observation['tick']-base)/20.,plays=[dict(p) for p in plays],
                   rejected=dict(rejected),tower_state=observation['tower_state'])
    except Exception as error:
        row['error']=f'{type(error).__name__}: {error}'
        close_engine()
    finally: signal.setitimer(signal.ITIMER_REAL,0)
    row['wall_seconds']=time.monotonic()-started
    trace_path=Path(CONFIG['run_dir'])/'replays'/(job['id']+'.json.gz')
    with gzip.open(trace_path,'wt') as f:json.dump({'game':row,'actions':trace},f)
    row['trace_sha256']=sha256(trace_path)
    return row


def standings(rows,models):
    counts={name:Counter() for name in models}
    for r in rows:
        result=r['result']
        counts[r['a']][result]+=1
        counts[r['b']][{'W':'L','L':'W','D':'D','VOID':'VOID'}[result]]+=1
    table=[]
    for name,c in counts.items():
        n=c['W']+c['L']+c['D']
        table.append(dict(model=name,wins=c['W'],losses=c['L'],draws=c['D'],voids=c['VOID'],
                          games=n,points=c['W']+.5*c['D'],score=(c['W']+.5*c['D'])/n if n else None))
    return sorted(table,key=lambda r:(-(r['score'] if r['score'] is not None else -1),-r['wins'],r['model']))


def summarize(rows,models):
    return dict(standings=standings(rows,models),
                pairings={a+' vs '+b:standings([r for r in rows if {r['a'],r['b']}=={a,b}],(a,b))
                          for a,b in combinations(models,2)},
                completed=len(rows),valid_games=sum(r['result']!='VOID' for r in rows),
                voids=sum(r['result']=='VOID' for r in rows),
                errors=dict(Counter(r.get('error') for r in rows if r.get('error'))))


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir',type=Path,required=True)
    p.add_argument('--binary',type=Path,default=ROOT/'rust-engine/target-banana-eval/release/rust_league_rl')
    p.add_argument('--workers',type=int,default=4)
    p.add_argument('--seeds',type=int,default=4)
    p.add_argument('--seed-start',type=int,default=3_824_100_000)
    p.add_argument('--decks',nargs='+')
    p.add_argument('--pair',nargs=2)
    p.add_argument('--checkpoint',action='append',default=[],metavar='NAME=PATH',
                   help='Replace the default trio. Repeat for each frozen actor. Relative paths are under engine/.')
    a=p.parse_args()
    if a.workers<1 or a.seeds<1 or a.seed_start<0 or a.seed_start+a.seeds>2**32:p.error('Invalid seed or worker count')
    a.run_dir=a.run_dir.resolve();a.binary=a.binary.resolve();a.run_dir.mkdir(parents=True,exist_ok=True)
    (a.run_dir/'replays').mkdir(exist_ok=True)
    lock=(a.run_dir/'.eval.lock').open('w');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    atomic_json(a.run_dir/'execution-owner.json',{'host':socket.gethostname(),'since':utc_now()})
    checkpoints={'banana-1':ROOT/'runs/banana-1/base.pt','banana-2':ROOT/'runs/banana-2/base.pt',
                 'bc-control':ROOT/'runs/banana-2/bc-control/best_model.pt'}
    if a.checkpoint:
        checkpoints={}
        for item in a.checkpoint:
            if item.count('=')!=1: p.error('Checkpoint must be NAME=PATH')
            name,raw=item.split('=')
            path=Path(raw)
            if not path.is_absolute(): path=ROOT/path
            if not name or not path.is_file(): p.error(f'Unknown checkpoint {item}')
            checkpoints[name]=path.resolve()
        if len(checkpoints)<2: p.error('At least two distinct checkpoints are required')
    if a.pair:
        if not set(a.pair)<=checkpoints.keys() or len(set(a.pair))<2:p.error('Unknown/distinct pair required')
        checkpoints={k:checkpoints[k] for k in a.pair}
    pool=json.loads((ROOT/'data/fast_league_decks.json').read_text())['decks']
    decks={d['id']:d for d in pool if a.decks is None or d['id'] in a.decks}
    if a.decks and set(a.decks)!=decks.keys():p.error('Unknown deck')
    config=dict(checkpoints={k:str(v) for k,v in checkpoints.items()},binary=str(a.binary),
                checkpoint=str(ROOT/'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state'),
                decks=decks,run_dir=str(a.run_dir))
    manifest={**config,'format':'banana-round-robin-v1','learning':False,
              'checkpoint_sha256':{k:sha256(v) for k,v in checkpoints.items()},
              'engine_sha256':sha256(a.binary),'script_sha256':sha256(Path(__file__)),
              'controller_sha256':{name:sha256(ROOT/'rl'/name) for name in
                                  ['banana_timed_policy.py','banana_fast_policy.py','banana_hog_adaptive.py','banana_policy.py','banana_placement.py']},
              'seeds':list(range(a.seed_start,a.seed_start+a.seeds)),'both_arena_sides':True,
              'deck_rule':'Identical deck on both sides; all six supported archetypes unless a subset is specified.',
              'controller':'sampled phone-style learned delay, poll1tick, native masks and resolved targets',
              'score':'Native terminal win=1 draw=0.5 loss=0; errors and truncations are VOID, never assigned a winner.'}
    path=a.run_dir/'manifest.json'
    if path.exists() and json.loads(path.read_text())!=manifest:raise RuntimeError('Tournament provenance changed; use a new directory')
    atomic_json(path,manifest)
    jobs=make_jobs(checkpoints,decks,manifest['seeds']);total=len(jobs)
    log=a.run_dir/'games.jsonl'
    rows=[json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []
    seen={r['id'] for r in rows};remaining=[j for j in jobs if j['id'] not in seen]
    start=time.monotonic();initial=len(rows);started=utc_now();stop=False
    def stopped(*_):
        nonlocal stop
        stop=True
    signal.signal(signal.SIGTERM,stopped);signal.signal(signal.SIGINT,stopped)
    def publish(status='running'):
        elapsed=time.monotonic()-start;done=len(rows);speed=(done-initial)/elapsed if elapsed else 0
        eta=(total-done)/speed if speed else None
        summary=summarize(rows,list(checkpoints))
        state=dict(kind='eval',method='banana_round_robin',host=socket.gethostname(),status=status,
                   started_at=started,updated_at=utc_now(),elapsed_seconds=elapsed,
                   phase='Frozen Banana round robin' if status=='running' else 'Banana round robin '+status,
                   progress=dict(done=done,total=total,eta_seconds=eta if status=='running' else 0,workers=a.workers),
                   completed_games=summary['valid_games'],tournament=summary,
                   scope='Frozen round robin among '+', '.join(checkpoints)+'; identical decks, seeds and both seats. No learning.')
        atomic_json(a.run_dir/'state.json',state)
        atomic_json(a.run_dir/'summary.json',summary|{'status':status})
        print(f"{done:>3}/{total:<3} DONE | ETA {int(eta//3600):02d}:{int(eta%3600//60):02d}:{int(eta%60):02d}" if eta is not None else f'{done:>3}/{total:<3} DONE | ETA estimating',flush=True)
        print(' | '.join(f"{r['model']}: {r['wins']}W {r['losses']}L {r['draws']}D ({r['voids']} void)" for r in summary['standings']),flush=True)
        return state
    publish()
    with ProcessPoolExecutor(max_workers=a.workers,mp_context=mp.get_context('spawn'),initializer=initialize,initargs=(config,)) as executor:
        jobs=iter(remaining);active={}
        def fill():
            while not stop and len(active)<a.workers:
                job=next(jobs,None)
                if job is None:break
                active[executor.submit(game,job)]=job
        fill();last=time.monotonic()
        with log.open('a',buffering=1) as out:
            while active:
                finished,_=wait(active,timeout=5,return_when=FIRST_COMPLETED)
                for task in finished:
                    active.pop(task);row=task.result();rows.append(row)
                    out.write(json.dumps(row)+'\n')
                    print(row['id'],row['result'],f"{row['wall_seconds']:.1f}s",row.get('error',''),flush=True)
                if (a.run_dir/'STOP').exists():stop=True
                if finished or time.monotonic()-last>=15:publish();last=time.monotonic()
                fill()
    publish('stopped' if stop else 'completed')
    if {k:sha256(v) for k,v in checkpoints.items()}!=manifest['checkpoint_sha256']:
        raise RuntimeError('Checkpoint changed during tournament')


if __name__=='__main__':main()
