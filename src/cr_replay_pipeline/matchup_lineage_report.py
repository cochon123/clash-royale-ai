"""Native HTML report for multi-policy wincon matchup lineage."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def write_matchup_lineage_report(
    payload: dict[str, Any],
    output_path: str | Path = "reports/matchup_lineage.html",
) -> Path:
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    data = json.dumps(payload, ensure_ascii=False).replace("<", "\\u003c")

    html = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Matchup lineage — human vs AI win rates</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@500;700&family=IBM+Plex+Sans:wght@400;600;700&display=swap" rel="stylesheet">
<style>
:root{--bg:#07100e;--panel:#0d1916;--line:#20332d;--text:#edf7f2;--muted:#91a9a0;--green:#70e1b1;--gold:#ffca63;--blue:#70a1ff;--red:#ff7e78;--star:#e8f58b;--ink:#06100d}
*{box-sizing:border-box}html{scroll-behavior:smooth}
body{margin:0;background:radial-gradient(circle at 12% -10%,#1d3f32 0,transparent 34%),radial-gradient(circle at 92% 0%,#3a3418 0,transparent 28%),var(--bg);color:var(--text);font:15px/1.55 "IBM Plex Sans",ui-sans-serif,system-ui,sans-serif}
body:before{content:"";position:fixed;inset:0;pointer-events:none;opacity:.045;background-image:linear-gradient(#fff 1px,transparent 1px),linear-gradient(90deg,#fff 1px,transparent 1px);background-size:42px 42px;mask-image:linear-gradient(to bottom,#000,transparent 70%)}
main{width:min(1180px,calc(100% - 36px));margin:auto;padding:48px 0 96px}
.eyebrow{font:700 11px/1.2 "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.16em;text-transform:uppercase;color:var(--star)}
h1{font-size:clamp(38px,6.8vw,74px);line-height:.94;letter-spacing:-.065em;margin:16px 0 18px;max-width:920px}
h1 em{color:var(--star);font-style:normal}
h2{font-size:clamp(24px,3.6vw,36px);letter-spacing:-.04em;margin:0 0 8px}
h3{font-size:17px;margin:0 0 8px}
p{color:var(--muted);margin:0}.lede{font-size:clamp(16px,2vw,20px);max-width:820px;color:#bfd0ca}
.stamp{margin-top:18px;color:#607d72;font:12px "IBM Plex Mono",ui-monospace,monospace}
.verdict{margin-top:34px;border:1px solid #5d6b34;background:linear-gradient(135deg,rgba(232,245,139,.12),rgba(112,225,177,.05));border-radius:22px;padding:22px;display:grid;grid-template-columns:auto 1fr;gap:16px;align-items:start}
.verdict .mark{width:48px;height:48px;border-radius:14px;display:grid;place-items:center;background:var(--star);color:var(--ink);font-size:22px;font-weight:900}
.verdict strong{display:block;font-size:19px;margin-bottom:4px}
.section{border-top:1px solid var(--line);padding:46px 0}
.section-head{display:flex;justify-content:space-between;align-items:end;gap:18px;margin-bottom:22px}.section-head p{max-width:540px}
.card,.kpi{border:1px solid var(--line);background:linear-gradient(160deg,rgba(17,33,29,.94),rgba(8,17,14,.94));border-radius:18px;padding:20px;position:relative;overflow:hidden}
.kpi:after{content:"";position:absolute;width:100px;height:100px;border-radius:50%;background:var(--tone,var(--star));filter:blur(55px);opacity:.12;right:-30px;top:-35px}
.kpi span{display:block;color:var(--muted);font-size:12px;text-transform:uppercase;letter-spacing:.08em}
.kpi b{display:block;font-size:clamp(24px,3.2vw,36px);letter-spacing:-.045em;margin:6px 0 2px}.kpi small{color:#789188}
.match-kpis{display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin:16px 0 18px}
.match-chips{display:flex;flex-wrap:wrap;gap:8px;margin:4px 0 8px}
.match-chip{border:1px solid var(--line);background:#0a1512;color:var(--muted);border-radius:999px;padding:9px 14px;font:700 12px "IBM Plex Mono",ui-monospace,monospace;cursor:pointer;transition:transform .18s ease,border-color .18s,background .18s,color .18s}
.match-chip:hover{border-color:#416459;color:var(--text);transform:translateY(-1px)}
.match-chip.active{background:var(--tone,var(--star));color:var(--ink);border-color:transparent;box-shadow:0 0 24px color-mix(in srgb,var(--tone,var(--star)) 35%,transparent)}
.compare-grid{display:grid;grid-template-columns:1.35fr .65fr;gap:14px}
.delta-chart{width:100%;height:auto;display:block}
.expr{margin-top:14px;padding:12px 14px;border:1px dashed #355247;border-radius:12px;color:#9bb6ad;font:12px/1.45 "IBM Plex Mono",ui-monospace,monospace}
.expr b{color:var(--text)}
.bars{display:grid;gap:10px}
.bar-row{display:grid;grid-template-columns:54px 1fr 64px;gap:10px;align-items:center}
.bar-track{height:12px;border-radius:99px;background:#08110f;border:1px solid #1c2d28;overflow:hidden}
.bar-fill{height:100%;width:0;background:var(--tone,var(--star));border-radius:99px;transition:width .7s cubic-bezier(.2,.8,.2,1)}
.bar-value{font:700 12px "IBM Plex Mono",ui-monospace,monospace;text-align:right;color:var(--muted)}
.callout{border:1px solid #355247;border-radius:12px;padding:12px 14px;color:#9bb6ad;font-size:13px;background:rgba(8,20,16,.55)}
.callout b{color:var(--text)}
.table-wrap{overflow:auto;border:1px solid var(--line);border-radius:16px}
table{width:100%;border-collapse:collapse;font-size:13px}
th,td{padding:11px 12px;border-bottom:1px solid var(--line);text-align:right;white-space:nowrap}
th:first-child,td:first-child{text-align:left}
thead th{font:700 11px "IBM Plex Mono",ui-monospace,monospace;text-transform:uppercase;letter-spacing:.08em;color:var(--muted);background:#0a1512;position:sticky;top:0}
tbody tr:hover{background:#13231e}
.flip{color:var(--red);font-weight:700}.ok{color:var(--green)}
.human{color:var(--gold)}
.lessons{display:grid;grid-template-columns:repeat(3,1fr);gap:12px}
.lesson .n{font:700 11px "IBM Plex Mono",ui-monospace,monospace;color:var(--star);letter-spacing:.1em;margin-bottom:10px}
.foot{color:#607d72;font:12px "IBM Plex Mono",ui-monospace,monospace;padding-top:10px}
.tip{position:fixed;pointer-events:none;z-index:40;background:#0f1f1a;border:1px solid #355247;color:var(--text);padding:8px 10px;border-radius:10px;font:12px/1.35 "IBM Plex Mono",ui-monospace,monospace;opacity:0;transform:translateY(4px);transition:opacity .12s,transform .12s;max-width:280px}
.tip.on{opacity:1;transform:none}
@media (max-width:900px){
  .match-kpis,.compare-grid,.lessons{grid-template-columns:1fr}
  .verdict{grid-template-columns:1fr}
}
</style>
</head>
<body>
<main>
  <div class="eyebrow">Clash Royale AI · offline matchup audit</div>
  <h1>Do policies keep the <em>human</em> matchup edges?</h1>
  <p class="lede">Mine high-support win-condition matchups from ladder replays, let each lineage policy play both seats on a shared schedule, then judge winners with the offline winner model — no live games.</p>
  <div class="stamp" id="stamp">—</div>
  <div class="verdict"><div class="mark">Δ</div><div><strong id="verdictTitle">—</strong><p id="verdictBody"></p></div></div>

  <section class="section">
    <div class="section-head">
      <div><div class="eyebrow">01 · toggle the lineage</div><h2>Favorite win rate vs human</h2></div>
      <p>Bars are AI − human on the favorite’s win rate. Switch models to morph the profile. Center is the empirical human edge.</p>
    </div>
    <div class="match-chips" id="matchChips"></div>
    <div class="match-kpis">
      <div class="kpi" style="--tone:var(--star)"><span>Overall mean |Δ|</span><b id="meanAbs">—</b><small id="meanAbsCI">95% bootstrap CI</small></div>
      <div class="kpi" style="--tone:var(--green)"><span>Preserve favorite</span><b id="preserve">—</b><small id="preserveNote">AI still favors the human favorite</small></div>
      <div class="kpi" style="--tone:var(--blue)"><span>Games / matchup</span><b id="gamesN">—</b><small id="protocol">shared schedule</small></div>
      <div class="kpi" style="--tone:var(--gold)"><span>Mean human fav WR</span><b id="humanMean">—</b><small>empirical on mined pairs</small></div>
    </div>
    <div class="compare-grid">
      <div class="card">
        <h3>Matchup Δ vs human</h3>
        <p style="margin-bottom:12px">Positive (coral) = policy overstates the favorite. Negative (teal) = washes or flips the edge.</p>
        <svg class="delta-chart" id="deltaChart" viewBox="0 0 720 460" role="img" aria-label="Matchup win-rate deltas"></svg>
        <div class="expr"><b>Δ</b> = WR<sub>AI</sub>(favorite) − WR<sub>human</sub>(favorite) &nbsp;·&nbsp; <b>mean |Δ|</b> averages absolute pp over these matchups</div>
      </div>
      <div class="card">
        <h3>Lineage closeness</h3>
        <p>Lower mean |Δ| means the policy’s self-play edges sit closer to the corpus.</p>
        <div class="bars" id="lineageBars" style="margin-top:16px"></div>
        <div class="callout" style="margin-top:16px"><b>Read:</b> both seats use the same checkpoint. This tests whether the policy internalizes matchup structure, not whether it beats another AI.</div>
      </div>
    </div>
  </section>

  <section class="section">
    <div class="section-head">
      <div><div class="eyebrow">02 · absolute rates</div><h2>Human vs AI favorite WR</h2></div>
      <p>Same matchups. Hover cells for CI and mean P(favorite).</p>
    </div>
    <div class="table-wrap">
      <table id="rateTable">
        <thead></thead>
        <tbody></tbody>
      </table>
    </div>
  </section>

  <section class="section">
    <div class="section-head">
      <div><div class="eyebrow">03 · how to read it</div><h2>What this can and cannot claim</h2></div>
      <p>Offline sequence self-play judged by another model.</p>
    </div>
    <div class="lessons">
      <div class="card lesson"><div class="n">01 / DATA</div><h3>Wincon archetypes.</h3><p>Each deck is labeled by its most common win condition in the corpus. Favorites are the side with &gt;50% human WR on that ordered pair.</p></div>
      <div class="card lesson"><div class="n">02 / PROTOCOL</div><h3>Shared schedule.</h3><p>Every policy sees the same decks, seats, and RNG seeds. Differences are policy behavior, not sampling noise.</p></div>
      <div class="card lesson"><div class="n">03 / JUDGE</div><h3>Winner model, not live CR.</h3><p>Outcomes come from the symmetric HGB winner ensemble. Treat flips as “offline edge failure,” not ladder proof.</p></div>
    </div>
  </section>

  <footer class="foot">Native HTML + SVG · no matplotlib · offline only · not a live Clash Royale readiness claim.</footer>
</main>
<div class="tip" id="tip"></div>
<script id="report-data" type="application/json">__PAYLOAD__</script>
<script>
const D = JSON.parse(document.getElementById('report-data').textContent);
const $ = s => document.querySelector(s);
const pct = (n, d=1) => (100*n).toFixed(d) + '%';
const pp = (n, d=1) => ((100*n)>=0?'+':'') + (100*n).toFixed(d) + ' pp';
const titleCase = s => String(s).replace(/[-_]/g,' ').replace(/\\b\\w/g, c => c.toUpperCase());
const ease = t => 1 - Math.pow(1-t, 3);
const lerp = (a,b,t) => a + (b-a)*t;

const byId = Object.fromEntries(D.models.map(m => [m.id, m]));
let activeId = byId['v4.4'] ? 'v4.4' : (byId['v4.3'] ? 'v4.3' : D.models[0].id);
let display = JSON.parse(JSON.stringify(byId[activeId]));
let animToken = 0;

$('#stamp').textContent = `${D.created_at || ''} · ${D.setup.policies.map(p=>p.id).join(' · ')} · ${((D.seconds||0)/60)|0}m wall · judge ${D.judge}`;
$('#verdictTitle').textContent = D.verdict || '—';
$('#verdictBody').textContent = D.setup.note || D.note || '';
$('#protocol').textContent = D.protocol || 'shared schedule';
$('#gamesN').textContent = String((D.setup && D.setup.games_per_matchup) || (D.models[0].matchups[0] && D.models[0].matchups[0].games) || '—');
$('#humanMean').textContent = pct(D.matchups.reduce((s,m)=>s+m.empirical_fav_wr,0) / Math.max(D.matchups.length,1));

function paintKpis(model, t=1){
  const from = display, to = model;
  const mean = lerp(from.meanAbsDelta, to.meanAbsDelta, t);
  const preserve = lerp(from.preserveFavoriteRate, to.preserveFavoriteRate, t);
  $('#meanAbs').textContent = (100*mean).toFixed(1) + ' pp';
  const ci = to.meanAbsDeltaCI || [mean, mean];
  $('#meanAbsCI').textContent = `95% CI ${(100*ci[0]).toFixed(1)}–${(100*ci[1]).toFixed(1)} · ${to.matchups.length} matchups`;
  $('#preserve').textContent = pct(preserve, 0);
  $('#preserveNote').textContent = `${to.policyId} · think=${to.thinkSteps||0}`;
}

function drawDelta(model, t=1){
  const svg = $('#deltaChart');
  const rows = model.matchups;
  const fromRows = display.matchups;
  const W=720, H=Math.max(320, 36 + rows.length*48), L=210, R=28, T=18, B=30;
  svg.setAttribute('viewBox', `0 0 ${W} ${H}`);
  const zs = rows.map((r,i) => lerp((fromRows[i] && fromRows[i].delta_wr) || 0, r.delta_wr, t));
  const maxAbs = Math.max(0.12, ...D.models.flatMap(m => m.matchups.map(r => Math.abs(r.delta_wr))));
  const rowH = (H-T-B) / rows.length;
  const x0 = L + (W-L-R)/2;
  const xScale = v => x0 + (v/maxAbs)*((W-L-R)/2);
  let out = `<line x1="${x0}" y1="${T}" x2="${x0}" y2="${H-B}" stroke="#355247" stroke-dasharray="4 4"/>`;
  out += `<text x="${x0}" y="${H-8}" text-anchor="middle" fill="#668078" font-size="11" font-family="IBM Plex Mono,monospace">human edge</text>`;
  rows.forEach((r,i) => {
    const z = zs[i], y = T + i*rowH + rowH*0.18, h = rowH*0.64;
    const x1 = xScale(0), x2 = xScale(z);
    const fill = z>=0 ? '#ff7e78' : '#70e1b1';
    const left = Math.min(x1,x2), width = Math.max(2, Math.abs(x2-x1));
    const label = `${titleCase(r.favorite)} > ${titleCase(r.underdog)}`;
    out += `<text x="${L-8}" y="${y+h*0.72}" text-anchor="end" fill="#9bb6ad" font-size="12" font-family="IBM Plex Mono,monospace">${label}</text>`;
    out += `<rect class="delta-bar" data-i="${i}" x="${left}" y="${y}" width="${width}" height="${h}" rx="5" fill="${fill}" opacity="0.88"/>`;
    out += `<text x="${z>=0?x2+6:x2-6}" y="${y+h*0.72}" text-anchor="${z>=0?'start':'end'}" fill="${fill}" font-size="12" font-family="IBM Plex Mono,monospace" font-weight="700">${pp(z)}</text>`;
  });
  svg.innerHTML = out;
  const tip = $('#tip');
  svg.onmousemove = e => {
    const bar = e.target.closest('.delta-bar');
    if(!bar){ tip.classList.remove('on'); return; }
    const r = rows[+bar.dataset.i];
    tip.innerHTML = `<b>${titleCase(r.favorite)} > ${titleCase(r.underdog)}</b><br>human ${pct(r.empirical_fav_wr)} (n=${r.empirical_n})<br>AI ${pct(r.policy_fav_wr)} ±${pct(r.policy_fav_wr_ci95)}<br>mean P(fav)=${r.policy_mean_P_fav.toFixed(3)}`;
    tip.style.left = (e.clientX+14)+'px';
    tip.style.top = (e.clientY+14)+'px';
    tip.classList.add('on');
  };
  svg.onmouseleave = () => tip.classList.remove('on');
}

function drawLineage(){
  const host = $('#lineageBars');
  const max = Math.max(...D.models.map(m => m.meanAbsDelta), 0.01);
  host.innerHTML = D.models.map(m => `
    <div class="bar-row" data-lineage="${m.id}" style="cursor:pointer">
      <b style="color:${m.color}">${m.id}</b>
      <div class="bar-track"><div class="bar-fill" style="--tone:${m.color}" data-width="${100*m.meanAbsDelta/max}"></div></div>
      <div class="bar-value">${(100*m.meanAbsDelta).toFixed(1)} pp</div>
    </div>`).join('');
  requestAnimationFrame(() => host.querySelectorAll('.bar-fill').forEach(b => b.style.width = b.dataset.width+'%'));
  host.onclick = e => {
    const row = e.target.closest('[data-lineage]');
    if(row) selectModel(row.dataset.lineage);
  };
}

function paintTable(){
  const head = $('#rateTable thead');
  const body = $('#rateTable tbody');
  head.innerHTML = `<tr><th>Matchup</th><th>Human n</th><th class="human">Human fav WR</th>${D.models.map(m=>`<th style="color:${m.color}">${m.id}</th>`).join('')}</tr>`;
  body.innerHTML = D.matchups.map((meta,i) => {
    const cells = D.models.map(m => {
      const r = m.matchups[i];
      const cls = r.preserves_favorite ? 'ok' : 'flip';
      return `<td class="${cls}" title="Δ ${pp(r.delta_wr)} · P=${r.policy_mean_P_fav.toFixed(3)}">${pct(r.policy_fav_wr)}</td>`;
    }).join('');
    return `<tr><td>${titleCase(meta.favorite)} > ${titleCase(meta.underdog)}</td><td>${meta.empirical_n}</td><td class="human">${pct(meta.empirical_fav_wr)}</td>${cells}</tr>`;
  }).join('');
}

function selectModel(id){
  if(!byId[id]) return;
  const target = byId[id];
  activeId = id;
  [...$('#matchChips').children].forEach(c => {
    const on = c.dataset.model===id;
    c.classList.toggle('active', on);
    if(on) c.style.setProperty('--tone', target.color);
  });
  const token = ++animToken;
  const start = performance.now();
  const dur = 420;
  (function frame(now){
    if(token !== animToken) return;
    const t = ease(Math.min(1, (now-start)/dur));
    paintKpis(target, t);
    drawDelta(target, t);
    if(t < 1) requestAnimationFrame(frame);
    else display = JSON.parse(JSON.stringify(target));
  })(start);
}

$('#matchChips').innerHTML = D.models.map(m =>
  `<button type="button" class="match-chip${m.id===activeId?' active':''}" data-model="${m.id}" style="--tone:${m.color}">${m.id}</button>`
).join('');
$('#matchChips').onclick = e => {
  const b = e.target.closest('[data-model]');
  if(b) selectModel(b.dataset.model);
};

paintKpis(byId[activeId], 1);
drawDelta(byId[activeId], 1);
drawLineage();
paintTable();
display = JSON.parse(JSON.stringify(byId[activeId]));
</script>
</body>
</html>
"""
    out.write_text(html.replace("__PAYLOAD__", data), encoding="utf-8")
    return out
