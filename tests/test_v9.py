import json
import pickle
from dataclasses import replace
from pathlib import Path
from unittest.mock import patch

import numpy as np
import pytest
import torch

from cr_replay_pipeline.causal_observation import prefix_observation
from cr_replay_pipeline.controlled_dataset import ActionExamples
from cr_replay_pipeline.feature_schema import CONTINUOUS_INDEX as C, GLOBAL_INDEX as G
from cr_replay_pipeline.policy_dataset import collate_policy_batch
from cr_replay_pipeline.policy_model import TILE_ROWS, TILE_COLS
from cr_replay_pipeline.v9_cache import PrefixBank, check_bank, _write_shard, ShardDataset
from cr_replay_pipeline.v9_policy import V9Policy, GhostRasterizer, tile_loss
from cr_replay_pipeline.winner_dataset import BattleExample, build_vocab


@pytest.fixture
def battles():
    deck=("knight","archers","goblins","giant","hog-rider","fireball","cannon","balloon")
    result=[]
    for k in range(4):
        events=[]
        for i in range(24):
            ability=i in {5,17}
            events.append(dict(seconds=float(i*3 if i!=9 else 24),side="team" if i%2==0 else "opponent",event_type="ability_activation" if ability else "card_play",card="_unknown_ability" if ability else deck[i%8],x=9000 if ability else 2500+i*250,y=16000 if ability else (8000 if i%2==0 else 24000),position_known=not ability))
        result.append(BattleExample(f"battle-{k}",deck,deck,k%2,tuple(events)))
    return result


def make_cache(tmp_path,battles):
    vocab=build_vocab(battles);costs={x:4 for x in battles[0].team_deck}
    root=tmp_path/"cache";root.mkdir()
    for split in ("train","validation"):
        folder=root/split;folder.mkdir()
        shard=_write_shard((str(folder),0,battles,0,vocab.to_dict(),costs))
        (folder/"complete.json").write_text(json.dumps(dict(shards=[shard],battles=len(battles))))
    (root/"vocab.json").write_text(json.dumps(vocab.to_dict()))
    (root/"contract.json").write_text(json.dumps(dict(observation="causal-observation-1",reference_ancestry_caveat="test cohort")))
    return root,vocab,costs


def test_bank_matches_canonical_and_future_is_absent(battles):
    vocab=build_vocab(battles);costs={x:4 for x in battles[0].team_deck}
    for b in battles:check_bank(b,vocab,costs)
    b=battles[0];future=[dict(e,card="fireball",x=15000) for e in b.events[12:]]
    altered=replace(b,events=b.events[:12]+tuple(future))
    for side in ("team","opponent"):
        a=PrefixBank(b,vocab,costs).encode(12,side)
        z=PrefixBank(altered,vocab,costs).encode(12,side)
        assert all(torch.equal(x,y) for x,y in zip(a,z))


def test_unlimited_actions_and_storage_parity(tmp_path,battles):
    root,vocab,costs=make_cache(tmp_path,battles)
    canonical=ActionExamples(battles,vocab,costs,max_samples=0)
    assert len(canonical)>len(ActionExamples(battles,vocab,costs,max_samples=2))
    ds=ShardDataset(root/"train",reaction_weight=1.)
    indices=list(range(len(ds)))
    batch=ds.__getitems__(indices)
    reference=collate_policy_batch([canonical[i] for i in indices])
    for i,(a,b) in enumerate(zip(batch,reference)):
        if a.is_floating_point():
            torch.testing.assert_close(a,b,atol=5e-4 if i==0 else 1e-6,rtol=5e-4 if i==0 else 1e-6)
        else:assert torch.equal(a,b)
    from cr_replay_pipeline.controlled_dataset import ClockExamples,collate_clock
    clocks=ClockExamples(battles,vocab,costs)
    cached=ShardDataset(root/"train",kind="clocks").__getitems__(list(range(len(clocks))))
    canonical=collate_clock([clocks[i] for i in range(len(clocks))])
    for a,b in zip(cached[0],canonical[0]):
        if a.is_floating_point():torch.testing.assert_close(a,b,atol=5e-4,rtol=5e-4)
        else:assert torch.equal(a,b)
    assert torch.equal(cached[1],canonical[1]) and torch.equal(cached[2],canonical[2])


