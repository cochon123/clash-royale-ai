"""Fixed-cohort comparisons and a transaction protecting one final holdout pass."""
from __future__ import annotations
import argparse
from datetime import datetime,timezone
import fcntl
import json
from pathlib import Path
import pickle
import shutil

import numpy as np
import torch
from .artifact_cache import sha256_file,sha256_json
from .controlled_experiment import atomic_json
from .policy_manifest import load_manifest
from .v9_cache import ShardDataset,_write_shard,card_event_fingerprint,FIELDS
from .v9_policy import load_v9_policy
from .v9_train import LegacyPolicy,score,COLUMNS
from .winner_dataset import CardVocab,collect_battles,load_card_costs
from .feature_schema import CONTINUOUS_INDEX

LEGACY_ADAPTER_VERSION="legacy-unused-pad-zero-1"


class LegacyScoringPolicy(LegacyPolicy):
    def forward(self,batch,teacher=False):
        # v9's position-unknown flag repurposes a column that was always zero
        # in legacy training. Preserve unknown coordinates, not an untrained flag.
        adapted=list(batch);continuous=batch[0].clone()
        continuous[:,:,CONTINUOUS_INDEX["pad"]]=0
        adapted[0]=continuous
        return super().forward(tuple(adapted),teacher)


def paired_comparison(left,right,draws=2000):
    with np.load(left) as a,np.load(right) as b:
        if not np.array_equal(a["groups"],b["groups"]) or not np.array_equal(a["events"],b["events"]):raise ValueError("Evaluation targets differ")
        if not np.array_equal(a["columns"],b["columns"]):raise ValueError("Evaluation row definitions differ")
        groups=a["groups"];delta=b["rows"]-a["rows"];columns=a["columns"].tolist()
    # Equal weight per battle: no long-battle leverage in the uncertainty interval.
    unique=np.unique(groups);battle=np.stack([delta[groups==g].mean(0) for g in unique])
    rng=np.random.default_rng(42)
    boot=np.empty((draws,battle.shape[1]))
    for i in range(draws):boot[i]=battle[rng.integers(0,len(battle),len(battle))].mean(0)
    ci=np.quantile(boot,[.025,.975],axis=0)
    return dict(battles=len(unique),actions=len(groups),unit="whole battle; equal battle-weighted interval",metrics={k:dict(action_weighted_delta=float(delta[:,j].mean()),battle_weighted_delta=float(battle[:,j].mean()),ci95_battle=ci[:,j].tolist()) for j,k in enumerate(columns) if k not in {"now_s","defense","winner"}})


def score_checkpoint(path,ds,vocab,device="cuda",legacy=False,output=None):
    device=torch.device(device)
    if legacy:model=LegacyScoringPolicy(path,device)
    else:
        model,loaded_vocab,_=load_v9_policy(path,device)
        if not vocab.compatible_with(loaded_vocab):raise ValueError("v9 scoring vocabulary changed")
    metrics,rows=score(model,ds,device,source_vocab=vocab)
    if output:
        np.savez_compressed(output,rows=rows,groups=ds.metadata("group"),events=ds.metadata("event"),columns=np.array(COLUMNS))
    del model
    if device.type=="cuda":torch.cuda.empty_cache()
    return metrics


def references(cache="data/v9_cache",output="models/policy_v9",device="cuda"):
    cache,output=Path(cache),Path(output);output.mkdir(parents=True,exist_ok=True)
    vocab=CardVocab.from_dict(json.loads((cache/"vocab.json").read_text()))
    ds=ShardDataset(cache/"validation",reaction_weight=1.)
    result={"cohort":"fixed validation","legacy_input_adapter":LEGACY_ADAPTER_VERSION,"cache_contract":json.loads((cache/"contract.json").read_text()),"runs":[]}
    for name in ["policy_bc_v4.4.1","policy_bc_v4.4.2"]:
        model_dir=Path("models")/name
        metrics=score_checkpoint(model_dir,ds,vocab,device,legacy=True,output=output/f"{name}_validation.npz")
        result["runs"].append(dict(model_name=name,metrics=metrics,checkpoint_sha256=sha256_file(model_dir/"best_model.pt"),validation_actions=len(ds),train_battles=None,limitations=["Common held-out cohort of the final reference manifests; older warm-start ancestry is incompletely recorded. The final new holdout is authoritative."]))
        print("REFERENCE",name,json.dumps(metrics),flush=True)
    atomic_json(output/"references.json",result)
    return result


