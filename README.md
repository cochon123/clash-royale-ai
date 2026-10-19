# Clash Rust: native engine recovery

The target is a desktop Rust port of the supplied Clash Royale build, with its
original interface and recovered battle behavior. Planned modes are human vs AI
and AI vs AI, selectable decks and controllers, and a fast headless engine for
reinforcement learning. **The recovered engine runs headlessly through both
the verified Unicorn reference and an experimental static Rust recompilation.
The playable desktop interface and complete Rust port remain unfinished.**

Work uses [morluto/rea](https://github.com/morluto/rea), JADX, Ghidra, Android
emulators and bounded ARM64 execution in Unicorn. Original APKs and libraries
remain unchanged; derived files, analysis interventions and source hashes are
recorded separately.

## Current recovery

- Decoded the original SC5 interface display trees, timeline frames, atlas
  textures and fonts. The separate [Rust UI viewer](native-ui-viewer/README.md)
  renders recovered components and plays their authored timelines. It is an
  asset viewer; native text, game bindings and battles are still pending.
- Reproduced the protection companion's native text-decryption routine, exposing
  approximately 4.86 MB of previously encrypted code. Its actual loader API now
  executes in the `libg.so` initializer probe. The exact 672-byte engine binding
  is recovered, and a [Rust decoder](docs/libg-cipher.md) decrypts all 19.2 MB of
  protected ranges. Every decoded byte matches the completed native loader.
  REA/Ghidra analysis of the actual engine code is now underway.
- Recovered 645 CSV/TOML resources and 2,458 original JSON logic/animation
  assets, with provenance. Many flow graphs belong to Merge Tactics; they do not
  establish ordinary Royale battle semantics.
- Executed all 763 original game constructors and loaded the original resource
  tables and asset database through native code. Native prefab jobs now execute
  with checkpointed ARM64 workers, stacks, TLS and allocation reuse. Full typed
  postload and all 209 original Location map loads complete in the
  [resource bootstrap investigation](docs/native-resource-bootstrap.md).
- Executed the original `Normal` AI harness to its native completion path at
  tick 1,354, with actual spawned entities and components. Replaying the same
  prepared state produces identical saved memory and architectural registers.
  The [battle ledger](docs/native-battle-loop.md) records the source-bound runs,
  native diagnostics and limits of this evidence. This is original ARM64 engine
  execution, not the finished Rust simulation.
- Added a [Rust host for the original engine](native-engine/README.md), restoring
  a hash-verified native snapshot through the Unicorn C API. Its one-update,
  100-update and complete native AI match executions match the independent
  Python adapter in every saved memory region, all 70 architectural registers
  and complete allocator state.
- Submitted an externally selected Mortar through the original card command
  path, observing native elixir spending, deployment and hand cycling. The Rust
  input path also matches an independent Python execution in complete saved
  state; invalid position and insufficient elixir are rejected. See the
  [card-input evidence](docs/native-human-card-command.md) and command-line
  reproduction in the Rust host instructions.
- Added a [minimal Gymnasium RL environment](rl/README.md) with persistent native
  stepping, card actions, partial observations and rewards from the original
  winner field. It runs the fixed recovered scenario/decks with a simple Python
  opponent. The smoke test includes a complete Python-controlled native match;
  the initial benchmark completed **12 games in 61.5 seconds**, including resets,
  observations and actions, with no native errors or truncations.
- Added a [native Rust recompilation backend](rust-engine/README.md) generated
  from pinned recovered routines, with no emulator fallback. It shares the Gym
  protocol, uses private checkpoint mappings for quick episode reset, and checks
  execution against the original reference. The current one-minute RL benchmark
  completes **30 games (29.94/minute)** versus 11 (10.97/minute) for the preserved
  slow implementation, with no errors or truncations. Development is on `rust-fast`; the
  preserved `slow-working` branch has its own checkout. See
  [branch instructions](docs/branches.md).
- Recovered the native scenario/deck fields and per-side deck construction path;
  [deck contracts](docs/native-deck-contracts.md) distinguish verified pointer
  layouts from the card-object lifetime rules still under investigation.
- Installed the unmodified APK splits in an isolated Android lab. An additional
  emulator executes ARM64 directly to investigate startup without ARM translation.

See [native recovery progress](docs/native-recovery-progress.md),
[original UI recovery](docs/native-ui-recovery.md),
[original flow/configuration recovery](docs/native-flow-logic.md), and the
[evidence ledger](docs/reverse-engineering.md).

## Earlier approximation

The root `src/`, `web/` and `run.sh` contain the earlier playable independent
simulation. Its combat rules and arena rendering are approximations. It is
retained as a prototype and is **not the requested native-engine port**.
Its run commands, AI bridge and throughput measurements are documented in
[the prototype notes](docs/approximation-prototype.md).

Desktop is the current target. A phone version and an original-game-compatible
offline server are not implemented.
