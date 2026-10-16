"""Play a match by submitting legal actions over the loopback protocol."""

from __future__ import annotations

import time
from typing import Callable

from .agents import side_observation
from .engine import IllegalAction
from .wire import ProtocolClient


def progress_line(work: int, sim_s: float, left_s: float, eta_s: float | None, note: str) -> str:
    eta = f"{eta_s:8.1f}" if eta_s is not None else f"{'—':>8}"
    return f"{work:6d} {sim_s:8.1f} {left_s:8.1f} {eta}  {note}"


def progress_table(work: int, sim_s: float, left_s: float, eta_s: float | None, phase: str, note: str) -> str:
    header = f"{'work':>6} {'sim_s':>8} {'left_s':>8} {'eta_s':>8} {'phase':<18} last"
    return header + "\n" + progress_line(work, sim_s, left_s, eta_s, f"{phase:<18} {note}").rstrip() + "\n"


def _eta(speed: float, sim_s: float, left_s: float, wall_s: float) -> float | None:
    if left_s <= 0:
        return 0.0
    if speed > 0:
        return left_s / speed
    if sim_s <= 0.05 or wall_s <= 0:
        return None
    return wall_s / sim_s * left_s


def run_match(
    client: ProtocolClient,
    agents: dict,
    *,
    red: list[str],
    blue: list[str],
    seed: int,
    max_seconds: float,
    speed: float = 0.0,
    stop_event=None,
    on_update: Callable[[dict], None] | None = None,
    echo: Callable[[str], None] | None = print,
) -> dict:
    hello = client.hello()
    if hello.get("protocol") != "cr-local-match/1":
        raise RuntimeError(f"endpoint speaks {hello.get('protocol')!r}, expected cr-local-match/1")
    snapshot = client.start(red, blue, seed=seed, max_seconds=max_seconds)
    if snapshot.get("phase") != "battle" or snapshot.get("menu_steps") != 0:
        raise RuntimeError("match did not open directly in battle")
    wall0 = time.monotonic()
    lines = [
        f"{'work':>6} {'sim_s':>8} {'left_s':>8} {'eta_s':>8}  play",
    ]
    if echo:
        echo(lines[0])
    next_at = {"red": 0.0, "blue": 0.0}
    t = 0.0
    last_note = "battle opened, no menu"
    last_echo = ""
    echoed_second = -1

    def publish(note: str) -> None:
        nonlocal last_echo, echoed_second
        left = max(0.0, max_seconds - snapshot["seconds"])
        eta = _eta(speed, float(snapshot["seconds"]), left, time.monotonic() - wall0)
        line = progress_line(int(snapshot["work"]), float(snapshot["seconds"]), left, eta, note)
        lines.append(line)
        second = int(snapshot["seconds"])
        if echo and (note != last_echo or second != echoed_second):
            echo(line)
            last_echo = note
            echoed_second = second
        view = {
            "running": snapshot.get("phase") == "battle" and float(snapshot["seconds"]) < max_seconds,
            "finished": snapshot.get("phase") != "battle" or float(snapshot["seconds"]) >= max_seconds - 1e-9,
            "phase": "battle" if float(snapshot["seconds"]) < max_seconds else "regulation_ended",
            "work": int(snapshot["work"]),
            "actions": int(snapshot["work"]),
            "sim_s": float(snapshot["seconds"]),
            "sim_left_s": left,
            "eta_wall_s": None if eta is None else round(eta, 2),
            "max_seconds": max_seconds,
            "engine": hello.get("engine"),
            "real_engine": bool(hello.get("real_engine")),
            "combat": bool(hello.get("combat")),
            "transport": client.base_url,
            "protocol": hello.get("protocol"),
            "blocker": hello.get("blocker"),
            "opened_in": "battle",
            "menu_steps": 0,
            "seed": seed,
            "red_deck": list(red),
            "blue_deck": list(blue),
            "red": snapshot["sides"].get("red"),
            "blue": snapshot["sides"].get("blue"),
            "events": snapshot.get("events") or [],
            "lines": lines[-40:],
            "last": note,
            "result": snapshot.get("result"),
        }
        if on_update:
            on_update(view)

    publish("battle opened, no menu")
    while t < max_seconds - 1e-9:
        if stop_event is not None and stop_event.is_set():
            last_note = "stopped"
            publish(last_note)
            break
        snapshot = client.advance(t)
        for side in ("red", "blue"):
            if next_at[side] > t + 1e-9:
                continue
            observation = side_observation(snapshot, side)
            decision = agents[side].choose(observation)
            if decision is None:
                # Ask the harness via a rejected empty wait: use elixir gap.
                gap = _wait_seconds(observation)
                next_at[side] = t + gap
                continue
            try:
                snapshot = client.act(side, decision)
            except IllegalAction as exc:
                next_at[side] = t + 0.5
                last_note = f"{side} rejected ({exc})"
                publish(last_note)
                continue
            event = snapshot.get("event") or {}
            next_at[side] = t + max(0.2, float(decision.recovery_s))
            last_note = (
                f"{side} {event.get('card')} "
                f"tile {event.get('tile_col')},{event.get('tile_row')} "
                f"id {event.get('card_id')}"
            )
            publish(last_note)
        nxt = min(next_at.values())
        if nxt <= t + 1e-9:
            nxt = t + 0.25
        step = min(max_seconds, nxt) - t
        if speed > 0 and step > 0:
            time.sleep(step / speed)
        t = min(max_seconds, nxt)
    if t >= max_seconds - 1e-9:
        snapshot = client.advance(max_seconds)
    left = max(0.0, max_seconds - float(snapshot["seconds"]))
    eta = 0.0
    final_note = last_note if t < max_seconds - 1e-9 else "regulation ended"
    line = progress_line(int(snapshot["work"]), float(snapshot["seconds"]), left, eta, final_note)
    lines.append(line)
    if echo:
        echo(line)
    result = {
        "running": False,
        "finished": True,
        "phase": "stopped" if stop_event is not None and stop_event.is_set() else "regulation_ended",
        "work": int(snapshot["work"]),
        "actions": int(snapshot["work"]),
        "sim_s": float(snapshot["seconds"]),
        "sim_left_s": left,
        "eta_wall_s": 0.0,
        "max_seconds": max_seconds,
        "engine": hello.get("engine"),
        "real_engine": bool(hello.get("real_engine")),
        "combat": bool(hello.get("combat")),
        "transport": client.base_url,
        "protocol": hello.get("protocol"),
        "blocker": hello.get("blocker"),
        "opened_in": "battle",
        "menu_steps": 0,
        "seed": seed,
        "red_deck": list(red),
        "blue_deck": list(blue),
        "red": snapshot["sides"].get("red"),
        "blue": snapshot["sides"].get("blue"),
        "events": snapshot.get("events") or [],
        "lines": lines[-40:],
        "last": final_note,
        "result": snapshot.get("result")
        or "regulation ended; crowns are not resolved without the native engine",
        "error": None,
    }
    if on_update:
        on_update(result)
    return result


def _wait_seconds(observation: dict) -> float:
    elixir = float(observation["elixir"])
    costs = [
        float(cost)
        for cost in observation["costs"].values()
        if cost is not None
    ]
    if not costs:
        return 1.0
    cheapest = min(costs)
    if elixir + 1e-6 >= cheapest:
        return 0.25
    rate = 1.0 / 2.8
    seconds = float(observation["seconds"])
    if seconds >= 240:
        rate = 1.0 / 0.9
    elif seconds >= 120:
        rate = 1.0 / 1.4
    return max(0.25, (cheapest - elixir) / rate)
