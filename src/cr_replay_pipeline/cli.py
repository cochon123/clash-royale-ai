from __future__ import annotations

import argparse
import json
import os
import sys
import textwrap
import time
from datetime import datetime
from pathlib import Path

from .api import discover_battles, normalize_battlelog_directory
from .cleaner import clean_directory
from .frontier import Frontier
from .server import serve


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="cr-replays")
    commands = parser.add_subparsers(dest="command", required=True)

    server = commands.add_parser("serve", help="Run the local extension ingest server")
    server.add_argument("--raw-dir", default="data/raw")
    server.add_argument("--host", default="127.0.0.1")
    server.add_argument("--port", type=int, default=8765)
    server.add_argument("--db", default="data/collector.sqlite3")

    status = commands.add_parser("status", help="Show durable collector status")
    status.add_argument("--db", default="data/collector.sqlite3")
    status.add_argument("--raw-dir", default="data/raw")
    status.add_argument("--json", action="store_true", help="Emit machine-readable JSON")
    status.add_argument("--no-color", action="store_true")

    watch = commands.add_parser("watch", help="Continuously show collector status")
    watch.add_argument("--db", default="data/collector.sqlite3")
    watch.add_argument("--raw-dir", default="data/raw")
    watch.add_argument("--interval", type=float, default=5)
    watch.add_argument("--json", action="store_true", help="Emit one JSON object per line")
    watch.add_argument("--no-color", action="store_true")

    policy_training_watch = commands.add_parser(
        "watch-policy-training",
        help="Stream policy training progress with completed work and ETA",
    )
    policy_training_watch.add_argument("--progress", required=True)
    policy_training_watch.add_argument("--interval", type=float, default=2.0)

    pause = commands.add_parser("pause", help="Pause new player claims")
    pause.add_argument("--db", default="data/collector.sqlite3")
    pause.add_argument("--reason", default="paused by operator")

    resume = commands.add_parser("resume", help="Resume player claims")
    resume.add_argument("--db", default="data/collector.sqlite3")

    seed = commands.add_parser("seed", help="Seed the frontier from tags or replay files")
    seed.add_argument("tags", nargs="*")
    seed.add_argument("--input", help="Existing raw replay directory")
    seed.add_argument("--db", default="data/collector.sqlite3")

    discover = commands.add_parser(
        "discover", help="Collect official leaderboard battlelog metadata"
    )
    discover.add_argument("--output", default="data/metadata/battles.jsonl")
    discover.add_argument("--limit", type=int, default=1000)
    discover.add_argument("--workers", type=int, default=12)

    normalize = commands.add_parser(
        "normalize-battlelogs",
        help="Deduplicate and normalize previously downloaded battlelog JSON files",
    )
    normalize.add_argument("--input", required=True)
    normalize.add_argument("--output", default="data/metadata/battles.jsonl")

    clean = commands.add_parser("clean", help="Parse, label, deduplicate, and filter")
    clean.add_argument("--input", required=True)
    clean.add_argument("--output", default="data/cleaned")
    clean.add_argument("--metadata")
    clean.add_argument("--legacy-roster", choices=["december-2025"])
    clean.add_argument("--audit-only", action="store_true")
    clean.add_argument("--report")

    train_winner = commands.add_parser(
        "train-winner",
        help="Train a GPU model that predicts the battle winner from action prefixes",
    )
    train_winner.add_argument("--input", default="data/raw")
    train_winner.add_argument("--output", default="models/winner_predictor")
    train_winner.add_argument("--card-costs", default="data/card_costs.json")
    train_winner.add_argument("--epochs", type=int, default=40)
    train_winner.add_argument("--batch-size", type=int, default=64)
    train_winner.add_argument("--lr", type=float, default=2e-4)
    train_winner.add_argument("--d-model", type=int, default=160)
    train_winner.add_argument("--num-layers", type=int, default=2)
    train_winner.add_argument("--min-card-plays", type=int, default=12)
    train_winner.add_argument("--seed", type=int, default=42)
    train_winner.add_argument("--device", default=None, help="cuda, cpu, or omit for auto")

    train_winner_hgb = commands.add_parser(
        "train-winner-hgb",
        help=(
            "Train a full-game symmetric tree ensemble "
            "(stronger than the sequence net on current replays)"
        ),
    )
    train_winner_hgb.add_argument("--input", default="data/raw")
    train_winner_hgb.add_argument("--output", default="models/winner_predictor")
    train_winner_hgb.add_argument("--card-costs", default="data/card_costs.json")
    train_winner_hgb.add_argument("--min-card-plays", type=int, default=12)
    train_winner_hgb.add_argument("--seed", type=int, default=42)
    train_winner_hgb.add_argument(
        "--trees",
        "--ensemble-size",
        dest="trees",
        type=int,
        default=100,
        help="Number of Extra Trees (the old --ensemble-size name remains an alias)",
    )

    report_winner = commands.add_parser(
        "report-winner",
        help="Generate HTML training reports for winner prediction models",
    )
    report_winner.add_argument("--model-dir", default="models/winner_predictor")
    report_winner.add_argument("--output-dir", default="reports")

    train_realism = commands.add_parser(
        "train-realism",
        help=(
            "Train a realism scorer that separates real battles from "
            "legal-but-random synthetic ones"
        ),
    )
    train_realism.add_argument("--input", default="data/raw")
    train_realism.add_argument("--output", default="models/realism_scorer")
    train_realism.add_argument("--card-costs", default="data/card_costs.json")
    train_realism.add_argument("--min-card-plays", type=int, default=12)
    train_realism.add_argument("--seed", type=int, default=42)
    train_realism.add_argument("--trees", type=int, default=120)
    train_realism.add_argument(
        "--per-tier",
        type=int,
        default=1,
        help="How many synthetic battles to generate per real battle per tier",
    )

    report_realism = commands.add_parser(
        "report-realism",
        help="Generate an HTML training report for the realism scorer",
    )
    report_realism.add_argument("--model-dir", default="models/realism_scorer")
    report_realism.add_argument("--output-dir", default="reports")

    train_style = commands.add_parser(
        "train-style",
        help=(
            "Train a human-vs-AI style discriminator on policy rollouts "
            "and rank policies by detectability"
        ),
    )
    train_style.add_argument("--input", default="data/raw")
    train_style.add_argument("--output", default="models/style_discriminator")
    train_style.add_argument("--card-costs", default="data/card_costs.json")
    train_style.add_argument(
        "--train-policy",
        default="models/policy_bc",
        help="Policy used to generate training negatives (default: v2)",
    )
    train_style.add_argument(
        "--eval-policies",
        default=(
            "models/policy_bc,models/policy_bc_v3,"
            "models/policy_bc_v4,models/policy_bc_v4.1,models/policy_bc_v5"
        ),
        help="Comma-separated policy dirs to evaluate",
    )
    train_style.add_argument("--min-card-plays", type=int, default=12)
    train_style.add_argument("--seed", type=int, default=42)
    train_style.add_argument("--trees", type=int, default=120)
    train_style.add_argument("--train-battles", type=int, default=2000)
    train_style.add_argument("--eval-battles", type=int, default=512)
    train_style.add_argument("--device", default=None, help="cuda, cpu, or omit for auto")
    train_style.add_argument(
        "--force-rollouts",
        action="store_true",
        help="Regenerate cached policy rollouts",
    )

    report_style = commands.add_parser(
        "report-style",
        help="Generate an HTML training report for the style discriminator",
    )
    report_style.add_argument("--model-dir", default="models/style_discriminator")
    report_style.add_argument("--output-dir", default="reports")

    train_policy = commands.add_parser(
        "train-policy",
        help=(
            "Train a behavior-cloning next-action policy "
            "(card slot, placement, timing) from replay history"
        ),
    )
    train_policy.add_argument("--input", default="data/raw")
    train_policy.add_argument("--output", default="models/policy_bc")
    train_policy.add_argument("--card-costs", default="data/card_costs.json")
    train_policy.add_argument(
        "--realism-model-dir",
        default="models/realism_scorer",
        help="Optional realism scorer for offline rollout scoring",
    )
    train_policy.add_argument("--epochs", type=int, default=25)
    train_policy.add_argument(
        "--batch-size",
        type=int,
        default=None,
        help="Batch size (v7 defaults to 512; earlier policies default to 256)",
    )
    train_policy.add_argument(
        "--max-samples-per-battle",
        type=int,
        default=40,
        help="Causal action windows retained per battle",
    )
    train_policy.add_argument("--lr", type=float, default=2e-4)
    train_policy.add_argument("--d-model", type=int, default=160)
    train_policy.add_argument("--num-layers", type=int, default=2)
    train_policy.add_argument("--min-card-plays", type=int, default=12)
    train_policy.add_argument(
        "--max-battles",
        type=int,
        default=None,
        help="Optional fixed battle count for reproducible same-data comparisons",
    )
    train_policy.add_argument("--seed", type=int, default=42)
    train_policy.add_argument("--device", default=None, help="cuda, cpu, or omit for auto")
    train_policy.add_argument(
        "--version",
        default="2",
        choices=["2", "3", "4", "4.1", "4.2", "4.3", "5", "6", "6.1", "7"],
        help=(
            "2=cycle features; 3=threat+reaction; "
            "4=v3 + jointly trained card-conditioned placement; "
            "4.1=same architecture as 4, new data cut; "
            "4.2=v4.1 plus mirrored training augmentation; "
            "4.3=v4.2 recipe + larger trunk + toggled latent think loop for inference compute; "
            "5=v4.1 + style feature matching + REINFORCE vs style judge; "
            "6=v4 trunk + card-conditioned placement heatmap + hidden opponent deck augmentation; "
            "6.1=v4.1 warm-start + frozen incumbent heads + tile-head-only isolation; "
            "7=v6.1 + causal decaying arena-memory placement adapter"
        ),
    )
    train_policy.add_argument(
        "--style-model-dir",
        default="models/style_discriminator",
        help="Frozen style discriminator for v5 anti-detector training",
    )
    train_policy.add_argument(
        "--action-clock-dir",
        default="models/action_clock_v1",
        help="Action clock for v5 clock-aware REINFORCE rollouts",
    )
    train_policy.add_argument(
        "--warmstart-dir",
        default="models/policy_bc_v4.1",
        help="Optional v4.1 checkpoint to warm-start v5 from",
    )
    train_policy.add_argument(
        "--style-match-weight",
        type=float,
        default=0.4,
        help="Weight of differentiable style moment-matching loss (v5)",
    )
    train_policy.add_argument(
        "--reaction-weight",
        type=float,
        default=3.0,
        help="Loss weight for post-threat reaction samples (v3)",
    )
    train_policy.add_argument(
        "--reaction-repeats",
        type=int,
        default=2,
        help="Extra index repeats for reaction samples (v3)",
    )
    train_policy.add_argument(
        "--hide-opponent-prob",
        type=float,
        default=0.0,
        help="For v6 training, probability of replacing unrevealed opponent cards with UNK",
    )
    train_policy.add_argument(
        "--freeze-backbone",
        action="store_true",
        help="Train only the v6.1 heatmap head after warm-starting",
    )
    train_policy.add_argument("--split-manifest", default=None)
    train_policy.add_argument("--write-split-manifest", default=None)
    train_policy.add_argument(
        "--training-stage",
        choices=["arena-adapter", "placement-calibration"],
        default=None,
        help="v7 adapter stage; defaults to arena-adapter",
    )
    train_policy.add_argument(
        "--arena-control",
        choices=["aligned", "shuffled"],
        default="aligned",
        help="v7 aligned memory or deterministic shuffled-memory control",
    )
    train_policy.add_argument("--arena-gate-bias", type=float, default=-2.2)
    train_policy.add_argument("--progress-path", default=None)
    train_policy.add_argument("--mirror-training", action="store_true")
    train_policy.add_argument("--training-log-path", default=None)
    train_policy.add_argument(
        "--max-think-steps",
        type=int,
        default=None,
        help="Max latent refine steps (v4.3 defaults to 8; 0 disables think loop)",
    )
    train_policy.add_argument(
        "--eval-think-steps",
        type=int,
        default=None,
        help="Fixed think depth used for val/test checkpoint selection (default=max)",
    )

    manifest_policy = commands.add_parser(
        "make-policy-manifest",
        help="Freeze battle IDs and splits for a reproducible policy experiment",
    )
    manifest_policy.add_argument("--input", default="data/raw")
    manifest_policy.add_argument("--output", default="data/splits/policy_v7_33558_seed42.json")
    manifest_policy.add_argument("--pilot-output", default=None)
    manifest_policy.add_argument("--card-costs", default="data/card_costs.json")
    manifest_policy.add_argument("--min-card-plays", type=int, default=12)
    manifest_policy.add_argument("--max-battles", type=int, default=33558)
    manifest_policy.add_argument("--pilot-train-battles", type=int, default=5000)
    manifest_policy.add_argument("--seed", type=int, default=42)

    showcase = commands.add_parser(
        "showcase-policy",
        help=(
            "Score two policy checkpoints on held-out actions and emit "
            "interactive-report data (arena heatmaps, fixes, scenarios)"
        ),
    )
    showcase.add_argument("--input", default="data/raw")
    showcase.add_argument("--new-policy-dir", default="models/policy_bc_v4")
    showcase.add_argument("--old-policy-dir", default="models/policy_bc_v3")
    showcase.add_argument("--card-costs", default="data/card_costs.json")
    showcase.add_argument("--output", default="reports/policy_showcase_v4.json")
    showcase.add_argument("--max-battles", type=int, default=700)
    showcase.add_argument("--max-samples-per-battle", type=int, default=10)
    showcase.add_argument("--top-cards", type=int, default=12)
    showcase.add_argument("--seed", type=int, default=42)
    showcase.add_argument("--device", default=None)

    report_showcase = commands.add_parser(
        "report-showcase",
        help="Render the interactive policy showcase HTML report",
    )
    report_showcase.add_argument(
        "--showcase-json", default="reports/policy_showcase_v4.json"
    )
    report_showcase.add_argument("--model-dir", default="models/policy_bc_v4")
    report_showcase.add_argument("--old-model-dir", default="models/policy_bc_v3")
    report_showcase.add_argument(
        "--slice-json", default="reports/defense_slice_eval_v4.json"
    )
    report_showcase.add_argument(
        "--support-json", default="reports/defense_support_audit_v4.json"
    )
    report_showcase.add_argument("--output", default="reports/policy_bc_v4_showcase.html")

    report_policy = commands.add_parser(
        "report-policy",
        help="Generate an HTML training report for the behavior-cloning policy",
    )
    report_policy.add_argument("--model-dir", default="models/policy_bc")
    report_policy.add_argument("--output-dir", default="reports")
    report_policy.add_argument(
        "--output-name",
        default=None,
        help="Optional HTML filename (default derived from model_name)",
    )

    compare_policy = commands.add_parser(
        "compare-policy",
        help="Render an HTML comparison report between two policy checkpoints",
    )
    compare_policy.add_argument("--old-model-dir", default="models/policy_bc_v4")
    compare_policy.add_argument("--new-model-dir", default="models/policy_bc_v4.1")
    compare_policy.add_argument(
        "--output", default="reports/policy_bc_v4_1_compare.html"
    )

    predict_policy = commands.add_parser(
        "predict-policy",
        help="Offline next-action prediction from a raw replay prefix (no live play)",
    )
    predict_policy.add_argument("replay", help="Path to a raw RoyaleAPI replay JSON")
    predict_policy.add_argument("--model-dir", default="models/policy_bc")
    predict_policy.add_argument("--card-costs", default="data/card_costs.json")
    predict_policy.add_argument("--prefix-events", type=int, default=20)
    predict_policy.add_argument("--side", choices=["team", "opponent"], default="team")
    predict_policy.add_argument("--device", default=None)
    predict_policy.add_argument(
        "--think-steps",
        type=int,
        default=0,
        help="v4.3 latent think depth (0=off/fast; higher spends more compute)",
    )

    matchup = commands.add_parser(
        "eval-matchups",
        help=(
            "Mine strong deck matchups from data, run policy-vs-policy games, "
            "and compare win rates via the winner model"
        ),
    )
    matchup.add_argument("--input", default="data/raw")
    matchup.add_argument("--policy-dir", default="models/policy_bc")
    matchup.add_argument("--winner-dir", default="models/winner_predictor")
    matchup.add_argument("--card-costs", default="data/card_costs.json")
    matchup.add_argument("--output", default="reports/matchup_eval.json")
    matchup.add_argument("--games", type=int, default=48)
    matchup.add_argument("--top-k", type=int, default=6)
    matchup.add_argument("--min-n", type=int, default=60)
    matchup.add_argument("--seed", type=int, default=42)
    matchup.add_argument("--device", default=None)

    royale = commands.add_parser(
        "battle-royale",
        help=(
            "Round-robin every policy AI offline; winner predictor judges "
            "games (optional confidence gate)"
        ),
    )
    royale.add_argument("--input", default="data/raw")
    royale.add_argument(
        "--policy-dir",
        action="append",
        dest="policy_dirs",
        default=None,
        help="Policy checkpoint dir (repeatable). Defaults to all policy_bc*",
    )
    royale.add_argument("--winner-dir", default="models/winner_predictor")
    royale.add_argument("--card-costs", default="data/card_costs.json")
    royale.add_argument("--output", default="reports/battle_royale.json")
    royale.add_argument("--html", default="reports/battle_royale.html")
    royale.add_argument("--games", type=int, default=48, help="Games per pair")
    royale.add_argument(
        "--min-confidence",
        type=float,
        default=0.80,
        help="Keep only judge decisions with calibrated confidence >= this",
    )
    royale.add_argument("--seed", type=int, default=42)
    royale.add_argument("--device", default=None)

    defense = commands.add_parser(
        "eval-defense",
        help=(
            "Probe defensive card choice: opponent drops a threat, hand has "
            "1 strong answer + 3 weak cards"
        ),
    )
    defense.add_argument("--policy-dir", default="models/policy_bc")
    defense.add_argument("--card-costs", default="data/card_costs.json")
    defense.add_argument("--output", default="reports/defense_eval.json")
    defense.add_argument("--trials", type=int, default=64)
    defense.add_argument("--seed", type=int, default=42)
    defense.add_argument("--device", default=None)

    defense_slice = commands.add_parser(
        "eval-defense-slice",
        help=(
            "Score policy on real held-out reaction windows after hog/balloon/"
            "graveyard/golem (no forced hand)"
        ),
    )
    defense_slice.add_argument("--input", default="data/raw")
    defense_slice.add_argument("--policy-dir", default="models/policy_bc")
    defense_slice.add_argument("--card-costs", default="data/card_costs.json")
    defense_slice.add_argument("--output", default="reports/defense_slice_eval.json")
    defense_slice.add_argument("--max-response-seconds", type=float, default=5.0)
    defense_slice.add_argument("--seed", type=int, default=42)
    defense_slice.add_argument("--device", default=None)

    report_defense_slice = commands.add_parser(
        "report-defense-slice",
        help="HTML report for real defense-slice + fair counterfactual probes",
    )
    report_defense_slice.add_argument(
        "--slice-json", default="reports/defense_slice_eval.json"
    )
    report_defense_slice.add_argument(
        "--fair-json", default="reports/defense_eval_fair.json"
    )
    report_defense_slice.add_argument(
        "--output", default="reports/defense_slice_v1.html"
    )

    support_audit = commands.add_parser(
        "audit-defense-support",
        help=(
            "Data-support + natural-hand counterfactual audit for defense "
            "probe cells (GY→poison, hog→tornado, controls)"
        ),
    )
    support_audit.add_argument("--input", default="data/raw")
    support_audit.add_argument("--policy-dir", default="models/policy_bc")
    support_audit.add_argument("--card-costs", default="data/card_costs.json")
    support_audit.add_argument(
        "--output", default="reports/defense_support_audit.json"
    )
    support_audit.add_argument("--max-response-seconds", type=float, default=5.0)
    support_audit.add_argument("--seed", type=int, default=42)
    support_audit.add_argument("--device", default=None)
    support_audit.add_argument(
        "--no-score-model",
        action="store_true",
        help="Coverage-only audit (skip policy scoring on test)",
    )

    report_support = commands.add_parser(
        "report-defense-support",
        help="HTML report for defense support / natural-hand audit",
    )
    report_support.add_argument(
        "--audit-json", default="reports/defense_support_audit.json"
    )
    report_support.add_argument(
        "--output", default="reports/defense_support_audit_v1.html"
    )

    placement = commands.add_parser(
        "placement-probe",
        help=(
            "Experiment B: card-conditioned placement probe + per-card "
            "lookup control (offline; freezes policy_bc_v3 trunk)"
        ),
    )
    placement.add_argument("--input", default="data/raw")
    placement.add_argument("--policy-dir", default="models/policy_bc_v3")
    placement.add_argument("--card-costs", default="data/card_costs.json")
    placement.add_argument(
        "--output-json", default="reports/placement_probe_v1.json"
    )
    placement.add_argument(
        "--output-html", default="reports/placement_probe_v1.html"
    )
    placement.add_argument("--epochs", type=int, default=10)
    placement.add_argument("--batch-size", type=int, default=256)
    placement.add_argument("--probe-batch-size", type=int, default=512)
    placement.add_argument("--lr", type=float, default=1e-3)
    placement.add_argument("--hidden", type=int, default=256)
    placement.add_argument("--min-card-plays", type=int, default=12)
    placement.add_argument("--seed", type=int, default=42)
    placement.add_argument("--device", default=None)

    phone_lab = commands.add_parser(
        "phone-lab",
        help=(
            "Open a browser lab for dual-phone calibration and live battle: "
            "streams, hand detection, test taps, and policy-driven AI vs AI"
        ),
    )
    phone_lab.add_argument("--host", default="127.0.0.1")
    phone_lab.add_argument("--port", type=int, default=8766)
    phone_lab.add_argument(
        "--pixel9",
        default="4B090DLAQ002ZT",
        help="ADB serial for Pixel 9",
    )
    phone_lab.add_argument(
        "--pixel8",
        default="41060DLJH000KW",
        help="ADB serial for Pixel 8",
    )
    phone_lab.add_argument(
        "--calib-dir", default="data/phone_lab/calibrations"
    )
    phone_lab.add_argument(
        "--yolo-model",
        default="/home/cochon/Documents/ClashRoyaleAI/models/yolo/card_detector.pt",
    )
    phone_lab.add_argument(
        "--card-costs", default="data/card_costs.json"
    )
    phone_lab.add_argument(
        "--policy-v3", default="models/policy_bc_v3"
    )
    phone_lab.add_argument(
        "--policy-v4", default="models/policy_bc_v4"
    )
    phone_lab.add_argument(
        "--policy-v41", default="models/policy_bc_v4.1",
        help="v4.1 checkpoint exposed as policy_bc_v4.1",
    )
    phone_lab.add_argument(
        "--policy-v42", default="models/policy_bc_v4.2_full",
        help="full-data v4.2 checkpoint exposed as policy_bc_v4.2",
    )
    phone_lab.add_argument(
        "--mirror-tta",
        action="store_true",
        help="Use two-pass horizontal-mirror inference for live policy battles",
    )
    phone_lab.add_argument(
        "--think-steps",
        type=int,
        default=0,
        help="v4.3 latent think depth for live policy battles (0=off)",
    )
    phone_lab.add_argument(
        "--no-open",
        action="store_true",
        help="Do not open a browser tab automatically",
    )

    place_cal = commands.add_parser(
        "phone-lab-calibrate",
        help=(
            "Matplotlib click calibration: set bridge left/right, my corners, "
            "and enemy corners on a live phone screenshot"
        ),
    )
    place_cal.add_argument(
        "--phone",
        default="pixel9",
        help="pixel9, pixel8, both, or an ADB serial",
    )
    place_cal.add_argument(
        "--calib-dir", default="data/phone_lab/calibrations"
    )
    place_cal.add_argument(
        "--pixel9", default="4B090DLAQ002ZT", help="Used when --phone both"
    )
    place_cal.add_argument(
        "--pixel8", default="41060DLJH000KW", help="Used when --phone both"
    )

    audit_hands = commands.add_parser(
        "audit-hands",
        help=(
            "Experiment A: exact CR cycle hand reconstruction vs oldest-four "
            "heuristic; rescore frozen policy masks (offline)"
        ),
    )
    audit_hands.add_argument("--input", default="data/raw")
    audit_hands.add_argument("--policy-dir", default="models/policy_bc_v3")
    audit_hands.add_argument("--card-costs", default="data/card_costs.json")
    audit_hands.add_argument(
        "--output-json", default="reports/hand_audit_v1.json"
    )
    audit_hands.add_argument(
        "--output-html", default="reports/hand_audit_v1.html"
    )
    audit_hands.add_argument("--seed", type=int, default=42)
    audit_hands.add_argument(
        "--device", default="cpu", help="cpu recommended for this audit"
    )
    audit_hands.add_argument(
        "--max-quality-battles",
        type=int,
        default=None,
        help="Optional cap on battles for heuristic-quality pass",
    )
    audit_hands.add_argument(
        "--cache",
        default=None,
        help="Optional battles cache pickle path",
    )
    audit_hands.add_argument(
        "--no-stale-cache",
        action="store_true",
        help="Refuse stale winner_battles_cache.pkl (rebuild if file_count drifts)",
    )
    audit_hands.add_argument(
        "--quality-workers",
        type=int,
        default=4,
        help="Process workers for heuristic-quality pass",
    )
    return parser


