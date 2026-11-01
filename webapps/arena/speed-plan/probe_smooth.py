#!/usr/bin/env python3
"""Frame pacing probe for the painter worker.

Starts a headed Chrome (real GPU) or headless Chrome, creates a watch match,
and instruments the painter worker before its first line runs:

- every compositor render (wall time, shown game time, cost, rAF or push),
- every worker rAF callback,
- painter_step / painter_observe / JSON.parse cost and display-list size,
- main-thread long tasks.

The summary answers "is it smooth": presented frame interval, the share of
frames whose shown game time moved by a different amount than wall time, and
the worker's busy share. Raw samples go to the output JSON.

    python probe_smooth.py [seconds] [--cpu 4] [--headless] [--human] [--out FILE]
"""
from __future__ import annotations

import argparse
import asyncio
import json
import os
import statistics
import subprocess
import time
import urllib.request

import websockets

BASE = os.environ.get("ARENA_BASE", "https://calgarypermit.ca/clashroyale")

WORKER_HOOK = r"""
(() => {
  const P = self.__probe = {newtex: {}, hashed: {}, renders: [], rafs: [], steps: [], observes: [], parses: [], pushes: [], intervals: [], prewarm: null, init: null};
  const now = () => performance.now();
  const raf = self.requestAnimationFrame && self.requestAnimationFrame.bind(self);
  let inRaf = false;
  if (raf) self.requestAnimationFrame = (cb) => raf((t) => {
    const s = now(); inRaf = true;
    try { cb(t); } finally { inRaf = false; P.rafs.push([Math.round(s * 10) / 10, Math.round((now() - s) * 100) / 100]); }
  });
  const si = self.setInterval.bind(self);
  self.setInterval = (cb, ms, ...rest) => si(() => {
    const s = now();
    try { cb(); } finally { P.intervals.push([Math.round(s * 10) / 10, Math.round((now() - s) * 100) / 100]); }
  }, ms, ...rest);
  const parse = JSON.parse;
  JSON.parse = function (text, reviver) {
    const s = now(); const value = parse(text, reviver);
    const d = now() - s;
    if (typeof text === "string" && text.length > 2000) P.parses.push([Math.round(s), Math.round(d * 100) / 100, text.length]);
    return value;
  };
  function wrapExports(instance) {
    const ex = instance.exports, wrapped = {};
    for (const [name, fn] of Object.entries(ex)) {
      if (typeof fn !== "function") { wrapped[name] = fn; continue; }
      if (name === "painter_step") wrapped[name] = (dt) => { const s = now(); const rc = fn(dt); P.steps.push([Math.round(s * 10) / 10, Math.round((now() - s) * 100) / 100, rc, rc === 1 ? ex.painter_result_len() : 0]); return rc; };
      else if (name === "painter_observe") wrapped[name] = (p, l) => { const s = now(); const rc = fn(p, l); P.observes.push([Math.round(s), Math.round((now() - s) * 100) / 100, l]); return rc; };
      else if (name === "painter_prewarm") wrapped[name] = () => { const s = now(); const rc = fn(); P.prewarm = now() - s; return rc; };
      else if (name === "painter_init") wrapped[name] = (...a) => { const s = now(); const rc = fn(...a); P.init = now() - s; return rc; };
      else wrapped[name] = fn;
    }
    return {exports: wrapped};
  }
  const wrapResult = (r) => r && r.instance ? {module: r.module, instance: wrapExports(r.instance)} : wrapExports(r);
  const ist = WebAssembly.instantiateStreaming, ins = WebAssembly.instantiate;
  WebAssembly.instantiateStreaming = (...a) => ist.apply(WebAssembly, a).then(wrapResult);
  WebAssembly.instantiate = (...a) => ins.apply(WebAssembly, a).then(wrapResult);
  let SC;
  Object.defineProperty(self, "SceneCompositor", {configurable: true, get: () => SC, set: (value) => {
    SC = value;
    const render = value.prototype.render, push = value.prototype.push;
    value.prototype.render = function (time) {
      const s = now(); const r = render.call(this, time);
      P.renders.push([Math.round(s * 10) / 10, Math.round(this.shownNativeMs * 10) / 10, Math.round((now() - s) * 100) / 100, inRaf ? 1 : 0, this.metrics.draws]);
      return r;
    };
    value.prototype.push = function (packet) {
      const before = this.textures.size, s = now();
      try {
        const fresh = new Map((packet.textures || []).map((t) => [t.id, t]));
        const walk = (draws) => { for (const d of draws || []) {
          if (d.kind === "shadow") { walk(d.draws); continue; }
          if (d.kind !== "sprite") continue;
          const scope = String(d.id || "?").replace(/-\d+$/, "").replace(/\d+/g, "#");
          const h = P.hashed[scope] || (P.hashed[scope] = [0, 0]);
          const t = fresh.get(d.tex);
          if (t) { const e = P.newtex[scope] || (P.newtex[scope] = [0, 0]); e[0]++; e[1] += t.width * t.height * 4; fresh.delete(d.tex); }
          h[0]++;
        } };
        walk(packet.draws);
      } catch (e) {}
      const r = push.call(this, packet);
      P.pushes.push([Math.round(s * 10) / 10, Number(packet && packet.display_tick), Math.round((now() - s) * 100) / 100, this.textures.size - before, (packet.draws || []).length]);
      return r;
    };
  }});
  self.addEventListener("message", (event) => {
    if (event.data && event.data.type === "__probe_dump") self.postMessage({type: "__probe", data: JSON.stringify(P)});
  });
  return "hooked";
})();
"""

