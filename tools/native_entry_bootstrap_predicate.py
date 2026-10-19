#!/usr/bin/env python3
"""Observe the first checksum equality in captured bootstrap code offline."""
from pathlib import Path
import hashlib
import json
from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_INVALID
from unicorn.arm64_const import *

root = Path(__file__).resolve().parents[1]
source = root/'analysis/android-runtime/memory/companion-live-parent-memory.bin'
manifest_path = source.with_suffix('.json')
manifest = json.loads(manifest_path.read_text())
memory = source.read_bytes()
if hashlib.sha256(memory).hexdigest() != manifest['sha256']:
    raise RuntimeError('Captured memory digest mismatch')
base = int(manifest['base'],16)
emu = Uc(UC_ARCH_ARM64,UC_MODE_ARM)
emu.mem_map(base,(len(memory)+4095)&-4096)
emu.mem_write(base,memory)
emu.mem_map(0x70000000,0x200000)
emu.mem_map(0x71000000,0x1000)
emu.mem_map(0x72000000,0x1000)
emu.reg_write(UC_ARM64_REG_SP,0x701fff00)
emu.reg_write(UC_ARM64_REG_TPIDR_EL0,0x71000000)
emu.reg_write(UC_ARM64_REG_CPACR_EL1,3<<20)
emu.reg_write(UC_ARM64_REG_X0,0x72000000)
emu.reg_write(UC_ARM64_REG_X1,0)
count=0
outcome={'status':'instruction_limit'}
calls=[]
def code(uc,address,size,_):
    global count,outcome
    count+=1
    rva=address-base
    if rva==0x6572fc:
        left=uc.reg_read(UC_ARM64_REG_W8)
        right=uc.reg_read(UC_ARM64_REG_W0)
        outcome={'status':'predicate_observed','rva':hex(rva),'instruction':'cmp w8,w0','aggregate_w8':hex(left),'probe_b_w0':hex(right),'equal':left==right}
        uc.emu_stop()
    if not base<=address<base+len(memory):
        raise RuntimeError(f'Bootstrap left supplied image at {address:x}')
def invalid(uc,access,address,size,value,_):
    raise RuntimeError(f'Unmapped memory {access} address={address:x} PC={uc.reg_read(UC_ARM64_REG_PC)-base:x}')
emu.hook_add(UC_HOOK_CODE,code)
emu.hook_add(UC_HOOK_MEM_INVALID,invalid)
try:
    emu.emu_start(base+0x65440c,0,count=10000000)
except Exception as error:
    outcome={'status':'error','reason':str(error),'pc_rva':hex(emu.reg_read(UC_ARM64_REG_PC)-base)}
report={'source':str(source),'sha256':manifest['sha256'],'entry_rva':'0x65440c','runtime_base':hex(base),'instructions':count,'outcome':outcome,'limitations':['Runs only until the first checksum predicate in bounded Unicorn; no Android or host imports execute.','The VM argument is an unused synthetic pointer before this predicate; no JNI behavior is modeled.','Passing this predicate does not establish that later native startup checks pass.']}
destination=root/'analysis/native-entry/bootstrap-first-predicate.json'
destination.write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