def freeze_selection(output,entries,recipe):
    output=Path(output)
    selection={"created_at":datetime.now(timezone.utc).isoformat(),"recipe":recipe,"entries":[]}
    for e in entries:
        checkpoint=Path(e["path"])/("best_model.pt" if e.get("legacy") else "best.pt")
        selection["entries"].append(dict(**e,checkpoint_sha256=sha256_file(checkpoint)))
    path=output/"final_selection.json"
    if path.exists():
        existing=json.loads(path.read_text())
        if existing["entries"]!=selection["entries"] or existing["recipe"]!=recipe:raise ValueError("Final candidate selection is already frozen")
        return existing
    atomic_json(path,selection)
    return selection


def resume_cohort_shard(task):
    """Reuse completed tensors; rebuild only an unfinished generated shard."""
    root,shard_id,battles,*_=task
    path=Path(root)/f"shard_{shard_id:04d}"
    complete=path/"complete.json"
    if complete.exists():
        record=json.loads(complete.read_text())
        if record["path"]!=path.name or record["battles"]!=len(battles):
            raise ValueError("Completed final cohort shard identity changed")
        for kind in ("actions","clocks"):
            for key,(dtype,shape) in FIELDS.items():
                array=np.load(path/kind/f"{key}.npy",mmap_mode="r")
                if array.dtype!=np.dtype(dtype) or array.shape!=(record[kind],*shape):
                    raise ValueError("Completed final cohort shard shape changed")
        return record
    if path.exists():shutil.rmtree(path)
    return _write_shard(task)


