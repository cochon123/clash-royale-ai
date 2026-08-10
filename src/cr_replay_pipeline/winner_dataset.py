from __future__ import annotations

import json
import os
import pickle
import random
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import numpy as np
import torch
from torch.nn.utils.rnn import pad_sequence
from torch.utils.data import DataLoader, Dataset

from .parser import ReplayParseError, parse_replay

CONTINUOUS_DIM = 28
GLOBAL_DIM = 48
DEFAULT_ELIXIR_COST = 4
ABILITY_ELIXIR_COST = 1

SPELL_CARDS = frozenset(
    {
        "arrows",
        "barbarian-barrel",
        "clone",
        "earthquake",
        "fireball",
        "freeze",
        "giant-snowball",
        "goblin-curse",
        "lightning",
        "mirror",
        "poison",
        "rage",
        "rocket",
        "royal-delivery",
        "snowball",
        "the-log",
        "tornado",
        "vines",
        "void",
        "zap",
    }
)

WIN_CONDITIONISH = frozenset(
    {
        "balloon",
        "battle-ram",
        "electro-giant",
        "elixir-golem",
        "goblin-barrel",
        "goblin-drill",
        "goblin-giant",
        "golem",
        "graveyard",
        "hog-rider",
        "lava-hound",
        "mega-knight",
        "miner",
        "mortar",
        "pekka",
        "ram-rider",
        "royal-giant",
        "royal-hogs",
        "sparky",
        "three-musketeers",
        "wall-breakers",
        "x-bow",
    }
)


@dataclass(frozen=True)
class BattleExample:
    battle_id: str
    team_deck: tuple[str, ...]
    opponent_deck: tuple[str, ...]
    team_wins: int
    events: tuple[dict[str, Any], ...]


