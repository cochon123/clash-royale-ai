"""Bounded, defensive filesystem readers. This module never mutates project data."""
from __future__ import annotations

import csv
import io
import json
import math
import os
import re
import shutil
import subprocess
import threading
import time
from collections import deque
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from pathlib import Path
from urllib.request import build_opener, ProxyHandler

from webapps.common import catalog, paths

OLD_ROOT = '/home/cochon/Documents/mini-project/clash-rust'
MAX_JSON = 12 * 1024 * 1024
CACHE = {}
CACHE_LOCK = threading.RLock()
LIVE = {'sampled_at': None, 'processes': [], 'units': [], 'errors': ['First observation pending'],
        'process_scan_ok': False, 'unit_scan_ok': False, 'services': [], 'hardware': {}}
STOP = threading.Event()


def now():
    return datetime.now(timezone.utc).isoformat()


def stamp(value):
    try:
        if isinstance(value, (int, float)) and math.isfinite(value):
            return float(value)
        dt = datetime.fromisoformat(str(value).replace('Z', '+00:00'))
        return dt.replace(tzinfo=dt.tzinfo or timezone.utc).timestamp()
    except (ValueError, TypeError, OverflowError, OSError):
        return None


def number(value):
    return value if isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) else None


def mapped(value):
    """Normalize historical absolute paths in both display text and metadata."""
    if isinstance(value, str):
        return value.replace(OLD_ROOT + '/', str(paths.ENGINE) + '/')
    if isinstance(value, dict):
        return {k: mapped(v) for k, v in value.items()}
    if isinstance(value, list):
        return [mapped(v) for v in value]
    if isinstance(value, float) and not math.isfinite(value):
        return None
    return value


def source(path):
    return paths.relative(path)


def entries(path):
    try:
        return sorted(path.iterdir(), key=lambda p: p.name)
    except OSError:
        return []


def info(path):
    result = {'path': source(path), 'present': False, 'bytes': None, 'modified_at': None}
    try:
        s = path.stat()
        if path.is_file():
            result.update(present=True, bytes=s.st_size,
                          modified_at=datetime.fromtimestamp(s.st_mtime, timezone.utc).isoformat())
    except (OSError, ValueError):
        pass
    return result


def read_json(path, default=None):
    try:
        with path.open('rb') as h:
            data = h.read(MAX_JSON + 1)
        if len(data) > MAX_JSON:
            return default
        return mapped(json.loads(data))
    except (OSError, ValueError, TypeError, RecursionError):
        return default


def obj(path):
    data = read_json(path, {})
    return data if isinstance(data, dict) else {}


def compact(data, depth=0):
    """Keep metadata reviewable without sending large prediction/split arrays."""
    if depth > 6:
        return '[nested data omitted]'
    if isinstance(data, dict):
        return {k: compact(v, depth + 1) for k, v in list(data.items())[:100]}
    if isinstance(data, list):
        if len(data) > 40:
            return {'items': len(data), 'preview': [compact(v, depth + 1) for v in data[:3]], 'truncated': True}
        return [compact(v, depth + 1) for v in data]
    return data


def cached(key, ttl, fn):
    # One expensive reader per key; requests for another key may proceed independently.
    with CACHE_LOCK:
        entry = CACHE.setdefault(key, {'lock': threading.Lock(), 'at': 0, 'value': None})
    if entry['value'] is not None and time.monotonic() - entry['at'] < ttl:
        return entry['value']
    with entry['lock']:
        if entry['value'] is None or time.monotonic() - entry['at'] >= ttl:
            value = fn()
            entry.update(value=value, at=time.monotonic())
    return entry['value']


def small_tree(root, depth=2, limit=4000):
    """Only explicitly small artifact roots, with depth and entry budgets. No symlinks."""
    queue = [(root, 0)]
    seen = 0
    while queue and seen < limit:
        directory, level = queue.pop(0)
        for p in entries(directory):
            seen += 1
            if seen > limit:
                return
            try:
                if p.is_symlink():
                    continue
                if p.is_file():
                    yield p
                elif level < depth and p.name not in {'archive', '__pycache__', '.git', 'target', 'node_modules'}:
                    queue.append((p, level + 1))
            except OSError:
                continue


def report_links(name):
    aliases = {'policy_bc': 'policy_bc_v2', 'winner_predictor': 'winner_',
               'realism_scorer': 'realism_scorer', 'style_discriminator': 'style_discriminator'}
    normalized = aliases.get(name, name.replace('.', '_'))
    return [{'name': p.name, 'url': '/reports/' + p.name, 'path': source(p)}
            for p in entries(paths.REPORTS) if p.suffix == '.html'
            and (p.stem == normalized or p.stem.startswith(normalized + '_')
                 or (normalized == 'winner_' and p.stem.startswith(normalized)))]


