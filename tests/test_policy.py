from pathlib import Path

from cr_replay_pipeline.policy_dataset import (
    PolicyActionDataset,
    deck_slot_for_card,
    encode_policy_sample,
)
from cr_replay_pipeline.policy_model import ArenaMemoryRasterizer, PolicyBC, xy_to_zone
from cr_replay_pipeline.policy_manifest import battles_from_manifest, build_manifest, load_manifest
from cr_replay_pipeline.policy_v7_report import render_policy_v7_report
from cr_replay_pipeline.policy_report import render_policy_report
from cr_replay_pipeline.policy_v6_report import render_policy_v6_report
from cr_replay_pipeline.winner_dataset import BattleExample, CardVocab
import torch


def _sample_battle() -> BattleExample:
    deck_a = (
        "knight",
        "archers",
        "goblins",
        "fireball",
        "cannon",
        "skeletons",
        "the-log",
        "ice-spirit",
    )
    deck_b = (
        "hog-rider",
        "musketeer",
        "ice-golem",
        "cannon",
        "skeletons",
        "ice-spirit",
        "fireball",
        "the-log",
    )
    events = []
    seconds = 8.0
    for index in range(20):
        side = "team" if index % 2 == 0 else "opponent"
        deck = deck_a if side == "team" else deck_b
        card = deck[index % 8]
        events.append(
            {
                "seconds": seconds,
                "side": side,
                "event_type": "card_play",
                "card": card,
                "x": 5000 + (index % 5) * 1000,
                "y": 7000 if side == "team" else 25000,
            }
        )
        seconds += 2.5
    return BattleExample(
        battle_id="battle-policy-1",
        team_deck=deck_a,
        opponent_deck=deck_b,
        team_wins=1,
        events=tuple(events),
    )


def test_deck_slot_zone_and_encode():
    battle = _sample_battle()
    assert deck_slot_for_card(battle.team_deck, "fireball") == 3
    assert 0 <= xy_to_zone(0.5, 0.3) < 12
    vocab = CardVocab(list(battle.team_deck) + list(battle.opponent_deck))
    sample = encode_policy_sample(battle, 10, vocab, {"knight": 3, "fireball": 4})
    assert sample is not None
    (
        continuous,
        card_ids,
        team_deck,
        opp_deck,
        globals_,
        slot_feats,
        hand_mask,
        slot,
        event_type,
        zone,
        xy,
        timing,
        length,
    ) = sample
    assert continuous.ndim == 2
    assert slot_feats.shape == (8, 8)
    assert hand_mask.shape == (8,)
    assert 0 <= int(slot.item()) < 8
    assert 0 <= int(zone.item()) < 12
    assert xy.shape == (2,)
    assert int(length.item()) == continuous.shape[0]


def test_arena_memory_is_causal_decayed_and_perspective_normalized():
    raster = ArenaMemoryRasterizer()
    continuous = torch.zeros(1, 3, 28)
    continuous[0, :, 0] = torch.tensor([0.10, 0.11, 0.12])
    continuous[0, :, 1] = torch.tensor([1.0, 0.0, 1.0])
    continuous[0, :, 4:6] = torch.tensor([[0.25, 0.25], [0.75, 0.75], [0.5, 0.5]])
    continuous[0, :, 9] = torch.tensor([0.3, 0.8, 0.4])
    continuous[0, 1, 11] = 1.0
    continuous[0, 2, 10] = 1.0
    out = raster(continuous, torch.tensor([3]), torch.zeros(1))
    assert out.shape == (1, 16, 18, 32)
    # Friendly and enemy action mass occupy separate channels; spell and
    # ability events do not leak into the persistent unit channels.
    assert float(out[0, 0].sum()) > 0.0
    assert float(out[0, 3].sum()) == 0.0
    assert float(out[0, 10].sum()) == 0.0
    assert float(out[0, 11].sum()) > 0.0
    assert float(out[0, 2].sum()) > 0.0


