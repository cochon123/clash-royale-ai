"""Red and blue controllers. Random is always loadable headlessly."""

from __future__ import annotations

import random
from dataclasses import dataclass
from pathlib import Path

from .catalog import repo_root
from .engine import tile_from_raw

# Directories that can actually be asked to play. v9 uses a different forward
# contract and is listed separately so the UI can show why it is not selectable.
POLICY_CANDIDATES: tuple[tuple[str, str, str], ...] = (
    ("policy_bc", "models/policy_bc", "Behavior-cloning policy"),
    ("policy_bc_v3", "models/policy_bc_v3", "Policy BC v3"),
    ("policy_bc_v4", "models/policy_bc_v4", "Policy BC v4"),
    ("policy_bc_v4.1", "models/policy_bc_v4.1", "Policy BC v4.1"),
    ("policy_bc_v4.2", "models/policy_bc_v4.2_full", "Policy BC v4.2"),
    ("policy_bc_v4.3", "models/policy_bc_v4.3", "Policy BC v4.3"),
    ("policy_bc_v4.4", "models/policy_bc_v4.4", "Policy BC v4.4"),
    ("policy_bc_v4.4.1", "models/policy_bc_v4.4.1", "Policy BC v4.4.1"),
    ("policy_bc_v4.4.2", "models/policy_bc_v4.4.2", "Policy BC v4.4.2"),
    ("policy_bc_v5", "models/policy_bc_v5", "Policy BC v5"),
    ("policy_bc_v6", "models/policy_bc_v6_full", "Policy BC v6"),
    ("policy_bc_v7", "models/policy_bc_v7_pilot_aligned", "Policy BC v7 pilot"),
    ("policy_bc_v8", "models/policy_bc_v8", "Policy BC v8"),
)


@dataclass(frozen=True)
class Decision:
    card: str
    tile_col: int
    tile_row: int
    event_type: str = "card_play"
    recovery_s: float = 0.6


def _checkpoint(directory: Path) -> Path | None:
    preferred = directory / "best_model.pt"
    if preferred.is_file():
        return preferred
    found = sorted(path for path in directory.glob("*.pt") if path.is_file())
    return found[0] if found else None


def list_agents(root: Path | None = None) -> list[dict]:
    root = Path(root) if root else repo_root()
    rows = [
        {
            "id": "random",
            "name": "Random legal",
            "loadable": True,
            "kind": "baseline",
            "detail": "Picks a legal card, tile, and time. No checkpoint required.",
            "path": None,
        }
    ]
    for agent_id, relative, label in POLICY_CANDIDATES:
        directory = root / relative
        if not directory.is_dir():
            continue
        checkpoint = _checkpoint(directory)
        rows.append(
            {
                "id": agent_id,
                "name": label,
                "loadable": checkpoint is not None,
                "kind": "policy_bc",
                "detail": (
                    f"Checkpoint {checkpoint.name}"
                    if checkpoint is not None
                    else (
                        f"No weights in {relative} (expected best_model.pt). "
                        "Checkpoints are gitignored; download the Hugging Face "
                        "snapshot into that directory, or use random."
                    )
                ),
                "path": relative,
            }
        )
    v9 = root / "models" / "policy_v9"
    if v9.is_dir():
        rows.append(
            {
                "id": "policy_v9",
                "name": "Policy v9",
                "loadable": False,
                "kind": "policy_v9",
                "detail": (
                    "v9 checkpoints use a different forward contract than "
                    "predict_next_action. They are listed so the gap is visible; "
                    "the headless player is random until a v9 adapter is added."
                ),
                "path": "models/policy_v9",
            }
        )
    return rows


class RandomAgent:
    id = "random"

    def __init__(self, seed: int):
        self._rng = random.Random(seed)

    def choose(self, observation: dict) -> Decision | None:
        elixir = float(observation["elixir"])
        costs: dict = observation["costs"]
        bounds: dict = observation["bounds"]
        playable = []
        for card in observation["hand"]:
            cost = costs.get(card)
            if cost is None:
                continue
            if elixir + 1e-6 >= float(cost):
                playable.append(card)
        if not playable:
            return None
        card = self._rng.choice(playable)
        lo, hi = bounds[card]
        return Decision(
            card=card,
            tile_col=self._rng.randint(2, 29),
            tile_row=self._rng.randint(int(lo), int(hi)),
            recovery_s=self._rng.uniform(0.35, 1.1),
        )


