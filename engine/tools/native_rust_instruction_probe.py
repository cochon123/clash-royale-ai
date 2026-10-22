#!/usr/bin/env python3
"""Differentially probe generated single-instruction AArch64 translations.

The probe selects original non-memory/non-control-flow scalar integer, SIMD
integer, and FMOV instructions from profile-combined.json, then compares
the generated Rust state transition with one Unicorn execution of the pinned
original instruction bytes.
"""

import argparse
import hashlib
import importlib.util
import json
import random
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = 0x10000000
SOURCE_SHA256 = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"
DEFAULT_BUILD = ROOT / "analysis/native-port/instruction-probe"


def load_codegen():
    path = ROOT / "tools/native_rust_codegen.py"
    spec = importlib.util.spec_from_file_location("native_rust_codegen", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def family(mnemonic, op_str):
    if mnemonic == "fmov":
        return "fmov"
    if re.search(r"(?<![A-Za-z0-9_])v\d+", op_str):
        return "simd/integer"
    if mnemonic in {"mov", "movk", "movn", "movz", "adr", "adrp"}:
        return "move/address"
    if mnemonic in {"cmp", "cmn", "tst", "ccmp", "ccmn", "adds", "subs", "ands", "bics",
                    "add", "sub", "and", "orr", "eor", "bic", "orn", "eon", "neg", "negs",
                    "mvn", "mul", "madd", "msub", "mneg", "smull", "umull", "smaddl", "umaddl",
                    "smsubl", "umsubl", "smulh", "umulh", "sdiv", "udiv"}:
        return "integer/arithmetic"
    if mnemonic in {"lsl", "lsr", "asr", "ror", "lslv", "lsrv", "asrv", "rorv", "clz", "cls",
                    "rbit", "rev", "rev16", "rev32", "uxtb", "uxth", "uxtw", "sxtb", "sxth", "sxtw",
                    "ubfx", "sbfx", "ubfiz", "sbfiz", "bfi", "bfxil", "csel", "csinc", "csinv",
                    "csneg", "cset", "csetm", "cinc", "cinv", "cneg"}:
        return "integer/bitops"
    if mnemonic in {"and", "orr", "eor", "bic", "orn", "bsl", "bit", "bif", "not", "mvn", "movi",
                    "mvni", "dup", "add", "sub", "mul", "cmeq", "cmgt", "cmge", "cmhi", "cmhs",
                    "sabd", "uabd", "cnt", "ushr", "sshr", "shl", "addv", "uaddlv", "saddlv", "smaxv",
                    "umaxv", "uzp1", "uzp2", "zip1", "zip2", "trn1", "trn2", "xtn", "shrn", "ushll",
                    "sshll", "shll", "ext", "umov", "smov"}:
        return "simd/integer"
    return None


def pick_cases(codegen, limit):
    from capstone import Cs, CS_ARCH_ARM64, CS_MODE_ARM

    profile_path = ROOT / "analysis/native-port/profile-combined.json"
    profile = json.loads(profile_path.read_text())
    if profile.get("source_sha256") != SOURCE_SHA256:
        raise ValueError("profile is not bound to the pinned original source")
    source = (ROOT / "analysis/libg-cipher/libg.decrypted.so").read_bytes()
    if hashlib.sha256(source).hexdigest() != SOURCE_SHA256:
        raise ValueError("original source SHA-256 mismatch")
    md = Cs(CS_ARCH_ARM64, CS_MODE_ARM)
    md.detail = True
    queues = {}
    seen = set()
    compiler = codegen.Compiler({})
    for block in profile["blocks"]:
        rva = int(block["rva"], 0)
        size = int(block["bytes"])
        if rva < 0 or rva + size > len(source):
            continue
        for ins in md.disasm(source[rva:rva + size], BASE + rva):
            cat = family(ins.mnemonic, ins.op_str)
            if cat is None:
                continue
            signature = (ins.mnemonic, ins.op_str)
            if signature in seen:
                continue
            seen.add(signature)
            try:
                code, terminates = compiler.emit(ins, ins.address, ins.address + 4)
                if terminates:
                    continue
            except (KeyError, ValueError, IndexError, TypeError):
                continue
            queues.setdefault((cat, ins.mnemonic), []).append({
                "id": -1, "rva": ins.address - BASE, "address": ins.address,
                "bytes": bytes(ins.bytes).hex(), "mnemonic": ins.mnemonic,
                "op_str": ins.op_str, "family": cat, "code": code,
            })
    chosen = []
    names = sorted(queues)
    while len(chosen) < limit and any(queues.values()):
        for name in names:
            if queues[name] and len(chosen) < limit:
                chosen.append(queues[name].pop(0))
    for idx, case in enumerate(chosen):
        case["id"] = idx
    if not chosen:
        raise ValueError("no translatable probe instructions found")
    return chosen, len(seen)


def rust_project(build_dir, cases):
    src = build_dir / "src"
    src.mkdir(parents=True, exist_ok=True)
    (build_dir / "Cargo.toml").write_text(
        '[package]\nname="instruction-probe"\nversion="0.1.0"\nedition="2021"\n'
        '[dependencies]\nserde_json="1"\n'
    )
    (src / "cpu.rs").write_text((ROOT / "rust-engine/src/cpu.rs").read_text())
    (src / "helpers.rs").write_text((ROOT / "rust-engine/src/generated/helpers.rs").read_text())
    arms = "\n".join(f"{c['id']} => {{{c['code']} Ok(())}}," for c in cases)
    main = r'''mod cpu;
mod helpers;
use cpu::Cpu;
use helpers::*;
use serde_json::{json, Value};
use std::io::{self, BufRead};
fn run(cpu: &mut Cpu, case: usize) -> Result<(), String> {
    match case {
        CASE_ARMS
        _ => Err(format!("unknown probe case {case}")),
    }
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        let mut cpu = Cpu::default();
        for (i, x) in v["x"].as_array().unwrap().iter().enumerate() { cpu.x[i] = x.as_u64().unwrap(); }
        for (i, q) in v["q"].as_array().unwrap().iter().enumerate() {
            cpu.q[i] = u128::from_str_radix(q.as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
        }
        cpu.sp = v["sp"].as_u64().unwrap();
        cpu.nzcv = v["nzcv"].as_u64().unwrap();
        if let Err(e) = run(&mut cpu, v["case"].as_u64().unwrap() as usize) {
            println!("{}", json!({"error": e})); continue;
        }
        println!("{}", json!({
            "x": cpu.x.iter().map(|x| format!("{x:#x}")).collect::<Vec<_>>(),
            "q": cpu.q.iter().map(|x| format!("{x:#x}")).collect::<Vec<_>>(),
            "nzcv": format!("{:#x}", cpu.nzcv), "sp": format!("{:#x}", cpu.sp)
        }));
    }
}
'''.replace("CASE_ARMS", arms)
    (src / "main.rs").write_text(main)


def state_cases(case, samples, seed):
    rng = random.Random(seed)
    palette = [0, (1 << 64) - 1, 0xAAAAAAAAAAAAAAAA, 0x5555555555555555,
               1 << 63, 1 << 31, 0x7FFFFFFFFFFFFFFF, 0x80000000]
    states = []
    # Deterministic mixed-edge states exercise width, extension, and carry paths.
    for index in range(samples):
        x = [rng.getrandbits(64) for _ in range(31)]
        q = [rng.getrandbits(128) for _ in range(32)]
        if index < len(palette):
            x[index % 31] = palette[index]
            x[(index * 7 + 3) % 31] = palette[(index + 3) % len(palette)]
            q[index % 32] = (palette[index] << 64) | palette[(index + 1) % len(palette)]
        flags = ((index * 7) % 16) << 28
        states.append({"case": case["id"], "x": x,
                       "q": [f"0x{value:032x}" for value in q],
                       "sp": rng.getrandbits(64) & ~15, "nzcv": flags})
    # Ensure all 16 condition inputs are observed across conditional selects.
    if case["mnemonic"] in {"csel", "csinc", "csinv", "csneg", "cset", "csetm", "cinc", "cinv", "cneg"}:
        base = states[0]
        for flags in range(16):
            item = dict(base); item["nzcv"] = flags << 28
            states.append(item)
    return states


def unicorn_registers():
    from unicorn.arm64_const import (UC_ARM64_REG_PC, UC_ARM64_REG_SP, UC_ARM64_REG_NZCV,
                                     UC_ARM64_REG_FPCR, UC_ARM64_REG_FPSR)
    from unicorn.arm64_const import __dict__ as constants
    ids = {"x": [constants[f"UC_ARM64_REG_X{i}"] for i in range(31)],
           "q": [constants[f"UC_ARM64_REG_Q{i}"] for i in range(32)],
           "sp": UC_ARM64_REG_SP, "pc": UC_ARM64_REG_PC, "nzcv": UC_ARM64_REG_NZCV,
           "fpcr": UC_ARM64_REG_FPCR, "fpsr": UC_ARM64_REG_FPSR}
    ids["x30"] = constants["UC_ARM64_REG_X30"]
    return ids


def oracle_step(uc, ids, case, state):
    for i, value in enumerate(state["x"]):
        uc.reg_write(ids["x"][i], value)
    for i, value in enumerate(state["q"]):
        uc.reg_write(ids["q"][i], int(value, 16))
    uc.reg_write(ids["sp"], state["sp"])
    uc.reg_write(ids["nzcv"], state["nzcv"])
    uc.reg_write(ids["fpcr"], 0)
    uc.reg_write(ids["fpsr"], 0)
    uc.reg_write(ids["pc"], case["address"])
    uc.emu_start(case["address"], 0, count=1)
    return {
        "x": [f"{uc.reg_read(reg):#x}" for reg in ids["x"]],
        "q": [f"{uc.reg_read(reg):#x}" for reg in ids["q"]],
        "nzcv": f"{uc.reg_read(ids['nzcv']):#x}",
        "sp": f"{uc.reg_read(ids['sp']):#x}",
    }


def run(args):
    import unicorn
    from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_PROT_ALL

    codegen = load_codegen()
    cases, unique_signatures = pick_cases(codegen, args.limit)
    rust_project(args.build_dir, cases)
    build = subprocess.run(["cargo", "build", "--release", "--offline"], cwd=args.build_dir,
                           text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if build.returncode:
        raise RuntimeError("probe Rust harness did not compile:\n" + build.stderr[-12000:])
    binary = args.build_dir / "target/release/instruction-probe"

    source_path = ROOT / "analysis/libg-cipher/libg.decrypted.so"
    source = source_path.read_bytes()
    uc = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
    map_size = (len(source) + 0xFFF) & ~0xFFF
    uc.mem_map(BASE, map_size, UC_PROT_ALL)
    uc.mem_write(BASE, source)
    ids = unicorn_registers()

    tests = []
    for case in cases:
        for state in state_cases(case, args.samples, args.seed + case["id"]):
            tests.append((case, state))
    payload = "".join(json.dumps(state) + "\n" for _, state in tests)
    proc = subprocess.run([str(binary)], input=payload, text=True,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode:
        raise RuntimeError("probe Rust harness failed:\n" + proc.stderr[-12000:])
    outputs = [json.loads(line) for line in proc.stdout.splitlines()]
    if len(outputs) != len(tests):
        raise RuntimeError(f"Rust harness returned {len(outputs)} rows for {len(tests)} inputs")

    mismatches = []
    mismatch_count = 0
    for row, (case, state) in zip(outputs, tests):
        if "error" in row:
            mismatch_count += 1
            if len(mismatches) < args.max_differences:
                mismatches.append({"case": case["id"], "instruction": f"{case['mnemonic']} {case['op_str']}",
                                   "error": row["error"]})
            continue
        try:
            expected = oracle_step(uc, ids, case, state)
        except Exception as error:
            raise RuntimeError(f"Unicorn failed at {case['address']:#x} {case['mnemonic']} {case['op_str']}: {error}")
        diffs = {}
        for register_set in ("x", "q"):
            names = (lambda i: f"x{i}") if register_set == "x" else (lambda i: f"q{i}")
            for i, (want, got) in enumerate(zip(expected[register_set], row[register_set])):
                if int(want, 16) != int(got, 16):
                    diffs[names(i)] = {"unicorn": want, "rust": got}
        for name in ("nzcv", "sp"):
            if int(expected[name], 16) != int(row[name], 16):
                diffs[name] = {"unicorn": expected[name], "rust": row[name]}
        if diffs:
            mismatch_count += 1
            if len(mismatches) < args.max_differences:
                mismatches.append({"case": case["id"], "rva": hex(case["rva"]),
                                   "instruction": f"{case['mnemonic']} {case['op_str']}",
                                   "bytes": case["bytes"], "differences": diffs})

    report = {
        "source_sha256": SOURCE_SHA256,
        "profile": "analysis/native-port/profile-combined.json",
        "unicorn_version": unicorn.__version__,
        "selected_signatures": len(cases), "eligible_unique_signatures_seen": unique_signatures,
        "selected_by_mnemonic": {name: sum(c["mnemonic"] == name for c in cases)
                                 for name in sorted({c["mnemonic"] for c in cases})},
        "selected_by_family": {name: sum(c["family"] == name for c in cases)
                                for name in sorted({c["family"] for c in cases})},
        "tested_transitions": len(tests), "samples_per_signature": args.samples,
        "match": mismatch_count == 0, "mismatch_count": mismatch_count,
        "mismatch_count_reported": len(mismatches),
        "differences": mismatches,
        "scope": "Single original instruction transitions for selected generated scalar integer, SIMD integer, move/address and FMOV cases. Memory, control-flow, FP arithmetic and system instructions are excluded. Compares X0-X30, Q0-Q31, NZCV and SP.",
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["match"] else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--limit", type=int, default=240, help="maximum unique instruction signatures")
    parser.add_argument("--samples", type=int, default=8, help="edge/random states per signature")
    parser.add_argument("--seed", type=int, default=0xA64)
    parser.add_argument("--max-differences", type=int, default=100)
    parser.add_argument("--build-dir", type=Path, default=DEFAULT_BUILD)
    parser.add_argument("--out", type=Path, default=ROOT / "analysis/native-port/instruction-probe/report.json")
    args = parser.parse_args()
    try:
        return run(args)
    except Exception as error:
        print(f"native_rust_instruction_probe: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
