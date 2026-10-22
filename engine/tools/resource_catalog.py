#!/usr/bin/env python3
"""Recover exact path/table-ID descriptors from the original native catalog code."""
import argparse,csv,hashlib,json
from pathlib import Path
from capstone import *
from capstone.arm64_const import ARM64_OP_REG,ARM64_OP_IMM
from native_engine_index import ROOT,EXPECTED

def build(source,out):
 b=source.read_bytes();assert hashlib.sha256(b).hexdigest()==EXPECTED
 rows=list(csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t'));functions={int(r['start'],16):int(r['size']) for r in rows};m=Cs(CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN);m.detail=True;records=[]
 for start in (0xe69930,0xe0cf80):
  regs={};pending=[];recent=[]
  for i in m.disasm(b[start:start+functions[start]],start):
   recent.append(f'0x{i.address:x} {i.mnemonic} {i.op_str}');recent=recent[-18:]
   o=i.operands
   def rn(op):
    name=i.reg_name(op.reg)
    if name in ('wzr','xzr'):return 31
    if name[0] in ('w','x') and name[1:].isdigit():return int(name[1:])
    return None
   def value(op):return op.imm if op.type==ARM64_OP_IMM else 0 if op.type==ARM64_OP_REG and rn(op)==31 else regs.get(rn(op)) if op.type==ARM64_OP_REG else None
   if i.mnemonic in ('adrp','adr'):
    regs[rn(o[0])]=o[1].imm
   elif i.mnemonic=='mov':regs[rn(o[0])]=value(o[1])
   elif i.mnemonic=='add' and len(o)==3 and o[2].type==ARM64_OP_IMM:
    x=value(o[1]);regs[rn(o[0])]=None if x is None else x+(o[2].imm<<o[2].shift.value)
   elif i.mnemonic=='bl':
    target=o[0].imm
    if target==0x142fde4:
     p=regs.get(1);text=None
     if p is not None and 0x2b1680<=p<0x3cc210:
      end=b.find(b'\0',p,p+1024)
      if end>=0:text=b[p:end].decode('utf8')
     pending.append({'literal_rva':hex(p) if p is not None else None,'path':text,'string_constructor_call':hex(i.address)})
    elif target in (0xe14440,0xe14408,0xe14450):
     pair=target==0xe14408;empty=target==0xe14450;idreg=1 if empty else 3 if pair else 2;flagreg=idreg+1
     paths=[] if empty else pending[-(2 if pair else 1):]
     records.append({'catalog_function':hex(start),'descriptor_constructor':hex(target),'constructor_call':hex(i.address),'table_id':regs.get(idreg),'flags':regs.get(flagreg),'paths':paths,'instructions':recent.copy()});pending=[]
    for n in range(19):regs.pop(n,None)
   else:
    try:
     for reg in i.regs_access()[1]:
      name=i.reg_name(reg)
      if len(name)>1 and name[0] in ('x','w') and name[1:].isdigit():regs.pop(int(name[1:]),None)
    except CsError:pass
  
 for r in records:
  for p in r['paths']:
   if p['path']:
    decoded=ROOT/'analysis/decoded/assets'/p['path'];p['decoded_exists']=decoded.exists()
    if decoded.exists():p['decoded_sha256']=hashlib.sha256(decoded.read_bytes()).hexdigest()
 result={'source':str(source),'source_sha256':EXPECTED,'method':'Linear constant tracking through original exact-FDE ARM64 catalog instructions; filenames captured at original String constructor calls; IDs/flags captured at descriptor constructors','records':records,'record_count':len(records),'unknown_ids':sum(r['table_id'] is None for r in records),'unknown_flags':sum(r['flags'] is None for r in records),'limitations':['Catalog helpers beyond these two bounded functions or dynamic global String paths are not resolved.','The integerflags are retained as observed; no higher-level format enum semantics are assumed.']};out.parent.mkdir(exist_ok=True,parents=True);out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='records'},indent=2));print(json.dumps([r for r in records if r['table_id'] in [72,121,54,10,26,27,28]],indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--source',type=Path,default=ROOT/'analysis/libg-cipher/libg.decrypted.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/native-focused/resources/catalog-descriptors.json');a=p.parse_args();build(a.source,a.out)
