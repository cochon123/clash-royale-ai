"""Deployment-shaped, causal v9 inference. Returns proposals; never taps phones."""
from __future__ import annotations
import math
from pathlib import Path
import torch
from .causal_observation import OBSERVATION_VERSION,prefix_observation
from .controlled_dataset import tensor_row
from .policy_dataset import collate_policy_batch
from .policy_model import TILE_COLS,TILE_ROWS
from .v9_policy import load_v9_policy


def load_model(path,device="cpu"):
    path=Path(path)
    artifact=torch.load(path,map_location="cpu",weights_only=False)
    if artifact.get("contract",{}).get("data",{}).get("observation")!=OBSERVATION_VERSION:
        raise ValueError("Checkpoint observation contract differs from inference")
    return load_v9_policy(path,device)


@torch.inference_mode()
def predict(model,vocab,costs,battle,confirmed_events,agent_side="team",now=None,observed_hand=None):
    observation=prefix_observation(battle,confirmed_events,agent_side,now)
    device=next(model.parameters()).device
    batch=tuple(t.to(device) for t in collate_policy_batch([tensor_row(observation.encode(vocab,costs))]))
    deck=battle.team_deck if agent_side=="team" else battle.opponent_deck
    out=model(batch)
    logits=out["slot_logits"][0]
    if observed_hand is not None:
        legal=torch.tensor([c in observed_hand for c in deck],device=device)
        if not legal.any():
            return dict(status="needs_observation",reason="No recognized hand card matches the declared deck",observation_seconds=observation.now)
        logits=logits.masked_fill(~legal,-torch.inf)
    slot=int(logits.argmax())
    selected=list(batch);selected[7]=torch.tensor([slot],device=device)
    out=model(tuple(selected),teacher=True)
    tile=int(out["tile_logits"][0].argmax())
    proposal=dict(status="proposal",card=deck[slot],slot=slot,event_type="card_play",x=(tile%TILE_COLS+.5)/TILE_COLS,y=(tile//TILE_COLS+.5)/TILE_ROWS,coordinate_frame="agent-relative normalized, same causal encoder frame",observation_seconds=observation.now,requery_after_seconds=.25,ability_actions_supported=False)
    if model.clock_head is not None:
        rates=out["clock_rates"][0];own=float(rates[0]/rates.sum())
        proposal.update(delay_seconds=float(math.log(2)/rates.sum()),own_next_probability=own,wait_for_opponent=own<.5,scheduler="joint_clock")
    else:proposal.update(delay_seconds=max(0,float(out["timing"][0].clamp(max=6).expm1())),scheduler="action_delay")
    return proposal
