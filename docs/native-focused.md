# Focused native decompilation

The recovered `libg.so` can be decompiled without another whole-program analysis. The focused runner imports the unchanged derived ELF with Ghidra `-noanalysis`, disassembles selected original unwind ranges, creates functions with exactly those bodies, and exports pseudocode, assembly, and provenance.

Input: `analysis/libg-cipher/libg.decrypted.so`, SHA-256 `79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`. Function bounds come from `analysis/native-engine-index/unwind-functions.tsv`, which was extracted from the original ELF unwind records. Ghidra uses image base `0x100000`; all RVAs below exclude that base.

## Reproduction

```sh
python3 tools/native_focus_decompile.py
python3 tools/native_focus_catalog.py analysis/native-focused

python3 tools/native_focus_decompile.py --out analysis/native-focused/example --rva fec180 feb200
python3 tools/native_focus_catalog.py analysis/native-focused/example
```

The runner uses the project-local Ghidra 12.1.4 installation and the existing SDKMAN JDK 21. It creates and deletes a separate temporary project and never opens or modifies a REA session's project. The source ELF is hashed before and after the run. `run.json` records the command and script/launcher/tool-configuration identities; `manifest.json` records source bytes, imported-memory bytes, FDE identity, instruction coverage, errors, and pseudocode hashes. `catalog.json` indexes original direct branches and source-bound printable constant candidates. The ELF loader applies relocations in its private imported program, so both file and imported-memory hashes are recorded; they match for all currently selected functions.

Two Ghidra behaviors require explicit handling:

* A decompiler may follow an unconditional branch into another FDE even when the caller's function body is bounded. Such branches receive an analysis-only `CALL_RETURN` override when the destination is another exact original FDE start. The manifest lists every override. The assembly records the original instruction and the original ELF remains unchanged.
* Without global analysis, PLT calls to `std::terminate` and other known nonreturning imports can fall through into the following FDE in pseudocode. The runner resolves exact AArch64 PLT stubs through the original `DT_JMPREL` symbols and declares known nonreturning import functions. `import-thunks.tsv` and the manifest preserve this evidence. This avoids merging the thread launcher with its neighboring worker function.

The first outputs before these controls remain under `first-pass/` and `before-flow-override/`. They are historical probes, not the preferred bounded outputs.

## Recovered deck object structure

The three constructors write the same vtable pointer, RVA `0x193bcb0`. Its 46 contiguous function pointers match `battle-deck-pointer-run.json`. The following structural names describe observed offsets; no battle hand or queue meaning is assigned.

| Object offset | Observed storage |
| --- | --- |
| `+0x00` | Vtable pointer |
| `+0x08` | Constructor-supplied owner/context pointer |
| `+0x10` | Separately initialized container/state |
| `+0x20` | Pointer array; capacity at `+0x28`, signed count at `+0x2c` |
| `+0x30` | Pointer array of auxiliary records; capacity at `+0x38`, count at `+0x3c` |
| `+0x40` | Second pointer array; capacity at `+0x48`, count at `+0x4c` |
| `+0x50` | 32-bit state field |

The native object allocation in the clone routine is `0x58` bytes.

| RVA | Observed behavior | Evidence |
| --- | --- | --- |
| `fea43c` | Initializes the object; first pointer array has eight null entries, second has one null entry; stores supplied context. | `deck/rva_fea43c.c` and `.asm` |
| `fea548` | Initializes from a supplied pointer container. Allocation length is capped at 40; copies nonnull pointers by index. | `deck/rva_fea548.c` |
| `fea6b4` | Similar initialization plus auxiliary record construction based on additional input. | `deck/rva_fea6b4.c` |
| `feda98` | Reserves an array of pointers: allocates requested capacity, copies the current count, frees the old buffer, sets buffer and capacity, leaves count unchanged. | `helpers/rva_feda98.c` |
| `feb200` | Resizes the second array. Shrink removes from the end with two object callbacks; growth reserves as needed and null-fills new entries. | `helpers/rva_feb200.c` |
| `fec0e0` | Replaces an entry in the first array, stores the replacement before calling the old object's vtable `+0x10` and `+0x08`. | `deck/rva_fec0e0.c` |
| `fec1a4` | Replaces an entry in the second array, calls old object's `+0x10` and `+0x08` before storing the replacement. | `deck/rva_fec1a4.c` |
| `fec280` | Clears a second-array entry, invoking the same callbacks before storing null. | `deck/rva_fec280.c` |
| `fec180` | Swaps two first-array pointer slots using signed 32-bit indexes. | `deck/rva_fec180.c` |
| `fec3c0` | Second-array indexed lookup; reports invalid bounds then still loads the indexed entry. | `deck/rva_fec3c0.c` |
| `fec49c` | Second-array pointer identity search, returns index or `0xffffffff`. | `deck/rva_fec49c.c` |
| `fec320` | True iff all second-array entries are nonnull; true for a nonpositive count. Rust port `all_nonzero_array_40` matches 1,024 native cases. | `deck/rva_fec320.c` |
| `fec370` | True iff some second-array entry is nonnull; false for a nonpositive count. Rust port `any_nonzero_array_40` matches 1,024 native cases. | `helpers/rva_fec370.c` |
| `fed2a0` | Allocates and initializes another object, resizes both arrays, clones nonnull first-array entries through their vtable `+0xd0`, copies second-array entries through helper `1059808`. | `helpers/rva_fed2a0.c` |
| `fed5e0` | Calls `fed2a0`, then clones auxiliary records through helper `f87248` and appends them. | `helpers/rva_fed5e0.c` |
| `fea930` | Clears/disposes entries and resets counts/context while retaining some container allocations. | `deck/rva_fea930.c` |
| `fed988`, `feda04` | Destroy array storage at `+0x40`, `+0x30`, `+0x20`, `+0x10`; the latter additionally deletes the object. | `deck/rva_fed988.c`, `deck/rva_feda04.c` |

