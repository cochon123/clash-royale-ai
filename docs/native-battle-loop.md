# Original native battle loop and initialization

The original ARM64 battle harness now executes a complete native AI battle in the offline execution adapter. The same prepared state replays with identical saved memory, architectural registers, allocator state and import counts. This ledger records the recovered code route and its execution evidence; the original binary is still executing in Unicorn, and its battle logic has not yet been rewritten in Rust.

All addresses below are RVAs, without Ghidra's `0x100000` image base. Input ELF: `analysis/libg-cipher/libg.decrypted.so`, SHA-256 `79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`. Every export uses original FDE bounds and the focused `-noanalysis` workflow documented in [native-focused.md](native-focused.md). The output root is `analysis/native-focused/engine-step/`; each subdirectory has source/function hashes, coverage, original assembly, and pseudocode in `manifest.json`.

## Native context comes first

The exact context factory is `e160c0()`. Its 44-byte original assembly allocates `0x10a8` bytes through operator new (`187d2b0`), calls `e15534(context)`, and returns the pointer in `X0`. It takes no input arguments. Evidence: `context-factory/rva_e160c0.asm` and `.c`.

The outer constructor `e15534` initializes cached record pointers and internal state, calls `e157a0` to reset the table pointer array and initialize containers, allocates index/hash support, and stores a further object at `context+0x1090`, constructed by `f75830`. Constructor and reset are separate functions: calling only `e157a0` does not establish the complete initialized context.

The context begins with an array indexed by native table ID: `context + tableID*8`. The constructor leaves these table pointers null. Relevant observed fields are:

| Context offset | Observed use |
| --- | --- |
| `+0x240` | Table ID 72; `e191c4` performs GameMode lookup through its virtual `+0x40`. Native configuration helpers look up `Friendly` or `Ladder`. |
| `+0x3c8` | Table ID 121, original `csv_logic/testing.csv`; harness lookup `e1b7e8` delegates to its virtual `+0x40`. This is a different table from GameMode. |
| `+0x6f8`, `+0x700`, `+0x704` | Auxiliary table-pointer vector, capacity, count. |
| `+0x848` | Cached record used by the Ladder configuration helper. Its native record identity must come from loading the real tables. |
| `+0x870` | Cached object read during harness deck construction. |
| `+0xfb0` | Cached object read by the harness's configuration branch. |
| `+0x1010`, `+0x1090` | Objects constructed by the outer context constructor. |

`e16890(context, tableID)` uses the table-specific dependency list from `e15180(tableID, vector)`, conditionally creates a generic `0xf0`-byte native table via `e02c90`, stores it in `context[tableID]`, and appends it to the auxiliary vector. It handles particular dependency-backed table kinds; it is not established as a replacement for the primary CSV table factory.

The primary native attach function is `e15e64(context, nativeCSVObject, tableID)`. Its exact ARM64 ABI is `X0=context`, `X1=nativeCSVObject`, `W2=tableID`. It rejects IDs at or above 223. If a native table already exists, it calls `e108c0(existingTable, nativeCSVObject)`. Otherwise it constructs the original table class with `(nativeCSVObject, tableID, context)`, stores the result in `context[tableID]`, and sets the table's native mode field at `+0x94` according to ID. Specialized allocations/constructors are ID 3 (`0x8b8`, `e367f4`), ID 20 (`0x820`, `df1b14`), ID 86 (`0x198`, `e7a800`), and ID 92 (`0x108`, `dedb5c`); other IDs use the original `0x98`-byte class constructed by `e10804`. That constructor loads the native CSV rows through `e108c0`; direct fabricated row memory is unnecessary. Evidence: `table-attach/rva_e15e64.c`, `.asm`, and `rva_e10804.c`.

