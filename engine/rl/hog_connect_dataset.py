"""Hog-connect counterfactuals: where a Hog Rider deals tower damage.

At every decision where Hog Rider is playable for the Hog 2.6 seat, the
generator snapshots the live session and tries a fixed set of tiles plus a
"no Hog" branch. Each branch is 8 seconds of lookahead with the opponent
submitting nothing. The session is restored before the real decision.

Output is one compressed npz and one jsonl per game, plus ``manifest.json``.
"""
from __future__ import annotations

import argparse
import json
import threading
import time
from pathlib import Path
from typing import Any, Mapping

import numpy as np

from .banana_hog_adaptive import AdaptiveHogPolicy
from .banana_policy import BananaEpisode
from .board_features import encode_board, load_row_table, vocab_for_table
from .board_policy import DEFAULT_HOG2_WEIGHTS
from .board_rewards import elixir_trade_potential
from .engine_ops import close, drop_snapshot, open_scoped, placement_mask, reset, restore, snapshot, step
from .hog_search import TOWER_ROWS, combined_potential, tile_native

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "rust-engine/target/release/rust_league_rl"
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
POOL = ROOT / "data/fast_league_decks.json"
FORMAT = "hog-connect-v1"
# Learner-perspective tiles: both bridges, a step back from each bridge,
# the pocket beside the king, and one tile in front of the king.
HOG_TILES = (
    8 * 32 + 6,
    8 * 32 + 5,
    7 * 32 + 6,
    8 * 32 + 25,
    8 * 32 + 26,
    7 * 32 + 25,
    3 * 32 + 11,
    3 * 32 + 20,
    2 * 32 + 16,
)
HOG_DECK = (
    "hog-rider", "musketeer", "ice-golem", "skeletons",
    "ice-spirit", "cannon", "fireball", "the-log",
)
HOG_NATIVE = [
    "HogRider", "Musketeer", "IceGolemite", "Skeletons",
    "IceSpirits", "Cannon", "Fireball", "Log",
]


def _compact(name: str) -> str:
    return "".join(ch.lower() for ch in str(name) if ch.isalnum())


def hog_hand_slot(hand, elixir: float):
    for card in hand:
        if not card.get("available", True):
            continue
        if _compact(card.get("name", "")) != "hogrider":
            continue
        if float(card.get("cost", 4)) <= float(elixir) + 1e-6:
            return int(card["slot"])
    return None


def enemy_tower_hp(board: Mapping[str, Any], side: int) -> int:
    total = 0
    for entity in board.get("entities", []):
        if entity.get("row") in TOWER_ROWS and int(entity.get("side", side)) != int(side):
            total += int(entity["hp"])
    return total


def hog_alive_after(before: Mapping[str, Any], after: Mapping[str, Any], side: int, placed: bool) -> int:
    if not placed:
        return 0
    start = int(before.get("tick", 0))
    for entity in after.get("entities", []):
        if entity.get("row") != "HogRider" or int(entity.get("side", -1)) != int(side):
            continue
        if int(entity.get("hp", 0)) <= 0:
            continue
        if int(entity.get("first_seen_tick", 0)) >= start:
            return 1
    return 0


def write_shard(directory: Path, name: str, decisions: list[dict[str, Any]]) -> dict[str, Any]:
    directory.mkdir(parents=True, exist_ok=True)
    rasters, tiles, damage, alive, trade, decision_ids = [], [], [], [], [], []
    meta_rows = []
    for index, decision in enumerate(decisions):
        public = {key: value for key, value in decision.items() if key != "raster"}
        public["rows"] = decision["rows"]
        meta_rows.append(public)
        for row in decision["rows"]:
            rasters.append(decision["raster"])
            tiles.append(int(row["tile"]))
            damage.append(float(row["tower_damage"]))
            alive.append(int(row["hog_alive"]))
            trade.append(float(row["elixir_trade"]))
            decision_ids.append(index)
    npz_path = directory / f"{name}.npz"
    jsonl_path = directory / f"{name}.jsonl"
    np.savez_compressed(
        npz_path,
        raster=np.stack(rasters).astype(np.float32),
        tile=np.asarray(tiles, dtype=np.int16),
        tower_damage=np.asarray(damage, dtype=np.float32),
        hog_alive=np.asarray(alive, dtype=np.uint8),
        elixir_trade=np.asarray(trade, dtype=np.float32),
        decision=np.asarray(decision_ids, dtype=np.int32),
    )
    with jsonl_path.open("w") as handle:
        for row in meta_rows:
            handle.write(json.dumps(row) + "\n")
    return {
        "npz": npz_path.name,
        "jsonl": jsonl_path.name,
        "decisions": len(decisions),
        "rows": len(tiles),
    }


