"""Finish reports, diagnostic rollouts and a frozen deployment artifact."""
from __future__ import annotations
import argparse
from datetime import datetime,timezone
import json
from pathlib import Path
import pickle
import shutil
import time

import numpy as np
import torch
from sklearn.metrics import log_loss
from threadpoolctl import threadpool_limits
from .artifact_cache import sha256_file
from .controlled_experiment import atomic_json
from .v9_cache import ShardDataset
from .v9_evaluate import score_checkpoint
from .v9_report import write_html,write_suite_report,write_run_report


def value_curves(cache,output):
    report_path=output/"advantage_report.json"
    if not report_path.exists():return
    report=json.loads(report_path.read_text())
    if report.get("history"):return
    ds=ShardDataset(cache/"validation",reaction_weight=1.)
    X=np.concatenate([s["g"] for s in ds.shards]);y=ds.metadata("winner")
    with (output/"prefix_value_models.pkl").open("rb") as f:models=pickle.load(f)
    history=[]
    with threadpool_limits(limits=2):
        iterators=[m.staged_predict_proba(X) for m in models]
        for stage,probabilities in enumerate(zip(*iterators),1):
            p=np.mean([a[:,1] for a in probabilities],axis=0)
            if stage==1 or stage%5==0 or stage==100:
                history.append(dict(epoch=stage,winner_accuracy=float(((p>=.5)==y).mean()),validation_nll=float(log_loss(y,p))))
    report.update(kind="classifier",created_at=datetime.now(timezone.utc).isoformat(),status="completed",train_battles=report["training_battles"],train_actions=report["training_prefixes"],compute=dict(device="CPU · 2 threads",threads=2,folds=3),history=history,chart_keys=[["winner_accuracy","Prefix winner accuracy","accuracy"],["validation_nll","Prefix binary NLL","loss"]],description="Three cross-fitted HGB models estimate eventual own-player win probability from 62 causal observation globals. Actions use values predicted by a fold that excluded their battle. Curves show the ensemble on development data; the policy never receives value labels or future state as inputs.",lessons=report["limitations"])
    atomic_json(report_path,report);write_html(output/"prefix_value_report.html",report)


def correct_pilot_tolerance(cache,output):
    path=output/"scale_5k_seed42";report_path=path/"report.json"
    if not report_path.exists():return
    report=json.loads(report_path.read_text())
    if report.get("physical_tolerance_rescored"):return
    from .winner_dataset import CardVocab
    vocab=CardVocab.from_dict(json.loads((cache/"vocab.json").read_text()))
    ds=ShardDataset(cache/"validation",reaction_weight=1.)
    metrics=score_checkpoint(path/"best.pt",ds,vocab,"cpu",output=path/"validation_rows.npz")
    for row in report["history"]:
        if "cell_teacher_near1" in row:row["cell_teacher_near1_bins"]=row.pop("cell_teacher_near1")
        if row["epoch"]==report["weights_epoch"]:row.update(metrics)
    report["selected_validation"]=next(r for r in report["history"] if r["epoch"]==report["weights_epoch"])
    report["physical_tolerance_rescored"]=True
    report["measurement_note"]="Initial pilot tolerance curve used grid bins; those points are archived as near1_bins. The selected checkpoint was re-scored using physical arena tiles, without retraining."
    write_run_report(path,report)


