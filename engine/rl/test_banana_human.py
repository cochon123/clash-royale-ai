import json
from pathlib import Path
import tempfile
import unittest

from .banana_human import HumanAction, HumanController, InputQueue, own_side_only_cards

ROOT = Path(__file__).resolve().parents[1]


class Engine:
    def __init__(self):
        self.requests = []
        self.valid = self.affordable = True

    def request(self, request):
        self.requests.append(request)
        return dict(valid=self.valid, affordable=self.affordable, cost=4)


class HumanTests(unittest.TestCase):
    def setUp(self):
        self.engine = Engine()
        self.controller = HumanController(self.engine, ('hog-rider',)*8)
        self.controller.reset(1)
        self.obs = dict(tick=100, elixir=[5., 5.], hands=[[
            dict(slot=0, card_id=26, name='HogRider', cost=4, available=True)], []])
        self.serial = 0

    def send(self, op, **kwargs):
        self.serial += 1
        value = dict(op=op, serial=self.serial, match_id=1, slot=0, card_id=26, x=4500, y=13000)
        value.update(kwargs)
        return self.controller.handle(value, self.obs)

    def test_only_native_accepted_input_can_be_recorded_and_selection_is_pinned_to_the_card(self):
        self.send('select')
        action = self.send('play')
        self.assertIsInstance(action, HumanAction)
        self.assertEqual(action.native_request(), dict(side=0, slot=0, x=4500, y=13000))
        self.assertEqual(action.card, 'hog-rider')
        self.controller.observe_result(action, {'accepted': False, 'rejection': 'invalid_target_code_3'}, 100)
        self.assertIsNotNone(self.controller.selection)
        self.controller.observe_result(action, {'accepted': True}, 101)
        self.assertIsNone(self.controller.selection)
        # A self-contained browser play needs no separate selection, but the
        # same acknowledged command must never be replayed.
        self.assertIsNone(self.controller.handle(dict(op='play', serial=self.serial, match_id=1,
                                                     slot=0, card_id=26, x=4500, y=13000), self.obs))
        self.send('select')
        self.obs['hands'][0][0]['card_id'] = 27
        self.assertIsNone(self.send('play'))
        self.assertIsNone(self.controller.selection)

    def test_invalid_unaffordable_paused_stale_and_duplicate_commands_never_produce_a_play(self):
        self.send('select')
        self.engine.valid = False
        self.assertIsNone(self.send('play'))
        self.engine.valid, self.engine.affordable = True, False
        self.assertIsNone(self.send('play'))
        self.engine.affordable = True
        command = dict(op='play', serial=99, match_id=1, slot=0, card_id=26, x=4500, y=13000)
        self.assertIsNone(self.controller.handle(command, self.obs, paused=True))
        self.assertIsNone(self.controller.handle(command, self.obs))
        command['serial'], command['match_id'] = 100, 0
        self.assertIsNone(self.controller.handle(command, self.obs))
        self.assertEqual(self.controller.handle(dict(op='reset', serial=101, match_id=1), self.obs), 'reset')
        self.controller.reset(2)
        self.assertIsNone(self.controller.handle(dict(command, serial=102, match_id=1), self.obs))

    def test_preview_uses_exact_native_coordinates_and_cancel_cannot_place(self):
        self.send('select')
        self.send('aim', x=4123, y=13210)
        self.assertEqual(self.engine.requests[-1], dict(op='validate_target', side=0, slot=0, x=4123, y=13210))
        self.assertTrue(self.controller.snapshot(self.obs)['target']['valid'])
        self.send('cancel')
        self.assertIsNone(self.send('aim'))
        self.assertEqual(len(self.engine.requests), 1)

    def test_cards_use_the_supplied_policy_mapping_and_unknown_cards_are_refused(self):
        deck = ('hog-rider', 'ice-golem', 'musketeer', 'skeletons', 'ice-spirit', 'cannon', 'fireball', 'the-log')
        aliases = {'IceGolemite': 'ice-golem'}
        self.controller = HumanController(self.engine, deck, card_name=lambda row: aliases.get(row['name'], 'x'))
        self.controller.reset(1)
        self.obs['hands'][0][0].update(name='IceGolemite', card_id=50, cost=2)
        self.send('select', card_id=50)
        action = self.send('play', card_id=50)
        self.assertEqual(action.card, 'ice-golem')
        self.obs['hands'][0][0].update(name='Mystery', card_id=51)
        self.assertIsNone(self.send('select', card_id=51))
        self.assertIsNone(self.controller.selection)
        self.assertIn('Unrecognized', self.controller.feedback)

    def test_own_side_only_cards_follow_original_card_data(self):
        csv_dir = ROOT / 'analysis/decoded/assets/csv_logic'
        if not csv_dir.exists():
            self.skipTest('decoded original card tables are not present')
        names = own_side_only_cards(csv_dir)
        self.assertTrue({'HogRider', 'IceGolemite', 'Cannon', 'Log', 'BarbLog'} <= names)
        self.assertFalse({'Fireball', 'Rocket', 'GoblinBarrel', 'Arrows', 'Zap'} & names)

    def test_troops_aimed_past_the_river_are_flagged_only_while_that_lanes_tower_stands(self):
        self.controller.own_side_only = {'HogRider'}
        towers = [dict(row='PrincessTower', side=1, x=3500, y=25500, hp=3052),
                  dict(row='PrincessTower', side=1, x=14500, y=25500, hp=3052)]
        self.send('select')
        self.controller.handle(dict(op='aim', serial=50, match_id=1, slot=0, card_id=26, x=3500, y=22000),
                               self.obs, entities=towers)
        self.assertTrue(self.controller.target['snaps'])
        self.assertEqual((self.controller.target['land_x'], self.controller.target['land_y']), (3500, 14500))
        self.assertEqual(self.engine.requests[-1]['y'], 14500)
        self.assertEqual(self.controller.snapshot(self.obs, entities=towers)['forbidden'],
                         [[0, 15000, 9000, 32000], [9000, 15000, 18000, 32000]])
        action = self.controller.handle(dict(op='play', serial=51, match_id=1, slot=0, card_id=26, x=3500, y=22000),
                                        self.obs, entities=towers[1:])
        self.assertFalse(action.snapped)  # left tower down: the pocket may be legal
        self.assertEqual((action.x, action.y), (3500, 22000))
        self.controller.handle(dict(op='aim', serial=52, match_id=1, slot=0, card_id=26, x=3500, y=14000),
                               self.obs, entities=towers)
        self.assertFalse(self.controller.target['snaps'])
        self.controller.own_side_only = set()
        self.controller.handle(dict(op='aim', serial=53, match_id=1, slot=0, card_id=26, x=3500, y=22000),
                               self.obs, entities=towers)
        self.assertFalse(self.controller.target['snaps'])

    def test_queue_preserves_clicks_and_incomplete_lines_without_replaying(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'inputs.jsonl'
            queue = InputQueue(path)
            try:
                with path.open('a') as out:
                    out.write('{"op":"select"}\n{"op":"play"')
                self.assertEqual(queue.poll(), [dict(op='select')])
                self.assertEqual(queue.poll(), [])
                with path.open('a') as out:
                    out.write('}\n' + json.dumps(dict(op='pause')) + '\n')
                self.assertEqual(queue.poll(), [dict(op='play'), dict(op='pause')])
                self.assertEqual(queue.poll(), [])
            finally:
                queue.close()


if __name__ == '__main__':
    unittest.main()
