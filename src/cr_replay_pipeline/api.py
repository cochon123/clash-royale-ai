from __future__ import annotations

import concurrent.futures
import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

from .metadata import battle_identity, normalize_battle

API_ROOT = "https://api.clashroyale.com/v1"


class ClashRoyaleAPI:
    def __init__(self, token: str | None = None, retries: int = 4, token_file: str | Path | None = None):
        source = token_file or os.environ.get("CR_TOKEN_FILE")
        self.token = token or (Path(source).read_text().strip() if source else None) or os.environ.get("CR_TOKEN")
        if not self.token:
            raise ValueError("Set CR_TOKEN or pass a token explicitly")
        self.retries = retries

    def get(self, path: str) -> dict | list:
        url = f"{API_ROOT}{path}"
        last_error = None
        for attempt in range(self.retries):
            request = urllib.request.Request(
                url, headers={"Authorization": f"Bearer {self.token}"}
            )
            try:
                with urllib.request.urlopen(request, timeout=30) as response:
                    return json.load(response)
            except (urllib.error.URLError, urllib.error.HTTPError) as exc:
                last_error = exc
                status = getattr(exc, "code", 0)
                if status and status not in (429,) and status < 500:
                    break
                time.sleep(0.5 * (2**attempt))
        raise RuntimeError(f"API request failed for {path}: {last_error}")

    def leaderboard(self, limit: int = 1000) -> list[dict]:
        payload = self.get(f"/locations/global/pathoflegend/players?limit={limit}")
        return payload.get("items", [])

    def battlelog(self, tag: str) -> list[dict]:
        encoded = urllib.parse.quote(tag if tag.startswith("#") else f"#{tag}", safe="")
        payload = self.get(f"/players/{encoded}/battlelog")
        return payload if isinstance(payload, list) else []


def discover_battles(
    output: str | Path,
    limit: int = 1000,
    workers: int = 12,
    token: str | None = None,
    token_file: str | Path | None = None,
) -> dict:
    api = ClashRoyaleAPI(token=token, token_file=token_file)
    players = api.leaderboard(limit)
    tags = [player["tag"] for player in players]
    battlelogs: list[list[dict]] = []
    failures = []

    def fetch(tag: str):
        try:
            return tag, api.battlelog(tag), None
        except Exception as exc:  # error is recorded in the manifest
            return tag, [], str(exc)

    with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as executor:
        for tag, records, error in executor.map(fetch, tags):
            battlelogs.append(records)
            if error:
                failures.append({"tag": tag, "error": error})

    unique = {}
    for records in battlelogs:
        for record in records:
            unique.setdefault(battle_identity(record), normalize_battle(record))

    destination = Path(output)
    destination.parent.mkdir(parents=True, exist_ok=True)
    # Preserve previously recovered metadata and never replace it with an empty
    # discovery result after authorization/network failures.
    def identity(record):
        players=tuple(sorted((normalize_tag(p.get("tag")),p.get("crowns")) for side in ("team","opponent") for p in record.get(side,[])))
        return record.get("battle_time"),players
    from .metadata import normalize_tag
    merged={}
    if destination.exists():
        for line in destination.read_text().splitlines():
            if line.strip():
                record=json.loads(line);merged[identity(record)]=record
    merged.update({identity(record):record for record in unique.values()})
    if unique:
        temp=destination.with_suffix(destination.suffix+".tmp")
        with temp.open("w",encoding="utf-8") as handle:
            for record in sorted(merged.values(),key=lambda row:row.get("battle_time") or "",reverse=True):
                handle.write(json.dumps(record,ensure_ascii=False,separators=(",",":"))+"\n")
        temp.replace(destination)

    manifest = {
        "leaderboard_players": len(tags),
        "battlelog_failures": failures,
        "unique_battles": len(unique),
        "total_metadata_battles": len(merged),
        "existing_metadata_preserved": True,
        "output": str(destination),
    }
    destination.with_suffix(".manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    return manifest


def normalize_battlelog_directory(
    input_dir: str | Path, output: str | Path
) -> dict:
    source = Path(input_dir)
    files = sorted(source.rglob("*.json"))
    unique = {}
    failures = []
    raw_battles = 0
    for path in files:
        try:
            records = json.loads(path.read_text(encoding="utf-8"))
            if not isinstance(records, list):
                raise ValueError("battlelog is not a JSON array")
            for record in records:
                raw_battles += 1
                unique.setdefault(battle_identity(record), normalize_battle(record))
        except (OSError, ValueError, json.JSONDecodeError) as exc:
            failures.append({"file": str(path), "error": str(exc)})

    destination = Path(output)
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open("w", encoding="utf-8") as handle:
        for record in sorted(
            unique.values(), key=lambda row: row.get("battle_time") or "", reverse=True
        ):
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")))
            handle.write("\n")
    manifest = {
        "input_files": len(files),
        "invalid_files": failures,
        "raw_battles": raw_battles,
        "unique_battles": len(unique),
        "output": str(destination),
    }
    destination.with_suffix(".manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    return manifest
