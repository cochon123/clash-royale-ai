"""Pack the whole replay corpus for an explicitly experimental native sweep."""
from __future__ import annotations

import argparse
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
import hashlib
import json
from pathlib import Path
import sqlite3
import time
import zlib

from .enrichment import atomic_json, now
from .parser import parse_replay


def exact_mapping(aliases, catalog):
    # Only normalize the catalog's documented separator, never infer similar names.
    mapping = {row['id'].replace('_', '-'): row['sc_name'] for row in catalog}
    mapping.update({row['policy_id']: row['internal_id'] for row in aliases['cards']})
    spell_names = {row['sc_name'] for row in catalog
                   if row.get('source', {}).get('member', '').endswith('/spells_other.csv')}
    spells = sorted(slug for slug, native in mapping.items() if native in spell_names)
    return mapping, spells


def pack_one(job):
    source, audited, mapping = job
    path = Path(source) / audited['file']
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    if digest != audited['sha256']:
        raise ValueError('Source changed since audit: ' + path.name)
    meta = {'file': path.name, 'sha256': digest, 'battle_id': path.stem + '-' + digest[:8],
            'original_battle_id': audited.get('battle_id'), 'content_hash': audited.get('content_hash')}
    decks = audited.get('decks', {})
    if not decks or any(len(decks.get(side, [])) != 8 for side in ('team', 'opponent')):
        return meta | {'outcome': 'skipped', 'skip_reason': 'invalid_or_incomplete_input'}, None
    missing = sorted({card for deck in decks.values() for card in deck} - mapping.keys())
    if missing:
        return meta | {'outcome': 'skipped', 'skip_reason': 'no_exact_card_mapping', 'unmapped_cards': missing}, None
    replay = parse_replay(path)
    cutoff = next((event.ticks for event in replay.events if event.event_type != 'card_play'), None)
    original_events = len(replay.events)
    if cutoff is not None:
        replay.events = [event for event in replay.events if event.ticks < cutoff]
        meta.update(stop_reason='unsupported_ability_activation', stop_tick=cutoff)
    if not replay.events:
        return meta | {'outcome': 'skipped', 'skip_reason': 'no_deployments_before_unsupported_event'}, None
    if any(event.card not in replay.decks[event.side] or event.x is None or event.y is None
           for event in replay.events):
        return meta | {'outcome': 'skipped', 'skip_reason': 'invalid_deployment_or_deck'}, None
    meta.update(outcome='candidate', original_event_count=original_events,
                prefix_event_count=len(replay.events), preset_pair=False)
    return meta, zlib.compress(json.dumps(replay.to_dict(), separators=(',', ':')).encode(), level=1)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for name in ('source', 'audit-db', 'aliases', 'catalog', 'out'):
        p.add_argument('--' + name, required=True, type=Path)
    p.add_argument('--workers', type=int, default=4)
    args = p.parse_args()
    if args.out.exists():
        raise RuntimeError('Refusing to replace an existing input package')
    args.out.parent.mkdir(parents=True, exist_ok=True)
    mapping, spells = exact_mapping(json.loads(args.aliases.read_text()), json.loads(args.catalog.read_text()))
    with sqlite3.connect(args.audit_db) as db:
        rows = [json.loads(row[0]) for row in db.execute('SELECT result FROM records ORDER BY file')]
    out = sqlite3.connect(args.out)
    out.execute('CREATE TABLE inputs (file TEXT PRIMARY KEY, metadata TEXT NOT NULL, payload BLOB)')
    out.execute('CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)')
    config = {'format': 'experimental-native-corpus-v1', 'mapping': mapping, 'spells': spells,
              'total': len(rows), 'created_at': now(), 'source': str(args.source.resolve()),
              'audit_sha256': hashlib.sha256(args.audit_db.read_bytes()).hexdigest(),
              'packer_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    counts, seen = Counter(), set()
    start = time.monotonic()
    # Bounded batches keep packing independent of corpus size in memory/IPC.
    with ProcessPoolExecutor(max_workers=args.workers) as pool:
        for offset in range(0, len(rows), 512):
            jobs = ((str(args.source), row, mapping) for row in rows[offset:offset+512])
            for meta, payload in pool.map(pack_one, jobs, chunksize=16):
                if payload and meta.get('content_hash') in seen:
                    meta.update(outcome='skipped', skip_reason='duplicate_content')
                    payload = None
                if payload:
                    seen.add(meta['content_hash'])
                counts[meta['outcome']] += 1
                if meta.get('skip_reason'):
                    counts[meta['skip_reason']] += 1
                if meta.get('stop_reason') and payload:
                    counts['ability_limited_prefix'] += 1
                out.execute('INSERT INTO inputs VALUES (?, ?, ?)', (meta['file'], json.dumps(meta), payload))
            out.commit()
            done = min(offset+512, len(rows))
            elapsed = time.monotonic()-start
            print(f'{done}/{len(rows)} candidate={counts["candidate"]} skipped={counts["skipped"]} '
                  f'ETA={(len(rows)-done)*elapsed/done:.0f}s', flush=True)
    config['counts'] = dict(counts)
    out.execute('INSERT INTO config VALUES (?, ?)', ('config', json.dumps(config)))
    out.commit()
    out.close()
    atomic_json(args.out.with_suffix('.json'), config | {
        'sha256': hashlib.sha256(args.out.read_bytes()).hexdigest(),
        'bytes': args.out.stat().st_size, 'elapsed_seconds': time.monotonic()-start})


if __name__ == '__main__':
    main()
