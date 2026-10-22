#!/usr/bin/env python3
"""Add exact recovered FDEs and bounded direct dependencies to the Rust build."""
from pathlib import Path
import argparse, collections, csv, hashlib, json, re
from capstone import Cs, CS_ARCH_ARM64, CS_MODE_ARM
from native_rust_codegen import ROOT, SOURCE, EXPECTED, BASE


def extend(seeds, depth, limit):
    raw = SOURCE.read_bytes()
    if hashlib.sha256(raw).hexdigest() != EXPECTED:
        raise ValueError("original code identity changed")
    rows = csv.DictReader((ROOT / 'analysis/native-engine-index/unwind-functions.tsv').open(), delimiter='\t')
    bounds = {int(r['start'], 0): int(r['end_exclusive'], 0) for r in rows}
    module = (ROOT / 'rust-engine/src/generated/mod.rs').read_text()
    known = {int(x, 16) - BASE for x in re.findall(r'0x([0-9a-f]+)\.\.=', module)}
    path = ROOT / 'rust-engine/codegen.json'
    config = json.loads(path.read_text()) if path.exists() else {}
    previous = {int(x, 0) for x in config.get('extra_rvas', [])}
    additions = set()
    md = Cs(CS_ARCH_ARM64, CS_MODE_ARM)
    md.detail = True
    todo = collections.deque((int(s, 0), 0) for s in seeds)
    while todo:
        start, level = todo.popleft()
        if start in known or start in previous or start in additions:
            continue
        if start not in bounds:
            raise ValueError(f"not an exact original FDE start: {start:#x}")
        if len(additions) >= limit:
            raise ValueError("bounded dependency limit exceeded; configuration unchanged")
        additions.add(start)
        if level < depth:
            for i in md.disasm(raw[start:bounds[start]], start):
                if i.mnemonic in ('bl', 'b'):
                    target = i.operands[0].imm
                    if target in bounds and not start <= target < bounds[start]:
                        todo.append((target, level + 1))
    config.update(source_sha256=EXPECTED, extra_rvas=[hex(x) for x in sorted(previous | additions)])
    path.write_text(json.dumps(config, indent=2) + '\n')
    print(json.dumps({'new_functions': len(additions), 'new_rvas': [hex(x) for x in sorted(additions)]}))


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--rva', action='append', required=True)
    p.add_argument('--depth', type=int, default=3)
    p.add_argument('--limit', type=int, default=128)
    a = p.parse_args()
    extend(a.rva, a.depth, a.limit)
