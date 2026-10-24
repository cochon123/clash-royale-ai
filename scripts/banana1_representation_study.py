#!/usr/bin/env python3
"""Offline banana-1 interpretability study. Never advances a game or trains the actor.

Use --source-root for the artifact-bearing checkout. Probe partitions are whole
battles from the actor's legacy validation manifest, not the reserved final test.
"""
from __future__ import annotations
import argparse, copy, hashlib, json, os, pickle, random, sys, time
from datetime import datetime, timezone
from pathlib import Path

os.environ.setdefault('OPENBLAS_NUM_THREADS', '4')
os.environ.setdefault('OMP_NUM_THREADS', '4')
import numpy as np
import torch
from sklearn.linear_model import Ridge
from sklearn.metrics import balanced_accuracy_score, roc_auc_score, r2_score, mean_absolute_error, accuracy_score
from sklearn.preprocessing import StandardScaler

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'src'))
from cr_replay_pipeline.policy_dataset import PolicyActionDataset, collate_policy_batch
from cr_replay_pipeline.policy_infer import load_policy
from cr_replay_pipeline.feature_schema import GLOBAL_INDEX as G, CONTINUOUS_INDEX as C
from cr_replay_pipeline.winner_dataset import load_card_costs, WIN_CONDITIONISH
from cr_replay_pipeline.artifact_cache import sha256_file
from cr_replay_pipeline.belief_policy import build_policy_model

CONCEPTS = [
    ('elapsed_seconds', 'Elapsed match time', 'regression', 300, 'seconds', 'Explicit clock; encoder saturates at 450 s.'),
    ('own_elixir', 'Own estimated elixir', 'regression', 10, 'elixir', 'Replay reconstruction, not authoritative game elixir.'),
    ('enemy_elixir', 'Opponent estimated elixir', 'regression', 10, 'elixir', 'Replay reconstruction, not authoritative game elixir.'),
    ('elixir_advantage', 'Estimated elixir advantage', 'regression', 10, 'elixir', 'Own estimate minus opponent estimate.'),
    ('enemy_left_fraction', 'Opponent historical left-lane fraction', 'regression', 1, 'fraction', 'Fraction of prior opponent card plays on the left; not live troops.'),
    ('own_spend_30', 'Own spend over last 30 s', 'regression', 20, 'elixir spent', 'Exact replay deployment-cost sum under the project cost table.'),
    ('enemy_spend_30', 'Opponent spend over last 30 s', 'regression', 20, 'elixir spent', 'Exact replay deployment-cost sum under the project cost table.'),
    ('enemy_revealed', 'Opponent distinct cards played', 'regression', 8, 'cards', 'Number of distinct opponent cards in the confirmed prefix.'),
    ('last_enemy_x', 'Last opponent deployment x', 'regression', 18, 'arena tiles', 'Most recent opponent deployment location; not current unit position.'),
    ('double_phase', 'Double-or-later clock phase', 'binary', 1, '', 'Observation clock >=120 s, matching this encoder, not a current rules claim.'),
    ('low_own_elixir', 'Own estimate below 3 elixir', 'binary', 1, '', 'Threshold on reconstructed own elixir.'),
    ('enemy_recent_wincon', 'Opponent win condition played within 5 s', 'binary', 1, '', 'Exact deployment-history tag, not evidence that the unit is alive.'),
    ('enemy_recent_hog', 'Opponent Hog Rider played within 5 s', 'binary', 1, '', 'Exact deployment-history tag, not a live Hog detector.'),
    ('enemy_last_left', 'Last opponent play was left', 'binary', 1, '', 'Last confirmed opponent x < arena midpoint.'),
    ('last_event_enemy', 'Most recent event was opponent', 'binary', 1, '', 'Perspective-normalized side of the final observed event.'),
]
LAYERS = ['gru_layer0', 'gru_layer1', 'global', 'deck', 'fused_before_think', 'fused_after_think']


