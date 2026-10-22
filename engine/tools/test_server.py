#!/usr/bin/env python3
"""Exercise the compiled local server through its public HTTP endpoints.

Run cargo build --release first, then python3 tools/test_server.py [--models].
This launches an isolated process group on a free loopback port and stops it
after the checks. It does not contact the official game or modify AI models.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import time
import traceback
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


class ServerChecks:
    def __init__(self, base_url):
        self.base_url = base_url
        self.results = []

    def request(self, path, body=None, expected=200, timeout=65):
        data = None if body is None else json.dumps(body).encode()
        req = urllib.request.Request(self.base_url + path, data=data, headers={"Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(req, timeout=timeout) as response:
                status, raw = response.status, response.read()
        except urllib.error.HTTPError as exc:
            status, raw = exc.code, exc.read()
        response = json.loads(raw)
        assert status == expected, (path, status, expected, response)
        return response

    def wait(self, match_id, predicate, deadline=4):
        end = time.monotonic() + deadline
        while True:
            state = self.request(f"/api/matches/{match_id}")
            assert not state.get("error"), state
            if predicate(state):
                return state
            summary = {key: state.get(key) for key in ("tick", "elapsed_ms", "remaining_ms", "plays", "outcome", "error")}
            assert time.monotonic() < end, f"Timed out waiting for match {match_id}: {summary}"
            time.sleep(.04)

    def check(self, name, function):
        start = time.monotonic()
        try:
            detail = function()
            self.results.append({"name": name, "passed": True, "seconds": round(time.monotonic()-start, 3), "detail": detail})
            print(f"PASS {name}", flush=True)
        except Exception as exc:
            self.results.append({"name": name, "passed": False, "seconds": round(time.monotonic()-start, 3), "error": str(exc), "traceback": traceback.format_exc()})
            print(f"FAIL {name}: {exc}", flush=True)

    def catalog(self):
        self.cards = self.request("/api/catalog")
        cards = self.cards["cards"]
        assert len(cards) == 97, len(cards)
        ids = {c["id"] for c in cards}
        assert len(ids) == len(cards)
        for preset in self.cards["presets"]:
            assert len(preset["deck"]) == 8 and len(set(preset["deck"])) == 8, preset
            assert set(preset["deck"]) <= ids, preset
        assert len(self.cards["presets"]) >= 4
        self.deck = self.cards["presets"][0]["deck"]
        self.card_costs = {c["id"]: c["elixir"] for c in cards}
        return {"cards": len(cards), "presets": [p["id"] for p in self.cards["presets"]], "controllers": [c["id"] for c in self.cards["controllers"]]}

    def new_match(self, blue, red, speed):
        return self.request("/api/matches", {"blue": {"controller": blue, "deck": self.deck}, "red": {"controller": red, "deck": self.deck}, "seed": 42, "speed": speed})

    def direct_launch(self):
        state = self.new_match("human", "balanced", .25)
        self.human_match = state["id"]
        initial_tick = state["tick"]
        progressed = self.wait(self.human_match, lambda s: s["tick"] > initial_tick and s["plays"] > 0)
        assert not progressed["paused"] and progressed["outcome"] is None
        return {"id": self.human_match, "tick": progressed["tick"], "plays": progressed["plays"]}

    def human_play(self):
        path = f"/api/matches/{self.human_match}"
        before = self.request(path)
        player = before["players"][0]
        choices = [(i, card) for i, card in enumerate(player["hand"]) if self.card_costs[card]*1000 <= player["elixir_milli"]]
        assert choices, player
        slot, card = min(choices, key=lambda pair: self.card_costs[pair[1]])
        after = self.request(path+"/action", {"team": "blue", "hand_index": slot, "x": 4500, "y": 24000})
        new_player = after["players"][0]
        assert new_player["hand"][slot] == player["queue"][0]
        assert new_player["queue"][-1] == card
        spent = player["elixir_milli"] - new_player["elixir_milli"]
        assert self.card_costs[card]*1000-100 <= spent <= self.card_costs[card]*1000, (card, spent)
        replay = self.request(path+"/replay")
        assert any(e["team"] == "blue" and e["card"] == card for e in replay["history"])
        return {"card": card, "hand_index": slot, "spent_milli": spent}

    def illegal_play(self):
        path = f"/api/matches/{self.human_match}"
        before = self.request(path)
        invalid = self.request(path+"/action", {"team": "blue", "hand_index": 0, "x": -1, "y": 24000}, expected=400)
        after = self.request(path)
        assert before["players"][0]["hand"] == after["players"][0]["hand"]
        assert before["players"][0]["queue"] == after["players"][0]["queue"]
        # Regeneration may advance naturally; an invalid action must never spend.
        assert after["players"][0]["elixir_milli"] >= before["players"][0]["elixir_milli"]
        replay = self.request(path+"/replay")
        assert all(e["x"] >= 0 for e in replay["history"])
        return invalid

    def reject_manual_ai(self):
        return self.request(f"/api/matches/{self.human_match}/action", {"team": "red", "hand_index": 0, "x": 4500, "y": 9000}, expected=400)

    def pause_resume(self):
        path = f"/api/matches/{self.human_match}"
        paused = self.request(path+"/control", {"paused": True})
        time.sleep(.3)
        frozen = self.request(path)
        assert frozen["tick"] == paused["tick"], (paused["tick"], frozen["tick"])
        self.request(path+"/control", {"paused": False, "speed": 2})
        resumed = self.wait(self.human_match, lambda s: s["tick"] > frozen["tick"])
        return {"paused_tick": frozen["tick"], "resumed_tick": resumed["tick"]}

    def speed_validation(self):
        path = f"/api/matches/{self.human_match}"
        speed = self.request(path)["config"]["speed"]
        for invalid in (0, -1, 33):
            self.request(path+"/control", {"speed": invalid}, expected=400)
        assert self.request(path)["config"]["speed"] == speed
        return {"rejected": [0, -1, 33], "retained_speed": speed}

    def replay(self):
        replay = self.request(f"/api/matches/{self.human_match}/replay")
        assert {event["team"] for event in replay["history"]} == {"red", "blue"}
        times = [event["elapsed_ms"] for event in replay["history"]]
        assert times == sorted(times)
        assert replay["format"] == "clash-rust-actions-v1"
        self.request(f"/api/matches/{self.human_match}/control", {"paused": True})
        return {"history_events": len(replay["history"]), "teams": sorted({e["team"] for e in replay["history"]})}

    def complete_match(self):
        state = self.new_match("cycle", "pressure", 32)
        finished = self.wait(state["id"], lambda s: s["outcome"] is not None, deadline=15)
        replay = self.request(f"/api/matches/{state['id']}/replay")
        assert {e["team"] for e in replay["history"]} == {"red", "blue"}
        assert finished["elapsed_ms"] <= 300050
        return {"id": state["id"], "elapsed_ms": finished["elapsed_ms"], "plays": finished["plays"], "outcome": finished["outcome"]}

    def actual_models(self):
        controllers = {c["id"] for c in self.cards["controllers"]}
        required = {"policy:v4.4.1", "policy:v4.4.2"}
        assert required <= controllers, (required, controllers)
        started = time.monotonic()
        state = self.new_match("policy:v4.4.1", "policy:v4.4.2", 32)
        progressed = self.wait(state["id"], lambda s: s["plays"] > 1, deadline=15)
        replay = self.request(f"/api/matches/{state['id']}/replay")
        assert {e["team"] for e in replay["history"]} == {"red", "blue"}, replay
        self.request(f"/api/matches/{state['id']}/control", {"paused": True})
        return {"id": state["id"], "plays": progressed["plays"], "elapsed_ms": progressed["elapsed_ms"], "startup_and_play_seconds": round(time.monotonic()-started, 3), "decisions": progressed["decisions"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--models", action="store_true", help="Also launch the actual v4.4.1 and v4.4.2 checkpoints")
    parser.add_argument("--binary", type=Path, default=ROOT/"target/release/clash-rust")
    parser.add_argument("--output", type=Path, default=ROOT/"analysis/server-smoke.json")
    args = parser.parse_args()
    with socket.socket() as reservation:
        reservation.bind(("127.0.0.1", 0))
        port = reservation.getsockname()[1]
    process = subprocess.Popen([str(args.binary.resolve()), "--host", "127.0.0.1", "--port", str(port)], cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True)
    checks = ServerChecks(f"http://127.0.0.1:{port}")
    report = {
        "binary": str(args.binary.resolve()), "base_url": checks.base_url, "actual_models_requested": args.models,
        "binary_sha256": hashlib.sha256(args.binary.read_bytes()).hexdigest(),
        "source_sha256": {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
                          for path in sorted((ROOT/"src").glob("*.rs"))},
        "runner_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    try:
        deadline = time.monotonic()+10
        while True:
            try:
                health = checks.request("/api/health", timeout=.5)
                assert health["ok"]
                break
            except Exception:
                assert process.poll() is None, "Server exited during startup"
                if time.monotonic() > deadline:
                    raise TimeoutError("Server health endpoint did not become ready")
                time.sleep(.05)
        checks.check("catalog and deck presets", checks.catalog)
        if not checks.results[-1]["passed"]:
            return_code = 1
        else:
            for name, function in [
                ("human versus AI launches directly", checks.direct_launch),
                ("human action spends elixir and cycles hand", checks.human_play),
                ("illegal action is rejected without mutation", checks.illegal_play),
                ("manual action on AI side is rejected", checks.reject_manual_ai),
                ("pause freezes and resume progresses", checks.pause_resume),
                ("invalid speed preserves configuration", checks.speed_validation),
                ("replay contains confirmed actions from both sides", checks.replay),
                ("bot versus bot completes at speed 32", checks.complete_match),
            ]:
                checks.check(name, function)
            if args.models:
                checks.check("actual v4.4.1 versus v4.4.2 policies play", checks.actual_models)
            return_code = int(any(not result["passed"] for result in checks.results))
    except Exception as exc:
        report["fatal_error"] = str(exc)
        report["traceback"] = traceback.format_exc()
        return_code = 1
    finally:
        try:
            os.killpg(process.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            log, _ = process.communicate(timeout=3)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            log, _ = process.communicate()
        report.update(results=checks.results, server_log=log[-12000:])
        report["passed"] = not report.get("fatal_error") and bool(checks.results) and all(r["passed"] for r in checks.results)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+"\n")
        print(f"Evidence: {args.output}", flush=True)
    return return_code


if __name__ == "__main__":
    raise SystemExit(main())
