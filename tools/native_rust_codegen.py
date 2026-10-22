#!/usr/bin/env python3
"""Compile source-bound original ARM64 function bodies into auditable Rust.

Only pinned original code is accepted. Unimplemented instructions and missing
functions fail explicitly; the generated runtime never falls back to emulation.
"""
from pathlib import Path
import argparse,bisect,collections,csv,hashlib,json,re
from capstone import Cs,CS_ARCH_ARM64,CS_MODE_ARM
from capstone.arm64_const import *
ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/'analysis/libg-cipher/libg.decrypted.so'
EXPECTED='79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6'
BASE=0x10000000
COND={'eq':0,'ne':1,'hs':2,'cs':2,'lo':3,'cc':3,'mi':4,'pl':5,'vs':6,'vc':7,'hi':8,'ls':9,'ge':10,'lt':11,'gt':12,'le':13,'al':14,'nv':15}

def parts(s):
 out=[];start=0;depth=0
 for j,ch in enumerate(s):
  if ch in '[{':depth+=1
  elif ch in ']}':depth-=1
  elif ch==',' and depth==0:out.append(s[start:j].strip());start=j+1
 if s[start:].strip():out.append(s[start:].strip())
 return out

def mask(bits):return hex((1<<bits)-1)+'u128'
def regname(i,op):return i.reg_name(op.reg)
def width(r):return 32 if r.startswith('w') else 64

def rr(r):
 r=r.strip();r={'fp':'x29','lr':'x30'}.get(r,r)
 if r in ('xzr','wzr'):return '0u64'
 if r in ('sp','wsp'):return 'cpu.sp' if r=='sp' else '(cpu.sp as u32 as u64)'
 if re.fullmatch('[xw][0-9]+',r):return f'cpu.x[{int(r[1:])}]'+(' as u32 as u64' if r[0]=='w' else '')
 if re.fullmatch('[bhsdqv][0-9]+',r):
  bits={'b':8,'h':16,'s':32,'d':64,'q':128,'v':128}[r[0]]
  return f'(cpu.q[{int(r[1:])}] & {mask(bits)})'+(' as u64' if bits<=64 else '')
 raise ValueError('register '+r)

def wr(r,e):
 r=r.strip();r={'fp':'x29','lr':'x30'}.get(r,r)
 if r in ('xzr','wzr'):return f'let _ = {e};'
 if r in ('sp','wsp'):return f'cpu.sp=({e}) as '+('u32 as u64;' if r=='wsp' else 'u64;')
 if re.fullmatch('[xw][0-9]+',r):return f'cpu.x[{int(r[1:])}]=({e}) as '+('u32 as u64;' if r[0]=='w' else 'u64;')
 if re.fullmatch('[bhsdqv][0-9]+',r):
  bits={'b':8,'h':16,'s':32,'d':64,'q':128,'v':128}[r[0]]
  return f'cpu.q[{int(r[1:])}]=({e}) as u128 & {mask(bits)};'
 raise ValueError('destination '+r)

def extend(expr,ext):
 casts={ARM64_EXT_UXTB:'u8 as u64',ARM64_EXT_UXTH:'u16 as u64',ARM64_EXT_UXTW:'u32 as u64',ARM64_EXT_UXTX:'u64',ARM64_EXT_SXTB:'i8 as i64 as u64',ARM64_EXT_SXTH:'i16 as i64 as u64',ARM64_EXT_SXTW:'i32 as i64 as u64',ARM64_EXT_SXTX:'i64 as u64'}
 if ext:return f'(({expr}) as {casts[ext]})'
 return f'({expr})'

def shift(expr,kind,amount,bits=64):
 if not kind or not amount:return f'({expr})'
 ty='u32' if bits==32 else 'u64';ity='i32' if bits==32 else 'i64'
 if kind==ARM64_SFT_LSL:return f'(({expr}) as {ty}).wrapping_shl({amount}) as u64'
 if kind==ARM64_SFT_LSR:return f'(({expr}) as {ty}).wrapping_shr({amount}) as u64'
 if kind==ARM64_SFT_ASR:return f'(({expr}) as {ity}).wrapping_shr({amount}) as u64'
 if kind==ARM64_SFT_ROR:return f'(({expr}) as {ty}).rotate_right({amount}) as u64'
 raise ValueError('shift '+str(kind))

def val(i,o,bits=64):
 if o.type==ARM64_OP_IMM: e=f'{o.imm & ((1<<64)-1):#x}u64'
 elif o.type==ARM64_OP_REG:e=rr(regname(i,o))
 else:raise ValueError('operand '+str(o.type))
 return shift(extend(e,o.ext),o.shift.type,o.shift.value,bits)

def vparse(s):
 m=re.fullmatch(r'([vqdshb])(\d+)(?:\.(\d+)?([bhsd]))?(?:\[(\d+|0x[0-9a-f]+)\])?',s)
 if not m:raise ValueError('vector '+s)
 idx=int(m[2]);lb={'b':8,'h':16,'s':32,'d':64}.get(m[4],{'b':8,'h':16,'s':32,'d':64,'q':128,'v':128}[m[1]])
 lanes=int(m[3]) if m[3] else 1
 return idx,lb,lanes,int(m[5],0) if m[5] else None

