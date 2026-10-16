"""Regression tests for train/eval split, causal features, caches, and judges."""

from __future__ import annotations

import json
import random
from pathlib import Path

import numpy as np
import pytest
import torch

from cr_replay_pipeline.artifact_cache import (
    default_dataset_cache_path,
    source_fingerprint,
    stable_int_seed,
)
from cr_replay_pipeline.feature_schema import (
    GLOBAL_DIM,
    GLOBAL_INDEX,
    OFFICIAL_ELIXIR_FEATURE_NAMES,
    THREAT_DIM,
    THREAT_INDEX,
)
from cr_replay_pipeline.metrics import binary_auc
from cr_replay_pipeline.parser import parse_replay
from cr_replay_pipeline.policy_dataset import (
    PolicyActionDataset,
    acting_cycle_features,
    apply_horizontal_mirror,
    baselines_for_dataset,
    encode_policy_sample,
    recent_opponent_threat,
)
from cr_replay_pipeline.policy_manifest import (
    battles_from_manifest,
    build_manifest,
    resolve_battle_splits,
)
from cr_replay_pipeline.style_train import _style_lessons
from cr_replay_pipeline.winner_dataset import (
    BattleExample,
    CardVocab,
    battle_example_from_replay,
    collect_battles,
    events_from_replay,
    split_battles,
)
from cr_replay_pipeline.winner_tabular import extract_tabular_features, score_winner_battles

FIXTURE = Path(__file__).parent / "fixtures" / "replay.json"


def _id_battles(n: int) -> list[BattleExample]:
    return [
        BattleExample(str(i), ("a",) * 8, ("b",) * 8, i % 2, tuple())
        for i in range(n)
    ]


def _asymmetric_battle() -> BattleExample:
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
        events.append(
            {
                "seconds": seconds,
                "side": side,
                "event_type": "card_play",
                "card": deck[index % 8],
                "x": 2500 if side == "team" else 15500,
                "y": 7000 if side == "team" else 25000,
            }
        )
        seconds += 2.5
    events[9] = {
        "seconds": float(events[8]["seconds"]) + 1.0,
        "side": "opponent",
        "event_type": "card_play",
        "card": "hog-rider",
        "x": 2500,
        "y": 24000,
    }
    return BattleExample(
        battle_id="asymmetric-left",
        team_deck=deck_a,
        opponent_deck=deck_b,
        team_wins=1,
        events=tuple(events),
    )


def _legacy_test_first(battles: list[BattleExample], seed: int = 42):
    ordered = list(battles)
    rng = random.Random(seed)
    rng.shuffle(ordered)
    n_test = int(len(ordered) * 0.15)
    return ordered[:n_test]


def _legacy_train_90(battles: list[BattleExample], seed: int = 42):
    ordered = list(battles)
    rng = random.Random(seed)
    rng.shuffle(ordered)
    return ordered[: int(len(ordered) * 0.9)]


def test_old_train90_and_test_first_splits_overlap():
    battles = _id_battles(100)
    train90_ids = {b.battle_id for b in _legacy_train_90(battles)}
    test_first_ids = {b.battle_id for b in _legacy_test_first(battles)}
    assert test_first_ids <= train90_ids


def test_canonical_split_without_train_fraction_is_70_15_15():
    battles = _id_battles(100)
    train, val, test = split_battles(battles, seed=42)
    assert len(train) == 70
    assert len(val) == 15
    assert len(test) == 15


def test_canonical_train_fraction_keeps_held_out_disjoint():
    battles = _id_battles(100)
    train, val, test = split_battles(battles, seed=42, train_fraction=0.9)
    train_ids = {b.battle_id for b in train}
    assert len(train) == 90
    assert not train_ids & {b.battle_id for b in val}
    assert not train_ids & {b.battle_id for b in test}


def test_train_fraction_80_is_eighty_percent():
    battles = _id_battles(100)
    train, val, test = split_battles(battles, seed=7, train_fraction=0.8)
    assert len(train) == 80
    assert len(val) + len(test) == 20


def test_manifest_freezes_ids_when_corpus_grows(tmp_path: Path):
    original = _id_battles(100)
    path = tmp_path / "split_manifest.json"
    manifest = build_manifest(original, path, seed=42)
    grown = original + _id_battles(20)
    grown = [
        BattleExample(b.battle_id if i < 100 else f"extra-{i}", b.team_deck, b.opponent_deck, 0, tuple())
        for i, b in enumerate(grown)
    ]
    train, val, test = battles_from_manifest(grown, manifest)
    assert [b.battle_id for b in train] == manifest["train_ids"]
    assert len(train) + len(val) + len(test) == 100


