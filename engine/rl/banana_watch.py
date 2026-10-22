"""Watch two saved specialists fight through the original-asset spectator.

Supports the current tower-aware ``banana-hog-1`` on Hog 2.6 and the
original ``banana-1`` on Classic Log Bait, plus the saved Mortar specialist.
Either side can be pointed at any ``banana-native-ppo-v1`` checkpoint, or at
the frozen 4.4.2 actor by passing the legacy ``--deck`` switch. The native
runner receives a separate ``deck_preset_side1`` so the two sides really use
different native decks. No optimizer runs and no training files are written.
"""
from __future__ import annotations

import argparse
from collections import Counter, deque
import hashlib
import json
import secrets
from pathlib import Path
import subprocess
import time

import torch

from .banana_policy import BananaEpisode, _native_card
from .banana_placement import resolved_play_target
from .banana_hog_adaptive import AdaptiveHogPolicy, SamplingPolicy
from .banana_tower_hp import tower_hp_features
from .banana_timed_policy import PhoneStyleScheduler
from .banana_human import HumanController, InputQueue, own_side_only_cards
from .banana_train import BASE_CHECKPOINT, DECKS, JsonlEngine, NAMES, PRESETS, atomic_json, utc_now

ROOT = Path(__file__).resolve().parents[1]
LEAGUE_RUN = ROOT / 'runs/banana-league-20261019'
# Keep training deck registries unchanged; the spectator has its own options.
DECKS, PRESETS, NAMES = dict(DECKS), dict(PRESETS), dict(NAMES)
_log_bait = next(d for d in json.loads((ROOT / 'data/hog2_opponent_pool.json').read_text())['decks'] if d['id'] == 'log-bait')
DECKS['log-bait'] = tuple(_log_bait['cards'])
PRESETS['log-bait'] = 'log-bait'
NAMES['log-bait'] = 'banana-1'
DECK_LABELS = {'hog': 'Hog 2.6', 'mortar': 'Mortar', 'log-bait': 'Classic Log Bait'}


def tower_state_from_frame(frame, initial=None):
    """Retain native starting tower identities, including destroyed towers."""
    rows = {}
    for entity in frame['entities']:
        if entity.get('row') not in ('PrincessTower', 'KingTower'):
            continue
        key = (entity['side'], entity['native_entity_id'])
        if key in rows:
            raise ValueError('duplicate native tower identity')
        rows[key] = dict(id=key[1], side=key[0], row=entity['row'], hp=entity['hp'], max_hp=entity['max_hp'])
    if initial is not None:
        prior = {(r['side'], r['id']): r for r in initial['towers']}
        if rows.keys() - prior.keys():
            raise ValueError('unexpected native tower identity')
        for key, row in rows.items():
            if any(row[field] != prior[key][field] for field in ('row', 'max_hp')):
                raise ValueError('native tower identity changed')
        rows = {key: rows.get(key, dict(row, hp=0)) for key, row in prior.items()}
    state = {'towers': list(rows.values())}
    tower_hp_features(state, 0)  # Fail before inference if native data is invalid.
    return state


def load_watched_policy(checkpoint, device, seed, specialization):
    weights = None if checkpoint == BASE_CHECKPOINT else checkpoint
    payload = None if weights is None else torch.load(weights, map_location='cpu', weights_only=False)
    tower_aware = payload is not None and any(k.startswith('tower_hp_adapter.') for k in payload['model_state'])
    policy_type = AdaptiveHogPolicy if tower_aware else SamplingPolicy
    policy = policy_type(device=device, seed=seed, specialization=specialization, weights=weights)
    policy.requires_grad_(False)
    policy.eval()
    return policy



