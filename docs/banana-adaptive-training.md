# Tower-aware adaptive training

`rl.banana_hog_adaptive` is a fresh experiment path. It does not resume or
overwrite the earlier Hog1-r2 runs. The current canonical run is `runs/banana-hog-1`, launched on 21 October 2026
from the original Banana-1 for two hours. See `launch.json` there for the fixed
deadline and command. Earlier Hog checkpoints and Hog-only run artifacts were
deleted at the user's request; Mortar checkpoints and the base model remain.

The six new inputs are HP/max-HP fractions: own king, weaker princess,
stronger princess, then the enemy equivalents. Destroyed towers remain at zero.
They come from the authoritative native `reward_state` at reset and every step,
including evaluation. Missing or invalid HP causes an error, never guessed HP.
The current reward snapshot has no lane coordinates, so these features do not
identify the damaged princess tower's lane.

A zero-output residual branch preserves the pretrained logits at initialization.
The adaptive policy uses the resulting context for waiting, value estimation
and placement, and also adjusts card logits. Old model inputs stay intact.
Load an adaptive checkpoint with `AdaptiveHogPolicy(weights=...)`; load a
conservative HP-only checkpoint with `TowerHPPolicy(weights=...)`. These loaders
also accept original checkpoints. The original `FastBananaPolicy` cannot load
HP checkpoints and will reject their additional weights.

## Changes from the conservative experiment

- `--trainable all` (default) enables representation, card and placement
  gradients. `placement` keeps the representation frozen; `heads` keeps both
  representation and placement parameters frozen. Every mode trains HP inputs,
  waiting and the critic. The body learning rate defaults to `3e-6`, versus
  `3e-5` for action heads and `3e-4` for the critic. Dropout remains disabled;
  GPU GRU backward retains its required workspace for both PPO and anchor loss.
- Rollout and evaluation sample from the full 576-tile grid, with the same
  temperatures. PPO retains the full support instead of a changing top-five
  set. Native validation still determines whether a placement is accepted;
  the rebuilt engine exports exact per-card tile masks through `placement_mask`.
  Native masks are enabled by default for fresh adaptive runs and the spectator;
  `--no-native-placement-masks` (trainer) or `--unmasked-placement` (spectator)
  reproduces historical unfiltered placement for diagnostics.
  Frozen opponents retain their existing action-selection settings.
- Sampled performance is the primary selection and independent confirmation
  metric. Both banks also measure greedy play. Parent and candidate use the
  same sampling settings. Each match has reproducible random numbers keyed by
  native seed, physical side and decision tick, independent of worker ordering.
- A complete evaluation that drops **two percentage points** against the best
  in either mode restores the entire best model and optimizer, then halves
  action-head and body learning rates. The greedy reference never decreases
  across promotions, preventing gradual degradation hidden by successive small
  drops. Incomplete evaluations cannot promote or trigger regression rollback.
  The PPO parent-KL safeguard also restores the entire model, including the
  representation, placement and HP branch.
- Evaluation results survive interruption and are keyed by the full weights
  digest, action mode and native matchup. Unfinished games are rerun. Selection
  and confirmation use separate seed ranges. No improvement claim follows from
  the plumbing pilots or their small match counts.

## Running a fresh experiment

Example only; running this command starts a new three-hour budget:

```sh
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python \
  -m rl.banana_hog_adaptive \
  --run-dir runs/banana-hog-adaptive-NEW \
  --engine native-engine/target/release/league_reference \
  --duration-seconds 10800 --workers 10 --device cuda
```

The parent defaults to original `banana-1` (`runs/banana-1/base.pt`), an exact
actor export from `policy-bc-v4.4.2` with fresh PPO heads and no specialist
training. The default opponents are frozen greedy and sampled original-model
bots. `--parent` can select another
compatible original Banana PPO checkpoint. The better parent remains an
experimental question. Use a fresh directory for a different parent,
trainability setting or evaluation setup. `--resume` retains the absolute
deadline and verifies settings and source hashes. Archived runs' source guards
will reject resuming their old experiments with these modified trainer files.

For a bounded plumbing check, add `--max-waves 1 --workers 2` and a dedicated
pilot directory. This saves `pilot.pt`, not a selected production model.

Validation includes native HP and adaptive training pilots, GPU rollout/PPO
likelihood agreement, actual encoder/placement/HP gradients, full-model
regression rollback, old-checkpoint compatibility, HP-checkpoint round trips,
per-game sampling independence, and original adapter regression tests.

```sh
PYTHONPATH=rl /home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python \
  -m unittest rl.test_banana_tower_hp rl.test_banana_hog_adaptive \
  rl.test_hog_retrain rl.test_hog_continue rl.test_banana_fast_policy
```

## Hog deployment rejection found after the first run

The first two-hour run and its recorded evaluations used unmasked placements.
In the watched Hog-vs-Log-Bait match, five selected Hog plays were all rejected
by the native validator (`invalid_target_code_5`), despite sufficient elixir.
The adapter now queries the same validator before choosing a tile, for the
selected native hand slot and side. PPO stores the resulting legal support.
Both learner and frozen opponents use this protocol in new masked runs.
The saved checkpoint weights are unchanged. Historical scores should not be
reported as scores for the corrected placement protocol without re-evaluation.
A same-seed replay with masks deployed Hog on its first decision; a bounded
12-second capture recorded eight accepted plays and zero rejections.

## Restored learned timing in spectator inference

The spectator defaults to `PhoneStyleScheduler` in `banana_timed_policy.py`,
following the two-phone harness's full-hand selection, learned delay,
last-confirmed-event anchor, and execution-time elixir checks. It does not use
PPO's Bernoulli wait head. Plans persist between polls; confirmed events and
actual hand changes can trigger replanning, so polling cannot continually push
a deadline forward or repeatedly replace an unaffordable choice with a cheap
card. Legal placement is checked when the selected card becomes ready.

This is an inference controller; the existing PPO likelihood/return machinery
continues to describe its historical wait-head policy. Evaluations under the
new controller must be identified separately, and its deterministic scheduling
must not be passed off as PPO actions with the old wait-head likelihood.
No saved weights were modified or training run launched for this repair.
