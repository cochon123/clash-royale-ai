"""Bounded native replay-alignment pilot, with unverified output quarantined.

Unknown seed, initial hand, game version and levels prevent treating simulated
boards as replay ground truth. Never force cards, elixir, HP or legal targets.
"""
from __future__ import annotations

import argparse
from concurrent.futures import ProcessPoolExecutor
from collections import deque
import gzip
import hashlib
from itertools import groupby, permutations
import json
import os
from pathlib import Path
import selectors
import socket
import subprocess
import time

from .enrichment import atomic_json, duration, now
from .parser import parse_replay

SPELLS = {'arrows', 'barbarian-barrel', 'fireball', 'goblin-barrel', 'lightning',
          'poison', 'rocket', 'the-log', 'tornado', 'zap'}


class Engine:
    def __init__(self, binary, checkpoint, log):
        self.log = log.open('ab')
        self.proc = subprocess.Popen([str(binary), '--checkpoint', str(checkpoint)],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     stderr=self.log, bufsize=0)
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.proc.stdout, selectors.EVENT_READ)
        self.pending = b''

    def request(self, request):
        self.proc.stdin.write(json.dumps(request).encode() + b'\n')
        end = time.monotonic() + 45
        while b'\n' not in self.pending:
            left = end - time.monotonic()
            if left <= 0 or not self.selector.select(left):
                raise TimeoutError('native request exceeded 45 seconds')
            value = os.read(self.proc.stdout.fileno(), 65536)
            if not value:
                raise RuntimeError(f'native process exited: {self.proc.poll()}')
            self.pending += value
        line, self.pending = self.pending.split(b'\n', 1)
        reply = json.loads(line)
        if not reply.get('ok'):
            raise RuntimeError(str(reply.get('error', reply)))
        return reply['result']

    def close(self):
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait()
        self.proc.stdin.close()
        self.proc.stdout.close()
        self.selector.close()
        self.log.close()


def side_hypothesis(replay):
    # A hypothesis only: use the first non-spell deployment in each camp.
    first = {}
    for event in replay.events:
        if event.card not in SPELLS and event.y is not None and event.y != 16000:
            first.setdefault(event.side, int(event.y > 16000))
    if len(first) != 2 or first['team'] == first['opponent']:
        raise ValueError('cannot infer two opposite camps from deployment coordinates')
    return first


def consistent_queue(deck, plays):
    """One possible eight-card queue; the original queue is not identified."""
    for waiting in permutations(deck, 4):
        initial = [card for card in deck if card not in waiting]
        hand, queue = set(initial), deque(waiting)
        for card in plays:
            if card not in hand:
                break
            hand.remove(card)
            hand.add(queue.popleft())
            queue.append(card)
        else:
            return initial + list(waiting)
    raise ValueError('deployment sequence has no consistent classic eight-card cycle')


