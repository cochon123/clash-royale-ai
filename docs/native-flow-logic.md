# Embedded original game-flow logic

The supplied asset database contains original executable graph descriptions
and declarative action chains. Recovery produced 2,458 original JSON assets
under `analysis/native-flow/assets/`, including 685 game-object/player flow
documents. They are original game data, not reconstructed rules. Executing
them still requires the native handlers that implement their operations.

The major boundary is mode coverage: all 509 `LogicGameObjectFlowAsset`, all
176 `LogicPlayerFlowAsset`, all six `LogicGameObjectBehaviorFSMAsset`, and
all plugin object/ability/buff assets belong to `logic/autochess/`. They do
not recover ordinary Royale troop movement, targeting or king-tower logic.
Another 477 `LogicChaosDataAsset` documents extend Royale tables and name
native action classes. Those contain useful original spell and troop action
chains, with the base engine still implemented natively.

## Reproduce and provenance

```sh
python3 tools/flow_recover.py
```

This uses Python's standard library and opens the database read-only. The
source SHA-256 is
`792abc4536d56a4e7dc11fb825b578b4d6b3da43afa19747411f48d8d504c882`.
Every selected source JSON is copied byte for byte. The helper checks that
the source digest remains unchanged after recovery.

The recovery outputs are:

- `asset-index.json`: selected original paths, types, UUIDs, formats, byte
  counts and source hashes.
- `database-record-index.json`: all 16,060 database records and both blob
  classifications, lengths and hashes, including records not extracted as
  logic.
- `graph-index.json`: 702 documents with explicit node/state arrays.
- `flow-activity.json`: all 685 flow documents with node/action counts;
  this avoids mistaking empty assets for implemented behavior.
- `operation-index.json`: 217 numeric operation shapes, scoped by owning
  asset type and operation context, with exact JSON examples and pointers.
- `native-class-index.json`: 57 encoded `ClassType` names, their field sets,
  counts and exact original examples.
- `component-index.json`: 13 component types used by plugin objects.
- `expressions.json`: 1,024 original formula strings with source pointers.
- `asset-references.json`: 6,615 references resolved to paths in this database.
- `observed-schemas/`: observed JSON field types per asset type. These are
  field observations, not an official schema, default-value map, or proof
  of operation semantics.

## Troop attack: concrete original graph

The Merge Tactics archer provides a linked example:

1. `logic/autochess/character/archer/archer` declares components `Ability`,
   `AutoChessMovement`, `Behavior`, `Hitpoint`, `Transform`, `Buff`,
   `Attribute`, `CircleCollision` and `Display`.
2. Its `AbilityBaseRef` points to
   `logic/autochess/character/archer/ability/archer_attack`.
3. The ability references `enemy_all`, `autochess_ability_attributes`, and
   `archer_attack_flow`. Its stored damage is 100, range is 3, speed is 90,
   critical rate is 15 and critical value is 50. These values belong to
   that mode's attributes; their units and rounding must come from the
   native attribute implementation.
4. `archer_attack_flow` encodes the following graph:

```text
1000 StartNode -> 1001 WindUp -> 1002 Loop -> 1006 CheckPowerShot
                                                 true  -> 1007 WindDownPower
                                                 false -> 1003 WindDown
1003 / 1007 -> 1004 CoolDown -> -1
2000 OnHit -> 1005 DurationNode -> -1
```

The `WindUp` duration is stored as 400 and `WindDown` as 600. The native time
unit is not established by this static graph alone. `WindDown` contains
action type 8 with the original `ProjectileAsset`, `InitHeight: 500` and
`CustomProjectileCount: {"Value":1}`. It also contains action type 39 with
`ability_phase: "PHASE_WINDDOWN"`. Its condition type 20 references
`archer_damage_buff`; that branch chooses the original super projectile.