class CardVocab:
    def __init__(self, names: list[str]):
        self.pad_id = 0
        self.unk_id = 1
        ordered = sorted(set(names))
        self.name_to_id = {name: index + 2 for index, name in enumerate(ordered)}
        self.id_to_name = {index: name for name, index in self.name_to_id.items()}
        self.vocab_size = len(self.name_to_id) + 2

    def encode(self, name: str | None) -> int:
        if not name:
            return self.unk_id
        return self.name_to_id.get(name, self.unk_id)

    def to_dict(self) -> dict[str, Any]:
        return {
            "pad_id": self.pad_id,
            "unk_id": self.unk_id,
            "name_to_id": self.name_to_id,
            "vocab_size": self.vocab_size,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CardVocab:
        vocab = cls([])
        vocab.pad_id = data["pad_id"]
        vocab.unk_id = data["unk_id"]
        vocab.name_to_id = dict(data["name_to_id"])
        vocab.id_to_name = {index: name for name, index in vocab.name_to_id.items()}
        vocab.vocab_size = data["vocab_size"]
        return vocab


def load_card_costs(path: str | Path | None) -> dict[str, int]:
    if path is None:
        return {}
    source = Path(path)
    if not source.exists():
        return {}
    with source.open() as handle:
        raw = json.load(handle)
    return {str(name): int(cost) for name, cost in raw.items()}


def _elixir_gain_rate(seconds: float) -> float:
    # Seconds per elixir: single / double / triple.
    if seconds >= 240:
        return 0.9
    if seconds >= 120:
        return 1.4
    return 2.8


def _event_cost(card: str, event_type: str, costs: dict[str, int]) -> float:
    if event_type == "ability_activation":
        return float(ABILITY_ELIXIR_COST)
    return float(costs.get(card, DEFAULT_ELIXIR_COST))


def _deck_stats(deck: tuple[str, ...], costs: dict[str, int]) -> tuple[float, float, float]:
    values = [float(costs.get(card, DEFAULT_ELIXIR_COST)) for card in deck]
    if not values:
        return 4.0, 0.0, 0.0
    mean = float(np.mean(values))
    std = float(np.std(values))
    spells = sum(1 for card in deck if card in SPELL_CARDS) / max(len(deck), 1)
    return mean, std, float(spells)


def collect_battles(
    input_dir: str | Path,
    min_card_plays: int = 12,
    cache_path: str | Path | None = None,
) -> list[BattleExample]:
    source = Path(input_dir)
    cache_file = (
        Path(cache_path)
        if cache_path is not None
        else source.parent / "winner_battles_cache.pkl"
    )

    file_count = sum(1 for _ in source.rglob("*.json"))
    if cache_file.exists():
        try:
            with cache_file.open("rb") as handle:
                cached = pickle.load(handle)
            if (
                isinstance(cached, dict)
                and (os.environ.get("CR_REPLAY_TRUST_CACHE") == "1" or cached.get("file_count") == file_count)
                and cached.get("min_card_plays") == min_card_plays
                and isinstance(cached.get("battles"), list)
            ):
                print(
                    f"Loaded {len(cached['battles'])} battles from cache "
                    f"({cache_file})",
                    flush=True,
                )
                return cached["battles"]
        except Exception:
            pass

    battles: list[BattleExample] = []
    seen_ids: set[str] = set()
    seen_hashes: set[str] = set()

    for path in sorted(source.rglob("*.json")):
        try:
            replay = parse_replay(path)
        except (ReplayParseError, OSError, ValueError, TypeError):
            continue
        if replay.battle_id in seen_ids or replay.content_hash in seen_hashes:
            continue
        seen_ids.add(replay.battle_id)
        seen_hashes.add(replay.content_hash)

        team_crowns = replay.crowns.get("team")
        opponent_crowns = replay.crowns.get("opponent")
        if team_crowns is None or opponent_crowns is None:
            continue
        if team_crowns == opponent_crowns:
            continue
        team_deck = tuple(replay.decks.get("team") or [])
        opponent_deck = tuple(replay.decks.get("opponent") or [])
        if len(team_deck) != 8 or len(opponent_deck) != 8:
            continue

        events = []
        for event in replay.events:
            if event.event_type == "card_play":
                if event.card is None or event.x is None or event.y is None:
                    continue
                events.append(
                    {
                        "seconds": float(event.seconds),
                        "side": event.side,
                        "event_type": "card_play",
                        "card": event.card,
                        "x": int(event.x),
                        "y": int(event.y),
                    }
                )
            elif event.event_type == "ability_activation":
                events.append(
                    {
                        "seconds": float(event.seconds),
                        "side": event.side,
                        "event_type": "ability_activation",
                        "card": event.ability_card or event.card or "_ability",
                        "x": 9000,
                        "y": 16000,
                    }
                )
        card_plays = sum(1 for event in events if event["event_type"] == "card_play")
        if card_plays < min_card_plays:
            continue
        events.sort(key=lambda item: (item["seconds"], item["side"], item["card"]))
        battles.append(
            BattleExample(
                battle_id=replay.battle_id,
                team_deck=team_deck,
                opponent_deck=opponent_deck,
                team_wins=1 if team_crowns > opponent_crowns else 0,
                events=tuple(events),
            )
        )

    try:
        with cache_file.open("wb") as handle:
            pickle.dump(
                {
                    "file_count": file_count,
                    "min_card_plays": min_card_plays,
                    "battles": battles,
                },
                handle,
            )
        print(f"Cached {len(battles)} battles -> {cache_file}", flush=True)
    except OSError:
        pass
    return battles


def split_battles(
    battles: list[BattleExample],
    val_fraction: float = 0.15,
    test_fraction: float = 0.15,
    seed: int = 42,
) -> tuple[list[BattleExample], list[BattleExample], list[BattleExample]]:
    ordered = list(battles)
    rng = random.Random(seed)
    rng.shuffle(ordered)
    n_test = int(len(ordered) * test_fraction)
    n_val = int(len(ordered) * val_fraction)
    test = ordered[:n_test]
    val = ordered[n_test : n_test + n_val]
    train = ordered[n_test + n_val :]
    return train, val, test


def build_vocab(battles: list[BattleExample]) -> CardVocab:
    names: list[str] = []
    for battle in battles:
        names.extend(battle.team_deck)
        names.extend(battle.opponent_deck)
        for event in battle.events:
            if event["card"]:
                names.append(event["card"])
    return CardVocab(names)


def _deck_ids(deck: tuple[str, ...], vocab: CardVocab) -> torch.Tensor:
    ids = [vocab.encode(card) for card in deck]
    while len(ids) < 8:
        ids.append(vocab.pad_id)
    return torch.tensor(ids[:8], dtype=torch.long)


def _window_sum(times: list[float], values: list[float], now: float, window: float) -> float:
    total = 0.0
    cutoff = now - window
    for time, value in zip(reversed(times), reversed(values)):
        if time < cutoff:
            break
        total += value
    return total


def _encode_prefix(
    battle: BattleExample,
    end_index: int,
    vocab: CardVocab,
    costs: dict[str, int],
    swap_sides: bool = False,
) -> tuple[torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor, int] | None:
    prefix = battle.events[:end_index]
    if len(prefix) < 8:
        return None

    team_elixir = 5.0
    opponent_elixir = 5.0
    team_spent = 0.0
    opponent_spent = 0.0
    team_plays = 0.0
    opponent_plays = 0.0
    team_deep = 0.0
    opponent_deep = 0.0
    team_left = 0.0
    opponent_left = 0.0
    team_bridge = 0.0
    opponent_bridge = 0.0
    team_y_sum = 0.0
    opponent_y_sum = 0.0
    team_abilities = 0.0
    opponent_abilities = 0.0
    team_spells = 0.0
    opponent_spells = 0.0
    team_wincons = 0.0
    opponent_wincons = 0.0
    team_double_spent = 0.0
    opponent_double_spent = 0.0
    team_triple_spent = 0.0
    opponent_triple_spent = 0.0
    team_unique: set[str] = set()
    opponent_unique: set[str] = set()
    team_cycle: list[str] = []
    opponent_cycle: list[str] = []
    team_spend_times: list[float] = []
    team_spend_vals: list[float] = []
    opp_spend_times: list[float] = []
    opp_spend_vals: list[float] = []
    team_deep_times: list[float] = []
    opp_deep_times: list[float] = []
    team_deep_streak = 0
    opp_deep_streak = 0
    team_max_deep_streak = 0
    opp_max_deep_streak = 0
    last_time = 0.0
    last_team_time = 0.0
    last_opp_time = 0.0
    streak = 0
    last_side: str | None = None
    continuous = []
    card_ids = []
    total_seconds = float(prefix[-1]["seconds"]) if prefix else 0.0

    for event in prefix:
        seconds = float(event["seconds"])
        gain = (seconds - last_time) / _elixir_gain_rate(seconds)
        team_elixir = min(10.0, team_elixir + gain)
        opponent_elixir = min(10.0, opponent_elixir + gain)

        side = event["side"]
        if swap_sides:
            side = "opponent" if side == "team" else "team"

        card = event["card"]
        cost = _event_cost(card, event["event_type"], costs)
        x = float(event["x"]) / 18000.0
        y = float(event["y"]) / 32000.0
        if swap_sides:
            y = 1.0 - y

        is_ability = 1.0 if event["event_type"] == "ability_activation" else 0.0
        is_spell = 1.0 if card in SPELL_CARDS else 0.0
        is_wincon = 1.0 if card in WIN_CONDITIONISH else 0.0
        is_deep = 0.0
        is_bridge = 1.0 if 0.42 <= y <= 0.58 else 0.0
        is_left = 1.0 if x < 0.5 else 0.0
        is_double = 1.0 if seconds >= 120 else 0.0
        is_triple = 1.0 if seconds >= 240 else 0.0

        if side == "team":
            team_elixir = max(0.0, team_elixir - cost)
            team_spent += cost
            team_plays += 1.0
            team_y_sum += y
            team_abilities += is_ability
            team_spells += is_spell
            team_wincons += is_wincon
            if is_double:
                team_double_spent += cost
            if is_triple:
                team_triple_spent += cost
            if card and is_ability < 0.5:
                team_unique.add(card)
                team_cycle.append(card)
                if len(team_cycle) > 8:
                    team_cycle = team_cycle[-8:]
            team_spend_times.append(seconds)
            team_spend_vals.append(cost)
            side_flag = 1.0
            if y > 0.55:
                team_deep += 1.0
                is_deep = 1.0
                team_deep_times.append(seconds)
                team_deep_streak += 1
                team_max_deep_streak = max(team_max_deep_streak, team_deep_streak)
            else:
                team_deep_streak = 0
            if is_left:
                team_left += 1.0
            if is_bridge:
                team_bridge += 1.0
            dt_same = min((seconds - last_team_time) / 30.0, 2.0) if last_team_time else 0.0
            last_team_time = seconds
            cycle_avg = (
                float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in team_cycle[-4:]]))
                / 10.0
                if team_cycle
                else 0.4
            )
        else:
            opponent_elixir = max(0.0, opponent_elixir - cost)
            opponent_spent += cost
            opponent_plays += 1.0
            opponent_y_sum += y
            opponent_abilities += is_ability
            opponent_spells += is_spell
            opponent_wincons += is_wincon
            if is_double:
                opponent_double_spent += cost
            if is_triple:
                opponent_triple_spent += cost
            if card and is_ability < 0.5:
                opponent_unique.add(card)
                opponent_cycle.append(card)
                if len(opponent_cycle) > 8:
                    opponent_cycle = opponent_cycle[-8:]
            opp_spend_times.append(seconds)
            opp_spend_vals.append(cost)
            side_flag = 0.0
            if y < 0.45:
                opponent_deep += 1.0
                is_deep = 1.0
                opp_deep_times.append(seconds)
                opp_deep_streak += 1
                opp_max_deep_streak = max(opp_max_deep_streak, opp_deep_streak)
            else:
                opp_deep_streak = 0
            if is_left:
                opponent_left += 1.0
            if is_bridge:
                opponent_bridge += 1.0
            dt_same = min((seconds - last_opp_time) / 30.0, 2.0) if last_opp_time else 0.0
            last_opp_time = seconds
            cycle_avg = (
                float(
                    np.mean(
                        [costs.get(c, DEFAULT_ELIXIR_COST) for c in opponent_cycle[-4:]]
                    )
                )
                / 10.0
                if opponent_cycle
                else 0.4
            )

        if side == last_side:
            streak += 1
        else:
            streak = 1
            last_side = side

        recent_team_30 = _window_sum(team_spend_times, team_spend_vals, seconds, 30.0)
        recent_opp_30 = _window_sum(opp_spend_times, opp_spend_vals, seconds, 30.0)
        play_scale = max(team_plays + opponent_plays, 1.0)
        continuous.append(
            [
                min(seconds / 300.0, 1.5),
                side_flag,
                is_double,
                is_triple,
                x,
                y,
                team_elixir / 10.0,
                opponent_elixir / 10.0,
                (team_elixir - opponent_elixir) / 10.0,
                cost / 10.0,
                is_ability,
                is_spell,
                is_wincon,
                team_spent / 40.0,
                opponent_spent / 40.0,
                (team_spent - opponent_spent) / 40.0,
                (team_plays - opponent_plays) / play_scale,
                (team_deep - opponent_deep) / play_scale,
                is_deep,
                is_bridge,
                min(streak / 6.0, 1.0),
                dt_same,
                recent_team_30 / 20.0,
                recent_opp_30 / 20.0,
                (recent_team_30 - recent_opp_30) / 20.0,
                cycle_avg,
                team_elixir / 10.0 if side_flag > 0.5 else opponent_elixir / 10.0,
                min((total_seconds - seconds) / 60.0, 2.0) if total_seconds else 0.0,
            ]
        )
        card_ids.append(vocab.encode(card))
        last_time = seconds

    play_scale = max(team_plays + opponent_plays, 1.0)
    team_mean_y = team_y_sum / max(team_plays, 1.0)
    opp_mean_y = opponent_y_sum / max(opponent_plays, 1.0)
    now = total_seconds
    team_15 = _window_sum(team_spend_times, team_spend_vals, now, 15.0)
    opp_15 = _window_sum(opp_spend_times, opp_spend_vals, now, 15.0)
    team_30 = _window_sum(team_spend_times, team_spend_vals, now, 30.0)
    opp_30 = _window_sum(opp_spend_times, opp_spend_vals, now, 30.0)
    team_60 = _window_sum(team_spend_times, team_spend_vals, now, 60.0)
    opp_60 = _window_sum(opp_spend_times, opp_spend_vals, now, 60.0)
    team_deep_30 = sum(1 for t in team_deep_times if t >= now - 30.0)
    opp_deep_30 = sum(1 for t in opp_deep_times if t >= now - 30.0)
    team_deep_60 = sum(1 for t in team_deep_times if t >= now - 60.0)
    opp_deep_60 = sum(1 for t in opp_deep_times if t >= now - 60.0)
    last_team_deep = team_deep_times[-1] if team_deep_times else -1.0
    last_opp_deep = opp_deep_times[-1] if opp_deep_times else -1.0
    if last_team_deep < 0 and last_opp_deep < 0:
        last_deep_side = 0.5
    elif last_team_deep >= last_opp_deep:
        last_deep_side = 1.0
    else:
        last_deep_side = 0.0
    team_cycle_avg = (
        float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in team_cycle[-4:]])) / 10.0
        if team_cycle
        else 0.4
    )
    opp_cycle_avg = (
        float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in opponent_cycle[-4:]]))
        / 10.0
        if opponent_cycle
        else 0.4
    )

    if swap_sides:
        team_deck_cards = battle.opponent_deck
        opp_deck_cards = battle.team_deck
    else:
        team_deck_cards = battle.team_deck
        opp_deck_cards = battle.opponent_deck
    team_avg, team_std, team_spell_frac = _deck_stats(team_deck_cards, costs)
    opp_avg, opp_std, opp_spell_frac = _deck_stats(opp_deck_cards, costs)
    shared = len(set(team_deck_cards) & set(opp_deck_cards)) / 8.0

    global_features = torch.tensor(
        [
            team_spent / 40.0,
            opponent_spent / 40.0,
            (team_spent - opponent_spent) / 40.0,
            team_plays / 40.0,
            opponent_plays / 40.0,
            (team_plays - opponent_plays) / play_scale,
            team_deep / play_scale,
            opponent_deep / play_scale,
            (team_deep - opponent_deep) / play_scale,
            team_left / max(team_plays, 1.0),
            opponent_left / max(opponent_plays, 1.0),
            team_bridge / play_scale,
            opponent_bridge / play_scale,
            team_mean_y,
            opp_mean_y,
            team_mean_y - opp_mean_y,
            team_abilities / 8.0,
            opponent_abilities / 8.0,
            team_elixir / 10.0,
            opponent_elixir / 10.0,
            (team_elixir - opponent_elixir) / 10.0,
            min(total_seconds / 300.0, 1.5),
            # Late tempo windows (strongest outcome signal).
            (team_15 - opp_15) / 15.0,
            (team_30 - opp_30) / 20.0,
            (team_60 - opp_60) / 30.0,
            team_15 / 15.0,
            opp_15 / 15.0,
            team_30 / 20.0,
            opp_30 / 20.0,
            (team_deep_30 - opp_deep_30) / 4.0,
            (team_deep_60 - opp_deep_60) / 6.0,
            last_deep_side,
            min((now - last_team_deep) / 60.0, 2.0) if last_team_deep >= 0 else 2.0,
            min((now - last_opp_deep) / 60.0, 2.0) if last_opp_deep >= 0 else 2.0,
            (team_double_spent - opponent_double_spent) / 30.0,
            (team_triple_spent - opponent_triple_spent) / 20.0,
            (team_spells - opponent_spells) / 8.0,
            (team_wincons - opponent_wincons) / 8.0,
            team_cycle_avg,
            opp_cycle_avg,
            team_cycle_avg - opp_cycle_avg,
            len(team_unique) / 8.0,
            len(opponent_unique) / 8.0,
            min(team_max_deep_streak / 5.0, 1.0),
            min(opp_max_deep_streak / 5.0, 1.0),
            (team_avg - opp_avg) / 10.0,
            (team_spell_frac - opp_spell_frac),
            shared,
        ],
        dtype=torch.float32,
    )

    label = battle.team_wins
    if swap_sides:
        label = 1 - label
        team_deck = _deck_ids(battle.opponent_deck, vocab)
        opponent_deck = _deck_ids(battle.team_deck, vocab)
    else:
        team_deck = _deck_ids(battle.team_deck, vocab)
        opponent_deck = _deck_ids(battle.opponent_deck, vocab)
    return (
        torch.tensor(continuous, dtype=torch.float32),
        torch.tensor(card_ids, dtype=torch.long),
        team_deck,
        opponent_deck,
        global_features,
        label,
    )


