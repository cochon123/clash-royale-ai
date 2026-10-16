# v9 overnight implementation — 11–12 October 2026

Authorized objective: implement the v9 plan and use the available machine until
06:00 America/Edmonton on 12 October. Stop training at 05:15 to reserve final
evaluation/report time. No commit or push was requested. Preserve the existing
dirty working tree and historical artifacts.

## Runtime

Actual hardware: RTX 3050 Laptop, 6 GB VRAM; about 30 GiB RAM. The old AGENTS.md
hardware description differs. Qwen ASR and ComfyUI were running initially and
were stopped with user authorization. Qwen automatically restarted once, so it
is temporarily masked with `systemctl --user mask --runtime`. The suite's
`finally` block and an independent one-shot 06:00 timer restore both services.

Units:

- `cr-v9-cache.service`: completed, builds immutable observation shards.
- `cr-v9-train.service`: completed 5k pilot, four epochs.
- `cr-v9-scale15k.service`: four-epoch 15k pilot.
- `cr-v9-advantage.service`: CPU cross-fitted prefix values and weight sidecars.
- `cr-v9-suite.service`: serial GPU queue; waits for existing pilots.
- `cr-v9-dashboard.service`: http://127.0.0.1:8766
- `cr-v9-report-server.service`: http://127.0.0.1:8767/policy_v9.html
- `cr-v9-deadline.timer`: one-shot 2026-10-12 06:00 America/Edmonton.

Commands to stream work done and ETA:

```sh
tail -f logs/v9_suite.log
```

```sh
.venv/bin/cr-replays watch-policy-training \
  --progress models/policy_v9/progress.jsonl --log logs/v9_suite.log \
  --port 8768
```

The original cache/preliminary trainer logs are `logs/v9_cache.log` and
`logs/v9_train.log`. Value preprocessing uses `logs/v9_advantage.log`.
`models/policy_v9/status.json` records the suite phase. Never start another GPU
job without checking these units and `nvidia-smi` first.

T3 scheduler ID (30-minute supervision, first run 22:28 MDT):

`scheduled-task:command:mcp:bf7db261-8dd2-4ddf-9847-00d7e2c2f237:schedule-task:v9-overnight-monitor-20261011`

Disable this task once final verification is complete or at the deadline. Its
prompt tells future turns to inspect this file, repair failures, avoid duplicate
jobs, preserve the holdout, restore resident services and report evidence.

## Data and causal contract

`data/v9_cache/` is 27 GB, containing 55,807 decisive training battles and
2,988,776 unique card-play actions and 3,581,980 clock-interval rows.
Clock rows describe next-event intervals at confirmed event boundaries; they are
shuffled uniformly by row, not sampled uniformly through quiet wall-clock time.
There is no 40-action cap and no repeated encoding each epoch. Unknown ability
activations remain position-unknown observations, never card/placement targets.
The policy observes its last 64 confirmed events.

The fixed dev cohort is the intersection of the final v4.4.1 and v4.4.2
validation/test manifests (524 before decisive-result filtering). Older legacy
warm-start ancestry is incompletely recorded; this cohort is not advertised as
unseen by every ancestor. The original 1,000-battle final holdout is authoritative.
Training excludes its IDs and perspective-reversed card-event fingerprints.
The cache has frozen metadata and costs copies, a train-only vocabulary, a
manifest, source/metadata/cost hashes and stored battle objects.

`PrefixBank` encodes each battle once per perspective, then recomputes prefix-
specific masked opponent deck statistics. Exact fp32 parity passed on 100 real
battles; every shard independently checks two battles against the canonical
encoder. Event features use fp16 storage; globals/cycle features/labels remain
fp32. Storage parity tests allow fp16 rounding only. The dense unidirectional
GRU matched packed inference to <1e-6 and permits the faster cuDNN path.

Mirroring reflects event X, target X, threat X and known left/right fractions.
Unknown-position abilities do not become left-lane observations after reflection.
The heatmap is **32 columns × 18 rows**, while the arena is 18 × 32 physical
tiles. The physical one-tile tolerance metric must use arena distances.
The first 5k pilot started before this metric correction; re-score its selected
checkpoint and label/remove its old bin-based tolerance curve, without changing
training evidence. Later runs use the corrected physical metric.

## Implemented experiments

