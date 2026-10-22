#!/usr/bin/env python3
"""Generate checkpoint-bound table preparation and final resource plans.

This reads a successful native post-attachment checkpoint, selects the actual
nonnull context table slots, and links the original table preparation, text
bootstrap, and typed postload plans. It never executes native code.
"""
import argparse
import csv
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"
PREFAB_REF = 0x562E51D8
SLOT_COUNT = 223
CONTEXT_SIZE = 0x10A8


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def ref(name, offset=0):
    return {"ref": name, **({"offset": hex(offset)} if offset else {})}


def load_json(path):
    return json.loads(path.read_bytes())


def fde_map():
    result = {}
    path = ROOT / "analysis/native-engine-index/unwind-functions.tsv"
    with path.open(newline="") as stream:
        for row in csv.DictReader(stream, delimiter="\t"):
            result[int(row["start"], 16)] = int(row["size"])
    return result


def checked_fde_evidence(source, rvas):
    fdes = fde_map()
    rows = []
    asm_paths = {
        0xE6DF50: "analysis/native-focused/resources/attach-caller/rva_e6df50.asm",
        0xE12548: "analysis/native-focused/table-prepare-contract/rva_e12548.asm",
    }
    for rva, meaning in rvas:
        if rva not in fdes:
            raise ValueError(f"No exact original FDE begins at {rva:#x}")
        row = {
            "rva": hex(rva), "exact_FDE_size": fdes[rva],
            "source_file_bytes_sha256": sha256(source[rva:rva + fdes[rva]]),
            "meaning": meaning,
        }
        asm = ROOT / asm_paths[rva] if rva in asm_paths else None
        if asm is not None and asm.is_file():
            row["assembly_evidence"] = str(asm.relative_to(ROOT))
            row["assembly_sha256"] = sha256(asm.read_bytes())
        rows.append(row)
    # The 8-byte getter disassembly was inspected in a scratch analysis; retain
    # the instruction sequence while the source-FDE hash binds the exact bytes.
    rows[-1]["instruction_evidence"] = "LDR X0,[X0,W1,SXTW #3]; RET (RVA 0xe16128)."
    return rows


