# opus5.5crazyrl: Hog 2.6 RL overhaul

Goal: a much stronger **Hog 2.6** specialist, trained much faster, ready for a
fresh **3-hour** RL run. Branch `opus5.5crazyrl` integrates everything. Base:
`rl-speed/integration` (main plus incremental encoder, step-group overlap and
shared opponent forwards: about 1.9x faster, 77 tests).

## Why (evidence from banana-hog-2, 22 Oct 2026, 13,632 games)

- The actor is blind: hands, elixir, play history and tower HP only. It played
  Hog Rider about **1 per game** and Skeletons and Ice Spirit about **16 per
  game each**: degenerate elixir dumping.
- PEKKA Bridge Spam: **0.5%** win rate over 2,248 games, never improving.
- The train win rate rose about linearly from 25% to 45% and had not
  plateaued, so the run was sample-bound.
- `rust_league_rl` already reads every entity (row, side, x, y, hp, max_hp)
  through the presentation snapshot each step and discards all but towers.
- The PGO engine runs about 60-80 games/min/core, but the trainer reaches about
  33 games/min on 16 workers. Python orchestration is the ceiling.

## Workstreams and branches

| ID | Branch / worktree | Scope |
| --- | --- | --- |
| W1 engine | `opus5.5crazyrl-engine` / `opus55-engine` | Rust: board export, snapshot/restore/save/load, multi-session batched host, PGO rebuild, validation |
| W2 board | `opus5.5crazyrl-board` / `opus55-board` | Python: board features, board-aware policy, privileged critic, elixir-trade potential, distillation |
| W3 speed | `opus5.5crazyrl-speed` / `opus55-speed` | Python: asynchronous continuous collection, batched engine-pool abstraction, inference/encoder speed |
| W4 trainer | phase 2 | `crazy_hog_train`: PFSP self-play league, state-bank curriculum, rewards, monitor, report |
| W5 search | phase 2 | Snapshot lookahead / expert iteration, Hog-connect dataset |

Worktrees live in `/home/cochon/Documents/mini-project/clash-royale-ai-worktrees/`.
Each agent commits only to its own branch. The integrator merges them.

## Interface contracts (fixed: implement exactly so the parallel work merges)

### C1. Board state in engine replies (W1 produces, W2/W4 consume)

`reset` accepts `"board": true`. Once it is enabled, every `reset`, `observe` and
`step` reply includes:

```json
"board": {"tick": 1234, "entities": [
  {"id": 17, "row": "HogRider", "side": 0, "x": 8500, "y": 12000,
   "hp": 1200, "max_hp": 1408, "dx": 0, "dy": 30, "first_seen_tick": 1180}
]}
```

- `row`: native CSV row name, exactly as in the presentation snapshot. Towers are
  included (`KingTower`, `PrincessTower`). Projectiles and spells are included
  only if the snapshot already contains them.
- `x`, `y`: native integer arena coordinates, physical (not mirrored); `side`
  is 0 or 1. `dx`, `dy`: native motion delta when available, else 0.
- Without `"board": true` the replies stay byte-identical to the current ones.
- W1 also writes `engine/data/board_rows.json`: every row name observed across
  validation games, mapped to its source card where known, with its elixir cost
  and units-per-card.

### C2. Snapshots (W1 produces, W4/W5 consume)

- `{"op":"snapshot"}` returns `{"snapshot_id": int, "tick": int, "bytes": int}`
  and keeps the complete session state in process: guest memory, registers,
  allocator, RNG, session/finish bookkeeping and tower-tracking state.
- `{"op":"restore","snapshot_id":n}` returns the same payload as `observe`. It
  is exact: continuing with identical actions must produce byte-identical
  replies and audit hashes to the unbranched run.
- `{"op":"drop_snapshot","snapshot_id":n}`; `--max-snapshots N` caps memory.
- `{"op":"save_snapshot","snapshot_id":n,"path":"..."}` and
  `{"op":"load_snapshot","path":"..."}` serialize a snapshot to disk, preferably
  as a dirty-page diff against the reset checkpoint, so any worker can load it
  (needed for the state bank). Loading verifies checkpoint and binary hashes.

### C3. Multi-session host (W1 produces, W3 consumes)

`rust_league_rl --sessions N`: every request may carry `"session": i`
(default 0). The new op
`{"op":"step_many","items":[{"session":i, ...step fields...}, ...]}` steps the
listed sessions concurrently (std threads, no new crates unless already vendored
offline) and returns `{"items":[reply, ...]}` in request order. Per-session
replies equal the single-session replies.

### C4. Python engine-pool abstraction (W3 produces, W4 consumes)

`rl/engine_pool.py`: `EnginePool.request_many(list[(slot, request)]) -> list[reply]`
over two implementations: the per-process thread pool (current) and the
multi-session `step_many` host (C3). A trainer flag picks one.

### C5. Board-aware policy (W2 produces, W4/W5 consume)

