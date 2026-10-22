# Native battle deck input contract

The `11d6f18` output is the per-side eight-entry battle deck assembled from `LogicTestingData`, not a bot-policy object. The strongest evidence is the typed row layout from native constructor `e8a5b8` and field-name initializer `e8a910`, combined with the consumers in `11d6f18`, `11d7250`, and `d3bd58`.

The scenario name passed as `X1` to `11d7968` is looked up through `e1b7e8` and table 121. The original table is `csv_logic/testing.csv`; its row schema is `Name, IsEnabled, TestType, Flags, AlwaysSpells, NeverSpells, GameMode, UseEvolution, UseHeroForm`. Native type constructor `e8ae98` registers `logic.data.LogicTestingData` as type ID `0x79` (121). The source CSV includes rows such as `Normal` and `BoatBattles`.

| Row field | Native row offset | Use established by original code |
| --- | ---: | --- |
| `IsEnabled` | `+0x61` | Boolean set by `e8a5b8` |
| `TestType` | `+0x64` | Enum used by `11d59dc` branch selection |
| `Flags` | `+0x68` | Parsed by typed row constructor |
| `AlwaysSpells` | vector at `+0x70`, count `+0x7c` | Entries are wrapped into the side's list by `11d6f18`/`11d7250` |
| `NeverSpells` | vector at `+0x80`, count `+0x8c` | Entries are excluded from random fill by `11d6d8c` |
| `GameMode` | `+0x90` | Optional referenced mode object used by setup builders |
| `UseEvolution` | byte `+0x98` | Read by `11d7250` when selecting a candidate form |
| `UseHeroForm` | byte `+0x99` | Read by `11d7250` when selecting a candidate form |

`11d6f18` reads `AlwaysSpells`, converts those entries through `11d7250`, fills the remaining slots from eligible candidates, shuffles the result, and creates the side object. Candidate selection excludes `NeverSpells` and avoids duplicates against entries already in the output. In the normal construction route, `11d59dc` calls `11d6f18` independently for side 0 and side 1, then passes each result to `d3bd58`. That setter stores each object at battle-state offset `+0x88 + side*8`; it also iterates list entries through the list's count/index virtual methods before storing it.

The separate `d2484c`/`d2aed4` side descriptors remain distinct from these eight-entry spell lists. In particular, `ctx+0xfa0` resolves the `BoatAttackNPC` resource and can supply the first side descriptor; this does not replace the explicit `AlwaysSpells`/`NeverSpells` deck inputs. The scenario constructor has no separate deck argument: the supported source-proven selection input is the scenario row name, with its deck entries and restrictions in table 121. The seed controls random candidate selection and ordering.

Evidence files: `analysis/native-focused/resources/testing-row-type/rva_e8a5b8.c`, `rva_e8a910.c`, `rva_e8ae98.c`; `analysis/native-focused/engine-step/callees/rva_11d59dc.asm`; `analysis/native-focused/engine-step/world/rva_11d6f18.asm`; `analysis/native-focused/deck-controller-luna-v2/rva_11d6d8c.asm`; `analysis/native-focused/deck-controller-luna-v3/rva_11d7334.asm`; `analysis/native-focused/deck-controller-luna-v4/rva_11d7250.asm`; and `analysis/native-focused/deck-controller-luna-v5/rva_d3bd58.c`. Function names above are ELF RVAs; Ghidra disassembly addresses in `.asm` files are offset by `+0x100000`.

`d3bd58` takes the battle-state object, not the enclosing world: it reads the configuration at `+0x1b0` and accesses side structures beyond `+0x270`, within the 0x378-byte battle. The enclosing 0x238-byte world stores this battle pointer at `+0xa8`.

The `11d6f18` assembly keeps candidate wrappers in an SP-backed vector and selected wrappers in a separate X21 list. Selected entries move into X21 at `0x11d702c..0x11d7064`; the cleanup loop at `0x11d7214` destroys only the leftover SP candidates. The selected list is passed to `fea548` (`X0=Deck`, `X1=context/world`, `X2=list`), which copies its nonnull pointers into the Deck. This corrects an earlier audit note that had incorrectly said selected wrappers were cleaned before construction.

`11d7250` is the source-row wrapper factory. Its ABI is `X0=context`, `X1=native source-row`, `W2=index`, and `X8=&result_slot`; it writes the resulting wrapper pointer through X8. The body invokes the source row's vtable method at `+0x180` with the row and `*(u64*)context`, then initializes the returned wrapper through its vtable method at `+0x38`. Thus the available native route to explicit entries starts from valid native source rows; this function is not a generic card-ID constructor. In the harness, the context passed as X0 is the harness object, whose first pointer and `+0x10` scenario pointer are read by the factory.

## Asymmetric side decks

`11d59dc` calls `11d6f18` twice back to back at `0x11d6058` and `0x11d6064`, then installs both results with `d3bd58(battle, 0, ...)` and `d3bd58(battle, 1, ...)` at `0x11d6140` and `0x11d6154`. `NativeSession::configure_normal_decks_and_rebuild` therefore leaves the loaded row's side-0 vector in place and installs the `deck_preset_side1` vector only for the second `11d6f18` entry, restoring the original vector at the call's return address `0x11d6068`. Both decks still come from the original source-row wrappers and the original constructors; no game instruction or constructed deck object is patched. `deck_source_rows` reports two arrays when the decks differ, and both installed decks are re-read from `battle+0x88+side*8` before a session is accepted.
