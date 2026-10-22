"""Human input queue and native validation for the local playable viewer."""
from __future__ import annotations

import csv
from dataclasses import dataclass
import json

from .banana_policy import _native_card

# Native arena units: side 0 owns tile rows 0-14 while the enemy Princess Tower
# in that lane stands; drops past this edge are moved back by the engine.
OWN_TERRITORY_END_Y = 15000


def own_side_only_cards(csv_dir):
    """Card names whose original ``CanDeployOnEnemySide`` flag is unset."""
    names = set()
    for table in ('spells_characters', 'spells_buildings', 'spells_other'):
        path = csv_dir / f'{table}.csv'
        if not path.exists():
            continue
        with path.open(newline='') as stream:
            rows = csv.reader(stream)
            column = next(rows).index('CanDeployOnEnemySide')
            next(rows, None)  # type row
            names.update(row[0] for row in rows if row and row[0] and row[column].upper() != 'TRUE')
    return names


class InputQueue:
    """Read complete append-only commands, retaining an unfinished last line."""
    def __init__(self, path):
        path.write_text('')
        self.stream = path.open()
        self.partial = ''

    def poll(self):
        *lines, self.partial = (self.partial + self.stream.read()).split('\n')
        result = []
        for line in lines:
            try:
                value = json.loads(line)
                if isinstance(value, dict):
                    result.append(value)
            except ValueError:
                pass
        return result

    def close(self):
        self.stream.close()


@dataclass
class HumanAction:
    slot: int
    card: str
    x: int
    y: int
    card_id: int
    input_serial: int
    side: int = 0
    wait: bool = False
    snapped: bool = False

    def native_request(self):
        return dict(side=self.side, slot=self.slot, x=self.x, y=self.y)


