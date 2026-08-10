"""HTML comparison report for policy BC checkpoints (native charts, no matplotlib)."""

from __future__ import annotations

import html
import json
from pathlib import Path
from typing import Any

from datetime import datetime, timezone

from .winner_report import (
    _base_styles,
    _chart_script,
    _fmt_float,
    _fmt_pct,
    _json_script,
)


def _load_report(model_dir: str | Path) -> dict[str, Any]:
    path = Path(model_dir) / "report.json"
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def _load_stages(model_dir: str | Path) -> list[dict[str, Any]]:
    path = Path(model_dir) / "training_stages.json"
    if not path.exists():
        return []
    with path.open(encoding="utf-8") as handle:
        data = json.load(handle)
    if isinstance(data, list):
        return data
    if isinstance(data, dict):
        for key in ("stages", "history", "epochs"):
            if isinstance(data.get(key), list):
                return data[key]
    return []


def _delta(new: float, old: float, *, higher_is_better: bool = True) -> dict[str, Any]:
    diff = float(new) - float(old)
    improved = diff > 0 if higher_is_better else diff < 0
    return {
        "new": float(new),
        "old": float(old),
        "diff": diff,
        "improved": improved,
        "pct_rel": (diff / abs(old) * 100.0) if abs(old) > 1e-12 else None,
    }


def build_compare_payload(
    old_dir: str | Path,
    new_dir: str | Path,
) -> dict[str, Any]:
    old = _load_report(old_dir)
    new = _load_report(new_dir)
    old_stages = _load_stages(old_dir) or old.get("history") or []
    new_stages = _load_stages(new_dir) or new.get("history") or []

    metrics = [
        ("slot_top1", "Slot top-1", True, True),
        ("slot_top3", "Slot top-3", True, True),
        ("zone_acc", "Zone accuracy", True, True),
        ("tile_acc", "Tile accuracy", True, True),
        ("xy_mae", "XY MAE", False, False),
        ("timing_mae", "Timing MAE (s)", False, False),
        ("loss", "Test loss", False, False),
        ("type_acc", "Type accuracy", True, True),
    ]
    test_rows = []
    for key, label, hib, is_pct in metrics:
        if key not in old.get("test", {}) or key not in new.get("test", {}):
            continue
        d = _delta(new["test"][key], old["test"][key], higher_is_better=hib)
        test_rows.append(
            {
                "key": key,
                "label": label,
                "higher_is_better": hib,
                "is_pct": is_pct,
                **d,
            }
        )

    def series(stages: list[dict[str, Any]], *keys: str) -> list[float]:
        out: list[float] = []
        for row in stages:
            if not isinstance(row, dict):
                continue
            val = None
            for key in keys:
                if key in row and row[key] is not None:
                    val = float(row[key])
                    break
                nested = row.get("val") or row.get("metrics") or {}
                if isinstance(nested, dict) and key in nested and nested[key] is not None:
                    val = float(nested[key])
                    break
            if val is not None:
                out.append(val)
        return out

    def curves_from(stages: list[dict[str, Any]], report: dict[str, Any]) -> dict[str, list[float]]:
        src = stages or report.get("history") or []
        return {
            "loss": series(src, "val_loss", "loss", "train_loss"),
            "slot_top1": series(src, "val_slot_top1", "slot_top1"),
            "zone_acc": series(src, "val_zone_acc", "zone_acc"),
            "xy_mae": series(src, "val_xy_mae", "xy_mae"),
        }

    curves = {
        "old": curves_from(old_stages, old),
        "new": curves_from(new_stages, new),
    }

    old_data = old.get("data") or {}
    new_data = new.get("data") or {}
    data_delta = {
        "battles_total": _delta(
            new_data.get("battles_total", 0), old_data.get("battles_total", 0)
        ),
        "train_samples": _delta(
            new_data.get("train_samples", 0), old_data.get("train_samples", 0)
        ),
    }

    rollout_rows = []
    for key, label, hib in [
        ("mean_score_policy", "Policy rollout score", True),
        ("policy_gap_to_real", "Gap to real (lower better)", False),
        ("policy_vs_easy_lift", "Lift vs easy", True),
        ("policy_vs_medium_lift", "Lift vs medium", True),
    ]:
        o = (old.get("rollouts") or {}).get(key)
        n = (new.get("rollouts") or {}).get(key)
        if o is None or n is None:
            continue
        d = _delta(n, o, higher_is_better=hib)
        rollout_rows.append({"key": key, "label": label, "higher_is_better": hib, **d})

    lessons = []
    for item in new.get("lessons") or []:
        lessons.append(str(item))
    # Auto lesson from deltas
    improved = [r for r in test_rows if r["improved"]]
    regressed = [r for r in test_rows if not r["improved"] and abs(r["diff"]) > 1e-6]
    if improved:
        lessons.append(
            "Test gains vs v4.0: "
            + ", ".join(f"{r['label']} ({r['diff']:+.4f})" for r in improved[:4])
            + "."
        )
    if regressed:
        lessons.append(
            "Test regressions vs v4.0: "
            + ", ".join(f"{r['label']} ({r['diff']:+.4f})" for r in regressed[:4])
            + "."
        )
    lessons.append(
        "This comparison holds architecture fixed (v4 card-conditioned placement); "
        "attribute metric shifts primarily to the larger replay cut and the new train/val/test split."
    )

    return {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC"),
        "old": old,
        "new": new,
        "old_dir": str(old_dir),
        "new_dir": str(new_dir),
        "test_rows": test_rows,
        "rollout_rows": rollout_rows,
        "data_delta": data_delta,
        "curves": curves,
        "lessons": lessons,
    }