def package(cache,output):
    selection=json.loads((output/"final_selection.json").read_text())
    holdout=json.loads((output/"final_holdout.json").read_text())
    # Seed42 was selected before final scoring. Never pick a lucky test seed.
    entry=next(e for e in selection["entries"] if e.get("role")=="candidate" and e.get("seed")==42)
    source=Path(entry["path"])/"best.pt"
    if sha256_file(source)!=entry["checkpoint_sha256"]:raise ValueError("Final candidate checksum changed")
    folder=output/"final_model";folder.mkdir(exist_ok=True)
    full=torch.load(source,map_location="cpu",weights_only=False)
    artifact={key:full[key] for key in ("model_state","vocab","config","contract","weights_epoch")}
    artifact["created_at"]=datetime.now(timezone.utc).isoformat()
    artifact["promoted"]=holdout["predeclared_gate"]["pass_gate"]
    torch.save(artifact,folder/"best.pt")
    shutil.copy2(cache/"costs.json",folder/"costs.json")
    atomic_json(folder/"manifest.json",dict(source=str(source),source_sha256=entry["checkpoint_sha256"],deployment_sha256=sha256_file(folder/"best.pt"),seed=42,promoted=artifact["promoted"],recipe=selection["recipe"],final_gate=holdout["predeclared_gate"],inference_module="cr_replay_pipeline.v9_infer",limitations=["Offline deployment proposals; no real-game win claim.","Current troop health, deaths and targeting are not observed.","Ability action selection is unsupported.",*holdout.get("limitations",[])]))
    (folder/"README.md").write_text('''# Policy v9 deployment candidate

Seed 42 is exported because that seed was specified before final scoring.
Read `manifest.json` for the final gate and whether this artifact was promoted.
This package omits optimizer/RNG state; resumable training checkpoints remain
in the source run. It proposes card plays and never taps a phone.

```python
from cr_replay_pipeline.v9_infer import load_model, predict
from cr_replay_pipeline.winner_dataset import load_card_costs
model, vocab, config = load_model("models/policy_v9/final_model/best.pt")
costs = load_card_costs("models/policy_v9/final_model/costs.json")
proposal = predict(model, vocab, costs, battle, confirmed_events,
                   agent_side="team", now=observation_time,
                   observed_hand=recognized_card_names)
```

Only confirmed events before `now` may enter the observation. Unrecognized
hands remain unknown. New events invalidate old proposals; requery within
250 ms. Coordinates follow the same frame used by the causal training encoder.
''')


def finish(cache="data/v9_cache",output="models/policy_v9",deadline="2026-10-12T06:00:00-06:00"):
    cache,output=Path(cache),Path(output)
    end=datetime.fromisoformat(deadline).timestamp()
    torch.set_num_threads(2)
    value_curves(cache,output)
    correct_pilot_tolerance(cache,output)
    while time.time()<end:
        final=output/"final_holdout.json"
        if final.exists() and json.loads((output/"final_holdout_state.json").read_text())["status"]=="completed":break
        print(f"postprocess waiting work=reports prepared ETA={max(0,end-time.time()):.0f}s",flush=True)
        time.sleep(30)
    state_path=output/"final_holdout_state.json"
    if not (output/"final_holdout.json").exists() or not state_path.exists() or json.loads(state_path.read_text()).get("status")!="completed":
        write_suite_report(output,dict(status="incomplete",lessons=["The deadline arrived before final holdout evaluation completed. No final improvement claim is available.","Inspect status.json and the preserved run reports for completed work."]))
        return
    package(cache,output)
    if (output/"sequence_judge.pkl").exists() and not (output/"battle_royale.json").exists() and end-time.time()>300:
        from .v9_royale import tournament
        tournament(cache,output,cases=32,device="cpu",deadline=end-180)
    previous=json.loads((output/"suite_report.json").read_text())
    for p in output.glob("*/report.json"):write_html(p.with_name("report.html"),json.loads(p.read_text()))
    previous["final_artifact"]="models/policy_v9/final_model/best.pt"
    if (output/"battle_royale.json").exists():previous["offline_royale"]=json.loads((output/"battle_royale.json").read_text())
    previous.pop("runs",None)
    write_suite_report(output,previous)
    atomic_json(output/"postprocess_status.json",dict(status="completed",created_at=datetime.now(timezone.utc).isoformat(),artifact="models/policy_v9/final_model/best.pt",reports="reports/policy_v9.html",needs_browser_qa=True))


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",default="models/policy_v9");p.add_argument("--deadline",default="2026-10-12T06:00:00-06:00")
    a=p.parse_args();finish(a.cache,a.output,a.deadline)
if __name__=="__main__":main()