class HumanController:
    def __init__(self, engine, deck, card_name=None, own_side_only=()):
        """``card_name`` maps a native hand row to the deck's policy vocabulary;
        pass the AI scheduler's mapper so both sides record cards identically.
        ``own_side_only`` holds native names that cannot land on the enemy side."""
        self.engine, self.deck = engine, deck
        self.own_side_only = set(own_side_only)
        self.card_name = card_name or (lambda row: _native_card(row['name'], deck))
        self.reset(0)

    def reset(self, match_id):
        self.match_id = match_id
        self.last_serial = 0
        self.selection = self.target = None
        self.feedback = 'Select a card with 1-4 or click your hand'
        self.feedback_until = None

    def message(self, text, tick):
        self.feedback = text
        self.feedback_until = tick + 60

    @staticmethod
    def card(obs, slot, card_id):
        return next((c for c in obs['hands'][0] if c.get('available', True)
                     and c['slot'] == slot and c['card_id'] == card_id), None)

    def synchronize(self, obs):
        if self.selection and self.card(obs, **self.selection) is None:
            self.selection = self.target = None
        if self.target:
            self.target['affordable'] = obs['elixir'][0] + 1e-6 >= self.target.get('cost', 0)

    def snapshot(self, obs, paused=False, entities=(), arena_size=(18000, 32000)):
        self.synchronize(obs)
        row = self.card(obs, **self.selection) if self.selection else None
        return dict(side=0, selection=self.selection, target=self.target, last_serial=self.last_serial,
                    forbidden=self.forbidden_zones(row, entities, arena_size),
                    paused=paused, feedback=self.feedback if self.feedback_until is None
                    or obs['tick'] <= self.feedback_until else '')

    def forbidden_zones(self, row, entities, arena_size):
        """Enemy-side lane rectangles [x0, y0, x1, y1] an own-side-only card
        cannot land in. A lane whose enemy Princess Tower fell is left open:
        its pocket rule is not modelled, and the engine still decides."""
        if row is None or row['name'] not in self.own_side_only:
            return []
        width, height = arena_size
        zones = []
        for x0, x1 in ((0, width // 2), (width // 2, width)):
            if any(e.get('row') == 'PrincessTower' and e.get('side') == 1 and e.get('hp', 0) > 0
                   and x0 <= e['x'] < x1 for e in entities):
                zones.append([x0, OWN_TERRITORY_END_Y, x1, height])
        return zones

    def snaps_back(self, row, x, y, entities, arena_size):
        return any(x0 <= x <= x1 and y0 <= y <= y1
                   for x0, y0, x1, y1 in self.forbidden_zones(row, entities, arena_size))

    def handle(self, command, obs, *, paused=False, ended=False, arena_size=(18000, 32000), entities=()):
        # Delayed requests from a previous round must neither act on this
        # round nor advance its serial barrier (including delayed close).
        if type(command.get('match_id')) is not type(self.match_id) or command.get('match_id') != self.match_id:
            return None
        serial = command.get('serial')
        if not isinstance(serial, int) or isinstance(serial, bool) or serial <= self.last_serial:
            return None
        self.last_serial = serial
        op = command.get('op')
        if op == 'close':
            return op
        if op in ('pause', 'reset'):
            return op
        if op == 'cancel':
            self.selection = self.target = None
            return None
        if op not in ('select', 'aim', 'play'):
            return None
        slot, card_id = command.get('slot'), command.get('card_id')
        if type(slot) is not int or type(card_id) is not int or not 0 <= slot < 4:
            return None
        row = self.card(obs, slot, card_id)
        if row is None:
            self.selection = self.target = None
            self.message('Your hand changed. Select a card again.', obs['tick'])
            return None
        card = self.card_name(row)
        if card not in self.deck:
            self.selection = self.target = None
            self.message(f"Unrecognized card {row['name']}", obs['tick'])
            return None
        selection = dict(slot=slot, card_id=card_id)
        if op == 'select':
            self.selection, self.target = selection, None
            self.message(f"{card.replace('-', ' ').title()}: click the arena", obs['tick'])
            return None
        if op == 'aim' and self.selection != selection:
            return None
        x, y = command.get('x'), command.get('y')
        if type(x) is not int or type(y) is not int or not (0 <= x <= arena_size[0] and 0 <= y <= arena_size[1]):
            return None
        if op == 'play':
            if ended or paused:
                self.message('Press R for a new match' if ended else 'Press Space to resume', obs['tick'])
                return None
            # Browser plays carry the exact current hand identity and target.
            # They need not wait for an earlier select or every mouse preview.
            # The native engine still validates the target and affordability.
            self.selection = selection
        # Like the real game, an own-side-only card dropped past the edge of
        # your territory lands at that edge in the same column (by the bridge).
        snaps = self.snaps_back(row, x, y, entities, arena_size)
        land_x, land_y = (x, OWN_TERRITORY_END_Y - 500) if snaps else (x, y)
        check = self.engine.request(dict(op='validate_target', side=0, slot=slot, x=land_x, y=land_y))
        self.target = dict(selection, x=x, y=y, land_x=land_x, land_y=land_y, valid=bool(check.get('valid')),
                           affordable=bool(check.get('affordable')), cost=check.get('cost', row['cost']),
                           snaps=snaps)
        if op == 'aim':
            return None
        if not check.get('valid'):
            self.message('You cannot place that card here', obs['tick'])
        elif not check.get('affordable'):
            self.message(f"Need {check.get('cost', row['cost'])} elixir", obs['tick'])
        else:
            return HumanAction(slot, card, land_x, land_y, card_id, serial, snapped=snaps)
        return None

    def observe_result(self, action, result, tick):
        if result.get('accepted'):
            self.selection = self.target = None
            self.message(f"Played {action.card.replace('-', ' ').title()}"
                         + (' (moved to your side)' if action.snapped else ''), tick)
        else:
            reason = result.get('rejection', '')
            self.message('Need more elixir' if reason == 'insufficient_mana' else
                         'Match is finishing' if reason == 'original_submission_rejected' else
                         'That placement was not accepted', tick)
