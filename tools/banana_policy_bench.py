#!/usr/bin/env python3
"""Measure original and batched policy work on the real 4.4.2 model, without an engine."""
import json
import sys
import time
from pathlib import Path
import torch
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from rl.banana_fast_policy import FastBananaPolicy
from rl.banana_policy import BananaPolicy,BananaEpisode
DECK=('hog-rider','musketeer','ice-golem','skeletons','ice-spirit','cannon','fireball','the-log')
torch.set_num_threads(2)
results=[]
for device in ['cpu','cuda'] if torch.cuda.is_available() else ['cpu']:
 p=FastBananaPolicy(device=device,seed=431);p.eval()
 ep=BananaEpisode((DECK,DECK))
 for i in range(60):ep.record_action(i%2,DECK[i%8],4500,4500 if i%2==0 else 28000,i*1.3)
 obs={'hands':[[{'slot':i,'name':DECK[(i+s*2)%8],'cost':p.costs.get(DECK[(i+s*2)%8],4),'available':True} for i in range(4)] for s in (0,1)],'elixir':[10.,10.]}
 requests=[(ep,obs,i%2,80.) for i in range(32)]
 def sync():
  if device=='cuda':torch.cuda.synchronize()
 def timeit(fn,n):
  for _ in range(2):fn()
  sync();t=time.perf_counter()
  for _ in range(n):fn()
  sync();return (time.perf_counter()-t)*1000/n
 for batch in [1,3,32]:
  rows=requests[:batch]
  results.append({'device':device,'kind':'act_batch','batch':batch,'ms':timeit(lambda:p.act_batch(rows,deterministic=True),10)})
 actions=p.act_batch(requests,deterministic=True)
 def backward():
  p.zero_grad(set_to_none=True)
  logp,ent,val=p.evaluate_actions(actions)
  (logp.mean()+.1*ent.mean()+.5*val.square().mean()).backward()
 results.append({'device':device,'kind':'batched_ppo_forward_backward','batch':32,'ms':timeit(backward,5)})
 print(json.dumps(results[-4:],indent=2),flush=True)
 del p
 if device=='cuda':torch.cuda.empty_cache()
path=ROOT/'analysis/fast-decks/policy-benchmark.json';path.write_text(json.dumps({'threads':2,'events':60,'results':results,'scope':'real 4.4.2 model microbenchmark, including legal-mask/action decoding; excludes engine and optimizer/checkpoint writes'},indent=2)+'\n')