def test_arena_memory_splats_tile_centers_and_boundaries():
    raster = ArenaMemoryRasterizer()
    continuous = torch.zeros(1, 1, 28)
    continuous[0, 0, 1] = 1.0
    continuous[0, 0, 4] = (7.5 / 32.0)
    continuous[0, 0, 5] = (4.5 / 18.0)
    center = raster(continuous, torch.tensor([1]), torch.zeros(1))[0, 0]
    assert int((center > 0).sum()) == 1
    assert int(center.argmax()) == 4 * 32 + 7

    continuous[0, 0, 4:6] = 0.0
    boundary = raster(continuous, torch.tensor([1]), torch.zeros(1))[0, 0]
    assert int((boundary > 0).sum()) == 1
    assert int(boundary.argmax()) == 0
    assert raster(continuous.half(), torch.tensor([1]), torch.zeros(1).half()).dtype == torch.float32


def test_v7_zero_residual_matches_heatmap_baseline():
    torch.manual_seed(7)
    baseline = PolicyBC(
        24,
        global_dim=62,
        d_model=24,
        num_layers=1,
        card_conditioned_placement=True,
        placement_mode="heatmap",
    )
    candidate = PolicyBC(
        24,
        global_dim=62,
        d_model=24,
        num_layers=1,
        card_conditioned_placement=True,
        placement_mode="heatmap",
        arena_memory_channels=16,
    )
    candidate.load_state_dict(baseline.state_dict(), strict=False)
    baseline.eval()
    candidate.eval()
    inputs = (
        torch.rand(2, 5, 28),
        torch.ones(2, 5, dtype=torch.long),
        torch.ones(2, 8, dtype=torch.long),
        torch.ones(2, 8, dtype=torch.long),
        torch.zeros(2, 62),
        torch.tensor([5, 4]),
        torch.zeros(2, 8, 8),
        torch.ones(2, 8, dtype=torch.bool),
    )
    with torch.no_grad():
        left = baseline(*inputs)
        right = candidate(*inputs)
    assert torch.allclose(left["tile_logits"], right["tile_logits"], atol=1e-6)
    assert torch.allclose(left["xy"], right["xy"], atol=1e-6)


def test_v7_zero_dynamic_memory_preserves_gru_history():
    torch.manual_seed(9)
    model = PolicyBC(
        24,
        global_dim=62,
        d_model=24,
        num_layers=1,
        card_conditioned_placement=True,
        placement_mode="heatmap",
        arena_memory_channels=16,
    ).eval()
    inputs = (
        torch.rand(2, 5, 28),
        torch.ones(2, 5, dtype=torch.long),
        torch.ones(2, 8, dtype=torch.long),
        torch.ones(2, 8, dtype=torch.long),
        torch.zeros(2, 62),
        torch.tensor([5, 4]),
        torch.zeros(2, 8, 8),
        torch.ones(2, 8, dtype=torch.bool),
    )
    with torch.no_grad():
        aligned = model(*inputs, return_debug=True)
        zeroed = model(*inputs, zero_arena_memory=True, return_debug=True)
    assert torch.allclose(aligned["fused"], zeroed["fused"], atol=1e-7)
    assert torch.allclose(
        zeroed["arena_memory"][:, :12],
        torch.zeros_like(zeroed["arena_memory"][:, :12]),
    )
    assert torch.allclose(aligned["arena_memory"][:, 12:], zeroed["arena_memory"][:, 12:])


def test_policy_manifest_round_trip(tmp_path):
    battles = [
        BattleExample(str(i), ("a",) * 8, ("b",) * 8, i % 2, tuple())
        for i in range(100)
    ]
    path = tmp_path / "manifest.json"
    manifest = build_manifest(battles, path, seed=42)
    loaded = load_manifest(path)
    train, val, test = battles_from_manifest(battles, loaded)
    assert len(train) + len(val) + len(test) == 100
    assert manifest["ordered_id_sha256"] == loaded["ordered_id_sha256"]


