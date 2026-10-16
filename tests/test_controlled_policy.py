from dataclasses import replace
import torch
import pytest
from test_eval_integrity import _asymmetric_battle, _id_battles
from cr_replay_pipeline.causal_observation import prefix_observation
from cr_replay_pipeline.controlled_dataset import (
    ActionExamples,
    ClockExamples,
)
from cr_replay_pipeline.controlled_experiment import (
    decide_clock_gate,
    decide_memory_gate,
)
from cr_replay_pipeline.controlled_policy import ControlledPolicy
from cr_replay_pipeline.event_clock import competing_risk_nll, positive_rates
from cr_replay_pipeline.belief_policy import TimeTransition, build_policy_model
from cr_replay_pipeline.feature_schema import GLOBAL_INDEX
from cr_replay_pipeline.winner_dataset import CardVocab
from cr_replay_pipeline.policy_manifest import build_manifest, resolve_battle_splits
from cr_replay_pipeline.policy_dataset import collate_policy_batch, encode_policy_sample


def setup():
    b = _asymmetric_battle()
    v = CardVocab(list(b.team_deck + b.opponent_deck))
    return b, v


def test_fixed_observation_independent_of_future_actor_card_time():
    b, v = setup()
    changed = replace(
        b,
        events=b.events[:12]
        + ({**b.events[12], "side": "opponent", "card": "made-up", "seconds": 900},),
    )
    a = prefix_observation(b, 12, "team").encode(v, {})
    z = prefix_observation(changed, 12, "team").encode(v, {})
    assert all(torch.equal(x, y) for x, y in zip(a, z))


def test_observation_object_propagates_quiet_time():
    b, v = setup()
    now = b.events[11]["seconds"]
    a = prefix_observation(b, 12, "team", now=now).encode(v, {})
    z = prefix_observation(b, 12, "team", now=now + 5).encode(v, {})
    assert float(z[4][GLOBAL_INDEX["now"]] - a[4][GLOBAL_INDEX["now"]]) * 300 == pytest.approx(
        5, abs=1e-4
    )
    assert float(z[4][GLOBAL_INDEX["team_elixir"]]) >= float(a[4][GLOBAL_INDEX["team_elixir"]])
    assert torch.equal(a[0], z[0])


def test_real_encoder_propagates_quiet_time():
    b, v = setup()
    now = b.events[11]["seconds"]
    a = encode_policy_sample(b, 12, v, {}, threat_dim=14, observation_seconds=now)
    z = encode_policy_sample(b, 12, v, {}, threat_dim=14, observation_seconds=now + 5)
    assert float(z[4][21] - a[4][21]) * 300 == pytest.approx(5, abs=1e-5)
    assert torch.equal(a[0], z[0])
    assert z[4][18] >= a[4][18]


def test_memory_residual_changes_when_an_event_identity_changes():
    b, v = setup()
    ds = ActionExamples([b], v, {})
    batch = collate_policy_batch([ds[0], ds[1]])
    model = ControlledPolicy(v.vocab_size, "memory", d_model=32)
    model.eval()
    with torch.no_grad():
        original = model.memory_residual(batch[0], batch[1], batch[12], batch[4])
        ids = batch[1].clone()
        valid = (ids[0] != 0).nonzero(as_tuple=False)
        assert len(valid), "prefix must contain an event"
        ids[0, int(valid[0])] = ids[0, int(valid[0])] + 1
        changed = model.memory_residual(batch[0], ids, batch[12], batch[4])
    assert (original - changed).abs().max() > 1e-6


def _report(epoch, **metrics):
    row = {"epoch": epoch, "card": 0.5, "joint": 0.09, "timing_mae": 1.75}
    row.update(metrics)
    return {
        "history": [row],
        "last_completed_epoch": epoch,
        "weights_epoch": epoch,
        "elapsed_seconds": 100,
        "empirical_clock_validation": {"clock_nll": 2.8, "clock_median_mae": 2.0},
    }


def test_clock_gate_uses_equal_epoch_and_rejects_action_regression():
    baseline = _report(5, card=0.50, joint=0.09, timing_mae=1.75)
    bad = _report(
        5,
        card=0.40,
        joint=0.07,
        timing_mae=1.65,
        mean_own_rate=0.2,
        mean_opponent_rate=0.2,
        clock_nll=2.4,
    )
    assert decide_clock_gate(baseline, bad)["pass"] is False
    good = _report(
        5,
        card=0.498,
        joint=0.089,
        timing_mae=1.64,
        mean_own_rate=0.2,
        mean_opponent_rate=0.19,
        clock_nll=2.4,
    )
    decision = decide_clock_gate(baseline, good)
    assert decision["pass"] is True
    assert decision["epoch"] == 5


