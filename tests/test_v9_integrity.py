import json
from pathlib import Path
from types import SimpleNamespace
import os

import numpy as np
import pytest
from PIL import Image

from cr_replay_pipeline.api import ClashRoyaleAPI,discover_battles
from cr_replay_pipeline.phone_lab.hand_detect import HandDetector
from cr_replay_pipeline.v9_evaluate import paired_comparison,freeze_selection,holdout_pass


def test_api_reads_private_key_file_without_recording_it(tmp_path):
    key=tmp_path/"key";key.write_text("  private-test-key\n")
    assert ClashRoyaleAPI(token_file=key).token=="private-test-key"


def test_empty_api_discovery_preserves_existing_metadata(tmp_path,monkeypatch):
    output=tmp_path/"battles.jsonl"
    existing=dict(battle_time="20260101T000000.000Z",team=[dict(tag="#ABC",crowns=1)],opponent=[dict(tag="#DEF",crowns=0)])
    output.write_text(json.dumps(existing)+"\n")
    monkeypatch.setattr(ClashRoyaleAPI,"leaderboard",lambda *a:[{"tag":"#ABC"}])
    monkeypatch.setattr(ClashRoyaleAPI,"battlelog",lambda *a:[])
    before=output.read_bytes();report=discover_battles(output,token="private-test-key")
    assert report["unique_battles"]==0 and output.read_bytes()==before
    assert "private-test-key" not in json.dumps(report)


def test_paired_rows_require_identical_targets(tmp_path):
    a,b=tmp_path/"a.npz",tmp_path/"b.npz"
    np.savez(a,groups=[0,0,1],events=[8,9,8],rows=np.array([[0.,1.],[1.,2.],[0.,3.]]),columns=["card","joint_nll"])
    np.savez(b,groups=[0,0,1],events=[8,10,8],rows=np.ones((3,2)),columns=["card","joint_nll"])
    with pytest.raises(ValueError,match="targets differ"):paired_comparison(a,b,20)
    np.savez(b,groups=[0,0,1],events=[8,9,8],rows=np.array([[1.,.5],[1.,1.5],[1.,2.5]]),columns=["card","joint_nll"])
    result=paired_comparison(a,b,200)
    assert result["battles"]==2
    assert result["metrics"]["joint_nll"]["ci95_battle"]==[-.5,-.5]


def test_final_selection_is_immutable_and_completed_holdout_is_read_only(tmp_path):
    run=tmp_path/"run";run.mkdir();(run/"best.pt").write_bytes(b"checkpoint")
    entries=[dict(name="candidate",path=str(run),role="candidate")]
    selection=freeze_selection(tmp_path,entries,dict(width=160))
    assert freeze_selection(tmp_path,entries,dict(width=160))==selection
    with pytest.raises(ValueError,match="already frozen"):freeze_selection(tmp_path,entries,dict(width=256))
    from cr_replay_pipeline.artifact_cache import sha256_json
    (tmp_path/"final_holdout_state.json").write_text(json.dumps(dict(selection_sha256=sha256_json(selection),status="completed",scored=["candidate"])))
    final=dict(already_scored=True);(tmp_path/"final_holdout.json").write_text(json.dumps(final))
    assert holdout_pass("nonexistent-cache",tmp_path,"cpu")==final


def test_truncated_candidate_cannot_start_holdout_transaction(tmp_path):
    run=tmp_path/"run";run.mkdir();(run/"best.pt").write_bytes(b"checkpoint")
    (run/"report.json").write_text(json.dumps(dict(status="budget_stopped",last_completed_epoch=3)))
    freeze_selection(tmp_path,[dict(name="candidate",path=str(run),role="candidate")],dict(base_epochs=12))
    with pytest.raises(ValueError,match="has not completed"):
        holdout_pass("nonexistent-cache",tmp_path,"cpu")
    assert not (tmp_path/"final_holdout_state.json").exists()


def test_degenerate_hand_identity_is_unknown(tmp_path):
    detector=HandDetector(tmp_path/"unused.pt")
    detector._model=SimpleNamespace(predict=lambda **kw:[None]*4)
    detector._best_from_result=lambda result:("snowball",.99,None)
    slots=detector.detect_crop_images([Image.new("RGB",(100,150)) for _ in range(4)])
    assert all(s["card_name"] is None for s in slots)
    assert all("repeated identity" in s["error"] for s in slots)


def test_recent_metadata_cannot_label_an_old_replay(tmp_path):
    from cr_replay_pipeline.metadata import MetadataIndex
    source=tmp_path/"replay.json";source.write_text("{}")
    from datetime import datetime,timezone
    stamp=datetime(2026,8,1,tzinfo=timezone.utc).timestamp();os.utime(source,(stamp,stamp))
    replay=SimpleNamespace(source_file=str(source),participant_tags={"team":["AAA"],"opponent":["BBB"]},crowns={"team":1,"opponent":0},decks={"team":[],"opponent":[]})
    record=dict(battle_time="20261011T000000.000Z",team=[dict(tag="AAA",crowns=1)],opponent=[dict(tag="BBB",crowns=0)])
    result=MetadataIndex([record]).match(replay)
    assert result.record is None and result.reason=="metadata_deck_or_time_mismatch"
