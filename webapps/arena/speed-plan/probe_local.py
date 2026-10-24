#!/usr/bin/env python3
"""Headless Chrome probe for the local painter harness.

Measures display freezes, lag and draw cadence. It does not claim public-site numbers.
"""
from __future__ import annotations

import asyncio
import json
import os
import statistics
import subprocess
import sys
import time
import urllib.request

import websockets

BASE = os.environ.get("ARENA_BASE", "http://127.0.0.1:8765")
SECONDS = float(sys.argv[1]) if len(sys.argv) > 1 else 12
LATENCY_MS = float(os.environ.get("ARENA_LATENCY_MS", "150"))
DOWNLOAD_BPS = float(os.environ.get("ARENA_DOWNLOAD_BPS", str(5 * 1024 * 1024 / 8)))
CPU_RATE = float(os.environ.get("ARENA_CPU_RATE", "1"))
PORT = int(os.environ.get("ARENA_CDP_PORT", "9341"))


def analyze(frames):
    freezes = []
    last_t = last_s = start = None
    for row in frames:
        t, s = row[0], row[1]
        if last_s is not None and s <= last_s:
            start = last_t if start is None else start
        else:
            if start is not None and last_t - start > 100:
                freezes.append(last_t - start)
            start = None
        last_t, last_s = t, s
    if start is not None and last_t is not None and last_t - start > 100:
        freezes.append(last_t - start)
    span = (frames[-1][0] - frames[0][0]) / 1000 if len(frames) > 1 else 0
    deltas = [frames[i][0] - frames[i - 1][0] for i in range(1, len(frames)) if frames[i][0] > frames[i - 1][0]]
    windows = {}
    for t, s, *_ in frames:
        windows.setdefault(int((t - frames[0][0]) // 2000), []).append(s)
    rates = []
    for values in windows.values():
        if len(values) >= 2:
            rates.append((values[-1] - values[0]) / 2000)
    pending = [row[2] for row in frames]
    return {
        "frames": len(frames),
        "span_s": round(span, 3),
        "frozen_s": round(sum(freezes) / 1000, 3),
        "freeze_count": len(freezes),
        "longest_freeze_ms": max(freezes, default=0),
        "freeze_ratio": round((sum(freezes) / 1000) / span, 4) if span else None,
        "cadence_ms_p50": round(statistics.median(deltas), 2) if deltas else None,
        "cadence_ms_p95": round(sorted(deltas)[max(0, int(len(deltas) * 0.95) - 1)], 2) if deltas else None,
        "game_per_wall": [round(v, 3) for v in rates],
        "pending_max": max(pending, default=0),
        "pending_p95": round(sorted(pending)[max(0, int(len(pending) * 0.95) - 1)], 2) if pending else None,
    }


async def main():
    profile = f"/tmp/arena-speed-w3-chrome-{PORT}"
    chrome = subprocess.Popen(
        ["google-chrome", "--headless=new", f"--remote-debugging-port={PORT}", f"--user-data-dir={profile}",
         "--no-first-run", "--window-size=600,1000", "--enable-unsafe-swiftshader", "about:blank"],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        tabs = None
        for _ in range(50):
            try:
                tabs = json.load(urllib.request.urlopen(f"http://127.0.0.1:{PORT}/json"))
                break
            except Exception:
                time.sleep(0.2)
        if not tabs:
            raise SystemExit("chrome debugger did not start")
        ws_url = [t for t in tabs if t["type"] == "page"][0]["webSocketDebuggerUrl"]
        async with websockets.connect(ws_url, max_size=2 ** 26) as ws:
            n = 0
            pending = {}
            requests = []

            async def reader():
                try:
                    while True:
                        msg = json.loads(await ws.recv())
                        if msg.get("id") in pending:
                            pending.pop(msg["id"]).set_result(msg)
                        elif msg.get("method") == "Network.requestWillBeSent":
                            requests.append((time.monotonic(), msg["params"]["request"]["url"]))
                except websockets.exceptions.ConnectionClosed:
                    return

            asyncio.create_task(reader())

            async def call(method, params=None):
                nonlocal n
                n += 1
                mid = n
                loop = asyncio.get_running_loop()
                fut = loop.create_future()
                pending[mid] = fut
                await ws.send(json.dumps({"id": mid, "method": method, "params": params or {}}))
                return await fut

            async def ev(expr):
                r = await call("Runtime.evaluate", {"expression": expr, "returnByValue": True})
                return r.get("result", {}).get("result", {}).get("value", r)

            await call("Page.enable")
            await call("Runtime.enable")
            await call("Network.enable")
            await call("Network.emulateNetworkConditions", {
                "offline": False, "latency": LATENCY_MS,
                "downloadThroughput": DOWNLOAD_BPS, "uploadThroughput": DOWNLOAD_BPS / 4,
            })
            if CPU_RATE != 1:
                await call("Emulation.setCPUThrottlingRate", {"rate": CPU_RATE})
            started = time.monotonic()
            url = f"{BASE}/play.html?id=demo&painter=fake&recording=/fixtures/recording.json"
            await call("Page.navigate", {"url": url})
            ready = None
            for _ in range(int(SECONDS * 10) + 80):
                await asyncio.sleep(0.1)
                if ready is None and await ev("typeof viewReady !== 'undefined' && viewReady ? 1 : 0") == 1:
                    ready = time.monotonic() - started
                    print(f"first frame visible {ready:.2f}s after navigation", flush=True)
                if time.monotonic() - started > SECONDS + 4 and ready is not None:
                    break
            diag = await ev("JSON.stringify(typeof arenaDiagnostics==='function'?arenaDiagnostics():null)")
            trace = await ev("JSON.stringify(typeof arenaDisplayTrace==='function'?arenaDisplayTrace():[])")
            loading = await ev("document.getElementById('loading-msg') ? document.getElementById('loading-msg').textContent : ''")
            frames = json.loads(trace) if isinstance(trace, str) else []
            report = {
                "ready_s": None if ready is None else round(ready, 3),
                "loading": loading,
                "diag": json.loads(diag) if isinstance(diag, str) and diag not in ("null", "undefined") else diag,
                "display": analyze(frames) if frames else None,
                "requests": len(requests),
                "requests_after_ready": None,
                "throttle": {"latency_ms": LATENCY_MS, "download_bps": DOWNLOAD_BPS, "cpu_rate": CPU_RATE},
            }
            if ready is not None:
                ready_at = started + ready
                report["requests_after_ready"] = sum(1 for t, _url in requests if t > ready_at + 0.05)
                report["urls_after_ready"] = [url for t, url in requests if t > ready_at + 0.05][:20]
            out = os.environ.get("ARENA_PROBE_OUT", "webapps/arena/speed-plan/probe-local.json")
            json.dump(report, open(out, "w"), indent=2)
            print(json.dumps(report, indent=2)[:4000])
    finally:
        chrome.terminate()
        chrome.wait(timeout=5)


if __name__ == "__main__":
    asyncio.run(main())
