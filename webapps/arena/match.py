"""One live match in the original engine, driven for browsers.

This reuses the desktop watch/play loop pieces from ``engine/rl``: the
``native-watch`` engine process, the learned-delay scheduler for model sides,
the human input validator and the tower-state features. The loop runs in its
own thread at 20 native ticks per second and publishes compact frames that
any number of browser tabs can follow over server-sent events.
"""
from __future__ import annotations

import datetime as dt
import hashlib
import json
import os
import queue
import random
import re
import sys
import threading
import time
from collections import Counter
from pathlib import Path

from webapps.common import catalog, paths

from .placement import CACHE
from .renderer import NativeRenderer, spectator_hands
from .scene import NativeSceneRenderer, browser_scene_enabled

if str(paths.ENGINE) not in sys.path:
    sys.path.insert(0, str(paths.ENGINE))

TICKS_PER_SECOND = 20
STARTUP_TICKS = 100          # the fresh native harness rejects plays before this
MAX_TICKS = 7300
IDLE_CLOSE_SECONDS = 120     # close a match nobody has watched for this long
SESSION_CLOSE_SECONDS = 900  # public browser sessions cannot hold a slot indefinitely
CSV_LOGIC = paths.ENGINE / "analysis/decoded/assets/csv_logic"

_LOAD_LOCK = threading.Lock()


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat()


def _compact(name: str) -> str:
    return "".join(ch.lower() for ch in str(name) if ch.isalnum())


def card_for_row(row) -> dict | None:
    """Card for a native hand row (by numeric id, then by native name)."""
    table = catalog.native_to_card()
    return table.get(row.get("name", "")) or table.get(_compact(row.get("name", "")))


# Units and effects spawned by cards whose native row is not the card's own name.
_DERIVED_ROWS = {
    "skeleton": "Skeletons", "goblinstab": "GoblinGang", "speargoblin": "SpearGoblins",
    "barrelgoblin": "GoblinBarrel", "goblinbarrelspell": "GoblinBarrel", "golemite": "Golem",
    "lavapups": "LavaHound", "lavapup": "LavaHound", "bat": "Bats", "minion": "Minions",
    "megaminion": "MegaMinion", "barbarian": "Barbarians", "goblin": "Goblins",
    "skeletonwarrior": "SkeletonWarriors", "skeletonwarriors": "SkeletonWarriors",
    "elixirgolemite": "ElixirGolem", "freezeicegolemite": "IceGolemite",
    "icegolemitedeatharea": "IceGolemite", "logprojectilerolling": "Log", "logprojectile": "Log",
    "barblogprojectilerolling": "BarbLog", "barblogprojectile": "BarbLog",
    "fireballspell": "Fireball", "rocketspell": "Rocket", "arrowsspell": "Arrows",
    "zapspell": "Zap", "poisonspell": "Poison", "lightningspell": "Lightning",
    "tornadospell": "Tornado", "icespiritsprojectile": "IceSpirits",
}


def describe_row(row_name: str) -> dict:
    """How the browser should draw an entity row: kind, art and label."""
    compact = _compact(row_name)
    table = catalog.native_to_card()
    if row_name in ("KingTower", "PrincessTower"):
        return {"kind": "king" if row_name == "KingTower" else "princess", "art": None, "label": row_name}
    card = table.get(row_name) or table.get(compact) or table.get(_compact(_DERIVED_ROWS.get(compact, "")))
    if card is None and compact.endswith("s") is False:
        card = table.get(compact + "s")
    is_projectile = "projectile" in compact or compact.endswith("deco") or compact.endswith("cannonball")
    is_area = compact.endswith("spell") or "area" in compact or compact.startswith("freeze")
    if is_projectile:
        kind = "projectile"
    elif is_area:
        kind = "area"
    elif card is not None and card["native"] in _building_natives():
        kind = "building"
    else:
        kind = "troop"
    return {"kind": kind, "art": card["art"] if card else None,
            "label": card["name"] if card else re.sub(r"(?<=[a-z])(?=[A-Z])", " ", row_name)}


