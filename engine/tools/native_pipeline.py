#!/usr/bin/env python3
"""Run ordered native call-plan stages with verified checkpoint resumption."""

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "analysis/libg-cipher/libg.decrypted.so"
EXPECTED_SOURCE_SHA256 = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"
EXPECTED_SENTRY_SHA256 = "4cc7fe8257686a2708624ea7edc1194ca4a0b06b6ae050076e5d8c6651834980"
PYTHON = ROOT / "tools/reverse-env/bin/python"
NATIVE_UNPACK = ROOT / "tools/native_unpack.py"
COMPANION = ROOT / "analysis/native-entry/companion-text-decrypt/companion-decrypted-memory.bin"
BINDING_INDEX = ROOT / "analysis/native-init-array/protection-investigation/binding-index.json"
TIMEOUT_FAILURE = re.compile(r"Call-plan step \d+ did not return within execution bound\Z")


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def atomic_json(path, value):
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def validate_checkpoint(checkpoint_dir):
    checkpoint_file = checkpoint_dir / "checkpoint.json"
    if not checkpoint_dir.is_dir() or not checkpoint_file.is_file():
        raise ValueError(f"Checkpoint directory is missing checkpoint.json: {checkpoint_dir}")
    checkpoint = read_json(checkpoint_file)
    if checkpoint.get("source_sha256") != EXPECTED_SOURCE_SHA256:
        raise ValueError("Checkpoint source ELF SHA256 mismatch")
    init = checkpoint.get("init_array") or {}
    if (init.get("count"), init.get("completed"), init.get("in_progress")) != (763, 763, None):
        raise ValueError("Checkpoint does not contain the fully completed 763-entry game init-array")
    cxx = checkpoint.get("cxx_runtime") or {}
    if (cxx.get("init_array_count"), cxx.get("init_array_completed"), cxx.get("in_progress")) != (3, 3, None):
        raise ValueError("Checkpoint does not contain the fully completed 3-entry C++ runtime init-array")
    sentry = checkpoint.get("sentry_runtime")
    if sentry:
        if sentry.get("source_sha256") != EXPECTED_SENTRY_SHA256:
            raise ValueError("Checkpoint Sentry SDK source SHA256 mismatch")
        if (sentry.get("init_array_completed") != sentry.get("init_array_count")
                or sentry.get("in_progress") is not None):
            raise ValueError("Checkpoint Sentry SDK init-array is incomplete")
    for region in checkpoint.get("regions", []):
        region_path = checkpoint_dir / region["file"]
        if not region_path.is_file() or digest(region_path) != region["sha256"]:
            raise ValueError(f"Checkpoint region missing or hash mismatch: {region_path}")
    cpu_context = checkpoint_dir / "cpu-context.bin"
    if not cpu_context.is_file() or digest(cpu_context) != checkpoint.get("context_sha256"):
        raise ValueError("Checkpoint CPU context missing or hash mismatch")
    return checkpoint


def validate_plan(path):
    if not path.is_file():
        raise ValueError(f"Plan file does not exist: {path}")
    plan = read_json(path)
    if plan.get("source_sha256") != EXPECTED_SOURCE_SHA256:
        raise ValueError(f"Plan source ELF SHA256 mismatch: {path}")
    if not isinstance(plan.get("steps"), list) or not plan["steps"]:
        raise ValueError(f"Plan has no steps: {path}")
    return plan, digest(path)


def plan_refs(plan):
    refs = set()

    def visit(value):
        if isinstance(value, dict):
            if "ref" in value:
                refs.add(value["ref"])
            for child in value.values():
                visit(child)
        elif isinstance(value, list):
            for child in value:
                visit(child)

    visit(plan["steps"])
    visit(plan.get("inspect", []))
    return refs


def step_refs(step):
    refs = set()

    def visit(value):
        if isinstance(value, dict):
            if "ref" in value:
                refs.add(value["ref"])
            for child in value.values():
                visit(child)
        elif isinstance(value, list):
            for child in value:
                visit(child)

    for key in ("rva", "args", "size", "read_u64", "read_u32", "write_u64", "value", "sret"):
        if key in step:
            visit(step[key])
    return refs


