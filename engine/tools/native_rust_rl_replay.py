#!/usr/bin/env python3
"""Record and replay deterministic JSONL requests for native-rl binaries."""

import argparse
import hashlib
import json
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))


def sha256_file(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def json_hash(value):
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def semantic_response(response):
    return {key: value for key, value in response.items()
            if key not in ("reset_seconds", "step_seconds")}


def record_trace(path, binary, seed, stepsmax):
    import numpy as np
    from rl.native_clash import NativeClashEnv, sample_affordable_action

    binary = binary.resolve()
    env = NativeClashEnv(binary=binary)
    records, timings = [], []
    original_request = env._request

    def capture(request):
        response = original_request(request)
        records.append({"request": request, "expected": semantic_response(response)})
        elapsed = {k: response[k] for k in ("reset_seconds", "step_seconds") if k in response}
        if elapsed:
            timings.append({"index": len(records) - 1, **elapsed})
        return response

    env._request = capture
    policy_rng = np.random.default_rng(seed + 1_000_000)
    final_reward = 0.0
    final_terminated = False
    final_truncated = False
    try:
        env.reset(seed=seed)
        for _ in range(stepsmax):
            action = sample_affordable_action(env.raw_observation, 0, policy_rng)
            _, reward, terminated, truncated, _ = env.step(action)
            final_reward = float(reward)
            final_terminated, final_truncated = bool(terminated), bool(truncated)
            if terminated or truncated:
                break
        raw = env.raw_observation or {}
        checkpoint = env.checkpoint / "checkpoint.json"
        checkpoint_json = json.loads(checkpoint.read_text())
        trace = {
            "format": "native-rust-rl-trace-v1",
            "source_sha256": checkpoint_json["source_sha256"],
            "binary_sha256": sha256_file(binary),
            "checkpoint_path": str(env.checkpoint),
            "checkpoint_sha256": sha256_file(checkpoint),
            "policy_seed": seed,
            "policy_rng_seed": seed + 1_000_000,
            "steps_max": stepsmax,
            "requests": records,
            "timings": timings,
            "final": {
                "tick": raw.get("tick"),
                "winner_side": raw.get("winner_side"),
                "reward": final_reward,
                "terminated": final_terminated,
                "truncated": final_truncated,
                "step_count": sum(r["request"].get("op") == "step" for r in records),
            },
        }
    finally:
        env.close()
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(trace, indent=2) + "\n")
    print(json.dumps({"recorded": str(path), "requests": len(records),
                      "trace_sha256": sha256_file(path), "binary_sha256": trace["binary_sha256"],
                      "final": trace["final"]}, indent=2))


def replay_trace(trace_path, binary, out):
    from rl.native_clash import NativeClashEnv

    trace_path = trace_path.resolve()
    binary = binary.resolve()
    trace = json.loads(trace_path.read_text())
    if trace.get("format") != "native-rust-rl-trace-v1":
        raise ValueError("unsupported trace format")
    checkpoint = Path(trace["checkpoint_path"])
    env = NativeClashEnv(binary=binary, checkpoint=checkpoint)
    differences = []
    compared = 0
    error = None
    started = time.monotonic()
    try:
        for index, item in enumerate(trace["requests"]):
            request = item["request"]
            try:
                actual = semantic_response(env._request(request))
            except Exception as exc:
                error = {"index": index, "request": request,
                         "expected": item["expected"], "error": f"{type(exc).__name__}: {exc}"}
                break
            compared += 1
            expected = item["expected"]
            if actual != expected:
                if len(differences) < 100:
                    differences.append({"index": index, "request": request,
                                        "expected": expected, "actual": actual})
    finally:
        env.close()
    report = {
        "matched": error is None and not differences and compared == len(trace["requests"]),
        "source_sha256": trace["source_sha256"],
        "checkpoint_sha256": trace["checkpoint_sha256"],
        "reference_binary_sha256": trace["binary_sha256"],
        "current_binary_sha256": sha256_file(binary),
        "trace_sha256": sha256_file(trace_path),
        "trace_content_sha256": json_hash(trace),
        "request_count": len(trace["requests"]),
        "requests_compared": compared,
        "differences": differences,
        "error": error,
        "replay_seconds": time.monotonic() - started,
        "scope": "Exact semantic response equality for every recorded reset/step JSON protocol request against this binary and checkpoint. Top-level reset_seconds and step_seconds fields are excluded from comparison.",
    }
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["matched"] else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--record", type=Path, help="Record a slow/reference rollout to this JSON trace")
    mode.add_argument("--replay", type=Path, help="Replay a JSON trace against the selected binary")
    parser.add_argument("--binary", required=True, type=Path, help="native-rl executable")
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--stepsmax", type=int, default=360)
    parser.add_argument("--out", type=Path, help="Replay report path (required with --replay)")
    args = parser.parse_args()
    try:
        if args.record:
            if args.stepsmax < 1:
                raise ValueError("--stepsmax must be positive")
            record_trace(args.record, args.binary, args.seed, args.stepsmax)
            return 0
        if args.out is None:
            raise ValueError("--out is required with --replay")
        return replay_trace(args.replay, args.binary, args.out)
    except Exception as exc:
        print(f"native_rust_rl_replay: {type(exc).__name__}: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
