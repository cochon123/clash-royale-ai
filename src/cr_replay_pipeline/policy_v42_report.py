"""Interactive v4 / v4.1 / v4.2 policy training retrospective.

The report is self-contained and uses native HTML/SVG/JS only.
"""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]


def _load(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def _js(data: Any) -> str:
    return json.dumps(data, separators=(",", ":")).replace("</", "<\\/")


def _fmt_duration(seconds: float) -> str:
    seconds = int(round(seconds))
    hours, rem = divmod(seconds, 3600)
    minutes, secs = divmod(rem, 60)
    if hours:
        return f"{hours}h {minutes:02d}m"
    return f"{minutes}m {secs:02d}s"


def _source_battles(report: dict[str, Any]) -> int:
    if report.get("model_version") == "4.2.0":
        splits = report["data"]["splits"]
        return int(splits[0]["battles"] // 2 + splits[1]["battles"] + splits[2]["battles"])
    trained = report.get("data", {}).get("battles_trained")
    return int(trained or report["data"]["battles_total"])


def _train_samples(report: dict[str, Any]) -> int:
    return int(report.get("data", {}).get("train_samples_trained") or report["data"]["train_samples"])


def render_policy_v42_report(
    output_path: str | Path = "reports/policy_bc_v4_2_full_showcase.html",
) -> Path:
    specs = [
        ("v4", ROOT / "models/policy_bc_v4"),
        ("v4.1", ROOT / "models/policy_bc_v4.1"),
        ("v4.2", ROOT / "models/policy_bc_v4.2_full"),
    ]
    reports: dict[str, dict[str, Any]] = {}
    histories: dict[str, list[dict[str, Any]]] = {}
    for label, model_dir in specs:
        reports[label] = _load(model_dir / "report.json")
        histories[label] = _load(model_dir / "training_stages.json")
    royale = _load(ROOT / "reports/battle_royale_v42_full.json")
    compact_report = _load(ROOT / "models/policy_bc_v4.2/report.json")
    tta_report = _load(ROOT / "reports/policy_bc_v4_2_mirror_tta.json")

    colors = {"v4": "#70a1ff", "v4.1": "#ffca63", "v4.2": "#70e1b1"}
    models: list[dict[str, Any]] = []
    for label, _ in specs:
        report = reports[label]
        compute = report["compute"]
        train_samples = _train_samples(report)
        epochs = int(compute["epochs_ran"])
        batch = int(compute["batch_size"])
        models.append(
            {
                "id": label,
                "name": report["model_name"],
                "color": colors[label],
                "created": report["created_at"],
                "seconds": float(report["seconds"]),
                "duration": _fmt_duration(float(report["seconds"])),
                "sourceBattles": _source_battles(report),
                "visibleBattles": int(report["data"]["battles_total"]),
                "trainSamples": train_samples,
                "sampleVisits": train_samples * epochs,
                "updates": math.ceil(train_samples / batch) * epochs,
                "epochs": epochs,
                "batch": batch,
                "sampleCap": int(compute["max_samples_per_battle"]),
                "mirrored": bool(compute.get("mirror_training", False)),
                "test": report["test"],
                "val": report["val"],
                "rollout": report.get("rollouts", {}),
                "history": histories[label],
            }
        )

    standings = royale["standings"]
    pairs = royale["pairs"]
    payload = {
        "models": models,
        "standings": standings,
        "pairs": pairs,
        "royale": {
            "confident": royale["progress"]["confident_games"],
            "raw": royale["progress"]["games_total"],
            "coverage": royale["progress"]["coverage"],
            "threshold": royale["setup"]["min_confidence"],
            "champion": royale["champion"],
        },
        "ablation": {
            "compact": {
                "label": "10 windows",
                "trainSamples": _train_samples(compact_report),
                "duration": _fmt_duration(float(compact_report["seconds"])),
                "test": compact_report["test"],
            },
            "full": {
                "label": "40 windows",
                "trainSamples": _train_samples(reports["v4.2"]),
                "duration": _fmt_duration(float(reports["v4.2"]["seconds"])),
                "test": reports["v4.2"]["test"],
            },
        },
        "tta": {
            "id": "v4.2 ★",
            "label": "v4.2 + mirror",
            "color": "#e8f58b",
            "test": tta_report["mirror_ensemble"],
            "baseline": tta_report["baseline"],
            "delta": tta_report["delta_ensemble_minus_baseline"],
            "changes": tta_report["decision_changes"],
            "computeMultiplier": tta_report["latency"]["compute_multiplier"],
            "method": tta_report["setup"]["method"],
        },
    }

    v41 = next(model for model in models if model["id"] == "v4.1")
    v42 = next(model for model in models if model["id"] == "v4.2")
    sample_gain = (v42["trainSamples"] / v41["trainSamples"] - 1.0) * 100.0
    update_drop = (1.0 - v42["updates"] / v41["updates"]) * 100.0
    source_gain = (v42["sourceBattles"] / v41["sourceBattles"] - 1.0) * 100.0
    card_delta = (v42["test"]["slot_top1"] - v41["test"]["slot_top1"]) * 100.0
    zone_delta = (v42["test"]["zone_acc"] - v41["test"]["zone_acc"]) * 100.0
    timing_delta = v42["test"]["timing_mae"] - v41["test"]["timing_mae"]
    tta_card_gain = tta_report["delta_ensemble_minus_baseline"]["slot_top1"] * 100.0
    tta_zone_gain = tta_report["delta_ensemble_minus_baseline"]["zone_acc"] * 100.0
    tta_timing_gain = -tta_report["delta_ensemble_minus_baseline"]["timing_mae"]
    tta_compute = tta_report["latency"]["compute_multiplier"]
    created = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")

    html = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v4.2 — full-data training report</title>
<style>
:root{{--bg:#07100e;--panel:#0d1916;--panel2:#11211d;--line:#20332d;--text:#edf7f2;--muted:#91a9a0;--green:#70e1b1;--gold:#ffca63;--blue:#70a1ff;--red:#ff7e78;--ink:#06100d}}
*{{box-sizing:border-box}}html{{scroll-behavior:smooth}}body{{margin:0;background:radial-gradient(circle at 12% -10%,#17372d 0,transparent 33%),radial-gradient(circle at 92% 9%,#2c2616 0,transparent 24%),var(--bg);color:var(--text);font:15px/1.55 Inter,ui-sans-serif,system-ui,sans-serif}}
body:before{{content:"";position:fixed;inset:0;pointer-events:none;opacity:.06;background-image:linear-gradient(#fff 1px,transparent 1px),linear-gradient(90deg,#fff 1px,transparent 1px);background-size:40px 40px;mask-image:linear-gradient(to bottom,#000,transparent 70%)}}
main{{width:min(1200px,calc(100% - 36px));margin:auto;padding:52px 0 90px}}a{{color:var(--green)}}
.eyebrow{{font:700 11px/1.2 ui-monospace,monospace;letter-spacing:.16em;text-transform:uppercase;color:var(--green)}}h1{{font-size:clamp(42px,8vw,86px);line-height:.92;letter-spacing:-.065em;margin:18px 0 24px;max-width:950px}}h1 em{{color:var(--green);font-style:normal}}h2{{font-size:clamp(25px,4vw,42px);letter-spacing:-.04em;margin:0 0 10px}}h3{{font-size:17px;margin:0 0 8px}}p{{color:var(--muted);margin:0}}.lede{{font-size:clamp(17px,2.2vw,23px);max-width:820px;color:#bfd0ca}}
.hero{{padding:26px 0 56px;position:relative}}.stamp{{margin-top:22px;color:#607d72;font:12px ui-monospace,monospace}}
.verdict{{margin-top:42px;border:1px solid #315647;background:linear-gradient(135deg,rgba(112,225,177,.13),rgba(255,202,99,.05));border-radius:22px;padding:25px;display:grid;grid-template-columns:auto 1fr;gap:20px;align-items:start}}.verdict .mark{{width:50px;height:50px;border-radius:14px;display:grid;place-items:center;background:var(--green);color:var(--ink);font-size:25px;font-weight:900}}.verdict strong{{display:block;font-size:21px;margin-bottom:5px}}.verdict p{{max-width:850px}}
.section{{border-top:1px solid var(--line);padding:54px 0}}.section-head{{display:flex;justify-content:space-between;align-items:end;gap:20px;margin-bottom:28px}}.section-head p{{max-width:560px}}
.kpis{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}}.kpi,.card{{border:1px solid var(--line);background:linear-gradient(160deg,rgba(17,33,29,.94),rgba(8,17,14,.94));border-radius:18px;padding:20px;position:relative;overflow:hidden}}.kpi:after{{content:"";position:absolute;width:100px;height:100px;border-radius:50%;background:var(--tone,var(--green));filter:blur(55px);opacity:.12;right:-30px;top:-35px}}.kpi span{{display:block;color:var(--muted);font-size:12px;text-transform:uppercase;letter-spacing:.08em}}.kpi b{{display:block;font-size:clamp(25px,3.6vw,40px);letter-spacing:-.045em;margin:6px 0 2px}}.kpi small{{color:#789188}}
.model-grid{{display:grid;grid-template-columns:repeat(3,1fr);gap:14px}}.model-card{{--tone:var(--blue);border-top:3px solid var(--tone)}}.model-card.v41{{--tone:var(--gold)}}.model-card.v42{{--tone:var(--green)}}.model-tag{{display:inline-flex;align-items:center;gap:7px;font:700 12px ui-monospace,monospace;color:var(--tone)}}.dot{{width:8px;height:8px;background:var(--tone);border-radius:50%;box-shadow:0 0 14px var(--tone)}}.big-number{{font-size:35px;font-weight:750;letter-spacing:-.05em;margin:17px 0 2px}}.facts{{display:grid;grid-template-columns:1fr 1fr;gap:10px;margin-top:18px;padding-top:16px;border-top:1px solid var(--line)}}.fact b,.fact span{{display:block}}.fact b{{font-size:14px}}.fact span{{font-size:11px;color:var(--muted);text-transform:uppercase;letter-spacing:.06em}}
.metric-table{{width:100%;border-collapse:separate;border-spacing:0}}.metric-table th,.metric-table td{{padding:14px 16px;border-bottom:1px solid var(--line);text-align:right}}.metric-table th:first-child,.metric-table td:first-child{{text-align:left}}.metric-table thead th{{font:700 11px ui-monospace,monospace;text-transform:uppercase;letter-spacing:.08em;color:var(--muted)}}.metric-table tbody tr:hover{{background:#13231e}}.winner{{color:var(--gold);font-weight:750}}.down{{color:var(--red)}}.up{{color:var(--green)}}.metric-note{{font-size:11px;color:#617d73;display:block}}
.tta-reference{{display:flex;align-items:center;gap:10px;width:max-content;max-width:100%;margin:8px 0 4px;padding:9px 12px;border:1px solid #778143;border-radius:11px;background:rgba(232,245,139,.07);color:#e8f58b;font:700 12px ui-monospace,monospace}}.tta-reference .star{{font-size:17px;line-height:1}}.tta-reference small{{color:#9fa88a;font:11px ui-monospace,monospace;font-weight:500}}
.star-col{{background:rgba(232,245,139,.045);border-left:1px solid #596331!important}}.star-point{{margin-top:18px;border:1px solid #778143;background:radial-gradient(circle at 88% 0,rgba(232,245,139,.14),transparent 36%),linear-gradient(145deg,#162018,#0b1511);border-radius:18px;padding:22px;display:grid;grid-template-columns:56px 1fr;gap:18px;align-items:start}}.star-icon{{width:56px;height:56px;border-radius:16px;display:grid;place-items:center;background:#e8f58b;color:#152014;font-size:30px;box-shadow:0 0 30px rgba(232,245,139,.18)}}.star-point h3{{font-size:21px}}.star-stats{{display:grid;grid-template-columns:repeat(4,1fr);gap:10px;margin-top:16px}}.star-stat{{padding:11px 13px;border:1px solid #33432d;border-radius:11px;background:#0a1410}}.star-stat b,.star-stat span{{display:block}}.star-stat b{{color:#e8f58b;font:750 18px ui-monospace,monospace}}.star-stat span{{color:var(--muted);font-size:11px}}
.chart-card{{padding:22px}}.chart-tools{{display:flex;flex-wrap:wrap;align-items:center;gap:9px;margin:16px 0}}button,.toggle{{border:1px solid var(--line);background:#0a1512;color:var(--muted);border-radius:10px;padding:8px 11px;font:600 12px ui-monospace,monospace;cursor:pointer}}button:hover,.toggle:hover{{border-color:#416459;color:var(--text)}}button.active{{background:var(--green);color:var(--ink);border-color:var(--green)}}.toggle input{{accent-color:var(--tone);vertical-align:-2px}}#curve{{width:100%;height:390px;display:block}}.gridline{{stroke:#20332d;stroke-width:1}}.axis-label{{fill:#668078;font:11px ui-monospace,monospace}}.curve{{fill:none;stroke-width:3;stroke-linecap:round;stroke-linejoin:round}}.point{{cursor:crosshair}}.tooltip{{position:fixed;pointer-events:none;z-index:9;background:#edf7f2;color:#07100e;border-radius:10px;padding:8px 10px;font:12px ui-monospace,monospace;box-shadow:0 8px 28px #0008;opacity:0;transform:translate(-50%,-120%)}}
.chart-card{{padding:22px}}.chart-tools{{display:flex;flex-wrap:wrap;align-items:center;gap:9px;margin:16px 0}}button,.toggle{{border:1px solid var(--line);background:#0a1512;color:var(--muted);border-radius:10px;padding:8px 11px;font:600 12px ui-monospace,monospace;cursor:pointer}}button:hover,.toggle:hover{{border-color:#416459;color:var(--text)}}button.active{{background:var(--green);color:var(--ink);border-color:var(--green)}}.toggle input{{accent-color:var(--tone);vertical-align:-2px}}#curve{{width:100%;height:390px;display:block}}.gridline{{stroke:#20332d;stroke-width:1}}.axis-label{{fill:#668078;font:11px ui-monospace,monospace}}.curve{{fill:none;stroke-width:3;stroke-linecap:round;stroke-linejoin:round}}.point{{cursor:crosshair}}.tta-line{{stroke:#e8f58b;stroke-width:1.5;stroke-dasharray:4 5;opacity:.8}}.tta-point{{fill:#e8f58b;stroke:#152014;stroke-width:3;cursor:crosshair}}.tta-label{{fill:#e8f58b;font:700 12px ui-monospace,monospace}}.tooltip{{position:fixed;pointer-events:none;z-index:9;background:#edf7f2;color:#07100e;border-radius:10px;padding:8px 10px;font:12px ui-monospace,monospace;box-shadow:0 8px 28px #0008;opacity:0;transform:translate(-50%,-120%)}}
.bars{{display:grid;gap:17px;margin-top:22px}}.bar-row{{display:grid;grid-template-columns:110px 1fr 95px;gap:14px;align-items:center}}.bar-track{{height:14px;border-radius:99px;background:#08110f;overflow:hidden;border:1px solid #1c2d28}}.bar-fill{{height:100%;width:0;border-radius:99px;background:var(--tone);transition:width 1s cubic-bezier(.2,.8,.2,1)}}.bar-value{{font:700 13px ui-monospace,monospace;text-align:right}}
.royale-grid{{display:grid;grid-template-columns:1fr 1.15fr;gap:14px}}.podium{{display:grid;gap:11px;margin-top:18px}}.rank{{display:grid;grid-template-columns:34px 1fr auto;align-items:center;gap:10px;padding:13px;border:1px solid var(--line);border-radius:13px}}.rank:first-child{{border-color:#6b5830;background:#211d10}}.rank-num{{font:800 18px ui-monospace,monospace;color:var(--muted)}}.rank b,.rank small{{display:block}}.rank small{{color:var(--muted)}}.score{{font:800 22px ui-monospace,monospace}}
.h2h{{display:grid;gap:10px;margin-top:18px}}.duel{{display:grid;grid-template-columns:1fr auto 1fr;align-items:center;gap:12px;padding:13px;border-radius:13px;background:#0a1512;border:1px solid var(--line)}}.duel .left{{text-align:right}}.duel strong{{font:800 18px ui-monospace,monospace}}.duel .vs{{color:#526a61;font:11px ui-monospace,monospace}}
.arena-wrap{{display:grid;grid-template-columns:1fr 1fr;gap:18px;align-items:center}}.arena{{aspect-ratio:18/32;max-height:390px;margin:auto;border:2px solid #47675c;border-radius:16px;position:relative;background:linear-gradient(to bottom,#193b31 0 48%,#527b91 48% 52%,#193b31 52%);overflow:hidden;box-shadow:inset 0 0 60px #0007}}.arena:before{{content:"";position:absolute;inset:0;background:repeating-linear-gradient(90deg,transparent 0 5.45%,#ffffff08 5.45% 5.6%),repeating-linear-gradient(0deg,transparent 0 3.05%,#ffffff08 3.05% 3.2%)}}.arena .axis{{position:absolute;top:0;bottom:0;left:50%;width:2px;background:var(--green);box-shadow:0 0 18px var(--green)}}.arena .arrow{{position:absolute;left:16%;right:16%;top:34%;font:700 12px ui-monospace,monospace;color:#d9f7eb;text-align:center}}.arena .arrow:last-child{{top:61%}}.mirror-copy{{display:grid;gap:16px}}.mirror-copy .step{{padding-left:16px;border-left:2px solid var(--green)}}.mirror-copy b{{display:block;margin-bottom:3px}}.mirror-copy p{{font-size:13px}}
.lessons{{display:grid;grid-template-columns:repeat(3,1fr);gap:13px}}.lesson .n{{font:800 12px ui-monospace,monospace;color:var(--green)}}.lesson h3{{margin:14px 0 7px}}.lesson p{{font-size:13px}}.callout{{border-left:3px solid var(--gold);padding:15px 18px;background:#19180f;border-radius:0 12px 12px 0;margin-top:18px;color:#d7cfaf}}
.foot{{padding-top:26px;color:#557067;font:11px ui-monospace,monospace}}.pill{{display:inline-block;padding:4px 8px;border:1px solid var(--line);border-radius:99px;color:var(--muted);font:11px ui-monospace,monospace}}
@media(max-width:850px){{.kpis{{grid-template-columns:1fr 1fr}}.model-grid,.lessons{{grid-template-columns:1fr}}.royale-grid,.arena-wrap{{grid-template-columns:1fr}}.star-stats{{grid-template-columns:1fr 1fr}}}}@media(max-width:540px){{main{{width:min(100% - 22px,1200px)}}.kpis{{grid-template-columns:1fr}}.metric-table{{font-size:12px}}.metric-table th,.metric-table td{{padding:10px 7px}}.bar-row{{grid-template-columns:80px 1fr 72px}}.star-point{{grid-template-columns:1fr}}.star-stats{{grid-template-columns:1fr}}}}
</style>
</head>
<body><main>
<header class="hero">
  <div class="eyebrow">Clash Royale AI · policy retrospective</div>
  <h1>Same model.<br><em>Better sampling.</em></h1>
  <p class="lede">The corrected v4.2 used all 40 windows per battle, 90% of the current dataset for training, and horizontal mirroring. It recovered the earlier regression, set the best card and timing scores in the lineage, and finished within striking distance of v4.1 in the offline arena.</p>
  <div class="stamp">REPORT GENERATED {created} · MODEL policy-bc-v4.2 · VERSION 4.2.0</div>
  <div class="verdict"><div class="mark">↑</div><div><strong>Verdict: v4.2 is the strongest supervised checkpoint; its starred mirror mode is the best offline inference variant.</strong><p>Against v4.1, v4.2 gains {card_delta:+.2f} points of card accuracy and improves timing by {abs(timing_delta):.3f}s. The no-retraining ★ variant adds another {tta_card_gain:.2f} card points and {tta_zone_gain:.2f} zone points. v4.1 remains the model-judged arena incumbent, so this is a deployment refinement rather than a new checkpoint.</p></div></div>
</header>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">01 · experiment at a glance</div><h2>The scale-up in four numbers</h2></div><p>Source battles exclude mirrored copies. Sample visits count training samples × epochs; optimizer updates account for batch size.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--green)"><span>Source battles</span><b>37,011</b><small>+{source_gain:.0f}% vs v4.1</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Samples / epoch</span><b>2.01m</b><small>+{sample_gain:.0f}% vs v4.1</small></div>
    <div class="kpi" style="--tone:var(--red)"><span>Optimizer updates</span><b>{v42['updates']:,}</b><small>−{update_drop:.0f}% vs v4.1</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Training wall time</span><b>{v42['duration']}</b><small>RTX 3050 Laptop · CUDA</small></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">02 · lineage</div><h2>Same brain, different training diets</h2></div><p>All three use the same 632,946-parameter v4 architecture. The meaningful differences are data cut, sample cap, batch size, epochs, and mirroring.</p></div>
  <div class="model-grid" id="modelCards"></div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">03 · held-out test set</div><h2>Final test scores + one starred inference mode</h2></div><p>The ★ column is not another trained model: it runs v4.2 on the original and horizontally mirrored state, maps the second answer back, and averages both predictions.</p></div>
  <div class="card" style="padding:4px 0;overflow:auto">
    <table class="metric-table" id="metricTable"></table>
  </div>
  <div class="star-point"><div class="star-icon">★</div><div><h3>Star point · useful extra compute, no retraining</h3><p>On the exact same 30,458 v4.2 test samples, the mirrored ensemble improved every tracked action metric. The card gain is small and not statistically decisive; the zone gain is stronger. Absolute live latency remains only a few milliseconds.</p><div class="star-stats"><div class="star-stat"><b>+{tta_card_gain:.3f}pt</b><span>card top-1</span></div><div class="star-stat"><b>+{tta_zone_gain:.3f}pt</b><span>zone accuracy</span></div><div class="star-stat"><b>−{tta_timing_gain:.3f}s</b><span>timing MAE</span></div><div class="star-stat"><b>{tta_compute:.2f}×</b><span>GPU model compute</span></div></div></div></div>
  <div class="callout">Fairness note: v4/v4.1 used 70/15/15 splits; v4.2 used 90/5/5, so cross-model sub-point gaps remain suggestive. Unlike that comparison, v4.2 ★ and plain v4.2 use identical samples and labels, making the starred delta a direct paired comparison.</div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">04 · validation during training</div><h2>Validation curves by epoch</h2></div><p>These curves use each run’s validation split, not the test scores in the table above. Choose a metric, toggle models, hover a point, or replay the animation.</p></div>
  <div class="card chart-card">
    <div class="callout" style="margin:0 0 18px"><b>Why the numbers differ:</b> for example, v4.1 zone accuracy is 43.30% on validation (the final yellow point below) and 44.32% on its test set (the table above). Both are correct measurements on different held-out samples.</div>
    <div class="chart-tools" id="metricButtons"></div>
    <div class="chart-tools" id="modelToggles"></div>
    <div id="ttaReference" class="tta-reference"></div>
    <button id="play">▶ replay curves</button>
    <svg id="curve" viewBox="0 0 1040 390" role="img" aria-label="Interactive training curves"></svg>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">05 · sampler ablation</div><h2>The missing windows were the regression</h2></div><p>The two v4.2 runs share architecture, split, mirroring, batch size, and epoch count. Raising the cap from 10 to 40 windows is the decisive correction.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--green)"><span>Card top-1 gain</span><b>+2.55pt</b><small>52.24% → 54.79%</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Zone gain</span><b>+2.08pt</b><small>42.12% → 44.20%</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Timing improvement</span><b>−0.146s</b><small>1.663s → 1.517s</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Training cost</span><b>2h 56m</b><small>vs 1h 10m compact run</small></div>
  </div>
  <div class="callout"><b>Controlled evidence:</b> 2.01m retained training samples beat 953k on every reported action metric. The previous v4.2 result was a sampling-budget failure, not a capacity ceiling.</div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">06 · optimization budget</div><h2>Scale without losing update count</h2></div><p>Batch 512 and ten epochs produced {v42['updates']:,} optimizer updates—within {update_drop:.1f}% of v4.1—while exposing the model to 20.10 million sample visits.</p></div>
  <div class="model-grid">
    <div class="card"><h3>Training samples per epoch</h3><p>Unique retained windows, including reaction repeats and mirrored examples.</p><div class="bars" id="sampleBars"></div></div>
    <div class="card"><h3>Total sample visits</h3><p>Samples multiplied by epochs: how much data flowed through the model.</p><div class="bars" id="visitBars"></div></div>
    <div class="card"><h3>Optimizer updates</h3><p>Approximate gradient steps. Batch size 512 halves updates per sample versus 256.</p><div class="bars" id="updateBars"></div></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">07 · symmetry augmentation</div><h2>What mirroring actually changed</h2></div><p>Horizontal reflection teaches left/right equivalence while preserving time, card identity, side, elixir, and vertical territory.</p></div>
  <div class="arena-wrap card">
    <div class="arena"><div class="axis"></div><div class="arrow">original x → 18,000 − x</div><div class="arrow">same y · same action · same target</div></div>
    <div class="mirror-copy">
      <div class="step"><b>Training only</b><p>33,309 original training battles received mirrored views. Validation and test stayed unmirrored.</p></div>
      <div class="step"><b>Memory-light implementation</b><p>Mirrored events share the original event records and lazily transform x, avoiding a second multi-gigabyte replay copy.</p></div>
      <div class="step"><b>What this run cannot prove</b><p>Because epoch count, batch size, and sample cap also changed, v4.2 is not a clean mirror-vs-no-mirror ablation.</p></div>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">08 · offline battle royale</div><h2>v4.1 wins; v4.2 stays close head-to-head</h2></div><p>{royale['progress']['games_total']} generated games, judged by the symmetric winner predictor. Only predictions at ≥80% calibrated confidence count; {royale['progress']['confident_games']} games survived that filter.</p></div>
  <div class="royale-grid">
    <div class="card"><h3>Standings</h3><p><span class="pill">{royale['progress']['confident_games']} confident / {royale['progress']['games_total']} raw</span> <span class="pill">{royale['progress']['coverage'] * 100:.1f}% coverage</span></p><div class="podium" id="podium"></div></div>
    <div class="card"><h3>Head to head</h3><p>Wins–losses among confident games. Small n: directional evidence, not a definitive ranking.</p><div class="h2h" id="duels"></div></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">09 · lessons</div><h2>What v4.2 taught us</h2></div><p>The corrected run separates a data-sampling problem from a model-capacity question.</p></div>
  <div class="lessons">
    <div class="card lesson"><div class="n">01 / SAMPLING</div><h3>Coverage mattered more than model size.</h3><p>Changing only the per-battle cap from 10 to 40 recovered 2.55 points of card accuracy and 0.146s of timing error.</p></div>
    <div class="card lesson"><div class="n">02 / CAPACITY</div><h3>The same small model still benefits from data.</h3><p>All checkpoints have 632,946 parameters. The full v4.2 improved without adding a single parameter, so the earlier run did not justify scaling the network.</p></div>
    <div class="card lesson"><div class="n">03 / EVIDENCE</div><h3>Offline metrics and simulated play disagree slightly.</h3><p>v4.2 leads card and timing metrics, while v4.1 wins the model-judged arena. A shared frozen test manifest or live evaluation is the next reliable tiebreaker.</p></div>
  </div>
  <div class="callout"><b>Recommended next experiment:</b> keep v4.2 and v4.1, evaluate both on one frozen manifest, then use a small live A/B only if the offline result remains ambiguous. Do not enlarge the model until that comparison shows a capacity-limited error pattern.</div>
</section>

<footer class="foot">Native HTML + SVG + JavaScript · no matplotlib · archived scores are run-local · battle royale is offline model-judged self-play, not live Clash Royale.</footer>
</main>
<div class="tooltip" id="tip"></div>
<script id="report-data" type="application/json">{_js(payload)}</script>
<script>
const D=JSON.parse(document.getElementById('report-data').textContent);
const $=s=>document.querySelector(s), fmt=n=>new Intl.NumberFormat('en-US').format(Math.round(n));
const pct=n=>(100*n).toFixed(2)+'%';
const modelClass=id=>id==='v4.1'?'v41':id==='v4.2'?'v42':'';

$('#modelCards').innerHTML=D.models.map(m=>`<article class="card model-card ${{modelClass(m.id)}}"><div class="model-tag"><i class="dot"></i>${{m.id}}</div><div class="big-number">${{fmt(m.sourceBattles)}}</div><p>source battles</p><div class="facts"><div class="fact"><b>${{fmt(m.trainSamples)}}</b><span>samples / epoch</span></div><div class="fact"><b>${{m.epochs}}</b><span>epochs</span></div><div class="fact"><b>${{m.batch}}</b><span>batch</span></div><div class="fact"><b>${{m.sampleCap}}</b><span>windows / battle</span></div><div class="fact"><b>${{fmt(m.updates)}}</b><span>optimizer updates</span></div><div class="fact"><b>${{m.duration}}</b><span>wall time</span></div></div>${{m.mirrored?'<p style="margin-top:15px;color:var(--green)">↔ mirrored training enabled</p>':''}}</article>`).join('');

const metrics=[
 ['Card top-1','slot_top1','pct',true,'Correct card slot'],['Card top-3','slot_top3','pct',true,'Human card within three guesses'],['Zone accuracy','zone_acc','pct',true,'Correct arena region'],['Within one tile','tile_acc','pct',true,'≤1,000 API units'],['Placement error','xy_mae','int',false,'API units · lower is better'],['Timing error','timing_mae','sec',false,'Seconds · lower is better'],['Test loss','loss','float',false,'Composite objective · lower is better']
];
const format=(v,t)=>t==='pct'?pct(v):t==='sec'?v.toFixed(3)+'s':t==='int'?fmt(v):v.toFixed(3);
const testVariants=[...D.models,D.tta];
$('#metricTable').innerHTML='<thead><tr><th>Metric</th>'+testVariants.map((m,i)=>`<th class="${{i===testVariants.length-1?'star-col':''}}" style="color:${{m.color}}">${{m.id}}</th>`).join('')+'</tr></thead><tbody>'+metrics.map(([label,key,type,higher,note])=>{{const vals=testVariants.map(m=>m.test[key]);const best=higher?Math.max(...vals):Math.min(...vals);return `<tr><td><b>${{label}}</b><span class="metric-note">${{note}}</span></td>`+vals.map((v,i)=>`<td class="${{v===best?'winner ':''}}${{i===vals.length-1?'star-col':''}}">${{format(v,type)}}</td>`).join('')+'</tr>'}}).join('')+'</tbody>';

const curveMetrics={{val_loss:['Validation loss',false],val_slot_top1:['Validation card top-1',true],val_zone_acc:['Validation zone accuracy',true],val_tile_acc:['Validation within one tile',true],val_timing_mae:['Validation timing MAE',false]}};
const ttaMetricMap={{val_loss:'loss',val_slot_top1:'slot_top1',val_zone_acc:'zone_acc',val_tile_acc:'tile_acc',val_timing_mae:'timing_mae'}};
let activeMetric='val_loss', visible=new Set(D.models.map(m=>m.id)), reveal=1;
$('#metricButtons').innerHTML=Object.entries(curveMetrics).map(([k,v],i)=>`<button data-metric="${{k}}" class="${{i===0?'active':''}}">${{v[0]}}</button>`).join('');
$('#modelToggles').innerHTML=D.models.map(m=>`<label class="toggle" style="--tone:${{m.color}}"><input type="checkbox" data-model="${{m.id}}" checked> ${{m.id}}</label>`).join('');
function updateTtaReference(){{const key=ttaMetricMap[activeMetric], value=D.tta.test[key], label=curveMetrics[activeMetric][0].replace('Validation ','');const formatted=activeMetric.includes('acc')?pct(value):activeMetric==='val_timing_mae'?value.toFixed(3)+'s':value.toFixed(3);$('#ttaReference').innerHTML=`<span class="star">★</span><span>v4.2 mirror · ${{label}}: <b>${{formatted}}</b></span><small>test reference · not a curve epoch</small>`}}
$('#metricButtons').onclick=e=>{{if(!e.target.dataset.metric)return;activeMetric=e.target.dataset.metric;document.querySelectorAll('[data-metric]').forEach(b=>b.classList.toggle('active',b===e.target));reveal=1;updateTtaReference();draw()}};
$('#modelToggles').onchange=e=>{{e.target.checked?visible.add(e.target.dataset.model):visible.delete(e.target.dataset.model);draw()}};
const svg=$('#curve'), tip=$('#tip');
function draw(){{const W=1040,H=390,p={{l:68,r:24,t:24,b:48}};const shown=D.models.filter(m=>visible.has(m.id));const ttaValue=D.tta.test[ttaMetricMap[activeMetric]];const all=shown.flatMap(m=>m.history.map(r=>r[activeMetric]).filter(Number.isFinite)).concat([ttaValue]);if(!all.length)return;let lo=Math.min(...all),hi=Math.max(...all);const pad=(hi-lo||1)*.1;lo-=pad;hi+=pad;const maxEpoch=Math.max(...shown.map(m=>m.history.length));const x=e=>p.l+(e-1)/Math.max(maxEpoch-1,1)*(W-p.l-p.r);const y=v=>p.t+(hi-v)/(hi-lo)*(H-p.t-p.b);let out='';for(let i=0;i<5;i++){{const yy=p.t+i*(H-p.t-p.b)/4,v=hi-i*(hi-lo)/4;out+=`<line class="gridline" x1="${{p.l}}" y1="${{yy}}" x2="${{W-p.r}}" y2="${{yy}}"/><text class="axis-label" x="${{p.l-10}}" y="${{yy+4}}" text-anchor="end">${{activeMetric.includes('acc')?pct(v):v.toFixed(2)}}</text>`}}for(let i=1;i<=maxEpoch;i+=Math.max(1,Math.ceil(maxEpoch/8)))out+=`<text class="axis-label" x="${{x(i)}}" y="${{H-17}}" text-anchor="middle">${{i}}</text>`;shown.forEach(m=>{{const rows=m.history.slice(0,Math.ceil(m.history.length*reveal));const pts=rows.map(r=>[x(r.epoch),y(r[activeMetric]),r]);out+=`<path class="curve" stroke="${{m.color}}" d="${{pts.map((q,i)=>(i?'L':'M')+q[0].toFixed(1)+','+q[1].toFixed(1)).join(' ')}}"/>`;out+=pts.map(q=>`<circle class="point" data-model="${{m.id}}" data-epoch="${{q[2].epoch}}" data-value="${{q[2][activeMetric]}}" fill="${{m.color}}" cx="${{q[0]}}" cy="${{q[1]}}" r="4"/>`).join('')}});const sx=W-p.r-10,sy=Math.max(p.t+12,Math.min(H-p.b-12,y(ttaValue)));out+=`<line class="tta-line" x1="${{p.l}}" y1="${{sy}}" x2="${{sx}}" y2="${{sy}}"/><circle class="tta-point" data-value="${{ttaValue}}" cx="${{sx}}" cy="${{sy}}" r="7"/><text class="tta-label" x="${{sx-12}}" y="${{sy-12}}" text-anchor="end">★ test</text>`;out+=`<text class="axis-label" x="${{W/2}}" y="${{H-2}}" text-anchor="middle">epoch</text>`;svg.innerHTML=out;svg.querySelectorAll('.point').forEach(el=>{{el.onmouseenter=e=>{{tip.style.opacity=1;tip.innerHTML=`<b>${{el.dataset.model}} · epoch ${{el.dataset.epoch}}</b><br>${{curveMetrics[activeMetric][0]}}: ${{Number(el.dataset.value).toFixed(4)}}`}};el.onmousemove=e=>{{tip.style.left=e.clientX+'px';tip.style.top=e.clientY+'px'}};el.onmouseleave=()=>tip.style.opacity=0}});svg.querySelectorAll('.tta-point').forEach(el=>{{el.onmouseenter=e=>{{tip.style.opacity=1;tip.innerHTML=`<b>★ v4.2 mirror</b><br>Test reference: ${{Number(el.dataset.value).toFixed(4)}}<br>Not a training epoch`}};el.onmousemove=e=>{{tip.style.left=e.clientX+'px';tip.style.top=e.clientY+'px'}};el.onmouseleave=()=>tip.style.opacity=0}})}}
$('#play').onclick=()=>{{reveal=0;const start=performance.now();function tick(t){{reveal=Math.min(1,(t-start)/1800);draw();if(reveal<1)requestAnimationFrame(tick)}}requestAnimationFrame(tick)}};draw();
updateTtaReference();

function bars(id,key){{const max=Math.max(...D.models.map(m=>m[key]));$(id).innerHTML=D.models.map(m=>`<div class="bar-row"><b style="color:${{m.color}}">${{m.id}}</b><div class="bar-track"><div class="bar-fill" style="--tone:${{m.color}}" data-width="${{100*m[key]/max}}"></div></div><div class="bar-value">${{m[key]>=1e6?(m[key]/1e6).toFixed(2)+'m':m[key]>=1e3?(m[key]/1e3).toFixed(1)+'k':fmt(m[key])}}</div></div>`).join('')}}bars('#sampleBars','trainSamples');bars('#visitBars','sampleVisits');bars('#updateBars','updates');requestAnimationFrame(()=>document.querySelectorAll('.bar-fill').forEach(b=>b.style.width=b.dataset.width+'%'));

$('#podium').innerHTML=D.standings.map(s=>`<div class="rank"><div class="rank-num">#${{s.rank}}</div><div><b>${{s.policy_id.replace('policy-bc-','')}}</b><small>${{s.wins}}–${{s.losses}} · Elo ${{Math.round(s.elo)}}</small></div><div class="score">${{pct(s.win_rate)}}</div></div>`).join('');
$('#duels').innerHTML=D.pairs.map(p=>`<div class="duel"><div class="left"><b>${{p.a.replace('policy-bc-','')}}</b></div><strong>${{p.a_wins}}–${{p.confident_games-p.a_wins}}</strong><div><b>${{p.b.replace('policy-bc-','')}}</b><div class="vs">${{p.confident_games}} confident · ${{pct(p.coverage)}} coverage</div></div></div>`).join('');
</script></body></html>"""

    output = Path(output_path)
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(html, encoding="utf-8")
    return output


if __name__ == "__main__":
    print(render_policy_v42_report())
