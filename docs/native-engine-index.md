# Native engine and original UI evidence index

This index binds recovered literals, code references, native pointer tables and
function intervals to the actual decoded Clash Royale 160402020 ARM64 binary.
It identifies concrete starting points for decompilation and small operations
that can already be checked against original instructions. It does not claim
that the complete battle simulation has been reconstructed.

Source: `analysis/libg-cipher/libg.decrypted.so`, SHA256
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`.
All addresses below are ELF virtual addresses/RVAs. Add the Ghidra session's
reported image base only when its interface requires that coordinate system.

## Reproduce

```sh
tools/reverse-env/bin/python tools/native_engine_index.py
tools/reverse-env/bin/python tools/native_engine_deck_probe.py
```

The first command verifies the source digest and uses no Ghidra import. Its
outputs are in `analysis/native-engine-index/`:

- `summary.json`: source identity, coverage, exact searches and limitations.
- `unwind-functions.tsv`: all 73,647 shipped unwind function intervals.
- `anchors.json` and `references.json`: selected exact literals and code/data references.
- `pointer-runs.json`: linked contiguous function-pointer runs and constructor references.
- `focused-method-anchors.json`: explicit subsystem/method assertion references.
- `battle-deck-pointer-run.json`: all 46 slots of the battle-deck candidate table, exact function intervals, source bytes and bounded disassembly.
- `native-ui-instance-bindings.json`: requested native UI instance names and following call instructions.

## Observed native function boundaries

`PT_GNU_EH_FRAME` begins at RVA/file offset `0x3cc210`. Its header contains
73,647 search-table entries using data-relative signed offsets. Every entry's
function start agrees with its referenced FDE's PC-relative start. All entries
refer to the same CIE at `0x45bf98`, which has the observed `zR` augmentation
and pointer encoding `0x1b`.

The FDE start is `fde_address + 8 + signed32[fde + 8]`; the function length is
`u32[fde + 12]`. These source-bound intervals cover starts from `0x7706b0` with
maximum end `0x187d10c`. They are observations from unwind metadata; compiler
outlining, aliases and unlisted code may require additional interpretation.

## Battle-deck class evidence

Five small functions reference explicit `LogicBattleDeck::...` assertions and
all occur in one 46-slot contiguous function-pointer run at RVA `0x193bcb0`.

| Function RVA | Table slot | Exact method name appearing in assertion |
| --- | ---: | --- |
| `0xfec710` | 24 | `LogicBattleDeck::getDeckSlotsConfig` |
| `0xfec740` | 25 | `LogicBattleDeck::getLoadoutData` |
| `0xfec764` | 44 | `LogicBattleDeck::getCardForm` |
| `0xfec788` | 45 | `LogicBattleDeck::getCardForm` |
| `0xfec7ac` | 43 | `LogicBattleDeck::setFlexSlotCardForm` |

Native functions `0xfea43c`, `0xfea548` and `0xfea6b4` materialize the table
address, then store it at the incoming object's first eight bytes with
`STP X8, XZR, [X0], #0x10`. This is constructor-style vtable installation.
The combination of that store, virtual-dispatch uses and five explicit method
assertions strongly supports interpreting this particular run as the battle-deck
vtable. This remains an inference from stripped code rather than retained RTTI.
Other pointer runs in the general index remain merely vtable/dispatch candidates.

The native instructions establish these smaller operations without assigning
unproven high-level field names:

| Function RVA | Slot | Observed operation |
| --- | ---: | --- |
| `0xfeda80` | 3 | Return the 64-bit field at object offset `0x8`. |
| `0xfeda88` | 11 | Return the 32-bit field at offset `0x2c`. |
| `0xfeda90` | 28 | Return the 32-bit field at offset `0x4c`. |
| `0xfec180` | 36 | Swap two eight-byte entries in the array addressed by object offset `0x20`; indices use signed 32-bit extension. |
| `0xfec49c` | 31 | Find the first pointer equal to the argument in the array at offset `0x40`, with signed count at `0x4c`; return its index, or signed 32-bit `-1`. |
| `0xfec320` | 27 | True when every identity in the array at offset `0x40` is nonzero. A signed count below 1 returns true without reading the array. |
| `0xfec370` | helper | True when any identity in that array is nonzero. A signed count below 1 returns false without reading the array. This function is adjacent to the vtable body and is not one of the 46 slots. |
| `0xfec3c0` | 29 | Load an indexed eight-byte entry from the array at offset `0x40`; invalid index logs an assertion and still performs the load. |

The assertion behavior in `0xfec3c0` means its out-of-range path is unsuitable
to reproduce as an unguarded load. The Rust port accepts only an index inside
the signed count and the supplied slice. Whether the two arrays represent a
hand, a draw queue, or another deck view is not established by these
instructions alone.