def checkpoint_inputs(checkpoint_arg):
    checkpoint_path = checkpoint_arg / "checkpoint.json" if checkpoint_arg.is_dir() else checkpoint_arg
    checkpoint_bytes = checkpoint_path.read_bytes()
    checkpoint = json.loads(checkpoint_bytes)
    if checkpoint.get("source_sha256") != EXPECTED:
        raise ValueError("Checkpoint source ELF identity mismatch")
    report_path = Path(checkpoint["report_path"])
    report_bytes = report_path.read_bytes()
    report = json.loads(report_bytes)
    if report.get("source_sha256") != EXPECTED:
        raise ValueError("Checkpoint report source ELF identity mismatch")
    if checkpoint.get("returned") is not True or checkpoint.get("failure") not in (None, ""):
        raise ValueError("Native checkpoint did not return successfully")
    if report.get("returned") is not True or report.get("failure") not in (None, ""):
        raise ValueError("Native report records a failure or nonreturn")
    if report.get("phase") != "game_call_plan":
        raise ValueError("Expected a completed native game call-plan report")
    init = checkpoint.get("init_array") or {}
    if init.get("completed") != init.get("count") or init.get("in_progress") is not None:
        raise ValueError("Original ELF init array did not complete")

    call_state = checkpoint.get("call_plan") or {}
    report_call = report.get("call_plan") or {}
    plan_path = Path(call_state.get("source", ""))
    plan_bytes = plan_path.read_bytes()
    plan = json.loads(plan_bytes)
    plan_sha = sha256(plan_bytes)
    if plan_sha != call_state.get("sha256") or report_call.get("sha256") != plan_sha:
        raise ValueError("Checkpoint/report call-plan identity mismatch")
    if plan.get("source_sha256") != EXPECTED:
        raise ValueError("Post-attachment plan source ELF identity mismatch")
    nsteps = len(plan.get("steps", []))
    if nsteps != 2 or [s.get("rva", "").lower() for s in plan["steps"]] != ["0xe6e3ec", "0xe16858"]:
        raise ValueError("Expected exactly the original post-attachment and dependency-build calls")
    for state in (call_state, report_call):
        if state.get("completed") != nsteps or state.get("in_progress") is not None:
            raise ValueError("Post-attachment call plan is incomplete or interrupted")
        if len(state.get("steps", [])) != nsteps or any(s.get("returned") is not True for s in state["steps"]):
            raise ValueError("A native post-attachment call did not return")
    values = call_state.get("values", {})
    report_values = report_call.get("values", {})
    context = values.get("context")
    if not isinstance(context, int) or not context or report_values.get("context") != context:
        raise ValueError("Post-attachment context pointer is missing or inconsistent")
    prefab = values.get("prefab_ref")
    if prefab != PREFAB_REF or report_values.get("prefab_ref") != PREFAB_REF:
        raise ValueError("Original prefab lookup did not resolve to the verified native object")

    heap = next((r for r in checkpoint.get("regions", []) if r.get("file") == "heap.bin"), None)
    if heap is None:
        raise ValueError("Checkpoint has no native heap region")
    heap_path = checkpoint_path.parent / heap["file"]
    heap_bytes = heap_path.read_bytes()
    heap_sha = sha256(heap_bytes)
    if heap_sha != heap.get("sha256"):
        raise ValueError("Checkpoint heap region hash mismatch")
    heap_base = int(heap["base"], 0)
    heap_end = heap_base + len(heap_bytes)
    if not (heap_base <= context and context + CONTEXT_SIZE <= heap_end):
        raise ValueError("Context pointer is outside the verified heap region")

    regions = []
    for region in checkpoint.get("regions", []):
        start = int(region["base"], 0)
        size = int(region["size"])
        if region.get("file") == "heap.bin":
            blob = heap_bytes
        else:
            region_path = checkpoint_path.parent / region["file"]
            blob = region_path.read_bytes()
        if len(blob) < size or sha256(blob[:size]) != region.get("sha256"):
            raise ValueError(f"Checkpoint memory region failed identity check: {region['file']}")
        regions.append((start, start + size, region["file"], blob))

    def read_qword(address):
        for start, end, name, blob in regions:
            if start <= address and address + 8 <= end:
                return struct.unpack_from("<Q", blob, address - start)[0]
        raise ValueError(f"Context slot address {address:#x} is outside checkpoint regions")

    slots = []
    for table_id in range(SLOT_COUNT):
        pointer = read_qword(context + table_id * 8)
        if pointer:
            if not any(start <= pointer < end for start, end, _, _ in regions):
                raise ValueError(f"Non-null context slot {table_id} points outside checkpoint memory")
            slots.append({"id": table_id, "pointer": pointer})
    return {
        "checkpoint_path": checkpoint_path.resolve(), "checkpoint_bytes": checkpoint_bytes,
        "checkpoint": checkpoint, "report_path": report_path.resolve(),
        "report_bytes": report_bytes, "report": report, "plan_path": plan_path.resolve(),
        "plan_bytes": plan_bytes, "plan": plan, "plan_sha256": plan_sha,
        "context": context, "prefab_ref": prefab, "heap": heap,
        "heap_path": heap_path.resolve(), "heap_bytes": heap_bytes,
        "heap_sha256": heap_sha, "slots": slots,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkpoint", required=True, type=Path,
                        help="Successful checkpoint.json path or its containing state directory")
    parser.add_argument("--out", required=True, type=Path,
                        help="New output directory for preparation, language, and typed plans")
    args = parser.parse_args()

    source_path = ROOT / "analysis/libg-cipher/libg.decrypted.so"
    source = source_path.read_bytes()
    if sha256(source) != EXPECTED:
        raise ValueError("Original source ELF identity mismatch")
    checkpoint = checkpoint_inputs(args.checkpoint)

    template_dir = ROOT / "analysis/native-context/bootstrap-v4"
    language_path = template_dir / "language-plan.json"
    typed_path = template_dir / "resource-typed-plan.json"
    language_template = load_json(language_path)
    typed_template = load_json(typed_path)
    if language_template.get("source_sha256") != EXPECTED or len(language_template.get("steps", [])) != 9:
        raise ValueError("Original language template identity or nine-step contract mismatch")
    text_step = next((s for s in language_template["steps"] if s.get("name") == "texts_original_bytes"), None)
    if text_step is None or "file" not in text_step:
        raise ValueError("Language template has no source-bounded original texts input")
    texts = Path(text_step["file"]).read_bytes()
    if sha256(texts) != text_step.get("sha256") or len(texts) < 10_000_000:
        raise ValueError("Original multilingual texts input hash/size mismatch")
    if typed_template.get("source_sha256") != EXPECTED or len(typed_template.get("steps", [])) != 9:
        raise ValueError("Original typed template identity or nine-step contract mismatch")
    if len(typed_template.get("inspect", [])) != 3:
        raise ValueError("Typed template inspection contract changed")

    evidence = checked_fde_evidence(source, [
        (0xE6DF50, "Original e6df50 iterates native type IDs through the table-slot accessor and calls original e12548 on each nonnull table pointer."),
        (0xE12548, "Original table preparation routine takes the native table object pointer in X0."),
        (0xE16128, "Original table-slot accessor reads the qword at context + 8*ID."),
    ])
    steps = []
    for slot in checkpoint["slots"]:
        table_id = slot["id"]
        slot_name = f"context_table_slot_{table_id:03d}"
        steps.append({"name": slot_name, "read_u64": ref("context", table_id * 8)})
        steps.append({
            "name": f"prepare_original_table_{table_id:03d}",
            "rva": "0xe12548", "args": [ref(slot_name)],
        })
    prepare = {
        "source_sha256": EXPECTED,
        "inherit_plan_sha256": checkpoint["plan_sha256"],
        "purpose": "Prepare every nonnull original native table slot present in the completed post-attachment checkpoint using the original e12548 routine.",
        "parent_checkpoint": {
            "checkpoint": str(checkpoint["checkpoint_path"]),
            "checkpoint_sha256": sha256(checkpoint["checkpoint_bytes"]),
            "report": str(checkpoint["report_path"]),
            "report_sha256": sha256(checkpoint["report_bytes"]),
            "call_plan": str(checkpoint["plan_path"]),
            "call_plan_sha256": checkpoint["plan_sha256"],
            "call_plan_completed": checkpoint["checkpoint"]["call_plan"]["completed"],
            "prefab_ref": hex(checkpoint["prefab_ref"]),
        },
        "checkpoint_context": {
            "context_pointer": hex(checkpoint["context"]),
            "context_bytes": CONTEXT_SIZE,
            "slot_count": SLOT_COUNT,
            "slot_ids_considered": [0, SLOT_COUNT - 1],
            "present_slots": checkpoint["slots"],
            "heap_file": str(checkpoint["heap_path"]),
            "heap_base": checkpoint["heap"]["base"],
            "heap_size": checkpoint["heap"]["size"],
            "heap_sha256": checkpoint["heap_sha256"],
        },
        "native_api_evidence": evidence,
        "steps": steps,
        "inspect": [{"address": ref("context"), "size": hex(CONTEXT_SIZE)}],
        "limitations": [
            "Plan generation only; this script does not execute native code.",
            "The selected IDs and pointers are read from the hash-verified checkpoint snapshot; no table pointer or row is synthesized.",
        ],
    }
    prepare_bytes = (json.dumps(prepare, indent=2) + "\n").encode()
    prepare_sha = sha256(prepare_bytes)

    language = dict(language_template)
    language.update({
        "inherit_plan_sha256": prepare_sha,
        "parent_plan": {"path": "table-prepare-plan.json", "sha256": prepare_sha},
        "generator_sha256": sha256(Path(__file__).read_bytes()),
    })
    language_bytes = (json.dumps(language, indent=2) + "\n").encode()
    language_sha = sha256(language_bytes)

    typed = dict(typed_template)
    typed.update({
        "inherit_plan_sha256": language_sha,
        "parent_plan": {"path": "language-plan.json", "sha256": language_sha},
        "generator_sha256": sha256(Path(__file__).read_bytes()),
    })
    typed_bytes = (json.dumps(typed, indent=2) + "\n").encode()
    typed_sha = sha256(typed_bytes)

    outputs = {
        "table-prepare-plan.json": prepare_bytes,
        "language-plan.json": language_bytes,
        "resource-typed-plan.json": typed_bytes,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    existing = [name for name in outputs if (args.out / name).exists()]
    if existing:
        raise FileExistsError("Refusing to overwrite retained plans: " + ", ".join(existing))
    for name, payload in outputs.items():
        with (args.out / name).open("xb") as stream:
            stream.write(payload)
    result = {
        "source_sha256": EXPECTED,
        "checkpoint": str(checkpoint["checkpoint_path"]),
        "checkpoint_call_plan_sha256": checkpoint["plan_sha256"],
        "context": hex(checkpoint["context"]), "prefab_ref": hex(checkpoint["prefab_ref"]),
        "nonnull_table_slots": len(checkpoint["slots"]),
        "texts_bytes": len(texts), "texts_sha256": text_step["sha256"],
        "plans": [
            {"path": name, "sha256": sha256(payload), "steps": len(json.loads(payload)["steps"])}
            for name, payload in outputs.items()
        ],
        "native_execution_performed": False,
    }
    with (args.out / "generation-report.json").open("xb") as stream:
        stream.write((json.dumps(result, indent=2) + "\n").encode())
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
