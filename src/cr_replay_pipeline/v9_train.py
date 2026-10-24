"""Cached, deadline-bounded v9 training and a shared causal scoreboard."""
from __future__ import annotations
import argparse
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import time
import traceback
import zipfile
import sys
import platform

import numpy as np
import torch
from torch import nn
from torch.utils.data import DataLoader
from .artifact_cache import sha256_json, sha256_file
from .controlled_experiment import atomic_json
from .event_clock import competing_risk_nll
from .feature_schema import GLOBAL_INDEX as G, GLOBAL_DIM, THREAT_INDEX
from .policy_infer import load_policy
from .policy_model import TILE_COLS, TILE_ROWS
from .v9_cache import ShardDataset, ShardSampler, identity
from .v9_policy import V9Policy, tile_loss, load_motion_stats
from .winner_dataset import CardVocab

COLUMNS = ("card","cell","joint","joint_nll","timing_mae","card_top3","card_nll","cell_teacher","now_s","cell_teacher_top5","cell_teacher_near1","defense","winner","tile_nll")


def init_worker(_):
    torch.set_num_threads(1)
    torch.multiprocessing.set_sharing_strategy("file_descriptor")


def loader(ds,batch_size=512,workers=2,seed=None):
    # v9 shares short-lived batches, not a huge tensor LRU. Descriptor sharing
    # also avoids orphan shm managers retaining forkserver shutdown pipes.
    torch.multiprocessing.set_sharing_strategy("file_descriptor")
    return DataLoader(ds,batch_size=batch_size,num_workers=workers,collate_fn=identity,worker_init_fn=init_worker,pin_memory=torch.cuda.is_available(),persistent_workers=False,sampler=ShardSampler(ds,seed) if seed is not None else None)


def move(batch,device):
    return tuple(x.to(device,non_blocking=True) for x in batch)


class LegacyPolicy(nn.Module):
    def __init__(self,path,device):
        super().__init__()
        self.model,self.vocab,self.config,_=load_policy(path,device_name=str(device))
        self.clock_head=None
    def forward(self,batch,teacher=False):
        c,ids,td,od,g,sf,hm,slots,_,_,_,_,lengths,_=batch
        return self.model(c,ids,td,od,g,lengths,sf,hm,placement_slots=slots if teacher else None,think_steps=self.config.get("eval_think_steps",0))


def remap_vocab(batch,source,target):
    if source.compatible_with(target):
        return batch
    device=batch[1].device
    mapping=torch.tensor([target.pad_id,target.unk_id]+[target.encode(source.id_to_name[i]) for i in range(2,source.vocab_size)],device=device)
    result=list(batch)
    for i in (1,2,3):
        result[i]=mapping[batch[i]]
    return tuple(result)