def load_rows(manifest_path: Path) -> list[dict[str, Any]]:
    manifest_path = Path(manifest_path)
    manifest = json.loads(manifest_path.read_text())
    if manifest.get("format") != FORMAT:
        raise ValueError(f"expected {FORMAT}, got {manifest.get('format')}")
    directory = manifest_path.parent
    rows = []
    for shard in manifest["shards"]:
        arrays = np.load(directory / shard["npz"])
        meta = [json.loads(line) for line in (directory / shard["jsonl"]).read_text().splitlines() if line]
        if len(meta) != int(shard["decisions"]):
            raise ValueError(f"{shard['jsonl']} decision count does not match the manifest")
        count = int(arrays["tile"].shape[0])
        if count != int(shard["rows"]):
            raise ValueError(f"{shard['npz']} row count does not match the manifest")
        for index in range(count):
            decision_index = int(arrays["decision"][index])
            decision = meta[decision_index]
            rows.append({
                "game": decision["game"],
                "decision": decision_index,
                "seed": decision["seed"],
                "opponent": decision["opponent"],
                "tick": decision["tick"],
                "raster": np.array(arrays["raster"][index], copy=True),
                "tile": int(arrays["tile"][index]),
                "tower_damage": float(arrays["tower_damage"][index]),
                "hog_alive": int(arrays["hog_alive"][index]),
                "elixir_trade": float(arrays["elixir_trade"][index]),
            })
    return rows


def _probe(client, board, elixir, slot: int, horizon: int, table) -> list[dict[str, Any]]:
    image = snapshot(client)
    snapshot_id = int(image["snapshot_id"])
    rows = []
    try:
        mask = placement_mask(client, 0, slot)["mask"]
        branches = []
        for tile in HOG_TILES:
            if tile < len(mask) and mask[tile]:
                branches.append((tile, tile_native(tile, 0)))
        branches.append((-1, None))
        for tile, point in branches:
            restore(client, snapshot_id)
            actions = []
            if point is not None:
                actions.append({"side": 0, "slot": slot, "x": point[0], "y": point[1]})
            reply = step(client, horizon, actions)
            accepted = True
            if actions:
                accepted = bool(reply["actions"][0].get("accepted"))
            after = reply["board"]
            after_elixir = reply["observation"]["elixir"]
            before_trade = elixir_trade_potential(board, 0, float(elixir[0]), float(elixir[1]), table).potential
            after_trade = elixir_trade_potential(after, 0, float(after_elixir[0]), float(after_elixir[1]), table).potential
            rows.append({
                "tile": int(tile),
                "x": None if point is None else point[0],
                "y": None if point is None else point[1],
                "accepted": accepted,
                "tower_damage": float(enemy_tower_hp(board, 0) - enemy_tower_hp(after, 0)),
                "hog_alive": hog_alive_after(board, after, 0, point is not None and accepted),
                "elixir_trade": float(after_trade - before_trade),
                "leaf_potential": float(combined_potential(after, after_elixir, 0, table)),
            })
    finally:
        try:
            restore(client, snapshot_id)
        finally:
            drop_snapshot(client, snapshot_id)
    return rows


def _play_one(client, policy, policy_lock, job, table, vocab, horizon, deadline) -> dict[str, Any]:
    opponent = job["opponent"]
    reply = reset(
        client, HOG_NATIVE, list(opponent["native_cards"]),
        seed=job["seed"], max_ticks=job["match_ticks"], reward_state=True, board=True,
    )
    episode = BananaEpisode((HOG_DECK, tuple(opponent["cards"])), battle_id=f"connect-{job['seed']}")
    episode.policy_seed = job["seed"]
    observation = reply["observation"]
    board = reply["board"]
    start_tick = int(observation["tick"])
    decisions = []
    steps = 0
    while steps < job["max_steps"] and time.monotonic() < deadline:
        if reply.get("terminated") or reply.get("truncated"):
            break
        now = (int(observation["tick"]) - start_tick) / 20.0
        view = dict(observation)
        view["board"] = board
        view["perspective"] = 0
        view["tower_state"] = reply.get("reward_state") or {
            "towers": [entity for entity in board["entities"] if entity["row"] in TOWER_ROWS],
        }
        slot = hog_hand_slot(observation["hands"][0], observation["elixir"][0])
        if slot is not None and time.monotonic() + 5.0 < deadline:
            encoded = encode_board(board, 0, table, vocab)
            rows = _probe(client, board, observation["elixir"], slot, horizon, table)
            decisions.append({
                "game": job["game"],
                "seed": job["seed"],
                "opponent": opponent["id"],
                "tick": int(observation["tick"]),
                "elixir": [float(value) for value in observation["elixir"]],
                "rows": rows,
                "raster": encoded.raster.detach().cpu().numpy().astype(np.float32),
            })
        chosen = []
        with policy_lock:
            for side in (0, 1):
                side_view = dict(view)
                side_view["perspective"] = side
                action = policy.act(
                    episode, side_view, side, now, deterministic=True, placement_top_k=576,
                )
                chosen.append((side, action))
        native = [action.native_request() for _, action in chosen if not action.wait]
        submitted = [(side, action) for side, action in chosen if not action.wait]
        reply = step(client, 20, native)
        for (side, action), response in zip(submitted, reply["actions"]):
            policy.record_action(episode, action, now, bool(response.get("accepted")))
        observation = reply["observation"]
        board = reply["board"]
        steps += 1
    return {"game": job["game"], "seed": job["seed"], "opponent": opponent["id"], "steps": steps, "decisions": decisions}


