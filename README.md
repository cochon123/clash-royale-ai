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

The trained checkpoint is published on Hugging Face:

```bash
hf download Cochon123/clash-royale-winner-predictor --local-dir models/winner_predictor
```