@torch.inference_mode()
def score(model,ds,device,batch_size=512,workers=2,source_vocab=None,deadline=None):
    model.eval()
    results=[]
    winners=ds.metadata("winner")
    offset=0
    for batch in loader(ds,batch_size,workers):
        if deadline and time.monotonic()>=deadline:
            raise TimeoutError("Evaluation deadline")
        batch=move(batch,device)
        if source_vocab is not None and isinstance(model,LegacyPolicy):
            batch=remap_vocab(batch,source_vocab,model.vocab)
        teacher=model(batch,teacher=True)
        greedy=model(batch)
        slots,xy=batch[7],batch[10]
        x=(xy[:,0]*TILE_COLS).long().clamp(0,TILE_COLS-1)
        y=(xy[:,1]*TILE_ROWS).long().clamp(0,TILE_ROWS-1)
        tile=y*TILE_COLS+x
        card=greedy["slot_logits"].argmax(-1)==slots
        cell=greedy["tile_logits"].argmax(-1)==tile
        pred=teacher["tile_logits"].argmax(-1)
        # The heatmap is 32 columns by 18 rows, whereas the arena is 18 by
        # 32 tiles. One grid cell is not one physical tile in either axis.
        predicted_x=(pred%TILE_COLS+.5)/TILE_COLS
        predicted_y=(pred//TILE_COLS+.5)/TILE_ROWS
        near=((predicted_x-xy[:,0]).abs()*18<=1)&((predicted_y-xy[:,1]).abs()*32<=1)
        top5=(teacher["tile_logits"].topk(5).indices==tile[:,None]).any(-1)
        card_nll=nn.functional.cross_entropy(teacher["slot_logits"],slots,reduction="none")
        placement_nll=nn.functional.cross_entropy(teacher["tile_logits"],tile,reduction="none")
        top3=(greedy["slot_logits"].topk(3).indices==slots[:,None]).any(-1)
        defense=(batch[4][:,GLOBAL_DIM+THREAT_INDEX["any_focus"]]>.5)|(batch[4][:,GLOBAL_DIM+THREAT_INDEX["any_wincon"]]>.5)
        n=len(slots)
        row=torch.stack([card.float(),cell.float(),(card&cell).float(),card_nll+placement_nll,(greedy["timing"].clamp(max=6).expm1()-batch[11].expm1()).abs(),top3.float(),card_nll,(pred==tile).float(),batch[4][:,G["now"]]*300,top5.float(),near.float(),defense.float(),torch.tensor(winners[offset:offset+n].copy(),device=device).float(),placement_nll],1)
        results.append(row.cpu().numpy());offset+=n
    if not results:
        raise ValueError("Empty evaluation population")
    rows=np.concatenate(results)
    metrics=dict(zip(COLUMNS,map(float,rows.mean(0))))
    metrics.pop("defense");metrics.pop("winner")
    masks={"opening":rows[:,8]<60,"mid":(rows[:,8]>=60)&(rows[:,8]<180),"late":rows[:,8]>=180,"defense":rows[:,11]>.5,"winners":rows[:,12]>.5,"losers":rows[:,12]<.5}
    for name,mask in masks.items():
        metrics[name+"_n"]=int(mask.sum())
        for key in ("card","joint","joint_nll","cell_teacher_top5","cell_teacher_near1","timing_mae"):
            metrics[name+"_"+key]=float(rows[mask,COLUMNS.index(key)].mean()) if mask.any() else None
    return metrics,rows


@torch.inference_mode()
def score_clock(model,ds,device,batch_size=512,workers=2,deadline=None):
    if model.clock_head is None:
        return {}
    model.eval();rows=[]
    for batch,side,observed in loader(ds,batch_size,workers):
        if deadline and time.monotonic()>=deadline:raise TimeoutError("Clock evaluation deadline")
        batch=move(batch,device);side=side.to(device);observed=observed.to(device)
        rates=model(batch)["clock_rates"]
        delay=batch[11].expm1()
        valid=observed&(side>=0)
        probability=rates[:,0]/rates.sum(-1)
        rows.append(torch.stack([competing_risk_nll(rates,delay,side,observed),((rates.argmax(-1)==side)&valid).float(),valid.float(),(probability-(side==0).float()).square()*valid,(.693147/rates.sum(-1)-delay).abs()*observed,observed.float(),rates[:,0],rates[:,1]],1).cpu().numpy())
    a=np.concatenate(rows)
    return dict(clock_nll=float(a[:,0].mean()),actor_accuracy=float(a[:,1].sum()/max(1,a[:,2].sum())),actor_brier=float(a[:,3].sum()/max(1,a[:,2].sum())),clock_median_mae=float(a[:,4].sum()/max(1,a[:,5].sum())),mean_own_rate=float(a[:,6].mean()),mean_opponent_rate=float(a[:,7].mean()))


def append_progress(path,row):
    with Path(path).open("a") as f:f.write(json.dumps(row,allow_nan=False)+"\n")


def save_checkpoint(path,payload):
    tmp=Path(path).with_suffix(".tmp")
    torch.save(payload,tmp);tmp.replace(path)


def train(args):
    if args.clock_every<1 or args.sigma<0 or args.winner_weight<=0 or args.reaction_weight<=0:
        raise ValueError("Invalid loss/sampling settings")
    if args.gate_epoch and (args.gate_max_joint_nll is None or args.gate_min_card is None):
        raise ValueError("A gate epoch needs both a joint-NLL ceiling and a card floor")
    cache=Path(args.cache);out=Path(args.output);out.mkdir(parents=True,exist_ok=True)
    contract=json.loads((cache/"contract.json").read_text())
    vocab=CardVocab.from_dict(json.loads((cache/"vocab.json").read_text()))
    torch.set_num_threads(args.threads)
    torch.manual_seed(args.seed);np.random.seed(args.seed)
    torch.backends.cuda.matmul.allow_tf32=True
    torch.backends.cudnn.allow_tf32=True
    device=torch.device(args.device or ("cuda" if torch.cuda.is_available() else "cpu"))
    from .winner_dataset import load_card_costs
    stats=load_motion_stats(vocab) if args.ghost != "none" else None
    model=V9Policy(vocab.vocab_size,args.d_model,args.clock,args.ghost,stats,trunk=args.trunk,trunk_layers=args.trunk_layers,trunk_heads=args.trunk_heads).to(device)
    optimizer=torch.optim.AdamW(model.parameters(),lr=args.lr,weight_decay=.01)
    train_ds=ShardDataset(cache/"train",limit_battles=args.limit_battles,winner_weight=args.winner_weight,reaction_weight=args.reaction_weight,mirror=args.mirror,advantage=args.advantage)
    dev_ds=ShardDataset(cache/"validation",reaction_weight=1.)
    clocks=ShardDataset(cache/"train",kind="clocks",limit_battles=args.limit_battles) if args.clock else None
    dev_clock=ShardDataset(cache/"validation",kind="clocks") if args.clock else None
    donor=None
    if args.warmstart:
        donor=torch.load(Path(args.warmstart)/"best.pt",map_location=device,weights_only=False)
        if donor["contract"]["data"] != contract or donor["config"]["d_model"] != args.d_model or donor["config"]["vocab_size"] != vocab.vocab_size:
            raise ValueError("Warm-start data, width or vocabulary differs")
        if donor["config"]["ghost"] != "none" and donor["config"]["ghost"] != args.ghost:
            raise ValueError("Warm-start ghost architecture differs")
        missing,unexpected=model.load_state_dict(donor["model_state"],strict=False)
        if unexpected or any(not key.startswith("ghost_") for key in missing):raise ValueError("Unexpected warm-start key mismatch")
    if args.advantage:
        for shard in train_ds.shards:
            path=Path(shard["g"].filename).parent/"advantage.npy"
            if not path.exists():raise FileNotFoundError("Advantage weights not prepared")
    advantage_digest=sha256_json({str(Path(s["g"].filename).parent.relative_to(cache.resolve())):sha256_file(Path(s["g"].filename).parent/"advantage.npy") for s in train_ds.shards}) if args.advantage else None
    motion_digest=sha256_json({p.name:sha256_file(p) for p in Path("data/v9_card_stats").glob("*.json")}) if args.ghost != "none" else None
    config=dict(vocab_size=vocab.vocab_size,d_model=args.d_model,clock=args.clock,ghost=args.ghost,limit_battles=args.limit_battles,seed=args.seed,batch_size=args.batch_size,lr=args.lr,sigma=args.sigma,winner_weight=args.winner_weight,reaction_weight=args.reaction_weight,mirror=args.mirror,clock_weight=.1,clock_every=args.clock_every,advantage=args.advantage,warmstart_sha256=sha256_file(Path(args.warmstart)/"best.pt") if args.warmstart else None,donor_weights_epoch=donor["weights_epoch"] if donor else None,observation=contract["observation"],max_context=64,padded_encoder=True,training_version="v9-1")
    if args.trunk != "gru":
        config.update(trunk=args.trunk,trunk_layers=args.trunk_layers,trunk_heads=args.trunk_heads,time_bins=64,training_version="banano-0")
    config.update(advantage_sha256=advantage_digest,motion_stats_sha256=motion_digest)
    source_names=("v9_train.py","v9_policy.py","v9_cache.py","policy_model.py","controlled_policy.py","causal_observation.py","policy_dataset.py","event_clock.py","feature_schema.py")
    full_contract=dict(data=contract,training=sha256_json(config),source={p:sha256_file(Path(__file__).with_name(p)) for p in source_names})
    if not (out/"source.zip").exists():
        with zipfile.ZipFile(out/"source.zip","w",zipfile.ZIP_DEFLATED) as archive:
            for name in source_names:archive.write(Path(__file__).with_name(name),name)
        atomic_json(out/"environment.json",dict(python=sys.version,platform=platform.platform(),torch=torch.__version__,numpy=np.__version__,cuda=torch.version.cuda))
    started=time.monotonic();history=[];done=0;best=math.inf;best_epoch=None;start_epoch=1
    report_path=out/"report.json"
    old=None
    if (out/"last.pt").exists():
        if not args.resume:raise ValueError("Existing run: pass --resume for identical configuration")
        artifact=torch.load(out/"last.pt",map_location=device,weights_only=False)
        if artifact["contract"] != full_contract:raise ValueError("Resume contract changed")
        model.load_state_dict(artifact["model_state"]);optimizer.load_state_dict(artifact["optimizer_state"])
        old=json.loads(report_path.read_text());history=old["history"];done=artifact.get("samples_presented",artifact["weights_epoch"]*len(train_ds))
        best=min(r["joint_nll"] for r in history);best_epoch=old["weights_epoch"];start_epoch=artifact["weights_epoch"]+1
        torch.set_rng_state(artifact["rng_state"].cpu())
        if device.type=="cuda":torch.cuda.set_rng_state_all([r.cpu() for r in artifact["cuda_rng_state"]])
    elif report_path.exists():raise ValueError("Run report exists without resumable checkpoint")
    elapsed_prior=old.get("elapsed_seconds",0) if old else 0
    initial_done=done
    report=dict(model_name=args.model_name or f"Policy v9 · {out.name}",version="9.0" if args.trunk=="gru" else "banano-0",created_at=old["created_at"] if old else datetime.now(timezone.utc).isoformat(),config=config,contract=full_contract,parameters=sum(p.numel() for p in model.parameters()),compute=dict(device=str(device),gpu=torch.cuda.get_device_name() if device.type=="cuda" else None,vram_bytes=torch.cuda.get_device_properties(device).total_memory if device.type=="cuda" else None,precision="fp32 with TF32",cpu_threads=args.threads,loader_workers=args.workers),train_battles=args.limit_battles or json.loads((cache/"train"/"complete.json").read_text())["battles"],train_actions=len(train_ds),validation_actions=len(dev_ds),history=history,metrics_definition="joint_nll = card NLL + true-card-conditioned tile NLL; timing MAE is separate. Clock NLL is a probabilistic time/actor score.",limitations=["Offline imitation metrics do not establish playing strength.","Ghost positions omit combat, deaths and targeting changes; movement and survival assumptions are tested with a shuffled control.",contract["reference_ancestry_caveat"]],status="running")
    if args.trunk == "transformer":
        report["lessons"]=["banano-0 swaps only the v9 GRU for a 4-layer causal transformer. Heads, loss, cache, seed 42, and the development cohort stay the same.","Position and event time are learned embeddings. There is no v8 memory bank and no extra clock.","Epoch 2 must beat the logged width-256 GRU by at least 0.01 joint NLL, with card accuracy no more than 0.5 points worse. Otherwise the run stops.","Epoch 4 is worth keeping only with about +1 card point or +1 joint point over the width-160 GRU. The 1,000-battle holdout is not scored."]
        report["gate_message"]="banano-0 epoch-2 gate against the logged width-256 GRU: joint NLL ≤ 4.3875 and card ≥ 52.74%. Epoch-4 keep rule against the width-160 GRU: card ≥ 54.52% or joint ≥ 12.2%. The untouched holdout is not used."
    atomic_json(out/"config.json",config);atomic_json(out/"contract.json",full_contract)
    total=args.epochs*len(train_ds)
    deadline=started+args.run_seconds if args.run_seconds else float("inf")
    if args.deadline:
        end=datetime.fromisoformat(args.deadline).timestamp()
        deadline=min(deadline,time.monotonic()+end-time.time())
    global_progress=Path(args.progress) if args.progress else None
    bad=0;status="completed"
    try:
        for epoch in range(start_epoch,args.epochs+1):
            model.train();losses=[];epoch_started=time.monotonic()
            tl=loader(train_ds,args.batch_size,args.workers,args.seed+epoch)
            cl=loader(clocks,args.batch_size,args.workers,args.seed+epoch+10000) if clocks else None
            ci=iter(cl) if cl else None
            for bi,batch in enumerate(tl):
                if time.monotonic()>=deadline:raise TimeoutError("Run budget reached")
                batch=move(batch,device);optimizer.zero_grad(set_to_none=True)
                output=model(batch,teacher=True)
                per=nn.functional.cross_entropy(output["slot_logits"],batch[7],reduction="none")+tile_loss(output["tile_logits"],batch[10],args.sigma)+.25*nn.functional.smooth_l1_loss(output["timing"],batch[11],reduction="none")
                weights=batch[13];loss=(per*weights).sum()/weights.sum().clamp_min(1e-6)
                if ci is not None and (bi+epoch)%args.clock_every==0:
                    try:cb,side,observed=next(ci)
                    except StopIteration:ci=iter(cl);cb,side,observed=next(ci)
                    cb=move(cb,device);rates=model(cb)["clock_rates"]
                    loss=loss+.1*args.clock_every*competing_risk_nll(rates,cb[11].expm1(),side.to(device),observed.to(device)).mean()
                if not torch.isfinite(loss):raise FloatingPointError("Non-finite training loss")
                loss.backward();nn.utils.clip_grad_norm_(model.parameters(),1.);optimizer.step()
                losses.append(float(loss.detach()));done+=len(batch[7])
                if bi%25==0:
                    elapsed=time.monotonic()-started+elapsed_prior
                    rate=(done-initial_done)/max(time.monotonic()-started,1e-6)
                    eta=min((total-done)/max(rate,1),max(0,deadline-time.monotonic()))
                    row=dict(timestamp=datetime.now(timezone.utc).isoformat(),model_name=report["model_name"],phase="train",epoch=epoch,epochs_total=args.epochs,batch=bi+1,batches_total=len(tl),work_done=done,work_total=total,samples_done=done,samples_total=total,elapsed_seconds=elapsed,eta_seconds=eta,progress_percent=100*done/total,loss=losses[-1],samples_per_second=rate,device=str(device),batch_size=args.batch_size,parameters=report["parameters"])
                    append_progress(out/"progress.jsonl",row)
                    if global_progress:append_progress(global_progress,row)
                    print(f"{out.name:30} epoch={epoch:2} work={done:10}/{total:<10} ETA={eta:7.0f}s rate={rate:7.0f}/s loss={losses[-1]:.4f}",flush=True)
            if ci is not None and hasattr(ci,"_shutdown_workers"):
                ci._shutdown_workers()
            ci=None
            metrics,rows=score(model,dev_ds,device,args.batch_size,args.workers,deadline=deadline)
            if dev_clock:metrics.update(score_clock(model,dev_clock,device,args.batch_size,args.workers,deadline=deadline))
            if not all(v is None or math.isfinite(v) for v in metrics.values()):raise FloatingPointError("Invalid evaluation metrics")
            history.append(dict(epoch=epoch,train_loss=float(np.mean(losses)),epoch_seconds=time.monotonic()-epoch_started,**metrics))
            payload=dict(model_state=model.state_dict(),optimizer_state=optimizer.state_dict(),vocab=vocab.to_dict(),config=config,contract=full_contract,weights_epoch=epoch,samples_presented=done,rng_state=torch.get_rng_state(),cuda_rng_state=torch.cuda.get_rng_state_all() if device.type=="cuda" else [])
            save_checkpoint(out/"last.pt",payload)
            np.savez_compressed(out/f"validation_e{epoch}.npz",rows=rows,groups=dev_ds.metadata("group"),events=dev_ds.metadata("event"),columns=np.array(COLUMNS))
            if metrics["joint_nll"]<best-1e-4:
                best=metrics["joint_nll"];best_epoch=epoch;bad=0
                save_checkpoint(out/"best.pt",payload)
                np.savez_compressed(out/"validation_rows.npz",rows=rows,groups=dev_ds.metadata("group"),events=dev_ds.metadata("event"),columns=np.array(COLUMNS))
            else:bad+=1
            report.update(weights_epoch=best_epoch,last_completed_epoch=epoch,samples_presented=done,elapsed_seconds=time.monotonic()-started+elapsed_prior,selected_validation=next(r for r in history if r["epoch"]==best_epoch))
            from .v9_report import write_run_report
            write_run_report(out,report)
            print(f"VALIDATION {out.name} epoch={epoch} card={metrics['card']:.4f} joint={metrics['joint']:.4f} NLL={metrics['joint_nll']:.4f} near1={metrics['cell_teacher_near1']:.4f}",flush=True)
            if args.gate_epoch and epoch==args.gate_epoch and (metrics["joint_nll"]>args.gate_max_joint_nll or metrics["card"]<args.gate_min_card):
                status="gate_stopped";break
            if bad>=args.patience:status="early_stopped";break
    except TimeoutError:status="budget_stopped"
    except BaseException:
        status="failed";(out/"crash.txt").write_text(traceback.format_exc());raise
    finally:
        report.update(status=status,weights_epoch=best_epoch,last_completed_epoch=history[-1]["epoch"] if history else 0,samples_presented=done,elapsed_seconds=time.monotonic()-started+elapsed_prior)
        from .v9_report import write_run_report
        write_run_report(out,report)
    return report


def parser():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("--cache",default="data/v9_cache");p.add_argument("--output",required=True)
    p.add_argument("--limit-battles",type=int);p.add_argument("--epochs",type=int,default=8);p.add_argument("--patience",type=int,default=3)
    p.add_argument("--batch-size",type=int,default=512);p.add_argument("--workers",type=int,default=2);p.add_argument("--threads",type=int,default=2)
    p.add_argument("--d-model",type=int,default=160);p.add_argument("--seed",type=int,default=42);p.add_argument("--lr",type=float,default=2e-4)
    p.add_argument("--trunk",choices=["gru","transformer"],default="gru");p.add_argument("--trunk-layers",type=int,default=4);p.add_argument("--trunk-heads",type=int,default=4)
    p.add_argument("--model-name");p.add_argument("--gate-epoch",type=int,default=0);p.add_argument("--gate-max-joint-nll",type=float);p.add_argument("--gate-min-card",type=float)
    p.add_argument("--clock",action=argparse.BooleanOptionalAction,default=True);p.add_argument("--mirror",action=argparse.BooleanOptionalAction,default=True)
    p.add_argument("--ghost",choices=["none","aligned","shuffled"],default="none");p.add_argument("--sigma",type=float,default=0.)
    p.add_argument("--winner-weight",type=float,default=1.);p.add_argument("--reaction-weight",type=float,default=3.)
    p.add_argument("--clock-every",type=int,default=4);p.add_argument("--warmstart");p.add_argument("--advantage",action="store_true")
    p.add_argument("--run-seconds",type=float,default=0.);p.add_argument("--deadline");p.add_argument("--progress");p.add_argument("--device");p.add_argument("--resume",action="store_true")
    return p

def main():train(parser().parse_args())
if __name__=="__main__":main()