def check_dependencies(plan, checkpoint, plan_sha):
    current = checkpoint.get("call_plan")
    if current and current.get("sha256") == plan_sha:
        # Exact-plan resumes retain its interrupted native call and prior values.
        return
    if current and current.get("in_progress") is not None:
        raise ValueError("Cannot switch plans while the checkpoint has an in-progress native call")
    if current and current.get("completed", 0) < len(current.get("steps", [])):
        raise ValueError("Cannot switch plans before the previous call plan is complete")
    if current and (checkpoint.get("failure") is not None or checkpoint.get("returned") is not True):
        raise ValueError("Cannot switch plans from a failed or incomplete native checkpoint")
    inherited_sha = plan.get("inherit_plan_sha256")
    if inherited_sha:
        if not current or current.get("sha256") != inherited_sha:
            raise ValueError(f"Plan inheritance mismatch: expected {inherited_sha}")
        available = set(current.get("values", {}))
    else:
        # native_unpack starts a new plan with an empty value map unless the
        # exact prior plan identity is declared for inheritance.
        available = set()
    for index, step in enumerate(plan["steps"]):
        missing = sorted(name for name in step_refs(step) if name not in available)
        if missing:
            raise ValueError(f"Plan step {index} references unavailable values: " + ", ".join(missing[:12]))
        if step.get("name"):
            available.add(step["name"])
    missing = sorted(name for name in plan_refs({"steps": plan.get("inspect", [])}) if name not in available)
    if missing:
        raise ValueError("Plan inspection references unavailable values: " + ", ".join(missing[:12]))


def completed_report(checkpoint):
    report_path = Path(checkpoint.get("report_path", ""))
    if not report_path.is_absolute():
        report_path = ROOT / report_path
    if not report_path.is_file():
        return None
    try:
        return read_json(report_path)
    except (OSError, ValueError):
        return None


def is_complete(plan, plan_sha, checkpoint):
    state = checkpoint.get("call_plan") or {}
    report = completed_report(checkpoint)
    return bool(
        state.get("sha256") == plan_sha
        and state.get("completed") == len(plan["steps"])
        and state.get("in_progress") is None
        and checkpoint.get("failure") is None
        and checkpoint.get("returned") is True
        and report
        and report.get("failure") is None
        and report.get("returned") is True
        and (report.get("call_plan") or {}).get("completed") == len(plan["steps"])
        and (report.get("call_plan") or {}).get("in_progress") is None
    )


