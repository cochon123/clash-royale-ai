# Astra's review of v8 and the next experiments

Review date: 6 September 2026. This is an implementation review and offline diagnostic, not a new trained model or a live-play evaluation. The implementation probe captured epoch 12; the matched comparison used v8's best-loss checkpoint at epoch 7. During this review, the trainer **early-stopped at epoch 14, at 20:51:50 local time**, after seven epochs without a better validation loss. I did not terminate it.

**My recommendation: stop this v8 experiment, preserve its evidence, and do not scale it up. Keep the project and the pipeline improvements. Require a corrected, small, controlled comparison before another expensive model run.**

I share the frustration behind the table. We have spent considerable effort increasing sophistication without establishing a reliable increase in playing strength. But “v8 is presently a worse checkpoint” and “the underlying idea cannot work” are different conclusions. The first is supported here. The second has not been tested properly.

I also owe the original proposal some criticism: it bundled memory, clocks, placement, and supervision changes before we had a clean baseline. The 57–61% card target in my architecture report was an ambitious experimental target, not a calibrated forecast. I would now organize that proposal into much smaller experiments. More elaborate architecture is not evidence of progress.

## What I inspected and ran

- The committed pipeline repair, `c850e14`, and the uncommitted v8 implementation, dataset/inference integration, training, resumption, reports, and monitoring.
- The current run's stage history, progress stream, first-run report, checkpoint configuration, and canonical cache.
- All **135 existing tests passed** on CPU. One scheduler-order warning comes from a test. Passing tests did not cover the semantic failures below.
- A reproducible CPU implementation audit: causal query time, hazard gradients, attention dependence, zero-time dynamics, warm-start manifests, exact sample counts, and 32 validation-prefix forwards through a trained checkpoint.
- A new CPU comparison of v4.4.1 and v8 on the same 512 diagnostic battles, using 8,136 identical indexed card targets. It took approximately **57 seconds**. No model was trained, no live game was launched, and production source was not edited in this review.

Evidence: [implementation audit](../reports/astra_v8_evidence/implementation_audit.json), [matched comparison](../reports/astra_v8_evidence/v8_matched_probe.json), [audit script](../scripts/astra_v8_implementation_audit.py), [comparison script](../scripts/astra_v8_matched_probe.py).

## What the run actually says

The quoted **48.3% test** belongs to an earlier seven-epoch report. That archived report contains infinite total loss and NaN entropy. The current trainer uses BF16 with float32 loss calculations and has finite validation losses. These are separate artifacts; do not splice their curves together or assume every field in the older report is current.

**Finalization update:** the completed run reports **48.07% test card accuracy**, **11.32% exact placement cell**, **8.42% joint card/cell**, and **1.790 s timing MAE** on 17,220 indexed targets. The test total loss is finite at **7.460**. Test evaluation reloads the best-loss weights, rather than using the epoch-14 weights. Unfortunately, the final report's validation block still describes epoch 14, and the final checkpoint metadata labels the selected weights with epoch 14; these should be separated. The completed report snapshot is preserved in the implementation evidence.

The current stage-history snapshot shows:

| Epoch | Training loss | Validation loss | Card accuracy | Exact placement cell | Placement NLL | Timing MAE |
|---|---:|---:|---:|---:|---:|---:|
| 7 | 6.077 | **7.464** | 47.66% | 10.67% | 4.698 | 1.750 s |
| 8 | 5.995 | 7.491 | 47.79% | 10.55% | 4.739 | 1.744 s |
| 9 | 5.925 | 7.494 | 47.81% | 11.06% | 4.739 | 1.740 s |
| 10 | 5.852 | 7.513 | 48.17% | 11.36% | 4.758 | 1.740 s |
| 11 | 5.779 | 7.547 | **48.67%** | 11.26% | 4.773 | 1.735 s |
| 12 | 5.708 | 7.570 | 47.93% | — | — | — |
| 13 | 5.674 | 7.569 | 48.16% | 11.09% | 4.792 | 1.731 s |
| 14 | 5.617 | 7.591 | 47.86% | 10.98% | 4.805 | 1.735 s |

It has not literally stopped updating or learning. Between epochs 7 and 11, card accuracy gained about **1.00 percentage point**, while timing improved by only **0.015 seconds**. Placement NLL worsened, and placement entropy fell from **5.32 to 5.06 bits**. That combination is consistent with increasingly concentrated predictions that are not improving their probability estimates on validation data.