METRIC_HELP = {
    'slot_top1': 'Fraction of replay actions where the predicted card matches the human card.',
    'card_acc': 'Fraction of replay actions with the correct card.',
    'slot_top3': 'Fraction where the human card is among the three highest predictions.',
    'joint_card_tile': 'Fraction with both the correct card and placement tile.',
    'tile_acc': 'Fraction of actions with the exact expected placement tile; definitions vary by model.',
    'zone_acc': 'Fraction placed in the expected coarse arena zone.',
    'timing_mae': 'Mean absolute timing error in seconds; lower is better.',
    'auc': 'Ability to rank positive examples above negatives; 0.5 is chance.',
    'accuracy': 'Fraction of correctly classified examples.',
    'loss': 'Training objective; lower is better within the same model and objective.',
    'brier': 'Squared probability error; lower is better.',
    'logloss': 'Probability prediction error; lower is better.',
    'delay_mae': 'Mean absolute delay error in seconds; lower is better.',
    'card': 'Fraction of examples with the correct predicted card.',
    'joint': 'Fraction with both the correct card and placement cell.',
    'cell': 'Fraction of examples with the correct placement cell.',
    'acc': 'Fraction of correctly classified examples.',
}


def extract_metrics(data, path):
    found = []
    def walk(value, prefix='', depth=0):
        if not isinstance(value, dict) or depth > 4:
            return
        for key, val in value.items():
            trail = f'{prefix}.{key}'.strip('.')
            if number(val) is not None and (key in {'card', 'joint', 'cell'} or any(t in key for t in ('acc', 'auc', 'slot_top', 'joint_card', 'timing_mae', 'delay_mae', 'loss', 'brier', 'battles_total', 'train_battles', 'parameters'))):
                if not any(t in trail for t in ('baseline', 'compute.loss_kwargs')):
                    help_text = next((v for k, v in METRIC_HELP.items() if k in key), 'Recorded metric; consult the source for its exact definition.')
                    found.append({'name': trail, 'value': val, 'source': source(path) + '#' + trail,
                                  'explanation': help_text})
            elif isinstance(val, dict) and key not in {'source_hashes', 'source_files_sha256', 'files', 'vocab'}:
                walk(val, trail, depth + 1)
    walk(data)
    return sorted(found, key=lambda m: ('test' not in m['name'].split('.'), not any(t in m['name'] for t in ('val.', 'validation.')), m['name']))[:24]


def model_scan():
    playable, errors = [], []
    try:
        for row in catalog.engine_models():
            row = mapped(row)
            row['source'] = source(paths.MODEL_REGISTRY)
            row['weights'] = info(paths.REPO / row['checkpoint']) if row.get('checkpoint') else None
            playable.append(row)
    except (OSError, ValueError, KeyError, TypeError, AttributeError) as exc:
        errors.append('Registry unavailable: ' + str(exc))
    offline = []
    for directory in entries(paths.MODELS):
        if not directory.is_dir() or directory.is_symlink():
            continue
        files = list(small_tree(directory))
        metadata = [p for p in files if p.suffix == '.json' and (
            p.name in {'report.json', 'manifest.json', 'config.json', 'status.json', 'suite_report.json', 'final_holdout.json', 'selected_recipe.json', 'final_selection.json', 'budget.json', 'contract.json', 'plan.json'}
            or p.name.endswith('_report.json'))]
        # Some auxiliary models keep their only numeric report in reports/.
        if not (directory / 'report.json').is_file():
            metadata += [paths.REPORTS / (Path(r['name']).stem + '.json')
                         for r in report_links(directory.name)
                         if (paths.REPORTS / (Path(r['name']).stem + '.json')).is_file()]
        metrics = []
        records = []
        for p in sorted(metadata, key=lambda p: (p.parent != directory, p.name != 'report.json', str(p)))[:70]:
            data = obj(p)
            metrics.extend(extract_metrics(data, p))
            records.append({'path': source(p), 'data': compact(data), 'readable': bool(data)})
        root_report = obj(directory / 'report.json')
        expected = []
        for record in records:
            checkpoint = record['data'].get('checkpoint') if isinstance(record['data'], dict) else None
            if isinstance(checkpoint, str):
                checkpoint_path = Path(checkpoint)
                if not checkpoint_path.is_absolute():
                    checkpoint_path = paths.REPO / checkpoint_path if checkpoint.startswith(('models/', 'engine/')) else directory / checkpoint_path
                if checkpoint_path.suffix in {'.pt', '.pkl', '.joblib', '.safetensors', '.onnx'}:
                    expected.append(checkpoint_path)
        cards = [p for p in files if p.suffix.lower() in {'.md', '.html'}]
        weights = [info(p) for p in sorted(set([p for p in files if p.suffix in {'.pt', '.pkl', '.joblib', '.safetensors', '.onnx'}] + expected))]
        offline.append({'id': directory.name, 'name': root_report.get('model_name', directory.name),
                        'source': source(directory), 'version': root_report.get('model_version'),
                        'created_at': root_report.get('created_at'),
                        'experimental': directory.name == 'policy_v9' or directory.name.startswith('controlled_'),
                        'family': 'policy' if directory.name.startswith('policy') else 'experiment' if directory.name.startswith('controlled') else 'auxiliary',
                        'metrics': metrics[:48], 'weights': weights, 'artifacts': records,
                        'lessons': root_report.get('lessons', []),
                        'cards': [{'name': str(p.relative_to(directory)), 'path': source(p),
                                   'url': '/api/document?path=' + source(p)} for p in cards],
                        'reports': report_links(directory.name),
                        'scan_scope': 'depth ≤ 2; archives excluded; at most 4,000 entries / 70 metadata files'})
    return {'sampled_at': now(), 'ttl_seconds': 60, 'playable': playable, 'offline': offline, 'errors': errors}