BOOT_HOOK = r"""
(() => {
  const log = window.__boot = [];
  const mark = (what) => log.push([Math.round(performance.timeOrigin + performance.now()), what]);
  mark("document");
  const W = window.Worker;
  if (!W) return;
  window.Worker = function (url, options) {
    const worker = new W(url, options);
    mark("worker-created");
    const seen = new Set();
    worker.addEventListener("message", (event) => {
      const m = event.data || {};
      const key = m.type === "progress" ? "progress:" + m.phase : m.type;
      if (!seen.has(key)) { seen.add(key); mark(key + (m.type === "progress" ? " " + m.loaded + "/" + m.total : "")); }
      if (m.type === "progress" && m.phase === "assets" && m.loaded === m.total) mark("assets-complete");
    });
    return worker;
  };
  window.Worker.prototype = W.prototype;
  const ES = window.EventSource;
})();
"""

PAGE_HOOK = r"""
(() => {
  window.__page = {longtasks: [], rafs: []};
  try { new PerformanceObserver((list) => { for (const e of list.getEntries()) window.__page.longtasks.push([Math.round(e.startTime), Math.round(e.duration)]); }).observe({type: "longtask", buffered: true}); } catch {}
  const loop = (t) => { window.__page.rafs.push(Math.round(t * 10) / 10); if (window.__page.rafs.length < 20000) requestAnimationFrame(loop); };
  requestAnimationFrame(loop);
  return "ok";
})()
"""


def pct(values, p):
    if not values:
        return None
    s = sorted(values)
    return s[min(len(s) - 1, max(0, int(round(p * (len(s) - 1)))))]


