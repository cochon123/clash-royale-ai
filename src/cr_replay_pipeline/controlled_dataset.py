"""Shared action indices and a separate clock loader that is not reaction-biased."""

from collections import OrderedDict
import math
import torch
from torch.utils.data import Dataset
from .causal_observation import prefix_observation
from .policy_dataset import recent_opponent_threat
from .policy_model import xy_to_zone


def tensor_row(encoded, slot=0, kind=0, xy=(0.5, 0.5), delay=0.0, weight=1.0):
    c, ids, td, od, g, sf, hm = encoded
    return (
        c,
        ids,
        td,
        od,
        g,
        sf,
        hm,
        torch.tensor(slot),
        torch.tensor(kind),
        torch.tensor(xy_to_zone(*xy)),
        torch.tensor(xy),
        torch.tensor(math.log1p(delay)),
        len(c),
        torch.tensor(float(weight)),
    )


class ActionExamples(Dataset):
    """Card-play targets; ability events remain in prefixes, never fake XY labels.

    Training may oversample reaction windows. Validation should pass
    ``reaction_repeats=1`` and ``reaction_weight=1`` so metrics are not
    reaction-biased. Clock intervals never use this index.
    """

    def __init__(
        self,
        battles,
        vocab,
        costs,
        max_samples=40,
        reaction_repeats=1,
        reaction_weight=1.0,
        reaction_seconds=5.0,
    ):
        self.battles, self.vocab, self.costs = battles, vocab, costs
        self.unique_index = []
        self.index = []
        self.cache = OrderedDict()
        repeats = max(1, int(reaction_repeats))
        weight_react = float(reaction_weight)
        for bi, b in enumerate(battles):
            candidates = [
                i
                for i, e in enumerate(b.events)
                if i >= 8
                and e["event_type"] == "card_play"
                and e["card"]
                in (b.team_deck if e["side"] == "team" else b.opponent_deck)
            ]
            # Deterministic uniform coverage, identical for every variant/seed.
            if max_samples > 0 and len(candidates) > max_samples:
                candidates = [
                    candidates[j * len(candidates) // max_samples]
                    for j in range(max_samples)
                ]
            for i in candidates:
                self.unique_index.append((bi, i))
                event = b.events[i]
                _, is_react = recent_opponent_threat(
                    b, i, event["side"], costs, max_age=reaction_seconds
                )
                copies = repeats if is_react else 1
                weight = weight_react if is_react else 1.0
                for _ in range(copies):
                    self.index.append((bi, i, weight))

    def __len__(self):
        return len(self.index)

    def __getitem__(self, index):
        if index in self.cache:
            self.cache.move_to_end(index)
            return self.cache[index]
        bi, i, weight = self.index[index]
        b = self.battles[bi]
        e = b.events[i]
        # Events tied with the target are not available as an ordered prefix.
        prefix_end = i
        while prefix_end and b.events[prefix_end - 1]["seconds"] == e["seconds"]:
            prefix_end -= 1
        obs = prefix_observation(b, prefix_end, e["side"])
        deck = b.team_deck if e["side"] == "team" else b.opponent_deck
        x, y = e["x"] / 18000, e["y"] / 32000
        if e["side"] == "opponent":
            y = 1 - y
        row = tensor_row(
            obs.encode(self.vocab, self.costs),
            deck.index(e["card"]),
            0,
            (x, y),
            float(e["seconds"]) - obs.now,
            weight,
        )
        self.cache[index] = row
        if len(self.cache) > 2048:
            self.cache.popitem(last=False)
        return row


class ClockExamples(Dataset):
    """One next-event interval from a fixed team perspective, no reaction repeats.

    Multi-actor timestamp ties have a marginalized cause (-1); no artificial
    actor ordering is invented. Optional recording endpoints add
    right-censored quiet tails; never infer endpoints from the last action.
    """

    def __init__(self, battles, vocab, costs, recording_ends=None):
        self.battles, self.vocab, self.costs = battles, vocab, costs
        self.index = []
        for bi, b in enumerate(battles):
            start = 0
            previous = 0.0
            while start < len(b.events):
                end = start + 1
                t = float(b.events[start]["seconds"])
                while end < len(b.events) and b.events[end]["seconds"] == t:
                    end += 1
                sides = {e["side"] for e in b.events[start:end]}
                # Skip zero-duration kickoff events, retain them in next prefix.
                if t > previous:
                    self.index.append(
                        (
                            bi,
                            start,
                            previous,
                            t - previous,
                            int("opponent" in sides) if len(sides) == 1 else -1,
                            True,
                        )
                    )
                previous = t
                start = end
            terminal = (recording_ends or {}).get(b.battle_id)
            if terminal is not None:
                if terminal < previous:
                    raise ValueError("Recording ends before last event")
                if terminal > previous:
                    self.index.append(
                        (bi, start, previous, terminal - previous, 0, False)
                    )

    def __len__(self):
        return len(self.index)

    def __getitem__(self, index):
        bi, i, now, delay, side, observed = self.index[index]
        obs = prefix_observation(self.battles[bi], i, "team", now)
        return (
            tensor_row(obs.encode(self.vocab, self.costs), delay=delay),
            torch.tensor(side),
            torch.tensor(observed),
        )


def collate_clock(rows):
    from .policy_dataset import collate_policy_batch

    return (
        collate_policy_batch([r[0] for r in rows]),
        torch.stack([r[1] for r in rows]),
        torch.stack([r[2] for r in rows]),
    )
