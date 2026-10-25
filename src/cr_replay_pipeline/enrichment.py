"""Resumable compatibility preflight for native replay enrichment.

Eligibility is a gate, not a simulation or a claim of accurate board labels.
Unknown initial hands, seeds, levels, evolutions and version parity still need
alignment checks even for candidates composed of the validated classic cards.
"""
from __future__ import annotations

import argparse
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import signal
import socket
import sqlite3
import time

from .parser import ReplayParseError, parse_replay


def now():
    return datetime.now(timezone.utc).isoformat()


def atomic_json(path, value):
    temporary = path.with_suffix('.tmp')
    temporary.write_text(json.dumps(value, indent=2) + '\n')
    temporary.replace(path)


def inspect_replay(job):
    filename, supported, presets = job
    path = Path(filename)
    raw = path.read_bytes()
    record = {'file': path.name, 'sha256': hashlib.sha256(raw).hexdigest()}
    try:
        replay = parse_replay(path)
    except ReplayParseError as error:
        return record | {'outcome': 'rejected', 'reasons': ['parse_error'], 'error': str(error)}
    decks = replay.decks
    cards = set(card for deck in decks.values() for card in deck)
    unknown = sorted(cards - set(supported))
    reasons = []
    if any(len(decks[side]) != 8 for side in ('team', 'opponent')):
        reasons.append('incomplete_deck')
    if unknown:
        reasons.append('cards_outside_validated_set')
    abilities = sum(event.event_type == 'ability_activation' for event in replay.events)
    if abilities:
        reasons.append('unsupported_ability_activation')
    if not replay.events:
        reasons.append('no_events')
    if any(event.event_type == 'card_play' and (event.x is None or event.y is None)
           for event in replay.events):
        reasons.append('missing_deployment_coordinates')
    preset_pair = all(frozenset(decks[side]) in presets for side in ('team', 'opponent'))
    return record | {
        'battle_id': replay.battle_id, 'content_hash': replay.content_hash,
        'outcome': 'rejected' if reasons else 'candidate', 'reasons': reasons,
        'unsupported_cards': unknown, 'ability_activations': abilities,
        'events': len(replay.events), 'preset_pair': preset_pair,
        'decks': decks, 'crowns': replay.crowns,
    }


def duration(seconds):
    if seconds is None:
        return 'estimating'
    return f'{int(seconds)//3600:02d}:{int(seconds)%3600//60:02d}:{int(seconds)%60:02d}'


