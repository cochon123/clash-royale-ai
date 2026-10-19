#!/usr/bin/env python3
"""Verify original entity/component pointers and tick callbacks in a saved battle."""
import argparse
from contextlib import ExitStack
import hashlib
import json
import mmap
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[1]
SOURCE_SHA = '79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6'
BASE = 0x10000000


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=Path)
    args = parser.parse_args()
    report_path = args.run / 'report.json'
    checkpoint_path = args.run / 'state/checkpoint.json'
    report = json.loads(report_path.read_bytes())
    checkpoint = json.loads(checkpoint_path.read_bytes())
    if report['source_sha256'] != SOURCE_SHA or checkpoint['source_sha256'] != SOURCE_SHA:
        raise ValueError('Native source identity mismatch')
    if Path(checkpoint['report_path']).resolve() != report_path.resolve():
        raise ValueError('Checkpoint/report path mismatch')
    if checkpoint['call_plan']['sha256'] != report['call_plan']['sha256']:
        raise ValueError('Checkpoint/report call-plan mismatch')
    source = ROOT / 'analysis/libg-cipher/libg.decrypted.so'
    if digest(source) != SOURCE_SHA:
        raise ValueError('Original decoded engine bytes changed')
    elf = source.read_bytes()
    phoff = struct.unpack_from('<Q', elf, 32)[0]
    phsize, phcount = struct.unpack_from('<HH', elf, 54)
    executable = []
    for index in range(phcount):
        kind, flags, _, start, _, _, size, _ = struct.unpack_from('<IIQQQQQQ', elf, phoff + index * phsize)
        if kind == 1 and flags & 1:
            executable.append((BASE + start, BASE + start + size))

    with ExitStack() as stack:
        regions = []
        for region in checkpoint['regions']:
            path = checkpoint_path.parent / region['file']
            if digest(path) != region['sha256']:
                raise ValueError(f'Changed checkpoint memory: {path}')
            stream = stack.enter_context(path.open('rb'))
            memory = stack.enter_context(mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ))
            start = int(region['base'], 0)
            regions.append((start, start + len(memory), memory))

        def read(address, size):
            for start, end, memory in regions:
                if start <= address and address + size <= end:
                    return memory[address - start:address - start + size]
            raise ValueError(f'Pointer outside retained native memory: {address:#x}+{size:#x}')

        def qword(address):
            return struct.unpack('<Q', read(address, 8))[0]

        def original_code(address):
            return any(start <= address < end for start, end in executable)

        def callback(vtable, offset):
            address = qword(vtable + offset)
            return {'slot': hex(offset), 'address': hex(address), 'rva': hex(address - BASE),
                    'original_executable_code': original_code(address)}

        values = report['call_plan']['values']
        manager = values['battle_entities']
        data = qword(manager + 8)
        count = struct.unpack('<i', read(manager + 0x14, 4))[0]
        if not 0 <= count <= 100000:
            raise ValueError('Implausible native entity count')
        entities = []
        pointers = []
        for index in range(count):
            pointer = qword(data + index * 8)
            pointers.append(pointer)
            vtable = qword(pointer)
            component_array = qword(pointer + 0x18)
            component_count = struct.unpack('<i', read(pointer + 0x24, 4))[0]
            mask = struct.unpack('<I', read(pointer + 0x30, 4))[0]
            if not 0 <= component_count <= 256:
                raise ValueError('Implausible native component slot count')
            components = []
            for slot in range(component_count):
                component = qword(component_array + slot * 8)
                if component:
                    component_vtable = qword(component)
                    components.append({'slot': slot, 'pointer': hex(component),
                                       'vtable': hex(component_vtable),
                                       'tick_callbacks': [callback(component_vtable, offset) for offset in (0x10, 0x18)]})
            entities.append({'index': index, 'pointer': hex(pointer), 'vtable': hex(vtable),
                             'tick_callbacks': [callback(vtable, offset) for offset in (0x40, 0x48)],
                             'component_slot_count': component_count, 'component_mask': hex(mask),
                             'components': components})
        all_callbacks = [c for e in entities for c in e['tick_callbacks']] + [
            c for e in entities for component in e['components'] for c in component['tick_callbacks']]
        world = values['battle_world']
        result = {'source_sha256': SOURCE_SHA, 'report_sha256': digest(report_path),
                  'checkpoint_sha256': digest(checkpoint_path), 'report_failure': report['failure'],
                  'world_counters': {hex(offset): struct.unpack('<I', read(world + offset, 4))[0]
                                     for offset in (0x60, 0x70)},
                  'manager': hex(manager), 'manager_vtable': hex(qword(manager)),
                  'entity_count': count, 'unique_entity_pointers': len(set(pointers)) == count,
                  'nonnull_component_count': sum(len(e['components']) for e in entities),
                  'all_tick_callbacks_in_original_code': bool(all_callbacks) and all(c['original_executable_code'] for c in all_callbacks),
                  'entities': entities,
                  'evidence': 'Original f8cd90 reads manager list+8/count+0x14; invokes entity vtable+0x40/+0x48 and component slots through entity+0x18/count+0x24, component vtable+0x10/+0x18. Entity vtables are distinct from the manager vtable.',
                  'limits': ['Structural/callback verification does not identify component semantics or establish full match/UI correctness.']}
        output = args.run / 'native-entity-audit.json'
        output.write_text(json.dumps(result, indent=2) + '\n')
        print(json.dumps({k: v for k, v in result.items() if k != 'entities'}, indent=2))


if __name__ == '__main__':
    main()