def test_eval_requires_split_manifest(tmp_path: Path):
    battles = _id_battles(100)
    with pytest.raises(ValueError, match="split manifest"):
        resolve_battle_splits(battles, create=False)
    train, val, test, manifest, stored = resolve_battle_splits(
        battles,
        create=True,
        output_dir=tmp_path,
        seed=42,
    )
    assert stored == tmp_path / "split_manifest.json"
    again = resolve_battle_splits(battles, create=False, model_dir=tmp_path)
    assert [b.battle_id for b in again[0]] == manifest["train_ids"]


def test_threat_features_ignore_future_action_timestamp():
    battle = _asymmetric_battle()
    costs = {"hog-rider": 4, "cannon": 3, "knight": 3}
    feat_a, react_a = recent_opponent_threat(battle, 10, "team", costs)
    mutated = list(battle.events)
    mutated[10] = dict(mutated[10], seconds=float(mutated[10]["seconds"]) + 30.0)
    later = BattleExample(
        battle.battle_id,
        battle.team_deck,
        battle.opponent_deck,
        battle.team_wins,
        tuple(mutated),
    )
    feat_b, react_b = recent_opponent_threat(later, 10, "team", costs)
    assert react_a and react_b
    assert torch.equal(feat_a, feat_b)
    sample_a = encode_policy_sample(battle, 10, CardVocab(list(battle.team_deck) + list(battle.opponent_deck)), costs, threat_dim=THREAT_DIM)
    sample_b = encode_policy_sample(later, 10, CardVocab(list(battle.team_deck) + list(battle.opponent_deck)), costs, threat_dim=THREAT_DIM)
    assert sample_a is not None and sample_b is not None
    assert torch.equal(sample_a[4], sample_b[4])


def test_live_observation_seconds_ages_threats():
    battle = _asymmetric_battle()
    costs = {"hog-rider": 4, "cannon": 3}
    hog_time = float(battle.events[9]["seconds"])
    fresh, fresh_react = recent_opponent_threat(
        battle, 10, "team", costs, observation_seconds=hog_time
    )
    stale, stale_react = recent_opponent_threat(
        battle, 10, "team", costs, observation_seconds=hog_time + 10.0
    )
    assert fresh_react
    assert float(fresh[1]) < float(stale[1]) or not stale_react
    assert not stale_react


def test_unresolved_abilities_are_dropped_not_invented():
    replay = parse_replay(FIXTURE)
    events, reasons = events_from_replay(replay)
    assert "unresolved_ability" in reasons
    assert all(event["card"] != "_ability" for event in events)
    battle, drop_reasons = battle_example_from_replay(
        replay, unresolved_abilities="drop_event", min_card_plays=2
    )
    assert battle is not None
    assert all(event["card"] != "_ability" for event in battle.events)
    quarantined, q_reasons = battle_example_from_replay(
        replay, unresolved_abilities="quarantine", min_card_plays=2
    )
    assert quarantined is None
    assert "unresolved_ability" in q_reasons


def test_dataset_cache_does_not_collide_across_sibling_dirs(tmp_path: Path):
    src = json.loads(FIXTURE.read_text(encoding="utf-8"))
    dir_a = tmp_path / "raw_a"
    dir_b = tmp_path / "raw_b"
    dir_a.mkdir()
    dir_b.mkdir()
    (dir_a / "replay.json").write_text(json.dumps(src), encoding="utf-8")
    src_b = json.loads(json.dumps(src).replace("TEST123", "TEST999"))
    (dir_b / "replay.json").write_text(json.dumps(src_b), encoding="utf-8")
    battles_a = collect_battles(dir_a, min_card_plays=2)
    battles_b = collect_battles(dir_b, min_card_plays=2)
    assert {b.battle_id for b in battles_a} != {b.battle_id for b in battles_b}
    fp_a = source_fingerprint(dir_a)
    fp_b = source_fingerprint(dir_b)
    assert fp_a["digest"] != fp_b["digest"]
    assert default_dataset_cache_path(
        dir_a, kind="winner", digest=fp_a["digest"], min_card_plays=2
    ) != default_dataset_cache_path(
        dir_b, kind="winner", digest=fp_b["digest"], min_card_plays=2
    )


def test_cache_invalidates_when_file_changes_without_count_change(tmp_path: Path):
    src = json.loads(FIXTURE.read_text(encoding="utf-8"))
    raw = tmp_path / "raw"
    raw.mkdir()
    path = raw / "replay.json"
    path.write_text(json.dumps(src), encoding="utf-8")
    first = collect_battles(raw, min_card_plays=2)
    assert first
    before = source_fingerprint(raw)["digest"]
    edited = json.dumps(src).replace("TEST123", "TEST124")
    path.write_text(edited, encoding="utf-8")
    after = source_fingerprint(raw)["digest"]
    assert before != after
    second = collect_battles(raw, min_card_plays=2)
    assert {b.battle_id for b in second} == {"TEST124"}


