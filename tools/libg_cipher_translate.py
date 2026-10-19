#!/usr/bin/env python3
"""Translate the original bounded libg loader CFG to exact host C operations.

This is instruction translation, not a guessed high-level cipher. All calls out
of the original function stop before executing so the owner can resume Unicorn.
Memory is supplied at original guest virtual addresses by the caller.
"""
from pathlib import Path
import argparse,hashlib,json,re,subprocess,sys
from libg_cipher_cfg import graph,ROOT
from capstone.arm64_const import ARM64_OP_REG,ARM64_OP_IMM,ARM64_OP_MEM

def split(s):
 out=[];start=0;depth=0
 for i,ch in enumerate(s):
  if ch=='[':depth+=1
  elif ch==']':depth-=1
  elif ch==',' and depth==0:out.append(s[start:i]);start=i+1
 out.append(s[start:]);return out
def mask(n):return f'((U128(1)<<{n})-1)' if n<128 else '(~U128(0))'
def reg(r):
 r=r.strip()
 if r in ('wzr','xzr'):return '0'
 if r=='sp':return 'sp'
 if r.startswith('w'):return f'uint32_t(x{r[1:]})'
 if r.startswith('x'):return r
 if r.startswith('d'):return f'uint64_t(v{r[1:]})'
 if r.startswith('q'):return f'v{r[1:]}'
 if r.startswith('v'):
  m=re.fullmatch(r'v(\d+)\.(\d+)([bsd])',r)
  if m:
   n=int(m[2])*dict(b=8,s=32,d=64)[m[3]]
   return f'(v{m[1]} & {mask(n)})'
  m=re.fullmatch(r'v(\d+)\.d\[(\d+)\]',r)
  if m:return f'uint64_t(v{m[1]}>>{64*int(m[2])})'
 raise ValueError(f'Unknown register {r}')
def dst(r,e):
 r=r.strip()
 if r in ('wzr','xzr'):return ';'
 if r=='sp':return f'sp={e};'
 if r.startswith('w'):return f'x{r[1:]}=uint32_t({e});'
 if r.startswith('x'):return f'{r}={e};'
 if r.startswith('d'):return f'v{r[1:]}=uint64_t({e});'
 if r.startswith('q'):return f'v{r[1:]}={e};'
 if r.startswith('v'):
  m=re.fullmatch(r'v(\d+)\.(\d+)([bsd])',r)
  if m:
   n=int(m[2])*dict(b=8,s=32,d=64)[m[3]]
   return f'v{m[1]}=({e}) & {mask(n)};'
 raise ValueError(f'Unknown destination {r}')
def val(s):
 s=s.strip()
 if s.startswith('#'):return f'uint64_t({int(s[1:],0)})'
 return reg(s)
def width(s):
 if s.strip().startswith('w'):return 32
 return 64

def mem(s):
 m=re.fullmatch(r'\[(\w+)(?:, (#?-?0x[0-9a-f]+|#?-?\d+|x[0-9]+))?\](!)?',s.strip())
 if not m:raise ValueError('Unknown address '+s)
 return f'({reg(m[1])}+int64_t({val(m[2]) if m[2] else 0}))',m[1],bool(m[3])
def size(s):
 s=s.strip()
 return 4 if s.startswith('w') else 16 if s.startswith('q') else 8

