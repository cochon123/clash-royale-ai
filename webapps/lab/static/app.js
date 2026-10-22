'use strict';
const $ = (q, root = document) => root.querySelector(q);
const $$ = (q, root = document) => [...root.querySelectorAll(q)];
const esc = value => String(value ?? '').replace(/[&<>"']/g, ch => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[ch]));
const arr = value => Array.isArray(value) ? value : [];
const dict = value => value && typeof value === 'object' && !Array.isArray(value) ? value : {};
const fmt = value => typeof value === 'number' && Number.isFinite(value) ? new Intl.NumberFormat(undefined, {maximumFractionDigits: 2}).format(value) : 'unknown';
const bytes = value => {
  if (typeof value !== 'number') return 'unknown';
  if (value < 1024) return fmt(value) + ' B';
  const i = Math.min(4, Math.floor(Math.log(value) / Math.log(1024)));
  return fmt(value / 1024 ** i) + ' ' + ['B','KiB','MiB','GiB','TiB'][i];
};
const duration = seconds => {
  if (typeof seconds !== 'number') return 'unknown';
  const m = Math.floor(Math.max(0, seconds) / 60);
  return m >= 60 ? `${Math.floor(m / 60)}h ${m % 60}m` : m ? `${m}m` : `${Math.floor(seconds)}s`;
};
const date = value => value && Number.isFinite(new Date(value).getTime()) ? new Date(value).toLocaleString() : 'unknown';
const percent = value => typeof value === 'number' ? fmt(value * 100) + '%' : 'unknown';
const src = (value, label = '') => `<span class="source" title="${esc(value)}">${esc(label || value || 'Source unknown')}</span>`;
const hint = text => `<span class="info" tabindex="0" title="${esc(text)}" aria-label="${esc(text)}">i</span>`;
const pill = (value, label = value) => `<span class="pill ${esc(value)}">${esc(label || 'unknown')}</span>`;
const empty = text => `<div class="empty">${esc(text)}</div>`;
const json = (data, label = 'Recorded metadata', source = '') => `<details><summary>${esc(label)}</summary>${source ? src(source) : ''}<pre>${esc(JSON.stringify(data, null, 2))}</pre></details>`;
const link = (url, label, title = '') => `<a href="${esc(url)}" ${title ? `title="${esc(title)}"` : ''}>${esc(label)}</a>`;
function localService(url) {
  try {
    const parsed = new URL(url);
    if (['127.0.0.1','localhost'].includes(parsed.hostname)) parsed.hostname = location.hostname;
    return parsed.href;
  } catch { return '#'; }
}
async function api(path) {
  const response = await fetch(path, {signal: AbortSignal.timeout(15000)});
  if (!response.ok) {
    let text;
    try { text = (await response.json()).error; } catch { text = response.statusText; }
    throw new Error(`${response.status}: ${text}`);
  }
  return response.json();
}
function route() {
  const raw = location.hash.slice(1) || location.pathname.slice(1) || 'overview';
  const [page, query = ''] = raw.split('?');
  return {page: ['overview','models','runs','engine','data','docs'].includes(page) ? page : 'overview', query: new URLSearchParams(query)};
}
function heading(eyebrow, title, subtitle, tag = 'LOCAL RESEARCH') {
  return `<div class="page-heading"><div><div class="eyebrow">${esc(eyebrow)}</div><h1>${esc(title)}</h1><p class="subtitle">${esc(subtitle)}</p></div>${pill('', tag)}</div>`;
}
function stat(label, value, note, source, help = '') {
  return `<div class="stat" title="${esc(source)}"><div class="stat-label">${esc(label)} ${help ? hint(help) : ''}</div><div class="stat-value">${esc(value)}</div><div class="stat-note">${esc(note)}</div>${src(source)}</div>`;
}
function section(title, note = '', action = '') {
  return `<div class="section-head"><div><h2>${esc(title)}</h2>${note ? `<p>${esc(note)}</p>` : ''}</div>${action}</div>`;
}
function runLink(id) { return '#runs?id=' + encodeURIComponent(id); }
function runTable(rows) {
  if (!rows.length) return empty('No runs match this view. Missing artifacts appear as unknown.');
  return `<div class="table-wrap"><table><thead><tr><th>Run / evidence</th><th>Type</th><th>Status</th><th>Games</th><th>Wins / losses</th><th>Latest eval</th><th>Elapsed</th></tr></thead><tbody>${rows.map(r => `<tr><td>${link(runLink(r.id), r.id)}${src(r.metric_source)}</td><td>${esc(r.kind)}</td><td title="${esc(arr(r.evidence).join('; '))}">${pill(r.status)}</td><td title="${esc(r.metric_source + '#counters or completed_games')}">${fmt(r.counters?.games)}</td><td title="${esc(r.metric_source + '#counters or models.*.stats')}">${fmt(r.counters?.wins)} / ${fmt(r.counters?.losses)}</td><td title="${esc(r.metric_source + '#evaluation.history · ' + (r.latest_evaluation?.bank || '') + ' · ' + (r.latest_evaluation?.mode || ''))}">${percent(r.latest_evaluation?.score)}</td><td title="${esc(r.metric_source + ' · recorded elapsed, or start to last update')}">${duration(r.elapsed_seconds)}</td></tr>`).join('')}</tbody></table></div>`;
}
function servicePanel(services) {
  return `<div class="panel">${arr(services).map(s => `<div class="service"><div><h3>${link(localService(s.url), s.name + ' ↗')}</h3><p class="muted">${s.data?.collector?.paused ? 'Collector paused · ' + esc(s.data.collector.pause_reason || 'reason unknown') : s.reachable ? 'Responding on this machine' : 'Unreachable at last observation'}</p>${src(s.source)}${s.error ? `<span class="source">${esc(s.error)}</span>` : ''}</div>${pill(s.data?.collector?.paused ? 'unknown' : s.reachable ? 'ok' : 'unknown', s.data?.collector?.paused ? 'paused' : s.reachable ? 'online' : 'offline')}</div>${s.data ? json(s.data, s.name + ' response', s.source) : ''}`).join('') || empty('First service observation pending.')}</div>`;
}
let overviewState = null;
function overviewHTML(d) {
  overviewState = d;
  const active = arr(d.active_runs), trainers = arr(d.processes).filter(p => p.category === 'training');
  const latest = active[0] || arr(d.recent_runs).find(r => r.kind === 'training run');
  const status = d.training_status;
  const title = status === 'running' ? 'Training activity detected' : status === 'idle' ? 'No training observed' : 'Training status is unknown';
  const hw = dict(d.hardware), gpu = arr(hw.gpu)[0], mem = dict(hw.memory), disk = dict(hw.disk);
  const text = active.length ? active.map(r => r.id).join(', ') : trainers.length ? trainers.map(p => `PID ${p.pid}`).join(', ') : status === 'idle' ? 'No trainer process, active training service, or recent active run state was found.' : 'One or more observation sources are unavailable. See the evidence below.';
  return heading('01 / LIVE OBSERVATIONS', 'The research, at a glance.', 'Models, experiments and the machine behind them. A live view of local evidence.', 'REFRESH / 5 SEC') +
    `<div class="hero ${esc(status)}"><div class="hero-head"><div><div class="status-line">${pill(status === 'running' ? 'running' : status === 'idle' ? 'finished' : 'unknown', status === 'running' ? '● LIVE TRAINING' : status === 'idle' ? '● IDLE' : '● UNKNOWN')}<span class="source-inline">${esc(date(d.sampled_at))}</span></div><h2>${esc(title)}</h2><p>${esc(text)}</p>${latest ? `<p>Latest run: ${link(runLink(latest.id), latest.id)} · ${esc(latest.phase || latest.status)}</p>` : ''}${latest ? `<p class="file-meta" title="${esc(latest.metric_source)}">${pill(latest.status)} · Elapsed ${esc(duration(latest.elapsed_seconds))}${latest.status === 'running' ? ' · Remaining ' + esc(duration(latest.remaining_seconds)) : ''} · Deadline ${esc(date(latest.deadline))} · Wins / losses ${fmt(latest.counters?.wins)} / ${fmt(latest.counters?.losses)}</p>` : ''}${src(d.source)}</div><span class="hero-mark" aria-hidden="true">⌁</span></div></div>` +
    `<div class="stats">${stat('Latest run · games', fmt(latest?.counters?.games), latest?.id || 'No run state', latest?.metric_source || 'engine/runs/*/state.json', 'Recorded total games; training and evaluation may both be included.')}${stat('Latest run · evaluation', percent(latest?.latest_evaluation?.score), latest?.latest_evaluation ? `${latest.latest_evaluation.bank || 'bank unknown'} · ${latest.latest_evaluation.mode || 'mode unknown'}` : 'No evaluation recorded', (latest?.metric_source || 'engine/runs/*/state.json') + '#evaluation.history', 'Wins plus half the draws, divided by games. This is the latest recorded bank, not necessarily the best checkpoint.')}${stat('GPU utilization', gpu ? fmt(gpu.utilization) + '%' : 'unknown', gpu?.name || 'nvidia-smi unavailable', 'nvidia-smi · whole device, all processes', 'Whole-device utilization; this includes services unrelated to this project.')}${stat('Memory in use', bytes(mem.used), `${bytes(mem.available)} available`, '/proc/meminfo', 'Total minus MemAvailable; includes every process on the host.')}</div>` +
    `<div class="grid"><div><div class="panel"><div class="panel-head"><div><h2>Machine pulse</h2><span class="muted">Whole host, sampled every five seconds</span></div>${pill('', 'HARDWARE')}</div><div class="service"><div>CPU load <span class="muted">1 / 5 / 15 min</span>${src('/proc/loadavg')}</div><span>${arr(hw.load).map(fmt).join(' / ') || 'unknown'}</span></div><div class="service"><div>GPU memory${src('nvidia-smi · MiB')}</div><span>${gpu ? fmt(gpu.memory_used_mib) + ' / ' + fmt(gpu.memory_total_mib) + ' MiB' : 'unknown'}</span></div><div class="service"><div>Disk free${src('statvfs · repository filesystem')}</div><span>${bytes(disk.free)} <span class="muted">/ ${bytes(disk.total)}</span></span></div>${arr(hw.errors).length ? json(hw.errors, 'Unavailable hardware observations') : ''}</div>${section('Observation evidence', 'Workers and dashboards alone do not imply training.')}<div class="panel"><p class="muted">${esc(d.status_explanation)}</p><ul class="evidence"><li>Process scan: ${d.process_scan_ok ? 'available' : 'incomplete / unknown'}</li><li>User service scan: ${d.unit_scan_ok ? 'available' : 'unknown'}</li></ul>${src('/proc/*/cmdline; systemctl --user list-units --all --output=json')}${json(d.processes, 'Observed processes (' + arr(d.processes).length + ')', '/proc/*/cmdline')}${json(d.units, 'Observed user services', 'systemctl --user list-units --all --output=json')}${d.error || arr(d.errors).length ? json({error:d.error, errors:d.errors}, 'Observation errors') : ''}</div></div><div><div class="panel-head"><div><h2>Local services</h2><span class="muted">Links use this browser’s host for LAN access</span></div>${pill('', 'GET ONLY')}</div>${servicePanel(d.services)}</div></div>` +
    section('Recent experiments', 'State, checkpoints and outcomes from the local run folders.', link('#runs', 'Browse all runs →')) + runTable(arr(d.recent_runs));
}
function metricValue(metric) {
  const name = metric.name || '';
  return /acc|auc|slot_top|joint_card|score|(?:^|\.)(?:card|joint|cell)$/.test(name) && metric.value >= 0 && metric.value <= 1 ? percent(metric.value) : fmt(metric.value);
}
function playableCard(m) {
  return `<article class="panel model-card ${m.recommended ? 'recommended' : ''}"><div class="panel-head"><div><div class="eyebrow">${esc(m.kind || 'model')} / ENGINE READY</div><h2>${esc(m.name)}</h2></div>${pill(m.available ? 'ok' : 'unknown', m.available ? 'available' : 'missing weights')}</div><div class="metadata">${m.recommended ? pill('ok', 'recommended') : ''}${pill('', m.trained_deck_name || m.trained_deck || 'any supported deck')}</div><p class="description">${esc(m.summary)}</p>${m.result ? `<p class="result">${esc(m.result)}</p>` : ''}<div class="file-meta" title="${esc(m.checkpoint || 'models/registry.json · checkpoint null')}">${m.weights ? `${m.weights.present ? 'Weights · ' + bytes(m.weights.bytes) : 'Weights absent locally'}${src(m.weights.path)}` : 'No checkpoint required · built-in baseline'}</div>${src(m.source)}<div class="actions">${m.run ? link(runLink(m.run.replace(/^engine\/runs\//,'')), 'Inspect run →') : ''}${link('#models?id=' + encodeURIComponent(m.id), 'Metadata →')}</div></article>`;
}
function offlineCard(m) {
  const metrics = arr(m.metrics).filter(x => /test/.test(x.name)).slice(0,4);
  const shown = metrics.length ? metrics : arr(m.metrics).slice(0,4);
  const weights = arr(m.weights), present = weights.filter(w => w.present);
  return `<article class="panel model-card"><div class="panel-head"><div><div class="eyebrow">${esc(m.family)} / OFFLINE RESEARCH</div><h2>${link('#models?id=' + encodeURIComponent(m.id), m.name || m.id)}</h2></div>${pill(m.experimental ? 'unknown' : '', m.experimental ? 'experimental' : 'replay')}</div><span class="muted">${esc(m.id)}${m.version ? ' · v' + esc(m.version) : ''}</span><div class="metric-list">${shown.map(x => `<div title="${esc(x.explanation + '\n' + x.source)}"><div class="value">${esc(metricValue(x))}</div><small>${esc(x.name)}</small></div>`).join('') || '<p class="muted">No recognized metrics in readable metadata.</p>'}</div><div class="actions">${link('#models?id=' + encodeURIComponent(m.id), 'Metrics & artifacts →')}${arr(m.reports).slice(0,1).map(r => link(r.url, 'HTML report ↗', r.path)).join('')}</div><div class="file-meta">${present.length ? `${present.length} local weight artifact${present.length === 1 ? '' : 's'} · ${bytes(present.reduce((s,w) => s + (w.bytes || 0), 0))}${weights.length > present.length ? ' · ' + (weights.length-present.length) + ' expected artifact(s) absent' : ''}` : 'No local weights found · may be backed up on Hugging Face'}</div>${src(m.source)}</article>`;
}
function modelDetail(m, kind) {
  let out = `<a class="back" href="#models">← All models</a>` + heading('02 / MODEL ARTIFACT', m.name || m.id, m.summary || (m.experimental ? 'Experimental research artifact. Not promoted to the playable registry.' : 'Replay-trained model and supporting evidence.'), kind === 'playable' ? 'ENGINE MODEL' : 'OFFLINE MODEL');
  if (kind === 'playable') return out + playableCard(m) + json(m, 'Full registry entry', m.source);
  const metrics = arr(m.metrics);
  out += `<div class="panel"><h2>Metrics from model artifacts</h2><p class="muted">Datasets and objectives differ between versions. Read the metric definition and source before comparing.</p>${src(m.source)}<div class="metric-list">${metrics.map(x => `<div title="${esc(x.explanation)}"><div class="value">${esc(metricValue(x))} ${hint(x.explanation)}</div><small>${esc(x.name)}</small>${src(x.source)}</div>`).join('') || '<p class="muted">Unknown — no recognized numeric metrics found.</p>'}</div></div>`;
  out += section('Local weights', 'Presence and size only; weight files are never loaded.') + weightTable(m.weights);
  out += section('Reports & model cards') + `<div class="panel report-links">${arr(m.reports).map(r => link(r.url,r.name,r.path)).join('')}${arr(m.cards).map(r => link('/model-card?path=' + encodeURIComponent(r.path),r.name,r.path)).join('') || ''}${!arr(m.reports).length && !arr(m.cards).length ? '<span class="muted">No matching reports or model cards.</span>' : ''}</div>`;
  out += section('Recorded metadata', m.scan_scope) + arr(m.artifacts).map(a => json(a.data, a.path + (a.readable ? '' : ' · unknown / unreadable'), a.path)).join('');
  return out;
}
function weightTable(weights) {
  if (!arr(weights).length) return empty('No local weight artifacts found in the bounded scan. Binary backups may exist on Hugging Face.');
  return `<div class="table-wrap"><table><thead><tr><th>Checkpoint / artifact</th><th>Presence</th><th>Size</th><th>Modified (filesystem)</th></tr></thead><tbody>${weights.map(w => `<tr><td>${esc(w.path)}</td><td>${pill(w.present ? 'ok' : 'unknown', w.present ? 'present' : 'absent')}</td><td title="${esc(w.path + ' · stat.st_size')}">${bytes(w.bytes)}</td><td title="${esc(w.path + ' · stat.st_mtime')}">${date(w.modified_at)}</td></tr>`).join('')}</tbody></table></div>`;
}
let modelData = null;
function modelsHTML(d) {
  modelData = d;
  return heading('02 / MODEL CATALOG', 'Every model. Its evidence.', 'Playable policies first, then the full replay and research collection. Local weight presence is separate from model quality.') +
    section('Playable in the recovered engine', 'Curated registry · Banana family and random baseline', src('models/registry.json')) +
    `<div class="grid">${arr(d.playable).map(playableCard).join('') || empty('Registry is unavailable.')}</div>` +
    section('Replay models & experiments', 'Behaviour cloning, auxiliary predictors and controlled experiments.') +
    `<div class="toolbar"><input id="model-search" type="search" placeholder="Find a model, version or metric…" aria-label="Search models"><select id="model-family" aria-label="Model family"><option value="">All families</option><option>policy</option><option>auxiliary</option><option>experiment</option></select></div><div id="offline-models" class="grid three">${arr(d.offline).map(offlineCard).join('')}</div>` +
    src('models/ · catalog cached 60s · sampled ' + date(d.sampled_at)) + (arr(d.errors).length ? json(d.errors,'Catalog errors') : '');
}
let runData = null;
function runsHTML(d) {
  runData = d;
  return heading('03 / EXPERIMENT LOG', 'Follow the run.', 'Training, benchmarks, human play, watch sessions and evaluation. Open an experiment for curves and its original metadata.') +
    `<div class="toolbar"><input id="run-search" type="search" placeholder="Search run folders…" aria-label="Search runs"><select id="run-kind" aria-label="Run type"><option value="">All run types</option>${['training run','benchmark','play session','watch','eval','other'].map(x => `<option>${x}</option>`).join('')}</select><select id="run-status" aria-label="Run status"><option value="">All statuses</option>${['running','finished','stopped','stale','unknown'].map(x => `<option>${x}</option>`).join('')}</select></div><div id="run-table">${runTable(arr(d.runs))}</div>${src('engine/runs/ · cached 5s · nested banana-play sessions included')}`;
}
function runDetailHTML(d) {
  const r = d.summary, c = dict(r.counters), at = r.metric_source;
  let out = `<a class="back" href="#runs">← All runs</a>` + heading('03 / ' + r.kind.toUpperCase(), r.id, r.phase || 'No phase recorded in state.json.', r.status.toUpperCase());
  out += `<div class="panel"><div class="panel-head"><div>${pill(r.status)} <span class="muted">Reported status: ${esc(r.reported_status)}</span></div>${r.dashboard ? link(localService(r.dashboard), 'Open run dashboard ↗') : ''}</div><ul class="evidence">${arr(r.evidence).map(x => `<li>${esc(x)}</li>`).join('')}</ul><p class="file-meta">Started ${esc(date(r.started_at))} · Updated ${esc(date(r.updated_at))} · State age ${esc(duration(r.state_age_seconds))}</p>${r.error ? `<p class="notice">${esc(r.error)}</p>` : ''}${src(at + '; STOP; /proc/*/cmdline; systemd user units')}</div>`;
  out += `<div class="stats">${stat('Games',fmt(c.games),`${fmt(c.training_games)} train · ${fmt(c.evaluation_games)} eval`,at + '#counters / completed_games')}${stat('Wins / losses',`${fmt(c.wins)} / ${fmt(c.losses)}`,`${fmt(c.draws)} draws`,at + '#counters / models.*.stats')}${stat('Elapsed',duration(r.elapsed_seconds),'Recorded elapsed or start → last update',at + '#elapsed_seconds / started_at / updated_at')}${stat('Time to deadline',r.status === 'running' ? duration(r.remaining_seconds) : '—',date(r.deadline),at + '#deadline',r.status === 'running' ? 'Remaining wall time; an expired deadline does not stop a process.' : 'This run is not currently observed as running.')}</div>`;
  out += section('Run curves', 'Hover or tap for values and provenance. Toggle a legend to compare curves.');
  out += '<div id="run-charts" class="grid"></div>';
  out += arr(d.logs).some(l => l.partial || l.malformed) ? `<p class="notice">Some logs are windowed or contain incomplete lines. Counts below describe the loaded window, not the whole run.</p>` : '';
  out += section('Log readers', 'At most 8 MiB / 12,000 lines per file; half-written lines are skipped.') + arr(d.logs).map(l => json(l, l.source + ' · ' + fmt(l.records) + ' records loaded', l.source)).join('');
  out += section('Checkpoints') + weightTable(d.weights);
  out += section('Run metadata') + json(d.state, 'State snapshot (large arrays abbreviated)', at) + arr(d.artifacts).map(a => json(a.data, a.path, a.path)).join('') + json(d.files, 'Root files · sizes and modification times',r.source);
  return out;
}
const COLORS = ['#b8ec8b','#92c8e7','#eac47c','#db9ddd','#ed9c91','#90d8c9','#bac4ff','#d1be98'];
function chart(container, title, curves, explanation) {
  if (!curves.length) return;
  const panel = document.createElement('section');
  panel.className = 'panel chart';
  panel.innerHTML = `<h3>${esc(title)}</h3><p class="chart-help">${esc(explanation)}</p><div class="legend"></div><div class="plot"></div>${src([...new Set(curves.map(c => c.source))].join('; '))}`;
  container.append(panel);
  const visible = curves.map(() => true);
  const legend = $('.legend',panel), plot = $('.plot',panel);
  curves.forEach((c,i) => {
    const b = document.createElement('button');
    b.type = 'button'; b.style.setProperty('--curve',COLORS[i % COLORS.length]);
    b.innerHTML = `<span class="legend-dot"></span>${esc(c.name)}`;
    b.title = c.explanation; b.setAttribute('aria-pressed','true');
    b.onclick = () => { visible[i] = !visible[i]; b.classList.toggle('off',!visible[i]); b.setAttribute('aria-pressed',String(visible[i])); draw(); };
    legend.append(b);
  });
  const W = 660,H = 260,L = 53,R = 14,T = 17,B = 40;
  function draw() {
    const points = curves.flatMap((c,i) => visible[i] ? c.points : []).filter(p => p.every(Number.isFinite));
    if (!points.length) { plot.innerHTML = '<div class="chart-empty">Choose a curve to display.</div>'; return; }
    const xs = points.map(p => p[0]), ys = points.map(p => p[1]);
    let minX = Math.min(...xs),maxX = Math.max(...xs), minY = Math.min(...ys),maxY = Math.max(...ys);
    if (minX === maxX) {minX -= 1;maxX += 1;}
    if (minY === maxY) {minY -= Math.abs(minY) * .1 || .05;maxY += Math.abs(maxY) * .1 || .05;}
    const padding = (maxY - minY) * .08; minY -= padding;maxY += padding;
    const x = n => L + (n - minX)/(maxX - minX)*(W - L - R);
    const y = n => H - B - (n - minY)/(maxY - minY)*(H - T - B);
    const isTime = curves[0].x_label === 'time';
    let svg = `<svg viewBox="0 0 ${W} ${H}" role="img" aria-label="${esc(title)}. Use the legend buttons to toggle curves."><title>${esc(title)}</title>`;
    for (let i = 0;i <= 4;i++) {
      const v = minY + (maxY - minY)*i/4,py = y(v);
      svg += `<line x1="${L}" x2="${W-R}" y1="${py}" y2="${py}" stroke="#303b33" stroke-dasharray="3 4"/><text x="${L-9}" y="${py+3}" text-anchor="end" fill="#91a092" font-size="10">${esc(fmt(v))}</text>`;
    }
    for (let i = 0;i <= 3;i++) {
      const v = minX + (maxX - minX)*i/3;
      const label = isTime ? new Date(v*1000).toLocaleTimeString([],{hour:'2-digit',minute:'2-digit'}) : fmt(v);
      svg += `<text x="${x(v)}" y="${H-15}" text-anchor="middle" fill="#91a092" font-size="10">${esc(label)}</text>`;
    }
    curves.forEach((c,i) => {
      if (!visible[i]) return;
      // Preserve the full API data; decimate the SVG only for responsive rendering.
      const stride = Math.max(1,Math.ceil(c.points.length/700));
      const pts = c.points.filter((p,index) => index % stride === 0 || index === c.points.length-1);
      const color = COLORS[i%COLORS.length];
      svg += `<polyline points="${pts.map(p => `${x(p[0]).toFixed(2)},${y(p[1]).toFixed(2)}`).join(' ')}" fill="none" stroke="${color}" stroke-width="2" stroke-linejoin="round"/>`;
      if (pts.length < 40) svg += pts.map(p => `<circle cx="${x(p[0])}" cy="${y(p[1])}" r="3" fill="${color}"/>`).join('');
    });
    svg += `<line class="crosshair" x1="0" x2="0" y1="${T}" y2="${H-B}" stroke="#b8c9b9" opacity="0" stroke-dasharray="3 3"/></svg>`;
    plot.innerHTML = svg;
    const element = $('svg',plot),tooltip = $('#tooltip');
    function move(event) {
      const rect = element.getBoundingClientRect(), localX = (event.clientX - rect.left)/rect.width*W;
      const valueX = minX + Math.max(0,Math.min(1,(localX-L)/(W-L-R)))*(maxX-minX);
      const values = [];
      curves.forEach((c,i) => {
        if (!visible[i] || !c.points.length) return;
        const p = c.points.reduce((best,p) => Math.abs(p[0]-valueX) < Math.abs(best[0]-valueX) ? p : best);
        values.push(c.name + ': ' + fmt(p[1]) + '\n' + (isTime ? date(p[0]*1000) : c.x_label + ' ' + fmt(p[0])));
      });
      tooltip.textContent = values.join('\n\n') + '\n\nSource: ' + [...new Set(curves.filter((c,i) => visible[i]).map(c => c.source))].join('; ');
      tooltip.hidden = false;
      const left = Math.min(window.innerWidth-tooltip.offsetWidth-12,event.clientX+12);
      tooltip.style.left = Math.max(8,left) + 'px';
      tooltip.style.top = Math.max(8,Math.min(window.innerHeight-tooltip.offsetHeight-8,event.clientY+15)) + 'px';
      const line = $('.crosshair',element); line.setAttribute('x1',x(valueX));line.setAttribute('x2',x(valueX));line.setAttribute('opacity','.65');
    }
    element.onpointermove = move;
    element.onpointerdown = move;
    element.onpointerleave = () => {tooltip.hidden=true;$('.crosshair',element).setAttribute('opacity','0');};
  }
  draw();
}
function mountRunCharts(data) {
  const node = $('#run-charts');
  if (!node) return;
  const curves = arr(data.charts);
  if (!curves.length) {node.innerHTML = empty('No timestamped numeric curves are available for this run. Inspect the recorded metadata below.');return;}
  const evals = curves.filter(c => c.source.includes('#evaluation'));
  const winRates = curves.filter(c => c.name.includes('recent win rate'));
  const outcomes = curves.filter(c => / · (wins|losses|draws)$/.test(c.name));
  const speeds = curves.filter(c => / · (wins|losses|net wins)\/min$/.test(c.name));
  const rates = curves.filter(c => c.name.includes('games/min') || c.name.includes('Completed game'));
  chart(node,'Evaluation score',evals,'Separate selection and confirmation banks; score = (wins + ½ draws) / games.');
  chart(node,'Recorded training win rates',winRates,'Recent training results as recorded by the trainer, indexed by game.');
  const groups = new Set(outcomes.map(c => c.name.replace(/ · (wins|losses|draws)$/,'')));
  for (const group of groups) {
    const curves=outcomes.filter(c => c.name.replace(/ · (wins|losses|draws)$/,'') === group);
    const groupSpeeds = speeds.filter(c => c.name.startsWith(group + ' · '));
    const speedMount = mount => {if (groupSpeeds.length) chart(mount,'Outcome speed · '+group,groupSpeeds,'Wins and losses per minute over the trailing ten minutes — the derivative of the cumulative outcome curves above. Net wins/min above zero means results are improving; below zero, worsening.');};
    if (group === 'training' || groups.size <= 3) {chart(node,'Outcomes · ' + group,curves,'Cumulative outcomes in the loaded log window.');speedMount(node);}
    else {
      const details=document.createElement('details');
      details.innerHTML='<summary>'+esc('Outcome breakdown · '+group)+'</summary>';
      node.append(details);
      let mounted=false;
      details.ontoggle=() => {if (details.open && !mounted) {mounted=true;chart(details,'Outcomes · '+group,curves,'Cumulative outcomes in the loaded log window.');speedMount(details);}};
    }
  }
  chart(node,'Throughput',rates,'Games per trailing minute, or recorded game events. See each curve’s definition.');
  for (const c of curves.filter(c => ![...evals,...winRates,...outcomes,...speeds,...rates].includes(c))) chart(node,c.name,[c],c.explanation);
}
function engineHTML(d) {
  return heading('04 / SIMULATION INFRASTRUCTURE', 'Inside the engines.', 'The original recovered game, the fast training engine, and the validation evidence connecting them.') +
    `<div class="grid">${arr(d.components).map(c => `<div class="panel"><h2>${esc(c.name)}</h2><p class="muted">${esc(c.description)}</p>${src(c.source)}</div>`).join('')}</div>` +
    section('Built binaries', 'File modification time is shown as build evidence; it is not a verified compiler timestamp.') + weightTable(d.binaries) +
    section('Validation records', 'Recorded comparisons are evidence for the tested paths, not proof of complete engine equivalence.') +
    arr(d.validations).map(v => `<div class="panel" style="margin-bottom:14px"><h3>${esc(v.path.split('/').pop())}</h3>${src(v.path)}${validationSummary(v)}${json(v.data,'Inspect validation record (large arrays abbreviated)',v.path)}</div>`).join('') +
    section('Training deck presets', 'The six saved decks shared with the engine and Arena.') + `<div class="grid three">${arr(d.presets).map(p => `<article class="panel"><div class="panel-head"><h3>${esc(p.name)}</h3>${pill('',p.archetype || 'deck')}</div><div class="deck-cards">${arr(p.cards).map(id => {const c = dict(d.cards?.[id]);return `<img src="${esc(c.art || 'data:,')}" alt="${esc(c.name || id)}" title="${esc((c.name || id) + ' · ' + c.elixir + ' elixir · ' + d.preset_source)}" loading="lazy">`;}).join('')}</div><p class="muted">${arr(p.cards).map(id => esc(d.cards?.[id]?.name || id)).join(' · ')}</p><span class="file-meta" title="${esc(d.preset_source + '; shared card catalog')}">Average elixir: ${fmt(p.average_elixir)}</span>${src(d.preset_source + '#' + p.id)}</article>`).join('')}</div>${arr(d.errors).length ? json(d.errors,'Unavailable catalog data') : ''}`;
}
function validationSummary(v) {
  const d = dict(v.data), reg = dict(d.league_regression), accuracy = dict(d.accuracy), overall = dict(accuracy.overall);
  const vals = [];
  if (typeof d.target_passed === 'boolean') vals.push(['Recorded target',d.target_passed ? 'passed' : 'failed','target_passed']);
  for (const [key,label] of [['total_games','Games compared'],['fully_matched','Fully matched'],['requests_compared','Requests compared']]) if (typeof reg[key] === 'number') vals.push([label,fmt(reg[key]),'league_regression.'+key]);
  for (const [key,label] of [['games','Games compared'],['winner_agreement','Winner agreement'],['every_reply_exact','Every reply exact']]) if (typeof overall[key] === 'number') vals.push([label,key === 'winner_agreement' ? percent(overall[key]) : fmt(overall[key]),'accuracy.overall.'+key]);
  return vals.length ? `<div class="metric-list">${vals.map(([label,value,key]) => `<div><div class="value">${esc(value)}</div><small>${esc(label)}</small>${src(v.path + '#' + key)}</div>`).join('')}</div>` : '<p class="muted">Expand the record for the recorded validation scope and results.</p>';
}
function dataHTML(d) {
  return heading('05 / REPLAY CORPUS', 'From human play to training data.', 'A bounded inventory of replay captures, derived datasets, logs and interactive reports. Large cache trees stay untouched.') +
    `<div class="stats">${stat('Replay capture files',fmt(d.raw?.files),'Direct regular files · cached five minutes',d.raw?.source || 'data/raw/',d.raw?.scope || '')}${stat('Raw capture size',bytes(d.raw?.bytes),'Sum of direct file sizes',d.raw?.source || 'data/raw/')}${stat('Interactive reports',fmt(arr(d.reports).length),'Top-level HTML report files','reports/ · shallow directory listing')}${stat('Training log files',fmt(arr(d.logs).length),'Top-level files only','logs/ · shallow directory listing')}</div>` +
    `<p class="muted">${esc(d.note)} Snapshot: ${esc(date(d.sampled_at))}.</p>${d.raw?.error ? `<p class="notice">${esc(d.raw.error)}</p>` : ''}` +
    section('Replay collector','Health response from the local collector service.') + (d.collector ? servicePanel([d.collector]) : empty('Collector health observation pending.')) +
    section('Data areas','Sizes are unknown when a directory has not been walked.') + `<div class="grid three">${arr(d.folders).map(f => `<div class="panel"><h3>${esc(f.name)}</h3><p class="muted">${esc(f.note)}</p>${src(f.source)}</div>`).join('')}</div>` +
    section('Interactive research reports','Existing HTML reports served directly from the repository.') + `<div class="panel"><div class="toolbar"><input id="report-search" type="search" placeholder="Filter report names…" aria-label="Filter reports"></div><div id="report-list" class="path-list">${arr(d.reports).map(r => `<div data-report="${esc(r.name.toLowerCase())}">${link(r.url,r.name,r.path)}</div>`).join('')}</div></div>` +
    section('Training logs','Presence, size and modification time; no expensive log traversal.') + weightTable(d.logs);
}
let docsData = null;
function docsHTML(d, selected) {
  docsData = d;
  const groups = [...new Set(arr(d.documents).map(x => x.group))];
  return heading('06 / RESEARCH NOTEBOOK', 'The reasoning behind the runs.', 'Research notes and recovered-engine documentation, rendered locally from Markdown.') +
    `<div class="doc-layout"><aside class="panel doc-list"><input id="doc-search" type="search" placeholder="Find a document…" aria-label="Search documents" style="width:100%">${groups.map(g => `<div class="doc-group">${esc(g)}</div>${arr(d.documents).filter(x => x.group === g).map(x => `<a class="${x.path === selected ? 'active' : ''}" href="#docs?doc=${encodeURIComponent(x.path)}" data-doc="${esc(x.path.toLowerCase())}" title="${esc(x.path)}">${esc(x.name)}</a>`).join('')}`).join('')}</aside><article class="panel"><div id="document-content">${empty('Choose a document to read. Links and embedded resources are handled locally.')}</div></article></div>`;
}
let generation = 0;
let busy = false;
async function render() {
  const token = ++generation;
  busy = true;
  $('#tooltip').hidden = true;
  const current = route(), page = current.page;
  $('#breadcrumb').textContent = page[0].toUpperCase()+page.slice(1);
  document.title = page[0].toUpperCase()+page.slice(1)+' · Clash Lab';
  $$('nav a').forEach(a => { a.classList.toggle('active',a.dataset.page === page);if (a.dataset.page === page) a.setAttribute('aria-current','page');else a.removeAttribute('aria-current'); });
  const main = $('#main');
  main.innerHTML = '<div class="loading">Reading local research artifacts…</div>';
  try {
    let data,content;
    const id = current.query.get('id');
    if (page === 'overview') {data=await api('/api/overview');content=overviewHTML(data);}
    else if (page === 'models' && id) {data=await api('/api/models/'+encodeURIComponent(id));content=modelDetail(data.model,data.kind);}
    else if (page === 'models') {data=await api('/api/models');content=modelsHTML(data);}
    else if (page === 'runs' && id) {data=await api('/api/runs/'+encodeURIComponent(id));content=runDetailHTML(data);}
    else if (page === 'runs') {data=await api('/api/runs');content=runsHTML(data);}
    else if (page === 'engine') {data=await api('/api/engine');content=engineHTML(data);}
    else if (page === 'data') {data=await api('/api/data');content=dataHTML(data);}
    else {data=await api('/api/docs');content=docsHTML(data,current.query.get('doc'));}
    if (token !== generation) return;
    main.innerHTML = content;
    if (page === 'runs' && id) mountRunCharts(data);
    if (page === 'docs' && current.query.get('doc')) {
      const doc = await api('/api/document?path='+encodeURIComponent(current.query.get('doc')));
      if (token !== generation) return;
      $('#document-content').innerHTML = src(doc.source) + `<p class="file-meta">${esc(doc.note)}</p><div class="prose">${doc.html}</div>`;
      if (current.query.get('anchor')) document.getElementById(current.query.get('anchor'))?.scrollIntoView({block:'start'});
    }
    wireFilters();
    $('#connection').textContent = page === 'overview' ? '● Observed '+(data.sampled_at ? new Date(data.sampled_at).toLocaleTimeString() : 'pending') : '● Local snapshot';
    $('#connection').title = 'Snapshot: '+date(data.sampled_at);
  } catch (error) {
    if (token !== generation) return;
    main.innerHTML = `<div class="error"><h2>Observation unavailable</h2><p>${esc(error.message)}</p><button id="retry">Try again</button></div>`;
    $('#retry').onclick = render;
    $('#connection').textContent = '● Unavailable';
  } finally {if (token === generation) busy=false;}
}
function wireFilters() {
  if ($('#model-search')) {
    const filter = () => {
      const query = $('#model-search').value.toLowerCase(),family=$('#model-family').value;
      const found=arr(modelData.offline).filter(m => (!family || m.family === family) && JSON.stringify({id:m.id,name:m.name,metrics:m.metrics}).toLowerCase().includes(query));
      $('#offline-models').innerHTML=found.map(offlineCard).join('') || empty('No models match this search.');
    };
    $('#model-search').oninput=filter;$('#model-family').onchange=filter;
  }
  if ($('#run-search')) {
    const filter = () => {
      const query=$('#run-search').value.toLowerCase(),kind=$('#run-kind').value,status=$('#run-status').value;
      $('#run-table').innerHTML=runTable(arr(runData.runs).filter(r => r.id.toLowerCase().includes(query) && (!kind || r.kind === kind) && (!status || r.status === status)));
    };
    $('#run-search').oninput=filter;$('#run-kind').onchange=filter;$('#run-status').onchange=filter;
  }
  if ($('#report-search')) $('#report-search').oninput = event => $$('[data-report]').forEach(n => n.hidden = !n.dataset.report.includes(event.target.value.toLowerCase()));
  if ($('#doc-search')) $('#doc-search').oninput = event => $$('[data-doc]').forEach(n => n.hidden = !n.dataset.doc.includes(event.target.value.toLowerCase()));
}
async function refreshOverview() {
  if (document.hidden || busy || route().page !== 'overview') return;
  const token=generation;
  try {
    const data=await api('/api/overview');
    if (token !== generation || route().page !== 'overview') return;
    // Preserve open evidence panels and scroll position during live updates.
    const open=$$('details[open]').map(n => $('summary',n)?.textContent);
    const scroll=window.scrollY;
    $('#main').innerHTML=overviewHTML(data);
    $$('details').forEach(n => {if (open.includes($('summary',n)?.textContent)) n.open=true;});
    window.scrollTo(0,scroll);
    $('#connection').textContent='● Observed '+(data.sampled_at ? new Date(data.sampled_at).toLocaleTimeString() : 'pending');
    $('#connection').title='Snapshot: '+date(data.sampled_at);
  } catch {$('#connection').textContent='● Disconnected · retrying';}
}
$('#arena-link').href=localService('http://127.0.0.1:8800/');
window.addEventListener('hashchange',render);
window.addEventListener('scroll',() => {$('#tooltip').hidden=true;},{passive:true});
setInterval(refreshOverview,5000);
render();
