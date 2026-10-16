#!/usr/bin/env python3
"""Publish the current research artifacts and a restorable dated data snapshot.

Requires huggingface_hub and an authenticated `hf auth login` session.
Staging uses hard links where possible; local source files are never removed.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

from huggingface_hub import HfApi, CommitOperationAdd, hf_hub_download
from tqdm.auto import tqdm

ROOT = Path(__file__).resolve().parents[1]
ARTIFACT_REPO = 'Cochon123/clash-royale-ai-artifacts'
DATA_REPO = 'Cochon123/clash-royale-replays'
DATA_DIRS = ('raw', 'metadata', 'cleaned-legacy', 'splits', 'tower_hp_runs',
             'board_oracle', 'phone_lab', 'v9_card_stats')
FROZEN_FILES = ('complete.json', 'contract.json', 'costs.json', 'metadata.jsonl',
                'split_manifest.json', 'vocab.json')


def sha256(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--date', default=datetime.now(timezone.utc).date().isoformat())
    args = parser.parse_args()
    stage = ROOT / '.artifact-publish' / args.date
    artifacts = stage / 'artifacts'
    artifacts.mkdir(parents=True, exist_ok=True)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    manifest = {'created_utc': datetime.now(timezone.utc).isoformat(), 'git_revision': revision,
                'git_dirty': bool(subprocess.check_output(['git', 'status', '--porcelain'], cwd=ROOT)),
                'files': []}
    for directory in ('models', 'reports', 'logs'):
        files = sorted(p for p in (ROOT / directory).rglob('*')
                       if p.is_file() and p.suffix != '.lock' and '.cache' not in p.parts)
        for path in tqdm(files, desc=f'Stage {directory}', unit='file'):
            rel = path.relative_to(ROOT)
            dest = artifacts / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            if dest.exists():
                dest.unlink()
            try:
                os.link(path, dest)
            except OSError:
                shutil.copy2(path, dest)
            manifest['files'].append({'path': str(rel), 'bytes': path.stat().st_size,
                                      'sha256': sha256(path)})
    (artifacts / 'snapshot_manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    (artifacts / 'README.md').write_text('''---
language: en
library_name: pytorch
tags:
- clash-royale
- behavior-cloning
- experimental
---
# Clash Royale AI — consolidated research artifacts

Code and navigation: https://github.com/cochon123/clash-royale-ai

This snapshot preserves all locally available model versions under `models/`,
including checkpoints, resumable runs, evaluation arrays and provenance archives.
`reports/` contains HTML and supporting evidence; `logs/` contains available logs.
`snapshot_manifest.json` records source Git revision and SHA-256 for each file.

The latest export is `models/policy_v9/final_model/`. **It is experimental and
not promoted.** Final card accuracy: 56.23%; joint card-and-cell accuracy: 12.76%.
The +1.55-point joint gain missed the required +2-point gate. Additional seeds
share a donor. These are offline metrics, not measured game win rates.
Read the export card, manifest and `reports/policy_v9.html` before using it.
The consumed final holdout is preserved as evidence, not a model-selection set.

```sh
hf download Cochon123/clash-royale-ai-artifacts --local-dir . \\
  --include 'models/**' 'reports/**' 'logs/**' 'snapshot_manifest.json'
```

Install the linked Python project and PyTorch for inference. Checkpoint/pickle
files require the matching project code and trusted deserialization. The data
snapshot is at https://huggingface.co/datasets/Cochon123/clash-royale-replays/tree/main/snapshots .
''')
    api = HfApi()
    api.create_repo(ARTIFACT_REPO, repo_type='model', exist_ok=True)
    commit = api.upload_folder(repo_id=ARTIFACT_REPO, folder_path=artifacts,
                               commit_message=f'Research snapshot {args.date} ({revision[:12]})')
    print('ARTIFACT_COMMIT', commit.oid, flush=True)
    paths = []
    for name in DATA_DIRS:
        paths.extend(p for p in (ROOT / 'data' / name).rglob('*') if p.is_file())
    paths.extend(p for p in [ROOT / 'data' / 'card_costs.json',
                            *(ROOT / 'data' / 'v9_cache' / name for name in FROZEN_FILES)] if p.is_file())
    paths = sorted(set(paths))
    archive = stage / f'data-{args.date}.tar.gz'
    data_manifest = {'created_utc': datetime.now(timezone.utc).isoformat(), 'git_revision': revision,
                     'artifact_commit': commit.oid, 'files': []}
    with tarfile.open(archive, 'w:gz', compresslevel=1) as tf:
        for path in tqdm(paths, desc='Archive data', unit='file'):
            rel = str(path.relative_to(ROOT))
            tf.add(path, arcname=rel, recursive=False)
            data_manifest['files'].append({'path': rel, 'bytes': path.stat().st_size,
                                           'sha256': sha256(path)})
    data_manifest['archive'] = {'name': archive.name, 'bytes': archive.stat().st_size,
                                'sha256': sha256(archive)}
    data_manifest['excluded'] = ['collector SQLite and leases', 'quarantine',
                                 'regenerable pickle caches', 'v9 observation shard arrays', 'secrets']
    metadata = stage / f'data-{args.date}.manifest.json'
    metadata.write_text(json.dumps(data_manifest, indent=2) + '\n')
    existing = Path(hf_hub_download(DATA_REPO, 'README.md', repo_type='dataset')).read_text()
    # Preserve the existing card (including its YAML metadata) as historical context.
    marker = '\n## Current complete data snapshot\n'
    existing = existing.split(marker)[0]
    card = stage / 'dataset_README.md'
    card.write_text(existing + marker + f'''
The current collection is `snapshots/{archive.name}` ({len(paths):,} files).
The older top-level `raw/` directory is a historical release. The archive contains
`data/` paths and restores at the project root. Its adjacent manifest records
per-file SHA-256, archive checksum, Git revision and model artifact revision.

Included: replay payloads, official metadata, cleaned legacy corpus, split IDs,
card costs, tower-HP frames/weak labels, device calibrations, board annotation
queue, frozen ghost stats, and v9 cache contracts/vocabulary/metadata.
Excluded: collector database, secrets, quarantine and regenerable binary caches.
Official metadata may not match historical replay battles. HP labels are weak
supervision; the board annotation queue does not imply verified labels.

```sh
hf download {DATA_REPO} --repo-type dataset \\
  --include 'snapshots/{archive.name}' 'snapshots/{metadata.name}' --local-dir /tmp/clash-royale-data
tar -xzf /tmp/clash-royale-data/snapshots/{archive.name}
```

Models/reports: https://huggingface.co/{ARTIFACT_REPO}
Code/navigation: https://github.com/cochon123/clash-royale-ai
''')
    result = api.create_commit(repo_id=DATA_REPO, repo_type='dataset',
        commit_message=f'Complete dated data snapshot {args.date}', operations=[
            CommitOperationAdd(path_in_repo=f'snapshots/{archive.name}', path_or_fileobj=archive),
            CommitOperationAdd(path_in_repo=f'snapshots/{metadata.name}', path_or_fileobj=metadata),
            CommitOperationAdd(path_in_repo='README.md', path_or_fileobj=card)])
    print('DATASET_COMMIT', result.oid, flush=True)
    receipt = {'date': args.date, 'git_revision': revision, 'artifact_repo': ARTIFACT_REPO,
               'artifact_commit': commit.oid, 'dataset_repo': DATA_REPO,
               'dataset_commit': result.oid, 'archive': data_manifest['archive'],
               'data_files': len(paths), 'artifact_files': len(manifest['files'])}
    receipt_path = stage / 'receipt.json'
    receipt_path.write_text(json.dumps(receipt, indent=2) + '\n')
    print('RECEIPT', receipt_path, flush=True)


if __name__ == '__main__':
    main()
