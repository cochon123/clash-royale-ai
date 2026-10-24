import json, sys, time, threading, http.cookiejar, urllib.request, gzip, zlib
BASE = sys.argv[1] if len(sys.argv) > 1 else "https://calgarypermit.ca/clashroyale"
SECONDS = float(sys.argv[2]) if len(sys.argv) > 2 else 60
jar = http.cookiejar.CookieJar()
opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(jar))
def req(method, path, body=None):
    data = json.dumps(body).encode() if body is not None else None
    r = urllib.request.Request(BASE + path, data=data, method=method, headers={"Content-Type": "application/json"})
    with opener.open(r, timeout=30) as resp:
        return json.loads(resp.read())
opener.open(BASE + "/api/catalog", timeout=30).read()
t0 = time.monotonic()
m = req("POST", "/api/matches", {"mode": "watch", "sides": [
    {"controller": "banana-hog-1", "deck": "hog-mirror"}, {"controller": "banana-1", "deck": "log-bait"}]})
mid = m["id"]; print("match", mid, flush=True)
stats = {"frames": [], "scenes": [], "scene_bytes": 0, "stream_bytes": 0, "first_scene": None, "first_running": None}
stop = threading.Event()
def sse(path, kind):
    r = urllib.request.Request(BASE + path, headers={"Accept-Encoding": "gzip", "Accept": "text/event-stream"})
    resp = opener.open(r, timeout=60)
    dec = zlib.decompressobj(16 + zlib.MAX_WBITS) if resp.headers.get("Content-Encoding") == "gzip" else None
    buf = b""; event = None
    while not stop.is_set():
        chunk = resp.read1(65536)
        if not chunk: break
        stats[kind + "_bytes"] += len(chunk)
        buf += dec.decompress(chunk) if dec else chunk
        while b"\n\n" in buf:
            block, buf = buf.split(b"\n\n", 1)
            ev, data = None, b""
            for line in block.split(b"\n"):
                if line.startswith(b"event:"): ev = line[6:].strip().decode()
                elif line.startswith(b"data:"): data += line[5:].strip()
            now = time.monotonic() - t0
            if kind == "stream" and ev == "frame":
                d = json.loads(data)
                if d.get("status") == "running" and stats["first_running"] is None: stats["first_running"] = now
                if "tick" in d: stats["frames"].append((now, d["tick"], d.get("status")))
            if kind == "scene" and ev == "scene":
                d = json.loads(data)
                if stats["first_scene"] is None: stats["first_scene"] = now
                stats["scenes"].append((now, d.get("tick"), d.get("display_tick"), len(data), len(d.get("blobs") or []), len(d.get("draws") or [])))
threading.Thread(target=sse, args=(f"/api/matches/{mid}/stream", "stream"), daemon=True).start()
def scene_loop():
    while not stop.is_set():
        try: sse(f"/api/matches/{mid}/scene", "scene"); return
        except Exception as e: time.sleep(0.5)
threading.Thread(target=scene_loop, daemon=True).start()
time.sleep(SECONDS)
stop.set()
try: req("POST", f"/api/matches/{mid}/close", {})
except Exception as e: print("close", e)
f = stats["frames"]; s = stats["scenes"]
print("first running frame at", stats["first_running"], "first scene at", stats["first_scene"])
run = [x for x in f if x[2] == "running"]
if len(run) > 2:
    for w0 in range(0, int(run[-1][0]) + 1, 10):
        win = [x for x in run if w0 <= x[0] < w0 + 10]
        if len(win) > 1:
            print(f"t={w0:3d}-{w0+10:3d}s stream frames/s={len(win)/10:5.1f} ticks/s={(win[-1][1]-win[0][1])/(win[-1][0]-win[0][0]):5.2f}")
    print("overall ticks/s", (run[-1][1]-run[0][1])/(run[-1][0]-run[0][0]))
if len(s) > 2:
    for w0 in range(0, int(s[-1][0]) + 1, 10):
        win = [x for x in s if w0 <= x[0] < w0 + 10]
        if len(win) > 1:
            lag = [ (x[1]-x[2]) if x[1] is not None and x[2] is not None else None for x in win]
            print(f"t={w0:3d}-{w0+10:3d}s scene pkts/s={len(win)/10:5.1f} display_ticks/s={(win[-1][2]-win[0][2])/(win[-1][0]-win[0][0]) if win[-1][2] is not None else 'na'} blobs={sum(x[4] for x in win)} json_kB={sum(x[3] for x in win)/1024:.0f} draws_avg={sum(x[5] for x in win)/len(win):.0f}")
print("wire bytes stream", stats["stream_bytes"], "scene", stats["scene_bytes"], "Mbps total", 8*(stats["stream_bytes"]+stats["scene_bytes"])/SECONDS/1e6)
json.dump(stats, open("stats.json", "w"))
