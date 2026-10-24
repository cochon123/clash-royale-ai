"""Stream Crazy Hog training: work done, budget, and estimated time left.

    python -m rl.crazy_hog_monitor --run-dir runs/crazy-hog-2.6
    python -m rl.crazy_hog_monitor --run-dir runs/crazy-hog-2.6 --dashboard --port 8788
"""
from __future__ import annotations

import argparse
import json
import time
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from .crazy_hog_core import budget_percent, eta_seconds, format_eta

HEADER = (
    "TIME      STATUS      GAMES  BANK  DECISIONS  UPDATES  BUDGET    ETA       "
    "GAMES/MIN  DEC/S   HOG/G  CONNECT"
)


def _parse_time(value):
    if not value:
        return None
    text = str(value).replace("Z", "+00:00")
    try:
        parsed = datetime.fromisoformat(text)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.timestamp()


def progress_view(state: dict, now: float | None = None) -> dict:
    """Work done and time left. Prefers the trainer's progress block."""
    now = time.time() if now is None else float(now)
    progress = dict(state.get("progress") or {})
    counters = state.get("counters") or {}
    started = _parse_time(state.get("started_at"))
    deadline = _parse_time(state.get("deadline"))
    if "eta_seconds" not in progress and started is not None and deadline is not None:
        progress["eta_seconds"] = eta_seconds(now, started, deadline, str(state.get("status") or "running"))
    if "budget_percent" not in progress and started is not None and deadline is not None:
        progress["budget_percent"] = budget_percent(now, started, deadline)
    progress.setdefault("games", int(counters.get("games") or 0))
    progress.setdefault("bank_episodes", int(counters.get("bank_episodes") or 0))
    progress.setdefault("decisions", int(counters.get("decisions") or 0))
    progress.setdefault("updates", int(counters.get("updates") or 0))
    progress.setdefault("episodes", int(progress["games"]) + int(progress["bank_episodes"]))
    progress.setdefault("eta_seconds", 0)
    progress.setdefault("budget_percent", 0.0)
    return progress


def render_row(state: dict, now: float | None = None) -> str:
    progress = progress_view(state, now)
    rates = state.get("rates") or {}
    extra = state.get("rates_extra") or {}
    hog = state.get("hog") or {}
    games_per_minute = extra.get("episodes_per_minute", rates.get("games_per_minute", 0.0))
    decisions_per_second = extra.get("decisions_per_second", rates.get("decisions_per_second", 0.0))
    hog_per_game = hog.get("hog_plays_per_game")
    connect = hog.get("connect_rate")
    hog_text = f"{hog_per_game:.2f}" if isinstance(hog_per_game, (int, float)) else "  -  "
    connect_text = f"{connect:.0%}" if isinstance(connect, (int, float)) else "  - "
    stamp = datetime.now().strftime("%H:%M:%S")
    return (
        f"{stamp}  {str(state.get('status') or 'unknown'):<10}  "
        f"{int(progress['games']):>5}  {int(progress['bank_episodes']):>4}  "
        f"{int(progress['decisions']):>9}  {int(progress['updates']):>7}  "
        f"{float(progress['budget_percent']):>6.1f}%  {format_eta(int(progress['eta_seconds']))}  "
        f"{float(games_per_minute):>8.2f}  {float(decisions_per_second):>6.2f}  "
        f"{hog_text:>6}  {connect_text:>7}"
    )


