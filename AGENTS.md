# Clash Royale AI

Our goal is to create an AI model that can play Clash Royale.

## Approach: Data-Driven

We are collecting data to build a **data-based approach**. Right now, the data collection script should still be running.

From what we know so far, every previous attempt at this used **Reinforcement Learning (RL)** — training a model to play the game via vision recognition — and it always performed very badly.

### Why Data-Based

- **Playing the game is expensive.** Getting the AI to actually play the game is a costly process that we should only do at the very end, once we have our final models — or when we are 95% sure it is unavoidable to continue.
- **The trade of.** The policy sees previous card deployments, not current troops, health, death, targeting, pathing, or real elixir. we can use tricks to make it sees stuff like elexir for example.
- **Test without playing.** If we can test something without actually running the game, we should do that.

## Training Reports

For every AI we train, we create an **HTML report**.

### Required Content

The report must include:

- **Name/version** of the model
- **Compute** allocated for it
- **Quantity of data**
- **Timestamp** of when the model was created
- **Curves** such as loss, accuracy, and others (depending on the model and what we can plot)
- **Videos of the curves evolving**, if possible
- **Lessons learned**, if there are any

### Report Styling Requirements

- **No matplotlib** inside the report — use native HTML graphs and animations.
- Include niceties like:
  - Hover effects on graphs
  - Explanations of expressions
  - Ability to add/remove curves in a graph for better visibility
- Be creative. Use these as references:
  - `reports/style_discriminator_v1.html`
  - `reports/policy_bc_v4_showcase.html`

> These are just the minimum — the actual report should include more model-specific content: diagrams, graphics, images, possibly interactive stuff.

## Versioning
When I ask you to commit and push do it on github and on huggingface.

## Hardware / Compute

Since we are compute-limited (3060 8GB vram, 13th gen core i5, 32GB ram), the scripts we write should aim to **maximize the available hardware**.

## Run Monitoring

When you launch a run, provide a command that lets me **stream the evolution of the run in real time**. Make sure there is:

- A column for an **estimation of the amount of time left**
- A column for the **amount of work done so far**

## Cursor Cloud specific instructions

Dependencies are locked in `uv.lock`. Install with `uv sync --frozen --inexact`, then install `pytest` into that virtualenv. `pytest` is required to run `tests/` and is not a project dependency, so a plain `uv sync` removes it. The README `python -m venv` plus `pip install -e .` path is the fallback when `uv` is not available.

`cr-replays` and `pytest` resolve from `.venv/bin` (also linked on `PATH` as `/usr/local/bin/cr-replays` and `/usr/local/bin/pytest` after the cloud install).

Offline checks do not need `CR_TOKEN`, a GPU, or phones:

- `pytest`
- `cr-replays clean --input tests/fixtures --output /tmp/cr-clean --audit-only`

`cr-replays serve` is the local ingest service at `http://127.0.0.1:8765/health`. Start it only while collecting replays. Phone lab (`cr-replays phone-lab`) needs `adb` and unlocked phones and is outside this environment.