def test_memory_gate_requires_one_point_action_gain():
    baseline = _report(3, card=0.46, joint=0.07)
    weak = _report(3, card=0.462, joint=0.071)
    strong = _report(3, card=0.472, joint=0.075)
    assert decide_memory_gate(baseline, weak)["pass"] is False
    assert decide_memory_gate(baseline, strong)["pass"] is True
    b, v = setup()
    now = b.events[11]["seconds"]
    a = encode_policy_sample(b, 12, v, {}, threat_dim=14, observation_seconds=now)
    z = encode_policy_sample(b, 12, v, {}, threat_dim=14, observation_seconds=now + 5)
    assert float(z[4][21] - a[4][21]) * 300 == pytest.approx(5, abs=1e-5)
    assert torch.equal(a[0], z[0])
    assert z[4][18] >= a[4][18]


def test_unrevealed_opponent_cards_do_not_leak_through_globals():
    b, v = setup()
    other = replace(b, opponent_deck=tuple("secret" + str(i) for i in range(8)))
    a = prefix_observation(b, 0, "team").encode(v, {})
    z = prefix_observation(other, 0, "team").encode(v, {})
    assert all(torch.equal(x, y) for x, y in zip(a, z))


def test_time_identity_and_composition():
    t = TimeTransition(16)
    m = torch.randn(3, 8, 16)
    zero = torch.zeros(3)
    one = zero + 1
    assert torch.equal(t(m, zero), m)
    assert torch.allclose(t(t(m, one), one), t(m, one * 2), atol=1e-6)


def test_balanced_clock_recovers_both_rates_and_censoring():
    raw = torch.nn.Parameter(torch.zeros(1, 2))
    opt = torch.optim.Adam([raw], lr=0.08)
    sides = torch.tensor([0, 1] * 50)
    delay = torch.ones(100) * 2
    for _ in range(150):
        opt.zero_grad()
        loss = competing_risk_nll(
            positive_rates(raw).expand(100, -1), delay, sides
        ).mean()
        loss.backward()
        opt.step()
    assert torch.allclose(positive_rates(raw), torch.full((1, 2), 0.25), atol=0.003)
    censored = competing_risk_nll(
        torch.tensor([[0.2, 0.3]]),
        torch.tensor([4.0]),
        torch.tensor([1]),
        torch.tensor([False]),
    )
    assert censored.item() == pytest.approx(2.0)


def test_clock_dataset_labels_both_actors_and_marginalizes_ties():
    b, v = setup()
    ds = ClockExamples([b], v, {}, recording_ends={b.battle_id: 100.0})
    assert {row[4] for row in ds.index} == {0, 1}
    assert ds.index[-1][-1] is False
    tied = replace(
        b, events=({**b.events[0], "seconds": 2.0}, {**b.events[1], "seconds": 2.0})
    )
    assert ClockExamples([tied], v, {}).index[0][-2:] == (-1, True)
    nll = competing_risk_nll(
        torch.tensor([[0.2, 0.3]]), torch.tensor([2.0]), torch.tensor([-1])
    )
    assert nll.item() == pytest.approx(1.0 - __import__("math").log(0.5))


def test_memory_gate_receives_gradients_and_scheduler_uses_clock():
    b, v = setup()
    ds = ActionExamples([b], v, {})
    batch = collate_policy_batch([ds[0], ds[1]])
    model = ControlledPolicy(v.vocab_size, "memory", d_model=32)
    out = model(batch, teacher=True)
    out["slot_logits"].square().mean().backward()
    assert model.update_gate.weight.grad.abs().max() > 1e-8
    assert model.memory.grad.abs().max() > 1e-8
    model.eval()
    with torch.no_grad():
        model.clock_head.weight.zero_()
        model.clock_head.bias.copy_(torch.tensor([-4.0, 2.0]))
    schedule = model.schedule(batch)
    assert schedule["wait_for_opponent"].all()
    assert torch.isfinite(schedule["delay_seconds"]).all()


def test_explicit_manifest_cannot_override_ancestor_exposure(tmp_path):
    battles = _id_battles(100)
    build_manifest(battles, tmp_path / "donor/split_manifest.json", seed=42)
    build_manifest(battles, tmp_path / "new.json", seed=43)
    with pytest.raises(ValueError, match="ancestor trained"):
        resolve_battle_splits(
            battles,
            split_manifest=tmp_path / "new.json",
            warmstart_dir=tmp_path / "donor",
            output_dir=tmp_path / "out",
            create=True,
        )


