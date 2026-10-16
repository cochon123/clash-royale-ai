"""Sequence-only winner judge and matched offline rollout diagnostics.

Generated deployments have no combat, troop deaths, health or crown results.
Judge preferences are diagnostic scores, never real-game win rates or a gate
for promoting a policy. The final policy holdout is never used to fit this judge.
"""
from __future__ import annotations
import argparse
from datetime import datetime,timezone
from itertools import combinations
import json
from pathlib import Path
import pickle
import random
import time

import numpy as np
import torch
from sklearn.ensemble import ExtraTreesClassifier,HistGradientBoostingClassifier
from sklearn.isotonic import IsotonicRegression
from sklearn.metrics import log_loss,roc_auc_score
from threadpoolctl import threadpool_limits
from .artifact_cache import sha256_file,sha256_json
from .causal_observation import prefix_observation
from .controlled_dataset import tensor_row
from .controlled_experiment import atomic_json
from .policy_dataset import collate_policy_batch
from .policy_model import TILE_COLS,TILE_ROWS
from .v9_policy import load_v9_policy
from .v9_evaluate import LegacyScoringPolicy
from .v9_report import write_html
from .winner_dataset import BattleExample,load_card_costs
from .winner_tabular import _build_card_index,_feature_matrix,_symmetric_probabilities,score_winner_battles


def prepare_judge(cache="data/v9_cache",output="models/policy_v9",train_limit=15000):
    cache,output=Path(cache),Path(output);output.mkdir(parents=True,exist_ok=True)
    artifact_path=output/"sequence_judge.pkl"
    if artifact_path.exists():raise ValueError("Judge already exists; refusing to overwrite")
    with (cache/"battles.pkl").open("rb") as f:population=pickle.load(f)
    train=population["train"][:train_limit];dev=population["validation"]
    rng=np.random.default_rng(42);order=rng.permutation(len(dev));half=len(dev)//2
    val=[dev[i] for i in order[:half]];test=[dev[i] for i in order[half:]]
    costs=load_card_costs(cache/"costs.json");cards=_build_card_index(train)
    started=time.monotonic()
    sets={}
    for name,battles in (("train",train),("validation",val),("test",test)):
        x=_feature_matrix(battles,costs,cards,{},include_official=False)
        rev=_feature_matrix(battles,costs,cards,{},swap_sides=True,include_official=False)
        y=np.array([b.team_wins for b in battles])
        sets[name]=(x,rev,y)
        print(f"sequence judge features work={name} battles={len(battles)} elapsed={time.monotonic()-started:.0f}s",flush=True)
    x,rev,y=sets["train"];xa=np.concatenate([x,rev]);ya=np.r_[y,1-y]
    with threadpool_limits(limits=2):
        hgb=HistGradientBoostingClassifier(max_iter=180,max_leaf_nodes=31,max_bins=127,learning_rate=.05,l2_regularization=1.,early_stopping=False,random_state=42)
        hgb.fit(xa,ya)
        trees=ExtraTreesClassifier(n_estimators=128,max_features=.6,min_samples_leaf=3,n_jobs=2,random_state=42)
        trees.fit(xa,ya)
        vx,vr,vy=sets["validation"];tx,tr,ty=sets["test"]
        hp=_symmetric_probabilities(hgb,vx,vr);tp=_symmetric_probabilities(trees,vx,vr)
        weights=np.linspace(0,1,21);weight=float(weights[np.argmin([log_loss(vy,w*hp+(1-w)*tp) for w in weights])])
        pv=weight*hp+(1-weight)*tp
        confidence=IsotonicRegression(out_of_bounds="clip",y_min=.5,y_max=1.)
        confidence.fit(abs(pv-.5),(pv>=.5)==vy)
        pt=weight*_symmetric_probabilities(hgb,tx,tr)+(1-weight)*_symmetric_probabilities(trees,tx,tr)
        history=[]
        for stage,(direct,reverse) in enumerate(zip(hgb.staged_predict_proba(tx),hgb.staged_predict_proba(tr)),1):
            p=(direct[:,1]+1-reverse[:,1])/2
            if stage==1 or stage%5==0 or stage==180:
                history.append(dict(epoch=stage,winner_accuracy=float(((p>=.5)==ty).mean()),validation_nll=float(log_loss(ty,p))))
    artifact=dict(models={"hist_gradient_boosting":hgb,"extra_trees":trees},hgb_weight=weight,confidence_hgb_weight=weight,confidence_calibrator=confidence,symmetric_inference=True,card_index=cards,seed=42,extra_trees=128,feature_version=6,feature_set="sequence",include_official=False)
    with artifact_path.open("wb") as f:pickle.dump(artifact,f,protocol=5)
    report=dict(kind="classifier",model_name="v9 sequence-only winner judge",version="1.0",created_at=datetime.now(timezone.utc).isoformat(),status="completed",train_battles=len(train),train_actions=len(train)*2,validation_battles=len(val),test_battles=len(test),compute=dict(device="CPU · 2 threads",precision="float64 features",threads=2),parameters="180 boosting stages + 128 extra trees",history=history,test_accuracy=float(((pt>=.5)==ty).mean()),test_auc=float(roc_auc_score(ty,pt)),test_logloss=float(log_loss(ty,pt)),test_brier=float(((pt-ty)**2).mean()),hgb_weight=weight,elapsed_seconds=time.monotonic()-started,chart_keys=[["winner_accuracy","HGB judge accuracy","accuracy"],["validation_nll","HGB judge test NLL","loss"]],description="Sequence-only outcome prediction from real replay histories. Official elixir/leak tables are excluded so generated sequences can use the same feature contract. Curves show the HGB component; final metrics use the validation-selected HGB/ExtraTrees blend.",contract=dict(cache=json.loads((cache/"contract.json").read_text()),training_ids_sha256=sha256_json([b.battle_id for b in train]),calibration_ids_sha256=sha256_json([b.battle_id for b in val]),test_ids_sha256=sha256_json([b.battle_id for b in test])),lessons=["The judge was fitted and calibrated without the final policy holdout.","Calibration on real replay outcomes need not transfer to generated deployment histories.","A judge-favored synthetic sequence does not prove that a policy can win an actual game."])
    atomic_json(output/"sequence_judge_report.json",report);write_html(output/"sequence_judge_report.html",report)
    return report