class Progress:
    def __init__(self, out):
        self.out = out; self.start = time.monotonic()
    def log(self, stage, done, total, note=''):
        elapsed = time.monotonic()-self.start
        row = dict(at=datetime.now(timezone.utc).isoformat(),stage=stage,done=done,total=total,elapsed_seconds=round(elapsed,2),note=note)
        # ETA is per stage, based on measured completed units.
        now=time.monotonic()
        if not hasattr(self,'stage') or self.stage!=stage:
            self.stage=stage; self.stage_start=now
        eta=(now-self.stage_start)*(total-done)/done if done else None
        row['eta_seconds']=None if eta is None else round(eta,1)
        with (self.out/'progress.jsonl').open('a') as f:f.write(json.dumps(row)+'\n')
        print(f"{stage:<22} {done:>7}/{total:<7} ETA {eta:>7.1f}s   {note}" if eta is not None else f"{stage:<22} {done:>7}/{total:<7} ETA pending   {note}",flush=True)


def forward(model,b):
    return model(*b[:5],b[12],b[5],b[6],think_steps=3)


class Capture:
    def __init__(self, model):
        self.values={}; self.handles=[]
        def save(name):
            def hook(module,args,out):self.values[name]=out.detach().float().cpu().numpy()
            return hook
        def gru(module,args,out):
            h=out[1].detach().float().cpu().numpy()
            self.values['gru_layer0']=h[0];self.values['gru_layer1']=h[1]
        self.handles.append(model.encoder.register_forward_hook(gru))
        for name,mod in [('global',model.global_encoder),('deck',model.deck_encoder),('fused_before_think',model.fuse),('fused_after_think',model.think)]:
            self.handles.append(mod.register_forward_hook(save(name)))
    def close(self):
        for h in self.handles:h.remove()


def labels(batch):
    c,g,l=batch[0].numpy(),batch[4].numpy(),batch[12].numpy()
    last=c[np.arange(len(c)),l-1]
    ex=[]
    for seq,n in zip(c,l):
        enemy=seq[:n][seq[:n,C['side_flag']]<.5]
        ex.append(enemy[-1,C['x']] if len(enemy) else np.nan)
    ex=np.asarray(ex)
    return np.column_stack([
        g[:,G['now']],g[:,G['team_elixir']],g[:,G['opp_elixir']],g[:,G['elixir_diff']],
        g[:,G['opp_left']],g[:,G['team_30']],g[:,G['opp_30']],g[:,G['opp_unique']],ex,
        g[:,G['now']]>=.4,g[:,G['team_elixir']]<.3,g[:,48+10]>.5,g[:,48]>.5,
        np.where(np.isfinite(ex),ex<.5,np.nan),last[:,C['side_flag']]<.5,
    ]).astype(np.float32)


def metric(y,p,binary):
    mask=np.isfinite(y);y=y[mask];p=p[mask]
    if binary:
        if len(np.unique(y))<2:return dict(auc=None,balanced_accuracy=None,accuracy=float(accuracy_score(y,p>=.5)))
        pred=p>=.5
        return dict(auc=float(roc_auc_score(y,p)),balanced_accuracy=float(balanced_accuracy_score(y,pred)),accuracy=float(accuracy_score(y,pred)),prevalence=float(y.mean()),tp=int(((pred==1)&(y==1)).sum()),fp=int(((pred==1)&(y==0)).sum()),fn=int(((pred==0)&(y==1)).sum()),tn=int(((pred==0)&(y==0)).sum()))
    return dict(r2=float(r2_score(y,p)),mae=float(mean_absolute_error(y,p)))


def score(y,p,binary):
    mask=np.isfinite(y);y=y[mask];p=p[mask]
    if binary:
        pos=y>.5
        if not pos.any() or pos.all():return -1e9
        return float(.5*((p[pos]>=.5).mean()+(p[~pos]<.5).mean()))
    return float(1-((y-p)**2).sum()/max(((y-y.mean())**2).sum(),1e-20))


def weights(y,binary):
    if not binary:return None
    return np.where(y>.5,.5/max(y.mean(),1e-8),.5/max(1-y.mean(),1e-8))