`v9_train.py` uses batched memory-map reads, two loader workers, batches of 512,
fp32/TF32, AdamW and teacher-forced selected-card placement. The clock is sampled
once every four action batches with its contribution multiplied by four, so its
mean nominal loss weight remains 0.1. Main policy objective: card CE + cell CE
+ 0.25 log-delay Huber. Joint **discrete** NLL is card + true-card cell NLL;
timing MAE and clock likelihood are separate. Reports must not call Huber a
proper timing likelihood. Validation has no reaction/winner weights or mirrors.

Measured GPU-only action training: ~9,500 actions/s, ~840 MiB peak allocation.
5k end-to-end, including loading/clock/validation: ~5,100 actions/s (old controlled
trainer ~550). Record actual full-run rates rather than promising 10k end-to-end.

Queue: 5k and 15k four-epoch scaling runs; full data **twelve epochs**; d=256 two-epoch
matched exposure probe; d=256 continues only if quality and remaining time gates
pass. Then two-epoch fresh-optimizer continuations from the same selected donor:
plain control, winner weight 3, Gaussian sigma .45 cells, cross-fitted value-change
weights if the value calibration gate passes, aligned ghost, shuffled ghost.
Combine winner+smoothing only if both clear their gates. Gate at the last shared
completed epoch: joint NLL improves >= .01 with card/joint regression <= .005.
Ghost must also beat the shuffled architecture's battle-bootstrap upper CI.
Select lowest validation joint NLL among eligible candidates; donor remains
eligible if continuations regress. Reproduce the selected recipe with seeds 43/44.

The value predictor is a 3-fold HGB trained on at most two causal prefixes per
training battle. All action weights use a model fitted on other whole battles.
Weights = clipped exp(3 * (V_after − V_before)), in [.25,4]. After-state and
eventual winner are **labels/weights**, never policy inputs. The value change
includes simultaneous events and is a heuristic, not a causal advantage.

Ghost stats are downloaded from RoyaleAPI/cr-api-data's client-data JSONs in
`data/v9_card_stats/`. Known troops use motion toward bridges/towers, air moves
directly, buildings remain still. Speed/60 tiles per second and troop survival
decay are explicit assumptions. Combat, targeting changes and deaths are absent;
the source stats are older and are not represented as current season ground truth.
The CNN and GRU train end-to-end; no frozen-trunk adapter claim is made.

## Final evaluation and reports

`final_selection.json` must freeze every candidate/reference checkpoint hash and
the recipe **before** final scoring. `v9_evaluate.py` uses an exclusive lock and
durable transaction bound to that selection. Completed checkpoints are never
rescored; interrupted scoring may resume the same frozen selection. Final
cohort preparation now persists a build contract bound to the frozen selection,
reuses completed shards after header/shape validation, and rebuilds unfinished
generated shards. Completed model scores remain immutable.

Predeclared meaningfully-better gate: at least +2 pp card and +2 pp joint
card+cell against v4.4.1 in **each of three seeds**, on the original untouched
holdout. Whole-battle bootstrap intervals are additional evidence, not seed
uncertainty. No actual win rate follows from these metrics.

Every policy run gets `best.pt`, `last.pt`, `report.json`, `report.html`, per-epoch
rows, progress, source.zip and environment/config/contract files. Native HTML/SVG
reports have curve toggles, hover details, playback, definitions and provenance.
Root HTML is `reports/policy_v9.html`. Improve final holdout visibility in the
report and inspect it with the T3 collaborative browser before delivery.

## Data/API/live-play limitations and pending work

Collector queue was resumed; status is unpaused but there are no active browser
workers. Do not claim new replays until the count actually rises. 100k+ battles
across two seasons cannot be achieved by extrapolating a resumed queue.

User supplied `/home/cochon/Downloads/Pasted image (10).png` containing a key.
Its allowed IP (96.51.237.85) matches this computer's public IP. OCR + manual
correction yielded valid JWT structure and UUIDs; all tested variants still got
HTTP 403 `Invalid authorization`. A private temporary token file is at
`/run/user/1000/clash-royale-v9-token`, mode 0600. No key is in the repo or logs.
An async question asks for a text-file path/environment variable. Use any reply
as steering and finish metadata recovery without stopping training. The original
data/metadata file is empty. API now supports CR_TOKEN_FILE/--token-file and
preserves existing metadata when discovery yields zero usable records.
If metadata is recovered, do not change the frozen overnight data contract.
Older July/August battlelogs may no longer be obtainable from recent official
logs; do not promise retrospective recovery.