def test_v7_report_smoke(tmp_path):
    model_dir = tmp_path / "policy_bc_v7"
    model_dir.mkdir()
    (model_dir / "report.json").write_text(
        __import__("json").dumps(
            {
                "model_name": "policy-bc-v7",
                "model_version": "7.0.0",
                "created_at": "2026-01-01T00:00:00Z",
                "compute": {"device": "cpu", "trainable_parameters": 100, "max_context": 64},
                "data": {"battles_total": 100, "train_samples": 200},
                "test": {"tile_class_acc": 0.12, "tile_top5_acc": 0.3, "tile_nll": 4.0, "xy_mae": 5000},
                "history": [{"val_loss": 4.0, "val_tile_class_acc": 0.12, "val_xy_mae": 5000}],
            }
        ),
        encoding="utf-8",
    )
    output = render_policy_v7_report(model_dir, output_path=tmp_path / "report.html")
    text = output.read_text(encoding="utf-8")
    assert "Arena-memory explorer" in text
    assert "causal" in text.lower()
    assert "bar-fill" in text
    assert "matplotlib.pyplot" not in text.lower()


def test_dataset_and_model_forward():
    battles = [_sample_battle() for _ in range(3)]
    vocab = CardVocab(
        [c for b in battles for c in list(b.team_deck) + list(b.opponent_deck)]
    )
    ds = PolicyActionDataset(battles, vocab, {}, max_samples_per_battle=12)
    assert len(ds) > 0
    batch = ds[0]
    model = PolicyBC(vocab_size=vocab.vocab_size, d_model=64, num_layers=1, dropout=0.0)
    continuous = batch[0].unsqueeze(0)
    card_ids = batch[1].unsqueeze(0)
    team_deck = batch[2].unsqueeze(0)
    opp_deck = batch[3].unsqueeze(0)
    globals_ = batch[4].unsqueeze(0)
    slot_feats = batch[5].unsqueeze(0)
    hand_mask = batch[6].unsqueeze(0)
    lengths = torch.tensor([batch[12]], dtype=torch.long)
    weights = batch[13].unsqueeze(0)
    out = model(
        continuous, card_ids, team_deck, opp_deck, globals_, lengths, slot_feats, hand_mask
    )
    assert out["slot_logits"].shape == (1, 8)
    assert out["zone_logits"].shape == (1, 12)
    losses = model.loss(
        out,
        batch[7].unsqueeze(0),
        batch[8].unsqueeze(0),
        batch[9].unsqueeze(0),
        batch[10].unsqueeze(0),
        batch[11].unsqueeze(0),
        sample_weights=weights,
    )
    assert torch.isfinite(losses["loss"])


def test_v4_card_conditioned_placement_forward():
    battles = [_sample_battle() for _ in range(2)]
    vocab = CardVocab(
        [c for b in battles for c in list(b.team_deck) + list(b.opponent_deck)]
    )
    ds = PolicyActionDataset(battles, vocab, {}, max_samples_per_battle=8)
    batch = ds[0]
    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        d_model=64,
        num_layers=1,
        dropout=0.0,
        card_conditioned_placement=True,
    )
    model.train()
    slots = batch[7].unsqueeze(0)
    out = model(
        batch[0].unsqueeze(0),
        batch[1].unsqueeze(0),
        batch[2].unsqueeze(0),
        batch[3].unsqueeze(0),
        batch[4].unsqueeze(0),
        torch.tensor([batch[12]], dtype=torch.long),
        batch[5].unsqueeze(0),
        batch[6].unsqueeze(0),
        target_slots=slots,
    )
    assert out["zone_logits"].shape == (1, 12)
    assert out["xy"].shape == (1, 2)
    model.eval()
    out_e = model(
        batch[0].unsqueeze(0),
        batch[1].unsqueeze(0),
        batch[2].unsqueeze(0),
        batch[3].unsqueeze(0),
        batch[4].unsqueeze(0),
        torch.tensor([batch[12]], dtype=torch.long),
        batch[5].unsqueeze(0),
        batch[6].unsqueeze(0),
    )
    assert out_e["zone_logits"].shape == (1, 12)


