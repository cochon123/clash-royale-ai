#!/usr/bin/env python3
"""Generate a source-bound plan for native dynamic TOML assets in data_manifest.toml.

This produces call-plan instructions only. Original TOML bytes are parsed and
attached by the recovered native functions; no engine rows are synthesized.
"""
import argparse
import csv
import hashlib
import json
import tomllib
from pathlib import Path, PurePosixPath

from native_engine_index import ROOT, EXPECTED

INHERIT_SHA256 = "e081976b50fee5a7ebe77ae8ef1920204f34f95bfc547db6a1c72e948de4f1ea"
MANIFEST_REL = Path("analysis/decoded/assets/data_manifest.toml")
CATALOG_REL = Path("analysis/native-focused/resources/catalog-descriptors.json")
ELF_REL = Path("analysis/libg-cipher/libg.decrypted.so")
FDE_REL = Path("analysis/native-engine-index/unwind-functions.tsv")


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def make(out: Path) -> dict:
    if out.exists():
        raise ValueError(f"Refusing to overwrite existing output: {out}")
    elf_path = ROOT / ELF_REL
    elf = elf_path.read_bytes()
    if sha(elf) != EXPECTED:
        raise ValueError("Recovered native ELF digest mismatch")
    source = ROOT / MANIFEST_REL
    manifest_bytes = source.read_bytes()
    manifest = tomllib.loads(manifest_bytes.decode("utf-8"))
    rows = []
    for key, values in manifest.items():
        path = values.get("Path")
        if path is None:
            continue
        rel = PurePosixPath(path)
        if rel.is_absolute() or ".." in rel.parts or rel.suffix.lower() != ".toml":
            raise ValueError(f"Unsupported manifest path for {key}: {path!r}")
        file = ROOT / "analysis/decoded/assets" / Path(*rel.parts)
        if not file.is_file():
            raise ValueError(f"Missing original decoded manifest asset: {path}")
        data = file.read_bytes()
        rows.append({"key": key, "path": str(rel), "file": file.resolve(),
                     "bytes": len(data), "sha256": sha(data)})
    if len(rows) != 261:
        raise ValueError(f"Unexpected path-bearing manifest row count: {len(rows)}")
    if sum(r["path"] == "csv_logic/characters/global_lightning.toml" for r in rows) != 1:
        raise ValueError("Global Lightning manifest record missing or duplicated")

    fdes = {int(r["start"], 16): int(r["end_exclusive"], 16)
            for r in csv.DictReader((ROOT / FDE_REL).open(), delimiter="\t")}
    required = [0x142FDE4, 0x142FDC4, 0x12A6E64, 0x128F5C0,
                0xE6E280, 0xE6E3EC, 0xE16858, 0xE12548,
                0xE16984, 0xE1B7E8, 0xE191C4]
    if any(rva not in fdes for rva in required):
        raise ValueError("Missing exact FDE for a required original native API")
    evidence = []
    for rva in required:
        end = fdes[rva]
        evidence.append({"rva": hex(rva), "exact_FDE_end": hex(end),
                         "exact_FDE_size": end-rva,
                         "source_file_bytes_sha256": sha(elf[rva:end])})
    if elf[0xE16984:0xE1698C] != bytes.fromhex("2100805201000014"):
        raise ValueError("Typed-postload wrapper bytes differ from verified MOV/B source")

    steps = []

    def ref(name, offset=0):
        result = {"ref": name}
        if offset:
            result["offset"] = hex(offset)
        return result

    def buf(name, data=None, file=None, size=None):
        step = {"name": name}
        if data is not None:
            step["data_hex"] = data.hex()
        if file is not None:
            step.update(file=str(file), sha256=next(r["sha256"] for r in rows if r["file"] == file))
            step["nul_terminate"] = True
        if size is not None:
            step["size"] = hex(size)
        steps.append(step)
        return ref(name)

    def call(name, rva, args, nonnull=False):
        step = {"name": name, "rva": hex(rva), "args": args}
        if nonnull:
            step["expect_nonzero"] = True
        steps.append(step)
        return ref(name)

    for index, row in enumerate(rows):
        prefix = f"dynamic_{index:03d}"
        name_bytes = buf(prefix + "_path_bytes", data=row["path"].encode("utf-8") + b"\0")
        name = buf(prefix + "_path_string", size=16)
        call(prefix + "_construct_path_string", 0x142FDE4, [name, name_bytes])
        raw = buf(prefix + "_original_toml_bytes", file=row["file"])
        text = buf(prefix + "_text_string", size=16)
        call(prefix + "_construct_text", 0x142FDC4, [text, raw, row["bytes"]])
        parsed = call(prefix + "_parse_original_toml", 0x12A6E64, [text, 0], True)
        root = call(prefix + "_require_original_toml_object", 0x128F5C0, [parsed, 1], True)
        call(prefix + "_native_attach_manifest_toml", 0xE6E280,
             [ref("context"), root, name])

    # Re-run the original context dependency/postload sequence after these
    # table mutations; the inherited bulk checkpoint prepared tables too early.
    call("native_post_dynamic_attach_stage", 0xE6E3EC, [ref("context")])
    call("native_rebuild_dynamic_dependency_tables", 0xE16858, [ref("context")])
    catalog_path = ROOT / CATALOG_REL
    catalog = json.loads(catalog_path.read_text())
    primary = {int(r["table_id"]) for r in catalog["records"]
               if r["flags"] in (0, 5)}
    # e6e3ec creates 212..214; e6e280/e14bac also create 221 and 222
    # from this build's manifest. The native outer loader prepares every
    # nonnull context table, including these non-catalog tables.
    for table_id in sorted(primary | {6, 7, 49, 97, 107, 169, 212, 213, 214, 221, 222}):
        # Resolve each original table pointer from its context slot.
        table_name = f"dynamic_pre_finalize_table_{table_id}"
        steps.append({"name": table_name,
                      "read_u64": ref("context", table_id * 8)})
        call(f"native_reprepare_dynamic_table_{table_id}", 0xE12548,
             [ref(table_name)])
    call("native_complete_dynamic_typed_postload", 0xE16984, [ref("context")])
    for lookup_name, rva in (("Normal", 0xE1B7E8), ("Friendly", 0xE191C4)):
        raw = buf(f"dynamic_lookup_{lookup_name}_bytes", data=lookup_name.encode() + b"\0")
        string = buf(f"dynamic_lookup_{lookup_name}_string", size=16)
        call(f"dynamic_construct_lookup_{lookup_name}", 0x142FDE4, [string, raw])
        call(f"dynamic_lookup_{lookup_name}", rva,
             [ref("context"), string, 0], True)

    plan = {
        "source_sha256": EXPECTED,
        "inherit_plan_sha256": INHERIT_SHA256,
        "purpose": "Native parsing and attachment of every original data_manifest.toml TOML path, followed by original context dependency rebuild and typed postload",
        "input_manifest": {
            "path": str(source.resolve()), "sha256": sha(manifest_bytes),
            "path_bearing_records": len(rows), "unique_paths": len({r["path"] for r in rows}),
            "record_order": "TOML document order; repeated path records are retained"
        },
        "native_api_evidence": evidence,
        "dynamic_assets": [{k: (str(v) if isinstance(v, Path) else v) for k, v in row.items()}
                            for row in rows],
        "input_adapter": "The manifest only enumerates original asset paths. Every decoded original TOML byte stream is passed through the original native string constructor, TOML parser, object validator, and e6e280 category/table attachment routine. No rows or typed engine objects are synthesized.",
        "steps": steps,
        "inspect": [
            {"address": ref("context"), "size": "0x10a8"},
            {"address": ref("dynamic_lookup_Normal"), "size": "0xa0"},
            {"address": ref("dynamic_lookup_Friendly"), "size": "0x100"}
        ],
        "limitations": [
            "Run only from a completed checkpoint whose call-plan SHA is the inherited bulk plan SHA.",
            "The native data_manifest loader and original resource-provider callbacks are not re-executed by this plan; the exact manifest-derived original paths are source-bound as plan inputs.",
            "Native completion, successful typed lookups, and inspection of the resulting context are required before claiming dynamic assets are loaded."
        ]
    }
    out.parent.mkdir(parents=True, exist_ok=True)
    encoded = (json.dumps(plan, indent=2) + "\n").encode()
    out.write_bytes(encoded)
    report = {
        "plan": str(out.resolve()), "plan_sha256": sha(encoded),
        "source_sha256": EXPECTED, "inherit_plan_sha256": INHERIT_SHA256,
        "manifest_sha256": sha(manifest_bytes), "manifest_records": len(rows),
        "unique_paths": len({r["path"] for r in rows}),
        "asset_bytes": sum(r["bytes"] for r in rows),
        "steps": len(steps), "native_code_executed": False,
        "global_lightning": next({"path": r["path"], "sha256": r["sha256"], "bytes": r["bytes"]}
                                  for r in rows if r["path"].endswith("global_lightning.toml"))
    }
    report_path = out.parent / "generation-report.json"
    report_path.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return report


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path,
                        default=ROOT / "analysis/native-context/dynamic-v1/dynamic-native-assets-plan.json")
    args = parser.parse_args()
    make(args.out)
