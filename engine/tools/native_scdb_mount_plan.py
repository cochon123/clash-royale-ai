#!/usr/bin/env python3
"""Generate an original-API SCDB mount and prefab lookup continuation plan.

The plan inherits a completed original SCDB-open checkpoint, asks the original
asset registry to mount that source, then performs one lookup for an original
SQLite metadata path. It does not execute native code or synthesize asset data.
"""
import argparse
import csv
import hashlib
import json
import sqlite3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"

DATABASE_SHA256 = "792abc4536d56a4e7dc11fb825b578b4d6b3da43afa19747411f48d8d504c882"
LOOKUP_PATH = "sc3d/characters/hero/hero_barbarian_barrel/hero_prefab_barbarian_barrel_projectile_fall"
DEFAULT_OPEN = ROOT / "analysis/native-context/scdb-open-v3/plan.json"
DEFAULT_PARENT = ROOT / "analysis/native-context/scdb-register-v1/plan.json"
DEFAULT_OUT = ROOT / "analysis/native-context/scdb-mount-v2/plan.json"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def ref(name, offset=0):
    return {"ref": name, **({"offset": hex(offset)} if offset else {})}


def fde_map():
    result = {}
    index = ROOT / "analysis/native-engine-index/unwind-functions.tsv"
    with index.open(newline="") as stream:
        for row in csv.DictReader(stream, delimiter="\t"):
            result[int(row["start"], 16)] = int(row["size"])
    return result


def asset_row(database):
    uri = f"file:{database.as_posix()}?mode=ro"
    connection = sqlite3.connect(uri, uri=True)
    connection.row_factory = sqlite3.Row
    try:
        row = connection.execute(
            "SELECT path, assetType, shortid, assetFilename, uuid, format, data "
            "FROM meta WHERE path = ?", (LOOKUP_PATH,)
        ).fetchone()
        if row is None:
            raise ValueError("The requested original path is absent from SCDB meta")
        asset = connection.execute(
            "SELECT path, format, data FROM asset WHERE path = ?", (LOOKUP_PATH,)
        ).fetchone()
        if asset is None:
            raise ValueError("The requested original path is absent from SCDB asset")
        meta_blob = bytes(row["data"] or b"")
        asset_blob = bytes(asset["data"] or b"")
        return {
            "meta": {
                "path": row["path"], "assetType": row["assetType"],
                "shortid": row["shortid"], "assetFilename": row["assetFilename"],
                "uuid": row["uuid"], "format": row["format"],
                "data_bytes": len(meta_blob), "data_sha256": sha256(meta_blob),
            },
            "asset": {
                "path": asset["path"], "format": asset["format"],
                "data_bytes": len(asset_blob), "data_sha256": sha256(asset_blob),
            },
        }
    finally:
        connection.close()