def vector(i,o):
 m=i.mnemonic;d,bits,lanes,lane=vparse(o[0]);total=bits*lanes
 def vec(s):
  n,b,l,j=vparse(s)
  return f'((cpu.q[{n}] >> {j*b}) & {mask(b)})' if j is not None else f'(cpu.q[{n}] & {mask(b*l)})'
 def put(e):
  if lane is not None:return f'cpu.q[{d}]=(cpu.q[{d}] & !({mask(bits)} << {lane*bits})) | ((({e}) as u128 & {mask(bits)}) << {lane*bits});'
  return f'cpu.q[{d}]=({e}) & {mask(total)};'
 try:a=(vec(o[1]) if not re.match('[xw]',o[1]) else f'({rr(o[1])}) as u128') if len(o)>1 and not o[1].startswith('#') else None
 except ValueError:a=None
 b=vec(o[2]) if len(o)>2 and o[2].startswith('v') else None
 if m=='mov':
  if re.match('[xw]',o[1]):return put(f'({rr(o[1])}) as u128')
  return put(a)
 if m in ('movi','mvni'):
  n=int(o[1][1:],0);sh=int(o[2].split('#')[1],0) if len(o)>2 else 0
  return put(f'lane_dup({n<<sh}u128,{bits},{lanes})'+(f' ^ {mask(total)}' if m=='mvni' else ''))
 if m=='dup':
  a=vec(o[1]) if o[1].startswith('v') else f'({rr(o[1])}) as u128'
  return put(f'lane_dup({a},{bits},{lanes})')
 if m in ('and','orr','eor','bic','orn','bsl','bit','bif','not','mvn'):
  old=f'cpu.q[{d}]'
  if len(o)>1 and o[1].startswith('#'):
   # Two-operand immediate form (e.g. `bic v21.4s, #3`): Vd = Vd op imm,
   # modified immediate imm8 optionally shifted (lsl #n), replicated per lane.
   n=int(o[1].split('#')[1].split(',')[0],0)
   sh=int(o[2].split('#')[1],0) if len(o)>2 else 0
   b=f'lane_dup({n<<sh}u128,{bits},{lanes})';a=old
  elif b is None and len(o)>2 and o[2].startswith('#'):
   n=int(o[2].split('#')[1].split(',')[0],0)
   sh=int(o[3].split('#')[1],0) if len(o)>3 else 0
   b=f'lane_dup({n<<sh}u128,{bits},{lanes})'
  ex={'and':f'{a}&{b}','orr':f'{a}|{b}','eor':f'{a}^{b}','bic':f'{a}&!{b}','orn':f'{a}|!{b}','bsl':f'({old}&{a})|(!{old}&{b})','bit':f'({old}&!{b})|({a}&{b})','bif':f'({old}&{b})|({a}&!{b})','not':f'!{a}','mvn':f'!{a}'}[m]
  return put(ex)
 if m in ('fmul','fadd','fsub','fdiv','fmax','fmin','fnmul','fabd'):
  if b is None:raise ValueError('vector '+m+' '+i.op_str)
  if '[' in o[2]:
   n2,lb2,_,j2=vparse(o[2]);b=f'lane_dup((cpu.q[{n2}] >> {j2*lb2}) & {mask(lb2)},{bits},{lanes})'
  return put(f'lane_fp_binary({a},{b},{bits},{lanes},"{m}")')
 if m in ('add','sub','mul','cmeq','cmgt','cmge','cmhi','cmhs','cmtst','sabd','uabd','smin','smax','cmlt','uqsub','umin','umax'):
  if '[' in o[2]:
   n2,lb2,_,j2=vparse(o[2]);b=f'lane_dup((cpu.q[{n2}] >> {j2*lb2}) & {mask(lb2)},{bits},{lanes})'
  b='0u128' if o[2]=='#0' else b
  return put(f'lane_binary({a},{b},{bits},{lanes},"{m}")')
 if m=='neg':return put(f'lane_binary(0u128,{a},{bits},{lanes},"neg")')
 if m=='fmov':
  if o[1].startswith('#'):
   imm=float(o[1][1:]);ty='f32' if bits==32 else 'f64'
   return put(f'lane_dup(({imm!r}{ty}).to_bits() as u128,{bits},{lanes})')
  return put(a)
 if m in ('fmul','fadd','fsub','fdiv','fsqrt','fabs','fneg','frintp','frintm','frintz','frinta'):
  bb='0u128' if b is None else b
  return put(f'lane_fp_binary({a},{bb},{bits},{lanes},"{m}")')
 if m in ('scvtf','ucvtf'):
  return put(f'lane_int_to_fp({a},{bits},{lanes},{str(m=="scvtf").lower()})')
 if m in ('fcvtzs','fcvtzu'):
  return put(f'lane_fp_to_int({a},{bits},{lanes},{str(m=="fcvtzs").lower()})')
 if m in ('fcvtl','fcvtl2'):
  return put(f'lane_fp_cvtl({a},{lanes},{str(m=="fcvtl2").lower()})')
 if m in ('fcvtn','fcvtn2'):
  return f'cpu.q[{d}]=lane_fp_cvtn(cpu.q[{d}],{a},{lanes},{str(m=="fcvtn2").lower()});'
 if m=='tbl':
  regs=[s.strip() for s in o[1].strip('{}').split(',')]
  tvals=[f'(cpu.q[{int(s[1:s.index(".")])}])' for s in regs]
  while len(tvals)<4:tvals.append('0u128')
  return put(f'lane_tbl({b},[{",".join(tvals)}],{16*len(regs)})')
 if m in ('fmla','fmls'):
  if '[' in o[2]:
   n2,lb2,_,j2=vparse(o[2]);b=f'lane_dup((cpu.q[{n2}] >> {j2*lb2}) & {mask(lb2)},{bits},{lanes})'
  return f'cpu.q[{d}]=lane_fp_tma(cpu.q[{d}],{a},{b},{bits},{lanes},{str(m=="fmls").lower()}) & {mask(total)};'
 if m in ('fmaxnm','fminnm'):
  return put(f'lane_fp_minmax({a},{b},{bits},{lanes},{str(m=="fmaxnm").lower()})')
 if m in ('fcmeq','fcmgt','fcmge'):
  if o[2].startswith('#'):b='0u128'
  elif '[' in o[2]:
   n2,lb2,_,j2=vparse(o[2]);b=f'lane_dup((cpu.q[{n2}] >> {j2*lb2}) & {mask(lb2)},{bits},{lanes})'
  return put(f'lane_fp_cmp({a},{b},{bits},{lanes},"{m}")')
 if m=='faddp':return put(f'lane_fp_addp({a},{b},{bits},{lanes})')
 if m in ('rev64','rev32','rev16'):
  return put(f'lane_rev({a},{bits},{lanes},{int(m[3:])})')
 if m=='ushl':return put(f'lane_ushl({a},{b},{bits},{lanes})')
 if m in ('ssra','usra'):
  n=int(o[2][1:],0);sh_op='sshr' if m=='ssra' else 'ushr'
  return put(f'lane_binary(cpu.q[{d}],lane_shift({a},{bits},{lanes},{n},"{sh_op}"),{bits},{lanes},"add")')
 if m in ('uaddw','uaddw2','saddw','saddw2'):
  _,sb,_,_=vparse(o[2]);n2=int(o[2][1:o[2].index('.')],0)
  src=f'(cpu.q[{n2}] >> {64 if m.endswith("2") else 0})'
  return put(f'lane_widen_add({a},{src},{bits},{sb},{lanes},{str(m.endswith("2")).lower()})')
 if m in ('cnt',):return put(f'lane_count({a},{bits},{lanes})')
 if m in ('ushr','sshr','shl'):
  n=int(o[2][1:],0);return put(f'lane_shift({a},{bits},{lanes},{n},"{m}")')
 if m in ('addv','uaddlv','saddlv','smaxv','umaxv','uminv'):
  _,sb,sl,_=vparse(o[1]);return put(f'lane_reduce({a},{sb},{sl},"{m}")')
 if m in ('uzp1','uzp2','zip1','zip2','trn1','trn2'):
  return put(f'lane_shuffle({a},{b},{bits},{lanes},"{m}")')
 if m in ('xtn','shrn','ushll','sshll','shll','ushll2','sshll2'):
  _,sb,sl,_=vparse(o[1]);n=int(o[2][1:],0) if len(o)>2 else 0
  if m.endswith('2'):a=f'({a} >> 64)'
  return put(f'lane_resize({a},{sb},{bits},{lanes},{n},"{m[:-1] if m.endswith("2") else m}")')
 if m=='ext':
  n=int(o[3][1:],0)*8
  return put(a if n==0 else f'({a} >> {n}) | ({b} << {total-n})')
 raise ValueError('vector '+m+' '+i.op_str)

