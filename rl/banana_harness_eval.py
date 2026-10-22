"""Frozen, paired Hog 2.6 evaluation through the viewer's corrected scheduler.

Every game polls at 20 Hz, retains unaffordable card plans, uses learned delays
and native placement masks. Only original-native terminal outcomes count.
Run with the sibling project's Python environment: python -m rl.banana_harness_eval.
"""
from __future__ import annotations

import argparse
import concurrent.futures as futures
from collections import Counter
import hashlib
import json
import multiprocessing as mp
from pathlib import Path
import time

import torch

from .banana_hog_league import metric, paired_delta
from .banana_policy import BananaEpisode, _native_card
from .banana_timed_policy import PhoneStyleScheduler
from .banana_train import ROOT, JsonlEngine, atomic_json, sha256, utc_now
from .banana_watch import load_watched_policy, tower_state_from_frame


def initialize(config):
    global CONFIG, POLICIES, ENGINE, DECKS
    CONFIG = config
    torch.set_num_threads(1)
    torch.manual_seed(991)
    POLICIES = {key: load_watched_policy(Path(path), 'cpu', 991, key)
                for key, path in config['checkpoints'].items()}
    ENGINE = JsonlEngine(Path(config['engine_checkpoint']), Path(config['engine_bin']))
    DECKS = {d['id']: d for d in json.loads((ROOT / 'data/hog2_opponent_pool.json').read_text())['decks']}


def verify_decks(response, expected):
    actual = response['configured_decks']
    for side, names in enumerate(actual):
        names = names.get('cards') if isinstance(names, dict) else names
        normalized = []
        for name in names:
            compact = ''.join(c.lower() for c in name if c.isalnum())
            normalized.append(POLICIES['banana-1'].native_name_to_policy.get(compact)
                              or _native_card(name, expected[side]))
        if Counter(normalized) != Counter(expected[side]):
            raise ValueError(f'wrong native deck side {side}: {names}')


def run_game(job):
    start = time.monotonic()
    row = dict(job, at=utc_now(), controller='banana-1-phone-style')
    side, opponent = job['seat'], DECKS[job['deck_id']]
    hog = DECKS['hog-mirror']
    native, slugs = [hog['native_cards'], opponent['native_cards']], [tuple(hog['cards']), tuple(opponent['cards'])]
    policies = [POLICIES[job['model']], POLICIES['banana-1']]
    if side == 1:
        native.reverse()
        slugs.reverse()
        policies.reverse()
    plays, rejected, saving_ticks = [Counter(), Counter()], Counter(), [0, 0]
    digest = hashlib.sha256()
    try:
        response = ENGINE.request(dict(op='reset', deck0=native[0], deck1=native[1],
                                       seed=job['seed'], max_ticks=CONFIG['max_ticks'], reward_state=True))
        verify_decks(response, slugs)
        response = ENGINE.request(dict(op='step', actions=[], ticks=100))
        if response['ticks_advanced'] != 100:
            raise RuntimeError('native warmup failed')
        obs = response['observation']
        obs['tower_state'] = response['reward_state']
        base_tick = obs['tick']
        episode = BananaEpisode(tuple(slugs), battle_id=f"harness-eval-{job['seed']}-{side}")
        episode.policy_seed = job['seed']
        def mask(request, native_slot):
            return ENGINE.request(dict(op='placement_mask', side=request[2], slot=native_slot))['mask']
        scheduler = PhoneStyleScheduler(policies, greedy=job['mode'] == 'greedy', placement_mask_provider=mask)
        last_tick = obs['tick']
        while not (response.get('terminated') or response.get('truncated')):
            now = (obs['tick'] - base_tick) / 20.
            actions = scheduler.choose(episode, obs, now)
            for s in (0, 1):
                saving_ticks[s] += scheduler.status[s].get('reason') == 'saving_elixir'
            response = ENGINE.request(dict(op='step', actions=[a.native_request() for a in actions], ticks=1))
            results = response['actions']
            if len(results) != len(actions):
                raise RuntimeError('mismatched native action results')
            for action, result in zip(actions, results):
                scheduler.observe_result(action, result)
                digest.update(json.dumps([obs['tick'], action.card, action.native_request(), result], sort_keys=True).encode())
                if result.get('accepted'):
                    policies[action.side].record_action(episode, action, now, True)
                    plays[action.side][action.card] += 1
                else:
                    rejected[result.get('rejection', 'unknown')] += 1
            obs = response['observation']
            obs['tower_state'] = response['reward_state']
            if obs['tick'] <= last_tick and not (response.get('terminated') or response.get('truncated')):
                raise RuntimeError('native world did not advance')
            last_tick = obs['tick']
        terminal = bool(response.get('terminated'))
        winner = obs.get('winner_side')
        if terminal and winner not in (None, -1, 0, 1):
            raise ValueError(f'unknown native winner {winner}')
        row.update(result=('VOID' if not terminal else 'D' if winner in (None, -1) else 'W' if winner == side else 'L'),
                   winner_side=winner, terminated=terminal, truncated=bool(response.get('truncated')),
                   seconds=(obs['tick'] - base_tick) / 20., plays=plays[side], opponent_plays=plays[1-side],
                   hog_plays=plays[side]['hog-rider'], rejected=rejected, saving_elixir_seconds=saving_ticks[side] / 20.,
                   tower_state=obs['tower_state'], action_trace_sha256=digest.hexdigest())
    except Exception as exc:
        row.update(result='VOID', error=f'{type(exc).__name__}: {exc}')
    row['wall_seconds'] = time.monotonic() - start
    return row