@torch.inference_mode()
def rollout(team,opponent,source,costs,rng,device,new_events=40):
    events=[dict(e) for e in source.events[:8]]
    while len(events)<len(source.events) and source.events[len(events)]["seconds"]==events[-1]["seconds"]:events.append(dict(source.events[len(events)]))
    next_side="opponent" if events[-1]["side"]=="team" else "team"
    now=float(events[-1]["seconds"])
    for step in range(new_events):
        loaded=team if next_side=="team" else opponent
        model,vocab=loaded
        battle=BattleExample("synthetic",source.team_deck,source.opponent_deck,0,tuple(events))
        observation=prefix_observation(battle,len(events),next_side,now=now)
        batch=tuple(t.to(device) for t in collate_policy_batch([tensor_row(observation.encode(vocab,costs))]))
        out=model(batch)
        logits=out["slot_logits"][0]/.8
        legal=batch[6][0]
        if legal.any():logits=logits.masked_fill(~legal,-torch.inf)
        probabilities=logits.softmax(-1).cpu().numpy()
        slot=int(rng.choices(range(8),weights=probabilities.tolist(),k=1)[0])
        batch=list(batch);batch[7]=torch.tensor([slot],device=device)
        selected=model(tuple(batch),teacher=True)
        tile=int(selected["tile_logits"][0].argmax())
        x=(tile%TILE_COLS+.5)/TILE_COLS
        y=(tile//TILE_COLS+.5)/TILE_ROWS
        if next_side=="opponent":y=1-y
        deck=source.team_deck if next_side=="team" else source.opponent_deck
        # Shared exogenous schedule, rather than comparing incompatible legacy
        # action-delay predictions with competing-risk actor clocks.
        now+=rng.uniform(.5,4.)
        events.append(dict(seconds=now,side=next_side,event_type="card_play",card=deck[slot],x=int(x*18000),y=int(y*32000)))
        next_side="opponent" if next_side=="team" else "team"
    return BattleExample("synthetic",source.team_deck,source.opponent_deck,0,tuple(events))


def tournament(cache="data/v9_cache",output="models/policy_v9",cases=32,device="cuda",deadline=None):
    cache,output=Path(cache),Path(output)
    if (output/"battle_royale.json").exists():raise ValueError("Tournament already recorded")
    selection=json.loads((output/"final_selection.json").read_text())
    seed42=next(e for e in selection["entries"] if e.get("role")=="candidate" and e.get("seed")==42)
    entries=[dict(name="v4.4.1",path="models/policy_bc_v4.4.1",legacy=True),dict(name="v4.4.2",path="models/policy_bc_v4.4.2",legacy=True),dict(name="v9",path=seed42["path"],legacy=False)]
    with (cache/"battles.pkl").open("rb") as f:dev=pickle.load(f)["validation"]
    with (output/"sequence_judge.pkl").open("rb") as f:judge=pickle.load(f)
    torch.set_num_threads(2);device=torch.device(device)
    loaded={}
    for e in entries:
        if e["legacy"]:
            m=LegacyScoringPolicy(e["path"],device);loaded[e["name"]]=(m,m.vocab)
        else:
            m,v,_=load_v9_policy(Path(e["path"])/"best.pt",device);loaded[e["name"]]=(m,v)
    costs=load_card_costs(cache/"costs.json");rng=random.Random(42)
    sources=[dev[i] for i in rng.sample(range(len(dev)),min(cases,len(dev)))]
    games=[];records=[];total=len(list(combinations(entries,2)))*len(sources)*2;started=time.monotonic()
    for left,right in combinations(entries,2):
        for case,source in enumerate(sources):
            for team,opp in [(left,right),(right,left)]:
                if deadline and time.time()>=deadline:break
                battle=rollout(loaded[team["name"]],loaded[opp["name"]],source,costs,random.Random(42+case),device)
                games.append(battle);records.append(dict(team=team["name"],opponent=opp["name"],case=case))
                elapsed=time.monotonic()-started
                if len(games)%8==0:print(f"offline royale work={len(games)}/{total} ETA={elapsed/len(games)*(total-len(games)):.0f}s",flush=True)
    if not games:
        result=dict(status="deadline_stopped",games=0,planned_games=total,summary=[],records=[],limitations=["No synthetic sequence was generated before the deadline; no judge result exists."])
        atomic_json(output/"battle_royale.json",result)
        return result
    with threadpool_limits(limits=2):probability,confidence=score_winner_battles(judge,games,costs)
    for row,p,c in zip(records,probability,confidence):row.update(judge_team_probability=float(p),judge_confidence_on_real_replays=float(c))
    summary=[]
    for left,right in combinations(entries,2):
        p=np.array([r["judge_team_probability"] if r["team"]==left["name"] else 1-r["judge_team_probability"] for r in records if {r["team"],r["opponent"]}=={left["name"],right["name"]}])
        summary.append(dict(left=left["name"],right=right["name"],generated_sequences=len(p),mean_judge_probability_left=float(p.mean()) if len(p) else None,judge_favored_left_fraction=float((p>.5).mean()) if len(p) else None))
    result=dict(created_at=datetime.now(timezone.utc).isoformat(),status="completed" if len(games)==total else "deadline_stopped",games=len(games),planned_games=total,summary=summary,records=records,judge_report=json.loads((output/"sequence_judge_report.json").read_text()),decoder=dict(card="sample at temperature .8 with inferred-hand mask",placement="selected-card tile argmax",schedule="shared alternating turns, uniform .5–4 s"),elapsed_seconds=time.monotonic()-started,limitations=["These are generated action histories, not a combat simulation.","Judge preferences are not actual win rates and do not affect promotion.","A shared turn schedule avoids favoring incompatible legacy and clock delay decoders."])
    atomic_json(output/"battle_royale.json",result)
    return result


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("mode",choices=["prepare","tournament"]);p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",default="models/policy_v9");p.add_argument("--cases",type=int,default=32);p.add_argument("--device",default="cuda")
    a=p.parse_args()
    if a.mode=="prepare":prepare_judge(a.cache,a.output)
    else:tournament(a.cache,a.output,a.cases,a.device)
if __name__=="__main__":main()
