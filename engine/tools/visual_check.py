#!/usr/bin/env python3
"""Deterministic visual QA for the Arena renderer.

Recorded original-engine matches are replayed through the real presenter
(`clash-visual-check`: same Painter, same display lists the browser receives),
on an exact display-time grid. What was drawn is then judged three ways:

1. Rules derived from original game data (logic rows, authored clips and
   effects) and from continuity: no pops, flicker or one-frame art swaps.
2. Comparison with an accepted baseline run: findings that are new or fixed,
   and every display-list change attributed to the rows whose drawing changed.
   ``--expect ROWS`` declares what a change is meant to affect; any other
   change fails the run.
3. Screenshots: per-entity filmstrips, finding thumbnails and a frame viewer
   in report.html, for humans and multimodal agents.

Exit status: 0 PASS, 1 FAIL, 2 harness error. See
engine/native-spectator/VISUAL_QA.md for the agent workflow.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import datetime as dt
import gzip
import hashlib
import json
import math
import multiprocessing
import os
import re
import shutil
import statistics
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ENGINE = ROOT / "engine"
SPECTATOR = ENGINE / "native-spectator"
FIXTURES = SPECTATOR / "visual-fixtures"
BINARY = SPECTATOR / "target/debug/clash-visual-check"
STATE = ROOT / ".visual-check"
DEFAULT_ASSETS = Path(os.environ.get("ARENA_ASSETS_DIR") or ENGINE / "analysis/native-spectator")
TICK_MS = 50.0

# Thresholds. Each is a presentation judgement, stated once here.
AREA_JUMP_ERROR = 4.0      # visible mass ratio between consecutive frames
AREA_JUMP_WARN = 2.5
AREA_MIN_MASS = 40.0       # ignore tiny sprites (pixels of full alpha)
POP_MAX_FRAMES = 8         # a gap longer than this is a real absence
FLICKER_FRAMES = 3         # A -> B -> A within this many frames
FROZEN_WALK_MS = 300.0
EFFECT_WINDOW_MS = 200.0   # death/spawn effect must start this close in time
EFFECT_RADIUS = 2500.0     # ... and this close in native units
ATTACK_SYNC_MS = 120.0     # authored action_frame vs observed hit
INVISIBLE_MAX = 0.6        # invisible: visible mass at most this x normal
VISIBLE_MIN = 0.8          # attacking/revealed: at least this x normal

RULES = {
    "unresolved-art": ("error", "A live engine row has no original artwork binding, so it is drawn as a fallback dot (or not drawn at all).",
                       "Bindings in assets.json; Painter.unresolved_rows."),
    "missing-logic": ("error", "A drawn row has artwork but no original logic row. Without it the Painter cannot show attack clips, deploy filters, spawn/death effects or mechanics such as invisibility.",
                      "Rows of presentation.json (original logic export)."),
    "foreign-art": ("error", "The chosen export belongs to another character's export family, or its name is not an authored action of this row. Example: Royal Ghost walking with its GhostOverlay outline instead of its body.",
                    "Original export names and BlueExportName/RedExportName of every logic row."),
    "state-art-mismatch": ("warning", "The presentation state and the authored action of the chosen clip disagree (e.g. walking with an attack clip).",
                           "action_token of animation_exports in assets.json."),
    "visible-area-jump": ("error", f"An entity's visible pixel mass changes by more than x{AREA_JUMP_WARN} between two consecutive display frames without a deploy or damage filter: its art was swapped for something different (x{AREA_JUMP_ERROR}+ is an error).",
                          "Display list: alpha of each texture times draw opacity, grouped by entity."),
    "pop": ("error", f"An entity alive in the engine is drawn, missing for 1-{POP_MAX_FRAMES} frames, then drawn again.",
            "Engine snapshots vs display list."),
    "flicker": ("warning", f"The entity's clip alternates A -> B -> A within {FLICKER_FRAMES} frames.",
                "Painter clip selection (probe)."),
    "frozen-walk": ("warning", f"Walking while the authored frame index does not advance for {FROZEN_WALK_MS:.0f} ms.",
                    "Painter clip selection (probe)."),
    "anim-restart": ("warning", "An idle or walk clip shows a frame far from where elapsed time puts it (a visible restart or skip); loop wraps are fine.",
                     "Authored frame count and fps of animation_exports."),
    "attack-jump": ("warning", "An attack is entered past half of its wind-up: the swing appears already in progress instead of starting.",
                    "action_frame label of the attack clip; logic HitSpeed."),
    "attack-sync": ("warning", f"The authored action_frame of the attack clip is more than {ATTACK_SYNC_MS:.0f} ms from the hit the engine produced.",
                    "action_frame label; hits attributed by the Painter (probe impacts)."),
    "hit-without-attack-clip": ("error", "The unit hit something (projectile launched or melee damage) but was not shown attacking around that moment.",
                                "Hits attributed by the Painter (probe impacts)."),
    "vanish-without-effect": ("error", "A unit removed by the engine disappeared without the DeathEffect its logic names.",
                              "Logic DeathEffect; resolved effects (probe)."),
    "spawn-without-effect": ("warning", "A deployed unit's logic SpawnEffect was not played.",
                             "Logic SpawnEffect; resolved effects (probe)."),
    "deploy-filter-missing": ("warning", "A deployed unit's logic DeployBaseAnimExportName filter was never applied.",
                              "Logic DeployBaseAnimExportName."),
    "invisibility-not-shown": ("error", f"Logic BuffWhenNotAttacking=Invisibility: after BuffWhenNotAttackingTime ms without attacking (fade HideTimeMs) the unit must look faded (visible mass <= {INVISIBLE_MAX} x normal) and must reappear (>= {VISIBLE_MIN} x) to attack.",
                               "Logic BuffWhenNotAttacking, BuffWhenNotAttackingTime, HideTimeMs, LoadTime, HitSpeed, DeployTime."),
    "drawn-after-removal": ("error", "An entity is drawn more than one tick after the engine removed it.",
                            "Engine snapshots vs display list."),
}
SEVERITY_ORDER = {"error": 0, "warning": 1, "info": 2}


# --------------------------------------------------------------------------
# Inputs
# --------------------------------------------------------------------------

class Assets:
    """Original logic rows, clip metadata and export families of a bundle."""

    def __init__(self, root: Path):
        self.root = root
        manifest = json.loads((root / "assets.json").read_text())
        presentation = json.loads((root / "presentation.json").read_text())
        self.logic = {name: row.get("fields", {}) for name, row in presentation.get("rows", {}).items()}
        self.clips = {}
        for doc in manifest.get("documents", []):
            for e in doc.get("animation_exports") or []:
                labels = {l["name"]: l["frame"] for l in e.get("labels") or []}
                self.clips[e["export"]] = {"token": e.get("action_token"), "fps": e.get("fps") or 30,
                                           "frames": e.get("frames") or 1, "labels": labels}
        self.actions = {c["token"] for c in self.clips.values() if c["token"]}
        families = defaultdict(set)  # row -> export prefixes

        def bind(row, *prefixes):
            for p in prefixes:
                if p:
                    families[row].add(p)
                    if row.endswith("s"):
                        families[row[:-1]].add(p)
        scene = manifest.get("scene", {})
        cards = (scene.get("observed_decks", {}).get("cards_observed_across_traces") or []) + \
            (scene.get("fixed_training_deck_assets") or [])
        for card in cards:
            row = card.get("native_card_row")
            if not row:
                continue
            b = card.get("character_logic_binding") or {}
            prefixes = card.get("export_prefixes") or []
            bind(row, b.get("blue_export") or card.get("blue_export") or (prefixes[0] if prefixes else None),
                 b.get("red_export") or card.get("red_export") or (prefixes[1] if len(prefixes) > 1 else None))
        for b in scene.get("observed_entity_bindings") or []:
            bind(b.get("native_row"), b.get("blue_export"), b.get("red_export"))
        for row, f in self.logic.items():
            bind(row, f.get("BlueExportName"), f.get("RedExportName"))
        self.families = families
        self.prefix_rows = defaultdict(set)
        for row, prefixes in families.items():
            for p in prefixes:
                self.prefix_rows[p].add(row)

    def has_attack_clip(self, row):
        prefixes = self.families.get(row, ())
        return any(c["token"] == "attack" and any(e.startswith(p + "_") for p in prefixes)
                   for e, c in self.clips.items())

    def field(self, row, key, default=None):
        return self.logic.get(row, {}).get(key, default)

    def foreign(self, row, export):
        """Why `export` is not this row's own authored art, or None."""
        if not export:
            return None

        def matches(p):
            return export == p or export.startswith(p + "_")
        own = [p for p in self.families.get(row, ()) if matches(p)]
        best = max((p for p in self.prefix_rows if matches(p)), key=len, default=None)
        if best and best not in self.families.get(row, ()):
            owners = sorted(self.prefix_rows[best] - {row})
            return f"`{export}` belongs to the `{best}` family of {', '.join(owners)}"
        if own:
            prefix = max(own, key=len)
            tail = export[len(prefix) + 1:]
            word = re.match(r"[A-Za-z]+", tail)
            if word and self.actions and word.group(0).lower() not in self.actions:
                return (f"`{export}` adds `{word.group(0)}` after `{prefix}`; authored actions are "
                        f"{', '.join(sorted(self.actions))}")
        return None


