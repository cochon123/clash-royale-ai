# Deterministic experiment interface

Build with `cargo build --release --bin headless`, then run `target/release/headless`.
The process reads one JSON object per line and flushes one JSON response per line.
There is no window, HTTP server, wall-clock pacing, or external API requirement.

```json
{"op":"reset","seed":42}
{"op":"step","actions":[{"team":"Blue","hand_index":0,"x":4500,"y":20000}],"ticks":20,"bots":["Red"]}
{"op":"observe"}
```

`reset` optionally takes `decks` as two arrays of eight distinct supported card IDs
and a `config` object with all four fields: `regulation_ms`, `overtime_ms`,
`initial_elixir_milli`, and `elixir_interval_ms`. Unknown/unsupported or duplicate
cards are rejected. Invalid reset requests preserve the previous match.

`step` applies actions in array order at the current tick, then advances 1–10000
fixed 50 ms ticks (default one). Each action independently succeeds or fails;
failure never spends elixir or changes the hand. `bots` is optional and enables
the deterministic practice policy for the named sides during those ticks. For
learning, omit `bots` and supply actions for both sides. Zero actions is a legal
wait action. An explicit seed fixes all bot choices.

Observations contain visible entities, HP, positions, readiness/cooldown counters,
elixir, hands, next cards, crowns, match time, and result. Distances and positions
are integer milli-tiles (1000 per tile). Each hand entry includes an affordable
`playable` mask and legal deployment rectangle. This mask does not discretize
positions; an external agent can choose a grid or continuous coordinates inside
the rectangle. Blue owns the bottom half, Red owns the top half. There is no
fog of war. The observation is fully informed, including both hands.

Rewards are sparse, zero sum, and given once on the transition to terminal:
`[1,-1]` for a Blue win, `[-1,1]` for a Red win, otherwise `[0,0]`. Further steps
after terminal freeze the match and return zero reward. `done` remains true.
Invalid JSON and out-of-range tick requests return `ok:false` without terminating
the process. Per-action failures appear in `action_results`.

`target/release/headless --benchmark 100` runs 100 complete bot matches and reports
CPU throughput. It includes match construction and bot decisions; it excludes
JSON serialization, rendering, and HTTP. Benchmark results depend on hardware,
compiler profile, decks, and match duration.

This is a local Rust reconstruction built from extracted data and explicit rule
approximations. It is not a validated original-game emulator. In particular,
navigation, collision, target choice, special abilities, tower spell damage,
elixir timing and tie breaking can differ. Use `data/cards.json` limitations when
selecting a training curriculum. Checkpointing the full serializable `Battle`
also preserves the hidden RNG state for exact local replay.