def test_worker_serialization_keeps_file_backed_arrays(tmp_path,battles):
    root,_,_=make_cache(tmp_path,battles)
    ds=ShardDataset(root/"train",reaction_weight=1.)
    raw=pickle.dumps(ds,protocol=5)
    assert len(raw)<10000
    restored=pickle.loads(raw)
    assert all(isinstance(a,np.memmap) for shard in restored.shards for a in shard.values())
    a,b=ds.__getitems__([0,5,11]),restored.__getitems__([0,5,11])
    assert all(torch.equal(x,y) for x,y in zip(a,b))


def test_interrupted_final_cohort_resumes_without_rewriting_complete_shards(tmp_path,battles):
    from cr_replay_pipeline.v9_evaluate import resume_cohort_shard
    vocab=build_vocab(battles);costs={c:4 for c in battles[0].team_deck}
    task=(str(tmp_path),0,battles,0,vocab.to_dict(),costs)
    original=resume_cohort_shard(task)
    file=tmp_path/"shard_0000/actions/g.npy";timestamp=file.stat().st_mtime_ns
    assert resume_cohort_shard(task)==original and file.stat().st_mtime_ns==timestamp
    (tmp_path/"shard_0000/complete.json").unlink();file.write_bytes(b"interrupted write")
    assert resume_cohort_shard(task)==original
    assert np.load(file).shape[1]==62
    np.save(file,np.zeros((1,1)))
    with pytest.raises(ValueError,match="shape changed"):resume_cohort_shard(task)


@pytest.mark.parametrize("scope",["independent_full_training","shared_donor_continuations"])
def test_final_holdout_scores_identical_control_and_candidate_once(tmp_path,battles,monkeypatch,scope):
    import shutil
    from cr_replay_pipeline.artifact_cache import sha256_file
    from cr_replay_pipeline import v9_evaluate as evaluation
    root,vocab,_=make_cache(tmp_path,battles);output=tmp_path/"output";output.mkdir()
    contract=json.loads((root/"contract.json").read_text())
    contract["holdout_sha256"]=sha256_file("data/splits/controlled_final_holdout.json")
    (root/"contract.json").write_text(json.dumps(contract))
    shutil.copytree(root/"validation",output/"final_cohort")
    monkeypatch.setattr(evaluation,"load_manifest",lambda *a:dict(test_ids=[b.battle_id for b in battles]))
    entries=[]
    for name in ("v4_4_1","v4_4_2","v9_base_seed42","v9_seed42","v9_seed43","v9_seed44"):
        folder=tmp_path/name;folder.mkdir()
        (folder/"best.pt").write_bytes(b"shared-donor" if name in {"v9_base_seed42","v9_seed42"} else name.encode())
        entry=dict(name=name,path=str(folder),role="candidate" if name in {"v9_seed42","v9_seed43","v9_seed44"} else "control")
        entries.append(entry)
    evaluation.freeze_selection(output,entries,dict(synthetic_test=True,seed_reproduction=scope))
    calls=[]
    def fake_score(path,ds,vocab,device,legacy=False,output=None):
        calls.append(str(path))
        baseline=Path(path).parent.name.startswith("v4")
        metrics=dict(card=.5 if baseline else .56,joint=.1 if baseline else .16,joint_nll=2. if baseline else 1.8)
        columns=evaluation.COLUMNS;rows=np.zeros((len(ds),len(columns)))
        for key,value in metrics.items():rows[:,columns.index(key)]=value
        np.savez(output,rows=rows,groups=ds.metadata("group"),events=ds.metadata("event"),columns=columns)
        return metrics
    monkeypatch.setattr(evaluation,"score_checkpoint",fake_score)
    result=evaluation.holdout_pass(root,output,"cpu")
    assert len(calls)==5 and result["predeclared_gate"]["accuracy_thresholds_met"]
    assert result["predeclared_gate"]["pass_gate"]==(scope=="independent_full_training")
    assert result["scores"]["v9_seed42"]["scored_as"]=="v9_base_seed42"
    assert (output/"holdout_v9_seed42.npz").read_bytes()==(output/"holdout_v9_base_seed42.npz").read_bytes()
    assert evaluation.holdout_pass(root,output,"cpu")==result and len(calls)==5


def test_loader_workers_reopen_cache_and_finish(tmp_path,battles):
    from cr_replay_pipeline.v9_train import loader
    root,_,_=make_cache(tmp_path,battles)
    ds=ShardDataset(root/"train",reaction_weight=1.)
    batches=list(loader(ds,batch_size=16,workers=2))
    assert sum(len(b[7]) for b in batches)==len(ds)
    expected=ds.__getitems__(list(range(16)))
    assert all(torch.equal(x,y) for x,y in zip(batches[0],expected))