def load_fixture_frames(path: Path):
    """First match of a gzipped every-tick capture, as parsed frames."""
    frames = []
    with gzip.open(path, "rt") as handle:
        for line in handle:
            if line.strip():
                frame = json.loads(line)
                if frames and frame.get("match_id") != frames[0].get("match_id"):
                    break
                frames.append(frame)
    return frames


def engine_lives(frames):
    """Engine truth per native object: rows, lifetime, positions and health."""
    lives = {}
    last_tick = frames[-1]["tick"] if frames else 0
    for frame in frames:
        tick = frame["tick"]
        for e in frame["entities"]:
            key = (e["address"], e.get("native_entity_id"))
            life = lives.get(key)
            if life is None or life["row"] != e["row"]:
                life = lives[key] = {"row": e["row"], "side": e["side"], "first": tick, "last": tick,
                                     "max_hp": e["max_hp"], "pos": {}}
            life["last"] = tick
            life["pos"][tick] = (e["x"], e["y"], e["z"], e["hp"])
    for life in lives.values():
        life["removed"] = life["last"] + 1 if life["last"] < last_tick else None
    return lives


def scenarios_catalog():
    return json.loads((FIXTURES / "scenarios.json").read_text())


# --------------------------------------------------------------------------
# Replay
# --------------------------------------------------------------------------

def build_binary():
    result = subprocess.run(["cargo", "build", "--offline", "--manifest-path", str(SPECTATOR / "Cargo.toml"),
                             "--bin", "clash-visual-check"], capture_output=True, text=True)
    if result.returncode:
        sys.stderr.write(result.stderr[-4000:])
        raise RuntimeError("cargo build failed")
    build_stamp().write_text(json.dumps({"source": source_fingerprint(), "binary": file_hash(BINARY)}))


def build_stamp():
    return BINARY.with_suffix(".visual-qa.json")