This is enough to stop extending the run. It is not enough to diagnose every cause as overfitting: training uses reaction weighting and teacher-forced card conditioning, whereas validation uses predicted-card conditioning for placement. The absolute train–validation loss gap is consequently not an apples-to-apples generalization gap. Its late validation deterioration is still a real warning for the chosen evaluation procedure.

The configuration is approximately **2.397 million parameters**, 5,000 training battles, 1,071 validation battles, and 1,072 test battles. The advertised **317,036 rows are only 124,272 distinct battle/event pairs**, expanded by mirroring and reaction repetition. Mirroring is useful; it does not create new independently observed situations. The canonical cache contains 57,334 battles, so this run is also a deliberately small corpus pilot.

## A fairer checkpoint comparison

Your historical table mixes different populations, feature contracts, data quantities, training histories, and warm starts. The old timing leakage can improve card prediction too. Fixing it can lower an honest score. Sample counts alone cannot normalize any of these differences.

I therefore reevaluated the existing v4.4.1 and v8 best-loss checkpoints using the current causal encoder, their own vocabularies, the same card-only targets from both sides, and only revealed opponent cards:

| Metric | v4.4.1 | v8, epoch 7 | v8 minus v4.4.1 |
|---|---:|---:|---:|
| Correct card | **52.13%** | 46.84% | **−5.29 pp** |
| Correct placement cell | **14.44%** | 11.76% | −2.68 pp |
| Correct card and cell | **11.21%** | 8.71% | −2.50 pp |
| Human cell in top five cells | **35.39%** | 30.30% | −5.09 pp |
| Timing MAE, lower is better | 1.881 s | **1.778 s** | **−0.103 s** |

Using 2,000 paired resamples of whole battles, the 95% interval for the card difference is **−6.41 to −4.19 percentage points**. For joint card/cell accuracy it is **−3.15 to −1.80 points**. The timing difference interval is **−0.129 to −0.078 seconds**. The action-imitation regression is larger than the sampling uncertainty on this cohort.

Limitations matter: these battles were excluded from the earlier checkpoint training caches/manifests and have zero ID overlap with v8's train/validation/test manifest. However, we already used this cohort to inform architecture design; it is a diagnostic set, not a pristine final benchmark. Preprocessing changed, so this comparison has 8,136 targets rather than the earlier report's 8,201. It also does **not** equalize how the models were trained. This compares existing checkpoints under a common input contract; it does not establish an intrinsic advantage of GRUs over memory tokens. Battle resampling measures cohort uncertainty, not variation across training seeds.

## My assessment of the work

There is worthwhile engineering here. The repair commit has meaningful changes across 35 files, including named feature schemas, shared AUC computation, exact vocabulary compatibility, split persistence, stronger caches, explicit sequence-only judging, and removal of the hardcoded OCR environment import. The new trainer has atomic checkpoint replacement, optimizer/scheduler resumption, finite-precision handling, and useful work/ETA monitoring. The model remains within a sensible parameter budget.

I would keep that infrastructure. I would not approve the claim that the original data/evaluation audit is completely closed, or that Stage A of the proposed architecture is correctly implemented. The missing pieces are behavioral contracts, not missing decorative components.

| Original issue | Assessment of the present code |
|---|---|
| Split integrity | Much better: frozen manifests and required evaluation manifests. Ancestor exposure is still not enforced when an explicit different manifest is supplied. |
| Causal threat and cycle time | Future-target threat time and the opponent-event cycle-clock issue are repaired. The new observation-time path still leaves the main global state stale. |
| Canonical ability attribution | Cleaner attribution is called and unresolved-event handling is explicit. Training does not pass the metadata path, so metadata-based recovery is not actually wired into this run. Missing-coordinate abilities still receive center coordinates. |
| Judge mismatch | Sequence-only training/scoring paths and rejection of official-feature judges are good fixes. New code does not itself validate or retrain old judge artifacts. |
| Cache correctness | Source identity, preprocessing/configuration, and checkpoint-sensitive caches are major improvements. Metadata content is not fingerprinted; the trust override bypasses most checks. |
| Mirror consistency | Central transformation and asymmetric-example regression coverage are good improvements. |
| AUC ties | Shared sklearn-based implementation fixes the demonstrated error. |
| Baseline populations | Baselines now use the indexed evaluation dataset. The frequency/cycle baselines remain weak controls for judging architectural gains. |
| Vocabulary compatibility | Exact mappings and donor mapping preservation are substantial improvements. |
| Configuration/reproducibility | Fractions, schemas, stable seeds, and conditional style prose are improved. Resume provenance and capability flags still deserve stricter integration checks. |

## The architectural problems that change my recommendation