def models():
    return cached('models', 60, model_scan)


def command(args, timeout=1.0):
    try:
        p = subprocess.run(args, capture_output=True, text=True, timeout=timeout, check=False)
        return (p.stdout, None) if p.returncode == 0 else ('', (p.stderr or 'exit ' + str(p.returncode)).strip()[:300])
    except (OSError, subprocess.TimeoutExpired) as exc:
        return '', str(exc)[:300]


def process_category(args):
    """Classify the executed entry point, never module names in a test's arguments."""
    if not args:
        return None
    executable = Path(args[0]).name
    if executable in {'league_reference', 'rust_deck_rl', 'rust_league_rl', 'native-watch'}:
        return 'watch' if executable == 'native-watch' else 'engine'
    module = None
    if '-m' in args[1:]:
        index = args.index('-m')
        if '-c' not in args[1:index] and index + 1 < len(args):
            module = args[index + 1]
    elif len(args) > 1:
        script = next((a for a in args[1:] if not a.startswith('-')), '')
        if script.endswith('.py') and re.search(r'(?:^|/)(?:rl|cr_replay_pipeline)/', script):
            module = script.replace('/', '.').removesuffix('.py')
        elif Path(script).name == 'cr-replays':
            module = 'cr_replay_pipeline'
    if executable == 'cr-replays':
        module = 'cr_replay_pipeline'
    if not module or not re.search(r'(?:^|\.)(?:rl|cr_replay_pipeline)(?:\.|$)', module):
        return None
    entry = module.rsplit('.', 1)[-1]
    if entry.startswith('test_'):
        return None
    if 'watch' in entry:
        return 'watch'
    is_train = bool(re.search(r'(?:^|\.)rl\.', module) and any(t in entry for t in ('train', 'adaptive', 'finetune', 'ppo')))
    if 'cr_replay_pipeline' in module:
        is_train |= 'train' in entry or any(a in {'train-winner', 'train-style', 'train-realism'} or a.startswith('train-policy') for a in args[1:])
    return 'training' if is_train else 'pipeline'


def process_scan():
    rows, errors = [], []
    try:
        with os.scandir('/proc') as scan:
            pids = [e.name for e in scan if e.name.isdigit()]
    except OSError as exc:
        return {'processes': [], 'process_scan_ok': False, 'errors': [str(exc)]}
    own = {os.getpid(), os.getppid()}
    for pid in pids:
        if int(pid) in own:
            continue
        try:
            args = (Path('/proc') / pid / 'cmdline').read_bytes().decode(errors='replace').strip('\0').split('\0')
            if not args or not args[0]:
                continue
            category = process_category(args)
            if category:
                normalized = [mapped(a) for a in args]
                try:
                    cwd = mapped(str((Path('/proc') / pid / 'cwd').resolve(strict=True)))
                except OSError:
                    cwd = None
                rows.append({'pid': int(pid), 'category': category, 'command': ' '.join(normalized)[:2000],
                             'argv': normalized, 'cwd': cwd, 'source': f'/proc/{pid}/cmdline'})
        except PermissionError:
            errors.append(f'/proc/{pid}/cmdline inaccessible')
        except OSError:
            pass  # Processes may exit between listing and reading.
    return {'processes': rows, 'process_scan_ok': not errors, 'errors': errors[:5]}


def unit_scan():
    raw, error = command(['systemctl', '--user', 'list-units', '--all', '--no-pager', '--output=json'])
    try:
        data = json.loads(raw)
        if not isinstance(data, list):
            raise ValueError('Expected systemd unit list')
        rows = [{'name': r.get('unit'), 'active': r.get('active'), 'sub': r.get('sub'),
                 'description': r.get('description'), 'source': 'systemctl --user list-units --all --output=json'}
                for r in data if isinstance(r, dict) and isinstance(r.get('unit'), str) and re.search(r'banana|clash|replay|arena|royale|v9.*train|policy.*train', r['unit'], re.I)]
        return {'units': rows, 'unit_scan_ok': True}
    except (ValueError, TypeError, AttributeError):
        return {'units': [], 'unit_scan_ok': False, 'error': error or 'Invalid systemd JSON'}


