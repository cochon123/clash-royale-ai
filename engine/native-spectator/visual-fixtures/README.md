# Visual QA fixtures

Gzipped every-tick snapshots of deterministic original-engine matches, replayed
by `engine/tools/visual_check.py` (see `../VISUAL_QA.md`). Each `<name>.json`
records the seed, both decks, played cards and observed rows of `<name>.jsonl.gz`.

These were recorded on 2026-10-22 with the original native engine, the pinned
`oracle-export-01` checkpoint and scripted card plays (no model). They are engine
inputs only: no original artwork is stored here.

- `custom-mortar`, `golem`, `hog-mirror`, `lava-loon`, `log-bait`, `pekka-bridge`:
  the six Arena deck presets, 1600 ticks each (seeds 7921-7926).
- `cards-000` ... `cards-112`: fifteen eight-card groups covering the 104 cards the
  engine accepted, up to 3600 ticks each (seeds 7921-7935).
