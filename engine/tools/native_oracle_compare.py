#!/usr/bin/env python3
"""Compare a Rust original-engine oracle audit with a Python native checkpoint.

The caller selects corresponding source-bound runs. Every retained region is
rehash-verified before comparison; no differences in guest memory, architected
registers or allocator state are ignored. Opaque Unicorn CPU context is excluded.
"""
import argparse
import hashlib
import json
from pathlib import Path

SOURCE_SHA256 = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1 << 20), b""):
            value.update(block)
    return value.hexdigest()


def integer(value):
    return int(value, 0) if isinstance(value, str) else value


def verified_regions(metadata, directory):
    result = {}
    for region in metadata["regions"]:
        path = directory / region["file"]
        base, size = integer(region["base"]), integer(region["size"])
        if base in result:
            raise ValueError(f"duplicate region at {base:#x}")
        if path.stat().st_size != size or digest(path) != region["sha256"]:
            raise ValueError(f"invalid memory-region identity: {path}")
        result[base] = {"size": size, "sha256": region["sha256"]}
    return result


def compare(python_state, rust_directory):
    python_path = python_state / "checkpoint.json"
    rust_path = rust_directory / "audit.json"
    native = json.loads(python_path.read_text())
    rust = json.loads(rust_path.read_text())
    if native["source_sha256"] != SOURCE_SHA256 or rust["source_sha256"] != SOURCE_SHA256:
        raise ValueError("unexpected original engine source SHA-256")
    initial_path = Path(rust["checkpoint"]) / "checkpoint.json"
    if digest(initial_path) != rust["checkpoint_sha256"]:
        raise ValueError("Rust input checkpoint identity changed")
    if native.get("failure") or native["call_plan"].get("in_progress") is not None:
        raise ValueError("Python reference is failed or interrupted")
    python_regions = verified_regions(native, python_state)
    rust_regions = verified_regions(rust, rust_directory)
    region_differences = [
        {"base": hex(base), "python": python_regions.get(base), "rust": rust_regions.get(base)}
        for base in sorted(python_regions.keys() | rust_regions.keys())
        if python_regions.get(base) != rust_regions.get(base)
    ]
    reference_registers = {k: integer(v) for k, v in native["registers"].items()}
    rust_registers = {k: integer(v) for k, v in rust["registers"].items()}
    register_differences = [
        {"register": name, "python": reference_registers.get(name), "rust": rust_registers.get(name)}
        for name in sorted(reference_registers.keys() | rust_registers.keys())
        if reference_registers.get(name) != rust_registers.get(name)
    ]
    allocator_path = rust_directory / rust["allocator_file"]
    allocator_equal = json.loads(allocator_path.read_text()) == native["native_heap"]
    matched = not region_differences and not register_differences and allocator_equal
    return {
        "source_sha256": SOURCE_SHA256,
        "python_state": str(python_state.resolve()),
        "python_checkpoint_sha256": digest(python_path),
        "python_plan_sha256": native["call_plan"]["sha256"],
        "rust_audit": str(rust_path.resolve()),
        "rust_audit_sha256": digest(rust_path),
        "rust_initial_checkpoint_sha256": rust["checkpoint_sha256"],
        "rust_allocator_file_sha256": digest(allocator_path),
        "verified_region_count": len(python_regions),
        "architectural_register_count": len(reference_registers),
        "all_memory_regions_equal": not region_differences,
        "architectural_registers_equal": not register_differences,
        "allocator_state_equal": allocator_equal,
        "memory_differences": region_differences,
        "register_differences": register_differences,
        "matched": matched,
        "limit": "Equality for the selected native calls and prepared state. This does not establish a Rust combat rewrite, general gameplay parity or full Android runtime equivalence. Opaque Unicorn CPU-context bytes are not compared.",
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-state", required=True, type=Path)
    parser.add_argument("--rust-audit", required=True, type=Path, help="Directory containing audit.json")
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    report = compare(args.python_state, args.rust_audit)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    raise SystemExit(0 if report["matched"] else 1)


if __name__ == "__main__":
    main()