def default_checkpoint(deck: str) -> Path:
    """Return the trained final checkpoint for a deck, if the run exists."""
    if deck == 'hog':
        return ROOT / 'runs/banana-hog-1/models/final.pt'
    if deck == 'log-bait':
        return ROOT / 'runs/banana-1/base.pt'
    path = LEAGUE_RUN / 'models' / deck / 'final.pt'
    return path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir', type=Path, default=ROOT / 'runs/banana-duel-watch')
    p.add_argument('--engine-bin', type=Path, default=ROOT / 'native-engine/target/release/native-watch')
    p.add_argument('--viewer-bin', type=Path, default=ROOT / 'native-spectator/target/debug/clash-native-spectator')
    p.add_argument('--checkpoint', type=Path, default=ROOT / 'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state')
    p.add_argument('--deck', choices=['mortar', 'hog'], help='Legacy switch: mirror one deck on both sides with the frozen 4.4.2 actor')
    p.add_argument('--deck-a', choices=['mortar', 'hog', 'log-bait'], help='Side 0 deck (default hog)')
    p.add_argument('--deck-b', choices=['mortar', 'hog', 'log-bait'], help='Side 1 deck (default mortar)')
    p.add_argument('--checkpoint-a', type=Path, help='Side 0 saved actor; defaults to the trained run final.pt')
    p.add_argument('--checkpoint-b', type=Path, help='Side 1 saved actor; defaults to the trained run final.pt')
    p.add_argument('--legacy-wait-head', action='store_true', help='Diagnostic only: reproduce the old affordable-card and PPO wait-head controller')
    p.add_argument('--unmasked-placement', action='store_true', help='Diagnostic only: reproduce historical unfiltered placement behavior')
    p.add_argument('--greedy', action='store_true', help='Choose highest-probability actions; default samples like training')
    p.add_argument('--seed', type=int, help='Reproduce a match with a fixed 32-bit seed; default picks a fresh random seed')
    p.add_argument('--device', default='cpu')
    p.add_argument('--torch-threads', type=int, default=1)
    p.add_argument('--max-ticks', type=int, default=7300)
    p.add_argument('--decision-ticks', type=int, default=20)
    p.add_argument('--max-play-ticks', type=int, help='Bound a diagnostic capture without claiming a finished match')
    p.add_argument('--headless', action='store_true', help='Diagnostic frame/action capture without window or real-time pacing')
    p.add_argument('--human', action='store_true', help='Play blue with mouse/1-4; the saved actor controls red')
    a = p.parse_args(argv)
    if a.human and (a.legacy_wait_head or a.deck):
        p.error('human play uses the learned-delay controller and explicit decks')
    random_seed = a.seed is None
    if random_seed:
        a.seed = secrets.randbits(32)
    if not 0 <= a.seed < 2**32:
        p.error("seed must be in 0..4294967295")
    if not 1 <= a.decision_ticks <= 200 or a.torch_threads < 1 or a.max_ticks < 101:
        p.error('invalid tick/thread setting')
    if a.deck:
        a.deck_a = a.deck_b = a.deck
        legacy = True
    else:
        a.deck_a = a.deck_a or 'hog'
        a.deck_b = a.deck_b or 'mortar'
        legacy = False
    deck_keys = (a.deck_a, a.deck_b)
    checkpoints = []
    for name, deck, explicit in (('a', a.deck_a, a.checkpoint_a), ('b', a.deck_b, a.checkpoint_b)):
        if a.human and name == 'a':
            checkpoints.append(None)
            continue
        if explicit is not None:
            checkpoints.append(explicit.resolve())
            continue
        if legacy:
            checkpoints.append(BASE_CHECKPOINT)
            continue
        trained = default_checkpoint(deck)
        if not trained.exists():
            p.error(f'no trained checkpoint at {trained}; pass --checkpoint-{name} or use the legacy --deck switch')
        checkpoints.append(trained.resolve())
    a.run_dir = a.run_dir.resolve()
    a.run_dir.mkdir(parents=True, exist_ok=True)
    frame_path, control_path = a.run_dir / 'frame.json', a.run_dir / 'control.json'
    ready_path, status_path = a.run_dir / 'viewer-ready', a.run_dir / 'status.json'
    ready_path.unlink(missing_ok=True)
    control_path.unlink(missing_ok=True)
    torch.set_num_threads(a.torch_threads)
    torch.manual_seed(a.seed)
    # Each side owns an immutable actor. The episode carries both physical
    # decks so either policy can encode its own causal perspective.
    policies = []
    labels = []
    for index, (deck, checkpoint) in enumerate(zip(deck_keys, checkpoints)):
        if a.human and index == 0:
            policies.append(None)
            labels.append('You')
            continue
        policy = load_watched_policy(checkpoint, a.device, a.seed + index, NAMES[deck])
        policies.append(policy)
        labels.append(policy.checkpoint_name or 'frozen 4.4.2')
    engine = JsonlEngine(a.checkpoint, a.engine_bin)
    inputs_path = a.run_dir / 'inputs.jsonl'
    input_queue = InputQueue(inputs_path) if a.human else None
    # Name the human's cards with the AI's alias table, exactly as the
    # scheduler does, so accepted plays enter the episode in policy vocabulary.
    human = (HumanController(engine, DECKS[a.deck_a],
                             card_name=lambda row: PhoneStyleScheduler._card(policies[1], row),
                             own_side_only=own_side_only_cards(ROOT / 'analysis/decoded/assets/csv_logic'))
             if a.human else None)
    human_commands = deque()
    human_pending = None
    viewer = None
    actions_log = (a.run_dir / 'actions.jsonl').open('a', buffering=1)
    frames_log = (a.run_dir / 'frames.jsonl').open('a', buffering=1)
    decisions_log = (a.run_dir / 'decisions.jsonl').open('a', buffering=1)
    manifest = {
        'created_at': utc_now(), 'controllers': labels, 'decks': [list(DECKS[key]) for key in deck_keys],
        'deck_presets': [PRESETS[key] for key in deck_keys],
        'actor_checkpoints': [str(path) if path else None for path in checkpoints],
        'actor_checkpoint_sha256': [sha256(path) if path else None for path in checkpoints],
        'human_side': 0 if a.human else None,
        'frozen_source_checkpoint': str(BASE_CHECKPOINT),
        'frozen_source_checkpoint_sha256': hashlib.sha256(BASE_CHECKPOINT.read_bytes()).hexdigest(),
        'native_binary_sha256': hashlib.sha256(a.engine_bin.read_bytes()).hexdigest(),
        'device': a.device, 'seed': a.seed, 'seed_mode': 'random' if random_seed else 'explicit', 'ticks_per_second': 20,
        'decision_ticks': a.decision_ticks, 'learning': False, 'action_selection': 'greedy' if a.greedy else 'sampled',
        'native_placement_masks': not a.unmasked_placement,
        'timing_controller': 'legacy_wait_head' if a.legacy_wait_head else 'phone_style_learned_delay',
        'reference_harness': str(ROOT.parent / 'src/cr_replay_pipeline/phone_lab/battle.py'),
        'note': 'Independent watched match. No optimizer, no RL-run files or weights are modified. Each side samples its own saved actor through the native adapter.',
    }
    atomic_json(a.run_dir / 'manifest.json', manifest)
    state = {'status': 'loading', 'seed': a.seed, 'updated_at': utc_now(), 'message': 'Loading the original engine and the two saved actors.'}
    atomic_json(status_path, state)
    episode = None
    obs = None
    frame = None
    initial_towers = None
    base_tick = 0
    accepted = 0
    rejected = 0
    match_number = 0
    paused = False
    ended = False
    speed = 1.
    latest_serial = -1
    pending = []
    recent_plays = []
    next_play_id = 0

    def publish(status):
        nonlocal state
        assert frame is not None and obs is not None
        frame['elapsed_ms'] = max(0, int(obs['tick']) - base_tick) * 50
        frame.update(runtime_status=status, runtime_error=state.get('error'),
                     controller_names=labels, playback_speed=speed,
                     hands=obs['hands'], plays=[p for p in recent_plays if int(obs['tick'])-p['tick']<=40],
                     match_id=match_number, planned_actions=scheduler.status if not a.legacy_wait_head else [])
        if human is not None:
            frame['human'] = human.snapshot(obs, paused=paused, entities=frame['entities'],
                                            arena_size=frame['arena_size'])
        atomic_json(frame_path, frame)
        frames_log.write(json.dumps(frame, separators=(',', ':')) + '\n')
        state.update(status=status, updated_at=utc_now(), tick=obs['tick'],
                     game_seconds=frame['elapsed_ms'] / 1000, accepted_plays=accepted,
                     rejected_plays=rejected, match=match_number,
                     winner_side=obs.get('winner_side'), terminated=obs.get('terminated'),
                     playback_speed=speed, paused=paused, learning=False,
                     planned_actions=scheduler.status if not a.legacy_wait_head else [])
        atomic_json(status_path, state)

    def placement_mask(request, native_slot):
        return engine.request({'op': 'placement_mask', 'side': request[2], 'slot': native_slot})['mask']

    scheduler = PhoneStyleScheduler(policies, greedy=a.greedy,
                                    placement_mask_provider=None if a.unmasked_placement else placement_mask,
                                    active_sides=(1,) if a.human else (0, 1))
    last_decisions = [None, None]

    def choose_actions(seconds):
        if not a.legacy_wait_head:
            actions = scheduler.choose(episode, obs, seconds)
            for side, decision in enumerate(scheduler.status):
                signature = (decision.get('card'), decision.get('reason'), decision.get('ready_at'))
                if signature != last_decisions[side]:
                    decisions_log.write(json.dumps(dict(match=match_number, game_seconds=seconds,
                                                        side=side, **decision)) + '\n')
                    last_decisions[side] = signature
            return actions
        return [policy.act_batch([(episode, obs, side, seconds)], deterministic=a.greedy,
                                 placement_top_k=576 if getattr(policy, "tower_hp_enabled", False) else 5,
                                 placement_mask_provider=None if a.unmasked_placement else placement_mask)[0]
                for side, policy in enumerate(policies)]

    def reset():
        nonlocal initial_towers, episode, obs, frame, base_tick, accepted, rejected, match_number, paused, ended, pending, recent_plays, human_pending
        match_number += 1
        match_seed = (a.seed + match_number - 1) % 2**32
        state["seed"] = match_seed
        scheduler.plans, scheduler.status, scheduler.revision = [None, None], [{}, {}], None
        last_decisions[:] = [None, None]
        r = engine.request({'op': 'reset', 'deck_preset': PRESETS[a.deck_a],
                            'deck_preset_side1': PRESETS[a.deck_b],
                            'seed': match_seed, 'max_ticks': a.max_ticks})
        initial_towers = tower_state_from_frame(r['frame'])
        decks = r.get('configured_decks')
        if not isinstance(decks, list) or len(decks) != 2:
            raise RuntimeError('Native reset omitted configured decks')
        for index, side in enumerate(decks):
            expected = DECKS[deck_keys[index]]
            policy = policies[index] or policies[1-index]
            cards = side.get('cards') if isinstance(side, dict) else side
            normalized = []
            for card in cards:
                compact = ''.join(ch.lower() for ch in card if ch.isalnum())
                normalized.append(policy.native_name_to_policy.get(compact) or _native_card(card, expected))
            if Counter(normalized) != Counter(expected):
                raise RuntimeError(f'Wrong native deck on side {index}: {cards!r}')
        # The recovered fresh harness rejects inputs during its startup period.
        # Skip that phase before starting the visible, model-controlled clock.
        r = engine.request({'op': 'step', 'actions': [], 'ticks': 100})
        obs, frame = r['observation'], r['frame']
        obs['tower_state'] = tower_state_from_frame(frame, initial_towers)
        if r.get('ticks_advanced') != 100:
            raise RuntimeError('Native startup did not advance 100 ticks')
        base_tick = int(obs['tick'])
        episode = BananaEpisode((DECKS[a.deck_a], DECKS[a.deck_b]), battle_id=f'watched-duel-{match_number}')
        episode.policy_seed = match_seed
        accepted = rejected = 0
        recent_plays = []
        human_pending = None
        if human is not None:
            human.reset(match_number)
        pending = choose_actions(0.)
        paused = ended = False
        state.pop('error', None)
        state['message'] = f'{labels[0]} vs {labels[1]} · no learning · normal speed'
        publish('waiting' if viewer is None and not a.headless else 'running')
        print(f'Match {match_number} ready: {labels[0]} vs {labels[1]} '
              f'({PRESETS[a.deck_a]} vs {PRESETS[a.deck_b]}) · seed {match_seed}', flush=True)

    try:
        reset()
        if not a.headless:
            title = f'{labels[0]} vs {labels[1]} · {DECK_LABELS[a.deck_a]} vs {DECK_LABELS[a.deck_b]}'
            viewer = subprocess.Popen([str(a.viewer_bin), '--follow', str(frame_path), '--control-file', str(control_path),
                                       '--ready-file', str(ready_path), '--title', title]
                                      + (['--input-file', str(inputs_path), '--display-lag-ticks', '2'] if a.human else []), cwd=ROOT)
            wait_until = time.monotonic() + 90
            while not ready_path.exists():
                if viewer.poll() is not None:
                    raise RuntimeError(f'Spectator exited before showing match ({viewer.returncode})')
                if time.monotonic() > wait_until:
                    raise RuntimeError('Spectator window did not become ready')
                time.sleep(.02)
        publish('running')
        due = time.monotonic()
        print('1-4 or click a card, click arena to play. Right click/Esc cancel. Space pause, R restart, Q close.'
              if a.human else 'Playing at 20 native ticks/second (1×). Space pause, R restart, Esc close.', flush=True)
        while True:
            if viewer is not None and viewer.poll() is not None:
                break
            input_changed = False
            close_requested = False
            if input_queue is not None:
                human_commands.extend(input_queue.poll())
                while human_commands and human_pending is None:
                    command = human_commands.popleft()
                    outcome = human.handle(command, obs, paused=paused, ended=ended,
                                           arena_size=frame['arena_size'], entities=frame['entities'])
                    input_changed = True
                    if outcome == 'close':
                        close_requested = True
                        break
                    if outcome == 'reset':
                        reset()
                        due = time.monotonic()
                    elif outcome == 'pause':
                        paused = not paused
                        due = time.monotonic()
                    elif outcome is not None:
                        human_pending = outcome
                if close_requested:
                    break
                if input_changed:
                    publish('ended' if ended else 'paused' if paused else 'running')
            if control_path.exists():
                try:
                    command = json.loads(control_path.read_text())
                except (OSError, ValueError):
                    command = {}
                serial = int(command.get('serial', -1))
                if serial > latest_serial:
                    latest_serial = serial
                    op = command.get('op')
                    if op == 'close':
                        break
                    if op == 'pause':
                        paused = not paused
                        due = time.monotonic()
                        publish('ended' if ended else 'paused' if paused else 'running')
                    elif op == 'reset':
                        reset()
                        due = time.monotonic()
                    elif op == 'speed':
                        speed = max(.125, min(16., float(command.get('speed', 1))))
                        due = time.monotonic()
                        publish('ended' if ended else 'paused' if paused else 'running')
            if paused or ended:
                if a.headless and ended:
                    break
                time.sleep(.02)
                continue
            now = time.monotonic()
            if (not a.headless or a.human) and now < due:
                time.sleep(min(.01, due - now))
                continue
            elapsed_ticks = int(obs['tick']) - base_tick
            game_seconds = elapsed_ticks / 20
            sampled = []
            if not a.legacy_wait_head or elapsed_ticks % a.decision_ticks == 0:
                sampled = pending or choose_actions(game_seconds)
                pending = []
            if human_pending is not None:
                sampled = [human_pending, *sampled]
                human_pending = None
            native_actions = [action.native_request() for action in sampled if not action.wait]
            r = engine.request({'op': 'step', 'actions': native_actions, 'ticks': 1})
            results = r.get('actions')
            if not isinstance(results, list) or len(results) != len(native_actions):
                raise RuntimeError('Native runner returned mismatched action results')
            rows = iter(results)
            for action in sampled:
                if action.wait:
                    continue
                result = next(rows)
                human_action = a.human and action.side == 0
                if human_action:
                    human.observe_result(action, result, int(obs['tick']))
                elif not a.legacy_wait_head:
                    scheduler.observe_result(action, result)
                if result.get('accepted') is True:
                    land_x, land_y = resolved_play_target(result)
                    accepted += 1
                    episode.record_action(action.side, action.card, land_x, land_y, game_seconds, True)
                    hand_card = next(row for row in obs['hands'][action.side] if int(row['slot']) == action.slot)
                    next_play_id += 1
                    recent_plays.append({'id':next_play_id, 'side':action.side, 'name':hand_card['name'],
                                         'card_id':int(hand_card['card_id']), 'cost':int(hand_card['cost']),
                                         'x':land_x, 'y':land_y, 'tick':int(obs['tick'])})
                    recent_plays = recent_plays[-32:]
                else:
                    rejected += 1
                actions_log.write(json.dumps({'match': match_number, 'at': utc_now(), 'game_seconds': game_seconds,
                                              'side': action.side, 'card': action.card,
                                              'controller': 'human' if human_action else 'ai',
                                              'request': action.native_request(), 'result': result}) + '\n')
            obs, frame = r['observation'], r['frame']
            obs['tower_state'] = tower_state_from_frame(frame, initial_towers)
            if r.get('ticks_advanced') != 1 and not r.get('terminated') and not r.get('truncated'):
                raise RuntimeError('Native tick did not advance')
            ended = bool(r.get('terminated') or r.get('truncated'))
            status = 'ended' if ended else 'running'
            if ended:
                state['message'] = ('Original native match ended · winner side ' + str(obs.get('winner_side'))
                                    if r.get('terminated') else 'Tick limit reached · incomplete match')
                state['truncated'] = bool(r.get('truncated'))
                print(state['message'], flush=True)
            publish(status)
            due += .05 / speed
            # Do not fast-forward a backlog after a long scheduler interruption.
            if time.monotonic() - due > .25:
                due = time.monotonic()
            if a.max_play_ticks and int(obs['tick']) - base_tick >= a.max_play_ticks:
                state.update(status='diagnostic_complete', message='Bounded capture; no completed-match claim.')
                atomic_json(status_path, state)
                break
    except Exception as exc:
        state.update(status='failed', error=f'{type(exc).__name__}: {exc}', updated_at=utc_now())
        atomic_json(status_path, state)
        if frame is not None and obs is not None:
            publish('failed')
        raise
    finally:
        engine.close()
        if input_queue is not None:
            input_queue.close()
        if viewer is not None and viewer.poll() is None:
            viewer.terminate()
            viewer.wait(timeout=10)
        actions_log.close()
        frames_log.close()
        decisions_log.close()
        if state.get('status') not in ('failed', 'diagnostic_complete'):
            state.update(status='closed', updated_at=utc_now())
            atomic_json(status_path, state)


if __name__ == '__main__':
    main()