def summarize(worker, page, start_ms=None):
    out = {}
    renders = worker["renders"]
    if start_ms is None and renders:
        start_ms = renders[0][0] + 2000  # skip the first 2 s after the first frame
    renders = [r for r in renders if r[0] >= start_ms]
    rafs = [r for r in worker["rafs"] if r[0] >= start_ms]
    painter_start = (worker["steps"][0][0] + 4000) if worker.get("split") and worker["steps"] else start_ms
    steps = [s for s in worker["steps"] if s[0] >= painter_start]
    pushes = [p for p in worker["pushes"] if p[0] >= start_ms]
    if len(rafs) > 2:
        gaps = [rafs[i][0] - rafs[i - 1][0] for i in range(1, len(rafs))]
        vsync = statistics.median(gaps)
        out["worker_raf"] = {"count": len(rafs), "interval_p50": round(vsync, 2), "interval_p95": round(pct(gaps, .95), 2),
                             "interval_max": round(max(gaps), 1), "missed_vsync_share": round(sum(g > 1.5 * vsync for g in gaps) / len(gaps), 4),
                             "callback_ms_p50": pct([r[1] for r in rafs], .5), "callback_ms_p95": pct([r[1] for r in rafs], .95)}
    # Frames actually composed per vsync window: the last render in each window is what reaches the screen.
    if renders:
        page_rafs = [t for t in page.get("rafs", [])]
        raf_renders = [r for r in renders if r[3] == 1]
        push_renders = [r for r in renders if r[3] == 0]
        out["renders"] = {"total": len(renders), "from_raf": len(raf_renders), "from_push": len(push_renders),
                          "render_ms_p50": pct([r[2] for r in renders], .5), "render_ms_p95": pct([r[2] for r in renders], .95),
                          "draws_p50": pct([r[4] for r in renders], .5)}
        # Motion error: per consecutive raf render, shown delta minus wall delta.
        seq = raf_renders
        errs, stuck, back = [], 0, 0
        for a, b in zip(seq, seq[1:]):
            dw, ds = b[0] - a[0], b[1] - a[1]
            if dw <= 0 or dw > 100:
                continue
            errs.append(ds - dw)
            if ds <= 0.01:
                stuck += 1
            if ds < -0.01:
                back += 1
        if errs:
            out["motion"] = {"frames": len(errs), "abs_err_ms_p50": round(pct([abs(e) for e in errs], .5), 2),
                             "abs_err_ms_p95": round(pct([abs(e) for e in errs], .95), 2),
                             "share_err_over_4ms": round(sum(abs(e) > 4 for e in errs) / len(errs), 4),
                             "share_err_over_8ms": round(sum(abs(e) > 8 for e in errs) / len(errs), 4),
                             "stuck_frames": stuck, "backward_frames": back}
        span = (renders[-1][0] - renders[0][0]) / 1000
        shown_span = (renders[-1][1] - renders[0][1]) / 1000
        out["pace"] = {"wall_s": round(span, 2), "shown_game_s": round(shown_span, 2), "ratio": round(shown_span / span, 4) if span else None}
    if steps:
        out["painter_step"] = {"count": len(steps), "ms_p50": pct([s[1] for s in steps], .5), "ms_p95": pct([s[1] for s in steps], .95),
                               "ms_max": max(s[1] for s in steps), "new_frames": sum(1 for s in steps if s[2] == 1),
                               "result_kb_p50": round(pct([s[3] for s in steps if s[2] == 1] or [0], .5) / 1024, 1)}
    if pushes:
        out["push"] = {"ms_p50": pct([p[2] for p in pushes], .5), "ms_p95": pct([p[2] for p in pushes], .95),
                       "uploads_total": sum(max(0, p[3]) for p in pushes)}
    parses = [p for p in worker["parses"] if p[0] >= painter_start]
    if parses:
        out["json_parse"] = {"count": len(parses), "ms_p50": pct([p[1] for p in parses], .5), "ms_p95": pct([p[1] for p in parses], .95),
                             "kb_p50": round(pct([p[2] for p in parses], .5) / 1024, 1)}
    intervals = [i for i in worker["intervals"] if i[0] >= painter_start]
    if worker.get("split") and renders:
        # Separate clocks per worker; shares are over each worker's own span.
        wall = renders[-1][0] - renders[0][0]
        out["compositor_busy_share"] = round((sum(r[1] for r in rafs) + sum(p[2] for p in pushes)) / wall, 3) if wall > 0 else None
        if intervals:
            span = intervals[-1][0] - intervals[0][0]
            out["painter_busy_share"] = round(sum(i[1] for i in intervals) / span, 3) if span > 0 else None
    elif renders and intervals:
        wall = renders[-1][0] - start_ms
        busy = sum(r[1] for r in rafs) + sum(i[1] for i in intervals)
        out["worker_busy_share"] = round(busy / wall, 3) if wall > 0 else None
        late = [intervals[i][0] - intervals[i - 1][0] for i in range(1, len(intervals))]
        out["step_interval_ms"] = {"p50": pct(late, .5), "p95": pct(late, .95), "max": max(late) if late else None}
    out["prewarm_ms"] = round(worker.get("prewarm") or 0)
    out["init_ms"] = round(worker.get("init") or 0)
    lt = [l for l in page.get("longtasks", [])]
    out["page_longtasks"] = {"count": len(lt), "total_ms": sum(l[1] for l in lt), "max_ms": max((l[1] for l in lt), default=0)}
    return out


