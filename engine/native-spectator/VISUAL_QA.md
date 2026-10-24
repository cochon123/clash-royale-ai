# Visual QA for the Arena renderer

`engine/tools/visual_check.py` answers "did my renderer change do what I meant,
and what else changed?" without playing a new game. It combines automated
checks with screenshots for review; passing the checks does not prove visual
correctness or official-client parity.

Recorded original-engine matches (`visual-fixtures/captures/`) are replayed
through the real presenter (`clash-visual-check`: the same `Painter` and the
same display lists `clash-web-scene` sends the browser) on an exact display-time
grid. The same code, asset bundle and fixture produce deterministic display
lists and pixels, so comparisons can isolate renderer changes. Each frame is then judged three ways:

1. **Rules** from original game data and continuity: wrong art family, art
   swapped between frames, pops, flicker, attacks without their clip,
   missing death/spawn effects, invisibility not shown. `visual_check.py rules`
   lists every rule, its severity and the data it is based on.
2. **Baseline comparison**: findings that are new, worsened, fixed or persisting, and
   every frame whose display list changed, attributed to the rows that changed
   (for example `Ghost: export ghost_red_overlay_run1_9 -> ghost_red_run1_9 x133`).
3. **Screenshots**: a frame viewer with entity boxes, per-entity filmstrips
   and thumbnail strips for entity findings, in `report.html`.

## Commands

Run from the repository root with the project Python (Pillow and NumPy):

```sh
PY=.venv/bin/python   # or the main checkout's .venv from a worktree
$PY engine/tools/visual_check.py run                 # quick suite, about 2 min with screenshots
$PY engine/tools/visual_check.py run --suite full    # 21 whole matches, data only, several minutes
$PY engine/tools/visual_check.py run --scenario ghost-lifecycle
$PY engine/tools/visual_check.py run --expect Ghost  # only Ghost may change
$PY engine/tools/visual_check.py accept --note "why this is the new reference"
$PY engine/tools/visual_check.py show --scenario ghost-lifecycle --row Ghost --side 0
$PY engine/tools/visual_check.py rules
```

`run` builds `clash-visual-check` (incremental `cargo build --offline`), runs
scenarios in isolated processes (`--jobs 1` by default), prints a verdict and writes
`.visual-check/runs/<time>-<label>/report.{html,json}`; `.visual-check/latest`
points at the newest run. `--assets DIR` (or `ARENA_ASSETS_DIR`) selects the
original asset bundle. Exit status: 0 PASS, 1 FAIL, 2 harness error.

The verdict FAILs on any new or worsened error finding (with `--strict`,
warnings too). Worsened means a higher severity or more occurrences of the
same finding. Without a baseline, `--strict` also fails on warnings. With
`--expect`, the selected scenarios must all have baseline traces on the same
display-time grid. With `--expect ROWS`, it also FAILs on any drawing change to a row not listed
(`effects` covers effects/HUD; `--expect none` means no visual change at all).
Without a baseline, any error finding fails.

## Workflow for an agent changing the renderer

1. **Baseline first.** Before editing, run the suite on the unmodified code and
   `accept` it, unless `.visual-check/baseline/report.json` already records the
   same `git.source` fingerprint as your starting point.
2. **Make the change**, then `run --expect <rows you intend to affect>`.
3. **Read the result, not just the exit code**:
   - the findings you targeted must be listed as *fixed*;
   - *changed rows* must match your intent; read their `details` (export,
     state, alpha or frame changes) in `report.json`;
   - open the filmstrips or `show` output for the affected rows and look at
     them. A rule passing is necessary, not sufficient.
4. **Iterate** until PASS. Do not edit rules, thresholds, scenarios or fixtures
   to make a renderer change pass. Change the checker only as a separate,
   explained task. Never `accept` a FAIL without a `--note` that says why the
   new behaviour is correct.
5. **Report** the verdict, the fixed, new and worsened findings, changed rows and the
   report path.

`--no-build` reuses the binary only when the source and executable content
fingerprints match the last harness build. This includes the local viewer
dependency, Cargo manifests and lockfiles. A stale binary once made a run
judge code that was no longer on disk.

To stream a run with completed-work and estimated-time-left columns:

```sh
$PY -u engine/tools/visual_check.py run --suite full > /tmp/visual-check.log 2>&1 &
tail -f /tmp/visual-check.log
```

`DONE` counts completed scenarios; `ETA` estimates the remaining wall time
from average scenario throughput. Estimates settle as more scenarios finish.

## Scenarios and fixtures