### 1. The joint clock is trained to erase the opponent

The loss is `L = (lambda_own + lambda_opponent) * delay - log(lambda_own)`. The dataset swaps perspective according to the *future target's* actor, and the loss never receives an opponent-event label. For every positive delay, `dL/dlambda_opponent = delay > 0`: minimizing this objective pushes the opponent rate downward. The module comment says competition trains the opponent rate; it trains it in the wrong direction for the intended task.

This is not hypothetical. In 32 validation prefixes from the epoch-12 checkpoint, mean opponent rate was **0.000100002**, essentially the hard floor of **0.0001**. The implied opponent-next probability averaged **0.044%**. This small probe illustrates the failure; it is not a corpus estimate.

Furthermore, live action inference still uses `expm1(out['timing'])`, clipped to 0.2–12 seconds. It does not schedule from `clock_rates`. The dashboard's “wait” hit rate measures timing error within a tolerance, not learned correctness of an explicit wait action.

References: `event_clock.py:24`, `belief_policy.py:468`, `policy_dataset.py:646`, `policy_infer.py:309`.

### 2. Observation time does not reach the memory clock

`encode_policy_sample` accepts an explicit current time, but the base global vector is still taken from the last event. The memory reads its query delay from that vector's `now` field. In a synthetic example, observation times **35.5 and 40.5 seconds** both encoded `now = 35.5`, and both produced **query_dt = 0**.

Passing current time only to threat/cycle helpers does not repair the whole observation. Elixir, rolling windows, and other time-dependent state need one consistent observation contract too. The current time-transition unit test manually edits the global time tensor; it bypasses the actual encoder/inference path and therefore misses this failure.

References: `policy_dataset.py:657`, `policy_dataset.py:666`, `belief_policy.py:323`.

### 3. Memory queries cannot select an event when there is only one key

Each memory-update attention call has one event as its sole key/value. Softmax over one key is always one. Changing the memory query therefore cannot change which information it receives. My probe changed the queries substantially and got **exactly zero change in the attention output**. Query/key gradients were approximately floating-point noise, while value-projection gradients were substantial.

This does not mean the entire network is inert. The Transformer, value projections, time updates, and later memory interactions can learn. It means the purported selective read into different memory slots is absent: all slots on the selected side receive the same event contribution. With dropout, random differences are possible; those are not learned selection.

Reference: `belief_policy.py:310`.

### 4. Zero elapsed time still changes state

The time transition adds a gated residual even when `dt = 0`. A random initialization changed a state coordinate by up to **0.666** at zero time. That scale is initialization-dependent; the identity violation follows directly from the code. A transition described as advancing state through elapsed time should satisfy `T(memory, 0) = memory`.

The implementation also rebuilds memory from the bounded prefix on every forward pass. That is acceptable for a pilot, but it does not establish persistent state beyond the context window. The auxiliary activity targets are thresholds of the same next-event delay, not the proposed future multi-event supervision. No board-grounding stage has been implemented; that was intentionally a later stage, not a defect by itself.

Reference: `belief_policy.py:97`.

### 5. Pipeline closure still matters

I reproduced an explicit new split being accepted with **11 of 15 new test battles in the donor's training set**. That is a synthetic warm-start test; this v8 was originally trained from scratch, so it is not evidence that this run has that contamination.

The training call also omits metadata, and the cache identity records its path rather than its contents. Resolved ability activations can still carry invented center positions into unmasked placement losses. Dropping unknown activations is safer than inventing a card label, but it also erases an observed action and its timing/resource implications. A canonical event can preserve “an unknown ability happened” while masking unsupported card/placement labels.

Finally, the final checkpoint writer labels best-loss weights with the last history epoch. The final report also combines the last epoch's validation metrics with the selected checkpoint's test metrics. Keep `weights_epoch`, `last_completed_epoch`, and `selected_metric` separate, and evaluate/report the selected checkpoint consistently. Do not silently resume different data/loss/configuration under one run ID merely because tensor shapes fit.

References: `policy_manifest.py:188`, `policy_train.py:818`, `winner_dataset.py:243`, `winner_dataset.py:319`, `policy_train.py:1522`.

## A concrete next plan

### Step 0 — Close this experiment cleanly

Do not add epochs, increase the corpus, tune loss weights against the test set, or warm-start a supposed v9 from this clock. Preserve the best-loss checkpoint, last checkpoint, exact code diff, configuration, manifests, source/cache identity, and logs together. Preserve a best-card checkpoint if one exists; epoch 11 cannot be reconstructed from its scalar metrics alone.

