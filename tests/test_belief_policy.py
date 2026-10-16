from cr_replay_pipeline.belief_policy import (
    CausalBeliefPolicy,
    build_policy_model,
    fourier_encode,
    fourier_freqs,
)
from cr_replay_pipeline.event_clock import activity_targets, competing_risk_nll, positive_rates
from cr_replay_pipeline.policy_dataset import collate_policy_batch
from cr_replay_pipeline.policy_model import PolicyBC
from cr_replay_pipeline.policy_spec import loss_kwargs_for_spec, policy_spec_from_version
import torch


def _fake_batch(model, length=9):
    batch = 3
    steps = length
    continuous = torch.zeros(batch, steps, 28)
    continuous[:, :, 0] = torch.linspace(0.02, 0.2, steps)
    continuous[:, :, 1] = torch.tensor([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0][:steps])
    continuous[:, :, 4] = 0.4
    continuous[:, :, 5] = 0.35
    card_ids = torch.ones(batch, steps, dtype=torch.long)
    team = torch.arange(8).unsqueeze(0).expand(batch, -1) + 1
    opp = team + 8
    globals_ = torch.zeros(batch, 62)
    globals_[:, 21] = continuous[:, -1, 0]
    lengths = torch.tensor([steps, steps - 1, steps - 2])
    slot_feats = torch.zeros(batch, 8, 8)
    hand = torch.ones(batch, 8, dtype=torch.bool)
    return continuous, card_ids, team, opp, globals_, lengths, slot_feats, hand


def test_v7_spec_stays_v7():
    spec = policy_spec_from_version("7")
    assert spec.family == "v7"
    assert spec.memory_tokens == 0
    assert spec.arena_memory_channels == 16


def test_v7_factory_still_policy_bc():
    spec = policy_spec_from_version("7")
    cfg = spec.to_dict()
    cfg["d_model"] = 32
    cfg["num_layers"] = 1
    cfg["dropout"] = 0.0
    model = build_policy_model(40, cfg)
    assert isinstance(model, PolicyBC)
    assert not isinstance(model, CausalBeliefPolicy)


def test_v8_spec_is_belief_memory_not_a_flag_soup():
    spec = policy_spec_from_version("8")
    assert spec.family == "v8"
    assert spec.memory_tokens == 24
    assert spec.event_clock is True
    assert spec.mixed_precision is True
    assert spec.placement_card_mode == "selected"
    assert spec.winner_only_default is False
    assert spec.freeze_backbone_default is False
    assert spec.default_d_model == 128
    assert spec.default_num_layers == 4
    assert spec.default_batch_size == 128


def test_belief_policy_parameter_budget():
    model = CausalBeliefPolicy(vocab_size=180, global_dim=62, d_model=128)
    n = sum(p.numel() for p in model.parameters())
    assert 1_500_000 <= n <= 4_500_000, n


def test_belief_policy_is_causal_in_padding():
    torch.manual_seed(8)
    model = CausalBeliefPolicy(vocab_size=32, global_dim=62, d_model=32, memory_tokens=8, num_layers=1)
    model.eval()
    batch = _fake_batch(model)
    with torch.no_grad():
        left = model(*batch)
        continuous = batch[0].clone()
        continuous[:, -1] = 9.0
        right = model(continuous, *batch[1:])
    # The shortest row (length 7 of 9) must ignore the corrupted tail.
    assert torch.allclose(left["fused"][2], right["fused"][2], atol=1e-5)


def test_empty_side_does_not_read_the_other_side():
    torch.manual_seed(4)
    model = CausalBeliefPolicy(
        vocab_size=32,
        global_dim=62,
        d_model=32,
        memory_tokens=8,
        num_layers=1,
        interaction_blocks=0,
        dropout=0.0,
    )
    model.eval()
    batch = list(_fake_batch(model))
    batch[0] = batch[0].clone()
    batch[0][:, :, 1] = 0.0
    with torch.no_grad():
        left = model(*batch)
        cards = batch[1].clone()
        cards[:] = 2
        right = model(batch[0], cards, *batch[2:])
    n_friend = model.memory_tokens // 2
    assert torch.allclose(left["memory"][:, :n_friend], right["memory"][:, :n_friend], atol=1e-5)
    assert not torch.allclose(left["memory"][:, n_friend:], right["memory"][:, n_friend:], atol=1e-4)


