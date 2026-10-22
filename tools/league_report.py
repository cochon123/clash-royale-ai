#!/usr/bin/env python3
"""Report winner and tower-HP agreement without dropping failed games."""
import argparse
import collections
import gzip
import json
import math
from pathlib import Path


def lower_wilson(successes, total):
    z=1.959963984540054; p=successes/total
    return (p+z*z/(2*total)-z*math.sqrt(p*(1-p)/total+z*z/(4*total*total)))/(1+z*z/total)


def summarize(rows):
    completed=[r for r in rows if r['candidate_completed']]
    agreements=sum(r['winner_matches'] for r in rows)
    hp={}
    for kind,count in [('PrincessTower',4),('KingTower',2)]:
        errors=[error for r in completed for error in r['hp_errors'][kind]]
        # A failed/unknown game receives the maximum possible error. This
        # bounds overall MAE instead of quietly excluding unavailable outcomes.
        penalty=(len(rows)-len(completed))*count*100
        hp[kind]={'measured_towers':len(errors),'mae_pp':sum(errors)/len(errors) if errors else None,
                  'maximum_error_pp':max(errors) if errors else None,
                  'mae_pp_with_failures_penalized':(sum(errors)+penalty)/(len(rows)*count)}
    return {'games':len(rows),'candidate_completed':len(completed),'winner_agreements':agreements,
            'winner_agreement':agreements/len(rows),'winner_agreement_wilson_95_lower':lower_wilson(agreements,len(rows)),
            'every_reply_exact':sum(r['exact'] for r in rows),'hp':hp,
            'reference_outcomes':dict(collections.Counter(str(r['reference_winner']) for r in rows)),
            'candidate_outcomes':dict(collections.Counter(str(r['candidate_winner']) for r in completed))}


def load_rows(directory,regression=False):
    manifest=json.loads((directory/'manifest.json').read_text());summary=json.loads((directory/'summary.json').read_text())
    assert regression or manifest.get('candidate_sha256_before_recording') in (None,summary['candidate_sha256']), 'changed candidate: use --regression for replaying an existing test set'
    rows=[]
    for job in manifest['jobs']:
        trace=json.load(gzip.open(directory/f"{job['id']:04d}.json.gz",'rt'))
        path=directory/f"{job['id']:04d}-result.json";result=json.loads(path.read_text()) if path.exists() else {}
        complete='winner' in result and 'tower_hp' in result and result.get('matched') is True
        errors=collections.defaultdict(list)
        if complete:
            actual={t['id']:t for t in result['tower_hp']['towers']}
            for t in trace['tower_hp']['towers']:
                got=actual[t['id']];assert t['max_hp']==got['max_hp'] and t['row']==got['row']
                errors[t['row']].append(abs(t['hp']-got['hp'])/t['max_hp']*100)
        rows.append({'id':job['id'],'seed':job['seed'],'deck0':job['deck0']['id'],'deck1':job['deck1']['id'],
                     'exact':bool(result.get('matched')),'candidate_completed':complete,'reference_winner':trace['winner'],
                     'candidate_winner':result.get('winner'),'winner_matches':complete and result['winner']==trace['winner'],
                     'hp_errors':dict(errors)})
    assert len(rows)==summary['total_games']
    assert sum(r['exact'] for r in rows)==summary['fully_matched']
    return manifest,summary,rows


def metrics(rows):
    return {'overall':summarize(rows),'per_deck':{d:summarize([r for r in rows if d in (r['deck0'],r['deck1'])]) for d in sorted({r['deck0'] for r in rows}|{r['deck1'] for r in rows})}}


def target_passes(values):
    return all(r['winner_agreement']>=.95 and all(h['mae_pp_with_failures_penalized']<5 for h in r['hp'].values())
               for r in [values['overall'],*values['per_deck'].values()])


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('directories',type=Path,nargs='+');p.add_argument('--out',type=Path)
    p.add_argument('--regression',action='store_true',help='explicitly label reuse of existing reference traces after a candidate change')
    a=p.parse_args()
    parts=[load_rows(directory,a.regression) for directory in a.directories]
    assert len({s['candidate_sha256'] for m,s,r in parts})==1,'cohorts used different candidates'
    assert len({m['reference_sha256'] for m,s,r in parts})==1,'cohorts used different references'
    rows=[r for m,s,rs in parts for r in rs]
    assert len({(r['deck0'],r['deck1'],r['seed']) for r in rows})==len(rows),'duplicate games across cohorts'
    overall=summarize(rows)
    report={'split':'regression-replay' if a.regression else parts[0][0]['split'] if len(parts)==1 else 'combined-holdout','candidate_sha256':parts[0][1]['candidate_sha256'],
            'trace_generation_candidates':[m.get('candidate_sha256_before_recording') for m,s,r in parts],
            'candidate_unchanged':all(s['candidate_unchanged'] for m,s,r in parts),
            'requests_compared':sum(s['requests_compared'] for m,s,r in parts),**metrics(rows),
            'cohorts':[{'directory':str(directory),**metrics(rs)} for directory,(m,s,rs) in zip(a.directories,parts)],
            'per_deck_denominator':'Each distinct game involving the deck, including mirrors once.',
            'scope':'Agreement with the pinned recovered build on the recorded scripted action distribution. Not accuracy against the current live game or every possible policy.',
            'confidence_note':'Wilson interval describes the tested distribution under a binomial model; it is not a guarantee for unseen strategies.'}
    report['target_passed']=bool(report['candidate_unchanged'] and target_passes(report) and all(target_passes(c) for c in report['cohorts']))
    output=a.out or a.directories[0]/'accuracy.json';output.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
    return 0 if report['target_passed'] else 1

if __name__=='__main__':raise SystemExit(main())
