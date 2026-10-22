# Original native UI asset recovery

The supplied Clash Royale asset APK contains recoverable original UI display
trees, texture atlases, embedded UI layouts, animation timelines and fonts.
These artifacts are now decoded under `analysis/native-ui/`. This is asset
recovery, not a running original Clash Royale client or a finished Rust port.

## Reproduce

From the project directory:

```sh
bash tools/ui_setup.sh
.venv-ui/bin/python tools/ui_recover.py
```

The setup creates an isolated Python environment and installs
[`obus-globus/sc5-parser`](https://github.com/obus-globus/sc5-parser) at commit
`9108080256a0df6cd772d81afdbc26047d79d78d` with its decoding dependencies.
`tools/ui_recover.py` keeps that package untouched and applies the two required
compatibility corrections in memory.

## Evidence and recovered files

`analysis/native-ui/recovery-manifest.json` records the supplied APK SHA-256,
individual SC/SCTX source SHA-256 digests, tool versions, decoded dimensions,
object counts and sample rendering results.

| Source display document | Shapes | Movie clips | Text fields | Named exports |
| --- | ---: | ---: | ---: | ---: |
| `ui.sc` | 3,087 | 4,871 | 2,695 | 2,079 |
| `ui_ability_button.sc` | 10 | 19 | 6 | 1 |
| `ui_deck_builder.sc` | 111 | 161 | 51 | 47 |
| `ui_spectate_hud.sc` | 37 | 59 | 22 | 9 |
| `ui_battle_end.sc` | 265 | 165 | 77 | 31 |
| `ui_arena.sc` | 240 | 216 | 15 | 110 |

The `display-trees/` JSON documents retain named export IDs, child names and
IDs, frame element indices, transforms, colors, mask modifiers, text metadata,
vertices and texture references. They provide concrete data for a Rust display
list renderer rather than an artist's reconstruction of the controls.

Examples observed in the decoded `ui.sc` data:

- `HUD_player`, object 1745, binds children named `slots`, `elixir_bar`,
  `elixir_warning`, `cardArea`, `nextCard1`, `TID_NEXT_CARD`, `NextSpellTimer`
  and `ability_location`.
- `HUD_topRight`, object 1660, contains `timeLeft`, `TID_TIME_LEFT`, and
  `elixirRegen`, with `out_start` and `out_end` frame labels.
- `ui_ability_button.sc` exports `ability_ingame_button`, object 39: 52 root
  timeline frames at 30 FPS. Labels occur at frame 0 (`appear`), 32 (`loop`),
  33 (`disappear`) and 51 (`end`). The extracted WebP renders these original
  root frame transforms; descendant state selectors stay at their default
  frame.

The `atlases/` PNGs are decoded original ASTC texture atlases. `rendered/`
contains examples composited from original polygons, UVs, matrices, colors
and clip structure. `recovered-controls.png` is a labelled contact sheet.
The original six bundled TrueType fonts are retained under `fonts/`.

The APK's `assets/assets.scdb` is an SQLite database, not an opaque encrypted
asset store. Its `asset` and `meta` tables join on `path`. It contains 162
`AssetUISchema` blobs and 155 `AssetUITimeline` blobs. All 317 decode as JSON;
their exact original bytes are saved under `schemas/`, indexed with SHA-256
digests in `ui-schema-index.json`. Most of these newer documents concern
autochess; the ordinary battle HUD remains in SC display documents and native
code. Examples include `ui/common/time_bar`,
`ui/home/home_social_screen`, and
`ui/popup/popup_friendly_challenge`. Layout documents preserve component
types, bounds, anchors, resource references, inheritance/diff records and
native `code_class` bindings. Timeline documents preserve animation property
names and key data.

## Format corrections backed by primary upstream readers

The source SC files begin with `SC` followed by little-endian version 5. Their
FileDescriptor precedes the Zstandard resource stream. Texture references
point to external `.sctx` files.

The pinned Python parser incorrectly subtracts one from string indices.
The original files contain an explicit empty string at index zero. The
[SupercellFlash C++ reader](https://github.com/sc-workshop/SupercellFlash/blob/41e894d5a20cc17e47fe32db3106c4c1bec60a3e/supercell-flash/source/flash/objects/ExportName.cpp)
uses direct zero-based lookups, as do its `MovieClip.cpp` and `TextField.cpp`.
Correcting all six Python lookup sites fixes export names, child instance
names, text metadata and frame labels. Without this correction, names were
silently shifted and the one-export ability document appeared to have no
exports.

The main `ui.sc` atlases use SCTX pixel type 208, which the pinned parser does
not support. The primary
[SupercellTexture pixel enum](https://github.com/sc-workshop/SupercellTexture/blob/ebd2b0e91f3114e40453c99d26d52cdb0d3cbf2c/supercell-texture/source/texture/backend/Supercell/ScPixel.hpp)
defines 208 as ASTC RGBA 6x6. The helper decodes it using
`texture2ddecoder.decode_astc` with a 6x6 block size and checks the expected
decompressed block byte count. The other selected UI atlases use type 212,
ASTC 8x8. This correction recovers all six main UI atlases, including three
4096 by 4096 textures.

## Rust display-object playback

`native-ui-viewer/` is a separate Rust crate implementing display-list
playback from the recovered original data. It reads display-tree JSON and
atlas PNGs; its native Rust software renderer traverses original movie-clip
frame elements, applies affine matrices and colors, rasterizes original
triangle strips/UVs, and composites mask groups and blend modes. Component
PNGs produced by Python are not inputs to that renderer.

```sh
./native-ui-viewer/run.sh --document ui --export HUD_player
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button --play
```

It can also export PNGs entirely without a display:

```sh
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button \
  --frame 32 --render analysis/native-ui/rust-rendered/ability.png
```

`tools/ui_compare_rust.py` independently renders the original SC files
through the corrected Python parser and compares the Rust pixels and
origins. All 56 cases match every RGBA channel and canvas origin exactly:
the original player HUD, spectator HUD, two mask-bearing exports and all
52 ability-button root frames. This establishes reference raster parity,
not parity with the executing official client. Its results are in
`analysis/native-ui/rust-verification/comparison.json`.

The desktop window's title explicitly calls it an asset viewer with no
battle client and reports skipped text fields. The original 52-frame root
timeline plays at its encoded 30 FPS; descendant states are held at their
defaults or selected explicitly with `--label` or `--object-frame`.
Native text/layout, nine-slice resizing, game-state bindings and original
runtime parity remain explicit gaps. See `native-ui-viewer/README.md` for
commands and implementation limits.

## Boundaries and remaining work

Static asset recovery establishes the original assets and structure. It does
not establish the runtime rules used to position, resize, show/hide or update
the controls. Raw first-frame renders include authoring placeholders and
overlapping state children because native game code normally selects and
populates them. The helper does not invent those decisions.

The upstream rasterizer renders shapes, masks, colors and affine transforms.
It does not render the original text fields or reproduce the client's
layout, input, resource loading or localization systems. Fonts and text
metadata are retained for subsequent implementation. The selected six SC
documents have no external matrix banks, so the parser's unimplemented
external compressed-matrix codec does not affect these selections.

Running the actual native UI requires executing the original Android client,
or recovering its bindings and implementing the corresponding renderer,
layout and input behavior in Rust. None of the PNGs, parsed trees or gallery
images should be described as an already working original native UI.
