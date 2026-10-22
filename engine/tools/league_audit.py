#!/usr/bin/env python3
"""Replay six development matchups and compare full original guest state."""
import argparse
import gzip
import json
from pathlib import Path

from league_validation import Engine, semantic, sha
from native_rust_compare import compare


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--reference',type=Path,required=True);p.add_argument('--candidate',type=Path,required=True)
    p.add_argument('--traces',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    p.add_argument('--ids',type=int,nargs='+',default=[5,11,17,23,29,30])
    a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True);rows=[]
    hashes={'reference':sha(a.reference),'candidate':sha(a.candidate)}
    for game in a.ids:
        trace=json.load(gzip.open(a.traces/f'{game:04d}.json.gz','rt'))
        paths={}
        for name in ('reference','candidate'):
            engine=Engine(getattr(a,name),a.out/f'{name}.log')
            try:
                for record in trace['records']:
                    assert semantic(engine.request(record['request']))==record['expected'],(name,game)
                path=(a.out/f'{game:04d}-{name}').resolve()
                engine.request({'op':'audit','out':str(path)});paths[name]=path
            finally:engine.close()
        report=compare(paths['reference'],paths['candidate'])
        report.update(game=game,deck0=trace['job']['deck0']['id'],deck1=trace['job']['deck1']['id'])
        (a.out/f'{game:04d}-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
        rows.append(report);print(json.dumps({k:report[k] for k in ('game','matched','region_count','register_count','differences')}),flush=True)
    assert hashes=={'reference':sha(a.reference),'candidate':sha(a.candidate)}
    report={'matched':all(r['matched'] for r in rows),'games':len(rows),'binary_sha256':hashes,'comparisons':rows}
    (a.out/'summary.json').write_text(json.dumps(report,indent=2)+'\n')
    return 0 if report['matched'] else 1

if __name__=='__main__':raise SystemExit(main())
