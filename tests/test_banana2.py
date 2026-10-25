"""Scientific boundary checks for experimental Banana-2 supervision."""
import copy
from dataclasses import replace
import unittest

import numpy as np
import torch

from cr_replay_pipeline.banana2_data import board_target, clean_partitions
from cr_replay_pipeline.policy_dataset import encode_policy_sample
from cr_replay_pipeline.winner_dataset import BattleExample, CardVocab


def battle(ident='a'):
    deck=tuple(['knight','archers','goblins','giant','pekka','minions','balloon','witch'])
    return BattleExample(ident,deck,deck,1,tuple({'seconds':float(j+1),'side':'team' if j%2==0 else 'opponent',
        'card':deck[j%8],'event_type':'card_play','x':3000,'y':7000 if j%2==0 else 25000} for j in range(12)))


def frame(own=0):
    towers=[{'id':5000000+i,'side':i//3,'row':'KingTower' if i%3==0 else 'PrincessTower',
             'hp':100 if i!=2 else 50,'max_hp':100,'x':9000 if i%3==0 else 3500 if i%3==1 else 14500,
             'y':3000 if i<3 else 29000} for i in range(6)]
    living={'id':1,'side':0,'row':'Knight','x':1000,'y':8000,'hp':600,'max_hp':1000}
    projectile={'id':2,'side':0,'row':'Arrow','x':1000,'y':8000,'hp':-1,'max_hp':0}
    return {'native_side':own,'state_before_action':{'board':{'entities':towers+[living,projectile]},
        'reward_state':{'towers':towers},'observation':{'elixir':[3,8]}}}


class Banana2DataTest(unittest.TestCase):
    def test_dead_projectiles_excluded_and_tower_identity(self):
        y=board_target(frame())
        np.testing.assert_allclose(y[:4],[1,1,.5,.3])
        self.assertAlmostEqual(float(y[4]),1/12)
        self.assertAlmostEqual(float(y[5]),.05)
        self.assertEqual(float(y[6:12].sum()),0)

    def test_physical_side_normalization(self):
        y=board_target(frame(1))
        self.assertAlmostEqual(float(y[3]),.8)
        self.assertAlmostEqual(float(y[15]),.3)
        # The side-0 knight appears in enemy-left-far relative to side 1.
        self.assertAlmostEqual(float(y[20]),1/12)

    def test_missing_towers_rejected(self):
        value=frame();value['state_before_action']['reward_state']['towers'].pop()
        with self.assertRaises(ValueError): board_target(value)

    def test_whole_trajectory_holdout_copies_removed(self):
        a=battle('train');held=replace(a,battle_id='validation')
        other=replace(a,battle_id='other',events=tuple(dict(e,x=5000) for e in a.events))
        by={b.battle_id:b for b in [a,held,other]}
        tr,va,n=clean_partitions(by,{'train_ids':['train','other'],'validation_ids':['validation'],'test_ids':[]})
        self.assertEqual(tr,['other']);self.assertEqual(va,['validation']);self.assertEqual(n,1)

    def test_auxiliary_actor_inputs_ignore_pending_action_and_future(self):
        a=battle();vocab=CardVocab(list(a.team_deck));costs={c:3 for c in a.team_deck}
        events=copy.deepcopy(list(a.events));events[8].update(card='archers',x=17000,y=12000,seconds=99.)
        events[9].update(card='witch',seconds=100.)
        changed=replace(a,events=tuple(events))
        x=encode_policy_sample(a,8,vocab,costs,threat_dim=14,observation_seconds=9.)
        y=encode_policy_sample(changed,8,vocab,costs,threat_dim=14,observation_seconds=9.)
        for k in range(7): torch.testing.assert_close(x[k],y[k],rtol=0,atol=0)
        self.assertFalse(torch.equal(x[7],y[7])) # Only discarded action labels differ.


if __name__=='__main__':unittest.main()
