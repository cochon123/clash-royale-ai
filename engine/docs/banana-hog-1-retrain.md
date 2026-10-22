# Banana Hog1: three-hour retraining experiment

Run: `banana-hog-1-r2`, under `runs/banana-hog-1-retrain-20261020`.
Dashboard: http://127.0.0.1:8783/.
Budget: 20 October 2026, 06:23:46–09:23:46 America/Edmonton
(12:23:46–15:23:46 UTC), including training and evaluation. The absolute end
survives process restarts. The initial original and failed overnight runs are
preserved.

## Starting point and model compatibility

The parent is the original `runs/banana-league-20261019/models/hog/final.pt`,
SHA-256 `f4cff713ebef11b3c1c5579c4f75e8d90541bb81a5dda821b5cbf42a15ecbd1c`.
Initial actor, waiting and value tensors exactly match that checkpoint.
The experiment starts afresh from this parent, not from the pilot or the failed
overnight last iterate.

The actor receives the same decks, own authoritative hand, both elixirs and
accepted-play history as before. Its architecture is unchanged. The pretrained
encoder, embeddings, timing and placement networks are frozen. Only card
scoring and waiting learn (35,042 parameters); the value head learns separately
(13,281 parameters), with no gradient into the pretrained representation.
Saved weights load with the existing `FastBananaPolicy` deployment adapter.

## Learning changes

The objective remains native terminal victory, defeat or draw. Native tower HP
provides intermediate reward feedback without entering the actor or critic.
The optional backend output is read from original native memory; it does not
advance the engine or change its actor observation.

Let Φ be own remaining tower-HP fraction minus the opponent's fraction, measured
over the six starting towers, with destroyed towers retained at zero HP. The
reward adds `0.5 × (Φ_next − Φ_current)`, setting terminal Φ to zero. At discount
1, these terms telescope: total episode reward equals the native W/L/D return
minus its action-independent initial potential. Every terminal training game
checks this identity and appends `reward-audit.jsonl`. There are no bonuses for
playing Hog Rider or particular cards.

GAE with lambda 0.97 replaces a single terminal return copied across every
decision. Games have equal total weight; actor advantages are normalized over
decisions where playing or voluntarily waiting is possible. Card and wait
distributions receive an exact KL penalty toward the frozen parent. Placement
is frozen, so PPO's stored placement support cannot drift during an update.
The actor starts at learning rate 0.00003; the detached critic uses 0.0003.

The run checks parent divergence, rolls back excessive updates and reduces the
actor learning rate. A complete selection evaluation more than eight percentage
points below the parent also restores the best snapshot and lowers that rate.
This limits deterioration; it does not guarantee improvement.

## Matches, evaluation and artifacts

Ten independent original-native CPU workers share CUDA inference and learning.
Each complete wave uses fixed learner weights. Only genuine native terminal
games enter PPO. Interrupted, errored and capped matches are VOID.

Nine classic decks train against frozen original-4.4.2 and frozen Hog1 bots.
Miner Wall Breakers and Giant Double Prince remain evaluation-only. Selection
uses 88 fixed games across all eleven decks, two controllers, both seats and
two seeds. Fresh seed ranges start at 2,100,000,000 for selection and
3,100,000,000 for confirmation. Sampled-policy diagnostics are reported
separately from greedy selection. Final confirmation never feeds learning.
These bots are not expert human deck pilots.

`initial.pt`, `latest.pt`, `best.pt`, `last-training.pt` and `final.pt` are saved
atomically with checksums. `final.pt` always uses the best selected snapshot,
including at an interrupted deadline; confirmation completeness is explicit.
If no candidate beats the parent, best/final can still be the parent. Runtime
state uses plain data instead of pickled sampler objects. Resume checks the
deadline, parent, engine, pool, retrain source and critical experiment settings.

## Startup validation and supervision

Fourteen tests passed, including CUDA rollout/learning likelihood consistency,
unchanged frozen weights after a real gradient update, reward telescoping,
terminal GAE, sampler serialization, resume configuration and selected-final
behavior. A separate ten-game pilot produced a real PPO update, ten valid
return checks and 389 intermediate HP-feedback steps. New checkpoints loaded
normally in a separate process and through the unchanged deployment adapter.

A controlled service restart at 12:25:47 UTC verified resume and installed the
finalization/configuration guards. Nine active evaluation attempts were
interrupted and counted VOID; that incomplete baseline is excluded and rerun.
The original three-hour deadline was retained. These startup interruptions are
not wins, losses, or training data.

User services: `banana-hog1-r2-train.service`,
`banana-hog1-r2-dashboard.service`, and `banana-hog1-r2-deadline.timer`.
The trainer holds a temporary sleep/idle inhibitor. Inspect with
`journalctl --user -u banana-hog1-r2-train.service -f`.
To stop cleanly, create `runs/banana-hog-1-retrain-20261020/STOP`.
The dashboard remains available after training ends.

## Additional three-hour continuation

At the user's request, a second phase runs on 20 October 2026 from
20:23:46 to 23:23:46 America/Edmonton (21 October 02:23:46–05:23:46 UTC).
Artifacts are under `runs/banana-hog-1-retrain-20261020-extension-1`;
the live dashboard continues at http://127.0.0.1:8783/.

The first phase's complete selection score rose from the original parent's
40.91% to 42.61%, with considerable uncertainty. Updates 78, 104 and 130
tied at that score. Confirmation reached only 63/88 valid matches before the
deadline, so its partial score does not establish improvement. The dashboard
now identifies incomplete confirmation and excludes it from the selection
curve.

The continuation starts from `last-training.pt` at update 130, preserving the
actor, value head, wait head, Adam state, sampler, and Python/Torch/CUDA RNG
state exactly. The selected-best snapshot at update 78 is retained separately
as the rollback/final fallback. The entire first phase remains unchanged.
Inputs, frozen networks, reward construction and learning hyperparameters are
unchanged, so this remains a test of more training with the conservative setup.

Selection runs every 45 minutes instead of every 20, and the unchanged frozen
parent's selection results are reused. Finalization avoids re-evaluating
unchanged weights. It reserves the measured cost of a final selection plus
both fresh confirmation banks, with a 20% margin and cleanup time. A periodic
evaluation may start only if that reserve still fits afterward.

The new independent confirmation uses four fresh native seeds beginning at
3,200,000,000: 176 games each for original Hog1 and the selected candidate,
covering eleven decks, two frozen controllers and both seats. Earlier partial
confirmation data is archived only. Final selection is frozen before looking
at new confirmation results; restarts resume only missing valid confirmation
games and cannot return to gradient training afterward.

Twenty tests passed, including exact continuation state and source-file hash
checks, disjoint seed banks, evaluation budgeting, phase metrics, missing-game
confirmation resume and the prohibition on training after confirmation.
The supervised run completed its first ten new native training games and
produced update 131 with the pretrained body still unchanged.

Cumulative counters follow the update-130 training checkpoint; the earlier
post-training confirmation remains in the original run. Phase counters,
throughput, card deployments and wait diagnostics describe the additional
budget independently. Services are `banana-hog1-r2-extension-train.service`,
`banana-hog1-r2-extension-dashboard.service` and
`banana-hog1-r2-extension-deadline.timer`. The continuation has its own STOP
file at `runs/banana-hog-1-retrain-20261020-extension-1/STOP`.