def hardware():
    result = {'source': '/proc/loadavg; /proc/meminfo; statvfs repository; nvidia-smi', 'gpu': [], 'errors': []}
    try:
        result['load'] = [float(x) for x in Path('/proc/loadavg').read_text().split()[:3]]
        mem = {k: int(v.strip().split()[0]) * 1024 for k, v in (line.split(':', 1) for line in Path('/proc/meminfo').read_text().splitlines())}
        result['memory'] = {'total': mem.get('MemTotal'), 'available': mem.get('MemAvailable'),
                            'used': mem['MemTotal'] - mem['MemAvailable']}
    except (OSError, ValueError, KeyError) as exc:
        result['errors'].append(str(exc))
    try:
        disk = shutil.disk_usage(paths.REPO)
        result['disk'] = dict(total=disk.total, used=disk.used, free=disk.free)
    except OSError as exc:
        result['errors'].append(str(exc))
    raw, error = command(['nvidia-smi', '--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu', '--format=csv,noheader,nounits'])
    for row in csv.reader(io.StringIO(raw)):
        if len(row) == 5:
            def numeric(v):
                try:
                    n = float(v)
                    return n if math.isfinite(n) else None
                except ValueError:
                    return None
            result['gpu'].append(dict(name=row[0].strip(), utilization=numeric(row[1]), memory_used_mib=numeric(row[2]), memory_total_mib=numeric(row[3]), temperature_c=numeric(row[4])))
    if error:
        result['errors'].append(error)
    return result


def probe(name, port, endpoint='/'):
    url = f'http://127.0.0.1:{port}{endpoint}'
    row = {'name': name, 'port': port, 'source': url, 'url': f'http://127.0.0.1:{port}/', 'reachable': False, 'data': None}
    try:
        # Explicitly bypass environment proxies; all observations are local GET requests.
        with build_opener(ProxyHandler({})).open(url, timeout=.6) as response:
            raw = response.read(256 * 1024)
            row['reachable'] = response.status == 200
            if endpoint != '/':
                row['data'] = compact(mapped(json.loads(raw)))
    except (OSError, ValueError) as exc:
        row['error'] = str(exc)[:180]
    return row


def sample_once():
    with ThreadPoolExecutor(max_workers=7) as pool:
        futures = [pool.submit(fn, *args) for fn, args in [
            (process_scan, ()), (unit_scan, ()), (hardware, ()),
            (probe, ('Arena', 8800, '/api/health')), (probe, ('Arena matches', 8800, '/api/matches')),
            (probe, ('Replay collector', 8765, '/health')), (probe, ('Hog dashboard', 8783))]]
        results = [f.result() for f in futures]
    proc, units, hw, *services = results
    return {**proc, **units, 'hardware': hw, 'services': services, 'sampled_at': now()}


def start_sampler():
    def loop():
        while not STOP.is_set():
            try:
                value = sample_once()
                with CACHE_LOCK:
                    LIVE.clear()
                    LIVE.update(value)
            except Exception as exc:
                with CACHE_LOCK:
                    LIVE['errors'] = [f'Observation failed: {type(exc).__name__}: {exc}']
            STOP.wait(5)
    thread = threading.Thread(target=loop, name='lab-observer', daemon=True)
    thread.start()
    return thread


def live():
    with CACHE_LOCK:
        return dict(LIVE)


def observation_fresh(observation):
    sampled = stamp(observation.get('sampled_at'))
    return sampled is not None and 0 <= time.time() - sampled < 20


def run_dirs():
    result = []
    for directory in entries(paths.ENGINE_RUNS):
        if directory.is_symlink() or not directory.is_dir():
            continue
        if directory.name == 'banana-play':
            result.extend(p for p in entries(directory) if p.is_dir() and not p.is_symlink())
        else:
            result.append(directory)
    return result


def classify(name):
    if name.startswith('perf-') or 'train-bench' in name or 'benchmark' in name:
        return 'benchmark'
    if name.startswith('banana-play/') or 'ui-ai-match' in name:
        return 'play session'
    if 'watch' in name or 'visual' in name:
        return 'watch'
    if 'eval' in name or 'coverage' in name or 'preflight' in name:
        return 'eval'
    if 'banana' in name and name != 'banana-1':
        return 'training run'
    return 'other'