def test_mirroring_matches_reflected_battle_with_unknown_abilities(tmp_path,battles):
    root,vocab,costs=make_cache(tmp_path,battles)
    mirrored=[replace(b,events=tuple(dict(e,x=18000-e["x"] if e.get("position_known",True) else e["x"]) for e in b.events)) for b in battles]
    canonical=ActionExamples(mirrored,vocab,costs,max_samples=0)
    ds=ShardDataset(root/"train",reaction_weight=1.,mirror=True)
    with patch("torch.rand",side_effect=lambda n:torch.zeros(n)):
        batch=ds.__getitems__(list(range(len(ds))))
    expected=collate_policy_batch([canonical[i] for i in range(len(canonical))])
    for i in [0,4,9,10]:
        a,b=batch[i],expected[i]
        if i==0:
            # Padded tokens are outside the observation and have no future effect.
            for j,length in enumerate(batch[12]):
                torch.testing.assert_close(a[j,:length],b[j,:length],atol=5e-4,rtol=5e-4)
        else:torch.testing.assert_close(a,b,atol=1e-6,rtol=1e-6)


def test_outcome_weights_never_enter_observations(tmp_path,battles):
    root,_,_=make_cache(tmp_path,battles)
    ds=ShardDataset(root/"train",winner_weight=1.,reaction_weight=1.)
    weighted=ShardDataset(root/"train",winner_weight=3.,reaction_weight=1.)
    indices=list(range(len(ds)));a,b=ds.__getitems__(indices),weighted.__getitems__(indices)
    assert all(torch.equal(x,y) for x,y in zip(a[:13],b[:13]))
    assert (b[13]==3).any() and (b[13]==1).any()


def test_dense_gru_parity_and_ghost_ignores_unknown_positions(tmp_path,battles):
    torch.set_num_threads(1)
    root,vocab,_=make_cache(tmp_path,battles)
    batch=ShardDataset(root/"train").__getitems__(list(range(8)))
    model=V9Policy(vocab.vocab_size,clock=False);model.eval()
    model.base.padded_encoder=False;a=model(batch,teacher=True)
    model.base.padded_encoder=True;b=model(batch,teacher=True)
    for key in ["slot_logits","tile_logits","timing"]:
        torch.testing.assert_close(a[key],b[key],atol=1e-6,rtol=1e-5)
    stats=torch.zeros(vocab.vocab_size,7);stats[:,0]=1;stats[:,4]=20;stats[:,6]=1
    raster=GhostRasterizer(stats)
    continuous=batch[0].clone();continuous[:,:,C["pad"]]=1
    ghost=raster(continuous,batch[1],batch[4],batch[12])
    assert ghost.count_nonzero()==0


def test_smoothing_is_normalized_and_preserves_tile_center():
    logits=torch.zeros(2,576);xy=torch.tensor([[.2,.2],[.99,.99]])
    loss=tile_loss(logits,xy,.45)
    torch.testing.assert_close(loss,torch.full((2,),np.log(576),dtype=torch.float32))
    sharp=logits.clone();sharp[0,int(.2*TILE_ROWS)*TILE_COLS+int(.2*TILE_COLS)]=8
    assert tile_loss(sharp,xy,.45)[0]<loss[0]


def test_inference_uses_confirmed_prefix_and_observed_hand(battles):
    from cr_replay_pipeline.v9_infer import predict
    torch.set_num_threads(1)
    vocab=build_vocab(battles);costs={c:4 for c in battles[0].team_deck}
    model=V9Policy(vocab.vocab_size,clock=True).eval()
    source=battles[0]
    altered=replace(source,events=source.events[:12]+tuple(dict(e,card="balloon",x=17000) for e in source.events[12:]))
    arguments=dict(confirmed_events=12,agent_side="opponent",now=35.,observed_hand=["cannon"])
    a=predict(model,vocab,costs,source,**arguments)
    b=predict(model,vocab,costs,altered,**arguments)
    assert a==b and a["card"]=="cannon" and a["slot"]==6
    assert 0<a["x"]<1 and 0<a["y"]<1
    assert a["delay_seconds"]>0 and 0<=a["own_next_probability"]<=1
    assert predict(model,vocab,costs,source,12,now=35.,observed_hand=["unrecognized"])["status"]=="needs_observation"
    with pytest.raises(ValueError,match="future"):
        predict(model,vocab,costs,source,12,now=1.)