def audit(args):
    run = args.run_dir.resolve()
    run.mkdir(parents=True, exist_ok=True)
    import fcntl
    lock = (run / '.lock').open('w')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    config = json.loads(args.decks.read_text())
    supported = sorted({card for deck in config['decks'] for card in deck['cards']})
    presets = {frozenset(deck['cards']) for deck in config['decks']}
    files = sorted(args.source.glob('*.json'))
    contract = {
        'format': 'native-enrichment-preflight-v1', 'source': str(args.source.resolve()),
        'worker_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'files': len(files), 'validated_cards': supported,
        'decks_sha256': hashlib.sha256(args.decks.read_bytes()).hexdigest(),
        'file_inventory_sha256': hashlib.sha256('\n'.join(
            f'{p.name}:{p.stat().st_size}:{p.stat().st_mtime_ns}' for p in files).encode()).hexdigest(),
    }
    manifest = run / 'manifest.json'
    if manifest.exists() and json.loads(manifest.read_text()) != contract:
        raise RuntimeError('Input contract changed; use a new run directory')
    atomic_json(manifest, contract)
    db = sqlite3.connect(run / 'audit.sqlite3')
    db.execute('CREATE TABLE IF NOT EXISTS records (file TEXT PRIMARY KEY, result TEXT NOT NULL)')
    existing = {row[0] for row in db.execute('SELECT file FROM records')}
    pending = [p for p in files if p.name not in existing]
    counts, reasons, unsupported = Counter(), Counter(), Counter()
    events = 0
    for (value,) in db.execute('SELECT result FROM records'):
        row = json.loads(value)
        counts[row['outcome']] += 1
        reasons.update(row['reasons'])
        unsupported.update(row.get('unsupported_cards', []))
        events += row.get('events', 0)
    started = now()
    previous = json.loads((run / 'state.json').read_text()) if (run / 'state.json').exists() else {}
    first_started = previous.get('started_at', started)
    clock = time.monotonic()
    stop = False
    def requested_stop(*unused):
        nonlocal stop
        stop = True
    signal.signal(signal.SIGTERM, requested_stop)
    signal.signal(signal.SIGINT, requested_stop)
    last_write = 0
    initial_done = sum(counts.values())

    def publish(status='running', final=False):
        nonlocal last_write
        elapsed = time.monotonic() - clock
        processed = sum(counts.values())
        speed = (processed - initial_done) / elapsed if elapsed else 0
        eta = (len(files) - processed) / speed if speed else None
        state = {
            'kind': 'data enrichment', 'host': socket.gethostname(), 'status': status,
            'phase': 'Compatibility audit complete' if final else 'Checking replay compatibility',
            'started_at': first_started, 'updated_at': now(), 'elapsed_seconds': elapsed,
            'progress': {'done': processed, 'total': len(files), 'candidate': counts['candidate'],
                         'rejected': counts['rejected'], 'enriched': 0,
                         'events_scanned': events, 'eta_seconds': 0 if final else eta,
                         'files_per_second': speed, 'workers': args.workers},
            'rejection_reasons': dict(reasons), 'unsupported_cards': dict(unsupported.most_common()),
            'scope': 'Compatibility gate only; no simulated states have been generated.',
            'next_step': 'Validate replay alignment before simulation' if counts['candidate'] else
                         'Extend and validate harness coverage before board-state enrichment',
            'output': str(run),
        }
        if final:
            state['completed_at'] = now()
            state['phase'] = ('Candidates require alignment validation' if counts['candidate'] else
                              'Blocked: no replays within validated card/ability scope')
            state['blocker'] = state['next_step']
        atomic_json(run / 'state.json', state)
        with (run / 'progress.jsonl').open('a') as log:
            log.write(json.dumps({'at': now(), **state['progress']}) + '\n')
        print(f"{processed:>7}/{len(files):<7} {counts['candidate']:>9} {counts['rejected']:>9} "
              f"{speed:>8.1f} {duration(eta):>12}  {state['phase']}", flush=True)
        last_write = time.monotonic()
    print('   DONE / TOTAL   CANDIDATE  REJECTED  FILES/S          ETA  PHASE', flush=True)
    publish()
    try:
        with ProcessPoolExecutor(max_workers=args.workers) as pool:
            jobs = ((str(path), supported, presets) for path in pending)
            for row in pool.map(inspect_replay, jobs, chunksize=32):
                db.execute('INSERT INTO records VALUES (?, ?)', (row['file'], json.dumps(row)))
                counts[row['outcome']] += 1
                reasons.update(row['reasons'])
                unsupported.update(row.get('unsupported_cards', []))
                events += row.get('events', 0)
                if time.monotonic() - last_write >= 3:
                    db.commit()
                    publish()
                if stop or (run / 'STOP').exists():
                    break
        db.commit()
        if stop or (run / 'STOP').exists():
            publish('stopped')
        else:
            with (run / 'candidates.jsonl').open('w') as out:
                for (value,) in db.execute('SELECT result FROM records ORDER BY file'):
                    if json.loads(value)['outcome'] == 'candidate':
                        out.write(value + '\n')
            publish('blocked', final=True)
            atomic_json(run / 'summary.json', json.loads((run / 'state.json').read_text()))
    except BaseException as error:
        db.rollback()
        publish('failed')
        state = json.loads((run / 'state.json').read_text())
        state['error'] = f'{type(error).__name__}: {error}'
        atomic_json(run / 'state.json', state)
        raise
    finally:
        db.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--run-dir', type=Path, required=True)
    parser.add_argument('--decks', type=Path, required=True)
    parser.add_argument('--workers', type=int, default=2)
    args = parser.parse_args()
    if args.workers < 1 or not args.source.is_dir():
        parser.error('source must be a directory and workers must be positive')
    audit(args)


if __name__ == '__main__':
    main()
