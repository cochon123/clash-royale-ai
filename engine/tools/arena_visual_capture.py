#!/usr/bin/env python3
"""Record deterministic original-engine frames for offline presentation QA.

No model is trained and no live game is launched. Every simulated tick is
retained so animation inference sees the same history during each replay.
The JSONL files are private engine input, not browser fixture responses.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
ENGINE = ROOT / "engine"


class Engine:
    def __init__(self):
        self.process = subprocess.Popen([
            str(ENGINE / "native-engine/target/release/native-watch"), "--checkpoint",
            str(ENGINE / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"),
        ], cwd=ENGINE, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)

    def request(self, value):
        self.process.stdin.write(json.dumps(value) + "\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError("capture engine exited before replying")
        response = json.loads(line)
        if not response.get("ok"):
            raise RuntimeError(response.get("error"))
        return response["result"]

    def close(self):
        if self.process.poll() is None:
            self.process.terminate()
        try:
            self.process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()


def record(engine, deck, opponent, target, ticks, seed, preferred=None):
    result = engine.request({"op": "reset", "deck0": deck, "deck1": opponent,
                             "seed": seed, "max_ticks": ticks + 200})
    base_tick = result["observation"]["tick"]
    seen, played, actions, rows = [set(), set()], [[], []], [], set()
    target.mkdir(parents=True, exist_ok=True)
    with (target / "frames.jsonl").open("w") as output:
        for index in range(ticks):
            obs, frame = result["observation"], result["frame"]
            commands = []
            if index >= 100 and index % 10 == 0:
                for side in (0, 1):
                    if preferred and side == 1:
                        continue
                    hand = sorted(obs["hands"][side], key=lambda card: (
                        card["name"] in seen[side], card.get("cost") or 0, card["slot"]))
                    # Accumulate elixir for an untested expensive card instead
                    # of endlessly spending it on already-captured cheap ones.
                    untested = [c for c in hand if c["name"] not in seen[side]]
                    if untested:
                        hand = untested
                    if preferred:
                        compact = lambda name: "".join(c.lower() for c in (name or '') if c.isalnum())
                        if any(compact(name) == compact(preferred) for name in seen[side]):
                            continue
                        requested = [c for c in hand if compact(c["name"]) == compact(preferred)]
                        if requested:
                            hand = requested
                    for card in hand:
                        if not card.get("available", True) or (card.get("cost") or 0) > obs["elixir"][side]:
                            continue
                        for x, y in ((3500, 14000), (14500, 11000), (9000, 9000), (3500, 24000)):
                            command = {"side": side, "slot": card["slot"], "x": x,
                                       "y": y if side == 0 else 32000 - y}
                            validation = engine.request(dict(command, op="validate_target"))
                            if validation.get("valid") and validation.get("affordable"):
                                commands.append(command)
                                break
                        if commands and commands[-1]["side"] == side:
                            break
            frame = dict(frame, elapsed_ms=(obs["tick"] - base_tick) * 50, match_id=1,
                         runtime_status="running", playback_speed=1,
                         controller_names=["Deterministic capture", "Deterministic capture"],
                         hands=[[dict(c, card_id=c.get("card_id") or 0,
                                      cost=c.get("cost") or 0, name=c.get("name") or "")
                                 for c in hand] for hand in obs["hands"]],
                         plays=[p for p in actions if obs["tick"] - p["tick"] <= 40])
            rows.update(entity["row"] for entity in frame["entities"])
            output.write(json.dumps(frame, separators=(",", ":")) + "\n")
            if result.get("terminated") or result.get("truncated"):
                break
            result = engine.request({"op": "step", "actions": commands, "ticks": 1})
            for command, accepted in zip(commands, result.get("actions", [])):
                if not accepted.get("accepted"):
                    continue
                side = command["side"]
                card = next(c for c in obs["hands"][side] if c["slot"] == command["slot"])
                seen[side].add(card["name"])
                played[side].append(card["name"])
                actions.append(dict(command, id=len(actions)+1, tick=obs["tick"],
                                    name=card["name"], card_id=card["card_id"], cost=card["cost"]))
    summary = {"seed": seed, "decks": [deck, opponent], "frames": index + 1,
               "played": played, "observed_rows": sorted(rows), "source": "original-native-engine"}
    if preferred:
        summary["preferred_card"] = preferred
    (target / "capture.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / ".local-arena/captures")
    parser.add_argument("--ticks", type=int, default=1200)
    parser.add_argument("--presets", nargs="*")
    parser.add_argument("--all-cards", action="store_true", help="Capture every selectable card in deterministic eight-card groups")
    parser.add_argument("--targets", nargs="+", help="Focus on each named native card against an idle opponent")
    args = parser.parse_args()
    if not 101 <= args.ticks <= 7200:
        parser.error("--ticks must be 101..7200")
    args.output = args.output.resolve()
    if (ENGINE / "analysis").resolve() in args.output.parents or (ENGINE / "runs").resolve() in args.output.parents:
        parser.error("capture output must be private to the worktree")
    os.environ.setdefault("CLASH_ENGINE_ROOT", str(ENGINE))
    presets = json.loads((ENGINE / "data/fast_league_decks.json").read_text())["decks"]
    if args.all_cards or args.targets:
        import sys
        sys.path.insert(0, str(ROOT))
        from webapps.common.catalog import cards
        natives = [c["native"] for c in cards().values()]
        groups = []
        if args.targets:
            unknown = set(args.targets) - set(natives)
            if unknown:
                parser.error(f"unknown native cards: {sorted(unknown)}")
            for index, name in enumerate(args.targets):
                deck = [name] + [n for n in natives if n != name][:7]
                groups.append({"id": f"target-{index:03d}", "native_cards": deck, "preferred": name})
        else:
            for start in range(0, len(natives), 8):
                deck = natives[start:start + 8]
                deck += [name for name in natives if name not in deck][:8 - len(deck)]
                groups.append({"id": f"cards-{start:03d}", "native_cards": deck})
        presets = groups
    if args.presets:
        unknown = set(args.presets) - {p["id"] for p in presets}
        if unknown:
            parser.error(f"unknown presets: {sorted(unknown)}")
        presets = [p for p in presets if p["id"] in args.presets]
    engine = Engine()
    started = time.monotonic()
    print("DONE/TOTAL  ELAPSED_SECONDS  ETA_SECONDS  PRESET", flush=True)
    try:
        for index, preset in enumerate(presets):
            record(engine, preset["native_cards"], preset["native_cards"],
                   args.output / preset["id"], args.ticks, 7921 + index, preset.get("preferred"))
            elapsed = time.monotonic() - started
            eta = elapsed / (index + 1) * (len(presets) - index - 1)
            print(f"{index+1}/{len(presets):<9} {elapsed:15.1f} {eta:12.1f}  {preset['id']}", flush=True)
    finally:
        engine.close()


if __name__ == "__main__":
    main()
