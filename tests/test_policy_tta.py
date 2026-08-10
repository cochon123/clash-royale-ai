import torch

from cr_replay_pipeline.policy_tta import ZONE_MIRROR, mirror_ensemble_outputs


def test_mirror_ensemble_maps_space_back_before_averaging():
    original = {
        "slot_logits": torch.tensor([[2.0, 0.0]]),
        "type_logits": torch.tensor([[1.0, 0.0]]),
        "zone_logits": torch.arange(12, dtype=torch.float32).unsqueeze(0),
        "tile_logits": None,
        "xy": torch.tensor([[0.25, 0.75]]),
        "timing": torch.tensor([1.0]),
        "fused": torch.zeros(1, 4),
    }
    mirrored = {
        "slot_logits": torch.tensor([[2.0, 0.0]]),
        "type_logits": torch.tensor([[1.0, 0.0]]),
        "zone_logits": torch.arange(12, dtype=torch.float32)[list(ZONE_MIRROR)].unsqueeze(0),
        "tile_logits": None,
        "xy": torch.tensor([[0.75, 0.75]]),
        "timing": torch.tensor([1.0]),
        "fused": torch.zeros(1, 4),
    }

    combined = mirror_ensemble_outputs(original, mirrored)

    assert torch.allclose(combined["xy"], original["xy"])
    assert torch.allclose(combined["timing"], original["timing"])
    assert combined["zone_logits"].argmax(-1).item() == 11
    assert combined["slot_logits"].argmax(-1).item() == 0