class Compiler:
 def __init__(self,functions):self.functions=functions
 def call(self,target,returnpc=None):
  prefix=f'cpu.x[30]={returnpc:#x};' if returnpc is not None else ''
  if isinstance(target,int) and target in self.functions:
   group,name=self.functions[target];call=f'super::chunk{group}::{name}(cpu,rt,{target:#x})?;'
  else:call=f'super::dispatch(cpu,rt,{hex(target) if isinstance(target,int) else target})?;'
  return prefix+call
 def emit(self,i,start,end):
  m=i.mnemonic;o=parts(i.op_str);ops=i.operands;p=i.address;nextpc=p+4
  m={'stlr':'str','stlrb':'strb','stlrh':'strh','ldar':'ldr','ldarb':'ldrb','ldarh':'ldrh'}.get(m,m)
  r=lambda j:rr(regname(i,ops[j]));w=lambda j,e:wr(regname(i,ops[j]),e)
  v=lambda j:val(i,ops[j],width(regname(i,ops[0])))
  bits=width(regname(i,ops[0])) if ops and ops[0].type==ARM64_OP_REG else 64
  cond=lambda c:f'cpu.condition({COND[c]})'
  def jump(t):
   if start<=t<end:return f'pc={t:#x};continue;'
   return self.call(t)+'return Ok(());'
  if m=='ret':return f'cpu.pc={r(0) if ops else "cpu.x[30]"};return Ok(());',True
  if m=='b':return jump(ops[0].imm),True
  if m.startswith('b.'):
   return f'if {cond(m[2:])}{{{jump(ops[0].imm)}}}pc={nextpc:#x};continue;',True
  if m in ('cbz','cbnz'):
   return f'if ({r(0)}) {"==" if m=="cbz" else "!="} 0 {{{jump(ops[1].imm)}}}pc={nextpc:#x};continue;',True
  if m in ('tbz','tbnz'):
   return f'if (({r(0)}) & (1u64 << {ops[1].imm})) {"==" if m=="tbz" else "!="} 0 {{{jump(ops[2].imm)}}}pc={nextpc:#x};continue;',True
  if m=='bl':return self.call(ops[0].imm,nextpc),False
  if m=='blr':return '{let target='+r(0)+';'+self.call('target',nextpc)+'}',False
  if m=='br':return '{let target='+r(0)+f';if target>={start:#x} && target<{end:#x}{{pc=target;continue;}}'+self.call('target')+'return Ok(());}',True
  if m in ('nop','hint','yield','dmb','dsb','isb','paciasp','autiasp','bti','prfm'):return '',False
  if m=='brk':return 'return Err('+json.dumps(f'software breakpoint hit at {p:#x}')+'.into());',True
  if m=='clrex':return 'rt.memory.clear_exclusive();',False
  if m in ('ldaxr','ldxr'):
   ty='u'+str(bits);address=rr(i.reg_name(ops[1].mem.base))
   return '{let address='+address+';let value=rt.memory.load_exclusive::<'+ty+'>(address);'+w(0,'value')+'}',False
  if m in ('stlxr','stxr'):
   ty='u'+str(width(regname(i,ops[1])));address=rr(i.reg_name(ops[2].mem.base))
   return '{let address='+address+';let value='+r(1)+';let status=rt.memory.store_exclusive::<'+ty+'>(address,value as '+ty+');'+w(0,'status')+'}',False
  if m in ('ldadd','ldadda','ldaddl','ldaddal'):
   ty='u'+str(bits);address=rr(i.reg_name(ops[2].mem.base))
   return '{let address='+address+';let value=('+r(0)+') as '+ty+';assert_eq!(address % std::mem::size_of::<'+ty+'>() as u64,0,"unaligned atomic add");let old=rt.memory.read::<'+ty+'>(address);rt.memory.write::<'+ty+'>(address,old.wrapping_add(value));'+w(1,'old')+'}',False
  if m in ('cas','casa','casl','casal'):
   # Compare and swap: if memory holds the expected value, store the data
   # register; the destination receives the old memory value either way.
   ty='u32' if o[0][0]=='w' else 'u64'
   mo=ops[2];addr=f'({rr(i.reg_name(mo.mem.base))}).wrapping_add({mo.mem.disp & ((1<<64)-1):#x}u64)'
   return '{let address='+addr+f';let old=rt.memory.read::<{ty}>(address);if old==(({r(1)}) as {ty}){{rt.memory.write::<{ty}>(address,({r(0)}) as {ty});}}'+w(0,'old as u64')+'}',False
  if m in ('adr','adrp'):return w(0,f'{ops[1].imm:#x}u64'),False
  if m in ('mrs','msr'):
   field=o[1] if m=='mrs' else o[0]
   field={'tpidr_el0':'tpidr_el0','nzcv':'nzcv','fpcr':'fpcr','fpsr':'fpsr'}.get(field.lower())
   if field:return (w(0,'cpu.'+field) if m=='mrs' else 'cpu.'+field+'='+r(1)+';'),False
   raise ValueError('system '+i.op_str)
  if m=='ld1r':
   d,lb,lanes,_=vparse(o[0].strip('{}'))
   mem=ops[-1].mem;address=f'({rr(i.reg_name(mem.base))}).wrapping_add({mem.disp & ((1<<64)-1):#x}u64)'
   return f'cpu.q[{d}]=lane_dup(rt.memory.read::<u{lb}>({address}) as u128,{lb},{lanes});',False
  if m in ('ld1','st1') and '[' in o[0]:
   # One element lane to/from memory: ld1 {vN.T}[i], [xM] (, #imm post-index)
   lm=re.fullmatch(r'([vqd]\d+)\.([bhsd])\[(\d+|0x[0-9a-f]+)\]',o[0].replace('{','').replace('}',''))
   if not lm:raise ValueError('lane mem '+i.op_str)
   n=int(lm[1][1:],0);eb={'b':8,'h':16,'s':32,'d':64}[lm[2]];lanei=int(lm[3],0)
   mo=ops[1];base=i.reg_name(mo.mem.base)
   addr=f'({rr(base)}).wrapping_add({mo.mem.disp & ((1<<64)-1):#x}u64)'
   if mo.mem.index:
    ir=shift(extend(rr(i.reg_name(mo.mem.index)),mo.ext),mo.shift.type,mo.shift.value);addr=f'({addr}).wrapping_add({ir})'
   code='{let address='+addr+';'
   if m=='ld1':
    code+=f'cpu.q[{n}]=(cpu.q[{n}] & !({mask(eb)} << {lanei*eb})) | ((rt.memory.read::<u{eb}>(address) as u128 & {mask(eb)}) << {lanei*eb});'
   else:
    code+=f'rt.memory.write::<u{eb}>(address,((cpu.q[{n}] >> {lanei*eb}) & {mask(eb)}) as u{eb});'
   if i.writeback:
    if len(ops)>2 and ops[2].type==ARM64_OP_IMM:code+=wr(base,f'({rr(base)}).wrapping_add({ops[2].imm & ((1<<64)-1)})')
    else:code+=wr(base,'address')
   return code+'}',False
  if m in ('ld1','ld2','ld3','ld4','st1','st2','st3','st4') and o and o[0].startswith('{') and '[' not in o[0]:
   # Interleaved multi-register structures: ldN/stN {vA.T,...,vK.T}, [xM]
   k=int(m[2]);parsed=[vparse(s.strip().strip('{}')) for s in o[0].strip('{}').split(',')]
   if len(parsed)!=k or len({(p[1],p[2]) for p in parsed})!=1:raise ValueError('structure '+i.op_str)
   eb=parsed[0][1];per=parsed[0][2]
   mo=ops[1];base=i.reg_name(mo.mem.base)
   addr=f'({rr(base)}).wrapping_add({mo.mem.disp & ((1<<64)-1):#x}u64)'
   if mo.mem.index:
    ir=shift(extend(rr(i.reg_name(mo.mem.index)),mo.ext),mo.shift.type,mo.shift.value);addr=f'({addr}).wrapping_add({ir})'
   code='{let address='+addr+';'
   for eidx in range(k*per):
    rn=parsed[eidx%k][0];ln=eidx//k
    if m[0]=='l':
     code+=f'cpu.q[{rn}]=(cpu.q[{rn}] & !({mask(eb)} << {ln*eb})) | ((rt.memory.read::<u{eb}>(address.wrapping_add({eidx*(eb//8)})) as u128 & {mask(eb)}) << {ln*eb});'
    else:
     code+=f'rt.memory.write::<u{eb}>(address.wrapping_add({eidx*(eb//8)}),((cpu.q[{rn}] >> {ln*eb}) & {mask(eb)}) as u{eb});'
   if i.writeback:
    if len(ops)>2 and ops[2].type==ARM64_OP_IMM:code+=wr(base,f'({rr(base)}).wrapping_add({ops[2].imm & ((1<<64)-1)})')
    else:code+=wr(base,f'address.wrapping_add({k*per*(eb//8)})')
   return code+'}',False
  if m.startswith(('ldr','ldur','ldar','str','stur')) or m in ('ldp','stp','ldpsw','ldnp','stnp'):
   pair=m in ('ldp','stp','ldpsw','ldnp','stnp');mi=2 if pair else 1;mo=ops[mi]
   if mo.type==ARM64_OP_IMM:addr=f'{mo.imm:#x}u64';base=None
   else:
    base=i.reg_name(mo.mem.base);addr=f'({rr(base)}).wrapping_add({mo.mem.disp & ((1<<64)-1):#x}u64)'
    if mo.mem.index:
     index=extend(rr(i.reg_name(mo.mem.index)),mo.ext)
     index=shift(index,mo.shift.type,mo.shift.value)
     addr=f'({addr}).wrapping_add({index})'
   dest=regname(i,ops[0]);nb=1 if m.endswith('b') else 2 if m.endswith('h') else 4 if m.endswith('sw') else {'w':4,'x':8,'b':1,'h':2,'s':4,'d':8,'q':16,'v':16}.get(dest[0],8)
   signed=bool(re.match(r'ld(?:ur|r)(?:sb|sh|sw)$',m)) or m=='ldpsw'
   ty=('i' if signed else 'u')+str(nb*8)
   code='{let address='+addr+';'
   load=m.startswith('ld')
   for j in range(2 if pair else 1):
    if load:
     e=f'rt.memory.read::<{ty}>(address.wrapping_add({j*nb}))'
     if signed:e+=' as i64 as u64'
     code+=f'let v{j}={e};'
    else:code+=f'let v{j}={r(j)};'
   for j in range(2 if pair else 1):
    code+= w(j,f'v{j}') if load else f'rt.memory.write::<{ty}>(address.wrapping_add({j*nb}),v{j} as {ty});'
   if i.writeback and base:
    if mi+1<len(ops):code+=wr(base,f'({rr(base)}).wrapping_add({val(i,ops[mi+1])})')
    else:code+=wr(base,'address')
   return code+'}',False
  if m in ('fmov','scvtf','ucvtf','fcvtzs','fcvtzu','fcvtpu','fcvtps','fcvtms','fcvtmu','fcvtns','fcvtnu','fcvtas','fcvtau','fcvt','frintp','frintm','frintz','frintn','frinta','frintx','frinti','fmul','fadd','fsub','fdiv','fneg','fabs','fsqrt','fmax','fmin','fmaxnm','fminnm','fnmul','fabd','fcmp','fcmpe','fccmp','fccmpe','fcsel','fmadd','fmsub','fnmadd','fnmsub','fmla','fmls','faddp'):
   if o[0][0]=='v':return vector(i,o),False
   if m=='fmov':
    if ops[1].type==ARM64_OP_FP:
     ty='f32' if o[0].startswith('s') else 'f64';return w(0,f'({ops[1].fp!r}{ty}).to_bits()'),False
    return w(0,r(1)),False
   def fp(j):
    if ops[j].type==ARM64_OP_FP:return repr(ops[j].fp)+('f32' if o[0].startswith('s') else 'f64')
    if o[j].startswith('v') and '[' in o[j]:
     # Scalar read of one vector lane, e.g. `fmul s25, s25, v1.s[1]`:
     # reinterpret the addressed lane, not the whole 128-bit q register.
     n,lb,_,ix=vparse(o[j]);ty='f32' if lb==32 else 'f64';uint='u32' if ty=='f32' else 'u64'
     return f'{ty}::from_bits((((cpu.q[{n}] >> {ix*lb}) & {mask(lb)})) as {uint})'
    ty='f32' if o[j].startswith('s') else 'f64';uint='u32' if ty=='f32' else 'u64'
    return f'{ty}::from_bits(({r(j)}) as {uint})'
   ty='f32' if o[0].startswith('s') else 'f64'
   if m in ('scvtf','ucvtf'):
    it=('i' if m=='scvtf' else 'u')+str(width(o[1]));return w(0,f'(({r(1)}) as {it} as {ty}).to_bits()'),False
   if m.startswith('fcvt') and m not in ('fcvt',):
    it=('u' if m.endswith('u') else 'i')+str(width(o[0]));rounding={'z':'trunc','p':'ceil','m':'floor','n':'round_ties_even','a':'round'}[m[4]]
    return w(0,f'({fp(1)}.{rounding}() as {it}) as u64'),False
   if m=='fcvt':return w(0,f'({fp(1)} as {ty}).to_bits()'),False
   if m.startswith('frint'):
    # Round to an integral value in place. FRINTP/FRINTM/FRINTZ/FRINTN/FRINTA are
    # directed; FRINTX/FRINTI follow FPCR, which fp_domain pins to nearest-even.
    rounding={'p':f'({fp(1)}).ceil()','m':f'({fp(1)}).floor()','z':f'({fp(1)}).trunc()','n':f'({fp(1)}).round_ties_even()','a':f'({fp(1)}).round()','x':f'({fp(1)}).round_ties_even()','i':f'({fp(1)}).round_ties_even()'}[m[5]]
    return w(0,f'({rounding}).to_bits()'),False
   if m in ('fcmp','fcmpe'):
    return f'{{let a={fp(0)};let b={fp(1)};cpu.nzcv=if a.is_nan()||b.is_nan(){{0x30000000}}else if a==b{{0x60000000}}else if a<b{{0x80000000}}else{{0x20000000}};}}',False
   if m in ('fccmp','fccmpe'):
    code=f'{{let a={fp(0)};let b={fp(1)};cpu.nzcv=if a.is_nan()||b.is_nan(){{0x30000000}}else if a==b{{0x60000000}}else if a<b{{0x80000000}}else{{0x20000000}};}}'
    return f'if {cond(o[3])}{{{code}}}else{{cpu.nzcv={ops[2].imm}u64 << 28;}}',False
   if m=='faddp':
    # Scalar pairwise: add the two lanes of one vector register.
    n,lb,_,_=vparse(o[1]);uint='u32' if lb==32 else 'u64'
    lo=f'{ty}::from_bits(((cpu.q[{n}]) & {mask(lb)}) as {uint})'
    hi=f'{ty}::from_bits(((cpu.q[{n}] >> {lb}) & {mask(lb)}) as {uint})'
    return w(0,f'({lo} + {hi}).to_bits()'),False
   if m in ('fmla','fmls'):
    # Scalar by-element fused multiply-accumulate into the destination.
    a0=fp(1);d0=fp(0)
    e=f'{a0}.mul_add({fp(2)},{d0})' if m=='fmla' else f'(-({a0})).mul_add({fp(2)},{d0})'
    return w(0,f'({e}).to_bits()'),False
   if m=='fcsel':return w(0,f'if {cond(o[3])}{{{r(1)}}}else{{{r(2)}}}'),False
   a=fp(1)
   if m in ('fmaxnm','fminnm'):
    # maxNum/minNum semantics; f32::maximum/minimum are unstable on stable Rust.
    fn='fp_minmax_f32' if ty=='f32' else 'fp_minmax_f64'
    return w(0,f'{fn}({a},{fp(2)},{str(m=="fmaxnm").lower()}).to_bits()'),False
   if m in ('fneg','fabs','fsqrt'):e=f'-({a})' if m=='fneg' else f'({a}).'+('abs' if m=='fabs' else 'sqrt')+'()'
   elif m in ('fmadd','fmsub','fnmadd','fnmsub'):
    b=fp(2);c=fp(3)
    # ARM64 fused, a=Sn b=Sm c=Sa: FMADD=Sa+Sn*Sm, FMSUB=Sa-Sn*Sm,
    # FNMADD=-(Sa+Sn*Sm), FNMSUB=-(Sa-Sn*Sm). Negation is exact so one rounding.
    e={'fmadd':f'{a}.mul_add({b},{c})','fmsub':f'(-({a})).mul_add({b},{c})','fnmadd':f'(-({a})).mul_add({b},-({c}))','fnmsub':f'{a}.mul_add({b},-({c}))'}[m]
   else:
    b=fp(2);e={'fmul':f'{a}*{b}','fadd':f'{a}+{b}','fsub':f'{a}-{b}','fdiv':f'{a}/{b}','fmax':f'({a}).max({b})','fmin':f'({a}).min({b})','fnmul':f'-(({a})*({b}))','fabd':f'(({a})-({b})).abs()'}[m]
   return w(0,f'({e}).to_bits()'),False
  if o and (o[0].startswith('v') or m in ('movi','mvni','cnt','uaddlv','saddlv','addv','smaxv','umaxv','uminv')):return vector(i,o),False
  if m=='addp' and not o[0].startswith('v'):
   # ADDP Dd,Vn.2D: integer pairwise add across one vector register.
   n,lb,_,_=vparse(o[1])
   return w(0,f'lane_binary((cpu.q[{n}] & {mask(lb)}),((cpu.q[{n}] >> {lb}) & {mask(lb)}),{lb},1,"add")'),False
  if m in ('umov','smov'):
   a,b,l,j=vparse(o[1]);e=f'((cpu.q[{a}] >> {j*b}) & {mask(b)})'
   if m=='smov':e=f'(({e}) as i{b} as i64 as u64)'
   return w(0,e),False
  if m=='mov':
   if o[1].startswith('v') and '[' in o[1]:
    n,lb,ln,ix=vparse(o[1]);return w(0,f'((cpu.q[{n}] >> {ix*lb}) & {mask(lb)})'),False
   return w(0,v(1)),False
  if m in ('movk','movz','movn'):
   sh=ops[1].shift.value;imm=ops[1].imm<<sh
   e=f'{imm:#x}u64'
   if m=='movk':e=f'({r(0)} & !({0xffff<<sh:#x}u64)) | {e}'
   if m=='movn':e='!'+e
   return w(0,e),False
  if m in ('cmp','cmn','tst','ccmp','ccmn','add','adds','sub','subs','and','ands','orr','eor','bic','bics','orn','eon'):
   comp=m in ('cmp','cmn','tst','ccmp','ccmn');j=0 if comp else 1;k=j+1
   a=val(i,ops[j],bits);b=val(i,ops[k],bits)
   op='add' if m in ('add','adds','cmn','ccmn') else 'sub' if m in ('sub','subs','cmp','ccmp') else '&' if m in ('and','ands','tst','bic','bics') else '|' if m in ('orr','orn') else '^'
   e=f'a.wrapping_{op}(b)' if op in ('add','sub') else f'a {op} '+('!b' if m in ('bic','bics','orn','eon') else 'b')
   code=f'{{let a={a};let b={b};let result={e};'
   if not comp:code+=w(0,'result')
   if m in ('cmp','cmn','adds','subs','ccmp','ccmn'):code+=f'cpu.flags(a,b,result,{bits},{str(op=="sub").lower()});'
   elif m in ('tst','ands','bics'):code+=f'cpu.logical_flags(result,{bits});'
   code+='}'
   if m in ('ccmp','ccmn'):code=f'if {cond(o[3])}{{{code}}}else{{cpu.nzcv={ops[2].imm}u64 << 28;}}'
   return code,False
  if m in ('neg','negs','mvn'):
   e=f'0u64.wrapping_sub({v(1)})' if m!='mvn' else f'!({v(1)})'
   if m=='negs':return '{let a='+v(1)+';let result=0u64.wrapping_sub(a);'+w(0,'result')+f'cpu.flags(0,a,result,{bits},true);}}',False
   return w(0,e),False
  if m in ('lsl','lsr','asr','ror','lslv','lsrv','asrv','rorv'):
   sh=f'(({v(2)}) & {bits-1}) as u32'
   kind={'lsl':ARM64_SFT_LSL,'lsr':ARM64_SFT_LSR,'asr':ARM64_SFT_ASR,'ror':ARM64_SFT_ROR}[m[:3]]
   return w(0,shift(v(1),kind,sh,bits)),False
  if m in ('mul','madd','msub','mneg','smull','umull','smaddl','umaddl','smsubl','umsubl','smulh','umulh'):
   long=m in ('smull','umull','smaddl','umaddl','smsubl','umsubl');high=m in ('smulh','umulh');signed=m.startswith('s')
   a=v(1);b=v(2)
   if long:prod=f'(({a}) as {"i32 as i64" if signed else "u32 as u64"}).wrapping_mul(({b}) as {"i32 as i64" if signed else "u32 as u64"}) as u64'
   elif high:prod=f'((({a}) as {"i64 as i128" if signed else "u128"}).wrapping_mul(({b}) as {"i64 as i128" if signed else "u128"}) >> 64) as u64'
   else:prod=f'({a}).wrapping_mul({b})'
   if m in ('mneg',):e=f'0u64.wrapping_sub({prod})'
   elif len(ops)>3:e=f'({v(3)}).wrapping_{"sub" if "sub" in m else "add"}({prod})'
   else:e=prod
   return w(0,e),False
  if m in ('sdiv','udiv'):
   ty=('i' if m=='sdiv' else 'u')+str(bits)
   return '{let a='+v(1)+f' as {ty};let b='+v(2)+f' as {ty};'+w(0,'if b==0{0}else{a.wrapping_div(b) as u64}')+'}',False
  if m in ('csel','csinc','csinv','csneg'):
   a=v(1);b=v(2);other={'csel':b,'csinc':f'({b}).wrapping_add(1)','csinv':f'!({b})','csneg':f'0u64.wrapping_sub({b})'}[m]
   return w(0,f'if {cond(o[3])}{{{a}}}else{{{other}}}'),False
  if m in ('cset','csetm','cinc','cinv','cneg'):
   c=o[-1]
   e=f'({cond(c)}) as u64' if m=='cset' else f'if {cond(c)}{{u64::MAX}}else{{0}}' if m=='csetm' else f'if {cond(c)}{{'+({'cinc':f'({v(1)}).wrapping_add(1)','cinv':f'!({v(1)})','cneg':f'0u64.wrapping_sub({v(1)})'}[m])+f'}}else{{{v(1)}}}'
   return w(0,e),False
  if m in ('uxtb','uxth','uxtw','sxtb','sxth','sxtw'):
   ty=('i' if m.startswith('s') else 'u')+str({'b':8,'h':16,'w':32}[m[-1]])
   return w(0,f'({v(1)}) as {ty} as i64 as u64'),False
  if m in ('ubfx','sbfx','ubfiz','sbfiz','bfi','bfxil'):
   sh=ops[2].imm;nb=ops[3].imm;ma=(1<<nb)-1;a=v(1)
   if m in ('ubfx','sbfx','bfxil'):e=f'(({a} >> {sh}) & {ma:#x}u64)'
   else:e=f'(({a} & {ma:#x}u64) << {sh})'
   if m=='sbfx':e=f'((({e}) << {64-nb}) as i64 >> {64-nb}) as u64'
   if m=='sbfiz':e=f'((({a} << {64-nb}) as i64 >> {64-nb}) as u64).wrapping_shl({sh})'
   if m=='bfi':e=f'({r(0)} & !{ma<<sh:#x}u64) | {e}'
   if m=='bfxil':e=f'({r(0)} & !{ma:#x}u64) | {e}'
   return w(0,e),False
  if m in ('clz','cls','rbit','rev','rev16','rev32'):
   ty='u32' if bits==32 else 'u64';a=f'({v(1)} as {ty})'
   if m=='clz':e=a+'.leading_zeros() as u64'
   elif m=='rbit':e=a+'.reverse_bits() as u64'
   elif m=='rev':e=a+'.swap_bytes() as u64'
   elif m=='rev16':
    e=f'((({a}) & 0x00ff00ff00ff00ff) << 8) | ((({a}) >> 8) & 0x00ff00ff00ff00ff)'
   else:
    b16=f'((({a}) & 0x00ff00ff00ff00ff) << 8) | ((({a}) >> 8) & 0x00ff00ff00ff00ff)'
    e=f'(({b16} & 0x0000ffff0000ffff) << 16) | (({b16} >> 16) & 0x0000ffff0000ffff)'
   return w(0,f'(({e}) & {hex((1<<bits)-1)})'),False
  raise ValueError(m+' '+i.op_str)

