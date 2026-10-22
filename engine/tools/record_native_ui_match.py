#!/usr/bin/env python3
"""Record two local policies playing under the recovered original HUD artwork.

The hand slots, elixir bar and panel are rasterized from the original SC5
display objects. Troop movement still comes from the local battle simulation:
the original battle loop is not what advances this match.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
PARTS = ROOT / "analysis/native-ui/rust-rendered/match-parts"
CARDS = ROOT / "web/cards"
HEADLESS = ROOT / "target/release/headless"
OUT = ROOT / "runs/native-ui-ai-match"
HOG = [
    "hog_rider",
    "musketeer",
    "ice_golem",
    "ice_spirit",
    "skeletons",
    "cannon",
    "fireball",
    "the_log",
]
LOGBAIT = [
    "goblin_barrel",
    "princess",
    "knight",
    "goblin_gang",
    "inferno_tower",
    "rocket",
    "the_log",
    "ice_spirit",
]

# Parent timeline placement of the original HUD_player children.
PLACEMENT = {
    "panel": ((-250.0, -227.0), (55.0, 21.0)),
    "slots": ((-210.0, -58.0), (54.54999923706055, -114.0)),
    "elixir": ((-40.0, -29.0), (-131.9499969482422, -25.950000762939453)),
}
SLOT_SPANS = [(0, 94), (108, 202), (217, 311), (327, 421)]
TRACK = (58, 438, 18, 43)
TIP_SPAN = (112, 140)


def font(size: int) -> ImageFont.ImageFont:
    for path in (
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
    ):
        if Path(path).is_file():
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


class Engine:
    def __init__(self) -> None:
        self.proc = subprocess.Popen(
            [str(HEADLESS)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
        )

    def call(self, payload: dict) -> dict:
        assert self.proc.stdin and self.proc.stdout
        self.proc.stdin.write(json.dumps(payload) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            raise RuntimeError("headless engine closed")
        reply = json.loads(line)
        if not reply.get("ok", True):
            raise RuntimeError(reply.get("error", reply))
        return reply

    def close(self) -> None:
        if self.proc.stdin:
            self.proc.stdin.close()
        self.proc.wait(timeout=5)


class Policy:
    def __init__(self, model: str, log_path: Path) -> None:
        self.model = model
        self.log = log_path.open("w")
        self.proc = subprocess.Popen(
            [sys.executable, str(ROOT / "tools/policy_bridge.py"), "--model", model],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=self.log,
            text=True,
        )
        assert self.proc.stdout
        ready = json.loads(self.proc.stdout.readline())
        if not ready.get("ready"):
            raise RuntimeError(f"{model} failed to load: {ready}")

    def ask(self, payload: dict) -> dict:
        assert self.proc.stdin and self.proc.stdout
        self.proc.stdin.write(json.dumps(payload) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            raise RuntimeError(f"{self.model} closed")
        return json.loads(line)

    def close(self) -> None:
        if self.proc.stdin:
            self.proc.stdin.close()
        self.proc.wait(timeout=10)
        self.log.close()


def load_parts() -> tuple[Image.Image, dict[str, tuple[int, int]]]:
    images = {name: Image.open(PARTS / f"{name}.png").convert("RGBA") for name in PLACEMENT}
    placed = {}
    for name, (origin, parent) in PLACEMENT.items():
        placed[name] = (origin[0] + parent[0], origin[1] + parent[1])
    min_x = min(placed[name][0] for name in placed)
    min_y = min(placed[name][1] for name in placed)
    max_x = max(placed[name][0] + images[name].width for name in placed)
    max_y = max(placed[name][1] + images[name].height for name in placed)
    pad = 76
    panel = images["panel"]
    background = panel.getpixel((panel.width // 2, panel.height // 2))
    canvas = Image.new(
        "RGBA",
        (int(max_x - min_x) + 1 + pad, int(max_y - min_y) + 1),
        background,
    )
    order = ["panel", "elixir", "slots"]
    offsets = {}
    for name in order:
        box = (int(round(placed[name][0] - min_x)) + pad, int(round(placed[name][1] - min_y)))
        canvas.alpha_composite(images[name], box)
        offsets[name] = box
    return canvas, offsets


def card_image(card_id: str, cache: dict[str, Image.Image]) -> Image.Image:
    if card_id not in cache:
        path = CARDS / f"{card_id}.png"
        if path.is_file():
            cache[card_id] = Image.open(path).convert("RGBA")
        else:
            tile = Image.new("RGBA", (128, 128), (70, 78, 96, 255))
            draw = ImageDraw.Draw(tile)
            draw.text((8, 52), card_id[:8], fill=(240, 236, 220, 255), font=font(16))
            cache[card_id] = tile
    return cache[card_id]


def rounded_paste(base: Image.Image, art: Image.Image, box: tuple[int, int, int, int]) -> None:
    x0, y0, x1, y1 = box
    width, height = x1 - x0, y1 - y0
    if width < 4 or height < 4:
        return
    fitted = art.resize((width, height), Image.Resampling.LANCZOS)
    mask = Image.new("L", (width, height), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, width - 1, height - 1), radius=max(6, width // 8), fill=255)
    fitted.putalpha(mask)
    base.alpha_composite(fitted, (x0, y0))


def paint_elixir(layer: Image.Image, elixir_box: tuple[int, int], fraction: float) -> None:
    elixir = Image.open(PARTS / "elixir.png").convert("RGBA")
    x0, x1, y0, y1 = TRACK
    tip = elixir.crop((TIP_SPAN[0], 0, TIP_SPAN[1], elixir.height))
    for x in range(TIP_SPAN[0], TIP_SPAN[1]):
        for y in range(y0, y1):
            elixir.putpixel((x, y), elixir.getpixel((200, (y0 + y1) // 2)))
    fraction = max(0.0, min(1.0, fraction))
    fill_end = x0 + int((x1 - x0) * fraction)
    for x in range(x0, fill_end):
        for y in range(y0, y1):
            t = (y - y0) / max(1, y1 - y0)
            elixir.putpixel((x, y), (int(236 - 50 * t), int(64 + 40 * t), int(214 - 30 * t), 255))
    if fraction > 0.03:
        elixir.alpha_composite(tip, (max(x0, fill_end - tip.width // 2), 0))
    layer.alpha_composite(elixir, elixir_box)


def compose_hud(
    base: Image.Image,
    offsets: dict[str, tuple[int, int]],
    hand: list[str],
    next_card: str | None,
    elixir: float,
    cache: dict[str, Image.Image],
) -> Image.Image:
    frame = base.copy()
    paint_elixir(frame, offsets["elixir"], elixir / 10)
    slots = Image.open(PARTS / "slots.png").convert("RGBA")
    slot_layer = Image.new("RGBA", slots.size, (0, 0, 0, 0))
    for span, card_id in zip(SLOT_SPANS, hand):
        rounded_paste(slot_layer, card_image(card_id, cache), (span[0] + 7, 8, span[1] - 7, slots.height - 8))
    frame.alpha_composite(slot_layer, offsets["slots"])
    if next_card:
        art = card_image(next_card, cache)
        small = art.resize((52, 64), Image.Resampling.LANCZOS)
        mask = Image.new("L", small.size, 0)
        ImageDraw.Draw(mask).rounded_rectangle((0, 0, 51, 63), radius=8, fill=255)
        small.putalpha(mask)
        sx, sy = offsets["slots"]
        frame.alpha_composite(small, (sx - 64, sy + 26))
    return frame


def arena_frame(width: int, height: int) -> Image.Image:
    image = Image.new("RGBA", (width, height), (46, 122, 62, 255))
    draw = ImageDraw.Draw(image)
    for y in range(0, height, 18):
        shade = 38 + (y // 18 % 2) * 10
        draw.rectangle((0, y, width, y + 9), fill=(shade, 118 + (y // 40 % 3) * 4, 54, 255))
    river_top = int(height * 0.47)
    river_bottom = int(height * 0.53)
    draw.rectangle((0, river_top, width, river_bottom), fill=(64, 138, 176, 255))
    draw.rectangle((0, river_top, width, river_top + 3), fill=(210, 196, 140, 255))
    draw.rectangle((0, river_bottom - 3, width, river_bottom), fill=(210, 196, 140, 255))
    for bridge_x in (0.25, 0.75):
        left = int(width * bridge_x) - 28
        draw.rounded_rectangle(
            (left, river_top - 6, left + 56, river_bottom + 6),
            radius=6,
            fill=(168, 132, 86, 255),
        )
    return image


def blit_unit(canvas: Image.Image, art: Image.Image, center: tuple[int, int], radius: int, team: str) -> None:
    diameter = radius * 2
    fitted = art.resize((diameter, diameter), Image.Resampling.LANCZOS)
    mask = Image.new("L", (diameter, diameter), 0)
    ImageDraw.Draw(mask).ellipse((0, 0, diameter - 1, diameter - 1), fill=255)
    fitted.putalpha(mask)
    canvas.alpha_composite(fitted, (center[0] - radius, center[1] - radius))
    color = (86, 164, 232, 255) if team == "Blue" else (214, 92, 86, 255)
    draw = ImageDraw.Draw(canvas)
    draw.ellipse(
        (center[0] - radius, center[1] - radius, center[0] + radius, center[1] + radius),
        outline=color,
        width=3,
    )


def draw_tower(draw: ImageDraw.ImageDraw, center: tuple[int, int], kind: str, team: str, hp: int, max_hp: int) -> None:
    radius = 34 if kind == "KingTower" else 26
    body = (186, 176, 160, 255) if hp > 0 else (90, 86, 80, 255)
    outline = (86, 164, 232, 255) if team == "Blue" else (214, 92, 86, 255)
    draw.rounded_rectangle(
        (center[0] - radius, center[1] - radius, center[0] + radius, center[1] + radius - 8),
        radius=6,
        fill=body,
        outline=outline,
        width=3,
    )
    if max_hp > 0 and hp > 0:
        span = radius * 2
        left = center[0] - radius
        draw.rectangle((left, center[1] - radius - 8, left + span, center[1] - radius - 3), fill=(30, 24, 20, 255))
        draw.rectangle(
            (left, center[1] - radius - 8, left + int(span * hp / max_hp), center[1] - radius - 3),
            fill=(92, 196, 96, 255),
        )


def render_match_frame(
    hud_blue: Image.Image,
    hud_red: Image.Image,
    observation: dict,
    cache: dict[str, Image.Image],
    names: tuple[str, str],
) -> Image.Image:
    width, height = 720, 1280
    image = Image.new("RGBA", (width, height), (18, 28, 22, 255))
    blue = hud_blue.resize((width, int(hud_blue.height * width / hud_blue.width)), Image.Resampling.LANCZOS)
    red = hud_red.resize(blue.size, Image.Resampling.LANCZOS).transpose(Image.Transpose.ROTATE_180)
    top_h = red.height
    bottom = height - blue.height
    arena_h = bottom - top_h
    field = arena_frame(width - 36, arena_h - 16)
    image.alpha_composite(field, (18, top_h + 8))
    image.alpha_composite(red, (0, 0))
    image.alpha_composite(blue, (0, bottom))

    field_box = (18, top_h + 8, 18 + field.width, top_h + 8 + field.height)

    def project(x: int, y: int) -> tuple[int, int]:
        px = field_box[0] + int((x / 18000) * (field_box[2] - field_box[0]))
        py = field_box[1] + int((y / 32000) * (field_box[3] - field_box[1]))
        return px, py

    draw = ImageDraw.Draw(image)
    entities = observation["entities"]
    for entity in entities:
        if entity["kind"] not in ("PrincessTower", "KingTower"):
            continue
        draw_tower(
            draw,
            project(entity["x"], entity["y"]),
            entity["kind"],
            entity["team"],
            entity["hp"],
            entity["max_hp"],
        )
    for entity in entities:
        if entity["kind"] in ("PrincessTower", "KingTower") or entity["hp"] <= 0:
            continue
        card_id = entity.get("card_id") or "knight"
        radius = 30 if entity["kind"] == "Building" else 22
        blit_unit(image, card_image(card_id, cache), project(entity["x"], entity["y"]), radius, entity["team"])
        if entity["max_hp"] > 0:
            cx, cy = project(entity["x"], entity["y"])
            left = cx - radius
            draw.rectangle((left, cy - radius - 7, left + radius * 2, cy - radius - 3), fill=(20, 16, 14, 255))
            draw.rectangle(
                (left, cy - radius - 7, left + int(radius * 2 * entity["hp"] / entity["max_hp"]), cy - radius - 3),
                fill=(92, 196, 96, 255),
            )

    remaining = int(observation["remaining_ms"])
    clock = f"{remaining // 60000}:{(remaining // 1000) % 60:02d}"
    if observation.get("overtime"):
        clock = "OT " + clock
    label = font(28)
    small = font(18)
    draw.text((width / 2, top_h - 8), clock, fill=(255, 244, 214, 255), font=label, anchor="ms")
    crowns = observation["crowns"]
    draw.text((24, top_h + 14), f"{names[1]}   {crowns[1]}", fill=(255, 214, 206, 255), font=small)
    draw.text((24, bottom - 28), f"{names[0]}   {crowns[0]}", fill=(206, 230, 255, 255), font=small)
    return image.convert("RGB")


def hand_ids(player: dict) -> list[str]:
    return [card["id"] for card in player["hand"]]


def main() -> None:
    if not HEADLESS.is_file():
        raise SystemExit(f"missing {HEADLESS}; build it with cargo build --release --bin headless")
    for name in ("panel", "slots", "elixir"):
        if not (PARTS / f"{name}.png").is_file():
            raise SystemExit(f"missing recovered HUD piece {PARTS / name}.png")
    OUT.mkdir(parents=True, exist_ok=True)
    frames = OUT / "frames"
    frames.mkdir(exist_ok=True)
    base, offsets = load_parts()
    cache: dict[str, Image.Image] = {}
    engine = Engine()
    blue = Policy("v4.4.1", OUT / "blue-policy.log")
    red = Policy("v4.4.2", OUT / "red-policy.log")
    try:
        engine.call({"op": "reset", "seed": 42, "decks": [HOG, LOGBAIT]})
        history: list[dict] = []
        pending: dict[str, tuple[int, dict]] = {}
        next_query = {"blue": 0, "red": 0}
        decks = {"blue": HOG, "red": LOGBAIT}
        models = {"blue": blue, "red": red}
        frame_index = 0
        last = None
        max_ticks = int(os.environ.get("CLASH_RECORD_MAX_TICKS", "0"))
        while True:
            state = engine.call({"op": "observe"})
            observation = state["observation"]
            elapsed = int(observation["elapsed_ms"])
            actions = []
            for side, team_name in (("blue", "Blue"), ("red", "Red")):
                scheduled = pending.get(side)
                if scheduled and elapsed >= scheduled[0]:
                    action = scheduled[1]
                    actions.append(
                        {
                            "side": side,
                            "payload": {
                                "team": team_name,
                                "hand_index": action["hand_index"],
                                "x": action["x"],
                                "y": action["y"],
                            },
                            "record": {
                                "team": side,
                                "elapsed_ms": elapsed,
                                "card": action["card"],
                                "x": action["x"],
                                "y": action["y"],
                            },
                        }
                    )
                    pending.pop(side, None)
                    next_query[side] = elapsed
                    continue
                if side not in pending and elapsed >= next_query[side] and observation["outcome"] is None:
                    player = observation["players"][0 if side == "blue" else 1]
                    reply = models[side].ask(
                        {
                            "model": models[side].model,
                            "match_id": "native-ui-record",
                            "team": side,
                            "elapsed_ms": elapsed,
                            "elixir": player["elixir_milli"] / 1000,
                            "deck": decks[side],
                            "opponent_deck": decks["red" if side == "blue" else "blue"],
                            "hand": hand_ids(player),
                            "history": history,
                        }
                    )
                    if reply.get("error"):
                        raise RuntimeError(f"{side} policy error: {reply['error']}")
                    if reply.get("wait"):
                        next_query[side] = elapsed + int(reply.get("requery_after_ms", 500))
                    else:
                        legal = next(card for card in player["hand"] if card["id"] == reply["card"])
                        x = min(legal["x_max"], max(legal["x_min"], int(reply["x"])))
                        y = min(legal["y_max"], max(legal["y_min"], int(reply["y"])))
                        delay = int(reply.get("delay_ms", 0))
                        pending[side] = (elapsed + delay, {**reply, "x": x, "y": y})
                        next_query[side] = elapsed + max(delay, int(reply.get("requery_after_ms", 500)))
            if observation["tick"] % 8 == 0 or observation["outcome"] is not None:
                blue_player = observation["players"][0]
                red_player = observation["players"][1]
                hud_blue = compose_hud(
                    base,
                    offsets,
                    hand_ids(blue_player),
                    blue_player.get("next_card"),
                    blue_player["elixir_milli"] / 1000,
                    cache,
                )
                hud_red = compose_hud(
                    base,
                    offsets,
                    hand_ids(red_player),
                    red_player.get("next_card"),
                    red_player["elixir_milli"] / 1000,
                    cache,
                )
                frame = render_match_frame(hud_blue, hud_red, observation, cache, ("v4.4.1", "v4.4.2"))
                frame.save(frames / f"frame_{frame_index:04d}.png")
                frame_index += 1
                last = observation
                if frame_index == 1 or frame_index % 25 == 0 or observation["outcome"] is not None:
                    print(f"frame {frame_index} tick {observation['tick']} elapsed {elapsed}", flush=True)
            if observation["outcome"] is not None:
                break
            if max_ticks and observation["tick"] >= max_ticks:
                break
            result = engine.call(
                {"op": "step", "ticks": 1, "actions": [item["payload"] for item in actions]}
            )
            for item, outcome in zip(actions, result["action_results"]):
                if outcome.get("ok"):
                    history.append(item["record"])
        video = OUT / "ai-vs-ai-native-hud.mp4"
        subprocess.run(
            [
                "ffmpeg",
                "-y",
                "-framerate",
                "12",
                "-i",
                str(frames / "frame_%04d.png"),
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-crf",
                "18",
                str(video),
            ],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        summary = {
            "video": str(video),
            "frames": frame_index,
            "seed": 42,
            "blue": {"model": "v4.4.1", "deck": "hog_2_6"},
            "red": {"model": "v4.4.2", "deck": "logbait"},
            "outcome": last["outcome"] if last else None,
            "elapsed_ms": last["elapsed_ms"] if last else None,
            "hud_sources": {
                "panel_object": 1717,
                "slots_object": 1729,
                "elixir_object": 1731,
                "document": "analysis/native-ui/display-trees/ui.json",
            },
            "note": "Original HUD artwork. Local simulation advances the troops; the original battle loop is not executing.",
        }
        (OUT / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(json.dumps(summary, indent=2))
    finally:
        engine.close()
        blue.close()
        red.close()


if __name__ == "__main__":
    main()
