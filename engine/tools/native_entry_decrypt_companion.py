#!/usr/bin/env python3
"""Reproduce the recovered companion text-decryption stage in Unicorn.

Uses a verified captured image and its native decryptor. Virtual mprotect and
errno functions implement only analysis memory permissions and an errno cell.
No native Android/host import or target process executes.
"""
from pathlib import Path
import hashlib
import json
import struct
import time
from elftools.elf.elffile import ELFFile
from unicorn import Uc,UC_ARCH_ARM64,UC_MODE_ARM,UC_HOOK_CODE,UC_HOOK_MEM_INVALID,UC_HOOK_INTR
from unicorn.arm64_const import *

root=Path(__file__).resolve().parents[1]
directory=root/'analysis/native-entry/companion-text-decrypt'
directory.mkdir(parents=True,exist_ok=True)
source=root/'analysis/android-runtime/memory/companion-live-parent-memory.bin'
manifest=json.loads(source.with_suffix('.json').read_text())
memory=source.read_bytes()
if hashlib.sha256(memory).hexdigest()!=manifest['sha256']:
    raise RuntimeError('Captured memory SHA-256 mismatch')
base=int(manifest['base'],16)
libc_base=0x7fda15ac7000
libc_path=root/'analysis/android-runtime/libc-arm64.so'
emu=Uc(UC_ARCH_ARM64,UC_MODE_ARM)
emu.mem_map(base,(len(memory)+4095)&-4096)
emu.mem_write(base,memory)
emu.mem_map(0x70000000,0x400000)
emu.mem_map(0x71000000,0x1000)
emu.mem_map(0x72000000,0x10000)
emu.mem_map(0x73000000,0x1000000)
emu.mem_map(libc_base,0x100000)
emu.mem_write(0x72000000,struct.pack('<Q',4096))
emu.reg_write(UC_ARM64_REG_SP,0x703fff00)
emu.reg_write(UC_ARM64_REG_TPIDR_EL0,0x71000000)
emu.reg_write(UC_ARM64_REG_CPACR_EL1,3<<20)
emu.reg_write(UC_ARM64_REG_X0,0x72000000)
emu.reg_write(UC_ARM64_REG_X30,0x7200fff0)
names={}
with libc_path.open('rb')as stream:
    elf=ELFFile(stream)
    dynamic=next(seg for seg in elf.iter_segments()if seg['p_type']=='PT_DYNAMIC')
    for sym in dynamic.iter_symbols():
        if sym['st_shndx']!='SHN_UNDEF' and sym['st_info']['type']=='STT_FUNC':
            names[libc_base+sym['st_value']]=sym.name
calls=[]
heap=0x73000000
failure=None
stages=[]
def external(uc,address,size,_):
    global heap
    name=names.get(address)
    args=[uc.reg_read(getattr(__import__('unicorn.arm64_const',fromlist=['x']),f'UC_ARM64_REG_X{i}'))for i in range(6)]
    calls.append({'name':name,'address':hex(address),'caller_rva':hex(uc.reg_read(UC_ARM64_REG_X30)-base),'args':[hex(a)for a in args]})
    if name in ['__errno','__errno_location']:
        result=0x71000080
    elif name=='mprotect':
        start=args[0]&-4096
        end=(args[0]+args[1]+4095)&-4096
        uc.mem_protect(start,end-start,args[2]&7)
        result=0
        stages.append({'stage':'virtual_mprotect','rva':hex(args[0]-base),'size':args[1],'protection':args[2]})
        (directory/f'memory-at-mprotect-{len(stages)}.bin').write_bytes(uc.mem_read(base,len(memory)))
    elif name in ['memcpy','memmove','__memcpy_chk','__memmove_chk']:
        uc.mem_write(args[0],bytes(uc.mem_read(args[1],args[2])))
        result=args[0]
    elif name in ['memset','__memset_chk']:
        uc.mem_write(args[0],bytes([args[1]&255])*args[2])
        result=args[0]
    elif name in ['malloc','calloc']:
        length=args[0]*args[1]if name=='calloc'else args[0]
        result=heap
        heap+=(length+4095)&-4096
        if heap>=0x74000000:raise RuntimeError('Virtual heap limit')
    elif name=='free':
        result=0
    else:
        raise RuntimeError(f'Unmodeled external {name} at {address:x}, args {args}')
    uc.reg_write(UC_ARM64_REG_X0,result)
    uc.reg_write(UC_ARM64_REG_PC,uc.reg_read(UC_ARM64_REG_X30))
def invalid(uc,access,address,size,value,_):
    raise RuntimeError(f'Invalid memory {access} {address:x}, PC_RVA={uc.reg_read(UC_ARM64_REG_PC)-base:x}')
def syscall(uc,number,_):
    syscall_number=uc.reg_read(UC_ARM64_REG_X8)
    args=[uc.reg_read(UC_ARM64_REG_X0+i)for i in range(6)]
    calls.append({'syscall':syscall_number,'interrupt':number,'pc_rva':hex(uc.reg_read(UC_ARM64_REG_PC)-base),'args':[hex(a)for a in args]})
    if syscall_number==226:
        start=args[0]&-4096
        end=(args[0]+args[1]+4095)&-4096
        uc.mem_protect(start,end-start,args[2]&7)
        uc.reg_write(UC_ARM64_REG_X0,0)
        stages.append({'stage':'virtual_mprotect_syscall','rva':hex(args[0]-base),'size':args[1],'protection':args[2]})
        (directory/f'memory-at-mprotect-{len(stages)}.bin').write_bytes(uc.mem_read(base,len(memory)))
    else:raise RuntimeError(f'Unmodeled syscall {syscall_number}')
emu.hook_add(UC_HOOK_CODE,external,begin=libc_base,end=libc_base+0xfffff)
emu.hook_add(UC_HOOK_MEM_INVALID,invalid)
emu.hook_add(UC_HOOK_INTR,syscall)
started=time.monotonic()
try:
    emu.emu_start(base+0x65bb90,0x7200fff0,timeout=40000000,count=1500000000)
except Exception as error:
    failure=str(error)
derived=bytes(emu.mem_read(base,len(memory)))
(directory/'companion-decrypted-memory.bin').write_bytes(derived)
region_start=0x18d0a0
region_size=0x4a2a28
changed=sum(a!=b for a,b in zip(memory[region_start:region_start+region_size],derived[region_start:region_start+region_size]))
result={'source':str(source),'source_sha256':manifest['sha256'],'derived':str(directory/'companion-decrypted-memory.bin'),'derived_sha256':hashlib.sha256(derived).hexdigest(),'entry_rva':'0x65bb90','returned':emu.reg_read(UC_ARM64_REG_PC)==0x7200fff0,'pc_rva':hex(emu.reg_read(UC_ARM64_REG_PC)-base),'return_value':hex(emu.reg_read(UC_ARM64_REG_X0)),'failure':failure,'seconds':time.monotonic()-started,'region_rva':hex(region_start),'region_size':region_size,'region_changed_bytes':changed,'calls':calls,'stages':stages,'limitations':['Runs captured native companion decryptor offline; no Android or host native import executes.','mprotect changes only Unicorn memory permissions; errno and allocation use explicit virtual state.','Recovered bytes require further verification before being treated as faithful executable behavior.']}
(directory/'report.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