def instruction(i):
 m=i.mnemonic; o=[x.strip() for x in split(i.op_str)];p=i.address;nextpc=p+4
 end=f'goto L{nextpc:x};'
 if m=='b':return f'goto L{int(o[0][1:],0):x};'
 if m.startswith('b.'):
  c={'eq':'z','ne':'!z','hs':'c','lo':'!c','lt':'(n!=vflag)'}[m[2:]]
  return f'if ({c}) goto L{int(o[0][1:],0):x}; {end}'
 if m in ('cbz','cbnz'):
  return f'if ({val(o[0])}{"==" if m=="cbz" else "!="}0) goto L{int(o[1][1:],0):x}; {end}'
 if m in ('ret','bl','br'):
  return f'pc=0x{p:x};status={1 if m=="ret" else 2};goto done;'
 if m in ('cmp','tst','adds','subs'):
  args=o if m in ('cmp','tst') else o[1:]
  a,b=map(val,args);bits=width(args[0]);maskv='UINT32_MAX' if bits==32 else 'UINT64_MAX'
  opt='&' if m=='tst' else '+' if m=='adds' else '-'
  code=f'{{ uint64_t a={a},b={b};uint64_t r=(a{opt}b)&{maskv};n=(r>>{bits-1})&1;z=r==0;'
  if opt=='-':code+=f'c=a>=b;vflag=(((a^b)&(a^r))>>{bits-1})&1;'
  elif opt=='+':code+=f'c=({"(a+b)>UINT32_MAX" if bits==32 else "r<a"});vflag=((~(a^b)&(a^r))>>{bits-1})&1;'
  else:code+='c=0;vflag=0;'
  if m in ('adds','subs'):code+=dst(o[0],'r')
  return code+'}'+end
 if m in ('ldr','ldur','ldrsb','ldursb','ldp','str','stur','strb','sturb','stp'):
  pair=m in ('ldp','stp');load=m.startswith('ld');memindex=2 if pair else 1
  adr,base,pre=mem(o[memindex]);byte=m.endswith('b');nb=1 if byte else size(o[0]);typ={1:'uint8_t',4:'uint32_t',8:'uint64_t',16:'U128'}[nb]
  lines=[f'{{uint64_t addr={adr};']
  for j in range(2 if pair else 1):
   ptr=f'(addr+{j*nb})';expr=f'load<{typ}>({ptr})'
   if load:
    if byte:expr=f'int64_t(int8_t({expr}))'
    lines.append(dst(o[j],expr))
   else:lines.append(f'store<{typ}>({ptr},{val(o[j])});')
  if pre:lines.append(dst(base,'addr'))
  elif len(o)>memindex+1:lines.append(dst(base,f'{reg(base)}+{val(o[memindex+1])}'))
  return ''.join(lines)+'}'+end
 if m=='movk':
  shift=int(o[2].split('#')[1],0) if len(o)>2 else 0
  e=f'({val(o[0])}&~(uint64_t(0xffff)<<{shift}))|({val(o[1])}<<{shift})'
  return dst(o[0],e)+end
 if m in ('ubfx','bfxil'):
  sh=int(o[2][1:],0);nb=int(o[3][1:],0)
  e=f'(({val(o[1])}>>{sh})&((uint64_t(1)<<{nb})-1))'
  if m=='bfxil':e=f'({val(o[0])}&~((uint64_t(1)<<{nb})-1))|{e}'
  return dst(o[0],e)+end
 if m in ('ushr','shl','shrn','xtn','ushll','shll','dup','ext') or o[0].startswith('v'):
  dest=o[0];dm=re.match(r'v(\d+)\.(\d+)([bsd])',dest);nb=int(dm[2]);lb=dict(b=8,s=32,d=64)[dm[3]];bits=nb*lb
  a=val(o[1]);b=val(o[2]) if len(o)>2 else None
  if m=='mov':e=a
  elif m in ('mvn','orr','and','bic','eor','bif','bit','bsl'):
   e={'mvn':f'~{a}','orr':f'({a}|{b})','and':f'({a}&{b})','bic':f'({a}&~{b})','eor':f'({a}^{b})','bit':f'(({val(dest)}&~{b})|({a}&{b}))','bif':f'(({val(dest)}&{b})|({a}&~{b}))','bsl':f'(({val(dest)}&{a})|(~{val(dest)}&{b}))'}[m]
  elif m in ('add','sub'):e=f'lane_bin({a},{b},{lb},{nb},\'{"+" if m=="add" else "-"}\')'
  elif m in ('ushr','shl'):e=f'lane_shift({a},{int(o[2][1:],0)},{lb},{nb},{str(m=="shl").lower()})'
  elif m in ('shrn','xtn'):e=f'lane_narrow({a},{0 if m=="xtn" else int(o[2][1:],0)},{lb},{nb})'
  elif m in ('ushll','shll'):e=f'lane_widen({a},{int(o[2][1:],0)},{lb},{nb})'
  elif m=='dup':e=f'lane_dup({a},{lb},{nb})'
  elif m=='ext':e=f'(({a}>>{int(o[3][1:],0)*8})|({b}<<{128-int(o[3][1:],0)*8}))'
  else:raise ValueError('Unknown vector '+m)
  return dst(dest,e)+end
 if m=='mov':return dst(o[0],val(o[1]))+end
 if m=='mvn':return dst(o[0],f'~{val(o[1])}')+end
 if m=='ror':return dst(o[0],f'ror<{"uint32_t" if width(o[0])==32 else "uint64_t"}>({val(o[1])},{int(o[2][1:],0)})')+end
 if m=='lsr':return dst(o[0],f'{val(o[1])}>>{val(o[2])}')+end
 if m in ('add','sub','and','orr','eor','bic'):
  a,b=val(o[1]),val(o[2])
  if len(o)>3:
   assert o[3].startswith('lsl #');b=f'({b}<<{int(o[3].split("#")[1],0)})'
  op={'add':'+','sub':'-','and':'&','orr':'|','eor':'^','bic':'&~'}[m]
  return dst(o[0],f'({a}{op}{b})')+end
 raise ValueError(f'Unknown {p:x} {m} {i.op_str}')
