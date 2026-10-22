#!/usr/bin/env python3
"""Render the recorded league accuracy and throughput evidence as local HTML."""
import argparse
import html
import json
from pathlib import Path


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('directory',type=Path);a=p.parse_args();root=Path(__file__).resolve().parents[1]
    report=json.loads((a.directory/'accuracy-combined.json').read_text())
    bench=json.loads((a.directory/'benchmark/report.json').read_text())
    names={d['id']:d['name'] for d in json.loads((root/'data/fast_league_decks.json').read_text())['decks']}
    overall=report['overall'];rows=[]
    for deck,r in report['per_deck'].items():
        rows.append(f"<tr><th>{html.escape(names[deck])}</th><td>{r['winner_agreements']}/{r['games']}</td><td>{r['hp']['PrincessTower']['mae_pp_with_failures_penalized']:.2f} pp</td><td>{r['hp']['KingTower']['mae_pp_with_failures_penalized']:.2f} pp</td></tr>")
    stats=bench['summaries'];rate=stats['candidate']['games_per_minute'];old=stats['reference']['games_per_minute']
    doc='''<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><style>
body{font-family:var(--font-sans,system-ui);color:var(--foreground,#222);margin:0;line-height:1.45}*{box-sizing:border-box}h2{font-size:23px;letter-spacing:-.5px;margin:0 0 5px}p{margin:5px 0 16px}.muted{color:var(--muted-foreground,#666);font-size:13px}.status{color:var(--success,#21834b);font-weight:650}.metrics{display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin:16px 0 22px}.metric{padding:16px;border-radius:var(--radius,10px);background:var(--secondary,#f1f3f5)}.value{display:block;font-size:25px;font-weight:700;letter-spacing:-.6px}.label{font-size:12px;color:var(--muted-foreground,#666)}table{width:100%;border-collapse:collapse;font-size:13px;margin-bottom:17px}th,td{text-align:right;padding:10px 5px;border-bottom:1px solid var(--border,#ddd)}th:first-child{text-align:left}thead{color:var(--muted-foreground,#666);font-size:11px}tbody th{font-weight:550}.speed{margin:22px 0 8px}h3{font-size:17px;margin:0 0 10px}.barrow{display:grid;grid-template-columns:130px 1fr 95px;align-items:center;gap:10px;margin:9px 0;font-size:13px}.track{height:14px;background:var(--secondary,#eee);border-radius:4px;overflow:hidden}.bar{height:14px;background:var(--chart-1,#3083c9);border-radius:4px}.reference{background:var(--muted-foreground,#888)}.number{text-align:right;font-variant-numeric:tabular-nums}footer{margin-top:18px;padding-top:12px;border-top:1px solid var(--border,#ddd)}@media(max-width:480px){.metrics{gap:6px}.metric{padding:16px 8px}.value{font-size:20px}th,td{font-size:11px;padding:9px 3px}.barrow{grid-template-columns:100px 1fr 82px;gap:6px}}
</style></head><body>
<h2>Six-deck simulator validation</h2><p class="muted">Compiled original engine · recovered build · fresh scripted games</p>
<div class="status">STATUS</div><div class="metrics"><div class="metric"><span class="value">WINNERS</span><span class="label">winner agreement</span></div><div class="metric"><span class="value">PRINCESS pp</span><span class="label">princess HP error</span></div><div class="metric"><span class="value">KING pp</span><span class="label">king HP error</span></div></div>
<table><thead><tr><th>Deck</th><th>Winners</th><th>Princess HP</th><th>King HP</th></tr></thead><tbody>ROWS</tbody></table>
<p class="muted">Two fresh cohorts: 36 ordered matchups × 7 seeds each. One varies affordable actions and placement; one saves elixir and cycles every card, including Golem. Mirrors count once per deck. HP error is mean absolute error as a percentage of maximum HP. Failures stay in the denominator.</p>
<div class="speed"><h3>SPEED× faster in the replay benchmark</h3>
<div class="barrow"><span>Original / Unicorn</span><div class="track"><div class="bar reference" style="width:OLDWIDTH%"></div></div><span class="number">OLD games/min</span></div>
<div class="barrow"><span>Compiled original</span><div class="track"><div class="bar" style="width:100%"></div></div><span class="number">NEW games/min</span></div></div>
<p class="muted">One engine process, identical complete games, alternating engine order. Includes reset, placement query, actions, tower HP and JSON IPC. Excludes neural inference and process startup.</p>
<footer class="muted">Accuracy is agreement with the recovered build on these tests. It is not a guarantee for every policy or today's live game. All full-state audits and the separate instruction checks are retained with the report. The root approximate simulator was not recalibrated.</footer>
</body></html>'''
    values={'STATUS':'95% target passed' if report['target_passed'] else 'Target not passed',
            'WINNERS':f"{overall['winner_agreements']}/{overall['games']}",
            'PRINCESS':f"{overall['hp']['PrincessTower']['mae_pp_with_failures_penalized']:.2f}",
            'KING':f"{overall['hp']['KingTower']['mae_pp_with_failures_penalized']:.2f}",
            'ROWS':''.join(rows),'SPEED':f"{bench['speedup']:.2f}",'OLDWIDTH':f'{min(100,old/rate*100):.1f}',
            'OLD':f'{old:.1f}','NEW':f'{rate:.1f}'}
    for key,value in values.items():doc=doc.replace(key,value)
    (a.directory/'report.html').write_text(doc)

if __name__=='__main__':main()
