"""Watch a separate frozen-4.4.2 match through the original-asset spectator."""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import subprocess
import time

import torch

from .banana_policy import BananaEpisode, BananaPolicy, _native_card
from .banana_train import BASE_CHECKPOINT, DECKS, JsonlEngine, atomic_json, utc_now

ROOT = Path(__file__).resolve().parents[1]


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir', type=Path, default=ROOT / 'runs/v442-custom-watch')
    p.add_argument('--engine-bin', type=Path, default=ROOT / 'native-engine/target/release/native-watch')
    p.add_argument('--viewer-bin', type=Path, default=ROOT / 'native-spectator/target/debug/clash-native-spectator')
    p.add_argument('--checkpoint', type=Path, default=ROOT / 'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state')
    p.add_argument('--deck', choices=['mortar', 'hog'], default='mortar')
    p.add_argument('--seed', type=int, default=44219)
    p.add_argument('--device', default='cpu')
    p.add_argument('--torch-threads', type=int, default=1)
    p.add_argument('--max-ticks', type=int, default=7300)
    p.add_argument('--decision-ticks', type=int, default=20)
    p.add_argument('--max-play-ticks', type=int, help='Bound a diagnostic capture without claiming a finished match')
    p.add_argument('--headless', action='store_true', help='Diagnostic frame/action capture without window or real-time pacing')
    a = p.parse_args()
    if not 1 <= a.decision_ticks <= 200 or a.torch_threads < 1 or a.max_ticks < 101:
        p.error('invalid tick/thread setting')
    a.run_dir = a.run_dir.resolve()
    a.run_dir.mkdir(parents=True, exist_ok=True)
    frame_path, control_path = a.run_dir / 'frame.json', a.run_dir / 'control.json'
    ready_path, status_path = a.run_dir / 'viewer-ready', a.run_dir / 'status.json'
    ready_path.unlink(missing_ok=True)
    control_path.unlink(missing_ok=True)
    torch.set_num_threads(a.torch_threads)
    torch.manual_seed(a.seed)
    # One immutable actor computes each seat from its own causal perspective.
    # There is no persistent recurrent state shared between model.act calls.
    policy = BananaPolicy(device=a.device, seed=a.seed, specialization='frozen-4.4.2')
    policy.requires_grad_(False)
    policy.eval()
    engine = JsonlEngine(a.checkpoint, a.engine_bin)
    viewer = None
    actions_log = (a.run_dir / 'actions.jsonl').open('a', buffering=1)
    frames_log = (a.run_dir / 'frames.jsonl').open('a', buffering=1)
    manifest = {
        'created_at': utc_now(), 'controllers': ['frozen v4.4.2', 'frozen v4.4.2'],
        'deck': list(DECKS[a.deck]), 'source_checkpoint': str(BASE_CHECKPOINT),
        'source_checkpoint_sha256': hashlib.sha256(BASE_CHECKPOINT.read_bytes()).hexdigest(),
        'native_binary_sha256': hashlib.sha256(a.engine_bin.read_bytes()).hexdigest(),
        'device': a.device, 'seed': a.seed, 'ticks_per_second': 20,
        'decision_ticks': a.decision_ticks, 'learning': False,
        'note': 'Independent watched match. No optimizer, no RL-run files or weights are modified. Actions sample the frozen actor using the native adapter.',
    }
    atomic_json(a.run_dir / 'manifest.json', manifest)
    state = {'status': 'loading', 'updated_at': utc_now(), 'message': 'Loading the original engine and frozen 4.4.2 actor.'}
    atomic_json(status_path, state)
    episode = None
    obs = None
    frame = None
    base_tick = 0
    accepted = 0
    rejected = 0
    match_number = 0
    paused = False
    ended = False
    speed = 1.
    latest_serial = -1
    pending = []

    def publish(status):
        nonlocal state
        assert frame is not None and obs is not None
        frame['elapsed_ms'] = max(0, int(obs['tick']) - base_tick) * 50
        frame.update(runtime_status=status, runtime_error=state.get('error'),
                     controller_names=['v4.4.2', 'v4.4.2'], playback_speed=speed)
        atomic_json(frame_path, frame)
        frames_log.write(json.dumps(frame, separators=(',', ':')) + '\n')
        state.update(status=status, updated_at=utc_now(), tick=obs['tick'],
                     game_seconds=frame['elapsed_ms'] / 1000, accepted_plays=accepted,
                     rejected_plays=rejected, match=match_number,
                     winner_side=obs.get('winner_side'), terminated=obs.get('terminated'),
                     playback_speed=speed, paused=paused, learning=False)
        atomic_json(status_path, state)

    def reset():
        nonlocal episode, obs, frame, base_tick, accepted, rejected, match_number, paused, ended, pending
        match_number += 1
        r = engine.request({'op': 'reset', 'deck_preset': 'pixel9_mortar' if a.deck == 'mortar' else 'hog26',
                            'seed': a.seed + match_number - 1, 'max_ticks': a.max_ticks})
        decks = r.get('configured_decks')
        if not isinstance(decks, list) or len(decks) != 2:
            raise RuntimeError('Native reset omitted configured decks')
        for side in decks:
            cards = side.get('cards') if isinstance(side, dict) else side
            normalized = []
            for card in cards:
                compact = ''.join(ch.lower() for ch in card if ch.isalnum())
                normalized.append(policy.native_name_to_policy.get(compact) or _native_card(card, DECKS[a.deck]))
            if Counter(normalized) != Counter(DECKS[a.deck]):
                raise RuntimeError(f'Wrong native deck: {cards!r}')
        # The recovered fresh harness rejects inputs during its startup period.
        # Skip that phase before starting the visible, model-controlled clock.
        r = engine.request({'op': 'step', 'actions': [], 'ticks': 100})
        obs, frame = r['observation'], r['frame']
        if r.get('ticks_advanced') != 100:
            raise RuntimeError('Native startup did not advance 100 ticks')
        base_tick = int(obs['tick'])
        episode = BananaEpisode((DECKS[a.deck], DECKS[a.deck]), battle_id=f'watched-442-{match_number}')
        accepted = rejected = 0
        pending = [policy.act(episode, obs, side, 0., deterministic=False) for side in (0, 1)]
        paused = ended = False
        state.pop('error', None)
        state['message'] = 'Frozen 4.4.2 vs frozen 4.4.2 · no learning · normal speed'
        publish('waiting' if viewer is None and not a.headless else 'running')
        print(f'Match {match_number} ready: {a.deck}, both frozen4.4.2, source {manifest["source_checkpoint_sha256"][:12]}', flush=True)

    try:
        reset()
        if not a.headless:
            title = f'v4.4.2 vs v4.4.2 · {"Custom Mortar" if a.deck == "mortar" else "Hog 2.6"}'
            viewer = subprocess.Popen([str(a.viewer_bin), '--follow', str(frame_path), '--control-file', str(control_path),
                                       '--ready-file', str(ready_path), '--title', title], cwd=ROOT)
            wait_until = time.monotonic() + 90
            while not ready_path.exists():
                if viewer.poll() is not None:
                    raise RuntimeError(f'Spectator exited before showing match ({viewer.returncode})')
                if time.monotonic() > wait_until:
                    raise RuntimeError('Spectator window did not become ready')
                time.sleep(.02)
        publish('running')
        due = time.monotonic()
        print('Playing at 20 native ticks/second (1×). Space pause, R restart, Esc close.', flush=True)
        while True:
            if viewer is not None and viewer.poll() is not None:
                break
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
                if a.headless:
                    break
                time.sleep(.02)
                continue
            now = time.monotonic()
            if not a.headless and now < due:
                time.sleep(min(.01, due - now))
                continue
            elapsed_ticks = int(obs['tick']) - base_tick
            game_seconds = elapsed_ticks / 20
            sampled = []
            if elapsed_ticks % a.decision_ticks == 0:
                sampled = pending or [policy.act(episode, obs, side, game_seconds, deterministic=False) for side in (0, 1)]
                pending = []
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
                if result.get('accepted') is True:
                    accepted += 1
                    policy.record_action(episode, action, game_seconds, True)
                else:
                    rejected += 1
                actions_log.write(json.dumps({'match': match_number, 'at': utc_now(), 'game_seconds': game_seconds,
                                              'card': action.card, 'request': action.native_request(), 'result': result}) + '\n')
            obs, frame = r['observation'], r['frame']
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
        if viewer is not None and viewer.poll() is None:
            viewer.terminate()
            viewer.wait(timeout=10)
        actions_log.close()
        frames_log.close()
        if state.get('status') not in ('failed', 'diagnostic_complete'):
            state.update(status='closed', updated_at=utc_now())
            atomic_json(status_path, state)


if __name__ == '__main__':
    main()
