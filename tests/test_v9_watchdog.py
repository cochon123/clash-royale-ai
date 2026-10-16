from datetime import datetime
import json
from types import SimpleNamespace
from cr_replay_pipeline import v9_watchdog


def test_watchdog_does_not_duplicate_running_queue(tmp_path,monkeypatch):
    calls=[]
    def run(argv,**kwargs):
        calls.append(argv);return SimpleNamespace(stdout="active\n")
    monkeypatch.setattr(v9_watchdog.subprocess,"run",run)
    assert v9_watchdog.check(tmp_path,datetime.fromisoformat("2026-10-12T04:00:00-06:00").timestamp())=="active"
    assert len(calls)==1 and not (tmp_path/"recovery_attempts.json").exists()


def test_watchdog_restarts_failure_at_most_twice_and_obeys_cutoff(tmp_path,monkeypatch):
    calls=[]
    def run(argv,**kwargs):
        calls.append(argv);return SimpleNamespace(stdout="failed\n")
    monkeypatch.setattr(v9_watchdog.subprocess,"run",run)
    now=datetime.fromisoformat("2026-10-12T04:00:00-06:00").timestamp()
    assert v9_watchdog.check(tmp_path,now)=="restarted"
    assert v9_watchdog.check(tmp_path,now)=="restarted"
    assert v9_watchdog.check(tmp_path,now)=="restart_limit"
    assert len(json.loads((tmp_path/"recovery_attempts.json").read_text()))==2
    assert sum("restart" in call for call in calls)==2
    assert v9_watchdog.check(tmp_path,datetime.fromisoformat("2026-10-12T05:20:00-06:00").timestamp())=="training_cutoff"
    assert v9_watchdog.check(tmp_path,datetime.fromisoformat("2026-10-12T06:00:00-06:00").timestamp())=="deadline"
