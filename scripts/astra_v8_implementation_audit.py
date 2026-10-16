"""Read-only CPU probes of the September 6 v8 implementation and run.

No training, checkpoint writes, replay collection, or phone interaction.
Results are a timestamped snapshot; the live run can advance afterwards.
Run: CUDA_VISIBLE_DEVICES='' OMP_NUM_THREADS=2 .venv/bin/python scripts/astra_v8_implementation_audit.py
"""
from __future__ import annotations

import hashlib
import json
import pickle
import runpy
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path

import torch

from cr_replay_pipeline.belief_policy import CausalBeliefPolicy, TimeTransition, build_policy_model
from cr_replay_pipeline.event_clock import competing_risk_nll
from cr_replay_pipeline.feature_schema import GLOBAL_INDEX
from cr_replay_pipeline.policy_dataset import PolicyActionDataset, encode_policy_sample
from cr_replay_pipeline.policy_manifest import build_manifest, resolve_battle_splits
from cr_replay_pipeline.winner_dataset import CardVocab, load_card_costs


def main():
    torch.set_num_threads(2)
    torch.manual_seed(42)
    start = time.monotonic()
    result = {"timestamp_utc": datetime.now(timezone.utc).isoformat()}
    print('stage=synthetic work=0/4 ETA=<60s', flush=True)
    fixtures = runpy.run_path('tests/test_eval_integrity.py')
    battle = fixtures['_asymmetric_battle']()
    vocab = CardVocab([*battle.team_deck, *battle.opponent_deck])
    index = 12
    observed = float(battle.events[index - 1]['seconds'])
    a = encode_policy_sample(battle, index, vocab, {}, threat_dim=14, observation_seconds=observed)
    b = encode_policy_sample(battle, index, vocab, {}, threat_dim=14, observation_seconds=observed + 5)
    model = CausalBeliefPolicy(vocab_size=vocab.vocab_size, global_dim=62, d_model=32,
                               memory_tokens=8, num_layers=1, dropout=0).eval()
    def query_dt(sample):
        seconds = sample[0][:, 0].unsqueeze(0) * 300
        return float(model._query_delay(seconds, sample[-1].reshape(1), sample[4].unsqueeze(0)))
    result['observation_clock'] = {
        'requested_seconds': [observed, observed + 5],
        'encoded_now_seconds': [float(s[4][GLOBAL_INDEX['now']]) * 300 for s in (a, b)],
        'memory_query_dt_seconds': [query_dt(s) for s in (a, b)],
        'global_features_changed': int((a[4] != b[4]).sum()),
    }
    rates = torch.tensor([[0.4, 0.3]], requires_grad=True)
    competing_risk_nll(rates, torch.tensor([2.0])).sum().backward()
    result['clock_gradient'] = {'rates': rates.detach().tolist(), 'delay': 2,
                                'gradient_own_opponent': rates.grad.tolist()}
    q = torch.randn(2, 8, 32, requires_grad=True)
    e = torch.randn(2, 1, 32)
    attn = model.cross_attn
    first, _ = attn(q, e, e, need_weights=False)
    second, _ = attn(q + torch.randn_like(q) * 10, e, e, need_weights=False)
    (first * torch.randn_like(first)).sum().backward()
    result['one_key_attention'] = {
        'max_change_when_query_changes': float((first - second).abs().max()),
        'max_query_input_gradient': float(q.grad.abs().max()),
        'max_qk_projection_gradient': float(attn.in_proj_weight.grad[:64].abs().max()),
        'max_value_projection_gradient': float(attn.in_proj_weight.grad[64:].abs().max()),
    }
    transition = TimeTransition(32)
    state = torch.randn(2, 8, 32)
    result['zero_time_transition'] = {
        'max_state_change_at_dt_zero': float((transition(state, torch.zeros(2)) - state).abs().max())}
    with tempfile.TemporaryDirectory() as folder:
        folder = Path(folder)
        battles = fixtures['_id_battles'](100)
        donor = build_manifest(battles, folder / 'donor/split_manifest.json', seed=42)
        newer = build_manifest(battles, folder / 'new.json', seed=43)
        _, _, test, _, _ = resolve_battle_splits(battles, split_manifest=folder / 'new.json',
                         warmstart_dir=folder / 'donor', output_dir=folder / 'output', create=True)
        result['warmstart_manifest'] = {'accepted': True, 'test_count': len(test),
            'donor_train_in_new_test': len(set(donor['train_ids']) & {x.battle_id for x in test})}
    print('stage=checkpoint work=1/4 ETA=<60s', flush=True)
    checkpoint_path = Path('models/policy_bc_v8/last.pt')
    # Atomic writer makes a single open/read a consistent checkpoint snapshot.
    import io
    payload = checkpoint_path.read_bytes()
    ckpt = torch.load(io.BytesIO(payload), map_location='cpu', weights_only=False)
    result['checkpoint'] = {'file': str(checkpoint_path), 'sha256': hashlib.sha256(payload).hexdigest(),
                            'epoch': ckpt.get('epoch'), 'config': ckpt.get('config')}
    cvocab = CardVocab.from_dict(ckpt['vocab'])
    current = build_policy_model(cvocab.vocab_size, ckpt['config']).eval()
    current.load_state_dict(ckpt['model_state'])
    with Path('data/.cr_replay_cache/raw_policy_p12_346201a2d03b5ae0.pkl').open('rb') as handle:
        cached = pickle.load(handle)
    result['cache_identity'] = {k: v for k, v in cached.items() if k != 'battles'}
    manifest = json.loads(Path('models/policy_bc_v8/split_manifest.json').read_text())
    by_id = {b.battle_id: b for b in cached['battles']}
    train = [by_id[i] for i in manifest['train_ids']]
    cost_path = Path('data/card_costs.json')
    costs = load_card_costs(cost_path)
    report = json.loads(Path('models/policy_bc_v8/report.json').read_text())
    compute = report['compute']
    ds = PolicyActionDataset(train, cvocab, costs, max_context=64, max_samples_per_battle=40, seed=11,
            threat_dim=14, reaction_seconds=5, prefer_reactions=True, reaction_repeats=2,
            reaction_weight=3, mirror_augmentation=True, stream_cache_size=2, winner_only=False)
    result['training_population'] = {'battle_count': len(train), 'dataset_rows': len(ds),
        'unique_battle_event_pairs': len(set(ds.index)),
        'mirror_rows': sum(ds._mirror_flags),
        'reported_rows': report['data']['train_samples'],
        'cache_ability_events': sum(e['event_type'] == 'ability_activation' for b in cached['battles'] for e in b.events),
        'cache_battles': len(cached['battles'])}
    print('stage=forward work=2/4 ETA=<30s', flush=True)
    from cr_replay_pipeline.policy_dataset import collate_policy_batch
    samples = []
    for bid in manifest['validation_ids'][:32]:
        battle = by_id[bid]
        sample = encode_policy_sample(battle, min(12, len(battle.events)-1), cvocab, costs,
                                      max_context=64, threat_dim=14)
        if sample is not None:
            samples.append((*sample[:-1], sample[-1], torch.tensor(1.0)))
    batch = collate_policy_batch(samples, pad_to=64)
    with torch.no_grad():
        out = current(batch[0], batch[1], batch[2], batch[3], batch[4], batch[12], batch[5], batch[6])
    clock = out['clock_rates']
    result['checkpoint_clock_probe'] = {'n': len(samples),
        'mean_own_rate': float(clock[:, 0].mean()), 'mean_opponent_rate': float(clock[:, 1].mean()),
        'opponent_rate_min_max': [float(clock[:, 1].min()), float(clock[:, 1].max())],
        'mean_implied_opponent_next_probability': float((clock[:, 1] / clock.sum(-1)).mean()),
        'query_dt_max': float(out['query_dt'].max())}
    # Current stage history is distinct from the archived first-run report.
    result['current_history'] = json.loads(Path('models/policy_bc_v8/training_stages.json').read_text())
    result['first_report'] = {'created_at': report['created_at'], 'compute': compute, 'data': report['data'],
                              'test': report['test'], 'baselines': report['baselines']}
    holdout = json.loads(Path('reports/astra_v8_evidence/cohort.json').read_text())
    hids = {row['battle_id'] for row in holdout}
    result['original_probe_overlap_with_v8'] = {
        split: len(hids & set(manifest[split])) for split in ('train_ids', 'validation_ids', 'test_ids')}
    result['elapsed_seconds'] = time.monotonic() - start
    path = Path('reports/astra_v8_evidence/implementation_audit.json')
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(result, indent=2) + '\n')
    for key in ('observation_clock', 'clock_gradient', 'one_key_attention', 'zero_time_transition',
                'warmstart_manifest', 'training_population', 'checkpoint_clock_probe', 'original_probe_overlap_with_v8'):
        print(key, json.dumps(result[key]))
    print(f'stage=done work=4/4 ETA=0s results={path}', flush=True)


if __name__ == '__main__':
    main()