def render_policy_compare_report(
    old_dir: str | Path = "models/policy_bc_v4",
    new_dir: str | Path = "models/policy_bc_v4.1",
    output_path: str | Path = "reports/policy_bc_v4_1_compare.html",
) -> Path:
    payload = build_compare_payload(old_dir, new_dir)
    old = payload["old"]
    new = payload["new"]
    old_name = html.escape(str(old.get("model_name", "v4.0")))
    new_name = html.escape(str(new.get("model_name", "v4.1")))
    old_ver = html.escape(str(old.get("model_version", "4.0.0")))
    new_ver = html.escape(str(new.get("model_version", "4.1.0")))

    def fmt_metric(row: dict[str, Any], which: str) -> str:
        val = row[which]
        if row.get("is_pct"):
            return _fmt_pct(val)
        if row["key"] in {"xy_mae"}:
            return f"{val:,.0f}"
        return _fmt_float(val, 4 if abs(val) < 10 else 2)

    rows_html = []
    for row in payload["test_rows"]:
        cls = "up" if row["improved"] else "down"
        arrow = "▲" if row["improved"] else "▼"
        diff = row["diff"]
        if row.get("is_pct"):
            diff_s = f"{diff * 100:+.2f} pp"
        elif row["key"] == "xy_mae":
            diff_s = f"{diff:+,.0f}"
        else:
            diff_s = f"{diff:+.4f}"
        rows_html.append(
            "<tr>"
            f"<td>{html.escape(row['label'])}</td>"
            f"<td class='num'>{fmt_metric(row, 'old')}</td>"
            f"<td class='num'>{fmt_metric(row, 'new')}</td>"
            f"<td class='num {cls}'>{arrow} {diff_s}</td>"
            "</tr>"
        )

    rollout_html = []
    for row in payload["rollout_rows"]:
        cls = "up" if row["improved"] else "down"
        arrow = "▲" if row["improved"] else "▼"
        rollout_html.append(
            "<tr>"
            f"<td>{html.escape(row['label'])}</td>"
            f"<td class='num'>{_fmt_float(row['old'])}</td>"
            f"<td class='num'>{_fmt_float(row['new'])}</td>"
            f"<td class='num {cls}'>{arrow} {row['diff']:+.4f}</td>"
            "</tr>"
        )

    old_compute = old.get("compute") or {}
    new_compute = new.get("compute") or {}
    old_data = old.get("data") or {}
    new_data = new.get("data") or {}

    lessons_html = "".join(f"<li>{html.escape(x)}</li>" for x in payload["lessons"])

    doc = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Policy BC — {new_name} vs {old_name}</title>
  <style>
    {_base_styles()}
    :root {{
      --old: #94a3b8;
      --new: #38bdf8;
      --up: #4ade80;
      --down: #f87171;
    }}
    .hero {{
      min-height: 42vh;
      display: grid;
      align-content: end;
      gap: 12px;
      padding: 48px 0 36px;
      background:
        radial-gradient(900px 420px at 15% 0%, rgba(56,189,248,0.18), transparent 60%),
        radial-gradient(700px 380px at 90% 20%, rgba(148,163,184,0.12), transparent 55%),
        linear-gradient(180deg, #0b1220 0%, var(--bg) 70%);
      border-bottom: 1px solid var(--line-soft);
      margin: -40px -24px 0;
      padding-left: 24px;
      padding-right: 24px;
    }}
    .hero h1 {{
      font-size: clamp(2rem, 4.5vw, 3rem);
      margin: 0;
      letter-spacing: -0.04em;
    }}
    .vs {{
      display: flex;
      flex-wrap: wrap;
      gap: 18px 28px;
      margin-top: 8px;
    }}
    .vs-card {{
      min-width: 200px;
    }}
    .vs-label {{
      font-size: 11px;
      letter-spacing: 0.08em;
      text-transform: uppercase;
      color: var(--muted);
    }}
    .vs-name {{ font-size: 1.25rem; font-weight: 650; }}
    .vs-name.old {{ color: var(--old); }}
    .vs-name.new {{ color: var(--new); }}
    table.compare {{
      width: 100%;
      border-collapse: collapse;
      font-variant-numeric: tabular-nums;
    }}
    table.compare th, table.compare td {{
      text-align: left;
      padding: 10px 12px;
      border-bottom: 1px solid var(--line-soft);
    }}
    table.compare th {{
      font-size: 11px;
      letter-spacing: 0.07em;
      text-transform: uppercase;
      color: var(--muted);
      font-weight: 600;
    }}
    td.num, th.num {{ text-align: right; }}
    .up {{ color: var(--up); }}
    .down {{ color: var(--down); }}
    .grid-2 {{
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 20px;
    }}
    @media (max-width: 860px) {{
      .grid-2 {{ grid-template-columns: 1fr; }}
      .kpi-row {{ grid-template-columns: repeat(2, minmax(0, 1fr)); }}
    }}
    .chart-panel {{
      border: 1px solid var(--line-soft);
      padding: 16px;
      min-height: 280px;
    }}
    .chart-panel h3 {{
      margin: 0 0 12px;
      font-size: 0.95rem;
      font-weight: 600;
    }}
    .legend-inline {{
      display: flex;
      gap: 16px;
      font-size: 12px;
      color: var(--muted);
      margin-bottom: 8px;
    }}
    .swatch {{
      display: inline-block;
      width: 10px;
      height: 10px;
      border-radius: 2px;
      margin-right: 6px;
    }}
    .swatch.old {{ background: var(--old); }}
    .swatch.new {{ background: var(--new); }}
    .play-row {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      align-items: center;
      margin: 8px 0 18px;
    }}
    button.ctrl {{
      background: transparent;
      border: 1px solid var(--line);
      color: var(--text);
      padding: 8px 14px;
      cursor: pointer;
      font: inherit;
    }}
    button.ctrl:hover {{ border-color: var(--accent); color: var(--accent); }}
    .scrub {{
      flex: 1;
      min-width: 180px;
    }}
    .fact {{
      font-size: 0.95rem;
      color: var(--muted);
    }}
    .fact strong {{ color: var(--text); font-weight: 600; }}
    svg.chart {{ width: 100%; height: auto; display: block; }}
  </style>
