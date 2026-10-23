"""Time real banana-hog-2-config training waves (scratch run dir; never touches engine/runs).

usage: python bench_wave.py --code-root <repo>/engine --workers 12 --waves 1 --seed 777 --run-dir /tmp/x [--extra "--flag value ..."]
Run with cwd = --code-root. Prints one JSON object with per-phase timings.
"""
import argparse, json, shlex, sys, time, datetime as dt
from pathlib import Path

ap = argparse.ArgumentParser()
ap.add_argument("--code-root", required=True)
ap.add_argument("--workers", type=int, default=12)
ap.add_argument("--waves", type=int, default=1)
ap.add_argument("--seed", type=int, default=777)
ap.add_argument("--run-dir", required=True)
ap.add_argument("--extra", default="", help="extra trainer CLI args")
opt = ap.parse_args()
code = Path(opt.code_root).resolve()
sys.path.insert(0, str(code))
MAIN = Path("/home/cochon/Documents/mini-project/clash-royale-ai/engine")
from rl import banana_hog_overnight as ov
from rl import banana_hog_retrain as rt
from rl import banana_hog_league as core
from rl import banana_fast_policy as fp

argv = ["--run-dir", opt.run_dir, "--engine", str(MAIN / "runs/banana-hog-2/bin/rust_league_rl"),
        "--pool", str(MAIN / "runs/banana-hog-2/pool.json"), "--parent", str(MAIN / "runs/banana-hog-1/models/final.pt"),
        "--checkpoint", str(MAIN / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"),
        "--workers", str(opt.workers), "--duration-seconds", "3600", "--max-waves", str(opt.waves),
        "--trainable", "all", "--seed", str(opt.seed)] + shlex.split(opt.extra)
captured = {}
orig_parse = __import__("argparse").ArgumentParser.parse_args
def grab(self, args=None, namespace=None):
    captured["args"] = orig_parse(self, argv, namespace); raise SystemExit(0)
__import__("argparse").ArgumentParser.parse_args = grab
try:
    rt.main(ov.OvernightLeague, ov.parser_config)
except SystemExit:
    pass
__import__("argparse").ArgumentParser.parse_args = orig_parse
args = captured["args"]
args.deadline = dt.datetime.fromtimestamp(time.time() + 3600, dt.timezone.utc).isoformat()

T = {"cache_request_s": 0., "act_batch_s": 0., "act_batch_calls": 0, "act_rows": 0, "launch_s": 0., "rounds": 0,
     "wait_steps_s": 0., "consume_s": 0., "publish_s": 0., "update_s": 0., "play_s": 0.}
ENG = {"step_s": 0., "step_n": 0, "reset_s": 0., "reset_n": 0, "mask_s": 0., "mask_n": 0}
def wrap(owner, name, key, count=None, static=False):
    if not hasattr(owner, name):
        return
    orig = getattr(owner, name)
    raw = orig
    def timed(*a, **k):
        t = time.perf_counter(); r = raw(*a, **k); T[key] += time.perf_counter() - t
        if count: T[count] += 1
        return r
    setattr(owner, name, staticmethod(timed) if isinstance(owner.__dict__.get(name), staticmethod) else timed)
wrap(fp.FastBananaPolicy, "_cache_request", "cache_request_s")
orig_act = fp.FastBananaPolicy.act_batch
def timed_act(self, requests, *a, **k):
    t = time.perf_counter(); r = orig_act(self, requests, *a, **k)
    T["act_batch_s"] += time.perf_counter() - t; T["act_batch_calls"] += 1; T["act_rows"] += len(requests); return r
fp.FastBananaPolicy.act_batch = timed_act
wrap(core.League, "_launch_steps", "launch_s", "rounds")
wrap(core.League, "_completed_steps", "wait_steps_s")
wrap(core.League, "_consume_step", "consume_s")
wrap(core.League, "publish", "publish_s")
wrap(core.League, "play", "play_s")
orig_update = rt.StableLeague.update
def timed_update(self, *a, **k):
    t = time.perf_counter(); r = orig_update(self, *a, **k); T["update_s"] += time.perf_counter() - t; return r
rt.StableLeague.update = timed_update
orig_req = core.NativeClient.request
def timed_req(self, request, timeout=90):
    t = time.perf_counter(); r = orig_req(self, request, timeout); d = time.perf_counter() - t
    key = {"step": "step", "reset": "reset", "placement_mask": "mask"}.get(request.get("op"))
    if key: ENG[key + "_s"] += d; ENG[key + "_n"] += 1
    return r
core.NativeClient.request = timed_req

constructor_start = time.perf_counter()
league = ov.OvernightLeague(args)
constructor_s = time.perf_counter() - constructor_start
for key in T: T[key] = 0 if isinstance(T[key], int) else 0.
for key in ENG: ENG[key] = 0 if isinstance(ENG[key], int) else 0.
wave_records = []
last_t, last_e, last_c = dict(T), dict(ENG), dict(league.counters)
orig_publish_measured = core.League.publish
def publish_wave(self, *a, **k):
    global last_wave_end, last_t, last_e, last_c
    result = orig_publish_measured(self, *a, **k)
    updates = self.counters.get("updates", 0)
    if k.get("force") and updates > len(wave_records):
        end = time.perf_counter()
        row = {"wave": updates, "elapsed_s": end-t0, "wave_s": end-last_wave_end,
               "games": self.counters.get("games",0)-last_c.get("games",0),
               "decisions": self.counters.get("decisions",0)-last_c.get("decisions",0),
               "phases": {key:T[key]-last_t[key] for key in T},
               "engine": {key:ENG[key]-last_e[key] for key in ENG}}
        wave_records.append(row)
        Path(opt.run_dir, "bench_waves.json").write_text(json.dumps(wave_records, indent=2))
        last_wave_end, last_t, last_e, last_c = end, dict(T), dict(ENG), dict(self.counters)
    return result
core.League.publish = publish_wave
t0 = last_wave_end = time.perf_counter()
league.run()   # --max-waves: train N waves (collection + PPO), no evaluation banks
total = time.perf_counter() - t0
c = league.counters
print(json.dumps({"code_root": str(code), "workers": opt.workers, "waves": opt.waves, "seed": opt.seed, "extra": opt.extra,
                  "constructor_s": constructor_s, "wave_records": wave_records, "cleanup_s": total-sum(w["wave_s"] for w in wave_records), "counters": dict(c), "errors": dict(league.errors), "total_s": total, "games": c.get("games", 0), "decisions": c.get("decisions", 0), "updates": c.get("updates", 0),
                  "games_per_min": 60 * c.get("games", 0) / total, "decisions_per_s": c.get("decisions", 0) / total,
                  **T, "engine": ENG, "pace": getattr(league, "pace", None)}, indent=1))