Bounds-reporting calls have not been established as nonreturning; several functions explicitly continue after them. A Rust implementation should define a valid-input domain rather than implement native out-of-bounds memory accesses. In particular, `fea548` and `fea6b4` cap allocated length at 40 but use the original source count in the copy loop; callers' preconditions remain to be recovered. Callback effects may mutate the object and counts, so callback ordering and reloads matter for behavioral equivalence.

The exported methods include serialization, validation, object lookup, and UI/logic helpers. The presence of these array operations does not yet establish battle card rotation or the battle simulation loop.

## Original UI bindings

The recovered code uses the same literal instance names as the recovered UI display trees. These references are direct native-code evidence for the original bindings:

| RVA | Referenced original names |
| --- | --- |
| `8b1430` | `ability_ingame_button`, `ability_mask`, `ability_button_mask`, `states`, `elixir_count`, `charge_count` |
| `8cf3b8` | `HUD_topRight`, `HUD_topLeft`, `timeLeft`, `enemyName`, `enemyClan`, `elixirRegen`, `viewers` |
| `8da088` | `panel_ingame_blue`, `panel_replay`, `cardArea`, `nextCard1`, `elixir_bar`, `elixirRequirementBar` |
| `8dc37c` | `NextSpellTimer` |
| `8dddb8` | `cardArea%d`, `elixir_bar_2`, `wins_player`, `wins_opponent`, `card_unknown_anim` |

Generic child lookup helpers `1242f34` and `1243194` scan names at object `+0x98`, using a signed 16-bit count at `+0xc0`, then return the corresponding child from `+0x90` only if the child's virtual predicate at `+0x80` or `+0x88` succeeds. The called name comparison routine remains an external dependency in this focused export.

## Startup and core-loop navigation

| Entry RVA | Direct native route |
| --- | --- |
| `146096c` (`JNI_OnLoad`) | Initializes JNI support, optionally logs `Loaded without GLOBAL init: `, returns JNI version `0x10006`. |
| `1460bd4` (`createGameMain`) | JNI/global configuration and asset-manager setup; environment probes and JNI string return are visible. The actual battle-loop creation is not established by this export alone. |
| `1461ed0` (`onCreate`) | Ensures a `0x108`-byte singleton constructed by `1442884`, then calls `1442dc8`. |
| `146220c` (`onSurfaceCreated`) | Converts a supplied surface, ensures the same singleton, calls `14440a8`. |
| `14616d4` | Ensures that singleton and forwards parameters to `1444924`. |
| `1442dc8` | Calls the ELF-resolved `pthread_create` PLT entry (`187d690`), passing entry RVA `1446280` and an allocated task containing callback RVA `1442ee8` and the singleton pointer. |
| `14440a8` | Allocates a surface task with vtable RVA `1966230`, inserts it into a work container obtained through `13860e4`, increments/signals work state under synchronization. |

These are startup/thread/task routes. Further navigation requires resolving the actual task invocations and GameMain virtual calls, then identifying battle update semantics. Decompiled unknown types and guessed prototypes are not evidence for class names or argument types.

## Coverage and remaining limits

The main export contains 15 selected functions. The deck export contains all 46 vtable targets plus three constructors. Every instruction byte in those 64 selected FDE ranges was disassembled, and all 64 decompilations completed. Additional neighboring helpers and lifecycle functions are separate exports. Some helper ranges include padding or unreachable trap-adjacent words, which are left undisassembled and reported through byte coverage.

This is recovered native pseudocode and assembly, not recovered original C++ source. It does not by itself make the original client run or implement a Rust battle engine. Runtime behavior, callback side effects, ownership contracts, and valid domains must be checked before porting each method. The original class layout and individual verified operations are a useful foundation; the simulation update loop remains a separate required recovery target.
