#!/usr/bin/env python3
"""Derive an ELF analysis copy from a verified contiguous runtime memory image.

PT_LOAD file bytes come from the supplied memory image; ELF relocation slots
are restored from the original object so Ghidra may relocate the copy normally.
The memory image and original ELF remain untouched.
"""
from pathlib import Path
import argparse
import hashlib
import json
import struct
from elftools.elf.elffile import ELFFile


def digest(data):
    return hashlib.sha256(data).hexdigest()


def rebuild(original_path, memory_path, manifest_path, output):
    original = original_path.read_bytes()
    memory = memory_path.read_bytes()
    manifest = json.loads(manifest_path.read_text())
    if digest(memory) != manifest['sha256']:
        raise RuntimeError('Runtime memory digest differs from its manifest')
    image = bytearray(original)
    with original_path.open('rb') as stream:
        elf = ELFFile(stream)
        loads = [seg for seg in elf.iter_segments() if seg['p_type'] == 'PT_LOAD']
        dynamic = next(seg for seg in elf.iter_segments() if seg['p_type'] == 'PT_DYNAMIC')
        tags = {tag.entry.d_tag: tag.entry.d_val for tag in dynamic.iter_tags() if 'd_val' in tag.entry}
        def offset(address):
            return next(address-seg['p_vaddr']+seg['p_offset'] for seg in loads if seg['p_vaddr'] <= address < seg['p_vaddr']+seg['p_filesz'])
        segments = []
        for seg in loads:
            first, count, destination = seg['p_vaddr'], seg['p_filesz'], seg['p_offset']
            selected = memory[first:first+count]
            if len(selected) != count:
                raise RuntimeError('Memory image does not cover a complete PT_LOAD')
            image[destination:destination+count] = selected
            segments.append({'rva': hex(first), 'file_offset': hex(destination), 'size': count, 'changed_bytes_before_relocation_restore': sum(a!=b for a,b in zip(original[destination:destination+count], selected))})
        restored = 0
        for start, length in [(tags.get('DT_RELA',0), tags.get('DT_RELASZ',0)), (tags.get('DT_JMPREL',0), tags.get('DT_PLTRELSZ',0))]:
            for address in range(start, start+length, 24):
                where, info, _ = struct.unpack_from('<QQq', original, offset(address))
                if info & 0xffffffff == 0:
                    continue
                try:
                    destination = offset(where)
                except StopIteration:
                    continue
                image[destination:destination+8] = original[destination:destination+8]
                restored += 1
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(image)
    result = {'original': str(original_path), 'original_sha256': digest(original), 'memory': str(memory_path), 'memory_sha256': digest(memory), 'memory_manifest': str(manifest_path), 'runtime_base': manifest['base'], 'derived': str(output), 'derived_sha256': digest(image), 'segments': segments, 'relocation_slots_restored': restored, 'limitations': ['Analysis copy from captured process bytes; not an original executable or a proven portable runnable ELF.', 'Runtime BSS state and external pointers outside PT_LOAD file bytes are not reproduced.', 'Original relocation slots are restored; other dynamically written absolute pointer values may remain.']}
    output.with_suffix(output.suffix+'.json').write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('original', type=Path)
    parser.add_argument('memory', type=Path)
    parser.add_argument('manifest', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    rebuild(args.original, args.memory, args.manifest, args.output)
