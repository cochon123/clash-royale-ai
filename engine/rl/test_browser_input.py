"""Atomic browser input contract, without importing models or starting an engine."""
import importlib.util
from pathlib import Path
import sys
import types
import unittest
from unittest.mock import patch


def load_input_module():
    # banana_human only needs the spelling mapper; don't import torch/Gym for
    # input validation tests or initialize any native engine/checkpoint.
    package = types.ModuleType('_arena_input_test')
    package.__path__ = []
    policy = types.ModuleType('_arena_input_test.banana_policy')
    policy._native_card = lambda name, deck: next(
        (card for card in deck if card.replace('-', '').lower() == name.lower()), name)
    spec = importlib.util.spec_from_file_location('_arena_input_test.banana_human',
                                                 Path(__file__).with_name('banana_human.py'))
    module = importlib.util.module_from_spec(spec)
    with patch.dict(sys.modules, {'_arena_input_test': package,
                                 '_arena_input_test.banana_policy': policy,
                                 spec.name: module}):
        spec.loader.exec_module(module)
    return module


human = load_input_module()


class Engine:
    def __init__(self):
        self.requests = []
        self.valid = self.affordable = True

    def request(self, value):
        self.requests.append(value)
        return dict(valid=self.valid, affordable=self.affordable, cost=4)


class BrowserInputTests(unittest.TestCase):
    def setUp(self):
        self.engine = Engine()
        self.controller = human.HumanController(self.engine, ('hog-rider', 'ice-golem'))
        self.controller.reset(1)
        self.obs = dict(tick=100, elixir=[5., 5.], hands=[[
            dict(slot=0, card_id=26, name='HogRider', cost=4, available=True),
            dict(slot=1, card_id=27, name='IceGolem', cost=2, available=True)], []])

    def command(self, **changes):
        return dict(dict(op='play', serial=1, match_id=1, slot=0, card_id=26,
                         x=4500, y=13000), **changes)

    def test_play_atomically_selects_current_card_without_select_or_aim(self):
        action = self.controller.handle(self.command(), self.obs)
        self.assertIsInstance(action, human.HumanAction)
        self.assertEqual(self.controller.selection, dict(slot=0, card_id=26))
        self.assertEqual(action.native_request(), dict(side=0, slot=0, x=4500, y=13000))
        self.assertEqual(self.engine.requests, [dict(op='validate_target', side=0, slot=0, x=4500, y=13000)])

    def test_play_replaces_an_older_selection_with_its_exact_available_card(self):
        self.controller.handle(self.command(op='select'), self.obs)
        action = self.controller.handle(self.command(serial=2, slot=1, card_id=27), self.obs)
        self.assertEqual(action.card, 'ice-golem')
        self.assertEqual(self.controller.selection, dict(slot=1, card_id=27))
        self.assertEqual(self.engine.requests[-1]['slot'], 1)

    def test_stale_match_cannot_act_close_or_raise_active_serial_barrier(self):
        for command in [self.command(serial=9999, match_id=0),
                        self.command(serial=9999, match_id=0, op='close'),
                        self.command(serial=9999, match_id=True)]:
            self.assertIsNone(self.controller.handle(command, self.obs))
        self.assertEqual(self.controller.last_serial, 0)
        self.assertIsInstance(self.controller.handle(self.command(), self.obs), human.HumanAction)

    def test_new_round_accepts_lower_serial_and_ignores_delayed_previous_round(self):
        self.controller.handle(self.command(serial=500), self.obs)
        self.controller.reset(2)
        self.assertEqual(self.controller.last_serial, 0)
        self.assertIsNone(self.controller.handle(self.command(serial=9999), self.obs))
        self.assertIsInstance(self.controller.handle(self.command(match_id=2), self.obs), human.HumanAction)
        self.assertEqual(self.controller.last_serial, 1)

    def test_duplicate_and_out_of_order_play_never_validate_twice(self):
        self.controller.handle(self.command(serial=50), self.obs)
        for serial in [50, 49, True, 51.0, '51', 0, -1]:
            self.assertIsNone(self.controller.handle(self.command(serial=serial), self.obs))
        self.assertEqual(len(self.engine.requests), 1)

    def test_wrong_slot_card_or_unavailable_card_cannot_be_played(self):
        commands = [dict(card_id=999), dict(slot=1), dict(slot=4), dict(slot=True),
                    dict(slot=0.0), dict(card_id=True), dict(card_id=26.0)]
        for serial, changes in enumerate(commands, 1):
            self.assertIsNone(self.controller.handle(self.command(serial=serial, **changes), self.obs))
        self.obs['hands'][0][0]['available'] = False
        self.assertIsNone(self.controller.handle(self.command(serial=100), self.obs))
        self.assertEqual(self.engine.requests, [])

    def test_paused_and_ended_plays_are_rejected_before_native_validation(self):
        for serial, state in enumerate([dict(paused=True), dict(ended=True)], 1):
            self.assertIsNone(self.controller.handle(self.command(serial=serial), self.obs, **state))
        self.assertEqual(self.engine.requests, [])
        self.assertIn('new match', self.controller.feedback)
        self.assertIsInstance(self.controller.handle(self.command(serial=3), self.obs), human.HumanAction)

    def test_invalid_coordinates_are_rejected_before_selection_or_native_call(self):
        for serial, coordinates in enumerate([dict(x=-1), dict(x=18001), dict(y=-1),
                                              dict(y=32001), dict(x=True), dict(y=1.5),
                                              dict(x='4500')], 1):
            self.assertIsNone(self.controller.handle(self.command(serial=serial, **coordinates), self.obs))
        self.assertIsNone(self.controller.selection)
        self.assertEqual(self.engine.requests, [])

    def test_native_target_validity_and_affordability_still_decide_play(self):
        self.engine.valid = False
        self.assertIsNone(self.controller.handle(self.command(), self.obs))
        self.assertIn('cannot place', self.controller.feedback)
        self.engine.valid, self.engine.affordable = True, False
        self.assertIsNone(self.controller.handle(self.command(serial=2), self.obs))
        self.assertIn('elixir', self.controller.feedback)
        self.assertEqual(len(self.engine.requests), 2)

    def test_atomic_play_keeps_original_own_side_snap_and_native_coordinates(self):
        self.controller.own_side_only = {'HogRider'}
        towers = [dict(row='PrincessTower', side=1, x=3500, y=25500, hp=3052)]
        action = self.controller.handle(self.command(x=3500, y=22000), self.obs, entities=towers)
        self.assertTrue(action.snapped)
        self.assertEqual(action.native_request(), dict(side=0, slot=0, x=3500, y=14500))
        self.assertEqual(self.engine.requests[-1]['y'], 14500)

    def test_aim_still_requires_the_exact_current_selection(self):
        self.assertIsNone(self.controller.handle(self.command(op='aim'), self.obs))
        self.assertEqual(self.engine.requests, [])
        self.controller.handle(self.command(op='select', serial=2), self.obs)
        self.controller.handle(self.command(op='aim', serial=3), self.obs)
        self.assertEqual(len(self.engine.requests), 1)


if __name__ == '__main__':
    unittest.main()