class WinnerSequenceDataset(Dataset):
    def __init__(
        self,
        battles: list[BattleExample],
        vocab: CardVocab,
        costs: dict[str, int],
        sample_ratios: list[float] | None = None,
        augment_swap: bool = False,
        max_seq_length: int = 180,
        seed: int = 42,
    ):
        self.sequences: list[torch.Tensor] = []
        self.card_ids: list[torch.Tensor] = []
        self.team_decks: list[torch.Tensor] = []
        self.opponent_decks: list[torch.Tensor] = []
        self.globals: list[torch.Tensor] = []
        self.labels: list[int] = []
        self.lengths: list[int] = []
        # Emphasize late prefixes where tempo/spent signal is strongest.
        ratios = sample_ratios or [0.85, 0.95, 1.0]
        rng = random.Random(seed)

        for battle in battles:
            n_events = len(battle.events)
            for ratio in ratios:
                end_index = max(8, min(max_seq_length, int(n_events * ratio)))
                encoded = _encode_prefix(
                    battle, end_index, vocab, costs, swap_sides=False
                )
                if encoded is None:
                    continue
                cont, cards, team_deck, opponent_deck, global_feat, label = encoded
                self.sequences.append(cont)
                self.card_ids.append(cards)
                self.team_decks.append(team_deck)
                self.opponent_decks.append(opponent_deck)
                self.globals.append(global_feat)
                self.labels.append(label)
                self.lengths.append(cont.size(0))
                if augment_swap and rng.random() < 0.7:
                    swapped = _encode_prefix(
                        battle, end_index, vocab, costs, swap_sides=True
                    )
                    if swapped is not None:
                        cont_s, cards_s, team_s, opp_s, global_s, label_s = swapped
                        self.sequences.append(cont_s)
                        self.card_ids.append(cards_s)
                        self.team_decks.append(team_s)
                        self.opponent_decks.append(opp_s)
                        self.globals.append(global_s)
                        self.labels.append(label_s)
                        self.lengths.append(cont_s.size(0))

    def __len__(self) -> int:
        return len(self.labels)

    def __getitem__(self, index: int):
        return (
            self.sequences[index],
            self.card_ids[index],
            self.team_decks[index],
            self.opponent_decks[index],
            self.globals[index],
            self.labels[index],
            self.lengths[index],
        )


