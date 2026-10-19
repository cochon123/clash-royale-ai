# Original Clash UI display-object viewer

This crate plays the **original recovered SC5 display objects in Rust**.
It is an asset viewer. It does not run Clash Royale's battle client or engine.

The renderer reads the recovered display-tree JSON and atlas PNGs. Rust
performs movie-clip traversal, original affine transforms, UV triangle-strip
rasterization, color transforms, mask groups and blend compositing. It does
not display pre-rendered Python component images. Python is only needed for
the preceding asset recovery and the independent comparison check.

Build and open the original ability button animation:

```sh
cargo build --release --manifest-path native-ui-viewer/Cargo.toml
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button --play
```

Open the original player hand/elixir display:

```sh
./native-ui-viewer/run.sh --document ui --export HUD_player
```

Window controls are Space to play/pause, Left/Right to step the root timeline,
Tab/Shift+Tab to browse original exports, and Escape to close. The root uses
its encoded frame rate. Descendants stay on their default authored state
unless selected explicitly; game code normally drives those selectors.
Playback loops the complete root timeline, including its appearance and
disappearance frames. It does not infer the client's state transitions.

Inspect original instance names and timeline labels:

```sh
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button --inspect
./native-ui-viewer/run.sh --document ui_deck_builder --list
```

Render original states without a desktop or graphics driver:

```sh
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button \
  --frame 32 --render analysis/native-ui/rust-rendered/ability.png \
  --metadata analysis/native-ui/rust-rendered/ability.json

# Select the original child state named "not_enough_elixir".
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button \
  --frame 32 --label not_enough_elixir --render analysis/native-ui/rust-rendered/ability-unavailable.png

# Object 38 is the original button's child named "states"; frame 22 is "cool_down".
./native-ui-viewer/run.sh --document ui_ability_button --export ability_ingame_button \
  --frame 32 --object-frame 38:22 --render analysis/native-ui/rust-rendered/ability-cooldown.png
```

Default inputs are the `analysis/native-ui/` directory beside this crate.
Use `--assets DIR` to supply another recovery directory. Recover missing
inputs with `bash tools/ui_setup.sh` followed by
`.venv-ui/bin/python tools/ui_recover.py` from the project directory.

## Verification

```sh
.venv-ui/bin/python tools/ui_compare_rust.py
```

The helper loads the original SC files through the independent corrected
Python parser, renders reference frames, then compares all RGBA channels and
canvas origins with the Rust outputs. It covers the original player HUD,
spectator HUD, shield health-bar mask, blue ingame panel, and all 52 frames
of the ability-button root timeline. Results and original-source hashes are
written to `analysis/native-ui/rust-verification/comparison.json`. The
completed run matched all 56 cases exactly, including every RGBA channel
and original canvas origin.

The desktop window was also launched and closed after 120 frames using
`--window-frames 120`; its final metadata is retained under
`analysis/native-ui/rust-rendered/desktop-playback.json`.

This establishes parity with the independent static asset reference. It is
not evidence of parity with an executing official Clash Royale client.

## Explicit gaps

- Native text fields remain skipped by default for static reference parity.
  The library's opt-in `Playback.render_text` renders source text fields with
  extracted original fonts; `text_values` and `hidden_objects` bind values and
  visibility for the battle spectator. Exact native client text layout remains
  unverified.
- Game code must populate portraits, counters, localization and card data,
  select child states, control visibility, position controls and handle input.
- Nine-slice scaling grids remain in the source documents; resizing/layout
  behavior is not implemented by this renderer.
- Color-transform inheritance and pixel sampling follow the independent
  asset reference. Exact original native rendering behavior remains a
  runtime comparison task.
- Raw authoring placeholders and default state children can overlap. The
  viewer preserves the encoded data rather than fabricating gameplay values.

The PNG export path uses software rendering and works without X11. The
optional desktop window currently uses minifb's X11 backend (including
XWayland). Third-party provenance and licenses are in `THIRD_PARTY.md`.
