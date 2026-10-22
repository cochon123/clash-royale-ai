"""Build an allowlisted Arena runtime archive, excluding research data/secrets."""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import io
import json
from pathlib import Path
import tarfile

ROOT = Path(__file__).resolve().parents[2]


def build(output):
    files = set()
    for directory in ("webapps/common", "webapps/arena", "src/cr_replay_pipeline", "engine/rl"):
        files.update(p for p in (ROOT / directory).rglob("*.py")
                     if not p.name.startswith("test_") and "__pycache__" not in p.parts)
    for directory in ("webapps/arena/static", "engine/data", "engine/web/cards",
                      "engine/analysis/native-spectator",
                      "engine/analysis/native-pipeline/oracle-export-01/stage01-attempt001/state",
                      "engine/analysis/decoded/assets/csv_logic"):
        files.update(p for p in (ROOT / directory).rglob("*") if p.is_file() and not p.is_symlink())
    for name in ("webapps/__init__.py", "models/registry.json", "data/card_costs.json",
                 "models/policy_bc_v4.4.2/best_model.pt", "models/policy_bc_v4.4.2/vocab.json",
                 "engine/runs/banana-1/base.pt", "engine/runs/banana-hog-1/models/final.pt",
                 "engine/runs/banana-league-20261019/models/mortar/final.pt",
                 "engine/native-engine/target/release/native-watch",
                 "engine/native-spectator/target/debug/clash-web-renderer",
                 "engine/analysis/libg-cipher/libg.decrypted.so",
                 "engine/tools/reverse-env/lib/python3.13/site-packages/unicorn/lib/libunicorn.so.2"):
        path = ROOT / name
        if not path.is_file():
            raise FileNotFoundError(path)
        files.add(path)
    manifest = {"created_at": dt.datetime.now(dt.timezone.utc).isoformat(), "files": {}}
    output = Path(output)
    output.parent.mkdir(parents=True, exist_ok=True)
    with tarfile.open(output, "w:gz", compresslevel=3) as archive:
        for path in sorted(files):
            relative = path.relative_to(ROOT).as_posix()
            manifest["files"][relative] = {"bytes": path.stat().st_size,
                                           "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
            archive.add(path, arcname=relative, recursive=False)
        encoded = json.dumps(manifest, indent=2).encode()
        info = tarfile.TarInfo("arena-runtime-manifest.json")
        info.size = len(encoded)
        info.mode = 0o644
        archive.addfile(info, io.BytesIO(encoded))
    print(json.dumps({"archive": str(output), "files": len(files), "bytes": output.stat().st_size,
                      "sha256": hashlib.sha256(output.read_bytes()).hexdigest()}), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    build(parser.parse_args().output)