def collate_winner_batch(batch):
    (
        sequences,
        card_ids,
        team_decks,
        opponent_decks,
        globals_,
        labels,
        lengths,
    ) = zip(*batch)
    return (
        pad_sequence(sequences, batch_first=True, padding_value=0.0),
        pad_sequence(card_ids, batch_first=True, padding_value=0),
        torch.stack(team_decks),
        torch.stack(opponent_decks),
        torch.stack(globals_),
        torch.tensor(labels, dtype=torch.long),
        torch.tensor(lengths, dtype=torch.long),
    )


def create_dataloaders(
    train_battles: list[BattleExample],
    val_battles: list[BattleExample],
    test_battles: list[BattleExample],
    vocab: CardVocab,
    costs: dict[str, int],
    batch_size: int = 64,
    sample_ratios: list[float] | None = None,
) -> tuple[DataLoader, DataLoader, DataLoader]:
    train_ds = WinnerSequenceDataset(
        train_battles,
        vocab,
        costs,
        sample_ratios=sample_ratios,
        augment_swap=True,
    )
    val_ds = WinnerSequenceDataset(
        val_battles,
        vocab,
        costs,
        sample_ratios=sample_ratios,
        augment_swap=False,
    )
    test_ds = WinnerSequenceDataset(
        test_battles,
        vocab,
        costs,
        sample_ratios=sample_ratios,
        augment_swap=False,
    )
    return (
        DataLoader(
            train_ds,
            batch_size=batch_size,
            shuffle=True,
            collate_fn=collate_winner_batch,
            num_workers=0,
        ),
        DataLoader(
            val_ds,
            batch_size=batch_size,
            shuffle=False,
            collate_fn=collate_winner_batch,
            num_workers=0,
        ),
        DataLoader(
            test_ds,
            batch_size=batch_size,
            shuffle=False,
            collate_fn=collate_winner_batch,
            num_workers=0,
        ),
    )


def summarize_split(name: str, battles: list[BattleExample]) -> dict[str, Any]:
    wins = sum(battle.team_wins for battle in battles)
    return {
        "split": name,
        "battles": len(battles),
        "team_win_rate": float(wins / len(battles)) if battles else 0.0,
        "mean_events": float(np.mean([len(b.events) for b in battles])) if battles else 0.0,
    }