def holdout_pass(cache="data/v9_cache",output="models/policy_v9",device="cuda"):
    cache,output=Path(cache),Path(output)
    selection=json.loads((output/"final_selection.json").read_text())
    selection_hash=sha256_json(selection)
    lock=(output/"final_holdout.lock").open("a")
    fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    state_path=output/"final_holdout_state.json"
    state=json.loads(state_path.read_text()) if state_path.exists() else dict(selection_sha256=selection_hash,status="running",scored=[])
    if state["selection_sha256"]!=selection_hash:raise ValueError("Holdout is bound to another frozen selection")
    if state["status"]=="completed":return json.loads((output/"final_holdout.json").read_text())
    # A deadline-truncated final seed is not a completed recipe reproduction.
    # Reject it before preparing or scoring the reserved cohort.
    for entry in selection["entries"]:
        report_path=Path(entry["path"])/"report.json"
        if entry.get("role") in {"candidate","control"} and report_path.exists():
            run_report=json.loads(report_path.read_text())
            if run_report.get("status") not in {"completed","early_stopped"}:
                raise ValueError("Final candidate run has not completed: "+entry["name"])
    atomic_json(state_path,state)
    vocab=CardVocab.from_dict(json.loads((cache/"vocab.json").read_text()))
    manifest=load_manifest("data/splits/controlled_final_holdout.json")
    contract=json.loads((cache/"contract.json").read_text())
    if sha256_file("data/splits/controlled_final_holdout.json")!=contract["holdout_sha256"]:raise ValueError("Reserved holdout changed")
    cohort=output/"final_cohort"
    if not (cohort/"complete.json").exists():
        if sha256_file(cache/"metadata.jsonl")!=contract["metadata_sha256"] or sha256_file(cache/"costs.json")!=contract["costs_sha256"]:raise ValueError("Frozen preprocessing inputs changed")
        battles=collect_battles("data/raw",battle_ids=set(manifest["test_ids"]),require_decisive_result=False,metadata_path=cache/"metadata.jsonl",unresolved_abilities="preserve_event")
        by_id={b.battle_id:b for b in battles}
        population=[by_id[x] for x in manifest["test_ids"]]
        with (cache/"battles.pkl").open("rb") as f:known=pickle.load(f)
        known_hashes={card_event_fingerprint(b) for b in known["train"]+known["validation"]}
        if any(card_event_fingerprint(b) in known_hashes for b in population):raise ValueError("Holdout content overlaps development")
        cohort.mkdir(exist_ok=True);shards=[]
        build=dict(selection_sha256=selection_hash,contract=contract,holdout_ids=manifest["test_ids"],vocab_sha256=sha256_json(vocab.to_dict()))
        build_path=cohort/"build.json"
        if build_path.exists():
            if json.loads(build_path.read_text())!=build:raise ValueError("Final cohort build contract changed")
        elif list(cohort.glob("shard_*")):
            raise ValueError("Final cohort shards have no build contract")
        else:atomic_json(build_path,build)
        for j,i in enumerate(range(0,len(population),256)):
            shards.append(resume_cohort_shard((str(cohort),j,population[i:i+256],i,vocab.to_dict(),load_card_costs(cache/"costs.json"))))
        atomic_json(cohort/"complete.json",dict(battles=len(population),shards=shards,contract=contract))
    ds=ShardDataset(cohort,reaction_weight=1.)
    scores={}
    for entry in selection["entries"]:
        name=entry["name"];path=Path(entry["path"]);artifact=path/("best_model.pt" if entry.get("legacy") else "best.pt")
        if sha256_file(artifact)!=entry["checkpoint_sha256"]:raise ValueError("Frozen final checkpoint changed")
        score_path=output/("holdout_"+name+".json")
        if name not in state["scored"]:
            # A selected donor can also appear under the control label. Score
            # each checkpoint/decoder once, then alias its unchanged evidence.
            prior=next((e for e in selection["entries"] if e["name"] in state["scored"] and e["checkpoint_sha256"]==entry["checkpoint_sha256"] and bool(e.get("legacy"))==bool(entry.get("legacy"))),None)
            if prior:
                previous=json.loads((output/("holdout_"+prior["name"]+".json")).read_text())
                metrics=previous["metrics"]
                shutil.copy2(output/("holdout_"+prior["name"]+".npz"),output/("holdout_"+name+".npz"))
            else:
                metrics=score_checkpoint(path if entry.get("legacy") else artifact,ds,vocab,device,legacy=entry.get("legacy",False),output=output/("holdout_"+name+".npz"))
            # Final holdout can include draws; BattleExample cannot encode an
            # unknown winner. Do not present misleading winner/loser slices.
            metrics={k:v for k,v in metrics.items() if not k.startswith(("winners_","losers_"))}
            atomic_json(score_path,dict(entry=entry,metrics=metrics,scored_as=prior["name"] if prior else name))
            state["scored"].append(name);atomic_json(state_path,state)
            print("FINAL HOLDOUT",name,"card",metrics["card"],"joint",metrics["joint"],flush=True)
        scores[name]=json.loads(score_path.read_text())
    base=scores["v4_4_1"]["metrics"]
    seeds=[e for e in selection["entries"] if e.get("role")=="candidate"]
    gates={e["name"]:dict(card_delta=scores[e["name"]]["metrics"]["card"]-base["card"],joint_delta=scores[e["name"]]["metrics"]["joint"]-base["joint"],joint_nll_delta=scores[e["name"]]["metrics"]["joint_nll"]-base["joint_nll"]) for e in seeds}
    accuracy_pass=len(seeds)==3 and all(x["card_delta"]>=.02 and x["joint_delta"]>=.02 for x in gates.values())
    scope=selection["recipe"].get("seed_reproduction","independent_full_training")
    passed=accuracy_pass and scope=="independent_full_training"
    comparisons={name:paired_comparison(output/"holdout_v4_4_1.npz",output/("holdout_"+name+".npz")) for name in gates}
    result=dict(cohort_battles=len(manifest["test_ids"]),actions=len(ds),selection=selection,scores=scores,predeclared_gate=dict(card_gain=.02,joint_gain=.02,seeds_required=3,all_seeds_required=True,independent_full_training_required=True,seed_reproduction=scope,accuracy_thresholds_met=accuracy_pass,pass_gate=passed,by_seed=gates),paired_comparisons=comparisons,limitations=["Offline policy accuracy is not a measured real-game win rate.","This holdout was used only after final checkpoints were frozen."])
    if scope!="independent_full_training":result["limitations"].append("The three continuation seeds share one pretrained donor. They do not establish variability across independent full-training runs; the original promotion gate remains incomplete.")
    result["legacy_input_adapter"]=LEGACY_ADAPTER_VERSION
    atomic_json(output/"final_holdout.json",result)
    state["status"]="completed";atomic_json(state_path,state)
    return result


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("mode",choices=["references","holdout"]);p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",default="models/policy_v9");p.add_argument("--device",default="cuda")
    a=p.parse_args();torch.set_num_threads(2)
    (references if a.mode=="references" else holdout_pass)(a.cache,a.output,a.device)
if __name__=="__main__":main()