def _status(frontier: Frontier, raw_dir: str | None = None) -> dict:
    status = frontier.status()
    if raw_dir is not None:
        status["raw_replays"] = sum(1 for _ in Path(raw_dir).rglob("*.json"))
        players = status["players"]
        actionable = players["queued"] + players["leased"] + players["completed"]
        status["crawl_progress_percent"] = (
            round(100 * players["completed"] / actionable, 1) if actionable else 100.0
        )
    return status


class _Palette:
    def __init__(self, enabled: bool) -> None:
        self.enabled = enabled

    def paint(self, text: str, code: str) -> str:
        return f"\033[{code}m{text}\033[0m" if self.enabled else text

    def green(self, text: str) -> str:
        return self.paint(text, "32")

    def yellow(self, text: str) -> str:
        return self.paint(text, "33")

    def red(self, text: str) -> str:
        return self.paint(text, "31")

    def cyan(self, text: str) -> str:
        return self.paint(text, "36")

    def dim(self, text: str) -> str:
        return self.paint(text, "2")

    def bold(self, text: str) -> str:
        return self.paint(text, "1")


def _palette(no_color: bool = False) -> _Palette:
    return _Palette(sys.stdout.isatty() and not no_color and "NO_COLOR" not in os.environ)


def _state(status: dict) -> tuple[str, str]:
    if status["paused"]:
        return "PAUSED", "yellow"
    if status["players"]["leased"]:
        return "RUN", "green"
    if status["players"]["queued"]:
        return "IDLE", "yellow"
    return "DONE", "cyan"