HEADER=r'''
#include <cstdint>
#include <cstring>
using U128=unsigned __int128;
struct State { uint64_t x[31],sp; U128 q[32];uint64_t pc,nzcv,status; };
template<class T> static inline T load(uint64_t addr){T r;std::memcpy(&r,reinterpret_cast<void*>(addr),sizeof(T));return r;}
template<class T> static inline void store(uint64_t addr,T value){std::memcpy(reinterpret_cast<void*>(addr),&value,sizeof(T));}
template<class T> static inline T ror(T x,unsigned s){return T((x>>s)|(x<<(sizeof(T)*8-s)));}
static inline U128 lane_bin(U128 a,U128 b,int bits,int n,char op){U128 r=0,m=(U128(1)<<bits)-1;for(int i=0;i<n;i++){auto x=(a>>(i*bits))&m,y=(b>>(i*bits))&m;r|=((op=='+'?x+y:x-y)&m)<<(i*bits);}return r;}
static inline U128 lane_shift(U128 a,int sh,int bits,int n,bool left){U128 r=0,m=(U128(1)<<bits)-1;for(int i=0;i<n;i++){auto x=(a>>(i*bits))&m;r|=((left?x<<sh:x>>sh)&m)<<(i*bits);}return r;}
static inline U128 lane_narrow(U128 a,int sh,int bits,int n){U128 r=0,m=(U128(1)<<bits)-1;for(int i=0;i<n;i++)r|=((a>>(i*bits*2+sh))&m)<<(i*bits);return r;}
static inline U128 lane_widen(U128 a,int sh,int bits,int n){U128 r=0,m=(U128(1)<<(bits/2))-1;for(int i=0;i<n;i++)r|=(((a>>(i*bits/2))&m)<<sh)<<(i*bits);return r;}
static inline U128 lane_dup(U128 a,int bits,int n){U128 r=0,m=(U128(1)<<bits)-1;for(int i=0;i<n;i++)r|=(a&m)<<(i*bits);return r;}
extern "C" int libg_loader_run(State *s, uint64_t budget){
'''

def generate(source,out):
 raw,ins,calls,outside=graph(source,0x1adfe4c)
 assert not outside
 lines=[HEADER]
 lines+=[''.join(f'uint64_t x{i}=s->x[{i}];' for i in range(31))+'uint64_t sp=s->sp,pc=s->pc;']
 lines+=[''.join(f'U128 v{i}=s->q[{i}];' for i in range(32))]
 lines+=['bool n=(s->nzcv>>31)&1,z=(s->nzcv>>30)&1,c=(s->nzcv>>29)&1,vflag=(s->nzcv>>28)&1;uint64_t status=0,ticks=0;']
 lines+=['switch(pc){']+[f'case 0x{p:x}:goto L{p:x};' for p in sorted(ins)]+['default:status=3;goto done;}']
 for p,i in sorted(ins.items()):
  code=instruction(i)
  # Bound at conditional branches: no Python hook on the hot arithmetic path.
  if i.mnemonic.startswith('b.') or i.mnemonic in ('cbz','cbnz'):
   code=f'if(++ticks>=budget){{pc=0x{p:x};status=0;goto done;}}'+code
  lines.append(f'L{p:x}: /* {i.mnemonic} {i.op_str} */ {code}')
 lines+=['done:']+[''.join(f's->x[{i}]=x{i};' for i in range(31))]+[''.join(f's->q[{i}]=v{i};' for i in range(32))]+['s->sp=sp;s->pc=pc;s->nzcv=(uint64_t(n)<<31)|(uint64_t(z)<<30)|(uint64_t(c)<<29)|(uint64_t(vflag)<<28);s->status=status;return status;','}']
 out.mkdir(parents=True,exist_ok=True);cpp=out/'loader-translated.cpp';cpp.write_text('\n'.join(lines)+'\n')
 report={'source':str(source),'source_sha256':hashlib.sha256(raw).hexdigest(),'cfg_entry':'0x1adfe4c','instruction_count':len(ins),'method':'Capstone decoded instruction to exact scalar/NEON C++ operation; direct branches retained; calls return before execution','calls_stop_at':[hex(i.address) for i in ins.values() if i.mnemonic=='bl'],'cpp':str(cpp),'cpp_sha256':hashlib.sha256(cpp.read_bytes()).hexdigest(),'limitations':['Must be differentially verified against Unicorn before using results.','Caller maps buffers at the same guest virtual addresses; never load untrusted virtual addresses without validating maps.','No high-level cipher identification is claimed.']}
 (out/'translation.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--source',type=Path,default=ROOT/'analysis/native/lib/arm64-v8a/libg.so');p.add_argument('--out',type=Path,default=ROOT/'analysis/libg-cipher');a=p.parse_args();generate(a.source,a.out)