def test_official_judge_feature_dim_and_synthetic_reject():
    battle = _asymmetric_battle()
    official = extract_tabular_features(battle, {}, {}, {}, include_official=True)
    sequence = extract_tabular_features(battle, {}, {}, {}, include_official=False)
    assert official.shape[0] - sequence.shape[0] == len(OFFICIAL_ELIXIR_FEATURE_NAMES)
    with pytest.raises(ValueError, match="hgb_sequence_ensemble"):
        score_winner_battles({"feature_set": "official"}, [battle], {})


def test_lazy_mirror_flips_asymmetric_left_ratios_and_active_threat_x():
    battle = _asymmetric_battle()
    vocab = CardVocab(list(battle.team_deck) + list(battle.opponent_deck))
    ds = PolicyActionDataset(
        [battle],
        vocab,
        {"hog-rider": 4, "knight": 3, "cannon": 3},
        max_samples_per_battle=12,
        mirror_augmentation=True,
        threat_dim=THREAT_DIM,
        prefer_reactions=True,
    )
    original = ds[0]
    reflected = ds[1]
    team_left = float(original[4][GLOBAL_INDEX["team_left"]])
    assert team_left > 0.7
    assert abs(float(reflected[4][GLOBAL_INDEX["team_left"]]) - (1.0 - team_left)) < 1e-5
    inactive = torch.zeros(GLOBAL_DIM + THREAT_DIM)
    inactive[GLOBAL_INDEX["team_left"]] = 0.8
    cont = torch.zeros(4, 28)
    cont[:, 4] = 0.2
    xy = torch.tensor([0.2, 0.3])
    _, mirrored_g, _ = apply_horizontal_mirror(cont, inactive, xy, threat_dim=THREAT_DIM)
    threat_x = GLOBAL_DIM + THREAT_INDEX["threat_x"]
    assert float(mirrored_g[threat_x]) == 0.0


def test_binary_auc_ties_are_half():
    labels = np.asarray([0, 1, 0, 1])
    scores = np.asarray([0.1, 0.1, 0.9, 0.9])
    assert binary_auc(labels, scores) == pytest.approx(0.5)


def test_baselines_use_dataset_index():
    battle = _asymmetric_battle()
    vocab = CardVocab(list(battle.team_deck) + list(battle.opponent_deck))
    ds = PolicyActionDataset([battle], vocab, {}, max_samples_per_battle=6)
    baselines = baselines_for_dataset(ds)
    assert baselines["frequency"]["n"] == len(ds.index)
    assert baselines["cycle"]["n"] == len(ds.index)
    assert baselines["frequency"]["n"] < sum(
        1 for event in battle.events if event["event_type"] == "card_play"
    )


def test_vocab_size_match_is_not_mapping_compatibility():
    left = CardVocab(["archers", "knight"])
    right = CardVocab.from_dict(
        {
            "pad_id": 0,
            "unk_id": 1,
            "name_to_id": {"archers": 3, "knight": 2},
            "vocab_size": left.vocab_size,
        }
    )
    assert left.vocab_size == right.vocab_size
    assert not left.compatible_with(right)
    assert left.mapping_signature() != right.mapping_signature()


def test_cycle_waits_advance_across_opponent_events():
    battle = _asymmetric_battle()
    costs = {"knight": 3, "hog-rider": 4}
    frozen, _ = acting_cycle_features(battle, 10, costs, swap_sides=False, observation_seconds=float(battle.events[8]["seconds"]))
    advanced, _ = acting_cycle_features(battle, 10, costs, swap_sides=False)
    # Slot 0 is knight, played at the last team action before index 10.
    assert float(advanced[0, 2]) > float(frozen[0, 2])


def test_stable_int_seed_is_process_independent():
    assert stable_int_seed("policy_bc_v5") == stable_int_seed("policy_bc_v5")
    assert stable_int_seed("policy_bc_v5") != stable_int_seed("policy_bc_v4")


def test_style_lessons_do_not_always_claim_the_gap_is_real():
    weak = _style_lessons([], {"auc": 0.51}, [{"auc": 0.5}])
    joined = " ".join(weak)
    assert "does not support a strong claim" in joined
    strong = _style_lessons([{"feature": "alt_rate"}], {"auc": 0.99}, [{"auc": 0.97}])
    assert any("still separate AI from human" in line for line in strong)
