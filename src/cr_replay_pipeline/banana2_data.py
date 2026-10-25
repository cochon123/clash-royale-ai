"""Experimental native-state labels for a replay-only Banana actor.

Quarantined files are read explicitly; their verification flags never change.
Native state is a training target, never an actor input.
"""
from __future__ import annotations

from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import random
import sqlite3
import zlib

import numpy as np
import torch

from .policy_dataset import encode_policy_sample, collate_policy_batch
from .v9_cache import card_event_fingerprint

# Per side: king/left/right tower HP, elixir, and living non-tower
# count + HP mass in left/right x own/enemy halves. Projectiles are excluded.
TARGET_NAMES = [f'{side}_{name}' for side in ('own', 'enemy') for name in
                ('king_hp', 'left_tower_hp', 'right_tower_hp', 'elixir',
                 'left_near_count', 'left_near_hp', 'right_near_count', 'right_near_hp',
                 'left_far_count', 'left_far_hp', 'right_far_count', 'right_far_hp')]


def board_target(frame):
    """Actor-relative scalar labels; native physical camps decide orientation."""
    own = int(frame['native_side'])
    before = frame['state_before_action']
    entities = before['board']['entities']
    target = np.zeros((2, 12), dtype=np.float32)
    # Missing/destroyed towers are zero. Require all six original tower slots
    # in reward_state or the board so malformed exports cannot invent labels.
    towers = before.get('reward_state', {}).get('towers', [])
    if len(towers) != 6:
        raise ValueError('Native export lacks six tower slots')
    for side in (0, 1):
        team = int(side != own)
        target[team, 3] = np.clip(float(before['observation']['elixir'][side])/10., 0, 1)
        for e in towers:
            if e['side'] != side:
                continue
            if e['row'] == 'KingTower':
                slot = 0
            elif e['row'] == 'PrincessTower':
                # The fixed native IDs identify left/right even after death.
                slot = 1 + ((int(e['id'])-5000000) % 3 == 2)
            else:
                continue
            target[team, slot] = np.clip(float(e['hp'])/max(1., float(e['max_hp'])), 0, 1)
        for e in entities:
            if e['side'] != side or e['row'] in {'KingTower','PrincessTower'}:
                continue
            if float(e.get('hp', 0)) <= 0 or float(e.get('max_hp', 0)) <= 0:
                continue
            # Reflect y for the actor at physical top; x stays screen-relative,
            # matching the policy's existing reflection convention.
            y = 32000-float(e['y']) if own else float(e['y'])
            region = int(y >= 16000)*2 + int(float(e['x']) >= 9000)
            target[team, 4+region*2] += 1/12.
            target[team, 5+region*2] += max(0.,float(e['hp']))/12000.
    return np.clip(target.flatten(), 0, 1)


def clean_partitions(by_id, manifest):
    """Keep legacy holdouts intact and remove side-swapped trajectory copies."""
    reserved = set(manifest['validation_ids']) | set(manifest['test_ids'])
    forbidden = {card_event_fingerprint(by_id[i]) for i in reserved if i in by_id}
    seen = set()
    train = []
    removed = 0
    for i in manifest['train_ids']:
        if i not in by_id:
            raise ValueError('Missing legacy training battle: '+i)
        fingerprint = card_event_fingerprint(by_id[i])
        if fingerprint in forbidden or fingerprint in seen:
            removed += 1
            continue
        seen.add(fingerprint)
        train.append(i)
    # Duplicate validation trajectories and trajectories in the reserved final
    # test are excluded from development; the final test itself is untouched.
    test = {card_event_fingerprint(by_id[i]) for i in manifest['test_ids'] if i in by_id}
    validation = []
    seen = set()
    for i in manifest['validation_ids']:
        fingerprint = card_event_fingerprint(by_id[i])
        if fingerprint not in seen and fingerprint not in test:
            seen.add(fingerprint)
            validation.append(i)
    assert not set(train)&reserved
    return train, validation, removed