def profile_top(profile, n=25):
    nodes = {node["id"]: node for node in profile["nodes"]}
    self_time = {}
    deltas = profile.get("timeDeltas", [])
    for sample, dt in zip(profile.get("samples", []), deltas):
        node = nodes.get(sample)
        if not node:
            continue
        frame = node["callFrame"]
        name = frame.get("functionName") or "(anonymous)"
        url = frame.get("url", "").rsplit("/", 1)[-1]
        key = f"{name} [{url}]" if url else name
        self_time[key] = self_time.get(key, 0) + dt
    total = sum(self_time.values()) or 1
    return [[k, round(v / 1000, 1), round(100 * v / total, 1)] for k, v in sorted(self_time.items(), key=lambda kv: -kv[1])[:n]]


async def run(args):
    port = args.port
    profile = args.profile or f"/tmp/arena-smooth-chrome-{port}"
    # --slow-cpu N: the whole browser shares N efficiency cores at a CPU quota,
    # which also slows workers (DevTools CPU throttling does not reach them).
    wrap = []
    if args.slow_cpu:
        cores = ",".join(str(8 + i) for i in range(int(args.slow_cpu)))
        wrap = ["systemd-run", "--user", "--scope", "-q", "-p", f"CPUQuota={int(args.slow_quota)}%", "taskset", "-c", cores]
    cmd = wrap + ["google-chrome", f"--remote-debugging-port={port}", f"--user-data-dir={profile}", "--no-first-run",
           "--no-default-browser-check", "--window-size=600,1000", "--disable-background-timer-throttling",
           "--disable-renderer-backgrounding", "--disable-backgrounding-occluded-windows"]
    if args.headless:
        cmd += ["--headless=new", "--enable-unsafe-swiftshader"]
    else:
        cmd += ["--ozone-platform=wayland"]
    log = open(f"/tmp/arena-smooth-chrome-{port}.log", "w")
    chrome = subprocess.Popen(cmd + ["about:blank"], stdout=log, stderr=log)
    try:
        version = None
        for _ in range(100):
            try:
                version = json.load(urllib.request.urlopen(f"http://127.0.0.1:{port}/json/version"))
                break
            except Exception:
                time.sleep(0.2)
        async with websockets.connect(version["webSocketDebuggerUrl"], max_size=2 ** 30, ping_interval=None) as ws:
            counter = 0
            pending: dict[int, asyncio.Future] = {}
            events: asyncio.Queue = asyncio.Queue()

            trace = open(f"/tmp/arena-smooth-cdp-{port}.log", "w")

            async def reader():
                async for raw in ws:
                    msg = json.loads(raw)
                    trace.write(raw[:300] + "\n"); trace.flush()
                    if "id" in msg and msg["id"] in pending:
                        pending.pop(msg["id"]).set_result(msg)
                    else:
                        await events.put(msg)

            reader_task = asyncio.create_task(reader())

            async def call(method, params=None, session=None):
                nonlocal counter
                counter += 1
                fut = asyncio.get_event_loop().create_future()
                pending[counter] = fut
                msg = {"id": counter, "method": method, "params": params or {}}
                if session:
                    msg["sessionId"] = session
                await ws.send(json.dumps(msg))
                return await asyncio.wait_for(fut, 60)

            async def ev(expr, session, await_promise=False):
                r = await call("Runtime.evaluate", {"expression": expr, "awaitPromise": await_promise, "returnByValue": True}, session)
                res = r.get("result", {})
                if "exceptionDetails" in res:
                    return {"__error": res["exceptionDetails"].get("text"), "detail": res["exceptionDetails"]}
                return res.get("result", {}).get("value")

            targets = await call("Target.getTargets")
            page_target = [t for t in targets["result"]["targetInfos"] if t["type"] == "page"][0]["targetId"]
            att = await call("Target.attachToTarget", {"targetId": page_target, "flatten": True})
            page = att["result"]["sessionId"]
            await call("Page.enable", session=page)
            await call("Runtime.enable", session=page)
            await call("Page.addScriptToEvaluateOnNewDocument", {"source": BOOT_HOOK}, page)
            stage = "Response" if not args.static else "Request"
            patterns = [{"urlPattern": "*painter-worker.js*", "requestStage": stage},
                        {"urlPattern": "*compositor-worker.js*", "requestStage": stage}]
            if args.wasm:
                patterns.append({"urlPattern": "*/assets/files/*.wasm", "requestStage": "Request"})
            if args.static:
                patterns.append({"urlPattern": "*/clashroyale/*.js*", "requestStage": "Request"})
            await call("Fetch.enable", {"patterns": patterns})
            await call("Target.setAutoAttach", {"autoAttach": True, "waitForDebuggerOnStart": False, "flatten": True}, page)
            if args.cpu > 1:
                await call("Emulation.setCPUThrottlingRate", {"rate": args.cpu}, page)
            await call("Page.navigate", {"url": BASE + "/"}, page)
            await asyncio.sleep(2.5)
            worker_session = {"id": None}

            async def pump():
                # Each event is handled in its own task: a worker blocked in
                # importScripts on an intercepted file cannot answer a call
                # made to it before that file is served.
                while True:
                    msg = await events.get()
                    asyncio.create_task(handle_event(msg))

            async def handle_event(msg):
                if True:
                    if msg.get("method") == "Fetch.requestPaused":
                        import base64
                        rid = msg["params"]["requestId"]
                        url = msg["params"]["request"]["url"].split("?")[0]
                        local = None
                        if args.wasm and url.endswith(".wasm") and "/assets/files/" in url:
                            local, kind = open(args.wasm, "rb").read(), "application/wasm"
                        elif args.static and "/assets/" not in url and "/api/" not in url:
                            name = url.rsplit("/", 1)[-1]
                            path = os.path.join(args.static, name)
                            if name.endswith(".js") and os.path.isfile(path):
                                local = open(path, "rb").read()
                                kind = {"js": "text/javascript", "html": "text/html", "css": "text/css"}.get(name.rsplit(".", 1)[-1], "application/octet-stream")
                        if local is not None:
                            if url.endswith("painter-worker.js") or url.endswith("compositor-worker.js"):
                                local = (WORKER_HOOK + "\n").encode() + local
                                print("worker script hooked (local)", flush=True)
                            await call("Fetch.fulfillRequest", {"requestId": rid, "responseCode": 200,
                                                                "responseHeaders": [{"name": "Content-Type", "value": kind}, {"name": "Cache-Control", "value": "no-store"}],
                                                                "body": base64.b64encode(local).decode()})
                            return
                        if msg["params"].get("responseStatusCode") is None:
                            await call("Fetch.continueRequest", {"requestId": rid})
                            return
                        body = await call("Fetch.getResponseBody", {"requestId": rid})
                        raw = body["result"]["body"]
                        text = base64.b64decode(raw).decode() if body["result"].get("base64Encoded") else raw
                        text = WORKER_HOOK + "\n" + text
                        headers = [h for h in msg["params"].get("responseHeaders", []) if h["name"].lower() not in ("content-length", "content-encoding")]
                        await call("Fetch.fulfillRequest", {"requestId": rid, "responseCode": msg["params"].get("responseStatusCode", 200),
                                                            "responseHeaders": headers, "body": base64.b64encode(text.encode()).decode()})
                        print("worker script hooked", flush=True)
                        return
                    if msg.get("method") == "Target.attachedToTarget":
                        info = msg["params"]["targetInfo"]
                        sid = msg["params"]["sessionId"]
                        if info["type"] == "worker" and "painter-worker" in info.get("url", ""):
                            await call("Runtime.enable", session=sid)
                            if args.cpu > 1:
                                try:
                                    await call("Emulation.setCPUThrottlingRate", {"rate": args.cpu}, sid)
                                except Exception:
                                    pass
                            worker_session["id"] = sid
                            if args.profile_worker:
                                await call("Profiler.enable", session=sid)
                                await call("Profiler.setSamplingInterval", {"interval": 200}, sid)
                                await call("Profiler.start", session=sid)

            pump_task = asyncio.create_task(pump())
            if args.human:
                sides = [{"controller": "human", "deck": args.deck_a}, {"controller": args.model_b, "deck": args.deck_b}]
                mode = "human"
            else:
                sides = [{"controller": args.model_a, "deck": args.deck_a}, {"controller": args.model_b, "deck": args.deck_b}]
                mode = "watch"
            body = json.dumps({"mode": mode, "sides": sides})
            # A crashed earlier probe can leave this profile's match open.
            await ev(f"fetch('{BASE}/api/matches').then(r=>r.json()).then(d=>Promise.all((d.matches||[]).filter(m=>m.mine!==false).map(m=>"
                     f"fetch('{BASE}/api/matches/'+m.id+'/close',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:'{{}}'}}))))", page, True)
            await asyncio.sleep(0.5)
            t_create = time.monotonic()
            t_create_epoch = time.time() * 1000
            res = await ev(f"fetch('{BASE}/api/matches',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:{json.dumps(body)}}}).then(r=>r.json())", page, True)
            if not isinstance(res, dict) or "id" not in res:
                raise SystemExit(f"create failed: {res}")
            mid = res["id"]
            print("match", mid, flush=True)
            await call("Page.navigate", {"url": f"{BASE}/play.html?id={mid}"}, page)
            ready_at = None
            for _ in range(1200):
                await asyncio.sleep(0.1)
                ok = await ev("typeof viewReady !== 'undefined' && viewReady ? 1 : 0", page)
                if ok == 1:
                    ready_at = time.monotonic() - t_create
                    break
            print(f"first frame {ready_at:.2f}s after create" if ready_at else "never ready", flush=True)
            print("page hook:", await ev(PAGE_HOOK, page), flush=True)
            boot = await ev("JSON.stringify(window.__boot || [])", page)
            boot = json.loads(boot) if isinstance(boot, str) else []
            boot_rows = [[round((t - t_create_epoch) / 1000, 2), what] for t, what in boot]
            print("boot timeline (s after create):", boot_rows, flush=True)
            plays = 0
            t0 = time.monotonic()
            rematched = False
            while time.monotonic() - t0 < args.seconds:
                await asyncio.sleep(1)
                if args.rematch_at and not rematched and time.monotonic() - t0 >= args.rematch_at:
                    rematched = True
                    print("rematch:", await ev("control('reset').then(() => 'sent')", page, True), flush=True)
                if args.human and int(time.monotonic() - t0) % 4 == 0:
                    # Select a slot and play it on our side of the field, like a person tapping.
                    await ev("(()=>{const c=cur&&cur.hands&&cur.hands[0]&&cur.hands[0].find(c=>c.available);"
                             "if(!c||!layout)return 0;selectSlot(c.slot);const[l,t,w,h]=layout.field;"
                             "send('play',{...selection,x:9000,y:10000});return 1;})()", page)
                    plays += 1
                    if args.shots and plays <= args.shots:
                        import base64
                        for delay in (0.25, 0.6):
                            await asyncio.sleep(delay if delay == 0.25 else 0.35)
                            shot = await call("Page.captureScreenshot", {"format": "png"}, page)
                            path = f"/tmp/arena-shot-{plays}-{int(delay * 1000)}.png"
                            open(path, "wb").write(base64.b64decode(shot["result"]["data"]))
                            print("screenshot", path, flush=True)
            diag = await ev("JSON.stringify(arenaDiagnostics())", page)
            wsid = worker_session["id"]
            profile = None
            if args.profile_worker and wsid:
                profile = (await call("Profiler.stop", session=wsid))["result"]["profile"]
            worker = await ev("new Promise((resolve) => { const on = (e) => { if (e.data && e.data.type === '__probe') { painterWorker.removeEventListener('message', on); resolve(e.data.data); } };"
                              " painterWorker.addEventListener('message', on); painterWorker.postMessage({type: '__probe_dump'}); setTimeout(() => resolve(null), 10000); })", page, True)
            pagedata = await ev("JSON.stringify(window.__page)", page)
            comp = await ev("typeof compositorWorker === 'undefined' || !compositorWorker ? null : new Promise((resolve) => { const on = (e) => { if (e.data && e.data.type === '__probe') { compositorWorker.removeEventListener('message', on); resolve(e.data.data); } };"
                            " compositorWorker.addEventListener('message', on); compositorWorker.postMessage({type: '__probe_dump'}); setTimeout(() => resolve(null), 10000); })", page, True)
            await ev(f"fetch('{BASE}/api/matches/{mid}/close',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:'{{}}'}}).then(r=>r.status)", page, True)
            pump_task.cancel()
            reader_task.cancel()
            worker = json.loads(worker) if isinstance(worker, str) else {"renders": [], "rafs": [], "steps": [], "observes": [], "parses": [], "pushes": [], "intervals": []}
            if isinstance(comp, str):
                comp = json.loads(comp)
                # The compositor worker draws; the painter worker steps.
                worker["painter_rafs"] = worker["rafs"]
                for key in ("renders", "rafs", "pushes", "newtex", "hashed"):
                    worker[key] = comp.get(key, worker.get(key))
                worker["split"] = True
            pagedata = json.loads(pagedata) if isinstance(pagedata, str) else {}
            summary = summarize(worker, pagedata)
            summary["first_frame_s"] = round(ready_at, 2) if ready_at else None
            summary["boot"] = boot_rows
            summary["cpu_rate"] = args.cpu
            summary["diag"] = json.loads(diag) if isinstance(diag, str) else diag
            result = {"summary": summary, "worker": worker, "page": pagedata, "args": vars(args)}
            if profile:
                summary["worker_profile_top"] = profile_top(profile)
            if args.out:
                with open(args.out, "w") as fh:
                    json.dump(result, fh)
            print(json.dumps(summary, indent=1))
    finally:
        chrome.terminate()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("seconds", nargs="?", type=float, default=30)
    ap.add_argument("--cpu", type=float, default=1)
    ap.add_argument("--headless", action="store_true")
    ap.add_argument("--human", action="store_true")
    ap.add_argument("--model-a", default="banana-hog-2")
    ap.add_argument("--model-b", default="banana-2")
    ap.add_argument("--deck-a", default="hog-mirror")
    ap.add_argument("--deck-b", default="log-bait")
    ap.add_argument("--port", type=int, default=9351)
    ap.add_argument("--profile", default="")
    ap.add_argument("--out", default="")
    ap.add_argument("--profile-worker", action="store_true")
    ap.add_argument("--slow-cpu", type=int, default=0, help="pin the browser to this many E-cores")
    ap.add_argument("--shots", type=int, default=0, help="human mode: screenshots after the first N plays")
    ap.add_argument("--rematch-at", type=float, default=0, help="send a rematch (control reset) after this many seconds")
    ap.add_argument("--slow-quota", type=float, default=80, help="CPU quota in percent for --slow-cpu")
    ap.add_argument("--wasm", default="", help="serve this painter wasm instead of the deployed one")
    ap.add_argument("--static", default="", help="serve page/worker scripts from this directory")
    args = ap.parse_args()
    asyncio.run(run(args))


if __name__ == "__main__":
    main()
