#!/usr/bin/env python3
"""Bounded offline emulation of the protection library constructor.

Only the supplied ELF's AArch64 instructions execute in Unicorn. Android and
host imports are never executed; the first unresolved call stops the probe.
"""
from pathlib import Path
import argparse
import hashlib
import json
import re
import struct
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_INVALID
from unicorn.arm64_const import *

BASE = 0x10000000
STACK = 0x40000000
TLS = 0x50000000
STUB = 0x60000000
HEAP = 0x70000000
LIBC_BASE = 0x20000000


def run(path, output, limit, resolve_symbols, model_maps, libc_path):
    data = path.read_bytes()
    with path.open('rb') as stream:
        elf = ELFFile(stream)
        loads = [seg for seg in elf.iter_segments() if seg['p_type'] == 'PT_LOAD']
        dynamic = next(seg for seg in elf.iter_segments() if seg['p_type'] == 'PT_DYNAMIC')
        tags = {tag.entry.d_tag: tag.entry.d_val for tag in dynamic.iter_tags() if 'd_val' in tag.entry}
        symbols = list(dynamic.iter_symbols())
        image_size = max(seg['p_vaddr'] + seg['p_memsz'] for seg in loads)
        emu = Uc(UC_ARCH_ARM64, UC_MODE_ARM)
        emu.mem_map(BASE, (image_size + 4095) & -4096)
        for seg in loads:
            emu.mem_write(BASE + seg['p_vaddr'], seg.data())
        emu.mem_map(STACK, 0x200000)
        emu.mem_map(TLS, 0x10000)
        emu.mem_map(STUB, 0x10000)
        emu.mem_map(HEAP, 0x1000000)
        emu.reg_write(UC_ARM64_REG_SP, STACK + 0x1fff00)
        emu.reg_write(UC_ARM64_REG_TPIDR_EL0, TLS + 0x8000)
        emu.reg_write(UC_ARM64_REG_LR, STUB + 0xfff0)
        emu.mem_write(TLS + 0x8028, bytes.fromhex('123456789abcdef0'))
        names = {}
        relocations = []
        for start, length in [(tags['DT_RELA'], tags['DT_RELASZ']), (tags['DT_JMPREL'], tags['DT_PLTRELSZ'])]:
            for address in range(start, start + length, 24):
                offset, info, addend = struct.unpack('<QQq', emu.mem_read(BASE + address, 24))
                kind, index = info & 0xffffffff, info >> 32
                if kind == 0:
                    continue
                if kind == 1027:
                    value = BASE + addend
                elif kind in [1025, 1026, 257]:
                    sym = symbols[index]
                    if sym['st_shndx'] != 'SHN_UNDEF':
                        value = BASE + sym['st_value'] + addend
                    else:
                        value = STUB + 16 * index
                        names[value] = {'index': index, 'name': sym.name, 'relocation': hex(offset)}
                else:
                    raise RuntimeError(f'Unsupported relocation {kind} at {offset:x}')
                emu.mem_write(BASE + offset, struct.pack('<Q', value))
                relocations.append({'offset': offset, 'value': value, 'kind': kind})
        before = bytes(emu.mem_read(BASE, (image_size + 4095) & -4096))
        count = 0
        outcome = {'status': 'instruction_limit', 'limit': limit}
        recent = []
        imports = []
        resolved = {}
        files = {}
        allocations = {}
        heap_cursor = HEAP
        virtual_status = b'Name:\tclash-re\nState:\tR (running)\nTgid:\t1234\nPid:\t1234\nPPid:\t1\nTracerPid:\t0\nUid:\t10000\t10000\t10000\t10000\nThreads:\t1\n'
        maps = ''.join(f'{BASE+seg["p_vaddr"]:x}-{BASE+((seg["p_vaddr"]+seg["p_memsz"]+4095)&-4096):x} r-xp {seg["p_offset"]:08x} 00:00 1 /data/app/com.supercell.clashroyale/lib/arm64/libsupercell_clashroyale.so\n' for seg in loads).encode()
        if libc_path:
            with libc_path.open('rb') as libc_file:
                libc_elf = ELFFile(libc_file)
                libc_loads = [seg for seg in libc_elf.iter_segments() if seg['p_type'] == 'PT_LOAD']
                libc_end = max(seg['p_vaddr']+seg['p_memsz'] for seg in libc_loads)
                emu.mem_map(LIBC_BASE, (libc_end+4095)&-4096)
                for seg in libc_loads:
                    emu.mem_write(LIBC_BASE+seg['p_vaddr'], seg.data())
                    maps += f'{LIBC_BASE+seg["p_vaddr"]:x}-{LIBC_BASE+((seg["p_vaddr"]+seg["p_memsz"]+4095)&-4096):x} r-xp {seg["p_offset"]:08x} 00:00 2 /system/lib64/arm64/libc.so\n'.encode()
                libc_dynamic = next(seg for seg in libc_elf.iter_segments() if seg['p_type'] == 'PT_DYNAMIC')
                for symbol in libc_dynamic.iter_symbols():
                    if symbol['st_shndx'] != 'SHN_UNDEF' and symbol['st_info']['type'] == 'STT_FUNC':
                        names[LIBC_BASE+symbol['st_value']] = {'name': symbol.name, 'libc_export': True}

        def cstring(uc, address, maximum=4096):
            value = bytearray()
            for i in range(maximum):
                item = bytes(uc.mem_read(address+i, 1))
                if item == b'\0':
                    break
                value.extend(item)
            return bytes(value)

        def virtual_return(uc, value, invocation):
            invocation['virtual_return'] = hex(value)
            uc.reg_write(UC_ARM64_REG_X0, value)
            uc.reg_write(UC_ARM64_REG_PC, uc.reg_read(UC_ARM64_REG_LR))

        def on_code(uc, address, size, _):
            nonlocal count, outcome, heap_cursor
            count += 1
            recent.append(hex(address - BASE))
            if len(recent) > 24:
                recent.pop(0)
            if address >= STUB or address in names:
                args = [uc.reg_read(getattr(__import__('unicorn.arm64_const', fromlist=['x']), f'UC_ARM64_REG_X{i}')) for i in range(8)]
                pointed = []
                for value in args:
                    try:
                        content = bytes(uc.mem_read(value, 160)).split(b'\0', 1)[0]
                        pointed.append(content.decode('ascii') if content and all(32 <= x <= 126 for x in content) else None)
                    except Exception:
                        pointed.append(None)
                invocation = {'address': hex(address), 'import': names.get(address), 'args': [hex(x) for x in args], 'ascii_args': pointed, 'caller': hex(uc.reg_read(UC_ARM64_REG_LR) - BASE)}
                imports.append(invocation)
                imported_name = names.get(address, {}).get('name')
                if model_maps and imported_name in ['open', 'open64'] and pointed[0] == '/proc/self/status':
                    handle = 32 + len(files)
                    files[handle] = [virtual_status, 0]
                    invocation['virtual_file'] = 'synthetic status; TracerPid 0, Pid 1234'
                    virtual_return(uc, handle, invocation)
                    return
                if model_maps and imported_name == 'read' and args[0] in files:
                    content, position = files[args[0]]
                    chunk = content[position:position+args[2]]
                    uc.mem_write(args[1], chunk)
                    files[args[0]][1] += len(chunk)
                    virtual_return(uc, len(chunk), invocation)
                    return
                if model_maps and imported_name == 'close' and args[0] in files:
                    files.pop(args[0])
                    virtual_return(uc, 0, invocation)
                    return
                if model_maps and imported_name == 'sysconf' and args[0] == 0x27:
                    virtual_return(uc, 4096, invocation)
                    return
                if model_maps and imported_name == 'mprotect':
                    first = args[0]&-4096
                    end = (args[0]+args[1]+4095)&-4096
                    uc.mem_protect(first, end-first, args[2]&7)
                    virtual_return(uc, 0, invocation)
                    return
                if model_maps and imported_name in ['memcpy', 'memmove', '__memcpy_chk', '__memmove_chk']:
                    uc.mem_write(args[0], bytes(uc.mem_read(args[1], args[2])))
                    virtual_return(uc, args[0], invocation)
                    return
                if model_maps and imported_name in ['memset', '__memset_chk']:
                    uc.mem_write(args[0], bytes([args[1]&255])*args[2])
                    virtual_return(uc, args[0], invocation)
                    return
                if model_maps and imported_name == 'strlen':
                    virtual_return(uc, len(cstring(uc, args[0])), invocation)
                    return
                if model_maps and imported_name in ['malloc', 'calloc', 'realloc']:
                    length = args[0]*args[1] if imported_name == 'calloc' else args[1] if imported_name == 'realloc' else args[0]
                    value = heap_cursor
                    heap_cursor += (length+4095)&-4096
                    if heap_cursor >= HEAP + 0x1000000:
                        raise RuntimeError('Bounded virtual heap exhausted')
                    if imported_name == 'realloc' and args[0] in allocations:
                        uc.mem_write(value, bytes(uc.mem_read(args[0], min(allocations[args[0]], length))))
                    allocations[value] = length
                    virtual_return(uc, value, invocation)
                    return
                if model_maps and imported_name == 'free':
                    virtual_return(uc, 0, invocation)
                    return
                if model_maps and imported_name == 'sscanf' and pointed[1] == '%lx-%*x %*s %*x %*s %*d %255s':
                    text = cstring(uc, args[0]).decode('ascii')
                    match = re.match(r'([0-9a-fA-F]+)-[0-9a-fA-F]+\s+\S+\s+\S+\s+\S+\s+\S+\s+(\S+)', text)
                    if not match:
                        virtual_return(uc, 0, invocation)
                    else:
                        uc.mem_write(args[2], struct.pack('<Q', int(match[1], 16)))
                        uc.mem_write(args[3], match[2].encode('ascii')[:255]+b'\0')
                        virtual_return(uc, 2, invocation)
                    return
                if resolve_symbols and imported_name == 'dlsym' and pointed[1]:
                    symbol = pointed[1]
                    value = resolved.setdefault(symbol, STUB + 0x4000 + len(resolved)*16)
                    names[value] = {'name': symbol, 'virtual_resolution': True}
                    virtual_return(uc, value, invocation)
                    return
                if model_maps and imported_name == 'fopen' and (pointed[0] == '/proc/self/maps' or (libc_path and pointed[0] == '/system/lib64/arm64/libc.so')):
                    handle = 0x58000000 + 16*len(files)
                    content = maps if pointed[0] == '/proc/self/maps' else libc_path.read_bytes()
                    files[handle] = [content, 0]
                    virtual_return(uc, handle, invocation)
                    return
                if model_maps and imported_name == 'fread' and args[3] in files:
                    content, position = files[args[3]]
                    length = args[1]*args[2]
                    chunk = content[position:position+length]
                    uc.mem_write(args[0], chunk)
                    files[args[3]][1] += len(chunk)
                    virtual_return(uc, len(chunk)//args[1] if args[1] else 0, invocation)
                    return
                if model_maps and imported_name == 'fseek' and args[0] in files:
                    content, position = files[args[0]]
                    offset = args[1] if args[1] < 1<<63 else args[1]-(1<<64)
                    files[args[0]][1] = offset if args[2] == 0 else position+offset if args[2] == 1 else len(content)+offset
                    virtual_return(uc, 0, invocation)
                    return
                if model_maps and imported_name in ['fgets', '__fgets_chk']:
                    handle = args[2] if imported_name == 'fgets' else args[3]
                    length = args[1] if imported_name == 'fgets' else args[2]
                    content, position = files[handle]
                    if position >= len(content):
                        virtual_return(uc, 0, invocation)
                        return
                    end = content.find(b'\n', position)+1
                    if end == 0:
                        end = len(content)
                    end = min(end, position+length-1)
                    chunk = content[position:end]
                    uc.mem_write(args[0], chunk+b'\0')
                    files[handle][1] = end
                    invocation['virtual_line'] = chunk.decode('ascii')
                    virtual_return(uc, args[0], invocation)
                    return
                if model_maps and imported_name == 'fclose' and args[0] in files:
                    files.pop(args[0])
                    virtual_return(uc, 0, invocation)
                    return
                outcome = {'status': 'unresolved_import', **invocation}
                uc.emu_stop()

        def on_invalid(uc, access, address, size, value, _):
            nonlocal outcome
            outcome = {'status': 'invalid_memory', 'access': access, 'address': hex(address), 'size': size, 'pc': hex(uc.reg_read(UC_ARM64_REG_PC) - BASE)}
            return False

        emu.hook_add(UC_HOOK_CODE, on_code)
        emu.hook_add(UC_HOOK_MEM_INVALID, on_invalid)
        try:
            emu.emu_start(BASE + 0x65ccf0, STUB + 0xfff0, count=limit)
        except Exception as exc:
            outcome['exception'] = str(exc)
        after = bytes(emu.mem_read(BASE, (image_size + 4095) & -4096))
        changed = []
        for offset in range(0, len(after), 4096):
            if before[offset:offset+4096] != after[offset:offset+4096]:
                changed.append(hex(offset))
        output.mkdir(parents=True, exist_ok=True)
        (output/'protection-constructor-partial.bin').write_bytes(after)
        result = {'source': str(path), 'sha256': hashlib.sha256(data).hexdigest(), 'entry_rva': '0x65ccf0', 'load_base': hex(BASE), 'instruction_count': count, 'outcome': outcome, 'imports': imports, 'callback_rva_value': hex(struct.unpack('<Q', emu.mem_read(BASE + 0x6c6470, 8))[0]), 'recent_rvas': recent, 'changed_pages_rva': changed, 'limitations': ['Offline emulation; no Android application execution.', 'Optional dlsym model supplies synthetic addresses for requested names, without resolving host symbols.', 'Unresolved imports stop execution; imported functions are not stubbed as success.', 'Memory file is partial relocated memory, not a recovered executable.']}
        (output/'protection-constructor-probe.json').write_text(json.dumps(result, indent=2)+'\n')
        print(json.dumps(result, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('elf', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--limit', type=int, default=2000000)
    parser.add_argument('--resolve-symbols', action='store_true')
    parser.add_argument('--model-maps', action='store_true')
    parser.add_argument('--libc', type=Path)
    args = parser.parse_args()
    run(args.elf, args.output, args.limit, args.resolve_symbols, args.model_maps, args.libc)
