# Recovered configuration data

Run `python3 tools/extract_game_data.py` from the project root. Python 3.11 or
newer is sufficient; no third-party packages or Android execution are needed.
The default input is the supplied `analysis/apks/install_time_asset_pack.apk`.
The script decodes every `.csv` and `.toml` member into `analysis/decoded`.
It also copies genuine packaged PNG card pictures unchanged into `web/cards`.

The compressed resource header contains five LZMA properties followed by a
four-byte little-endian output size. Adding four zero bytes after that size
produces a standard LZMA-alone header. Decoding validates both UTF-8 and the
declared byte length. Archive paths, SHA-256 digests of packaged and decoded
members, and extraction coverage are recorded in `provenance.json`.

## Exports

| File | Contents |
| --- | --- |
| `cards.json` | 97 cards usable by the simplified Rust engine; recovered combat values with explicit assumptions and omitted mechanics. |
| `towers.json` | Two crown tower configurations using the same combat schema. |
| `card_catalog.json` | All 153 base card records, including unsupported entries; stable IDs, internal names, support status, limitations, and original art provenance. |
| `recovered_catalog.json` | All 375 card records including evolution and hero variants; raw character, building, projectile, area-effect, buff, and extension definitions; original TOML documents. |
| `rules.json` | Raw globals, battle timelines, game modes, and rarity tables, plus reconstruction assumptions. |
| `provenance.json` | Input identity, all 645 decoded resource hashes, export counts, and search boundaries. |

`cards.json` is an array. Required combat fields are `id`, `name`, `kind`,
`elixir`, `hp`, `damage`, `attack_interval_ms`, `range`, `speed`, `count`,
`target`, `flying`, `radius`, `splash_radius`, `lifetime_ms`, and `deploy_ms`.
Additional fields include `can_deploy_enemy_side`, `kamikaze`, `sc_name`,
`raw_record`, `raw_speed`, `support_status`, `limitations`, `sources`,
`reconstruction_defaults`, and optional `art` and crown-tower damage fields.
An art path such as `cards/knight.png` is relative to the web root.

## Evidence boundaries

These are raw configuration baselines, not verified statistics at a displayed
card level. Several Rare or Legendary cards reference Common character or
projectile scaling. No rarity-level or upgrade multipliers are applied.
The engine uses an assumed 1,000 distance units per tile and converts raw
`Speed` to milli-tiles per second as `Speed * 1000 / 80`. Native movement and
rounding behavior have not been recovered. Time fields are interpreted as
milliseconds from resource context.

Blank CSV cells remain absent. Blank-name CSV rows are preserved as
continuations of their preceding named record. Raw definitions are retained
without arbitrary merging; selected playable extension records resolve their
explicit `EXT.Base` reference and preserve both source identities.

`supported` means the selected basic combat representation uses stored values.
It does not establish behavioral equivalence with the original game.
`approximate` adds explicit omissions such as death spawns, champion abilities,
shields, damage ramps, knockback, buffs, jumping, or multiple projectile waves.
Timed spell damage is flattened to one stored damage tick or one second of
stored DPS. The Goblin Barrel directly spawns three recovered barrel goblins;
its projectile travel is omitted. A mixed Goblin Gang uses the primary goblin
type for its total stored count. Unsupported cards retain their catalog entry
and reason instead of receiving invented combat values.

The decoded King Tower contains an explicit inactive action, activation on
damage or tower destruction, and a 3,300 ms activation stage. The local engine
implements that activation wait. Raw `CrownTowerDamagePercent`
values such as Fireball's `-77` are retained; interpreting this as 23% damage
is a reconstruction assumption unless native code confirms the formula.

The JSON exports deliberately exclude the application's full binaries and
original resource bundles. Their source identity still appears in provenance.