def summarize(rows):
    def cells(subset):
        return {model: metric([r for r in subset if r['model'] == model]) for model in ('banana-1', 'banana-hog-1')}
    modes = {}
    for mode in sorted({r['mode'] for r in rows}):
        selected = [r for r in rows if r['mode'] == mode]
        candidate = [r for r in selected if r['model'] == 'banana-hog-1']
        baseline = [r for r in selected if r['model'] == 'banana-1']
        modes[mode] = dict(overall=cells(selected), paired_delta=paired_delta(candidate, baseline),
                           by_deck={deck: cells([r for r in selected if r['deck_id'] == deck])
                                    for deck in sorted({r['deck_id'] for r in selected})},
                           by_seat={str(seat): cells([r for r in selected if r['seat'] == seat]) for seat in (0, 1)},
                           behavior={model: dict(
                               mean_hog_plays=sum(r.get('hog_plays', 0) for r in subset) / len(subset),
                               no_hog_games=sum(r.get('hog_plays', 0) == 0 for r in subset),
                               rejected=sum(sum(r.get('rejected', {}).values()) for r in subset),
                               mean_seconds=sum(r.get('seconds', 0) for r in subset) / len(subset))
                               for model in ('banana-1', 'banana-hog-1')
                               if (subset := [r for r in selected if r['model'] == model and r['result'] != 'VOID'])})
    return dict(updated_at=utc_now(), completed=len(rows), modes=modes,
                notes=['Frozen original Banana-1 pilots every opponent deck with the same corrected harness.',
                       'Each model receives identical seed/deck/seat/mode cells; sampling uses common per-tick random numbers.',
                       'Score counts draws as half; paired bootstrap blocks are seed/seat.',
                       'Only original-native terminal outcomes count. Truncated or failed games are VOID.'])


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir', type=Path, required=True)
    p.add_argument('--seeds', type=int, default=4)
    p.add_argument('--seed-start', type=int, default=3_700_100_000)
    p.add_argument('--workers', type=int, default=4)
    p.add_argument('--decks', nargs='+')
    p.add_argument('--modes', nargs='+', choices=['sampled', 'greedy'], default=['sampled'])
    p.add_argument('--max-ticks', type=int, default=7300)
    a = p.parse_args()
    if a.seeds < 1 or a.workers < 1 or not 0 <= a.seed_start <= a.seed_start + a.seeds - 1 < 2**32:
        p.error('invalid worker or seed settings')
    a.run_dir.mkdir(parents=True, exist_ok=True)
    checkpoints = {'banana-1': ROOT / 'runs/banana-1/base.pt',
                   'banana-hog-1': ROOT / 'runs/banana-hog-1/models/final.pt'}
    config = dict(checkpoints={k: str(v) for k, v in checkpoints.items()},
                  engine_checkpoint=str(ROOT / 'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state'),
                  engine_bin=str(ROOT / 'native-engine/target/release/league_reference'), max_ticks=a.max_ticks)
    manifest = dict(config, checkpoint_sha256={k: sha256(v) for k, v in checkpoints.items()},
                    source_sha256={name: sha256(ROOT / 'rl' / name) for name in
                                   ['banana_harness_eval.py', 'banana_timed_policy.py', 'banana_watch.py',
                                    'banana_fast_policy.py', 'banana_hog_adaptive.py']},
                    engine_sha256=sha256(Path(config['engine_bin'])),
                    pool_sha256=sha256(ROOT / 'data/hog2_opponent_pool.json'),
                    timing_controller='phone_style_learned_delay', native_placement_masks=True,
                    poll_ticks=1, device='cpu', learning=False)
    manifest_path = a.run_dir / 'manifest.json'
    if manifest_path.exists() and json.loads(manifest_path.read_text()) != manifest:
        p.error('run provenance changed; use a new output directory')
    atomic_json(manifest_path, manifest)
    all_decks = [d['id'] for d in json.loads((ROOT / 'data/hog2_opponent_pool.json').read_text())['decks']]
    decks = a.decks or all_decks
    if set(decks) - set(all_decks):
        p.error('unknown opponent deck')
    jobs = [dict(model=model, deck_id=deck, controller='banana-1-phone-style', seat=seat,
                 seed=a.seed_start + seed, mode=mode)
            for seed in range(a.seeds) for deck in decks for seat in (0, 1)
            for mode in a.modes for model in checkpoints]
    key = lambda r: (r['model'], r['deck_id'], r['seat'], r['seed'], r['mode'])
    results_path = a.run_dir / 'matches.jsonl'
    rows = [json.loads(line) for line in results_path.read_text().splitlines()] if results_path.exists() else []
    existing = {key(r) for r in rows}
    jobs = [job for job in jobs if key(job) not in existing]
    start = time.monotonic()
    print(f'{len(jobs)} remaining games, {a.workers} workers, corrected harness, no learning', flush=True)
    atomic_json(a.run_dir / 'schedule.json', vars(a) | {'run_dir': str(a.run_dir), 'remaining': len(jobs)})
    with futures.ProcessPoolExecutor(max_workers=a.workers, mp_context=mp.get_context('spawn'),
                                     initializer=initialize, initargs=(config,)) as pool:
        pending = [pool.submit(run_game, job) for job in jobs]
        with results_path.open('a', buffering=1) as out:
            for future in futures.as_completed(pending):
                row = future.result()
                out.write(json.dumps(row) + '\n')
                rows.append(row)
                summary = summarize(rows)
                summary.update(status='running', wall_seconds=time.monotonic() - start,
                               scheduled_this_run=len(jobs), existing_games=len(existing))
                atomic_json(a.run_dir / 'summary.json', summary)
                print(f"{len(rows)} games | {row['model']} vs {row['deck_id']} seat {row['seat']} seed {row['seed']} "
                      f"{row['result']} | {row.get('seconds', 0):.0f}s native, {row['wall_seconds']:.1f}s wall "
                      f"| {row.get('hog_plays', 0)} Hogs | {row.get('error', '')}", flush=True)
    summary = summarize(rows)
    summary.update(status='completed', wall_seconds=time.monotonic() - start)
    atomic_json(a.run_dir / 'summary.json', summary)
    if {k: sha256(v) for k, v in checkpoints.items()} != manifest['checkpoint_sha256']:
        raise RuntimeError('checkpoint files changed during evaluation')
    print(json.dumps(summary, indent=2), flush=True)


if __name__ == '__main__':
    main()