def test_time_transition_moves_clock_with_elapsed_query():
    torch.manual_seed(3)
    model = CausalBeliefPolicy(vocab_size=32, global_dim=62, d_model=32, memory_tokens=8, num_layers=1)
    model.eval()
    batch = list(_fake_batch(model))
    with torch.no_grad():
        now = model(*batch)
        late = batch[4].clone()
        late[:, 21] = batch[0][:, -1, 0] + 0.05
        batch[4] = late
        later = model(*batch)
    assert not torch.allclose(now["clock_rates"], later["clock_rates"], atol=1e-6)
    assert float(later["query_dt"].mean()) > float(now["query_dt"].mean())


def test_belief_policy_loss_and_factory_roundtrip():
    torch.manual_seed(1)
    spec = policy_spec_from_version("8")
    cfg = spec.to_dict()
    cfg["d_model"] = 32
    cfg["num_layers"] = 1
    cfg["memory_tokens"] = 8
    cfg["dropout"] = 0.0
    model = build_policy_model(40, cfg)
    assert isinstance(model, CausalBeliefPolicy)
    assert not isinstance(model, PolicyBC)
    batch = _fake_batch(model)
    out = model(*batch, target_slots=torch.zeros(3, dtype=torch.long))
    losses = model.loss(
        out,
        torch.zeros(3, dtype=torch.long),
        torch.zeros(3, dtype=torch.long),
        torch.zeros(3, dtype=torch.long),
        torch.tensor([[0.4, 0.3], [0.5, 0.4], [0.2, 0.6]]),
        torch.tensor([0.4, 0.7, 0.2]),
        **loss_kwargs_for_spec(spec, freeze_backbone=False),
    )
    assert torch.isfinite(losses["loss"])
    half = {
        key: value.half() if torch.is_tensor(value) and value.is_floating_point() else value
        for key, value in out.items()
    }
    half_losses = model.loss(
        half,
        torch.zeros(3, dtype=torch.long),
        torch.zeros(3, dtype=torch.long),
        torch.zeros(3, dtype=torch.long),
        torch.tensor([[0.4, 0.3], [0.5, 0.4], [0.2, 0.6]]),
        torch.tensor([0.4, 0.7, 0.2]),
        **loss_kwargs_for_spec(spec, freeze_backbone=False),
    )
    assert torch.isfinite(half_losses["loss"].float())
    delay = torch.tensor([1.0, 3.0, 8.0])
    nll = competing_risk_nll(positive_rates(torch.zeros(3, 2)), delay)
    assert nll.shape == (3,)
    assert activity_targets(delay).shape == (3, 3)


def test_fourier_encode_reuses_cached_freqs():
    values = torch.tensor([0.0, 1.5, 8.25])
    live = fourier_encode(values, 16, max_period=32.0)
    cached = fourier_encode(values, 16, max_period=32.0, freqs=fourier_freqs(16, 32.0))
    assert torch.allclose(live, cached)


def test_collate_pads_sequences_to_fixed_context():
    def item(steps: int):
        return (
            torch.zeros(steps, 28),
            torch.zeros(steps, dtype=torch.long),
            torch.zeros(8, dtype=torch.long),
            torch.zeros(8, dtype=torch.long),
            torch.zeros(62),
            torch.zeros(8, 8),
            torch.ones(8, dtype=torch.bool),
            torch.zeros((), dtype=torch.long),
            torch.zeros((), dtype=torch.long),
            torch.zeros((), dtype=torch.long),
            torch.tensor([0.4, 0.3]),
            torch.zeros(()),
            steps,
            torch.ones(()),
        )

    batch = collate_policy_batch([item(3), item(7)], pad_to=16)
    assert batch[0].shape == (2, 16, 28)
    assert batch[1].shape == (2, 16)
    assert batch[12].tolist() == [3, 7]
