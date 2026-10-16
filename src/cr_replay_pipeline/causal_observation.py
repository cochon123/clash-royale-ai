"""Explicit observation contract for new controlled experiments.

No action label is accepted by this API. Unknown opponent deck slots are
masked before feature extraction, including aggregate deck statistics.
"""

from dataclasses import dataclass, replace
import math
import torch
from .winner_dataset import BattleExample, CardVocab
from .feature_schema import CONTINUOUS_INDEX, GLOBAL_INDEX, CONTINUOUS_DIM, GLOBAL_DIM
from .policy_dataset import (
    _encode_causal_stream,
    acting_cycle_features,
    recent_opponent_threat,
)

OBSERVATION_VERSION = "causal-observation-1"
POSITION_UNKNOWN_INDEX = CONTINUOUS_INDEX[
    "pad"
]  # Repurposed only in this versioned contract.


@dataclass(frozen=True)
class PolicyObservation:
    battle: BattleExample  # Contains confirmed prefix events only.
    agent_side: str
    now: float

    def __post_init__(self):
        if self.agent_side not in {"team", "opponent"}:
            raise ValueError("Invalid perspective")
        if not math.isfinite(self.now) or self.now < 0:
            raise ValueError("Invalid observation time")
        times = [float(e["seconds"]) for e in self.battle.events]
        if times != sorted(times) or any(
            not math.isfinite(t) or t < 0 or t > self.now for t in times
        ):
            raise ValueError("Prefix is unordered or contains future events")

    def encode(self, vocab: CardVocab, costs: dict, max_context: int = 64):
        b = self.battle
        enemy = "opponent" if self.agent_side == "team" else "team"
        seen = {e["card"] for e in b.events if e["side"] == enemy}
        field = "opponent_deck" if enemy == "opponent" else "team_deck"
        b = replace(
            b,
            **{field: tuple(c if c in seen else "_unknown" for c in getattr(b, field))},
        )
        swap = self.agent_side == "opponent"
        stream = _encode_causal_stream(
            b, vocab, costs, swap_sides=swap, min_events=1, observation_seconds=self.now
        )
        if stream is None:
            # No fabricated event in the observation; a masked padding token is
            # only a tensor representation of the empty sequence.
            c = torch.zeros(1, CONTINUOUS_DIM)
            c[:, POSITION_UNKNOWN_INDEX] = 1
            ids = torch.zeros(1, dtype=torch.long)
            td = torch.tensor(
                [vocab.encode(x) for x in (b.opponent_deck if swap else b.team_deck)]
            )
            od = torch.tensor(
                [vocab.encode(x) for x in (b.team_deck if swap else b.opponent_deck)]
            )
            g = torch.zeros(GLOBAL_DIM)
            g[GLOBAL_INDEX["now"]] = min(self.now / 300, 1.5)
            gain = sum(
                max(0.0, min(self.now, end) - start) / period
                for start, end, period in (
                    (0.0, 120.0, 2.8),
                    (120.0, 240.0, 1.4),
                    (240.0, float("inf"), 0.9),
                )
            )
            g[GLOBAL_INDEX["team_elixir"]] = g[GLOBAL_INDEX["opp_elixir"]] = (
                min(10.0, 5.0 + gain) / 10
            )
        else:
            c, ids, td, od, globals_all = stream
            g = globals_all[-1]
            c, ids = c[-max_context:], ids[-max_context:]
            c[:, CONTINUOUS_INDEX["seconds"]] = torch.tensor(
                [float(e["seconds"]) / 300 for e in b.events[-max_context:]]
            )
            c[:, POSITION_UNKNOWN_INDEX] = torch.tensor(
                [not e.get("position_known", True) for e in b.events[-max_context:]]
            )
        g[GLOBAL_INDEX["now"]] = self.now / 300
        threat, _ = recent_opponent_threat(
            b, len(b.events), self.agent_side, costs, observation_seconds=self.now
        )
        sf, hm = acting_cycle_features(
            b, len(b.events), costs, swap, observation_seconds=self.now
        )
        return c, ids, td, od, torch.cat([g, threat]), sf, hm


def prefix_observation(battle, event_index, agent_side, now=None):
    if not 0 <= event_index <= len(battle.events):
        raise ValueError("Invalid prefix boundary")
    prefix = tuple(dict(e) for e in battle.events[:event_index])
    if now is None:
        now = float(prefix[-1]["seconds"]) if prefix else 0.0
    return PolicyObservation(replace(battle, events=prefix), agent_side, float(now))
