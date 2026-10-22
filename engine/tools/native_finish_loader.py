#!/usr/bin/env python3
"""Alternate verified loader translation with original ARM64 call execution.

Each input checkpoint stays immutable. Translated code stops before BL/RET;
Unicorn executes the exact call to its next instruction, or the outer return.
The final result is complete only if the original DT_INIT returns successfully.
"""
import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
BASE = 0x10000000
RETURN = 0x90000000


def run(checkpoint, output, steps):
    if output.exists():
        raise ValueError('Choose a new output directory')
    output.mkdir(parents=True)
    current = checkpoint.resolve()
    history = []
    for index in range(steps):
        accelerated = output/f'{index:02d}-translated'
        with (output/f'{index:02d}-translated.log').open('w') as log:
            subprocess.run([sys.executable, str(ROOT/'tools/libg_cipher_checkpoint.py'),
                            str(current/'checkpoint.json'), '--out', str(accelerated)],
                           cwd=ROOT, check=True, stdout=log)
        acceleration = json.loads((accelerated/'acceleration.json').read_text())
        status = acceleration['stop_status']
        pc = int(acceleration['pc'], 16)
        history.append({'translated': str(accelerated), 'pc': hex(pc), 'status': status})
        print(json.dumps(history[-1]), flush=True)
        current = accelerated
        if status == 0:
            continue
        stop = RETURN if status == 1 else pc+4
        native = output/f'{index:02d}-native'
        command = [sys.executable, str(ROOT/'tools/native_unpack.py'),
                   '--out', str(native), '--companion-memory',
                   str(ROOT/'analysis/native-entry/companion-text-decrypt/companion-decrypted-memory.bin'),
                   '--recovered-binding', str(ROOT/'analysis/native-unpack/companion-binding-decoded/binding-decoded.bin'),
                   '--resume-from', str(current), '--checkpoint-out', str(native/'state'),
                   '--timeout', '55', '--instructions', '300000000']
        if stop != RETURN:
            command += ['--until-rva', hex(stop-BASE)]
        with (output/f'{index:02d}-native.log').open('w') as log:
            subprocess.run(command, cwd=ROOT, check=True, stdout=log)
        report = json.loads((native/'report.json').read_text())
        current = native/'state'
        history[-1].update(native=str(current), native_pc=report['pc'], failure=report['failure'])
        summary = {'checkpoint': str(current), 'history': history,
                   'returned': report['returned'], 'failure': report['failure']}
        (output/'completion.json').write_text(json.dumps(summary, indent=2)+'\n')
        print(json.dumps({k: summary[k] for k in ('checkpoint', 'returned', 'failure')}), flush=True)
        if report['failure']:
            raise RuntimeError('Original call stopped at an unresolved dependency; see the retained report')
        if report['returned']:
            return
        if int(report['pc'], 16) != stop:
            raise RuntimeError('Original call execution reached its bound before returning; resume its saved checkpoint')
    raise RuntimeError('Bounded translation/native handoff count exhausted')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('checkpoint', type=Path, help='Directory containing checkpoint.json')
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--steps', type=int, default=30)
    args = parser.parse_args()
    run(args.checkpoint, args.out, args.steps)
