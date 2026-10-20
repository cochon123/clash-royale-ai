# Source-bound Rust backend

This crate is an experimental static recompilation path for the recovered
Clash Royale native engine. It translates original AArch64 function bodies and
observed executable blocks into generated Rust operations, then runs those
operations on a small host-side AArch64 state model. The game routines remain
the original recovered code; the Rust is generated from that code, not a
hand-written replacement simulation. The runtime has no Unicorn fallback: a
missing function, unsupported instruction/import, or guarded out-of-scope
floating-point input returns an explicit error. Release memory accesses use
unsafe direct pointers; an invalid address can terminate the process. The
`checked-memory` feature adds mapping assertions for diagnostics.

The implementation currently targets Linux x86_64 and reserves the original
guest addresses with fixed mappings. Since the address space is process-wide,
run one engine session per process. Checkpoint pages are private mappings and
episode reset discards copy-on-write changes to restore the prepared baseline.
The backend requires the pinned recovered source at
`analysis/libg-cipher/libg.decrypted.so` and the prepared checkpoint under
`analysis/native-pipeline/oracle-export-01/stage01-attempt001/state`. It checks
both identities before execution. The source SHA-256 is
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`; the
checkpoint SHA-256 is
`66f9bf75bfde9b5e0c1f4056573079016d41576020018b9e57fca538fd1b840e`.

## Coverage and limits

The checked-in generation currently covers 1,910 recovered function bodies and runtime block ranges
(103,754 translated source instructions), including the earlier 32 fixed-scenario
rollouts and fresh Hog/Mortar constructor, card and model-action coverage.
The original C++ `__next_prime` runtime helper is included in full, bounded by
its ELF symbol and checked byte-for-byte against the checkpoint runtime image.
Larger table sizes reached in longer training exposed this coverage gap.
Nine profiled hot FDEs use bounded
forward-edge threading: generated code keeps callable entry cases for the
original blocks while threading observed forward edges through the same
translated source operations, reducing repeated PC dispatch. The generator
reports no unsupported encodings in this generation set, but that does not
mean every game code path is covered. Any call outside the generated set,
unmodeled import, or other unsupported runtime case fails explicitly. FP
operations are guarded: the supported translation path rejects non-finite and
subnormal operands and checks the applicable floating-point control state.
Exceptional result flags, all signed-zero cases, and alternate FP modes are not
fully modeled; these guards and the recorded parity checks do not establish
general AArch64 FP parity.

The earlier fixed-scenario optimized release passes the canonical full-match audit across all
12 guest-memory regions, 70 architectural registers, and the allocator snapshot;
see [`full-comparison-threaded-release.json`](../analysis/native-port/full-comparison-threaded-release.json).
All 32 recorded RL games also match every semantic protocol response: 3,874
requests, zero differences. The [replay summary](../analysis/native-port/rl-replay-threaded-release-summary.json)
binds those results to the release executable. The [arithmetic probe](../analysis/native-port/instruction-probe/report-threaded-current.json)
matches 2,368 transitions across 240 instruction signatures. Three allocator
and memory-reset unit tests pass. A further [persistent reset replay](../analysis/native-port/rl-replay-reset-sequence-release.json)
matches all 3,319 requests across 28 successive full games in one process.
[validation.json](validation.json) records source/build identities and evidence
hashes alongside the benchmark summaries. Recorded-run agreement does not establish
complete game coverage or platform equivalence.

The original Gym interface restores the same native scenario, decks, and engine
RNG state each episode; policy seeds vary the external actions. Observations
include tick, elixir, hand cards/costs, availability, and entity count. Positions
and unit/tower HP are not exposed yet. This is a headless training foundation;
arbitrary deck setup and Android port remain separate work. The separate
`rust_deck_rl` runner now calls original constructors for mirrored Hog 2.6 and
custom Mortar games with caller-selected native seeds. It rejects unsupported
asymmetric-deck requests. See [fast Banana training](../docs/fast-banana-training.md)
for the batched learner, independent worker processes, validation and commands.
The fresh-deck [validation summary](deck-validation.json) records exact replay
of ten complete games (2,491 protocol responses), full state audits for four
Hog/Mortar games, and the canonical fixed-scenario state audit. Current short
training trials measured 3.43× decision throughput; a separate identical-input
engine replay measured 2.02×. Those measurements precede the latest CPU/GPU
pipeline changes. The [compute validation](compute-validation.json) records
the current build, 21 Python checks, 1,080 semantic replay replies, 606 complete
integer-register helper comparisons, and the latest power-dependent timings.
These results cover the recorded paths and do not
establish general game or platform equivalence.

## Build and run

From the repository root, build the command-line audit runner and JSON-lines RL
server with the locked offline dependencies:

```sh
cargo build --manifest-path rust-engine/Cargo.toml --release --offline \
  --bin clash-recompiled --bin rust_rl