def fit_probe(X,y,split,binary):
    tr=(split==0)&np.isfinite(y);va=(split==1)&np.isfinite(y)
    scaler=StandardScaler().fit(X[tr]);z=scaler.transform(X)
    best=None
    for alpha in [1.,10.,100.,1000.]:
        model=Ridge(alpha=alpha).fit(z[tr],y[tr],sample_weight=weights(y[tr],binary))
        p=model.predict(z);s=score(y[va],p[va],binary)
        if best is None or s>best[0]:best=(s,model,p,alpha)
    return best[2],dict(mean=scaler.mean_,scale=scaler.scale_,coef=best[1].coef_,intercept=float(best[1].intercept_),alpha=best[3])


def single_neurons(X,y,split,binary):
    tr=(split==0)&np.isfinite(y);va=(split==1)&np.isfinite(y)
    w=weights(y[tr],binary)
    if w is None:w=np.ones(tr.sum())
    mu=np.average(X[tr],axis=0,weights=w);ym=np.average(y[tr],weights=w)
    xc=X[tr]-mu;yc=y[tr]-ym
    beta=(xc*(w*yc)[:,None]).sum(0)/((xc*xc*w[:,None]).sum(0)+1.)
    pred=(X-mu)*beta+ym
    ss=np.array([score(y[va],pred[va,j],binary) for j in range(X.shape[1])])
    j=int(ss.argmax())
    # Training-only marginal ranking for population curves and ablation.
    corr=(xc*(w*yc)[:,None]).sum(0)/np.sqrt((xc*xc*w[:,None]).sum(0)*(w*yc*yc).sum()+1e-20)
    return pred[:,j],j,dict(mean=float(mu[j]),coef=float(beta[j]),intercept=float(ym),validation_score=float(ss[j])),np.argsort(-np.abs(corr)),corr


def bootstrap(y,p,groups,binary,seed=747,repeats=300):
    mask=np.isfinite(y);y=y[mask];p=p[mask];groups=groups[mask]
    unique=np.unique(groups);indices=[np.flatnonzero(groups==g) for g in unique]
    rng=np.random.default_rng(seed);values=[]
    for _ in range(repeats):
        ix=np.concatenate([indices[j] for j in rng.integers(len(unique),size=len(unique))])
        v=score(y[ix],p[ix],binary)
        if v>-1e8:values.append(v)
    return [float(v) for v in np.quantile(values,[.025,.975])]


def jsonable(x):
    if isinstance(x,np.ndarray):return x.tolist()
    if isinstance(x,np.generic):return x.item()
    if isinstance(x,Path):return str(x)
    raise TypeError(type(x))


def audit_recordings(source):
    result=[]
    for file in sorted((source/'engine/runs').glob('*/frames.jsonl')):
        n=resets=down=damaged=0;last=-1.;maxt=0.
        for line in file.open():
            row=json.loads(line);t=row.get('elapsed_ms',0)/1000
            resets+=t<last;last=t;maxt=max(maxt,t);n+=1
            towers=[e for e in row.get('entities',[]) if e.get('row') in ('KingTower','PrincessTower')]
            down+=bool(any(e.get('hp',1)<=0 for e in towers) or len(towers)<6)
            damaged+=any(0<e.get('hp',0)<e.get('max_hp',0) for e in towers)
        result.append(dict(run=file.parent.name,frames=n,clock_resets=resets,max_seconds=maxt,frames_with_missing_or_zero_tower=down,frames_with_damage=damaged,sha256=sha256_file(file)))
    queue=source/'data/board_oracle/annotation_queue.jsonl'
    rows=[json.loads(l) for l in queue.read_text().splitlines()]
    return dict(native_recordings=result,board_annotation_rows=len(rows),verified_board_rows=sum(bool(r.get('verified_by') and r.get('verified_at') and r.get('battle_id') is not None and r.get('event_index') is not None) for r in rows),hp_accuracy_status='Not estimable from audited, sufficiently independent aligned labels in this study. Missing tower entries are NOT automatically treated as destroyed.')