`rl/board_policy.py: BoardHogPolicy`, a subclass or wrapper of
`AdaptiveHogPolicy`.
- It loads hog-2 `final.pt` with every new branch zero-initialized. With a board
  attached, outputs must be **exactly equal** to the parent at initialization
  (tested).
- Board inputs: set encoder over up to 64 entities, perspective-normalized so
  the learner is always the bottom side, plus an 18x32 (or native-tile) raster
  for a placement residual CNN.
- The privileged critic sees the board.
- `observe_board(game_key, board_json, perspective)` caches per-match state.

### C6. Rewards

Potential-based terms only, telescoping, with terminal potential 0. Every game
audits them in `reward-audit.jsonl`:
- the existing tower-HP potential;
- the elixir-trade potential
  `Phi = (own_elixir + own_board_value) - (enemy_elixir + enemy_board_value)`,
  where `board_value = sum(card_cost / units_per_card * hp / max_hp)` over
  non-tower units; it is scaled by a coefficient flag.

## Global constraints for every agent

- Work only in your own worktree and branch. Never write to the main checkout
  `/home/cochon/Documents/mini-project/clash-royale-ai` or its `engine/runs`.
  Read-only references there are fine, for example
  `engine/runs/banana-hog-2/models/final.pt` and
  `engine/runs/banana-hog-2/bin/rust_league_rl`.
- Python: `/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python`,
  run from `<worktree>/engine` (`python -m pytest rl/...`). The worktree's
  `engine/analysis` symlinks to the shared native checkpoints.
- Hardware: RTX 3050 **6 GB**, i5-13420H with 12 threads, 30 GB RAM shared with
  other parallel agents, a VM and browsers. **At most 2 engine workers per agent**
  for tests and pilots. Run anything with real engines under
  `systemd-run --user --scope -p MemoryMax=6G -p MemorySwapMax=0`. Do not start
  long runs; pilots are at most 10 minutes. GPU use is small and short.
- Exactness culture: engine changes keep semantics exact and are proved with the
  existing league regression and PGO validation tooling (`tools/league_pgo.py`,
  `rust-engine/league-validation.json`). Policy changes are proved by parity
  tests. Never fake results; report failures honestly.
- Keep existing entry points and tests passing (`python -m pytest rl/`, plus
  `cargo test` where relevant).
- Commit in small logical commits on your branch. Do not push, do not merge
  other branches, and do not rewrite history.
- Add a short doc section under `engine/docs/` for what you built.

## Phase 2 (after W1-W3 merged at `3784a41`)

Phase-1 facts that phase 2 relies on:
- Engine (W1): `rust-engine/target/pgo/rust_league_rl` sha256 `32b6deb2...`.
  The 540 saved league games replay exactly. Snapshot capture takes 0.135 s
  (352 KB of dirty pages); restore takes **0.190 s**; a disk save is 47 MB.
  `--sessions` is slower than separate processes (66 vs 173 games/min with
  random play over 8), so the per-process thread pool stays the default.
  `engine/data/board_rows.json` holds 74 rows. `native_ai_side` is not
  feasible.
- Board (W2): `rl/board_policy.py` `BoardHogPolicy`, `rl/board_rewards.py`,
  `rl/hog_distill.py`; API in `docs/board-policy.md`.
- Speed (W3): `--async-collection`, `--max-policy-lag`, `--engine-pool`,
  `rl/engine_pool.py`. Main finding: **every policy forward had batch size 1**
  at 2 workers, and inference was about two thirds of wall time. The
  throughput lever is many concurrent games with one batched forward per
  scheduling round.
- Random-play engine ceiling: 173 games/min on 8 processes. The trainer
  reaches about 36.

### C7. Search / expert-iteration interface (W5 produces, W4 consumes)

`rl/hog_search.py`:

```python
@dataclass
class SearchConfig:
    candidates: int = 6           # top-k (card, tile) from the learner, plus "wait"
    horizon_ticks: int = 120      # 6 s lookahead per candidate
    search_fraction: float = 0.05 # upper bound on the share of learner decisions searched
    temperature: float = 0.5      # softmax over candidate Q into target probabilities
    aux_coef: float = 0.5

class ExpertIteration:
    def __init__(self, config: SearchConfig, row_table): ...
    def should_search(self, observation: dict, board: dict, side: int, rng) -> bool: ...
    def search(self, pool, slot: int, policy, episode, side: int,
               observation: dict, board: dict, opponent_act=None) -> "SearchTarget | None":
        """Uses snapshot/restore on the pool slot's own engine session. It must
        leave the engine exactly in its pre-search state and must not mutate
        episode or policy caches. Candidate Q = change in (tower-HP potential +
        elixir-trade potential) over the horizon, optionally plus critic
        bootstrap. opponent_act=None means the opponent submits nothing during
        lookahead."""
    def aux_loss(self, policy, targets: list["SearchTarget"]) -> torch.Tensor:
        """KL(target || policy) over the searched card/placement/wait choices,
        differentiable through the policy and recomputed from stored inputs."""
```

W4 calls this behind `--search none|expert` (default `none`). When the module is
missing, W4 uses a stub with the same signature for its tests.