def thread_forward_edges(lines,budget):
 """Duplicate bounded forward successors, leaving backedges in the PC loop.

 Every original block remains a valid entry. Only `pc=constant;continue;`
 transfers are replaced, so instruction order, calls, returns and backward or
 indirect transfers are unchanged. A shared budget per entry bounds code size.
 """
 blocks={};address=None;body=[]
 for line in lines[1:-1]:
  match=re.fullmatch(r'(0x[0-9a-f]+)=>\{',line)
  if match:address=int(match[1],0);body=[]
  elif line=='}':blocks[address]='\n'.join(body)
  else:body.append(line)
 transfers=re.compile(r'(?<![\w.])pc=(0x[0-9a-f]+);continue;')
 costs={address:max(1,body.count('// 0x')) for address,body in blocks.items()}
 stats={'forward_edges':0,'duplicated_instructions':0}
 def expand(address,remaining):
  def replace(match):
   target=int(match[1],0)
   if target<=address or target not in blocks or costs[target]>remaining[0]:return match[0]
   remaining[0]-=costs[target]
   stats['forward_edges']+=1;stats['duplicated_instructions']+=costs[target]
   return '{'+expand(target,remaining)+'}'
  return transfers.sub(replace,blocks[address])
 result=[lines[0]]
 for address in blocks:result += [f'{address:#x}=>{{',expand(address,[budget]),'}']
 return result+[lines[-1]],stats

