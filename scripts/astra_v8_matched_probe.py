"""CPU comparison on the earlier 512-battle diagnostic cohort.

Frozen models, current causal encoding, common card-only targets and revealed
opponent cards. This cohort informed architecture design: diagnostic, not a
new pristine test set. No training or live games.
"""
import hashlib
import io
import json
import pickle
import time
from datetime import datetime, timezone
from pathlib import Path

import numpy as np
import torch
from torch.utils.data import DataLoader
from cr_replay_pipeline.belief_policy import build_policy_model
from cr_replay_pipeline.policy_dataset import PolicyActionDataset, collate_policy_batch
from cr_replay_pipeline.policy_model import TILE_ROWS, TILE_COLS
from cr_replay_pipeline.winner_dataset import CardVocab, load_card_costs

torch.set_num_threads(2)
started = time.monotonic()
root = Path('reports/astra_v8_evidence')
cohort = json.loads((root / 'cohort.json').read_text())
with Path('data/.cr_replay_cache/raw_policy_p12_346201a2d03b5ae0.pkl').open('rb') as handle:
    by_id = {b.battle_id: b for b in pickle.load(handle)['battles']}
battles = [by_id[row['battle_id']] for row in cohort if row['battle_id'] in by_id]
assert len(battles) == 512, 'Do not silently change cohort'
costs = load_card_costs('data/card_costs.json')
result = {'timestamp_utc': datetime.now(timezone.utc).isoformat(), 'battles': len(battles),
          'contract': 'current causal encoding, both sides, card-only, revealed opponent deck, diagnostic cohort', 'models': {}}
arrays = {}
common_index = None
for mi, name in enumerate(('policy_bc_v4.4.1', 'policy_bc_v8')):
    payload = (Path('models') / name / 'best_model.pt').read_bytes()
    ckpt = torch.load(io.BytesIO(payload), map_location='cpu', weights_only=False)
    cfg = ckpt['config']
    vocab = CardVocab.from_dict(ckpt['vocab'])
    model = build_policy_model(vocab.vocab_size, cfg).eval()
    model.load_state_dict(ckpt['model_state'])
    ds = PolicyActionDataset(battles, vocab, costs, max_context=64, max_samples_per_battle=24,
          stride=3, seed=20260906, threat_dim=14, stream_cache_size=64)
    ds.index = [(bi, ei) for bi, ei in ds.index if battles[bi].events[ei]['event_type'] == 'card_play']
    ds._mirror_flags = [False] * len(ds.index)
    if common_index is None:
        common_index = ds.index.copy()
    assert ds.index == common_index
    revealed = []
    for bi, ei in ds.index:
        b = battles[bi]
        own = b.events[ei]['side']
        opp = 'opponent' if own == 'team' else 'team'
        deck = b.opponent_deck if own == 'team' else b.team_deck
        seen = {e['card'] for e in b.events[:ei] if e['side'] == opp}
        revealed.append(torch.tensor([vocab.encode(c) if c in seen else vocab.unk_id for c in deck]))
    revealed = torch.stack(revealed)
    rows = []
    offset = 0
    with torch.inference_mode():
        for batch in DataLoader(ds, batch_size=128, collate_fn=collate_policy_batch):
            c, ids, td, od, g, sf, hm, slot, types, zone, xy, timing, lengths, weights = batch
            n = len(slot)
            out = model(c, ids, td, revealed[offset:offset+n], g, lengths, sf, hm,
                        think_steps=int(cfg.get('eval_think_steps', cfg.get('max_think_steps', 0))))
            target_tile = (xy[:, 1] * TILE_ROWS).long().clamp(0, TILE_ROWS-1) * TILE_COLS + (xy[:, 0] * TILE_COLS).long().clamp(0, TILE_COLS-1)
            card_ok = out['slot_logits'].argmax(-1) == slot
            tile_ok = out['tile_logits'].argmax(-1) == target_tile
            rows.append(torch.stack([card_ok.float(), tile_ok.float(), (card_ok & tile_ok).float(),
                    (out['tile_logits'].topk(5).indices == target_tile[:, None]).any(1).float(),
                    (out['timing'].expm1() - timing.expm1()).abs()], 1).numpy())
            offset += n
            if offset % 1024 == 0 or offset == len(ds):
                done = mi * len(ds) + offset
                elapsed = time.monotonic() - started
                eta = elapsed / done * (2*len(ds)-done)
                print(f'model={name} work={done}/{2*len(ds)} ETA={eta:.0f}s elapsed={elapsed:.0f}s', flush=True)
    arrays[name] = np.concatenate(rows)
    result['models'][name] = {'checkpoint_sha256': hashlib.sha256(payload).hexdigest(), 'checkpoint_epoch': ckpt.get('epoch')}
groups = np.array([bi for bi, ei in common_index])
group_n = np.bincount(groups, minlength=len(battles))
draws = np.random.default_rng(20260906).integers(0, len(battles), (2000, len(battles)))
denom = group_n[draws].sum(1)
columns = ['card_top1', 'grid_top1', 'joint_card_grid', 'grid_top5', 'timing_mae_s']
boot = {}
for name, data in arrays.items():
    sums = np.stack([np.bincount(groups, weights=data[:, j], minlength=len(battles)) for j in range(len(columns))], 1)
    boot[name] = sums[draws].sum(1) / denom[:, None]
    ci = np.quantile(boot[name], [.025, .975], axis=0)
    result['models'][name]['metrics'] = {c: {'mean': float(data[:, j].mean()), 'ci95': ci[:, j].tolist()} for j, c in enumerate(columns)}
names = list(arrays)
delta = boot[names[1]] - boot[names[0]]
ci = np.quantile(delta, [.025, .975], axis=0)
result['v8_minus_v441'] = {c: {'mean': float((arrays[names[1]][:, j] - arrays[names[0]][:, j]).mean()), 'ci95': ci[:, j].tolist()} for j, c in enumerate(columns)}
result['samples'] = len(groups)
result['sample_index_sha256'] = hashlib.sha256(json.dumps([(battles[bi].battle_id, ei) for bi, ei in common_index]).encode()).hexdigest()
result['seconds'] = time.monotonic() - started
(root / 'v8_matched_probe.json').write_text(json.dumps(result, indent=2) + '\n')
np.savez_compressed(root / 'v8_matched_probe_rows.npz', groups=groups, **arrays)
print(json.dumps(result, indent=2), flush=True)