```

Run one canonical match and write a new state audit directory:

```sh
rust-engine/target/release/clash-recompiled --until-terminal \
  --audit-out analysis/native-port/rust-audit-run
```

The RL executable accepts the same sequential JSON-lines `reset`, `observe`,
`step`, and `close` protocol as the native reference. For the existing Gym
wrapper and benchmark harness, pass its executable explicitly:

```sh
tools/reverse-env/bin/python -m rl.benchmark --seconds 60 \
  --binary rust-engine/target/release/rust_rl \
  --out analysis/native-port/rust-benchmark.json
```

## Measured RL throughput

These earlier measurements use the fixed scenario and affordable-random
policies. The neural Hog/Mortar training measurements are recorded separately
in [fast Banana training](../docs/fast-banana-training.md).

Paired one-minute runs on the local Intel Core i5-13420H measured:

| Backend | Completed games | Elapsed seconds | Games/minute | Median warm reset |
| --- | ---: | ---: | ---: | ---: |
| Preserved Rust/Unicorn host | 11 | 60.143 | 10.97 | 1,683 ms |
| Static Rust recompilation | 30 | 60.116 | 29.94 | 1.14 ms |

That is **2.73× end-to-end RL throughput** in this sample. Both runs had zero
errors and zero truncations; unfinished episodes were excluded. The benchmarks
include first load, snapshot resets, IPC, observations, Python policies and
card actions, with 20 native ticks per decision and the same fixed scenario.
The gain includes the cheaper reset: private mapped pages are discarded and
allocator changes are undone, avoiding checkpoint JSON parsing each episode.
This is one-process throughput under desktop load, without neural-network
inference, training, or rendering. It applies to this external-policy RL loop;
the internal native bot/harness is a different workload.

The [Rust run](../analysis/native-port/benchmark-fast-60s.json) and
[slow run](../analysis/native-port/benchmark-slow-60s.json) retain timings,
per-episode outcomes, action counts and executable hashes. All 30 Rust benchmark
outcomes match their captured original references after repeated resets.

The original, slower verified Unicorn implementation is preserved in the
separate `/home/cochon/Documents/mini-project/clash-rust-slow` worktree on the
`slow-working` branch. This Rust backend is being developed on `rust-fast` in
the main worktree.

## Regenerate and audit

Generated Rust is checked in. Regeneration additionally requires the retained
coverage profiles, unwind index, Capstone environment, and checkpoint files:

```sh
tools/reverse-env/bin/python tools/native_rust_codegen.py \
  --profile analysis/native-port/profile-combined.json
cargo build --manifest-path rust-engine/Cargo.toml --release --offline
tools/reverse-env/bin/python tools/native_rust_compare.py \
  --reference analysis/native-oracle/full-match-01 \
  --candidate analysis/native-port/rust-audit-run \
  --out analysis/native-port/rust-audit-comparison.json
```

`src/generated/manifest.json` binds generated files to the source, checkpoint,
and merged profile. Audit metadata also embeds source hashes, compiler/profile
settings, and the executable hash. Large original inputs and audit dumps are
local, ignored Git inputs, shared with the preserved slow checkout.
