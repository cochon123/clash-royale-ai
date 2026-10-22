#!/usr/bin/env python3
"""Persistent JSON-lines adapter to the neighboring repository's real policies.

No sibling files are modified. stdout contains exactly one JSON reply per input
line; diagnostics go to stderr. See docs/ai-bridge.md for the coordinate contract.
"""
from __future__ import annotations

import argparse
import contextlib
import importlib.util
import json
import math
import os
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_AI_REPO = ROOT.parent
MODELS = {
    "v9": ("v9", "models/policy_v9/final_model/best.pt", "Policy v9 (experimental)"),
    "v4.4.1": ("legacy", "models/policy_bc_v4.4.1/best_model.pt", "Policy v4.4.1"),
    "v4.4.2": ("legacy", "models/policy_bc_v4.4.2/best_model.pt", "Policy v4.4.2"),
}


def python_for(repo: Path) -> str:
    candidate = repo / ".venv/bin/python"
    return str(candidate) if candidate.exists() else sys.executable


def discover(repo: Path) -> dict:
    return {
        "protocol": "clash-policy-bridge-1",
        "python": python_for(repo),
        "script": str(Path(__file__).resolve()),
        "ai_repo": str(repo),
        "models": [
            {
                "id": name, "name": title,
                "available": (repo / checkpoint).is_file()
                and (repo / "src/cr_replay_pipeline").is_dir(),
                "checkpoint": str(repo / checkpoint),
                "experimental": name == "v9",
            }
            for name, (_, checkpoint, title) in MODELS.items()
        ],
    }


