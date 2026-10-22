# Clash Rust / Arena Lab

A local desktop match server and deterministic Rust arena simulation, built using
data extracted from the supplied Clash Royale Android package with REA and Python.
Choose human or AI controllers for each side, edit eight-card decks, and launch
directly into a rendered battle.

**This is a playable research reconstruction, not a complete decompilation or
faithful port of the original game.** Native battle algorithms have not been
recovered. Many special mechanics are approximated or absent, and the simulation
has not been validated for transfer to real Clash Royale. The interface uses
original card art from your local package and simple locally drawn arena graphics.

## Desktop

```sh
./run.sh
```

Open **http://127.0.0.1:8788**. Rust and Cargo are required; the first build fetches
the dependencies. For another port: `./run.sh --port 8790`.

- Select **Human** on one side and a bot or policy on the other, or select two AIs.
- Edit blue/red decks card by card, or load Starter Eight, Hog 2.6, Logbait, or
  Golem Beatdown. The famous presets include approximated special cards.
- **Launch match** starts the battle immediately. Human controls: click a card
  (or press **1–4**), then click the arena. Blue deploys at the bottom, red at the top.
- **Space** pauses/resumes. Change match speed, or export the confirmed action
  replay. Finished matches also save under `runs/`.
- Launching another match from the page stops its previous match and releases its
  policy workers. Refreshing `?match=ID` reconnects to that match.

The simulation and HTTP server run in Rust. The browser only renders and submits
actions. Local PyTorch inference remains in a persistent Python subprocess per
trained AI. No official game client, account, or game server is involved.

## Your trained policies

The bridge discovers checkpoints in the repository root (`../models`) and uses
its existing Python environment (`../.venv`). Available models appear automatically in the UI:
v9 (experimental), v4.4.1, and v4.4.2. The three built-in bots work without Python
or model weights. Override the checkout with `CLASH_AI_REPO=/path/to/repo`.

```sh
python3 tools/policy_bridge.py --list
```

These deployment-history policies do not see the exact simulated troop state.
Suggestions outside a legal deployment zone are clamped to that zone; the export
records original predictions, adjustments, and rejections. New deployments
invalidate outstanding predictions. Model failures stop the match visibly;
another controller is never substituted. See [the bridge contract](ai-bridge.md).

## Fast experiments / future RL

```sh
cargo build --release --bin headless
target/release/headless --benchmark 100
target/release/headless
```

The headless process takes JSON lines:

```json
{"op":"reset","seed":42}
{"op":"step","actions":[{"team":"Blue","hand_index":0,"x":4500,"y":20000}],"ticks":20,"bots":["Red"]}
```

There is no rendering, HTTP, or real-time pacing in this mode. It provides legal
hand masks, placement bounds, observations, and sparse terminal rewards. The
serializable `Battle` preserves simulation and RNG state. Rust callers can use
the library directly for lower overhead and independent environments.

One release measurement on this machine's i5-13420H ran 100 complete default-deck
bot matches in 0.526 seconds: about **952,000 ticks/second and 190 matches/second**.
This includes match initialization and built-in bot decisions, but excludes
observation serialization, rendering, HTTP, and neural inference. It measures
throughput for the reconstructed rules, not original-game fidelity.
[Benchmark evidence](../analysis/benchmark.json) · [environment protocol](headless.md).

## Recovery and evidence

- 645 CSV/TOML resources decoded with source hashes.
- 375 raw card variants, 153 base cards catalogued; 97 generic playable combat
  implementations, with per-card support limits.
- Two tower definitions and 137 unchanged PNG card images extracted.
- REA 6.5.0 / JADX 0.7.1 inspected Android declarations and Java methods; Ghidra
  12.1.4 decompiled a native initializer thunk. Full native battle source remains
  unresolved.

Recreate the exports from the supplied XAPK using:

```sh
python3 tools/unpack_xapk.py
python3 tools/extract_game_data.py
```

See [data schemas and assumptions](../data/README.md) and the
[reverse-engineering ledger](reverse-engineering.md). The source XAPK,
extracted APKs, analysis tools, and build output are excluded by `.gitignore`.

To repeat the static REA queries (requires Node and a full JDK 21+):

```sh
python3 tools/bootstrap_analysis.py
python3 tools/analyze.py --native
```

These install project-local tools and analyze files; they do not launch the game
or change your agent's MCP configuration.

## Verification

```sh
cargo test
python3 tools/test_policy_bridge.py
python3 tools/test_server.py --models
```

The tests cover deterministic matches/checkpoints, action validation, deployment,
targeting, air/ground attacks, bridge routing, king activation, outcomes, trained
policy contracts, and the live HTTP match flow. The model smoke test requires
your neighboring checkpoints and environment.

Desktop is the current target. Android packaging and an official-game-compatible
server are not implemented.
