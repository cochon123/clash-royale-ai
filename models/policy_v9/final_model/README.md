# Policy v9 deployment candidate

Seed 42 is exported because that seed was specified before final scoring.
Read `manifest.json` for the final gate and whether this artifact was promoted.
This package omits optimizer/RNG state; resumable training checkpoints remain
in the source run. It proposes card plays and never taps a phone.

This artifact is **experimental and not promoted**. On the frozen 1,000-battle
final comparison, seed 42 reached 56.23% card accuracy, 12.76% joint card-and-cell
accuracy and 4.125 joint discrete NLL. v4.4.1 reached 53.21%, 11.21% and 4.412.
The +1.55 percentage-point joint gain missed the predeclared +2-point threshold.
The continuation seeds also share one donor; independent full-training
reproduction remains incomplete. No actual game wins were measured.

`verification.json` records exact source/export tensor equality and a CPU
causal-inference check. The export was specified before final scoring.

```python
from cr_replay_pipeline.v9_infer import load_model, predict
from cr_replay_pipeline.winner_dataset import load_card_costs
model, vocab, config = load_model("models/policy_v9/final_model/best.pt")
costs = load_card_costs("models/policy_v9/final_model/costs.json")
proposal = predict(model, vocab, costs, battle, confirmed_events,
                   agent_side="team", now=observation_time,
                   observed_hand=recognized_card_names)
```

Only confirmed events before `now` may enter the observation. Unrecognized
hands remain unknown. New events invalidate old proposals; requery within
250 ms. Coordinates follow the same frame used by the causal training encoder.
