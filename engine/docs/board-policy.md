# Board-aware Hog policy

Python side of the Hog 2.6 board contract. W1 exports the C1 JSON. This
branch turns it into tensors, a zero-init `BoardHogPolicy`, an elixir-trade
potential, and a distillation step into the board-free policy a phone can run.
No trainer is wired up here. That is W4.

The loaded actor is hog-2 `final.pt` at
`/home/cochon/Documents/mini-project/clash-royale-ai/engine/runs/banana-hog-2/models/final.pt`.
The file is read only. The hog-2 value head is kept as-is. It is not
reinitialized. A separate zero linear is the privileged critic path, so values
match `AdaptiveHogPolicy` at initialization.

## What the trainer calls

```python
from rl.board_policy import BoardHogPolicy
from rl.board_rewards import audit_elixir_trade, elixir_trade_potential, shaped_elixir_terms

policy = BoardHogPolicy(device=device)  # hog-2 weights, new branches zero
policy.observe_board(episode, reply["board"], perspective=seat)
actions = policy.act_batch(requests, deterministic=False, placement_top_k=576)
log_probs, entropy, values = policy.evaluate_actions(actions)

trade = elixir_trade_potential(
    reply["board"], seat, elixir[seat], elixir[1 - seat], policy.row_table,
)
# store trade.potential the same way tower potentials are stored
audit = audit_elixir_trade(elixir_potentials, coefficient)
```

`requests` stay `(episode, observation, side, now_seconds)`. The observation
still needs `tower_state`. The board comes from `observe_board` or from
`observation["board"]`. The acting side must equal the perspective used when
that board was encoded. A missing board raises. The copy stored on
`action.input_cache["board"]` is what PPO replays. A later `observe_board`
does not rewrite it.

Pass the `BananaEpisode` itself as `game_key`. Lookup prefers that object's
identity. The default battle id `native-self-play` is shared by every episode
that does not set one, so a battle-id key collides across matches.

`placement_top_k=576` is unchanged. `evaluate_actions` still reads
`tile_candidates` from the rollout cache and scores that fixed support.

Optimizer groups:

- `policy.board_parameters()` is the set encoder, the context residual and the
  raster CNN. Give it its own group.
- `policy.critic.parameters()` already contains the loaded value head and
  `critic.proj`. Do not also put `critic.proj` in the board group. Adam rejects
  a parameter listed twice.
- The loaded trunk, wait head and base critic stay on their existing groups.

```python
policy.save_checkpoint(path)          # format board-hog-policy-v1
BoardHogPolicy.from_checkpoint(path, device=device)
```

`critic_state` in that file is the unwrapped hog-2 value head. The row
vocabulary is stored beside it. Loading refuses a different vocabulary, which
happens if `engine/data/board_rows.json` appears after the checkpoint was
saved. Start a run with the row table you intend to keep.

Distillation, after the teacher has actually learned the board:

```sh
cd engine
../.venv/bin/python -m rl.hog_distill \
  --teacher runs/.../board.pt \
  --student-weights /home/cochon/Documents/mini-project/clash-royale-ai/engine/runs/banana-hog-2/models/final.pt \
  --rollout rollout.pt --out student.pt --steps 100 --device cuda
```

`rollout.pt` is `torch.save({"format": "hog-distill-rollout-v1", "caches": [...], "placement_slots": [...]})`.
Caches are `SampledAction.input_cache` values. The student is an
`AdaptiveHogPolicy` and ignores the board. The loss is KL(teacher || student)
on wait, masked card slots and the 576-tile placement distribution, using each
cache's temperatures. `distill_batch(teacher, student, caches, placement_slots)`
is the same step without the CLI.

## Board tensors

`rl/board_features.py` encodes C1.

Coordinates are the placement frame already used by `BananaPolicy.act`. The
arena is 18000 by 32000. `native_clash` reports `arena_cells` `[36, 64]` and
places actions on 500-unit cells, and `36 * 500 = 18000`, `64 * 500 = 32000`.
The policy's 18 by 32 tile grid covers that same rectangle, so each placement
tile is a 2 by 2 block of native cells. Raster cell `(row, col)` is tile
`row * 32 + col`. Column is `floor(x / 18000 * 32)`. Row is
`floor(y_learner / 32000 * 18)`.

Perspective matches the side swap in that act path. Side 0 keeps native
coordinates. Side 1 mirrors Y only (`ny = 1 - y / 32000`) and flips `dy`. X is
not mirrored. The learner is always the bottom side. `own` is
`entity.side == perspective`.

`encode_board(board_json, perspective, table, vocab)` returns `BoardTensors`:

- `type_ids` `[64]` long. 0 is padding. Known rows are sorted ids starting at 1.
  Any other row is `blake2s(name) % 32` in the hash buckets after the known ids.
- `features` `[64, 7]`: own, x, y, hp fraction, motion x, motion y, age.
  Motion is native `dx, dy` divided by 1000, with `dy` flipped when Y is
  mirrored. Age is `clip((tick - first_seen_tick) / 4000, 0, 1)`.
- `mask` `[64]`.
- `raster` `[8, 18, 32]`. Channels, in order: own hp, enemy hp, own elixir
  value, enemy elixir value, own tower hp, enemy tower hp, own motion
  magnitude, enemy motion magnitude. Values sum when several entities share a
  tile.
- `scalars` `[5]`: own left value, own right value, enemy left value, enemy
  right value, enemy value on the learner's half (`y < 0.5`). Left is
  `x < 0.5`. These scalars, and `own_value` / `enemy_value`, see every entity.
  Only the set is truncated.

