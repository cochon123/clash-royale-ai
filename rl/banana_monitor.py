"""Stream work done and ETA from Banana League's atomic state file."""
import argparse
import datetime as dt
import json
import time
from pathlib import Path


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir', type=Path, default=Path(__file__).resolve().parents[1] / 'runs/banana-league-20261019')
    p.add_argument('--interval', type=float, default=5.)
    p.add_argument('--once', action='store_true')
    args = p.parse_args()
    print('TIME      STATUS          ETA       GAMES   DECISIONS  UPDATES  HOG W/L/D  MORTAR W/L/D  GAMES/MIN', flush=True)
    while True:
        try:
            s = json.loads((args.run_dir / 'state.json').read_text())
            elapsed = float(s.get('elapsed_seconds', 0))
            remaining = max(0, int(float(s.get('duration_seconds', 10800)) - elapsed))
            if s.get('deadline') and s.get('status') == 'running':
                remaining = max(0, int(dt.datetime.fromisoformat(s['deadline']).timestamp() - time.time()))
            eta = f'{remaining//3600:02}:{remaining//60%60:02}:{remaining%60:02}'
            models = s.get('models', {})
            def score(key):
                t = models.get(key, {}).get('stats', {})
                return '/'.join(str(t.get(k, 0)) for k in ('wins', 'losses', 'draws'))
            updates = sum(m.get('updates', 0) for m in models.values())
            games = int(s.get('completed_games', 0))
            rate = games * 60 / elapsed if elapsed > 0 else 0
            print(f"{dt.datetime.now():%H:%M:%S}  {s.get('status','unknown'):<14}  {eta}  {games:>5}  {int(s.get('decision_steps',0)):>10}  {updates:>7}  {score('hog'):>9}  {score('mortar'):>12}  {rate:>9.2f}", flush=True)
            if s.get('error'):
                print('ERROR: ' + str(s['error']), flush=True)
            if args.once or s.get('status') in ('completed', 'failed', 'stopped'):
                break
        except (OSError, ValueError) as exc:
            print(f'Waiting for state: {exc}', flush=True)
            if args.once:
                raise SystemExit(1)
        time.sleep(max(.5, args.interval))


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        pass
