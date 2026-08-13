# Clash Royale replay pipeline

This repository replaces the old replay collector and CSV cleaner with a
loss-aware pipeline. Its first goal is to preserve and label Hero/Champion
ability activations instead of silently dropping them.

See [the investigation report](docs/investigation.md) for the old-pipeline
audit, corpus measurements, and retraining recommendation.

## What the investigation found

RoyaleAPI replay payloads expose an ability activation with:

- a raw time tick (`data-t`);
- a side (`data-s`);
- `data-ability="1"`;
- no arena position;
- often `data-card="_invalid"`.

Older Champion events usually identify the ability in their icon URL, for
example `ability-skeleton-king.png`. New Hero events can have an empty icon.
For an unnamed event, this pipeline labels the event only when exactly one
active Hero is present in that side's battle metadata. As a legacy fallback,
a versioned Hero roster can infer the only possible Hero from the replay deck.
Anything ambiguous is quarantined.

The old pipeline discarded these events because their coordinates were
`None`. It also reconstructed decks from the first eight unique cards played,
treated those cards as the starting hand/order, contained thousands of
duplicate downloads, and augmented before a robust split. Those issues are at
least as important as the missing ability labels.

## Setup

```bash
python -m venv --system-site-packages .venv
.venv/bin/pip install --no-build-isolation -e .
```

The commands below use `cr-replays`. If it is not on your shell path, use
`.venv/bin/cr-replays` instead.

No API key is stored in this repository. Supply it at runtime:

```bash
export CR_TOKEN='...'
```

## Collection

Version 0.3 uses a durable SQLite crawl frontier. Existing replay files seed
the frontier automatically: already-browsed players are marked complete and
their previously unseen opponents are queued. Each new replay discovers more
players, so the crawl is not limited to the leaderboard's first 1,000 entries.

Start the local ingest service manually:

```bash
cr-replays serve --raw-dir data/raw --db data/collector.sqlite3
```

Load `extension/` as an unpacked Chrome extension, open the RoyaleAPI
leaderboard, and use the **CR Replay Collector** panel. The extension captures
`/data/replay` responses in the page's main JavaScript world, posts them to the
local service, and falls back to a browser download if the service is offline.

The extension maintains two reusable background player tabs. Measurements
showed that two already keep the globally paced replay endpoint saturated;
four did not increase accepted replays per minute. Excess tabs from an update
finish their current page before closing. Page loading overlaps,
but a global request lease prevents both tabs from hitting the replay endpoint
at once. The initial interval is the known-safe 2.2 seconds. After 40 clean
responses it becomes 7% faster; a rate limit immediately applies the server's
retry delay and increases the interval by 25%. A rolling 200-request window is
reported in status so isolated and sustained limiting can be distinguished. This optimizes accepted
replays per minute instead of raw request volume.

Before requesting a replay, the extension also applies a conservative DOM
prefilter. It skips a battle only when RoyaleAPI exposes two recognizable deck
containers and either side visibly contains multiple Hero forms. Unknown page
layouts are collected normally rather than risking false exclusions.

The ingest service never allows an upstream error to overwrite a valid replay.
Rate-limit and other error payloads are retained separately in
`data/quarantine/capture-errors/`. After three failed visits a player moves to
the small `manual` queue instead of making the browser automation increasingly
complex.

For a persistent user service, install the included unit:

```bash
mkdir -p ~/.config/systemd/user
cp systemd/cr-replay-collector.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now cr-replay-collector.service
```

The service survives terminal closure. It freezes safely during suspend and
continues after resume. To keep it running across a full logout, enable user
lingering once with `loginctl enable-linger "$USER"`; otherwise it starts again
at the next login. Chrome's extension workers reconnect automatically whenever
Chrome is running.

Control or monitor the collector without owning its process:

```bash
cr-replays status
cr-replays watch --interval 5
cr-replays pause --reason "checking a browser challenge"
cr-replays resume
```

Closing `watch` does not stop collection. Status reports queued, leased,
completed, and manual players; active worker leases; the current request
interval; and observed rate limits. The HTTP form remains available at
`http://127.0.0.1:8765/health`.

`watch` prints a compact timestamped table and `status` prints a detailed
dashboard. For scripts, use `cr-replays status --json` or
`cr-replays watch --json`; set `NO_COLOR=1` or pass `--no-color` when needed.