class PolicyAgent:
    """Behavior-cloning checkpoint. Legal tiles are clamped before submit."""

    def __init__(self, agent_id: str, directory: Path, device: str | None = None):
        self.id = agent_id
        checkpoint = _checkpoint(directory)
        if checkpoint is None:
            raise FileNotFoundError(f"no checkpoint in {directory}")
        from ..policy_infer import load_policy, rollout_decode_settings

        model, vocab, cfg, device_obj = load_policy(directory, device_name=device)
        self._model = model
        self._vocab = vocab
        self._cfg = cfg
        self._device = device_obj
        self._decode = rollout_decode_settings(cfg)
        self._predict = None

    def choose(self, observation: dict) -> Decision | None:
        from ..policy_infer import predict_next_action
        from ..winner_dataset import BattleExample

        table = {}
        for source in (
            observation.get("card_elixir") or {},
            observation.get("opponent_card_elixir") or {},
        ):
            for key, value in source.items():
                table[str(key)] = int(value)
        for key, value in observation["costs"].items():
            if value is not None:
                table[str(key)] = int(value)
        costs = table
        side = observation["side"]
        acting = "team" if side == "red" else "opponent"
        events = []
        for event in observation["events"]:
            events.append(
                {
                    "seconds": float(event["seconds"]),
                    "side": "team" if event["side"] == "red" else "opponent",
                    "event_type": event["event_type"],
                    "card": event["card"],
                    "x": int(event["x"]) if event.get("x") is not None else 9000,
                    "y": int(event["y"]) if event.get("y") is not None else 16000,
                }
            )
        if side == "red":
            team_deck = tuple(observation["deck"])
            opponent_deck = tuple(observation["opponent_deck"])
        else:
            team_deck = tuple(observation["opponent_deck"])
            opponent_deck = tuple(observation["deck"])
        battle = BattleExample(
            battle_id="local-match",
            team_deck=team_deck,
            opponent_deck=opponent_deck,
            team_wins=0,
            events=tuple(events),
        )
        try:
            prediction = predict_next_action(
                self._model,
                self._vocab,
                costs,
                battle,
                self._device,
                acting_side=acting,
                temperature=float(self._decode["temperature"]),
                slot_decode=str(self._decode["slot_decode"]),
                max_context=int(self._cfg.get("max_context", 64)),
                threat_dim=int(self._cfg.get("threat_dim", 0) or 0),
                min_context=0,
                prefer_cards=set(observation["hand"]),
                placement_decode=str(self._decode["placement_decode"]),
                placement_temperature=float(self._decode["placement_temperature"]),
                placement_top_k=self._decode["placement_top_k"],
                think_steps=int(self._decode["think_steps"]),
                now_seconds=float(observation["seconds"]),
                rng=random.Random(len(events) + (0 if side == "red" else 17)),
            )
        except (RuntimeError, ValueError):
            return None
        ranked = [row["card"] for row in prediction.get("ranked_slots") or []]
        chosen = prediction["card"]
        if chosen not in observation["hand"] or costs.get(chosen) is None:
            chosen = next(
                (
                    card
                    for card in ranked
                    if card in observation["hand"] and costs.get(card) is not None
                ),
                None,
            )
        if chosen is None:
            return None
        if float(observation["elixir"]) + 1e-6 < float(costs[chosen]):
            affordable = [
                card
                for card in ranked
                if card in observation["hand"]
                and costs.get(card) is not None
                and float(observation["elixir"]) + 1e-6 >= float(costs[card])
            ]
            if not affordable:
                return None
            chosen = affordable[0]
        col, row = tile_from_raw(prediction["x"], prediction["y"])
        # clamp_tile needs a Card. Bounds from the harness are authoritative.
        lo, hi = observation["bounds"].get(chosen, [0, 17])
        row = max(int(lo), min(int(hi), row))
        col = max(0, min(31, col))
        event_type = prediction.get("event_type") or "card_play"
        if event_type != "card_play":
            event_type = "card_play"
        return Decision(
            card=chosen,
            tile_col=col,
            tile_row=row,
            event_type=event_type,
            recovery_s=max(0.2, min(12.0, float(prediction.get("delay_seconds") or 0.6))),
        )


def build_agent(
    agent_id: str,
    *,
    seed: int,
    root: Path | None = None,
    device: str | None = None,
):
    if agent_id == "random":
        return RandomAgent(seed)
    root = Path(root) if root else repo_root()
    for candidate, relative, _label in POLICY_CANDIDATES:
        if candidate == agent_id:
            return PolicyAgent(agent_id, root / relative, device=device)
    known = ", ".join(row["id"] for row in list_agents(root) if row["loadable"])
    raise ValueError(f"unknown or unloadable agent {agent_id!r}; loadable: {known}")


def side_observation(snapshot: dict, side: str) -> dict:
    other = "blue" if side == "red" else "red"
    me = snapshot["sides"][side]
    them = snapshot["sides"][other]
    return {
        "side": side,
        "seconds": snapshot["seconds"],
        "elixir": me["elixir"],
        "hand": list(me["hand"]),
        "deck": list(me["deck"]),
        "opponent_deck": list(them["deck"]),
        "costs": dict(me["costs"]),
        "card_elixir": dict(me.get("card_elixir") or {}),
        "opponent_card_elixir": dict(them.get("card_elixir") or {}),
        "bounds": {key: list(value) for key, value in me["bounds"].items()},
        "events": list(snapshot["events"]),
        "phase": snapshot["phase"],
    }