def make(parent_path, open_path, out_path):
    source_path = ROOT / "analysis/libg-cipher/libg.decrypted.so"
    source_bytes = source_path.read_bytes()
    if sha256(source_bytes) != EXPECTED:
        raise ValueError("Recovered ELF digest mismatch")

    database = ROOT / "analysis/native-ui/assets.scdb"
    database_bytes = database.read_bytes()
    database_sha = sha256(database_bytes)
    if database_sha != DATABASE_SHA256:
        raise ValueError("Original SCDB identity mismatch")

    parent_bytes = parent_path.read_bytes()
    parent = json.loads(parent_bytes)
    parent_sha = sha256(parent_bytes)
    if parent.get("source_sha256") != EXPECTED:
        raise ValueError("Parent plan source ELF identity mismatch")
    open_bytes = open_path.read_bytes()
    open_plan = json.loads(open_bytes)
    open_sha = sha256(open_bytes)
    if open_plan.get("source_sha256") != EXPECTED:
        raise ValueError("Open-plan source ELF identity mismatch")
    parent_is_open = parent_sha == open_sha
    if not parent_is_open and parent.get("inherit_plan_sha256") != open_sha:
        raise ValueError("Parent must be the exact open plan or directly inherit its SHA-256")
    parent_db = open_plan.get("input_database", {})
    if parent_db.get("sha256") != DATABASE_SHA256 or parent_db.get("bytes") != len(database_bytes):
        raise ValueError("Open plan does not bind the verified original SCDB")
    opens = [s for s in open_plan.get("steps", []) if s.get("rva", "").lower() == "0x122dc74"]
    source_reads = [s for s in open_plan.get("steps", []) if s.get("name") == "original_scdb_source"]
    if len(opens) != 1 or len(source_reads) != 1 or source_reads[0].get("read_u64") != {"ref": "scdb_source_sret"}:
        raise ValueError("Open plan lacks the original source open and sret pointer read")
    if not parent_is_open:
        required_calls = {"0x138df64": 1, "0x779398": 1, "0x122de50": 1}
        parent_steps = parent.get("steps", [])
        for rva, count in required_calls.items():
            if sum(step.get("rva", "").lower() == rva for step in parent_steps) != count:
                raise ValueError(f"Registration parent must contain exactly one original call to {rva}")

    evidence_files = {
        0x1223928: ROOT / "analysis/native-focused/scdb-luna-v1/rva_1223928.asm",
        0x1223950: ROOT / "analysis/native-focused/scdb-sql-loader/rva_1223950.asm",
        0x142FDE4: None,
        0x12816C0: ROOT / "analysis/native-focused/typed-resource-manager/rva_12816c0.asm",
    }
    fdes = fde_map()
    evidence = []
    purposes = {
        0x1223928: "Thin original wrapper sets W3=0 and branches to 0x1223950; X0-X2 pass through unchanged.",
        0x1223950: "Original mount routine receives X0=registry service, X1=std::string root prefix, X2=source smart pointer; reads X2[0] and X2[8].",
        0x142FDE4: "Original std::string constructor takes X0=destination and X1=NUL-terminated UTF-8 source; creates the native string in place.",
        0x12816C0: "Original lookup receives X0=registry-holder pointer and X1=std::string; saves hidden X8 result address and loads manager from [X0].",
    }
    for rva, asm_path in evidence_files.items():
        if rva not in fdes:
            raise ValueError(f"No original FDE starts at {rva:#x}")
        if asm_path is not None and not asm_path.is_file():
            raise ValueError(f"Missing focused assembly evidence: {asm_path}")
        size = fdes[rva]
        evidence.append({
            "rva": hex(rva), "purpose": purposes[rva], "exact_FDE_size": size,
            "source_file_bytes_sha256": sha256(source_bytes[rva:rva + size]),
            **({
                "assembly_evidence": str(asm_path.relative_to(ROOT)),
                "assembly_sha256": sha256(asm_path.read_bytes()),
            } if asm_path is not None else {
                "instruction_evidence": "FDE disassembly: MOV X19,X0; MOV X20,X1; constructs from the NUL-terminated source; native function accepts destination and source in X0/X1.",
            }),
        })

    record = asset_row(database)
    path_bytes = LOOKUP_PATH.encode("utf-8") + b"\0"
    steps = [
        {
            "name": "mount_original_scdb_source_in_asset_registry",
            "rva": "0x1223928",
            "args": [ref("original_asset_registry"), ref("scdb_root_std_string"), ref("scdb_source_sret")],
        },
        {"name": "prefab_lookup_path_bytes", "data_hex": path_bytes.hex()},
        {"name": "prefab_lookup_path_string", "size": "0x18"},
        {
            "name": "construct_prefab_lookup_path_string",
            "rva": "0x142fde4",
            "args": [ref("prefab_lookup_path_string"), ref("prefab_lookup_path_bytes")],
        },
        {"name": "prefab_lookup_sret", "data_hex": "00" * 16},
        {
            "name": "lookup_original_prefab_by_path",
            "rva": "0x12816c0",
            "args": [ref("original_asset_registry_holder"), ref("prefab_lookup_path_string")],
            "sret": ref("prefab_lookup_sret"),
        },
        {"name": "prefab_ref", "read_u64": ref("prefab_lookup_sret")},
    ]
    plan = {
        "source_sha256": EXPECTED,
        "inherit_plan_sha256": parent_sha,
        "purpose": "Mount the exact original SCDB source through the original asset-registry API, then perform one original registry lookup for a path present in the original SQLite metadata.",
        "parent_plan": {"path": str(parent_path.resolve()), "sha256": parent_sha},
        "open_plan": {"path": str(open_path.resolve()), "sha256": open_sha},
        "input_database": {"path": str(database.resolve()), "sha256": database_sha, "bytes": len(database_bytes)},
        "original_asset_record": record,
        "native_api_evidence": evidence,
        "abi_contract": {
            "mount_wrapper_0x1223928": "X0=original_asset_registry service pointer; X1=empty 24-byte native root-prefix std::string; X2=&scdb_source_sret (the original 16-byte source smart pointer); wrapper sets W3=0 and branches to 0x1223950 without shifting X0-X2.",
            "lookup_0x12816c0": "X0=original_asset_registry_holder (8-byte pointer holder), X1=&native path std::string, X8=&16-byte result buffer. The first result qword is recorded as prefab_ref without requiring it to be nonzero.",
            "string_constructor_0x142fde4": "X0=24-byte destination std::string object; X1=NUL-terminated UTF-8 path bytes.",
        },
        "steps": steps,
        "inspect": [
            {"address": ref("scdb_source_sret"), "size": "0x10"},
            {"address": ref("prefab_lookup_path_string"), "size": "0x18"},
            {"address": ref("prefab_lookup_sret"), "size": "0x10"},
            {"address": ref("original_asset_registry_holder"), "size": "0x8"},
        ],
        "limitations": [
            "Plan generation only; this script does not execute native code.",
            "The original SQLite database and its original metadata row are only inspected for provenance; the native registry performs the mount and lookup at execution time.",
            "A zero prefab_ref is retained as a result for diagnosis; no record or object is fabricated and no lookup success is assumed.",
        ],
    }
    out_path.parent.mkdir(parents=True, exist_ok=True)
    encoded = (json.dumps(plan, indent=2) + "\n").encode()
    out_path.write_bytes(encoded)
    print(json.dumps({
        "plan": str(out_path.resolve()), "plan_sha256": sha256(encoded),
        "source_sha256": EXPECTED, "parent_plan_sha256": parent_sha,
        "database_sha256": database_sha, "lookup_path": LOOKUP_PATH,
        "native_execution_performed": False,
    }, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--parent-plan", required=True, type=Path)
    parser.add_argument("--open-plan", type=Path, default=DEFAULT_OPEN)
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()
    if args.out.exists():
        raise ValueError(f"Refusing to overwrite retained plan: {args.out}")
    make(args.parent_plan, args.open_plan, args.out)


if __name__ == "__main__":
    main()