To seed a fresh database from the current corpus or explicit player tags:

```bash
cr-replays seed --input data/raw
cr-replays seed PLAYER_TAG_1 PLAYER_TAG_2
```

If RoyaleAPI presents Cloudflare or a login screen, the extension pauses new
claims and leaves the browser available for a normal human resolution. Resume
with the CLI afterward.

The old Firefox-only `filterResponseData` API is not used.

Collect official battlelog metadata for deck forms:

```bash
cr-replays discover \
  --limit 1000 \
  --output data/metadata/battles.jsonl
```

This calls the official Clash Royale leaderboard and battlelog endpoints,
deduplicates battles seen from both players, and records active Hero forms
using `evolutionLevel` plus the `heroMedium` asset.

If direct Python networking is restricted, download each player's official
battlelog JSON with an authorized HTTP client and normalize the directory:

```bash
cr-replays normalize-battlelogs \
  --input data/metadata/raw-battlelogs \
  --output data/metadata/battles.jsonl
```

## Cleaning

Clean newly captured data with official battle metadata:

```bash
cr-replays clean \
  --input data/raw \
  --metadata data/metadata/battles.jsonl \
  --output data/cleaned
```

Audit the December 2025 legacy corpus using the four Heroes available then
(Mini P.E.K.K.A, Musketeer, Giant, and Knight):

```bash
cr-replays clean \
  --input /home/cochon/Documents/ClashRoyaleAI/data/raw_replays \
  --output data/cleaned-legacy \
  --legacy-roster december-2025 \
  --audit-only \
  --report reports/legacy_audit.json
```

Outputs:

- `matches.jsonl`: one canonical record per accepted battle;
- `events.jsonl`: normalized card-play and ability events;
- `manifest.json`: quality, rejection, and attribution statistics;
- `quarantine.jsonl`: rejected battle IDs and explicit reasons.

Raw ticks are always retained. Seconds are derived as `ticks / 20`, matching
RoyaleAPI's timeline scale.

## Training recommendation

Do not retrain the old 167 MB checkpoint directly. First build a reproducible
behavior-cloning baseline with:

1. exact decks and active forms from battlelogs;
2. card plays and ability activations as separate action types;
3. deterministic battle-level deduplication and splitting;
4. online geometric augmentation after the split;
5. baselines such as most-common legal action and small GRU/Transformer;
6. validation by battle and season, not random action rows.

Replay actions alone do not contain the full live arena state (unit health,
positions, targets, projectiles). They can train an action-sequence prior, but
a strong real-time player still needs synchronized visual/game-state features.

## Realism scorer

Train a model that scores how likely a battle is a real human game versus a
legal-but-random synthetic sequence (easy / medium / hard negatives):

```bash
cr-replays train-realism --input data/raw --trees 120
cr-replays report-realism --model-dir models/realism_scorer --output-dir reports
```

This is fully offline: no live play required. The HTML report includes
per-tier separation, overlapping score histograms, a scrubbable training
curve, and a spot-the-fake quiz built from held-out battles.

## Dual-phone lab

Open a browser calibration lab when both USB phones are connected (Pixel 9 + Pixel 8 by default). Live low-latency H.264 mirrors (scrcpy-server → WebSocket → WebCodecs) with click/drag touch control, full-res screencap hand detection (auto ~1500ms after each play), and a TEST button that taps a card slot then a placement preset:

```bash
cr-replays phone-lab
# or without auto-opening a tab:
cr-replays phone-lab --no-open --port 8766

# expose v4.1 and full-data v4.2 in the controller dropdown,
# with mirrored two-pass inference enabled:
cr-replays phone-lab \
  --policy-v41 models/policy_bc_v4.1 \
  --policy-v42 models/policy_bc_v4.2_full \
  --mirror-tta

# Click 6 placement landmarks on a live screenshot (matplotlib):
# bridge left/right → my corner left/right → enemy corner left/right
cr-replays phone-lab-calibrate --phone pixel9
cr-replays phone-lab-calibrate --phone pixel8
# then restart phone-lab so TEST uses the new points
```