`deck-operation-native-probes.json` records 1,024 controlled cases for each of
four functions (`0xfec180`, `0xfec49c`, `0xfeda88`, `0xfeda90`), for 4,096
original-instruction executions in Unicorn 2.1.4. Every result matches the
stated operation. Search tests include signed negative/zero counts, null values,
duplicate pointer values, present and absent targets. Swap tests use valid
allocated indices, including equal indices.

`deck-leaf-rust-differential.json` repeats those four operations and adds
`0xfec320`, `0xfec370`, `0xfeda80`, and the in-range path of `0xfec3c0`.
That is 8,192 original executions, each compared with `native-deck-probe`.
Every Rust result matched. The indexed-load cases stay inside the signed count;
the native assertion helper and its following unguarded load are not executed.
These checks verify the leaf operations, not full battle or card behavior.

```sh
cargo build --release --manifest-path native-engine/Cargo.toml --bin native-deck-probe
tools/reverse-env/bin/python tools/native_engine_deck_probe.py \
  --rust-binary native-engine/target/release/native-deck-probe \
  --out analysis/native-engine-index/deck-leaf-rust-differential.json
```

## Original native UI instance bindings

The exact names requested from the recovered SC5 UI are present in native code.
The index records their literal addresses, all bounded references, native function
intervals and nearby call instructions.

| Exact native UI name | Literal RVA | Referencing native function RVAs |
| --- | --- | --- |
| `HUD_player` | `0x2bd2b7` | `0x8d9d30` |
| `HUD_topRight` | `0x2f4125` | `0x8cf3b8` |
| `ability_ingame_button` | `0x32af9a` | `0x8b1430` |
| `NextSpellTimer` | `0x31e26f` | `0x8dc37c` |
| `elixir_bar` | `0x31e231` | `0x8da088`, `0x8dddb8` |
| `nextCard1` | `0x2bd2ad` | `0x8da088` |
| `ability_location` | `0x2f87c6` | `0x8cf3b8`, `0x8d9c64`, `0x8da088`, `0x8db8e0` |
| `cardArea` | `0x2f8735` | `0x8b7064`, `0x8b739c`, `0x8b768c`, `0x8b78b0`, `0x8d8898`, `0x8da088`, `0x8dddb8`, `0x8df9fc` |

At `0x8da73c`, the native function passes `elixir_bar` to function `0x1242f34`
and stores the returned pointer at object offset `0x1a8`. A second branch at
`0x8dab1c` performs the same lookup and store. The same shared function is
called with `ability_location` and `cardArea`. This is direct evidence for a
native named-child lookup/binding path; the helper's exact class and return
contract remain for decompilation.

`nextCard1` and `NextSpellTimer` are passed to another shared helper at
`0x1243194`. `HUD_topRight` is passed to `0x120cc98`, followed by a store to
object offset `0x150`. `ability_ingame_button` is first wrapped by the string
helper at `0x142fde4`, then passed to `0x88f45c`. These are concrete native UI
lookup/loading anchors, not substitute UI implementations.

## Simulation data types versus runtime classes

Exact recovered reflection literals include `logic.data.LogicCharacterData`
(`0x306352`), `logic.data.LogicProjectileData` (`0x330a3b`) and
`logic.data.LogicGameObjectData` (`0x30e9bb`). They have validated code references
in native data loading/reflection functions. Those names support data-type
registration and parsing anchors; they do not by themselves identify the
runtime troop, projectile update loop, collision system or damage handler.

Examples of focused function starts for decompilation:

- `0xde5c34` and `0xe35558`: reference `logic.data.LogicCharacterData`.
- `0xdca7f4` and `0xe5c31c`: reference `logic.data.LogicProjectileData`.
- `0xe31b38` and `0xe73634`: reference `logic.data.LogicGameObjectData`.
- `0xd46f40`: contains the explicit `LogicBattleBoostMode` component assertion.
- `0x8c3350`: contains `battle end hud: cannot find taunt area!`.

`LogicGameObjectFlowAsset` and other `autochess` assets can belong to Merge
Tactics. They must not be labeled ordinary Royale simulation without tracing
native users. This index deliberately leaves that distinction unresolved where
only a name is available.

## Search boundaries and limitations

The entire derived ELF contains nine `LogicBattle`, four `LogicCharacter`,
eleven `LogicGameObject` and three `LogicProjectile` substring occurrences.
`LogicBattleDeck::` occurs four times, with its `getCardForm` assertion shared by
two native functions. Exact substrings `LogicLevel`, `BattleView` and
`battle_controls` have zero occurrences in this binary. This does not establish
that corresponding concepts are absent or recover their stripped names.

Code references cover ADR and validated ADRP/ADD or ADRP/LDR sequences within
an eight-instruction linear window. Intervening writes to the page register and
branch boundaries are rejected. Longer computations, other relocation forms,
indirect lookups and computed dispatch are outside that bounded scan. Data
references use original `R_AARCH64_RELATIVE` addends. Names identify literal
contents, and all semantic conclusions above state which additional native
instructions support them.