</head>
<body>
<main>
  <section class="hero">
    <div class="badge-row">
      <span class="badge">comparison</span>
      <span class="badge">same architecture</span>
      <span class="badge">data scale-up</span>
    </div>
    <h1>{new_name} vs {old_name}</h1>
    <p class="meta">Generated {html.escape(payload["created_at"])} · architecture locked · only the replay cut changed</p>
    <div class="vs">
      <div class="vs-card">
        <div class="vs-label">Baseline</div>
        <div class="vs-name old">{old_name} <span class="meta">({old_ver})</span></div>
        <div class="fact">{int(old_data.get('battles_total', 0)):,} battles · {int(old_data.get('train_samples', 0)):,} train samples · {float(old.get('seconds', 0))/3600:.2f}h</div>
      </div>
      <div class="vs-card">
        <div class="vs-label">Challenger</div>
        <div class="vs-name new">{new_name} <span class="meta">({new_ver})</span></div>
        <div class="fact">{int(new_data.get('battles_total', 0)):,} battles · {int(new_data.get('train_samples', 0)):,} train samples · {float(new.get('seconds', 0))/3600:.2f}h</div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <h2>Data & compute</h2>
    <div class="kpi-row">
      <div>
        <span class="kpi-label">Battles (+)</span>
        <span class="kpi-value">{int(payload['data_delta']['battles_total']['diff']):+,}</span>
      </div>
      <div>
        <span class="kpi-label">Train samples (+)</span>
        <span class="kpi-value">{int(payload['data_delta']['train_samples']['diff']):+,}</span>
      </div>
      <div>
        <span class="kpi-label">Params (both)</span>
        <span class="kpi-value">{int(new_compute.get('parameters') or old_compute.get('parameters') or 0):,}</span>
      </div>
      <div>
        <span class="kpi-label">Device (v4.1)</span>
        <span class="kpi-value">{html.escape(str(new_compute.get('device', '?')))}</span>
      </div>
    </div>
    <p class="fact" style="margin-top:18px">
      Both runs: d_model={html.escape(str(new_compute.get('d_model', old_compute.get('d_model'))))},
      layers={html.escape(str(new_compute.get('num_layers', old_compute.get('num_layers'))))},
      batch={html.escape(str(new_compute.get('batch_size', old_compute.get('batch_size'))))},
      epochs={html.escape(str(new_compute.get('epochs_ran', old_compute.get('epochs_ran'))))},
      lr={html.escape(str(new_compute.get('learning_rate', old_compute.get('learning_rate'))))},
      card-conditioned placement on.
    </p>
  </section>

  <section class="report-section">
    <h2>Held-out test metrics</h2>
    <table class="compare">
      <thead>
        <tr>
          <th>Metric</th>
          <th class="num">{old_ver}</th>
          <th class="num">{new_ver}</th>
          <th class="num">Delta</th>
        </tr>
      </thead>
      <tbody>
        {''.join(rows_html)}
      </tbody>
    </table>
  </section>

  <section class="report-section">
    <h2>Training curves — side by side</h2>
    <div class="play-row">
      <button class="ctrl" id="playBtn" type="button">Play evolution</button>
      <button class="ctrl" id="resetBtn" type="button">Reset</button>
      <input class="scrub" id="epochScrub" type="range" min="1" max="25" value="25">
      <span class="meta" id="epochLabel">epoch 25</span>
    </div>
    <div class="legend-inline">
      <span><i class="swatch old"></i>{old_ver}</span>
      <span><i class="swatch new"></i>{new_ver}</span>
    </div>
    <div class="grid-2">
      <div class="chart-panel chart-block"><h3>Val loss</h3><div class="chart-wrap"><svg class="chart" id="cLoss"></svg></div></div>
      <div class="chart-panel chart-block"><h3>Val slot top-1</h3><div class="chart-wrap"><svg class="chart" id="cSlot"></svg></div></div>
      <div class="chart-panel chart-block"><h3>Val zone accuracy</h3><div class="chart-wrap"><svg class="chart" id="cZone"></svg></div></div>
      <div class="chart-panel chart-block"><h3>Val XY MAE</h3><div class="chart-wrap"><svg class="chart" id="cXy"></svg></div></div>
    </div>
  </section>

  <section class="report-section">
    <h2>Rollout realism</h2>
    {"<table class='compare'><thead><tr><th>Metric</th><th class='num'>"+old_ver+"</th><th class='num'>"+new_ver+"</th><th class='num'>Delta</th></tr></thead><tbody>"+''.join(rollout_html)+"</tbody></table>" if rollout_html else "<p class='fact'>Rollout scores unavailable for one or both runs.</p>"}
  </section>

  <section class="report-section">
    <h2>Lessons</h2>
    <ul>{lessons_html}</ul>
    <p class="fact">
      Full single-model reports:
      <a href="policy_bc_v4.html">policy_bc_v4.html</a> ·
      <a href="policy_bc_v4_1.html">policy_bc_v4_1.html</a>
    </p>
  </section>
