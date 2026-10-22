"""Resumable copy of the BC causal fold; arithmetic order is intentional.

Only the final global row is materialized. Event elixir is never topped up by
an observation clock: those quiet-interval gains belong to a temporary snapshot.
Keep arithmetic aligned with policy_dataset._encode_causal_stream.
"""
from __future__ import annotations

import numpy as np
import torch


class CausalState:
    def __init__(self, dataset, battle, vocab, costs, swap_sides):
        self.dataset, self.vocab, self.costs = dataset, vocab, costs
        self.swap_sides = swap_sides
        self.events = []
        self.event_values = []
        self.decks = (battle.team_deck, battle.opponent_deck)
        self.cost_values = dict(costs)
        self.vocab_values = dict(vocab.name_to_id)
        self.vocab_unk = vocab.unk_id
        self.last_play = [-1.0] * 8
        self.play_count = [0.0] * 8
        self.side_plays = 0
        self.team_elixir = 5.0
        self.opponent_elixir = 5.0
        self.team_spent = 0.0
        self.opponent_spent = 0.0
        self.team_plays = 0.0
        self.opponent_plays = 0.0
        self.team_deep = 0.0
        self.opponent_deep = 0.0
        self.team_left = 0.0
        self.opponent_left = 0.0
        self.team_bridge = 0.0
        self.opponent_bridge = 0.0
        self.team_y_sum = 0.0
        self.opponent_y_sum = 0.0
        self.team_abilities = 0.0
        self.opponent_abilities = 0.0
        self.team_spells = 0.0
        self.opponent_spells = 0.0
        self.team_wincons = 0.0
        self.opponent_wincons = 0.0
        self.team_double_spent = 0.0
        self.opponent_double_spent = 0.0
        self.team_triple_spent = 0.0
        self.opponent_triple_spent = 0.0
        self.team_unique: set[str] = set()
        self.opponent_unique: set[str] = set()
        self.team_cycle: list[str] = []
        self.opponent_cycle: list[str] = []
        self.team_spend_times: list[float] = []
        self.team_spend_vals: list[float] = []
        self.opp_spend_times: list[float] = []
        self.opp_spend_vals: list[float] = []
        self.team_deep_times: list[float] = []
        self.opp_deep_times: list[float] = []
        self.team_deep_streak = 0
        self.opp_deep_streak = 0
        self.team_max_deep_streak = 0
        self.opp_max_deep_streak = 0
        self.last_time = 0.0
        self.last_team_time = 0.0
        self.last_opp_time = 0.0
        self.streak = 0
        self.last_side: str | None = None
        self.continuous_rows: list[list[float]] = []
        self.card_id_rows: list[int] = []
        if self.swap_sides:
            self.team_deck_cards = battle.opponent_deck
            self.opp_deck_cards = battle.team_deck
        else:
            self.team_deck_cards = battle.team_deck
            self.opp_deck_cards = battle.opponent_deck
        self.team_avg, team_std, self.team_spell_frac = self.dataset._deck_stats(self.team_deck_cards, self.costs)
        self.opp_avg, opp_std, self.opp_spell_frac = self.dataset._deck_stats(self.opp_deck_cards, self.costs)
        self.shared = len(set(self.team_deck_cards) & set(self.opp_deck_cards)) / 8.0
        del team_std, opp_std
        self.team_deck = self.dataset._deck_ids(self.team_deck_cards, self.vocab)
        self.opponent_deck = self.dataset._deck_ids(self.opp_deck_cards, self.vocab)

    def feed(self, event):
        seconds = float(event['seconds'])
        gain = (seconds - self.last_time) / self.dataset._elixir_gain_rate(seconds)
        self.team_elixir = min(10.0, self.team_elixir + gain)
        self.opponent_elixir = min(10.0, self.opponent_elixir + gain)
        side = event['side']
        if self.swap_sides:
            side = 'opponent' if side == 'team' else 'team'
        card = event['card']
        cost = self.dataset._event_cost(card, event['event_type'], self.costs)
        position_known = bool(event.get('position_known', True))
        x = float(event['x']) / 18000.0 if position_known else 0.5
        y = float(event['y']) / 32000.0 if position_known else 0.5
        if self.swap_sides:
            y = 1.0 - y
        is_ability = 1.0 if event['event_type'] == 'ability_activation' else 0.0
        is_spell = 1.0 if card in self.dataset.SPELL_CARDS else 0.0
        is_wincon = 1.0 if card in self.dataset.WIN_CONDITIONISH else 0.0
        is_deep = 0.0
        is_bridge = 1.0 if position_known and 0.42 <= y <= 0.58 else 0.0
        is_left = 1.0 if position_known and x < 0.5 else 0.0
        is_double = 1.0 if seconds >= 120 else 0.0
        is_triple = 1.0 if seconds >= 240 else 0.0
        if side == 'team':
            self.team_elixir = max(0.0, self.team_elixir - cost)
            self.team_spent += cost
            self.team_plays += 1.0
            self.team_y_sum += y
            self.team_abilities += is_ability
            self.team_spells += is_spell
            self.team_wincons += is_wincon
            if is_double:
                self.team_double_spent += cost
            if is_triple:
                self.team_triple_spent += cost
            if card and is_ability < 0.5:
                self.team_unique.add(card)
                self.team_cycle.append(card)
                if len(self.team_cycle) > 8:
                    self.team_cycle = self.team_cycle[-8:]
            self.team_spend_times.append(seconds)
            self.team_spend_vals.append(cost)
            side_flag = 1.0
            if y > 0.55:
                self.team_deep += 1.0
                is_deep = 1.0
                self.team_deep_times.append(seconds)
                self.team_deep_streak += 1
                self.team_max_deep_streak = max(self.team_max_deep_streak, self.team_deep_streak)
            else:
                self.team_deep_streak = 0
            if is_left:
                self.team_left += 1.0
            if is_bridge:
                self.team_bridge += 1.0
            dt_same = min((seconds - self.last_team_time) / 30.0, 2.0) if self.last_team_time else 0.0
            self.last_team_time = seconds
            cycle_avg = float(np.mean([self.costs.get(c, self.dataset.DEFAULT_ELIXIR_COST) for c in self.team_cycle[-4:]])) / 10.0 if self.team_cycle else 0.4
        else:
            self.opponent_elixir = max(0.0, self.opponent_elixir - cost)
            self.opponent_spent += cost
            self.opponent_plays += 1.0
            self.opponent_y_sum += y
            self.opponent_abilities += is_ability
            self.opponent_spells += is_spell
            self.opponent_wincons += is_wincon
            if is_double:
                self.opponent_double_spent += cost
            if is_triple:
                self.opponent_triple_spent += cost
            if card and is_ability < 0.5:
                self.opponent_unique.add(card)
                self.opponent_cycle.append(card)
                if len(self.opponent_cycle) > 8:
                    self.opponent_cycle = self.opponent_cycle[-8:]
            self.opp_spend_times.append(seconds)
            self.opp_spend_vals.append(cost)
            side_flag = 0.0
            if y < 0.45:
                self.opponent_deep += 1.0
                is_deep = 1.0
                self.opp_deep_times.append(seconds)
                self.opp_deep_streak += 1
                self.opp_max_deep_streak = max(self.opp_max_deep_streak, self.opp_deep_streak)
            else:
                self.opp_deep_streak = 0
            if is_left:
                self.opponent_left += 1.0
            if is_bridge:
                self.opponent_bridge += 1.0
            dt_same = min((seconds - self.last_opp_time) / 30.0, 2.0) if self.last_opp_time else 0.0
            self.last_opp_time = seconds
            cycle_avg = float(np.mean([self.costs.get(c, self.dataset.DEFAULT_ELIXIR_COST) for c in self.opponent_cycle[-4:]])) / 10.0 if self.opponent_cycle else 0.4
        if side == self.last_side:
            self.streak += 1
        else:
            self.streak = 1
            self.last_side = side
        recent_team_30 = self.dataset._window_sum(self.team_spend_times, self.team_spend_vals, seconds, 30.0)
        recent_opp_30 = self.dataset._window_sum(self.opp_spend_times, self.opp_spend_vals, seconds, 30.0)
        play_scale = max(self.team_plays + self.opponent_plays, 1.0)
        self.continuous_rows.append(
            [
                min(seconds / 300.0, 1.5),
                side_flag,
                is_double,
                is_triple,
                x,
                y,
                self.team_elixir / 10.0,
                self.opponent_elixir / 10.0,
                (self.team_elixir - self.opponent_elixir) / 10.0,
                cost / 10.0,
                is_ability,
                is_spell,
                is_wincon,
                self.team_spent / 40.0,
                self.opponent_spent / 40.0,
                (self.team_spent - self.opponent_spent) / 40.0,
                (self.team_plays - self.opponent_plays) / play_scale,
                (self.team_deep - self.opponent_deep) / play_scale,
                is_deep,
                is_bridge,
                min(self.streak / 6.0, 1.0),
                dt_same,
                recent_team_30 / 20.0,
                recent_opp_30 / 20.0,
                (recent_team_30 - recent_opp_30) / 20.0,
                cycle_avg,
                self.team_elixir / 10.0 if side_flag > 0.5 else self.opponent_elixir / 10.0,
                0.0,
            ]
        )
        self.card_id_rows.append(self.vocab.encode(card))
        self.last_time = seconds
        # The hand heuristic counts card plays only, unlike the causal fold.
        raw_side = "opponent" if self.swap_sides else "team"
        if event["side"] == raw_side and event["event_type"] == "card_play":
            self.side_plays += 1
            slot = self.dataset.deck_slot_for_card(self.team_deck_cards, event["card"])
            if slot is not None:
                self.last_play[slot] = float(event["seconds"])
                self.play_count[slot] += 1.0
        self.events.append(event)
        self.event_values.append(dict(event))

    def global_row(self, observation_seconds):
        team_elixir = self.team_elixir
        opponent_elixir = self.opponent_elixir
        seconds = self.last_time
        play_scale = max(self.team_plays + self.opponent_plays, 1.0)
        team_mean_y = self.team_y_sum / max(self.team_plays, 1.0)
        opp_mean_y = self.opponent_y_sum / max(self.opponent_plays, 1.0)
        now = float(observation_seconds)
        if now < seconds:
            raise ValueError("Observation precedes the confirmed prefix")
        # Preserve the reference's sum order across both multiplier boundaries.
        gain = sum(max(0.0, min(now, end) - max(seconds, start)) / period
                   for start, end, period in ((0., 120., 2.8), (120., 240., 1.4),
                                              (240., float("inf"), .9)))
        team_elixir = min(10., team_elixir + gain)
        opponent_elixir = min(10., opponent_elixir + gain)
        team_15 = self.dataset._window_sum(self.team_spend_times, self.team_spend_vals, now, 15.0)
        opp_15 = self.dataset._window_sum(self.opp_spend_times, self.opp_spend_vals, now, 15.0)
        team_30 = self.dataset._window_sum(self.team_spend_times, self.team_spend_vals, now, 30.0)
        opp_30 = self.dataset._window_sum(self.opp_spend_times, self.opp_spend_vals, now, 30.0)
        team_60 = self.dataset._window_sum(self.team_spend_times, self.team_spend_vals, now, 60.0)
        opp_60 = self.dataset._window_sum(self.opp_spend_times, self.opp_spend_vals, now, 60.0)
        team_deep_30 = sum((1 for t in self.team_deep_times if t >= now - 30.0))
        opp_deep_30 = sum((1 for t in self.opp_deep_times if t >= now - 30.0))
        team_deep_60 = sum((1 for t in self.team_deep_times if t >= now - 60.0))
        opp_deep_60 = sum((1 for t in self.opp_deep_times if t >= now - 60.0))
        last_team_deep = self.team_deep_times[-1] if self.team_deep_times else -1.0
        last_opp_deep = self.opp_deep_times[-1] if self.opp_deep_times else -1.0
        if last_team_deep < 0 and last_opp_deep < 0:
            last_deep_side = 0.5
        elif last_team_deep >= last_opp_deep:
            last_deep_side = 1.0
        else:
            last_deep_side = 0.0
        team_cycle_avg = float(np.mean([self.costs.get(c, self.dataset.DEFAULT_ELIXIR_COST) for c in self.team_cycle[-4:]])) / 10.0 if self.team_cycle else 0.4
        opp_cycle_avg = float(np.mean([self.costs.get(c, self.dataset.DEFAULT_ELIXIR_COST) for c in self.opponent_cycle[-4:]])) / 10.0 if self.opponent_cycle else 0.4
        return torch.tensor(
            [
                self.team_spent / 40.0,
                self.opponent_spent / 40.0,
                (self.team_spent - self.opponent_spent) / 40.0,
                self.team_plays / 40.0,
                self.opponent_plays / 40.0,
                (self.team_plays - self.opponent_plays) / play_scale,
                self.team_deep / play_scale,
                self.opponent_deep / play_scale,
                (self.team_deep - self.opponent_deep) / play_scale,
                self.team_left / max(self.team_plays, 1.0),
                self.opponent_left / max(self.opponent_plays, 1.0),
                self.team_bridge / play_scale,
                self.opponent_bridge / play_scale,
                team_mean_y,
                opp_mean_y,
                team_mean_y - opp_mean_y,
                self.team_abilities / 8.0,
                self.opponent_abilities / 8.0,
                team_elixir / 10.0,
                opponent_elixir / 10.0,
                (team_elixir - opponent_elixir) / 10.0,
                min(now / 300.0, 1.5),
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
                (self.team_double_spent - self.opponent_double_spent) / 30.0,
                (self.team_triple_spent - self.opponent_triple_spent) / 20.0,
                (self.team_spells - self.opponent_spells) / 8.0,
                (self.team_wincons - self.opponent_wincons) / 8.0,
                team_cycle_avg,
                opp_cycle_avg,
                team_cycle_avg - opp_cycle_avg,
                len(self.team_unique) / 8.0,
                len(self.opponent_unique) / 8.0,
                min(self.team_max_deep_streak / 5.0, 1.0),
                min(self.opp_max_deep_streak / 5.0, 1.0),
                (self.team_avg - self.opp_avg) / 10.0,
                self.team_spell_frac - self.opp_spell_frac,
                self.shared,
            ], dtype=torch.float32,
        )

    def cycle_features(self, observation_seconds):
        acting_deck = self.team_deck_cards
        last_play, play_count = self.last_play, self.play_count
        now = observation_seconds
        waits = sorted(((-1.0 if last_play[slot] < 0 else last_play[slot], slot)
                        for slot in range(len(acting_deck))), key=lambda item: (item[0], item[1]))
        ranks = {slot: rank for rank, (_, slot) in enumerate(waits)}
        rows = [[0.0] * self.dataset.SLOT_FEAT_DIM for _ in range(8)]
        for slot, card in enumerate(acting_deck):
            cost = float(self.costs.get(card, self.dataset.DEFAULT_ELIXIR_COST)) / 10.0
            never = 1.0 if last_play[slot] < 0 else 0.0
            dt = 2.0 if never else min((now - last_play[slot]) / 60.0, 2.0)
            rows[slot] = [cost, min(play_count[slot] / 10.0, 1.5), dt, never,
                          ranks[slot] / 7.0, 1.0 if cost <= 0.3 else 0.0,
                          1.0 if card in self.dataset.SPELL_CARDS else 0.0,
                          1.0 if card in self.dataset.WIN_CONDITIONISH else 0.0]
        hand = [False] * 8
        if self.side_plays >= 4:
            for _, slot in waits[:4]:
                hand[slot] = True
        # Same float32 conversions as individual reference assignments, once
        # per matrix instead of dozens of scalar tensor writes.
        return torch.tensor(rows, dtype=torch.float32), torch.tensor(hand, dtype=torch.bool)
