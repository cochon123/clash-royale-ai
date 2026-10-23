import json,csv,statistics,datetime
from pathlib import Path
from collections import Counter
R=Path('/tmp/cr-prof/bench-safe');med=statistics.median
S=json.loads((R/'summary.json').read_text());runs=[json.loads(Path(s['result']).read_text()) for s in S]
names=['BASELINE8','NEW8g1','NEW8g2'];fields=['deck_id','controller','seat','result','seconds'];jobs=fields[:3]
rows=[];waves=[]
for r in runs:
 name=r['configuration'];seed=r['seed'];tail=r['wave_records'][1:]
 row={k:r[k] for k in ['configuration','seed','total_s','games','decisions','games_per_min','decisions_per_s','constructor_s','cleanup_s','cache_request_s','act_batch_s','act_batch_calls','launch_s','wait_steps_s','consume_s','publish_s','update_s','memory_peak_bytes']}
 row.update(first_wave_s=r['wave_records'][0]['wave_s'],first_wave_games_per_min=60*r['wave_records'][0]['games']/r['wave_records'][0]['wave_s'],first_wave_decisions_per_s=r['wave_records'][0]['decisions']/r['wave_records'][0]['wave_s'],later_games_per_min=60*sum(w['games'] for w in tail)/sum(w['wave_s'] for w in tail),later_decisions_per_s=sum(w['decisions'] for w in tail)/sum(w['wave_s'] for w in tail),act_minus_encoding_s=r['act_batch_s']-r['cache_request_s'],residual_reset_orchestration_s=r['total_s']-r['launch_s']-r['wait_steps_s']-r['consume_s']-r['publish_s']-r['update_s'],engine_reset_sum_s=r['engine']['reset_s'],engine_step_sum_s=r['engine']['step_s'],engine_mask_sum_s=r['engine']['mask_s'])
 rows.append(row)
 for w in r['wave_records']:
  waves.append(dict(configuration=name,seed=seed,wave=w['wave'],wave_s=w['wave_s'],games=w['games'],decisions=w['decisions'],games_per_min=60*w['games']/w['wave_s'],decisions_per_s=w['decisions']/w['wave_s'],**w['phases']))
for fn,data in [('results.csv',rows),('waves.csv',waves)]:
 with (R/fn).open('w') as f:
  wr=csv.DictWriter(f,fieldnames=list(data[0]));wr.writeheader();wr.writerows(data)
aggregates={n:{k:med([r[k] for r in rows if r['configuration']==n]) for k in rows[0] if isinstance(rows[0][k],(int,float)) and k!='seed'} for n in names if any(r['configuration']==n for r in rows)}
paired=[]
for a,b in [('BASELINE8','NEW8g1'),('BASELINE8','NEW8g2'),('NEW8g1','NEW8g2')]:
 pairs=[]
 for seed in [777,2026]:
  ra=next((r for r in rows if r['configuration']==a and r['seed']==seed),None);rb=next((r for r in rows if r['configuration']==b and r['seed']==seed),None)
  if ra and rb:pairs.append({'seed':seed,**{metric:rb[metric]/ra[metric] for metric in ['games_per_min','decisions_per_s','first_wave_games_per_min','first_wave_decisions_per_s']}})
 paired.append({'a':a,'b':b,'seeds':pairs,'range':{m:{'min':min(p[m] for p in pairs),'max':max(p[m] for p in pairs),'median':med(p[m] for p in pairs)} for m in ['games_per_min','decisions_per_s','first_wave_games_per_min','first_wave_decisions_per_s']} if pairs else {}})
def outcomes(r):
 data=[json.loads(l) for l in (R/(r['label']+'-matches.jsonl')).read_text().splitlines() if l]
 d={(x['policy_version'],x['seed']):x for x in data};assert len(d)==len(data)
 return d
