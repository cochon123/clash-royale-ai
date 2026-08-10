"""Interactive HTML report for the policy battle royale."""

from __future__ import annotations

import html
import json
from pathlib import Path
from typing import Any

from .winner_report import _base_styles, _fmt_pct, _json_script


def _esc(value: Any) -> str:
    return html.escape("" if value is None else str(value))


def _fmt_wr(value: float | None) -> str:
    if value is None:
        return "—"
    return _fmt_pct(value)


def write_battle_royale_report(
    report: dict[str, Any] | str | Path,
    output_path: str | Path = "reports/battle_royale.html",
) -> Path:
    if not isinstance(report, dict):
        with Path(report).open(encoding="utf-8") as handle:
            report = json.load(handle)

    setup = report.get("setup") or {}
    progress = report.get("progress") or {}
    standings = report.get("standings") or []
    pairs = report.get("pairs") or []
    policies = setup.get("policies") or []
    champion = report.get("champion")
    min_conf = float(setup.get("min_confidence", 0.8))
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    podium = standings[:3]
    podium_html = "".join(
        f"""
        <div class="podium-card rank-{row.get('rank', i+1)}">
          <div class="podium-rank">#{row.get('rank', i+1)}</div>
          <div class="podium-name">{_esc(row.get('policy_id'))}</div>
          <div class="podium-wr">{_fmt_wr(row.get('win_rate'))}</div>
          <div class="podium-meta">{row.get('wins', 0)}-{row.get('losses', 0)}
            · Elo {row.get('elo', 0):.0f}</div>
        </div>
        """
        for i, row in enumerate(podium)
    )

    standing_rows = "".join(
        f"""
        <tr class="{'champ' if row.get('policy_id') == champion else ''}">
          <td>{row.get('rank', '')}</td>
          <td>{_esc(row.get('policy_id'))}</td>
          <td>{row.get('wins', 0)}</td>
          <td>{row.get('losses', 0)}</td>
          <td>{_fmt_wr(row.get('win_rate'))}</td>
          <td>{row.get('elo', 0):.1f}</td>
          <td>{row.get('games', 0)}</td>
          <td>{_fmt_wr(row.get('raw_win_rate'))}</td>
          <td>{_esc(row.get('created_at') or '—')}</td>
        </tr>
        """
        for row in standings
    )

    pair_rows = "".join(
        f"""
        <tr>
          <td>{_esc(p.get('a'))}</td>
          <td>{_esc(p.get('b'))}</td>
          <td>{p.get('a_wins', 0)}-{p.get('b_wins', 0)}</td>
          <td>{_fmt_wr(p.get('a_win_rate'))}</td>
          <td>{p.get('confident_games', 0)}/{p.get('games', 0)}</td>
          <td>{_fmt_pct(p.get('coverage'))}</td>
        </tr>
        """
        for p in pairs
    )

    policy_cards = "".join(
        f"""
        <div class="policy-card">
          <h3>{_esc(p.get('policy_id'))}</h3>
          <p class="meta">{_esc(p.get('model_dir'))}</p>
          <dl>
            <div><dt>Created</dt><dd>{_esc(p.get('created_at') or '—')}</dd></div>
            <div><dt>Threat dim</dt><dd>{p.get('threat_dim', 0)}</dd></div>
            <div><dt>Card-conditioned XY</dt>
              <dd>{'yes' if p.get('card_conditioned_placement') else 'no'}</dd></div>
            <div><dt>Test slot@1</dt>
              <dd>{_fmt_pct((p.get('test') or {}).get('slot_top1'))}</dd></div>
          </dl>
        </div>
        """
        for p in policies
    )

    lessons = "".join(f"<li>{_esc(item)}</li>" for item in report.get("lessons") or [])

    page = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Battle Royale — {_esc(report.get('model_name', 'policy-battle-royale'))}</title>
  <style>
    {_base_styles()}
    .podium {{
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 18px;
      margin: 18px 0 8px;
    }}
    .podium-card {{
      padding: 18px 16px;
      border: 1px solid var(--line);
      background: linear-gradient(180deg, #101826, #0b1220);
    }}
    .podium-card.rank-1 {{ border-color: #fbbf24; box-shadow: inset 0 0 0 1px rgba(251,191,36,0.25); }}
    .podium-card.rank-2 {{ border-color: #94a3b8; }}
    .podium-card.rank-3 {{ border-color: #b45309; }}
    .podium-rank {{ color: var(--muted); font-size: 0.75rem; letter-spacing: 0.08em; text-transform: uppercase; }}
    .podium-name {{ font-size: 1.2rem; font-weight: 650; margin: 8px 0 4px; color: var(--text); }}
    .podium-wr {{ font-size: 1.8rem; font-weight: 700; letter-spacing: -0.03em; }}
    .podium-meta {{ color: #64748b; font-size: 0.88rem; margin-top: 6px; }}
    table {{
      width: 100%;
      border-collapse: collapse;
      font-size: 0.92rem;
    }}
    th, td {{
      text-align: left;
      padding: 10px 8px;
      border-bottom: 1px solid var(--line-soft);
    }}
    th {{
      color: #64748b;
      font-size: 0.72rem;
      letter-spacing: 0.06em;
      text-transform: uppercase;
      font-weight: 600;
    }}
    tr.champ td {{ color: #fde68a; }}
    .policy-grid {{
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 16px;
    }}
    .policy-card {{
      border: 1px solid var(--line-soft);
      padding: 14px 16px;
    }}
    .policy-card h3 {{ margin: 0 0 4px; color: var(--text); font-size: 1.05rem; }}
    .policy-card dl {{
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 8px 14px;
      margin: 12px 0 0;
    }}
    .policy-card dt {{ color: #64748b; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.06em; }}
    .policy-card dd {{ margin: 2px 0 0; color: var(--text); }}
    .matrix-wrap {{ overflow-x: auto; }}
    .verdict {{
      font-size: 1.15rem;
      color: var(--text);
      max-width: 820px;
      line-height: 1.45;
    }}
    .flow {{
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 12px;
      margin-top: 12px;
    }}
    .flow-step {{
      border: 1px solid var(--line-soft);
      padding: 12px;
      min-height: 110px;
    }}
    .flow-num {{
      display: inline-block;
      width: 22px; height: 22px;
      border-radius: 999px;
      background: #1e293b;
      color: #93c5fd;
      text-align: center;
      line-height: 22px;
      font-size: 0.75rem;
      margin-bottom: 8px;
    }}
    @media (max-width: 800px) {{
      .podium, .policy-grid, .flow, .kpi-row {{ grid-template-columns: 1fr; }}
    }}
  </style>
</head>
<body>
<main>
  <header class="report-header">
    <div class="badge-row">
      <span class="badge">{_esc(report.get('model_name', 'policy-battle-royale-v1'))}</span>
      <span class="badge">judge: {_esc(report.get('judge'))}</span>
      <span class="badge">conf ≥ {_fmt_pct(min_conf)}</span>
      <span class="badge">{_esc(report.get('created_at'))}</span>
    </div>
    <h1>Policy battle royale</h1>
    <p class="verdict">{_esc(report.get('verdict') or 'In progress…')}</p>
    <p class="meta">
      Offline round-robin · {len(policies)} policies ·
      {progress.get('confident_games', 0)} confident / {progress.get('games_done', 0)} raw games ·
      {setup.get('device', '?')} · {report.get('seconds', '?')}s
    </p>
  </header>

  <section class="report-section">
    <h2>Podium</h2>
    <div class="podium">{podium_html or '<p>No standings yet.</p>'}</div>
  </section>

  <section class="report-section kpi-row">
    <div>
      <span class="kpi-label">Champion</span>
      <span class="kpi-value">{_esc(champion or '—')}</span>
    </div>
    <div>
      <span class="kpi-label">Confident coverage</span>
      <span class="kpi-value">{_fmt_pct(progress.get('coverage'))}</span>
    </div>
    <div>
      <span class="kpi-label">Mean kept confidence</span>
      <span class="kpi-value">{_fmt_pct(progress.get('mean_kept_confidence'))}</span>
    </div>
    <div>
      <span class="kpi-label">Games / pair</span>
      <span class="kpi-value">{setup.get('games_per_pair', '—')}</span>
    </div>
  </section>

  <section class="report-section">
    <h2>How the tournament works</h2>
    <div class="flow">
      <div class="flow-step"><span class="flow-num">1</span><b>Sample real decks</b><br>
        <small>Train-split decks from the replay corpus</small></div>
      <div class="flow-step"><span class="flow-num">2</span><b>Cross-play</b><br>
        <small>Each policy sits one seat; seats alternate</small></div>
      <div class="flow-step"><span class="flow-num">3</span><b>Winner judge</b><br>
        <small>Symmetric HGB+trees ensemble scores P(team wins)</small></div>
      <div class="flow-step"><span class="flow-num">4</span><b>Confidence gate</b><br>
        <small>Keep only games with calibrated conf ≥ {_fmt_pct(min_conf)}</small></div>
    </div>
  </section>

  <section class="report-section">
    <h2>Standings (confident games only)</h2>
    <table>
      <thead>
        <tr>
          <th>#</th><th>Policy</th><th>W</th><th>L</th><th>WR</th>
          <th>Elo</th><th>Games</th><th>Raw WR</th><th>Trained</th>
        </tr>
      </thead>
      <tbody>{standing_rows}</tbody>
    </table>
  </section>

  <section class="report-section chart-grid">
    <div class="chart-block">
      <h2>Win rate by policy</h2>
      <div class="chart-wrap"><svg class="chart" id="wrChart"></svg></div>
      <p class="caption">Bars use confident decisions only. Hover for W-L and Elo.</p>
    </div>
    <div class="chart-block">
      <h2>Head-to-head matrix</h2>
      <div class="matrix-wrap"><svg class="chart" id="h2hChart"></svg></div>
      <p class="caption">Cell = row's win rate vs column on confident games. Diagonal is empty.</p>
    </div>
    <div class="chart-block">
      <h2>Judge confidence distribution</h2>
      <div class="chart-wrap"><svg class="chart" id="confChart"></svg></div>
      <p class="caption">Dashed line marks the {_fmt_pct(min_conf)} gate. Coverage on held-out real games
        at this threshold is ~48% with ~94% accuracy.</p>
    </div>
  </section>

  <section class="report-section">
    <h2>Pair results</h2>
    <table>
      <thead>
        <tr><th>A</th><th>B</th><th>Score (A-B)</th><th>A WR</th><th>Kept/Raw</th><th>Coverage</th></tr>
      </thead>
      <tbody>{pair_rows}</tbody>
    </table>
  </section>

  <section class="report-section">
    <h2>Contenders</h2>
    <div class="policy-grid">{policy_cards}</div>
  </section>

  <section class="report-section">
    <h2>Lessons</h2>
    <ul>{lessons}</ul>
    <p class="meta">JSON source: reports/battle_royale.json · stream:
      <code>tail -f reports/battle_royale_progress.jsonl</code></p>
  </section>
</main>
<script>
const DATA = {_json_script(report)};

function renderWinRates(svgId, standings) {{
  const svg = document.getElementById(svgId);
  if (!svg || !standings.length) return;
  const W = 1000, H = 540, ML = 70, MR = 24, MT = 28, MB = 90;
  svg.setAttribute("viewBox", `0 0 ${{W}} ${{H}}`);
  svg.innerHTML = "";
  const plotW = W - ML - MR, plotH = H - MT - MB;
  const n = standings.length;
  const gap = 18;
  const barW = (plotW - gap * (n + 1)) / n;
  const maxY = Math.max(0.6, ...standings.map(s => s.win_rate || 0));
  const colors = ["#fbbf24", "#93c5fd", "#fca5a5", "#86efac", "#c4b5fd", "#fdba74"];

  // axes
  svg.appendChild(svgEl("line", {{x1: ML, y1: MT, x2: ML, y2: MT+plotH, stroke: "#334155"}}));
  svg.appendChild(svgEl("line", {{x1: ML, y1: MT+plotH, x2: ML+plotW, y2: MT+plotH, stroke: "#334155"}}));
  for (let t = 0; t <= 4; t++) {{
    const yv = t / 4 * maxY;
    const y = MT + plotH - (yv / maxY) * plotH;
    svg.appendChild(svgEl("line", {{x1: ML, y1: y, x2: ML+plotW, y2: y, stroke: "#1e293b"}}));
    svg.appendChild(svgText({{x: ML-10, y: y+4, fill: "#64748b", "font-size": 12, "text-anchor": "end"}},
      (100*yv).toFixed(0) + "%"));
  }}
  standings.forEach((s, i) => {{
    const h = ((s.win_rate || 0) / maxY) * plotH;
    const x = ML + gap + i * (barW + gap);
    const y = MT + plotH - h;
    const rect = svgEl("rect", {{
      x, y, width: barW, height: Math.max(h, 1),
      fill: colors[i % colors.length], opacity: 0.9, rx: 4
    }});
    rect.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "title")).textContent =
      `${{s.policy_id}}: ${{((s.win_rate||0)*100).toFixed(1)}}% (${{s.wins}}-${{s.losses}}) Elo ${{s.elo.toFixed(0)}}`;
    svg.appendChild(rect);
    svg.appendChild(svgText({{
      x: x + barW/2, y: MT + plotH + 22, fill: "#94a3b8", "font-size": 12,
      "text-anchor": "middle"
    }}, s.policy_id.replace("policy-bc-", "")));
    svg.appendChild(svgText({{
      x: x + barW/2, y: y - 8, fill: "#e2e8f0", "font-size": 13,
      "text-anchor": "middle", "font-weight": 600
    }}, ((s.win_rate||0)*100).toFixed(1) + "%"));
  }});
}}

function renderH2H(svgId, standings, h2h) {{
  const svg = document.getElementById(svgId);
  if (!svg || !standings.length) return;
  const ids = standings.map(s => s.policy_id);
  const W = 1000, H = 540, ML = 140, MR = 40, MT = 40, MB = 40;
  svg.setAttribute("viewBox", `0 0 ${{W}} ${{H}}`);
  svg.innerHTML = "";
  const n = ids.length;
  const size = Math.min((W - ML - MR) / n, (H - MT - MB) / n);
  const ox = ML, oy = MT;
  function color(wr) {{
    if (wr == null) return "#0f172a";
    const t = Math.max(0, Math.min(1, wr));
    const r = Math.round(248 * (1-t) + 52 * t);
    const g = Math.round(113 * (1-t) + 211 * t);
    const b = Math.round(113 * (1-t) + 153 * t);
    return `rgb(${{r}},${{g}},${{b}})`;
  }}
  ids.forEach((rowId, i) => {{
    svg.appendChild(svgText({{
      x: ox - 10, y: oy + i*size + size/2 + 4, fill: "#94a3b8", "font-size": 12,
      "text-anchor": "end"
    }}, rowId.replace("policy-bc-", "")));
    ids.forEach((colId, j) => {{
      const x = ox + j*size, y = oy + i*size;
      let wr = null, label = "—";
      if (i === j) {{
        label = "·";
      }} else {{
        const cell = (h2h[rowId] || {{}})[colId] || {{}};
        const g = cell.games || 0;
        if (g > 0) {{
          wr = (cell.wins || 0) / g;
          label = (100*wr).toFixed(0) + "%";
        }}
      }}
      const rect = svgEl("rect", {{
        x: x+2, y: y+2, width: size-4, height: size-4,
        fill: i===j ? "#111827" : color(wr),
        stroke: "#1e293b"
      }});
      rect.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "title")).textContent =
        `${{rowId}} vs ${{colId}}: ${{label}}`;
      svg.appendChild(rect);
      svg.appendChild(svgText({{
        x: x + size/2, y: y + size/2 + 4, fill: "#0b1220", "font-size": 13,
        "text-anchor": "middle", "font-weight": 650
      }}, label));
    }});
    svg.appendChild(svgText({{
      x: ox + i*size + size/2, y: oy - 12, fill: "#94a3b8", "font-size": 12,
      "text-anchor": "middle"
    }}, ids[i].replace("policy-bc-", "")));
  }});
}}

function renderConf(svgId, hist, minConf) {{
  const svg = document.getElementById(svgId);
  if (!svg || !hist) return;
  const edges = hist.edges || [];
  const counts = hist.counts || [];
  const W = 1000, H = 540, ML = 60, MR = 24, MT = 28, MB = 60;
  svg.setAttribute("viewBox", `0 0 ${{W}} ${{H}}`);
  svg.innerHTML = "";
  const plotW = W - ML - MR, plotH = H - MT - MB;
  const maxC = Math.max(1, ...counts);
  const n = counts.length;
  const barW = plotW / n;
  counts.forEach((c, i) => {{
    const h = (c / maxC) * plotH;
    const x = ML + i * barW;
    const mid = (edges[i] + edges[i+1]) / 2;
    const kept = mid >= minConf;
    const rect = svgEl("rect", {{
      x: x+3, y: MT + plotH - h, width: barW-6, height: Math.max(h,1),
      fill: kept ? "#34d399" : "#64748b", opacity: 0.85, rx: 3
    }});
    svg.appendChild(rect);
    svg.appendChild(svgText({{
      x: x + barW/2, y: MT + plotH + 22, fill: "#64748b", "font-size": 11,
      "text-anchor": "middle"
    }}, (100*edges[i]).toFixed(0)));
  }});
  const gx = ML + ((minConf - edges[0]) / (edges[edges.length-1] - edges[0])) * plotW;
  svg.appendChild(svgEl("line", {{
    x1: gx, y1: MT, x2: gx, y2: MT+plotH, stroke: "#fbbf24",
    "stroke-dasharray": "6 4", "stroke-width": 2
  }}));
  svg.appendChild(svgText({{
    x: gx + 6, y: MT + 14, fill: "#fbbf24", "font-size": 12
  }}, "gate " + (100*minConf).toFixed(0) + "%"));
}}

function svgEl(name, attrs) {{
  const el = document.createElementNS("http://www.w3.org/2000/svg", name);
  Object.entries(attrs || {{}}).forEach(([k,v]) => el.setAttribute(k, v));
  return el;
}}
function svgText(attrs, text) {{
  const el = svgEl("text", attrs);
  el.textContent = text;
  return el;
}}

renderWinRates("wrChart", DATA.standings || []);
renderH2H("h2hChart", DATA.standings || [], DATA.head_to_head || {{}});
renderConf("confChart", DATA.confidence_hist, (DATA.setup || {{}}).min_confidence || 0.8);
</script>
</body>
</html>
"""
    out.write_text(page, encoding="utf-8")
    return out
