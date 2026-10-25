# Native replay enrichment

## Full corpus experimental sweep

The follow-up run is `replay-enrichment-full-20261024`. It accounts for all
64,275 files: 50,641 native candidates and 13,634 explicit preflight skips.
44,550 candidates are limited to the prefix before their first recorded ability.
This expands beyond the six-deck validation distribution using exact policy
aliases and separator-normalized recovered catalog IDs. A name mapping is not
proof of native code coverage or combat fidelity. Unknown mappings are skipped;
uncovered native code stops the replay and is recorded as an error. No mechanics
are replaced with approximate simulator rules.

`enrichment_corpus` checks each raw source SHA-256 against the earlier audit,
packs parsed records into a 94 MB SQLite database, and stops ability-limited
prefixes before every event at the first ability tick. It retains original input
hashes and the original event count. The package has 7,282 unmapped-card skips,
6,348 invalid/incomplete-input skips, and four with no deployment before the
unsupported event. Original captures remain on the laptop.

`native_enrichment_full` schedules at most twice the worker count in flight,
retains per-file results in `results.sqlite3`, and stores complete/partial
unverified board exports in `quarantine/`. Completed entries are skipped when
resuming the identical input/code contract. Each job has a wall-time limit of
the hypothesis budget plus 90 seconds (180 seconds by default). Engine response
timeouts close the engine before the next job. The active full service now runs
on the laptop with four workers, a 400% CPU quota and a 4 GiB memory ceiling.
The home full-run service is disabled and stopped.

The move preserved 197 native attempts and 973 exported state rows. The copied
results database passed SQLite integrity checks and matched the stopped home
database SHA-256. All 180 copied gzip artifacts passed decompression checks.
The original manifest, stopped state and results database are retained under
`migration-home/` in the run directory. `operator-notes.json` records the move,
the historical artifact path mapping, and the sole manifest change: workers
increased from two to four. Input, engine binary, worker algorithm and
coordinator hashes are unchanged. Existing result records remain unchanged.

A four-worker laptop benchmark processed the same 64 candidates in 26.68
seconds, matching the home outcomes and frame counts. Its extrapolation is
about six hours for the whole native candidate set; allow roughly 6–10 hours
and use the live ETA as the workload develops. Keep the laptop plugged in and
awake. The service restarts after a failure and resumes committed results.

Clash Lab: <http://100.88.135.1:8801/#runs?id=replay-enrichment-full-20261024>.
Both Lab instances use native attempts out of native candidates for the main
progress bar. Completed input screening and its instant skips are shown in a
separate card, alongside complete sequences, partial sequences and failures.
ETA uses completed native work, excluding the cheap preflight skips. Early
estimates can vary widely with unseen card paths. All exports remain unverified
and are not automatically added to any training dataset.

```sh
tail -F ~/Documents/mini-project/clash-royale-ai/engine/runs/replay-enrichment-full-20261024/console.log
```

`clash-enrichment-full-local.service` owns this execution. The local
`execution-owner.json` makes `clash-enrichment-sync.timer` publish small full-run
status artifacts from laptop to home every 15 seconds, preserving the same
home Lab URL. The pilot still mirrors from home to laptop. The active full-run
results database and quarantined board exports stay on the laptop; home keeps
its historical results copy. A stale heartbeat indicates a connectivity or
execution interruption.

```sh
systemctl --user status clash-enrichment-full-local.service --no-pager
```

## Initial conservative pilot

The October 24 compatibility audit examined 64,275 raw files and 4,109,943
events. Only 17 files contained complete eight-card decks drawn entirely from
the 39 validated classic cards, with no recorded ability activations and with
deployment coordinates. Of these, five used a pair of the validated preset decks.
These are candidates for alignment testing, not verified simulations of the
recorded matches. The recovered build predates current cards and abilities.

