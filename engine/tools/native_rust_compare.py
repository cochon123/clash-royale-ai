#!/usr/bin/env python3
"""Compare an original Unicorn audit with a compiled Rust audit exactly."""

import argparse
import hashlib
import json
import sys
from pathlib import Path

SOURCE_SHA256 = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"
CHUNK_SIZE = 1024 * 1024
MAX_ADDRESSES_PER_REGION = 8


def digest_file(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(CHUNK_SIZE), b""):
            h.update(chunk)
    return h.hexdigest()


def integer(value):
    if isinstance(value, int):
        return value
    if isinstance(value, str):
        try:
            return int(value, 0)
        except ValueError:
            return int(value, 16)
    raise ValueError(f"expected integer or numeric string, got {value!r}")


def load_audit(directory):
    audit_path = directory / "audit.json"
    audit = json.loads(audit_path.read_text())
    source = audit.get("source_sha256")
    if source != SOURCE_SHA256:
        raise ValueError(f"{audit_path}: unexpected source SHA-256 {source!r}")
    if not isinstance(audit.get("regions"), list):
        raise ValueError(f"{audit_path}: regions must be an array")
    registers = audit.get("registers")
    if not isinstance(registers, dict):
        raise ValueError(f"{audit_path}: registers must be an object")
    required_registers = ({f"x{i}" for i in range(31)} | {f"q{i}" for i in range(32)} |
                          {"sp", "pc", "nzcv", "fpcr", "fpsr", "tpidr_el0", "cpacr_el1"})
    if set(registers) != required_registers:
        raise ValueError(f"{audit_path}: expected all 70 registers including cpacr_el1")
    if not audit.get("allocator_file"):
        raise ValueError(f"{audit_path}: allocator_file is missing")
    return audit_path, audit


def verified_regions(directory, metadata):
    regions = {}
    for row in metadata["regions"]:
        name = row.get("file")
        if not isinstance(name, str):
            raise ValueError("region file name is missing")
        base, size = integer(row["base"]), integer(row["size"])
        if size < 0:
            raise ValueError(f"negative region size for {name}")
        path = (directory / name).resolve()
        try:
            path.relative_to(directory.resolve())
        except ValueError as exc:
            raise ValueError(f"region path escapes audit directory: {name}") from exc
        if base in regions:
            raise ValueError(f"duplicate region base {base:#x}")
        if not path.is_file() or path.stat().st_size != size:
            raise ValueError(f"region size/file mismatch: {path} (expected {size} bytes)")
        actual_hash = digest_file(path)
        if actual_hash != row.get("sha256"):
            raise ValueError(f"region hash mismatch: {path}")
        regions[base] = {"file": name, "base": base, "size": size,
                         "sha256": actual_hash, "path": path}
    return regions


def compare_region_bytes(reference, candidate):
    addresses = []
    compared = 0
    with reference["path"].open("rb") as left, candidate["path"].open("rb") as right:
        while True:
            a, b = left.read(CHUNK_SIZE), right.read(CHUNK_SIZE)
            if not a and not b:
                break
            common = min(len(a), len(b))
            if len(addresses) < MAX_ADDRESSES_PER_REGION:
                for offset, (x, y) in enumerate(zip(a[:common], b[:common])):
                    if x != y:
                        addresses.append(reference["base"] + compared + offset)
                        if len(addresses) == MAX_ADDRESSES_PER_REGION:
                            break
            compared += common
    return addresses


def compare(reference_dir, candidate_dir):
    ref_audit_path, reference = load_audit(reference_dir)
    cand_audit_path, candidate = load_audit(candidate_dir)
    ref_regions = verified_regions(reference_dir, reference)
    cand_regions = verified_regions(candidate_dir, candidate)
    differences = []

    for base in sorted(ref_regions.keys() | cand_regions.keys()):
        left, right = ref_regions.get(base), cand_regions.get(base)
        if left is None or right is None:
            differences.append({"type": "region_presence", "base": hex(base),
                                "reference": left is not None, "candidate": right is not None})
            continue
        if left["size"] != right["size"]:
            differences.append({"type": "region_size", "base": hex(base),
                                "reference": left["size"], "candidate": right["size"]})
        if left["sha256"] != right["sha256"]:
            first = compare_region_bytes(left, right)
            differences.append({"type": "region_bytes", "base": hex(base),
                                "reference_sha256": left["sha256"],
                                "candidate_sha256": right["sha256"],
                                "first_differing_addresses": [hex(x) for x in first],
                                "addresses_truncated": len(first) == MAX_ADDRESSES_PER_REGION})

    ref_regs = {name: integer(value) for name, value in reference["registers"].items()}
    cand_regs = {name: integer(value) for name, value in candidate["registers"].items()}
    for name in sorted(ref_regs.keys() | cand_regs.keys()):
        if ref_regs.get(name) != cand_regs.get(name):
            differences.append({"type": "register", "name": name,
                                "reference": None if name not in ref_regs else hex(ref_regs[name]),
                                "candidate": None if name not in cand_regs else hex(cand_regs[name])})

    ref_allocator_path = (reference_dir / reference["allocator_file"]).resolve()
    cand_allocator_path = (candidate_dir / candidate["allocator_file"]).resolve()
    for directory, path in ((reference_dir, ref_allocator_path), (candidate_dir, cand_allocator_path)):
        try:
            path.relative_to(directory.resolve())
        except ValueError as exc:
            raise ValueError(f"allocator path escapes audit directory: {path}") from exc
        if not path.is_file():
            raise ValueError(f"allocator file is missing: {path}")
    ref_allocator = json.loads(ref_allocator_path.read_text())
    cand_allocator = json.loads(cand_allocator_path.read_text())
    if ref_allocator != cand_allocator:
        differences.append({"type": "allocator_state", "reference_sha256": digest_file(ref_allocator_path),
                            "candidate_sha256": digest_file(cand_allocator_path)})

    return {
        "matched": not differences,
        "source_sha256": SOURCE_SHA256,
        "region_count": len(ref_regions),
        "register_count": len(ref_regs),
        "differences": differences,
        "artifact_hashes": {
            "reference_audit_sha256": digest_file(ref_audit_path),
            "candidate_audit_sha256": digest_file(cand_audit_path),
            "reference_allocator_sha256": digest_file(ref_allocator_path),
            "candidate_allocator_sha256": digest_file(cand_allocator_path),
            "reference_regions": {hex(base): item["sha256"] for base, item in sorted(ref_regions.items())},
            "candidate_regions": {hex(base): item["sha256"] for base, item in sorted(cand_regions.items())},
        },
        "scope": "Exact comparison of the audited guest-memory regions, 70 architectural registers, and allocator JSON for these two source-bound runs. It does not establish parity outside the audited runs or full platform equivalence.",
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", required=True, type=Path,
                        help="Old Rust/Unicorn audit directory containing audit.json")
    parser.add_argument("--candidate", required=True, type=Path,
                        help="New AOT Rust audit directory containing audit.json")
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    try:
        report = compare(args.reference, args.candidate)
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"native_rust_compare: {error}", file=sys.stderr)
        return 2
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["matched"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
