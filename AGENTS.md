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