Separate the first FP16-era report from the resumed finite-loss run. Mark the current checkpoint “experimental imitation policy; joint clock invalid.” Keep v4.4.1 as the empirical reference, not as proof of live readiness. The trainer's existing early stopping has now ended training at epoch 14; let its finalization finish and do not resume this objective. This review itself did not terminate it.

### Step 1 — Fix the contracts before training

Build one observation object with battle ID, fixed agent perspective, current time, confirmed event prefix, revealed opponent information, feature schema, and provenance. Future labels belong in a separate object. Exercise exactly this object through training and deployment.

Acceptance checks should demonstrate behavior:

- Changing a future target's timestamp/card/actor cannot change a fixed observation's tensors.
- Advancing time five seconds with no event advances the model query by five seconds and consistently updates time-dependent features. `T(m,0)=m` must hold.
- Clock examples include both actors under a fixed perspective. Mirroring or swapping an entire match changes actor labels consistently.
- A toy balanced competing-event process recovers nonzero rates for both actors. Zero-time/tied events and end-of-replay censoring have explicit semantics.
- Memory reads depend on the intended memory/event information and receive meaningful gradients. Add a multi-event distractor example, not just tensor-shape checks.
- Unknown abilities remain explicit uncertain observations; unsupported card/XY targets are masked. Wire metadata into the actual CLI-to-loader path and fingerprint it.
- Any train/validation/test overlap with any warm-start ancestor is rejected, including an explicitly supplied new manifest.
- Changing preprocessing, loss semantics, vocabulary, or corpus on resume requires a new run lineage. Verify end-to-end checkpoint selection and epoch metadata.

These are cheap CPU acceptance tests. The current 135 passing tests are a starting point, not a reason to skip them.

### Step 2 — Establish a clean, strong baseline

Train a v4.4.1-shaped policy **from scratch** on the same 5,000 training battles and exact sampled examples used by the corrected candidate. Keep card/placement heads, optimizer, reaction weighting, context, opponent information, and evaluation indices identical. Call it a baseline by its configuration, not another exciting version number.

Use the already exposed validation/test artifacts as development evidence. Reserve a new, deduplicated, preferably later-time final holdout before the next comparison, checking all checkpoint ancestors. Do not consume that final holdout after every small choice. Record unique battles, distinct target decisions, augmentation repeats, and optimizer presentations separately.

Include simple frequency/cycle controls on the same indices, but make the strong trained baseline the decision-maker. Compare both equal data exposure and actual wall time: a model needing five times the compute should demonstrate enough value to justify that cost.

### Step 3 — Test the clock separately from memory

First attach a correctly supervised clock to the strong baseline. Use a fixed agent perspective and labels for **which side acts next and when**. A valid constant-rate competing-risk model is a reasonable first sanity check. A richer discrete hazard over 250 ms intervals can follow only if calibration shows the constant-rate assumption is limiting.

Account for quiet intervals and right-censoring. Same-timestamp events should be grouped or treated as interval-censored, not given arbitrary causal order by card name. Duration-aware losses can avoid materializing every quiet tick as a huge dataset. Reaction oversampling must not silently redefine the natural timing distribution.

Wire clock output into offline scheduling and test repeated queries with no new event. Report next-actor accuracy, survival calibration, clock NLL, and timing MAE against a simple empirical timing baseline. The present two rate outputs and “wait accuracy” display do not meet that standard.

### Step 4 — Test the smallest useful memory

Only after the clock works, compare a small memory variant against the same baseline. Replace one-key attention with a meaningful learned gate depending on both memory slot and event, or let memory queries attend to several causally available events. Start with 8–16 slots if that makes the experiment cheaper; 24 is not a requirement.

Use dynamics with a real zero-time identity—for example a learned decay toward a state-dependent baseline with rates held fixed between event updates. Keep event updates distinct from elapsed-time updates. Start without extra future heads or an offset decoder change.

Run aligned-memory, shuffled-memory, and frozen-time controls. If aligned memory cannot beat the controls, retire it. Do not call the tokens troops, health estimates, or a world model until independent supervision supports that interpretation.

### Step 5 — Use explicit continuation gates

The following are proposed engineering gates, not predicted gains:

| Decision | Proposed evidence required |
|---|---|
| Keep a clock variant | Both actor rates behave sensibly; better held-out survival calibration/NLL; approximately 0.10 s or larger timing MAE improvement if MAE is the chosen benefit; no material action regression. |
| Keep a memory variant | About **+1 pp card accuracy**, or **+1 pp joint card/cell accuracy** with card no worse than −0.5 pp; paired uncertainty supports the claimed benefit; aligned beats shuffled/frozen controls. |
| Fund confirmation | One clean pilot clears the predeclared gate. Then repeat baseline and candidate with two additional seeds; report all three, not the best seed. |
| Scale the corpus | The benefit survives seed variation and equal-data comparison, with acceptable memory and measured inference latency. Then test 5k → 15k battles before a full-corpus run. |
| Retire the memory branch | No repeatable advantage after one corrected implementation and one bounded ablation round. Return to the simpler baseline. |

Begin with a **three-GPU-hour ceiling** for the first baseline/clock/memory screening round, timed on the actual machine. Current v8 is roughly 4½ minutes per epoch, so a 12-epoch pilot is around 55 minutes before overhead. Faster baseline runs may cost less; this is a budget, not a throughput guarantee. Equal-exposure comparisons and wall-time comparisons must be labeled separately. Fund extra seeds only after a promising result. Do not buy another overnight run just to see whether a stalled curve wakes up.

### Step 6 — Evaluate the actual bottleneck, not only card top-1

Track card top-1/top-3 and NLL; placement conditional on the human card; greedy end-to-end card/placement; timing and actor calibration; legality; and inference latency at batch size one. A proper joint action likelihood uses the true card's conditional placement probability. Keep that separate from placement error after the model's greedy card choice. Current validation mixes those roles.

Break results down by defense windows, spells, buildings, openings, late game, repeated matchups, unseen deck combinations, and observed/unknown abilities. Include probabilities assigned to human actions rather than assuming every different action is wrong. Inspect a small fixed set of failure cases manually, selected before model comparison, with event timelines and uncertainty displayed.

The earlier human-card oracle probe suggested substantial placement uncertainty could be recovered when card choice was known. That makes a disciplined card/placement decomposition worth testing. It does not prove that search over the existing judge will select a winning action.

### Step 7 — Investigate missing observations before inventing a larger network

Our fundamental input remains deployment history. It does not directly identify which units survived, current HP, targeting, or the exact fight outcome. A better memory can exploit predictable structure, but it cannot uniquely reconstruct information the record does not contain. I cannot quantify the irreducible error from the present experiments.

Use existing recorded material first. Create a small synchronized, manually checked set of replay prefixes with actual tower HP and coarse troop presence in important regions. The earlier inventory had only ten games with HP-related material; many labels were carried forward rather than independent direct OCR observations. Split by game/session, never by adjacent frames.

Perform an **oracle information experiment**: give a compact policy accurate coarse board labels on held-out annotated games. Compare it with the same model without those labels and with shuffled labels. If meaningful board information helps, improve the observation pipeline and collect passive replay coverage before adding more model capacity. If it does not help, inspect annotation alignment and task design before building a large vision system. This can remain data-driven imitation; adding board observations does not require jumping to live RL.

If existing recordings cannot answer this question, make a specific passive-data collection plan. Live controlled games are for the later point where offline progress is credible and playing strength has become the remaining uncertainty. Neither human-style scores nor sequence-only winner judges can supply a justified win-rate forecast.

## What gains I would actually claim

Right now I claim **no playing-strength gain**. The measured v8 checkpoint loses about 5.3 card points and 2.5 joint card/cell points to the existing reference on a common diagnostic cohort, while gaining about 0.10 seconds in timing MAE.

Correctness repairs may lower scores further by removing shortcuts. Recovering the old checkpoint's performance is a useful near-term checkpoint, but it is not a numerical prediction for a new model trained from scratch. The next architecture should earn continued compute with a repeatable 1-point-scale action benefit or a clear timing/calibration benefit. We do not have evidence for a promised 60% card score, a particular live win rate, or “massive gains” from more tokens.

## Astra's rambeling

I wanted the little memory tokens to remember a battle. At the moment, twelve of them are attending a meeting where there is exactly one slide, and everyone receives the same slide. Meanwhile, the opponent clock has discovered the most optimistic possible strategy: assume the opponent will almost never do anything.

That is a wonderful personality for a garden ornament. It is less useful when a Hog Rider is crossing the bridge.

The encouraging part is that these are concrete failures. “The model feels dumb” gives us nowhere to stand. “This loss mathematically rewards deleting the opponent” gives us a line of code, a toy process, and a test that can fail honestly.

I still like the ambition. I like it more when the next experiment is small enough to embarrass the idea quickly. Give me one boring baseline, one functioning clock, and a memory module that can demonstrate why it deserves electricity. The fancy architecture can earn its cape later.