`e1698c(context, flags)` is **post-load finalization**, not a constructor. It visits already-loaded tables at offsets `+8` through `+0x6f0`, calls their virtual `+0x50`, and resolves hundreds of native names into cached pointers. Its direct callers found by scanning original ARM64 direct branches are `cbef38` (a loading-state callback) and `e16984` (a wrapper passing flag 1). The wrapper's exact original bytes are `2100805201000014`: `MOV W1,#1; B e1698c`. Flag 1 enables the typed-field phases; calling `e1698c` with zero omits them. Calling this finalizer on empty tables is not a valid resource bootstrap.

The initial store scan also found `df24d4`, but that function is a large per-row/global-property loader. Its use of a field at `+0x3c8` was coincidental and does not establish a context type. It must not be treated as the context constructor. The native CSV/catalog investigation owns the real primary table loading route.

## Native input and harness

`11d7968(out, context, nameString, seed, extra)` constructs a `0x28`-byte input record. `11d77fc(out, context, index, seed)` chooses among eligible names from native table data and builds the same record. Observed layout:

The `out` argument in those decompiler signatures is an AArch64 hidden return destination in **X8**. It must not be passed as an ordinary first argument. Verified assembly ABI:

| Function | Registers |
| --- | --- |
| `e160c0` | No arguments; context pointer returned in `X0` |
| `142fde4` | `X0=&nativeString`, `X1=NUL-terminated UTF-8 text`; native string construction used throughout the original code |
| `11d7968` | `X0=context`, `X1=&nativeString`, `W2=seed`, `W3=extra`, `X8=&8-byte-result-slot` |
| `11d77fc` | `X0=context`, `X1=index`, `W2=seed`, `X8=&8-byte-result-slot` |
| `11d58e0` | `X0=harness-storage`, `X1=context`, `X2=&input-pointer-slot` |
| `11d6398` | `X0=harness`, `X8=&8-byte-result-slot` |
| `10ca81c` | `X0=world`; ordinary return value in `X0` |

The supplied decoded table `analysis/decoded/assets/csv_logic/testing.csv` contains enabled `Normal` with `TestType=NormalBattle`. This supplies a concrete original scenario name for the first native harness attempt. The input builder must receive a genuine native string containing `Normal`, and table ID 121 must contain the rows created by the original CSV/table code.

| Input offset | Value written |
| --- | --- |
| `+0x00` | Context pointer |
| `+0x08` | Native 16-byte string object |
| `+0x18` | 32-bit seed |
| `+0x1c` | Flag; explicit-name builder sets it to 1 |
| `+0x20` | Extra integer, default `-1` or explicit argument |
| `+0x24` | Additional byte flag, initially 0 |

`11d58e0(harness, context, inputPointerPointer)` stores the input pointer, looks up its name through `e1b7e8(context, input+8, 0)`, constructs a native RNG seeded with `input.seed + 0x3ade68b1`, allocates three supporting objects, and calls `11d59dc(harness)` only when the named lookup succeeds. Observed harness fields include:

| Harness offset | Observed storage |
| --- | --- |
| `+0x00` | Context |
| `+0x08` | Input record |
| `+0x10` | Selected native table row |
| `+0x18` | Native RNG |
| `+0x20` | World/level object |
| `+0x28` | Integer vector; count at `+0x34` |
| `+0x38` | Integer vector; count at `+0x44` |
| `+0x48` | Harness tick limit counter |
| `+0x60`, `+0x68`, `+0x70` | Supporting objects allocated by the harness constructor |

The selected row's `+0x64` field controls initialization branches 0, 1, or 2. Its card/template vector is read at `+0x70`, with count at `+0x7c`. The harness builds native decks using `11d6f18`; it does not accept an arbitrary Rust approximation of a deck or a data row.

## Creation of a populated native world

`11d59dc` supplies an existing original initialization sequence:

1. Allocate a `0xd8`-byte battle configuration through `d17cb4(config, context)`. Depending on the selected row, native helpers select `Friendly`, `Ladder`, or build the additional configuration objects.
2. Construct native player/session inputs. The branch uses original `d2484c` or `d2aed4` helpers, not a new data layout.
3. Allocate `0x238` bytes and call `10c9514(world, 1, context, f07778(), 0)`. Store the world in `harness+0x20`; set its mode with `10c9904`, seed with `10c9968`, and set the harness flag through `10cb6a8`.
4. Read `battle = *(world+0xa8)`. Configure it through `d43448(battle, config)`, `d3bd50`, and `d36854(battle, playerA, playerB)`. Set `battle+0x60` to two participants.
5. Select the arena/map input, call `d3bb90`, construct two native decks through `11d6f18`, and install them through `d3bd58(battle, side, deck)`.
6. Construct two `0xf8`-byte native objects through `1027418` and pass them through `d338c4` for the two sides. Call `10cc4d0` for both native player inputs.
7. Call `d3778c` and `d378d8` to finish initialization and spawn initial objects.

`d3bb90` stores the arena at `battle+0xb8`, obtains its tilemap through `e48c7c(arena)`, and calls `d3c020` with the tilemap. `d378d8` explicitly reports `spawnInitialObjects called before gamemode config initialized` if `battle+0x1b0` is null and `spawnInitialObjects called before tilemap data initialized` if `battle+0xc0` is null. It includes the original `KingTowerMiddle` lookup and spawn paths. These conditions establish concrete resources that must be loaded before claiming a real native match.

The world constructor `10c9514` creates the command manager at `world+0x38`, an entity manager at `world+0x40`, and a `0x378`-byte battle object at `world+0xa8` in the standard branch (`arg1&1`, final constructor argument zero). Context is stored at `world+0x208`; the global/context parameter from `f07778()` is stored at `+0x210`. The main tick state starts at `world+0x60` and includes a second counter at `+0x70`.

## Exact step route

The original harness `11d6398` generates/submits native commands, then reaches this route:

```text
11d6398(harness tick)
  -> 10ca81c(world)                 active-state and stop-gate wrapper
     -> 10ca9a0(world)              command and battle orchestration
        -> d75ed0(world+0x38)       native command scheduling/execution
        -> d33f74(world+0xa8)       battle state update
           -> (world+0x40).virtual+0x30(tick)
              -> f8cd90(manager,tick)
     -> 11d9d58(world+0x60)         increment both tick counters
```

The wrapper preserves `world+0x1e8` on an internal byte stack, marks the active state, checks battle/end/controller gates, and restores the previous state on exit. The counters increment only on the path that enters the core update. Several battle timers advance by `0x32` (50) per update; real-time cadence has not been measured here.

`d75ed0` is a **command manager**, not the entity update. Its original strings report failed/late commands and compare scheduled command ticks. It invokes command virtual methods and removes executed/expired commands.

`d33f74` handles battle phase/time state, native managers and mode hooks, then calls the concrete entity manager's virtual slot `+0x30` at original instruction RVA `d343f0`. The manager was constructed by `f961f0`, which writes vtable RVA `1936450`. The original relocation at `1936480` (`vtable+0x30`) points to exact original function `f8cd90`. Reproduce that resolution with:

```sh
tools/reverse-env/bin/python tools/native_battle_navigation.py
```

The source/function/FDE identities and 19 resolved slots are saved in `engine-step/entity-vtable.json`.

## Original entity/component update

`f8cd90(manager,tick)` is a concrete native loop over the original objects and their components:

1. Calls manager virtual `+0x18` to reconcile its object list and snapshots `manager+8`, count at `+0x14`.
2. Calls manager virtual `+0x70`, then each entity's virtual `+0x40`.
3. Runs phase 1 through `f89534(entity,1)`.
4. Iterates manager-defined component categories (virtual `+0x60`/`+0x68`). For each present component in each snapshot entity, invokes component virtual `+0x10`, then virtual `+0x18` when the entity's component mask allows it.
5. Calls `f89524(entity,tick)` on each entity, runs phase 2, then each entity's virtual `+0x48`.
6. Calls manager virtual `+0x90`, runs phase 3, then manager virtual `+0x78`.
7. Reconciles the list again and invokes `f89544` for the remaining current entities. Frees the temporary pointer snapshot.