def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source-root',type=Path,required=True);ap.add_argument('--output',type=Path,default=ROOT/'reports/banana1_study');ap.add_argument('--battles',type=int,default=1500);ap.add_argument('--rows-per-battle',type=int,default=24);ap.add_argument('--seed',type=int,default=20261023);a=ap.parse_args()
    a.output.mkdir(exist_ok=True,parents=True);prog=Progress(a.output);torch.set_num_threads(4)
    torch.manual_seed(a.seed);np.random.seed(a.seed);random.seed(a.seed)
    source=a.source_root.resolve();device='cuda' if torch.cuda.is_available() else 'cpu'
    model,vocab,cfg,_=load_policy(source/'models/policy_bc_v4.4.2',device)
    banana=source/'engine/runs/banana-1/base.pt';artifact=torch.load(banana,map_location='cpu',weights_only=False)
    assert set(artifact['model_state'])==set(model.state_dict())
    assert all(torch.equal(v.cpu(),artifact['model_state'][k]) for k,v in model.state_dict().items())
    frozen_hash=sha256_file(banana)
    bank=pickle.load((source/'data/v9_cache/battles.pkl').open('rb'));by_id={b.battle_id:b for seq in bank.values() for b in seq};del bank
    manifest_path=source/'data/splits/policy_v4_4_2_winner_seed42.json';manifest=json.loads(manifest_path.read_text())
    ids=sorted(set(manifest['validation_ids'])&by_id.keys());random.Random(a.seed).shuffle(ids)
    from cr_replay_pipeline.v9_cache import card_event_fingerprint
    seen=set();kept=[];duplicates=0
    for ident in ids:
        signature=card_event_fingerprint(by_id[ident])
        if signature in seen:duplicates+=1;continue
        seen.add(signature);kept.append(ident)
    ids=kept[:a.battles]
    assert not set(ids)&set(manifest['train_ids']);assert not set(ids)&set(manifest['test_ids'])
    boundaries=[int(len(ids)*.6),int(len(ids)*.8)]
    split_by_group=np.array([0 if j<boundaries[0] else 1 if j<boundaries[1] else 2 for j in range(len(ids))])
    (a.output/'split_manifest.json').write_text(json.dumps(dict(seed=a.seed,source_manifest_sha256=sha256_file(manifest_path),probe_train_ids=ids[:boundaries[0]],probe_validation_ids=ids[boundaries[0]:boundaries[1]],probe_test_ids=ids[boundaries[1]:],ancestry_caveat='Legacy validation, not pristine actor holdout; warm-start ancestors may have seen these games. Entire battles separated for probes.'),indent=2))
    prog.log('prepare',0,len(ids),'Building prefix streams')
    ds=PolicyActionDataset([by_id[i] for i in ids],vocab,load_card_costs(source/'data/card_costs.json'),max_samples_per_battle=a.rows_per_battle,stride=1,seed=a.seed,threat_dim=14,stream_cache_size=8,winner_only=False)
    # Same-timestamp actions are simultaneous evidence; never include a sibling
    # action at the target time in its observation.
    eligible=[j for j,(bi,ei) in enumerate(ds.index) if ds.battles[bi].events[ei-1]['seconds']<ds.battles[bi].events[ei]['seconds']]
    ds.index=[ds.index[j] for j in eligible];ds._mirror_flags=[False]*len(eligible)
    del by_id
    capture=Capture(model);acts={k:[] for k in LAYERS};ys=[];globals_=[];groups=[];splits=[];predslots=[];targetslots=[];predtiles=[];targettiles=[]
    random_models=[];random_captures=[];random_values=[[],[]]
    for seed in [17,83]:
        torch.manual_seed(seed);rm=build_policy_model(vocab.vocab_size,cfg).to(device).eval();random_models.append(rm);random_captures.append(Capture(rm))
    raw_input=[]
    with torch.inference_mode():
        for start in range(0,len(ds),256):
            batch=collate_policy_batch([ds[j] for j in range(start,min(start+256,len(ds)))])
            ys.append(labels(batch));globals_.append(batch[4].numpy());
            group=np.array([ds.index[j][0] for j in range(start,min(start+256,len(ds)))]);groups.append(group);splits.append(split_by_group[group])
            b=tuple(t.to(device) for t in batch);out=forward(model,b)
            for k in LAYERS:acts[k].append(capture.values[k].copy())
            for ri,(rm,cap) in enumerate(zip(random_models,random_captures)):
                forward(rm,b);random_values[ri].append(cap.values['fused_after_think'].copy())
            predslots.extend(out['slot_logits'].argmax(-1).cpu().tolist());targetslots.extend(batch[7].tolist())
            predtiles.extend(out['tile_logits'].argmax(-1).cpu().tolist());xy=batch[10].numpy();targettiles.extend(((xy[:,1]*18).astype(int).clip(0,17)*32+(xy[:,0]*32).astype(int).clip(0,31)).tolist())
            if start%2048==0 or start+256>=len(ds):prog.log('extract activations',min(start+256,len(ds)),len(ds))
    capture.close()
    for cap in random_captures:cap.close()
    del random_models,random_captures
    acts={k:np.concatenate(v) for k,v in acts.items()};y=np.concatenate(ys);g=np.concatenate(globals_);group=np.concatenate(groups);split=np.concatenate(splits);test=split==2
    random_values=[np.concatenate(v) for v in random_values]
    np.savez_compressed(a.output/'activations.npz',**acts,random_seed17=random_values[0],random_seed83=random_values[1],targets=y,globals=g,group=group,split=split,predicted_slots=np.array(predslots),target_slots=np.array(targetslots))
    reps={**acts,'random_seed17':random_values[0],'random_seed83':random_values[1],'raw_globals':g,'time_only':g[:,[G['now']]]}
    results=[];predictions={};probe_params={};curves={};neuron_map={};ci={}
    total=len(CONCEPTS)*len(reps);done=0
    for ci_idx,(key,title,kind,scale,unit,definition) in enumerate(CONCEPTS):
        binary=kind=='binary';target=y[:,ci_idx];entry=dict(key=key,title=title,kind=kind,scale=scale,unit=unit,definition=definition,results={})
        for name,X in reps.items():
            p,params=fit_probe(X,target,split,binary);m=metric(target[test],p[test],binary)
            if not binary:m['mae_units']=m['mae']*scale
            entry['results'][name]=m;predictions[f'{key}__{name}']=p[test];probe_params[f'{key}__{name}']=params
            if name=='fused_after_think':
                m['score_ci95']=bootstrap(target[test],p[test],group[test],binary)
                # Sanity control: training labels permuted; true test labels stay untouched.
                shuffled=target.copy();tr=np.flatnonzero((split==0)&np.isfinite(target));shuffled[tr]=np.random.default_rng(a.seed+ci_idx).permutation(target[tr]);sp,_=fit_probe(X,shuffled,split,binary);entry['shuffled_train_control']=metric(target[test],sp[test],binary)
            done+=1
            if done%10==0:prog.log('linear probes',done,total,key)
        singles={};best_layer=None;best_valid=-1e9
        for name in LAYERS:
            p,j,params,ranking,corr=single_neurons(acts[name],target,split,binary)
            m=metric(target[test],p[test],binary)
            if not binary:m['mae_units']=m['mae']*scale
            singles[name]=dict(neuron=j,metrics=m,fit=params)
            neuron_map[f'{key}__{name}']=dict(correlation=corr,ranking=ranking)
            if params['validation_score']>best_valid:best_layer=name;best_valid=params['validation_score']
        entry['single_neurons']=singles;entry['best_single_layer_selected_on_validation']=best_layer
        _,_,_,rank,_=single_neurons(acts['fused_after_think'],target,split,binary)
        curve=[]
        for k in [1,2,4,8,16,32,64,160]:
            p,_=fit_probe(acts['fused_after_think'][:,rank[:k]],target,split,binary);curve.append(dict(neurons=k,**metric(target[test],p[test],binary)))
        curves[key]=curve
        # Paired test predictions for auditing and recomputing uncertainty.
        results.append(entry)
    prog.log('linear probes',total,total,'Finished probes, starting interventions')
    np.savez_compressed(a.output/'test_predictions.npz',targets=y[test],group=group[test],**predictions)
    (a.output/'probe_parameters.json').write_text(json.dumps(probe_params,default=jsonable))
    # Fixed test sample across battles. Hooks replace ONLY the final fused state.
    rng=np.random.default_rng(a.seed);test_indices=rng.choice(np.flatnonzero(test),size=min(2048,test.sum()),replace=False);test_indices.sort()
    fused=acts['fused_after_think'];mean=fused[split==0].mean(0)
    base_outputs=[];batches=[]
    with torch.inference_mode():
        for start in range(0,len(test_indices),256):
            ix=test_indices[start:start+256];b=tuple(t.to(device) for t in collate_policy_batch([ds[int(j)] for j in ix]));batches.append(b)
            out=forward(model,b);base_outputs.append({k:out[k].detach() for k in ['slot_logits','tile_logits','timing']})
    interventions=[]
    conditions=[]
    for key in ['own_elixir','enemy_elixir','enemy_recent_wincon','enemy_last_left','double_phase']:
        rank=neuron_map[f'{key}__fused_after_think']['ranking']
        for k in [1,4,16,32]:conditions.append((key,k,'target',rank[:k]))
    for k in [1,4,16,32]:
        for rep in range(8):conditions.append(('random',k,f'random_{rep}',rng.choice(160,k,replace=False)))
    with torch.inference_mode():
        for no,(key,k,control,indices) in enumerate(conditions):
            changed=[];js=[];tile_changed=[];timing_change=[];fused_changed=[]
            idx=torch.as_tensor(indices.copy(),device=device);mu=torch.tensor(mean,device=device)
            def intervention(module,args,out):
                h=out.clone();h[:,idx]=mu[idx];fused_changed.append(h.detach().cpu().numpy());return h
            handle=model.think.register_forward_hook(intervention)
            for b,base in zip(batches,base_outputs):
                out=forward(model,b);p=base['slot_logits'].softmax(-1);q=out['slot_logits'].softmax(-1);mid=(p+q)/2
                divergence=.5*(p*(p.clamp_min(1e-9).log()-mid.clamp_min(1e-9).log())).sum(-1)+.5*(q*(q.clamp_min(1e-9).log()-mid.clamp_min(1e-9).log())).sum(-1)
                changed.extend((p.argmax(-1)!=q.argmax(-1)).cpu().tolist());js.extend(divergence.cpu().tolist());tile_changed.extend((base['tile_logits'].argmax(-1)!=out['tile_logits'].argmax(-1)).cpu().tolist());timing_change.extend((base['timing'].expm1()-out['timing'].expm1()).abs().cpu().tolist())
            handle.remove();row=dict(concept=key,k=k,control=control,neurons=indices,card_change_rate=float(np.mean(changed)),card_js_nats=float(np.mean(js)),tile_change_rate=float(np.mean(tile_changed)),delay_absolute_change_seconds=float(np.mean(timing_change)))
            if key!='random':
                j=[c[0] for c in CONCEPTS].index(key);par=probe_params[f'{key}__fused_after_think'];z=(np.concatenate(fused_changed)-par['mean'])/par['scale'];p=z@par['coef']+par['intercept'];row['concept_after_ablation']=metric(y[test_indices,j],p,CONCEPTS[j][2]=='binary')
            interventions.append(row)
            if no%8==0:prog.log('neuron interventions',no+1,len(conditions),key)
    # A direct inference-level demonstration of board input blindness.
    sys.path.insert(0,str(ROOT/'engine'))
    from rl.banana_policy import BananaPolicy, BananaEpisode
    actor=BananaPolicy(model_dir=source/'models/policy_bc_v4.4.2',device='cpu',weights=banana)
    decks=(('hog-rider','musketeer','ice-golem','skeletons','ice-spirit','cannon','fireball','the-log'),('goblin-barrel','princess','knight','goblin-gang','inferno-tower','rocket','the-log','ice-spirit'))
    ep=BananaEpisode(decks)
    for i in range(16):
        side=i%2;ep.record_action(side,decks[side][(i//2)%8],3500 if i%3 else 14500,12000 if side==0 else 20000,3.+i*3.)
    hands=[[dict(slot=i,name=d[i],cost=actor.costs[d[i]],available=True) for i in range(4)] for d in decks]
    obs=dict(hands=hands,elixir=[8.,7.],towers=[dict(hp=3052,max_hp=3052) for _ in range(6)],entities=[])
    pairs=[]
    with torch.inference_mode():
        for side in [0,1]:
            baseline=actor.act(ep,obs,side,52.,deterministic=True)
            for label,hp in [('full',3052),('half',1526),('one_hp',1),('destroyed',0)]:
                altered=copy.deepcopy(obs);altered['towers'][0]['hp']=hp;altered['entities']=[dict(row='PrincessTower',hp=hp,max_hp=3052,side=0)]
                action=actor.act(ep,altered,side,52.,deterministic=True)
                equal=all(torch.equal(x,z) for x,z in zip(baseline.input_cache['inputs'],action.input_cache['inputs']))
                pairs.append(dict(side=side,tower_state=label,hp=hp,all_actor_input_tensors_identical=equal,same_action=(baseline.card,baseline.x,baseline.y,baseline.wait)==(action.card,action.x,action.y,action.wait)))
                assert equal
    ps=np.array(predslots);ts=np.array(targetslots);pt=np.array(predtiles);tt=np.array(targettiles)
    assert sha256_file(banana)==frozen_hash
    evidence = dict(
        study='banana-1 representation atlas v1',
        created_at=datetime.now(timezone.utc).isoformat(), seed=a.seed, source_root=source,
        checkpoint=dict(path=banana, sha256=frozen_hash, actor_equals_v442=True,
            source_sha256=sha256_file(source/'models/policy_bc_v4.4.2/best_model.pt'),
            parameters=sum(p.numel() for p in model.parameters()), config=cfg),
        compute=dict(device=device, gpu=torch.cuda.get_device_name(0) if device=='cuda' else None,
            threads=torch.get_num_threads(), torch=torch.__version__, numpy=np.__version__,
            elapsed_seconds=time.monotonic()-prog.start),
        population=dict(battles=len(ids), rows=len(ds),
            battle_splits=[int((split_by_group==i).sum()) for i in range(3)],
            row_splits=[int((split==i).sum()) for i in range(3)],
            max_rows_per_battle=a.rows_per_battle, context=64, legacy_partition='validation',
            winner_only=False, opponent_deck='Known full decks, matching native adapter.',
            duplicate_trajectories_removed=duplicates,
            independence='All perspectives and rows of each battle kept together; identical side-canonical card trajectories deduplicated.'),
        concepts=results, population_curves=curves, neuron_map=neuron_map,
        interventions=interventions, intervention_rows=len(test_indices), tower_twins=pairs,
        label_audit=audit_recordings(source),
        descriptive_policy_test=dict(card_accuracy=float((ps[test]==ts[test]).mean()),
            joint_card_tile_accuracy=float(((ps[test]==ts[test]) & (pt[test]==tt[test])).mean())),
        files={str(p.relative_to(ROOT)):sha256_file(p) for p in [Path(__file__),
            ROOT/'src/cr_replay_pipeline/policy_model.py', ROOT/'src/cr_replay_pipeline/policy_dataset.py',
            ROOT/'engine/rl/banana_policy.py']},
    )
    (a.output/'results.json').write_text(json.dumps(evidence,indent=2,default=jsonable,allow_nan=False))
    prog.log('complete',1,1,f"Evidence saved to {a.output}")


if __name__=='__main__':main()
