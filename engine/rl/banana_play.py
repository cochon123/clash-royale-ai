"""Play locally against a saved Banana actor with mouse and keyboard controls."""
from __future__ import annotations

import argparse
import datetime as dt
import os
from pathlib import Path
import subprocess

from .banana_train import ROOT
from .banana_watch import main as watch_main


def ensure_binary(path, project, binary):
    sources = [project / 'Cargo.toml', *(project / 'src').rglob('*.rs')]
    if project.name == 'native-spectator':
        sources.extend((ROOT / 'native-ui-viewer/src').rglob('*.rs'))
    if not path.exists() or any(p.stat().st_mtime > path.stat().st_mtime for p in sources):
        subprocess.run(['cargo', 'build', '--manifest-path', str(project / 'Cargo.toml'),
                        *(['--release'] if project.name == 'native-engine' else []), '--bin', binary],
                       cwd=ROOT, check=True)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--opponent', choices=['banana-1', 'banana-hog-1'], default='banana-1')
    p.add_argument('--deck', choices=['hog', 'log-bait', 'mortar'], default='hog', help='Your deck')
    p.add_argument('--ai-deck', choices=['hog', 'log-bait', 'mortar'], default='hog')
    p.add_argument('--seed', type=int)
    p.add_argument('--run-dir', type=Path)
    p.add_argument('--headless', action='store_true', help='Input-queue integration diagnostic without a window')
    p.add_argument('--max-play-ticks', type=int, help='Bound a diagnostic session')
    p.add_argument('--greedy', action='store_true', help='Use greedy AI actions instead of checkpoint sampling')
    a = p.parse_args()
    viewer = ROOT / 'native-spectator/target/debug/clash-native-spectator'
    engine = ROOT / 'native-engine/target/release/native-watch'
    ensure_binary(engine, ROOT / 'native-engine', 'native-watch')
    ensure_binary(viewer, ROOT / 'native-spectator', 'clash-native-spectator')
    stamp = dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    run_dir = a.run_dir or ROOT / 'runs/banana-play' / f'{stamp}-{os.getpid()}'
    checkpoint = ROOT / ('runs/banana-1/base.pt' if a.opponent == 'banana-1' else 'runs/banana-hog-1/models/final.pt')
    args = ['--human', '--deck-a', a.deck, '--deck-b', a.ai_deck, '--checkpoint-b', str(checkpoint),
            '--run-dir', str(run_dir), '--viewer-bin', str(viewer), '--engine-bin', str(engine)]
    if a.seed is not None:
        args.extend(['--seed', str(a.seed)])
    if a.headless:
        args.append('--headless')
    if a.max_play_ticks:
        args.extend(['--max-play-ticks', str(a.max_play_ticks)])
    if a.greedy:
        args.append('--greedy')
    print(f'Session: {run_dir}\nYou: {a.deck} | Opponent: {a.opponent} ({a.ai_deck})', flush=True)
    watch_main(args)


if __name__ == '__main__':
    main()