def generate(args) -> dict[str, Any]:
    workers = int(args.workers)
    if workers < 1 or workers > 2:
        raise ValueError("this generator uses 1 or 2 engine processes")
    if not Path(args.binary).is_file():
        raise FileNotFoundError(args.binary)
    if not Path(args.weights).is_file():
        raise FileNotFoundError(args.weights)
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    pool = json.loads(Path(args.pool).read_text())["decks"]
    opponents = [deck for deck in pool if deck.get("id") != "hog-mirror" and not deck.get("holdout")]
    if not opponents:
        raise RuntimeError("the deck pool has no opponent decks")
    table = load_row_table()
    vocab = vocab_for_table(table)
    policy = AdaptiveHogPolicy(device=args.device, weights=args.weights)
    policy.eval()
    lock = threading.Lock()
    clients = [open_scoped(args.binary, args.checkpoint, out / f"engine-{index}.log") for index in range(workers)]
    started = time.monotonic()
    deadline = started + float(args.max_seconds)
    jobs = []
    for index in range(int(args.games)):
        jobs.append({
            "game": index,
            "seed": int(args.seed) + index,
            "opponent": opponents[index % len(opponents)],
            "match_ticks": int(args.match_ticks),
            "max_steps": int(args.max_steps),
        })
    results = []
    try:
        cursor = 0
        while cursor < len(jobs) and time.monotonic() < deadline:
            wave = jobs[cursor:cursor + workers]
            cursor += len(wave)
            threads = []
            wave_results: list[dict[str, Any] | None] = [None] * len(wave)

            def run(offset, job, client):
                wave_results[offset] = _play_one(
                    client, policy, lock, job, table, vocab, int(args.horizon_ticks), deadline,
                )

            for offset, job in enumerate(wave):
                thread = threading.Thread(target=run, args=(offset, job, clients[offset]), daemon=True)
                thread.start()
                threads.append(thread)
            for thread in threads:
                thread.join()
            for result in wave_results:
                if result is None:
                    continue
                results.append(result)
                elapsed = time.monotonic() - started
                done = len(results)
                remaining = max(0, len(jobs) - done)
                left = elapsed / done * remaining if done else 0.0
                decisions = sum(len(item["decisions"]) for item in results)
                print(
                    f"game {done}/{len(jobs)} decisions {decisions} "
                    f"elapsed {elapsed:.0f}s left ~{left:.0f}s",
                    flush=True,
                )
    finally:
        for client in clients:
            close(client)
    shards = []
    for result in results:
        if not result["decisions"]:
            continue
        shards.append(write_shard(out, f"shard-{result['game']:03d}", result["decisions"]))
    manifest = {
        "format": FORMAT,
        "created": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "weights": str(args.weights),
        "horizon_ticks": int(args.horizon_ticks),
        "tiles": list(HOG_TILES),
        "seconds": time.monotonic() - started,
        "games": [
            {"game": item["game"], "seed": item["seed"], "opponent": item["opponent"],
             "steps": item["steps"], "decisions": len(item["decisions"])}
            for item in results
        ],
        "shards": shards,
        "decisions": sum(len(item["decisions"]) for item in results),
        "rows": sum(shard["rows"] for shard in shards),
    }
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({key: manifest[key] for key in ("decisions", "rows", "seconds", "games")}, default=str), flush=True)
    return manifest


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=ROOT / "analysis-local/hog-connect")
    parser.add_argument("--binary", type=Path, default=BINARY)
    parser.add_argument("--checkpoint", type=Path, default=CHECKPOINT)
    parser.add_argument("--weights", type=Path, default=DEFAULT_HOG2_WEIGHTS)
    parser.add_argument("--pool", type=Path, default=POOL)
    parser.add_argument("--games", type=int, default=4)
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--max-seconds", type=float, default=540)
    parser.add_argument("--horizon-ticks", type=int, default=160)
    parser.add_argument("--match-ticks", type=int, default=2400)
    parser.add_argument("--max-steps", type=int, default=80)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--device", default="cpu")
    args = parser.parse_args(argv)
    generate(args)


if __name__ == "__main__":
    main()