def _progress_bar(percent: float, width: int = 28) -> str:
    filled = min(width, max(0, round(width * percent / 100)))
    return "█" * filled + "░" * (width - filled)


def _print_dashboard(status: dict, colors: _Palette) -> None:
    players = status["players"]
    rate = status["rate"]
    state, color_name = _state(status)
    width = 58

    def box(text: str = "") -> str:
        return colors.dim("│") + f" {text:<{width - 2}} " + colors.dim("│")

    print(colors.dim("┌" + "─" * width + "┐"))
    title = f"Clash Royale Collector  •  {state}"
    title_line = box(title).replace(state, getattr(colors, color_name)(state), 1)
    print(colors.bold(title_line))
    print(colors.dim("├" + "─" * width + "┤"))
    target = rate.get("recommended_workers", "—")
    print(box(f"Replays  {status.get('raw_replays', '—'):<5}   Players  {players['total']:<5}   Workers {players['leased']} / {target}"))
    print(box(f"Queue    {players['queued']:<5}   Completed {players['completed']:<5}   Manual  {players['manual']}"))
    print(box(f"Pace     {rate['interval_seconds']:.2f}s   Clean     {rate['clean_streak']:<5}   429s    {rate['total_rate_limits']}"))

    progress = float(status.get("crawl_progress_percent", 0))
    print(box())
    print(box(f"{_progress_bar(progress)}  {progress:5.1f}%"))

    if status["paused"]:
        reason = status.get("pause_reason") or "No reason supplied"
        print(box())
        wrapped = textwrap.wrap(reason, width=width - 13) or [reason]
        for index, line in enumerate(wrapped):
            label = "Reason" if index == 0 else ""
            rendered = box(f"{label:<8} {line}")
            if label:
                rendered = rendered.replace(label, colors.yellow(label), 1)
            print(rendered)
    elif status["active"]:
        print(box())
        worker_heading = box("Workers").replace("Workers", colors.bold("Workers"), 1)
        print(worker_heading)
        for worker in status["active"]:
            lease = datetime.fromtimestamp(worker["lease_until"]).strftime("%H:%M:%S")
            worker_line = box(
                f"● {worker['worker_id']:<10} {worker['tag']:<12} lease until {lease}"
            ).replace("●", colors.green("●"), 1)
            print(worker_line)
    elif players["queued"]:
        print(box())
        waiting = box("● Waiting for a Chrome worker to claim the queue")
        print(waiting.replace("●", colors.yellow("●"), 1))

    print(colors.dim("└" + "─" * width + "┘"), flush=True)