DASHBOARD = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Crazy Hog live</title>
<style>
  :root { color-scheme: dark; --bg:#10140f; --ink:#f3f0e6; --muted:#b7b1a3; --line:#2c3328; --good:#d6ff4a; --bad:#ff8b7b; }
  body { margin:0; font:15px/1.45 ui-sans-serif, system-ui, sans-serif; background:var(--bg); color:var(--ink); }
  main { width:min(1100px, calc(100% - 28px)); margin:auto; padding:28px 0 60px; }
  h1 { font-size:clamp(2rem, 4vw, 3rem); letter-spacing:-.04em; margin:0 0 8px; }
  .grid { display:grid; grid-template-columns:repeat(auto-fit, minmax(160px, 1fr)); gap:10px; margin:18px 0; }
  .card { border:1px solid var(--line); border-radius:14px; padding:12px 14px; background:#171c15; }
  .card b { display:block; font-size:1.4rem; margin-top:4px; }
  .muted { color:var(--muted); }
  table { width:100%; border-collapse:collapse; font-variant-numeric:tabular-nums; }
  th, td { text-align:left; padding:6px 8px; border-bottom:1px solid var(--line); }
  .bad { color:var(--bad); } .good { color:var(--good); }
</style>
</head>
<body>
<main>
  <p class="muted">Crazy Hog 2.6 · live · read only</p>
  <h1 id="title">Waiting for state</h1>
  <p id="phase" class="muted"></p>
  <section class="grid" id="cards"></section>
  <h2>Matchups</h2>
  <table><thead><tr><th>Deck</th><th>Train</th><th>Score</th></tr></thead><tbody id="matchups"></tbody></table>
  <h2>Recent</h2>
  <table><thead><tr><th>When</th><th>Deck</th><th>Controller</th><th>Result</th><th>Hog</th></tr></thead><tbody id="recent"></tbody></table>
</main>
<script>
const $ = id => document.getElementById(id);
function card(label, value){ return '<div class="card"><span class="muted">'+label+'</span><b>'+value+'</b></div>'; }
function pct(v){ return v==null ? '—' : (100*v).toFixed(1)+'%'; }
async function tick(){
  let state;
  try { state = await (await fetch('/api/state', {cache:'no-store'})).json(); }
  catch (err) { $('phase').textContent = String(err); return; }
  const p = state.progress || {};
  const hog = state.hog || {};
  const eta = p.eta_seconds||0;
  const hh = String(Math.floor(eta/3600)).padStart(2,'0');
  const mm = String(Math.floor(eta/60)%60).padStart(2,'0');
  const ss = String(eta%60).padStart(2,'0');
  $('title').textContent = (state.name||'Crazy Hog') + ' · ' + (state.status||'');
  $('phase').textContent = state.phase || '';
  $('cards').innerHTML = [
    card('Games', (p.games||0) + ' + ' + (p.bank_episodes||0) + ' bank'),
    card('Decisions', p.decisions||0),
    card('Updates', p.updates||0),
    card('Budget', ((p.budget_percent)||0).toFixed(1)+'%'),
    card('Time left', hh+':'+mm+':'+ss),
    card('Hog / game', hog.hog_plays_per_game==null?'—':Number(hog.hog_plays_per_game).toFixed(2)),
    card('Connect', pct(hog.connect_rate)),
    card('Decisions/s', ((state.rates_extra||state.rates||{}).decisions_per_second||0).toFixed(2))
  ].join('');
  $('matchups').innerHTML = (state.matchups||[]).map(row => {
    const train = row.train||{};
    return '<tr><td>'+row.deck_name+'</td><td>'+(train.wins||0)+'/'+(train.losses||0)+'/'+(train.draws||0)+'</td><td>'+pct(train.score)+'</td></tr>';
  }).join('');
  $('recent').innerHTML = (state.recent_matches||[]).slice(0,12).map(row =>
    '<tr><td>'+(row.at||'')+'</td><td>'+(row.deck_name||'')+'</td><td>'+(row.controller||'')+'</td><td class="'+(row.result==='W'?'good':row.result==='L'?'bad':'')+'">'+(row.result||'')+'</td><td>'+(row.hog_plays==null?'':row.hog_plays)+'</td></tr>'
  ).join('');
}
tick(); setInterval(tick, 2000);
</script>
</body>
</html>
"""


def read_state(run_dir: Path) -> dict:
    path = run_dir / "state.json"
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return {}
    return value if isinstance(value, dict) else {}


def serve(run_dir: Path, host: str, port: int) -> None:
    directory = run_dir.resolve()

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            if self.path in {"/", "/index.html"}:
                body = DASHBOARD.encode()
                kind = "text/html; charset=utf-8"
            elif self.path == "/api/state":
                body = json.dumps(read_state(directory)).encode()
                kind = "application/json"
            else:
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, fmt, *args):
            return

    server = ThreadingHTTPServer((host, port), Handler)
    print(f"Crazy Hog dashboard at http://{host}:{port}/", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--interval", type=float, default=5.0)
    parser.add_argument("--once", action="store_true")
    parser.add_argument("--dashboard", action="store_true", help="Serve a local live page instead of streaming rows")
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8788)
    args = parser.parse_args()
    if args.dashboard:
        serve(args.run_dir, args.host, args.port)
        return
    print(HEADER, flush=True)
    print("WORK is games (win-rate), bank episodes, decisions, updates, and percent of the wall-clock budget.", flush=True)
    while True:
        state = read_state(args.run_dir)
        if not state:
            print("Waiting for state.json", flush=True)
            if args.once:
                raise SystemExit(1)
        else:
            print(render_row(state), flush=True)
            if state.get("error"):
                print("ERROR: " + str(state["error"]), flush=True)
            if args.once or state.get("status") in {"completed", "failed", "stopped"}:
                break
        time.sleep(max(0.5, args.interval))


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