def test_legacy_adapter_restores_unused_feature_without_changing_v9_inputs(tmp_path,battles):
    from cr_replay_pipeline.v9_evaluate import LegacyScoringPolicy
    root,_,_=make_cache(tmp_path,battles)
    batch=ShardDataset(root/"train").__getitems__(list(range(8)))
    original=batch[0].clone()
    assert original[:,:,C["pad"]].any()
    class Recorder(torch.nn.Module):
        def forward(self,c,*args,placement_slots=None,think_steps=0):
            return dict(c=c.clone(),slots=placement_slots,think_steps=think_steps)
    adapter=LegacyScoringPolicy.__new__(LegacyScoringPolicy);torch.nn.Module.__init__(adapter)
    adapter.model=Recorder();adapter.config=dict(eval_think_steps=3)
    out=adapter(batch,teacher=True)
    assert out["c"][:,:,C["pad"]].count_nonzero()==0
    torch.testing.assert_close(out["c"][:,:,:C["pad"]],original[:,:,:C["pad"]])
    assert torch.equal(batch[0],original) and torch.equal(out["slots"],batch[7]) and out["think_steps"]==3


def test_offline_rollout_preserves_opening_and_generates_only_deployments(battles):
    from cr_replay_pipeline.v9_royale import rollout
    import random
    torch.set_num_threads(1)
    vocab=build_vocab(battles);costs={c:4 for c in battles[0].team_deck}
    model=V9Policy(vocab.vocab_size,clock=True).eval()
    source=battles[0];original=pickle.dumps(source)
    generated=rollout((model,vocab),(model,vocab),source,costs,random.Random(42),"cpu",new_events=10)
    assert generated.events[:8]==source.events[:8]
    assert len(generated.events)==18 and pickle.dumps(source)==original
    added=generated.events[8:]
    assert all(e["event_type"]=="card_play" and 0<=e["x"]<18000 and 0<=e["y"]<32000 for e in added)
    assert [e["seconds"] for e in generated.events]==sorted(e["seconds"] for e in generated.events)
    assert all(a["side"]!=b["side"] for a,b in zip(added,added[1:]))


def test_training_checkpoint_resume_and_native_report(tmp_path,battles):
    from cr_replay_pipeline.v9_train import parser,train
    root,_,_=make_cache(tmp_path,battles);out=tmp_path/"run"
    args=parser().parse_args(["--cache",str(root),"--output",str(out),"--epochs","1","--batch-size","16","--workers","0","--threads","1","--device","cpu","--no-clock"])
    report=train(args)
    assert report["weights_epoch"]==1 and (out/"best.pt").exists()
    html=(out/"report.html").read_text()
    assert "Replay curves" in html and "matplotlib" not in html
    args.resume=True;args.epochs=2
    resumed=train(args)
    assert resumed["last_completed_epoch"]==2
    args.winner_weight=2
    with pytest.raises(ValueError,match="Resume contract changed"):train(args)


def test_continuations_share_donor_and_reset_optimizer(tmp_path,battles):
    import os
    from cr_replay_pipeline.artifact_cache import sha256_file
    from cr_replay_pipeline.v9_train import parser,train
    root,_,_=make_cache(tmp_path,battles);donor=tmp_path/"donor"
    common=["--cache",os.path.relpath(root),"--epochs","1","--batch-size","16","--workers","0","--threads","1","--device","cpu"]
    train(parser().parse_args(common+["--output",str(donor)]))
    n=len(ShardDataset(root/"train"))
    np.save(root/"train/shard_0000/actions/advantage.npy",np.linspace(.5,1.5,n,dtype=np.float32))
    variants={"control":[],"winner":["--winner-weight","3"],"smooth":["--sigma",".45"],"advantage":["--advantage"],"ghost":["--ghost","aligned"],"shuffled":["--ghost","shuffled"]}
    for name,options in variants.items():
        output=tmp_path/name
        report=train(parser().parse_args(common+["--output",str(output),"--warmstart",str(donor),"--lr",".0001"]+options))
        assert report["status"]=="completed" and report["last_completed_epoch"]==1
        assert report["config"]["warmstart_sha256"]==sha256_file(donor/"best.pt")
        artifact=torch.load(output/"last.pt",weights_only=False)
        steps={int(s["step"]) for s in artifact["optimizer_state"]["state"].values()}
        # The clock head receives gradients only on sampled clock batches;
        # action-trunk steps still expose accidental optimizer continuation.
        assert max(steps)==(n+15)//16 and min(steps)>=1