def run_status(state, stop, processes, units, run_id, current=None, directory=None):
    current = time.time() if current is None else current
    updated = stamp(state.get('updated_at'))
    deadline = stamp(state.get('deadline'))
    reported = str(state.get('status', 'unknown')).lower()
    directory = directory or paths.ENGINE_RUNS / run_id
    location = str(directory.resolve())
    def owns(p):
        args = p.get('argv', [])
        destinations = [a.partition('=')[2] for a in args if a.startswith('--run-dir=')]
        destinations.extend(args[i + 1] for i, a in enumerate(args[:-1]) if a == '--run-dir')
        # A trainer's seed checkpoint belongs to a different run. Only its output
        # directory proves ownership; missing run-dir still appears in Overview.
        candidates = destinations if p.get('category') == 'training' or destinations else args
        for value in candidates:
            if value.startswith('-'):
                continue
            target = Path(value)
            if not target.is_absolute():
                if not p.get('cwd'):
                    continue
                target = Path(p['cwd']) / target
            target = str(target.resolve())
            if target == location or (not destinations and target.startswith(location + '/')):
                return True
        return False
    matching = [p for p in processes if owns(p)]
    train_proc = [p for p in matching if p['category'] == 'training']
    active_units = [u for u in units if u.get('active') == 'active' and u.get('sub') in {'running', 'start'}
                    and u['name'] in {run_id.replace('/', '-') + '-train.service', run_id.replace('/', '-') + '.service'}]
    terminal = reported in {'completed', 'complete', 'finished', 'done', 'success', 'succeeded'}
    evidence = []
    if train_proc or active_units:
        status = 'running'
        evidence.append('Trainer process or active training unit observed')
        if stop:
            evidence.append('STOP exists: stop requested; trainer is still observed')
        if terminal:
            evidence.append('State reports completion but trainer is still observed')
    elif terminal:
        status = 'finished'
        evidence.append('State reports ' + reported)
    elif stop or reported in {'stopped', 'failed', 'error', 'cancelled', 'interrupted'}:
        status = 'stopped'
        evidence.append('STOP file present' if stop else 'State reports ' + reported)
    elif updated is not None and 0 <= current - updated < 120 and (deadline is None or deadline > current) and reported not in {'unknown', 'idle'}:
        status = 'running'
        evidence.append('Recent nonterminal state (<120s); no trainer process confirmed')
    elif state:
        status = 'stale'
        evidence.append('No active trainer confirmed; state is old, deadline passed, or status unknown')
    else:
        status = 'unknown'
        evidence.append('No readable state or confirmed trainer')
    return {'status': status, 'reported_status': reported, 'evidence': evidence,
            'processes': matching, 'units': active_units, 'state_age_seconds': max(0, current - updated) if updated else None}


def run_scan():
    rows = []
    for directory in run_dirs():
        state = obj(directory / 'state.json')
        launch = obj(directory / 'launch.json')
        manifest = obj(directory / 'manifest.json') or obj(directory / 'run-manifest.json')
        rows.append({'id': str(directory.relative_to(paths.ENGINE_RUNS)), 'source': source(directory),
                     'kind': classify(str(directory.relative_to(paths.ENGINE_RUNS))),
                     'state': state, 'launch': launch, 'manifest': compact(manifest),
                     'stop': (directory / 'STOP').is_file(),
                     'modified_at': info(directory / 'state.json')['modified_at']})
    return rows


def run_summary(row, observation):
    state = row['state']
    fresh = observation_fresh(observation)
    status = run_status(state, row['stop'], observation.get('processes', []) if fresh else [], observation.get('units', []) if fresh else [], row['id'])
    if not fresh:
        status['evidence'].append('Process/service observation is stale or pending; previous activity is not confirmation')
    counters = state.get('counters') if isinstance(state.get('counters'), dict) else {}
    if not counters:
        counters = {k: state.get(k) for k in ('wins', 'losses', 'draws')}
        counters['games'] = state.get('completed_games', state.get('games'))
        model_rows = state.get('models', {})
        if isinstance(model_rows, dict):
            stats = [r['stats'] for r in model_rows.values() if isinstance(r, dict) and isinstance(r.get('stats'), dict)]
            for k in ('wins', 'losses', 'draws'):
                vals = [s[k] for s in stats if number(s.get(k)) is not None]
                if vals:
                    counters[k] = sum(vals)
    evaluation = state.get('evaluation', {})
    history = evaluation.get('history', []) if isinstance(evaluation, dict) else []
    latest = next((h for h in reversed(history) if isinstance(h, dict) and number(h.get('score')) is not None), None) if isinstance(history, list) else None
    started = stamp(state.get('started_at', row['launch'].get('started_at')))
    ended = stamp(state.get('updated_at')) if status['status'] in {'finished', 'stopped', 'stale'} else time.time()
    elapsed = number(state.get('elapsed_seconds'))
    if elapsed is None and started and ended:
        elapsed = max(0, ended - started)
    deadline_value = state.get('deadline', row['launch'].get('deadline'))
    deadline = stamp(deadline_value)
    dashboard = row['launch'].get('dashboard')
    if not isinstance(dashboard, str) or not re.match(r'^http://(?:127\.0\.0\.1|localhost):\d+/', dashboard):
        dashboard = None
    return {k: row[k] for k in ('id', 'source', 'kind', 'stop', 'modified_at')} | status | {
        'phase': state.get('phase', state.get('message')), 'started_at': state.get('started_at', row['launch'].get('started_at')),
        'updated_at': state.get('updated_at'), 'deadline': deadline_value,
        'elapsed_seconds': elapsed, 'remaining_seconds': max(0, deadline - time.time()) if deadline and status['status'] == 'running' else None,
        'counters': counters, 'rates': state.get('rates', {}), 'latest_evaluation': latest, 'dashboard': dashboard,
        'error': state.get('error'), 'metric_source': row['source'] + '/state.json'}


