#!/usr/bin/env python3
"""Mirror small run artifacts from the execution host to the other Clash Lab."""
from pathlib import Path
import subprocess
import json
import socket

root = Path(__file__).resolve().parents[1] / 'engine/runs'
for run in ('replay-enrichment-home-20261024', 'replay-enrichment-full-20261024', 'banana-2', 'banana-2-e7-20261025', 'banana-battle-royal-eval-20261024', 'banana-battle-royal-e7-20261025'):
    destination = root / run
    destination.mkdir(parents=True, exist_ok=True)
    owner_file = destination / 'execution-owner.json'
    if run.startswith('banana-') and not owner_file.exists():
        continue
    local_owner = owner_file.is_file() and json.loads(owner_file.read_text()).get('host') == socket.gethostname()
    remote = 'cochonhome:projects/clash-royale-ai/engine/runs/' + run + '/'
    source, target = (str(destination)+'/', remote) if local_owner else (remote, str(destination)+'/')
    subprocess.run([
        'rsync', '-az', '--delay-updates', '--timeout=8',
        '-e', 'ssh -o BatchMode=yes -o ConnectTimeout=5',
        '--include=state.json', '--include=progress.jsonl', '--include=run-manifest.json',
        '--include=summary.json', '--include=operator-notes.json',
        '--include=training-manifest.json', '--include=metrics.json', '--exclude=*',
        source, target,
    ], check=True, timeout=12)
    if run.startswith('banana-2') and local_owner:
        reports = root.parents[1] / 'reports'
        subprocess.run(['rsync','-az','--delay-updates','--timeout=8','-e',
                        'ssh -o BatchMode=yes -o ConnectTimeout=5',
                        str(reports/'banana2.html'), str(reports/'banana2-live.json'),
                        'cochonhome:projects/clash-royale-ai/reports/'],check=True,timeout=12)
