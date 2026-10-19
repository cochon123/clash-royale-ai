#!/usr/bin/env python3
"""Check concrete battle-deck pointer-array operations against original ARM64."""
import argparse,hashlib,json,random,struct,subprocess
from pathlib import Path
from unicorn import Uc,UC_ARCH_ARM64,UC_MODE_ARM
from unicorn.arm64_const import *
from native_engine_index import ROOT,EXPECTED
BASE=0x10000000;OBJECT=0x20000000;ARRAY=0x21000000;RETURN=0x30000000;STACK=0x40000000

def run(source,out,rust_binary=None):
 b=source.read_bytes();assert hashlib.sha256(b).hexdigest()==EXPECTED
 uc=Uc(UC_ARCH_ARM64,UC_MODE_ARM)
 uc.mem_map(BASE+0xfec000,0x2000);uc.mem_write(BASE+0xfec000,b[0xfec000:0xfee000]);uc.mem_map(OBJECT,0x1000);uc.mem_map(ARRAY,0x1000);uc.mem_map(RETURN,0x1000);uc.mem_map(STACK,0x10000);uc.reg_write(UC_ARM64_REG_SP,STACK+0x8000)
 def call(rva,*args):
  for i,x in enumerate(args):uc.reg_write(UC_ARM64_REG_X0+i,x&((1<<64)-1))
  uc.reg_write(UC_ARM64_REG_X30,RETURN);uc.emu_start(BASE+rva,RETURN,count=1000)
  assert uc.reg_read(UC_ARM64_REG_PC)==RETURN
  return uc.reg_read(UC_ARM64_REG_W0)
 def call_x(rva,*args):
  for i,x in enumerate(args):uc.reg_write(UC_ARM64_REG_X0+i,x&((1<<64)-1))
  uc.reg_write(UC_ARM64_REG_X30,RETURN);uc.reg_write(UC_ARM64_REG_SP,STACK+0x8000);uc.emu_start(BASE+rva,RETURN,count=1000)
  assert uc.reg_read(UC_ARM64_REG_PC)==RETURN
  return uc.reg_read(UC_ARM64_REG_X0)
 rust=None;rust_comparisons=0
 if rust_binary:
  rust=subprocess.Popen([str(rust_binary.resolve())],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
 def compare_rust(request,actual,operation,case):
  nonlocal rust_comparisons
  if rust is None:return
  rust.stdin.write(json.dumps(request)+'\n');rust.stdin.flush()
  response=rust.stdout.readline()
  if not response:raise RuntimeError('Rust probe exited before producing an output')
  response=json.loads(response);rust_comparisons+=1
  if response.get('result')!=actual:failures.append({'operation':operation,'case':case,'native':actual,'rust':response})
 rng=random.Random(0x160402020);vectors=[];counts={'swap_pointer_array_20':0,'find_pointer_array_40':0,'load_u32_2c':0,'load_u32_4c':0,'all_nonzero_array_40':0,'any_nonzero_array_40':0,'load_u64_08':0,'load_array_40':0};failures=[]
 for t in range(1024):
  n=rng.randrange(1,33);values=[rng.choice([0,rng.getrandbits(64)]) for _ in range(n)];i=rng.randrange(n);j=rng.randrange(n)
  uc.mem_write(OBJECT+0x20,struct.pack('<Q',ARRAY));uc.mem_write(ARRAY,struct.pack('<'+'Q'*n,*values));call(0xfec180,OBJECT,i,j);actual=list(struct.unpack('<'+'Q'*n,bytes(uc.mem_read(ARRAY,n*8))));expected=values.copy();expected[i],expected[j]=expected[j],expected[i]
  if actual!=expected:failures.append({'operation':'swap_pointer_array_20','case':t})
  compare_rust({'op':'swap_array_20','values':values,'first':i,'second':j},actual,'swap_array_20',t)
  counts['swap_pointer_array_20']+=1
  if t<16:vectors.append({'operation':'swap_pointer_array_20','function_rva':'0xfec180','values':values,'indices':[i,j],'native_result':actual})
  # Signed nonpositive counts intentionally avoid dereferencing array data.
  n=rng.randrange(-3,33);values=[rng.choice([0,rng.getrandbits(64)]) for _ in range(max(n,0))];needle=rng.choice(values+[0,rng.getrandbits(64)])
  uc.mem_write(OBJECT+0x40,struct.pack('<Q',ARRAY));uc.mem_write(OBJECT+0x4c,struct.pack('<i',n));uc.mem_write(ARRAY,struct.pack('<'+'Q'*len(values),*values) if values else b'\x00'*8)
  raw=call(0xfec49c,OBJECT,needle);actual=raw if raw<1<<31 else raw-(1<<32);expected=next((k for k,x in enumerate(values) if x==needle),-1)
  if actual!=expected:failures.append({'operation':'find_pointer_array_40','case':t,'actual':actual,'expected':expected})
  compare_rust({'op':'find_array_40','values':values,'count':n,'needle':needle},actual,'find_array_40',t)
  counts['find_pointer_array_40']+=1
  if t<16:vectors.append({'operation':'find_pointer_array_40','function_rva':'0xfec49c','signed_count':n,'values':values,'needle':needle,'native_result':actual})
  for offset,rva in [(0x2c,0xfeda88),(0x4c,0xfeda90)]:
   value=rng.getrandbits(32);uc.mem_write(OBJECT+offset,struct.pack('<I',value));actual=call(rva,OBJECT)
   if actual!=value:failures.append({'operation':f'load_u32_{offset:x}','case':t})
   compare_rust({'op':f'load_field_{offset:x}','prefix':list(bytes(uc.mem_read(OBJECT,0x50)))},actual,f'load_field_{offset:x}',t)
   counts[f'load_u32_{offset:x}']+=1
 pred=random.Random(0x160402021)
 for t in range(1024):
  n=pred.randrange(-3,33);values=[pred.choice([0,pred.getrandbits(64)]) for _ in range(max(n,0))]
  uc.mem_write(OBJECT+0x40,struct.pack('<Q',ARRAY));uc.mem_write(OBJECT+0x4c,struct.pack('<i',n));uc.mem_write(ARRAY,struct.pack('<'+'Q'*len(values),*values) if values else b'\x00'*8)
  for rva,operation,expected in ((0xfec320,'all_nonzero_array_40',n<=0 or all(x!=0 for x in values)),(0xfec370,'any_nonzero_array_40',n>0 and any(x!=0 for x in values))):
   actual=call(rva,OBJECT)
   if (actual==1)!=expected:failures.append({'operation':operation,'case':t,'actual':actual,'expected':int(expected)})
   compare_rust({'op':operation,'values':values,'count':n},actual,operation,t)
   counts[operation]+=1
   if t<8:vectors.append({'operation':operation,'function_rva':hex(rva),'signed_count':n,'values':values,'native_result':actual})
  value=pred.getrandbits(64);uc.mem_write(OBJECT+8,struct.pack('<Q',value));actual=call_x(0xfeda80,OBJECT)
  if actual!=value:failures.append({'operation':'load_u64_08','case':t,'actual':actual,'expected':value})
  compare_rust({'op':'load_field_08','prefix':list(bytes(uc.mem_read(OBJECT,0x50)))},actual,'load_field_08',t)
  counts['load_u64_08']+=1
  n=pred.randrange(1,33);values=[pred.getrandbits(64) for _ in range(n)];index=pred.randrange(n)
  uc.mem_write(OBJECT+0x40,struct.pack('<Q',ARRAY));uc.mem_write(OBJECT+0x4c,struct.pack('<i',n));uc.mem_write(ARRAY,struct.pack('<'+'Q'*n,*values))
  actual=call_x(0xfec3c0,OBJECT,index)
  if actual!=values[index]:failures.append({'operation':'load_array_40','case':t,'actual':actual,'expected':values[index]})
  compare_rust({'op':'load_array_40','values':values,'count':n,'index':index},actual,'load_array_40',t)
  counts['load_array_40']+=1
  if t<8:vectors.append({'operation':'load_array_40','function_rva':'0xfec3c0','signed_count':n,'index':index,'values':values,'native_result':actual})
 if rust:
  rust.stdin.close()
  if rust.wait(timeout=5)!=0:raise RuntimeError('Rust differential probe failed')
 result={'source':str(source),'source_sha256':EXPECTED,'method':'Original recovered ARM64 instructions executed in Unicorn2.1.4 with controlled object and array memory; no imports or game startup','counts':counts,'failures':failures,'all_equal':not failures,'rust_binary':str(rust_binary)if rust_binary else None,'rust_binary_sha256':hashlib.sha256(rust_binary.read_bytes()).hexdigest()if rust_binary else None,'rust_comparisons':rust_comparisons,'interpretation':['0xfec180 swaps two8-byte entries in the array addressed by object+0x20; indices use signed32-bit extension. Tests use valid allocated indices.','0xfec49c scans the pointer array at object+0x40 using signed32-bit count at object+0x4c; returns first match or signed32-bit -1; nonpositive counts return -1.','0xfeda88 and0xfeda90 load32-bit fields at object+0x2c and+0x4c. Their higher-level field names remain unproven.','0xfec320 and 0xfec370 scan the array at object+0x40 using the signed count at +0x4c. Counts below 1 return true and false respectively without reading entries.','0xfeda80 loads the 64-bit field at object+0x08. 0xfec3c0 loads an in-range identity from the array at +0x40; its assertion-and-out-of-range path is not executed.'],'limitations':['Controlled memory probes verify these operations only, not full battle semantics, actual card identities or valid game-state invariants.','Pointer equality is represented by exact64-bit values; pointed-to objects are not dereferenced.','0xfec3c0 tests cover only indices inside the signed count. The native assertion helper and the following unguarded load are not reproduced.'],'vectors':vectors}
 out.parent.mkdir(exist_ok=True,parents=True);out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='vectors'},indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--source',type=Path,default=ROOT/'analysis/libg-cipher/libg.decrypted.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/native-engine-index/deck-operation-native-probes.json');p.add_argument('--rust-binary',type=Path);a=p.parse_args();run(a.source,a.out,a.rust_binary)