</main>
<script>
const COMPARE = {_json_script(payload)};
{_chart_script()}
(function() {{
  const data = COMPARE;
  const oldC = data.curves.old;
  const newC = data.curves.new;
  const maxLen = Math.max(
    (oldC.loss||[]).length, (newC.loss||[]).length,
    (oldC.slot_top1||[]).length, (newC.slot_top1||[]).length, 1
  );
  const scrub = document.getElementById('epochScrub');
  const label = document.getElementById('epochLabel');
  scrub.max = String(maxLen);
  scrub.value = String(maxLen);

  function slice(arr, n) {{
    return (arr || []).slice(0, n).map(v => Number(v));
  }}
  function labels(n) {{
    return Array.from({{length: n}}, (_, i) => String(i + 1));
  }}
  function dual(id, key, yFormat) {{
    const n = Number(scrub.value);
    mountInteractiveLineChart(id, {{
      xLabels: labels(n),
      yFormat: yFormat,
      series: [
        {{ name: 'v4.0', color: '#94a3b8', values: slice(oldC[key], n) }},
        {{ name: 'v4.1', color: '#38bdf8', values: slice(newC[key], n) }},
      ],
    }});
  }}
  function drawAll(n) {{
    label.textContent = 'epoch ' + n;
    scrub.value = String(n);
    dual('cLoss', 'loss', 'float');
    dual('cSlot', 'slot_top1', 'percent');
    dual('cZone', 'zone_acc', 'percent');
    dual('cXy', 'xy_mae', 'float');
  }}

  drawAll(maxLen);
  scrub.addEventListener('input', () => drawAll(Number(scrub.value)));

  let timer = null;
  document.getElementById('playBtn').onclick = () => {{
    if (timer) {{ clearInterval(timer); timer = null; return; }}
    let e = 1;
    drawAll(1);
    timer = setInterval(() => {{
      e += 1;
      if (e > maxLen) {{ clearInterval(timer); timer = null; return; }}
      drawAll(e);
    }}, 180);
  }};
  document.getElementById('resetBtn').onclick = () => {{
    if (timer) {{ clearInterval(timer); timer = null; }}
    drawAll(maxLen);
  }};
}})();
</script>
</body>
</html>
"""
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(doc, encoding="utf-8")
    return out
