import asyncio, json, sys, subprocess, time, urllib.request, os
import websockets
BASE = os.environ.get("ARENA_BASE", "https://calgarypermit.ca/clashroyale")
SECONDS = float(sys.argv[1]) if len(sys.argv) > 1 else 45
MODE = sys.argv[2] if len(sys.argv) > 2 else "watch"
port = 9339
chrome = subprocess.Popen(["google-chrome", "--headless=new", f"--remote-debugging-port={port}", "--user-data-dir=/tmp/arena-probe-chrome",
    "--no-first-run", "--window-size=600,1000", "--enable-unsafe-swiftshader", "about:blank"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
for _ in range(50):
    try:
        tabs = json.load(urllib.request.urlopen(f"http://127.0.0.1:{port}/json")); break
    except Exception: time.sleep(0.2)
ws_url = [t for t in tabs if t["type"] == "page"][0]["webSocketDebuggerUrl"]
MON = r"""
(() => {
  window.__mon = {frames: [], packets: [], t0: performance.now(), source: ""};
  if (typeof arenaDisplayTrace === "function") {
    window.__mon.source = "arenaDisplayTrace";
    const pull = () => {
      try {
        const rows = arenaDisplayTrace();
        if (rows && rows.length) window.__mon.frames = rows;
      } catch (error) { window.__mon.error = String(error); }
    };
    pull();
    window.__mon.timer = setInterval(pull, 250);
    return "trace";
  }
  const r = typeof sceneRenderer !== "undefined" ? sceneRenderer : null;
  if (!r) return "no renderer";
  window.__mon.source = "sceneRenderer";
  const origRender = r.render.bind(r);
  r.render = function(time) {
    const tl = this.timeline, latest = tl[tl.length-1];
    if (latest) {
      const readyAt = latest.readyAt ?? latest.received;
      const speed = Number(latest.packet.playback_speed || 1);
      const target = latest.nativeTime + (Math.max(0, time - readyAt) - this.delay) * speed;
      const shown = Math.min(Math.max(target, tl[0].nativeTime), latest.nativeTime);
      window.__mon.frames.push([Math.round(time), Math.round(shown), this.pending.length, this.inflight.size]);
    }
    return origRender(time);
  };
  const origPush = r.push.bind(r);
  r.push = function(p) { window.__mon.packets.push([Math.round(performance.now()), p.tick, p.display_tick, (p.blobs||[]).length]); return origPush(p); };
  return "hook";
})()
"""
async def main():
    async with websockets.connect(ws_url, max_size=2**28) as ws:
        n = 0
        async def call(method, params=None):
            nonlocal n; n += 1; mid = n
            await ws.send(json.dumps({"id": mid, "method": method, "params": params or {}}))
            while True:
                msg = json.loads(await ws.recv())
                if msg.get("id") == mid: return msg
        async def ev(expr, await_promise=True):
            r = await call("Runtime.evaluate", {"expression": expr, "awaitPromise": await_promise, "returnByValue": True})
            return r.get("result", {}).get("result", {}).get("value", r)
        await call("Page.enable"); await call("Runtime.enable")
        await call("Page.navigate", {"url": BASE + "/"}); await asyncio.sleep(3)
        body = {"mode": "watch", "sides": [{"controller": "banana-hog-1", "deck": "hog-mirror"}, {"controller": "banana-1", "deck": "log-bait"}]} if MODE == "watch" else \
               {"mode": "human", "sides": [{"controller": "human", "deck": "hog-mirror"}, {"controller": "banana-1", "deck": "log-bait"}]}
        t_click = time.monotonic()
        res = await ev(f"fetch('{BASE}/api/matches',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify({json.dumps(body)})}}).then(r=>r.json())")
        mid = res["id"]; print("match", mid, flush=True)
        await call("Page.navigate", {"url": f"{BASE}/play.html?id={mid}"})
        for _ in range(200):
            await asyncio.sleep(0.1)
            ok = await ev("(typeof arenaDisplayTrace === 'function' || (typeof sceneRenderer !== 'undefined' && sceneRenderer)) ? 1 : 0", False)
            if ok == 1: break
        print("install", await ev(MON, False))
        ready = None
        t_start = time.monotonic()
        while time.monotonic() - t_start < SECONDS:
            await asyncio.sleep(1)
            if ready is None and await ev("viewReady ? 1 : 0", False) == 1:
                ready = time.monotonic() - t_click; print(f"first frame visible {ready:.2f}s after create", flush=True)
        diag = await ev("JSON.stringify(arenaDiagnostics())", False)
        data = await ev("JSON.stringify(window.__mon)", False)
        await ev(f"fetch('{BASE}/api/matches/{mid}/close',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:'{{}}'}}).then(r=>r.status)")
        json.dump({"diag": json.loads(diag), "mon": json.loads(data), "ready": ready}, open("cdp-" + MODE + ".json", "w"))
        print("diag", diag[:600])
def analyze(path):
    """Display freezes: wall time during which the shown game time did not advance."""
    d = json.load(open(path)); f = d["mon"]["frames"]
    freezes, last_t, last_s, start = [], None, None, None
    for t, s, *_ in f:
        if last_s is not None and s <= last_s:
            start = last_t if start is None else start
        else:
            if start is not None and last_t - start > 100: freezes.append(last_t - start)
            start = None
        last_t, last_s = t, s
    span = (f[-1][0] - f[0][0]) / 1000
    print(f"display span {span:.1f}s, frozen {sum(freezes)/1000:.1f}s in {len(freezes)} freezes >100ms, longest {max(freezes, default=0)}ms")
    buckets = {}
    for t, s, *_ in f: buckets.setdefault(int((t - f[0][0]) // 2000), []).append(s)
    print("game ms shown per 2 s of wall time:", [v[-1] - v[0] for v in buckets.values()])
try: asyncio.run(main())
finally: chrome.terminate()
analyze("cdp-" + MODE + ".json")