def test_think_refiner_toggle_and_depth():
    battles = [_sample_battle() for _ in range(2)]
    vocab = CardVocab(
        [c for b in battles for c in list(b.team_deck) + list(b.opponent_deck)]
    )
    ds = PolicyActionDataset(battles, vocab, {}, max_samples_per_battle=8)
    batch = ds[0]
    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        d_model=64,
        num_layers=1,
        dropout=0.0,
        card_conditioned_placement=True,
        max_think_steps=4,
    )
    model.eval()
    args = (
        batch[0].unsqueeze(0),
        batch[1].unsqueeze(0),
        batch[2].unsqueeze(0),
        batch[3].unsqueeze(0),
        batch[4].unsqueeze(0),
        torch.tensor([batch[12]], dtype=torch.long),
        batch[5].unsqueeze(0),
        batch[6].unsqueeze(0),
    )
    off = model(*args, think_steps=0)
    on = model(*args, think_steps=4)
    mid = model(*args, think_steps=2)
    assert int(off["think_steps"].item()) == 0
    assert int(on["think_steps"].item()) == 4
    assert int(mid["think_steps"].item()) == 2
    # Deeper refine should move the fused latent (and usually the logits).
    assert not torch.allclose(off["fused"], on["fused"])
    assert not torch.allclose(off["slot_logits"], on["slot_logits"])
    # Off path matches a model with no think module for shape compatibility.
    bare = PolicyBC(
        vocab_size=vocab.vocab_size,
        d_model=64,
        num_layers=1,
        dropout=0.0,
        card_conditioned_placement=True,
        max_think_steps=0,
    )
    bare.eval()
    bare_out = bare(*args)
    assert bare_out["slot_logits"].shape == off["slot_logits"].shape


def test_threat_features_and_v3_encode():
    from cr_replay_pipeline.policy_dataset import THREAT_DIM, recent_opponent_threat

    battle = _sample_battle()
    # Opponent plays hog at an early index; team reacts soon after.
    events = list(battle.events)
    events[9] = {
        "seconds": float(events[8]["seconds"]) + 1.0,
        "side": "opponent",
        "event_type": "card_play",
        "card": "hog-rider",
        "x": 9000,
        "y": 24000,
    }
    events[10] = {
        "seconds": float(events[9]["seconds"]) + 1.5,
        "side": "team",
        "event_type": "card_play",
        "card": "cannon",
        "x": 9000,
        "y": 8000,
    }
    battle = BattleExample(
        battle_id=battle.battle_id,
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=1,
        events=tuple(events),
    )
    feat, is_react = recent_opponent_threat(
        battle, 10, "team", {"hog-rider": 4, "cannon": 3}
    )
    assert is_react
    assert feat.shape == (THREAT_DIM,)
    assert float(feat[0]) == 1.0  # hog-rider active
    vocab = CardVocab(list(battle.team_deck) + list(battle.opponent_deck))
    sample = encode_policy_sample(
        battle, 10, vocab, {"knight": 3, "cannon": 3, "hog-rider": 4}, threat_dim=THREAT_DIM
    )
    assert sample is not None
    assert sample[4].shape[-1] == 48 + THREAT_DIM  # GLOBAL_DIM + threat


