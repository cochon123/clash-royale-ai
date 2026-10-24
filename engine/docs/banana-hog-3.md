# Banana Hog 3

The production entry point is `python -m rl.crazy_hog_train`. The launcher
`tools/launch_crazy_hog.sh RUN_DIR [SECONDS]` starts a three-hour run by default.
It warm-starts the exact Banana Hog 2 final checkpoint and uses the unchanged,
validated PGO engine whose SHA-256 begins `32b6deb2`.

## What changes

- The actor reads current entity types, positions, HP, motion and a spatial
  board raster, alongside the original causal history and native tower HP.
  New branches start at zero, preserving the parent's initial policy.
- PPO trains the board branch, placement/action heads, backbone and critic in
  separate learning-rate groups. Parent KL and evaluation regression guards
  remain active. Stronger play is an experiment outcome, not guaranteed.
- Native tower-HP and estimated elixir-trade potentials shape rewards. Every
  training episode has a telescoping-reward audit. Projectile/effect entities
  without a health component retain their type/position but have zero HP
  fraction and elixir value.
- Hard matchups receive more training probability. Frozen self-play snapshots
  are limited to Hog mirrors. The production run keeps two policy snapshots
  and two engine snapshots per worker, with 15% state-bank episode sampling.
- State-bank episodes bootstrap at truncation and are excluded from win rates.
  Hog connect rate is a proxy based on proximity or subsequent tower damage,
  not exact damage attribution.

## Desktop resource limits

The trainer and all engine children share **one** systemd service: 4 GiB hard
RAM limit, 3.5 GiB memory high watermark, no swap, 200% CPU quota (two core
equivalents), low scheduling priority, and two workers. The launcher requires
6 GiB available RAM and takes a shared nonblocking compute lock. It does not
restart automatically. The service lifetime is the training budget plus three
minutes for cleanup. A STOP file requests a graceful checkpointed stop.

Synchronous batched waves are used so updates, self-play rotation and evaluation
start at game boundaries. The experimental asynchronous collector and unfinished
lookahead/search and Rust restore optimizations are outside this run. No rebuild
is needed. Do not start another training/build job alongside this service.

## Validation on 23 October 2026

- Initial pilot exposed a mismatch between native non-health entities and the
  Python HP validator. Fixed and covered by a regression test.
- Pilot B: two full matches, one PPO update, 498 learner decisions, one self-play
  snapshot, no runtime errors; approximately 2.6 GiB peak service memory.
- Pilot C: eight full matches plus four state-bank episodes, six PPO updates,
  six self-play snapshots with rotation, 42 engine snapshots captured, 12/12
  successful reward audits; approximately 3.5 GiB peak service memory.
  Its initially over-tight 3 GiB high watermark throttled restoration; the
  production watermark is 3.5 GiB with the hard cap still 4 GiB.
- Python RL suite excluding unchanged real-engine operation tests: 155 passed,
  5 skipped and 69 subtests. The existing engine-operation and full replay
  exactness results remain recorded in `opus55-engine-validation.json`.
- `tools/smoke_crazy_hog.py` exercises parent/candidate selection and confirmation
  on a deliberately tiny bank. It is integration validation, not a performance
  benchmark or strength estimate. The smoke passed all four parent/candidate
  selection/confirmation evaluations in both decode modes, exercised a real
  regression rollback, saved the selected final model, and loaded it for inference.
- Resume smoke restored update 6 and its frozen self-play opponents, completed
  update 7, rotated opponents and exited cleanly with no runtime errors. Engine
  state banks intentionally start empty after a restart.

Monitor with `python -m rl.crazy_hog_monitor --run-dir RUN_DIR`. Columns include
games, bank episodes, decisions, updates, budget percentage and remaining time.
`--dashboard` serves a local live view. `report.html` is generated automatically
on exit, with native HTML/SVG curves, tooltips, series toggles and curve playback;
regenerate it at any time with `python -m rl.crazy_hog_report --run-dir RUN_DIR`.

`models/final.pt` is the selected best checkpoint; `models/latest.pt` retains
training/resume state. Load the final model with
`BoardHogPolicy.from_checkpoint(path, device="cuda")`. Confirmation seeds are
separate from selection seeds; training matchup scores are not human ladder
performance and are not mixed with evaluation in report win-rate curves.

## Relaunch in Lab

The first three-hour attempt exited after 3m38s during selection evaluation: a
native entity extended outside arena bounds, and board encoding rejected it.
Finite positions are now clamped only for feature/raster projection; raw board
data and combat are unchanged. Regression coverage includes off-arena troops
and non-health projectiles in both perspectives. The focused suite passes 29
tests. The replacement run lives in the main checkout at
`engine/runs/banana-hog-3`, so the existing Lab discovers it without symlinks
or a server restart. Open `http://127.0.0.1:8801/#runs?id=banana-hog-3`.

## Checkpoint metadata memory experiment

Production was checkpointed at update 61 before sequential, bounded trials.
Immutable guest mappings were already shared; the host retained a duplicate
JSON allocation table after constructing its owned `GuestHeap`. The candidate
host releases that duplicate and calls `malloc_trim(0)` once at initialization,
behind `CR_TRIM_CHECKPOINT_JSON=1`. It preserves the on-disk checkpoint, guest
allocator and snapshot contents. The separately staged artifact is
`rust-engine/target/pgo-lean/rust_league_rl`; its adjacent provenance identifies
the new host and reused validated PGO library explicitly. The original engine
is preserved. Do not infer the new host source hash from embedded old library
provenance.

Trials resumed disposable copies of the same learner/optimizer/RNG checkpoint;
their weights are discarded. Eight PPO updates per trial, four-core CPU quota:

| Configuration | Full games + bank episodes | Episodes/min | Decisions/s | Peak service GiB |
| --- | ---: | ---: | ---: | ---: |
| Original, 4 workers | 27 + 5 | 21.70 | 127.85 | 5.15 |
| Metadata released, 4 workers | 27 + 5 | 22.20 | 130.81 | 3.58 |
| Metadata released, 6 workers | 41 + 7 | 24.68 | 138.61 | 4.72 |

Six workers improved episodes/min by 13.7% and decisions/s by 8.4% versus the
original four-worker trial, using 8.4% less peak RAM. These are short local
throughput measurements, not evidence of stronger play. All 80 candidate
training reward audits passed, with no new invalid games, memory-high events
or OOM events. Per-engine PSS with two snapshots fell from approximately
924 MiB to 537 MiB (42%). The initial 12-game/1,846-reply replay and 24 restore
probes matched exactly. Saved guest state was byte-identical with the feature
off/on, and identical to the old engine after normalizing only executable hash.
Full regression passed 540/540 games and 83,857 semantic replies (36 development,
252 holdout-final and 252 holdout-cycle). Production resumed from update 61 on
the validated artifact with six workers, the same 6 GiB hard/5.5 GiB high RAM
limits, zero swap and four-core CPU quota. The original 20:59:38 local deadline
was preserved. This run explicitly requires both `--engine` pointing at the
promoted artifact and `--trim-checkpoint-json` when using the launcher.

Allocator-only alternatives were rejected: glibc tuning saved no RAM; jemalloc
saved memory but slowed training, and six workers encountered an address-space
collision during startup. The production launcher therefore uses the system
allocator. Heavy experiments and compilation ran sequentially with aggregate
memory, no-swap and CPU limits while production was stopped.

Experiment evidence, trial reports and the explicit resume migration live under
the main run's `memory-experiment/` directory. The migration must preserve
`models/latest.pt` and the original absolute deadline. Engine state banks are
recreated on resume, as for previous restarts.
