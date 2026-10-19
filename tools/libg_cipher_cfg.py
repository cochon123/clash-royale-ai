#!/usr/bin/env python3
"""Bounded CFG and source-bound instruction inventory for original libg loader."""
import argparse,collections,hashlib,json
from pathlib import Path
from capstone import *
from capstone.arm64_const import *
ROOT=Path(__file__).resolve().parents[1]
def graph(source,entry):
 raw=source.read_bytes(); md=Cs(CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN);md.detail=True
 todo=[entry];seen={};calls=[];outside=[]
 while todo:
  pc=todo.pop()
  if pc in seen:continue
  if not 0x1ab8000<=pc<0x1ae3c20:
   outside.append(pc);continue
  off=pc-0x8000
  decoded=list(md.disasm(raw[off:off+4],pc,count=1))
  if not decoded:raise ValueError(f'Invalid instruction at {pc:x}')
  i=decoded[0];seen[pc]=i
  if i.mnemonic in ('ret','br'):continue
  elif i.mnemonic=='bl':calls.append(i.operands[0].imm);todo.append(pc+4)
  elif i.mnemonic=='b':todo.append(i.operands[0].imm)
  elif i.mnemonic.startswith('b.') or i.mnemonic in ('cbz','cbnz','tbz','tbnz'):
   todo.extend((pc+4,i.operands[-1].imm))
  else:todo.append(pc+4)
 return raw,seen,calls,outside
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--entry',type=lambda x:int(x,0),default=0x1adfe4c);p.add_argument('--source',type=Path,default=ROOT/'analysis/native/lib/arm64-v8a/libg.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/libg-cipher');a=p.parse_args()
 raw,ins,calls,outside=graph(a.source,a.entry);a.out.mkdir(exist_ok=True,parents=True)
 (a.out/'reachable-disassembly.txt').write_text('\n'.join(f'{i.address:x} {i.mnemonic} {i.op_str}' for _,i in sorted(ins.items()))+'\n')
 report={'source':str(a.source),'sha256':hashlib.sha256(raw).hexdigest(),'entry':hex(a.entry),'instruction_count':len(ins),'mnemonics':dict(collections.Counter(i.mnemonic for i in ins.values())),'direct_calls':[hex(x) for x in calls],'outside_branches':[hex(x) for x in outside],'indirect_terminators':[f'{i.address:x} {i.mnemonic} {i.op_str}' for i in ins.values() if i.mnemonic in ('ret','br')]}
 (a.out/'cfg.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
