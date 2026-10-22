#!/usr/bin/env python3
"""Generate a source-bound plan to build original native map-cache objects.

The location table is read only to enumerate original row names and resource
paths. Original native CSV parsers, table lookup, row-index conversion, and map
construction perform the runtime work when the plan is later executed.
"""
import argparse
import csv
import hashlib
import json
from pathlib import Path

from native_engine_index import EXPECTED, ROOT

TYPED_TAIL_SHA256 = "b2fea9faa791f3c1dc5c234e649ee45bede9d0b835817d9f5fee8e2560c22236"
NATIVE_APIS = {
    0x142FDE4: "Construct an original native String from NUL-terminated UTF-8.",
    0x1288F2C: "Construct an original native CSV root node from bytes and filename String.",
    0xE12F08: "Look up an original native table row by native String name.",
    0xE0B668: "Return the original native row's decoded data index.",
    0xE1A4F4: "Create or fetch a native map-cache entry and load it from two CSV roots and row index.",
    0xE8B320: "Construct the original native map-cache object.",
    0xE8B440: "Set the map-cache object's two original CSV-root pointers.",
    0xE8B74C: "Parse the map from the location and tile-data CSV roots.",
}


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def ref(name, offset=0):
    return {"ref": name, **({"offset": hex(offset)} if offset else {})}