def prepare_aux(run, by_id, partitions, vocab, costs, seed, callback, max_per_battle=12):
    """Join by source hash and exact prefix. Never join on filename alone."""
    output = {key: [] for key in partitions}
    membership = {i:key for key, ids in partitions.items() for i in ids}
    counts = Counter()
    results = sqlite3.connect(f'file:{run / "results.sqlite3"}?mode=ro', uri=True)
    inputs = sqlite3.connect(f'file:{run / "inputs/input.sqlite3"}?mode=ro', uri=True)
    jobs = results.execute('SELECT file,frames,result FROM results WHERE frames>8 ORDER BY file').fetchall()
    for done, (file, nframes, result_json) in enumerate(jobs, 1):
        meta, payload = inputs.execute('SELECT metadata,payload FROM inputs WHERE file=?', (file,)).fetchone()
        meta = json.loads(meta)
        ident = meta['original_battle_id']
        split = membership.get(ident)
        if split is None:
            counts['outside_training_and_development'] += 1
            continue
        battle = by_id[ident]
        # Unknown ability tokens/cards are never silently embedded as a known
        # card in this new native-target path.
        if any(vocab.encode(c) == vocab.unk_id for c in (*battle.team_deck,*battle.opponent_deck)):
            counts['unknown_deck_card'] += 1
            continue
        result = json.loads(result_json)
        artifact_path = run/'quarantine'/Path(result['output']).name
        with gzip.open(artifact_path, 'rt') as handle:
            artifact = json.load(handle)
        if artifact['verified'] or artifact['training_usable']:
            raise ValueError('Unexpected data provenance flags')
        if artifact['source_sha256'] != meta['sha256'] or result['source_sha256'] != meta['sha256']:
            raise ValueError('Native source hash mismatch: '+file)
        replay = json.loads(zlib.decompress(payload))
        events = replay['events']
        # Native events and old policy events can differ after unresolved
        # abilities. Only a fully identical leading prefix may be paired.
        prefix_ok = 0
        for a,b in zip(events,battle.events):
            if any(a[k] != b[k] for k in ('seconds','side','event_type','card','x','y')):
                break
            prefix_ok += 1
        candidates = []
        for frame in artifact['frames'][:nframes]:
            index = int(frame['event_index'])
            if index < 8 or index >= prefix_ok:
                counts['insufficient_or_mismatched_prefix'] += 1
                continue
            event = battle.events[index]
            if (frame['side'],frame['card'],frame['replay_tick']) != (event['side'],event['card'],round(event['seconds']*20)):
                raise ValueError('Frame/event mismatch: '+file)
            # A sample cannot observe another deployment at the query's tick.
            if battle.events[index-1]['seconds'] >= event['seconds']:
                counts['same_tick'] += 1
                continue
            # Existing policy coordinates assume team is the physical bottom.
            # Keep auxiliary orientation consistent; opposite-camp source
            # records remain usable for ordinary imitation, never mislabeled.
            expected = int(event['side'] == 'opponent')
            if int(frame['native_side']) != expected:
                counts['orientation_not_policy_canonical'] += 1
                continue
            candidates.append(frame)
        random.Random(f'{seed}:{ident}').shuffle(candidates)
        selected = sorted(candidates[:max_per_battle], key=lambda f:f['event_index'])
        for frame in selected:
            index = int(frame['event_index'])
            sample = encode_policy_sample(battle,index,vocab,costs,max_context=64,threat_dim=14,
                                          observation_seconds=frame['replay_tick']/20.)
            if sample is None:
                continue
            # At this independently queried clock, only the already confirmed
            # prefix is visible. The pending human card/XY/timing labels are
            # discarded. Actual native state is supervision only.
            sample = (*sample[:12],int(sample[12]),torch.tensor(1.))
            output[split].append((ident,index,sample,torch.from_numpy(board_target(frame)),
                                  hashlib.sha256(artifact_path.read_bytes()).hexdigest()))
        counts['aux_'+split+'_battles'] += bool(selected)
        if done % 100 == 0:
            callback(done,len(jobs),dict(counts))
    callback(len(jobs),len(jobs),dict(counts))
    results.close(); inputs.close()
    counts.update({key+'_rows':len(rows) for key,rows in output.items()})
    return output,dict(counts)


def aux_batch(rows, indices):
    chosen = [rows[int(i)] for i in indices]
    return collate_policy_batch([r[2] for r in chosen]), torch.stack([r[3] for r in chosen])
