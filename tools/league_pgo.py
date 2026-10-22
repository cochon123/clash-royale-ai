#!/usr/bin/env python3
"""Build CPU-tuned league engines and train LLVM PGO on development replays.

Install the matching profiler first: rustup component add llvm-tools-preview.
All targets, binaries, profiles and build recipes stay under --out. Existing
release executables and reference traces are never overwritten.
"""
import argparse
import gzip
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

from league_validation import Engine, ROOT, replay_game, sha

TARGET = "x86_64-unknown-linux-gnu"


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def build(args, kind):
    target_dir = args.out / ("native-target" if kind == "native" else "pgo-target")
    flags = ["-C", "target-cpu=" + args.cpu]
    env = os.environ.copy()
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env.pop("CLASH_PGO_SHA256", None)
    profile = args.out / "profile.profdata"
    if kind == "instrument":
        flags += ["-C", "profile-generate=" + str(args.out / "raw")]
    elif kind == "pgo":
        trained_build = json.loads((args.out / "instrument-build.json").read_text())
        training = json.loads((args.out / "training.json").read_text())
        if trained_build["lto"] != args.lto or trained_build["rustflags"][:2] != flags:
            raise ValueError("PGO use must match the instrumented CPU and LTO settings")
        if training["profile_sha256"] != sha(profile):
            raise ValueError("merged profile identity differs from the completed training run")
        env["CLASH_PGO_SHA256"] = sha(profile)
        flags += ["-C", "profile-use=" + str(profile), "-C", "llvm-args=-pgo-warn-missing-function"]
    env.update(CARGO_TARGET_DIR=str(target_dir), CARGO_PROFILE_RELEASE_LTO=args.lto,
               CARGO_PROFILE_RELEASE_CODEGEN_UNITS="16", RUSTFLAGS=" ".join(flags))
    command = ["cargo", "build", "--manifest-path", str(ROOT / "rust-engine/Cargo.toml"),
               "--release", "--target", TARGET, "--bin", args.bin]
    started = time.monotonic()
    name = kind if args.bin == "rust_league_rl" else f"{kind}-{args.bin}"
    with (args.out / (name + "-build.log")).open("w") as log:
        subprocess.run(command, env=env, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
    # The profile covers the shared generated engine, so other runners may reuse it.
    destination = args.out / "bin" / name
    destination.parent.mkdir(exist_ok=True)
    shutil.copy2(target_dir / TARGET / "release" / args.bin, destination)
    recipe = dict(kind=kind, bin=args.bin, command=command, rustflags=flags, lto=args.lto,
                  codegen_units=16, compiler=subprocess.check_output(["rustc", "-vV"], text=True),
                  pgo_sha256=env.get("CLASH_PGO_SHA256"), binary_sha256=sha(destination),
                  build_seconds=time.monotonic()-started, cpu_info=subprocess.check_output(["lscpu"], text=True))
    # Retain the embedded source/build identity alongside the frozen executable.
    records = sorted((target_dir / TARGET / "release/build").glob("clash-recompiled-*/out/provenance.json"),
                     key=lambda p: p.stat().st_mtime)
    recipe["provenance"] = json.loads(records[-1].read_text())
    if kind == "pgo" and recipe["provenance"]["source_files_sha256"] != trained_build["provenance"]["source_files_sha256"]:
        destination.unlink()
        raise ValueError("source changed after PGO instrumentation; retrain a fresh output directory")
    write_json(args.out / (name + "-build.json"), recipe)
    print(json.dumps({k: v for k, v in recipe.items() if k in ("kind", "binary_sha256", "build_seconds")}), flush=True)


def train(args):
    manifest = json.loads((args.traces / "manifest.json").read_text())
    if manifest["split"] != "development":
        raise ValueError("PGO training requires development traces; reserve held-out games for checking")
    raw = args.out / "raw"
    raw.mkdir(exist_ok=True)
    if any(raw.glob("*.profraw")):
        raise ValueError("raw profiles already exist; use a fresh --out or remove them explicitly")
    previous = os.environ.get("LLVM_PROFILE_FILE")
    os.environ["LLVM_PROFILE_FILE"] = str(raw / "%m-%p.profraw")
    engine = None
    results = []
    started = time.monotonic()
    try:
        engine = Engine(args.out / "bin/instrument", args.out / "training-engine.log")
        for job in manifest["jobs"]:
            trace_path = args.traces / f"{job['id']:04d}.json.gz"
            with gzip.open(trace_path, "rt") as handle:
                result = replay_game(json.load(handle), engine)
            results.append(dict(id=job["id"], trace_sha256=sha(trace_path), **result))
            print(json.dumps({k: results[-1][k] for k in ("id", "matched", "compared", "seconds")}), flush=True)
            if not result["matched"]:
                raise RuntimeError(f"instrumented build differs on development game {job['id']}")
        # SIGTERM bypasses LLVM's atexit profile flush. Use the protocol's close
        # request, then wait for successful normal process termination.
        engine.request({"op": "close"})
        if engine.proc.wait(timeout=30) != 0:
            raise RuntimeError("instrumented engine failed on normal exit")
    finally:
        if engine:
            engine.close()
        if previous is None:
            os.environ.pop("LLVM_PROFILE_FILE", None)
        else:
            os.environ["LLVM_PROFILE_FILE"] = previous
    sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip())
    profdata = sysroot / "lib/rustlib" / TARGET / "bin/llvm-profdata"
    profiles = sorted(raw.glob("*.profraw"))
    if not profiles:
        raise RuntimeError("normal exit produced no LLVM profiles")
    merged = args.out / "profile.profdata"
    subprocess.run([str(profdata), "merge", "-sparse", *map(str, profiles), "-o", str(merged)], check=True)
    write_json(args.out / "training.json", dict(
        split=manifest["split"], manifest_sha256=sha(args.traces / "manifest.json"),
        binary_sha256=sha(args.out / "bin/instrument"), profile_sha256=sha(merged),
        elapsed_seconds=time.monotonic()-started, games=len(results),
        requests=sum(r["compared"] for r in results), results=results))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--traces", type=Path, default=ROOT / "analysis/fast-league-worktree-20261022/development")
    parser.add_argument("--phase", choices=("native", "instrument", "train", "pgo", "all"), default="all")
    parser.add_argument("--cpu", default="native", help="Rust target-cpu; native binaries require a compatible host")
    parser.add_argument("--lto", choices=("off", "thin", "fat"), default="thin")
    parser.add_argument("--bin", default="rust_league_rl",
                        help="runner to build; training always replays league traces with rust_league_rl")
    args = parser.parse_args()
    args.out = args.out.resolve()
    args.traces = args.traces.resolve()
    args.out.mkdir(parents=True, exist_ok=True)
    for phase in (("native", "instrument", "train", "pgo") if args.phase == "all" else (args.phase,)):
        train(args) if phase == "train" else build(args, phase)


if __name__ == "__main__":
    main()