def inferred_deck_inputs(engine, replay, mapping, seed):
    """Calibrate native shuffle with legal dummy plays, then set deck inputs.

    This uses original reset inputs, not memory edits or forced hand cards.
    The resulting arrangement is only one replay-consistent hypothesis.
    """
    sides = side_hypothesis(replay)
    decks, desired = [None, None], [None, None]
    for side, index in sides.items():
        decks[index] = [mapping[c] for c in replay.decks[side]]
        queue = consistent_queue(replay.decks[side], [e.card for e in replay.events if e.side == side])
        desired[index] = [mapping[c] for c in queue]
    reply = engine.request({'op': 'reset', 'deck0': decks[0], 'deck1': decks[1],
                            'seed': seed, 'max_ticks': 7300})
    initial = [[c['name'] for c in hand] for hand in reply['observation']['hands']]
    queues = [list(hand) for hand in initial]
    reply = engine.request({'op': 'step', 'ticks': 100, 'actions': []})
    for _ in range(4):
        for side in (0, 1):
            while True:
                hand = reply['observation']['hands'][side]
                card = hand[0]
                if card['available'] and reply['observation']['elixir'][side] >= card['cost']:
                    break
                reply = engine.request({'op': 'step', 'ticks': 200, 'actions': []})
                if reply.get('terminated') or reply.get('truncated'):
                    raise ValueError('shuffle calibration ended before all queue cards were observed')
            mask = engine.request({'op': 'placement_mask', 'side': side, 'slot': 0})['mask']
            tile = next((i for i, legal in enumerate(mask) if legal), None)
            if tile is None:
                raise ValueError('no legal calibration placement')
            x = round((tile % 32 + .5) * 18000 / 32)
            y = (tile // 32 + .5) * 32000 / 18
            y = round(32000-y if side else y)
            before_hand = {value['name'] for value in hand}
            reply = engine.request({'op': 'step', 'ticks': 20, 'actions': [
                {'side': side, 'slot': 0, 'x': x, 'y': y}]})
            if not reply['actions'][0].get('accepted'):
                raise ValueError('original engine rejected calibration action')
            new_cards = {value['name'] for value in reply['observation']['hands'][side]} - before_hand
            for _ in range(12):
                if new_cards or reply.get('terminated') or reply.get('truncated'):
                    break
                reply = engine.request({'op': 'step', 'ticks': 10, 'actions': []})
                new_cards = {value['name'] for value in reply['observation']['hands'][side]} - before_hand
            drawn = next(iter(new_cards)) if len(new_cards) == 1 else None
            if drawn is None or drawn in queues[side]:
                raise ValueError('native draw order is not a distinct eight-card queue')
            queues[side].append(drawn)
    reordered = []
    for side in (0, 1):
        result = [None] * 8
        for position, actual in enumerate(queues[side]):
            result[decks[side].index(actual)] = desired[side][position]
        reordered.append(result)
    return reordered


def run_trace(engine, replay, mapping, seed, offset, *, save_frames=False, deck_override=None):
    sides = side_hypothesis(replay)
    decks = [None, None]
    for side, index in sides.items():
        decks[index] = [mapping[card] for card in replay.decks[side]]
    if deck_override is not None:
        decks = deck_override
    reply = engine.request({'op': 'reset', 'deck0': decks[0], 'deck1': decks[1],
                            'seed': seed, 'max_ticks': 7300, 'board': save_frames,
                            'reward_state': True})
    frames, accepted = [], 0
    base = reply['observation']['tick']
    current = base
    for replay_tick, group in groupby(enumerate(replay.events), key=lambda pair: pair[1].ticks):
        group = list(group)
        index = group[0][0]
        target = base + offset + replay_tick
        if target < current or len({e.side for _, e in group}) != len(group):
            return {'aligned': False, 'accepted': accepted, 'failure': 'same_tick_same_side_or_tick_overlap',
                    'event_index': index}, frames
        while current < target:
            try:
                reply = engine.request({'op': 'step', 'actions': [], 'ticks': min(200, target-current)})
            except RuntimeError as error:
                return {'aligned': False, 'accepted': accepted, 'failure': 'native_runtime_error',
                        'event_index': index, 'error': str(error)}, frames
            current = reply['observation']['tick']
            if reply.get('terminated') or reply.get('truncated'):
                return {'aligned': False, 'accepted': accepted, 'failure': 'native_ended_before_replay',
                        'event_index': index}, frames
        actions = []
        for index, event in group:
            native_side = sides[event.side]
            card = mapping[event.card]
            hand = reply['observation']['hands'][native_side]
            slot = next((value['slot'] for value in hand if value.get('name') == card and value.get('available')), None)
            if slot is None:
                return {'aligned': False, 'accepted': accepted, 'failure': 'recorded_card_not_in_native_hand',
                        'event_index': index, 'card': event.card, 'hand': hand}, frames
            if save_frames:
                frames.append({'event_index': index, 'replay_tick': event.ticks, 'native_tick': current,
                               'side': event.side, 'native_side': native_side, 'card': event.card,
                               'state_before_action': reply})
            actions.append({'side': native_side, 'slot': slot, 'x': event.x, 'y': event.y})
        try:
            reply = engine.request({'op': 'step', 'ticks': 1, 'actions': actions})
        except RuntimeError as error:
            return {'aligned': False, 'accepted': accepted, 'failure': 'native_runtime_error',
                    'event_index': group[0][0], 'error': str(error)}, frames
        for index, result in enumerate(reply['actions']):
            if not result.get('accepted'):
                return {'aligned': False, 'accepted': accepted, 'failure': result.get('rejection', 'action_rejected'),
                        'event_index': group[index][0], 'action_result': result}, frames
            accepted += 1
        current = reply['observation']['tick']
    return {'aligned': True, 'accepted': accepted, 'side_hypothesis': sides,
            'final_observation': reply['observation'], 'final_towers': reply.get('reward_state')}, frames


_engine = None


def worker_engine(args):
    global _engine
    if _engine is None or _engine.proc.poll() is not None:
        if _engine:
            _engine.close()
        _engine = Engine(args.binary, args.checkpoint, args.run_dir / f'native-worker-{os.getpid()}.log')
        from multiprocessing.util import Finalize
        Finalize(None, _engine.close, exitpriority=10)
    return _engine


def attempt(job):
    args, candidate, mapping = job
    battle = candidate['battle_id']
    started = time.monotonic()
    result = {'battle_id': battle, 'file': candidate['file'], 'source_sha256': candidate['sha256'],
              'verified': False, 'training_usable': False, 'outcome': 'alignment_failed', 'attempts': []}
    engine = None
    try:
        if getattr(args, 'input_db', None):
            import sqlite3
            import zlib
            from .models import ParsedReplay, ReplayEvent
            with sqlite3.connect(args.input_db.as_uri() + '?mode=ro', uri=True) as db:
                packed_meta, payload = db.execute('SELECT metadata, payload FROM inputs WHERE file=?',
                                                 (candidate['file'],)).fetchone()
            if json.loads(packed_meta)['sha256'] != candidate['sha256']:
                raise ValueError('packed candidate identity mismatch')
            value = json.loads(zlib.decompress(payload))
            value['events'] = [ReplayEvent(**event) for event in value['events']]
            replay = ParsedReplay(**value)
        else:
            source = args.source / candidate['file']
            if hashlib.sha256(source.read_bytes()).hexdigest() != candidate['sha256']:
                raise ValueError('candidate source hash changed')
            replay = parse_replay(source)
        engine = worker_engine(args)
        try:
            inferred = inferred_deck_inputs(engine, replay, mapping, 1)
        except (ValueError, RuntimeError) as error:
            inferred = None
            result['initial_order_inference_error'] = str(error)
        # Reuse the loaded engine across declared seed/clock/queue hypotheses.
        # None is a recovery of the original match configuration.
        hypotheses = [(1, offset, inferred) for offset in (0, 100)] if inferred else []
        hypotheses += [(seed, offset, None) for seed in range(1, args.seeds + 1) for offset in (0, 100)]
        best = None
        for seed, offset, deck_inputs in hypotheses:
            value, _ = run_trace(engine, replay, mapping, seed, offset, deck_override=deck_inputs)
            metadata = {'seed_hypothesis': seed, 'tick_offset_hypothesis': offset,
                        'deck_order_hypothesis': 'cycle_consistent' if deck_inputs else 'source_deck_order'}
            result['attempts'].append({**metadata, **value})
            if best is None or value['accepted'] > best[0]['accepted']:
                best = value, metadata, deck_inputs
            if value['aligned']:
                break
            if value.get('failure') == 'native_runtime_error':
                result['error'] = value.get('error')
                result['outcome'] = 'native_error'
                break
            if time.monotonic() - started > args.battle_timeout:
                result['outcome'] = 'hypothesis_budget_exhausted'
                break
        if best and best[0]['accepted']:
            baseline, metadata, deck_inputs = best
            if getattr(engine, 'proc', None) and engine.proc.poll() is not None:
                engine = worker_engine(args)
            value, frames = run_trace(engine, replay, mapping, metadata['seed_hypothesis'],
                                     metadata['tick_offset_hypothesis'], save_frames=True,
                                     deck_override=deck_inputs)
            if any(value.get(k) != baseline.get(k) for k in ('aligned', 'accepted', 'event_index', 'failure')):
                raise RuntimeError('board export changed deployment alignment')
            frames = frames[:value['accepted']]
            output = args.run_dir / 'quarantine' / f'{battle}.json.gz'
            temporary = output.with_suffix('.tmp')
            with gzip.open(temporary, 'wt') as f:
                json.dump({'format': 'native-replay-unverified-v1', 'verified': False,
                           'training_usable': False,
                           'alignment_complete': value['aligned'] and not candidate.get('stop_reason'),
                           'supported_prefix_aligned': value['aligned'],
                           'stop_reason': candidate.get('stop_reason'), 'stop_tick': candidate.get('stop_tick'),
                           **metadata, 'deck_input_hypothesis': deck_inputs,
                           'source_sha256': candidate['sha256'],
                           'limitations': ['old recovered build', 'unknown original seed',
                                          'unverified original levels/evolutions',
                                          'unverified initial hand and clock alignment',
                                          'accepted command does not prove executed deployment'],
                           'frames': frames, 'result': value}, f)
            temporary.replace(output)
            result['outcome'] = ('simulated_unverified' if value['aligned'] and not candidate.get('stop_reason')
                                 else 'partial_unverified')
            result['frames'] = len(frames)
            result['output'] = str(output)
    except Exception as error:
        result['outcome'] = 'native_error'
        result['error'] = f'{type(error).__name__}: {error}'
        if isinstance(error, TimeoutError) and engine:
            engine.close()
    finally:
        result['seconds'] = time.monotonic() - started
    return result


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for key in ('source', 'run-dir', 'binary', 'checkpoint', 'decks', 'audit-dir'):
        p.add_argument('--' + key, type=Path, required=True)
    p.add_argument('--workers', type=int, default=1)
    p.add_argument('--seeds', type=int, default=4)
    p.add_argument('--battle-timeout', type=float, default=90)
    p.add_argument('--limit', type=int)
    args = p.parse_args()
    if min(args.workers, args.seeds) < 1 or args.battle_timeout <= 0 or (args.limit is not None and args.limit < 1):
        p.error('workers, seeds, timeout and limit must be positive')
    for key in ('source', 'run_dir', 'binary', 'checkpoint', 'decks', 'audit_dir'):
        setattr(args, key, getattr(args, key).resolve())
    args.run_dir.mkdir(parents=True, exist_ok=True)
    (args.run_dir / 'quarantine').mkdir(exist_ok=True)
    import fcntl
    lock = (args.run_dir / '.native.lock').open('w')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    candidates = [json.loads(line) for line in (args.audit_dir / 'candidates.jsonl').read_text().splitlines()]
    if args.limit:
        candidates = candidates[:args.limit]
    mapping = {card: native for deck in json.loads(args.decks.read_text())['decks']
               for card, native in zip(deck['cards'], deck['native_cards'])}
    contract = {'format': 'native-enrichment-alignment-v1', 'candidates': candidates,
                'binary_sha256': hashlib.sha256(args.binary.read_bytes()).hexdigest(),
                'seeds': args.seeds, 'clock_offsets': [0, 100], 'board_export': True,
                'initial_order_inference': 'classic cycle consistency and native shuffle calibration',
                'workers': args.workers, 'battle_timeout': args.battle_timeout,
                'source_elf_sha256': hashlib.sha256(Path(os.environ['CR_SOURCE_ELF']).read_bytes()).hexdigest(),
                'checkpoint_sha256': hashlib.sha256((args.checkpoint / 'checkpoint.json').read_bytes()).hexdigest(),
                'worker_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    manifest = args.run_dir / 'run-manifest.json'
    if manifest.exists() and json.loads(manifest.read_text()) != contract:
        raise RuntimeError('Alignment configuration changed; use a new run directory')
    atomic_json(manifest, contract)
    results = []
    for file in (args.run_dir / 'results').glob('*.json'):
        results.append(json.loads(file.read_text()))
    (args.run_dir / 'results').mkdir(exist_ok=True)
    seen = {row['battle_id'] for row in results}
    started, clock = now(), time.monotonic()
    initial_done = len(results)
    last_publish = 0

    def publish(final=False):
        nonlocal last_publish
        elapsed = time.monotonic() - clock
        speed = (len(results) - initial_done) / elapsed if elapsed else 0
        eta = (len(candidates) - len(results)) / speed if speed else None
        simulated = sum(row['outcome'] == 'simulated_unverified' for row in results)
        partial = sum(row['outcome'] == 'partial_unverified' for row in results)
        progress = {'done': len(results), 'total': len(candidates), 'candidate': len(candidates),
                    'rejected': sum(row['outcome'] != 'simulated_unverified' for row in results),
                    'enriched': 0, 'simulated_unverified': simulated, 'attempted': len(results),
                    'partial_unverified': partial, 'state_rows_exported': sum(row.get('frames', 0) for row in results),
                    'eta_seconds': None if final else eta, 'files_per_second': speed, 'workers': args.workers}
        state = {'kind': 'data enrichment', 'host': socket.gethostname(),
                 'status': 'completed' if final else 'running', 'started_at': started, 'updated_at': now(),
                 'phase': 'Alignment pilot finished' if final else 'Testing native replay alignment',
                 'progress': progress, 'elapsed_seconds': elapsed,
                 'scope': 'Bounded seed/clock hypotheses on classic-card candidates. Exported boards remain quarantined and unverified.',
                 'output': str(args.run_dir)}
        if final:
            state['completed_at'] = now()
            state['validation'] = {
                'status': 'unverified', 'training_usable': False,
                'message': ('Original seed/hand, levels, evolutions and game-version parity are unverified. '
                            'No output is approved for training; extend coverage before processing the rest of the corpus.')}
        atomic_json(args.run_dir / 'state.json', state)
        with (args.run_dir / 'progress.jsonl').open('a') as f:
            f.write(json.dumps({'at': now(), **progress}) + '\n')
        print(f"{len(results):>4}/{len(candidates):<4} {simulated:>9} {partial:>7} {progress['state_rows_exported']:>7} {progress['rejected']:>8} "
              f"{speed * 60:>9.2f} {duration(eta):>12}  {state['phase']}", flush=True)
        if final:
            atomic_json(args.run_dir / 'summary.json', {**state, 'outcomes': results})
        last_publish = time.monotonic()
    print('DONE/TOTAL  COMPLETE PARTIAL    ROWS REJECTED ITEMS/MIN          ETA  PHASE', flush=True)
    publish()
    with ProcessPoolExecutor(max_workers=args.workers) as pool:
        tasks = {pool.submit(attempt, (args, row, mapping)) for row in candidates if row['battle_id'] not in seen}
        while tasks:
            from concurrent.futures import wait, FIRST_COMPLETED
            completed, tasks = wait(tasks, timeout=3, return_when=FIRST_COMPLETED)
            for task in completed:
                value = task.result()
                atomic_json(args.run_dir / 'results' / (value['battle_id'] + '.json'), value)
                results.append(value)
            if completed or time.monotonic() - last_publish > 3:
                publish()
    publish(final=True)


if __name__ == '__main__':
    main()