def runs():
    observation = live()
    rows = [run_summary(r, observation) for r in cached('run-scan', 5, run_scan)]
    return {'sampled_at': now(), 'ttl_seconds': 5, 'runs': sorted(rows, key=lambda r: (r['status'] != 'running', r['id']), reverse=False)}


def jsonl(path, max_bytes=8 * 1024 * 1024, max_rows=12000):
    result = {'source': source(path), 'rows': [], 'malformed': 0, 'partial': False, 'bytes_read': 0}
    try:
        with path.open('rb') as h:
            size = os.fstat(h.fileno()).st_size
            offset = max(0, size - max_bytes)
            if offset:
                h.seek(offset)
                h.readline()  # Discard a partial first line.
            raw = h.read(max_bytes)
        result['bytes_read'] = len(raw)
        lines = raw.splitlines()
        result['partial'] = bool(offset or len(lines) > max_rows or len(raw) == max_bytes)
        for line in lines[-max_rows:]:
            if not line.strip():
                continue
            try:
                row = mapped(json.loads(line))
                if isinstance(row, dict):
                    result['rows'].append(row)
            except (ValueError, RecursionError):
                result['malformed'] += 1
    except OSError as exc:
        result['error'] = str(exc)
    return result


def series(name, points, src, explanation, x_label='time'):
    good = [[x, y] for x, y in points if number(x) is not None and number(y) is not None]
    return {'name': name, 'points': good, 'source': src, 'explanation': explanation, 'x_label': x_label}


SPEED_WINDOW_SECONDS = 600.


def outcome_speed_series(kind, records, src, window=SPEED_WINDOW_SECONDS):
    """Derivative of the cumulative outcome curves for one match group.

    At each completed game the trailing window counts wins and losses and both
    are expressed per minute. The denominator is the observed span inside the
    window (at least one minute), so the first minutes of a log are not diluted
    by time no games existed for. Draws are rare and excluded from the speeds.
    """
    events = deque()
    wins, losses, net = [], [], []
    for r in sorted(records, key=lambda r: stamp(r.get('at')) or 0):
        at, result = stamp(r.get('at')), r.get('result')
        if at is None or result not in ('W', 'L'):
            continue
        events.append((at, result))
        while events and at - events[0][0] > window:
            events.popleft()
        span = max(at - events[0][0], 60.)
        count_w = sum(1 for _, outcome in events if outcome == 'W')
        count_l = len(events) - count_w
        per_minute = 60. / span
        wins.append((at, count_w * per_minute))
        losses.append((at, count_l * per_minute))
        net.append((at, (count_w - count_l) * per_minute))
    return [series(kind + ' · wins/min', wins, src, 'Wins per minute over the trailing ten minutes; the derivative of the cumulative wins curve.'),
            series(kind + ' · losses/min', losses, src, 'Losses per minute over the trailing ten minutes; the derivative of the cumulative losses curve.'),
            series(kind + ' · net wins/min', net, src, 'Wins minus losses per minute over the trailing ten minutes. Above zero: results are improving; below zero: worsening.')]