agreement=[]
for seed in [777,2026]:
 for a,b in [('BASELINE8','NEW8g1'),('BASELINE8','NEW8g2'),('NEW8g1','NEW8g2')]:
  ra=next((r for r in runs if r['configuration']==a and r['seed']==seed),None);rb=next((r for r in runs if r['configuration']==b and r['seed']==seed),None)
  if not(ra and rb):continue
  aa,bb=outcomes(ra),outcomes(rb);common=aa.keys()&bb.keys();union=aa.keys()|bb.keys()
  per=[]
  for v in range(3):
   ks=[k for k in common if k[0]==v]
   per.append({'wave':v+1,'count_a':sum(k[0]==v for k in aa),'count_b':sum(k[0]==v for k in bb),'paired':len(ks),'jobs_equal':sum(all(aa[k][f]==bb[k][f] for f in jobs) for k in ks),'exact':sum(all(aa[k][f]==bb[k][f] for f in fields) for k in ks),'fields_equal':{f:sum(aa[k][f]==bb[k][f] for k in ks) for f in fields}})
  agreement.append({'a':a,'b':b,'seed':seed,'paired':len(common),'union':len(union),'exact':sum(all(aa[k][f]==bb[k][f] for f in fields) for k in common),'per_wave':per,'mismatches':[{'policy_version':k[0],'seed':k[1],'a':{f:aa[k][f] for f in fields},'b':{f:bb[k][f] for f in fields}} for k in sorted(common) if any(aa[k][f]!=bb[k][f] for f in fields)]})
training={r['label']:[json.loads(l) for l in (R/(r['label']+'-updates.jsonl')).read_text().splitlines() if l] for r in runs if (R/(r['label']+'-updates.jsonl')).exists()}
for r in runs:
 assert r['workers']==8 and r['waves']==r['updates']==len(r['wave_records'])==3
 assert r['games']==24 and r['counters']['training_games']==24 and all(w['games']==8 for w in r['wave_records'])
 assert not r['errors']
 assert len(outcomes(r))==24
 if r['label'] in training:assert len(training[r['label']])==3
old=json.loads(Path('/tmp/cr-prof/bench/summary.json').read_text())
old={'status':'earlier incomplete matrix, before final correctness fix; not final-code measurements; unsafe worker counts are not recommendations','completed_runs':len(old),'observations':[{k:s[k] for k in ['configuration','seed','total_s','games_per_min','decisions_per_s']} for s in old]}
result={'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'model':'banana-hog-2 configured learner, 3 PPO waves per scratch run','baseline_head':'8b623650017a22b546d69c8f1a6eed21eddd462a','final_head':'fc4d9ebe6d7330aa3d4e261a5cb848890765a623','preflight':json.loads((R/'preflight.json').read_text()),'method':{'workers':8,'waves_per_run':3,'seeds':[777,2026],'order':['BASELINE8','NEW8g1','NEW8g2','NEW8g2','NEW8g1','BASELINE8'],'memory_cap_bytes':8589934592,'swap_cap_bytes':0,'watchdog_available_bytes':6442450944,'prelaunch_available_bytes':12884901888,'timing':'total_s includes 3 collect+PPO waves and cleanup; constructor_s excluded; per-wave includes first native startup/reset costs','caveat':'Two seeds, one machine, no confidence intervals. First wave is isolated; later policy trajectories can differ. Outcome equality does not establish action equality. Native request durations and pace engine_s overlap; never add these to wall time.'},'runs':rows,'wave_records':waves,'raw_runs':runs,'medians':aggregates,'paired_speedups':paired,'outcome_agreement':agreement,'training_curves':training,'earlier_incomplete_matrix':old,'validation':{'same_updates':True,'same_sampler_games_per_wave':True,'games_per_run':24,'decisions_per_game_variable':True,'all_runs_errors_empty':True,'all_memory_events_cap_hits_zero':all(not any(x.get('memory_events',{}).get(k,0) for k in ['max','oom','oom_kill']) for r in runs for x in json.loads((R/(r['label']+'-telemetry.json')).read_text())['samples'])}}
(R/'analysis.json').write_text(json.dumps(result,indent=2));(R/'earlier_incomplete_matrix_summary.json').write_text(json.dumps(old,indent=2))
print(json.dumps({'medians':aggregates,'paired_speedups':paired,'outcome_agreement':[{k:v for k,v in a.items() if k!='mismatches'} for a in agreement],'validation':result['validation']},indent=2))