`logic/autochess/character/archer/projectile/archer_attack_projectile_flow`
has `StartNode -> dohurt`, containing action type 1 with `HitVFX` and
`DamageExpression`. It also has an `OnHitNode` supplying variable `dmg` to
an action type 17 with `HPAbsorbRawValue`.

These are substantive original behavior descriptions. They do not include
the native implementations for projectile motion, hit detection, damage
evaluation, critical hits or attribute lookup.

## Movement and target selection

`logic/autochess/fsm/shared/normal_character` is the original `NormalCharacter`
FSM referenced by the archer blueprint. Its `Move` state has ID 6 and
contains these exact callbacks:

```json
{
  "onEnter": [
    {"type":11,"obj":{}},
    {"type":7,"obj":{"BuiltInAnimNameID":"None","AnimName":"Walk"}},
    {"type":18,"obj":{"signals":[700869030]}}
  ],
  "onExit": [
    {"type":18,"obj":{"signals":[700869031]}},
    {"type":16,"obj":{"m_stopMove":true,"m_stopForceMove":false}}
  ]
}
```

The `Idle` state's updates include callback type 8 with
`m_candidateAbilitySlots: [2,0]`, type 9 with `m_refreshInterval: 500`, and
type 14 with an empty object. The pathfinding/grid/collision implementation
is not in this graph. Callback type 11's empty payload also means its
function cannot be inferred from that payload alone.

An explicit selection configuration occurs in
`logic/autochess/character/goblin_machine/ability/goblin_machine_attack_flow`:

```json
{
  "node_action_type":26,
  "node_action_obj":{
    "m_track":1,
    "TargetAlgorithm":1,
    "TargetCount":2,
    "RangeAsset":"logic/autochess/ranges/circle_all | 39ddd5c6-eec5-60db-4e86-2bc5e80dcd55",
    "FilterAsset":"logic/autochess/filter/enemy_all | 3508bc9a-0f82-622f-2fda-a8aba282f0ef"
  }
}
```

The reference `logic/autochess/filter/enemy_all` stores `{"Filter":4140}`.
The algorithm enum meaning, filter bit meaning, tie ordering, target validity
and refresh behavior remain native implementation questions. The index
does not assign invented meanings to numeric algorithm 1 or filter 4140.

## King activation: unresolved for ordinary Royale

No standard king-tower activation graph was identified in the recovered
flow/FSM categories. Their complete path inventory is limited to autochess.
The similarly named
`logic/autochess/rulers/king/passive_ability/king_passive_1_flow` is a ruler
passive concerning round outcomes and resource rewards. It includes
`isWinner`, `isTie`, `sp_roundwinnerindex`, and `random(4)+4` stored as
`elixirgain`. It is not evidence of a Royale king tower activating on damage.

Searching the extracted logic JSON for activation/wakeup terms produced
other behavior such as a goblin hut waiting action, not a king activation
implementation. This is a bounded asset search; the behavior can remain in
`libg.so` or other original tables.

## Spells: original Royale extension action chains

The recovered
`logic/chaos_arena/game/fireball_chaos_meteor_impact` is substantive Royale
extension data. Its `data.EXT.FireballSpell_chaos_meteor_impact` inherits
`PROJECTILE.FireballSpell`, sets `Scale: 200`, `Damage: 400`, and sets
`OnTargetReachedAction` to `FireballSpell_chaos_meteor_spawn_children`.

That action has `ClassType: "ActionGroup"`, six named subactions and six
zero delays. Each child is an original `ActionSpawnToLocation` with
`SpawnType: "ProjectileType"`. The first child's fields are:

```json
{
  "ClassType":"ActionSpawnToLocation",
  "SpawnType":"ProjectileType",
  "SpawnData":"FireballSpell_chaos_meteor_child_1",
  "ParentGOAsSource":true,
  "XPositionExpression":"x + 1000",
  "YPositionExpression":"y",
  "TargetExprX":"x + 3000",
  "TargetExprY":"y"
}
```