_BUILDINGS: set[str] | None = None


def _building_natives() -> set[str]:
    global _BUILDINGS
    if _BUILDINGS is None:
        rows = json.loads((paths.ENGINE_DATA / "cards.json").read_text())
        by_id = {row["id"]: row for row in rows if isinstance(row, dict)}
        _BUILDINGS = set()
        for card in catalog.cards().values():
            row = by_id.get(card["id"].replace("-", "_")) or by_id.get(card["id"])
            if row and row.get("kind") == "building":
                _BUILDINGS.add(card["native"])
        _BUILDINGS |= {"Cannon", "InfernoTower", "Tesla", "Mortar", "XBow", "BombTower", "Tombstone",
                       "GoblinHut", "Furnace", "BarbarianHut", "ElixirCollector", "GoblinCage", "GoblinDrill"}
    return _BUILDINGS


class RandomController:
    """Baseline: a random affordable card on a random legal tile every 2-6 s."""

    def __init__(self, side: int, deck: list[str], seed: int, own_side_only: set[str]):
        self.side, self.deck = side, deck
        self.rng = random.Random(seed)
        self.own_side_only = own_side_only
        self.next_at = 2.0

    def choose(self, engine, obs, frame, seconds):
        from rl.banana_human import HumanAction
        if seconds < self.next_at:
            return None
        elixir = float(obs["elixir"][self.side])
        hand = [row for row in obs["hands"][self.side]
                if row.get("available", True) and float(row.get("cost", 99)) <= elixir + 1e-6]
        if not hand:
            return None
        row = self.rng.choice(hand)
        mask = CACHE.lookup(engine, "tiles", self.side, int(row["slot"]), row["name"], frame)
        own = (lambda y: y < 15000) if self.side == 0 else (lambda y: y >= 17000)
        tiles = [i for i, legal in enumerate(mask) if legal]
        if row["name"] in self.own_side_only:
            tiles = [i for i in tiles if own((i // 18) * 1000 + 500)]
        if not tiles:
            return None
        tile = self.rng.choice(tiles)
        card = card_for_row(row)
        self.next_at = seconds + self.rng.uniform(2.0, 6.0)
        return HumanAction(int(row["slot"]), card["id"] if card else row["name"], (tile % 18) * 1000 + 500,
                           (tile // 18) * 1000 + 500, int(row["card_id"]), 0, side=self.side)

    def observe_result(self, action, result):
        if not result.get("accepted"):
            self.next_at = 0.0


class Match:
    def __init__(self, match_id: str, config: dict):
        self.id = match_id
        self.config = config
        self.owner = config.get("owner")
        self.sides = config["sides"]
        self.human = self.sides[0]["controller"] == "human"
        self.created_at = utc_now()
        self.seed = int(config["seed"])
        self.status = "loading"
        self.message = "Starting the original engine and loading the models..."
        self.error = None
        self.inputs: queue.Queue = queue.Queue()
        self.controls: queue.Queue = queue.Queue()
        self.cond = threading.Condition()
        self.payload = None
        self.seq = 0
        self.subscribers = 0
        self.last_watched = time.monotonic()
        self.closed = threading.Event()
        self.rows: dict[str, dict] = {}
        self.labels = [side.get("label") or side["controller"] for side in self.sides]
        stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        self.run_dir = paths.ENGINE_RUNS / "arena" / f"{stamp}-{match_id}"
        self.result_history: list[dict] = []
        self.renderer = None
        self.log_streams = []
        self.thread = threading.Thread(target=self._run, name=f"match-{match_id}", daemon=True)
        self.expiry_timer = None
        if self.owner is not None:
            self.expiry_timer = threading.Timer(SESSION_CLOSE_SECONDS, self.close)
            self.expiry_timer.daemon = True
            self.expiry_timer.start()
        self.thread.start()

    # ----- public, thread-safe -------------------------------------------------
    def summary(self) -> dict:
        return {"id": self.id, "status": self.status, "message": self.message, "error": self.error,
                "mode": "human" if self.human else "watch", "created_at": self.created_at,
                "labels": self.labels, "decks": [s["deck"] for s in self.sides],
                "controllers": [s["controller"] for s in self.sides], "seed": self.seed,
                "subscribers": self.subscribers, "results": self.result_history,
                "run_dir": paths.relative(self.run_dir)}

    def submit_input(self, command: dict):
        self.inputs.put(command)

    def control(self, command: dict):
        self.controls.put(command)

    def close(self):
        self.closed.set()
        if self.expiry_timer is not None:
            self.expiry_timer.cancel()
        with self.cond:
            self.cond.notify_all()

    def meta(self) -> dict:
        cards = catalog.cards()
        return {"id": self.id, "mode": "human" if self.human else "watch", "labels": self.labels,
                "decks": [[{"id": c, "name": cards[c]["name"], "art": cards[c]["art"], "elixir": cards[c]["elixir"]}
                           for c in side["deck"]] for side in self.sides],
                "rows": dict(self.rows), "arena": {"width": 18000, "height": 32000, "columns": 18, "rows": 32},
                "renderer": {"kind": "browser-scene" if browser_scene_enabled() else "original-spectator",
                             "width": 540, "height": 900},
                "ticks_per_second": TICKS_PER_SECOND}

    def events(self):
        """Server-sent event stream for one browser tab."""
        self.subscribers += 1
        last = -1
        interval = 1 / 30
        next_at = 0.
        try:
            yield "meta", self.meta()
            sent_rows = set(self.rows)
            while not self.closed.is_set():
                if self.closed.wait(max(0., next_at - time.monotonic())):
                    break
                with self.cond:
                    self.cond.wait_for(lambda: self.seq != last or self.closed.is_set(), timeout=10)
                    payload, seq = self.payload, self.seq
                self.last_watched = time.monotonic()
                if seq == last or payload is None:
                    yield "status", self.summary()
                    continue
                last = seq
                new_rows = {k: v for k, v in self.rows.items() if k not in sent_rows}
                if new_rows:
                    sent_rows |= new_rows.keys()
                    yield "rows", new_rows
                yield "frame", payload
                next_at = time.monotonic() + interval
            yield "closed", {"id": self.id}
        finally:
            self.subscribers -= 1
            self.last_watched = time.monotonic()

    # ----- match thread ----------------------------------------------------------
    def _publish(self, payload: dict):
        with self.cond:
            self.payload = payload
            self.seq += 1
            self.cond.notify_all()

    def _status(self, status: str, message: str | None = None):
        self.status = status
        if message is not None:
            self.message = message
        with self.cond:
            self.seq += 1
            if self.payload is not None:
                self.payload = dict(self.payload, status=status, message=self.message)
            else:
                self.payload = {"status": status, "message": self.message, "loading": True}
            self.cond.notify_all()

    def _run(self):
        engine = None
        try:
            with _LOAD_LOCK:  # one engine + model load at a time keeps startup predictable
                engine, policies = self._load()
            if self.closed.is_set():
                return
            known_cards = catalog.cards()
            native_cards = [known_cards[slug]["native"] for side in self.sides for slug in side["deck"]]
            self.renderer = (NativeSceneRenderer if browser_scene_enabled() else NativeRenderer)(
                self.run_dir, cards=native_cards)
            self._loop(engine, policies)
        except Exception as exc:  # surface to the browser instead of dying silently
            import traceback
            traceback.print_exc()
            self.error = f"{type(exc).__name__}: {exc}"
            self._status("failed", self.error)
        finally:
            if engine is not None:
                engine.close()
            if self.renderer is not None:
                self.renderer.close()
            for stream in self.log_streams:
                stream.close()
            if self.status != "failed":
                self._status("closed", "Match closed.")
            self.close()

    def _load(self):
        import torch
        from rl.banana_train import JsonlEngine
        from rl.banana_watch import load_watched_policy
        torch.set_num_threads(max(1, int(os.environ.get("ARENA_TORCH_THREADS", min(2, os.cpu_count() or 1)))))
        torch.manual_seed(self.seed)
        policies = [None, None]
        for side, spec in enumerate(self.sides):
            controller = spec["controller"]
            if controller in ("human", "random"):
                continue
            model = catalog.engine_model(controller)
            self.message = f"Loading {model['name']}..."
            self._status("loading")
            policy = load_watched_policy(paths.REPO / model["checkpoint"], "cpu", self.seed + side, model["name"])
            policies[side] = policy
        self.message = "Starting the original engine..."
        self._status("loading")
        if not paths.NATIVE_WATCH.is_file():
            raise RuntimeError("native-watch is not built: cargo build --release --offline "
                               "--manifest-path engine/native-engine/Cargo.toml --bin native-watch")
        engine = JsonlEngine(paths.ENGINE_CHECKPOINT, paths.NATIVE_WATCH)
        return engine, policies

    def _loop(self, engine, policies):
        from rl.banana_human import HumanController, own_side_only_cards
        from rl.banana_policy import BananaEpisode
        from rl.banana_placement import resolved_play_target
        from rl.banana_timed_policy import PhoneStyleScheduler
        from rl.banana_watch import tower_state_from_frame

        cards = catalog.cards()
        decks = [list(side["deck"]) for side in self.sides]
        natives = [[cards[c]["native"] for c in deck] for deck in decks]
        own_side_only = own_side_only_cards(CSV_LOGIC)
        self.run_dir.mkdir(parents=True, exist_ok=True)
        (self.run_dir / "manifest.json").write_text(json.dumps({
            "created_at": self.created_at, "match": self.id, "mode": "human" if self.human else "watch",
            "controllers": [s["controller"] for s in self.sides], "labels": self.labels, "decks": decks,
            "native_decks": natives, "seed": self.seed, "engine": paths.relative(paths.NATIVE_WATCH),
            "engine_sha256": hashlib.sha256(paths.NATIVE_WATCH.read_bytes()).hexdigest(),
            "checkpoints": [catalog.engine_model(s["controller"]).get("checkpoint")
                            if s["controller"] not in ("human", "random") else None for s in self.sides],
            "learning": False, "source": "webapps.arena",
            "placement_history": "native-resolved-target-v1",
        }, indent=2) + "\n")
        actions_log = (self.run_dir / "actions.jsonl").open("a", buffering=1)
        results_log = (self.run_dir / "results.jsonl").open("a", buffering=1)
        self.log_streams.extend((actions_log, results_log))

        def slug_for_row(row):
            card = card_for_row(row)
            return card["id"] if card else _compact(row.get("name", ""))

        def placement_mask(request, native_slot):
            _episode, observation, side, _now = request
            row = next(h for h in observation["hands"][side] if int(h["slot"]) == native_slot)
            return CACHE.lookup(engine, "policy", side, native_slot, row["name"], game["frame"])

        active = tuple(side for side in (0, 1) if policies[side] is not None)
        scheduler = PhoneStyleScheduler(policies, placement_mask_provider=placement_mask,
                                        active_sides=active) if active else None
        randoms = {side: RandomController(side, decks[side], self.seed * 7 + side, own_side_only)
                   for side in (0, 1) if self.sides[side]["controller"] == "random"}
        human = HumanController(engine, decks[0], card_name=slug_for_row, own_side_only=own_side_only) \
            if self.human else None

        state = dict(match_number=0, paused=False, ended=False, speed=1.0)
        game = {}

        def reset():
            state["match_number"] += 1
            match_seed = (self.seed + state["match_number"] - 1) % 2**32
            self.message = "Building a fresh original-engine battle..."
            self._status("loading")
            if scheduler is not None:
                scheduler.plans, scheduler.status, scheduler.revision = [None, None], [{}, {}], None
            r = engine.request({"op": "reset", "deck0": natives[0], "deck1": natives[1],
                                "seed": match_seed, "max_ticks": MAX_TICKS})
            initial_towers = tower_state_from_frame(r["frame"])
            configured = r.get("configured_decks") or []
            for side in (0, 1):
                got = configured[side].get("cards") if side < len(configured) else None
                if Counter(got or []) != Counter(natives[side]):
                    raise RuntimeError(f"engine built the wrong deck for side {side}: {got}")
            r = engine.request({"op": "step", "actions": [], "ticks": STARTUP_TICKS})
            obs, frame = r["observation"], r["frame"]
            obs["tower_state"] = tower_state_from_frame(frame, initial_towers)
            episode = BananaEpisode((tuple(decks[0]), tuple(decks[1])), battle_id=f"arena-{self.id}-{state['match_number']}")
            episode.policy_seed = match_seed
            for controller in randoms.values():
                controller.next_at = 2.0
            if human is not None:
                human.reset(state["match_number"])
            game.update(obs=obs, frame=frame, initial_towers=initial_towers, base_tick=int(obs["tick"]),
                        episode=episode, plays=[], play_id=0, accepted=[0, 0], rejected=[0, 0],
                        match_seed=match_seed)
            state.update(paused=False, ended=False)
            self._status("running", f"{self.labels[0]} vs {self.labels[1]}")

        def towers():
            rows = game["obs"]["tower_state"]["towers"]
            result = []
            for row in rows:
                result.append({"side": row["side"], "kind": "king" if row["row"] == "KingTower" else "princess",
                               "hp": row["hp"], "max_hp": row["max_hp"]})
            return result

        def crowns():
            score = [0, 0]
            for row in towers():
                if row["hp"] <= 0:
                    score[1 - row["side"]] += 3 if row["kind"] == "king" else 1
            return [min(3, s) for s in score]

        def publish():
            obs, frame = game["obs"], game["frame"]
            entities = []
            for e in frame["entities"]:
                row = e["row"]
                if row not in self.rows:
                    self.rows[row] = describe_row(row)
                entities.append({"i": e["native_entity_id"], "r": row, "s": e["side"], "x": e["x"], "y": e["y"],
                                 "z": e.get("z") or 0, "hp": e.get("hp") or 0, "mhp": e.get("max_hp") or 0,
                                 "a": bool(e.get("attacking")), "m": bool(e.get("moving"))})
            hands = []
            for side in (0, 1):
                if self.human and side == 1:
                    hands.append(None)  # the opponent's hand stays hidden, as in the real game
                    continue
                hand = []
                for row in obs["hands"][side]:
                    card = card_for_row(row)
                    hand.append({"slot": row["slot"], "card_id": row["card_id"], "cost": row["cost"],
                                 "available": row.get("available", True), "name": card["name"] if card else row["name"],
                                 "id": card["id"] if card else None, "art": card["art"] if card else None})
                hands.append(hand)
            tick = int(obs["tick"])
            plans = []
            if scheduler is not None:
                for side in (0, 1):
                    status = scheduler.status[side] if side in active else {}
                    plans.append(None if (self.human and side == 1) or not status else
                                 {k: status.get(k) for k in ("card", "reason", "ready_at", "cost")})
            payload = {
                "status": "ended" if state["ended"] else "paused" if state["paused"] else "running",
                "message": self.message, "match_number": state["match_number"], "seed": game["match_seed"],
                "tick": tick, "elapsed_ms": max(0, tick - game["base_tick"]) * 50,
                "elixir": frame.get("elixir") or obs["elixir"], "hands": hands, "entities": entities,
                "towers": towers(), "crowns": crowns(), "winner_side": obs.get("winner_side"),
                "plays": [p for p in game["plays"] if tick - p["tick"] <= 40],
                "accepted": game["accepted"], "rejected": game["rejected"], "speed": state["speed"],
                "plans": plans,
            }
            self.status = payload["status"]
            if human is not None:
                snapshot = human.snapshot(obs, paused=state["paused"], entities=frame["entities"],
                                          arena_size=frame.get("arena_size", (18000, 32000)))
                payload["human"] = {"selection": snapshot["selection"], "target": snapshot["target"],
                                    "forbidden": snapshot["forbidden"], "feedback": snapshot["feedback"],
                                    "last_serial": snapshot["last_serial"],
                                    "match_id": state["match_number"]}
            # Feed the exact native frame schema to the desktop spectator's
            # Painter. Keep motion, focus, elevation and impact fields intact.
            original = dict(frame, elapsed_ms=payload["elapsed_ms"],
                            controller_names=self.labels, runtime_status=payload["status"],
                            playback_speed=state["speed"], match_id=state["match_number"],
                            terminated=state["ended"], winner_side=obs.get("winner_side"),
                            hands=spectator_hands(obs["hands"], hide_opponent=self.human),
                            plays=[dict(p, name=p["native_name"]) for p in game["plays"]
                                   if tick - p["tick"] <= 40])
            if human is not None:
                original["human"] = snapshot
            self.renderer.submit(original)
            self._publish(payload)

        def record_end(r):
            obs = game["obs"]
            winner = obs.get("winner_side")
            result = {"match_number": state["match_number"], "seed": game["match_seed"], "ended_at": utc_now(),
                      "terminated": bool(r.get("terminated")), "truncated": bool(r.get("truncated")),
                      "winner_side": winner, "crowns": crowns(),
                      "game_seconds": (int(obs["tick"]) - game["base_tick"]) / TICKS_PER_SECOND,
                      "accepted": game["accepted"], "rejected": game["rejected"]}
            self.result_history.append(result)
            results_log.write(json.dumps(result) + "\n")
            if result["truncated"]:
                self.message = "Time limit reached without a native result."
            elif winner is None:
                self.message = "Draw."
            else:
                self.message = f"{self.labels[winner]} wins."

        reset()
        publish()
        due = time.monotonic()
        human_action = None  # a validated human play waits here for the next engine tick
        while not self.closed.is_set():
            if self.subscribers == 0 and time.monotonic() - self.last_watched > IDLE_CLOSE_SECONDS:
                break
            changed = False
            while not self.controls.empty():
                command = self.controls.get_nowait()
                op = command.get("op")
                if op == "pause":
                    state["paused"] = not state["paused"] if command.get("value") is None else bool(command["value"])
                elif op == "reset":
                    reset()
                    human_action = None
                elif op == "speed":
                    state["speed"] = max(0.25, min(8.0, float(command.get("value", 1))))
                due = time.monotonic()
                changed = True
            if human is not None:
                while not self.inputs.empty() and human_action is None:
                    command = self.inputs.get_nowait()
                    outcome = human.handle(command, game["obs"], paused=state["paused"], ended=state["ended"],
                                           arena_size=game["frame"].get("arena_size", (18000, 32000)),
                                           entities=game["frame"]["entities"])
                    changed = True
                    if outcome == "reset":
                        reset()
                        human_action = None
                        due = time.monotonic()
                    elif outcome == "pause":
                        state["paused"] = not state["paused"]
                        due = time.monotonic()
                    elif outcome not in (None, "close"):
                        human_action = outcome
                # Original viewer previews use exact validate_target queries
                # plus native forbidden zones; no full-grid scan on selection.
            if changed:
                publish()
            if state["paused"] or state["ended"]:
                with self.cond:
                    self.cond.wait(timeout=0.03)
                continue
            now = time.monotonic()
            if now < due:
                time.sleep(min(0.01, due - now))
                continue
            obs = game["obs"]
            seconds = (int(obs["tick"]) - game["base_tick"]) / TICKS_PER_SECOND
            actions = []
            if human_action is not None:
                actions.append(human_action)
                human_action = None
            if scheduler is not None:
                actions.extend(scheduler.choose(game["episode"], obs, seconds))
            for side, controller in randoms.items():
                if not any(a.side == side for a in actions):
                    chosen = controller.choose(engine, obs, game["frame"], seconds)
                    if chosen is not None:
                        actions.append(chosen)
            played = [a for a in actions if not a.wait]
            r = engine.request({"op": "step", "actions": [a.native_request() for a in played], "ticks": 1})
            results = r.get("actions") or []
            if len(results) != len(played):
                raise RuntimeError("engine returned mismatched action results")
            for action, result in zip(played, results):
                controller = ("human" if human is not None and action.side == 0
                              else "random" if action.side in randoms else "model")
                if controller == "human":
                    human.observe_result(action, result, int(obs["tick"]))
                elif controller == "random":
                    randoms[action.side].observe_result(action, result)
                else:
                    scheduler.observe_result(action, result)
                if result.get("accepted") is True:
                    land_x, land_y = resolved_play_target(result)
                    game["accepted"][action.side] += 1
                    game["episode"].record_action(action.side, action.card, land_x, land_y, seconds, True)
                    row = next((h for h in obs["hands"][action.side] if int(h["slot"]) == action.slot), {})
                    card = card_for_row(row) if row else None
                    game["play_id"] += 1
                    game["plays"].append({"id": game["play_id"], "side": action.side, "x": land_x, "y": land_y,
                                          "tick": int(obs["tick"]), "name": card["name"] if card else action.card,
                                          "native_name": row.get("name", action.card),
                                          "card_id": int(row.get("card_id", 0)), "cost": int(row.get("cost", 0)),
                                          "art": card["art"] if card else None})
                    game["plays"] = game["plays"][-24:]
                else:
                    game["rejected"][action.side] += 1
                actions_log.write(json.dumps({"match_number": state["match_number"], "game_seconds": seconds,
                                              "side": action.side, "controller": controller, "card": action.card,
                                              "request": action.native_request(), "result": result}) + "\n")
            obs, frame = r["observation"], r["frame"]
            obs["tower_state"] = tower_state_from_frame(frame, game["initial_towers"])
            game["obs"], game["frame"] = obs, frame
            if r.get("terminated") or r.get("truncated"):
                state["ended"] = True
                record_end(r)
                self.status = "ended"
            elif self.status != "running":
                self.status = "running"
            publish()
            due += 1.0 / (TICKS_PER_SECOND * state["speed"])
            if time.monotonic() - due > 0.25:  # never fast-forward a backlog
                due = time.monotonic()


class MatchManager:
    def __init__(self, max_matches: int = 3):
        self.max_matches = max_matches
        self.matches: dict[str, Match] = {}
        self.lock = threading.Lock()

    def create(self, config: dict) -> Match:
        with self.lock:
            for match_id in [m for m, match in self.matches.items() if match.closed.is_set()]:
                del self.matches[match_id]
            owner = config.get("owner")
            if owner is not None and any(match.owner == owner for match in self.matches.values()):
                raise RuntimeError("you already have a match running; end it first")
            if len(self.matches) >= self.max_matches:
                raise RuntimeError(f"{self.max_matches} matches are already running; end one first")
            match_id = hashlib.blake2b(os.urandom(16), digest_size=5).hexdigest()
            match = Match(match_id, config)
            self.matches[match_id] = match
            return match

    def get(self, match_id: str) -> Match | None:
        return self.matches.get(match_id)

    def list(self, owner: str | None = None) -> list[dict]:
        with self.lock:
            return [match.summary() for match in self.matches.values()
                    if not match.closed.is_set() and (owner is None or match.owner == owner)]

    def close(self, match_id: str) -> bool:
        match = self.matches.get(match_id)
        if match is None:
            return False
        match.close()
        return True

    def close_all(self):
        for match in list(self.matches.values()):
            match.close()
