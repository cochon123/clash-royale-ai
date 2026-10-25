"""Resumable, bounded experimental native sweep of every packaged corpus file."""
from __future__ import annotations

import argparse
from collections import Counter
from concurrent.futures import ProcessPoolExecutor, wait, FIRST_COMPLETED
import hashlib
import json
from pathlib import Path
import signal
import socket
import sqlite3
import time

from . import native_enrichment as native
from .enrichment import atomic_json, duration, now


def worker(job):
    args, meta, mapping, spells = job
    native.SPELLS.update(spells)
    def timeout(*unused):
        raise TimeoutError('Full-sweep per-file wall-time limit exceeded')
    signal.signal(signal.SIGALRM, timeout)
    signal.setitimer(signal.ITIMER_REAL, args.battle_timeout + 90)
    try:
        return native.attempt((args, meta, mapping))
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for key in ('input-db', 'run-dir', 'binary', 'checkpoint'):
        parser.add_argument('--' + key, type=Path, required=True)
    parser.add_argument('--workers', type=int, default=2)
    parser.add_argument('--seeds', type=int, default=4)
    parser.add_argument('--battle-timeout', type=float, default=90)
    parser.add_argument('--limit', type=int)
    args = parser.parse_args()
    if min(args.workers, args.seeds) < 1 or args.battle_timeout <= 0 or (args.limit is not None and args.limit < 1):
        parser.error('workers, seeds, timeout and optional limit must be positive')
    for key in ('input_db', 'run_dir', 'binary', 'checkpoint'):
        setattr(args, key, getattr(args, key).resolve())
    args.source = args.input_db.parent
    run = args.run_dir
    run.mkdir(parents=True, exist_ok=True)
    (run / 'quarantine').mkdir(exist_ok=True)
    import fcntl
    lock = (run / '.full.lock').open('w')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    inputs = sqlite3.connect(args.input_db.as_uri() + '?mode=ro', uri=True)
    config = json.loads(inputs.execute("SELECT value FROM config WHERE key='config'").fetchone()[0])
    rows = [(file, json.loads(meta)) for file, meta in inputs.execute('SELECT file, metadata FROM inputs ORDER BY file')]
    if args.limit:
        # A smoke test samples native candidates, not the cheap rejected records.
        rows = [(file, meta) for file, meta in rows if meta['outcome'] == 'candidate'][:args.limit]
    contract = {'format': 'experimental-native-full-sweep-v1', 'input_sha256': hashlib.sha256(args.input_db.read_bytes()).hexdigest(),
                'binary_sha256': hashlib.sha256(args.binary.read_bytes()).hexdigest(),
                'native_worker_sha256': hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),
                'coordinator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                'workers': args.workers, 'seeds': args.seeds, 'limit': args.limit,
                'battle_timeout': args.battle_timeout, 'config': config}
    manifest = run / 'run-manifest.json'
    if manifest.exists() and json.loads(manifest.read_text()) != contract:
        raise RuntimeError('Sweep inputs or code changed; use a new run directory')
    atomic_json(manifest, contract)
    results = sqlite3.connect(run / 'results.sqlite3')
    results.execute('CREATE TABLE IF NOT EXISTS results (file TEXT PRIMARY KEY, outcome TEXT, frames INTEGER, seconds REAL, result TEXT)')
    results.execute('CREATE INDEX IF NOT EXISTS result_outcome ON results(outcome)')
    for file, meta in rows:
        if meta['outcome'] == 'skipped':
            results.execute('INSERT OR IGNORE INTO results VALUES (?, ?, 0, 0, ?)',
                            (file, 'skipped', json.dumps(meta)))
    results.commit()
    seen = {file for (file,) in results.execute('SELECT file FROM results')}
    candidates = sum(meta['outcome'] == 'candidate' for _, meta in rows)
    skipped = len(rows)-candidates
    previous = json.loads((run/'state.json').read_text()) if (run/'state.json').exists() else {}
    started = previous.get('started_at', now())
    prior_elapsed = previous.get('elapsed_seconds', 0)
    clock = time.monotonic()
    initial_native = len(seen)-skipped
    last_write = 0
    stop = False
    def requested_stop(*unused):
        nonlocal stop
        stop = True
    signal.signal(signal.SIGTERM, requested_stop)
    signal.signal(signal.SIGINT, requested_stop)
    errors = Counter(previous.get('error_counts', {}))

    def publish(status='running', error=None):
        nonlocal last_write
        groups = {outcome: (count, frames or 0) for outcome, count, frames in results.execute(
            'SELECT outcome, COUNT(*), SUM(frames) FROM results GROUP BY outcome')}
        count = lambda name: groups.get(name, (0, 0))[0]
        done = sum(value[0] for value in groups.values())
        native_done = done-skipped
        elapsed_this = time.monotonic()-clock
        speed = (native_done-initial_native)/elapsed_this if elapsed_this else 0
        eta = (candidates-native_done)/speed if speed else None
        full, partial = count('simulated_unverified'), count('partial_unverified')
        failed = native_done-full-partial
        progress = {'done': done, 'total': len(rows), 'candidate': candidates, 'attempted': native_done,
                    'preflight_skipped': skipped, 'failed': failed, 'enriched': 0,
                    'rejected': skipped+failed+partial, 'simulated_unverified': full,
                    'partial_unverified': partial, 'state_rows_exported': sum(value[1] for value in groups.values()),
                    'workers': args.workers, 'eta_seconds': eta if status == 'running' else 0,
                    'files_per_second': speed}
        state = {'kind': 'data enrichment', 'host': socket.gethostname(), 'status': status,
                 'started_at': started, 'updated_at': now(), 'elapsed_seconds': prior_elapsed+elapsed_this,
                 'phase': 'Full corpus experimental sweep' if status == 'running' else 'Full corpus sweep '+status,
                 'progress': progress, 'error_counts': dict(errors.most_common(20)),
                 'error_counts_scope': 'Top error messages; all per-file errors are retained in results.sqlite3.',
                 'scope': 'All corpus files accounted for. Exact mapped decks only; stop before unsupported abilities. Native coverage errors are recorded. All snapshots are unverified.',
                 'validation': {'status': 'unverified', 'training_usable': False,
                                'message': 'Experimental recovered-build simulations; original levels, evolutions, seed and current-game fidelity remain unverified.'}}
        if error:
            state['error'] = error
        if status == 'completed':
            state['completed_at'] = now()
        atomic_json(run/'state.json', state)
        with (run/'progress.jsonl').open('a') as log:
            log.write(json.dumps({'at': now(), **progress})+'\n')
        print(f'{done:>6}/{len(rows):<6} {native_done:>6}/{candidates:<6} '
              f'{full:>5} {partial:>7} {failed:>6} {skipped:>6} {progress["state_rows_exported"]:>8} '
              f'{duration(eta):>12}  {state["phase"]}', flush=True)
        if status == 'completed':
            atomic_json(run/'summary.json', state | {'outcome_counts': {k:v[0] for k,v in groups.items()}})
        last_write = time.monotonic()
    print(' DONE / TOTAL  NATIVE / TOTAL  FULL PARTIAL FAILED   SKIP     ROWS          ETA  PHASE', flush=True)
    publish()
    jobs = iter((file, meta) for file, meta in rows if file not in seen)
    try:
        with ProcessPoolExecutor(max_workers=args.workers) as pool:
            active = {}
            def fill():
                while not stop and len(active) < args.workers*2:
                    item = next(jobs, None)
                    if item is None:
                        break
                    file, meta = item
                    active[pool.submit(worker, (args, meta, config['mapping'], config['spells']))] = file
            fill()
            while active:
                completed, _ = wait(active, timeout=3, return_when=FIRST_COMPLETED)
                for task in completed:
                    file = active.pop(task)
                    value = task.result()
                    results.execute('INSERT INTO results VALUES (?, ?, ?, ?, ?)',
                                    (file, value['outcome'], value.get('frames', 0), value.get('seconds', 0), json.dumps(value)))
                    if value.get('error'):
                        errors[value['error'][:200]] += 1
                    results.commit()
                if stop or (run/'STOP').exists():
                    stop = True
                    publish('stopped')
                    for task in active:
                        task.cancel()
                    break
                if completed or time.monotonic()-last_write > 3:
                    publish()
                fill()
        if not stop:
            publish('completed')
    except BaseException as error:
        publish('failed', f'{type(error).__name__}: {error}')
        raise
    finally:
        results.close()
        inputs.close()
        lock.close()


if __name__ == '__main__':
    main()