def file_hash(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_paths():
    paths = []
    for package in (SPECTATOR, ENGINE / "native-ui-viewer"):
        paths.extend(package.glob("src/**/*.rs"))
        paths.extend(p for p in (package / "Cargo.toml", package / "Cargo.lock", package / "build.rs") if p.exists())
    return sorted(paths)


def binary_is_stale():
    # Content, not mtimes: git restores and rebuilding a temporary regression
    # can otherwise make a binary appear fresh while containing different code.
    if not BINARY.exists() or not build_stamp().exists():
        return True
    try:
        stamp = json.loads(build_stamp().read_text())
        return stamp != {"source": source_fingerprint(), "binary": file_hash(BINARY)}
    except (OSError, ValueError):
        return True


def replay(scenario, assets_dir: Path, out: Path, images: bool):
    """Run the presenter over a fixture; returns its summary."""
    fixture = FIXTURES / "captures" / f"{scenario['capture']}.jsonl.gz"
    frames_path = out / "frames.jsonl"
    with gzip.open(fixture, "rb") as src, frames_path.open("wb") as dst:
        shutil.copyfileobj(src, dst)
    render = out / "render"
    command = [str(BINARY), "--frames", str(frames_path), "--out", str(render), "--assets", str(assets_dir),
               "--fps", str(scenario.get("fps", 20)), "--lag", str(scenario.get("lag", 0))]
    if "from_tick" in scenario:
        command += ["--from-tick", str(scenario["from_tick"])]
    if "to_tick" in scenario:
        command += ["--to-tick", str(scenario["to_tick"])]
    if images:
        command.append("--images")
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{scenario['name']}: clash-visual-check failed: {result.stderr.strip()[-2000:]}")
    return json.loads((render / "summary.json").read_text())


# --------------------------------------------------------------------------
# Analysis
# --------------------------------------------------------------------------

ENTITY_PART = re.compile(r"^entity-(\d+)-part-")
ENTITY_ANY = re.compile(r"^entity-(\d+)-")


class TextureMass:
    """Sum of alpha/255 per content-addressed texture (shared across runs)."""

    def __init__(self):
        self.cache = {}

    def __call__(self, tex, directory: Path):
        if tex not in self.cache:
            from PIL import Image
            import numpy as np
            with Image.open(directory / f"{tex}.png") as image:
                alpha = np.asarray(image.convert("RGBA"))[:, :, 3]
            self.cache[tex] = float(alpha.sum()) / 255.0
        return self.cache[tex]


TEXTURE_MASS = TextureMass()


def load_trace(path: Path):
    opener = gzip.open if path.suffix == ".gz" else open
    with opener(path, "rt") as handle:
        return [json.loads(line) for line in handle if line.strip()]


def entity_masses(record, textures: Path):
    masses = defaultdict(float)
    for draw in record["draws"]:
        if draw.get("kind") != "sprite":
            continue
        m = ENTITY_PART.match(draw.get("id", ""))
        if m:
            masses[int(m.group(1))] += draw.get("alpha", 1.0) * TEXTURE_MASS(draw["tex"], textures)
    return masses


def draw_groups(record):
    """Canonical drawing per entity uid, plus 'scene' for everything else."""
    groups = defaultdict(list)
    for draw in record["draws"]:
        items = draw.get("draws") if draw.get("kind") == "shadow" else [draw]
        if draw.get("kind") == "shadow":
            groups["scene"].append(json.dumps({k: v for k, v in draw.items() if k != "draws"}, sort_keys=True))
        for item in items:
            m = ENTITY_ANY.match(item.get("id", ""))
            key = int(m.group(1)) if m else "scene"
            canon = {k: v for k, v in item.items() if k != "draws"}
            groups[key].append(json.dumps(canon, sort_keys=True))
    return {k: hashlib.sha1("\n".join(v).encode()).hexdigest()[:16] for k, v in groups.items()}


class Findings:
    def __init__(self, scenario):
        self.scenario = scenario
        self.items = {}

    def add(self, rule, row, side, uid, t, message, frame=None, severity=None, evidence=None):
        key = f"{self.scenario}/{rule}/{row}#{uid if uid is not None else '*'}"
        item = self.items.get(key)
        if item is None:
            item = self.items[key] = {
                "key": key, "scenario": self.scenario, "rule": rule, "severity": severity or RULES[rule][0],
                "row": row, "side": side, "uid": uid, "message": message, "first_t": t, "frame": frame,
                "count": 0, "occurrences": [], "evidence": evidence or {}}
        item["count"] += 1
        if len(item["occurrences"]) < 12:
            item["occurrences"].append({"t": t, "frame": frame})
        if severity and SEVERITY_ORDER[severity] < SEVERITY_ORDER[item["severity"]]:
            item["severity"] = severity

    def sorted(self):
        return sorted(self.items.values(), key=lambda f: (SEVERITY_ORDER[f["severity"]], f["first_t"], f["key"]))


def analyze(scenario, assets: Assets, trace, lives, textures: Path):
    """All rule findings for one scenario plus per-entity timelines."""
    findings = Findings(scenario["name"])
    window = (trace[0]["t"], trace[-1]["t"]) if trace else (0, 0)
    by_uid = defaultdict(list)   # uid -> [(i, t, shown, mass)]
    markers = defaultdict(list)
    impacts = defaultdict(list)
    effects = []
    for record in trace:
        masses = entity_masses(record, textures)
        for s in record["shown"]:
            if s["kind"] in ("marker", "missing"):
                markers[s["row"]].append((record["i"], record["t"], s))
                continue
            by_uid[s["uid"]].append((record["i"], record["t"], s, masses.get(s["uid"], 0.0)))
        for hit in record["impacts"]:
            if hit["at_ms"] not in impacts[hit["uid"]]:
                impacts[hit["uid"]].append(hit["at_ms"])
        effects.extend(record["effects"])

    for row, seen in markers.items():
        i, t, s = seen[0]
        how = "drawn as a fallback dot" if s["kind"] == "marker" else "not drawn at all"
        findings.add("unresolved-art", row, s["side"], None, t,
                     f"{row} has no original artwork binding and is {how} ({len(seen)} entity-frames).", i)

    drawn_rows = {seen[0][2]["row"]: seen[0] for seen in by_uid.values()}
    for row, (i, t, s, _) in sorted(drawn_rows.items()):
        if row not in assets.logic and s["kind"] in ("unit", "projectile"):
            findings.add("missing-logic", row, s["side"], None, t,
                          f"{row} has art but no logic row in this bundle: no attack clip, deploy filter, "
                          "spawn/death effect or mechanic can be shown.", i)

    meta = {}
    for uid, seen in by_uid.items():
        first = seen[0][2]
        row, side, kind = first["row"], first["side"], first["kind"]
        life = lives.get((first["address"], first.get("native_id")))
        meta[uid] = {"row": row, "side": side, "kind": kind,
                     "spawn": life["first"] if life else None, "removed": life["removed"] if life else None}
        logic = assets.logic.get(row, {})
        prev = None
        walk_frame = None
        recent = []
        for i, t, s, mass in seen:
            # Art identity and state.
            if kind == "unit" and s["export"]:
                why = assets.foreign(row, s["export"])
                if why:
                    findings.add("foreign-art", row, side, uid, t, f"{row} #{uid} ({s['state']}): {why}.", i,
                                 evidence={"export": s["export"], "state": s["state"]})
                clip = assets.clips.get(s["export"])
                expected = {"moving": "run", "attacking": "attack", "idle": "idle"}.get(s["state"])
                if clip and clip["token"] and expected and clip["token"] not in (expected, "deploy"):
                    findings.add("state-art-mismatch", row, side, uid, t,
                                 f"{row} #{uid} is {s['state']} but shows `{s['export']}` "
                                 f"(authored action `{clip['token']}`).", i)
            # Continuity with the previous display frame of this entity.
            if prev is not None:
                pi, pt, ps, pmass = prev
                calm = not (s["deploying"] or ps["deploying"] or s["flash"] or ps["flash"])
                if kind == "unit" and calm and min(mass, pmass) >= AREA_MIN_MASS and i == pi + 1:
                    ratio = max(mass, pmass) / max(min(mass, pmass), 1e-9)
                    if ratio >= AREA_JUMP_WARN:
                        severity = "error" if ratio >= AREA_JUMP_ERROR else "warning"
                        findings.add("visible-area-jump", row, side, uid, t,
                                     f"{row} #{uid}: visible mass {pmass:.0f} -> {mass:.0f} px (x{ratio:.1f}) "
                                     f"from `{ps['export']}` ({ps['state']}) to `{s['export']}` ({s['state']}).",
                                     i, severity=severity,
                                     evidence={"from": ps["export"], "to": s["export"], "ratio": round(ratio, 2)})
                gap = i - pi - 1
                if 0 < gap <= POP_MAX_FRAMES and life:
                    ticks = [int(math.floor(trace[k]["t"] + 1e-9)) for k in range(pi + 1, i)]
                    if all(tk in life["pos"] for tk in ticks):
                        findings.add("pop", row, side, uid, trace[pi + 1]["t"],
                                     f"{row} #{uid} vanished for {gap} frame(s) while alive in the engine.", pi + 1)
                clip = assets.clips.get(s["export"])
                if s["export"] == ps["export"] and i == pi + 1 and s["state"] in ("idle", "moving") and clip \
                        and clip["frames"] >= 8:
                    # Idle and walk clips advance with time at their authored fps.
                    advance = clip["fps"] * (t - pt) * TICK_MS / 1000
                    expected = (ps["frame"] + advance) % clip["frames"]
                    off = abs(s["frame"] - expected)
                    off = min(off, clip["frames"] - off)
                    if off > max(3, 1.5 * advance):
                        findings.add("anim-restart", row, side, uid, t,
                                     f"{row} #{uid}: `{s['export']}` went from frame {ps['frame']} to {s['frame']} "
                                     f"of {clip['frames']} (time predicts ~{expected:.0f}).", i)
            # Flicker A -> B -> A.
            recent.append((i, s["export"]))
            recent = recent[-(FLICKER_FRAMES + 1):]
            odd = [e for _, e in recent[1:-1] if e != recent[0][1]]
            if len(recent) >= 3 and recent[-1][1] == recent[0][1] and odd and \
                    recent[-1][0] - recent[0][0] <= FLICKER_FRAMES:
                findings.add("flicker", row, side, uid, t,
                             f"{row} #{uid}: `{recent[0][1]}` -> `{odd[0]}` -> `{recent[-1][1]}` "
                             f"within {recent[-1][0] - recent[0][0]} frames.", i)
                recent = recent[-1:]
            # Walking animation that does not advance.
            if s["state"] == "moving" and kind == "unit":
                if walk_frame and walk_frame[1] == (s["export"], s["frame"]):
                    if (t - walk_frame[0]) * TICK_MS >= FROZEN_WALK_MS:
                        findings.add("frozen-walk", row, side, uid, t,
                                     f"{row} #{uid}: walking on frame {s['frame']} of `{s['export']}` for "
                                     f"{(t - walk_frame[0]) * TICK_MS:.0f} ms.", i)
                        walk_frame = (t, walk_frame[1])
                elif not walk_frame or walk_frame[1] != (s["export"], s["frame"]):
                    walk_frame = (t, (s["export"], s["frame"]))
            else:
                walk_frame = None
            # Entering an attack mid-swing.
            if s["state"] == "attacking" and (prev is None or prev[2]["state"] != "attacking"):
                clip = assets.clips.get(s["export"])
                hit = logic.get("HitSpeed")
                if clip and "action_frame" in clip["labels"] and hit:
                    speed = max(clip["frames"] / clip["fps"] * 1000 / hit, 1)
                    action_ms = clip["labels"]["action_frame"] / clip["fps"] * 1000 / speed
                    entered_ms = s["frame"] / clip["fps"] * 1000 / speed
                    if entered_ms > max(0.5 * action_ms, 150):
                        findings.add("attack-jump", row, side, uid, t,
                                     f"{row} #{uid}: attack shown from {entered_ms:.0f} ms into a wind-up of "
                                     f"{action_ms:.0f} ms.", i)
            # Removed by the engine yet still drawn.
            if life and life["removed"] is not None and math.floor(t + 1e-9) > life["removed"]:
                findings.add("drawn-after-removal", row, side, uid, t,
                             f"{row} #{uid} drawn at tick {t:.2f}; the engine removed it at {life['removed']}.", i)
            prev = (i, t, s, mass)

        # Hits and their attack clips (rows with authored attack art only).
        hit_speed = logic.get("HitSpeed") if assets.has_attack_clip(row) else None
        for at in sorted(impacts.get(uid, [])):
            if not (window[0] * TICK_MS <= at <= window[1] * TICK_MS) or not hit_speed:
                continue
            around = [(i, t, s) for i, t, s, _ in seen if at - hit_speed * 0.9 <= t * TICK_MS <= at + 150]
            if around and not any(s["state"] == "attacking" for _, _, s in around):
                findings.add("hit-without-attack-clip", row, side, uid, at / TICK_MS,
                             f"{row} #{uid} hit at {at / 1000:.2f} s but was shown "
                             f"{', '.join(sorted({s['state'] for _, _, s in around}))} around it.", around[-1][0])
            nearest = min(seen, key=lambda r: abs(r[1] * TICK_MS - at))
            i, t, s, _ = nearest
            clip = assets.clips.get(s["export"])
            if s["state"] == "attacking" and clip and "action_frame" in clip["labels"] and abs(t * TICK_MS - at) <= 50:
                speed = max(clip["frames"] / clip["fps"] * 1000 / hit_speed, 1)
                offset = (s["frame"] - clip["labels"]["action_frame"]) / clip["fps"] * 1000 / speed
                offset -= t * TICK_MS - at
                meta[uid].setdefault("sync", []).append(round(offset))
        sync = meta[uid].get("sync") or []
        if sync and statistics.median(abs(o) for o in sync) > ATTACK_SYNC_MS:
            findings.add("attack-sync", row, side, uid, seen[0][1],
                         f"{row} #{uid}: action_frame vs hit offsets {sync[:8]} ms "
                         f"(median |offset| {statistics.median(abs(o) for o in sync):.0f} ms).", seen[0][0],
                         evidence={"offsets_ms": sync})

        # Lifecycle effects.
        if life and kind == "unit" and life["max_hp"] > 0:
            if life["removed"] is not None and window[0] < life["removed"] <= window[1]:
                x, y, *_ = life["pos"][life["last"]]
                at = life["removed"] * TICK_MS
                near = [e for e in effects if abs(e["start_ms"] - at) <= EFFECT_WINDOW_MS
                        and math.hypot(e["x"] - x, e["y"] - y) <= EFFECT_RADIUS and e["depth"] == 0]
                death = logic.get("DeathEffect")
                # Rows without logic are reported once by missing-logic.
                if death and not any(e["name"] == death for e in near):
                    findings.add("vanish-without-effect", row, side, uid, life["removed"],
                                 f"{row} #{uid} removed at tick {life['removed']}; logic DeathEffect `{death}` "
                                 f"was not played (nearby: {sorted({e['name'] for e in near}) or 'none'}).", None)
            if window[0] < life["first"] <= window[1] and life["first"] > 1:
                x, y, *_ = life["pos"][life["first"]]
                at = life["first"] * TICK_MS
                spawn = logic.get("SpawnEffect")
                if spawn and not any(e["name"] == spawn and abs(e["start_ms"] - at) <= EFFECT_WINDOW_MS
                                     and math.hypot(e["x"] - x, e["y"] - y) <= EFFECT_RADIUS for e in effects):
                    findings.add("spawn-without-effect", row, side, uid, life["first"],
                                 f"{row} #{uid} deployed at tick {life['first']} without SpawnEffect `{spawn}`.", seen[0][0])
                deploy = logic.get("DeployBaseAnimExportName")
                early = [s for _, t, s, _ in seen if (t - life["first"]) * TICK_MS <= 600]
                if deploy and early and not any(s["deploying"] for s in early):
                    findings.add("deploy-filter-missing", row, side, uid, life["first"],
                                 f"{row} #{uid} deployed without its `{deploy}` filter.", seen[0][0])

        # Invisibility (Royal Ghost): faded after not attacking, visible to attack.
        if logic.get("BuffWhenNotAttacking") == "Invisibility" and life:
            check_invisibility(findings, row, side, uid, seen, sorted(impacts.get(uid, [])), logic, life)

    return findings, by_uid, meta, effects, impacts


def check_invisibility(findings, row, side, uid, seen, hits, logic, life):
    wait = logic.get("BuffWhenNotAttackingTime", 2000)
    hide = logic.get("HideTimeMs", 400)
    load = logic.get("LoadTime", 0)
    hit_speed = logic.get("HitSpeed", 1000)
    spawn_ms = life["first"] * TICK_MS + logic.get("DeployTime", 1000)
    active = [(spawn_ms - logic.get("DeployTime", 1000), spawn_ms)]
    active += [(h - load, h + max(hit_speed - load, 0)) for h in hits]
    active.sort()

    def phase(ms):
        """'visible', 'invisible' or None (transition margin)."""
        last_end = None
        for start, end in active:
            if start - 150 <= ms <= end + 150:
                return "visible"
            if end < ms:
                last_end = end
        if last_end is None:
            return None
        if ms >= last_end + wait + hide + 150:
            # Reveal starts when the next attack wind-up begins.
            nxt = [s for s, _ in active if s > ms]
            if not nxt or nxt[0] - ms > 150:
                return "invisible"
        return None
    visible = [m for _, t, s, m in seen if phase(t * TICK_MS) == "visible" and not s["deploying"]]
    reference = statistics.median(visible) if visible else max((m for *_, m in seen), default=0)
    if reference <= 0:
        return
    hidden = [(i, t, m) for i, t, s, m in seen if phase(t * TICK_MS) == "invisible"]
    shown = [(i, t, m) for i, t, s, m in seen if phase(t * TICK_MS) == "visible" and not s["deploying"]]
    not_faded = [(i, t, m) for i, t, m in hidden if m > INVISIBLE_MAX * reference]
    if hidden and len(not_faded) > 0.1 * len(hidden):
        i, t, m = not_faded[0]
        findings.add("invisibility-not-shown", row, side, uid, t,
                     f"{row} #{uid} should be invisible for {len(hidden)} frames (no attack for {wait} ms) but "
                     f"{len(not_faded)} are not faded (mass {m:.0f} vs normal {reference:.0f}).", i,
                     evidence={"invisible_frames": len(hidden), "not_faded": len(not_faded), "reference_mass": round(reference)})
    too_faint = [(i, t, m) for i, t, m in shown if m < VISIBLE_MIN * reference]
    if shown and len(too_faint) > 0.1 * len(shown):
        i, t, m = too_faint[0]
        findings.add("invisibility-not-shown", row, side, uid, t,
                     f"{row} #{uid} should be visible while deploying/attacking but {len(too_faint)} of "
                     f"{len(shown)} frames are faint (mass {m:.0f} vs normal {reference:.0f}).", i)


# --------------------------------------------------------------------------
# Baseline comparison
# --------------------------------------------------------------------------

def compare_traces(base_trace, trace, base_rows, rows):
    """Display-list differences per frame, attributed to rows."""
    changed_frames, row_frames, details = [], defaultdict(int), defaultdict(lambda: defaultdict(int))
    if [r["t"] for r in base_trace] != [r["t"] for r in trace]:
        raise RuntimeError("baseline display-time grid differs; run the same scenario and FPS before comparing")
    base_by_t = {r["t"]: r for r in base_trace}
    for record in trace:
        base = base_by_t[record["t"]]
        a, b = draw_groups(base), draw_groups(record)
        keys = [k for k in set(a) | set(b) if a.get(k) != b.get(k)]
        def ordered_ids(r):
            return [(d.get("kind"), d.get("id")) for d in r["draws"]]
        order_a, order_b = ordered_ids(base), ordered_ids(record)
        common = set(order_a) & set(order_b)
        if [k for k in order_a if k in common] != [k for k in order_b if k in common] and "scene" not in keys:
            keys.append("scene")
            details["effects/HUD"]["draw order changed"] += 1
        if not keys:
            continue
        changed_frames.append(record["i"])
        shown_a = {s["uid"]: s for s in base["shown"]}
        shown_b = {s["uid"]: s for s in record["shown"]}
        for k in keys:
            row = "effects/HUD" if k == "scene" else rows.get(k) or base_rows.get(k) or f"uid {k}"
            row_frames[row] += 1
            if k != "scene":
                sa, sb = shown_a.get(k), shown_b.get(k)
                if sa and sb:
                    for field in ("state", "export", "alpha", "deploying", "flash"):
                        if sa.get(field) != sb.get(field):
                            details[row][f"{field}: {sa.get(field)} -> {sb.get(field)}"] += 1
                    if sa.get("frame") != sb.get("frame") and sa.get("export") == sb.get("export"):
                        details[row]["authored frame index changed"] += 1
                elif sa or sb:
                    details[row]["drawn" if sb else "no longer drawn"] += 1
                else:
                    details[row]["pixels/transform changed"] += 1
    return {"frames": len(trace), "changed_frames": changed_frames,
            "rows": dict(sorted(row_frames.items(), key=lambda kv: -kv[1])),
            "details": {r: dict(sorted(d.items(), key=lambda kv: -kv[1])[:8]) for r, d in details.items()}}


# --------------------------------------------------------------------------
# Screenshots
# --------------------------------------------------------------------------

def label_font(size):
    from PIL import ImageFont
    for path in ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                 "/usr/share/fonts/TTF/DejaVuSans.ttf"):
        if Path(path).exists():
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def crop_box(s, size):
    x0, y0, x1, y1 = s["bbox"] or (s["anchor"][0] - 20, s["anchor"][1] - 40, s["anchor"][0] + 20, s["anchor"][1])
    cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
    w, h = size
    return (int(cx - w / 2), int(cy - h / 2), int(cx - w / 2) + w, int(cy - h / 2) + h)