_WATCH_COLUMNS = (
    ("TIME", 8, "left"),
    ("STATE", 6, "left"),
    ("RAW", 5, "right"),
    ("TOTAL", 5, "right"),
    ("QUEUE", 5, "right"),
    ("ACT", 5, "right"),
    ("DONE", 5, "right"),
    ("MAN", 3, "right"),
    ("PACE", 5, "right"),
    ("OK", 4, "right"),
    ("429", 3, "right"),
    ("PROG", 6, "right"),
)


def _watch_cells(status: dict) -> list[str]:
    players = status["players"]
    rate = status["rate"]
    state, _ = _state(status)
    return [
        datetime.now().strftime("%H:%M:%S"),
        state,
        str(status.get("raw_replays", "—")),
        str(players["total"]),
        str(players["queued"]),
        f"{players['leased']}/{rate.get('recommended_workers', '—')}",
        str(players["completed"]),
        str(players["manual"]),
        f"{rate['interval_seconds']:.2f}s",
        str(rate["clean_streak"]),
        str(rate["total_rate_limits"]),
        f"{status.get('crawl_progress_percent', 0):.1f}%",
    ]


def _format_watch_cells(cells: list[str]) -> str:
    rendered = []
    for value, (_, width, alignment) in zip(cells, _WATCH_COLUMNS):
        rendered.append(f"{value:<{width}}" if alignment == "left" else f"{value:>{width}}")
    return " ".join(rendered)


