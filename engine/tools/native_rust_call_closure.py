#!/usr/bin/env python3
"""Extend generated coverage with direct call targets that still dispatch.

Every compiled routine whose `bl` target is not itself compiled falls back to
`super::dispatch`. Those targets are real original function entries that the
profile never observed. This tool resolves each unresolved dispatch target to
its FDE start and appends it to `extra_rvas`, so one regeneration closes the
whole statically visible call closure instead of one address per replay.

Metadata only: generated instruction bodies are never edited here.
"""
from pathlib import Path
import argparse, bisect, csv, json, re

ROOT = Path(__file__).resolve().parents[1]
BASE = 0x1000_0000
GENERATED = ROOT / 'rust-engine/src/generated'
FDE_INDEX = ROOT / 'analysis/native-engine-index/unwind-functions.tsv'
CONFIG = ROOT / 'rust-engine/codegen.json'


def compiled_ranges():
    text = (GENERATED / 'mod.rs').read_text()
    return [(int(a, 0), int(b, 0)) for a, b in re.findall(r'(0x[0-9a-f]+)\.\.=(0x[0-9a-f]+)=>', text)]


def dispatch_targets():
    targets = set()
    for path in sorted(GENERATED.glob('chunk*.rs')):
        targets.update(int(x, 0) for x in re.findall(r'super::dispatch\(cpu,rt,(0x[0-9a-f]+)\)', path.read_text()))
    return targets


def fde_rows():
    rows = []
    with FDE_INDEX.open() as handle:
        for row in csv.DictReader(handle, delimiter='\t'):
            rows.append((int(row['start'], 0) + BASE, int(row['end_exclusive'], 0) + BASE))
    return sorted(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='report only, do not write codegen.json')
    args = parser.parse_args()

    ranges = compiled_ranges()
    starts = [lo for lo, _ in ranges]
    fdes = fde_rows()
    fde_starts = [lo for lo, _ in fdes]

    def compiled(pc):
        j = bisect.bisect_right(starts, pc) - 1
        return j >= 0 and pc < ranges[j][1]

    config = json.loads(CONFIG.read_text())
    extra = list(config.get('extra_rvas', []))
    known = {int(rva, 0) for rva in extra}

    unresolved = set()
    for pc in sorted(dispatch_targets()):
        if compiled(pc):
            continue
        j = bisect.bisect_right(fde_starts, pc) - 1
        if j < 0 or not (fdes[j][0] <= pc < fdes[j][1]):
            continue
        unresolved.add(fdes[j][0])

    new = [pc for pc in sorted(unresolved) if (pc - BASE) not in known]
    if new and not args.check:
        extra += [hex(pc - BASE) for pc in new]
        config['extra_rvas'] = extra
        CONFIG.write_text(json.dumps(config, indent=2) + '\n')
    print(json.dumps({
        'compiled_functions': len(ranges),
        'unresolved_dispatch_entries': len(unresolved),
        'new_entries': [hex(pc) for pc in new],
        'extra_rvas': len(extra),
        'written': bool(new) and not args.check,
    }, indent=2))


if __name__ == '__main__':
    main()
