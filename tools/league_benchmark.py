#!/usr/bin/env python3
"""Benchmark identical complete league replays in isolated, persistent engines."""
import argparse
import collections
import gzip
import json
import os
from pathlib import Path
import platform
import statistics
import time

from league_validation import Engine, semantic, sha


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--reference',type=Path,required=True);p.add_argument('--candidate',type=Path,required=True)
    p.add_argument('--traces',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    p.add_argument('--rounds',type=int,default=3)
    p.add_argument('--ids',type=int,nargs='+',default=[5,11,17,23,29,30])
    p.add_argument('--cpu',type=int,help='pin both engine variants to the same logical CPU')
    a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
    # Both sides and all six decks: five Mortar-opponent games plus its reverse.
    ids=a.ids
    traces=[json.load(gzip.open(a.traces/f'{i:04d}.json.gz','rt')) for i in ids]
    hashes={'reference':sha(a.reference),'candidate':sha(a.candidate)};rows=[]
    for repeat in range(a.rounds):
        for name in (('reference','candidate') if repeat%2==0 else ('candidate','reference')):
            engine=Engine(getattr(a,name),a.out/f'{name}.log')
            try:
                if a.cpu is not None:os.sched_setaffinity(engine.proc.pid,{a.cpu})
                # Warm reset/page reads; process startup and this reset excluded.
                engine.request(traces[0]['records'][0]['request'])
                for trace in traces:
                    timings=collections.defaultdict(float);engine_times=collections.defaultdict(float)
                    ticks=0;start=time.perf_counter()
                    for record in trace['records']:
                        req=record['request'];before=time.perf_counter();reply=engine.request(req)
                        timings[req['op']]+=time.perf_counter()-before
                        if semantic(reply)!=record['expected']:
                            raise RuntimeError(f"{name} benchmark replay differs: game {trace['job']['id']} {req['op']}")
                        for field in ('reset_seconds','step_seconds','query_seconds'):
                            engine_times[field]+=reply.get(field,0)
                        ticks+=reply.get('ticks_advanced',0)
                    rows.append({'engine':name,'round':repeat,'game':trace['job']['id'],'ticks':ticks,
                                 'seconds':time.perf_counter()-start,'request_wall_seconds':dict(timings),'reported_seconds':dict(engine_times)})
            finally:engine.close()
    summaries={}
    for name in ('reference','candidate'):
        subset=[r for r in rows if r['engine']==name];seconds=sum(r['seconds'] for r in subset)
        summaries[name]={'games':len(subset),'wall_seconds':seconds,'games_per_minute':len(subset)*60/seconds,
                         'ticks_per_second':sum(r['ticks'] for r in subset)/seconds,
                         'median_game_seconds':statistics.median(r['seconds'] for r in subset),
                         'engine_step_seconds':sum(r['reported_seconds']['step_seconds'] for r in subset)}
    assert hashes=={'reference':sha(a.reference),'candidate':sha(a.candidate)},'binary changed during benchmark'
    report={'binary_sha256':hashes,'hardware':platform.uname()._asdict(),'workers':1,'rounds':a.rounds,'engine_cpu':a.cpu,'trace_ids':ids,
            'summaries':summaries,'speedup':summaries['candidate']['games_per_minute']/summaries['reference']['games_per_minute'],
            'scope':'Complete games including reset, legal-mask query, action submission, observations, tower HP and JSON IPC. Excludes process startup and neural-policy inference. Engines run sequentially, alternating order by round. Host load may affect results.',
            'rows':rows}
    (a.out/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k!='rows'},indent=2))

if __name__=='__main__':main()
