#!/usr/bin/env python3
"""Bounded original ARM64 references to literal resource paths or direct calls."""
import argparse,array,bisect,csv,hashlib,json,struct
from pathlib import Path
from capstone import *
from native_engine_index import ROOT,EXPECTED

def scan(queries,addresses,source,out):
 b=source.read_bytes();assert hashlib.sha256(b).hexdigest()==EXPECTED
 literals={}
 for m in __import__('re').finditer(rb'[\x20-\x7e]{4,}\0',b[0x2b1680:0x3cc210]):
  text=m.group()[:-1].decode();va=0x2b1680+m.start()
  if any(q in text for q in queries):literals[va]=text
 targets=set(literals)|set(addresses);rows=list(csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t'));fs=[(int(r['start'],16),int(r['end_exclusive'],16)) for r in rows];starts=[x[0] for x in fs]
 def owner(a):
  n=bisect.bisect_right(starts,a)-1
  return hex(fs[n][0]) if n>=0 and a<fs[n][1] else None
 def sign(n,k):return n-(1<<k) if n&(1<<(k-1)) else n
 md=Cs(CS_ARCH_ARM64,CS_MODE_LITTLE_ENDIAN);md.detail=True
 w=array.array('I');w.frombytes(b[0x7706b0:0x187d110]);refs=[]
 for n,v in enumerate(w):
  pc=0x7706b0+n*4
  if v&0x7c000000==0x14000000:
   target=pc+(sign(v&0x3ffffff,26)<<2)
   if target in targets:refs.append({'kind':'BL' if v&(1<<31) else 'B','location':hex(pc),'target':hex(target),'function':owner(pc)})
  if v&0x9f000000!=0x90000000:continue
  rd=v&31;page=(pc&~4095)+(sign(((v>>5)&0x7ffff)<<2|((v>>29)&3),21)<<12)
  for d in range(1,min(9,len(w)-n)):
   v2=w[n+d]
   if v2&0xff000000!=0x91000000 or (v2>>5)&31!=rd:continue
   target=page+((v2>>10)&0xfff)*(4096 if v2&(1<<22) else 1)
   if target not in targets:continue
   at=pc+d*4;ins=list(md.disasm(b[pc:at+4],pc));valid=True
   for i in ins[1:-1]:
    if i.mnemonic in ('b','bl','br','blr','ret') or i.mnemonic.startswith('b.'):valid=False;break
    if any(md.reg_name(r) in (f'x{rd}',f'w{rd}') for r in i.regs_access()[1]):valid=False;break
   if valid:refs.append({'kind':'ADRP_ADD','location':hex(at),'target':hex(target),'literal':literals.get(target),'function':owner(at),'instructions':[f'{i.address:x} {i.mnemonic} {i.op_str}' for i in ins]})
 result={'source':str(source),'source_sha256':EXPECTED,'queries':queries,'literals':{hex(k):v for k,v in literals.items()},'references':refs,'limitations':['Only exact literal starts and validated linearADRP/ADD sequences within8instructions are scanned; directBLscan covers originalunwindtext range.']};out.parent.mkdir(exist_ok=True,parents=True);out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--query',nargs='*',default=[]);p.add_argument('--address',nargs='*',default=[]);p.add_argument('--source',type=Path,default=ROOT/'analysis/libg-cipher/libg.decrypted.so');p.add_argument('--out',type=Path,required=True);a=p.parse_args();scan(a.query,[int(s,16) for s in a.address],a.source,a.out)
