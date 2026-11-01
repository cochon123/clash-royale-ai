"""Local create -> first tick, with and without the legacy scene presenter.

The worktree does not contain the release binaries. This reads the copies in
the main checkout and writes match output only under the worktree's
``.local-arena`` directory. It does not claim the public-site 2.5 s target.
"""
from __future__ import annotations

import json
import os
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
MAIN = Path("/home/cochon/Documents/mini-project/clash-royale-ai")
RUNS = ROOT / ".local-arena" / "runs"
SCENE_CACHE = ROOT / ".local-arena" / "scene-cache"


def _rss_kb(pid: int) -> list[tuple[str, int]]:
    """RSS of this process and descendants, including children of other threads."""
    rows = []
    seen = set()

    def walk(process: int) -> None:
        if process in seen:
            return
        seen.add(process)
        try:
            comm = Path(f"/proc/{process}/comm").read_text().strip()
            status = Path(f"/proc/{process}/status").read_text()
        except OSError:
            return
        children = []
        try:
            for task in Path(f"/proc/{process}/task").iterdir():
                try:
                    children.extend((task / "children").read_text().split())
                except OSError:
                    continue
        except OSError:
            pass
        rss = 0
        for line in status.splitlines():
            if line.startswith("VmRSS:"):
                rss = int(line.split()[1])
        rows.append((comm, rss))
        for child in children:
            walk(int(child))

    walk(pid)
    return rows


def measure(legacy: str) -> dict:
    os.environ["ARENA_RUNS_DIR"] = str(RUNS)
    os.environ["ARENA_ENGINE_POOL"] = "0"
    os.environ["ARENA_WARM_PLACEMENTS"] = "0"
    os.environ["ARENA_SCENE_LEGACY"] = legacy
    os.environ["ARENA_BROWSER_SCENE"] = "1"
    os.environ["ARENA_REMAP_ADDRESSES"] = "0"
    os.environ.setdefault("ARENA_SCENE_BIN", str(MAIN / "engine/native-spectator/target/debug/clash-web-scene"))
    os.environ.setdefault("ARENA_RENDERER_BIN", str(MAIN / "engine/native-spectator/target/debug/clash-web-renderer"))
    os.environ["ARENA_SCENE_CACHE"] = str(SCENE_CACHE)
    RUNS.mkdir(parents=True, exist_ok=True)

    started = time.perf_counter()
    from webapps.common import catalog, paths
    paths.NATIVE_WATCH = MAIN / "engine/native-engine/target/release/native-watch"
    from webapps.arena.engine_pool import get_pool
    from webapps.arena.match import Match

    imported = time.perf_counter()
    presets = [preset for preset in catalog.presets() if len(preset["cards"]) == 8]
    config = {
        "seed": 1,
        "sides": [
            {"controller": "random", "deck": list(presets[0]["cards"]), "label": "A"},
            {"controller": "random", "deck": list(presets[1]["cards"]), "label": "B"},
        ],
    }
    match = Match("measure", config)
    created = time.perf_counter()
    first = None
    first_tick = None
    error = None
    rss_at_tick = []
    rss_later = []
    running_status = None
    running_message = None
    running_error = None
    renderer = None
    try:
        deadline = time.perf_counter() + 150
        stream = match.tick_stream(None)
        for event, payload, _event_id in stream:
            if time.perf_counter() > deadline:
                error = "timed out waiting for the first tick"
                break
            if event == "ticks" and payload.get("items"):
                first = time.perf_counter()
                first_tick = payload["items"][0]["snapshot"].get("tick")
                break
            if event == "error":
                error = payload
                break
        rss_at_tick = _rss_kb(os.getpid())
        time.sleep(5)
        rss_later = _rss_kb(os.getpid())
        running_status = match.status
        running_message = match.message
        running_error = match.error
        renderer = type(match.renderer).__name__ if match.renderer is not None else None
    finally:
        match.close()
        match.thread.join(timeout=45)
        get_pool().close()
    return {
        "legacy": legacy == "1",
        "renderer": renderer,
        "import_s": round(imported - started, 3),
        "construct_s": round(created - imported, 3),
        "create_to_first_tick_s": None if first is None else round(first - created, 3),
        "import_plus_create_to_first_tick_s": None if first is None else round(first - started, 3),
        "first_tick": first_tick,
        "status": running_status,
        "message": running_message,
        "error": error or running_error,
        "rss_kib_at_first_tick": rss_at_tick,
        "rss_kib_after_5s": rss_later,
        "binary": str(paths.NATIVE_WATCH),
        "runs": str(RUNS),
    }


def delivery_latency() -> dict:
    """In-process publish to a waiting subscriber. No engine, no deepcopy of a battle."""
    os.environ.setdefault("ARENA_RUNS_DIR", str(RUNS))
    from webapps.arena.ticks import TickLog

    log = TickLog(batch_seconds=0, idle_seconds=30)
    samples = []

    def reader():
        for event, payload, _event_id in log.events(None, match_id="latency", summary=lambda: {"status": "running"}):
            if event == "ticks":
                samples.append(time.perf_counter())
                if len(samples) >= 20:
                    break

    import threading
    thread = threading.Thread(target=reader)
    thread.start()
    time.sleep(0.05)
    starts = []
    for tick in range(20):
        starts.append(time.perf_counter())
        log.publish(1, tick, {"tick": tick, "entities": []}, {"tick": tick})
        while len(samples) <= tick:
            time.sleep(0.0002)
    log.close()
    thread.join(timeout=5)
    delays_ms = [(samples[i] - starts[i]) * 1000 for i in range(min(len(samples), len(starts)))]
    delays_ms.sort()
    def pct(p):
        if not delays_ms:
            return None
        return round(delays_ms[min(len(delays_ms) - 1, int(p * (len(delays_ms) - 1)))], 3)
    return {"n": len(delays_ms), "p50_ms": pct(0.50), "p95_ms": pct(0.95), "max_ms": pct(1)}


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] in ("0", "1"):
        print(json.dumps(measure(sys.argv[1])))
    else:
        print(json.dumps({"delivery": delivery_latency()}))
