# Banana-2: imitation with experimental native supervision

The Banana-2 actor used from here on is the epoch-7 export at
`engine/runs/banana-2/base.pt`. It is registered as **Banana 2** in
`models/registry.json`. The epoch-3 weights that played the first round robin
are unchanged at `engine/runs/banana-2/archive-epoch3/`. The matched BC-only
checkpoints in the same run directory are the epoch-7 twin.

Banana-2 continues the original Banana-1 actor, which is an exact export of
policy-bc-v4.4.2 human behavior cloning. This run uses no reinforcement learning
and launches no native or real-game matches.

The run is on the laptop, under `banana-2-train.service`. It runs three epochs
on the legacy training population, with up to 40 winner-side actions per battle,
batch size 192, learning rate 1e-5, two Torch CPU threads and a 300% CPU ceiling.
Its memory ceiling is 6 GiB. The actual GPU is the RTX 3050 6 GB laptop GPU.
The trainer uses bfloat16 for training when supported, and float32 evaluation.

- [Live report](http://100.88.135.1:8801/reports/banana2.html)
- [Clash Lab run](http://100.88.135.1:8801/#runs?id=banana-2)

```sh
tail -F ~/Documents/mini-project/clash-royale-ai/engine/runs/banana-2/console.log
systemctl --user status banana-2-train.service --no-pager
```

Console columns show work completed, total work and ETA. Preparation and training
are separate phases. Training ETA uses observed update throughput; development
evaluations add time. The unit continues after the terminal closes. Keep the
laptop powered and awake.

## Experiment

Two actors start at the same Banana-1 checkpoint. Both receive the same batches,
dropout RNG, human card/placement/timing losses and a 0.1-weight parent card KL
penalty. One is a BC-only control. The Banana-2 arm additionally learns a small
24-output state-summary head with Smooth L1 loss, weight 0.05 every fourth BC
batch. Target means and scales are fitted on auxiliary training rows only;
standard deviation is floored at 0.05. The head is training-only. Native board,
hand and elixir values never enter the actor as features.

Native targets include each side's tower HP fractions, elixir and living
non-tower count/HP in four coarse regions. Nonpositive-HP entities and
zero-max-HP projectiles are excluded. Board coordinates are normalized to the
acting physical side. Only records whose source orientation agrees with the
existing replay policy convention are paired.

For auxiliary queries the actor receives only preceding confirmed replay events
and the known observation clock at the frame timestamp. The pending human
action's card, placement and timing labels are discarded on this path. The
ordinary BC path retains the parent's previous-event observation convention.

## Source quality and splits

The enrichment sweep produced 258,150 snapshots, but most are short prefixes.
This experiment requires at least eight prior events, excludes same-tick
siblings and caps supervision at twelve frames per battle. Source hashes and
the complete leading event sequence must match before a native frame can be
joined to its replay. Only committed enrichment result records are used.

Original training, development and final-test battle IDs are retained.
Side-canonical full card trajectories are deduplicated, and copies of reserved
held-out trajectories are excluded from training. The final test is not opened
for evaluation. The parent and older ancestors have legacy development ancestry;
development metrics measure a controlled continuation, not pristine generalization.

All source flags remain `verified: false` and `training_usable: false`. The
user-requested experiment explicitly reads quarantine files as uncertain
auxiliary targets, without approving them as ground truth or changing the
existing training datasets. Original levels, evolutions, seed, hand, clock and
balance-version fidelity are still unverified.

An Opus 5.5 subagent was requested through all four configured routes. Claude
and Claude Proxy hit rate limits; Cursor failed its provider turn; OpenCode's
OpenRouter route lacked credits. No Opus advice was received. Those outcomes
are recorded in the run's `operator-notes.json`.

## Artifacts and selection

`training-manifest.json` records source/code hashes and hyperparameters;
`data-contract.json` pins input artifacts; `split-manifest.json` contains battle
IDs; `auxiliary-manifest.jsonl` lists each auxiliary frame and its artifact hash.
`prepared.pt` is an isolated experiment cache. Quarantined sources and the
original human data remain unchanged.

Each arm saves its best and latest actor by development human-action loss.
After completion, `best_model.pt` and `base.pt` export the selected Banana-2
actor. Its checkpoint container remains compatible with native Banana adapters:
the neutral parent critic/wait states are preserved, with explicit metadata
stating that training was behavior cloning. No PPO updates are implied by the
legacy container format. Auxiliary head best/latest checkpoints remain separate.

`summary.json` compares the candidate, the matched BC continuation and the
frozen parent. No model is automatically made recommended or substituted for
Banana-1. A lower action loss alone establishes neither improved win rate nor
reconstruction accuracy.

The run handles STOP/SIGTERM between batches by saving interrupted actor
checkpoints. It does not automatically resume an interrupted optimizer state;
the unit deliberately does not restart or automatically rerun a completed job.

Small metadata files and `reports/banana2.html` plus `banana2-live.json` mirror
from laptop to home every fifteen seconds. Checkpoints and the prepared cache
stay local. The report has interactive SVG curves, curve toggles, point details,
epoch measurements and a live heartbeat; no matplotlib or external chart library.