def make(out, report_path, parent_plan=None):
    elf = ROOT / "analysis/libg-cipher/libg.decrypted.so"
    elf_bytes = elf.read_bytes()
    if sha256(elf_bytes) != EXPECTED:
        raise ValueError("Recovered ELF digest mismatch")
    parent_sha=TYPED_TAIL_SHA256
    if parent_plan is not None:
        parent_bytes=parent_plan.read_bytes();parent=json.loads(parent_bytes)
        if parent['source_sha256']!=EXPECTED or not any(step.get('rva')=='0xe16984' for step in parent['steps']):raise ValueError('Map predecessor lacks original complete typed postload')
        parent_sha=sha256(parent_bytes)

    csv_root = ROOT / "analysis/decoded/assets"
    locations_path = csv_root / "csv_logic/locations.csv"
    locations_bytes = locations_path.read_bytes()
    with locations_path.open(encoding="utf-8-sig", newline="") as stream:
        rows = list(csv.reader(stream))
    if len(rows) < 2 or not {"Name", "FileName", "TileDataFileName"}.issubset(rows[0]):
        raise ValueError("locations.csv is missing the expected original columns")
    if len(rows[0]) != len(rows[1]):
        raise ValueError("locations.csv header/type width mismatch")
    locations = [dict(zip(rows[0], row)) for row in rows[2:]]

    for required in ("Name", "FileName", "TileDataFileName"):
        if required not in rows[0]:
            raise ValueError(f"Missing location column {required}")
    locations = [r for r in locations if (r.get("FileName", "").strip() or r.get("TileDataFileName", "").strip())]
    if any(not r.get("Name", "").strip() for r in locations):
        raise ValueError("A location with map inputs has no native row name")
    names = [r["Name"].strip() for r in locations]
    if len(names) != len(set(names)):
        raise ValueError("Duplicate location names make native table lookup ambiguous")

    # Preserve first occurrence order: the map inputs are original paths, not
    # generated grids. Each distinct CSV root is parsed once and then reused.
    input_paths = []
    for row in locations:
        for column in ("FileName", "TileDataFileName"):
            path = row.get(column, "").strip()
            if path and path not in input_paths:
                input_paths.append(path)
    files = {}
    for path in input_paths:
        if Path(path).is_absolute() or ".." in Path(path).parts:
            raise ValueError(f"Unsafe original resource path: {path}")
        source = (csv_root / path).resolve()
        if not source.is_relative_to(csv_root.resolve()):
            raise ValueError(f"Resource escapes recovered asset tree: {path}")
        if not source.is_file():
            raise ValueError(f"Unrecovered nonempty location resource: {path}")
        if source.suffix.lower() != ".csv":
            raise ValueError(f"No verified original CSV parser route for {path}")
        data = source.read_bytes()
        files[path] = {"path": path, "file": str(source), "bytes": len(data), "sha256": sha256(data)}

    unwind = {}
    with (ROOT / "analysis/native-engine-index/unwind-functions.tsv").open(newline="") as stream:
        for row in csv.DictReader(stream, delimiter="\t"):
            unwind[int(row["start"], 16)] = int(row["size"])
    api_evidence = []
    for address, purpose in sorted(NATIVE_APIS.items()):
        size = unwind.get(address)
        if size is None:
            raise ValueError(f"No exact original FDE begins at {address:#x}")
        api_evidence.append({
            "rva": hex(address),
            "purpose": purpose,
            "exact_FDE_size": size,
            "source_file_bytes_sha256": sha256(elf_bytes[address:address + size]),
        })

    steps = []
    path_roots = {}
    for index, path in enumerate(input_paths):
        tag = f"map_input_{index:03d}"
        asset = files[path]
        raw_name = f"{tag}_filename_bytes"
        native_name = f"{tag}_filename_string"
        steps.append({"name": raw_name, "data_hex": (path.encode("utf-8") + b"\0").hex()})
        steps.append({"name": native_name, "size": "0x10"})
        steps.append({"name": f"{tag}_construct_filename", "rva": "0x142fde4", "args": [ref(native_name), ref(raw_name)]})
        data_name = f"{tag}_bytes"
        steps.append({"name": data_name, "file": asset["file"], "sha256": asset["sha256"], "nul_terminate": True})
        root_name = f"{tag}_csv_root"
        steps.append({"name": root_name, "size": "0x68"})
        steps.append({
            "name": f"{tag}_parse_original_csv_root",
            "rva": "0x1288f2c",
            "args": [ref(root_name), ref(data_name), asset["bytes"], ref(native_name)],
        })
        path_roots[path] = root_name

    location_table = ref("location_table")
    steps.append({"name": "location_table", "read_u64": ref("context", 15 * 8)})
    loaded = []
    for index, row in enumerate(locations):
        name = row["Name"].strip()
        tag = f"location_{index:03d}"
        name_bytes, name_string = tag + "_name_bytes", tag + "_name_string"
        steps.append({"name": name_bytes, "data_hex": (name.encode("utf-8") + b"\0").hex()})
        steps.append({"name": name_string, "size": "0x10"})
        steps.append({"name": tag + "_construct_name", "rva": "0x142fde4", "args": [ref(name_string), ref(name_bytes)]})
        row_ref = tag + "_native_row"
        steps.append({"name": row_ref, "rva": "0xe12f08", "args": [location_table, ref(name_string), 0], "expect_nonzero": True})
        index_ref = tag + "_native_index"
        steps.append({"name": index_ref, "rva": "0xe0b668", "args": [ref(row_ref)]})
        location_path = row.get("FileName", "").strip()
        tile_path = row.get("TileDataFileName", "").strip()
        if not location_path or not tile_path:
            raise ValueError(f"Location {name!r} has an empty map input path")
        steps.append({
            "name": tag + "_load_original_map_cache",
            "rva": "0xe1a4f4",
            "args": [ref("context"), ref(path_roots[location_path]), ref(path_roots[tile_path]), ref(index_ref)],
        })
        loaded.append({"name": name, "file_name": location_path, "tile_data_file_name": tile_path, "native_index": "resolved by original e12f08/e0b668 at execution"})

    plan = {
        "source_sha256": EXPECTED,
        "inherit_plan_sha256": parent_sha,
        "purpose": "Build original native map-cache entries for original locations using native CSV parsing, native table row lookup/index decoding, and the original native map loader.",
        "context": ref("context"),
        "input_provenance": {
            "locations_csv": {"file": str(locations_path.resolve()), "bytes": len(locations_bytes), "sha256": sha256(locations_bytes), "enumeration_columns": ["Name", "FileName", "TileDataFileName"], "header_rows": 2},
            "files": [files[path] for path in input_paths],
            "location_rows": loaded,
        },
        "native_api_evidence": api_evidence,
        "steps": steps,
        "inspect": [{"address": ref("context"), "size": "0x10a8"}],
        "limitations": [
            "Plan generation only; no native execution or battle simulation was performed.",
            "The original location CSV is used only to enumerate row names and original resource paths; no map grid is synthesized.",
            "The map-cache vector at context+0xfd8 is verified through the original whole-context inspection; its count is a 32-bit field at context+0xfe4.",
        ],
    }
    out.parent.mkdir(parents=True, exist_ok=True)
    encoded = (json.dumps(plan, indent=2) + "\n").encode()
    out.write_bytes(encoded)
    summary = {
        "plan": str(out),
        "plan_sha256": sha256(encoded),
        "source_sha256": EXPECTED,
        "inherit_plan_sha256": parent_sha,
        "steps": len(steps),
        "locations": len(locations),
        "unique_original_csv_inputs": len(input_paths),
        "input_bytes": sum(asset["bytes"] for asset in files.values()),
        "all_nonempty_paths_resolved": True,
        "native_execution_performed": False,
    }
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=ROOT / "analysis/native-context/maps-v1/native-map-plan.json")
    parser.add_argument("--report", type=Path, default=ROOT / "analysis/native-context/maps-v1/generation-report.json")
    parser.add_argument("--parent-plan", type=Path, help="Exact completed resource plan containing the original typed postload")
    args = parser.parse_args()
    make(args.out, args.report,args.parent_plan)
