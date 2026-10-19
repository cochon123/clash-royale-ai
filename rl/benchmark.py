"""Measure actual end-to-end games/minute through the Gym interface."""
import argparse
import json
from pathlib import Path
import time

import numpy as np

from .native_clash import NativeClashEnv, sample_affordable_action


def run(seconds=60.0, seed=1, ticks_per_step=20, max_ticks=7200):
    env = NativeClashEnv(ticks_per_step=ticks_per_step, max_ticks=max_ticks)
    started = time.perf_counter()
    episodes, decision_steps, ticks, accepted, attempts = [], 0, 0, 0, 0
    active = None
    error = None
    try:
        while time.perf_counter() - started < seconds:
            index = len(episodes)
            episode_start = time.perf_counter()
            _, info = env.reset(seed=seed + index)
            rng = np.random.default_rng(seed + index + 1_000_000)
            active = {"seed": seed + index, "reset_seconds": info["reset_seconds"], "steps": 0}
            while time.perf_counter() - started < seconds:
                action = np.array(sample_affordable_action(env.raw_observation, 0, rng), dtype=np.int64)
                _, reward, terminated, truncated, info = env.step(action)
                active["steps"] += 1
                decision_steps += 1
                ticks += info["ticks_advanced"]
                for result in info["actions"]:
                    attempts += 1
                    accepted += bool(result["accepted"])
                if terminated or truncated:
                    active.update(duration_seconds=time.perf_counter() - episode_start,
                                  terminated=terminated, truncated=truncated,
                                  reward=reward, final_tick=env.raw_observation["tick"],
                                  winner_side=env.raw_observation.get("winner_side"))
                    episodes.append(active)
                    print(json.dumps({"episode": len(episodes), **active}), flush=True)
                    active = None
                    break
            if active is not None:
                break
    except Exception as exc:
        error = f"{type(exc).__name__}: {exc}"
    finally:
        env.close()
    elapsed = time.perf_counter() - started
    complete = sum(e["terminated"] for e in episodes)
    report = {
        "status": "error" if error else "complete", "error": error,
        "requested_seconds": seconds, "elapsed_seconds": elapsed,
        "completed_games": complete,
        "completed_games_per_minute": complete * 60 / elapsed,
        "truncated_episodes": sum(e["truncated"] for e in episodes),
        "unfinished_episode": active,
        "decision_steps": decision_steps, "native_ticks": ticks,
        "native_ticks_per_second": ticks / elapsed,
        "card_attempts": attempts, "accepted_cards": accepted,
        "episodes": episodes, "ticks_per_step": ticks_per_step,
        "max_ticks": max_ticks, "seed": seed,
        "scope": "One process, two external affordable-random policies through the original native engine. Includes snapshot reset, observations, IPC and actions. Fixed native scenario/decks. No training or rendering. Truncations and unfinished episodes are excluded from completed-game throughput.",
    }
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seconds", type=float, default=60)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--ticks-per-step", type=int, default=20)
    parser.add_argument("--max-ticks", type=int, default=7200)
    parser.add_argument("--out", type=Path, default=Path("analysis/native-rl/benchmark.json"))
    args = parser.parse_args()
    if args.seconds <= 0:
        parser.error("--seconds must be positive")
    report = run(args.seconds, args.seed, args.ticks_per_step, args.max_ticks)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    raise SystemExit(1 if report["error"] else 0)


if __name__ == "__main__":
    main()
