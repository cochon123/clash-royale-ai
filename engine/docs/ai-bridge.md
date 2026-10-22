# Trained AI subprocess bridge

`tools/policy_bridge.py` runs the existing checkpoints from the repository root
(`../models`, `../src`). It neither copies their weights nor modifies them. The battle simulation and server are Rust; trained model inference
currently remains Python/PyTorch.

```sh
python3 tools/policy_bridge.py --list
python3 tools/policy_bridge.py --model v4.4.1
```

The bridge automatically uses `../.venv/bin/python` if the
invoking Python does not have PyTorch. Set `CLASH_AI_REPO` or `--ai-repo PATH`
to use another checkout. CPU inference uses one PyTorch thread per process.
The server should create one persistent subprocess for each policy player.

Supported local artifacts, checked with real CPU inference:

| Model ID | Artifact relative to AI checkout | Status |
| --- | --- | --- |
| `v9` | `models/policy_v9/final_model/best.pt` | Experimental; the source report says its promotion gate was missed |
| `v4.4.1` | `models/policy_bc_v4.4.1/best_model.pt` | Legacy policy with selected-card placement |
| `v4.4.2` | `models/policy_bc_v4.4.2/best_model.pt` | Legacy policy with selected-card placement |

`--list` returns a JSON object with `models`, `python`, `script`, and `ai_repo`.
Each model has `id`, `name`, `available`, `checkpoint`, and `experimental`.
Availability checks files; subprocess startup verifies actual checkpoint loading.
No heuristic policy is substituted if model loading or prediction fails.

## JSON-lines contract

Startup emits `{"ready":true,"model":"v9"}` once loading succeeds. A loading
failure emits `{"ready":false,"error":"..."}` and exits nonzero. The first
request must follow this handshake. All subsequent stdout lines are JSON replies;
diagnostics and upstream library messages go to stderr.

Send one complete JSON object per line:

```json
{
  "model": "v4.4.1",
  "match_id": "local-1",
  "team": "blue",
  "elapsed_ms": 3000,
  "elixir": 6,
  "deck": ["hog_rider", "musketeer", "ice_golem", "skeletons", "ice_spirit", "cannon", "fireball", "the_log"],
  "opponent_deck": ["knight", "archers", "goblins", "giant", "minions", "spear_goblins", "arrows", "zap"],
  "hand": ["hog_rider", "musketeer", "ice_golem", "skeletons"],
  "history": [{"team": "red", "elapsed_ms": 1000, "card": "knight", "x": 4500, "y": 9000}]
}
```

`model` defaults to the command-line selection. Each deck has eight distinct
cards; the hand has four cards in its current UI order. `history` contains all
confirmed deployments in chronological order, including both teams. Never send
future events. All positions use absolute simulator coordinates, in 1,000 units
per tile: x 0–18,000, y 0–32,000. Blue starts at high y; red starts at low y.

Example action reply:

```json
{"wait":false,"hand_index":0,"card":"hog_rider","x":3000,"y":16889,"delay_ms":1607,"model":"v4.4.1","requery_after_ms":500}
```

`hand_index` is zero-based. `delay_ms` is model timing relative to the request's
`elapsed_ms`. The server schedules it in simulation time and invalidates that
proposal when a new confirmed deployment changes history. Model positions are
returned as predicted: the Rust engine must enforce its placement rules before
execution. These policies can propose placements outside the legal deployment
zone. Any position adjustment is a simulator execution decision.

No affordable card, a model clock waiting for an opponent event, or an unsupported
ability produces `{"wait":true,"reason":"...","requery_after_ms":500}`.
Invalid input and inference failures return `{"wait":true,"error":"..."}`;
the process remains alive for subsequent requests. The caller must surface errors.

## Observation and card mapping

`data/policy_card_aliases.json` records explicit engine snake_case IDs, recovered
Supercell internal names, numeric card IDs, and the replay policy's kebab-case
IDs. For example, `archers` / `Archer` map to `archers`, `mini_pekka` /
`MiniPekka` map to `mini-pekka`, and `the_log` / `Log` map to `the-log`.
Unknown mappings and cards missing from the checkpoint vocabulary fail explicitly.
Evolutions and champion abilities are not silently converted into base cards.

The replay encoder uses blue/team at low y, the opposite of this simulator.
The bridge transforms history using `replay_y = 32000 - simulator_y` and maps
predictions back into the simulator frame. Red uses the encoder's opponent
perspective, which performs a second y reflection. x is unchanged. The smoke
artifact checks that empty blue/red observations produce mirrored outputs.

The trained observations encode deployment history and estimated elixir. They do
not consume this simulator's exact troop health, targeting, deaths or pathing.
The supplied real elixir is used as a hard affordability mask with current
catalog costs. It does not replace the encoder's learned observation features.
Observed hand membership is also enforced as a hard mask. The v9 encoder hides
unrevealed opponent deck slots; the legacy encoder retains its original contract.
These adapters do not establish official engine fidelity or actual game strength.

`analysis/policy_bridge_smoke.json` records startup and real opening predictions
from all three checkpoints, on CPU, for both sides. No training or official
game connection is involved.

Run `python3 tools/test_policy_bridge.py` for dependency-free contract checks.
These checks validate the current playable catalog's alias and cost coverage,
absolute coordinates for both teams, confirmed-event causality, hard hand and
affordability masks, and model wait/error behavior. Inference is stubbed for
these checks; the separate smoke artifact uses actual checkpoints.