**Resolved at 22:36:** user supplied `/home/cochon/Downloads/Pasted image (11).png`
with a fresh key. Manual transcription passed JWT checks and `/v1/cards` returned
HTTP 200 (123 cards). The verified key is now in the same private runtime file,
0600. `cr-v9-metadata.service` runs official discovery (1,000 players, 8 workers)
to `data/metadata/battles.jsonl`; log is `logs/v9_metadata.log`. Use the verified
key without another permission question. Keep new metadata separate from this
run's frozen `data/v9_cache/metadata.jsonl`, which remains unchanged.

The default phone hand detector path
`/home/cochon/Documents/ClashRoyaleAI/models/yolo/card_detector.pt` is missing.
No actual card detector checkpoint has yet been located in this repo. The default
now points to the training script's repo-local output and impossible repeated
identities across >=3 slots are rejected as unknown. This prevents all-snowball
outputs from becoming execution identities; it does not supply a trained detector.
`adb devices -l` found no connected phones. Detector accuracy still needs diagnosis
against recorded frames and a valid checkpoint.
Board-oracle frames have unverified alignment/labels; never fill them with guesses
and call them verified. Live 20-game checks are conditional on a promoted candidate
and a working detector; no phone game has been started in this run.

The sequence-only HGB/ExtraTrees judge completed as `cr-v9-judge.service`
(`logs/v9_judge.log`). It uses the first 15k training battles and separate halves
of development for calibration and judge test, never the final policy holdout.
The offline diagnostic compares v9/v4.4.1/v4.4.2 with matched real openings and
shared schedules/decoders. Judge scores are not real wins and do not affect
promotion. The waiting postprocessing supervisor, prefix-value curves, pilot
video and inference/export integration are implemented. Pending: selected-recipe
three-seed reproduction, the single final holdout, final tournament and artifact,
and final HTML/browser verification.

## Verification so far

166 tests passed, including seven new meaningful v9 tests: fast/canonical encoder
and future invariance, unlimited actions + fp16 storage parity, full reflection
with unknown abilities, label-weight separation, packed/dense inference parity,
ghost unknown-position masking, normalized smoothing and actual checkpoint/resume/
native-report flow. Two existing warning categories were present (BrokenPipe test
finalizer; scheduler-before-optimizer test). Re-run targeted tests after additional
source changes. Do not broadly repeat unchanged tests without a reason.

Pilot repair: first 15k attempt OOM-killed after one epoch because Python 3.14
forkserver pickled memmaps as data. Preserved under
`models/policy_v9/archive/scale_15k_before_worker_fix_seed42/`. Workers now pickle
filenames only and reopen maps. Legacy `file_system` sharing retained forkserver
shutdown pipes via shm managers; v9 uses `file_descriptor` sharing in both
parent/workers, nonpersistent workers and explicit clock-worker shutdown. Nine
v9 tests passed with clean process exit; 44 targeted v9/integrity/metadata/phone
tests passed. Full-cache worker pickle is ~456 KB rather than gigabytes.
Restarted 15k completed four epochs at ~7,600 actions/s with stable memory. Full
corpus runs near 7,900 actions/s. v4.4.1 fixed dev: card .5299, joint .1133, NLL
4.4071; v4.4.2: card .5299, joint .1166, NLL 4.4231. Prefix-value gate passed
weakly: dev binary NLL .6873 vs constant .6931, AUC .548; do not overstate it.

At 22:26, base seed42 was safely stopped after a complete first checkpoint and
resumed with the same source/config/optimizer/RNG to extend nominal base training
from 8 to 12 epochs. The controller runs with `--base-epochs 12`; repeats use 12.
Partial epoch2 progress was discarded. Loss/LR schedule and causal cache did not
change. Source hashes reject semantic changes on resume.

## 23:08 follow-up

- Official discovery completed: 27,020 records, zero failures; verified secret
  remains `/run/user/1000/clash-royale-v9-token`, mode 0600. Do not print it.
- New metadata may contain older logs for inactive players (earliest 6 August),
  so do not claim all records are current or that zero legacy matches exist.
  `models/policy_v9/metadata_match_audit.json` is being prepared by a read-only
  CPU scan; matches are candidates, not proof of exact battle identity.
  Completed audit of all 64,275 files found **zero** matches: 64,264 not found,
  nine rejected by deck/time checks, two parse failures. The new official logs
  did not recover old ability identities; preserve their unknown observations.
