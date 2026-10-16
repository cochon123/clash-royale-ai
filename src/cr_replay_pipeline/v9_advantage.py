"""Train-only cross-fitted prefix values and clipped value-change action weights.

This is an offline supervised heuristic. A change in predicted win probability
is not a causal advantage and does not establish that an action improves wins.
The value predictor never sees future events, hidden enemy deck cards or the
target result as an input. Each action is weighted by a value model trained on
different battles. Validation values are measured, not used to fit weights.
"""
from __future__ import annotations
import argparse
from concurrent.futures import ProcessPoolExecutor
import json
import multiprocessing as mp
from pathlib import Path
import pickle
import time
from dataclasses import replace

import numpy as np
import torch
from sklearn.ensemble import HistGradientBoostingClassifier
from sklearn.metrics import log_loss, roc_auc_score
from threadpoolctl import threadpool_limits
from .controlled_experiment import atomic_json
from .v9_cache import ShardDataset, PrefixBank
from .winner_dataset import CardVocab, load_card_costs


def _weight_shard(task):
    cache,shard,first_group,battles,model_path,beta=task
    torch.set_num_threads(1)
    cache=Path(cache)
    vocab=CardVocab.from_dict(json.loads((cache/"vocab.json").read_text()))
    costs=load_card_costs("data/card_costs.json")
    with open(model_path,"rb") as f:models=pickle.load(f)
    folder=cache/"train"/shard["path"]/"actions"
    groups=np.load(folder/"group.npy",mmap_mode="r")
    events=np.load(folder/"event.npy",mmap_mode="r")
    before=np.load(folder/"g.npy",mmap_mode="r")
    after=np.empty_like(before)
    banks={}
    for j,(group,index) in enumerate(zip(groups,events)):
        bi=int(group)-first_group
        if bi not in banks:banks[bi]=PrefixBank(battles[bi],vocab,costs)
        # All same-timestamp events become known together; no ordered-tie fiction.
        end=int(index)+1;b=battles[bi];side=b.events[index]["side"]
        while end<len(b.events) and b.events[end]["seconds"]==b.events[index]["seconds"]:end+=1
        after[j]=banks[bi].encode(end,side)[4].numpy()
    delta=np.zeros(len(groups),np.float32)
    with threadpool_limits(limits=1):
        for fold,model in enumerate(models):
            mask=groups%len(models)==fold
            if mask.any():delta[mask]=model.predict_proba(after[mask])[:,1]-model.predict_proba(before[mask])[:,1]
    weights=np.exp(beta*delta).clip(.25,4.).astype(np.float32)
    tmp=folder/"advantage.tmp.npy";np.save(tmp,weights);tmp.replace(folder/"advantage.npy")
    return dict(shard=shard["path"],n=len(weights),mean=float(weights.mean()),min=float(weights.min()),max=float(weights.max()))


def prepare(cache="data/v9_cache",output="models/policy_v9",workers=4,beta=3.):
    cache,output=Path(cache),Path(output);output.mkdir(parents=True,exist_ok=True)
    if (output/"advantage_report.json").exists():raise ValueError("Advantage evidence already exists")
    ds=ShardDataset(cache/"train",reaction_weight=1.)
    rng=np.random.default_rng(42)
    features=[];labels=[];groups=[]
    for shard in ds.shards:
        # At most two varied prefixes per battle, so long battles do not dominate.
        group=shard["group"]
        starts=np.flatnonzero(np.r_[True,group[1:]!=group[:-1]])
        ends=np.r_[starts[1:],len(group)]
        indices=[]
        for start,end in zip(starts,ends):
            indices.extend(rng.choice(np.arange(start,end),size=min(2,end-start),replace=False).tolist())
        indices=np.asarray(indices)
        features.append(shard["g"][indices]);labels.append(shard["winner"][indices]);groups.append(group[indices])
    X,y,groups=np.concatenate(features),np.concatenate(labels),np.concatenate(groups)
    models=[];pred=np.empty(len(y))
    started=time.monotonic()
    with threadpool_limits(limits=2):
        for fold in range(3):
            mask=groups%3!=fold
            model=HistGradientBoostingClassifier(max_iter=100,max_leaf_nodes=15,max_bins=127,l2_regularization=1.,early_stopping=False,random_state=42+fold)
            model.fit(X[mask],y[mask]);pred[~mask]=model.predict_proba(X[~mask])[:,1];models.append(model)
            print(f"prefix value fold={fold+1}/3 work={int(mask.sum())} ETA={(time.monotonic()-started)/(fold+1)*(2-fold):.0f}s",flush=True)
        vd=ShardDataset(cache/"validation",reaction_weight=1.)
        val_X=np.concatenate([s["g"] for s in vd.shards]);val_y=vd.metadata("winner")
        val_p=np.mean([m.predict_proba(val_X)[:,1] for m in models],axis=0)
    model_path=output/"prefix_value_models.pkl"
    with model_path.open("wb") as f:pickle.dump(models,f,protocol=5)
    report=dict(model_name="v9 cross-fitted causal prefix value",training_battles=len(np.unique(groups)),training_prefixes=len(y),folds=3,fold_unit="whole battle",features="62 causal globals including masked hidden-deck statistics",crossfit_logloss=float(log_loss(y,pred)),crossfit_auc=float(roc_auc_score(y,pred)),validation_logloss=float(log_loss(val_y,val_p)),validation_auc=float(roc_auc_score(val_y,val_p)),constant_validation_logloss=float(log_loss(val_y,np.full(len(val_y),float(y.mean())))),beta=beta,clip=[.25,4.],limitations=["Predicted value change includes time and other same-timestamp actions; it is not a causal treatment effect.","No future action enters the policy inputs. Outcome and after-action information are training weights only.","Each battle's weights use a value model fitted without that battle."])
    report["value_gate_pass"]=report["validation_logloss"]<report["constant_validation_logloss"] and report["validation_auc"]>.5
    with (cache/"battles.pkl").open("rb") as f:population=pickle.load(f)["train"]
    meta=json.loads((cache/"train"/"complete.json").read_text())
    tasks=[];first=0
    for shard in meta["shards"]:
        tasks.append((str(cache),shard,first,population[first:first+shard["battles"]],str(model_path),beta));first+=shard["battles"]
    results=[]
    with ProcessPoolExecutor(workers,mp_context=mp.get_context("spawn")) as pool:
        for result in pool.map(_weight_shard,tasks):
            results.append(result)
            elapsed=time.monotonic()-started
            eta=elapsed/max(len(results),1)*(len(tasks)-len(results))
            print(f"advantage work={len(results)}/{len(tasks)} ETA={eta:.0f}s",flush=True)
    report.update(shards=results,elapsed_seconds=time.monotonic()-started)
    atomic_json(output/"advantage_report.json",report)
    from .v9_report import write_html
    write_html(output/"prefix_value_report.html",dict(**report,status="completed",compute={"device":"cpu","workers":workers},created_at=__import__('datetime').datetime.now(__import__('datetime').timezone.utc).isoformat(),lessons=report["limitations"]))
    return report


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",default="models/policy_v9");p.add_argument("--workers",type=int,default=4);p.add_argument("--beta",type=float,default=3.)
    a=p.parse_args();prepare(a.cache,a.output,a.workers,a.beta)
if __name__=="__main__":main()