def run_detail(run_id):
    def scan():
        row = next((r for r in cached('run-scan', 5, run_scan) if r['id'] == run_id), None)
        if not row:
            return None
        directory = paths.ENGINE_RUNS / run_id
        charts, logs = [], []
        state = row['state']
        evaluation = state.get('evaluation', {})
        history = evaluation.get('history', []) if isinstance(evaluation, dict) else []
        if isinstance(history, list):
            groups = {}
            for h in history:
                if not isinstance(h, dict):
                    continue
                group = str(h.get('bank', 'evaluation')) + ' · ' + str(h.get('mode', 'score'))
                groups.setdefault(group, []).append((stamp(h.get('at')), h.get('score')))
                greedy = h.get('greedy')
                if isinstance(greedy, dict):
                    groups.setdefault(str(greedy.get('bank', 'greedy')), []).append((stamp(greedy.get('at')), greedy.get('score')))
                groups.setdefault(group + ' parent', []).append((stamp(h.get('at')), h.get('parent_score')))
            charts.extend(series(k, v, row['source'] + '/state.json#evaluation.history', 'Evaluation score: wins plus half of draws, divided by games. Banks and inference modes are separate curves.') for k, v in groups.items())
        old_history = state.get('history', [])
        if isinstance(old_history, list):
            for key in ('hog', 'mortar'):
                charts.append(series(key + ' recent win rate', [(r.get('game'), r.get(key)) for r in old_history if isinstance(r, dict)], row['source'] + '/state.json#history', 'Recent training win rate, as recorded by the trainer; not a held-out evaluation.', 'game'))
        for filename in ('matches.jsonl', 'updates.jsonl', 'progress.jsonl', 'actions.jsonl'):
            p = directory / filename
            if not p.is_file():
                continue
            log = jsonl(p)
            data = log.pop('rows')
            log['records'] = len(data)
            log['recent'] = compact(data[-6:])
            logs.append(log)
            if filename == 'matches.jsonl':
                groups = {}
                for r in data:
                    kind = 'training' if r.get('kind') == 'train' else str(r.get('kind', 'unknown'))
                    groups.setdefault(kind, []).append(r)
                for kind, records in groups.items():
                    counts = {'W': 0, 'L': 0, 'D': 0}
                    points = {k: [] for k in counts}
                    rates, times = [], deque()
                    for r in sorted(records, key=lambda r: stamp(r.get('at')) or 0):
                        at = stamp(r.get('at'))
                        if at is None:
                            continue
                        if r.get('result') in counts:
                            counts[r['result']] += 1
                        for k in counts:
                            points[k].append((at, counts[k]))
                        times.append(at)
                        while times and at - times[0] > 60:
                            times.popleft()
                        rates.append((at, len(times)))
                    for k, pts in points.items():
                        charts.append(series(kind + ' · ' + {'W':'wins','L':'losses','D':'draws'}[k], pts, source(p), 'Cumulative outcomes in the loaded log window; training and evaluation are separate.'))
                    charts.extend(outcome_speed_series(kind, records, source(p)))
                    charts.append(series(kind + ' games/min', rates, source(p), 'Completed games in the trailing 60 seconds of wall time.'))
            else:
                groups = {}
                for record in data:
                    model_name = str(record.get('model', 'trainer'))
                    groups.setdefault(model_name, []).append(record)
                for model_name, records in groups.items():
                    for key in ('loss', 'policy_loss', 'value_loss', 'entropy', 'approx_kl', 'parent_kl', 'completed_games', 'games_per_minute'):
                        pts = [(stamp(r.get('at')), r.get(key)) for r in records]
                        if any(number(y) is not None for x, y in pts):
                            charts.append(series(model_name + ' · ' + key, pts, source(p), 'Recorded trainer metric for this specialist. Compare values within this curve; units differ between metrics.'))
                    if filename == 'progress.jsonl':
                        games = [r for r in records if r.get('event') in {'game', 'game_completed', 'match'}]
                        counts = {'wins': 0, 'losses': 0, 'draws': 0}
                        outcome_points = {k: [] for k in counts}
                        times, rates = deque(), []
                        for record in sorted(games, key=lambda r: stamp(r.get('at')) or 0):
                            at = stamp(record.get('at'))
                            if at is None:
                                continue
                            outcome = number(record.get('outcome'))
                            if outcome is not None:
                                counts['wins' if outcome > 0 else 'losses' if outcome < 0 else 'draws'] += 1
                                for key in counts:
                                    outcome_points[key].append((at, counts[key]))
                            times.append(at)
                            while times and at - times[0] > 60:
                                times.popleft()
                            rates.append((at, len(times)))
                        for key, pts in outcome_points.items():
                            charts.append(series(model_name + ' · ' + key, pts, source(p), 'Cumulative specialist outcomes in the loaded progress log window.'))
                        if rates:
                            charts.append(series(model_name + ' games/min', rates, source(p), 'Completed specialist game events in the trailing 60 seconds of wall time.'))
        artifacts = [{'path': source(p), 'data': compact(obj(p))} for p in entries(directory)
                     if p.name in {'manifest.json', 'launch.json', 'operator-notes.json', 'run-manifest.json', 'summary.json', 'result.json'}]
        return {'row': row, 'charts': [c for c in charts if c['points']], 'logs': logs, 'artifacts': artifacts,
                'weights': [info(p) for p in small_tree(directory / 'models') if p.suffix == '.pt'],
                'files': [info(p) for p in entries(directory) if p.is_file()]}
    detail = cached('run:' + run_id, 5, scan)
    if detail is None:
        return None
    return {'sampled_at': now(), 'summary': run_summary(detail['row'], live()),
            'state': compact(detail['row']['state']), **{k: v for k, v in detail.items() if k != 'row'}}


