# Controlled policy experiments

The original v8 training objective is retired. Its checkpoints and source were
preserved in `models/policy_bc_v8/review_archive/`. Archived checkpoints still
load with their original memory update. Do not resume that run.

## What changed

- `PolicyObservation` separates a confirmed prefix, an explicitly chosen agent
  perspective, and the observation clock from future action labels. Revealed
  opponent information is applied before extracting deck statistics, not just
  before the deck embedding. The new contract gives the formerly reserved event
  feature an explicit position-unknown flag.
- Quiet time advances global elixir estimates and rolling windows. The model's
  query time advances too. Memory decay has an exact zero-time identity and
  composes between event updates.
- Clock data uses a fixed team perspective and labels either actor. Events at
  a shared timestamp form one group; if both actors appear, the loss marginalizes
  the unknown first actor. Optional known recording endpoints provide censored
  tails. The last deployment is never treated as a recording endpoint.
- Unresolved abilities remain explicit unknown observations. Card-play targets
  are the action task in this screening round; ability action selection is **not**
  implemented. Ability XY labels do not supervise placement in either policy loss.
- All controlled variants share the same v4.4.1-shaped GRU trunk (d_model 160,
  dropout 0.2, two layers) and selected-card heatmap heads. No offset head, extra
  future objective, or backbone replacement is mixed into the memory experiment.
  The optional memory is eight gated latent slots.
- Training action examples keep reaction repeats/weights. Validation metrics and
  the clock loader do not. Frequency and cycle slot baselines are scored on the
  same validation indices.
- Metadata content, preprocessing, manifests, costs, and source code are recorded
  in experiment contracts. Cache trust no longer waives identity validation.
  Legacy trainer resume rejects changed or missing contracts; new screening runs
  refuse to overwrite an existing run directory and intentionally start fresh.
- Warm starts check donor ancestry even when an explicit different manifest is
  supplied. Selected weight epochs and last completed epochs are separate, and
  the legacy report now reevaluates validation with the selected weights.

## Current data limitations

`data/metadata/battles.jsonl` currently contains **zero usable records**. The 1,000
downloaded files in `data/metadata/raw-battlelogs/` are authorization-error
responses, not battlelog arrays. The metadata argument is wired through the
canonical loader and CLI, but cannot recover identities from those responses.
Unknown activations are retained, not silently converted into known abilities.
After obtaining usable battlelogs, normalize them and start a new experiment
suite; the changed metadata hash will reject reuse of the old suite contract.

The new final holdout contains 1,000 battles. Its reservation excludes known
historical caches, all discovered split manifests, and the design-probe cohort,
with card-event fingerprints including reversed perspectives. It was not scored.
Capture mtime selects newer files; it is not claimed to be match chronology.

## Run the bounded screen

Prepare the immutable plan and source snapshot without training:

```sh
.venv/bin/python -m cr_replay_pipeline.controlled_experiment --prepare
```

Run **baseline, then clock**. Memory starts only if the clock gate passes at the
last shared completed epoch. There is no one-hour per-variant cap; variants share
the three-hour suite budget so they can finish the same epoch count.

```sh
PYTHONUNBUFFERED=1 .venv/bin/python -m cr_replay_pipeline.controlled_experiment \
  --variant screen > logs/controlled_screening_gated.log 2>&1
```

Stream work done and ETA:

```sh
.venv/bin/python -m cr_replay_pipeline.cli watch-policy-training \
  --progress models/controlled_screening_gated/progress.jsonl \
  --log logs/controlled_screening_gated.log
```

It opens at `http://127.0.0.1:8766`. Closing the dashboard does not stop
training. `tail -f logs/controlled_screening_gated.log` remains a text fallback.

The suite writes `clock_gate.json` after clock. If that gate fails, memory is not
run. If memory runs, `memory_gate.json` says whether shuffled/frozen belong in a
**new** budgeted suite. Extra seeds are never launched automatically.

The time limit is checked at batch boundaries; the last batch and checkpoint
finalization can add a small overhead. The suite uses an exclusive file lock and
persists spent time across invocations. It never increases its own budget.
Initialization and data loading count against the shared budget. Compare
`equal_epoch` in the comparison JSON, not selected checkpoints with different
epoch counts. Wall-time is recorded separately.

Each run writes `best.pt`, `last.pt`, `report.json`, `report.html`,
`validation_rows.npz`, `validation_e{epoch}.npz`, and `progress.jsonl`. Reports
identify the selected validation epoch. `source.zip`, `environment.json`, and
`contract.json` live at suite level. A budget stop before any complete validation
epoch produces a report but no valid best checkpoint. A partially trained final
epoch is not
presented as a completed epoch.

