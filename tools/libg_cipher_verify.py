#!/usr/bin/env python3
"""Differentially verify translated libg loader from a retained Unicorn checkpoint."""
import argparse,ctypes,hashlib,json,sys,time
from pathlib import Path
from unicorn import *
from unicorn.arm64_const import *
from libg_cipher_cfg import ROOT,graph
BASE=0x10000000
class Q(ctypes.Structure):_fields_=[('lo',ctypes.c_uint64),('hi',ctypes.c_uint64)]
class State(ctypes.Structure):_fields_=[('x',ctypes.c_uint64*31),('sp',ctypes.c_uint64),('q',Q*32),('pc',ctypes.c_uint64),('nzcv',ctypes.c_uint64),('status',ctypes.c_uint64),('reserved_padding',ctypes.c_uint64)]
def xreg(i):return globals()[f'UC_ARM64_REG_X{i}']
def load_checkpoint(path):
 j=json.loads(path.read_text()); assert j['source_sha256']=='347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433'
 regions=[]
 for r in j['regions']:
  raw=(path.parent/r['file']).read_bytes();assert hashlib.sha256(raw).hexdigest()==r['sha256'];regions.append((int(r['base'],16),raw))
 return j,regions

def state_from(j):
 s=State();r=j['registers']
 if ctypes.addressof(s)%16:raise RuntimeError('Native State must be16-byte aligned')
 for i in range(31):s.x[i]=int(r[f'x{i}'],16)
 for i in range(32):
  v=int(r[f'q{i}'],16);s.q[i].lo=v&((1<<64)-1);s.q[i].hi=v>>64
 s.pc=int(r['pc'],16)-BASE;s.sp=int(r['sp'],16);s.nzcv=int(r['nzcv'],16)
 return s

def run(checkpoint,library,out,budgets):
 j,regions=load_checkpoint(checkpoint);raw,ins,*_=graph(ROOT/'analysis/native/lib/arm64-v8a/libg.so',0x1adfe4c)
 conditional={BASE+p for p,i in ins.items() if i.mnemonic.startswith('b.') or i.mnemonic in ('cbz','cbnz')}
 lib=ctypes.CDLL(str(library.resolve()));lib.libg_loader_run.argtypes=[ctypes.POINTER(State),ctypes.c_uint64];lib.libg_loader_run.restype=ctypes.c_int
 libc=ctypes.CDLL(None,use_errno=True);libc.mmap.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_long];libc.mmap.restype=ctypes.c_void_p
 for base,raw in regions:
  if not raw:continue
  ptr=libc.mmap(base,len(raw),3,0x22|0x100000,-1,0)
  if ptr!=base:raise RuntimeError(f'Could not reserve guestaddress {base:x}: {ctypes.get_errno()}')
 results=[]
 for budget in budgets:
  uc=Uc(UC_ARCH_ARM64,UC_MODE_ARM)
  for base,raw in regions:
   if not raw:continue
   ctypes.memmove(base,raw,len(raw));uc.mem_map(base,len(raw));uc.mem_write(base,raw)
  s=state_from(j);r=j['registers']
  for i in range(31):uc.reg_write(xreg(i),s.x[i])
  for i in range(32):uc.reg_write(UC_ARM64_REG_Q0+i,s.q[i].lo|(s.q[i].hi<<64))
  for name in ('sp','nzcv','fpcr','fpsr','tpidr_el0','cpacr_el1'):uc.reg_write(globals()['UC_ARM64_REG_'+name.upper()],int(r[name],16))
  ticks=0;stopped=None
  def hook(u,address,size,data):
   nonlocal ticks,stopped
   if address in conditional:
    ticks+=1
    if ticks>=budget:stopped=address;u.emu_stop()
   elif address-BASE in ins and ins[address-BASE].mnemonic in ('bl','ret','br'):
    stopped=address;u.emu_stop()
  uc.hook_add(UC_HOOK_CODE,hook)
  start=time.monotonic();uc.emu_start(int(r['pc'],16),0,count=10000000);ut=time.monotonic()-start
  start=time.monotonic();status=lib.libg_loader_run(ctypes.byref(s),budget);ct=time.monotonic()-start
  mismatches={}
  for i in range(31):
   a=uc.reg_read(xreg(i));b=s.x[i]
   if a!=b:mismatches[f'x{i}']=[hex(a),hex(b)]
  for i in range(32):
   a=uc.reg_read(UC_ARM64_REG_Q0+i);b=s.q[i].lo|(s.q[i].hi<<64)
   if a!=b:mismatches[f'q{i}']=[hex(a),hex(b)]
  for k,a,b in [('pc',uc.reg_read(UC_ARM64_REG_PC),s.pc+BASE),('sp',uc.reg_read(UC_ARM64_REG_SP),s.sp),('nzcv',uc.reg_read(UC_ARM64_REG_NZCV),s.nzcv)]:
   if a!=b:mismatches[k]=[hex(a),hex(b)]
  for base,raw in regions:
   if not raw:continue
   a=bytes(uc.mem_read(base,len(raw)));b=ctypes.string_at(base,len(raw))
   if a!=b:
    offsets=[k for k,(x,y) in enumerate(zip(a,b)) if x!=y]
    mismatches[f'memory_{base:x}']={'differences':len(offsets),'first':hex(offsets[0]),'last':hex(offsets[-1])}
  result={'conditional_budget':budget,'ticks':ticks,'status':status,'pc':hex(s.pc+BASE),'unicorn_seconds':ut,'translated_seconds':ct,'mismatches':mismatches}
  results.append(result);print(json.dumps(result),flush=True)
  if mismatches:break
 out.mkdir(parents=True,exist_ok=True);(out/'differential-verification.json').write_text(json.dumps({'checkpoint':str(checkpoint),'translation_sha256':hashlib.sha256(library.read_bytes()).hexdigest(),'results':results,'all_equal':all(not x['mismatches'] for x in results)},indent=2)+'\n')
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--checkpoint',type=Path,default=ROOT/'analysis/native-unpack/game-checkpoint/state/checkpoint.json');p.add_argument('--library',type=Path,default=ROOT/'analysis/libg-cipher/loader-translated.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/libg-cipher');p.add_argument('--budgets',type=int,nargs='+',default=[1,10,100,1000]);a=p.parse_args();run(a.checkpoint,a.library,a.out,a.budgets)
