#!/usr/bin/env python3
"""Run the recovered pure bootstrap checksum probes in bounded Unicorn.

No Android or host imports execute. Original encrypted file bytes can replace
only the bytes being checksummed for an explicitly labeled comparison.
"""
from pathlib import Path
import argparse
import hashlib
import json
import struct
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_INVALID
from unicorn.arm64_const import *


def check(memory_path, manifest_path, original_path, output):
    memory = memory_path.read_bytes()
    manifest = json.loads(manifest_path.read_text())
    if hashlib.sha256(memory).hexdigest() != manifest['sha256']:
        raise RuntimeError('Memory image digest mismatch')
    base = int(manifest['base'], 16)
    original = original_path.read_bytes()
    with original_path.open('rb') as stream:
        loads = [dict(s.header) for s in ELFFile(stream).iter_segments() if s['p_type'] == 'PT_LOAD']
    def original_range(address, length):
        seg = next(s for s in loads if s['p_vaddr'] <= address and address+length <= s['p_vaddr']+s['p_filesz'])
        offset = address-seg['p_vaddr']+seg['p_offset']
        return original[offset:offset+length]
    probes = [('A', 0x65ae4c, 0x6522ec, 0x6c4250, 0x41e6d70d), ('B', 0x65aeb4, 0x6541fc, 0x6c4260, 0x99df9df4)]
    results = []
    for label, entry, code, table, key in probes:
        expected, length = struct.unpack_from('<II', memory, table)
        for variant in ['captured_runtime', 'original_encrypted_file_bytes']:
            uc = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
            uc.mem_map(base, (len(memory)+4095)&-4096)
            uc.mem_write(base, memory)
            selected = memory[code:code+length]
            if variant != 'captured_runtime':
                selected = original_range(code, length)
                uc.mem_write(base+code, selected)
            uc.mem_map(0x70000000, 0x10000)
            uc.mem_map(0x71000000, 0x1000)
            uc.reg_write(UC_ARM64_REG_SP, 0x7000ff00)
            uc.reg_write(UC_ARM64_REG_X0, base+code)
            uc.reg_write(UC_ARM64_REG_LR, 0x71000000)
            count = 0
            failure = None
            def instruction(emu, address, size, _):
                nonlocal count
                count += 1
                rva = address-base
                if not (0x641fd4 <= rva <= 0x642108 or entry <= rva < entry+0x68 or 0x65b0a8 <= rva <= 0x65b214):
                    raise RuntimeError(f'Probe left recovered pure checksum code at {address:x}')
            def invalid(emu, access, address, size, value, _):
                raise RuntimeError(f'Unmapped memory {access} {address:x}; PC {emu.reg_read(UC_ARM64_REG_PC):x}')
            uc.hook_add(UC_HOOK_CODE, instruction)
            uc.hook_add(UC_HOOK_MEM_INVALID, invalid)
            try:
                uc.emu_start(base+entry, 0x71000000, count=10000000)
            except Exception as error:
                failure = str(error)
            value = uc.reg_read(UC_ARM64_REG_X0)&0xffffffff
            results.append({'probe': label, 'variant': variant, 'probe_entry_rva': hex(entry), 'sample_rva': hex(code), 'sample_length': length, 'sample_sha256': hashlib.sha256(selected).hexdigest(), 'table_rva': hex(table), 'table_xor_word': hex(expected), 'literal_xor_word': hex(key), 'returned': uc.reg_read(UC_ARM64_REG_PC) == 0x71000000, 'result_u32': hex(value), 'implied_checksum_low_u32': hex(value^expected^key), 'instructions': count, 'failure': failure})
    report = {'memory': str(memory_path), 'memory_sha256': hashlib.sha256(memory).hexdigest(), 'original': str(original_path), 'original_sha256': hashlib.sha256(original).hexdigest(), 'runtime_base': hex(base), 'results': results, 'limitations': ['Bounded execution of recovered pure checksum code only; no Android runtime or host imports.', 'The original file comparison uses protected on-disk bytes, not recovered pristine decrypted code.', 'Probe values alone do not establish which later bootstrap branch executed in Android.']}
    output.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('memory', type=Path)
    parser.add_argument('manifest', type=Path)
    parser.add_argument('original', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    check(args.memory, args.manifest, args.original, args.output)