def test_archived_memory_factory_keeps_legacy_transition():
    model = build_policy_model(
        32,
        dict(
            memory_tokens=8,
            d_model=32,
            num_layers=1,
            placement_mode="heatmap",
            placement_card_mode="selected",
            global_dim=62,
        ),
    )
    assert model.memory_revision == 1 and model.time_transition.legacy


@pytest.mark.parametrize(
    "variant", ["baseline", "clock", "memory", "shuffled", "frozen_time"]
)
def test_screening_end_to_end_cpu(tmp_path, variant):
    from argparse import Namespace
    from cr_replay_pipeline.controlled_experiment import Budget, run_variant

    b, v = setup()
    args = Namespace(
        output=str(tmp_path),
        seed=42,
        device="cpu",
        d_model=32,
        samples_per_battle=4,
        batch_size=4,
        epochs=1,
        run_seconds=20,
        patience=2,
    )
    budget = Budget(tmp_path / "budget.json", 60)
    result = run_variant(
        args,
        variant,
        [b],
        [replace(b, battle_id="validation")],
        v,
        {},
        {"test": "smoke"},
        budget,
    )
    assert result["status"] == "completed"
    assert result["weights_epoch"] == 1
    assert (tmp_path / f"{variant}_seed42/report.html").exists()
    assert (tmp_path / f"{variant}_seed42/best.pt").exists()


def test_durable_log_keeps_writing_after_broken_pipe(tmp_path):
    import os
    from cr_replay_pipeline.controlled_experiment import DurableLog

    reader, writer = os.pipe()
    os.close(reader)
    dead = os.fdopen(writer, "w")
    alive = tmp_path / "trainer.log"
    log = DurableLog(dead, alive.open("w"))
    log.write("still training\n")
    log.flush()
    assert alive.read_text() == "still training\n"


def test_suite_budget_cannot_be_reset(tmp_path):
    from cr_replay_pipeline.controlled_experiment import Budget

    b = Budget(tmp_path / "budget.json", 10)
    b.used = 11
    b.persist()
    assert Budget(tmp_path / "budget.json", 10).remaining == 0
    with pytest.raises(ValueError, match="change suite budget"):
        Budget(tmp_path / "budget.json", 20)


def test_board_oracle_rejects_unverified_future_and_cross_game_split():
    from cr_replay_pipeline.board_oracle import (
        oracle_features,
        validate_group_splits,
        FIELDS,
    )

    row = dict(
        game_id="game",
        split="train",
        battle_id="battle",
        event_index=10,
        observation_seconds=20,
        frame_seconds=20,
        labels={f: None for f in FIELDS},
    )
    with pytest.raises(ValueError, match="verification"):
        oracle_features(row)
    row.update(verified_by="human", verified_at="2026-09-07", labels={FIELDS[0]: 0.5})
    assert oracle_features(row)[0] == 0.5
    row["frame_seconds"] = 21
    with pytest.raises(ValueError, match="future"):
        oracle_features(row)
    with pytest.raises(ValueError, match="cross splits"):
        validate_group_splits([row, {**row, "split": "test"}])


def test_ability_position_does_not_supervise_placement():
    b, v = setup()
    ds = ActionExamples([b], v, {})
    batch = list(collate_policy_batch([ds[0], ds[1]]))
    model = ControlledPolicy(v.vocab_size, d_model=32)
    out = model(tuple(batch), teacher=True)
    types = torch.ones(2, dtype=torch.long)
    a = model.base.loss(
        out, batch[7], types, batch[9], batch[10], batch[11], tile_weight=0.55
    )
    z = model.base.loss(
        out,
        batch[7],
        types,
        batch[9],
        torch.zeros_like(batch[10]),
        batch[11],
        tile_weight=0.55,
    )
    assert torch.equal(a["loss"], z["loss"])


def test_unknown_ability_is_retained_as_unknown_observation():
    from cr_replay_pipeline.parser import parse_replay
    from cr_replay_pipeline.winner_dataset import events_from_replay
    from pathlib import Path

    replay = parse_replay(Path(__file__).parent / "fixtures/replay.json")
    events, _ = events_from_replay(replay, unresolved_abilities="preserve_event")
    unknown = [e for e in events if e.get("card_known") is False]
    assert unknown and not unknown[0]["position_known"]
