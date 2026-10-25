"""Banana-1 continuation: human imitation with experimental native-state targets.

No simulator or game is run here. The paired BC-only arm measures whether the
auxiliary targets help beyond simply continuing the parent on the same replays.
"""
from __future__ import annotations

import argparse
from contextlib import nullcontext
from datetime import datetime, timezone
import fcntl
import hashlib
import json
import math
import os
from pathlib import Path
import pickle
import random
import shutil
import signal
import socket
import time

import numpy as np
import torch
from torch import nn
from torch.nn import functional as F
from torch.utils.data import DataLoader

from .banana2_data import TARGET_NAMES, aux_batch, clean_partitions, prepare_aux
from .belief_policy import build_policy_model
from .enrichment import atomic_json, duration
from .policy_dataset import PolicyActionDataset, GroupedBattleSampler, collate_policy_batch
from .policy_train import evaluate_policy
from .winner_dataset import CardVocab, load_card_costs

ROOT = Path(__file__).resolve().parents[2]
LOSS = dict(slot_weight=1.4,type_weight=.25,zone_weight=.7,xy_weight=.35,timing_weight=.25,tile_weight=1.)


def now():
    return datetime.now(timezone.utc).isoformat()


def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda:f.read(1024*1024),b''): h.update(block)
    return h.hexdigest()


def save_torch(path,value):
    tmp=path.with_suffix('.tmp')
    torch.save(value,tmp)
    tmp.replace(path)


def forward(model,b):
    return model(b[0],b[1],b[2],b[3],b[4],b[12],b[5],b[6],think_steps=3)


def to_device(batch,device):
    return tuple(t.to(device,non_blocking=True) for t in batch)


class Monitor:
    def __init__(self,run,report):
        self.run,self.report=run,report
        self.start=time.monotonic(); self.started=now(); self.history=[]
        self.data={}; self.compute={}; self.last=0; self.stop=False
        self.report.parent.mkdir(parents=True,exist_ok=True)
        template=ROOT/'src/cr_replay_pipeline/assets/banana2_report.html'
        self.report.write_text(template.read_text())
    def publish(self,phase,done,total,eta=None,status='running',**extra):
        state={'kind':'training run','method':'imitation_native_aux','host':socket.gethostname(),
               'status':status,'started_at':self.started,'updated_at':now(),
               'elapsed_seconds':time.monotonic()-self.start,'phase':phase,
               'progress':{'done':done,'total':total,'eta_seconds':eta,**extra},
               'scope':'Paired human imitation continuation; unverified native states are low-weight auxiliary targets only. No RL.',
               'data':self.data,'compute':self.compute,'history':self.history,
               'report':'/reports/'+self.report.name,
               'validation':{'status':'experimental','training_usable':False,
                             'message':'Research checkpoint; native labels remain unverified. No live-game accuracy claim.'}}
        atomic_json(self.run/'state.json',state)
        atomic_json(self.report.with_name('banana2-live.json'),state)
        with (self.run/'progress.jsonl').open('a') as f:
            f.write(json.dumps({'at':now(),'model':'banana-2',**state['progress'],'phase':phase})+'\n')
        print(f'{done:>9,}/{total:<9,} ETA {duration(eta):>11}  {phase}',flush=True)
        self.last=time.monotonic()
        return state


