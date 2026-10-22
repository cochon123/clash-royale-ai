# Original engine oracle and recovered Rust operations

`native-oracle` is a Rust host for the supplied original ARM64 engine. It restores
a verified prepared battle snapshot and executes original engine functions in
Unicorn through its C API. Its battle behavior comes from `libg.so`; this is not
yet the Rust combat rewrite or a playable desktop client.

From the project root, execute 100 original world updates:

```sh
cargo run --manifest-path native-engine/Cargo.toml --release --offline \
  --bin native-oracle -- \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --world-steps 100
```

Use `--world-steps 0 --harness-steps N` for original bot decisions and battle
updates. Use `--world-steps 0 --until-terminal` to invoke the original
`11d6364` complete-match loop. A nonnull terminal event is reported with its
original code; that code is not a winner field. `--audit-out NEW_DIRECTORY`
writes guest memory, architectural registers and allocator state for comparison.

Play one externally selected native card, then let the original world apply it:

```sh
cargo run --manifest-path native-engine/Cargo.toml --release --offline \
  --bin native-oracle -- \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --world-steps 100 --play-card 0 0 4500 4500 --after-play-steps 100
```

The four play arguments are side (0/1), current hand slot (0–3), and two signed
native coordinates. In this prepared scenario the selection is Mortar, costing
4 elixir. The original command enters the queue, spends elixir, finishes its
deployment, cycles the hand and increases the entity count from six to seven.
This is a command-line input probe, not the human desktop interface. Placement
and elixir checks use original native code; native submission must actually
grow the queue to be reported as accepted. The original command manager owns
and frees accepted commands.

The snapshot records the engine SHA-256, every saved region, exact local Unicorn
library hash, ARM64 register IDs and external-import bindings. The host verifies
those inputs before execution. Observed memory/string, allocation, clock and
bounded integer/string `vsnprintf` imports are modeled explicitly. Unsupported
imports/conversions fail. This runner starts from a prepared native battle;
it does not reproduce the entire resource loader, Java client or Android OS.

## Verification

Independent Python and Rust adapters executing the same original calls from
the same prepared tick-2 state match after one and 100 world updates. Both
comparisons verify all 12 saved memory regions, all 70 architectural registers
and the complete allocator state. The original complete-match loop also matches
the Python reference exactly: counters `[2,2]` become `[1354,1354]` and native
terminal event code 0 is returned. The warm Rust-hosted emulation took 13.93
seconds, excluding snapshot loading; this is not Rust combat throughput. Reports:

- `analysis/native-oracle/world1-comparison.json`
- `analysis/native-oracle/world100-comparison.json`
- `analysis/native-oracle/full-match-comparison.json`
- `analysis/native-oracle/manual-card-comparison.json`

The manual-card comparison also matches all memory, registers and allocator
state against the independent Python adapter. Separate insufficient-elixir and
invalid-position runs leave the queue unchanged and allocate no command. Their
reports are `analysis/native-oracle/manual-card-slot3.json` and
`analysis/native-oracle/manual-card-invalid.json`.

Recheck the 100-update comparison with:

```sh
python3 tools/native_oracle_compare.py \
  --python-state analysis/native-pipeline/oracle-reference-world100-01/stage01-attempt001/state \
  --rust-audit analysis/native-oracle/world100-01 \
  --out analysis/native-oracle/world100-comparison.json
```

The comparison rehashes both sets of saved regions. It excludes only the opaque
Unicorn CPU-context serialization, which is not an architectural register
dump. Equality here verifies these prepared executions, not general gameplay
equivalence. The snapshot retains allocations from the earlier bump-only
bootstrap prefix; its allocation count is not a measurement of original live
game memory.

The library also contains eight small recovered deck operations with native
differential checks. Those operations and this execution host are foundations
for the port; entity/component combat logic, interactive human controls,
selectable controllers and original battle presentation still need implementation.

## Minimal RL interface

`native-rl` provides a persistent JSON-lines session (`reset`, `step`, `observe`,
`close`) using the same original engine. The [Python Gymnasium wrapper](../rl/README.md)
adds partial structured observations, externally chosen actions for both sides,
sparse native win/loss rewards and a complete-games-per-minute benchmark.
Reset reloads the original fixed prepared snapshot; this version prioritizes a
working end-to-end loop over throughput or complete observations. The initial
measurement completed 12 native games in 61.476 seconds (11.71 per minute),
including resets, actions and observations.
## Specialized native deck runner

The separate `deck-setup-probe` JSONL executable rebuilds the original `Normal`
harness using exact source card rows. Presets are `hog26` and `pixel9_mortar`
(`mortar` is an alias). The deck is mirrored on both sides unless the request
also carries `deck_preset_side1`, which supplies a distinct side-1 deck by
intercepting the second original `11d6f18` deck build. An optional u32 `seed` is
passed to the original input constructor.

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml --bin deck-setup-probe
native-engine/target/release/deck-setup-probe \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state
```

Requests are one JSON object per line:

```json
{"op":"reset","deck_preset":"hog26","seed":42,"max_ticks":7300}
{"op":"step","actions":[],"ticks":100}
{"op":"observe"}
{"op":"close"}
```

Reset reports both actual native eight-card deck lists. It updates only the
scenario configuration input vector, then invokes original input/harness
constructors `11d7968` and `11d58e0`; no battle instructions are patched.
Fresh games have a startup period during which native submission rejects
deployments, so the Banana adapter advances 100 action-free world ticks before
sampling. The specialized training loop calls only world steps after submitting
external actions; it does not call the original autonomous bot harness loop.
See [Banana run instructions](../rl/README.md#banana-deck-specialists).

`native-watch` is a separate executable with the same deck/reset/action inputs
and read-only entity snapshots in `frame`. Reset, observe and step return the
policy observation alongside the frame. `NativeSession::presentation_snapshot`
reads verified entity positions, health classes and elixir layouts directly;
host-side position history supplies movement flags. It does not call guest
functions or infer combat. The watched-match driver advances only world ticks
with external model actions. Building this executable leaves the running
`deck-setup-probe` training binary intact.
