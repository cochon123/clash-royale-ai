#!/usr/bin/env python3
"""Execute selected local ARM64 ELF initialization in Unicorn and retain bytes.

This does not launch Android or connect to a game server. Imports/syscalls use
explicit emulation stubs, recorded in the output. Source files remain unchanged.
"""
import argparse
from collections import Counter
import ctypes
import hashlib
import json
import os
from pathlib import Path
import struct
import time
import unicorn
import zipfile

from capstone import Cs, CS_ARCH_ARM64, CS_MODE_LITTLE_ENDIAN
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationTable
from unicorn import Uc, UcError, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_INTR, UC_HOOK_MEM_INVALID
from unicorn.arm64_const import *
from native_printf import vsnprintf_arm64
from native_dependency import map_elf_dependency
from native_vfs import ReadOnlyVfs, ReadOnlyStdio
from native_threads import NativeThreads
from native_heap import GuestHeap

ROOT=Path(__file__).resolve().parents[1]
PAGE=0x1000
BASE=0x10000000
EXTERNAL=0x80000000
STACK=0x70000000
HEAP=0x50000000
HEAP_SIZE=0x10000000
TLS=0x60000000
RETURN=0x90000000
CXX_BASE=0x30000000
SENTRY_BASE=0x31000000
HOST_LIBC=ctypes.CDLL(None,use_errno=True)
# Unsampled hot imports need only their actual ABI arguments. Sampled entries
# still retain all six registers, preserving the existing diagnostic trace.
HOT_IMPORT_ARITY={
    'strlen':1,'strnlen':2,'__strlen_chk':2,
    'malloc':1,'calloc':2,'realloc':2,'free':1,
    '_Znwm':1,'_Znam':1,'_ZdlPv':1,'_ZdaPv':1,'_ZdlPvm':2,'_ZdaPvm':2,
    'memcpy':3,'memmove':3,'__memcpy_chk':4,'__memmove_chk':4,
    'memset':3,'__memset_chk':4,'memcmp':3,'memchr':3,
    'strcmp':2,'strcasecmp':2,'strncmp':3,'strncasecmp':3,
    'atoi':1,'atol':1,'atoll':1,'atof':1,
}
class HostCalendar(ctypes.Structure):
    _fields_=[(name,ctypes.c_int) for name in ('tm_sec','tm_min','tm_hour','tm_mday','tm_mon','tm_year','tm_wday','tm_yday','tm_isdst')]+[('tm_gmtoff',ctypes.c_long),('tm_zone',ctypes.c_char_p)]
def align(n):return (n+PAGE-1)&-PAGE


