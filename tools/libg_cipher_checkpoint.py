#!/usr/bin/env python3
"""Advance a verified libg checkpoint using translated original loader instructions.

Source checkpoints stay immutable. The native translation stops immediately
before an original BL/RET and exports a normal Unicorn 2.1.4 checkpoint. Resume
that output with native_unpack.py for original out-of-function execution.
"""
import argparse,ctypes,hashlib,json,time
from pathlib import Path
import unicorn
from unicorn import Uc,UC_ARCH_ARM64,UC_MODE_ARM
from unicorn.arm64_const import *
from libg_cipher_verify import ROOT,State,load_checkpoint,state_from,xreg,BASE

def advance(checkpoint,library,out,budget):
 if out.exists():raise ValueError('Choose a new output directory; existing checkpoint outputs are never overwritten')
 verified=json.loads((library.parent/'differential-verification.json').read_text())
 digest=hashlib.sha256(library.read_bytes()).hexdigest()
 if not verified['all_equal'] or digest!=verified['translation_sha256']:raise ValueError('Translation does not have matching successful differential verification')
 j,regions=load_checkpoint(checkpoint)
 if j['unicorn_version']!=unicorn.__version__:raise ValueError('Unicorn version mismatch')
 raw_context=(checkpoint.parent/'cpu-context.bin').read_bytes()
 if hashlib.sha256(raw_context).hexdigest()!=j['context_sha256']:raise ValueError('Source CPU context SHA256 mismatch')
 lib=ctypes.CDLL(str(library.resolve()));lib.libg_loader_run.argtypes=[ctypes.POINTER(State),ctypes.c_uint64];lib.libg_loader_run.restype=ctypes.c_int
 libc=ctypes.CDLL(None,use_errno=True);libc.mmap.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_long];libc.mmap.restype=ctypes.c_void_p
 for base,raw in regions:
  if not raw:continue
  ptr=libc.mmap(base,len(raw),3,0x22|0x100000,-1,0)
  if ptr!=base:raise RuntimeError(f'Could not reserve guestaddress {base:x}: errno {ctypes.get_errno()}; mapping uses MAP_FIXED_NOREPLACE')
  ctypes.memmove(base,raw,len(raw))
 s=state_from(j);start=time.monotonic();status=lib.libg_loader_run(ctypes.byref(s),budget);seconds=time.monotonic()-start
 if status==3:raise ValueError(f'PC {s.pc:x} is outside translated original CFG')
 uc=Uc(UC_ARCH_ARM64,UC_MODE_ARM)
 for base,raw in regions:
  if not raw:continue
  uc.mem_map(base,len(raw));uc.mem_write(base,ctypes.string_at(base,len(raw)))
 context=uc.context_save()
 if len(raw_context)!=context.size:raise ValueError('CPU context size mismatch')
 ctypes.memmove(context.context,raw_context,len(raw_context));uc.context_restore(context)
 for i in range(31):uc.reg_write(xreg(i),s.x[i])
 for i in range(32):uc.reg_write(UC_ARM64_REG_Q0+i,s.q[i].lo|(s.q[i].hi<<64))
 uc.reg_write(UC_ARM64_REG_PC,s.pc+BASE);uc.reg_write(UC_ARM64_REG_SP,s.sp);uc.reg_write(UC_ARM64_REG_NZCV,s.nzcv)
 out.mkdir(parents=True)
 newregions=[];changed=[]
 for old,(base,raw) in zip(j['regions'],regions):
  data=ctypes.string_at(base,len(raw)) if raw else b'';file=old['file'];(out/file).write_bytes(data)
  newregions.append({**old,'sha256':hashlib.sha256(data).hexdigest()})
  n=sum(x!=y for x,y in zip(raw,data))
  if n:changed.append({'file':file,'base':hex(base),'changed_bytes':n})
 raw_context=bytes(uc.context_save());(out/'cpu-context.bin').write_bytes(raw_context)
 r=j['registers'].copy()
 for i in range(31):r[f'x{i}']=hex(s.x[i])
 for i in range(32):r[f'q{i}']=hex(s.q[i].lo|(s.q[i].hi<<64))
 r.update(pc=hex(s.pc+BASE),sp=hex(s.sp),nzcv=hex(s.nzcv))
 proof={'method':'Faithful translation of original bounded ARM64 loader CFG to host scalar/NEON C++ operations','source_checkpoint':str(checkpoint.resolve()),'source_checkpoint_sha256':hashlib.sha256(checkpoint.read_bytes()).hexdigest(),'library':str(library.resolve()),'library_sha256':digest,'validation':str((library.parent/'differential-verification.json').resolve()),'seconds':seconds,'conditional_budget':budget,'stop_status':status,'stop_meaning':{0:'before budget-th conditional branch',1:'before original RET',2:'before original BL'}[status],'pc':r['pc'],'changed_memory':changed,'limitations':['Validation covers the retained checkpoint and bounded paths; this is not a proof of all possible initializer paths.','Calls outside this original function are not executed; resume with original Unicorn emulation.','Original runtime startup guards and battle logic remain outside this translation.']}
 new={**j,'regions':newregions,'context_sha256':hashlib.sha256(raw_context).hexdigest(),'registers':r,'failure':None,'returned':False,'acceleration':proof}
 (out/'checkpoint.json').write_text(json.dumps(new,indent=2)+'\n');(out/'acceleration.json').write_text(json.dumps(proof,indent=2)+'\n');print(json.dumps(proof,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('checkpoint',type=Path);p.add_argument('--library',type=Path,default=ROOT/'analysis/libg-cipher/loader-translated.so');p.add_argument('--out',type=Path,required=True);p.add_argument('--budget',type=int,default=1_000_000_000);a=p.parse_args();advance(a.checkpoint,a.library,a.out,a.budget)
