import math
import unittest
from unittest.mock import patch

import torch

from .banana_hog_adaptive import SamplingPolicy
from .banana_policy import BananaEpisode, ROOT
from .banana_timed_policy import CardPlan, PhoneStyleScheduler, ready_time


class TimedHarnessTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        cls.policy = SamplingPolicy(device="cpu", weights=ROOT / "runs/banana-1/base.pt")
        cls.policy.requires_grad_(False)
        cls.policy.eval()
        cls.hog = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
        cls.bait = ("goblin-barrel", "princess", "knight", "goblin-gang", "inferno-tower", "rocket", "the-log", "ice-spirit")

    def setup_match(self, red_available=False):
        episode = BananaEpisode((self.hog, self.bait))
        episode.policy_seed = 44219
        hands = [[dict(slot=i, name=card, cost=cost, available=True)
                  for i, (card, cost) in enumerate(zip(self.hog[:4], [4, 4, 2, 1]))],
                 [dict(slot=i, name=card, cost=3, available=red_available)
                  for i, card in enumerate(self.bait[:4])]]
        obs = dict(hands=hands, elixir=[3., 3.])
        def mask(request, native_slot):
            value = torch.zeros(576, dtype=torch.bool)
            value[100] = True
            return value
        return episode, obs, PhoneStyleScheduler([self.policy, self.policy], greedy=True, placement_mask_provider=mask)

    def test_original_selects_unaffordable_hog_and_keeps_saving_instead_of_cycling(self):
        episode, obs, scheduler = self.setup_match()
        # Reusing the PPO wait head would fail this test, even though the
        # checkpoint's learned delay and affordable cheap cards remain valid.
        with patch.object(self.policy.wait_head, "forward", side_effect=AssertionError("wait head must be bypassed")):
            self.assertEqual(scheduler.choose(episode, obs, 0.), [])
            plan = scheduler.plans[0]
            self.assertEqual(plan.card, "hog-rider")
            self.assertGreater(plan.ready_at, 0.)
            self.assertEqual(scheduler.status[0]["reason"], "timing")
            self.assertEqual(scheduler.choose(episode, obs, plan.ready_at + .1), [])
            self.assertIs(scheduler.plans[0], plan)
            self.assertEqual(scheduler.status[0]["reason"], "saving_elixir")
            self.assertEqual(scheduler.choose(episode, obs, plan.ready_at + 8), [])
            self.assertIs(scheduler.plans[0], plan)
            obs["elixir"][0] = 4.
            actions = scheduler.choose(episode, obs, plan.ready_at + 8.05)
        self.assertEqual(len(actions), 1)
        self.assertEqual((actions[0].card, actions[0].slot, actions[0].tile), ("hog-rider", 0, 100))
        scheduler.observe_result(actions[0], dict(accepted=True))
        self.assertIsNone(scheduler.plans[0])

    def test_human_side_is_never_planned_and_accepted_human_play_replans_ai(self):
        episode, obs, _ = self.setup_match(red_available=True)
        scheduler = PhoneStyleScheduler([None, self.policy], active_sides=(1,))
        with patch.object(scheduler, '_plan', return_value=CardPlan('goblin-barrel', 0, 0, 3., 10., 10.)) as plan:
            self.assertEqual(scheduler.choose(episode, obs, 0.), [])
            self.assertEqual(plan.call_args.args[0], 1)
            self.assertIsNone(scheduler.plans[0])
            episode.record_action(0, 'hog-rider', 4500, 13000, 1.)
            scheduler.choose(episode, obs, 1.)
            self.assertEqual(plan.call_count, 2)
            self.assertEqual(plan.call_args.args[0], 1)

    def test_failed_earlier_candidate_cannot_advance_other_side_clock(self):
        episode, obs, scheduler = self.setup_match(red_available=True)
        with patch.object(scheduler, "_plan", side_effect=[
                CardPlan("hog-rider", 0, 0, 4., 1., 1.),
                CardPlan("goblin-barrel", 0, 0, 3., 3., 3.)]):
            self.assertEqual(scheduler.choose(episode, obs, 0.), [])
        self.assertEqual(scheduler.choose(episode, obs, 1.), [])
        self.assertEqual(scheduler.status[0]["reason"], "saving_elixir")
        self.assertEqual(scheduler.status[1]["reason"], "timing")
        action = scheduler.choose(episode, obs, 3.)[0]
        self.assertEqual((action.side, action.card), (1, "goblin-barrel"))
        scheduler.observe_result(action, dict(accepted=False, rejection="insufficient_mana"))
        self.assertIsNotNone(scheduler.plans[1])

    def test_replanning_after_confirmed_event_preserves_same_card_deadline(self):
        episode, obs, scheduler = self.setup_match()
        scheduler.choose(episode, obs, 0.)
        original = scheduler.plans[0]
        episode.record_action(1, "knight", 4000, 22000, .1)
        scheduler.choose(episode, obs, .2)
        if scheduler.plans[0].card == original.card:
            self.assertLessEqual(scheduler.plans[0].ready_at, original.ready_at)
        self.assertEqual(scheduler.revision, (id(episode), 1))

    def test_delay_is_anchored_to_last_event_and_does_not_slide_each_poll(self):
        due, delay = ready_time(3., 2., math.log1p(4.))
        self.assertAlmostEqual(due, 6.)
        self.assertAlmostEqual(delay, 4.)
        due, delay = ready_time(10., 2., math.log1p(4.))
        self.assertAlmostEqual(due, 10.)
        self.assertAlmostEqual(delay, 4.)
        self.assertEqual(ready_time(0., 0., -1.), (.2, .2))
        self.assertEqual(ready_time(0., 0., 1000.), (12., 12.))
        with self.assertRaises(ValueError):
            ready_time(0., 0., float("nan"))


if __name__ == "__main__":
    unittest.main()
