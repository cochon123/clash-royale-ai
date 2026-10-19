# Reverse-engineering ledger

Active work now targets the original native engine and original interface.
See [current native recovery progress](native-recovery-progress.md),
[decoded UI evidence](native-ui-recovery.md) and
[original flow recovery](native-flow-logic.md) for subsequent findings. The
ledger below also records the earlier independent Rust reconstruction and its
limitations. That prototype does not establish native engine equivalence.

## Evidence and identity

REA means the requested [morluto/rea](https://github.com/morluto/rea) tooling.
The following retained JSON files contain the actual evidence records.

| Evidence | Observed result | Boundary |
| --- | --- | --- |
| [`rea-xapk.json`](../analysis/rea-xapk.json), `ev_b17abe53eb10ebaed674c1e307ad2d4507d7a2d6d7b7da230255fe0a760f5394` | Package inventory identifies the base APK, ARM64 split and install-time asset pack. | Archive inspection, no Android execution. |
| [`rea-android-package.json`](../analysis/rea-android-package.json), `ev_a54964f26eb456937068850145026b4d3e9974713353325d0289d56ad4fe7aa6` | Package `com.supercell.clashroyale`, version name/code `160402020`; manifest launcher `com.supercell.titan.GameApp`. | Static JADX manifest/metadata; no runtime or signature verification. |
| [`rea-gameapp.json`](../analysis/rea-gameapp.json), `ev_a75fa3c0faaa5680a33e5f8144be68425d135b5d8a9bff242bc0f7fba720af15` | Class inventory lists `createGameMain`, `beforeLogicCallback`, surface and activity lifecycle methods. | Method inventory identifies an integration surface; it does not recover battle algorithms. |
| [`rea-libg-entry-decompile.json`](../analysis/rea-libg-entry-decompile.json), `ev_e7cf929879d44c2c1f3cf46a00d36e4a096172dc63ef2c635bb515cb5dfbada0` | Ghidra 12.1.4 decompiles `__DT_INIT` at file offset `0x1adbc20`, mapped address `0x1be3c20`. It calls `thunk_FUN_01be148c` and returns parameter-shaped data. | This is an initializer thunk only. No movement, targeting, combat loop, collision or spell algorithm was recovered by this decompilation. Pseudocode is not original source. |

The earlier [`rea-libg-entry.json`](../analysis/rea-libg-entry.json) records an
invalid file-offset request. It is a failed query, not supporting evidence for
native behavior. The corrected request above returned initializer pseudocode.

SHA-256 identities:

| Artifact | SHA-256 |
| --- | --- |
| Supplied `Clash+Royale_160402020_APKPure.xapk` | `3dc0085605d495ee40b443084f7767ecc547d329be30b5f11884b0d345643f13` |
| `analysis/apks/com.supercell.clashroyale.apk` | `0d0dfed74fdb755b71f5424e70af9234ed25471c029d09863b657fed9057b13e` |
| `analysis/apks/install_time_asset_pack.apk` | `3e62afcf73786919be08b0a48f1744d82b618642fed8e8db7d1a76b5b2eb3fd3` |
| `analysis/native/lib/arm64-v8a/libg.so` | `347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433` |

## Recovered resources

[`tools/extract_game_data.py`](../tools/extract_game_data.py) decodes all 645
`.csv`/`.toml` members of the supplied asset pack. Their five LZMA property
bytes and four-byte size become a standard LZMA-alone header by inserting four
zero bytes after the size. Each output is validated against its declared byte
length and UTF-8 encoding. This is reproducible resource recovery independent
of REA's native decompiler, not recovered original game source.

[`data/provenance.json`](../data/provenance.json) records the archive identity
and packaged/decoded SHA-256 for every resource. Selected combat entries also
carry source member paths, record names, CSV line numbers where applicable and
hashes. All 645 decoded files remain under `analysis/decoded` locally.

| Recovered/exported material | Count |
| --- | ---: |
| Base troop / building / other card records | 108 / 18 / 27 |
| Evolution / hero-form card records | 109 / 113 |
| Total raw card records | 375 |
| Raw character / building / projectile definitions | 147 / 35 / 91 |
| Raw area-effect / buff / extension definitions | 103 / 108 / 185 |
| Base-card catalog entries | 153 |
| Simplified playable cards / crown tower configurations | 97 / 2 |
| Original PNG pictures copied unchanged | 137 |

[`data/recovered_catalog.json`](../data/recovered_catalog.json) preserves raw
definitions and card records, including blank-name CSV continuation rows and
unused/event variants. Selected extension records follow their explicit
`EXT.Base` reference while retaining both source identities. Presence in a
package does not establish live availability.

[`data/rules.json`](../data/rules.json) retains globals, battle timelines,
game modes and rarity tables. [`data/card_catalog.json`](../data/card_catalog.json)
records whether a base card has a usable local representation, its omitted
mechanics and art provenance. Unsupported entries receive no invented combat
statistics. [`data/README.md`](../data/README.md) documents the export schema.

Observed examples include Knight `Hitpoints=690`, `Damage=79`, `HitSpeed=1200`
and `Speed=60`; Princess Tower `Hitpoints=1400`, projectile damage `50` and
`HitSpeed=800`; and King Tower `Hitpoints=2400`, projectile damage `50` and
`HitSpeed=1000`. The King Tower configuration explicitly waits for damage or
tower destruction and contains a 3,300 ms activation stage; the local engine
implements this activation wait. This does not establish equivalence of the
remaining native tower or battle behavior.

## Reconstruction choices and remaining uncertainty

The local engine consumes raw baselines. Displayed card-level scaling,
rarity offsets, upgrade rounding and current live balance are unverified.
Coordinates use an assumed 1,000 raw units per tile; movement uses the assumed
conversion `Speed * 1000 / 80` milli-tiles per second. Millisecond time fields
are interpreted from their names and resource context. Fireball's stored
`CrownTowerDamagePercent=-77` is interpreted as a 77% reduction; the original
native formula has not been traced.

Basic combat approximations omit or simplify native navigation, collision,
target selection, special actions, shields, charges, damage ramps, spawned
units, buffs, knockback and projectile patterns. Timed spells use one stored
damage tick or one second of stored DPS as immediate damage. Goblin Barrel
directly summons three recovered barrel goblins; Goblin Gang uses the primary
unit type for its combined count. Per-card `limitations` describe these choices.
The preset decks are authored local configurations, not claims that those
precise presets were recovered from the package.

Extraction was rerun and the six generated JSON exports compared byte for
byte. Checks confirmed 97 unique playable IDs, 153 unique catalog IDs, positive
troop/building HP, bounded elixir/count values, valid source hash references
and existing art paths. These checks verify extraction consistency. Original
Android runtime behavior has not been observed or compared to the Rust model.

## Request checklist

| Question | Status | Result / remaining work |
| --- | --- | --- |
| Identify the supplied game/package | Answered | XAPK inventory and Android manifest identify the target and version. |
| Use REA against the shipped artifacts | Answered | Archive inspection, Android metadata and a native initializer decompilation retained with evidence IDs. |
| Recover usable packaged data and card pictures | Answered within stated boundary | All 645 CSV/TOML resources decoded; 137 actual PNGs copied; source hashes retained. Subsequent SC5 display trees and textures are covered in the linked UI recovery report. |
| Build a Rust game suitable for local battles | Partial | An independent battle model uses recovered data; supported and approximate cards are identified. Local functionality does not establish original-game fidelity. |
| Recover native battle algorithms and special mechanics | Unresolved | Companion text decryption and the real engine-loader handshake are recovered; main-game code and battle algorithms remain unresolved. |
| Validate equivalent behavior against the original game | Unresolved | No matching original-runtime traces, replay differential tests or tick-level equivalence results exist. |
| Decompile the entire game and rewrite it faithfully in Rust | Unresolved | A complete binary/source rewrite has not been achieved; configuration recovery and a bounded reconstruction are the current result. |
