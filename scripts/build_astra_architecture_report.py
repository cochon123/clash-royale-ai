"""Build the standalone architecture report from the measured evidence bundle."""
from pathlib import Path
import hashlib
import html
import json

ROOT = Path(__file__).resolve().parents[1]
evidence = json.loads((ROOT / "reports/astra_v8_evidence/results.json").read_text())
inventory = []
for path in sorted((ROOT / "models").glob("*/report.json")):
    d = json.loads(path.read_text())
    c, data = d.get("compute", {}), d.get("data", {})
    splits = data.get("splits", [])
    unique = data.get("battles_total", d.get("battles_total"))
    note = "archived population"
    if c.get("mirror_training") and not c.get("lazy_mirror_training") and splits:
        unique = sum(s["battles"] / 2 if s["split"] == "train" else s["battles"] for s in splits)
        note = "source count reconstructed; excludes training mirrors"
    inventory.append({"name": path.parent.name, "date": d.get("created_at", "not recorded")[:10],
        "parameters": c.get("parameters"), "battles": unique, "note": note,
        "path": "../" + str(path.relative_to(ROOT))})
evidence["inventory"] = inventory
evidence["source_inventory"] = [{"path": str(p.relative_to(ROOT)), "sha256": hashlib.sha256(p.read_bytes()).hexdigest()}
    for pattern in ("models/*/report.json", "reports/*.json", "src/cr_replay_pipeline/*.py", "src/cr_replay_pipeline/phone_lab/*.py")
    for p in sorted(ROOT.glob(pattern))]
evidence["collector_status"] = {"checked_date": "2026-09-06", "service_healthy": True, "paused": True, "reason": "paused by operator", "raw_files": 64275}
rows = "".join(f'<tr><td><a href="{r["path"]}">{html.escape(r["name"])}</a></td><td>{r["date"]}</td><td>{r["parameters"]:,}</td><td>{int(r["battles"]):,}</td><td>{r["note"]}</td></tr>'
    if r["parameters"] is not None and r["battles"] is not None else
    f'<tr><td><a href="{r["path"]}">{html.escape(r["name"])}</a></td><td>{r["date"]}</td><td>—</td><td>{int(r["battles"] or 0):,}</td><td>auxiliary model / archived report</td></tr>' for r in inventory)
template = (ROOT / "scripts/astra_v8_report_template.html").read_text()
page = template.replace("@@DATA@@", json.dumps(evidence).replace("</", "<\\/")).replace("@@INVENTORY@@", rows)
target = ROOT / "reports/astra_v8_architecture.html"
target.write_text(page)
print(f"Built {target} ({target.stat().st_size:,} bytes)")
