# Original native resource bootstrap

The decoded game library exposes complete original CSV, TOML and JSON parsers
that can load the supplied resources without Android asset-manager I/O. The
resource plan feeds those parser results into original native table constructors
and the original context postload sequence. It does not construct replacement
rows or substitute Rust game rules.

Source ELF: `analysis/libg-cipher/libg.decrypted.so`, SHA256
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`.
All addresses are original ARM64 ELF RVAs.

## Native contracts established from original code

| RVA | Observed API |
| --- | --- |
| `0xe160c0` | Allocate `0x10a8` bytes, run the original context constructor, return the context. |
| `0x142fde4` | Construct a native 16-byte String from NUL-terminated text. |
| `0x142fdc4` | Construct a native String from a byte buffer and explicit length. |
| `0x1288f2c` | Construct a `0x68`-byte CSV root node from `(node, raw_bytes, byte_count, filename_String)`. |
| `0x1289a04` | Return the native CSV table owned by a parsed CSV root node. |
| `0x12a6e64` | Parse TOML from `(native_text_String, optional_error_sink)`; return a native object or null on failure. |
| `0x1292230` | Parse JSON from `(native_text_String, optional_error_sink, maximum_depth)`; the original caller uses depth 100. |
| `0x128f5c0` | Validate a native JSON value as object type 2 and return the object, or null. |
| `0xe14440` | Construct a native resource descriptor from `(descriptor, path_String, table_ID, flags)`. |
| `0xe14408` | Construct the paired-path descriptor from `(descriptor, path1_String, path2_String, table_ID, flags)`. |
| `0xe14450` | Construct a descriptor with no file paths from `(descriptor, table_ID, flags)`. |
| `0xe15e64` | Attach the native descriptor to the native context's table identified by the ID. |
| `0xe1b7e8` | Look up a named row in context table 121 (`context + 0x3c8`). |
| `0xe191c4` | Look up a named row in context table 72 (`context + 0x240`). |
| `0xe16984` | Set W1 to 1, then invoke original complete context postload `0xe1698c`, enabling typed-field phases. |

CSV parsing is not a Python CSV conversion: the original code splits LF/CRLF
lines, reads native column types, builds native CSV columns and values, and runs
original conversions in `0x1289194` and `0x128a1a8`. TOML and JSON are likewise
parsed by original native routines.

## Exact resource input boundary

A native resource descriptor contains the file-entry pointer array at offset
`0x8`, capacity at `0x10`, count at `0x14`, table ID at `0x18` and flags at
`0x1c`. Each file entry starts with its native path String, followed by:

| File-entry offset | Parsed resource pointer |
| --- | --- |
| `0x10` | Native CSV root node |
| `0x18` | Native TOML/JSON object |

Original loader `0xe147a0` obtains those parsed objects from its resource-provider
virtual methods and stores them at these offsets. The default provider at
RVA `0x1a5f430` has vtable `0x1920a00`; its `+0x18` method tail-calls
`0x120e800`, while its `+0x30` method tail-calls `0x120da30`. Those two routines
use the Android/client resource manager to obtain already-parsed assets.

The headless input adapter replaces that delivery step only. It binds the
original parser's result to the original file entry with the exact original
one-store helper `0xd19b78` (`STR X1, [X0]; RET`). The raw asset bytes, native
parser, native descriptor, native typed-row construction and postload code all
remain original. The adapter does not write card properties, health, elixir,
row fields or battle state.

This contract is corroborated by original `0xe108c0`: it iterates the descriptor
file entries, reads CSV root offset `0x10` and calls `0x1289a04` followed by
`0xe128b4`, or reads TOML/JSON offset `0x18` and calls `0xe12a80`. Function
`0xe149d4` implements the first-file CSV-table accessor.

## Catalog recovered from original native instructions

`tools/resource_catalog.py` follows constants through the original catalog
builders `0xe69930` and `0xe0cf80`, capturing filename String construction and
native descriptor calls. It recovers 157 descriptors with every numeric table
ID and flag known. Source-bound output, including individual call instructions,
is `analysis/native-focused/resources/catalog-descriptors.json`.

The catalog includes:

| Path | Native table ID / role |
| --- | --- |
| `csv_logic/testing.csv` | 121 (`0x79`), native test scenarios |
| `csv_logic/game_modes.csv` | 72 (`0x48`), native game-mode rows |
| `csv_logic/arenas.csv` | 54 (`0x36`), arena rows |
| `csv_logic/global_ids_oldformat.csv` | Dedicated global-ID loading branch, flag 2 |
| `csv_logic/game_tags.csv` | Dedicated game-tag loading branch, flag 3 |

The source `testing.csv` is present in `analysis/decoded/assets/csv_logic/`.
Its 1,167 bytes have SHA256
`604b326ad7d69fe037fe561d8247ae98f63e5b64025a1b1d62fa1703573060ee`.
Its first original row is `Normal,true,NormalBattle`; other rows provide original
champion, evolution and alternate-mode test scenarios. The harness's context
lookup at `+0x3c8` therefore selects a test row, not a game-mode row.

The catalog's two language/localization descriptors use separate client-global
language APIs rather than the battle context. They are explicitly recorded as
excluded from the bulk engine-context plan. The plan covers the remaining 155
engine descriptors, including the original no-file descriptor for table 179.
Its 158 original files total 6,783,020 bytes. Paired-path descriptors contribute
more than one input file.

The two plain JSON resources `legacy_vfx/effects.json` and
`legacy_vfx/emitters.json` are extracted unmodified from the original asset APK.
Their original APK digest and member digests are retained in the plan. All CSV
and TOML inputs are the previously recovered original decoded files, checked by
individual SHA256 before each native call sequence.

## Typed-row population requires the native postload phases

Attaching a table creates original typed row objects and name lookup, but does
not alone populate every typed field. These stages must not be conflated.

The Testing row's `IsEnabled` field is populated by original function
`0xe8a5b8`, which loads the native column index, converts the source value, and
stores the Boolean at row offset `0x61`. This is row virtual method `+0x48`.
The original generic table dispatcher `0xe12cc4(table,start,count)`, table
virtual method `+0x68`, invokes that method for its rows. Its context wrapper is
`0xe19398(context,table_ID,start,count)`.

Table virtual method `+0x50`, implemented by `0xe12be4`, performs a different
base-finalization phase. It clears row flag `+0x60` through `0xe0bff0` and calls
row method `+0x38`; it does not perform the typed `IsEnabled` load. The full
context postload function `0xe1698c` later invokes table method `+0x68` for all
present tables, with its call visible at `0xe18ce4`.

Observed native evidence:

- `analysis/native-context/testing-lookup-03`: original CSV parsing, native
  descriptor/table attachment and original lookup of row `Normal` completed.
- `analysis/native-context/testing-populate-01`: original typed-row dispatcher
  completed for that row and populated `IsEnabled` as true, matching the source.

The diagnostic text `Unwanted replacement table discriminator ...` is formatted
unconditionally by `0xe11a24` before a later condition determines whether it is
logged. Merely observing that format at `vsnprintf` does not establish a CSV
header error or a native diagnostic failure.

## Reproduce the call plans

```sh
tools/reverse-env/bin/python tools/resource_plan.py
tools/reverse-env/bin/python tools/resource_bulk_plan.py
tools/reverse-env/bin/python tools/resource_typed_tail.py
```

The small plan is
`analysis/native-focused/resources/testing-lookup-plan.json`. It includes the
original context factory, filename and lookup Strings, original CSV parser,
native descriptor and table attachment, then the original `Normal` row lookup.

The bulk plan is
`analysis/native-focused/resources/bulk-native-assets-plan.json`. Its generated
SHA256 is
`e081976b50fee5a7ebe77ae8ef1920204f34f95bfc547db6a1c72e948de4f1ea`.
It contains 2,424 explicit source-bound steps. The context return-value key is
`context`. Every source, input and native API range is bound by hash.

After asset parsing and native table attachment in original catalog order, the
bulk plan follows the observed native outer loader's relevant context stages:

1. Dedicated global-ID and game-tag loading through `0xe16468` and `0xe16234`.
2. Native post-attachment stage `0xe6e3ec`.
3. Dependency-table creation `0xe16858`, including the six synthetic table IDs
   explicitly enumerated by original `0xe15180`.
4. Native per-table preparation `0xe12548`.
5. Original context postload `0xe1698c` with W1 equal to zero.
6. Original native name lookups of `Normal` and `Friendly`.

The immutable first bulk plan supplies W1 equal to zero, so the original
conditional typed-field passes remain disabled in that prefix. The separate
continuation `bulk-native-assets-typed-tail.json`, SHA256
`b2fea9faa791f3c1dc5c234e649ee45bede9d0b835817d9f5fee8e2560c22236`,
inherits the completed prefix values and calls original wrapper `0xe16984`.
Its exact instructions set W1 to 1 before branching to `0xe1698c`, enabling
table virtual methods `+0x58`, `+0x68` and `+0x70`. It then repeats original
`Normal` and `Friendly` lookups and retains typed-row inspections. The battle
plan must inherit this typed continuation after its successful completion.
The prefix file stays unchanged while native execution is resumed.

The plan is consumed by `tools/native_unpack.py --call-plan`, resuming an
immutable checkpoint after original static constructors. An interrupted plan
must resume with the same plan SHA256. Its native call results and full mutable
memory are retained in the checkpoint; no step is declared successful merely
because an import or execution limit was reached.

## Manifest resources and localization (2026-10-18)

The fixed catalog is not the complete battle data set. Original `e69930` also
expands `data_manifest.toml` through `e693f4`/`e69758`. The supplied manifest has
261 path-bearing entries (256 distinct paths); repeated entries are preserved.
Original `e6df50` delivers each parsed TOML object to `e6e280(context,root,path)`.
That function dispatches the original categories into their native tables.
The recovered Global Lightning definition is one concrete dependency that the
fixed catalog alone omitted.

`tools/resource_dynamic_plan.py` and `tools/resource_complete_plan.py` retain
these original inputs and execute native attachment before table preparation.
The table preparation argument must be the table pointer read from the context
slot, not the address of that slot. An earlier generated dynamic plan made that
mistake; its affected runs are not valid execution evidence. Retained corrected
plans use explicit `read_u64` steps.

The latest successful pretyped resource checkpoint is
`analysis/native-pipeline/complete-run-04/stage01-attempt001/state`. All 4,764
steps returned, with zero invalid memory accesses and zero native diagnostics.
It contains 164 nonnull context tables, after original fixed and manifest input
loading. Five tables (212, 213, 214, 221, 222) were created by original dynamic or
post-attachment code outside the fixed preparation set. The source's outer
loader prepares every nonnull table. `tools/native_prepare_remaining.py`
verifies the saved context and generates native preparation calls for precisely
those five; the dynamic generator now includes them for fresh runs.

Complete typed postload also needs the original localization singleton. Native
Language row loader `e476cc` calls `12ad7e8()` and `12ad7f4()` to retrieve its CSV
table, so excluding localization entirely caused a null dereference. Original
flag-1/table-0 handling in `e6df50` calls `12ad530(originalCSVTable)`. This
function constructs the actual native text index and publishes its singleton.
`tools/native_language_plan.py` loads the complete, unfiltered 11,077,040-byte
`csv_client/texts.csv` through original `1288f2c`/`1289a04` and `12ad530`.

That stage completed in
`analysis/native-pipeline/complete-run-04/stage02-attempt001/report.json`:
all nine steps returned, with zero invalid accesses or native diagnostics. The
singleton's `+0x28` pointer matches the original parser's CSV table pointer.
The immutable plans in `analysis/native-context/bootstrap-v4/` partition loading
before typed postload, preserving all original resource steps. Their checkpoints
allow later bootstrap corrections without reloading all assets.

## Original asset database and outer context (2026-10-18)

The bare context factory leaves its asset-service holder at `context+0x1098`
null. The original outer factory `8264f0()` creates the context, obtains the
original asset registry, installs the holder, and publishes its manager at
global RVA `1a46a80`. The full resource plan
`analysis/native-context/outer-resources-v1/plan.json` uses that factory and
loads the fixed and manifest resources before post-attachment. All 4,448
steps completed in `analysis/native-pipeline/outer-resources-01/`.

Original SCDB loading also needs explicit type registration. The generic
registrar `138df64()` and game registrar `779398()` populate the original
type registry; the ELF static constructors alone leave it empty. Original
filesystem setters `1358058` and `1358100` install the offline root for the
`data:` and `update:` providers. Then `122dc74` creates the original file
source, using the SQLite implementation compiled into `libg`. The supplied
23,543,808-byte `assets.scdb` has SHA256
`792abc4536d56a4e7dc11fb825b578b4d6b3da43afa19747411f48d8d504c882`.
Its records and metadata are interpreted by the original native code.

`tools/native_vfs.py` provides explicitly bound, hash-verified, read-only file
bytes and an opaque stdio API. The emulator also supplies read-only mappings
of those bytes, retaining them in checkpoints. It does not replace SQLite,
the asset registry, metadata parsing, or prefab resolution. Native Bionic
`bsearch`, `pthread_cond_destroy`, and `pthread_once` execute from the captured
ARM64 libc; its original PLT references are linked to native exports or the
explicit OS import boundary. These are selected-function execution results,
not a claim that the entire Bionic runtime has been initialized.

The main context's database source completed in
`analysis/native-context/scdb-full-resume-03/`. The subsequent original mount
and lookup completed in
`analysis/native-pipeline/scdb-mount-full-01/stage01-attempt001/`.
`12816c0` successfully resolves the original projectile prefab
`sc3d/characters/hero/hero_barbarian_barrel/hero_prefab_barbarian_barrel_projectile_fall`
through that context's asset service. This previously blocked typed projectile
loading. No hand-written asset record or type-name insertion was used.

Both original post-attachment calls, `e6e3ec(context)` and
`e16858(context)`, then returned successfully with no invalid accesses in
`analysis/native-context/postattach-resume-03/`. Its completed two-step plan
has SHA256 `95ea26544423f7bd79e92813e72942d8814ba515993d23f1e0d612a3889d5ad8`.
`tools/native_finalize_plan.py` verifies that checkpoint, reads its actual
223 table slots, and generates preparation for all 164 nonnull tables followed
by original localization and flag-1 typed postload. Table preparation has
completed in `analysis/native-pipeline/finalize-full-01/`; the following stages
are being verified separately.

Full typed loading also creates the original native asset worker pool. The
trampoline is `13c0570`, the worker body is `13bf674`, and queued jobs execute
through `13bf788`. `tools/native_threads.py` models a single virtual CPU with
cooperative switching at blocking POSIX calls. Original callbacks and jobs
execute in Unicorn with separate stacks, TLS, and saved CPU contexts. Mutex
ownership, condition wakeups/reacquisition, and worker state survive
checkpoints. Busy-wait preemption, timed waits, and TLS-destructor teardown are
not yet supported. The real ARM64 execution/restore test in
`tools/test_native_threads.py` passes in `tools/reverse-env`.

The first actual worker reached the original prefab JSON deserializer and
exposed another startup dependency: the object-type factory at global RVA
`1a90f18`. Its original creation and provider installation are the contiguous
instructions `7718ac..771930` inside startup function `771818`. The bounded
plan `analysis/native-context/object-registry-v1/plan.json` executes those
exact source-hashed instructions, stopping before the subsequent client
startup step. This is a bounded native region, not a claim that the containing
client function returned or that the full client was initialized. The harness
does not write replacement provider vtables or registry records. The original
lookup `12a3748(5000)` then returns a nonnull native type factory in
`analysis/native-pipeline/object-registry-01/stage01-attempt001/`.

The POSIX boundary now supports C-locale numeric conversion and aligned
allocation. An earlier blanket `sysconf` result also returned the page size
for the processor-count query; that state is superseded. Android's
`bits/sysconf.h` assigns the page queries `0x27/0x28` and processor queries
`0x60/0x61`. The corrected model reports 4 KiB pages and one virtual processor,
and typed loading was replayed from before the first processor query.
Source identity and the superseded run are retained in
`analysis/android-runtime/bionic-sysconf-source.json`.

After real prefab jobs ran, the original 256 MiB bump-only analysis heap filled.
The state from `object-registry-01/stage02-attempt005` is superseded: its old
allocator advanced its pointer past the limit before reporting failure.
`battle-full-03` resumes the preceding valid timeout checkpoint instead.
`tools/native_heap.py` now models aligned allocation, size-class reuse on free,
and up to four additional 256 MiB arenas. Extra arena bytes and allocator
bookkeeping are checkpointed separately, without overlapping TLS. `calloc`
clears reused storage and `realloc` copies the smaller of the old and new
requested sizes before releasing the old allocation.
Four allocator tests exercise actual Unicorn memory, alignment, allocation
bounds, and continuation in a fresh mapped instance; all pass. The retained
legacy snapshot includes old allocations from the bump-only prefix, so this
does not establish the original engine's live-memory requirement. The import
budget is now 100 million across the retained resource/battle chain; the prior
20-million loader guard was nearly exhausted by progressing resource work.

`battle-full-03` finished the native asset workers and returned to main-thread
postload, where it requested `csv_logic/globalIdDb.csv`. Exact inventories of
all five supplied APK splits confirm that name is absent. The package boundary
now returns null only for verified absent assets, recording archive hashes and
inventory identities; it explicitly stops if asked to open a present packaged
asset whose byte-backed AAsset API is not yet modeled. No replacement CSV is
supplied under that name.

The following failure was a separate null service, not a CSV parse of absent
bytes: original `12c2e78` calls singleton getter `12ca904`, then predicate
`12cad38`; global RVA `1a91140` was not initialized. Original factory
`12ca84c` constructs and publishes that service. Its actual constructor
`12ca910` has a supported synchronous path when its first flag is zero.
`analysis/native-context/async-loader-v1/plan.json` selects `(0,0)` explicitly
for offline execution, and verifies a nonnull getter and false async predicate.
All three native calls returned in
`analysis/native-pipeline/battle-full-04/stage01-attempt001/`.
This configuration is not presented as the Android client's startup default.
The full typed call completes after this initialization in
`battle-full-04/stage02-attempt007`, including its nine explicit checks. All
209 original Location loads from 202 CSVs then complete in stage 3, followed by
the original battle constructor and both world and bot/harness step probes.
The resulting state has real native configuration, arena, tilemap and spawned
entities. The [battle ledger](native-battle-loop.md) records the complete native
AI run and exact replay from that prepared state.

## Current limitations

The original bulk context, typed continuation, map cache and native battle
construction have executed successfully. Observed Android/Bionic imports are
modeled explicitly by the emulator harness. This is not an Android
runtime-equivalence claim. Native diagnostic formatting remains enabled;
formatting-call counts alone do not establish failures, as detailed in the
battle ledger.

The older fixed-only bulk plan excludes manifest extras and localization; the
new stages above supply those dependencies separately. The original asset
database is now mounted and a concrete native prefab lookup succeeds, but
desktop rendering and full client UI initialization remain separate work.
The original native bots and battle loop now execute through the completion
path. Human input, selectable external AIs, full native presentation and the
Rust combat rewrite are not complete.