Sort order, then the first 64: towers, then own entities, then smaller learner
Y, then X, row name, id. The key does not use input position, so a reversed
entity list encodes to the same tensors. `dropped` counts entities left out of
the set. `collate_boards` stacks a list of `BoardTensors`.

## Row costs

`load_row_table()` reads `engine/data/board_rows.json` when it exists. The
expected document is `format: board-rows-v1` with `rows` as a map or a list of
`{row, card, elixir, units_per_card, kind}` and an optional `unknown` object.
An explicit path that is missing raises. The default path, when the file is
absent, uses the fallback.

The fallback is the six decks in `engine/data/fast_league_decks.json`. Elixir
and `count` come from `cards.json` via `sc_name`. Tombstone is only in
`card_catalog.json`, so its count is 1 and its elixir is 3. A `raw_record`
that differs from `sc_name` is registered as an alias with the same cost and
the same units-per-card: `Skeleton` for Skeletons (1 elixir, 3 units),
`BarrelGoblin`, `Goblin_Stab`, `Minion`. That alias is not a spawn table. A
tombstone skeleton shares the Skeletons cost because its raw row is
`Skeleton`, and one `Goblin_Stab` is valued as one sixth of Goblin Gang.
Anything else, including spells or troops outside the six decks, is the
unknown bucket: elixir 0, so it cannot invent a trade. Towers (`KingTower`,
`PrincessTower`) are never part of board value.

Entity value is `elixir / units_per_card * hp / max_hp`.

## Policy

`BoardHogPolicy` subclasses `AdaptiveHogPolicy`.

The set encoder is one pre-norm transformer layer, width 64, 4 heads, a CLS
token, dropout 0. Its output goes through `context_residual`, a linear to
`8 + d_model` whose weight and bias are zero. The first 8 entries add to card
logits. The rest add to the fused context used for waiting and for placement.
The raster CNN is two 3x3 convolutions and a 1x1 whose weight and bias are
zero. Its output adds to the 576 tile logits inside `_placement_from_context`,
which is the path `act_batch` and `evaluate_actions` both use, including the
fixed-support PPO recompute.

`critic.proj` is a second zero linear from the same embedding onto the critic
input only. Card logits, waiting and placement do not see it.

At initialization those output layers are exact zeros, including when the
board is non-empty. Compared with `AdaptiveHogPolicy` on hog-2, actions,
log-probabilities, entropy, values, slot logits, fused context and tile logits
match with `torch.equal`. The first linear of a zero-init residual gets no
gradient until that output layer moves, same as `tower_hp_adapter`. The tests
check the output layers on the first backward, then the encoder and the first
convolution after the output weights are nonzero.

New parameters, counted on the hog-2 architecture: set encoder 39,552, context
residual 10,920, raster 3,505, critic projection 10,400. Total added 64,377.
The loaded actor (model plus wait head) is 973,371 parameters. The loaded
critic base is 13,281.

On the RTX 3050 6 GB, batch 256, 9 entities, two warmup steps and five measured
steps, cuDNN GRU backend, host memory capped at 6 GB with swap off:

| Context | Forward | Backward | Peak allocated |
| --- | --- | --- | --- |
| 1 token, the real opening encode | 15.10 ms | 17.71 ms | 165 MiB |
| 64 tokens, that token repeated | 22.21 ms | 25.97 ms | 425.6 MiB |

The repeated context is an upper bound on sequence length, not a real 64-play
history. Five samples at length 1 were 14.93, 15.01, 15.40, 15.46, 14.71 ms
forward and 17.52, 18.01, 18.04, 17.52, 17.44 ms backward.

## Elixir-trade potential

```text
Phi = (own_elixir + own_board_value) - (enemy_elixir + enemy_board_value)
```

`own_elixir` is `elixir[perspective]` in elixir points, the same units as card
cost, not the policy's 0–1 normalized global. Board value sums the entity
values above and skips towers. Unknown entities add 0 and are counted on
`ElixirTrade.unknown_entities`.

Shaping matches `banana_hog_retrain.shaped_rewards`. For a list of length
`decisions + 1`, the last measured potential is replaced by terminal Phi = 0:

```text
states = potentials[:-1] + [0]
shaped[i] = coefficient * (states[i + 1] - states[i])
sum(shaped) = -coefficient * potentials[0]
```

`audit_elixir_trade` checks that identity and, if you pass the trainer's
terms, checks those terms too. It does not add the win/loss outcome. Add the
outcome once, on the last reward, as the tower potential already does.
`coefficient` defaults to 1. W4 should expose the flag. These terms are
separate from the tower-HP potential.

## Tests

From `engine`, the suite's bare imports need the rl directory on the path.
Without it, collection fails on the existing `from banana_fast_policy import ...`
tests:

```sh
PYTHONPATH=rl ../.venv/bin/python -m pytest rl/ -q
```

This branch: 129 passed, 5 skipped, 67 subtests. The parity, gradient,
checkpoint, telescoping and synthetic KL tests are in `rl/test_board_policy.py`,
`rl/test_board_features.py`, `rl/test_board_rewards.py` and
`rl/test_hog_distill.py`.

## Not in this branch

W1's engine export and `board_rows.json` are not here. The fallback is what
runs until that file exists. W4 still has to call `observe_board`, add
`board_parameters()` to the optimizer, and write the elixir terms into
`reward-audit.jsonl`. No native games were played.