- `MetadataIndex` rejects incompatible eight-card rosters and official times
  newer than replay capture mtime + five minutes. Matching version is included
  in subsequent dataset cache identities. Frozen overnight tensors are unchanged.
- Seeded 11,519 distinct player tags from discovery; 9,053 were new queued tags.
  Raw deployment count is still 64,275. T3 `tab_3` holds a valid RoyaleAPI player
  page, whose replay response says **login is required**. A user login request
  is pending. No anti-bot or login bypass was attempted. The extension worker
  still needs an authenticated session; official API cannot supply deployments.
- Sequence judge completed: independent development test accuracy .7557,
  AUC .8450, NLL .4748. These are real-replay outcome-prediction results, not
  generated-policy win rates. Final diagnostics are never a promotion gate.
- `cr-v9-postprocess.service` was restarted to load the latest report/template
  and deadline guard. It waits for the completed holdout transaction, exports
  preselected seed42, runs the diagnostic tournament on **CPU** to avoid racing
  restored GPU services, then writes final native HTML. See its log.
- `v9_infer.py` passed causal/future-invariance, hand-mask, coordinate/time tests.
  A real stable 15k checkpoint plus fitted judge passed a CPU rollout smoke check
  (`inference_smoke.json`). Thin export/inference load also passed with a false
  promotion gate preserved. No phone execution was attempted.
- The 15k four-epoch curve replay video was captured in T3 and reviewed at start,
  middle and end; FFmpeg output is
  `/home/cochon/Videos/remotion/clash-royale-v9-curves/pilot-curves.mp4`.
  `models/policy_v9/pilot_curves.mp4` is embedded into the self-contained suite
  report on regeneration. Native browser playback/toggles passed. The video
  shows the pilot, not the final selected model.
- 23 targeted tests passed; 18 v9/integrity tests passed after adding final-cache
  crash/reuse checks. Full suite: **177 passed**, the same two preexisting warning
  categories. No additional broad repeat is needed without another source change.
- Base seed42 is healthy around epoch7/12 at ~7,700 actions/s, 1.7 GiB used VRAM.
  Selected epoch7 dev: card .54636, joint .12041, NLL 4.26129. Final evidence is
  still pending; do not declare promotion from these development scores.
  Report regeneration after completion must use current `v9_report.py`; the
  long-lived controller imported an older template, which postprocessing replaces.
- Suite HTML now exposes auxiliary predictor checks, ablation gates and the
  sequence diagnostic as visible panels. Current auxiliary metrics, native SVG
  rendering, video playback and toggles passed browser inspection. Final panels
  still require inspection when real final results become available.
- The 30-minute supervisor is enabled; next run 23:28 MDT. Keep it enabled until
  final artifact/report QA is finished, then disable it. The independent hard
  stop still fires at 06:00 MDT; stop training by 05:15 for the evaluation window.

## 23:14 supervisor check

The suite and postprocessor are active; only one GPU trainer is present. Base
seed42 is in epoch9/12 with roughly 25 minutes left, and 238 GB disk space remains.
Latest completed epoch8: card .54757, joint .12267, joint NLL 4.23894. Every hashed
training source still matches its saved contract. The final selection and holdout
transaction have not begun.

Added a CPU integration check of all six continuation paths (plain, winner,
smoothing, value weights, aligned ghost, shuffled ghost). Each loaded the same
donor and began a fresh optimizer; all passed. Clock-head optimizer steps are
lower because clock batches are sampled, while action-trunk steps restart at
zero. This supplements the earlier 177-test full run. No production training
source was changed and no extra GPU job was launched. Suite HTML was regenerated
from completed evidence; final QA remains pending. Next scheduled check: 23:28.

## 23:18 final-scoring repair

Found an alias edge case: when the base donor remains the selected seed42
candidate, final entries include the same checkpoint under both control and
candidate labels. `holdout_pass` now reuses its first metrics/row artifact for
the second label, recording `scored_as`, rather than running that checkpoint
again. Reuse requires the identical SHA and decoder type. A synthetic fixture
verified five actual scoring calls for six entries, identical aliased NPZs,
and read-only behavior after completion; both targeted checks passed. The real
holdout remains untouched. The evaluation subprocess will load the repair;
no training source or active trainer was changed.