def load_data(args,parent,monitor):
    cache=args.run_dir/'prepared.pt'
    parent_manifest=ROOT/'data/splits/policy_v4_4_2_winner_seed42.json'
    contract={'format':'banana2-data-v1','parent_sha256':sha(args.parent),
              'legacy_split_sha256':sha(parent_manifest),
              'battle_bank_sha256':sha(ROOT/'data/v9_cache/battles.pkl'),
              'enrichment_manifest_sha256':sha(args.enrichment/'run-manifest.json'),
              'enrichment_results_sha256':sha(args.enrichment/'results.sqlite3'),
              'data_code_sha256':sha(Path(__file__).with_name('banana2_data.py')),
              'seed':args.seed,'smoke_battles':args.smoke_battles}
    old=args.run_dir/'data-contract.json'
    if old.exists() and json.loads(old.read_text())!=contract:
        raise RuntimeError('Data contract changed; use a fresh run directory')
    atomic_json(old,contract)
    if cache.exists():
        return torch.load(cache,map_location='cpu',weights_only=False)
    monitor.publish('Loading and separating replay battles',0,1)
    bank=pickle.load((ROOT/'data/v9_cache/battles.pkl').open('rb'))
    by_id={b.battle_id:b for seq in bank.values() for b in seq};del bank
    manifest=json.loads(parent_manifest.read_text())
    train_ids,val_ids,removed=clean_partitions(by_id,manifest)
    if args.smoke_battles:
        train_ids=train_ids[:args.smoke_battles]
        val_ids=val_ids[:max(8,args.smoke_battles//4)]
    vocab=CardVocab.from_dict(parent['vocab']);costs=load_card_costs(ROOT/'data/card_costs.json')
    monitor.publish('Joining experimental native-state targets',0,1)
    clock=time.monotonic()
    def progress(done,total,counts):
        monitor.publish('Joining experimental native-state targets',done,total,
                        (total-done)*(time.monotonic()-clock)/max(1,done),**counts)
    aux,counts=prepare_aux(args.enrichment,by_id,{'train':train_ids,'validation':val_ids},vocab,costs,args.seed,progress)
    if not aux['train'] or not aux['validation']:
        raise ValueError('No eligible auxiliary training or validation rows')
    data={'train':[by_id[i] for i in train_ids],'validation':[by_id[i] for i in val_ids],
          'aux':aux,'counts':counts,'costs':costs,
          'duplicate_train_battles_removed':removed}
    atomic_json(args.run_dir/'split-manifest.json',{'seed':args.seed,'train_ids':train_ids,
                'validation_ids':val_ids,'reserved_test_ids':manifest['test_ids'],
                'source':str(parent_manifest),'duplicates_removed':removed,
                'caveat':'Parent and older ancestors used legacy development sets. This is a continuation comparison, not a pristine generalization test.'})
    with (args.run_dir/'auxiliary-manifest.jsonl').open('w') as f:
        for split,rows in aux.items():
            for ident,index,_,_,digest in rows:
                f.write(json.dumps({'split':split,'battle_id':ident,'event_index':index,'artifact_sha256':digest})+'\n')
    save_torch(cache,data)
    return data


@torch.no_grad()
def evaluate_aux(model,head,rows,mean,scale,device):
    model.eval();head.eval();squared=0.;absolute=0.;n=0
    for start in range(0,len(rows),256):
        b,y=aux_batch(rows,range(start,min(start+256,len(rows))))
        pred=head(forward(model,to_device(b,device))['fused'].float())
        y=y.to(device); decoded=pred*scale+mean
        absolute+=(decoded-y).abs().sum().item()
        squared+=((decoded-y)/scale).square().sum().item()
        n+=y.numel()
    return {'aux_mae':absolute/max(1,n),'aux_standardized_mse':squared/max(1,n)}


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir',type=Path,default=ROOT/'engine/runs/banana-2')
    p.add_argument('--parent',type=Path,default=ROOT/'engine/runs/banana-1/base.pt')
    p.add_argument('--enrichment',type=Path,default=ROOT/'engine/runs/replay-enrichment-full-20261024')
    p.add_argument('--epochs',type=int,default=3)
    p.add_argument('--batch-size',type=int,default=192)
    p.add_argument('--learning-rate',type=float,default=1e-5)
    p.add_argument('--aux-weight',type=float,default=.05)
    p.add_argument('--aux-every',type=int,default=4)
    p.add_argument('--seed',type=int,default=20261024)
    p.add_argument('--smoke-battles',type=int,default=0)
    p.add_argument('--prepare-only',action='store_true')
    p.add_argument('--continue-from',type=Path,
                   help='Prior run directory: reuse prepared data and warm-start both arms from latest checkpoints')
    p.add_argument('--teacher',type=Path,
                   help='Frozen KL teacher checkpoint (default: original --parent, or banana-1 on continuation)')
    args=p.parse_args()
    for k in ('run_dir','parent','enrichment'): setattr(args,k,getattr(args,k).resolve())
    if args.continue_from:
        args.continue_from=args.continue_from.resolve()
        if args.teacher is None:
            prior=json.loads((args.continue_from/'training-manifest.json').read_text())
            args.teacher=Path(prior['parent']).resolve()
    if args.teacher is not None:
        args.teacher=args.teacher.resolve()
    args.run_dir.mkdir(parents=True,exist_ok=True)
    if args.continue_from:
        for name in ('data-contract.json','split-manifest.json','auxiliary-manifest.jsonl'):
            src,dst=args.continue_from/name,args.run_dir/name
            if src.is_file() and not dst.exists():
                shutil.copy2(src,dst)
        prepared=args.run_dir/'prepared.pt'
        if not prepared.exists():
            shutil.copy2(args.continue_from/'prepared.pt',prepared)
    lock=(args.run_dir/'.train.lock').open('w');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    if (args.run_dir/'training-started.json').exists():
        raise RuntimeError('Run already started; checkpoints preserved. Use a new run directory for another run.')
    torch.set_num_threads(2)
    torch.manual_seed(args.seed);np.random.seed(args.seed);random.seed(args.seed)
    device=torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    report=ROOT/'reports'/('banana2-smoke.html' if args.smoke_battles else 'banana2.html')
    monitor=Monitor(args.run_dir,report)
    def stop(*_): monitor.stop=True
    signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
    monitor.compute={'device':str(device),'gpu':torch.cuda.get_device_name() if device.type=='cuda' else None,
                     'torch':torch.__version__,'cpu_threads':2,'batch_size':args.batch_size,
                     'epochs':args.epochs,'method':'behavior cloning, paired BC-only and native-aux arms'}
    teacher_ckpt=torch.load((args.teacher or args.parent),map_location='cpu',weights_only=False)
    parent=teacher_ckpt
    cfg=parent['config'].copy();vocab=CardVocab.from_dict(parent['vocab'])
    epoch_offset=0;continued_history=[];continued_best={}
    if args.continue_from:
        prior_metrics=json.loads((args.continue_from/'metrics.json').read_text())
        continued_history=list(prior_metrics['history'])
        epoch_offset=max(r['epoch'] for r in continued_history if r.get('epoch'))
        for name in ('bc-control','banana-2'):
            rows=[r for r in continued_history if r.get('model')==name and 'loss' in r]
            if rows: continued_best[name]=min(r['loss'] for r in rows)
    try:
        data=load_data(args,parent,monitor)
        monitor.data={**data['counts'],'train_battles':len(data['train']),
                      'validation_battles':len(data['validation']),
                      'source_state_rows':258150,'targets':TARGET_NAMES,
                      'source_validation':'unverified','original_data_unchanged':True}
        if args.prepare_only:
            monitor.publish('Data preparation complete',1,1,0,status='completed')
            return
        atomic_json(args.run_dir/'training-started.json',{'at':now(),'pid':os.getpid()})
        atomic_json(args.run_dir/'execution-owner.json',{'host':socket.gethostname(),'since':now()})
        train=PolicyActionDataset(data['train'],vocab,data['costs'],max_samples_per_battle=40,
                                  stride=1,seed=args.seed,threat_dim=14,winner_only=True,stream_cache_size=128)
        val=PolicyActionDataset(data['validation'],vocab,data['costs'],max_samples_per_battle=24,
                                stride=1,seed=args.seed,threat_dim=14,winner_only=True,stream_cache_size=128)
        # Entire battle groups stay adjacent: each causal stream is encoded once
        # with a bounded LRU rather than once for each random prefix.
        val_loader=DataLoader(val,batch_size=args.batch_size,collate_fn=collate_policy_batch,num_workers=0)
        monitor.data.update(train_action_rows=len(train),validation_action_rows=len(val))
        def actor():
            m=build_policy_model(vocab.vocab_size,cfg)
            m.load_state_dict(parent['model_state'],strict=True)
            m.padded_encoder=True
            return m.to(device)
        teacher=actor().eval()
        for param in teacher.parameters(): param.requires_grad_(False)
        models={name:actor() for name in ('bc-control','banana-2')}
        head=nn.Sequential(nn.LayerNorm(160),nn.Linear(160,64),nn.GELU(),nn.Linear(64,len(TARGET_NAMES))).to(device)
        targets=torch.stack([r[3] for r in data['aux']['train']]).to(device)
        mean=targets.mean(0);scale=targets.std(0).clamp_min(.05)
        del targets
        optimizers={name:torch.optim.AdamW(list(m.parameters())+(list(head.parameters()) if name=='banana-2' else []),
                     lr=args.learning_rate,weight_decay=1e-4) for name,m in models.items()}
        amp=device.type=='cuda' and torch.cuda.is_bf16_supported()
        autocast=lambda:torch.autocast('cuda',dtype=torch.bfloat16) if amp else nullcontext()
        manifest={'format':'banana2-experimental-imitation-v1','created_at':now(),
                  'parent':str(args.teacher or args.parent),'parent_sha256':sha(args.teacher or args.parent),
                  'trainer_sha256':sha(__file__),'data_code_sha256':sha(Path(__file__).with_name('banana2_data.py')),
                  'data_contract_sha256':sha(args.run_dir/'data-contract.json'),
                  'seed':args.seed,'epochs':args.epochs,'learning_rate':args.learning_rate,
                  'aux_weight':args.aux_weight,'aux_every':args.aux_every,'parent_kl_weight':.1,
                  'loss_weights':LOSS,'aux_target_normalization':'train mean/std, std floor 0.05',
                  'aux_inputs':'causal replay prefix + known query clock; no actual board inputs',
                  'experimental_authorization':'User requested Banana-2 using newly enriched data; flags on quarantined sources unchanged.',
                  'compute':monitor.compute,'data':monitor.data,
                  'selection':'Lowest development human action loss per arm; no automatic replacement of Banana-1.'}
        if args.continue_from:
            manifest.update(continued_from=str(args.continue_from),epoch_offset=epoch_offset,
                            epoch_range=f'{epoch_offset+1}-{epoch_offset+args.epochs}',
                            continuation_note='Warm-started from prior arm latest.pt; fresh AdamW optimizers; frozen Banana-1 KL teacher.')
        atomic_json(args.run_dir/'training-manifest.json',manifest)
        best={name:continued_best.get(name,math.inf) for name in models}
        steps_per_epoch=math.ceil(len(train)/args.batch_size)
        total_steps=steps_per_epoch*args.epochs
        step=0;train_clock=None
        if continued_history:
            monitor.history.extend(continued_history)
            baseline=next(r for r in continued_history if r.get('model')=='banana-1' and r.get('epoch')==0)
            monitor.publish(f'Continuing from epoch {epoch_offset}; {args.epochs} more epoch(s)',0,total_steps)
        else:
            monitor.publish('Evaluating frozen Banana-1 reference',0,total_steps)
            baseline=evaluate_policy(teacher,val_loader,device,loss_kwargs=LOSS,think_steps=3,amp_enabled=False)
            monitor.history.append({'at':now(),'epoch':0,'model':'banana-1',**baseline})
        for name,m in models.items():
            if args.continue_from:
                ckpt=torch.load(args.continue_from/name/'latest.pt',map_location='cpu',weights_only=False)
                m.load_state_dict(ckpt['model_state'],strict=True)
        if args.continue_from and (args.continue_from/'aux-head-latest.pt').is_file():
            head_ckpt=torch.load(args.continue_from/'aux-head-latest.pt',map_location='cpu',weights_only=False)
            head.load_state_dict(head_ckpt['state_dict'],strict=True)
            mean=head_ckpt['mean'].to(device);scale=head_ckpt['scale'].to(device)
        def checkpoint(name,epoch,metrics,path):
            actor_state={k:v.detach().cpu().clone() for k,v in models[name].state_dict().items()}
            configuration={**cfg,'version':'banana-2' if name=='banana-2' else 'banana-2-bc-control',
                           'batch_size':args.batch_size,'learning_rate':args.learning_rate,
                           'banana2_experimental_native_aux':name=='banana-2',
                           'banana2_training_manifest':str(args.run_dir/'training-manifest.json')}
            save_torch(path,{'format':'banana-native-ppo-v1','training_method':'behavior_cloning_native_aux' if name=='banana-2' else 'behavior_cloning',
                            'name':name,'model_state':actor_state,
                            'critic_state':parent['critic_state'],'wait_state':parent['wait_state'],'updates':0,
                            'vocab':parent['vocab'],'config':configuration,'epoch':epoch,
                            'metrics':metrics,'parent_sha256':manifest['parent_sha256'],
                            'experimental':True,'native_labels_verified':False})
        for epoch in range(epoch_offset+1,epoch_offset+args.epochs+1):
            sampler=GroupedBattleSampler(train,seed=args.seed+epoch)
            loader=DataLoader(train,batch_size=args.batch_size,sampler=sampler,
                              collate_fn=collate_policy_batch,num_workers=0)
            rng=np.random.default_rng(args.seed+epoch)
            aux_order=rng.permutation(len(data['aux']['train']));aux_cursor=0
            sums={name:0. for name in models};aux_sum=0.;aux_steps=0
            epoch_clock=time.monotonic()
            for bi,batch in enumerate(loader):
                if monitor.stop or (args.run_dir/'STOP').exists():
                    for name in models:
                        folder=args.run_dir/name;folder.mkdir(exist_ok=True)
                        checkpoint(name,epoch,{},folder/'interrupted.pt')
                    monitor.publish('Stopped with research checkpoints saved',step,total_steps,0,status='stopped')
                    return
                if train_clock is None: train_clock=time.monotonic()
                b=to_device(batch,device)
                teacher.eval()
                with torch.no_grad(),autocast(): parent_out=forward(teacher,b)
                cpu_rng=torch.get_rng_state();gpu_rng=torch.cuda.get_rng_state() if device.type=='cuda' else None
                for name,m in models.items():
                    torch.set_rng_state(cpu_rng)
                    if gpu_rng is not None: torch.cuda.set_rng_state(gpu_rng)
                    m.train();optimizer=optimizers[name];optimizer.zero_grad(set_to_none=True)
                    with autocast(): out=forward(m,b)
                    out={k:v.float() if v.is_floating_point() else v for k,v in out.items()}
                    losses=m.loss(out,b[7],b[8],b[9],b[10],b[11],**LOSS)
                    kl=F.kl_div(F.log_softmax(out['slot_logits'],-1),F.softmax(parent_out['slot_logits'].float(),-1),reduction='batchmean')
                    loss=losses['loss']+.1*kl
                    if name=='banana-2' and bi%args.aux_every==0:
                        if aux_cursor>=len(aux_order): aux_order=rng.permutation(len(aux_order));aux_cursor=0
                        chosen=aux_order[aux_cursor:aux_cursor+128];aux_cursor+=len(chosen)
                        ab,y=aux_batch(data['aux']['train'],chosen)
                        with autocast(): ao=forward(m,to_device(ab,device))
                        head.train();prediction=head(ao['fused'].float())
                        auxiliary=F.smooth_l1_loss(prediction,(y.to(device)-mean)/scale)
                        loss=loss+args.aux_weight*auxiliary
                        aux_sum+=float(auxiliary.detach());aux_steps+=1
                    if not torch.isfinite(loss): raise RuntimeError('Non-finite training loss')
                    loss.backward();nn.utils.clip_grad_norm_(list(m.parameters())+(list(head.parameters()) if name=='banana-2' else []),1.)
                    optimizer.step();sums[name]+=float(losses['loss'].detach())
                step+=1
                if time.monotonic()-monitor.last>=10 or bi+1==len(loader):
                    eta=(total_steps-step)*(time.monotonic()-train_clock)/max(1,step)
                    monitor.publish(f'Epoch {epoch}/{epoch_offset+args.epochs}: paired imitation training',step,total_steps,eta,
                                    epoch=epoch,loss=sums['banana-2']/(bi+1),control_loss=sums['bc-control']/(bi+1),
                                    aux_loss=aux_sum/max(1,aux_steps),action_rows_seen=min(step*args.batch_size,args.epochs*len(train)))
            for name,m in models.items():
                monitor.publish(f'Epoch {epoch}/{epoch_offset+args.epochs}: evaluating {name}',step,total_steps,
                                (total_steps-step)*(time.monotonic()-train_clock)/max(1,step))
                metrics=evaluate_policy(m,val_loader,device,loss_kwargs=LOSS,think_steps=3,amp_enabled=False)
                record={'at':now(),'epoch':epoch,'model':name,'train_loss':sums[name]/len(loader),
                        'epoch_seconds':time.monotonic()-epoch_clock,**metrics}
                if name=='banana-2':
                    record.update(evaluate_aux(m,head,data['aux']['validation'],mean,scale,device))
                    record['train_aux_loss']=aux_sum/max(1,aux_steps)
                    head_checkpoint={'state_dict':{k:v.detach().cpu().clone() for k,v in head.state_dict().items()},
                                     'mean':mean.cpu(),'scale':scale.cpu(),'targets':TARGET_NAMES,'epoch':epoch}
                    save_torch(args.run_dir/'aux-head-latest.pt',head_checkpoint)
                monitor.history.append(record)
                folder=args.run_dir/name;folder.mkdir(exist_ok=True)
                checkpoint(name,epoch,metrics,folder/'latest.pt')
                if metrics['loss']<best[name]:
                    best[name]=metrics['loss'];checkpoint(name,epoch,metrics,folder/'best_model.pt')
                    if name=='banana-2': save_torch(args.run_dir/'aux-head-best.pt',head_checkpoint)
                atomic_json(args.run_dir/'metrics.json',{'history':monitor.history})
            monitor.publish(f'Epoch {epoch}/{epoch_offset+args.epochs} complete',step,total_steps,
                            (total_steps-step)*(time.monotonic()-train_clock)/max(1,step))
        # Export only the chosen auxiliary actor; keep the paired control too.
        selected=torch.load(args.run_dir/'banana-2/best_model.pt',map_location='cpu',weights_only=False)
        save_torch(args.run_dir/'best_model.pt',selected)
        save_torch(args.run_dir/'base.pt',selected)
        chosen_aux=min((r for r in monitor.history if r['model']=='banana-2'),key=lambda r:r['loss'])
        chosen_control=min((r for r in monitor.history if r['model']=='bc-control'),key=lambda r:r['loss'])
        atomic_json(args.run_dir/'summary.json',{'selected':chosen_aux,'bc_control':chosen_control,'parent':baseline,
                    'aux_minus_control_loss':chosen_aux['loss']-chosen_control['loss'],
                    'promotion':'Research candidate only. Native label fidelity and live-play benefit remain unverified.'})
        monitor.publish('Completed: experimental actor and matched control saved',total_steps,total_steps,0,status='completed')
    except BaseException as error:
        monitor.publish('Run failed',0,1,status='failed',error=f'{type(error).__name__}: {error}')
        raise


if __name__=='__main__': main()
