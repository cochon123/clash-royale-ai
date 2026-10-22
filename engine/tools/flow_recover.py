#!/usr/bin/env python3
"""Extract and index original embedded logic graphs without inventing handlers.

Only Python's standard library is required. JSON is copied byte for byte;
numeric operation types remain scoped to the original owning asset type.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import sqlite3


def digest(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n")


def safe_relative(path):
    rel = Path(path)
    if rel.is_absolute() or ".." in rel.parts:
        raise ValueError(f"Unsafe asset path: {path}")
    return rel


def walk(value, pointer=""):
    yield pointer, value
    if isinstance(value, dict):
        for key, child in value.items():
            escaped = str(key).replace("~", "~0").replace("/", "~1")
            yield from walk(child, pointer + "/" + escaped)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from walk(child, pointer + "/" + str(index))


def kind(value):
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        return "number"
    if isinstance(value, dict):
        return "object"
    if isinstance(value, list):
        return "array"
    return "string"


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--database", type=Path, default=Path("analysis/native-ui/assets.scdb"))
    ap.add_argument("--output", type=Path, default=Path("analysis/native-flow"))
    args = ap.parse_args()
    database_hash = digest(args.database.read_bytes())
    connection = sqlite3.connect(f"file:{args.database.resolve()}?mode=ro", uri=True)
    connection.execute("pragma query_only=on")
    rows = connection.execute("""select a.path,a.data,a.extref,a.format,
        m.assetType,m.uuid,m.data,m.format,m.shortid,m.parent
        from asset a join meta m using(path) order by a.path""").fetchall()
    known_paths = {row[0] for row in rows}
    logic_index, record_index, references, expressions, graph_index, flow_activity = [], [], [], [], [], []
    schemas = defaultdict(lambda: defaultdict(Counter))
    operations = defaultdict(lambda: {"count": 0, "fields": Counter(), "samples": []})
    components = defaultdict(lambda: {"count": 0, "fields": Counter(), "samples": []})
    native_classes = defaultdict(lambda: {"count": 0, "fields": Counter(), "samples": []})
    scans = {"asset": Counter(), "meta": Counter()}
    logic_types = Counter()
    logic_scopes = Counter()
    meta_variants = Counter()
    for path, data, external, fmt, asset_type, uuid, metadata, metadata_format, shortid, parent in rows:
        selected = asset_type.startswith("Logic") or asset_type in {"AssetASMData", "AssetAnimStatesRemapping", "AssetBindingPointsDefinition"}
        row_info = {"path": path, "type": asset_type, "asset_format": fmt, "meta_format": metadata_format, "external": external}
        parsed = None
        for table, blob in [("asset", data), ("meta", metadata)]:
            if blob is None:
                scans[table]["null"] += 1
                row_info[table + "_classification"] = "null"
                continue
            blob = bytes(blob)
            row_info[table + "_bytes"] = len(blob)
            row_info[table + "_sha256"] = digest(blob)
            if table == "meta":
                classification = "repeated_7_byte_descriptor" if blob == bytes.fromhex("00000100002401") else "binary_metadata"
                meta_variants[digest(blob)] += 1
                target = args.output / "metadata-blobs" / (digest(blob) + ".bin")
                if not target.exists():
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(blob)
            else:
                try:
                    parsed = json.loads(blob)
                    classification = "json"
                except UnicodeDecodeError:
                    classification = "non_utf8"
                except json.JSONDecodeError:
                    classification = "utf8_non_json"
                if classification != "json":
                    target = args.output / "database-exceptions" / safe_relative(path + ".blob")
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(blob)
            scans[table][classification] += 1
            row_info[table + "_classification"] = classification
        record_index.append(row_info)
        if not selected:
            continue
        entry = {**row_info, "uuid": uuid, "shortid": shortid, "parent": parent}
        logic_types[asset_type] += 1
        logic_scopes["autochess" if path.startswith("logic/autochess/") else "other"] += 1
        if data is not None:
            target = args.output / "assets" / safe_relative(path + (".json" if parsed is not None else ".bin"))
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        logic_index.append(entry)
        if parsed is None:
            continue
        graph = parsed.get("r", {}) if isinstance(parsed, dict) else {}
        if isinstance(graph, dict) and ("nodes" in graph or "states" in graph):
            graph_index.append({"path": path, "type": asset_type, "name": graph.get("name"),
                "nodes": [{"type": n.get("type", "<omitted>"), "id": n.get("obj", {}).get("id"), "name": n.get("obj", {}).get("name")} for n in graph.get("nodes", [])],
                "states": [{"type": n.get("type", "<omitted>"), "id": n.get("obj", {}).get("id"), "name": n.get("obj", {}).get("name")} for n in graph.get("states", [])]})
        if asset_type in {"LogicGameObjectFlowAsset", "LogicPlayerFlowAsset"}:
            nodes = graph.get("nodes", [])
            flow_activity.append({"path": path, "type": asset_type, "nodes": len(nodes),
                "node_actions": sum(len(n.get("obj", {}).get("node_actions", [])) for n in nodes),
                "single_terminated_node": len(nodes) == 1 and nodes[0].get("obj", {}).get("m_nextNodeID") == -1})
        for pointer, value in walk(parsed):
            # Field observations are evidence, not an official schema or default map.
            normalized = "/".join("*" if token.isdigit() else token for token in pointer.split("/"))
            schemas[asset_type][normalized][kind(value)] += 1
            if isinstance(value, str):
                candidate = value.split(" | ", 1)[0]
                if candidate in known_paths:
                    references.append({"source": path, "pointer": pointer, "target": candidate})
                if pointer.endswith("/m_raw") or pointer.rsplit("/", 1)[-1].endswith("Expression") or pointer.rsplit("/", 1)[-1].startswith("TargetExpr"):
                    expressions.append({"asset": path, "pointer": pointer, "expression": value})
            if not isinstance(value, dict):
                continue
            category, opcode, fields = None, None, None
            if "node_action_obj" in value:
                category, opcode, fields = "flow_action", value.get("node_action_type", "<omitted>"), value["node_action_obj"]
            elif "obj" in value and isinstance(value["obj"], dict):
                category = "fsm_callback" if any(token in pointer for token in ["/onEnter/", "/onExit/", "/onUpdate/"]) else "condition" if "/conditions/" in pointer else "graph_node_or_state"
                opcode, fields = value.get("type", "<omitted>"), value["obj"]
            if category:
                operation = operations[(asset_type, category, str(opcode))]
                operation["count"] += 1
                operation["fields"].update(fields.keys())
                if len(operation["samples"]) < 5:
                    operation["samples"].append({"asset": path, "pointer": pointer, "original": value})
            if "_c" in value:
                component = components[(asset_type, str(value["_c"]))]
                component["count"] += 1
                component["fields"].update(value.keys())
                if len(component["samples"]) < 3:
                    component["samples"].append({"asset": path, "pointer": pointer, "original": value})
            if "ClassType" in value:
                native_class = native_classes[(asset_type, str(value["ClassType"]))]
                native_class["count"] += 1
                native_class["fields"].update(value.keys())
                if len(native_class["samples"]) < 3:
                    native_class["samples"].append({"asset": path, "pointer": pointer, "original": value})
    write_json(args.output / "asset-index.json", logic_index)
    write_json(args.output / "database-record-index.json", record_index)
    write_json(args.output / "graph-index.json", graph_index)
    write_json(args.output / "flow-activity.json", flow_activity)
    write_json(args.output / "asset-references.json", references)
    write_json(args.output / "expressions.json", expressions)
    write_json(args.output / "operation-index.json", [{"asset_type": key[0], "category": key[1], "encoded_type": key[2], **value} for key, value in sorted(operations.items())])
    write_json(args.output / "component-index.json", [{"asset_type": key[0], "component": key[1], **value} for key, value in sorted(components.items())])
    write_json(args.output / "native-class-index.json", [{"asset_type": key[0], "encoded_class": key[1], **value} for key, value in sorted(native_classes.items())])
    for asset_type, fields in schemas.items():
        write_json(args.output / "observed-schemas" / (asset_type + ".json"), {"status": "observed JSON field types; not an official schema or inferred default values", "fields": fields})
    summary = {"database": str(args.database), "database_sha256": database_hash,
        "records_scanned": len(rows), "blob_classifications": scans,
        "selected_assets": len(logic_index), "selected_types": logic_types, "selected_scopes": logic_scopes,
        "graph_assets": len(graph_index), "expressions": len(expressions), "references": len(references),
        "unique_operation_shapes": len(operations), "unique_component_shapes": len(components),
        "named_native_classes": len(native_classes),
        "metadata_blob_variants": len(meta_variants),
        "execution": "none; original data only; native operation handlers unresolved"}
    write_json(args.output / "recovery-manifest.json", summary)
    if digest(args.database.read_bytes()) != database_hash:
        raise RuntimeError("Source database changed during recovery")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
