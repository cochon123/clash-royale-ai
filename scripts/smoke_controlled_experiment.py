"""One tiny real-data epoch per variant; capped at three minutes total.

Integration/throughput check, not an architecture comparison or promotion.
"""

from argparse import Namespace
import json
from pathlib import Path
import torch
from cr_replay_pipeline.controlled_experiment import Budget, run_variant
from cr_replay_pipeline.winner_dataset import (
    collect_battles,
    build_vocab,
    load_card_costs,
)
from cr_replay_pipeline.causal_observation import OBSERVATION_VERSION

torch.set_num_threads(2)
output = Path("models/controlled_smoke")
output.mkdir(exist_ok=True)
manifest = json.loads(Path("data/splits/policy_v8_pilot_5000_seed42.json").read_text())
train_ids = manifest["train_ids"][:32]
val_ids = manifest["validation_ids"][:8]
print("phase=load work=0/3 ETA=<180s", flush=True)
battles = collect_battles(
    "data/raw",
    require_decisive_result=False,
    metadata_path="data/metadata/battles.jsonl",
    unresolved_abilities="preserve_event",
    battle_ids=set(train_ids + val_ids),
)
by_id = {b.battle_id: b for b in battles}
train = [by_id[i] for i in train_ids]
val = [by_id[i] for i in val_ids]
vocab = build_vocab(train)
costs = load_card_costs("data/card_costs.json")
args = Namespace(
    output=str(output),
    seed=42,
    device="cuda" if torch.cuda.is_available() else "cpu",
    d_model=32,
    samples_per_battle=8,
    batch_size=32,
    epochs=1,
    run_seconds=60,
    patience=2,
)
budget = Budget(output / "budget.json", 180)
for i, variant in enumerate(("baseline", "clock", "memory")):
    result = run_variant(
        args,
        variant,
        train,
        val,
        vocab,
        costs,
        {
            "observation": OBSERVATION_VERSION,
            "purpose": "integration smoke only",
            "train_ids": train_ids,
            "validation_ids": val_ids,
        },
        budget,
    )
    print(
        f"phase=done variant={variant} work={i + 1}/3 ETA=<{budget.remaining:.0f}s status={result['status']}",
        flush=True,
    )