The child inherits `PROJECTILE.FireballSpell` and sets `Damage:200`,
`Radius:1500`, `Speed:400`, `Gravity:30`, a hit effect, and a further
`OnTargetReachedAction` chain. This provides the original authored spell
composition. The base `FireballSpell` implementation, expression arithmetic,
projectile physics and collision remain native dependencies.

The 57 named classes also include `ActionTakeDamage`, `ActionResetPath`,
`ActionSetTargetFromContext`, `ActionDashingAttackChain`,
`ActionGoblinDrillEvoRelocate`, `ActionMegaKnightUppercut`, `ActionKnockback`,
`ActionTaunt`, and `ActionHeal`. Their exact names and configuration fields
are recovered; their native bodies are unresolved.

## Operation dispatch and expression boundary

The numeric operation namespace is not global. For example, game-object
flow action type 1 has damage fields, while player-flow action type 1 has
`Expression` and `Result`. `operation-index.json` therefore keeps
`asset_type`, `category` and `encoded_type` together. Omitted operation
types remain `"<omitted>"`; their runtime defaults are not invented.

Representative game-object action field families are:

| Encoded type | Observed fields | Required native behavior |
| --- | --- | --- |
| 1 | `DamageExpression`, `HitVFX`, `TrueDamage` | damage evaluation/application |
| 3 | `ModifyAttrName`, `ModifyValue`, `ModifyValueType` | attribute modifiers and reset rules |
| 6 | `Expression`, `Result` | expression interpreter and variable storage |
| 8 | `ProjectileAsset`, `InitHeight`, `CustomProjectileCount` | projectile creation |
| 9 | `AOEAsset`, `AOEDuration` | area effect creation and target updates |
| 10 | `ForceMoveSpeed`, `ForceMoveRange`, `ForceMoveHeight` | forced movement |
| 18 | `JumpDuration`, `JumpHeight`, `JumpToEmptyCell` | grid movement/jumping |
| 26 | `TargetAlgorithm`, `FilterAsset`, `RangeAsset` | target selection |
| 39 | `ability_phase` | ability phase bookkeeping |

The behavior descriptions in this table are inferences from original field
names, not recovered handler bodies or a complete enum map. Expression
strings call `bb`, `percent`, `attr`, `abi`, `random`, `attrbase`, `abibase`,
`gridcellindex`, and `min`. Their parsing, fixed-point/overflow rules,
percentage rounding, RNG algorithm and variable scopes require the native
interpreter to be recovered before a faithful Rust executor can be written.

## Empty graphs and binary-bytecode search

There are 2,328 encoded actions in game-object flows and 1,257 in player
flows. Three game-object flow documents have no `nodes` field. Nine have
one terminated node and no further graph. Examples include
`gold_dummy_attack_flow` and `goblin_machine_stun_buff_flow`. These assets
must not be counted as implementations of the behavior suggested by their
filenames. Their emptiness may be intentional; static data does not explain
why.

All 16,060 `asset` records declare storage format 0. Of those, 9,472 have no
embedded data, 6,558 parse as JSON, and 30 are UTF-8 `AssetPrefabTimeline`
documents containing nonstandard `Inf`/`-Inf` numeric tokens. Replacing only
those tokens outside strings with JSON-parser-supported infinity tokens
allows all 30 to parse. Their exact original bytes are retained in
`database-exceptions/`; no rewrite is applied to the source.

The `meta` table contains 4,471 null data values, 11,280 copies of the exact
seven-byte descriptor `00 00 01 00 00 24 01`, and 309 larger binary metadata
blobs. All 309 belong to `FMODSoundBank`; their prefix is `events` followed
by a binary record structure with sound event names, event paths and UUIDs.
All distinct original metadata blobs are preserved by hash under
`metadata-blobs/`.

No embedded machine-code/VM-bytecode payload was identified in these blob
classifications. This does not establish that the game lacks bytecode,
that external assets contain none, or that native logic was recovered.
The original graph instructions and named native classes still dispatch
into code outside this database.
