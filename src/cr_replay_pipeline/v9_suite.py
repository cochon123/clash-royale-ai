"""One GPU queue for the authorized overnight scaling/ablation/seed suite."""
from __future__ import annotations
import argparse
from datetime import datetime,timezone
import fcntl
import json
import os
from pathlib import Path
import subprocess
import sys
import time

from .controlled_experiment import atomic_json
from .v9_evaluate import paired_comparison,freeze_selection
from .v9_report import write_suite_report,write_run_report


class Suite:
    def __init__(self,args):
        self.args=args;self.output=Path(args.output);self.output.mkdir(parents=True,exist_ok=True)
        self.lock=(self.output/"suite.lock").open("a");fcntl.flock(self.lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        self.deadline=datetime.fromisoformat(args.deadline).timestamp()
        self.training_end=self.deadline-45*60
        self.started=time.time();self.jobs=[]
        self.status("initializing")

    def status(self,phase,**extra):
        state=dict(status="running",phase=phase,deadline=self.args.deadline,remaining_seconds=max(0,self.deadline-time.time()),jobs=self.jobs,updated_at=datetime.now(timezone.utc).isoformat(),**extra)
        atomic_json(self.output/"status.json",state)
        print(f"SUITE phase={phase} work={len(self.jobs)} jobs ETA={state['remaining_seconds']:.0f}s",flush=True)

    def command(self,argv,phase):
        self.status(phase,command=argv)
        subprocess.run([sys.executable,*argv],check=True,env={**os.environ,"PYTHONUNBUFFERED":"1"})

    def run(self,name,epochs=8,width=160,limit=None,warmstart=None,seed=42,**options):
        path=self.output/name
        report_path=path/"report.json"
        if report_path.exists():
            report=json.loads(report_path.read_text())
            if report["last_completed_epoch"]>=epochs or report["status"]=="early_stopped":
                self.jobs.append(dict(name=name,status=report["status"],epochs=report["last_completed_epoch"]))
                return report
        if time.time()>=self.training_end:raise TimeoutError("Reserved final-evaluation window reached")
        argv=["-m","cr_replay_pipeline.v9_train","--cache",self.args.cache,"--output",str(path),"--epochs",str(epochs),"--d-model",str(width),"--seed",str(seed),"--deadline",datetime.fromtimestamp(self.training_end,timezone.utc).isoformat(),"--progress",str(self.output/"progress.jsonl")]
        if limit:argv += ["--limit-battles",str(limit)]
        if warmstart:argv += ["--warmstart",str(warmstart),"--lr","0.0001"]
        for key,value in options.items():
            if value is True:argv += ["--"+key.replace("_","-")]
            elif value is not False:argv += ["--"+key.replace("_","-"),str(value)]
        if (path/"last.pt").exists():argv += ["--resume"]
        self.command(argv,name)
        report=json.loads(report_path.read_text())
        self.jobs.append(dict(name=name,status=report["status"],epochs=report["last_completed_epoch"],seconds=report["elapsed_seconds"]))
        write_suite_report(self.output,dict(train_battles=report["train_battles"],train_actions=report["train_actions"]))
        return report


def action_metrics(report):
    return report["selected_validation"]


def gate(base,candidate,comparison):
    common=set(h["epoch"] for h in base["history"])&set(h["epoch"] for h in candidate["history"])
    epoch=max(common)
    a=next(h for h in base["history"] if h["epoch"]==epoch)
    b=next(h for h in candidate["history"] if h["epoch"]==epoch)
    card,joint,nll=b["card"]-a["card"],b["joint"]-a["joint"],b["joint_nll"]-a["joint_nll"]
    # Keep the joint scoring rule primary. Spatial tolerance cannot hide a
    # material regression in exact actions or probability assigned to them.
    passed=nll<=-.01 and card>=-.005 and joint>=-.005
    return dict(pass_gate=passed,equal_completed_epoch=epoch,card_delta=card,joint_delta=joint,joint_nll_delta=nll,paired=comparison)


def run(args):
    suite=Suite(args)
    # Existing pilot services are owned by this same authorized run. Wait for
    # them instead of starting a second GPU trainer.
    for unit in ("cr-v9-train.service","cr-v9-scale15k.service"):
        while subprocess.run(["systemctl","--user","is-active","--quiet",unit]).returncode==0:
            pilot="scale_5k_seed42" if unit=="cr-v9-train.service" else "scale_15k_seed42"
            report_path=suite.output/pilot/"report.json"
            if report_path.exists() and json.loads(report_path.read_text())["status"] in {"completed","early_stopped"}:
                # Early prototypes used file_system sharing and can hang at
                # interpreter shutdown after all checkpoint/report writes.
                subprocess.run(["systemctl","--user","stop",unit],check=True)
                break
            suite.status("waiting for "+unit);time.sleep(15)
    if not (suite.output/"references.json").exists():
        suite.command(["-m","cr_replay_pipeline.v9_evaluate","references","--cache",args.cache,"--output",args.output],"reference scoring")
    suite.run("scale_5k_seed42",epochs=4,limit=5000)
    suite.run("scale_15k_seed42",epochs=4,limit=15000)
    base=suite.run("base_full_seed42",epochs=args.base_epochs)
    donor=suite.output/"base_full_seed42";width=160
    large=suite.run("size_256_seed42",epochs=2,width=256)
    comparison=paired_comparison(donor/"validation_e2.npz",suite.output/"size_256_seed42"/"validation_e2.npz")
    lm=comparison["metrics"]
    size_quality=lm["joint_nll"]["action_weighted_delta"]<=-.05 and lm["card"]["action_weighted_delta"]>=-.005 and (lm["card"]["action_weighted_delta"]>=.01 or lm["joint"]["action_weighted_delta"]>=.01)
    epoch_seconds=sum(h["epoch_seconds"] for h in large["history"])/len(large["history"])
    future_epochs=(args.base_epochs-2)+args.ablation_epochs*6+2*(args.base_epochs+args.ablation_epochs)
    feasible=epoch_seconds*future_epochs < (suite.training_end-time.time())*.85
    size_gate=dict(quality_pass=size_quality,deadline_feasible=feasible,forecast_seconds=epoch_seconds*future_epochs,comparison=comparison)
    atomic_json(suite.output/"size_gate.json",size_gate)
    if size_quality and feasible:
        base=suite.run("size_256_seed42",epochs=args.base_epochs,width=256)
        donor=suite.output/"size_256_seed42";width=256
    candidates={}
    control=suite.run("control_seed42",epochs=args.ablation_epochs,width=width,warmstart=donor)
    candidates["control_seed42"]=(control,{})
    trials=[("winner_seed42",dict(winner_weight=3.)),("smooth_seed42",dict(sigma=.45))]
    advantage_report=suite.output/"advantage_report.json"
    if advantage_report.exists() and json.loads(advantage_report.read_text())["value_gate_pass"]:
        trials.append(("advantage_seed42",dict(advantage=True)))
    trials += [("ghost_seed42",dict(ghost="aligned")),("ghost_shuffled_seed42",dict(ghost="shuffled"))]
    gates={}
    for name,options in trials:
        report=suite.run(name,epochs=args.ablation_epochs,width=width,warmstart=donor,**options)
        shared=min(control["last_completed_epoch"],report["last_completed_epoch"])
        comparison=paired_comparison(suite.output/"control_seed42"/f"validation_e{shared}.npz",suite.output/name/f"validation_e{shared}.npz")
        gates[name]=gate(control,report,comparison)
        if gates[name]["pass_gate"] and name!="ghost_shuffled_seed42":candidates[name]=(report,options)
        atomic_json(suite.output/"ablation_gates.json",gates)
    if "ghost_seed42" in candidates:
        shuffled=paired_comparison(suite.output/"ghost_shuffled_seed42"/f"validation_e{args.ablation_epochs}.npz",suite.output/"ghost_seed42"/f"validation_e{args.ablation_epochs}.npz")
        earned=shuffled["metrics"]["joint_nll"]["ci95_battle"][1]<0
        gates["ghost_seed42"]["shuffled_control_pass"]=earned
        gates["ghost_seed42"]["shuffled_control_comparison"]=shuffled
        if not earned:candidates.pop("ghost_seed42")
        atomic_json(suite.output/"ablation_gates.json",gates)
    if "winner_seed42" in candidates and "smooth_seed42" in candidates:
        options=dict(winner_weight=3.,sigma=.45)
        combined=suite.run("winner_smooth_seed42",epochs=args.ablation_epochs,width=width,warmstart=donor,**options)
        comparison=paired_comparison(suite.output/"control_seed42"/f"validation_e{args.ablation_epochs}.npz",suite.output/"winner_smooth_seed42"/f"validation_e{args.ablation_epochs}.npz")
        gates["winner_smooth_seed42"]=gate(control,combined,comparison)
        if gates["winner_smooth_seed42"]["pass_gate"]:candidates["winner_smooth_seed42"]=(combined,options)
    chosen=min(candidates,key=lambda name:action_metrics(candidates[name][0])["joint_nll"])
    selected_report,options=candidates[chosen]
    # A continuation that worsens the donor is not a required final stage.
    continue_recipe=action_metrics(selected_report)["joint_nll"]<=action_metrics(base)["joint_nll"]
    selected=suite.output/chosen if continue_recipe else donor
    if args.continuation_seeds_only and not continue_recipe:
        raise ValueError("Shared-donor seed checks require an eligible continuation")
    scope="shared_donor_continuations" if args.continuation_seeds_only else "independent_full_training"
    recipe=dict(width=width,base_epochs=args.base_epochs,continuation_epochs=args.ablation_epochs if continue_recipe else 0,options=options if continue_recipe else {},seed42_source=str(selected),selection_metric="validation joint discrete NLL",chosen_before_holdout=True,seed_reproduction=scope)
    atomic_json(suite.output/"selected_recipe.json",recipe)
    final_paths={42:selected}
    for seed in (43,44):
        seed_base=donor if args.continuation_seeds_only else suite.output/f"base_full_seed{seed}"
        if not args.continuation_seeds_only:suite.run(seed_base.name,epochs=args.base_epochs,width=width,seed=seed)
        if continue_recipe:
            target=suite.output/f"candidate_seed{seed}"
            suite.run(target.name,epochs=args.ablation_epochs,width=width,warmstart=seed_base,seed=seed,**options)
            final_paths[seed]=target
        else:final_paths[seed]=seed_base
    entries=[dict(name="v4_4_1",path="models/policy_bc_v4.4.1",legacy=True,role="reference"),dict(name="v4_4_2",path="models/policy_bc_v4.4.2",legacy=True,role="reference"),dict(name="v9_base_seed42",path=str(donor),role="control")]
    entries += [dict(name=f"v9_seed{seed}",path=str(path),role="candidate",seed=seed) for seed,path in final_paths.items()]
    freeze_selection(suite.output,entries,recipe)
    suite.command(["-m","cr_replay_pipeline.v9_evaluate","holdout","--cache",args.cache,"--output",args.output],"one final holdout pass")
    holdout=json.loads((suite.output/"final_holdout.json").read_text())
    message="The predeclared +2 pp card and +2 pp joint gate passed in all three seeds." if holdout["predeclared_gate"]["pass_gate"] else "The predeclared improvement gate was not met in all three seeds. Keep the reference as the promoted model; v9 remains an experimental candidate."
    if args.continuation_seeds_only:
        message="Not promoted: the deadline allowed only three fresh continuation seeds sharing one trained donor. The original independent full-training reproduction requirement is incomplete."
    write_suite_report(suite.output,dict(status="completed",train_battles=base["train_battles"],train_actions=base["train_actions"],gate_message=message,final_holdout=holdout,selected_recipe=recipe,ablation_gates=gates,lessons=[message,"Training throughput and corpus scaling are measured separately from architecture and loss changes.","Winner weighting and ghost motion were kept only if their predeclared validation gates passed.","The final holdout was consumed once, after all candidate checkpoints were frozen. Real playing strength is still unmeasured."]))
    suite.status("completed",final_gate_pass=holdout["predeclared_gate"]["pass_gate"],selected_recipe=recipe)
    state=json.loads((suite.output/"status.json").read_text());state["status"]="completed";atomic_json(suite.output/"status.json",state)


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",default="models/policy_v9");p.add_argument("--deadline",default="2026-10-12T06:00:00-06:00");p.add_argument("--base-epochs",type=int,default=8);p.add_argument("--ablation-epochs",type=int,default=2);p.add_argument("--continuation-seeds-only",action="store_true",help="Deadline recovery only; shared donor checks cannot satisfy the original promotion gate")
    a=p.parse_args()
    try:run(a)
    except BaseException as e:
        import traceback
        Path(a.output,"suite_crash.txt").write_text(traceback.format_exc())
        state_path=Path(a.output,"status.json")
        state=json.loads(state_path.read_text()) if state_path.exists() else {}
        state.update(status="failed",suite_error=repr(e),updated_at=datetime.now(timezone.utc).isoformat())
        atomic_json(state_path,state)
        write_suite_report(a.output,dict(status="failed",suite_error=repr(e)))
        raise
    finally:
        # Release the user's resident GPU services even after a failed suite.
        subprocess.run(["systemctl","--user","unmask","--runtime","lettera-qwen-asr.service"])
        subprocess.run(["systemctl","--user","start","lettera-qwen-asr.service","comfyui.service"])
if __name__=="__main__":main()