The phase helpers delegate through the native state object at `entity+0x28`; no movement, attack, projectile, or collision semantics have been invented here. Actual component classes and their native update methods still require per-class recovery and runtime validation before a faithful Rust port.

## Executed initialization, ticks and complete native battle

`analysis/native-pipeline/battle-full-04/manifest.json` records the successful
chain: original synchronous-loader service initialization; full flag-1 typed
postload; all 209 Location map loads from 202 original CSVs; original `Normal`
input/harness construction; one world step; and one bot/harness step. Earlier
checkpoints contain the original constructors, CSV tables, localization, SCDB
metadata and prefab loading described in [native-resource-bootstrap.md](native-resource-bootstrap.md).
The outer context route is `8264f0()`, which also supplies the native asset
registry holder at `context+0x1098`; the bare `e160c0()` context is insufficient
for these resource jobs.

The constructed battle has nonnull native configuration, arena, tilemap,
world, command manager and entity manager. Original `Normal` has enable byte
1 at row `+0x61` and TestType 0 at `+0x64`. The initial entity audit finds six
entities with 18 nonnull components. Every audited entity/component tick slot
points into executable segments of the original engine, rather than a host
replacement. `10ca81c` advances the two 32-bit counters from 0 to 1;
`11d6398` advances them from 1 to 2 and returns a null terminal result.

Starting from that tick-2 checkpoint, the original `11d6364(harness)` loop
executes the original bots, command queue and world updates until terminal.
`analysis/native-pipeline/full-match-01/stage01-attempt001/` records paired
counters **1354 / 1354**, 11 final entities with 39 nonnull components, and no
invalid memory accesses. Terminal event code **0** is constructed by
`11d7a30` after the original completion byte at `battle+0x1e0` becomes true.
The separate 36,000-step harness limit constructs code 3 through `11d79f0`;
that limit did not end this run. Event code 0 establishes the completion path;
the event's string is not a recovered winner field.

The 1,352 additional ticks took 24.495 seconds in the Python/Unicorn adapter,
excluding cold resource loading. This is an emulation measurement and does
not measure Rust simulation throughput. The original deterministic host clock
and selected offline loader settings are analysis interventions recorded in
the execution model.

`analysis/native-pipeline/full-match-02/stage01-attempt001/` repeats the same
prepared state and seed. [The replay audit](../analysis/native-pipeline/full-match-replay-audit.json)
reports equality of every saved memory-region hash, all architectural
registers, allocator state, import counts and call-plan values. Opaque Unicorn
CPU-context bytes differ and are not used as evidence of architectural
divergence. This verifies repeatability for one scenario/seed, not general
gameplay parity. Run `tools/native_entity_audit.py --run RUN_DIRECTORY` to
recheck source/checkpoint hashes and the entity/component callback provenance.

An instrumented repeat at `analysis/native-pipeline/full-match-formats-01/`
records 10,222 card-stat message-formatting calls and three same-global-ID
message-formatting calls. These counts do **not** establish validation failures:
original `d483a8` formats the card-stat message unconditionally, then sets its
failure byte only for a count greater than 4,096. Likewise, `dd8854` formats the
pointer-identity message whenever IDs match and calls its logger only when the
pointers differ. Exact assembly and pseudocode are retained under
`analysis/native-focused/native-match-diagnostics-v1/`. Human deployment
commands, selectable decks/controllers, the original battle presentation and
a Rust implementation of the recovered combat components remain work in
progress.

## Rust execution host

The [Rust oracle](../native-engine/README.md) restores the prepared native state
through Unicorn's C API and invokes the same original world/harness functions.
`analysis/native-oracle/full-match-comparison.json` compares its complete
`11d6364` run with the Python reference above. All 12 saved memory regions,
70 architectural registers and full allocator metadata match, including the
terminal pointer `0x5d748530` and event code 0. One-update and 100-update
comparisons also pass. Its warm complete-match call took 13.93 seconds;
snapshot loading and report writing are excluded. The combat implementation
remains original ARM64 code; this establishes an independent Rust execution
adapter to use as the reference for the later Rust component ports.
