# Branches and history

This engine was developed as the separate `clash-rust` repository and merged
into `clash-royale-ai` under `engine/` on 21 October 2026, with its full
history (`git log -- engine` shows post-merge work; `git log 0f57ee4^2` shows
the original clash-rust commits).

- `main` contains everything: the compiled Rust engine (including the
  six-deck fast league simulator and its PGO build), the original reference
  runtime, the spectator and RL training.
- `archive/clash-rust-slow-working` (tag) is the preserved slow Unicorn-only
  implementation.
- `archive/clash-rust-six-deck-accuracy` (tag) is the fast-simulator branch as
  it was before being merged into main.

These archive tags contain the old repository layout (files at the root, not
under `engine/`).

The six-deck PGO build uses the validated helper and memory implementation.
Main’s unfinished SIMD helper/exclusive-memory drafts are retained in snapshot
commit `2a5322f` (old root layout); those drafts were not part of the validated
fast build. The relocated PGO binary and all 35 pinned source files match
`analysis/rust-pgo-relocated-20261021/pgo-build.json`. Its saved holdout replay
matches all 252 games and 46,044 requests; evidence is under
`analysis/pgo-relocated-holdout-replay-20261021/`.

See [fast training](fast-banana-training.md) for the current CPU/GPU runner and
[engine validation](../rust-engine/README.md) for recovered-code coverage.