Requires `adb` and the phones unlocked. Calibration JSON lives in `data/phone_lab/calibrations/` (seeded from the old dual-phone unified zones; scaled per device resolution). Hand detection defaults to the previous detector at `/home/cochon/Documents/ClashRoyaleAI/models/yolo/card_detector.pt`. Set `CR_CARD_DETECTOR_MODEL` or use `--yolo-model` to override it.

The **Tower data** tab runs the complete friend-battle loop without manual
phone taps: it dismisses old result sheets, challenges `cochon` from Pixel 8,
accepts on Pixel 9, lets both selected policies play, and samples all six tower
HP labels. Each game stores `battle.json`, raw OCR in `tower_hp.jsonl`, corrected
labels in `tower_hp_relabelled.jsonl`, and a full-frame WEBP beside every sample
under `data/tower_hp_runs/`. The optional calibration button asks for the six HP
label centres on Pixel 9. The defaults match the current 1080×2424 battle UI.

The browser shows work completed, sample count, and a decreasing worst-case
ETA. The same status can be streamed in a terminal:

```bash
watch -n 2 'curl -s http://127.0.0.1:8766/api/tower-data/status | jq "{phase,work,eta_s,samples,error}"'
```

HP is weak supervision, not ground truth. Clean OCR at either dormant or
activated king-bar geometry is accepted immediately; borderline decreases need
confirmation and values may never increase. Princess destruction is inferred
only when exactly one tower on that side was damaged, its label disappears for
two samples, and the king bar activates. Ambiguous cases remain nonzero for
offline audit. Every row carries `game_phase`, `training_mask`, label source,
and per-tower `training_weight`, while every original frame is retained so
labels can be rebuilt without replaying an expensive match.

Train the detector locally with:

```bash
pip install -e '.[detector-training]'
export ROBOFLOW_API_KEY=...  # required by Roboflow dataset exports
python scripts/train_card_detector.py
```

The v3 dataset’s filename-style labels are normalized by the phone harness (`archers.png` → `archers`, `archers evoluted.png` → `archers-evo`, `giant_hero.png` → `giant`).

## Next-action policy (behavior cloning)

Train a causal policy that predicts the next deck-slot, placement zone, and
timing from action history. Evaluation stays offline: battle-level held-out
metrics, frequency/cycle baselines, and realism-scored autoregressive rollouts.

```bash
cr-replays train-policy --input data/raw --epochs 25
cr-replays train-policy --version 3 --input data/raw --epochs 25 --device cuda
cr-replays train-policy --version 4 --input data/raw --epochs 25 --device cuda
cr-replays report-policy --model-dir models/policy_bc_v4 --output-dir reports
cr-replays predict-policy data/raw/SOME_BATTLE.json --prefix-events 20
```

`policy-bc-v2` uses per-card cycle features and a card-conditioned slot head.
`policy-bc-v3` adds recent-opponent-threat features plus reaction-window
upweighting (targets GY→poison / hog→tornado style misses).
`policy-bc-v4` keeps v3 threat/reaction and jointly trains card-conditioned
zone/XY heads (offline probe finding). The HTML report includes a live-play
readiness checklist — treat it as suspect until rollout XY/initiative gates
are fixed; only smoke-test on the real client after those pass.

### Interactive showcase (v4 vs v3)

Because each training run sees a different snapshot of the growing replay
corpus, archived `report.json` numbers are not a fair head-to-head. The
showcase rescores **both frozen checkpoints on the same held-out actions** and
renders the result as an explorable page: a to-scale arena bubble map of where
each model places cards, a per-card placement league table, a gallery of real
plays v4 fixed, and a defense quiz you can play against v3 and v4.

```bash
cr-replays showcase-policy --device cuda --max-battles 700
cr-replays report-showcase   # -> reports/policy_bc_v4_showcase.html
```

`showcase-policy` writes `reports/policy_showcase_v4.json`; pass
`--new-policy-dir` / `--old-policy-dir` to compare any two checkpoints.

### Interactive reports (not copy-pasted)

Every model/experiment report has its own visual language via `report_kit.py`:

| Report | What it's for | Distinctive visual |
|---|---|---|
| `policy_bc_v{2,3,4}.html` | BC policy versions | Version-specific architecture diagram (what that version added) |
| `policy_bc_v4_showcase.html` | Fair v3↔v4 head-to-head | Arena lab + defense quiz |
| `rollout_autopsy_v1.html` | Why rollouts collapse | 27×5 ablation recovery matrix |
| `action_clock_v1.html` | Who acts next / when | Initiative conveyor + phase dial |
| `placement_probe_v1.html` | Does card identity unlock placement? | Oracle ladder on an arena |
| `hand_audit_v1.html` | Is oldest-four the bottleneck? | 8-card cycle wheel (null-result) |
| `defense_support_audit_v*.html` | Is the answer even in the data? | Per-cell support funnel |
| `defense_slice_v*.html` | Real reaction windows | Threat radar vs frequency |
| `realism_scorer_v1.html` | Sequence realism judge | Spot-the-fake quiz |
| `winner_hgb_v1.html` | Full-game winner judge | Risk–coverage dial |
| `winner_transformer_v1.html` | Prefix winner probe | Match-timeline scrubber |

### Matchup stress test

Mine win-condition matchups that favor one side in the corpus, then run the
same policy against itself on those decks and judge winners with the offline
winner model:

```bash
cr-replays eval-matchups --games 48 --top-k 6
```

Results land in `reports/matchup_eval.json`.

### Defense evals

```bash
# Real held-out reaction windows (no forced hand) — primary defense metric
cr-replays eval-defense-slice --device cuda

# Hard counterfactual probe: 1 strong + 3 weak in a cycle-consistent hand
cr-replays eval-defense --trials 96 --output reports/defense_eval_fair.json

# Data-support + natural-hand audit (decide if failing probe cells justify v3)
cr-replays audit-defense-support --device cuda
cr-replays report-defense-support
```

The support audit measures, per (threat, answer) cell, how often the answer
appears in the defender's deck/hand after the threat on train, the human-use
rate, and whether the policy picks it on test when humans naturally held it.
Unsupported cells should be dropped from success gates; only
`supported_but_model_fails` justifies policy v3 threat conditioning.

## Full-game winner model

Train the strongest current full-game model with:

```bash
cr-replays train-winner-hgb --input data/raw --trees 100
```

Despite the legacy command name, this now trains a perspective-symmetric blend
of HistGradientBoosting and Extra Trees. Every training battle is also viewed
from the opposite player's perspective, and inference averages both views. The
blend weight is selected on validation log-loss; the held-out test split is
used only for the final report.

Confidence is selected separately from the winner blend by minimizing
validation area under the risk-coverage curve. An isotonic calibrator then maps
that score to an estimated probability that the prediction is correct. This
keeps winner accuracy and selective confidence as distinct objectives.

In addition to `hgb_ensemble.pkl` and `hgb_report.json`, training writes:

- `accuracy_vs_confidence.png`, comparing the previous and improved models;
- `accuracy_vs_confidence.json`, containing both curves;
- `confidence_training_stages.json`, recording every cumulative tree stage;
- `accuracy_vs_confidence_training.mp4`, a Matplotlib animation of those stages.

## Hugging Face artifacts

```bash
# Full-game winner predictor
hf download Cochon123/clash-royale-winner-predictor --local-dir models/winner_predictor

# Next-action policy (behavior cloning v4)
hf download Cochon123/clash-royale-policy-bc-v4 --local-dir models/policy_bc_v4

# Realism scorer (real vs synthetic sequences)
hf download Cochon123/clash-royale-realism-scorer --local-dir models/realism_scorer

# Style discriminator (distinguishes policies by play style)
hf download Cochon123/clash-royale-style-discriminator --local-dir models/style_discriminator

# Replay corpus
hf download Cochon123/clash-royale-replays --repo-type dataset --local-dir data
```

Each policy BC version also has its own model repo, e.g.:

```bash
# Latest policy checkpoints (one repo per version)
hf download Cochon123/clash-royale-policy-bc-v7-pilot-aligned --local-dir models/policy_bc_v7_pilot_aligned
hf download Cochon123/clash-royale-policy-bc-v7-pilot-shuffled --local-dir models/policy_bc_v7_pilot_shuffled
hf download Cochon123/clash-royale-policy-bc-v6 --local-dir models/policy_bc_v6
hf download Cochon123/clash-royale-policy-bc-v5 --local-dir models/policy_bc_v5
hf download Cochon123/clash-royale-policy-bc-v4-3 --local-dir models/policy_bc_v4.3
```