def generate(profile,out,extra):
 source=SOURCE.read_bytes();assert hashlib.sha256(source).hexdigest()==EXPECTED
 cpdir=ROOT/'analysis/native-pipeline/oracle-export-01/stage01-attempt001/state';cp=json.loads((cpdir/'checkpoint.json').read_text())
 rows=list(csv.DictReader((ROOT/'analysis/native-engine-index/unwind-functions.tsv').open(),delimiter='\t'));starts=[int(r['start'],0)+BASE for r in rows]
 rows=[(int(r['start'],0)+BASE,int(r['end_exclusive'],0)+BASE) for r in rows]
 x=json.loads(profile.read_text());assert x['source_sha256']==EXPECTED
 ranges={};observed=collections.defaultdict(dict);md=Cs(CS_ARCH_ARM64,CS_MODE_ARM);md.detail=True
 # Bytes outside game text are bound to their exact retained checkpoint region.
 regions=[]
 for r in cp['regions']:
  if r['file'] in ('game.bin','cxx_runtime.bin','sentry_runtime.bin'):
   data=(cpdir/r['file']).read_bytes();assert hashlib.sha256(data).hexdigest()==r['sha256'];regions.append((int(r['base'],0),data))
 def read(p,n):
  if BASE+0x7706b0<=p<BASE+0x187d10c:return source[p-BASE:p-BASE+n]
  for start,data in regions:
   if start<=p and p+n<=start+len(data):return data[p-start:p-start+n]
  raise ValueError(f'Code outside retained executable inputs: {p:#x}')
 for block in x['blocks']:
  pc=int(block['rva'],0)+BASE
  if pc in (int(str(cp['emulation_abi']['return_address']),0),):continue
  external=int(str(cp['emulation_abi']['external_base']),0)
  if external<=pc<external+cp['emulation_abi']['external_size']:continue
  j=bisect.bisect_right(starts,pc)-1
  if j>=0 and pc<rows[j][1]:ranges[rows[j][0]]=rows[j]
  else:ranges[pc]=(pc,pc+block['bytes'])
 config=ROOT/'rust-engine/codegen.json'
 settings=json.loads(config.read_text()) if config.exists() else {}
 extra=list(extra)+settings.get('extra_rvas',[])
 thread_targets={int(rva,0)+BASE for rva in settings.get('forward_thread_rvas',[])}
 thread_budget=settings.get('forward_thread_instruction_budget',64)
 for rva in extra:
  pc=int(rva,0)+BASE;j=bisect.bisect_right(starts,pc)-1
  if j<0 or not rows[j][0]<=pc<rows[j][1]:raise ValueError('Extra RVA not in FDE '+rva)
  ranges[rows[j][0]]=rows[j]
 # Whole external functions are bounded by the original ELF symbol table,
 # not guessed coverage extents. Bind every byte to the retained checkpoint.
 runtime_functions=[]
 for entry in settings.get('extra_runtime_functions',[]):
  from io import BytesIO
  from elftools.elf.elffile import ELFFile
  from elftools.elf.sections import SymbolTableSection
  raw=(ROOT/entry['source_elf']).read_bytes()
  if hashlib.sha256(raw).hexdigest()!=entry['source_sha256']:raise ValueError('Runtime ELF identity mismatch')
  elf=ELFFile(BytesIO(raw))
  symbols=[sym for section in elf.iter_sections() if isinstance(section,SymbolTableSection) for sym in section.iter_symbols() if sym.name==entry['symbol'] and sym['st_size']]
  bounds={(int(sym['st_value']),int(sym['st_size'])) for sym in symbols}
  if len(bounds)!=1:raise ValueError('Runtime symbol extent is missing or ambiguous')
  offset,size=next(iter(bounds))
  region=next(r for r in cp['regions'] if r['file']==entry['region'])
  lo=int(region['base'],0)+offset;hi=lo+size
  segments=[s for s in elf.iter_segments() if s['p_type']=='PT_LOAD' and s['p_flags']&1 and s['p_vaddr']<=offset and offset+size<=s['p_vaddr']+s['p_filesz']]
  if len(segments)!=1:raise ValueError('Runtime symbol outside one executable segment')
  segment=segments[0];file_offset=segment['p_offset']+offset-segment['p_vaddr']
  if read(lo,size)!=raw[file_offset:file_offset+size]:raise ValueError('Runtime function differs from checkpoint bytes')
  ranges[lo]=(lo,hi)
  runtime_functions.append(dict(entry,start=hex(lo),end_exclusive=hex(hi),bytes=size))
 merged={}
 for lo,hi in sorted(ranges.values()):
  if merged and (lo>=BASE+0x187d10c):
   last=next(reversed(merged))
   if lo<=merged[last][1] and last>=BASE+0x187d10c:
    merged[last]=(last,max(hi,merged[last][1]));continue
  merged[lo]=(lo,hi)
 ranges=merged
 out.mkdir(parents=True,exist_ok=True)
 functions={start:(int.from_bytes(hashlib.sha256(str(start).encode()).digest()[:2],'little')%16,f'f_{start:x}') for start in sorted(ranges)}
 compiler=Compiler(functions);chunks=[[]for _ in range(16)];unsupported=collections.Counter();total=0;exports=[];threaded={}
 for start,(lo,hi) in sorted(ranges.items()):
  ins=list(md.disasm(read(lo,hi-lo),lo));total+=len(ins)
  boundaries={lo}|{int(block['rva'],0)+BASE for block in x['blocks'] if lo<=int(block['rva'],0)+BASE<hi}
  for i in ins:
   m=i.mnemonic
   if m=='b' or m.startswith('b.') or m in ('cbz','cbnz','tbz','tbnz'):
    t=i.operands[-1].imm
    if lo<=t<hi:boundaries.add(t)
    boundaries.add(i.address+4)
   elif m in ('ret','br'):boundaries.add(i.address+4)
  group,name=functions[start];lines=[f'#[inline]\npub(super) unsafe fn {name}(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{{loop{{match pc{{']
  opened=False;terminated=False
  for i in ins:
   if i.address in boundaries:
    if opened:
     if not terminated:lines.append(f'pc={i.address:#x};continue;')
     lines.append('}')
    lines.append(f'{i.address:#x}=>{{');opened=True;terminated=False
   try:
    code,terminated=compiler.emit(i,lo,hi)
    if i.mnemonic.startswith('f') and i.mnemonic!='fmov':
     inputs=[]
     for op in i.operands[1:] if i.mnemonic not in ('fcmp','fcmpe','fccmp','fccmpe') else i.operands:
      if op.type==ARM64_OP_REG and re.fullmatch('[sd][0-9]+',i.reg_name(op.reg)):
       fpreg=i.reg_name(op.reg);typ='f32' if fpreg[0]=='s' else 'f64';uint='u32' if typ=='f32' else 'u64'
       inputs.append(f'{typ}::from_bits(({rr(fpreg)}) as {uint}) as f64')
       if typ=='f32':code=f'if f32::from_bits(({rr(fpreg)}) as u32).is_subnormal(){{return Err(\"subnormal f32 operand requires architectural handling\".into());}}'+code
     code='fp_domain(cpu,&['+','.join(inputs)+'])?;'+code
     if i.operands and i.operands[0].type==ARM64_OP_REG and re.fullmatch('[sd][0-9]+',i.reg_name(i.operands[0].reg)) and i.mnemonic not in ('fcmp','fcmpe','fccmp','fccmpe'):
      fpreg=i.reg_name(i.operands[0].reg);typ='f32' if fpreg[0]=='s' else 'f64';uint='u32' if typ=='f32' else 'u64'
      code+=f'fp_domain(cpu,&[{typ}::from_bits(({rr(fpreg)}) as {uint}) as f64])?;'

   except (ValueError,KeyError,IndexError) as e:
    unsupported[str(e)]+=1;code='return Err('+json.dumps(f'unsupported {i.address:#x}: {i.mnemonic} {i.op_str}')+'.into());';terminated=True
   # Match the source-bound reference's asymmetric Normal-row deck hooks.
   # These change the scenario vector at precisely the same original PCs;
   # all translated game instructions and CPU state remain unchanged.
   if i.address in (0x111d6f18,0x111d6068):
    code=f'rt.deck_swap_hook({i.address:#x})?;'+code
   lines.append(f'// {i.address:#x} {i.mnemonic} {i.op_str}'.rstrip()+'\n'+code)
  if opened:
   if not terminated:lines.append(f'return Err("fell through function {lo:#x}".into());')
   lines.append('}')
  lines.append(f'_=>return Err(format!("untranslated block {{pc:#x}} in {lo:#x}")),}}}}}}')
  if start in thread_targets:
   lines,stats=thread_forward_edges(lines,thread_budget);threaded[hex(start-BASE)]=stats
  chunks[group].append('\n'.join(lines));exports.append((lo,hi,group,name))
 for j,chunk in enumerate(chunks):
  (out/f'chunk{j}.rs').write_text('#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]\nuse crate::{cpu::Cpu,runtime::Runtime};\nuse super::helpers::*;\n'+'\n'.join(chunk)+'\n')
 mod='//! Generated source-bound original routine translations. Do not edit by hand.\n'
 mod+='#![allow(unreachable_patterns)]\nmod helpers;\n'+''.join(f'mod chunk{j};\n' for j in range(16))
 mod+='use crate::{cpu::Cpu,runtime::Runtime};\n#[inline]\npub unsafe fn dispatch(cpu:&mut Cpu,rt:&mut Runtime,target:u64)->Result<(),String>{cpu.pc=target;match target{\n'
 mod+=''.join(f'{a:#x}..={b-1:#x}=>chunk{j}::{name}(cpu,rt,target),\n'for a,b,j,name in exports)
 mod+='address=>{if rt.imported(cpu,address)? {Ok(())}else{Err(format!("untranslated function {address:#x}"))}}}}\n'
 (out/'mod.rs').write_text(mod)
 report={'source_sha256':EXPECTED,'checkpoint_sha256':hashlib.sha256((cpdir/'checkpoint.json').read_bytes()).hexdigest(),'profile_sha256':hashlib.sha256(profile.read_bytes()).hexdigest(),'runtime_functions':runtime_functions,'function_count':len(ranges),'instruction_count':total,'unsupported_encodings':dict(unsupported),'forward_threading':threaded,'method':'Original FDE bodies and profiled external runtime blocks compiled into Rust scalar/vector operations; no emulator fallback. Unsupported code stops explicitly.','generated_files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in sorted(out.glob('*.rs'))}}
 (ROOT/'analysis/native-port/rust-generation.json').write_text(json.dumps(report,indent=2)+'\n')
 (out/'manifest.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({k:v for k,v in report.items() if k!='generated_files'},indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--profile',type=Path,default=ROOT/'analysis/native-port/profile-combined.json');p.add_argument('--out',type=Path,default=ROOT/'rust-engine/src/generated');p.add_argument('--extra-rva',action='append',default=[]);a=p.parse_args();generate(a.profile,a.out,a.extra_rva)
