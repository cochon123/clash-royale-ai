#!/usr/bin/env python3
"""Record/replay source-engine league games; failures stay in the denominator.

Development traces are for coverage discovery. Held-out seeds are generated
only after freezing a candidate, and never used to regenerate that candidate.
"""
from __future__ import annotations
import argparse
import collections
import concurrent.futures as futures
import gzip
import hashlib
import json
import os
from pathlib import Path
import random
import selectors
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
TIMING = {"reset_seconds", "step_seconds", "query_seconds"}
SPELLS = {"Fireball", "Log", "Rocket", "Poison", "Zap", "Tornado", "Lightning", "Arrows", "BarbLog"}
BUILDINGS = {"Cannon", "InfernoTower", "Tombstone", "Mortar"}

def sha(path):
    h=hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda:f.read(1024*1024),b""):h.update(block)
    return h.hexdigest()

def semantic(reply):return {k:v for k,v in reply.items() if k not in TIMING}

class Engine:
    def __init__(self,binary,log):
        self.err=Path(log).open("ab")
        self.proc=subprocess.Popen([str(Path(binary).resolve()),"--checkpoint",str(CHECKPOINT)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=self.err,bufsize=0,cwd=ROOT)
        self.selector=selectors.DefaultSelector();self.selector.register(self.proc.stdout,selectors.EVENT_READ);self.pending=b""
    def request(self,obj):
        self.proc.stdin.write(json.dumps(obj,separators=(",",":")).encode()+b"\n")
        end=time.monotonic()+120
        while b"\n" not in self.pending:
            remaining=end-time.monotonic()
            if remaining<=0 or not self.selector.select(remaining):raise TimeoutError("engine request timeout")
            data=os.read(self.proc.stdout.fileno(),65536)
            if not data:raise RuntimeError(f"engine exited: {self.proc.poll()}")
            self.pending+=data
        line,self.pending=self.pending.split(b"\n",1);reply=json.loads(line)
        if reply.get("ok") is not True:raise RuntimeError(str(reply.get("error",reply)))
        return reply["result"]
    def close(self):
        if self.proc.poll() is None:
            self.proc.terminate()
            try:self.proc.wait(timeout=5)
            except subprocess.TimeoutExpired:self.proc.kill();self.proc.wait(timeout=5)
        self.selector.close();self.proc.stdin.close();self.proc.stdout.close();self.err.close()

def choose(obs,side,rng,seen,style):
    hand=[c for c in obs["hands"][side] if c["available"]]
    if style=="cycle" and sum(s==side for s,name in seen)==8:
        seen.difference_update({key for key in seen if key[0]==side})
    unseen=[c for c in hand if (side,c["name"]) not in seen]
    target=max(unseen,key=lambda c:c["cost"]) if unseen and style in ("coverage","cycle") else None
    legal=([target] if target and target["cost"]<=obs["elixir"][side] else [] if target else [c for c in hand if c["cost"]<=obs["elixir"][side]])
    if not legal or rng.random()<.12:return None
    card=rng.choice(legal);name=card["name"]
    if name in SPELLS:
        x=rng.choice((3500,14500)); y=rng.choice((24500,25500,26500)) if side==0 else rng.choice((5500,6500,7500))
    elif name in BUILDINGS:
        x=rng.choice((6500,9000,11500));y=rng.choice((9000,11000,13000)) if side==0 else rng.choice((19000,21000,23000))
    elif style=="grid":
        x=round((rng.randrange(32)+.5)*18000/32);y=round((rng.randrange(18)+.5)*32000/18)
    else:
        x=rng.choice((3500,4500,9000,13500,14500));y=rng.randint(3000,14500) if side==0 else rng.randint(17500,29000)
    return dict(side=side,slot=card["slot"],x=x,y=y),name

def record_game(job,reference,out):
    records=[];seen=set();accepted=collections.Counter();rng=random.Random(job["seed"]+7000000)
    def capture(req):
        reply=reference.request(req);records.append(dict(request=req,expected=semantic(reply)));return reply
    reset=capture(dict(op="reset",deck0=job["deck0"]["native_cards"],deck1=job["deck1"]["native_cards"],seed=job["seed"],max_ticks=7600,reward_state=True))
    reply=capture(dict(op="step",ticks=100,actions=[]))
    # Validate the legal-placement protocol as well as combat/outcome/HP.
    capture(dict(op="placement_mask",side=job["id"]%2,slot=job["id"]%4))
    while not reply["terminated"] and not reply["truncated"]:
        obs=reply["observation"];choices=[x for side in (0,1) if (x:=choose(obs,side,rng,seen,job["style"]))]
        reply=capture(dict(op="step",ticks=job["ticks_per_step"],actions=[x for x,name in choices]))
        for (action,name),result in zip(choices,reply["actions"]):
            if result["accepted"]:seen.add((action["side"],name));accepted[name]+=1
    trace=dict(format="native-league-trace-v1",job=job,records=records,reference_completed=reply["terminated"],reference_truncated=reply["truncated"],
               winner=reply["observation"]["winner_side"],terminal_tick=reply["observation"]["tick"],tower_hp=reply["reward_state"],accepted_cards=dict(accepted))
    path=out/f"{job['id']:04d}.json.gz";tmp=path.with_suffix(".tmp")
    with gzip.open(tmp,"wt") as f:json.dump(trace,f,separators=(",",":"))
    tmp.replace(path)
    return trace

def replay_game(trace,candidate):
    started=time.monotonic();compared=0;last=None
    for index,item in enumerate(trace["records"]):
        try:actual=candidate.request(item["request"])
        except Exception as error:return dict(matched=False,compared=compared,difference=dict(index=index,request=item["request"],error=str(error)),seconds=time.monotonic()-started)
        if semantic(actual)!=item["expected"]:
            return dict(matched=False,compared=compared,difference=dict(index=index,request=item["request"],expected=item["expected"],actual=semantic(actual)),seconds=time.monotonic()-started)
        compared+=1;last=actual
    return dict(matched=bool(trace["reference_completed"]),compared=compared,difference=None,seconds=time.monotonic()-started,
                winner=last["observation"]["winner_side"],terminal_tick=last["observation"]["tick"],tower_hp=last["reward_state"])

def run_worker(jobs,args,worker):
    reference=Engine(args.reference,args.out/f"reference-{worker}.log") if args.record else None
    candidate=Engine(args.candidate,args.out/f"candidate-{worker}.log") if args.candidate else None
    results=[]
    try:
        for job in jobs:
            path=args.out/f"{job['id']:04d}.json.gz";start=time.monotonic()
            trace=json.load(gzip.open(path,"rt")) if path.exists() else record_game(job,reference,args.out)
            result=dict(id=job["id"],deck0=job["deck0"]["id"],deck1=job["deck1"]["id"],seed=job["seed"],reference_completed=trace["reference_completed"],reference_requests=len(trace["records"]))
            if candidate:
                result.update(replay_game(trace,candidate))
                if not result["matched"]:
                    candidate.close();candidate=Engine(args.candidate,args.out/f"candidate-{worker}.log")
            result["total_seconds"]=time.monotonic()-start
            (args.out/f"{job['id']:04d}-result.json").write_text(json.dumps(result,indent=2)+"\n")
            results.append(result);print(json.dumps(result),flush=True)
    finally:
        if reference:reference.close()
        if candidate:candidate.close()
    return results

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("--record",action="store_true");p.add_argument("--reference",type=Path)
    p.add_argument("--candidate",type=Path);p.add_argument("--out",type=Path,required=True);p.add_argument("--workers",type=int,default=6)
    p.add_argument("--seeds-per-matchup",type=int,default=1);p.add_argument("--seed-base",type=int,default=710000);p.add_argument("--split",choices=("development","holdout"),default="development")
    p.add_argument("--policy",choices=("auto","cycle"),default="auto",help="cycle saves elixir and repeatedly plays all eight cards")
    p.add_argument("--limit",type=int,default=0);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    decks=json.loads((ROOT/"data/fast_league_decks.json").read_text())["decks"]
    if not args.record and not (args.out/"manifest.json").exists():p.error("replay requires a recorded manifest")
    if args.record and not args.reference:p.error("record requires --reference")
    if args.record and (args.out/"manifest.json").exists():p.error("record requires a fresh output directory; replay existing traces without --record")
    if args.record:
        jobs=[]
        for a in decks:
            for b in decks:
                for repeat in range(args.seeds_per_matchup):
                    jobs.append(dict(id=len(jobs),deck0=a,deck1=b,seed=args.seed_base+len(jobs)*17,
                                     style="cycle" if args.policy=="cycle" else "coverage" if args.split=="development" else ("random","grid","random","grid","random")[repeat%5],
                                     ticks_per_step=20 if args.split=="development" else (10,20,40,20,20)[repeat%5]))
        if args.limit:jobs=jobs[:args.limit]
        manifest=dict(format="native-league-validation-v1",split=args.split,jobs=jobs,reference=str(args.reference.resolve()),reference_sha256=sha(args.reference),
                      candidate_sha256_before_recording=sha(args.candidate) if args.candidate else None,
                      policy_source_sha256=sha(Path(__file__)),
                      deck_file_sha256=sha(ROOT/"data/fast_league_decks.json"),checkpoint_sha256=sha(CHECKPOINT/"checkpoint.json"))
        (args.out/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    else:
        manifest=json.loads((args.out/"manifest.json").read_text());jobs=manifest["jobs"];args.split=manifest["split"]
    candidate_hash=sha(args.candidate) if args.candidate else None;started=time.monotonic();results=[]
    with futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        tasks=[pool.submit(run_worker,jobs[i::args.workers],args,i) for i in range(args.workers)]
        for task in futures.as_completed(tasks):results.extend(task.result())
    summary=dict(split=args.split,candidate_sha256=candidate_hash,candidate_unchanged=not args.candidate or sha(args.candidate)==candidate_hash,
                 total_games=len(jobs),reference_completed=sum(r["reference_completed"] for r in results),
                 fully_matched=sum(r.get("matched",False) for r in results),requests_compared=sum(r.get("compared",0) for r in results),
                 elapsed_seconds=time.monotonic()-started,failures=[r for r in results if r.get("matched") is False])
    (args.out/"summary.json").write_text(json.dumps(summary,indent=2)+"\n");print(json.dumps(summary,indent=2),flush=True)
    return 0 if not args.candidate or summary["fully_matched"]==len(jobs) else 1

if __name__=="__main__":raise SystemExit(main())
