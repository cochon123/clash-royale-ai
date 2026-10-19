"""Local live dashboard for an explicitly launched Banana training run."""
from __future__ import annotations
import argparse
from collections import deque
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import threading
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]


def serve(run_dir: Path, host: str, port: int):
    run_dir = run_dir.resolve()
    history_cache = {'stamp': None, 'history': []}
    history_lock = threading.Lock()
    def full_history():
        """Rebuild run-wide curves from genuine completed training events."""
        file = run_dir / 'progress.jsonl'
        if not file.exists():
            return []
        stat = file.stat()
        stamp = (stat.st_mtime_ns, stat.st_size)
        with history_lock:
            if stamp != history_cache['stamp']:
                windows = {key: deque(maxlen=20) for key in ('hog', 'mortar')}
                history = []
                for line in file.read_text().splitlines():
                    try:
                        row = json.loads(line)
                    except ValueError:
                        continue  # An append may still be in progress.
                    if row.get('event') != 'game' or row.get('truncated') or row.get('outcome') is None:
                        continue
                    if not str(row.get('kind', '')).startswith('training vs frozen'):
                        continue
                    key = {'banana-hog-1':'hog', 'banana-mortar-1':'mortar'}.get(row.get('model'))
                    if key is None:
                        continue
                    windows[key].append((float(row['outcome']) + 1) / 2)
                    history.append({'game': row['game'], **{k:sum(v)/len(v) if v else None for k,v in windows.items()}})
                history_cache.update(stamp=stamp, history=history)
            return history_cache['history']
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass
        def reply(self, data: bytes, mime: str, status=200):
            self.send_response(status)
            self.send_header('Content-Type', mime)
            self.send_header('Content-Length', str(len(data)))
            self.send_header('Cache-Control', 'no-store')
            self.end_headers()
            self.wfile.write(data)
        def do_GET(self):
            path = urlparse(self.path).path
            if path == '/':
                self.reply((ROOT / 'rl/banana_dashboard.html').read_bytes(), 'text/html; charset=utf-8')
            elif path == '/report':
                self.reply((ROOT / 'rl/banana_report.html').read_bytes(), 'text/html; charset=utf-8')
            elif path == '/api/state':
                try:
                    state = json.loads((run_dir / 'state.json').read_text())
                    state['stop_requested'] = (run_dir / 'STOP').exists()
                    history = full_history()
                    if history:
                        state['history'] = history
                    self.reply(json.dumps(state, allow_nan=False).encode(), 'application/json')
                except (OSError, ValueError) as exc:
                    self.reply(json.dumps({'status':'preparing','message':str(exc)}).encode(), 'application/json')
            elif path == '/api/health':
                self.reply(json.dumps({'ok': True, 'run_dir': str(run_dir)}).encode(), 'application/json')
            elif path == '/api/log':
                file = run_dir / 'train.log'
                data = file.read_bytes()[-48000:] if file.exists() else b'Preparing run.\n'
                self.reply(data, 'text/plain; charset=utf-8')
            else:
                self.reply(b'Not found', 'text/plain', 404)
        def do_POST(self):
            if urlparse(self.path).path == '/api/stop':
                run_dir.mkdir(parents=True, exist_ok=True)
                (run_dir / 'STOP').touch()
                self.reply(b'{"ok":true}', 'application/json')
            else:
                self.reply(b'Not found', 'text/plain', 404)
    server = ThreadingHTTPServer((host, port), Handler)
    print(f'Banana dashboard http://{host}:{port} — {run_dir}', flush=True)
    server.serve_forever()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run-dir', type=Path, required=True)
    p.add_argument('--host', default='127.0.0.1')
    p.add_argument('--port', type=int, default=8780)
    a = p.parse_args()
    serve(a.run_dir, a.host, a.port)

if __name__ == '__main__':
    main()