`visual-fixtures/scenarios.json` defines scenarios (capture, tick window, focus
rows, `fps`, `lag`, `images`) and suites:

- `quick`: nine focused windows with screenshots (Royal Ghost, Bandit and
  Electro Wizard, Battle Ram, Hog-mirror defence, air units, Night Witch,
  Guards and Tombstone, log bait, Mortar);
- `ghost`: the Royal Ghost window only;
- `full`: every capture's whole first match, data only.

Each fixture is a gzipped every-tick capture from the original native engine
(`frames.jsonl` snapshots plus `capture.json` metadata: seed, decks, played
cards, observed rows). Six are the Arena deck presets; fifteen `cards-NNN`
groups cover the 104 cards the engine accepted in eight-card decks. The replay
always starts at the first snapshot and draws every step, as the live presenter
does, and records only the scenario window. To add one, gzip a new every-tick
capture into `captures/<name>.jsonl.gz` with its `<name>.json`, then add a
scenario entry.

`lag` and `fps` default to the deployed presenter: one pose per native tick,
drawn on the newest snapshot (`lag 0`). A presenter with a fractional display
clock should set its own `fps`/`lag` in a scenario.

## What it cannot tell you

- **Official parity.** There are no official client frames here. Rules encode
  original data (logic rows, authored clips, labels, effects) and continuity,
  not the real game's exact look. Thresholds such as "invisible means at most
  0.6 of normal visible mass" are presentation judgements, stated once at the
  top of `visual_check.py`.
- **Browser interpolation.** The display lists are exactly what the browser
  receives. How `scene-renderer.js` interpolates between them is covered by
  `webapps/arena/test_scene_renderer.cjs`, not here.
- **Inferred engine state.** The engine does not export animation state, buffs
  or attack flags. Hits come from the Painter's own inference (projectile
  births and melee HP loss), so rules about attack timing judge the Painter
  against those observations.

## Checking the checker

```sh
$PY -m unittest discover -s engine/tools -p test_visual_check.py -v
cargo test --offline -j 1 --manifest-path engine/native-spectator/Cargo.toml --lib -- --test-threads=1
node --test webapps/arena/test_scene_renderer.cjs
```

Synthetic traces exercise wrong-family art, area jumps, missing effects,
invisibility, flicker, clip timing, exact drawing differences, draw order,
baseline verdicts, build provenance and memory limits. The Rust regression checks both Ghost
teams and all animation states against the original bundle. The browser tests
cover display-list interpolation and rendering. Use the original asset bundle
required by the spectator tests.

## Memory budget

Use Python 3.11 or newer. The default is one scenario at a time, with a fresh
worker for each scenario so parsed traces and asset caches are released.
On Linux, `--memory-gb 4` caps the address space of each worker and its replay
child. The runner refuses worker budgets that would leave less than 4 GiB of
currently available RAM for the desktop. Increase `--jobs` only when that
budget fits. Keep the asset-heavy Rust tests serial with `--test-threads=1`,
and run them separately from replay suites. The earlier four-way replay plus
parallel Rust tests exhausted this machine's RAM (the kernel killed the Rust
test process at approximately 3.7 GiB resident memory). Accepted baselines are
independent file copies so later edits to a run cannot change the reference.

## Verified example (2026-10-22)

The Ghost export selector was accidentally accepting `ghost_*_overlay_run*`
from the attached GhostOverlay family. Prefer the body family's direct action
exports. The screenshot suite against the pre-fix baseline produced:

- 2,708 frames across nine scenarios: regression **PASS**, eight fixed findings,
  zero new or worsened findings; only Ghost drawing changed.
- 271 Ghost frames repeated: the entire trace and every lossless PNG were
  byte-identical, and `--expect none` passed.
- 21 whole-match captures / 43,983 frames: completed in 272 s with one worker.
  The unbaselined verdict remains **FAIL**, with 167 unresolved-art findings
  and five missing-logic findings (including Ghost). These are remaining
  presentation/data gaps, not eight additional Ghost fixes. There were no
  missing-death-effect findings with the correctly rebuilt binary.
- Checker tests: 20 passed. Rust library: 29 passed, three ignored contact-sheet
  utilities. Browser scene renderer: 23 passed. Serial Rust tests peaked at
  772 MiB RSS and finished in 13 s, measured with `/usr/bin/time -v`.

Lossless frames, filmstrips and generated report JavaScript were checked.
Live report UI inspection in this session was blocked by T3's browser sandbox
setup on the host (AppArmor); the fallback browser was unavailable. The HTML
report can still be opened in an ordinary browser.