The audit is `engine/runs/replay-enrichment-audit-20261024/`. Its SQLite database
records input hashes and all overlapping rejection reasons per file. Native
attempts run on `home`, under
`~/projects/clash-royale-ai/engine/runs/replay-enrichment-home-20261024/`.
Clash Lab on the server is available at
<http://100.88.135.1:8801/#runs?id=replay-enrichment-home-20261024>.
The existing laptop Lab also receives a state/results mirror every 15 seconds
through `clash-enrichment-sync.timer`; native quarantined frames stay on home.
If connectivity or the laptop stops, the home run continues independently.
An old heartbeat is shown as stale rather than continuing to imply activity.

## Method and limits

`cr_replay_pipeline.enrichment` performs the cheap compatibility gate before
any native simulation. Its SQLite results permit resuming an unchanged input
contract. Only the 17 candidate raw files, the audit, and required engine inputs
need to be transferred; neither the complete raw corpus nor the 27 GB training
cache is required on the server.

`cr_replay_pipeline.native_enrichment` tests native seeds 1–4 and tick offsets
0 and 100. These are bounded hypotheses, not recovery of the original seed or
clock. It first tries one eight-card queue consistent with the full recorded
card sequence. The original native shuffle is calibrated in a separate reset
using legal dummy deployments and observed draws, then ordinary deck-constructor
inputs are reordered for this queue hypothesis. This does not recover a unique
original hand. Native camps are inferred from each side's first non-spell deployment;
ambiguous orientation fails explicitly. Simultaneous opposite-side plays are
submitted together. Native reset, tick progression, current hand, elixir and
placement checks remain authoritative. A missing card, rejected deployment,
early terminal state or uncovered native path ends that hypothesis. Nothing
forces card availability, elixir, targets, health or the game outcome.

Each worker reuses its loaded engine across battles and hypotheses. The
`CR_TRIM_CHECKPOINT_JSON=1` option drops redundant parsed allocator metadata.
The portable source path is supplied with `CR_SOURCE_ELF`; the original pinned
source and checkpoint SHA-256 checks remain enforced.

The best accepted prefix is replayed with board export enabled, and its
pre-action boards go into `quarantine/*.json.gz`. Board export must preserve the
same accepted-prefix length and first divergence. Complete and partial sequences
are counted separately; partial files explicitly carry `alignment_complete: false`.
They remain
`verified: false` and `training_usable: false`: original seed, initial hand,
levels, evolution configuration, precise clock mapping and current-game parity
have not been established. Deployment acceptance alone is insufficient to
validate the board. No existing training cache is changed. Failed attempts
retain the first divergence and the accepted-prefix length.

An accepted native command enters the queue before execution. The initializer
waits for the observed hand to change; acceptance alone does not prove that the
card has deployed. The first local export probe yielded 51 board rows before
the native battle ended earlier than the remaining recorded deployments.

The portable build matched 770 retained reference replies over six games on
the laptop. A four-replay baseline on home, before the initializer was added,
took 41.44 seconds with one worker and 37.05 seconds with two (about 12% more
throughput). This measures the bounded failure-heavy alignment workload, not
full-game enrichment throughput or current-game accuracy.

## Historical pilot operations

The home service uses at most two workers, 180% CPU quota and a 2,300 MB cgroup
memory ceiling. It restarts after failures and skips atomic per-battle result
files already completed under the same manifest. Configuration changes require
a new run directory. The default hypothesis budget is 90 seconds per battle;
an in-flight native request can extend this by its 45-second request timeout.

Stream progress, including work done and estimated time remaining:

```sh
ssh cochonhome 'tail -F ~/projects/clash-royale-ai/engine/runs/replay-enrichment-home-20261024/console.log'
```

Inspect the service:

```sh
ssh cochonhome 'systemctl --user status clash-enrichment-home.service --no-pager'
```

Clash Lab refreshes the run details every 15 seconds and shows phase progress,
host, heartbeat, worker count, estimated time remaining, candidates, rejections,
unverified simulations and verified enrichments separately. Audit completion
does not imply simulation success. A finished pilot shows **Finished** for
execution and **Data unverified** for output quality. Broader corpus processing
still requires fidelity/coverage work. Historical pilot states that recorded
`blocked` after all attempts are recognized as finished only with a completion
timestamp and matching done/attempted/total counts; their raw metadata is retained.
New worker runs store execution completion and data validation separately.