def _print_watch_header(colors: _Palette) -> None:
    print(colors.bold(colors.dim(_format_watch_cells([column[0] for column in _WATCH_COLUMNS]))), flush=True)


def _print_watch_row(status: dict, colors: _Palette) -> None:
    cells = _watch_cells(status)
    _, color_name = _state(status)
    line = _format_watch_cells(cells)
    # Color only the state word, preserving the table's fixed widths.
    padded_state = f"{cells[1]:<6}"
    line = line.replace(padded_state, getattr(colors, color_name)(padded_state), 1)
    print(line, flush=True)


def _print_json(status: dict, *, compact: bool = False) -> None:
    print(
        json.dumps(status, ensure_ascii=False, separators=(",", ":") if compact else None, indent=None if compact else 2),
        flush=True,
    )


def _watch_policy_training(progress_path: str, interval: float = 2.0) -> None:
    """Tail JSONL policy progress with completed work and ETA columns."""
    target = Path(progress_path)
    last_line = ""
    print("PHASE EPOCH BATCH WORK% ELAPSED ETA TILE_LOSS LR VRAM_MB", flush=True)
    while True:
        if target.exists():
            lines = target.read_text(encoding="utf-8").splitlines()
            if lines:
                line = lines[-1]
                if line != last_line:
                    last_line = line
                    row = json.loads(line)
                    print(
                        f"{str(row.get('phase', '—'))[:14]:<14} "
                        f"{row.get('epoch', '—')}/{row.get('epochs_total', '—'):<5} "
                        f"{row.get('batch', '—')}/{row.get('batches_total', '—'):<6} "
                        f"{row.get('progress_percent', 0):6.1f}% "
                        f"{row.get('elapsed_seconds', 0):7.0f}s "
                        f"{row.get('eta_seconds', 0):7.0f}s "
                        f"{row.get('tile_loss', 0):9.4f} "
                        f"{row.get('learning_rate', 0):.2e} "
                        f"{row.get('gpu_memory_mb', 0):7.0f}",
                        flush=True,
                    )
        time.sleep(max(float(interval), 0.25))