Also reject a candidate/control report marked running, failed or budget-stopped
before starting final cohort preparation/scoring. This prevents a deadline-
truncated final seed from being described as a completed recipe reproduction.
The rejection preserves the holdout transaction as unstarted. Completed early
stopping remains eligible under the common training procedure.

## 03:35 recovery — narrower evidence, no promotion

The suite crashed at 00:32 starting the advantage trial: `np.memmap.filename`
was absolute, but its digest code called `relative_to` with a relative cache
path. The failed run wrote no checkpoint or report. Completed base/control/
winner/smoothing artifacts are preserved. The training source repair resolves
the cache root for this path-only digest; losses, optimizer, observations and
metrics are unchanged. New source hashes/archive record the repair. Resident
ASR/ComfyUI had been restored by the failure handler; both were stopped again
under the existing authorization, with ASR runtime-masked.

At 03:35, two more full 12-epoch bases plus continuations no longer fit before
06:00. Restarted the serial suite with `--continuation-seeds-only` and an absolute
cache path. Remaining advantage/aligned/shuffled ablations still run; selection
still uses development only. Seeds43/44 then repeat the selected two-epoch
continuation with fresh RNG/optimizers from the **same seed42 donor**. This is
conditional continuation variability, not independent full-training variability.
The original predeclared gate now explicitly requires independent full training;
shared-donor recovery always sets `pass_gate=false`, even if all accuracy gains
exceed +2 pp. Never describe this recovery as successful full three-seed
reproduction or promote it. A synthetic check verifies that distinction.

Base seed42 completed12: dev card .55851, joint .13027, NLL4.16039. Plain
continuation: .56211/.13256/4.13395. Winner weighting failed its matched NLL gate
(+0.00360); smoothing failed (+0.16818). d256 met the quality gate but failed
the time gate. Real holdout still unstarted. The hard 06:00 timer remains active;
suite training still stops at05:15. The restart uses the latest report template.

Recovery verification: **183 tests passed**, same two preexisting warnings.
The continuation integration test now uses a relative cache path and covers the
original failure. A clearly marked synthetic HTML fixture verified the final
table, ablation/diagnostic panels, and the explicit shared-donor/nonpromotion
message; the fixture was removed and the browser returned to the real report.
The postprocessor was restarted to load this current template and include
evaluation limitations in the exported manifest.

`cr-v9-recovery.timer` now checks service failure every60s without touching a
running queue. `v9_watchdog.py` can restart the saved suite at most twice. After
05:15 it permits only an already-frozen final evaluation, never more training;
it stops itself by05:58. The independent hard deadline also stops that timer.
Watchdog checks passed (no duplicate active run, bounded retries, cutoff).
On a future suite exception the status JSON now records `failed`, not a stale
`running` phase. Stop the recovery timer together with the T3 monitor once final
QA is complete. The failed pre-repair suite trace is retained in the log.

## 04:02 check

Recovery is healthy, no watchdog restart has been needed. Advantage completed
but failed the matched NLL gate (+.00412 vs plain control). Aligned ghost also
completed and failed the required -.01 NLL improvement (about -.006), despite
small joint-accuracy gain. Shuffled ghost is now running at about6,400 actions/s;
then the selected eligible continuation gets conditional seeds43/44. All current
ablation decisions were verified in the native HTML table. No final selection
or holdout transaction exists yet. Remaining queue should fit before05:15 using
the measured times; check again at04:28. Do not change the conditional recovery
scope or claim independent full-training evidence.

## 04:37 final-comparison preparation

Selected recipe is the plain d160 two-epoch continuation, without winner,
smoothing, value or ghost options. Seed43 completed: dev card .56267, joint
.13287, NLL4.13274. Seed44 is in its last epoch; final selection/holdout are still
unstarted. Aligned vs shuffled ghost diagnostic NLL difference is -.00775;
equal-battle CI[-.01099,-.00677]. This does not override its failed -.01 improvement
gate against plain control. Saved in `motion_control_comparison.json`.