class Bridge:
    def __init__(self, repo: Path, default_model: str, device: str):
        self.repo, self.default_model, self.device = repo, default_model, device
        self.loaded = {}
        manifest = json.loads((ROOT / "data/policy_card_aliases.json").read_text())
        self.aliases, self.costs = {}, {}
        for row in manifest["cards"]:
            aliases = [row["internal_id"], row["policy_id"], row["engine_id"]]
            if "numeric_id" in row:
                aliases.append(str(row["numeric_id"]))
            for alias in aliases:
                self.aliases[alias] = row["policy_id"]
            self.costs[row["policy_id"]] = row["elixir"]
        # Match the simulator's recovered card costs when that catalog exists.
        catalog_path = ROOT / "data/cards.json"
        if catalog_path.exists():
            for row in json.loads(catalog_path.read_text()):
                if row["id"] in self.aliases:
                    self.costs[self.aliases[row["id"]]] = row["elixir"]

    def card(self, value):
        key = str(value)
        if key not in self.aliases:
            raise ValueError(f"No explicit policy card mapping for {key!r}")
        return self.aliases[key]

    def load(self, name):
        if name not in MODELS:
            raise ValueError(f"Unknown policy {name!r}; expected {', '.join(MODELS)}")
        if name not in self.loaded:
            kind, checkpoint, _ = MODELS[name]
            path = self.repo / checkpoint
            if not path.is_file():
                raise FileNotFoundError(f"Policy checkpoint is unavailable: {path}")
            if kind == "v9":
                from cr_replay_pipeline.v9_infer import load_model
                model, vocab, config = load_model(path, self.device)
            else:
                from cr_replay_pipeline.policy_infer import load_policy
                model, vocab, config, _ = load_policy(path.parent, device_name=self.device)
            self.loaded[name] = model, vocab, config
        return self.loaded[name]

    def predict(self, request: dict) -> dict:
        from cr_replay_pipeline.winner_dataset import BattleExample
        name = request.get("model", self.default_model)
        team = request["team"]
        if team not in {"red", "blue"}:
            raise ValueError("team must be red or blue")
        now = float(request["elapsed_ms"]) / 1000
        elixir = float(request["elixir"])
        if not math.isfinite(now) or now < 0 or not math.isfinite(elixir) or not 0 <= elixir <= 10:
            raise ValueError("elapsed_ms and elixir must be finite, nonnegative; elixir <= 10")
        deck = tuple(self.card(c) for c in request["deck"])
        opponent = tuple(self.card(c) for c in request["opponent_deck"])
        hand = [self.card(c) for c in request["hand"]]
        if len(deck) != 8 or len(opponent) != 8 or len(set(deck)) != 8 or len(set(opponent)) != 8:
            raise ValueError("Each deck must contain eight distinct supported cards")
        if len(hand) != 4 or len(set(hand)) != 4 or not set(hand) <= set(deck):
            raise ValueError("hand must contain four distinct cards from the acting deck")
        model, vocab, config = self.load(name)
        unsupported = sorted(set(deck + opponent) - set(vocab.name_to_id))
        if unsupported:
            raise ValueError(f"{name} was not trained on these cards: {', '.join(unsupported)}")
        affordable = {c for c in hand if self.costs[c] <= elixir + 1e-6}
        events = []
        previous = -1.0
        for event in request.get("history", []):
            seconds = float(event["elapsed_ms"]) / 1000
            if not math.isfinite(seconds) or not previous <= seconds <= now or seconds < 0:
                raise ValueError("history must be ordered confirmed events, never future events")
            previous = seconds
            side = event["team"]
            if side not in {"red", "blue"}:
                raise ValueError("history team must be red or blue")
            x, y = float(event["x"]), float(event["y"])
            if not 0 <= x <= 18000 or not 0 <= y <= 32000:
                raise ValueError("history position outside arena")
            card = self.card(event["card"])
            declared = deck if side == team else opponent
            if card not in declared:
                raise ValueError("history card does not belong to that side's deck")
            events.append({"seconds": seconds, "side": "team" if side == "blue" else "opponent",
                           "event_type": "card_play", "card": card, "x": round(x), "y": round(32000-y)})
        if not affordable:
            return {"wait": True, "reason": "No affordable card in hand", "requery_after_ms": 500}
        battle = BattleExample(
            battle_id=str(request.get("match_id", "local-match")),
            team_deck=deck if team == "blue" else opponent,
            opponent_deck=opponent if team == "blue" else deck,
            team_wins=0, events=tuple(events),
        )
        acting_side = "team" if team == "blue" else "opponent"
        if MODELS[name][0] == "v9":
            from cr_replay_pipeline.v9_infer import predict
            proposal = predict(model, vocab, self.costs, battle, len(events),
                               agent_side=acting_side, now=now, observed_hand=affordable)
            if proposal.get("status") != "proposal":
                return {"wait": True, "reason": proposal.get("reason", "Needs observation"), "requery_after_ms": 500}
            if proposal.get("wait_for_opponent", False):
                return {"wait": True, "reason": "Model clock predicts opponent event first",
                        "requery_after_ms": 500, "own_next_probability": proposal["own_next_probability"]}
            x = round(proposal["x"] * 18000)
            y = round((1-proposal["y"] if team == "blue" else proposal["y"]) * 32000)
        else:
            from cr_replay_pipeline.policy_infer import predict_next_action
            import torch
            proposal = predict_next_action(
                model, vocab, self.costs, battle, torch.device(self.device),
                acting_side=acting_side, slot_decode="argmax", placement_decode="argmax",
                min_context=0, max_context=int(config.get("max_context", 64)),
                threat_dim=int(config.get("threat_dim", 0)), prefer_cards=affordable,
                think_steps=int(config.get("eval_think_steps", config.get("max_think_steps", 0))),
                now_seconds=now,
            )
            if proposal["event_type"] != "card_play":
                return {"wait": True, "reason": "Champion abilities are unsupported", "requery_after_ms": 500}
            x, y = int(proposal["x"]), 32000-int(proposal["y"])
        card = proposal["card"]
        if card not in affordable:
            raise RuntimeError("Policy output violated the affordable hand mask")
        delay = float(proposal.get("delay_seconds", 0))
        if not math.isfinite(delay):
            raise RuntimeError("Nonfinite model timing prediction")
        return {
            "wait": False, "hand_index": hand.index(card), "card": request["hand"][hand.index(card)],
            "x": x, "y": y, "delay_ms": round(max(0, min(delay, 12)) * 1000),
            "model": name, "requery_after_ms": 500,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ai-repo", type=Path, default=Path(os.environ.get("CLASH_AI_REPO", DEFAULT_AI_REPO)))
    parser.add_argument("--model", default="v9", choices=MODELS)
    parser.add_argument("--device", default="cpu", choices=("cpu", "cuda"))
    parser.add_argument("--list", action="store_true")
    args = parser.parse_args()
    args.ai_repo = args.ai_repo.resolve()
    if args.list:
        print(json.dumps(discover(args.ai_repo)))
        return
    # Reuse the existing environment without installing or altering anything.
    if importlib.util.find_spec("torch") is None:
        candidate = python_for(args.ai_repo)
        if candidate != sys.executable and not os.environ.get("CLASH_BRIDGE_REEXEC"):
            os.environ["CLASH_BRIDGE_REEXEC"] = "1"
            os.execv(candidate, [candidate, str(Path(__file__).resolve()), *sys.argv[1:]])
        raise RuntimeError(f"PyTorch is unavailable. Run with {candidate}")
    sys.path.insert(0, str(args.ai_repo / "src"))
    os.environ.setdefault("OMP_NUM_THREADS", "1")
    os.environ.setdefault("MKL_NUM_THREADS", "1")
    try:
        with contextlib.redirect_stdout(sys.stderr):
            import torch
            torch.set_num_threads(1)
            torch.set_num_interop_threads(1)
            bridge = Bridge(args.ai_repo, args.model, args.device)
            bridge.load(args.model)
    except Exception as exc:
        print(json.dumps({"ready": False, "error": str(exc)}), flush=True)
        return 1
    print(json.dumps({"ready": True, "model": args.model}), flush=True)
    for line in sys.stdin:
        try:
            request = json.loads(line)
            if not isinstance(request, dict):
                raise ValueError("Each request must be a JSON object")
            with contextlib.redirect_stdout(sys.stderr), torch.inference_mode():
                result = bridge.predict(request)
        except Exception as exc:
            print(f"policy bridge: {type(exc).__name__}: {exc}", file=sys.stderr, flush=True)
            result = {"wait": True, "error": str(exc), "requery_after_ms": 500}
        print(json.dumps(result, allow_nan=False), flush=True)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as exc:
        print(json.dumps({"ready": False, "error": str(exc)}), flush=True)
        sys.exit(1)