def test_policy_report_smoke(tmp_path: Path):
    model_dir = tmp_path / "policy"
    model_dir.mkdir()
    report = {
        "model_name": "policy-bc-v2",
        "model_version": "2.0.0",
        "created_at": "2026-08-06T00:00:00Z",
        "seconds": 1.0,
        "compute": {
            "device": "cpu",
            "parameters": 1000,
            "d_model": 64,
            "num_layers": 1,
        },
        "data": {
            "battles_total": 10,
            "train_samples": 100,
            "val_samples": 20,
            "test_samples": 20,
            "splits": [
                {"split": "train", "battles": 6, "team_win_rate": 0.5, "mean_events": 40},
                {"split": "val", "battles": 2, "team_win_rate": 0.5, "mean_events": 40},
                {"split": "test", "battles": 2, "team_win_rate": 0.5, "mean_events": 40},
            ],
        },
        "baselines": {
            "frequency": {"slot_top1": 0.2, "slot_top3": 0.4, "n": 20},
            "cycle": {"slot_top1": 0.22, "slot_top3": 0.45, "n": 20},
            "chance_slot_top1": 0.125,
        },
        "test": {
            "loss": 1.2,
            "slot_top1": 0.35,
            "slot_top3": 0.6,
            "type_acc": 0.95,
            "zone_acc": 0.3,
            "xy_mae": 2000.0,
            "tile_acc": 0.15,
            "timing_mae": 1.5,
            "n": 20,
        },
        "rollouts": {
            "available": True,
            "n": 4,
            "mean_score_real": 0.8,
            "mean_score_policy": 0.5,
            "mean_score_easy": 0.1,
            "mean_score_medium": 0.2,
        },
        "live_play_readiness": {
            "ready_for_live_smoke_test": False,
            "checks": {"beats_baselines": True, "slot_floor": True},
            "rationale": "test",
        },
        "history": [
            {
                "epoch": 1,
                "train_loss": 2.0,
                "val_loss": 1.8,
                "val_slot_top1": 0.2,
                "val_slot_top3": 0.4,
                "val_tile_acc": 0.1,
                "val_type_acc": 0.9,
                "val_zone_acc": 0.25,
            }
        ],
        "lessons": ["lesson"],
    }
    import json

    (model_dir / "report.json").write_text(json.dumps(report), encoding="utf-8")
    out = render_policy_report(model_dir, tmp_path / "policy.html")
    text = out.read_text(encoding="utf-8")
    assert "Next-action policy" in text
    assert "policy-bc-v2" in text

    report["model_name"] = "policy-bc-v6"
    report["model_version"] = "6.0.0"
    report["compute"].update(
        {
            "batch_size": 32,
            "learning_rate": 0.0002,
            "max_context": 64,
            "max_samples_per_battle": 8,
            "hide_opponent_prob": 0.0,
            "loss_kwargs": {"tile_weight": 0.35},
        }
    )
    report["test"]["tile_class_acc"] = 0.09
    report["history"][0]["val_tile_class_acc"] = 0.08
    (model_dir / "report.json").write_text(json.dumps(report), encoding="utf-8")
    probe = {
        "ablations": {
            "oracle_full_opponent_deck": {
                "slot_top1": 0.40,
                "zone_acc": 0.32,
                "xy_mae_units": 7000,
            },
            "revealed_opponent_cards_only": {
                "slot_top1": 0.399,
                "zone_acc": 0.319,
                "xy_mae_units": 7005,
            },
            "no_recent_threat_vector": {
                "zone_acc": 0.30,
                "xy_mae_units": 7200,
            },
            "neutralized_history_xy": {
                "zone_acc": 0.29,
                "xy_mae_units": 7500,
            },
        },
        "outcome_slice": {
            "winner_actions": {"slot_top1": 0.41},
            "loser_actions": {"slot_top1": 0.39},
        },
        "placement_spread": {
            "global": {},
            "heatmap_decode": {
                "expected_xy": {
                    "mean_l1_units": 7000,
                    "x_std": 0.14,
                    "y_std": 0.13,
                },
                "argmax_tile": {
                    "mean_l1_units": 8200,
                    "x_std": 0.27,
                    "y_std": 0.18,
                    "tile_top1": 0.09,
                    "tile_top5": 0.26,
                },
                "human": {"x_std": 0.27, "y_std": 0.22},
            },
        },
    }
    (tmp_path / "blind_spot_probe_v6_full.json").write_text(
        json.dumps(probe), encoding="utf-8"
    )
    v6_out = render_policy_v6_report(
        model_dir,
        output_path=tmp_path / "policy_bc_v6_full.html",
    )
    v6_text = v6_out.read_text(encoding="utf-8")
    assert "The heatmap learned modes" in v6_text
    assert "DO NOT PROMOTE" in v6_text
    assert "Argmax tile" in v6_text
    assert "@@" not in v6_text