Found and fixed a legacy input-adapter mismatch before real final scoring:
v9 repurposes the old zero-valued `pad` feature as a position-unknown flag.
`LegacyScoringPolicy` in `v9_evaluate.py` zeros only that column for old models,
preserving neutral unknown coordinates, input immutability and their3 thinking
steps. `v9_royale.py` also uses the corrected adapter. Active v9 training source
and tensors were **not** changed. Three targeted adapter/holdout checks passed.
Old development reference rows/JSON were archived under
`archive/pre_legacy_pad_adapter/`; corrected references were scored on CPU and
saved. v4.4.1 dev: card .53009, joint .11420, NLL4.40629; v4.4.2: .52974/.11621/
4.42090. The final GPU comparison will use the same corrected adapter and
records `legacy-unused-pad-zero-1`. Candidate selection was unchanged because
all loss/architecture gates compare v9 runs with the plain v9 control.

The waiting postprocessor was restarted after the adapter addition, avoiding a
stale imported module when it later loads the offline diagnostic. Both modules
imported successfully. Final QA must verify all final seed deltas, shared-donor
wording, exported checkpoint equality and inference, the diagnostic disclaimer,
resident service restoration, and then disable the T3/recovery timers.

## 05:05 final outcome and shutdown

The recovered queue finished all 12 policy jobs at 04:43 MDT; final
postprocessing finished at 04:44. No further GPU training is active. Selection
was frozen at 04:42 before the one final comparison on 1,000 battles / 52,329
actions. The completed holdout ledger remains read-only; do not re-score it or
choose a different seed using these results.

| Final holdout | Card accuracy | Joint card + cell | Joint discrete NLL |
| --- | ---: | ---: | ---: |
| v4.4.1 | 53.21% | 11.21% | 4.412 |
| v4.4.2 | 53.05% | 11.30% | 4.425 |
| v9 seed42, preselected export | 56.23% | 12.76% | 4.125 |
| v9 conditional seed43 | 56.11% | 12.76% | 4.124 |
| v9 conditional seed44 | 56.25% | 12.83% | 4.125 |

Seed42 gains +3.02 percentage points on cards and +1.55 points jointly over
v4.4.1. Its equal-battle paired 95% intervals are +2.66 to +3.41 points on cards,
+1.27 to +1.82 points jointly, and -0.302 to -0.277 on joint NLL. The scoreboard
weights actions equally; these intervals resample and weight whole battles.
They measure battle sampling uncertainty, not independent training variability.

**The original promotion gate failed.** All joint gains (+1.55 to +1.62 points)
missed the required +2 points. Independent full-training reproduction also
remains incomplete: seeds43/44 are fresh two-epoch continuations of the same
seed42 base. The artifact is experimental and explicitly `promoted=false`.
Winner, smoothing, advantage and ghost changes did not pass their matched
plain-continuation gates; the preselected recipe has none of those options.

Export: `models/policy_v9/final_model/best.pt`, with costs, manifest, inference
example and verification record. Exact state-tensor equality with the selected
source, source/export checksums, removal of optimizer/RNG state and CPU causal
inference passed. The source selection hash remains
`1a658f5e49da2ce2502b30ab40a798826edb00572443ee2f4919633162a9d749`.

The 192-history offline sequence diagnostic showed no clear v9 advantage. Its
generated histories have no combat, health, deaths or crown outcomes. The
judge probabilities and preference fractions are **not actual win rates**.
No live games were played. The 100k-battle / multi-season collection target,
independent full training seeds, and detector/live validation remain unfinished.
The newly collected official metadata did not match the old replay corpus.

Final HTML: `reports/policy_v9.html` (also `models/policy_v9/report.html`). Browser
QA verified six final rows, three paired confidence interval rows, five ablation
decisions, explicit gate/scope limitations, toggles, tooltip, replay and the
embedded four-epoch **15k pilot** video. The final report/resume test passed;
the prior full suite had 183 passing tests, followed by three targeted
legacy-adapter/holdout checks. `git diff --check` passed. Details and screenshot
are in `models/policy_v9/final_qa.json`; `needs_browser_qa=false` is recorded.

Qwen ASR and ComfyUI were restored by the successful suite and are active;
Qwen's temporary runtime mask is gone. The existing replay collector remains
active. Following completed final QA, `cr-v9-recovery.timer` and
`cr-v9-deadline.timer` were stopped, and the saved T3 recurring task above was
disabled (`enabled=false`, `nextRunAt=null`). Cleanup completed before 06:00;
there is no reason to restart training against this consumed final holdout.
`cleanup.json` records this shutdown. Report/dashboard servers remain available
on ports8767/8766. No commit or push was performed.