def engine_scan():
    bins = [paths.ENGINE / 'native-engine/target/release' / n for n in ('native-watch', 'league_reference', 'deck-setup-probe', 'deck_reference', 'league_validation_reference')]
    bins += [paths.ENGINE / 'rust-engine/target' / build / n for build in ('pgo', 'release') for n in ('rust_league_rl', 'rust_deck_rl')]
    validations = [{'path': source(p), 'data': compact(obj(p)), 'metrics': extract_metrics(obj(p), p)}
                   for p in entries(paths.ENGINE / 'rust-engine') if p.suffix == '.json' and 'validation' in p.name]
    errors = []
    try:
        decks, known = catalog.presets(), catalog.cards()
    except (OSError, ValueError, KeyError, TypeError, AttributeError) as exc:
        decks, known = [], {}
        errors.append(str(exc))
    return {'sampled_at': now(), 'ttl_seconds': 60, 'binaries': [info(p) for p in bins],
            'validations': validations, 'presets': decks, 'cards': known, 'errors': errors,
            'preset_source': source(paths.PRESET_DECKS),
            'components': [{'name': n, 'source': source(paths.ENGINE / p), 'description': d} for n, p, d in [
                ('Reference engine', 'native-engine', 'Original ARM64 engine running through Unicorn emulation.'),
                ('Fast engine', 'rust-engine', 'Statically recompiled engine for training. Validation applies to recorded paths and decks.'),
                ('Spectator', 'native-spectator', 'Desktop game viewer.'), ('Reinforcement learning', 'rl', 'Banana policies, training loops and evaluation code.')]]}


def data_scan():
    # Count direct files with scandir, never recurse into raw, v9_cache or engine/analysis.
    count, total, error = 0, 0, None
    try:
        with os.scandir(paths.DATA / 'raw') as scan:
            for e in scan:
                if e.is_file(follow_symlinks=False):
                    count += 1
                    total += e.stat(follow_symlinks=False).st_size
    except OSError as exc:
        error = str(exc)
        count, total = None, None  # Never report a partial count as exact.
    folders = []
    for p in entries(paths.DATA):
        if p.is_dir():
            folders.append({'name': p.name, 'source': source(p), 'recursive_size': None,
                            'note': 'Direct file count only; no recursive scan.' if p.name == 'raw' else 'Contents not walked.'})
    logs = [info(p) for p in entries(paths.LOGS) if p.is_file()]
    reports = [{'name': p.name, 'path': source(p), 'url': '/reports/' + p.name}
               for p in entries(paths.REPORTS) if p.suffix == '.html']
    return {'sampled_at': now(), 'ttl_seconds': 300, 'raw': {'files': count, 'bytes': total, 'source': 'data/raw/', 'error': error, 'scope': 'Direct regular files, no symlinks; cached for five minutes.'},
            'folders': folders, 'logs': logs, 'reports': reports,
            'note': 'v9_cache and engine/analysis are never traversed. Directory sizes are unknown rather than estimated.'}


def docs_scan():
    docs = []
    for root in (paths.DOCS, paths.ENGINE / 'docs'):
        for p in small_tree(root, depth=2):
            if p.suffix == '.md':
                docs.append({'name': p.stem.replace('-', ' ').replace('_', ' '), 'path': source(p),
                             'group': source(root), 'modified_at': info(p)['modified_at']})
    return {'sampled_at': now(), 'ttl_seconds': 60, 'documents': docs}


def overview():
    observation = live()
    run_rows = runs()['runs']
    running = [r for r in run_rows if r['status'] == 'running' and r['kind'] == 'training run']
    trainers = [p for p in observation.get('processes', []) if p['category'] == 'training']
    active_train_units = [u for u in observation.get('units', []) if u.get('active') == 'active' and u.get('sub') == 'running' and re.search(r'train.*\.service$', u.get('name', ''))]
    fresh = observation_fresh(observation)
    status = 'running' if running or (fresh and (trainers or active_train_units)) else 'idle' if fresh and observation.get('process_scan_ok') and observation.get('unit_scan_ok') else 'unknown'
    return {'sampled_at': observation.get('sampled_at'), 'refresh_seconds': 5, 'training_status': status,
            'status_explanation': 'Trainer processes, active training units and recent run state are combined. Engine worker or dashboard processes alone do not prove training.',
            'source': '/proc/*/cmdline; systemd user units; engine/runs/*/state.json; STOP files',
            'active_runs': running, 'recent_runs': sorted(run_rows, key=lambda r: r.get('updated_at') or '', reverse=True)[:5],
            **observation}