def main() -> None:
    args = build_parser().parse_args()
    if args.command == "serve":
        serve(args.raw_dir, args.host, args.port, args.db)
    elif args.command == "status":
        status = _status(Frontier(args.db), args.raw_dir)
        if args.json:
            _print_json(status)
        else:
            _print_dashboard(status, _palette(args.no_color))
    elif args.command == "watch":
        frontier = Frontier(args.db)
        colors = _palette(args.no_color)
        if not args.json:
            _print_watch_header(colors)
        try:
            while True:
                status = _status(frontier, args.raw_dir)
                if args.json:
                    _print_json(status, compact=True)
                else:
                    _print_watch_row(status, colors)
                time.sleep(max(0.2, args.interval))
        except KeyboardInterrupt:
            print()
    elif args.command == "watch-policy-training":
        try:
            _watch_policy_training(args.progress, args.interval)
        except KeyboardInterrupt:
            print()
    elif args.command == "pause":
        frontier = Frontier(args.db)
        frontier.pause(args.reason)
        _print_dashboard(_status(frontier, "data/raw"), _palette())
    elif args.command == "resume":
        frontier = Frontier(args.db)
        frontier.resume()
        _print_dashboard(_status(frontier, "data/raw"), _palette())
    elif args.command == "seed":
        frontier = Frontier(args.db)
        result = {"tags_added": frontier.seed(args.tags)}
        if args.input:
            result["bootstrap"] = frontier.bootstrap(args.input)
        result["status"] = frontier.status()
        print(json.dumps(result, indent=2, ensure_ascii=False))
    elif args.command == "discover":
        print(
            json.dumps(
                discover_battles(args.output, args.limit, args.workers),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "normalize-battlelogs":
        print(
            json.dumps(
                normalize_battlelog_directory(args.input, args.output),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "clean":
        print(
            json.dumps(
                clean_directory(
                    args.input,
                    args.output,
                    metadata_path=args.metadata,
                    legacy_roster=args.legacy_roster,
                    audit_only=args.audit_only,
                    report_path=args.report,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "train-winner":
        from .winner_train import train_winner_model

        print(
            json.dumps(
                train_winner_model(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    epochs=args.epochs,
                    batch_size=args.batch_size,
                    learning_rate=args.lr,
                    d_model=args.d_model,
                    num_layers=args.num_layers,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "train-winner-hgb":
        from .winner_tabular import train_winner_tabular

        print(
            json.dumps(
                train_winner_tabular(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    extra_trees=args.trees,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "report-winner":
        from .winner_report import render_winner_reports

        paths = render_winner_reports(args.model_dir, args.output_dir)
        print(json.dumps({"reports": [str(path) for path in paths]}, indent=2))
    elif args.command == "train-realism":
        from .realism_train import train_realism_scorer

        print(
            json.dumps(
                train_realism_scorer(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    trees=args.trees,
                    per_tier=args.per_tier,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "report-realism":
        from .realism_report import render_realism_report

        path = render_realism_report(
            args.model_dir,
            Path(args.output_dir) / "realism_scorer_v1.html",
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "train-style":
        from .style_train import train_style_discriminator

        print(
            json.dumps(
                train_style_discriminator(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    train_policy=args.train_policy,
                    eval_policies=args.eval_policies,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    trees=args.trees,
                    train_battles=args.train_battles,
                    eval_battles=args.eval_battles,
                    device_name=args.device,
                    force_rollouts=args.force_rollouts,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "report-style":
        from .style_report import render_style_report

        path = render_style_report(
            args.model_dir,
            Path(args.output_dir) / "style_discriminator_v1.html",
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "make-policy-manifest":
        from .policy_manifest import build_manifest
        from .policy_dataset import collect_battles

        battles = collect_battles(args.input, min_card_plays=args.min_card_plays)
        selected = battles[: args.max_battles]
        full = build_manifest(
            selected,
            args.output,
            seed=args.seed,
            min_card_plays=args.min_card_plays,
        )
        result = {"output": args.output, "manifest": full}
        if args.pilot_output:
            pilot = build_manifest(
                selected,
                args.pilot_output,
                seed=args.seed,
                pilot_train_battles=args.pilot_train_battles,
                min_card_plays=args.min_card_plays,
            )
            result["pilot_output"] = args.pilot_output
            result["pilot_manifest"] = pilot
        print(json.dumps(result, indent=2, ensure_ascii=False))
    elif args.command == "train-policy":
        output = args.output
        if args.version == "5" and output == "models/policy_bc":
            output = "models/policy_bc_v5"
        elif args.version == "3" and output == "models/policy_bc":
            output = "models/policy_bc_v3"
        elif args.version == "4.3" and output == "models/policy_bc":
            output = "models/policy_bc_v4.3"
        elif args.version == "4.2" and output == "models/policy_bc":
            output = "models/policy_bc_v4.2"
        elif args.version == "4.1" and output == "models/policy_bc":
            output = "models/policy_bc_v4.1"
        elif args.version == "4" and output == "models/policy_bc":
            output = "models/policy_bc_v4"
        elif args.version == "6" and output == "models/policy_bc":
            output = "models/policy_bc_v6"
        elif args.version == "6.1" and output == "models/policy_bc":
            output = "models/policy_bc_v6_1"
        elif args.version == "7" and output == "models/policy_bc":
            output = "models/policy_bc_v7"
        if args.version == "5":
            from .policy_train_v5 import train_policy_v5

            print(
                json.dumps(
                    train_policy_v5(
                        input_dir=args.input,
                        output_dir=output,
                        card_costs_path=args.card_costs,
                        realism_model_dir=args.realism_model_dir,
                        style_model_dir=args.style_model_dir,
                        action_clock_dir=args.action_clock_dir,
                        warmstart_dir=args.warmstart_dir,
                        epochs=args.epochs,
                        batch_size=args.batch_size or 256,
                        max_samples_per_battle=args.max_samples_per_battle,
                        learning_rate=args.lr,
                        d_model=args.d_model,
                        num_layers=args.num_layers,
                        min_card_plays=args.min_card_plays,
                        seed=args.seed,
                        device_name=args.device,
                        reaction_weight=args.reaction_weight,
                        reaction_repeats=args.reaction_repeats,
                        style_match_weight=args.style_match_weight,
                    ),
                    indent=2,
                    ensure_ascii=False,
                    default=str,
                )
            )
        else:
            from .policy_train import train_policy_model

            warmstart_dir = args.warmstart_dir
            if args.version == "7" and warmstart_dir == "models/policy_bc_v4.1":
                warmstart_dir = "models/policy_bc_v6_1"

            print(
                json.dumps(
                    train_policy_model(
                        input_dir=args.input,
                        output_dir=output,
                        card_costs_path=args.card_costs,
                        realism_model_dir=args.realism_model_dir,
                        epochs=args.epochs,
                        batch_size=args.batch_size
                        or (512 if args.version in {"4.2", "4.3", "7"} else 256),
                        learning_rate=args.lr,
                        d_model=args.d_model,
                        num_layers=args.num_layers,
                        min_card_plays=args.min_card_plays,
                        max_samples_per_battle=args.max_samples_per_battle,
                        max_battles=args.max_battles,
                        seed=args.seed,
                        device_name=args.device,
                        version=args.version,
                        reaction_weight=args.reaction_weight,
                        mirror_training=args.mirror_training
                        or args.version in {"4.2", "4.3"},
                        training_log_path=args.training_log_path,
                        reaction_repeats=args.reaction_repeats,
                        hide_opponent_deck=args.version == "6",
                        hide_opponent_prob=args.hide_opponent_prob,
                        warmstart_dir=(
                            warmstart_dir
                            if args.version in {"6.1", "7"}
                            else None
                        ),
                        freeze_backbone=(
                            args.freeze_backbone or args.version in {"6.1", "7"}
                        ),
                        split_manifest=args.split_manifest,
                        write_split_manifest=args.write_split_manifest,
                        training_stage=args.training_stage,
                        arena_control=args.arena_control,
                        arena_gate_bias=args.arena_gate_bias,
                        progress_path=args.progress_path,
                        max_think_steps=args.max_think_steps,
                        eval_think_steps=args.eval_think_steps,
                    ),
                    indent=2,
                    ensure_ascii=False,
                    default=str,
                )
            )
    elif args.command == "showcase-policy":
        from .policy_showcase import build_policy_showcase

        build_policy_showcase(
            input_dir=args.input,
            new_policy_dir=args.new_policy_dir,
            old_policy_dir=args.old_policy_dir,
            card_costs_path=args.card_costs,
            output_path=args.output,
            max_battles=args.max_battles,
            max_samples_per_battle=args.max_samples_per_battle,
            top_cards=args.top_cards,
            seed=args.seed,
            device_name=args.device,
        )
    elif args.command == "report-showcase":
        from .policy_showcase_report import render_policy_showcase_report

        path = render_policy_showcase_report(
            showcase_path=args.showcase_json,
            model_dir=args.model_dir,
            old_model_dir=args.old_model_dir,
            slice_path=args.slice_json,
            support_path=args.support_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "report-policy":
        name = args.output_name
        if name is None:
            md = str(args.model_dir)
            if "v7" in md:
                name = "policy_bc_v7.html"
            elif "v6_1" in md or "v6.1" in md:
                name = "policy_bc_v6_1.html"
            elif "v6" in md:
                name = "policy_bc_v6.html"
            elif "v5" in md:
                name = "policy_bc_v5.html"
            elif "v4.3" in md or "v4_3" in md:
                name = "policy_bc_v4_3.html"
            elif "v4.2" in md or "v4_2" in md:
                name = "policy_bc_v4_2_full_showcase.html"
            elif "v4.1" in md or "v4_1" in md:
                name = "policy_bc_v4_1.html"
            elif "v4" in md:
                name = "policy_bc_v4.html"
            elif "v3" in md:
                name = "policy_bc_v3.html"
            else:
                name = "policy_bc_v2.html"
        model_dir_s = str(args.model_dir)
        if "v7" in model_dir_s or (name and "v7" in name):
            from .policy_v7_report import render_policy_v7_report

            path = render_policy_v7_report(
                model_dir=args.model_dir,
                output_path=Path(args.output_dir) / name,
            )
        elif "v6" in model_dir_s or (name and "v6" in name):
            from .policy_v6_report import render_policy_v6_report

            path = render_policy_v6_report(
                model_dir=args.model_dir,
                output_path=Path(args.output_dir) / name,
            )
        elif "v5" in model_dir_s or (name and "v5" in name):
            from .policy_v5_report import render_policy_v5_report

            path = render_policy_v5_report(
                model_dir=args.model_dir,
                battle_royale_path=Path(args.output_dir) / "battle_royale_v5.json",
                output_path=Path(args.output_dir) / name,
            )
        elif "v4.3" in model_dir_s or "v4_3" in model_dir_s or (name and "v4_3" in name):
            from .policy_v43_report import render_policy_v43_report

            path = render_policy_v43_report(
                model_dir=args.model_dir,
                output_path=Path(args.output_dir) / name,
            )
        elif "v4.2" in model_dir_s or "v4_2" in model_dir_s or (
            name and "v4_2" in name
        ):
            from .policy_v42_report import render_policy_v42_report

            path = render_policy_v42_report(
                output_path=Path(args.output_dir) / name,
            )
        else:
            from .policy_report import render_policy_report

            path = render_policy_report(
                args.model_dir,
                Path(args.output_dir) / name,
            )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "compare-policy":
        from .policy_compare_report import render_policy_compare_report

        path = render_policy_compare_report(
            old_dir=args.old_model_dir,
            new_dir=args.new_model_dir,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "predict-policy":
        from .policy_infer import demo_predict_from_raw

        print(
            json.dumps(
                demo_predict_from_raw(
                    args.replay,
                    model_dir=args.model_dir,
                    card_costs_path=args.card_costs,
                    prefix_events=args.prefix_events,
                    acting_side=args.side,
                    think_steps=args.think_steps,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "eval-matchups":
        from .matchup_eval import evaluate_matchups

        print(
            json.dumps(
                evaluate_matchups(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    winner_dir=args.winner_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    games_per_matchup=args.games,
                    top_k=args.top_k,
                    min_n=args.min_n,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "battle-royale":
        from .battle_royale import DEFAULT_POLICIES, run_battle_royale

        print(
            json.dumps(
                run_battle_royale(
                    policy_dirs=args.policy_dirs or list(DEFAULT_POLICIES),
                    input_dir=args.input,
                    winner_dir=args.winner_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    html_output=args.html,
                    games_per_pair=args.games,
                    min_confidence=args.min_confidence,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "eval-defense":
        from .defense_eval import evaluate_defense

        print(
            json.dumps(
                evaluate_defense(
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    trials_per_scenario=args.trials,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "eval-defense-slice":
        from .defense_slice_eval import evaluate_defense_slice

        print(
            json.dumps(
                evaluate_defense_slice(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    max_response_seconds=args.max_response_seconds,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "report-defense-slice":
        from .defense_slice_report import render_defense_slice_report

        path = render_defense_slice_report(
            slice_path=args.slice_json,
            fair_probe_path=args.fair_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "audit-defense-support":
        from .defense_support_audit import audit_defense_support

        print(
            json.dumps(
                audit_defense_support(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    max_response_seconds=args.max_response_seconds,
                    seed=args.seed,
                    device_name=args.device,
                    score_model=not args.no_score_model,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "report-defense-support":
        from .defense_support_report import render_defense_support_report

        path = render_defense_support_report(
            audit_path=args.audit_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "placement-probe":
        from .placement_probe import run_placement_probe

        print(
            json.dumps(
                run_placement_probe(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_json=args.output_json,
                    output_html=args.output_html,
                    epochs=args.epochs,
                    batch_size=args.batch_size,
                    probe_batch_size=args.probe_batch_size,
                    lr=args.lr,
                    hidden=args.hidden,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "audit-hands":
        from .hand_audit import run_hand_audit

        print(
            json.dumps(
                run_hand_audit(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_json=args.output_json,
                    output_html=args.output_html,
                    seed=args.seed,
                    device_name=args.device,
                    max_quality_battles=args.max_quality_battles,
                    allow_stale_cache=not args.no_stale_cache,
                    cache_path=args.cache,
                    quality_workers=args.quality_workers,
                ).get("verdict"),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "phone-lab":
        from .phone_lab import run_phone_lab

        try:
            run_phone_lab(
                host=args.host,
                port=args.port,
                pixel9=args.pixel9,
                pixel8=args.pixel8,
                calib_dir=args.calib_dir,
                yolo_model=args.yolo_model,
                card_costs=args.card_costs,
                policy_v3=args.policy_v3,
                policy_v4=args.policy_v4,
                policy_v41=args.policy_v41,
                policy_v42=args.policy_v42,
                mirror_tta=args.mirror_tta,
                think_steps=args.think_steps,
                open_browser=not args.no_open,
            )
        except RuntimeError as exc:
            print(f"phone-lab error: {exc}", file=sys.stderr)
            raise SystemExit(1) from exc
    elif args.command == "phone-lab-calibrate":
        from .phone_lab.place_calibrate import (
            run_place_calibrate,
            run_place_calibrate_both,
        )

        try:
            if args.phone.strip().lower() == "both":
                result = run_place_calibrate_both(
                    calib_dir=args.calib_dir,
                    pixel9=args.pixel9,
                    pixel8=args.pixel8,
                )
            else:
                result = run_place_calibrate(
                    phone=args.phone,
                    calib_dir=args.calib_dir,
                )
            print(json.dumps(result, indent=2, ensure_ascii=False, default=str))
        except (RuntimeError, FileNotFoundError) as exc:
            print(f"phone-lab-calibrate error: {exc}", file=sys.stderr)
            raise SystemExit(1) from exc


if __name__ == "__main__":
    main()