The runner scores **validation only**. It rejects a screening manifest touching
the final holdout. It does not generate phone actions, score a winner/style
judge, automatically run more seeds, or scale the corpus.

## Variant meanings and losses

| Variant | Shared policy | Joint clock | Gated memory |
|---|---|---|---|
| `baseline` | Yes | No | No |
| `clock` | Yes | Yes | No |
| `memory` | Yes | Yes | Aligned |
| `shuffled` | Yes | Yes | Other row's coherent observation |
| `frozen_time` | Yes | Yes | No elapsed-time transitions |

All variants use the same deterministic unique card-play indices. Training may
repeat reaction windows; validation does not, and there is still no mirroring.
cross entropy + true-card-conditioned cell cross entropy + 0.25 log-delay loss,
with reaction weights on the training loader only.
Clock variants add 0.1 competing-risk NLL on a separate uniform interval loader.
The sampled clock intervals do not inherit the action dataset's reaction bias.

Validation reports proper teacher-forced joint NLL separately from greedy card,
cell, and joint accuracy. It also reports card top-3, teacher-forced cell, card
NLL, opening/mid/late slices, batch-1 inference latency, and the action timing
head's MAE. Clock models additionally report next-actor accuracy/Brier score,
clock NLL, survival Brier scores at 2/5/10 seconds, and **clock median MAE**.
Do not confuse that last metric with the separate action timing head. Constant
rates fitted on training intervals provide a validation clock reference.
Frequency and cycle slot baselines use the same validation indices.

Checkpoint selection uses lowest validation joint NLL for every variant. Read
clock metrics at the **shared completed epoch** for gate decisions, not
whichever epoch gives the nicest selected-checkpoint clock result. The controls
and repeat seeds are separate, conditional experiments.

Compare paired action metrics after a screen (equal-epoch first):

```sh
.venv/bin/python scripts/compare_controlled_runs.py \
  models/controlled_screening_gated/baseline_seed42 \
  models/controlled_screening_gated/clock_seed42 \
  --output reports/controlled_clock_comparison.json
```

The comparison verifies matching contracts and target groups, then bootstraps
whole battles 2,000 times. It never promotes a model automatically. These are
validation intervals, not uncertainty over independently trained seeds.

Continue only for an approximately 1-point action benefit or a meaningful
clock/calibration benefit without action regression. If memory looks promising,
run `shuffled` and `frozen_time` in a newly budgeted suite with matching data and
configuration, then baseline/candidate seeds 43 and 44. Fund that work explicitly
after reading the initial results. Retire memory if aligned cannot beat controls.

## Offline inference

```python
from cr_replay_pipeline.controlled_policy import load_controlled_policy, predict_observation
from cr_replay_pipeline.causal_observation import prefix_observation

model, vocab = load_controlled_policy("models/controlled_screening_gated/clock_seed42/best.pt")
observation = prefix_observation(battle, confirmed_event_count, "team", now=current_time)
proposal = predict_observation(model, observation, vocab, costs)
```

This uses the same encoder as training. Clock variants schedule from clock
rates, report the probability of an own event, and request another observation
within 250 ms. A new observed event invalidates the previous proposal. The
returned coordinates are normalized in the agent's frame; this interface does
not tap a phone. It proposes card plays only and does not certify live legality.

## Board-information experiment

`data/board_oracle/annotation_queue.jsonl` contains 100 frames from ten recorded
games. Whole games are assigned to train/validation/test. Six HP fractions and
six coarse troop-presence labels are blank; OCR values are suggestions only.

Before use, fill `battle_id`, `event_index`, the labels, `verified_by`, and
`verified_at`, checking the saved frame against the confirmed event prefix.
Unknown labels remain null, never zero HP by default. `oracle_features()` returns
values plus masks and rejects unverified or future observations.

No verified board-label dataset currently exists, so no oracle model was trained
and no board-information gain is claimed. Accurate-label versus absent-label
versus shuffled-label training is the next conditional experiment after this
annotation and alignment work; a larger vision system is not implemented here.

## Verification

```sh
CUDA_VISIBLE_DEVICES='' OMP_NUM_THREADS=2 .venv/bin/python -m pytest -q
```

Tests cover future-label invariance, observation-object clock advancement, hidden-deck
statistics, both-actor rate recovery, ties/censoring, memory gradients, a multi-event
distractor, zero-time identity/composition, ancestry rejection, budget persistence,
equal-epoch continuation gates, masking ability placement losses, and CPU
training/checkpoint/report flows for all five variants.
A separate tiny real-data smoke run is saved in `models/controlled_smoke/`;
its one-epoch metrics are integration evidence, not performance claims.