def call_context_fingerprint(checkpoint):
    state = checkpoint.get("call_plan") or {}
    return (
        checkpoint.get("context_sha256"),
        state.get("completed"),
        checkpoint.get("heap_total_bytes", checkpoint.get("heap_pointer")),
    )


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkpoint", required=True, type=Path, help="Existing verified native state directory")
    parser.add_argument("--plan", required=True, action="append", type=Path, help="Call-plan JSON; repeat in execution order")
    parser.add_argument("--out", required=True, type=Path, help="New output directory; must not already exist")
    parser.add_argument("--timeout", type=int, default=40, help="Native per-call wall-clock bound in seconds (default: 40)")
    parser.add_argument("--instructions", type=int, default=0, help="Native instruction bound per call; 0 means unlimited (default)")
    parser.add_argument("--virtual-files", type=Path, help="Hash-verified read-only virtual input bindings for the first native stage; inherited by later checkpoints")
    parser.add_argument("--max-attempts", type=int, default=100, help="Maximum total native attempts across stages (default: 100)")
    args = parser.parse_args(argv)
    if args.timeout < 1 or args.instructions < 0 or args.max_attempts < 1:
        parser.error("--timeout and --max-attempts must be positive; --instructions must be nonnegative")

    checkpoint_dir = args.checkpoint.resolve()
    out_dir = args.out.resolve()
    if out_dir.exists():
        parser.error(f"Refusing to use existing output directory: {out_dir}")
    if not SOURCE.is_file() or digest(SOURCE) != EXPECTED_SOURCE_SHA256:
        parser.error("Original native source ELF is missing or its SHA256 does not match the verified source")
    for required in (PYTHON, NATIVE_UNPACK, COMPANION, BINDING_INDEX):
        if not required.is_file():
            parser.error(f"Required native execution input is missing: {required}")

    try:
        checkpoint = validate_checkpoint(checkpoint_dir)
        validated_plans = [(path.resolve(), *validate_plan(path.resolve())) for path in args.plan]
        _, first_plan, first_sha = validated_plans[0]
        check_dependencies(first_plan, checkpoint, first_sha)
    except (OSError, KeyError, TypeError, ValueError, json.JSONDecodeError) as error:
        parser.error(str(error))

    try:
        out_dir.mkdir(parents=True, exist_ok=False)
    except FileExistsError:
        parser.error(f"Refusing to use existing output directory: {out_dir}")
    manifest_path = out_dir / "manifest.json"
    manifest = {
        "source": str(SOURCE),
        "source_sha256": EXPECTED_SOURCE_SHA256,
        "out": str(out_dir),
        "initial_checkpoint": str(checkpoint_dir),
        "plans": [{"path": str(path), "sha256": plan_sha, "steps": len(plan["steps"]), "status": "pending"} for path, plan, plan_sha in validated_plans],
        "timeout_seconds": args.timeout,
        "instruction_limit": args.instructions,
        "max_attempts": args.max_attempts,
        "attempts": [],
        "status": "running",
        "final_checkpoint": str(checkpoint_dir),
    }
    atomic_json(manifest_path, manifest)
    total_attempts = 0

    for stage_index, (plan_path, plan, plan_sha) in enumerate(validated_plans, 1):
        stage = manifest["plans"][stage_index - 1]
        if manifest.get("timeout_stage") != stage_index:
            manifest["last_timeout_fingerprint"] = None
            manifest["identical_timeout_streak"] = 0
            manifest["timeout_stage"] = stage_index
        atomic_json(manifest_path, manifest)
        while True:
            checkpoint = validate_checkpoint(checkpoint_dir)
            check_dependencies(plan, checkpoint, plan_sha)
            if is_complete(plan, plan_sha, checkpoint):
                stage["status"] = "complete"
                stage["completed_steps"] = len(plan["steps"])
                stage["report"] = checkpoint.get("report_path")
                manifest["final_checkpoint"] = str(checkpoint_dir)
                atomic_json(manifest_path, manifest)
                break
            current = checkpoint.get("call_plan") or {}
            if current.get("sha256") == plan_sha and current.get("completed", 0) < len(plan["steps"]):
                prior_failure = checkpoint.get("failure")
                if not prior_failure or not TIMEOUT_FAILURE.fullmatch(prior_failure):
                    manifest["status"] = "stopped_on_non_timeout_checkpoint"
                    manifest["final_checkpoint"] = str(checkpoint_dir)
                    manifest["last_failure"] = prior_failure
                    atomic_json(manifest_path, manifest)
                    print(json.dumps({"status": manifest["status"], "stage": stage_index, "failure": prior_failure, "checkpoint": str(checkpoint_dir)}, separators=(",", ":")), flush=True)
                    return 2
            elif current.get("sha256") == plan_sha and not is_complete(plan, plan_sha, checkpoint):
                manifest["status"] = "stopped_on_incomplete_completed_plan"
                manifest["final_checkpoint"] = str(checkpoint_dir)
                atomic_json(manifest_path, manifest)
                print(json.dumps({"status": manifest["status"], "stage": stage_index, "checkpoint": str(checkpoint_dir)}, separators=(",", ":")), flush=True)
                return 2
            if total_attempts >= args.max_attempts:
                manifest["status"] = "max_attempts_reached"
                manifest["final_checkpoint"] = str(checkpoint_dir)
                atomic_json(manifest_path, manifest)
                print(json.dumps({"status": manifest["status"], "stage": stage_index, "attempts": total_attempts, "checkpoint": str(checkpoint_dir)}, separators=(",", ":")), flush=True)
                return 2

            total_attempts += 1
            attempt_number = sum(1 for record in manifest["attempts"] if record["stage"] == stage_index) + 1
            attempt_dir = out_dir / f"stage{stage_index:02d}-attempt{attempt_number:03d}"
            attempt_dir.mkdir()
            next_checkpoint = attempt_dir / "state"
            report_path = attempt_dir / "report.json"
            log_path = attempt_dir / "stdout.log"
            command = [
                str(PYTHON), str(NATIVE_UNPACK), str(SOURCE),
                "--runtime-cxx",
                "--companion-memory", str(COMPANION),
                "--binding-index", str(BINDING_INDEX),
                "--call-plan", str(plan_path),
                "--resume-from", str(checkpoint_dir),
                "--out", str(attempt_dir),
                "--checkpoint-out", str(next_checkpoint),
                "--timeout", str(args.timeout),
                "--instructions", str(args.instructions),
            ]
            if checkpoint.get("sentry_runtime"):
                command.append("--runtime-sentry")
            if args.virtual_files:
                command += ["--virtual-files", str(args.virtual_files.resolve())]
            elif checkpoint.get("virtual_files"):
                command += ["--virtual-files", checkpoint["virtual_files"]["manifest"]]
            with log_path.open("w", encoding="utf-8") as log:
                process = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=False)

            report = read_json(report_path) if report_path.is_file() else None
            failure = report.get("failure") if report else f"native_unpack exited {process.returncode} without report"
            if report is not None and process.returncode != 0 and not failure:
                failure = f"native_unpack exited {process.returncode} despite a failure-free report"
            new_checkpoint_dir = next_checkpoint if (next_checkpoint / "checkpoint.json").is_file() else checkpoint_dir
            new_checkpoint = validate_checkpoint(new_checkpoint_dir) if new_checkpoint_dir != checkpoint_dir else checkpoint
            record = {
                "stage": stage_index,
                "stage_attempt": attempt_number,
                "plan": str(plan_path),
                "plan_sha256": plan_sha,
                "returncode": process.returncode,
                "report": str(report_path) if report else None,
                "log": str(log_path),
                "failure": failure,
                "completed": ((report or {}).get("call_plan") or {}).get("completed"),
                "total": len(plan["steps"]),
                "checkpoint": str(new_checkpoint_dir),
                "context_sha256": new_checkpoint.get("context_sha256"),
                "heap_pointer": new_checkpoint.get("heap_pointer"),
            }
            manifest["attempts"].append(record)
            manifest["final_checkpoint"] = str(new_checkpoint_dir)
            stage["attempts"] = attempt_number
            stage["last_failure"] = failure
            stage["report"] = record["report"]
            atomic_json(manifest_path, manifest)
            print(json.dumps({
                "stage": stage_index,
                "attempt": attempt_number,
                "completed": record["completed"],
                "total": record["total"],
                "failure": failure,
                "report": record["report"],
            }, separators=(",", ":")), flush=True)

            if report is None:
                manifest["status"] = "missing_report"
                atomic_json(manifest_path, manifest)
                return process.returncode or 2
            if failure is None:
                state = (report.get("call_plan") or {})
                if (state.get("sha256") == plan_sha
                        and state.get("completed") == len(plan["steps"])
                        and state.get("in_progress") is None
                        and report.get("returned") is True):
                    checkpoint_dir = new_checkpoint_dir
                    stage["status"] = "complete"
                    stage["completed_steps"] = len(plan["steps"])
                    stage["last_failure"] = None
                    manifest["final_checkpoint"] = str(checkpoint_dir)
                    atomic_json(manifest_path, manifest)
                    break
                manifest["status"] = "incomplete_without_native_timeout"
                manifest["final_checkpoint"] = str(new_checkpoint_dir)
                atomic_json(manifest_path, manifest)
                return 2
            if not TIMEOUT_FAILURE.fullmatch(failure):
                manifest["status"] = "stopped_on_native_failure"
                manifest["final_checkpoint"] = str(new_checkpoint_dir)
                atomic_json(manifest_path, manifest)
                return 2

            fingerprint = call_context_fingerprint(new_checkpoint)
            if (manifest.get("last_timeout_stage") == stage_index
                    and manifest.get("last_timeout_fingerprint") == fingerprint):
                manifest["identical_timeout_streak"] = manifest.get("identical_timeout_streak", 1) + 1
            else:
                manifest["identical_timeout_streak"] = 1
            manifest["last_timeout_fingerprint"] = fingerprint
            manifest["last_timeout_stage"] = stage_index
            if manifest["identical_timeout_streak"] >= 3:
                manifest["status"] = "stalled_identical_checkpoints"
                manifest["final_checkpoint"] = str(new_checkpoint_dir)
                atomic_json(manifest_path, manifest)
                return 2
            checkpoint_dir = new_checkpoint_dir
            manifest["final_checkpoint"] = str(checkpoint_dir)
            atomic_json(manifest_path, manifest)

    manifest["status"] = "complete"
    manifest["final_checkpoint"] = str(checkpoint_dir)
    atomic_json(manifest_path, manifest)
    return 0


if __name__ == "__main__":
    sys.exit(main())