def unpack(source:Path,out:Path,timeout:int,max_instructions:int,companion_memory:Path|None=None,companion_init:int|None=None,companion_init_only:bool=False,metadata_block:Path|None=None,companion_sret:bool=False,decode_binding:Path|None=None,recovered_binding:Path|None=None,checkpoint_out:Path|None=None,resume_from:Path|None=None,until_rva:int|None=None,run_init_array:bool=False,entry_rva:int|None=None,entry_arguments:tuple=(),runtime_cxx:bool=False,call_plan:Path|None=None,binding_index:Path|None=None,runtime_sentry:bool=False,virtual_files:Path|None=None):
    if run_init_array and until_rva is not None:raise ValueError('--until-rva cannot interrupt an init-array sequence')
    if entry_arguments and entry_rva is None:raise ValueError('--entry-arg requires --entry-rva')
    if len(entry_arguments)>8:raise ValueError('At most eight register arguments are supported')
    if call_plan and (entry_rva is not None or until_rva is not None):raise ValueError('--call-plan cannot be combined with --entry-rva or --until-rva')
    out.mkdir(parents=True,exist_ok=True)
    model_directory=out/'execution-model';model_directory.mkdir(exist_ok=True)
    model_sources={}
    for model in (Path(__file__),Path(vsnprintf_arm64.__code__.co_filename),Path(map_elf_dependency.__code__.co_filename),Path(ReadOnlyVfs.__init__.__code__.co_filename),Path(NativeThreads.__init__.__code__.co_filename),Path(GuestHeap.__init__.__code__.co_filename)):
        contents=model.read_bytes();(model_directory/model.name).write_bytes(contents)
        model_sources[model.name]=hashlib.sha256(contents).hexdigest()
    original=source.read_bytes()
    source_digest=hashlib.sha256(original).hexdigest()
    vfs=None;stdio=None;virtual_info=None
    if virtual_files:
        virtual_raw=virtual_files.read_bytes();virtual_bindings=json.loads(virtual_raw)
        vfs=ReadOnlyVfs(virtual_bindings);stdio=ReadOnlyStdio(vfs)
        virtual_info={'manifest':str(virtual_files.resolve()),'manifest_sha256':hashlib.sha256(virtual_raw).hexdigest(),'bindings':virtual_bindings}
    with source.open('rb') as stream:
        elf=ELFFile(stream)
        segments=[dict(s.header) for s in elf.iter_segments() if s['p_type']=='PT_LOAD']
        dynamic=next(s for s in elf.iter_segments() if s['p_type']=='PT_DYNAMIC')
        tags={t.entry.d_tag:t.entry.get('d_val',t.entry.get('d_ptr')) for t in dynamic.iter_tags()}
        symbols=list(dynamic.iter_symbols())
        def offset(address):
            for s in segments:
                if s['p_vaddr']<=address<s['p_vaddr']+s['p_filesz']:
                    return s['p_offset']+address-s['p_vaddr']
            raise ValueError(f'Unmapped virtual address {address:x}')
        relocations=[]
        for address,size in [('DT_RELA','DT_RELASZ'),('DT_JMPREL','DT_PLTRELSZ')]:
            if address in tags:
                table=RelocationTable(elf,offset(tags[address]),tags[size],True)
                relocations.extend(dict(r.entry) for r in table.iter_relocations())
    uc=Uc(UC_ARCH_ARM64,UC_MODE_ARM)
    end=max(s['p_vaddr']+s['p_memsz'] for s in segments)
    uc.mem_map(BASE,align(end))
    for s in segments:
        uc.mem_write(BASE+s['p_vaddr'],original[s['p_offset']:s['p_offset']+s['p_filesz']])
    uc.mem_map(STACK,0x400000);uc.mem_map(HEAP,HEAP_SIZE);uc.mem_map(TLS,PAGE)
    external_names={EXTERNAL+i*16:symbol.name for i,symbol in enumerate(symbols)}
    cxx_symbols={};cxx_info=None;cxx_end=0
    if runtime_cxx:
        cxx_source=ROOT/'analysis/native/lib/arm64-v8a/libc++_shared.so'
        cxx_original=cxx_source.read_bytes()
        with cxx_source.open('rb') as stream:
            cxx=ELFFile(stream)
            cxx_segments=[dict(s.header) for s in cxx.iter_segments() if s['p_type']=='PT_LOAD']
            cxx_end=align(max(s['p_vaddr']+s['p_memsz'] for s in cxx_segments))
            uc.mem_map(CXX_BASE,cxx_end)
            for s in cxx_segments:uc.mem_write(CXX_BASE+s['p_vaddr'],cxx_original[s['p_offset']:s['p_offset']+s['p_filesz']])
            cxx_dynamic=next(s for s in cxx.iter_segments() if s['p_type']=='PT_DYNAMIC')
            cxx_tags={t.entry.d_tag:t.entry.get('d_val',t.entry.get('d_ptr')) for t in cxx_dynamic.iter_tags()}
            cxx_all_symbols=list(cxx_dynamic.iter_symbols())
            cxx_symbols={s.name:CXX_BASE+s['st_value'] for s in cxx_all_symbols if s['st_shndx']!='SHN_UNDEF' and s.name}
            cxx_external=EXTERNAL+len(symbols)*16
            external_names.update({cxx_external+i*16:s.name for i,s in enumerate(cxx_all_symbols)})
            for address,size in [('DT_RELA','DT_RELASZ'),('DT_JMPREL','DT_PLTRELSZ')]:
                if address not in cxx_tags:continue
                segment=next(s for s in cxx_segments if s['p_vaddr']<=cxx_tags[address]<s['p_vaddr']+s['p_filesz'])
                table_offset=segment['p_offset']+cxx_tags[address]-segment['p_vaddr']
                for r in RelocationTable(cxx,table_offset,cxx_tags[size],True).iter_relocations():
                    symbol=cxx_all_symbols[r['r_info_sym']];kind=r['r_info_type']
                    if kind==1027:value=CXX_BASE+r['r_addend']
                    elif kind in (257,1025,1026):
                        if symbol['st_shndx']=='SHN_UNDEF':value=0 if symbol['st_info']['bind']=='STB_WEAK' else cxx_symbols.get(symbol.name,cxx_external+r['r_info_sym']*16)
                        else:value=CXX_BASE+symbol['st_value']
                        value+=r['r_addend']
                    elif kind==0:continue
                    else:raise ValueError(f'Unsupported C++ runtime relocation type {kind}')
                    uc.mem_write(CXX_BASE+r['r_offset'],struct.pack('<Q',value&((1<<64)-1)))
            cxx_info={'source':str(cxx_source),'source_sha256':hashlib.sha256(cxx_original).hexdigest(),'base':hex(CXX_BASE),'size':cxx_end,'init_array_rva':hex(cxx_tags.get('DT_INIT_ARRAY',0)),'init_array_count':cxx_tags.get('DT_INIT_ARRAYSZ',0)//8,'init_array_completed':0}
    if 'dl_iterate_phdr' not in external_names.values():external_names[max(external_names)+16]='dl_iterate_phdr'
    external_names[max(external_names)+16]='_analysis_dl_iterate_return'
    sentry_info=None;sentry_symbols={}
    if runtime_sentry:
        sentry_source=ROOT/'analysis/native/lib/arm64-v8a/libsentry.so'
        mapped_sentry=map_elf_dependency(uc,sentry_source,SENTRY_BASE,external_names,cxx_symbols)
        sentry_info=mapped_sentry['info'];sentry_symbols=mapped_sentry['symbols'];sentry_tags=mapped_sentry['tags']
    dependency_symbols={**cxx_symbols,**sentry_symbols}
    external_by_name={name:address for address,name in external_names.items()}
    game_symbols={s.name:BASE+s['st_value'] for s in symbols if s['st_shndx']!='SHN_UNDEF' and s.name}
    external_end=align(max(external_names)-EXTERNAL+16+PAGE)
    uc.mem_map(EXTERNAL,external_end);uc.mem_map(RETURN,PAGE)
    uc.mem_write(EXTERNAL,b'\xc0\x03\x5f\xd6'*(external_end//4))
    uc.mem_write(RETURN,b'\xc0\x03\x5f\xd6')
    companion_info=None;companion_base=None;libc_names={};native_libc={};extra_maps=b''
    if companion_memory:
        snapshot=companion_memory.read_bytes()
        manifest=json.loads(companion_memory.with_suffix('.json').read_text())
        expected=manifest.get('sha256',manifest.get('derived_sha256'))
        digest=hashlib.sha256(snapshot).hexdigest()
        if expected and expected!=digest:raise ValueError('Companion snapshot SHA-256 mismatch')
        companion_base=int(manifest.get('base','0x7fda04465000'),16)
        uc.mem_map(companion_base,align(len(snapshot)));uc.mem_write(companion_base,snapshot)
        extra_maps+=f'{companion_base:x}-{companion_base+align(len(snapshot)):x} r-xp 00000000 00:00 2 /data/app/com.supercell.clashroyale/lib/arm64/libsupercell_clashroyale.so\n'.encode()
        libc_source=ROOT/'analysis/android-runtime/libc-arm64.so'
        libc_base=0x7fda15ac7000
        libc_digest=hashlib.sha256(libc_source.read_bytes()).hexdigest()
        with libc_source.open('rb') as stream:
            libc=ELFFile(stream)
            libc_segments=[s for s in libc.iter_segments() if s['p_type']=='PT_LOAD']
            uc.mem_map(libc_base,align(max(s['p_vaddr']+s['p_memsz'] for s in libc_segments)))
            for segment in libc_segments:uc.mem_write(libc_base+segment['p_vaddr'],segment.data())
            libc_dynamic=next(s for s in libc.iter_segments() if s['p_type']=='PT_DYNAMIC')
            libc_symbols=list(libc_dynamic.iter_symbols())
            libc_tags={t.entry.d_tag:t.entry.get('d_val',t.entry.get('d_ptr')) for t in libc_dynamic.iter_tags()}
            libc_plt_linked=0
            def libc_file_offset(address):
                segment=next(s for s in libc_segments if s['p_vaddr']<=address<s['p_vaddr']+s['p_filesz'])
                return segment['p_offset']+address-segment['p_vaddr']
            for relocation in RelocationTable(libc,libc_file_offset(libc_tags['DT_JMPREL']),libc_tags['DT_PLTRELSZ'],True).iter_relocations():
                symbol=libc_symbols[relocation['r_info_sym']]
                if relocation['r_info_type']!=1026:raise ValueError('Unexpected captured libc PLT relocation')
                if symbol['st_shndx']=='SHN_UNDEF':
                    if symbol['st_info']['bind']=='STB_WEAK':target=0
                    else:
                        if symbol.name not in external_by_name:
                            slot=max(external_names)+16
                            if slot>=EXTERNAL+external_end:raise ValueError('Captured libc import table exceeds reserved external range')
                            external_names[slot]=symbol.name;external_by_name[symbol.name]=slot
                        target=external_by_name[symbol.name]
                else:target=libc_base+symbol['st_value']
                uc.mem_write(libc_base+relocation['r_offset'],struct.pack('<Q',target+relocation['r_addend']));libc_plt_linked+=1
            for symbol in libc_symbols:
                if symbol['st_shndx']!='SHN_UNDEF' and symbol['st_info']['type']=='STT_FUNC':
                    libc_names[libc_base+symbol['st_value']]=symbol.name
                    if symbol.name in ('bsearch','pthread_cond_destroy','pthread_once'):
                        segment=next(s for s in libc_segments if s['p_vaddr']<=symbol['st_value']<s['p_vaddr']+s['p_filesz'])
                        position=symbol['st_value']-segment['p_vaddr'];function_bytes=segment.data()[position:position+symbol['st_size']]
                        native_libc[symbol.name]={'address':libc_base+symbol['st_value'],'rva':hex(symbol['st_value']),'size':symbol['st_size'],'function_sha256':hashlib.sha256(function_bytes).hexdigest(),'source_sha256':libc_digest}
        companion_info={'path':str(companion_memory),'sha256':digest,'base':hex(companion_base),'size':len(snapshot),'libc_source':str(libc_source),'libc_source_sha256':libc_digest,'libc_base':hex(libc_base),'libc_original_plt_relocations_linked':libc_plt_linked}
    # Append after all prior imports so saved PLT/import addresses remain stable.
    thread_return=max(external_names)+16
    if thread_return>=EXTERNAL+external_end:raise ValueError('Thread return import exceeds reserved range')
    external_names[thread_return]='_analysis_thread_return';external_by_name['_analysis_thread_return']=thread_return
    original_relocations=[]
    for r in relocations:
        where=BASE+r['r_offset'];kind=r['r_info_type'];symbol=symbols[r['r_info_sym']]
        original_relocations.append((r['r_offset'],bytes(uc.mem_read(where,8))))
        if kind==1027:value=BASE+r['r_addend']
        elif kind in (257,1025,1026):
            value=(dependency_symbols.get(symbol.name,EXTERNAL+r['r_info_sym']*16) if symbol['st_shndx']=='SHN_UNDEF' else BASE+symbol['st_value'])+r['r_addend']
        elif kind==0:continue
        else:raise ValueError(f'Unsupported relocation type {kind}')
        uc.mem_write(where,struct.pack('<Q',value&((1<<64)-1)))
    uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
    uc.reg_write(UC_ARM64_REG_TPIDR_EL0,TLS);uc.reg_write(UC_ARM64_REG_CPACR_EL1,3<<20)
    threads=NativeThreads(uc,TLS,thread_return)
    logs=[];invalid=[];heap=GuestHeap(uc,HEAP,HEAP_SIZE);metadata_info=None;import_counts=Counter();run_import_counts=Counter();printf_observations={};total_import_calls=0;lookup_info=None;lookup_payloads={}
    if recovered_binding:
        lookup_payload=recovered_binding.read_bytes()
        if len(lookup_payload)!=672:raise ValueError('Expected exactly 672 decoded binding bytes for this libg build')
        lookup_info={'source':str(recovered_binding),'sha256':hashlib.sha256(lookup_payload).hexdigest(),'size':len(lookup_payload),'key':'0x7e33eb5e','intervention':'The recovered native-decoded metadata supplies the known lookup result; the companion startup guard and tree are not initialized in this analysis snapshot.'}
        lookup_payloads[0x7e33eb5e]=lookup_payload
    if binding_index:
        index_data=binding_index.read_bytes();index=json.loads(index_data)
        if index['source_asset_sha256']!='834217feba3588c1f4d28e8dcf2a309c9baa1903eb5c08f535a628b50751d60b':raise ValueError('Unexpected protection metadata asset identity')
        for key,item in index['bindings'].items():
            key=int(key,0)&0xffffffff;payload=Path(item['path']).read_bytes()
            if len(payload)!=item['size'] or hashlib.sha256(payload).hexdigest()!=item['sha256']:raise ValueError(f'Binding hash/size mismatch: {key:x}')
            if key in lookup_payloads and lookup_payloads[key]!=payload:raise ValueError(f'Conflicting binding bytes: {key:x}')
            lookup_payloads[key]=payload
        lookup_info={'index':str(binding_index),'sha256':hashlib.sha256(index_data).hexdigest(),'source_asset_sha256':index['source_asset_sha256'],'keys':{hex(key):{'size':len(payload),'sha256':hashlib.sha256(payload).hexdigest()} for key,payload in lookup_payloads.items()},'intervention':'Original native-decoded bytes supply metadata lookups by original binding ID; companion startup guard and tree are not initialized in this snapshot.'}
    if metadata_block:
        if companion_base is None:raise ValueError('--metadata-block requires a companion snapshot')
        block=metadata_block.read_bytes()
        block_pointer=heap.allocate(align(len(block)+1))
        uc.mem_write(block_pointer,block)
        # Native buffer layout established by reserve/append and move routines:
        # pointer, capacity, length. The parser consumes singleton+0x98.
        uc.mem_write(companion_base+0x6cc258,struct.pack('<QQQ',block_pointer,len(block),len(block)))
        metadata_info={'path':str(metadata_block),'sha256':hashlib.sha256(block).hexdigest(),'size':len(block),'address':hex(block_pointer),'destination_rva':'0x6cc258','intervention':'Original block injected into recovered parser input buffer; upstream asset-reader path is not executed.'}
    files={};next_fd=10;error_strings={};thread_keys={};next_thread_key=0;dl_modules=[];dl_frames=[];file_mappings=[];mmap_pointer=0xa0000000
    apk_asset_index=None;apk_asset_info=[]
    maps=''.join(f'{BASE+s["p_vaddr"]:x}-{BASE+align(s["p_vaddr"]+s["p_memsz"]):x} r-xp {s["p_offset"]:08x} 00:00 1 /data/app/com.supercell.clashroyale/lib/arm64/libg.so\n' for s in segments).encode()+extra_maps
    def cstring(address,limit=0x100000):
        data=bytearray()
        while len(data)<limit:
            cursor=address+len(data);length=min(256,PAGE-(cursor&(PAGE-1)),limit-len(data))
            chunk=bytes(uc.mem_read(cursor,length));terminator=chunk.find(b'\0')
            if terminator>=0:data.extend(chunk[:terminator]);break
            data.extend(chunk)
        return bytes(data)
    def initialize_dl_modules():
        if dl_modules:return
        inputs=[(BASE,source,'/data/app/com.supercell.clashroyale/lib/arm64/libg.so')]
        if runtime_cxx:inputs.append((CXX_BASE,cxx_source,'/data/app/com.supercell.clashroyale/lib/arm64/libc++_shared.so'))
        if runtime_sentry:inputs.append((SENTRY_BASE,sentry_source,'/data/app/com.supercell.clashroyale/lib/arm64/libsentry.so'))
        if companion_memory:
            inputs.extend([(companion_base,ROOT/'analysis/native/lib/arm64-v8a/libsupercell_clashroyale.so','/data/app/com.supercell.clashroyale/lib/arm64/libsupercell_clashroyale.so'),(libc_base,libc_source,'/apex/com.android.runtime/lib64/bionic/libc.so')])
        for load_base,path,name in inputs:
            contents=path.read_bytes()
            with path.open('rb') as stream:
                module=ELFFile(stream);offset=module.header['e_phoff'];count=module.header['e_phnum'];entry_size=module.header['e_phentsize']
            if entry_size!=56:raise ValueError('Expected ELF64 program headers')
            name_bytes=name.encode()+b'\0';name_pointer=heap.allocate(len(name_bytes));uc.mem_write(name_pointer,name_bytes)
            headers=contents[offset:offset+count*entry_size];header_pointer=heap.allocate(len(headers));uc.mem_write(header_pointer,headers)
            info_pointer=heap.allocate(64)
            info=struct.pack('<QQQH6xQQQQ',load_base,name_pointer,header_pointer,count,len(inputs),0,0,0);uc.mem_write(info_pointer,info)
            dl_modules.append({'name':name,'base':hex(load_base),'source':str(path),'source_sha256':hashlib.sha256(contents).hexdigest(),'program_header_pointer':header_pointer,'program_header_count':count,'info_pointer':info_pointer,'info_size':len(info)})
    def dl_callback(emu):
        frame=dl_frames[-1];module=dl_modules[frame['index']]
        emu.reg_write(UC_ARM64_REG_X0,module['info_pointer']);emu.reg_write(UC_ARM64_REG_X1,module['info_size']);emu.reg_write(UC_ARM64_REG_X2,frame['data'])
        emu.reg_write(UC_ARM64_REG_X30,external_by_name['_analysis_dl_iterate_return']);emu.reg_write(UC_ARM64_REG_PC,frame['callback'])
    def dl_callback_return(emu):
        if not dl_frames:raise RuntimeError('Unexpected dl_iterate_phdr continuation')
        frame=dl_frames[-1];result=emu.reg_read(UC_ARM64_REG_X0)
        frame['index']+=1
        if not result and frame['index']<len(dl_modules):dl_callback(emu)
        else:
            dl_frames.pop();emu.reg_write(UC_ARM64_REG_X0,result);emu.reg_write(UC_ARM64_REG_PC,frame['return'])
    def imported(emu,address,size,data):
        nonlocal next_thread_key,mmap_pointer,total_import_calls,apk_asset_index
        name=libc_names.get(address)
        if name is None:
            if address not in external_names:raise RuntimeError(f'Unrecognized libc entry {address:x}')
            name=external_names[address]
        if name=='_analysis_dl_iterate_return':dl_callback_return(emu);return
        import_counts[name]+=1
        run_import_counts[name]+=1
        total_import_calls+=1
        sampled=total_import_calls<=1000 or run_import_counts[name]<=8 or import_counts[name]&(import_counts[name]-1)==0
        args=[emu.reg_read(UC_ARM64_REG_X0+i) for i in range(6 if sampled else HOT_IMPORT_ARITY.get(name,6))]
        invocation={'import':name,'address':hex(address),'caller':hex(emu.reg_read(UC_ARM64_REG_X30)),'arguments':[hex(a) for a in args]}
        if sampled:logs.append(invocation)
        if total_import_calls>100000000:raise RuntimeError('Import call limit (100 million across the retained initialization/resource/battle chain)')
        if name in native_libc:
            invocation['model']='execute original captured Android Bionic routine; callbacks, when present, remain original native code'
            invocation['native_function']=native_libc[name]
            if address!=native_libc[name]['address']:emu.reg_write(UC_ARM64_REG_PC,native_libc[name]['address'])
            return
        if name in ('memcpy','memmove','__memcpy_chk','__memmove_chk'):
            if name.endswith('_chk') and args[2]>args[3]:raise RuntimeError('Checked memory copy destination overflow')
            emu.mem_write(args[0],bytes(emu.mem_read(args[1],args[2])));result=args[0]
        elif name in ('memset','__memset_chk'):
            if name.endswith('_chk') and args[2]>args[3]:raise RuntimeError('Checked memory fill destination overflow')
            emu.mem_write(args[0],bytes([args[1]&255])*args[2]);result=args[0]
        elif name in ('malloc','calloc','realloc','_Znwm','_Znam'):
            size=args[0]*args[1] if name=='calloc' else args[1] if name=='realloc' else args[0]
            result=heap.allocate(size)
            if name=='calloc':emu.mem_write(result,b'\0'*size)
            if name=='realloc' and args[0]:
                if args[0] not in heap.allocations:raise RuntimeError(f'realloc source is not a modeled allocation: {args[0]:x}')
                emu.mem_write(result,bytes(emu.mem_read(args[0],min(size,heap.allocations[args[0]]))))
                heap.free(args[0])
        elif name=='posix_memalign':
            alignment,size=args[1:3]
            if alignment<8 or alignment&(alignment-1):result=22
            else:
                try:pointer=heap.allocate(size,alignment)
                except MemoryError:result=12
                else:
                    emu.mem_write(args[0],struct.pack('<Q',pointer));result=0
            invocation['model']='POSIX aligned allocation in the guest heap; returns EINVAL/ENOMEM directly and writes the result pointer only on success'
        elif name in ('free','_ZdlPv','_ZdaPv','_ZdlPvm','_ZdaPvm'):
            heap.free(args[0]);result=0
        elif name in ('mprotect','__android_log_print','__android_log_write'):result=0
        elif name=='strlen':result=len(cstring(args[0]))
        elif name in ('strcpy','stpcpy','__strcpy_chk','__stpcpy_chk'):
            payload=cstring(args[1])+b'\0'
            if name.endswith('_chk') and len(payload)>args[2]:raise RuntimeError('Checked string copy destination overflow')
            emu.mem_write(args[0],payload);result=args[0]+len(payload)-1 if 'stpcpy' in name else args[0]
        elif name in ('strchr','strrchr'):
            payload=cstring(args[0])+b'\0';needle=bytes([args[1]&255])
            match=payload.find(needle) if name=='strchr' else payload.rfind(needle);result=args[0]+match if match>=0 else 0
        elif name=='strnlen':result=len(cstring(args[0],args[1]))
        elif name=='__strlen_chk':
            result=len(cstring(args[0],args[1]))
            if result>=args[1]:raise RuntimeError('Checked string length reached unterminated buffer bound')
        elif name in ('atoi','atol','atoll','atof','strtol','strtoul','strtoll','strtoull','strtod','strtof','strtol_l','strtoul_l','strtoll_l','strtoull_l','strtod_l','strtof_l'):
            raw=cstring(args[0]);buffer=ctypes.create_string_buffer(raw)
            numeric_name=name[:-2] if name.endswith('_l') else name
            floating=numeric_name in ('atof','strtod','strtof')
            if name.endswith('_l'):
                locale_argument=args[2] if floating else args[3]
                if locale_argument!=TLS+0x180:raise RuntimeError(f'Numeric conversion received an unknown locale handle: {locale_argument:x}')
                invocation['locale']='explicit modeled C/POSIX locale'
            function=getattr(HOST_LIBC,numeric_name)
            restypes={'atoi':ctypes.c_int,'atol':ctypes.c_long,'atoll':ctypes.c_longlong,'atof':ctypes.c_double,'strtol':ctypes.c_long,'strtoul':ctypes.c_ulong,'strtoll':ctypes.c_longlong,'strtoull':ctypes.c_ulonglong,'strtod':ctypes.c_double,'strtof':ctypes.c_float}
            function.restype=restypes[numeric_name];ctypes.set_errno(0)
            if numeric_name.startswith('strto'):
                end_pointer=ctypes.c_void_p()
                function.argtypes=[ctypes.c_char_p,ctypes.POINTER(ctypes.c_void_p)]+([] if floating else [ctypes.c_int])
                result=function(buffer,ctypes.byref(end_pointer),*([] if floating else [ctypes.c_int(args[2])]))
                consumed=end_pointer.value-ctypes.addressof(buffer)
                if not 0<=consumed<=len(raw):raise RuntimeError('Numeric conversion end pointer outside input')
                if args[1]:emu.mem_write(args[1],struct.pack('<Q',args[0]+consumed))
            else:
                function.argtypes=[ctypes.c_char_p];result=function(buffer)
            errno=ctypes.get_errno()
            if errno:emu.mem_write(threads.tls+0x80,struct.pack('<I',errno))
            if floating:
                register=UC_ARM64_REG_S0 if numeric_name=='strtof' else UC_ARM64_REG_D0
                bits=struct.unpack('<I' if numeric_name=='strtof' else '<Q',struct.pack('<f' if numeric_name=='strtof' else '<d',result))[0]
                emu.reg_write(register,bits);result=0
            invocation['model']='C-locale numeric conversion of copied guest bytes; guest end pointer and ARM64 floating return registers marshalled explicitly'
        elif name in ('mmap','mmap64'):
            if vfs is None:raise RuntimeError('Virtual file manifest required for file mmap')
            if args[0] or args[2]!=1 or args[3] not in (1,2) or not 0<args[1]<=0x10000000 or args[5]%PAGE:raise RuntimeError(f'Unsupported file mmap arguments: {args}')
            payload=vfs.pread(args[4],args[1],args[5])
            if len(payload)!=args[1]:raise RuntimeError('File mmap beyond verified input bytes is outside the current model')
            length=align(args[1]);result=mmap_pointer;mmap_pointer+=length+PAGE
            emu.mem_map(result,length);emu.mem_write(result,payload);emu.mem_protect(result,length,1)
            file_mappings.append({'address':result,'length':length,'requested_length':args[1],'protection':1,'flags':args[3],'fd':args[4],'offset':args[5],'input_bytes_sha256':hashlib.sha256(payload).hexdigest()})
            invocation['mapping']=dict(file_mappings[-1]);invocation['model']='read-only native address mapping of hash-verified original file bytes; no database parsing substituted'
        elif name=='munmap':
            mapping=next((m for m in file_mappings if m['address']==args[0] and m['length']==align(args[1])),None)
            if mapping is None:raise RuntimeError('Unsupported partial or unknown file munmap')
            emu.mem_unmap(mapping['address'],mapping['length']);file_mappings.remove(mapping);result=0
        elif name=='madvise':
            if args[2] not in (0,1,2,3) or not any(m['address']<=args[0] and args[0]+args[1]<=m['address']+m['length'] for m in file_mappings):raise RuntimeError('Unsupported memory advice or range')
            invocation['model']='non-mutating access-pattern/prefetch hint on a completely resident read-only file mapping';result=0
        elif name in ('fopen','fopen64','fclose','fread','fread_unlocked','fseek','fseeko','fseeko64','ftell','ftello','ftello64','feof','ferror','clearerr','fileno','fflush'):
            if stdio is None:raise RuntimeError(f'Virtual file manifest required for {name}')
            def signed_offset(number):return number-(1<<64) if number&(1<<63) else number
            try:
                if name in ('fopen','fopen64'):
                    path=cstring(args[0]).decode('utf-8');mode=cstring(args[1]).decode('ascii');invocation.update(path=path,mode=mode)
                    handle=heap.allocate(32)
                    result=stdio.fopen(path,mode,handle)
                elif name=='fclose':result=stdio.fclose(args[0])
                elif name in ('fread','fread_unlocked'):
                    payload,result=stdio.fread(args[3],args[1],args[2]);emu.mem_write(args[0],payload)
                elif name in ('fseek','fseeko','fseeko64'):result=stdio.fseek(args[0],signed_offset(args[1]),args[2])
                elif name in ('ftell','ftello','ftello64'):result=stdio.ftell(args[0])
                else:result=getattr(stdio,name)(args[0])
            except OSError as error:
                emu.mem_write(threads.tls+0x80,struct.pack('<I',error.errno));invocation['errno']=error.errno
                result=0 if name in ('fopen','fopen64','fread','fread_unlocked') else -1
            invocation['model']='opaque read-only stdio stream over hash-verified virtual files; original native SQLite and database importer execute unchanged'
        elif name in ('stat','stat64','lstat','lstat64','fstat','fstat64','access','open','open64','__open_2','__open64_2','close','read','pread','pread64','lseek','lseek64'):
            if vfs is None:raise RuntimeError(f'Virtual file manifest required for {name}')
            def signed64(number):return number-(1<<64) if number&(1<<63) else number
            try:
                if name in ('stat','stat64','lstat','lstat64'):
                    path=cstring(args[0]).decode('utf-8');invocation['path']=path
                    emu.mem_write(args[1],vfs.stat(path));result=0
                elif name in ('fstat','fstat64'):emu.mem_write(args[1],vfs.fstat(args[0]));result=0
                elif name=='access':
                    path=cstring(args[0]).decode('utf-8');invocation['path']=path;result=vfs.access(path,args[1])
                elif name in ('open','open64','__open_2','__open64_2'):
                    path=cstring(args[0]).decode('utf-8');invocation['path']=path;result=vfs.open(path,args[1])
                elif name=='close':result=vfs.close(args[0])
                elif name in ('read','pread','pread64'):
                    chunk=vfs.read(args[0],args[2]) if name=='read' else vfs.pread(args[0],args[2],signed64(args[3]))
                    emu.mem_write(args[1],chunk);result=len(chunk)
                else:result=vfs.lseek(args[0],signed64(args[1]),args[2])
            except OSError as error:
                result=-1;emu.mem_write(threads.tls+0x80,struct.pack('<I',error.errno));invocation['errno']=error.errno
            invocation['model']='read-only virtual filesystem containing only hash-verified explicit source inputs; native SQLite and database parsing remain original'
        elif name=='getpagesize':result=PAGE
        elif name=='AAssetManager_open':
            # Only verified absence is modeled here. Present assets require a
            # byte-backed AAsset implementation; they must never appear missing.
            if apk_asset_index is None:
                apk_asset_index=set()
                sources={
                    'com.supercell.clashroyale.apk':'0d0dfed74fdb755b71f5424e70af9234ed25471c029d09863b657fed9057b13e',
                    'install_time_asset_pack.apk':'3e62afcf73786919be08b0a48f1744d82b618642fed8e8db7d1a76b5b2eb3fd3',
                    'config.arm64_v8a.apk':'ff4bebcf863fe24e78a49e73c09d9cc52a412a1bf737d61033290b96fc40dcf8',
                    'config.en.apk':'64d524e6a22b9b287c053f53168b1fcda2f2d662044376825f29d0308107e667',
                    'config.xxhdpi.apk':'b7d0162dd0f735bac5b13c3511a4109e9d3b9c7bdba7b7f966f436997fd3577d',
                }
                for filename,expected in sources.items():
                    path=ROOT/'analysis/apks'/filename
                    actual=hashlib.sha256(path.read_bytes()).hexdigest()
                    if actual!=expected:raise ValueError('Original APK source mismatch during packaged asset lookup')
                    with zipfile.ZipFile(path) as archive:names=sorted(n[7:] for n in archive.namelist() if n.startswith('assets/') and not n.endswith('/'))
                    apk_asset_index.update(names)
                    apk_asset_info.append({'path':str(path),'sha256':actual,'asset_count':len(names),'sorted_asset_names_sha256':hashlib.sha256(('\n'.join(names)+'\n').encode()).hexdigest()})
            filename=cstring(args[1]).decode('utf-8');invocation['asset_path']=filename
            if args[2]!=0:raise RuntimeError('Only the observed default packaged-asset mode is currently modeled')
            if filename in apk_asset_index:raise RuntimeError(f'Present packaged asset requires a byte-backed AAsset implementation: {filename}')
            invocation['model']='virtual read-only APK asset boundary: exact name absent from every hash-verified supplied split; return null. Android AssetManager/JNI itself is not initialized.'
            result=0
        elif name=='getenv':
            invocation['variable']=cstring(args[0]).decode('ascii')
            invocation['model']='empty virtual process environment; no host environment variables exposed';result=0
        elif name=='getauxval':
            auxiliary={6:PAGE,16:3,26:0}
            if args[0] not in auxiliary:raise RuntimeError(f'Unmodeled auxiliary vector key {args[0]}')
            invocation['model']='virtual ARM64 CPU: FP and ASIMD baseline capabilities, no optional HWCAP2 features'
            result=auxiliary[args[0]]
        elif name=='dlsym':
            symbol_name=cstring(args[1]).decode('ascii')
            if args[0] not in (0,0xffffffffffffffff):raise RuntimeError(f'Unmodeled dynamic library handle {args[0]:x}')
            result=game_symbols.get(symbol_name,dependency_symbols.get(symbol_name,external_by_name.get(symbol_name)))
            if result is None:raise RuntimeError(f'Unmodeled dynamic symbol {symbol_name}')
            invocation['symbol']=symbol_name
            invocation['model']='symbol lookup in explicitly mapped game, original C++ runtime and virtual libc import table'
        elif name=='dl_iterate_phdr':
            initialize_dl_modules()
            invocation['model']='original program headers of explicitly mapped ELF modules, delivered to the original native callback using the Android ARM64 dl_phdr_info ABI'
            invocation['module_count']=len(dl_modules)
            dl_frames.append({'callback':args[0],'data':args[1],'return':emu.reg_read(UC_ARM64_REG_X30),'index':0})
            dl_callback(emu);return
        elif name=='__system_property_get':
            key=cstring(args[0]).decode('ascii')
            if key!='ro.arch':raise RuntimeError(f'Unmodeled Android property {key}')
            invocation['model']='virtual Android environment has no optional ro.arch property'
            invocation['property']=key
            emu.mem_write(args[1],b'\0');result=0
        elif name=='newlocale':
            locale=cstring(args[1]).decode('ascii')
            if locale not in ('C','POSIX'):raise RuntimeError(f'Unmodeled locale {locale}')
            invocation['model']='opaque C/POSIX locale handle; locale services are modeled explicitly'
            invocation['locale']=locale;result=TLS+0x180
        elif name in ('duplocale','uselocale'):
            invocation['model']='single-thread C locale';result=TLS+0x180
        elif name=='freelocale':result=0
        elif name=='mbtowc' and not args[1]:
            invocation['model']='reset stateless C-locale multibyte conversion';result=0
        elif name=='__ctype_get_mb_cur_max':
            invocation['model']='C/POSIX locale maximum multibyte length';result=1
        elif name=='localeconv':
            invocation['model']='C locale lconv fields from captured Android libc initializer RVA0x54df0: decimal point, nine empty strings, fourteen 0xff bytes'
            emu.mem_write(TLS+0x400,b'.\0\0')
            emu.mem_write(TLS+0x300,struct.pack('<10Q',TLS+0x400,*([TLS+0x402]*9))+b'\xff'*14)
            result=TLS+0x300
        elif name=='sysconf':
            if args[0] in (0x27,0x28):result=PAGE
            elif args[0] in (0x60,0x61):result=1
            else:raise RuntimeError(f'Unmodeled Android sysconf query {args[0]:x}')
            invocation['model']='Android bits/sysconf.h constants: 4 KiB guest pages, one configured/online virtual processor; all other queries fail explicitly'
        elif name in ('__errno','__errno_location'):result=threads.tls+0x80
        elif name in ('getpid','gettid'):result=4591+(threads.current if name=='gettid' else 0)
        elif name=='time':
            invocation['model']='fixed virtual wall clock: Unix epoch seconds 1700000000'
            result=1700000000
            if args[0]:emu.mem_write(args[0],struct.pack('<q',result))
        elif name=='clock_gettime':
            if args[0] not in (0,1,4,7):raise RuntimeError(f'Unmodeled clock ID {args[0]}')
            nanoseconds=import_counts[name]*1000000
            seconds,nanoseconds=divmod(nanoseconds,1000000000)
            if args[0]==0:seconds+=1700000000
            emu.mem_write(args[1],struct.pack('<qq',seconds,nanoseconds));result=0
            invocation['model']='virtual clock advances 1 ms per query; realtime has fixed Unix epoch origin 1700000000'
        elif name=='gettimeofday':
            seconds,microseconds=divmod(import_counts[name]*1000,1000000)
            emu.mem_write(args[0],struct.pack('<qq',1700000000+seconds,microseconds))
            if args[1]:emu.mem_write(args[1],struct.pack('<ii',0,0))
            invocation['model']='virtual UTC wall clock advances 1 ms per query from fixed Unix epoch 1700000000';result=0
        elif name=='gmtime_r':
            seconds=struct.unpack('<q',emu.mem_read(args[0],8))[0];calendar=time.gmtime(seconds)
            emu.mem_write(TLS+0x480,b'UTC\0')
            fields=(calendar.tm_sec,calendar.tm_min,calendar.tm_hour,calendar.tm_mday,calendar.tm_mon-1,calendar.tm_year-1900,(calendar.tm_wday+1)%7,calendar.tm_yday-1,0)
            emu.mem_write(args[1],struct.pack('<9i4xqQ',*fields,0,TLS+0x480));result=args[1]
            invocation['model']='UTC time conversion with explicit Android ARM64 struct tm layout'
        elif name=='strftime':
            if args[1]>0x100000:raise RuntimeError('strftime output exceeds analysis bound')
            fields=struct.unpack('<9i4xqQ',emu.mem_read(args[3],56));zone=cstring(fields[-1]) if fields[-1] else None
            calendar=HostCalendar(*fields[:10],zone);buffer=ctypes.create_string_buffer(max(1,args[1]))
            function=HOST_LIBC.strftime;function.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_char_p,ctypes.POINTER(HostCalendar)];function.restype=ctypes.c_size_t
            fmt=cstring(args[2]);result=function(buffer,args[1],fmt,ctypes.byref(calendar))
            if args[1]:emu.mem_write(args[0],buffer.raw[:result+1] if result else buffer.raw[:args[1]])
            invocation['model']='C-locale time formatting with guest struct tm and zone string copied into host values';invocation['format']=fmt.decode('ascii',errors='replace')
        elif name=='uname':
            fields=('Linux','native-reference','4.4.0','offline ARM64 analysis','aarch64','(none)')
            emu.mem_write(args[0],b''.join(value.encode().ljust(65,b'\0') for value in fields));result=0
            invocation['model']='deterministic virtual ARM64 Linux utsname used by the original diagnostic SDK'
        elif name=='pthread_self':
            invocation['model']='opaque pthread identity at the current virtual thread TLS';result=threads.tls+0x100
        elif name=='pthread_equal':result=int(args[0]==args[1])
        elif name=='pthread_key_create':
            key=next_thread_key;next_thread_key+=1
            thread_keys[key]={'destructor':args[1],'values':{}}
            emu.mem_write(args[0],struct.pack('<I',key));result=0
            invocation['model']='process-local pthread key with isolated values per virtual thread'
        elif name=='pthread_key_delete':
            result=0 if args[0] in thread_keys else 22
            thread_keys.pop(args[0],None)
        elif name in ('pthread_getspecific','pthread_setspecific'):
            if args[0] not in thread_keys:raise RuntimeError(f'Unknown virtual pthread key {args[0]}')
            key=thread_keys[args[0]]
            if 'values' not in key:key['values']={'0':key.pop('value',0)}
            if name=='pthread_getspecific':result=key['values'].get(str(threads.current),0)
            else:key['values'][str(threads.current)]=args[1];result=0
        elif name=='pthread_create':
            if args[1]:raise RuntimeError('Explicit pthread attributes are not yet modeled')
            identity=threads.create(args[2],args[3]);emu.mem_write(args[0],struct.pack('<Q',identity));result=0
            invocation['model']='create runnable ARM64 callback with isolated stack/TLS on one virtual CPU; callback runs at the next blocking scheduling boundary'
        elif name in ('pthread_exit','_analysis_thread_return'):
            if any(key.get('destructor') and key.get('values',{}).get(str(threads.current),0) for key in thread_keys.values()):raise RuntimeError('Native thread exit requires TLS destructor execution')
            threads.exit(args[0]);return
        elif name=='pthread_join':
            result=threads.join(args[0],args[1])
            if result is None:return
        elif name=='pthread_cond_wait':
            threads.cond_wait(args[0],args[1]);return
        elif name=='pthread_mutexattr_init':
            emu.mem_write(args[0],b'\0'*4);result=0
        elif name=='pthread_mutexattr_settype':
            emu.mem_write(args[0],struct.pack('<I',args[1]));result=0
        elif name=='pthread_mutexattr_destroy':result=0
        elif name=='strerror':
            if args[0] not in error_strings:
                error_strings[args[0]]=heap.allocate(PAGE)
                emu.mem_write(error_strings[args[0]],os.strerror(args[0]).encode()+b'\0')
            invocation['model']='host Linux errno description used only for diagnostics string initialization';result=error_strings[args[0]]
        elif name=='syscall' and args[0] in (172,178):
            invocation['model']='virtual captured process ID and isolated thread IDs';result=4591+(threads.current if args[0]==178 else 0)
        elif name=='syscall' and args[0]==98 and args[2]&0x7f==1:
            invocation['model']='FUTEX_WAKE in a single-thread virtual process with no waiting threads';result=0
        elif name in ('pthread_mutex_lock','pthread_mutex_unlock','pthread_mutex_init','pthread_mutex_destroy'):
            if name=='pthread_mutex_lock':
                result=threads.mutex_lock(args[0])
                if result is None:return
            elif name=='pthread_mutex_unlock':result=threads.mutex_unlock(args[0])
            elif name=='pthread_mutex_init':
                kind=struct.unpack('<I',emu.mem_read(args[1],4))[0] if args[1] else 0
                threads.mutex_init(args[0],kind);result=0
            else:
                mutex=threads.mutexes.get(args[0])
                result=16 if mutex and mutex['owner'] is not None else 0
                if not result:threads.mutexes.pop(args[0],None)
            invocation['model']='one virtual CPU with tracked mutex ownership, recursive depth, blocking and wakeup'
        elif name in ('pthread_cond_broadcast','pthread_cond_signal'):
            threads.cond_signal(args[0],name=='pthread_cond_broadcast')
            invocation['model']='wake actual virtual condition waiters, reacquiring their mutex before returning';result=0
        elif name=='pthread_condattr_init':emu.mem_write(args[0],b'\0'*8);result=0
        elif name=='pthread_condattr_setclock':
            result=22 if args[1] not in (0,1) else 0
            if result==0:
                current=struct.unpack('<Q',emu.mem_read(args[0],8))[0]
                emu.mem_write(args[0],struct.pack('<Q',current|(args[1]<<1)))
        elif name=='pthread_condattr_destroy':emu.mem_write(args[0],struct.pack('<Q',0xdeada11d));result=0
        elif name=='pthread_cond_init':
            flags=struct.unpack('<I',emu.mem_read(args[1],4))[0]&3 if args[1] else 0
            emu.mem_write(args[0],struct.pack('<II',flags,0));result=0
        elif name=='__cxa_atexit':
            invocation['model']='record destructor registration; process teardown is outside this probe';result=0
        elif name=='memcmp':
            a,b=bytes(emu.mem_read(args[0],args[2])),bytes(emu.mem_read(args[1],args[2]));result=(a>b)-(a<b)
        elif name=='memchr':
            match=bytes(emu.mem_read(args[0],args[2])).find(bytes([args[1]&255]));result=args[0]+match if match>=0 else 0
        elif name in ('strcmp','strncmp','strcasecmp','strncasecmp'):
            limit=args[2] if name in ('strncmp','strncasecmp') else 0x100000
            a,b=cstring(args[0],limit),cstring(args[1],limit)
            if name in ('strcasecmp','strncasecmp'):a,b=a.lower(),b.lower()
            result=(a>b)-(a<b)
        elif name in ('vsnprintf','__vsnprintf_chk'):
            if name=='__vsnprintf_chk':
                if args[1]>args[3]:raise RuntimeError('Checked printf destination overflow')
                format_address,va_address=args[4],args[5]
            else:format_address,va_address=args[2],args[3]
            result,fmt=vsnprintf_arm64(emu,args[0],args[1],format_address,va_address,cstring)
            invocation['model']='AAPCS64 variadic arguments copied into host libc C-locale formatting; guest pointers are marshalled explicitly'
            invocation['format']=fmt.decode('utf-8',errors='replace')
            invocation['formatted']=cstring(args[0],args[1]).decode('utf-8',errors='replace') if args[1] else ''
            observation=printf_observations.setdefault(invocation['format'],{'calls':0,'examples':[]})
            observation['calls']+=1
            if len(observation['examples'])<3 and invocation['formatted'] not in observation['examples']:observation['examples'].append(invocation['formatted'])
        elif name in ('abort','__stack_chk_fail','exit','_exit'):
            raise RuntimeError(f'Initializer called {name}')
        else:raise RuntimeError(f'Unimplemented import {name}: {args}')
        emu.reg_write(UC_ARM64_REG_X0,result&((1<<64)-1));emu.reg_write(UC_ARM64_REG_PC,emu.reg_read(UC_ARM64_REG_X30))
    def interrupt(emu,number,data):
        nonlocal next_fd
        syscall=emu.reg_read(UC_ARM64_REG_X8)
        args=[emu.reg_read(UC_ARM64_REG_X0+i) for i in range(6)]
        logs.append({'interrupt':number,'syscall':syscall,'arguments':[hex(a) for a in args]})
        if syscall==226:emu.reg_write(UC_ARM64_REG_X0,0)
        elif syscall==98 and args[1]&0x7f==1:
            logs[-1]['model']='FUTEX_WAKE in a single-thread virtual process with no waiting threads';emu.reg_write(UC_ARM64_REG_X0,0)
        elif syscall in (172,178):
            logs[-1]['virtual_pid']=4591;emu.reg_write(UC_ARM64_REG_X0,4591+(threads.current if syscall==178 else 0))
        elif syscall==56:
            name=cstring(args[1]).decode('utf-8',errors='replace');logs[-1]['path']=name
            if name=='/proc/self/maps':content=maps
            elif name.endswith('/libg.so'):content=original
            else:raise RuntimeError(f'Unimplemented virtual file: {name}')
            files[next_fd]=[content,0];emu.reg_write(UC_ARM64_REG_X0,next_fd);next_fd+=1
        elif syscall==63:
            if args[0] not in files:raise RuntimeError(f'Unmapped file descriptor {args[0]}')
            content,position=files[args[0]];chunk=content[position:position+args[2]]
            emu.mem_write(args[1],chunk);files[args[0]][1]+=len(chunk);emu.reg_write(UC_ARM64_REG_X0,len(chunk))
        elif syscall==57:
            files.pop(args[0],None);emu.reg_write(UC_ARM64_REG_X0,0)
        elif syscall==62:
            if args[0] not in files:raise RuntimeError(f'Unmapped file descriptor {args[0]}')
            files[args[0]][1]=args[1];emu.reg_write(UC_ARM64_REG_X0,args[1])
        else:raise RuntimeError(f'Unimplemented syscall {syscall}')
    def missing(emu,access,address,size,value,data):
        invalid.append({'access':access,'address':hex(address),'size':size,'pc':hex(emu.reg_read(UC_ARM64_REG_PC))})
        return False
    uc.hook_add(UC_HOOK_CODE,imported,begin=EXTERNAL,end=EXTERNAL+external_end-1)
    if companion_memory:
        for address in libc_names:uc.hook_add(UC_HOOK_CODE,imported,begin=address,end=address)
        def companion_call(emu,address,size,data):
            logs.append({'companion_entry':hex(address-companion_base),'caller':hex(emu.reg_read(UC_ARM64_REG_X30)),'arguments':[hex(emu.reg_read(UC_ARM64_REG_X0+i)) for i in range(6)]})
        traced_returns={}
        def companion_return(emu,address,size,data):
            logs.append({'companion_return':hex(traced_returns[address]),'return_address':hex(address),'result':hex(emu.reg_read(UC_ARM64_REG_X0)),'hidden_result':hex(emu.reg_read(UC_ARM64_REG_X8))})
        def traced_call(emu,address,size,data):
            companion_call(emu,address,size,data)
            if address==companion_base+0x19fcec and lookup_payloads:
                argument=emu.reg_read(UC_ARM64_REG_X0)&0xffffffff
                key=argument^4591
                if key not in lookup_payloads:raise RuntimeError(f'Unexpected recovered metadata lookup key {key:x}')
                payload=lookup_payloads[key]
                destination=emu.reg_read(UC_ARM64_REG_X1)
                emu.mem_write(destination,payload)
                logs[-1]['recovered_metadata_copy']={'key':hex(key),'destination':hex(destination),'size':len(payload),'sha256':hashlib.sha256(payload).hexdigest()}
                emu.reg_write(UC_ARM64_REG_X0,0);emu.reg_write(UC_ARM64_REG_PC,emu.reg_read(UC_ARM64_REG_X30));return
            return_address=emu.reg_read(UC_ARM64_REG_X30)
            if return_address not in traced_returns:
                traced_returns[return_address]=address-companion_base
                uc.hook_add(UC_HOOK_CODE,companion_return,begin=return_address,end=return_address)
        for rva in (0x681780,0x19fee0,0x19fcec,0x19fd60,0x45ef98,0x559358,0x56b1f0,0x195eb4,0x195f9c,0x5595d8,0x559b50,0x55a6dc):uc.hook_add(UC_HOOK_CODE,traced_call,begin=companion_base+rva,end=companion_base+rva)
    uc.hook_add(UC_HOOK_INTR,interrupt);uc.hook_add(UC_HOOK_MEM_INVALID,missing)
    if source_digest in ('79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6','347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433'):
        def native_diagnostic(emu,address,size,data):
            entry={'native_diagnostic_rva':'0x128df18','message':cstring(emu.reg_read(UC_ARM64_REG_X0)).decode('utf-8',errors='replace'),'flag':emu.reg_read(UC_ARM64_REG_X1),'caller':hex(emu.reg_read(UC_ARM64_REG_X30)),'registers':{f'x{i}':hex(emu.reg_read(globals()[f'UC_ARM64_REG_X{i}'])) for i in range(19,25)}}
            frame=emu.reg_read(UC_ARM64_REG_X29);frames=[]
            for _ in range(8):
                if not STACK<=frame<STACK+0x400000-16:break
                next_frame,return_address=struct.unpack('<QQ',emu.mem_read(frame,16));frames.append({'frame':hex(frame),'return':hex(return_address),'saved_bytes':bytes(emu.mem_read(frame,64)).hex()});frame=next_frame
            entry['frames']=frames;logs.append(entry)
        uc.hook_add(UC_HOOK_CODE,native_diagnostic,begin=BASE+0x128df18,end=BASE+0x128df18)
    started=time.monotonic();failure=None;stages=[];phase='game_dt_init';binding_info=None;init_array_state=None;plan_state=None
    if resume_from:
        resume=json.loads((resume_from/'checkpoint.json').read_text())
        if resume['source_sha256']!=source_digest:raise ValueError('Checkpoint source mismatch')
        if resume['unicorn_version']!=unicorn.__version__:raise ValueError('Checkpoint Unicorn version mismatch')
        if bool(resume.get('cxx_runtime'))!=runtime_cxx:raise ValueError('Checkpoint C++ runtime configuration mismatch')
        if runtime_cxx:
            if resume['cxx_runtime']['source_sha256']!=cxx_info['source_sha256']:raise ValueError('Checkpoint C++ runtime source mismatch')
            cxx_info=resume['cxx_runtime']
        if bool(resume.get('sentry_runtime'))!=runtime_sentry:raise ValueError('Checkpoint Sentry runtime configuration mismatch')
        if runtime_sentry:
            if resume['sentry_runtime']['source_sha256']!=sentry_info['source_sha256']:raise ValueError('Checkpoint Sentry runtime source mismatch')
            sentry_info=resume['sentry_runtime']
        file_mappings=resume.get('file_mappings',[]);mmap_pointer=resume.get('mmap_pointer',0xa0000000)
        for mapping in file_mappings:uc.mem_map(mapping['address'],mapping['length'])
        for arena in resume.get('native_heap',{}).get('extra_heaps',[]):uc.mem_map(arena['base'],arena['capacity'])
        for region in resume['regions']:
            if region['file'].startswith('thread_'):uc.mem_map(int(region['base'],16),region['size'])
        for region in resume['regions']:
            data=(resume_from/region['file']).read_bytes()
            if hashlib.sha256(data).hexdigest()!=region['sha256']:raise ValueError('Checkpoint region hash mismatch')
            uc.mem_write(int(region['base'],16),data)
        for mapping in file_mappings:uc.mem_protect(mapping['address'],mapping['length'],mapping['protection'])
        context=uc.context_save();raw=(resume_from/'cpu-context.bin').read_bytes()
        if len(raw)!=context.size or hashlib.sha256(raw).hexdigest()!=resume['context_sha256']:raise ValueError('Checkpoint context mismatch')
        ctypes.memmove(context.context,raw,len(raw));uc.context_restore(context)
        if resume.get('native_threads'):threads.restore(resume['native_threads'])
        heap.restore(resume.pop('native_heap') if resume.get('native_heap') else {'primary_pointer':resume['heap_pointer'],'allocations':resume.pop('allocations')})
        import_counts.update(resume['import_counts']);total_import_calls=sum(import_counts.values());phase=resume['phase']
        thread_keys={int(key):item for key,item in resume.get('thread_keys',{}).items()};next_thread_key=resume.get('next_thread_key',0)
        dl_modules=resume.get('dl_modules',[]);dl_frames=resume.get('dl_frames',[])
        if resume.get('virtual_files'):
            if virtual_info is None or resume['virtual_files']['manifest_sha256']!=virtual_info['manifest_sha256']:raise ValueError('Checkpoint virtual filesystem input mismatch')
            vfs.restore(resume['virtual_files']['state'])
            if resume['virtual_files'].get('stdio'):stdio.restore(resume['virtual_files']['stdio'])
        init_array_state=resume.get('init_array')
        plan_state=resume.get('call_plan')
        stages.append({'phase':'resume','checkpoint':str(resume_from),'pc':hex(uc.reg_read(UC_ARM64_REG_PC))})
    try:
        if not run_init_array and entry_rva is None and not call_plan and 'DT_INIT' not in tags:raise ValueError('No DT_INIT; select an explicit constructor separately')
        if companion_init is not None and not resume_from:
            if companion_base is None:raise ValueError('--companion-init requires --companion-memory')
            phase='companion_init'
            if companion_sret:uc.reg_write(UC_ARM64_REG_X8,threads.tls+0x800)
            uc.emu_start(companion_base+companion_init,RETURN,timeout=timeout*1000000,count=max_instructions)
            stages.append({'phase':phase,'entry':hex(companion_init),'returned':uc.reg_read(UC_ARM64_REG_PC)==RETURN,'result':hex(uc.reg_read(UC_ARM64_REG_X0))})
            if uc.reg_read(UC_ARM64_REG_PC)!=RETURN:raise RuntimeError('Companion initializer did not return within execution bound')
        if decode_binding and not resume_from:
            if not companion_sret:raise ValueError('--decode-binding requires constructor hidden result via --companion-sret')
            codec=struct.unpack('<Q',uc.mem_read(threads.tls+0x800,8))[0]
            ciphertext=decode_binding.read_bytes();cipher_pointer=heap.allocate(len(ciphertext)+16)
            uc.mem_write(cipher_pointer,ciphertext)
            uc.mem_write(TLS+0xa00,struct.pack('<QQQ',((len(ciphertext)+16)&-16)|1,len(ciphertext),cipher_pointer))
            uc.mem_write(TLS+0xb00,b'\0'*128)
            uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
            uc.reg_write(UC_ARM64_REG_X0,codec);uc.reg_write(UC_ARM64_REG_X1,TLS+0xa00);uc.reg_write(UC_ARM64_REG_X8,TLS+0xb00)
            binding_info={'source':str(decode_binding),'source_sha256':hashlib.sha256(ciphertext).hexdigest(),'size':len(ciphertext),'codec':hex(codec),'entry_rva':'0x2170b0'}
            phase='binding_decode'
            uc.emu_start(companion_base+0x2170b0,RETURN,timeout=timeout*1000000,count=max_instructions)
            binding_info['returned']=uc.reg_read(UC_ARM64_REG_PC)==RETURN
            binding_info['result_descriptor']=bytes(uc.mem_read(TLS+0xb00,24)).hex()
            if not binding_info['returned']:raise RuntimeError('Binding decoder did not return within execution bound')
            pointer,capacity,length=struct.unpack('<QQQ',uc.mem_read(TLS+0xb00,24))
            if not 0<length<=capacity<=0x1000000:raise RuntimeError(f'Unexpected decoded buffer descriptor {pointer:x}, {capacity:x}, {length:x}')
            decoded=bytes(uc.mem_read(pointer,length));(out/'binding-decoded.bin').write_bytes(decoded)
            binding_info.update({'decoded_size':length,'decoded_sha256':hashlib.sha256(decoded).hexdigest(),'decoded':str(out/'binding-decoded.bin')})
        if not companion_init_only:
            if run_init_array:
                if runtime_cxx:
                    phase='cxx_init_array'
                    for index in range(cxx_info['init_array_completed'],cxx_info['init_array_count']):
                        pointer=struct.unpack('<Q',uc.mem_read(CXX_BASE+cxx_tags['DT_INIT_ARRAY']+index*8,8))[0]
                        if pointer in (0,0xffffffffffffffff):cxx_info['init_array_completed']=index+1;continue
                        continuing=cxx_info.get('in_progress')==index
                        if not continuing:uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                        cxx_info['in_progress']=index
                        uc.emu_start(uc.reg_read(UC_ARM64_REG_PC) if continuing else pointer,RETURN,timeout=timeout*1000000,count=max_instructions)
                        if uc.reg_read(UC_ARM64_REG_PC)!=RETURN:raise RuntimeError(f'C++ init-array constructor {index} did not return')
                        cxx_info['init_array_completed']=index+1;cxx_info['in_progress']=None
                if runtime_sentry:
                    phase='sentry_init_array'
                    for index in range(sentry_info['init_array_completed'],sentry_info['init_array_count']):
                        pointer=struct.unpack('<Q',uc.mem_read(SENTRY_BASE+sentry_tags['DT_INIT_ARRAY']+index*8,8))[0]
                        if pointer in (0,0xffffffffffffffff):sentry_info['init_array_completed']=index+1;continue
                        continuing=sentry_info.get('in_progress')==index
                        if not continuing:uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                        sentry_info['in_progress']=index
                        uc.emu_start(uc.reg_read(UC_ARM64_REG_PC) if continuing else pointer,RETURN,timeout=timeout*1000000,count=max_instructions)
                        if uc.reg_read(UC_ARM64_REG_PC)!=RETURN:raise RuntimeError(f'Sentry init-array constructor {index} did not return')
                        sentry_info['init_array_completed']=index+1;sentry_info['in_progress']=None
                if 'DT_INIT_ARRAY' not in tags:raise ValueError('No DT_INIT_ARRAY')
                count=tags['DT_INIT_ARRAYSZ']//8
                if init_array_state is None:init_array_state={'rva':hex(tags['DT_INIT_ARRAY']),'count':count,'completed':0,'in_progress':None,'calls':[]}
                if init_array_state['count']!=count:raise ValueError('Checkpoint init-array size mismatch')
                phase='game_init_array'
                for index in range(init_array_state['completed'],count):
                    pointer=struct.unpack('<Q',uc.mem_read(BASE+tags['DT_INIT_ARRAY']+index*8,8))[0]
                    if pointer in (0,0xffffffffffffffff):
                        init_array_state['calls'].append({'index':index,'skipped_sentinel':hex(pointer)})
                        init_array_state['completed']=index+1;continue
                    if not BASE<=pointer<BASE+end:raise ValueError(f'Init-array entry outside game image: {pointer:x}')
                    continuing=init_array_state['in_progress']==index
                    if not continuing:
                        uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                        for reg in range(8):uc.reg_write(globals()[f'UC_ARM64_REG_X{reg}'],0)
                    init_array_state['in_progress']=index
                    remaining=timeout-(time.monotonic()-started)
                    if remaining<=0:raise RuntimeError('Init-array wall-clock execution bound reached')
                    before=time.monotonic()
                    uc.emu_start(uc.reg_read(UC_ARM64_REG_PC) if continuing else pointer,RETURN,timeout=max(1,int(remaining*1000000)),count=max_instructions)
                    if uc.reg_read(UC_ARM64_REG_PC)!=RETURN:raise RuntimeError(f'Init-array constructor {index} did not return within execution bound')
                    init_array_state['calls'].append({'index':index,'rva':hex(pointer-BASE),'returned':True,'seconds':time.monotonic()-before})
                    init_array_state['completed']=index+1;init_array_state['in_progress']=None
            if call_plan:
                plan_bytes=call_plan.read_bytes();plan=json.loads(plan_bytes);plan_sha=hashlib.sha256(plan_bytes).hexdigest()
                if plan['source_sha256']!=source_digest:raise ValueError('Call-plan source mismatch')
                if plan_state is None or plan_state['sha256']!=plan_sha:
                    if plan_state and plan_state['in_progress'] is not None:raise ValueError('Cannot replace an interrupted call-plan')
                    values={}
                    if 'inherit_plan_sha256' in plan:
                        if not plan_state or plan['inherit_plan_sha256']!=plan_state['sha256']:raise ValueError('Inherited call-plan identity mismatch')
                        values=dict(plan_state['values'])
                    plan_state={'source':str(call_plan),'sha256':plan_sha,'values':values,'completed':0,'in_progress':None,'steps':[]}
                def value(argument):
                    if isinstance(argument,dict):return plan_state['values'][argument['ref']]+int(str(argument.get('offset',0)),0)
                    return int(str(argument),0)
                phase='game_call_plan'
                for index in range(plan_state['completed'],len(plan['steps'])):
                    step=plan['steps'][index];record={'index':index,'name':step['name']}
                    progress={'source_sha256':source_digest,'plan_sha256':plan_sha,'completed':plan_state['completed'],'total':len(plan['steps']),'current_step':index,'current_name':step['name'],'heap_bytes':heap.used_bytes,'import_calls':total_import_calls,'elapsed_seconds':time.monotonic()-started}
                    progress_file=out/'progress.json';temporary=progress_file.with_suffix('.tmp')
                    temporary.write_text(json.dumps(progress)+'\n');temporary.replace(progress_file)
                    if 'rva' in step:
                        address=BASE+value(step['rva']);arguments=[value(arg) for arg in step.get('args',[])]
                        stop=BASE+value(step['stop_rva']) if 'stop_rva' in step else RETURN
                        if 'stop_rva' in step:
                            if not address<stop<=BASE+end:raise ValueError('Invalid bounded native region')
                            region=original[offset(address-BASE):offset(address-BASE)+stop-address]
                            if hashlib.sha256(region).hexdigest()!=step.get('region_sha256'):raise ValueError('Bounded native region source hash mismatch')
                        if len(arguments)>8:raise ValueError('Call-plan supports at most eight register arguments')
                        continuing=plan_state['in_progress']==index
                        if not continuing:
                            uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                            for reg in range(8):uc.reg_write(globals()[f'UC_ARM64_REG_X{reg}'],arguments[reg] if reg<len(arguments) else 0)
                            uc.reg_write(UC_ARM64_REG_X8,value(step.get('sret',0)))
                        plan_state['in_progress']=index
                        before=time.monotonic()
                        threads.emulate(uc.reg_read(UC_ARM64_REG_PC) if continuing else address,stop,timeout=timeout*1000000,count=max_instructions)
                        if uc.reg_read(UC_ARM64_REG_PC)!=stop:raise RuntimeError(f'Call-plan step {index} did not return within execution bound')
                        result=uc.reg_read(UC_ARM64_REG_X0)
                        if step.get('expect_nonzero') and not result:raise RuntimeError(f'Call-plan step {index} returned null')
                        record.update(rva=hex(address-BASE),arguments=[hex(arg) for arg in arguments],returned=stop==RETURN,result=hex(result),seconds=time.monotonic()-before)
                        if stop!=RETURN:
                            record.update(boundary_reached=True,stop_rva=hex(stop-BASE),region_sha256=step['region_sha256'],method='Original instruction region executed to declared end; containing function did not return. Analysis call adapter regains control at that boundary.')
                            uc.reg_write(UC_ARM64_REG_PC,RETURN)
                    elif 'read_u64' in step:
                        address=value(step['read_u64']);result=struct.unpack('<Q',uc.mem_read(address,8))[0];record.update(address=hex(address),result=hex(result))
                    elif 'read_u32' in step:
                        address=value(step['read_u32']);result=struct.unpack('<I',uc.mem_read(address,4))[0];record.update(address=hex(address),result=hex(result))
                    elif 'write_u64' in step:
                        address=value(step['write_u64']);result=value(step['value'])
                        if not any(pointer<=address and address+8<=pointer+size for pointer,size in heap.allocations.items()):
                            raise ValueError('ABI input pointer write must be inside an allocated guest buffer')
                        uc.mem_write(address,struct.pack('<Q',result))
                        record.update(address=hex(address),result=hex(result),method='Explicit ABI input pointer in allocated guest storage; original game instructions remain unchanged')
                    elif 'size' in step or 'file' in step or 'data_hex' in step:
                        payload=Path(step['file']).read_bytes() if 'file' in step else bytes.fromhex(step.get('data_hex',''))
                        if 'file' in step:
                            digest=hashlib.sha256(payload).hexdigest()
                            if digest!=step['sha256']:raise ValueError(f'Input file hash mismatch at step {index}')
                            record.update(file=step['file'],sha256=digest)
                        if step.get('nul_terminate'):payload+=b'\0'
                        size=value(step.get('size',len(payload)))
                        if size<len(payload):raise ValueError('Call-plan allocation smaller than supplied input')
                        result=heap.allocate(size)
                        if payload:uc.mem_write(result,payload)
                        record.update(address=hex(result),size=size,input_bytes=len(payload))
                    else:raise ValueError(f'Unsupported call-plan step {index}')
                    plan_state['values'][step['name']]=result
                    plan_state['steps'].append(record);plan_state['completed']=index+1;plan_state['in_progress']=None
                for index,selection in enumerate(plan.get('inspect',[])):
                    address=value(selection['address']);length=value(selection['size'])
                    payload=bytes(uc.mem_read(address,length));filename=f'plan-inspect-{index}.bin';(out/filename).write_bytes(payload)
                    stages.append({'phase':'inspect','file':filename,'address':hex(address),'size':length,'sha256':hashlib.sha256(payload).hexdigest()})
            elif entry_rva is not None:
                phase='game_explicit_call'
                uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                for reg in range(8):uc.reg_write(globals()[f'UC_ARM64_REG_X{reg}'],entry_arguments[reg] if reg<len(entry_arguments) else 0)
                uc.emu_start(BASE+entry_rva,BASE+until_rva if until_rva is not None else RETURN,timeout=timeout*1000000,count=max_instructions)
                stages.append({'phase':phase,'entry_rva':hex(entry_rva),'arguments':[hex(value) for value in entry_arguments],'returned':uc.reg_read(UC_ARM64_REG_PC)==RETURN,'result':hex(uc.reg_read(UC_ARM64_REG_X0))})
            elif not run_init_array:
                phase='game_dt_init'
                if not resume_from:uc.reg_write(UC_ARM64_REG_SP,STACK+0x3ff000);uc.reg_write(UC_ARM64_REG_X30,RETURN)
                entry=uc.reg_read(UC_ARM64_REG_PC) if resume_from else BASE+tags['DT_INIT']
                uc.emu_start(entry,BASE+until_rva if until_rva is not None else RETURN,timeout=timeout*1000000,count=max_instructions)
    except Exception as error:failure=str(error)
    pc=uc.reg_read(UC_ARM64_REG_PC)
    if virtual_info is not None:
        virtual_info['state']=vfs.snapshot();virtual_info['stdio']=stdio.snapshot()
    if checkpoint_out:
        if files:raise RuntimeError('Checkpoint currently requires all virtual files closed')
        checkpoint_out.mkdir(parents=True,exist_ok=True);regions=[]
        for name,address,length in [('game',BASE,align(end)),('stack',STACK,0x400000),('tls',TLS,PAGE),('heap',HEAP,align(heap.primary_pointer-HEAP))]+([('cxx_runtime',CXX_BASE,cxx_end)] if runtime_cxx else [])+([('sentry_runtime',SENTRY_BASE,sentry_info['size'])] if runtime_sentry else [])+([('companion',companion_base,align(len(snapshot)))] if companion_memory else [])+[(f'file_mapping_{i:03d}',m['address'],m['length']) for i,m in enumerate(file_mappings)]+threads.regions()+[(name,address,align(length)) for name,address,length in heap.regions()]:
            data=bytes(uc.mem_read(address,length));filename=name+'.bin';(checkpoint_out/filename).write_bytes(data)
            regions.append({'file':filename,'base':hex(address),'size':length,'sha256':hashlib.sha256(data).hexdigest()})
        raw=bytes(uc.context_save());(checkpoint_out/'cpu-context.bin').write_bytes(raw)
        register_ids={**{f'x{i}':globals()[f'UC_ARM64_REG_X{i}'] for i in range(31)},**{f'q{i}':globals()[f'UC_ARM64_REG_Q{i}'] for i in range(32)},**{name:globals()['UC_ARM64_REG_'+name.upper()] for name in ('pc','sp','nzcv','fpcr','fpsr','tpidr_el0','cpacr_el1')}}
        checkpoint={'source_sha256':source_digest,'unicorn_version':unicorn.__version__,'context_sha256':hashlib.sha256(raw).hexdigest(),'regions':regions,'registers':{name:hex(uc.reg_read(identifier))for name,identifier in register_ids.items()},'heap_pointer':hex(heap.primary_pointer),'heap_total_bytes':heap.used_bytes,'native_heap':heap.snapshot(),'import_counts':dict(import_counts),'phase':phase,'returned':pc==RETURN,'failure':failure,'report_path':str(out/'report.json'),'init_array':init_array_state,'cxx_runtime':cxx_info,'sentry_runtime':sentry_info,'call_plan':plan_state,'thread_keys':thread_keys,'next_thread_key':next_thread_key,'dl_modules':dl_modules,'dl_frames':dl_frames,'virtual_files':virtual_info,'file_mappings':file_mappings,'mmap_pointer':mmap_pointer}
        checkpoint['native_threads']=threads.snapshot()
        unicorn_library=Path(unicorn.__file__).resolve().parent/'lib/libunicorn.so.2'
        checkpoint['emulation_abi']={'architecture':'ARM64 little endian','unicorn_library':str(unicorn_library),'unicorn_library_sha256':hashlib.sha256(unicorn_library.read_bytes()).hexdigest(),'external_base':EXTERNAL,'external_size':external_end,'return_address':RETURN,'call_stack_pointer':STACK+0x3ff000,'register_ids':register_ids,'import_symbols':{hex(address):name for address,name in external_names.items()},'primary_heap_base':HEAP,'primary_heap_capacity':HEAP_SIZE,'native_libc_delegates':native_libc,'scope':'Prepared native snapshot; imported functions are explicit OS models, not rewritten battle logic.'}
        (checkpoint_out/'checkpoint.json').write_text(json.dumps(checkpoint,indent=2)+'\n')
    result=bytearray(original);changes=[]
    for s in segments:
        memory=bytes(uc.mem_read(BASE+s['p_vaddr'],s['p_filesz']))
        result[s['p_offset']:s['p_offset']+s['p_filesz']]=memory
        before=original[s['p_offset']:s['p_offset']+s['p_filesz']]
        changes.append({'virtual_address':hex(s['p_vaddr']),'file_offset':hex(s['p_offset']),'size':len(memory),'changed_bytes_including_relocations':sum(a!=b for a,b in zip(before,memory))})
    for address,data in original_relocations:
        try:off=offset(address);result[off:off+8]=data
        except ValueError:pass
    suffix='call' if call_plan or entry_rva is not None else 'init-array' if run_init_array else 'dt-init'
    destination=out/(source.stem+'.'+suffix+'.so');destination.write_bytes(result)
    (out/(source.stem+'.memory.bin')).write_bytes(uc.mem_read(BASE,align(end)))
    if companion_memory:
        derived_companion=bytes(uc.mem_read(companion_base,len(snapshot)))
        (out/'companion.memory.bin').write_bytes(derived_companion)
        (out/'companion.memory.json').write_text(json.dumps({'base':hex(companion_base),'size':len(snapshot),'sha256':hashlib.sha256(derived_companion).hexdigest(),'source':companion_info,'phase':phase,'returned':pc==RETURN},indent=2)+'\n')
    if companion_sret:
        (out/'heap.memory.bin').write_bytes(uc.mem_read(HEAP,align(heap.primary_pointer-HEAP)))
    code=Cs(CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN)
    registers={f'x{i}':hex(uc.reg_read(UC_ARM64_REG_X0+i)) for i in range(29)}
    pointed={}
    for register,address in registers.items():
        try:pointed[register]=bytes(uc.mem_read(int(address,16),128)).hex()
        except UcError:pass
    report={'source':str(source),'source_sha256':source_digest,'derived':str(destination),'derived_sha256':hashlib.sha256(result).hexdigest(),
        'method':'Unicorn ARM64 selected native initialization; relocations applied then original relocation slots restored in output',
        'base':hex(BASE),'entry':hex(entry_rva if entry_rva is not None else tags.get('DT_INIT_ARRAY',0) if run_init_array else tags.get('DT_INIT',0)),'returned':pc==RETURN,'pc':hex(pc),'failure':failure,'seconds':time.monotonic()-started,'companion':companion_info,'phase':phase,'stages':stages,'init_array':init_array_state,'metadata_input':metadata_info,'binding_decode':binding_info,'recovered_metadata_lookup':lookup_info,'virtual_files':virtual_info,'file_mappings':file_mappings,'native_threads':threads.summary(),'selected_stop_rva':hex(until_rva) if until_rva is not None else None,'selected_boundary_reached':pc==BASE+until_rva if until_rva is not None else None,
        'segments':changes,'execution_model_sources':model_sources,'heap_capacity':HEAP_SIZE+GuestHeap.MAX_EXTRAS*GuestHeap.EXTRA_CAPACITY,'native_heap':heap.summary(),'total_import_calls':total_import_calls,'printf_observations':printf_observations,'packaged_asset_sources':apk_asset_info,'cxx_runtime':cxx_info,'sentry_runtime':sentry_info,'call_plan':plan_state,'dl_modules':dl_modules,'external_calls':logs,'external_call_counts':dict(import_counts),'external_call_sampling':'First 1000 chain imports, first 8 per symbol in this run, and cumulative power-of-two occurrence counts retained; all calls counted. Native traced entries/returns and syscalls retained.','invalid_accesses':invalid,'registers':registers,'pointed_bytes':pointed,
        'sret_scratch':{'address':hex(threads.tls+0x800),'bytes':bytes(uc.mem_read(threads.tls+0x800,128)).hex(),'heap_base':hex(HEAP),'heap_size':align(heap.primary_pointer-HEAP)} if companion_sret else None,
        'pc_disassembly':[f'{i.address:x} {i.mnemonic} {i.op_str}' for i in code.disasm(bytes(uc.mem_read(pc&-4,80)),pc&-4)] if BASE<=pc<BASE+align(end) else [],
        'limitations':['Only the explicitly selected native stages executed; this does not establish complete Android startup or a playable battle.','Imports/syscalls are explicit analysis models; no Android behavior equivalence is claimed.','Derived file remains partial if returned is false or initialization is incomplete.']}
    (out/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source',nargs='?',type=Path,default=ROOT/'analysis/native/lib/arm64-v8a/libg.so')
    parser.add_argument('--out',type=Path,default=ROOT/'analysis/native-unpack')
    parser.add_argument('--timeout',type=int,default=40)
    parser.add_argument('--instructions',type=int,default=100_000_000)
    parser.add_argument('--companion-memory',type=Path,help='Verified already-relocated companion memory snapshot; its original capture base is preserved')
    parser.add_argument('--companion-init',type=lambda s:int(s,0),help='Execute this companion RVA before the game initializer')
    parser.add_argument('--companion-init-only',action='store_true',help='Probe only the explicitly selected companion initializer')
    parser.add_argument('--metadata-block',type=Path,help='Original signed protobuf block injected into the recovered parser input buffer for analysis')
    parser.add_argument('--companion-sret',action='store_true',help='Provide a zeroed hidden X8 result buffer for an explicitly selected native function')
    parser.add_argument('--decode-binding',type=Path,help='Execute original binding decoder after an explicitly selected codec constructor')
    parser.add_argument('--recovered-binding',type=Path,help='Supply the native-decoded metadata for the verified libg lookup ID; records this analysis intervention')
    parser.add_argument('--checkpoint-out',type=Path,help='Save complete CPU context, architectural registers and all mutable mapped memory after the bounded call')
    parser.add_argument('--resume-from',type=Path,help='Resume a verified checkpoint produced by this script and exact Unicorn version')
    parser.add_argument('--until-rva',type=lambda s:int(s,0),help='Stop before this explicit game RVA instead of the outer initializer return')
    parser.add_argument('--run-init-array',action='store_true',help='Execute original DT_INIT_ARRAY constructors in order, skipping DT_INIT; use an already decrypted ELF')
    parser.add_argument('--entry-rva',type=lambda s:int(s,0),help='Execute this game routine instead of DT_INIT, optionally after init-array constructors')
    parser.add_argument('--entry-arg',type=lambda s:int(s,0),action='append',default=[],help='Explicit X0..X7 argument value, repeat in register order')
    parser.add_argument('--runtime-cxx',action='store_true',help='Map and link the original packaged ARM64 libc++_shared.so, executing its constructors before game constructors')
    parser.add_argument('--call-plan',type=Path,help='Source-bound JSON sequence of explicit native calls and input buffers; never invokes DT_INIT implicitly')
    parser.add_argument('--binding-index',type=Path,help='Verified native-decoded protection binding index; supplies original metadata by exact lookup ID')
    parser.add_argument('--runtime-sentry',action='store_true',help='Map and link the original packaged libsentry.so; execute its original constructors with --run-init-array')
    parser.add_argument('--virtual-files',type=Path,help='Hash-verified read-only virtual file bindings; native file and SQLite code remain original')
    args=parser.parse_args();unpack(args.source,args.out,args.timeout,args.instructions,args.companion_memory,args.companion_init,args.companion_init_only,args.metadata_block,args.companion_sret,args.decode_binding,args.recovered_binding,args.checkpoint_out,args.resume_from,args.until_rva,args.run_init_array,args.entry_rva,tuple(args.entry_arg),args.runtime_cxx,args.call_plan,args.binding_index,args.runtime_sentry,args.virtual_files)
