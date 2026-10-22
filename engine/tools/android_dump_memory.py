#!/usr/bin/env python3
"""Capture a mapped original game library and its adjacent BSS from the lab AVD.

The resulting file is a memory image indexed by virtual offset from module base;
it is not a rebuilt ELF file. Unmapped holes contain zeroes and are listed only
through their absence from the range manifest.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess

import android_runtime as runtime


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pid", type=int, required=True)
    parser.add_argument("--module", choices=["libg.so", "libsupercell_clashroyale.so"], required=True)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--arm", action="store_true", help="capture the genuine ARM64 AVD on port 5582")
    args = parser.parse_args()
    if args.arm:
        import android_arm_runtime  # Selects the isolated ARM64 profile constants.
    runtime.assert_ours()
    cmdline = runtime.adb("shell", "cat", f"/proc/{args.pid}/cmdline", capture=True).stdout
    if cmdline.split("\0", 1)[0] != runtime.PACKAGE:
        raise SystemExit("PID does not belong to the original game process")
    maps = runtime.adb("shell", "cat", f"/proc/{args.pid}/maps", capture=True).stdout
    rows = []
    for line in maps.splitlines():
        fields = line.split()
        start, end = (int(value, 16) for value in fields[0].split("-"))
        rows.append((start, end, fields[1], int(fields[2], 16), line))
    candidates = [(index, row) for index, row in enumerate(rows)
                  if row[4].endswith("/" + args.module)]
    if not candidates:
        raise SystemExit(f"{args.module} is not present in PID {args.pid} maps")
    # NativeBridge can transiently map duplicate ELF metadata elsewhere. Select
    # the lowest mapped copy and the ranges within 256 MiB of its first page.
    base = min(row[0] for index, row in candidates)
    selected = [(index, row) for index, row in candidates if row[0] < base + 256 * 1024 * 1024]
    index = selected[-1][0]
    if index + 1 < len(rows) and rows[index + 1][0] == rows[index][1] and "[anon:.bss]" in rows[index + 1][4]:
        selected.append((index + 1, rows[index + 1]))
    size = selected[-1][1][1] - base
    image = bytearray(size)
    ranges = []
    adb = [str(runtime.SDK / "platform-tools/adb"), "-s", runtime.SERIAL]
    for index, (start, end, permissions, file_offset, line) in selected:
        if "r" not in permissions:
            continue
        command = adb + ["exec-out", "dd", f"if=/proc/{args.pid}/mem", "bs=1048576",
                         f"skip={start}", f"count={end-start}", "iflag=skip_bytes,count_bytes", "status=none"]
        result = subprocess.run(command, capture_output=True, check=True, timeout=60)
        if len(result.stdout) != end - start:
            raise SystemExit(f"Short memory read at {start:#x}: expected {end-start}, received {len(result.stdout)}")
        image[start - base:end - base] = result.stdout
        ranges.append({"start": hex(start), "end": hex(end), "memory_offset": hex(start - base),
                       "file_offset": hex(file_offset), "permissions": permissions,
                       "sha256": hashlib.sha256(result.stdout).hexdigest(), "map": line})
    output = args.output or runtime.RUNTIME / "memory" / f"{args.module}-pid{args.pid}-memory.bin"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(image)
    record = {"pid": args.pid, "module": args.module, "base": hex(base), "size": hex(size),
              "filename": output.name, "sha256": hashlib.sha256(image).hexdigest(), "ranges": ranges}
    if args.module == "libsupercell_clashroyale.so":
        callbacks = {}
        for offset in (0x6C6468, 0x6C6470):
            value = struct.unpack_from("<Q", image, offset)[0]
            callbacks[hex(offset)] = {"value": hex(value), "relative_to_base": hex(value - base) if value else None}
        record["observed_callback_slots"] = callbacks
    output.with_suffix(".json").write_text(json.dumps(record, indent=2) + "\n")
    output.with_suffix(".maps.txt").write_text(maps)
    print(json.dumps({key: value for key, value in record.items() if key != "ranges"}, indent=2))


if __name__ == "__main__":
    main()
