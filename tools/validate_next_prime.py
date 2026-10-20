#!/usr/bin/env python3
"""Compare the generated runtime helper against the pinned original ARM code."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM
import unicorn.arm64_const as arm

ROOT = Path(__file__).resolve().parents[1]
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "rust-engine/target/release/examples/next_prime_probe")
    parser.add_argument("--output", type=Path, default=ROOT / "analysis/cpu-gpu-tuning/next-prime-validation.json")
    args = parser.parse_args()
    checkpoint = json.loads((CHECKPOINT / "checkpoint.json").read_text())
    region = next(r for r in checkpoint["regions"] if r["file"] == "cxx_runtime.bin")
    raw = (CHECKPOINT / region["file"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == region["sha256"]
    base = int(region["base"], 0)
    uc = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
    uc.mem_map(base, (len(raw) + 4095) & ~4095)
    uc.mem_write(base, raw)
    uc.mem_map(0x70000000, 0x400000)
    uc.mem_map(0x90000000, 0x1000)
    registers = [getattr(arm, f"UC_ARM64_REG_X{i}") for i in range(31)]
    for i, register in enumerate(registers):
        uc.reg_write(register, int(checkpoint["registers"][f"x{i}"], 0))
    uc.reg_write(arm.UC_ARM64_REG_NZCV, int(checkpoint["registers"]["nzcv"], 0))
    values = list(range(0, 600)) + [1000, 1024, 4096, 10000, 65536, 1000000]
    result = subprocess.run([str(args.binary), str(CHECKPOINT), *map(str, values)],
                            text=True, capture_output=True, check=True)
    rows = json.loads(result.stdout)
    differences = []
    for value, row in zip(values, rows):
        for register in registers[:9]:
            uc.reg_write(register, 0)
        uc.reg_write(arm.UC_ARM64_REG_X0, value)
        uc.reg_write(arm.UC_ARM64_REG_X30, 0x90000000)
        uc.reg_write(arm.UC_ARM64_REG_SP, checkpoint["emulation_abi"]["call_stack_pointer"])
        uc.emu_start(base + 0xCFFBC, 0x90000000, count=1000000)
        assert uc.reg_read(arm.UC_ARM64_REG_PC) == 0x90000000, "reference failed to return"
        actual = {"input": value, "result": uc.reg_read(arm.UC_ARM64_REG_X0),
                  "x": [uc.reg_read(r) for r in registers],
                  "sp": uc.reg_read(arm.UC_ARM64_REG_SP), "pc": uc.reg_read(arm.UC_ARM64_REG_PC),
                  "nzcv": uc.reg_read(arm.UC_ARM64_REG_NZCV)}
        if row != actual:
            differences.append({"input": value, "reference": actual, "compiled": row})
    report = {"matched": not differences and len(rows) == len(values), "cases": len(values),
              "differences": differences, "checkpoint_region_sha256": region["sha256"],
              "function": "std::__ndk1::__next_prime(unsigned long)",
              "range": ["0x300cffbc", "0x300d06dc"],
              "binary_sha256": hashlib.sha256(args.binary.read_bytes()).hexdigest()}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["matched"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