def short_export(row_prefixes, export):
    for p in sorted(row_prefixes, key=len, reverse=True):
        if export.startswith(p + "_"):
            return export[len(p) + 1:]
    return export


def strip(frames_dir: Path, tiles, size, path: Path, columns=8, scale=1.6, mark=None):
    """Grid of entity crops: tiles = [(i, t, shown, mass, label_lines)]."""
    from PIL import Image, ImageDraw
    tw, th = int(size[0] * scale), int(size[1] * scale)
    rows = (len(tiles) + columns - 1) // columns
    sheet = Image.new("RGB", (columns * tw, rows * (th + 30)), (18, 22, 28))
    font = label_font(11)
    for n, (i, t, s, mass, lines) in enumerate(tiles):
        image_path = frames_dir / f"{i:05}.jpg"
        if not image_path.exists():
            continue
        with Image.open(image_path) as frame:
            tile = frame.crop(crop_box(s, size)).resize((tw, th))
        x, y = (n % columns) * tw, (n // columns) * (th + 30)
        sheet.paste(tile, (x, y))
        draw = ImageDraw.Draw(sheet)
        if mark is not None and i == mark:
            draw.rectangle((x, y, x + tw - 1, y + th - 1), outline=(255, 70, 70), width=3)
        for k, line in enumerate(lines[:2]):
            draw.text((x + 3, y + th + 2 + 13 * k), line, fill=(235, 235, 235) if k == 0 else (160, 200, 255), font=font)
    sheet.save(path)


def tile_lines(t, s, mass, reference, prefixes):
    pct = f"{100 * mass / reference:.0f}%" if reference else "-"
    return [f"{t / 20:.2f}s {s['state']} f{s['frame']}", f"{short_export(prefixes, s['export'])[:22]} {pct}"]


def screenshots(scen_dir: Path, scenario, assets: Assets, by_uid, findings):
    frames_dir = scen_dir / "frames"
    (scen_dir / "filmstrips").mkdir(exist_ok=True)
    (scen_dir / "findings").mkdir(exist_ok=True)
    focus = set(scenario.get("focus", []))
    strips = []
    for uid, seen in sorted(by_uid.items()):
        row, kind = seen[0][2]["row"], seen[0][2]["kind"]
        if row not in focus or kind != "unit":
            continue
        reference = statistics.median(m for *_, m in seen) or 1
        size = (min(max(max((s["bbox"][2] - s["bbox"][0]) for _, _, s, _ in seen if s["bbox"]) + 24, 60), 200),
                min(max(max((s["bbox"][3] - s["bbox"][1]) for _, _, s, _ in seen if s["bbox"]) + 24, 70), 220)) \
            if any(s["bbox"] for _, _, s, _ in seen) else (80, 90)
        changes = {k for k in range(1, len(seen)) if (seen[k][2]["state"], seen[k][2]["export"]) !=
                   (seen[k - 1][2]["state"], seen[k - 1][2]["export"])}
        stride = max(1, len(seen) // 40)
        picks = sorted({k for k in range(0, len(seen), stride)} | changes | {k - 1 for k in changes if k} | {len(seen) - 1})
        tiles = [(seen[k][0], seen[k][1], seen[k][2], seen[k][3],
                  tile_lines(seen[k][1], seen[k][2], seen[k][3], reference, assets.families.get(row, ())))
                 for k in picks]
        name = f"{row}-{uid}.png"
        strip(frames_dir, tiles, size, scen_dir / "filmstrips" / name)
        strips.append({"uid": uid, "row": row, "side": seen[0][2]["side"], "file": f"filmstrips/{name}",
                       "tiles": len(tiles)})
    for n, finding in enumerate(findings):
        uid, frame = finding["uid"], finding["frame"]
        if uid is None or frame is None or uid not in by_uid:
            continue
        seen = by_uid[uid]
        index = min(range(len(seen)), key=lambda k: abs(seen[k][0] - frame))
        chosen = seen[max(0, index - 3): index + 4]
        reference = statistics.median(m for *_, m in seen) or 1
        size = (120, 140)
        tiles = [(i, t, s, m, tile_lines(t, s, m, reference, assets.families.get(s["row"], ()))) for i, t, s, m in chosen]
        name = f"{n:03}-{finding['rule']}-{finding['row']}-{uid}.png"
        strip(frames_dir, tiles, size, scen_dir / "findings" / name, columns=len(tiles), mark=frame)
        finding["thumbnail"] = f"findings/{name}"
    return strips


def compress_frames(scen_dir: Path, keep_png: bool):
    from PIL import Image
    src = scen_dir / "render" / "frames"
    dst = scen_dir / "frames"
    dst.mkdir(exist_ok=True)
    for png in sorted(src.glob("*.png")):
        with Image.open(png) as image:
            image.save(dst / f"{png.stem}.jpg", quality=78)
    if keep_png:
        shutil.move(str(src), str(scen_dir / "png"))


# --------------------------------------------------------------------------
# Run
# --------------------------------------------------------------------------

def run_scenario(scenario, assets_dir, assets, run_dir, baseline, keep_png):
    name = scenario["name"]
    scen_dir = run_dir / name
    scen_dir.mkdir(parents=True)
    images = scenario.get("images", True)
    started = time.time()
    summary = replay(scenario, assets_dir, scen_dir, images)
    frames = load_fixture_frames(FIXTURES / "captures" / f"{scenario['capture']}.jsonl.gz")
    (scen_dir / "frames.jsonl").unlink()
    lives = engine_lives(frames)
    trace = load_trace(scen_dir / "render" / "trace.jsonl")
    textures = scen_dir / "render" / "textures"
    findings, by_uid, meta, effects, impacts = analyze(scenario, assets, trace, lives, textures)
    findings = findings.sorted()
    strips = []
    if images:
        compress_frames(scen_dir, keep_png)
        strips = screenshots(scen_dir, scenario, assets, by_uid, findings)
    rows = {uid: m["row"] for uid, m in meta.items()}
    comparison = None
    if baseline and (baseline / name / "trace.jsonl.gz").exists():
        base_trace = load_trace(baseline / name / "trace.jsonl.gz")
        base_rows = json.loads((baseline / name / "scenario.json").read_text()).get("rows", {})
        comparison = compare_traces(base_trace, trace, {int(k): v for k, v in base_rows.items()}, rows)
    # Compact per-frame view for the report's frame viewer.
    view = [{"i": r["i"], "t": r["t"], "e": [
        [s["uid"], s["row"], s["side"], s["state"], short_export(assets.families.get(s["row"], ()), s["export"]),
         s["frame"], round(s["alpha"], 2), [round(v) for v in s["bbox"]] if s["bbox"] else None]
        for s in r["shown"] if s["kind"] != "missing"]} for r in trace]
    timelines = {uid: {"row": m["row"], "side": m["side"], "kind": m["kind"],
                       "states": [[i, s["state"], s["export"], round(mass, 1), round(s["alpha"], 2)]
                                  for i, _, s, mass in by_uid[uid]],
                       "impacts": [round(a / TICK_MS, 2) for a in sorted(impacts.get(uid, []))],
                       "spawn": m["spawn"], "removed": m["removed"]}
                 for uid, m in meta.items() if m["kind"] in ("unit", "princess", "king")}
    with gzip.open(scen_dir / "trace.jsonl.gz", "wt") as handle:
        for record in trace:
            handle.write(json.dumps(record, separators=(",", ":")) + "\n")
    shutil.rmtree(scen_dir / "render")
    result = {"name": name, "scenario": scenario, "summary": summary, "findings": findings, "filmstrips": strips,
              "comparison": comparison, "rows": rows, "images": images,
              "effects": [[round(e["start_ms"] / TICK_MS, 2), e["name"], e["known"]] for e in effects if e["depth"] == 0],
              "seconds": round(time.time() - started, 1)}
    (scen_dir / "scenario.json").write_text(json.dumps(result))
    (scen_dir / "view.json").write_text(json.dumps({"frames": view, "timelines": timelines}, separators=(",", ":")))
    return result


def verdict(results, baseline, expect, strict):
    base = {}
    if baseline:
        for result in results:
            path = baseline / result["name"] / "scenario.json"
            if path.exists():
                for f in json.loads(path.read_text())["findings"]:
                    base[f["key"]] = f
    current = {f["key"]: f for r in results for f in r["findings"]}
    for key, f in current.items():
        f["status"] = ("persisting" if key in base else "new") if baseline else "unbaselined"
        if key in base:
            f["baseline_count"] = base[key]["count"]
            if (f["count"] > base[key]["count"] or
                    SEVERITY_ORDER[f["severity"]] < SEVERITY_ORDER[base[key]["severity"]]):
                f["status"] = "regressed"
    fixed = [dict(f, status="fixed") for key, f in base.items() if key not in current]
    reasons = []
    gating = ("error", "warning") if strict else ("error",)
    new = [f for f in current.values() if f["status"] == "new" and f["severity"] in gating]
    if new:
        reasons.append(f"{len(new)} new {'/'.join(gating)} finding(s)")
    regressed = [f for f in current.values() if f["status"] == "regressed" and f["severity"] in gating]
    if regressed:
        reasons.append(f"{len(regressed)} worsened {'/'.join(gating)} finding(s)")
    if not baseline:
        errors = [f for f in current.values() if f["severity"] in gating]
        if errors:
            reasons.append(f"{len(errors)} {'/'.join(gating)} finding(s) and no baseline to compare with")
    if expect is not None and (not baseline or any(r["comparison"] is None for r in results)):
        reasons.append("--expect requires a baseline trace for every selected scenario")
    unexpected = {}
    if expect is not None:
        allowed = set(expect)
        for r in results:
            for row, count in (r["comparison"] or {}).get("rows", {}).items():
                key = "effects" if row == "effects/HUD" else row
                if key not in allowed:
                    unexpected.setdefault(r["name"], {})[row] = count
        if unexpected:
            total = sum(sum(v.values()) for v in unexpected.values())
            reasons.append(f"{total} frame change(s) outside --expect {','.join(expect) or 'none'}")
    return ("FAIL" if reasons else "PASS"), reasons, fixed, unexpected


def source_fingerprint():
    """Hash of the renderer sources, so uncommitted states can be told apart."""
    digest = hashlib.sha1()
    for path in source_paths():
        digest.update(str(path.relative_to(ENGINE)).encode())
        digest.update(path.read_bytes())
    return digest.hexdigest()[:12]


def git_info():
    def git(*args):
        return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True).stdout.strip()
    return {"commit": git("rev-parse", "--short", "HEAD"), "branch": git("rev-parse", "--abbrev-ref", "HEAD"),
            "source": source_fingerprint(),
            "dirty": [line[3:] for line in git("status", "--porcelain").splitlines()
                      if "native-spectator/src" in line or "visual" in line][:30]}


def available_memory_gb():
    try:
        line = next(line for line in Path("/proc/meminfo").read_text().splitlines() if line.startswith("MemAvailable:"))
        return int(line.split()[1]) / 1024**2
    except (OSError, StopIteration):
        return None


def worker_memory_limit(memory_gb):
    # The isolated worker and its Rust replay inherit a per-process address
    # space cap. Single-threaded NumPy avoids unused BLAS thread reservations.
    os.environ["OPENBLAS_NUM_THREADS"] = "1"
    os.environ["OMP_NUM_THREADS"] = "1"
    if sys.platform == "linux":
        import resource
        limit = int(memory_gb * 1024**3)
        resource.setrlimit(resource.RLIMIT_AS, (limit, limit))


def cmd_run(args):
    catalog = scenarios_catalog()
    suites = catalog["suites"]
    if not args.scenario and args.suite not in suites:
        raise RuntimeError(f"unknown suite {args.suite!r}; choose {', '.join(suites)}")
    if args.jobs < 1 or not math.isfinite(args.memory_gb) or args.memory_gb < 1:
        raise RuntimeError("--jobs must be at least 1 and --memory-gb must be finite and at least 1")
    available = available_memory_gb()
    if available is not None and args.jobs * args.memory_gb + 4 > available:
        raise RuntimeError(f"requested workers could use {args.jobs * args.memory_gb:.1f} GiB, "
                           f"but only {available:.1f} GiB is available; reserve 4 GiB for the desktop and lower --jobs")
    names = list(dict.fromkeys(args.scenario or suites[args.suite]))
    by_name = {s["name"]: s for s in catalog["scenarios"]}
    unknown = [n for n in names if n not in by_name]
    if unknown:
        raise SystemExit(f"visual-check: unknown scenario(s) {unknown}; see `rules`/scenarios.json")
    if not args.no_build:
        build_binary()
    elif binary_is_stale():
        raise RuntimeError("clash-visual-check has no matching build fingerprint; run without --no-build")
    assets_dir = Path(args.assets).resolve()
    assets = Assets(assets_dir)
    stamp = dt.datetime.now().strftime("%Y%m%d-%H%M%S")
    label = re.sub(r"[^A-Za-z0-9_.-]+", "-", args.label or args.suite)
    run_dir = (Path(args.out) if args.out else STATE / "runs" / f"{stamp}-{label}").resolve()
    if run_dir.exists():
        raise SystemExit(f"visual-check: {run_dir} exists")
    run_dir.mkdir(parents=True)
    baseline = None
    if args.baseline != "none":
        candidate = Path(args.baseline) if args.baseline else STATE / "baseline"
        baseline = candidate.resolve() if (candidate / "report.json").exists() else None
        if args.baseline and baseline is None:
            raise SystemExit(f"visual-check: no report.json in baseline {candidate}")
    expect = None if args.expect is None else [r for r in args.expect.split(",") if r and r != "none"]
    if expect is not None and (not baseline or any(
            not (baseline / n / "trace.jsonl.gz").exists() or not (baseline / n / "scenario.json").exists()
            for n in names)):
        raise RuntimeError("--expect requires a baseline trace for every selected scenario; run and accept that suite first")
    code = git_info()
    started = time.time()
    results = []
    print(f"Workers: {args.jobs}; per-process memory cap: {args.memory_gb:g} GiB; fresh process per scenario", flush=True)
    print(f"{'SCENARIO':<22} {'FRAMES':>6} {'ERRORS':>6} {'OTHER':>5} {'DONE':>7} {'ELAPSED':>9} {'ETA':>9}", flush=True)
    with concurrent.futures.ProcessPoolExecutor(
            max_workers=args.jobs, mp_context=multiprocessing.get_context("spawn"),
            max_tasks_per_child=1, initializer=worker_memory_limit, initargs=(args.memory_gb,)) as pool:
        futures = {pool.submit(run_scenario, by_name[n], assets_dir, assets, run_dir, baseline, args.keep_png): n
                   for n in names}
        for future in concurrent.futures.as_completed(futures):
            try:
                result = future.result()
            except MemoryError:
                raise RuntimeError(f"{futures[future]} exceeded its {args.memory_gb:g} GiB worker memory cap") from None
            results.append(result)
            errors = sum(f["severity"] == "error" for f in result["findings"])
            elapsed = time.time() - started
            eta = elapsed / len(results) * (len(names) - len(results))
            print(f"{result['name']:<22} {result['summary']['frames']:>6} {errors:>6} "
                  f"{len(result['findings']) - errors:>5} {len(results):>3}/{len(names):<3} "
                  f"{elapsed:>8.1f}s {eta:>8.1f}s", flush=True)
    results.sort(key=lambda r: names.index(r["name"]))
    if source_fingerprint() != code["source"]:
        raise RuntimeError("renderer sources changed during the run; results do not certify the current sources")
    status, reasons, fixed, unexpected = verdict(results, baseline, expect, args.strict)
    report = {"version": 1, "verdict": status, "reasons": reasons, "created": dt.datetime.now().isoformat(timespec="seconds"),
              "seconds": round(time.time() - started, 1), "git": code, "assets": str(assets_dir),
              "compute": {"jobs": args.jobs, "memory_gb_per_process": args.memory_gb},
              "suite": args.suite, "label": args.label, "baseline": str(baseline) if baseline else None,
              "baseline_report": json.loads((baseline / "report.json").read_text()).get("git") if baseline else None,
              "expect": expect, "strict": args.strict, "fixed": fixed, "unexpected_changes": unexpected,
              "rules": {k: {"severity": v[0], "what": v[1], "basis": v[2]} for k, v in RULES.items()},
              "scenarios": [{k: r[k] for k in ("name", "scenario", "summary", "findings", "filmstrips", "comparison",
                                               "images", "seconds")} for r in results]}
    (run_dir / "report.json").write_text(json.dumps(report, indent=1))
    from visual_check_report import write_report
    write_report(run_dir, report)
    STATE.mkdir(parents=True, exist_ok=True)
    latest = STATE / "latest"
    if latest.is_symlink() or latest.exists():
        latest.unlink()
    latest.symlink_to(run_dir)
    prune_runs(args.keep_runs)
    print_summary(run_dir, report)
    return 0 if status == "PASS" else 1


def prune_runs(keep):
    """Delete all but the newest `keep` runs (the baseline is a separate copy)."""
    runs = sorted((p for p in (STATE / "runs").glob("*") if (p / "report.json").exists()),
                  key=lambda p: p.stat().st_mtime)
    for old in runs[:-keep] if keep > 0 else []:
        shutil.rmtree(old, ignore_errors=True)


def print_summary(run_dir, report):
    findings = [f for s in report["scenarios"] for f in s["findings"]]
    print(f"\nVERDICT: {report['verdict']}" + (f" ({'; '.join(report['reasons'])})" if report["reasons"] else ""))
    if report["baseline"]:
        new = [f for f in findings if f["status"] == "new"]
        print(f"baseline {report['baseline']} (source {(report['baseline_report'] or {}).get('source')}): {len(new)} new, "
              f"{sum(f['status'] == 'regressed' for f in findings)} worsened, {len(report['fixed'])} fixed, "
              f"{sum(f['status'] == 'persisting' for f in findings)} persisting findings")
        for f in report["fixed"][:10]:
            print(f"  FIXED  {f['severity']:<7} {f['key']}")
        for s in report["scenarios"]:
            c = s["comparison"]
            if c and c["changed_frames"]:
                print(f"  {s['name']}: {len(c['changed_frames'])}/{c['frames']} frames changed; rows: "
                      + ", ".join(f"{r} ({n})" for r, n in list(c["rows"].items())[:6]))
                for row, details in list(c["details"].items())[:4]:
                    print(f"      {row}: " + "; ".join(f"{d} x{n}" for d, n in list(details.items())[:3]))
        for scen, rows in report["unexpected_changes"].items():
            print(f"  UNEXPECTED CHANGE {scen}: {rows}")
    shown = sorted(findings, key=lambda f: (f.get("status") != "new", SEVERITY_ORDER[f["severity"]]))
    print(f"{sum(f['severity'] == 'error' for f in findings)} errors, "
          f"{sum(f['severity'] == 'warning' for f in findings)} warnings")
    for f in shown[:25]:
        print(f"  {f.get('status', ''):<11} {f['severity']:<7} {f['rule']:<24} {f['message'][:150]}")
    if len(shown) > 25:
        print(f"  ... {len(shown) - 25} more in report.json")
    print(f"\nreport:  {run_dir / 'report.html'}\njson:    {run_dir / 'report.json'}")


def cmd_accept(args):
    run_dir = Path(args.run).resolve() if args.run else (STATE / "latest").resolve()
    if not (run_dir / "report.json").exists():
        raise SystemExit(f"visual-check: no report.json in {run_dir}")
    target = STATE / "baseline"
    if run_dir == target.resolve() or target.resolve() in run_dir.parents:
        raise RuntimeError("baseline source must be outside .visual-check/baseline")
    report = json.loads((run_dir / "report.json").read_text())
    if report["verdict"] != "PASS" and not args.note.strip():
        raise RuntimeError("accepting a FAIL requires --note explaining the known findings")
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(run_dir, target)
    note = {"accepted": dt.datetime.now().isoformat(timespec="seconds"), "from": str(run_dir), "note": args.note}
    (target / "ACCEPTED.json").write_text(json.dumps(note, indent=1))
    print(f"baseline <- {run_dir}")


def cmd_show(args):
    run_dir = Path(args.run).resolve() if args.run else (STATE / "latest").resolve()
    scen_dir = run_dir / args.scenario
    if not (scen_dir / "frames").exists():
        raise SystemExit(f"visual-check: {scen_dir} has no frames (scenario ran without images)")
    view = json.loads((scen_dir / "view.json").read_text())
    from PIL import Image  # noqa: F401  (fail early if Pillow is missing)
    tiles = []
    for frame in view["frames"]:
        t = frame["t"]
        if (args.from_tick is not None and t < args.from_tick) or (args.to_tick is not None and t > args.to_tick):
            continue
        for uid, row, side, state, export, index, alpha, bbox in frame["e"]:
            if (args.uid is not None and uid != args.uid) or (args.row and row != args.row) or \
                    (args.side is not None and side != args.side):
                continue
            s = {"bbox": bbox, "anchor": [0, 0], "state": state, "frame": index}
            tiles.append((frame["i"], t, s, 0, [f"{t / 20:.2f}s {state} f{index}", f"{row}#{uid} {export[:16]}"]))
            break
    if not tiles:
        raise SystemExit("visual-check: nothing matched")
    stride = max(1, math.ceil(len(tiles) / args.max_tiles))
    tiles = tiles[::stride]
    out = Path(args.output) if args.output else scen_dir / f"show-{args.row or args.uid}-{tiles[0][0]}.png"
    strip(scen_dir / "frames", tiles, (args.width, args.height), out, columns=args.columns)
    print(out)


def cmd_rules(_args):
    for name, (severity, what, basis) in RULES.items():
        print(f"{name} [{severity}]\n  {what}\n  basis: {basis}\n")
    catalog = scenarios_catalog()
    for suite, names in catalog["suites"].items():
        print(f"suite {suite}: {', '.join(names)}")


def main(argv=None):
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    run = sub.add_parser("run", help="replay scenarios, judge them, compare with the baseline")
    run.add_argument("--suite", default="quick", help="scenario suite from scenarios.json (default quick)")
    run.add_argument("--scenario", action="append", help="run only this scenario (repeatable)")
    run.add_argument("--assets", default=str(DEFAULT_ASSETS), help="original asset bundle (default ARENA_ASSETS_DIR or engine/analysis/native-spectator)")
    run.add_argument("--baseline", help="baseline run directory; 'none' to skip (default .visual-check/baseline)")
    run.add_argument("--expect", help="comma-separated rows this change may affect ('effects' for effects/HUD); 'none' = no visual change allowed")
    run.add_argument("--strict", action="store_true", help="new warnings also fail")
    run.add_argument("--label", help="short name for the run directory")
    run.add_argument("--out", help="explicit new run directory")
    run.add_argument("--jobs", type=int, default=1, help="parallel scenarios (default 1; each uses a fresh worker)")
    run.add_argument("--memory-gb", type=float, default=4, help="per-worker/replay address-space cap in GiB on Linux (default 4)")
    run.add_argument("--no-build", action="store_true", help="skip cargo build of clash-visual-check")
    run.add_argument("--keep-png", action="store_true", help="keep lossless PNG frames next to the JPEGs")
    run.add_argument("--keep-runs", type=int, default=6, help="newest runs kept under .visual-check/runs (default 6)")
    accept = sub.add_parser("accept", help="make a run (default: latest) the baseline")
    accept.add_argument("run", nargs="?")
    accept.add_argument("--note", default="", help="why this run is the new reference")
    show = sub.add_parser("show", help="filmstrip of one entity from a run (default: latest)")
    show.add_argument("--scenario", required=True)
    show.add_argument("--run")
    show.add_argument("--uid", type=int)
    show.add_argument("--row")
    show.add_argument("--side", type=int)
    show.add_argument("--from-tick", type=float)
    show.add_argument("--to-tick", type=float)
    show.add_argument("--max-tiles", type=int, default=40)
    show.add_argument("--columns", type=int, default=8)
    show.add_argument("--width", type=int, default=120)
    show.add_argument("--height", type=int, default=140)
    show.add_argument("--output")
    sub.add_parser("rules", help="list rules and suites")
    args = parser.parse_args(argv)
    try:
        return {"run": cmd_run, "accept": cmd_accept, "show": cmd_show, "rules": cmd_rules}[args.command](args) or 0
    except (RuntimeError, OSError, ValueError) as error:
        print(f"visual-check: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
