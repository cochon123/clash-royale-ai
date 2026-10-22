#!/usr/bin/env python3
"""Merge retained original-engine coverage, retaining each producer's identity."""
from pathlib import Path
import argparse, hashlib, json
from native_rust_codegen import EXPECTED


def merge(paths, out):
    blocks = {}
    inputs = []
    for path in paths:
        raw = path.read_bytes()
        data = json.loads(raw)
        if data['source_sha256'] != EXPECTED:
            raise ValueError(f'original source identity mismatch: {path}')
        inputs.append({'path': str(path), 'sha256': hashlib.sha256(raw).hexdigest()})
        for row in data['blocks']:
            address = int(row['rva'], 0)
            value = blocks.setdefault(address, {'rva': hex(address), 'bytes': 0, 'executions': 0})
            value['bytes'] = max(value['bytes'], row['bytes'])
            value['executions'] += row['executions']
    report = {'source_sha256': EXPECTED, 'input_profiles': inputs,
              'blocks': [blocks[k] for k in sorted(blocks)],
              'scope': 'Union of observed original-engine instruction coverage. This is a generation input, not a claim of complete game or instruction coverage.'}
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'profiles': len(inputs), 'blocks': len(blocks), 'out': str(out)}))


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('profiles', type=Path, nargs='+')
    a = p.parse_args()
    merge(a.profiles, a.out)
