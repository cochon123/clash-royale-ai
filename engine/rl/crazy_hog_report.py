"""Self-contained HTML report for a Crazy Hog run.

    python -m rl.crazy_hog_report --run-dir runs/crazy-hog-2.6
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path


def _read_json(path: Path, default):
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return default


def _read_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        return []
    rows = []
    for line in path.read_text(errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            rows.append(value)
    return rows


def load_run(run_dir: Path) -> dict:
    state = _read_json(run_dir / "state.json", {})
    manifest = _read_json(run_dir / "manifest.json", {})
    notes_path = run_dir / "notes.md"
    notes = notes_path.read_text(errors="replace") if notes_path.exists() else "This run did not write notes.md."
    return {
        "state": state,
        "manifest": manifest,
        "notes": notes,
        "updates": _read_jsonl(run_dir / "updates.jsonl"),
        "matches": _read_jsonl(run_dir / "matches.jsonl"),
        "audits": _read_jsonl(run_dir / "reward-audit.jsonl"),
        "metrics": _read_jsonl(run_dir / "metrics.jsonl"),
    }


def _cumulative_winrate(matches: list[dict]) -> list[dict]:
    per_deck: dict[str, list] = {}
    overall = []
    wins = losses = draws = 0
    deck_counts: dict[str, list[int]] = {}
    for index, row in enumerate(matches):
        if (row.get("kind") != "train" or not row.get("counts_for_winrate")
                or row.get("result") not in {"W", "L", "D"}):
            continue
        result = row["result"]
        wins += result == "W"
        losses += result == "L"
        draws += result == "D"
        total = wins + losses + draws
        score = (wins + 0.5 * draws) / total
        overall.append({"x": index + 1, "y": score, "tip": f"{wins}-{losses}-{draws}"})
        name = row.get("deck_name") or row.get("deck_id") or "deck"
        bucket = deck_counts.setdefault(name, [0, 0, 0])
        bucket[{"W": 0, "L": 1, "D": 2}[result]] += 1
        played = sum(bucket)
        per_deck.setdefault(name, []).append({
            "x": index + 1,
            "y": (bucket[0] + 0.5 * bucket[2]) / played,
            "tip": f"{bucket[0]}-{bucket[1]}-{bucket[2]}",
        })
    return overall, per_deck


def build_payload(run: dict) -> dict:
    state = run["state"]
    manifest = run["manifest"]
    args = manifest.get("args") or {}
    counters = state.get("counters") or {}
    progress = state.get("progress") or {}
    hog = state.get("hog") or {}
    overall, per_deck = _cumulative_winrate(run["matches"])
    hog_series = []
    connect_series = []
    running_hog = running_games = running_connect = running_hog_plays = 0
    for index, row in enumerate(run["matches"]):
        if row.get("kind") != "train" or row.get("result") == "VOID":
            continue
        running_games += 1
        running_hog += int(row.get("hog_plays") or 0)
        if row.get("hog_plays"):
            running_hog_plays += int(row["hog_plays"])
            running_connect += float(row.get("connect_rate") or 0) * int(row["hog_plays"])
        hog_series.append({"x": index + 1, "y": running_hog / running_games,
                           "tip": f"{running_hog} hogs / {running_games} games"})
        if running_hog_plays:
            connect_series.append({"x": index + 1, "y": running_connect / running_hog_plays, "tip": "connect rate"})
    trade_series = []
    for index, row in enumerate(run["audits"]):
        trade = (row.get("trade") or {}).get("sum_shaped")
        if isinstance(trade, (int, float)):
            trade_series.append({"x": index + 1, "y": trade, "tip": "sum of elixir-trade shaping"})
    return {
        "name": state.get("name") or manifest.get("name") or "crazy-hog-2.6",
        "version": manifest.get("format") or "crazy-hog-v1",
        "created": state.get("started_at") or manifest.get("deadline"),
        "deadline": state.get("deadline") or manifest.get("deadline"),
        "status": state.get("status"),
        "device": state.get("device") or args.get("device"),
        "workers": args.get("workers") or len(state.get("workers") or []),
        "engine": (state.get("engine") or manifest.get("engine") or {}).get("sha256"),
        "parent": (state.get("parent") or manifest.get("parent") or {}).get("name"),
        "parent_sha": (state.get("parent") or manifest.get("parent") or {}).get("sha256"),
        "games": counters.get("games", progress.get("games")),
        "bank_episodes": counters.get("bank_episodes", progress.get("bank_episodes")),
        "decisions": counters.get("decisions", progress.get("decisions")),
        "updates": counters.get("updates", progress.get("updates")),
        "hog": hog,
        "notes": run["notes"],
        "series": {
            "loss": _metric(run["updates"], "loss"),
            "policy_loss": _metric(run["updates"], "policy_loss"),
            "value_loss": _metric(run["updates"], "value_loss"),
            "entropy": _metric(run["updates"], "entropy"),
            "approx_kl": _metric(run["updates"], "approx_kl"),
            "parent_kl": _metric(run["updates"], "parent_kl"),
            "aux_loss": _metric(run["updates"], "aux_loss"),
            "winrate": overall,
            "decks": per_deck,
            "hog_per_game": hog_series,
            "connect": connect_series,
            "trade": trade_series,
            "decisions_per_second": _metric(run["metrics"], "decisions_per_second"),
            "games_per_minute": _metric(run["metrics"], "games_per_minute"),
        },
    }


def _metric(rows: list[dict], key: str) -> list[dict]:
    points = []
    for index, row in enumerate(rows):
        value = row.get(key)
        if isinstance(value, (int, float)):
            points.append({"x": row.get("updates", index + 1), "y": value, "tip": row.get("at", "")})
    return points


EXPLANATIONS = {
    "loss": "Mean PPO objective on the update: clipped policy loss, value loss, parent KL anchor, entropy bonus, and search aux loss when search is on.",
    "approx_kl": "Approximate KL between the behavior policy that sampled the batch and the updated policy. The update stops early past the target KL.",
    "parent_kl": "Exact card and wait KL from the frozen hog-2 parent onto the learner. Above the cap, the whole update, including the board branch, is rolled back.",
    "entropy": "Mean entropy of the legal card, wait, and placement distributions on controllable decisions. It falls when the policy gets peaky.",
    "winrate": "Cumulative (wins + half draws) / games. Bank episodes and VOID games are excluded. Draws are half a win.",
    "hog_per_game": "Accepted Hog Rider plays by the learner, divided by completed games including bank episodes. banana-hog-2 sat near 1.",
    "connect": "Share of Hog plays that were alive inside enemy tower range, or were followed by enemy tower damage within 8 seconds.",
    "trade": "Sum of the elixir-trade shaping terms in one audited game. With a true ending this equals minus the coefficient times the opening potential.",
    "throughput": "Decisions per second and games per minute from the live metrics log. Games per minute counts win-rate games. Bank episodes are shorter and are counted separately in the header.",
}


def render_html(payload: dict) -> str:
    data = json.dumps(payload).replace("<", "\\u003c")
    explanations = json.dumps(EXPLANATIONS).replace("<", "\\u003c")
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{payload['name']} report</title>
<style>
  :root {{ color-scheme: dark; --bg:#12110e; --ink:#f6f1e7; --muted:#b5ad9e; --line:#343026; --accent:#e2f07a; --panel:#1b1914; }}
  * {{ box-sizing: border-box; }}
  body {{ margin:0; background:radial-gradient(circle at 10% 0%, #2a281c, var(--bg) 42%); color:var(--ink);
         font:15px/1.5 ui-sans-serif, system-ui, sans-serif; }}
  main {{ width:min(1180px, calc(100% - 32px)); margin:auto; padding:36px 0 80px; }}
  h1 {{ font-size:clamp(2.2rem, 5vw, 3.6rem); letter-spacing:-.05em; line-height:.95; margin:8px 0 12px; }}
  h2 {{ margin:28px 0 8px; letter-spacing:-.03em; }}
  .muted {{ color:var(--muted); }}
  .grid {{ display:grid; grid-template-columns:repeat(auto-fit, minmax(180px, 1fr)); gap:10px; }}
  .card, .chart, .prose {{ background:var(--panel); border:1px solid var(--line); border-radius:16px; padding:14px 16px; }}
  .card b {{ display:block; font-size:1.25rem; margin-top:4px; }}
  .chart {{ margin-top:12px; position:relative; }}
  canvas {{ width:100%; height:280px; display:block; background:#14130f; border-radius:12px; }}
  .toggles label {{ margin-right:12px; cursor:pointer; }}
  .tip {{ position:absolute; pointer-events:none; background:#0e0d0a; border:1px solid var(--line); color:var(--ink);
         padding:6px 8px; border-radius:8px; font-size:12px; display:none; z-index:2; max-width:280px; }}
  button {{ background:var(--accent); color:#1a1a10; border:0; border-radius:999px; padding:6px 12px; font-weight:700; cursor:pointer; }}
  svg.arch {{ width:100%; height:auto; background:#14130f; border-radius:12px; }}
  .lessons {{ white-space:pre-wrap; }}
  code {{ font-family:ui-monospace, monospace; }}
</style>
</head>
<body>
<main>
  <p class="muted">Crazy Hog 2.6 · board-aware league</p>
  <h1 id="title"></h1>
  <p class="muted" id="stamp"></p>
  <section class="grid" id="facts"></section>

  <h2>Architecture</h2>
  <div class="prose">
    <svg class="arch" viewBox="0 0 980 220" role="img" aria-label="Training pipeline">
      <rect x="20" y="40" width="150" height="64" rx="12" fill="#242218" stroke="#e2f07a"/>
      <text x="95" y="68" fill="#f6f1e7" text-anchor="middle" font-size="13">8–12 engines</text>
      <text x="95" y="88" fill="#b5ad9e" text-anchor="middle" font-size="11">board + snapshot</text>
      <rect x="210" y="40" width="160" height="64" rx="12" fill="#242218" stroke="#9ad7ff"/>
      <text x="290" y="68" fill="#f6f1e7" text-anchor="middle" font-size="13">one waiting batch</text>
      <text x="290" y="88" fill="#b5ad9e" text-anchor="middle" font-size="11">learner + one/opponent</text>
      <rect x="410" y="30" width="180" height="84" rx="12" fill="#242218" stroke="#e2f07a"/>
      <text x="500" y="58" fill="#f6f1e7" text-anchor="middle" font-size="13">BoardHogPolicy</text>
      <text x="500" y="78" fill="#b5ad9e" text-anchor="middle" font-size="11">set encoder + raster</text>
      <text x="500" y="96" fill="#b5ad9e" text-anchor="middle" font-size="11">privileged critic</text>
      <rect x="630" y="40" width="140" height="64" rx="12" fill="#242218" stroke="#ffb087"/>
      <text x="700" y="68" fill="#f6f1e7" text-anchor="middle" font-size="13">PPO + parent KL</text>
      <text x="700" y="88" fill="#b5ad9e" text-anchor="middle" font-size="11">GAE, aux if search</text>
      <rect x="210" y="140" width="200" height="56" rx="12" fill="#242218" stroke="#d6c4ff"/>
      <text x="310" y="164" fill="#f6f1e7" text-anchor="middle" font-size="13">PFSP league</text>
      <text x="310" y="182" fill="#b5ad9e" text-anchor="middle" font-size="11">deck × controller, snapshots</text>
      <rect x="450" y="140" width="220" height="56" rx="12" fill="#242218" stroke="#d6c4ff"/>
      <text x="560" y="164" fill="#f6f1e7" text-anchor="middle" font-size="13">state bank</text>
      <text x="560" y="182" fill="#b5ad9e" text-anchor="middle" font-size="11">restore, short horizon, bootstrap</text>
      <path d="M170 72 H210 M370 72 H410 M590 72 H630" stroke="#e2f07a" stroke-width="2" fill="none"/>
    </svg>
    <p class="muted">The learner reads the board. Frozen bc-greedy, bc-sampled, and the hog-2 parent do not. Self-play snapshots are copies of the learner, so they do, and they only play the Hog mirror. Rewards are the native ±1/0 plus two telescoping potentials: tower HP and elixir trade.</p>
  </div>

  <h2>Curves</h2>
  <p class="muted">Toggle a series to hide it. Play walks the curves forward in update order. Hover a point for the logged value.</p>
  <div id="charts"></div>

  <h2>What each curve means</h2>
  <div class="prose" id="meanings"></div>

  <h2>Lessons learned</h2>
  <div class="prose lessons" id="lessons"></div>
</main>
<script>
const DATA = {data};
const MEANINGS = {explanations};
const COLORS = ['#e2f07a','#9ad7ff','#ffb087','#d6c4ff','#ffd27a','#f3f0e6','#8dffa8','#ff8fa3'];
const $ = id => document.getElementById(id);
function fact(label, value){{
  return '<div class="card"><span class="muted">'+label+'</span><b>'+value+'</b></div>';
}}
$('title').textContent = DATA.name + ' · ' + (DATA.version||'');
$('stamp').textContent = 'Created ' + (DATA.created||'unknown') + ' · deadline ' + (DATA.deadline||'') + ' · status ' + (DATA.status||'');
const hog = DATA.hog||{{}};
$('facts').innerHTML = [
  fact('Name / version', DATA.name + ' / ' + (DATA.version||'')),
  fact('Compute', (DATA.device||'?') + ' · ' + (DATA.workers||'?') + ' workers'),
  fact('Data', (DATA.games||0) + ' games · ' + (DATA.bank_episodes||0) + ' bank · ' + (DATA.decisions||0) + ' decisions · ' + (DATA.updates||0) + ' updates'),
  fact('Timestamp', DATA.created||''),
  fact('Parent', (DATA.parent||'') + ''),
  fact('Engine', (DATA.engine||'').slice(0,16)),
  fact('Hog plays / game', hog.hog_plays_per_game==null?'—':Number(hog.hog_plays_per_game).toFixed(2)),
  fact('Connect rate', hog.connect_rate==null?'—':(100*hog.connect_rate).toFixed(1)+'%'),
  fact('Elixir seconds at 10', hog.elixir_leak_seconds==null?'—':Number(hog.elixir_leak_seconds).toFixed(2))
].join('');
$('lessons').textContent = DATA.notes || 'No notes.md was written for this run.';
$('meanings').innerHTML = Object.keys(MEANINGS).map(key => '<p><b>'+key+'.</b> '+MEANINGS[key]+'</p>').join('');

function seriesOf(spec){{
  if(spec==='decks'){{
    return Object.keys(DATA.series.decks||{{}}).map((name,i)=>[{{name:name,color:COLORS[i%COLORS.length],points:DATA.series.decks[name]}}][0]);
  }}
  const table = {{
    learning:['loss','policy_loss','value_loss','entropy','aux_loss'],
    divergence:['approx_kl','parent_kl'],
    winrate:['winrate'],
    hog:['hog_per_game','connect'],
    trade:['trade'],
    throughput:['decisions_per_second','games_per_minute']
  }};
  return (table[spec]||[]).map((name,i)=>({{name, color:COLORS[i%COLORS.length], points:DATA.series[name]||[]}}));
}}

const CHARTS = [
  ['learning','Loss, value, entropy, aux'],
  ['divergence','KL'],
  ['winrate','Win rate'],
  ['decks','Win rate by deck'],
  ['hog','Hog plays per game and connect rate'],
  ['trade','Elixir-trade reward'],
  ['throughput','Throughput']
];

function mount(spec, title){{
  const wrap = document.createElement('div');
  wrap.className = 'chart';
  const head = document.createElement('div');
  head.innerHTML = '<strong>'+title+'</strong> ';
  const play = document.createElement('button');
  play.textContent = 'Play';
  head.appendChild(play);
  const toggles = document.createElement('div');
  toggles.className = 'toggles';
  const canvas = document.createElement('canvas');
  canvas.width = 1000; canvas.height = 280;
  const tip = document.createElement('div');
  tip.className = 'tip';
  wrap.append(head, toggles, canvas, tip);
  const series = seriesOf(spec).map(item => ({{...item, on:true}}));
  series.forEach((item, index) => {{
    const label = document.createElement('label');
    const box = document.createElement('input');
    box.type = 'checkbox'; box.checked = true;
    box.addEventListener('change', () => {{ item.on = box.checked; draw(1); }});
    label.append(box, document.createTextNode(' '+item.name));
    label.style.color = item.color;
    toggles.appendChild(label);
  }});
  let frame = 1;
  let timer = 0;
  function points(item){{
    const count = Math.max(1, Math.round(item.points.length * frame));
    return item.points.slice(0, count);
  }}
  function draw(next){{
    frame = next;
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0,0,canvas.width,canvas.height);
    const shown = series.filter(item => item.on && item.points.length);
    const all = shown.flatMap(points);
    if(!all.length){{ ctx.fillStyle='#b5ad9e'; ctx.fillText('No points yet', 24, 40); return; }}
    const ys = all.map(point => point.y);
    let lo = Math.min(...ys), hi = Math.max(...ys);
    if(lo===hi){{ lo -= 1; hi += 1; }}
    const pad = 36;
    const xOf = index => pad + (canvas.width-pad-16) * (index / Math.max(1, Math.max(...shown.map(item => item.points.length))-1 || 1));
    const yOf = value => 16 + (canvas.height-48) * (1 - (value-lo)/(hi-lo));
    ctx.strokeStyle = '#343026'; ctx.beginPath();
    for(let i=0;i<4;i++){{ const y=16+(canvas.height-48)*i/3; ctx.moveTo(pad,y); ctx.lineTo(canvas.width-8,y); }}
    ctx.stroke();
    ctx.fillStyle = '#b5ad9e'; ctx.font = '12px ui-monospace,monospace';
    ctx.fillText(hi.toFixed(3), 4, 20); ctx.fillText(lo.toFixed(3), 4, canvas.height-28);
    shown.forEach(item => {{
      const rows = points(item);
      ctx.strokeStyle = item.color; ctx.lineWidth = 2; ctx.beginPath();
      rows.forEach((point, index) => {{
        const x = xOf(index), y = yOf(point.y);
        if(index===0) ctx.moveTo(x,y); else ctx.lineTo(x,y);
      }});
      ctx.stroke();
      rows.forEach((point, index) => {{
        ctx.fillStyle = item.color;
        ctx.beginPath(); ctx.arc(xOf(index), yOf(point.y), 3.2, 0, Math.PI*2); ctx.fill();
      }});
    }});
    canvas.onmousemove = event => {{
      const rect = canvas.getBoundingClientRect();
      const mx = (event.clientX-rect.left) * (canvas.width/rect.width);
      const my = (event.clientY-rect.top) * (canvas.height/rect.height);
      let best = null;
      shown.forEach(item => points(item).forEach((point, index) => {{
        const dx = xOf(index)-mx, dy = yOf(point.y)-my;
        const dist = dx*dx+dy*dy;
        if(!best || dist<best.dist) best = {{dist, item, point, index}};
      }}));
      if(!best || best.dist>400){{ tip.style.display='none'; return; }}
      tip.style.display='block';
      tip.style.left = (event.offsetX+12)+'px';
      tip.style.top = (event.offsetY+12)+'px';
      tip.textContent = best.item.name+' @ '+best.point.x+': '+Number(best.point.y).toFixed(4)+(best.point.tip?(' · '+best.point.tip):'');
    }};
    canvas.onmouseleave = () => {{ tip.style.display='none'; }};
  }}
  play.addEventListener('click', () => {{
    if(timer){{ cancelAnimationFrame(timer); timer=0; play.textContent='Play'; return; }}
    play.textContent = 'Pause';
    const start = performance.now();
    const step = now => {{
      frame = Math.min(1, (now-start)/4000);
      draw(frame);
      if(frame<1) timer = requestAnimationFrame(step);
      else {{ timer=0; play.textContent='Play'; }}
    }};
    timer = requestAnimationFrame(step);
  }});
  draw(1);
  return wrap;
}}
CHARTS.forEach(([spec, title]) => $('charts').appendChild(mount(spec, title)));
</script>
</body>
</html>
"""


def build_report(run_dir: Path, output: Path | None = None) -> Path:
    run_dir = Path(run_dir)
    payload = build_payload(load_run(run_dir))
    target = Path(output) if output else run_dir / "report.html"
    target.write_text(render_html(payload))
    return target


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    path = build_report(args.run_dir, args.output)
    print(path, flush=True)


if __name__ == "__main__":
    main()
