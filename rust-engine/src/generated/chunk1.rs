#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_10790864(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10790864=>{
// 0x10790864 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10790868 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1079086c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10790870 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10790874 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10790878 ldr x21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1079087c ldrsw x22, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[22]=(v0) as u64;}
// 0x10790880 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10790884 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x10790888 cbz x21, #0x107908b0
if (cpu.x[21]) == 0 {pc=0x107908b0;continue;}pc=0x1079088c;continue;
}
0x1079088c=>{
// 0x1079088c fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x10790890 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10790894 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10790898 fmov w8, s0
cpu.x[8]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x1079089c cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107908a0 b.hi #0x107908b4
if cpu.condition(8){pc=0x107908b4;continue;}pc=0x107908a4;continue;
}
0x107908a4=>{
// 0x107908a4 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x107908a8 and x23, x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
// 0x107908ac b #0x107908c8
pc=0x107908c8;continue;
}
0x107908b0=>{
// 0x107908b0 b #0x10790970
pc=0x10790970;continue;
}
0x107908b4=>{
// 0x107908b4 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107908b8 mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x107908bc b.hi #0x107908c8
if cpu.condition(8){pc=0x107908c8;continue;}pc=0x107908c0;continue;
}
0x107908c0=>{
// 0x107908c0 udiv x9, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x107908c4 msub x23, x9, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x107908c8;continue;
}
0x107908c8=>{
// 0x107908c8 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x107908cc ldr x9, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x107908d0 cbz x9, #0x10790970
if (cpu.x[9]) == 0 {pc=0x10790970;continue;}pc=0x107908d4;continue;
}
0x107908d4=>{
// 0x107908d4 ldr x0, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x107908d8 cbz x0, #0x10790970
if (cpu.x[0]) == 0 {pc=0x10790970;continue;}pc=0x107908dc;continue;
}
0x107908dc=>{
// 0x107908dc cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107908e0 b.hs #0x107908fc
if cpu.condition(2){pc=0x107908fc;continue;}pc=0x107908e4;continue;
}
0x107908e4=>{
// 0x107908e4 sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x107908e8 b #0x10790940
pc=0x10790940;continue;
}
0x107908ec=>{
// 0x107908ec cmp x8, x23
{let a=((cpu.x[8]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107908f0 b.ne #0x10790970
if cpu.condition(1){pc=0x10790970;continue;}pc=0x107908f4;continue;
}
0x107908f4=>{
// 0x107908f4 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x107908f8 cbz x0, #0x10790970
if (cpu.x[0]) == 0 {pc=0x10790970;continue;}pc=0x107908fc;continue;
}
0x107908fc=>{
// 0x107908fc ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790900 cmp x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790904 b.ne #0x10790918
if cpu.condition(1){pc=0x10790918;continue;}pc=0x10790908;continue;
}
0x10790908=>{
// 0x10790908 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1079090c cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10790910 b.ne #0x107908f4
if cpu.condition(1){pc=0x107908f4;continue;}pc=0x10790914;continue;
}
0x10790914=>{
// 0x10790914 b #0x10790958
pc=0x10790958;continue;
}
0x10790918=>{
// 0x10790918 cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1079091c b.lo #0x107908ec
if cpu.condition(3){pc=0x107908ec;continue;}pc=0x10790920;continue;
}
0x10790920=>{
// 0x10790920 udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10790924 msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x10790928 b #0x107908ec
pc=0x107908ec;continue;
}
0x1079092c=>{
// 0x1079092c and x9, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10790930 cmp x9, x23
{let a=((cpu.x[9]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790934 b.ne #0x10790970
if cpu.condition(1){pc=0x10790970;continue;}pc=0x10790938;continue;
}
0x10790938=>{
// 0x10790938 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1079093c cbz x0, #0x10790970
if (cpu.x[0]) == 0 {pc=0x10790970;continue;}pc=0x10790940;continue;
}
0x10790940=>{
// 0x10790940 ldr x9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790944 cmp x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790948 b.ne #0x1079092c
if cpu.condition(1){pc=0x1079092c;continue;}pc=0x1079094c;continue;
}
0x1079094c=>{
// 0x1079094c ldr w9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10790950 cmp w9, w22
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10790954 b.ne #0x10790938
if cpu.condition(1){pc=0x10790938;continue;}pc=0x10790958;continue;
}
0x10790958=>{
// 0x10790958 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x1079095c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10790960 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10790964 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10790968 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1079096c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10790970=>{
// 0x10790970 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10790974 bl #0x1187d2b0
cpu.x[30]=0x10790978;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10790978;continue;
}
0x10790978=>{
// 0x10790978 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1079097c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790980 ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x10790984 stp xzr, x22, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10790988 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x1079098c ldr w9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10790990 ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x10790994 stp w9, wzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10790998 cbz x21, #0x107909c4
if (cpu.x[21]) == 0 {pc=0x107909c4;continue;}pc=0x1079099c;continue;
}
0x1079099c=>{
// 0x1079099c ucvtf s2, x21
cpu.q[2]=(((cpu.x[21]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x107909a0 fmul s2, s1, s2
fp_domain(cpu,&[f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[2]=((f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)*f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x107909a4 fcmp s2, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}{let a=f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32);let b=f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32);cpu.nzcv=if a.is_nan()||b.is_nan(){0x30000000}else if a==b{0x60000000}else if a<b{0x80000000}else{0x20000000};}
// 0x107909a8 b.mi #0x107909c4
if cpu.condition(4){pc=0x107909c4;continue;}pc=0x107909ac;continue;
}
0x107909ac=>{
// 0x107909ac ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x107909b0 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x107909b4 cbz x8, #0x10790af8
if (cpu.x[8]) == 0 {pc=0x10790af8;continue;}pc=0x107909b8;continue;
}
0x107909b8=>{
// 0x107909b8 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x107909bc str x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x107909c0 b #0x10790b78
pc=0x10790b78;continue;
}
0x107909c4=>{
// 0x107909c4 lsl x8, x21, #1
cpu.x[8]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x107909c8 cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107909cc b.lo #0x107909e0
if cpu.condition(3){pc=0x107909e0;continue;}pc=0x107909d0;continue;
}
0x107909d0=>{
// 0x107909d0 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x107909d4 tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x107909d8 cset w9, ne
cpu.x[9]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x107909dc b #0x107909e4
pc=0x107909e4;continue;
}
0x107909e0=>{
// 0x107909e0 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
pc=0x107909e4;continue;
}
0x107909e4=>{
// 0x107909e4 fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x107909e8 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x107909ec fcvtpu x10, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[10]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x107909f0 cmp x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x107909f4 csel x1, x8, x10, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[8]))}else{((cpu.x[10]))}) as u64;
// 0x107909f8 subs x8, x1, #1
{let a=((cpu.x[1]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x107909fc b.ne #0x10790a08
if cpu.condition(1){pc=0x10790a08;continue;}pc=0x10790a00;continue;
}
0x10790a00=>{
// 0x10790a00 mov w1, #2
cpu.x[1]=(((0x2u64))) as u32 as u64;
// 0x10790a04 b #0x10790a28
pc=0x10790a28;continue;
}
0x10790a08=>{
// 0x10790a08 tst x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10790a0c b.eq #0x10790a28
if cpu.condition(0){pc=0x10790a28;continue;}pc=0x10790a10;continue;
}
0x10790a10=>{
// 0x10790a10 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10790a14 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10790a18 bl #0x1187d3f0
cpu.x[30]=0x10790a1c;super::dispatch(cpu,rt,0x1187d3f0)?;
// 0x10790a1c ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10790a20 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10790a24 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
pc=0x10790a28;continue;
}
0x10790a28=>{
// 0x10790a28 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790a2c b.ls #0x10790a44
if cpu.condition(9){pc=0x10790a44;continue;}pc=0x10790a30;continue;
}
0x10790a30=>{
// 0x10790a30 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10790a34 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10790a38 bl #0x10790ba0
cpu.x[30]=0x10790a3c;super::chunk14::f_10790ba0(cpu,rt,0x10790ba0)?;
pc=0x10790a3c;continue;
}
0x10790a3c=>{
// 0x10790a3c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10790a40 b #0x10790ad8
pc=0x10790ad8;continue;
}
0x10790a44=>{
// 0x10790a44 b.hs #0x10790ad8
if cpu.condition(2){pc=0x10790ad8;continue;}pc=0x10790a48;continue;
}
0x10790a48=>{
// 0x10790a48 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790a4c ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x10790a50 cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790a54 ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x10790a58 fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x10790a5c fcvtpu x8, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[8]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x10790a60 b.lo #0x10790aac
if cpu.condition(3){pc=0x10790aac;continue;}pc=0x10790a64;continue;
}
0x10790a64=>{
// 0x10790a64 fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x10790a68 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10790a6c uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10790a70 fmov w9, s0
cpu.x[9]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10790a74 cmp x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790a78 b.hi #0x10790aac
if cpu.condition(8){pc=0x10790aac;continue;}pc=0x10790a7c;continue;
}
0x10790a7c=>{
// 0x10790a7c sub x9, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10790a80 mov w10, #1
cpu.x[10]=(((0x1u64))) as u32 as u64;
// 0x10790a84 cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790a88 clz x9, x9
cpu.x[9]=((((cpu.x[9])) as u64).leading_zeros() as u64) as u64;
// 0x10790a8c neg x9, x9
cpu.x[9]=(0u64.wrapping_sub(((cpu.x[9])))) as u64;
// 0x10790a90 lsl x9, x10, x9
cpu.x[9]=(((((cpu.x[10]))) as u64).wrapping_shl(((((cpu.x[9]))) & 63) as u32) as u64) as u64;
// 0x10790a94 csel x8, x8, x9, lo
cpu.x[8]=(if cpu.condition(3){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x10790a98 cmp x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790a9c csel x1, x1, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[1]))}else{((cpu.x[8]))}) as u64;
// 0x10790aa0 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790aa4 b.lo #0x10790a30
if cpu.condition(3){pc=0x10790a30;continue;}pc=0x10790aa8;continue;
}
0x10790aa8=>{
// 0x10790aa8 b #0x10790ad8
pc=0x10790ad8;continue;
}
0x10790aac=>{
// 0x10790aac mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10790ab0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10790ab4 mov x23, x1
cpu.x[23]=(((cpu.x[1]))) as u64;
// 0x10790ab8 bl #0x1187d3f0
cpu.x[30]=0x10790abc;super::dispatch(cpu,rt,0x1187d3f0)?;
// 0x10790abc mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10790ac0 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10790ac4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10790ac8 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790acc csel x1, x23, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[23]))}else{((cpu.x[8]))}) as u64;
// 0x10790ad0 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790ad4 b.lo #0x10790a30
if cpu.condition(3){pc=0x10790a30;continue;}pc=0x10790ad8;continue;
}
0x10790ad8=>{
// 0x10790ad8 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10790adc sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10790ae0 tst x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10790ae4 b.ne #0x10790b2c
if cpu.condition(1){pc=0x10790b2c;continue;}pc=0x10790ae8;continue;
}
0x10790ae8=>{
// 0x10790ae8 and x23, x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
// 0x10790aec ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790af0 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790af4 cbnz x8, #0x107909b8
if (cpu.x[8]) != 0 {pc=0x107909b8;continue;}pc=0x10790af8;continue;
}
0x10790af8=>{
// 0x10790af8 mov x8, x19
cpu.x[8]=(((cpu.x[19]))) as u64;
// 0x10790afc ldr x10, [x8, #0x10]!
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[8]=(address) as u64;}
// 0x10790b00 str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10790b04 str x10, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10790b08 str x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10790b0c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790b10 cbz x8, #0x10790b7c
if (cpu.x[8]) == 0 {pc=0x10790b7c;continue;}pc=0x10790b14;continue;
}
0x10790b14=>{
// 0x10790b14 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10790b18 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790b1c tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10790b20 b.ne #0x10790b60
if cpu.condition(1){pc=0x10790b60;continue;}pc=0x10790b24;continue;
}
0x10790b24=>{
// 0x10790b24 and x8, x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10790b28 b #0x10790b70
pc=0x10790b70;continue;
}
0x10790b2c=>{
// 0x10790b2c cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790b30 b.ls #0x10790b48
if cpu.condition(9){pc=0x10790b48;continue;}pc=0x10790b34;continue;
}
0x10790b34=>{
// 0x10790b34 mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x10790b38 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790b3c ldr x8, [x9, x22, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790b40 cbnz x8, #0x107909b8
if (cpu.x[8]) != 0 {pc=0x107909b8;continue;}pc=0x10790b44;continue;
}
0x10790b44=>{
// 0x10790b44 b #0x10790af8
pc=0x10790af8;continue;
}
0x10790b48=>{
// 0x10790b48 udiv x8, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10790b4c msub x23, x8, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[8]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x10790b50 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790b54 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790b58 cbnz x8, #0x107909b8
if (cpu.x[8]) != 0 {pc=0x107909b8;continue;}pc=0x10790b5c;continue;
}
0x10790b5c=>{
// 0x10790b5c b #0x10790af8
pc=0x10790af8;continue;
}
0x10790b60=>{
// 0x10790b60 cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10790b64 b.lo #0x10790b70
if cpu.condition(3){pc=0x10790b70;continue;}pc=0x10790b68;continue;
}
0x10790b68=>{
// 0x10790b68 udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10790b6c msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x10790b70;continue;
}
0x10790b70=>{
// 0x10790b70 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10790b74 add x8, x9, x8, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10790b78;continue;
}
0x10790b78=>{
// 0x10790b78 str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10790b7c;continue;
}
0x10790b7c=>{
// 0x10790b7c ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10790b80 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10790b84 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10790b88 str x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10790b8c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10790b90 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10790b94 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10790b98 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10790b9c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10790864")),}}}
#[inline]
pub(super) unsafe fn f_108346ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x108346ec=>{
// 0x108346ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x108346ec")),}}}
#[inline]
pub(super) unsafe fn f_108346f8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x108346f8=>{
// 0x108346f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x108346f8")),}}}
#[inline]
pub(super) unsafe fn f_10834724(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834724=>{
// 0x10834724 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834724")),}}}
#[inline]
pub(super) unsafe fn f_10d17cb4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d17cb4=>{
// 0x10d17cb4 fmov d0, xzr
cpu.q[0]=(0u64) as u128 & 0xffffffffffffffffu128;
// 0x10d17cb8 movi v1.2d, #0000000000000000
cpu.q[1]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d17cbc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d17cc0 str w8, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17cc4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d17cc8 strh wzr, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10d17ccc mov v0.d[1], v0.d[0]
cpu.q[0]=(cpu.q[0] & !(0xffffffffffffffffu128 << 64)) | (((((cpu.q[0] >> 0) & 0xffffffffffffffffu128)) as u128 & 0xffffffffffffffffu128) << 64);
// 0x10d17cd0 str xzr, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17cd4 stur q1, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17cd8 stur q1, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17cdc stur q1, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17ce0 stur q0, [x0, #0x68]
{let address=(cpu.x[0]).wrapping_add(0x68u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17ce4 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d17ce8 stur q1, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17cec stur q1, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d17cf0 str xzr, [x0, #0x80]
{let address=(cpu.x[0]).wrapping_add(0x80u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17cf4 str wzr, [x0, #0x88]
{let address=(cpu.x[0]).wrapping_add(0x88u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17cf8 stp xzr, xzr, [x0, #0x98]
{let address=(cpu.x[0]).wrapping_add(0x98u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d17cfc str xzr, [x0, #0x90]
{let address=(cpu.x[0]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d00 str wzr, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17d04 stur d0, [x0, #0xac]
{let address=(cpu.x[0]).wrapping_add(0xacu64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d08 str w8, [x0, #0xb4]
{let address=(cpu.x[0]).wrapping_add(0xb4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17d0c strb wzr, [x0, #0xb8]
{let address=(cpu.x[0]).wrapping_add(0xb8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d17d10 stp xzr, xzr, [x0, #0xc0]
{let address=(cpu.x[0]).wrapping_add(0xc0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d17d14 strb wzr, [x0, #0xd0]
{let address=(cpu.x[0]).wrapping_add(0xd0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d17d18 str x1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d1c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d17cb4")),}}}
#[inline]
pub(super) unsafe fn f_10d17d20(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d17d20=>{
// 0x10d17d20 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d17d24 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d28 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d17d2c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d17d30 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d17d34 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d17d38 ldr x0, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d3c cbz x0, #0x10d17d54
if (cpu.x[0]) == 0 {pc=0x10d17d54;continue;}pc=0x10d17d40;continue;
}
0x10d17d40=>{
// 0x10d17d40 bl #0x10d58768
cpu.x[30]=0x10d17d44;super::dispatch(cpu,rt,0x10d58768)?;
// 0x10d17d44 ldr x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d48 cbz x0, #0x10d17d50
if (cpu.x[0]) == 0 {pc=0x10d17d50;continue;}pc=0x10d17d4c;continue;
}
0x10d17d4c=>{
// 0x10d17d4c bl #0x1187d2c0
cpu.x[30]=0x10d17d50;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17d50;continue;
}
0x10d17d50=>{
// 0x10d17d50 str xzr, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d17d54;continue;
}
0x10d17d54=>{
// 0x10d17d54 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d58 cbz x0, #0x10d17d60
if (cpu.x[0]) == 0 {pc=0x10d17d60;continue;}pc=0x10d17d5c;continue;
}
0x10d17d5c=>{
// 0x10d17d5c bl #0x1187d2c0
cpu.x[30]=0x10d17d60;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17d60;continue;
}
0x10d17d60=>{
// 0x10d17d60 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d64 str xzr, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d68 cbz x0, #0x10d17d70
if (cpu.x[0]) == 0 {pc=0x10d17d70;continue;}pc=0x10d17d6c;continue;
}
0x10d17d6c=>{
// 0x10d17d6c bl #0x1187d2c0
cpu.x[30]=0x10d17d70;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17d70;continue;
}
0x10d17d70=>{
// 0x10d17d70 ldr x0, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d74 str xzr, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17d78 cbz x0, #0x10d17da0
if (cpu.x[0]) == 0 {pc=0x10d17da0;continue;}pc=0x10d17d7c;continue;
}
0x10d17d7c=>{
// 0x10d17d7c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17d80 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17d84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d17d88;super::dispatch(cpu,rt,target)?;}
// 0x10d17d88 ldr x0, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17d8c cbz x0, #0x10d17d9c
if (cpu.x[0]) == 0 {pc=0x10d17d9c;continue;}pc=0x10d17d90;continue;
}
0x10d17d90=>{
// 0x10d17d90 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17d94 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17d98 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d17d9c;super::dispatch(cpu,rt,target)?;}
pc=0x10d17d9c;continue;
}
0x10d17d9c=>{
// 0x10d17d9c str xzr, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d17da0;continue;
}
0x10d17da0=>{
// 0x10d17da0 ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17da4 cbz x0, #0x10d17dcc
if (cpu.x[0]) == 0 {pc=0x10d17dcc;continue;}pc=0x10d17da8;continue;
}
0x10d17da8=>{
// 0x10d17da8 bl #0x10d65070
cpu.x[30]=0x10d17dac;super::dispatch(cpu,rt,0x10d65070)?;
// 0x10d17dac ldr x20, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d17db0 cbz x20, #0x10d17dc8
if (cpu.x[20]) == 0 {pc=0x10d17dc8;continue;}pc=0x10d17db4;continue;
}
0x10d17db4=>{
// 0x10d17db4 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17db8 cbz x0, #0x10d17dc0
if (cpu.x[0]) == 0 {pc=0x10d17dc0;continue;}pc=0x10d17dbc;continue;
}
0x10d17dbc=>{
// 0x10d17dbc bl #0x1187d360
cpu.x[30]=0x10d17dc0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d17dc0;continue;
}
0x10d17dc0=>{
// 0x10d17dc0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d17dc4 bl #0x1187d2c0
cpu.x[30]=0x10d17dc8;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17dc8;continue;
}
0x10d17dc8=>{
// 0x10d17dc8 str xzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d17dcc;continue;
}
0x10d17dcc=>{
// 0x10d17dcc ldr x0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17dd0 cbz x0, #0x10d17df8
if (cpu.x[0]) == 0 {pc=0x10d17df8;continue;}pc=0x10d17dd4;continue;
}
0x10d17dd4=>{
// 0x10d17dd4 bl #0x10d63544
cpu.x[30]=0x10d17dd8;super::dispatch(cpu,rt,0x10d63544)?;
// 0x10d17dd8 ldr x20, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d17ddc cbz x20, #0x10d17df4
if (cpu.x[20]) == 0 {pc=0x10d17df4;continue;}pc=0x10d17de0;continue;
}
0x10d17de0=>{
// 0x10d17de0 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17de4 cbz x0, #0x10d17dec
if (cpu.x[0]) == 0 {pc=0x10d17dec;continue;}pc=0x10d17de8;continue;
}
0x10d17de8=>{
// 0x10d17de8 bl #0x1187d360
cpu.x[30]=0x10d17dec;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d17dec;continue;
}
0x10d17dec=>{
// 0x10d17dec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d17df0 bl #0x1187d2c0
cpu.x[30]=0x10d17df4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17df4;continue;
}
0x10d17df4=>{
// 0x10d17df4 str xzr, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d17df8;continue;
}
0x10d17df8=>{
// 0x10d17df8 ldr x20, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d17dfc cbz x20, #0x10d17e80
if (cpu.x[20]) == 0 {pc=0x10d17e80;continue;}pc=0x10d17e00;continue;
}
0x10d17e00=>{
// 0x10d17e00 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d17e04 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d17e08 b.le #0x10d17e98
if cpu.condition(13){pc=0x10d17e98;continue;}pc=0x10d17e0c;continue;
}
0x10d17e0c=>{
// 0x10d17e0c lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d17e10 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d17e14 b #0x10d17e24
pc=0x10d17e24;continue;
}
0x10d17e18=>{
// 0x10d17e18 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d17e1c cmp x22, x23
{let a=((cpu.x[22]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d17e20 b.eq #0x10d17e5c
if cpu.condition(0){pc=0x10d17e5c;continue;}pc=0x10d17e24;continue;
}
0x10d17e24=>{
// 0x10d17e24 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17e28 ldr x21, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d17e2c cbz x21, #0x10d17e18
if (cpu.x[21]) == 0 {pc=0x10d17e18;continue;}pc=0x10d17e30;continue;
}
0x10d17e30=>{
// 0x10d17e30 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d17e34 bl #0x10f20278
cpu.x[30]=0x10d17e38;super::dispatch(cpu,rt,0x10f20278)?;
// 0x10d17e38 add x0, x21, #0x20
{let a=((cpu.x[21]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d17e3c bl #0x1142fcc4
cpu.x[30]=0x10d17e40;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d17e40 add x0, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d17e44 bl #0x1142fcc4
cpu.x[30]=0x10d17e48;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d17e48 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d17e4c bl #0x1142fcc4
cpu.x[30]=0x10d17e50;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d17e50 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d17e54 bl #0x1187d2c0
cpu.x[30]=0x10d17e58;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d17e58 b #0x10d17e18
pc=0x10d17e18;continue;
}
0x10d17e5c=>{
// 0x10d17e5c ldr x8, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d17e60 str wzr, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17e64 mov x20, x8
cpu.x[20]=(((cpu.x[8]))) as u64;
// 0x10d17e68 cbz x8, #0x10d17e80
if (cpu.x[8]) == 0 {pc=0x10d17e80;continue;}pc=0x10d17e6c;continue;
}
0x10d17e6c=>{
// 0x10d17e6c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17e70 cbz x0, #0x10d17e78
if (cpu.x[0]) == 0 {pc=0x10d17e78;continue;}pc=0x10d17e74;continue;
}
0x10d17e74=>{
// 0x10d17e74 bl #0x1187d360
cpu.x[30]=0x10d17e78;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d17e78;continue;
}
0x10d17e78=>{
// 0x10d17e78 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d17e7c bl #0x1187d2c0
cpu.x[30]=0x10d17e80;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d17e80;continue;
}
0x10d17e80=>{
// 0x10d17e80 str xzr, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d17e84 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d17e88 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d17e8c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d17e90 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d17e94 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d17e98=>{
// 0x10d17e98 str wzr, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d17e9c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d17ea0 cbnz x0, #0x10d17e74
if (cpu.x[0]) != 0 {pc=0x10d17e74;continue;}pc=0x10d17ea4;continue;
}
0x10d17ea4=>{
// 0x10d17ea4 b #0x10d17e78
pc=0x10d17e78;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d17d20")),}}}
#[inline]
pub(super) unsafe fn f_10d2391c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2391c=>{
// 0x10d2391c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d23920 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d23924 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d23928 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d2392c ldr x20, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d23930 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d23934 cbz x20, #0x10d2399c
if (cpu.x[20]) == 0 {pc=0x10d2399c;continue;}pc=0x10d23938;continue;
}
0x10d23938=>{
// 0x10d23938 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d2393c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d23940 b.lt #0x10d239b8
if cpu.condition(11){pc=0x10d239b8;continue;}pc=0x10d23944;continue;
}
0x10d23944=>{
// 0x10d23944 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d23948 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d2394c b #0x10d2395c
pc=0x10d2395c;continue;
}
0x10d23950=>{
// 0x10d23950 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d23954 cmp x22, x21
{let a=((cpu.x[22]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23958 b.eq #0x10d23988
if cpu.condition(0){pc=0x10d23988;continue;}pc=0x10d2395c;continue;
}
0x10d2395c=>{
// 0x10d2395c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23960 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23964 cbz x8, #0x10d23950
if (cpu.x[8]) == 0 {pc=0x10d23950;continue;}pc=0x10d23968;continue;
}
0x10d23968=>{
// 0x10d23968 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2396c cbz x0, #0x10d23950
if (cpu.x[0]) == 0 {pc=0x10d23950;continue;}pc=0x10d23970;continue;
}
0x10d23970=>{
// 0x10d23970 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d23974 bl #0x10e0bc20
cpu.x[30]=0x10d23978;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d23978;continue;
}
0x10d23978=>{
// 0x10d23978 tbz w0, #0, #0x10d23950
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d23950;continue;}pc=0x10d2397c;continue;
}
0x10d2397c=>{
// 0x10d2397c mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d23980 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23984 b.ne #0x10d239cc
if cpu.condition(1){pc=0x10d239cc;continue;}pc=0x10d23988;continue;
}
0x10d23988=>{
// 0x10d23988 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d2398c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d23990 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d23994 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d23998 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d2399c=>{
// 0x10d2399c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d239a0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d239a4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d239a8 bl #0x1128df18
cpu.x[30]=0x10d239ac;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d239ac ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d239b0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d239b4 b.ge #0x10d23944
if cpu.condition(10){pc=0x10d23944;continue;}pc=0x10d239b8;continue;
}
0x10d239b8=>{
// 0x10d239b8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d239bc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d239c0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d239c4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d239c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d239cc=>{
// 0x10d239cc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d239d0 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d239d4 ldr w0, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10d239d8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d239dc ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d239e0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d239e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2391c")),}}}
#[inline]
pub(super) unsafe fn f_10d3352c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d3352c=>{
// 0x10d3352c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d33530 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d33534 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d33538 adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10d3353c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d33540 ldr x8, [x8, #0x130]
{let address=(cpu.x[8]).wrapping_add(0x130u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33544 ldr x9, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d33548 stp xzr, xzr, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3354c add x8, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d33550 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33554 cbz x9, #0x10d33598
if (cpu.x[9]) == 0 {pc=0x10d33598;continue;}pc=0x10d33558;continue;
}
0x10d33558=>{
// 0x10d33558 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3355c cbz x0, #0x10d33570
if (cpu.x[0]) == 0 {pc=0x10d33570;continue;}pc=0x10d33560;continue;
}
0x10d33560=>{
// 0x10d33560 ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33564 bl #0x1187d2c0
cpu.x[30]=0x10d33568;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d33568 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3356c cbnz x20, #0x10d33560
if (cpu.x[20]) != 0 {pc=0x10d33560;continue;}pc=0x10d33570;continue;
}
0x10d33570=>{
// 0x10d33570 ldr x8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33574 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33578 cbz x8, #0x10d33594
if (cpu.x[8]) == 0 {pc=0x10d33594;continue;}pc=0x10d3357c;continue;
}
0x10d3357c=>{
// 0x10d3357c mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10d33580;continue;
}
0x10d33580=>{
// 0x10d33580 ldr x10, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d33584 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33588 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d3358c cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d33590 b.ne #0x10d33580
if cpu.condition(1){pc=0x10d33580;continue;}pc=0x10d33594;continue;
}
0x10d33594=>{
// 0x10d33594 str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33598;continue;
}
0x10d33598=>{
// 0x10d33598 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3359c cbz x0, #0x10d335b0
if (cpu.x[0]) == 0 {pc=0x10d335b0;continue;}pc=0x10d335a0;continue;
}
0x10d335a0=>{
// 0x10d335a0 ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d335a4 bl #0x1187d2c0
cpu.x[30]=0x10d335a8;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d335a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d335ac cbnz x20, #0x10d335a0
if (cpu.x[20]) != 0 {pc=0x10d335a0;continue;}pc=0x10d335b0;continue;
}
0x10d335b0=>{
// 0x10d335b0 ldr x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d335b4 str xzr, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d335b8 cbz x0, #0x10d335c0
if (cpu.x[0]) == 0 {pc=0x10d335c0;continue;}pc=0x10d335bc;continue;
}
0x10d335bc=>{
// 0x10d335bc bl #0x1187d2c0
cpu.x[30]=0x10d335c0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d335c0;continue;
}
0x10d335c0=>{
// 0x10d335c0 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d335c4 cbz x8, #0x10d33608
if (cpu.x[8]) == 0 {pc=0x10d33608;continue;}pc=0x10d335c8;continue;
}
0x10d335c8=>{
// 0x10d335c8 ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d335cc cbz x0, #0x10d335e0
if (cpu.x[0]) == 0 {pc=0x10d335e0;continue;}pc=0x10d335d0;continue;
}
0x10d335d0=>{
// 0x10d335d0 ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d335d4 bl #0x1187d2c0
cpu.x[30]=0x10d335d8;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d335d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d335dc cbnz x20, #0x10d335d0
if (cpu.x[20]) != 0 {pc=0x10d335d0;continue;}pc=0x10d335e0;continue;
}
0x10d335e0=>{
// 0x10d335e0 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d335e4 str xzr, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d335e8 cbz x8, #0x10d33604
if (cpu.x[8]) == 0 {pc=0x10d33604;continue;}pc=0x10d335ec;continue;
}
0x10d335ec=>{
// 0x10d335ec mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10d335f0;continue;
}
0x10d335f0=>{
// 0x10d335f0 ldr x10, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d335f4 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d335f8 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d335fc cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d33600 b.ne #0x10d335f0
if cpu.condition(1){pc=0x10d335f0;continue;}pc=0x10d33604;continue;
}
0x10d33604=>{
// 0x10d33604 str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33608;continue;
}
0x10d33608=>{
// 0x10d33608 ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3360c cbz x0, #0x10d33620
if (cpu.x[0]) == 0 {pc=0x10d33620;continue;}pc=0x10d33610;continue;
}
0x10d33610=>{
// 0x10d33610 ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33614 bl #0x1187d2c0
cpu.x[30]=0x10d33618;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d33618 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3361c cbnz x20, #0x10d33610
if (cpu.x[20]) != 0 {pc=0x10d33610;continue;}pc=0x10d33620;continue;
}
0x10d33620=>{
// 0x10d33620 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33624 str xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33628 cbz x0, #0x10d33638
if (cpu.x[0]) == 0 {pc=0x10d33638;continue;}pc=0x10d3362c;continue;
}
0x10d3362c=>{
// 0x10d3362c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d33630 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d33634 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
0x10d33638=>{
// 0x10d33638 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3363c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d33640 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d3352c")),}}}
#[inline]
pub(super) unsafe fn f_10d36098(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d36098=>{
// 0x10d36098 ldr w8, [x0, #0x1fc]
{let address=(cpu.x[0]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3609c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d360a0 cset w0, gt
cpu.x[0]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10d360a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d36098")),}}}
#[inline]
pub(super) unsafe fn f_10d374d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d374d4=>{
// 0x10d374d4 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d374d8 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d374dc cbz x0, #0x10d374e4
if (cpu.x[0]) == 0 {pc=0x10d374e4;continue;}pc=0x10d374e0;continue;
}
0x10d374e0=>{
// 0x10d374e0 b #0x10e2d928
super::chunk15::f_10e2d928(cpu,rt,0x10e2d928)?;return Ok(());
}
0x10d374e4=>{
// 0x10d374e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d374d4")),}}}
#[inline]
pub(super) unsafe fn f_10d40aa0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d40aa0=>{
// 0x10d40aa0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d40aa4 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40aa8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40aac mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d40ab0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d40ab4 ldr x0, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40ab8 mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d40abc mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d40ac0 bl #0x10d193d4
cpu.x[30]=0x10d40ac4;super::chunk4::f_10d193d4(cpu,rt,0x10d193d4)?;
pc=0x10d40ac4;continue;
}
0x10d40ac4=>{
// 0x10d40ac4 tbz w0, #0, #0x10d40cac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d40cac;continue;}pc=0x10d40ac8;continue;
}
0x10d40ac8=>{
// 0x10d40ac8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d40acc mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d40ad0 bl #0x10d41a04
cpu.x[30]=0x10d40ad4;super::chunk6::f_10d41a04(cpu,rt,0x10d41a04)?;
pc=0x10d40ad4;continue;
}
0x10d40ad4=>{
// 0x10d40ad4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ad8 b.lt #0x10d40b14
if cpu.condition(11){pc=0x10d40b14;continue;}pc=0x10d40adc;continue;
}
0x10d40adc=>{
// 0x10d40adc mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40ae0 cmp w21, #6
{let a=((cpu.x[21] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ae4 b.lo #0x10d40af8
if cpu.condition(3){pc=0x10d40af8;continue;}pc=0x10d40ae8;continue;
}
0x10d40ae8=>{
// 0x10d40ae8 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d40aec add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d40af0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d40af4 bl #0x1128df18
cpu.x[30]=0x10d40af8;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d40af8;continue;
}
0x10d40af8=>{
// 0x10d40af8 add x8, x19, w21, sxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[21] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d40afc ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40b00 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b04 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40b08 bl #0x110cc5b4
cpu.x[30]=0x10d40b0c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d40b0c mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d40b10 bl #0x10d28788
cpu.x[30]=0x10d40b14;super::dispatch(cpu,rt,0x10d28788)?;
pc=0x10d40b14;continue;
}
0x10d40b14=>{
// 0x10d40b14 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d40b18 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d40b1c bl #0x10d41a04
cpu.x[30]=0x10d40b20;super::chunk6::f_10d41a04(cpu,rt,0x10d41a04)?;
pc=0x10d40b20;continue;
}
0x10d40b20=>{
// 0x10d40b20 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40b24 b.lt #0x10d40b60
if cpu.condition(11){pc=0x10d40b60;continue;}pc=0x10d40b28;continue;
}
0x10d40b28=>{
// 0x10d40b28 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40b2c cmp w20, #6
{let a=((cpu.x[20] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40b30 b.lo #0x10d40b44
if cpu.condition(3){pc=0x10d40b44;continue;}pc=0x10d40b34;continue;
}
0x10d40b34=>{
// 0x10d40b34 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d40b38 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d40b3c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d40b40 bl #0x1128df18
cpu.x[30]=0x10d40b44;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d40b44;continue;
}
0x10d40b44=>{
// 0x10d40b44 add x8, x19, w20, sxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[20] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d40b48 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40b4c ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b50 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40b54 bl #0x110cc5b4
cpu.x[30]=0x10d40b58;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d40b58 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d40b5c bl #0x10d28788
cpu.x[30]=0x10d40b60;super::dispatch(cpu,rt,0x10d28788)?;
pc=0x10d40b60;continue;
}
0x10d40b60=>{
// 0x10d40b60 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40b64 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b68 ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b6c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d40b70;super::dispatch(cpu,rt,target)?;}
pc=0x10d40b70;continue;
}
0x10d40b70=>{
// 0x10d40b70 tbz w0, #0, #0x10d40cac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d40cac;continue;}pc=0x10d40b74;continue;
}
0x10d40b74=>{
// 0x10d40b74 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40b78 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d40b7c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b80 ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d40b88;super::dispatch(cpu,rt,target)?;}
// 0x10d40b88 ldr x8, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40b8c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40b90 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d40b94 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d40b98 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d40b9c ldr x9, [x9, #0x50]
{let address=(cpu.x[9]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d40ba0 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10d40ba4;super::dispatch(cpu,rt,target)?;}
// 0x10d40ba4 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40ba8 tbnz w21, #0x1f, #0x10d40c24
if ((cpu.x[21] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d40c24;continue;}pc=0x10d40bac;continue;
}
0x10d40bac=>{
// 0x10d40bac ldr x8, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40bb0 ldr w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d40bb4 cmp w21, w9
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40bb8 b.ge #0x10d40c24
if cpu.condition(10){pc=0x10d40c24;continue;}pc=0x10d40bbc;continue;
}
0x10d40bbc=>{
// 0x10d40bbc ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40bc0 ldr x8, [x8, w21, uxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[21] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40bc4 cbz x8, #0x10d40c24
if (cpu.x[8]) == 0 {pc=0x10d40c24;continue;}pc=0x10d40bc8;continue;
}
0x10d40bc8=>{
// 0x10d40bc8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d40bcc mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d40bd0 bl #0x10d41a04
cpu.x[30]=0x10d40bd4;super::chunk6::f_10d41a04(cpu,rt,0x10d41a04)?;
// 0x10d40bd4 cmp w21, #0x64
{let a=((cpu.x[21] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40bd8 b.eq #0x10d40c24
if cpu.condition(0){pc=0x10d40c24;continue;}pc=0x10d40bdc;continue;
}
0x10d40bdc=>{
// 0x10d40bdc mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40be0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40be4 b.lt #0x10d40c24
if cpu.condition(11){pc=0x10d40c24;continue;}pc=0x10d40be8;continue;
}
0x10d40be8=>{
// 0x10d40be8 cmp w21, #6
{let a=((cpu.x[21] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40bec mov w21, w21
cpu.x[21]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d40bf0 b.lo #0x10d40c04
if cpu.condition(3){pc=0x10d40c04;continue;}pc=0x10d40bf4;continue;
}
0x10d40bf4=>{
// 0x10d40bf4 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d40bf8 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d40bfc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d40c00 bl #0x1128df18
cpu.x[30]=0x10d40c04;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d40c04;continue;
}
0x10d40c04=>{
// 0x10d40c04 add x8, x19, x21, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[21])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d40c08 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40c0c ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40c10 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40c14 bl #0x110cc5b4
cpu.x[30]=0x10d40c18;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d40c18 cbz x0, #0x10d40c24
if (cpu.x[0]) == 0 {pc=0x10d40c24;continue;}pc=0x10d40c1c;continue;
}
0x10d40c1c=>{
// 0x10d40c1c mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d40c20 bl #0x10d28788
cpu.x[30]=0x10d40c24;super::dispatch(cpu,rt,0x10d28788)?;
pc=0x10d40c24;continue;
}
0x10d40c24=>{
// 0x10d40c24 tbnz w20, #0x1f, #0x10d40cac
if ((cpu.x[20] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d40cac;continue;}pc=0x10d40c28;continue;
}
0x10d40c28=>{
// 0x10d40c28 ldr x8, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40c2c ldr w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d40c30 cmp w20, w9
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40c34 b.ge #0x10d40cac
if cpu.condition(10){pc=0x10d40cac;continue;}pc=0x10d40c38;continue;
}
0x10d40c38=>{
// 0x10d40c38 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40c3c ldr x8, [x8, w20, uxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[20] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40c40 cbz x8, #0x10d40cac
if (cpu.x[8]) == 0 {pc=0x10d40cac;continue;}pc=0x10d40c44;continue;
}
0x10d40c44=>{
// 0x10d40c44 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d40c48 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d40c4c bl #0x10d41a04
cpu.x[30]=0x10d40c50;super::chunk6::f_10d41a04(cpu,rt,0x10d41a04)?;
// 0x10d40c50 cmp w20, #0x64
{let a=((cpu.x[20] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40c54 b.eq #0x10d40cac
if cpu.condition(0){pc=0x10d40cac;continue;}pc=0x10d40c58;continue;
}
0x10d40c58=>{
// 0x10d40c58 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40c5c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40c60 b.lt #0x10d40cac
if cpu.condition(11){pc=0x10d40cac;continue;}pc=0x10d40c64;continue;
}
0x10d40c64=>{
// 0x10d40c64 cmp w20, #6
{let a=((cpu.x[20] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40c68 mov w20, w20
cpu.x[20]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d40c6c b.lo #0x10d40c80
if cpu.condition(3){pc=0x10d40c80;continue;}pc=0x10d40c70;continue;
}
0x10d40c70=>{
// 0x10d40c70 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d40c74 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d40c78 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d40c7c bl #0x1128df18
cpu.x[30]=0x10d40c80;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d40c80;continue;
}
0x10d40c80=>{
// 0x10d40c80 add x8, x19, x20, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[20])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d40c84 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40c88 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40c8c ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40c90 bl #0x110cc5b4
cpu.x[30]=0x10d40c94;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d40c94 cbz x0, #0x10d40cac
if (cpu.x[0]) == 0 {pc=0x10d40cac;continue;}pc=0x10d40c98;continue;
}
0x10d40c98=>{
// 0x10d40c98 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d40c9c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d40ca0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d40ca4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d40ca8 b #0x10d28788
super::dispatch(cpu,rt,0x10d28788)?;return Ok(());
}
0x10d40cac=>{
// 0x10d40cac ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d40cb0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d40cb4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d40cb8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d40aa0")),}}}
#[inline]
pub(super) unsafe fn f_10d40cbc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d40cbc=>{
// 0x10d40cbc sub sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d40cc0 stp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40cc4 stp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40cc8 stp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40ccc stp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40cd0 stp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40cd4 stp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d40cd8 add x29, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d40cdc ldr x8, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40ce0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d40ce4 ldp x27, x26, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[27]=(v0) as u64;cpu.x[26]=(v1) as u64;}
// 0x10d40ce8 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40cec ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40cf0 bl #0x110cc5b4
cpu.x[30]=0x10d40cf4;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
pc=0x10d40cf4;continue;
}
0x10d40cf4=>{
// 0x10d40cf4 ldr x9, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d40cf8 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40cfc stur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d40d00 ldp w1, w2, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40d04 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d40d08 bl #0x110cc5b4
cpu.x[30]=0x10d40d0c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
pc=0x10d40d0c;continue;
}
0x10d40d0c=>{
// 0x10d40d0c ldr x9, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d40d10 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d40d14 ldr w8, [x9, #0xc]
{let address=(cpu.x[9]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d40d18 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40d1c b.lt #0x10d40d38
if cpu.condition(11){pc=0x10d40d38;continue;}pc=0x10d40d20;continue;
}
0x10d40d20=>{
// 0x10d40d20 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d40d24 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40d28 ldr x25, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d40d2c b.ne #0x10d40d44
if cpu.condition(1){pc=0x10d40d44;continue;}pc=0x10d40d30;continue;
}
0x10d40d30=>{
// 0x10d40d30 stur xzr, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d40d34 b #0x10d40d4c
pc=0x10d40d4c;continue;
}
0x10d40d38=>{
// 0x10d40d38 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10d40d3c stur xzr, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d40d40 b #0x10d40d4c
pc=0x10d40d4c;continue;
}
0x10d40d44=>{
// 0x10d40d44 ldr x8, [x9, #8]
{let address=(cpu.x[9]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d40d48 stur x8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d40d4c;continue;
}
0x10d40d4c=>{
// 0x10d40d4c ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d40d50 bl #0x10e16208
cpu.x[30]=0x10d40d54;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d40d54;continue;
}
0x10d40d54=>{
// 0x10d40d54 ldp w1, w2, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d40d58 bl #0x10e3e240
cpu.x[30]=0x10d40d5c;super::chunk13::f_10e3e240(cpu,rt,0x10e3e240)?;
pc=0x10d40d5c;continue;
}
0x10d40d5c=>{
// 0x10d40d5c stur w0, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d40d60 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d40d64 mov x1, x26
cpu.x[1]=(((cpu.x[26]))) as u64;
// 0x10d40d68 bl #0x10fa4038
cpu.x[30]=0x10d40d6c;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d40d6c;continue;
}
0x10d40d6c=>{
// 0x10d40d6c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40d70 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d40d74 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10d40d78 bl #0x10fa4038
cpu.x[30]=0x10d40d7c;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d40d7c;continue;
}
0x10d40d7c=>{
// 0x10d40d7c mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d40d80 cmp w23, w0
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40d84 b.le #0x10d40dc0
if cpu.condition(13){pc=0x10d40dc0;continue;}pc=0x10d40d88;continue;
}
0x10d40d88=>{
// 0x10d40d88 ldr w8, [x19, #0x1bc]
{let address=(cpu.x[19]).wrapping_add(0x1bcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d40d8c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40d90 b.eq #0x10d40e04
if cpu.condition(0){pc=0x10d40e04;continue;}pc=0x10d40d94;continue;
}
0x10d40d94=>{
// 0x10d40d94 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40d98 b.ne #0x10d40e30
if cpu.condition(1){pc=0x10d40e30;continue;}pc=0x10d40d9c;continue;
}
0x10d40d9c=>{
// 0x10d40d9c mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d40da0 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
// 0x10d40da4 subs w10, w9, wzr
{let a=((cpu.x[9] as u32 as u64));let b=((0u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40da8 ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40dac bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40db0 csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40db4 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40db8 b.ne #0x10d40e20
if cpu.condition(1){pc=0x10d40e20;continue;}pc=0x10d40dbc;continue;
}
0x10d40dbc=>{
// 0x10d40dbc b #0x10d40e50
pc=0x10d40e50;continue;
}
0x10d40dc0=>{
// 0x10d40dc0 b.ge #0x10d40dfc
if cpu.condition(10){pc=0x10d40dfc;continue;}pc=0x10d40dc4;continue;
}
0x10d40dc4=>{
// 0x10d40dc4 ldr w8, [x19, #0x1bc]
{let address=(cpu.x[19]).wrapping_add(0x1bcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d40dc8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40dcc b.eq #0x10d40f38
if cpu.condition(0){pc=0x10d40f38;continue;}pc=0x10d40dd0;continue;
}
0x10d40dd0=>{
// 0x10d40dd0 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40dd4 b.ne #0x10d40f64
if cpu.condition(1){pc=0x10d40f64;continue;}pc=0x10d40dd8;continue;
}
0x10d40dd8=>{
// 0x10d40dd8 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d40ddc mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
// 0x10d40de0 negs w10, w9
{let a=((cpu.x[9] as u32 as u64));let result=0u64.wrapping_sub(a);cpu.x[10]=(result) as u32 as u64;cpu.flags(0,a,result,32,true);}
// 0x10d40de4 ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40de8 bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40dec csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40df0 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40df4 b.ne #0x10d40f54
if cpu.condition(1){pc=0x10d40f54;continue;}pc=0x10d40df8;continue;
}
0x10d40df8=>{
// 0x10d40df8 b #0x10d40f84
pc=0x10d40f84;continue;
}
0x10d40dfc=>{
// 0x10d40dfc stur wzr, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d40e00 b #0x10d41070
pc=0x10d41070;continue;
}
0x10d40e04=>{
// 0x10d40e04 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10d40e08 negs w10, w8
{let a=((cpu.x[8] as u32 as u64));let result=0u64.wrapping_sub(a);cpu.x[10]=(result) as u32 as u64;cpu.flags(0,a,result,32,true);}
// 0x10d40e0c ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40e10 bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40e14 csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40e18 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e1c b.eq #0x10d40e50
if cpu.condition(0){pc=0x10d40e50;continue;}pc=0x10d40e20;continue;
}
0x10d40e20=>{
// 0x10d40e20 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e24 b.ne #0x10d40e54
if cpu.condition(1){pc=0x10d40e54;continue;}pc=0x10d40e28;continue;
}
0x10d40e28=>{
// 0x10d40e28 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40e2c b #0x10d40e54
pc=0x10d40e54;continue;
}
0x10d40e30=>{
// 0x10d40e30 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10d40e34 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d40e38 negs w10, wzr
{let a=((0u64));let result=0u64.wrapping_sub(a);cpu.x[10]=(result) as u32 as u64;cpu.flags(0,a,result,32,true);}
// 0x10d40e3c ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40e40 bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40e44 csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40e48 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e4c b.ne #0x10d40e20
if cpu.condition(1){pc=0x10d40e20;continue;}pc=0x10d40e50;continue;
}
0x10d40e50=>{
// 0x10d40e50 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40e54;continue;
}
0x10d40e54=>{
// 0x10d40e54 subs w12, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40e58 ldr w11, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40e5c cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e60 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40e64 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e68 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40e6c cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e70 b.eq #0x10d40e84
if cpu.condition(0){pc=0x10d40e84;continue;}pc=0x10d40e74;continue;
}
0x10d40e74=>{
// 0x10d40e74 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e78 b.ne #0x10d40e88
if cpu.condition(1){pc=0x10d40e88;continue;}pc=0x10d40e7c;continue;
}
0x10d40e7c=>{
// 0x10d40e7c add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40e80 b #0x10d40e88
pc=0x10d40e88;continue;
}
0x10d40e84=>{
// 0x10d40e84 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40e88;continue;
}
0x10d40e88=>{
// 0x10d40e88 subs w12, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40e8c ldr w11, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40e90 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e94 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40e98 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40e9c csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40ea0 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ea4 b.eq #0x10d40eb8
if cpu.condition(0){pc=0x10d40eb8;continue;}pc=0x10d40ea8;continue;
}
0x10d40ea8=>{
// 0x10d40ea8 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40eac b.ne #0x10d40ebc
if cpu.condition(1){pc=0x10d40ebc;continue;}pc=0x10d40eb0;continue;
}
0x10d40eb0=>{
// 0x10d40eb0 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40eb4 b #0x10d40ebc
pc=0x10d40ebc;continue;
}
0x10d40eb8=>{
// 0x10d40eb8 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40ebc;continue;
}
0x10d40ebc=>{
// 0x10d40ebc subs w12, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40ec0 ldr w11, [x19, #0x1cc]
{let address=(cpu.x[19]).wrapping_add(0x1ccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40ec4 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ec8 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40ecc cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ed0 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40ed4 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ed8 b.eq #0x10d40eec
if cpu.condition(0){pc=0x10d40eec;continue;}pc=0x10d40edc;continue;
}
0x10d40edc=>{
// 0x10d40edc cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ee0 b.ne #0x10d40ef0
if cpu.condition(1){pc=0x10d40ef0;continue;}pc=0x10d40ee4;continue;
}
0x10d40ee4=>{
// 0x10d40ee4 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40ee8 b #0x10d40ef0
pc=0x10d40ef0;continue;
}
0x10d40eec=>{
// 0x10d40eec add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40ef0;continue;
}
0x10d40ef0=>{
// 0x10d40ef0 subs w12, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40ef4 ldr w11, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40ef8 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40efc csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40f00 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f04 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40f08 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f0c b.eq #0x10d40f20
if cpu.condition(0){pc=0x10d40f20;continue;}pc=0x10d40f10;continue;
}
0x10d40f10=>{
// 0x10d40f10 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f14 b.ne #0x10d40f24
if cpu.condition(1){pc=0x10d40f24;continue;}pc=0x10d40f18;continue;
}
0x10d40f18=>{
// 0x10d40f18 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40f1c b #0x10d40f24
pc=0x10d40f24;continue;
}
0x10d40f20=>{
// 0x10d40f20 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40f24;continue;
}
0x10d40f24=>{
// 0x10d40f24 subs w11, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40f28 cmp w11, w10
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f2c csel w11, w11, w10, gt
cpu.x[11]=(if cpu.condition(12){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40f30 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f34 b #0x10d41068
pc=0x10d41068;continue;
}
0x10d40f38=>{
// 0x10d40f38 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10d40f3c subs w10, w8, wzr
{let a=((cpu.x[8] as u32 as u64));let b=((0u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40f40 ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40f44 bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40f48 csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40f4c cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f50 b.eq #0x10d40f84
if cpu.condition(0){pc=0x10d40f84;continue;}pc=0x10d40f54;continue;
}
0x10d40f54=>{
// 0x10d40f54 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f58 b.ne #0x10d40f88
if cpu.condition(1){pc=0x10d40f88;continue;}pc=0x10d40f5c;continue;
}
0x10d40f5c=>{
// 0x10d40f5c add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40f60 b #0x10d40f88
pc=0x10d40f88;continue;
}
0x10d40f64=>{
// 0x10d40f64 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10d40f68 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d40f6c negs w10, wzr
{let a=((0u64));let result=0u64.wrapping_sub(a);cpu.x[10]=(result) as u32 as u64;cpu.flags(0,a,result,32,true);}
// 0x10d40f70 ldr w11, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40f74 bic w10, w10, w10, asr #31
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[10]=(result) as u32 as u64;}
// 0x10d40f78 csel w10, w10, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d40f7c cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f80 b.ne #0x10d40f54
if cpu.condition(1){pc=0x10d40f54;continue;}pc=0x10d40f84;continue;
}
0x10d40f84=>{
// 0x10d40f84 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40f88;continue;
}
0x10d40f88=>{
// 0x10d40f88 subs w12, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40f8c ldr w11, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40f90 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f94 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40f98 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40f9c csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40fa0 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fa4 b.eq #0x10d40fb8
if cpu.condition(0){pc=0x10d40fb8;continue;}pc=0x10d40fa8;continue;
}
0x10d40fa8=>{
// 0x10d40fa8 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fac b.ne #0x10d40fbc
if cpu.condition(1){pc=0x10d40fbc;continue;}pc=0x10d40fb0;continue;
}
0x10d40fb0=>{
// 0x10d40fb0 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40fb4 b #0x10d40fbc
pc=0x10d40fbc;continue;
}
0x10d40fb8=>{
// 0x10d40fb8 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40fbc;continue;
}
0x10d40fbc=>{
// 0x10d40fbc subs w12, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40fc0 ldr w11, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40fc4 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fc8 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40fcc cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fd0 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d40fd4 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fd8 b.eq #0x10d40fec
if cpu.condition(0){pc=0x10d40fec;continue;}pc=0x10d40fdc;continue;
}
0x10d40fdc=>{
// 0x10d40fdc cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40fe0 b.ne #0x10d40ff0
if cpu.condition(1){pc=0x10d40ff0;continue;}pc=0x10d40fe4;continue;
}
0x10d40fe4=>{
// 0x10d40fe4 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d40fe8 b #0x10d40ff0
pc=0x10d40ff0;continue;
}
0x10d40fec=>{
// 0x10d40fec add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d40ff0;continue;
}
0x10d40ff0=>{
// 0x10d40ff0 subs w12, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d40ff4 ldr w11, [x19, #0x1cc]
{let address=(cpu.x[19]).wrapping_add(0x1ccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d40ff8 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d40ffc csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
pc=0x10d41000;continue;
}
0x10d41000=>{
// 0x10d41000 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41004 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d41008 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4100c b.eq #0x10d41020
if cpu.condition(0){pc=0x10d41020;continue;}pc=0x10d41010;continue;
}
0x10d41010=>{
// 0x10d41010 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41014 b.ne #0x10d41024
if cpu.condition(1){pc=0x10d41024;continue;}pc=0x10d41018;continue;
}
0x10d41018=>{
// 0x10d41018 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d4101c b #0x10d41024
pc=0x10d41024;continue;
}
0x10d41020=>{
// 0x10d41020 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d41024;continue;
}
0x10d41024=>{
// 0x10d41024 subs w12, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d41028 ldr w11, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d4102c cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41030 csel w12, w12, w10, gt
cpu.x[12]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d41034 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41038 csel w10, w12, w10, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d4103c cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41040 b.eq #0x10d41054
if cpu.condition(0){pc=0x10d41054;continue;}pc=0x10d41044;continue;
}
0x10d41044=>{
// 0x10d41044 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41048 b.ne #0x10d41058
if cpu.condition(1){pc=0x10d41058;continue;}pc=0x10d4104c;continue;
}
0x10d4104c=>{
// 0x10d4104c add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d41050 b #0x10d41058
pc=0x10d41058;continue;
}
0x10d41054=>{
// 0x10d41054 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x10d41058;continue;
}
0x10d41058=>{
// 0x10d41058 subs w11, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d4105c cmp w11, w10
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41060 csel w11, w11, w10, gt
cpu.x[11]=(if cpu.condition(12){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d41064 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10d41068;continue;
}
0x10d41068=>{
// 0x10d41068 csel w8, w11, w10, hi
cpu.x[8]=(if cpu.condition(8){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d4106c stur w8, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d41070;continue;
}
0x10d41070=>{
// 0x10d41070 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41074 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d41078 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4107c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41080;super::dispatch(cpu,rt,target)?;}
pc=0x10d41080;continue;
}
0x10d41080=>{
// 0x10d41080 cmp w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41084 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10d41088 b.le #0x10d410c4
if cpu.condition(13){pc=0x10d410c4;continue;}pc=0x10d4108c;continue;
}
0x10d4108c=>{
// 0x10d4108c cbz x0, #0x10d410c4
if (cpu.x[0]) == 0 {pc=0x10d410c4;continue;}pc=0x10d41090;continue;
}
0x10d41090=>{
// 0x10d41090 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41094 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d41098 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4109c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d410a0;super::dispatch(cpu,rt,target)?;}
pc=0x10d410a0;continue;
}
0x10d410a0=>{
// 0x10d410a0 bl #0x10fca234
cpu.x[30]=0x10d410a4;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d410a4;continue;
}
0x10d410a4=>{
// 0x10d410a4 ldr x9, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d410a8 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10d410ac mul w20, w0, w8
cpu.x[20]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10d410b0 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d410b4 ldr x8, [x9, #0x120]
{let address=(cpu.x[9]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d410b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d410bc;super::dispatch(cpu,rt,target)?;}
pc=0x10d410bc;continue;
}
0x10d410bc=>{
// 0x10d410bc bl #0x10fcbf74
cpu.x[30]=0x10d410c0;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d410c0;continue;
}
0x10d410c0=>{
// 0x10d410c0 sdiv w20, w20, w0
{let a=((cpu.x[20] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[20]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
pc=0x10d410c4;continue;
}
0x10d410c4=>{
// 0x10d410c4 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d410c8 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d410cc ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d410d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d410d4;super::dispatch(cpu,rt,target)?;}
pc=0x10d410d4;continue;
}
0x10d410d4=>{
// 0x10d410d4 cmp w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d410d8 b.ge #0x10d41114
if cpu.condition(10){pc=0x10d41114;continue;}pc=0x10d410dc;continue;
}
0x10d410dc=>{
// 0x10d410dc cbz x0, #0x10d41114
if (cpu.x[0]) == 0 {pc=0x10d41114;continue;}pc=0x10d410e0;continue;
}
0x10d410e0=>{
// 0x10d410e0 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d410e4 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d410e8 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d410ec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d410f0;super::dispatch(cpu,rt,target)?;}
pc=0x10d410f0;continue;
}
0x10d410f0=>{
// 0x10d410f0 bl #0x10fca234
cpu.x[30]=0x10d410f4;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d410f4;continue;
}
0x10d410f4=>{
// 0x10d410f4 ldr x9, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d410f8 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10d410fc mul w20, w0, w8
cpu.x[20]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10d41100 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41104 ldr x8, [x9, #0x120]
{let address=(cpu.x[9]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41108 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d4110c;super::dispatch(cpu,rt,target)?;}
pc=0x10d4110c;continue;
}
0x10d4110c=>{
// 0x10d4110c bl #0x10fcbf74
cpu.x[30]=0x10d41110;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d41110;continue;
}
0x10d41110=>{
// 0x10d41110 sdiv w20, w20, w0
{let a=((cpu.x[20] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[20]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
pc=0x10d41114;continue;
}
0x10d41114=>{
// 0x10d41114 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41118 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d4111c str w20, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d41120 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41124 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41128 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d4112c;super::dispatch(cpu,rt,target)?;}
pc=0x10d4112c;continue;
}
0x10d4112c=>{
// 0x10d4112c mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d41130 cmp w0, #2
{let a=((cpu.x[0] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41134 b.lo #0x10d41148
if cpu.condition(3){pc=0x10d41148;continue;}pc=0x10d41138;continue;
}
0x10d41138=>{
// 0x10d41138 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d4113c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d41140 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d41144 bl #0x1128df18
cpu.x[30]=0x10d41148;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d41148;continue;
}
0x10d41148=>{
// 0x10d41148 add x8, x19, w26, sxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[26] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d4114c mov w28, #0x64
cpu.x[28]=(((0x64u64))) as u32 as u64;
// 0x10d41150 str x22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d41154 ldr x20, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d41158 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d4115c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41160 b.lt #0x10d41270
if cpu.condition(11){pc=0x10d41270;continue;}pc=0x10d41164;continue;
}
0x10d41164=>{
// 0x10d41164 cmp w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41168 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10d4116c b.le #0x10d41284
if cpu.condition(13){pc=0x10d41284;continue;}pc=0x10d41170;continue;
}
0x10d41170=>{
// 0x10d41170 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41174 ldr x26, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d41178 cbz x26, #0x10d411d8
if (cpu.x[26]) == 0 {pc=0x10d411d8;continue;}pc=0x10d4117c;continue;
}
0x10d4117c=>{
// 0x10d4117c ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41180 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41184 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41188 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d4118c;super::dispatch(cpu,rt,target)?;}
pc=0x10d4118c;continue;
}
0x10d4118c=>{
// 0x10d4118c cbz x0, #0x10d411d8
if (cpu.x[0]) == 0 {pc=0x10d411d8;continue;}pc=0x10d41190;continue;
}
0x10d41190=>{
// 0x10d41190 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41194 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41198 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4119c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d411a0;super::dispatch(cpu,rt,target)?;}
pc=0x10d411a0;continue;
}
0x10d411a0=>{
// 0x10d411a0 bl #0x10fca234
cpu.x[30]=0x10d411a4;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d411a4;continue;
}
0x10d411a4=>{
// 0x10d411a4 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d411a8 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d411ac mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d411b0 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d411b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d411b8;super::dispatch(cpu,rt,target)?;}
pc=0x10d411b8;continue;
}
0x10d411b8=>{
// 0x10d411b8 bl #0x10fcbf74
cpu.x[30]=0x10d411bc;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d411bc;continue;
}
0x10d411bc=>{
// 0x10d411bc mul w8, w27, w21
cpu.x[8]=((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[21] as u32 as u64)))) as u32 as u64;
// 0x10d411c0 sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d411c4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d411c8 ccmp w27, #0, #4, eq
if cpu.condition(0){{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d411cc csinc w8, w8, wzr, le
cpu.x[8]=(if cpu.condition(13){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10d411d0 cmp w27, #0
{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d411d4 csel w21, w21, w8, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
pc=0x10d411d8;continue;
}
0x10d411d8=>{
// 0x10d411d8 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d411dc cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d411e0 b.lt #0x10d41284
if cpu.condition(11){pc=0x10d41284;continue;}pc=0x10d411e4;continue;
}
0x10d411e4=>{
// 0x10d411e4 str x25, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d411e8 mov w25, #1
cpu.x[25]=(((0x1u64))) as u32 as u64;
// 0x10d411ec mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
// 0x10d411f0 b #0x10d41204
pc=0x10d41204;continue;
}
0x10d411f4=>{
// 0x10d411f4 ldrsw x8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d411f8 add x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d411fc cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d41200 b.ge #0x10d41278
if cpu.condition(10){pc=0x10d41278;continue;}pc=0x10d41204;continue;
}
0x10d41204=>{
// 0x10d41204 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41208 ldr x26, [x8, x25, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[25])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d4120c cbz x26, #0x10d411f4
if (cpu.x[26]) == 0 {pc=0x10d411f4;continue;}pc=0x10d41210;continue;
}
0x10d41210=>{
// 0x10d41210 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41214 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41218 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4121c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41220;super::dispatch(cpu,rt,target)?;}
pc=0x10d41220;continue;
}
0x10d41220=>{
// 0x10d41220 cbz x0, #0x10d411f4
if (cpu.x[0]) == 0 {pc=0x10d411f4;continue;}pc=0x10d41224;continue;
}
0x10d41224=>{
// 0x10d41224 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41228 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d4122c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41230 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41234;super::dispatch(cpu,rt,target)?;}
pc=0x10d41234;continue;
}
0x10d41234=>{
// 0x10d41234 bl #0x10fca234
cpu.x[30]=0x10d41238;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d41238;continue;
}
0x10d41238=>{
// 0x10d41238 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4123c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d41240 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41244 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41248 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d4124c;super::dispatch(cpu,rt,target)?;}
pc=0x10d4124c;continue;
}
0x10d4124c=>{
// 0x10d4124c bl #0x10fcbf74
cpu.x[30]=0x10d41250;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d41250;continue;
}
0x10d41250=>{
// 0x10d41250 mul w8, w27, w28
cpu.x[8]=((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64)))) as u32 as u64;
// 0x10d41254 sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d41258 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4125c ccmp w27, #0, #4, eq
if cpu.condition(0){{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d41260 csinc w8, w8, wzr, le
cpu.x[8]=(if cpu.condition(13){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10d41264 cmp w27, #0
{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41268 csel w22, w28, w8, eq
cpu.x[22]=(if cpu.condition(0){((cpu.x[28] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d4126c b #0x10d411f4
pc=0x10d411f4;continue;
}
0x10d41270=>{
// 0x10d41270 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10d41274 b #0x10d41284
pc=0x10d41284;continue;
}
0x10d41278=>{
// 0x10d41278 mov w28, w22
cpu.x[28]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d4127c ldr x22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d41280 ldr x25, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
pc=0x10d41284;continue;
}
0x10d41284=>{
// 0x10d41284 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41288 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d4128c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41290 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41294 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41298;super::dispatch(cpu,rt,target)?;}
pc=0x10d41298;continue;
}
0x10d41298=>{
// 0x10d41298 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d4129c cmp w0, #2
{let a=((cpu.x[0] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d412a0 b.lo #0x10d412b4
if cpu.condition(3){pc=0x10d412b4;continue;}pc=0x10d412a4;continue;
}
0x10d412a4=>{
// 0x10d412a4 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d412a8 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d412ac mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d412b0 bl #0x1128df18
cpu.x[30]=0x10d412b4;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d412b4;continue;
}
0x10d412b4=>{
// 0x10d412b4 add x8, x19, w26, sxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[26] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d412b8 ldr x20, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d412bc ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d412c0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d412c4 ccmp w23, w24, #0, ge
if cpu.condition(10){{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10d412c8 b.ge #0x10d413d8
if cpu.condition(10){pc=0x10d413d8;continue;}pc=0x10d412cc;continue;
}
0x10d412cc=>{
// 0x10d412cc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d412d0 ldr x26, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d412d4 cbz x26, #0x10d41338
if (cpu.x[26]) == 0 {pc=0x10d41338;continue;}pc=0x10d412d8;continue;
}
0x10d412d8=>{
// 0x10d412d8 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d412dc mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d412e0 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d412e4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d412e8;super::dispatch(cpu,rt,target)?;}
pc=0x10d412e8;continue;
}
0x10d412e8=>{
// 0x10d412e8 cbz x0, #0x10d41338
if (cpu.x[0]) == 0 {pc=0x10d41338;continue;}pc=0x10d412ec;continue;
}
0x10d412ec=>{
// 0x10d412ec ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d412f0 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d412f4 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d412f8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d412fc;super::dispatch(cpu,rt,target)?;}
pc=0x10d412fc;continue;
}
0x10d412fc=>{
// 0x10d412fc bl #0x10fca234
cpu.x[30]=0x10d41300;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d41300;continue;
}
0x10d41300=>{
// 0x10d41300 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41304 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d41308 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d4130c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41310 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41314;super::dispatch(cpu,rt,target)?;}
pc=0x10d41314;continue;
}
0x10d41314=>{
// 0x10d41314 bl #0x10fcbf74
cpu.x[30]=0x10d41318;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d41318;continue;
}
0x10d41318=>{
// 0x10d41318 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10d4131c mul w9, w27, w8
cpu.x[9]=((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10d41320 sdiv w9, w9, w0
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d41324 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41328 ccmp w27, #0, #4, eq
if cpu.condition(0){{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d4132c csinc w9, w9, wzr, le
cpu.x[9]=(if cpu.condition(13){((cpu.x[9] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10d41330 cmp w27, #0
{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41334 csel w21, w8, w9, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
pc=0x10d41338;continue;
}
0x10d41338=>{
// 0x10d41338 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d4133c cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41340 b.lt #0x10d413d8
if cpu.condition(11){pc=0x10d413d8;continue;}pc=0x10d41344;continue;
}
0x10d41344=>{
// 0x10d41344 str x25, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d41348 mov w25, #1
cpu.x[25]=(((0x1u64))) as u32 as u64;
// 0x10d4134c mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
// 0x10d41350 b #0x10d41364
pc=0x10d41364;continue;
}
0x10d41354=>{
// 0x10d41354 ldrsw x8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d41358 add x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d4135c cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d41360 b.ge #0x10d413d0
if cpu.condition(10){pc=0x10d413d0;continue;}pc=0x10d41364;continue;
}
0x10d41364=>{
// 0x10d41364 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41368 ldr x26, [x8, x25, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[25])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d4136c cbz x26, #0x10d41354
if (cpu.x[26]) == 0 {pc=0x10d41354;continue;}pc=0x10d41370;continue;
}
0x10d41370=>{
// 0x10d41370 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41374 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d41378 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4137c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41380;super::dispatch(cpu,rt,target)?;}
pc=0x10d41380;continue;
}
0x10d41380=>{
// 0x10d41380 cbz x0, #0x10d41354
if (cpu.x[0]) == 0 {pc=0x10d41354;continue;}pc=0x10d41384;continue;
}
0x10d41384=>{
// 0x10d41384 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41388 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d4138c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41390 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d41394;super::dispatch(cpu,rt,target)?;}
pc=0x10d41394;continue;
}
0x10d41394=>{
// 0x10d41394 bl #0x10fca234
cpu.x[30]=0x10d41398;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10d41398;continue;
}
0x10d41398=>{
// 0x10d41398 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4139c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d413a0 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d413a4 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d413a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d413ac;super::dispatch(cpu,rt,target)?;}
pc=0x10d413ac;continue;
}
0x10d413ac=>{
// 0x10d413ac bl #0x10fcbf74
cpu.x[30]=0x10d413b0;super::chunk8::f_10fcbf74(cpu,rt,0x10fcbf74)?;
pc=0x10d413b0;continue;
}
0x10d413b0=>{
// 0x10d413b0 mul w8, w27, w22
cpu.x[8]=((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64)))) as u32 as u64;
// 0x10d413b4 sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d413b8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d413bc ccmp w27, #0, #4, eq
if cpu.condition(0){{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d413c0 csinc w8, w8, wzr, le
cpu.x[8]=(if cpu.condition(13){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10d413c4 cmp w27, #0
{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d413c8 csel w28, w22, w8, eq
cpu.x[28]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d413cc b #0x10d41354
pc=0x10d41354;continue;
}
0x10d413d0=>{
// 0x10d413d0 ldr x22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d413d4 ldr x25, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
pc=0x10d413d8;continue;
}
0x10d413d8=>{
// 0x10d413d8 ldur x8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d413dc ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d413e0 ldr w26, [x25, #0xec]
{let address=(cpu.x[25]).wrapping_add(0xecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10d413e4 ldr w27, [x8, #0xec]
{let address=(cpu.x[8]).wrapping_add(0xecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10d413e8 bl #0x10d25c3c
cpu.x[30]=0x10d413ec;super::chunk13::f_10d25c3c(cpu,rt,0x10d25c3c)?;
pc=0x10d413ec;continue;
}
0x10d413ec=>{
// 0x10d413ec sdiv w20, w0, w26
{let a=((cpu.x[0] as u32 as u64)) as i32;let b=((cpu.x[26] as u32 as u64)) as i32;cpu.x[20]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d413f0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d413f4 bl #0x10d25c3c
cpu.x[30]=0x10d413f8;super::chunk13::f_10d25c3c(cpu,rt,0x10d25c3c)?;
pc=0x10d413f8;continue;
}
0x10d413f8=>{
// 0x10d413f8 sdiv w8, w0, w27
{let a=((cpu.x[0] as u32 as u64)) as i32;let b=((cpu.x[27] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10d413fc cmp w20, #0x12c
{let a=((cpu.x[20] as u32 as u64));let b=((0x12cu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10d41400;continue;
}
0x10d41400=>{
// 0x10d41400 mov w9, #-0xbb8
cpu.x[9]=(((0xfffffffffffff448u64))) as u32 as u64;
// 0x10d41404 b.le #0x10d41438
if cpu.condition(13){pc=0x10d41438;continue;}pc=0x10d41408;continue;
}
0x10d41408=>{
// 0x10d41408 mov w10, #0xa
cpu.x[10]=(((0xau64))) as u32 as u64;
// 0x10d4140c mov w11, #0x851f
cpu.x[11]=(((0x851fu64))) as u32 as u64;
// 0x10d41410 madd w10, w20, w10, w9
cpu.x[10]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[20] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64))))) as u32 as u64;
// 0x10d41414 movk w11, #0x51eb, lsl #16
cpu.x[11]=((cpu.x[11] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10d41418 smull x10, w10, w11
cpu.x[10]=(((((cpu.x[10] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[11] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d4141c lsr x11, x10, #0x3f
cpu.x[11]=(((((cpu.x[10]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d41420 asr x10, x10, #0x25
cpu.x[10]=(((((cpu.x[10]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10d41424 add w10, w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d41428 add w20, w10, #0x50
{let a=((cpu.x[10] as u32 as u64));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d4142c cmp w8, #0x12d
{let a=((cpu.x[8] as u32 as u64));let b=((0x12du64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41430 b.ge #0x10d41444
if cpu.condition(10){pc=0x10d41444;continue;}pc=0x10d41434;continue;
}
0x10d41434=>{
// 0x10d41434 b #0x10d41468
pc=0x10d41468;continue;
}
0x10d41438=>{
// 0x10d41438 mov w20, #0x50
cpu.x[20]=(((0x50u64))) as u32 as u64;
// 0x10d4143c cmp w8, #0x12d
{let a=((cpu.x[8] as u32 as u64));let b=((0x12du64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41440 b.lt #0x10d41468
if cpu.condition(11){pc=0x10d41468;continue;}pc=0x10d41444;continue;
}
0x10d41444=>{
// 0x10d41444 mov w10, #0xa
cpu.x[10]=(((0xau64))) as u32 as u64;
// 0x10d41448 madd w8, w8, w10, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64))))) as u32 as u64;
// 0x10d4144c mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10d41450 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10d41454 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d41458 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d4145c asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10d41460 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d41464 add w20, w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
pc=0x10d41468;continue;
}
0x10d41468=>{
// 0x10d41468 cmp w21, w28
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4146c csel w8, w21, w28, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[28] as u32 as u64))}) as u32 as u64;
// 0x10d41470 subs w9, w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d41474 b.ne #0x10d41490
if cpu.condition(1){pc=0x10d41490;continue;}pc=0x10d41478;continue;
}
0x10d41478=>{
// 0x10d41478 add w8, w23, w23, lsl #2
{let a=((cpu.x[23] as u32 as u64));let b=(((cpu.x[23] as u32 as u64)) as u32).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d4147c cmp w23, #3
{let a=((cpu.x[23] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41480 mov w9, #0x3c
cpu.x[9]=(((0x3cu64))) as u32 as u64;
// 0x10d41484 lsl w8, w8, #1
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d41488 csel w8, w9, w8, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d4148c b #0x10d4151c
pc=0x10d4151c;continue;
}
0x10d41490=>{
// 0x10d41490 ldr w10, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d41494 cmp w10, #0x1e
{let a=((cpu.x[10] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41498 b.gt #0x10d414a8
if cpu.condition(12){pc=0x10d414a8;continue;}pc=0x10d4149c;continue;
}
0x10d4149c=>{
// 0x10d4149c mov w8, #0x65
cpu.x[8]=(((0x65u64))) as u32 as u64;
// 0x10d414a0 sub w8, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d414a4 b #0x10d4151c
pc=0x10d4151c;continue;
}
0x10d414a8=>{
// 0x10d414a8 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d414ac add w10, w28, w21
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d414b0 mov w11, #0x5c
cpu.x[11]=(((0x5cu64))) as u32 as u64;
// 0x10d414b4 cneg w9, w9, mi
cpu.x[9]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[9] as u32 as u64)))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d414b8 cmp w28, #0x1e
{let a=((cpu.x[28] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d414bc mov w12, #0x1e
cpu.x[12]=(((0x1eu64))) as u32 as u64;
// 0x10d414c0 sub w10, w11, w10
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d414c4 ccmp w21, #0x1e, #0, lt
if cpu.condition(11){{let a=((cpu.x[21] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10d414c8 add w11, w24, w23
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10d414cc csel w10, w10, w12, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[12] as u32 as u64))}) as u32 as u64;
// 0x10d414d0 cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d414d4 mov w13, #0x5b
cpu.x[13]=(((0x5bu64))) as u32 as u64;
// 0x10d414d8 add w12, w10, #0xa
{let a=((cpu.x[10] as u32 as u64));let b=((0xau64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10d414dc sub w13, w13, w8
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10d414e0 csel w10, w12, w10, gt
cpu.x[10]=(if cpu.condition(12){((cpu.x[12] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10d414e4 cmp w28, #0x1e
{let a=((cpu.x[28] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d414e8 mov w12, #0xa
cpu.x[12]=(((0xau64))) as u32 as u64;
// 0x10d414ec ccmp w21, #0x1e, #0, lt
if cpu.condition(11){{let a=((cpu.x[21] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10d414f0 csel w12, w12, wzr, lt
cpu.x[12]=(if cpu.condition(11){((cpu.x[12] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10d414f4 cmp w8, #0x1e
{let a=((cpu.x[8] as u32 as u64));let b=((0x1eu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d414f8 mov w8, #0x3c
cpu.x[8]=(((0x3cu64))) as u32 as u64;
// 0x10d414fc csel w8, w13, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[13] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d41500 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41504 add w13, w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10d41508 csel w8, w13, w8, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[13] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d4150c cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41510 csel w8, w12, w8, ne
cpu.x[8]=(if cpu.condition(1){((cpu.x[12] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d41514 cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41518 csel w8, w10, w8, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
pc=0x10d4151c;continue;
}
0x10d4151c=>{
// 0x10d4151c ldur w21, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10d41520 cmp w8, #0x50
{let a=((cpu.x[8] as u32 as u64));let b=((0x50u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41524 mov w9, #0x50
cpu.x[9]=(((0x50u64))) as u32 as u64;
// 0x10d41528 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d4152c csel w9, w8, w9, gt
cpu.x[9]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d41530 cmp w21, #0
{let a=((cpu.x[21] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41534 csel w22, w9, w8, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d41538 bl #0x10e16208
cpu.x[30]=0x10d4153c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d4153c;continue;
}
0x10d4153c=>{
// 0x10d4153c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d41540 bl #0x10e3e218
cpu.x[30]=0x10d41544;super::chunk2::f_10e3e218(cpu,rt,0x10e3e218)?;
pc=0x10d41544;continue;
}
0x10d41544=>{
// 0x10d41544 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41548 cmp w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4154c mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10d41550 csel w25, w21, w0, eq
cpu.x[25]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x10d41554 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d41558 bl #0x10e16208
cpu.x[30]=0x10d4155c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d4155c;continue;
}
0x10d4155c=>{
// 0x10d4155c mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d41560 mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10d41564 mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10d41568 mov w4, w27
cpu.x[4]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10d4156c bl #0x10e3e598
cpu.x[30]=0x10d41570;super::chunk14::f_10e3e598(cpu,rt,0x10e3e598)?;
pc=0x10d41570;continue;
}
0x10d41570=>{
// 0x10d41570 ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d41574 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10d41578 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10d4157c mul w8, w22, w8
cpu.x[8]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10d41580 mul w8, w8, w25
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[25] as u32 as u64)))) as u32 as u64;
// 0x10d41584 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d41588 lsr x10, x8, #0x20
cpu.x[10]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x20u64))) & 63) as u32) as u64) as u64;
// 0x10d4158c lsr x8, x8, #0x3f
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d41590 add w8, w8, w10, asr #5
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(5) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d41594 mov w10, #0x8bad
cpu.x[10]=(((0x8badu64))) as u32 as u64;
// 0x10d41598 movk w10, #0x68db, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x68db0000u64) as u32 as u64;
// 0x10d4159c mul w8, w8, w0
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10d415a0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d415a4 smull x8, w8, w10
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d415a8 lsr x10, x8, #0x20
cpu.x[10]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x20u64))) & 63) as u32) as u64) as u64;
// 0x10d415ac lsr x8, x8, #0x3f
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d415b0 add w8, w8, w10, asr #12
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as i32).wrapping_shr(12) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d415b4 mul w8, w8, w20
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[20] as u32 as u64)))) as u32 as u64;
// 0x10d415b8 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d415bc lsr x9, x8, #0x20
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x20u64))) & 63) as u32) as u64) as u64;
// 0x10d415c0 lsr x8, x8, #0x3f
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d415c4 add w20, w8, w9, asr #5
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as i32).wrapping_shr(5) as u64;let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d415c8 cbz x0, #0x10d415f8
if (cpu.x[0]) == 0 {pc=0x10d415f8;continue;}pc=0x10d415cc;continue;
}
0x10d415cc=>{
// 0x10d415cc bl #0x110cb698
cpu.x[30]=0x10d415d0;super::chunk5::f_110cb698(cpu,rt,0x110cb698)?;
pc=0x10d415d0;continue;
}
0x10d415d0=>{
// 0x10d415d0 mov w8, #0xc31
cpu.x[8]=(((0xc31u64))) as u32 as u64;
// 0x10d415d4 movk w8, #0x30c3, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x30c30000u64) as u32 as u64;
// 0x10d415d8 smull x8, w0, w8
cpu.x[8]=(((((cpu.x[0] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10d415dc lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10d415e0 asr x8, x8, #0x22
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x22u64))) & 63) as u32) as u64) as u64;
// 0x10d415e4 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d415e8 mov w9, #0x15
cpu.x[9]=(((0x15u64))) as u32 as u64;
// 0x10d415ec msub w8, w8, w9, w0
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_sub((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64))))) as u32 as u64;
// 0x10d415f0 add w8, w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d415f4 sub w20, w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
pc=0x10d415f8;continue;
}
0x10d415f8=>{
// 0x10d415f8 cmp w20, #0x64
{let a=((cpu.x[20] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d415fc ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41600 csel w8, w20, w21, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[20] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
// 0x10d41604 bic w20, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[20]=(result) as u32 as u64;}
// 0x10d41608 bl #0x10e16208
cpu.x[30]=0x10d4160c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d4160c;continue;
}
0x10d4160c=>{
// 0x10d4160c ldrb w8, [x0, #0x167]
{let address=(cpu.x[0]).wrapping_add(0x167u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d41610 cbz w8, #0x10d416b0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d416b0;continue;}pc=0x10d41614;continue;
}
0x10d41614=>{
// 0x10d41614 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41618 subs w8, w26, w27
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d4161c cneg w21, w8, mi
cpu.x[21]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d41620 bl #0x10e16208
cpu.x[30]=0x10d41624;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d41624;continue;
}
0x10d41624=>{
// 0x10d41624 ldr x8, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41628 ldr w9, [x0, #0x168]
{let address=(cpu.x[0]).wrapping_add(0x168u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d4162c cmp w21, w9
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41630 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d41634 csel w20, wzr, w20, gt
cpu.x[20]=(if cpu.condition(12){((0u64))}else{((cpu.x[20] as u32 as u64))}) as u32 as u64;
// 0x10d41638 bl #0x10dc7018
cpu.x[30]=0x10d4163c;super::chunk12::f_10dc7018(cpu,rt,0x10dc7018)?;
pc=0x10d4163c;continue;
}
0x10d4163c=>{
// 0x10d4163c cmp w0, #0x8fb
{let a=((cpu.x[0] as u32 as u64));let b=((0x8fbu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41640 b.gt #0x10d416b0
if cpu.condition(12){pc=0x10d416b0;continue;}pc=0x10d41644;continue;
}
0x10d41644=>{
// 0x10d41644 ldr x0, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41648 bl #0x10dc7018
cpu.x[30]=0x10d4164c;super::chunk12::f_10dc7018(cpu,rt,0x10dc7018)?;
pc=0x10d4164c;continue;
}
0x10d4164c=>{
// 0x10d4164c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d41650 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41654 bl #0x10d29ffc
cpu.x[30]=0x10d41658;super::chunk6::f_10d29ffc(cpu,rt,0x10d29ffc)?;
pc=0x10d41658;continue;
}
0x10d41658=>{
// 0x10d41658 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4165c subs w9, w22, w0
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d41660 cneg w21, w9, mi
cpu.x[21]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[9] as u32 as u64)))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d41664 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d41668 bl #0x10e16208
cpu.x[30]=0x10d4166c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d4166c;continue;
}
0x10d4166c=>{
// 0x10d4166c ldr w8, [x0, #0x16c]
{let address=(cpu.x[0]).wrapping_add(0x16cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d41670 cmp w21, w8
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d41674 b.gt #0x10d416ac
if cpu.condition(12){pc=0x10d416ac;continue;}pc=0x10d41678;continue;
}
0x10d41678=>{
// 0x10d41678 ldr x0, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d4167c bl #0x10dc7018
cpu.x[30]=0x10d41680;super::chunk12::f_10dc7018(cpu,rt,0x10dc7018)?;
pc=0x10d41680;continue;
}
0x10d41680=>{
// 0x10d41680 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d41684 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d41688 bl #0x10d29ffc
cpu.x[30]=0x10d4168c;super::chunk6::f_10d29ffc(cpu,rt,0x10d29ffc)?;
pc=0x10d4168c;continue;
}
0x10d4168c=>{
// 0x10d4168c ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d41690 subs w9, w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d41694 cneg w21, w9, mi
cpu.x[21]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[9] as u32 as u64)))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d41698 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d4169c bl #0x10e16208
cpu.x[30]=0x10d416a0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d416a0;continue;
}
0x10d416a0=>{
// 0x10d416a0 ldr w8, [x0, #0x16c]
{let address=(cpu.x[0]).wrapping_add(0x16cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d416a4 cmp w21, w8
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d416a8 b.le #0x10d416b0
if cpu.condition(13){pc=0x10d416b0;continue;}pc=0x10d416ac;continue;
}
0x10d416ac=>{
// 0x10d416ac mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
pc=0x10d416b0;continue;
}
0x10d416b0=>{
// 0x10d416b0 ldr x8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d416b4 cbz x8, #0x10d416bc
if (cpu.x[8]) == 0 {pc=0x10d416bc;continue;}pc=0x10d416b8;continue;
}
0x10d416b8=>{
// 0x10d416b8 str w20, [x8, #0x34]
{let address=(cpu.x[8]).wrapping_add(0x34u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d416bc;continue;
}
0x10d416bc=>{
// 0x10d416bc ldp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d416c0 ldp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d416c4 ldp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d416c8 ldp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d416cc ldp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d416d0 ldp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d416d4 add sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d416d8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d40cbc")),}}}
#[inline]
pub(super) unsafe fn f_10d42060(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d42060=>{
// 0x10d42060 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d42064 str x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d42068 stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d4206c stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d42070 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d42074 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d42078 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d4207c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d42080 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d42084 bl #0x110c98a0
cpu.x[30]=0x10d42088;super::chunk7::f_110c98a0(cpu,rt,0x110c98a0)?;
pc=0x10d42088;continue;
}
0x10d42088=>{
// 0x10d42088 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d4208c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d42090 bl #0x110c9904
cpu.x[30]=0x10d42094;super::chunk8::f_110c9904(cpu,rt,0x110c9904)?;
pc=0x10d42094;continue;
}
0x10d42094=>{
// 0x10d42094 ldr x20, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d42098 cbnz x20, #0x10d420b0
if (cpu.x[20]) != 0 {pc=0x10d420b0;continue;}pc=0x10d4209c;continue;
}
0x10d4209c=>{
// 0x10d4209c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d420a0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d420a4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d420a8 bl #0x1128df18
cpu.x[30]=0x10d420ac;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d420ac ldr x20, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
pc=0x10d420b0;continue;
}
0x10d420b0=>{
// 0x10d420b0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d420b4 bl #0x10e8bcb8
cpu.x[30]=0x10d420b8;super::chunk10::f_10e8bcb8(cpu,rt,0x10e8bcb8)?;
pc=0x10d420b8;continue;
}
0x10d420b8=>{
// 0x10d420b8 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d420bc ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d420c0 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d420c4 b.lt #0x10d421b8
if cpu.condition(11){pc=0x10d421b8;continue;}pc=0x10d420c8;continue;
}
0x10d420c8=>{
// 0x10d420c8 ldr x22, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d420cc mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10d420d0 b #0x10d420f0
pc=0x10d420f0;continue;
}
0x10d420d4=>{
// 0x10d420d4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d420d8 mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10d420dc mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10d420e0 bl #0x10f8c274
cpu.x[30]=0x10d420e4;super::dispatch(cpu,rt,0x10f8c274)?;
pc=0x10d420e4;continue;
}
0x10d420e4=>{
// 0x10d420e4 add w23, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10d420e8 cmp w21, w23
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d420ec b.eq #0x10d421b4
if cpu.condition(0){pc=0x10d421b4;continue;}pc=0x10d420f0;continue;
}
0x10d420f0=>{
// 0x10d420f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d420f4 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d420f8 bl #0x10e8bcc0
cpu.x[30]=0x10d420fc;super::chunk8::f_10e8bcc0(cpu,rt,0x10e8bcc0)?;
pc=0x10d420fc;continue;
}
0x10d420fc=>{
// 0x10d420fc mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10d42100 bl #0x10e0b614
cpu.x[30]=0x10d42104;super::chunk0::f_10e0b614(cpu,rt,0x10e0b614)?;
pc=0x10d42104;continue;
}
0x10d42104=>{
// 0x10d42104 cmp w0, #0x2a
{let a=((cpu.x[0] as u32 as u64));let b=((0x2au64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d42108 b.eq #0x10d4211c
if cpu.condition(0){pc=0x10d4211c;continue;}pc=0x10d4210c;continue;
}
0x10d4210c=>{
// 0x10d4210c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d42110 bl #0x10e0b614
cpu.x[30]=0x10d42114;super::chunk0::f_10e0b614(cpu,rt,0x10e0b614)?;
// 0x10d42114 cmp w0, #0x62
{let a=((cpu.x[0] as u32 as u64));let b=((0x62u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d42118 b.ne #0x10d420e4
if cpu.condition(1){pc=0x10d420e4;continue;}pc=0x10d4211c;continue;
}
0x10d4211c=>{
// 0x10d4211c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d42120 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d42124 bl #0x10e8bccc
cpu.x[30]=0x10d42128;super::chunk6::f_10e8bccc(cpu,rt,0x10e8bccc)?;
pc=0x10d42128;continue;
}
0x10d42128=>{
// 0x10d42128 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d4212c mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10d42130 bl #0x11289ec4
cpu.x[30]=0x10d42134;super::chunk5::f_11289ec4(cpu,rt,0x11289ec4)?;
pc=0x10d42134;continue;
}
0x10d42134=>{
// 0x10d42134 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d42138 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d4213c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d42140 bl #0x11289ec4
cpu.x[30]=0x10d42144;super::chunk5::f_11289ec4(cpu,rt,0x11289ec4)?;
pc=0x10d42144;continue;
}
0x10d42144=>{
// 0x10d42144 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d42148 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d4214c bl #0x10f8a168
cpu.x[30]=0x10d42150;super::chunk13::f_10f8a168(cpu,rt,0x10f8a168)?;
pc=0x10d42150;continue;
}
0x10d42150=>{
// 0x10d42150 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10d42154 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10d42158 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10d4215c mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10d42160 str x25, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d42164 bl #0x10f94250
cpu.x[30]=0x10d42168;super::chunk3::f_10f94250(cpu,rt,0x10f94250)?;
pc=0x10d42168;continue;
}
0x10d42168=>{
// 0x10d42168 ldr x0, [x19, #0x358]
{let address=(cpu.x[19]).wrapping_add(0x358u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d4216c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d42170 mov w2, w26
cpu.x[2]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10d42174 mov w3, w27
cpu.x[3]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10d42178 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4217c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d42180 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d42184;super::dispatch(cpu,rt,target)?;}
pc=0x10d42184;continue;
}
0x10d42184=>{
// 0x10d42184 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d42188 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d4218c bl #0x10f94050
cpu.x[30]=0x10d42190;super::chunk10::f_10f94050(cpu,rt,0x10f94050)?;
pc=0x10d42190;continue;
}
0x10d42190=>{
// 0x10d42190 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d42194 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d42198 ldr x8, [x8, #0x1a8]
{let address=(cpu.x[8]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4219c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d421a0;super::dispatch(cpu,rt,target)?;}
pc=0x10d421a0;continue;
}
0x10d421a0=>{
// 0x10d421a0 tbz w0, #0, #0x10d420d4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d420d4;continue;}pc=0x10d421a4;continue;
}
0x10d421a4=>{
// 0x10d421a4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d421a8 mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10d421ac bl #0x10f8c1ac
cpu.x[30]=0x10d421b0;super::chunk5::f_10f8c1ac(cpu,rt,0x10f8c1ac)?;
pc=0x10d421b0;continue;
}
0x10d421b0=>{
// 0x10d421b0 b #0x10d420e4
pc=0x10d420e4;continue;
}
0x10d421b4=>{
// 0x10d421b4 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10d421b8;continue;
}
0x10d421b8=>{
// 0x10d421b8 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d421bc ldr x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d421c0 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d421c4 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d421c8 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d421cc ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10d421d0 b #0x110c99ac
super::chunk12::f_110c99ac(cpu,rt,0x110c99ac)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d42060")),}}}
#[inline]
pub(super) unsafe fn f_10d430e4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d430e4=>{
// 0x10d430e4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d430e8 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d430ec mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d430f0 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d430f4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d430f8 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d430fc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d43100 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d43104 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d43108;super::dispatch(cpu,rt,target)?;}
pc=0x10d43108;continue;
}
0x10d43108=>{
// 0x10d43108 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4310c b.ne #0x10d4323c
if cpu.condition(1){pc=0x10d4323c;continue;}pc=0x10d43110;continue;
}
0x10d43110=>{
// 0x10d43110 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d43114 bl #0x10f7e050
cpu.x[30]=0x10d43118;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10d43118;continue;
}
0x10d43118=>{
// 0x10d43118 bl #0x10de02c4
cpu.x[30]=0x10d4311c;super::chunk15::f_10de02c4(cpu,rt,0x10de02c4)?;
pc=0x10d4311c;continue;
}
0x10d4311c=>{
// 0x10d4311c tbz w0, #0, #0x10d4323c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d4323c;continue;}pc=0x10d43120;continue;
}
0x10d43120=>{
// 0x10d43120 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d43124 bl #0x10f7e050
cpu.x[30]=0x10d43128;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10d43128;continue;
}
0x10d43128=>{
// 0x10d43128 bl #0x10de02cc
cpu.x[30]=0x10d4312c;super::chunk6::f_10de02cc(cpu,rt,0x10de02cc)?;
pc=0x10d4312c;continue;
}
0x10d4312c=>{
// 0x10d4312c tbnz w0, #0, #0x10d4323c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d4323c;continue;}pc=0x10d43130;continue;
}
0x10d43130=>{
// 0x10d43130 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d43134 bl #0x10f940fc
cpu.x[30]=0x10d43138;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10d43138;continue;
}
0x10d43138=>{
// 0x10d43138 bl #0x10fc0674
cpu.x[30]=0x10d4313c;super::chunk10::f_10fc0674(cpu,rt,0x10fc0674)?;
pc=0x10d4313c;continue;
}
0x10d4313c=>{
// 0x10d4313c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d43140 bl #0x10f94124
cpu.x[30]=0x10d43144;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10d43144;continue;
}
0x10d43144=>{
// 0x10d43144 add x8, x20, w0, sxtw #3
{let a=((cpu.x[20]));let b=((((cpu.x[0] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d43148 ldr x13, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10d4314c ldr w14, [x13, #0xc]
{let address=(cpu.x[13]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10d43150 cmp w14, #1
{let a=((cpu.x[14] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d43154 b.lt #0x10d4323c
if cpu.condition(11){pc=0x10d4323c;continue;}pc=0x10d43158;continue;
}
0x10d43158=>{
// 0x10d43158 ldr x10, [x13]
{let address=(cpu.x[13]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d4315c sub w11, w14, #1
{let a=((cpu.x[14] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10d43160 lsl x15, x14, #3
cpu.x[15]=(((((cpu.x[14]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d43164 mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10d43168 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
// 0x10d4316c mov w12, w11
cpu.x[12]=(((cpu.x[11] as u32 as u64))) as u32 as u64;
pc=0x10d43170;continue;
}
0x10d43170=>{
// 0x10d43170 ldr x16, [x10, x8]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[16]=(v0) as u64;}
// 0x10d43174 cmp x16, x19
{let a=((cpu.x[16]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d43178 b.eq #0x10d43194
if cpu.condition(0){pc=0x10d43194;continue;}pc=0x10d4317c;continue;
}
0x10d4317c=>{
// 0x10d4317c add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d43180 sub w12, w12, #1
{let a=((cpu.x[12] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10d43184 sub x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10d43188 cmp x15, x8
{let a=((cpu.x[15]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d4318c b.ne #0x10d43170
if cpu.condition(1){pc=0x10d43170;continue;}pc=0x10d43190;continue;
}
0x10d43190=>{
// 0x10d43190 b #0x10d4323c
pc=0x10d4323c;continue;
}
0x10d43194=>{
// 0x10d43194 neg x15, x9
cpu.x[15]=(0u64.wrapping_sub(((cpu.x[9])))) as u64;
// 0x10d43198 tbnz w15, #0x1f, #0x10d4323c
if ((cpu.x[15] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d4323c;continue;}pc=0x10d4319c;continue;
}
0x10d4319c=>{
// 0x10d4319c sub w14, w14, #1
{let a=((cpu.x[14] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u32 as u64;}
// 0x10d431a0 cmp w14, w15
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[15] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d431a4 str w14, [x13, #0xc]
{let address=(cpu.x[13]).wrapping_add(0xcu64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d431a8 b.le #0x10d4323c
if cpu.condition(13){pc=0x10d4323c;continue;}pc=0x10d431ac;continue;
}
0x10d431ac=>{
// 0x10d431ac add x13, x14, x9
{let a=((cpu.x[14]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10d431b0 cmp w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d431b4 b.lt #0x10d4323c
if cpu.condition(11){pc=0x10d4323c;continue;}pc=0x10d431b8;continue;
}
0x10d431b8=>{
// 0x10d431b8 cmp w13, #3
{let a=((cpu.x[13] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d431bc b.hi #0x10d431c8
if cpu.condition(8){pc=0x10d431c8;continue;}pc=0x10d431c0;continue;
}
0x10d431c0=>{
// 0x10d431c0 add x8, x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d431c4 b #0x10d4321c
pc=0x10d4321c;continue;
}
0x10d431c8=>{
// 0x10d431c8 mov w12, w12
cpu.x[12]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x10d431cc add x16, x9, w11, uxtw
{let a=((cpu.x[9]));let b=(((cpu.x[11] as u32 as u64) as u32 as u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x10d431d0 and x11, x13, #0xffffffff
{let a=((cpu.x[13]));let b=((0xffffffffu64));let result=a & b;cpu.x[11]=(result) as u64;}
// 0x10d431d4 lsr x12, x12, #2
cpu.x[12]=(((((cpu.x[12]))) as u64).wrapping_shr(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10d431d8 add x17, x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[17]=(result) as u64;}
// 0x10d431dc neg w15, w12, lsl #2
cpu.x[15]=(0u64.wrapping_sub((((cpu.x[12] as u32 as u64)) as u32).wrapping_shl(2) as u64)) as u32 as u64;
// 0x10d431e0 add x13, x15, w14, uxtw
{let a=((cpu.x[15]));let b=(((cpu.x[14] as u32 as u64) as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10d431e4 add x14, x10, x12, lsl #5
{let a=((cpu.x[10]));let b=(((cpu.x[12])) as u64).wrapping_shl(5) as u64;let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10d431e8 add x12, x17, #0x10
{let a=((cpu.x[17]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10d431ec and x10, x11, #0xfffffffc
{let a=((cpu.x[11]));let b=((0xfffffffcu64));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10d431f0 add x13, x13, x9
{let a=((cpu.x[13]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10d431f4 add x8, x14, x8
{let a=((cpu.x[14]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d431f8 and x9, x16, #0xfffffffc
{let a=((cpu.x[16]));let b=((0xfffffffcu64));let result=a & b;cpu.x[9]=(result) as u64;}
pc=0x10d431fc;continue;
}
0x10d431fc=>{
// 0x10d431fc ldur q0, [x12, #-8]
{let address=(cpu.x[12]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10d43200 ldur q1, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10d43204 subs x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10d43208 stp q0, q1, [x12, #-0x10]
{let address=(cpu.x[12]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d4320c add x12, x12, #0x20
{let a=((cpu.x[12]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10d43210 b.ne #0x10d431fc
if cpu.condition(1){pc=0x10d431fc;continue;}pc=0x10d43214;continue;
}
0x10d43214=>{
// 0x10d43214 cmp x10, x11
{let a=((cpu.x[10]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d43218 b.eq #0x10d4323c
if cpu.condition(0){pc=0x10d4323c;continue;}pc=0x10d4321c;continue;
}
0x10d4321c=>{
// 0x10d4321c add w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d43220 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10d43224;continue;
}
0x10d43224=>{
// 0x10d43224 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d43228 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d4322c cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d43230 stur x10, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d43234 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d43238 b.hi #0x10d43224
if cpu.condition(8){pc=0x10d43224;continue;}pc=0x10d4323c;continue;
}
0x10d4323c=>{
// 0x10d4323c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d43240 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d43244 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d430e4")),}}}
#[inline]
pub(super) unsafe fn f_10d49048(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d49048=>{
// 0x10d49048 adrp x8, #0x102b2000
cpu.x[8]=(0x102b2000u64) as u64;
// 0x10d4904c str wzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d49050 ldr d0, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10d49054 str d0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d49058 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d49048")),}}}
#[inline]
pub(super) unsafe fn f_10d55720(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d55720=>{
// 0x10d55720 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d55724 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d55728 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d5572c bl #0x10d33f6c
cpu.x[30]=0x10d55730;super::chunk12::f_10d33f6c(cpu,rt,0x10d33f6c)?;
pc=0x10d55730;continue;
}
0x10d55730=>{
// 0x10d55730 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d55734 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10d55738 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x10d5573c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d55720")),}}}
#[inline]
pub(super) unsafe fn f_10d55a08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d55a08=>{
// 0x10d55a08 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d55a0c stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d55a10 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d55a14 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d55a18 ldrsw x8, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d55a1c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d55a20 cbz w8, #0x10d55a58
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d55a58;continue;}pc=0x10d55a24;continue;
}
0x10d55a24=>{
// 0x10d55a24 ldr x21, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d55a28 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d55a2c b #0x10d55a3c
pc=0x10d55a3c;continue;
}
0x10d55a30=>{
// 0x10d55a30 subs x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10d55a34 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d55a38 b.eq #0x10d55a58
if cpu.condition(0){pc=0x10d55a58;continue;}pc=0x10d55a3c;continue;
}
0x10d55a3c=>{
// 0x10d55a3c ldr x20, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d55a40 cbz x20, #0x10d55a30
if (cpu.x[20]) == 0 {pc=0x10d55a30;continue;}pc=0x10d55a44;continue;
}
0x10d55a44=>{
// 0x10d55a44 add x0, x20, #0x10
{let a=((cpu.x[20]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d55a48 bl #0x1142fcc4
cpu.x[30]=0x10d55a4c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d55a4c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55a50 bl #0x1187d2c0
cpu.x[30]=0x10d55a54;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d55a54 b #0x10d55a30
pc=0x10d55a30;continue;
}
0x10d55a58=>{
// 0x10d55a58 str wzr, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55a5c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d55a60 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d55a64 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d55a68 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d55a08")),}}}
#[inline]
pub(super) unsafe fn f_10d56e88(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d56e88=>{
// 0x10d56e88 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d56e8c str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d56e90 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d56e94 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d56e98 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d56e9c bl #0x10d56798
cpu.x[30]=0x10d56ea0;super::chunk13::f_10d56798(cpu,rt,0x10d56798)?;
pc=0x10d56ea0;continue;
}
0x10d56ea0=>{
// 0x10d56ea0 str w0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56ea4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d56ea8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d56eac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d56e88")),}}}
#[inline]
pub(super) unsafe fn f_10d58abc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d58abc=>{
// 0x10d58abc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d58ac0 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58ac4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58ac8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d58acc ldr x9, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d58ad0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d58ad4 mov x19, x8
cpu.x[19]=(((cpu.x[8]))) as u64;
// 0x10d58ad8 cbz x9, #0x10d58bd0
if (cpu.x[9]) == 0 {pc=0x10d58bd0;continue;}pc=0x10d58adc;continue;
}
0x10d58adc=>{
// 0x10d58adc ldr x21, [x9, #0x18]
{let address=(cpu.x[9]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d58ae0 cbz x21, #0x10d58bd0
if (cpu.x[21]) == 0 {pc=0x10d58bd0;continue;}pc=0x10d58ae4;continue;
}
0x10d58ae4=>{
// 0x10d58ae4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58ae8 bl #0x10d39854
cpu.x[30]=0x10d58aec;super::chunk13::f_10d39854(cpu,rt,0x10d39854)?;
pc=0x10d58aec;continue;
}
0x10d58aec=>{
// 0x10d58aec tbz w0, #0, #0x10d58b18
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58b18;continue;}pc=0x10d58af0;continue;
}
0x10d58af0=>{
// 0x10d58af0 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58af4 bl #0x1187d2b0
cpu.x[30]=0x10d58af8;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58af8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58afc mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58b00 bl #0x10d4ea24
cpu.x[30]=0x10d58b04;super::dispatch(cpu,rt,0x10d4ea24)?;
// 0x10d58b04 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58b08 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58b0c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58b10 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58b14 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58b18=>{
// 0x10d58b18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58b1c bl #0x10d34864
cpu.x[30]=0x10d58b20;super::chunk9::f_10d34864(cpu,rt,0x10d34864)?;
pc=0x10d58b20;continue;
}
0x10d58b20=>{
// 0x10d58b20 tbz w0, #0, #0x10d58b38
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58b38;continue;}pc=0x10d58b24;continue;
}
0x10d58b24=>{
// 0x10d58b24 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d58b28 bl #0x1187d2b0
cpu.x[30]=0x10d58b2c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58b2c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58b30 mov w2, #3
cpu.x[2]=(((0x3u64))) as u32 as u64;
// 0x10d58b34 b #0x10d58b58
pc=0x10d58b58;continue;
}
0x10d58b38=>{
// 0x10d58b38 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58b3c bl #0x10d36084
cpu.x[30]=0x10d58b40;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10d58b40;continue;
}
0x10d58b40=>{
// 0x10d58b40 tbz w0, #0, #0x10d58b74
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58b74;continue;}pc=0x10d58b44;continue;
}
0x10d58b44=>{
// 0x10d58b44 ldr w22, [x21, #0x110]
{let address=(cpu.x[21]).wrapping_add(0x110u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10d58b48 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d58b4c bl #0x1187d2b0
cpu.x[30]=0x10d58b50;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58b50 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58b54 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
pc=0x10d58b58;continue;
}
0x10d58b58=>{
// 0x10d58b58 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58b5c bl #0x10d4f00c
cpu.x[30]=0x10d58b60;super::dispatch(cpu,rt,0x10d4f00c)?;
// 0x10d58b60 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58b64 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58b68 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58b6c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58b70 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58b74=>{
// 0x10d58b74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58b78 bl #0x10d374d4
cpu.x[30]=0x10d58b7c;super::chunk1::f_10d374d4(cpu,rt,0x10d374d4)?;
pc=0x10d58b7c;continue;
}
0x10d58b7c=>{
// 0x10d58b7c tbz w0, #0, #0x10d58b94
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58b94;continue;}pc=0x10d58b80;continue;
}
0x10d58b80=>{
// 0x10d58b80 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d58b84 bl #0x1187d2b0
cpu.x[30]=0x10d58b88;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58b88 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58b8c mov w2, #2
cpu.x[2]=(((0x2u64))) as u32 as u64;
// 0x10d58b90 b #0x10d58bb4
pc=0x10d58bb4;continue;
}
0x10d58b94=>{
// 0x10d58b94 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58b98 bl #0x10d33f6c
cpu.x[30]=0x10d58b9c;super::chunk12::f_10d33f6c(cpu,rt,0x10d33f6c)?;
pc=0x10d58b9c;continue;
}
0x10d58b9c=>{
// 0x10d58b9c cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d58ba0 b.lt #0x10d58bd0
if cpu.condition(11){pc=0x10d58bd0;continue;}pc=0x10d58ba4;continue;
}
0x10d58ba4=>{
// 0x10d58ba4 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d58ba8 bl #0x1187d2b0
cpu.x[30]=0x10d58bac;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58bac mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58bb0 mov w2, #3
cpu.x[2]=(((0x3u64))) as u32 as u64;
pc=0x10d58bb4;continue;
}
0x10d58bb4=>{
// 0x10d58bb4 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58bb8 bl #0x10d4ed28
cpu.x[30]=0x10d58bbc;super::dispatch(cpu,rt,0x10d4ed28)?;
// 0x10d58bbc str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58bc0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58bc4 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58bc8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58bcc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58bd0=>{
// 0x10d58bd0 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58bd4 bl #0x1187d2b0
cpu.x[30]=0x10d58bd8;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d58bd8;continue;
}
0x10d58bd8=>{
// 0x10d58bd8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58bdc mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58be0 bl #0x10d4f12c
cpu.x[30]=0x10d58be4;super::chunk0::f_10d4f12c(cpu,rt,0x10d4f12c)?;
pc=0x10d58be4;continue;
}
0x10d58be4=>{
// 0x10d58be4 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58be8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58bec ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58bf0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58bf4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d58abc")),}}}
#[inline]
pub(super) unsafe fn f_10d5a254(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5a254=>{
// 0x10d5a254 mov w8, #0xcc28
cpu.x[8]=(((0xcc28u64))) as u32 as u64;
// 0x10d5a258 str x1, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d5a25c movk w8, #0xc9d, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10d5a260 str w8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d5a264 adrp x8, #0x1190e000
cpu.x[8]=(0x1190e000u64) as u64;
// 0x10d5a268 add x8, x8, #0xb20
{let a=((cpu.x[8]));let b=((0xb20u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d5a26c str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d5a270 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5a254")),}}}
#[inline]
pub(super) unsafe fn f_10d7275c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d7275c=>{
// 0x10d7275c movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d72760 strb wzr, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d72764 stp wzr, wzr, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=0u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d72768 str xzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d7276c stur d0, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d72770 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d7275c")),}}}
#[inline]
pub(super) unsafe fn f_10d7939c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d7939c=>{
// 0x10d7939c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d793a0 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d793a4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d793a8 mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10d793ac mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d793b0 bl #0x10d72718
cpu.x[30]=0x10d793b4;super::chunk2::f_10d72718(cpu,rt,0x10d72718)?;
pc=0x10d793b4;continue;
}
0x10d793b4=>{
// 0x10d793b4 adrp x8, #0x11910000
cpu.x[8]=(0x11910000u64) as u64;
// 0x10d793b8 add x8, x8, #0x850
{let a=((cpu.x[8]));let b=((0x850u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d793bc str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d793c0 mov x8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u64;
// 0x10d793c4 stp x8, x8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d793c8 cbz x20, #0x10d793f0
if (cpu.x[20]) == 0 {pc=0x10d793f0;continue;}pc=0x10d793cc;continue;
}
0x10d793cc=>{
// 0x10d793cc ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d793d0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d793d4 str w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d793d8 bl #0x10f7e050
cpu.x[30]=0x10d793dc;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10d793dc;continue;
}
0x10d793dc=>{
// 0x10d793dc bl #0x10e0b624
cpu.x[30]=0x10d793e0;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
pc=0x10d793e0;continue;
}
0x10d793e0=>{
// 0x10d793e0 str w0, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d793e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d793e8 bl #0x10f95c08
cpu.x[30]=0x10d793ec;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10d793ec;continue;
}
0x10d793ec=>{
// 0x10d793ec str w0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d793f0;continue;
}
0x10d793f0=>{
// 0x10d793f0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d793f4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d793f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d7939c")),}}}
#[inline]
pub(super) unsafe fn f_10dda984(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dda984=>{
// 0x10dda984 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10dda988 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10dda98c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10dda990 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10dda994 ldr w8, [x0, #0xf4]
{let address=(cpu.x[0]).wrapping_add(0xf4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10dda998 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10dda99c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10dda9a0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10dda9a4 b.lt #0x10dda9d4
if cpu.condition(11){pc=0x10dda9d4;continue;}pc=0x10dda9a8;continue;
}
0x10dda9a8=>{
// 0x10dda9a8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10dda9ac bl #0x10e0b6b4
cpu.x[30]=0x10dda9b0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10dda9b0;continue;
}
0x10dda9b0=>{
// 0x10dda9b0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10dda9b4 ldr w1, [x19, #0xf4]
{let address=(cpu.x[19]).wrapping_add(0xf4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10dda9b8 and x2, x20, #0xffffffff
{let a=((cpu.x[20]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10dda9bc ldr x4, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10dda9c0 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10dda9c4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10dda9c8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10dda9cc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10dda9d0 b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
0x10dda9d4=>{
// 0x10dda9d4 ldr w21, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10dda9d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10dda9dc bl #0x10e0b6b4
cpu.x[30]=0x10dda9e0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10dda9e0 ldr x4, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10dda9e4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10dda9e8 and x2, x20, #0xffffffff
{let a=((cpu.x[20]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10dda9ec mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10dda9f0 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10dda9f4 bl #0x10e7e418
cpu.x[30]=0x10dda9f8;super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;
// 0x10dda9f8 ldr w8, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10dda9fc sub w9, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10ddaa00 cmp w9, #0x3e7
{let a=((cpu.x[9] as u32 as u64));let b=((0x3e7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10ddaa04 b.hi #0x10ddaa1c
if cpu.condition(8){pc=0x10ddaa1c;continue;}pc=0x10ddaa08;continue;
}
0x10ddaa08=>{
// 0x10ddaa08 and w9, w8, #0xffff
{let a=((cpu.x[8] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10ddaa0c mov w10, #0x3e8
cpu.x[10]=(((0x3e8u64))) as u32 as u64;
// 0x10ddaa10 udiv w9, w10, w9
{let a=((cpu.x[10] as u32 as u64)) as u32;let b=((cpu.x[9] as u32 as u64)) as u32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10ddaa14 sdiv w10, w0, w9
{let a=((cpu.x[0] as u32 as u64)) as i32;let b=((cpu.x[9] as u32 as u64)) as i32;cpu.x[10]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10ddaa18 mul w0, w10, w9
cpu.x[0]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
pc=0x10ddaa1c;continue;
}
0x10ddaa1c=>{
// 0x10ddaa1c ldr w9, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10ddaa20 mov w10, #-0x64
cpu.x[10]=(((0xffffffffffffff9cu64))) as u32 as u64;
// 0x10ddaa24 bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10ddaa28 cmn w9, #0x64
{let a=((cpu.x[9] as u32 as u64));let b=((0x64u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10ddaa2c csel w9, w9, w10, gt
cpu.x[9]=(if cpu.condition(12){((cpu.x[9] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10ddaa30 mov w10, #0x63
cpu.x[10]=(((0x63u64))) as u32 as u64;
// 0x10ddaa34 add w9, w9, #0x64
{let a=((cpu.x[9] as u32 as u64));let b=((0x64u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10ddaa38 madd w9, w9, w0, w10
cpu.x[9]=((((cpu.x[10] as u32 as u64))).wrapping_add((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[0] as u32 as u64))))) as u32 as u64;
// 0x10ddaa3c mov w10, #0x851f
cpu.x[10]=(((0x851fu64))) as u32 as u64;
// 0x10ddaa40 movk w10, #0x51eb, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10ddaa44 smull x9, w9, w10
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10ddaa48 lsr x10, x9, #0x3f
cpu.x[10]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10ddaa4c asr x9, x9, #0x25
cpu.x[9]=(((((cpu.x[9]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10ddaa50 add w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10ddaa54 mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10ddaa58 mov w9, #0x4dd3
cpu.x[9]=(((0x4dd3u64))) as u32 as u64;
// 0x10ddaa5c movk w9, #0x1062, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10ddaa60 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10ddaa64 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10ddaa68 asr x8, x8, #0x26
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x26u64))) & 63) as u32) as u64) as u64;
// 0x10ddaa6c add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10ddaa70 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ddaa74 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10ddaa78 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10ddaa7c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dda984")),}}}
#[inline]
pub(super) unsafe fn f_10ddae78(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddae78=>{
// 0x10ddae78 ldr x0, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10ddae7c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddae78")),}}}
#[inline]
pub(super) unsafe fn f_10de0550(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0550=>{
// 0x10de0550 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10de0554 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0550")),}}}
#[inline]
pub(super) unsafe fn f_10de05cc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de05cc=>{
// 0x10de05cc ldr w0, [x0, #0x344]
{let address=(cpu.x[0]).wrapping_add(0x344u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de05d0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de05cc")),}}}
#[inline]
pub(super) unsafe fn f_10de0730(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0730=>{
// 0x10de0730 ldr x0, [x0, #0x248]
{let address=(cpu.x[0]).wrapping_add(0x248u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de0734 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0730")),}}}
#[inline]
pub(super) unsafe fn f_10de0ae4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0ae4=>{
// 0x10de0ae4 ldr w0, [x0, #0x324]
{let address=(cpu.x[0]).wrapping_add(0x324u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0ae8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0ae4")),}}}
#[inline]
pub(super) unsafe fn f_10de0c48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0c48=>{
// 0x10de0c48 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10de0c4c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10de0c50 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10de0c54 ldr w8, [x0, #0x3d4]
{let address=(cpu.x[0]).wrapping_add(0x3d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de0c58 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10de0c5c mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10de0c60 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de0c64 b.gt #0x10de0c78
if cpu.condition(12){pc=0x10de0c78;continue;}pc=0x10de0c68;continue;
}
0x10de0c68=>{
// 0x10de0c68 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10de0c6c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10de0c70 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10de0c74 bl #0x1128df18
cpu.x[30]=0x10de0c78;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10de0c78;continue;
}
0x10de0c78=>{
// 0x10de0c78 ldr x8, [x19, #0x3c8]
{let address=(cpu.x[19]).wrapping_add(0x3c8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10de0c7c ldr x9, [x19, #0x2a8]
{let address=(cpu.x[19]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10de0c80 ldrsw x8, [x8, w20, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[20] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10de0c84 ldr x8, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10de0c88 ldrb w0, [x8, #0xac]
{let address=(cpu.x[8]).wrapping_add(0xacu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0c8c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de0c90 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10de0c94 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0c48")),}}}
#[inline]
pub(super) unsafe fn f_10de1228(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1228=>{
// 0x10de1228 ldrb w0, [x0, #0x57c]
{let address=(cpu.x[0]).wrapping_add(0x57cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de122c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1228")),}}}
#[inline]
pub(super) unsafe fn f_10de1278(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1278=>{
// 0x10de1278 ldr w0, [x0, #0x3e4]
{let address=(cpu.x[0]).wrapping_add(0x3e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de127c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1278")),}}}
#[inline]
pub(super) unsafe fn f_10de1360(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1360=>{
// 0x10de1360 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10de1364 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10de1368 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10de136c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10de1370 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10de1374 bl #0x10e0b6b4
cpu.x[30]=0x10de1378;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10de1378;continue;
}
0x10de1378=>{
// 0x10de1378 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de137c ldr w1, [x20, #0x7c0]
{let address=(cpu.x[20]).wrapping_add(0x7c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10de1380 and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10de1384 ldr x4, [x20, #0x270]
{let address=(cpu.x[20]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10de1388 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10de138c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de1390 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10de1394 b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1360")),}}}
#[inline]
pub(super) unsafe fn f_10de1398(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1398=>{
// 0x10de1398 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10de139c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10de13a0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10de13a4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10de13a8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10de13ac bl #0x10e0b6b4
cpu.x[30]=0x10de13b0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10de13b0;continue;
}
0x10de13b0=>{
// 0x10de13b0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de13b4 ldr w1, [x20, #0x7c4]
{let address=(cpu.x[20]).wrapping_add(0x7c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10de13b8 and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10de13bc ldr x4, [x20, #0x270]
{let address=(cpu.x[20]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10de13c0 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10de13c4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de13c8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10de13cc b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1398")),}}}
#[inline]
pub(super) unsafe fn f_10e0bc20(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e0bc20=>{
// 0x10e0bc20 cbz x1, #0x10e0bc58
if (cpu.x[1]) == 0 {pc=0x10e0bc58;continue;}pc=0x10e0bc24;continue;
}
0x10e0bc24=>{
// 0x10e0bc24 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e0bc28 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e0bc2c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e0bc30 ldr w20, [x1, #0x40]
{let address=(cpu.x[1]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10e0bc34 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10e0bc38 cbz w20, #0x10e0bc60
if (cpu.x[20] as u32 as u64) == 0 {pc=0x10e0bc60;continue;}pc=0x10e0bc3c;continue;
}
0x10e0bc3c=>{
// 0x10e0bc3c ldr w8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e0bc40 cbz w8, #0x10e0bc80
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e0bc80;continue;}pc=0x10e0bc44;continue;
}
0x10e0bc44=>{
// 0x10e0bc44 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e0bc48 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10e0bc4c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e0bc50 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e0bc54 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e0bc58=>{
// 0x10e0bc58 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e0bc5c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e0bc60=>{
// 0x10e0bc60 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10e0bc64 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10e0bc68 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10e0bc6c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10e0bc70 bl #0x1128df18
cpu.x[30]=0x10e0bc74;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10e0bc74 ldr w20, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10e0bc78 ldr w8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e0bc7c cbnz w8, #0x10e0bc44
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10e0bc44;continue;}pc=0x10e0bc80;continue;
}
0x10e0bc80=>{
// 0x10e0bc80 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10e0bc84 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10e0bc88 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10e0bc8c bl #0x1128df18
cpu.x[30]=0x10e0bc90;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10e0bc90 ldr w8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e0bc94 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e0bc98 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10e0bc9c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e0bca0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e0bca4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e0bc20")),}}}
#[inline]
pub(super) unsafe fn f_10e48c7c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e48c7c=>{
// 0x10e48c7c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e48c80 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e48c84 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e48c88 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10e48c8c bl #0x10e0b668
cpu.x[30]=0x10e48c90;super::chunk4::f_10e0b668(cpu,rt,0x10e0b668)?;
pc=0x10e48c90;continue;
}
0x10e48c90=>{
// 0x10e48c90 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10e48c94 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10e48c98 bl #0x10e0b6b4
cpu.x[30]=0x10e48c9c;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10e48c9c;continue;
}
0x10e48c9c=>{
// 0x10e48c9c ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e48ca0 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10e48ca4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e48ca8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e48cac b #0x10e1a808
super::chunk12::f_10e1a808(cpu,rt,0x10e1a808)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e48c7c")),}}}
#[inline]
pub(super) unsafe fn f_10e7e97c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e97c=>{
// 0x10e7e97c ldr x0, [x0, #0xc0]
{let address=(cpu.x[0]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e7e980 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e97c")),}}}
#[inline]
pub(super) unsafe fn f_10ed0478(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ed0478=>{
// 0x10ed0478 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10ed047c b #0x10f942a0
super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ed0478")),}}}
#[inline]
pub(super) unsafe fn f_10f74184(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74184=>{
// 0x10f74184 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74184")),}}}
#[inline]
pub(super) unsafe fn f_10f75068(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f75068=>{
// 0x10f75068 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7506c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f75070 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f75074 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f75078 ldr w8, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7507c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f75080 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f75084 cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f75088 b.ne #0x10f750a0
if cpu.condition(1){pc=0x10f750a0;continue;}pc=0x10f7508c;continue;
}
0x10f7508c=>{
// 0x10f7508c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f75090 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f75094 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f75098 bl #0x1128df18
cpu.x[30]=0x10f7509c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10f7509c ldr w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f750a0;continue;
}
0x10f750a0=>{
// 0x10f750a0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f750a4 b.lt #0x10f750cc
if cpu.condition(11){pc=0x10f750cc;continue;}pc=0x10f750a8;continue;
}
0x10f750a8=>{
// 0x10f750a8 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10f750ac;continue;
}
0x10f750ac=>{
// 0x10f750ac ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f750b0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f750b4 ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f750b8 bl #0x10f742ec
cpu.x[30]=0x10f750bc;super::dispatch(cpu,rt,0x10f742ec)?;
// 0x10f750bc ldrsw x8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f750c0 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f750c4 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f750c8 b.lt #0x10f750ac
if cpu.condition(11){pc=0x10f750ac;continue;}pc=0x10f750cc;continue;
}
0x10f750cc=>{
// 0x10f750cc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f750d0 str w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f750d4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f750d8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f750dc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f750e0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f75068")),}}}
#[inline]
pub(super) unsafe fn f_10f79e7c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f79e7c=>{
// 0x10f79e7c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f79e80 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79e84 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79e88 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f79e8c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f79e90 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f79e94 bl #0x10f93bd8
cpu.x[30]=0x10f79e98;super::chunk14::f_10f93bd8(cpu,rt,0x10f93bd8)?;
pc=0x10f79e98;continue;
}
0x10f79e98=>{
// 0x10f79e98 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79e9c ldr w1, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79ea0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ea4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ea8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79eac;super::dispatch(cpu,rt,target)?;}
pc=0x10f79eac;continue;
}
0x10f79eac=>{
// 0x10f79eac ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79eb0 ldr w1, [x20, #0xfc]
{let address=(cpu.x[20]).wrapping_add(0xfcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79eb4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79eb8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ebc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79ec0;super::dispatch(cpu,rt,target)?;}
pc=0x10f79ec0;continue;
}
0x10f79ec0=>{
// 0x10f79ec0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ec4 ldr w1, [x20, #0x104]
{let address=(cpu.x[20]).wrapping_add(0x104u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79ec8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ecc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ed0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79ed4;super::dispatch(cpu,rt,target)?;}
pc=0x10f79ed4;continue;
}
0x10f79ed4=>{
// 0x10f79ed4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ed8 ldr w1, [x20, #0x114]
{let address=(cpu.x[20]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79edc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ee0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ee4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79ee8;super::dispatch(cpu,rt,target)?;}
pc=0x10f79ee8;continue;
}
0x10f79ee8=>{
// 0x10f79ee8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79eec ldrb w1, [x20, #0x118]
{let address=(cpu.x[20]).wrapping_add(0x118u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79ef0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ef4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ef8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79efc;super::dispatch(cpu,rt,target)?;}
pc=0x10f79efc;continue;
}
0x10f79efc=>{
// 0x10f79efc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f00 ldrb w1, [x20, #0x119]
{let address=(cpu.x[20]).wrapping_add(0x119u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79f04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79f08 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f0c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79f10;super::dispatch(cpu,rt,target)?;}
pc=0x10f79f10;continue;
}
0x10f79f10=>{
// 0x10f79f10 ldr x8, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f14 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f79f18 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79f1c cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f79f20 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f24 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f79f28 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79f2c;super::dispatch(cpu,rt,target)?;}
pc=0x10f79f2c;continue;
}
0x10f79f2c=>{
// 0x10f79f2c ldr x8, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f30 cbz x8, #0x10f79f9c
if (cpu.x[8]) == 0 {pc=0x10f79f9c;continue;}pc=0x10f79f34;continue;
}
0x10f79f34=>{
// 0x10f79f34 ldr w9, [x20, #0x114]
{let address=(cpu.x[20]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f79f38 ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f3c tbz w9, #0x1f, #0x10f79f44
if ((cpu.x[9] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10f79f44;continue;}pc=0x10f79f40;continue;
}
0x10f79f40=>{
// 0x10f79f40 ldr w9, [x8, #0x148]
{let address=(cpu.x[8]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f79f44;continue;
}
0x10f79f44=>{
// 0x10f79f44 ldr w10, [x8, #0x114]
{let address=(cpu.x[8]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f79f48 ldr w11, [x8, #0x10c]
{let address=(cpu.x[8]).wrapping_add(0x10cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f79f4c ldr w8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f79f50 sub w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f79f54 sdiv w9, w9, w11
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[11] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f79f58 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f79f5c csel w10, w8, w9, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f79f60 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f79f64 csel w8, w9, w10, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[9] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10f79f68 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f79f6c b.lt #0x10f79f9c
if cpu.condition(11){pc=0x10f79f9c;continue;}pc=0x10f79f70;continue;
}
0x10f79f70=>{
// 0x10f79f70 ubfiz x21, x8, #2, #0x20
cpu.x[21]=(((((cpu.x[8])) & 0xffffffffu64) << 2)) as u64;
// 0x10f79f74 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10f79f78;continue;
}
0x10f79f78=>{
// 0x10f79f78 ldr x8, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f7c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f79f80 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79f84 ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79f88 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79f8c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79f90;super::dispatch(cpu,rt,target)?;}
pc=0x10f79f90;continue;
}
0x10f79f90=>{
// 0x10f79f90 add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10f79f94 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f79f98 b.ne #0x10f79f78
if cpu.condition(1){pc=0x10f79f78;continue;}pc=0x10f79f9c;continue;
}
0x10f79f9c=>{
// 0x10f79f9c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79fa0 ldr w1, [x20, #0x110]
{let address=(cpu.x[20]).wrapping_add(0x110u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79fa4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79fa8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79fac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79fb0;super::dispatch(cpu,rt,target)?;}
pc=0x10f79fb0;continue;
}
0x10f79fb0=>{
// 0x10f79fb0 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f79fb4 ldr x2, [x20, #0x130]
{let address=(cpu.x[20]).wrapping_add(0x130u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f79fb8 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f79fbc bl #0x10f8c0ac
cpu.x[30]=0x10f79fc0;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f79fc0;continue;
}
0x10f79fc0=>{
// 0x10f79fc0 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f79fc4 ldr x2, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f79fc8 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f79fcc bl #0x10f8c0ac
cpu.x[30]=0x10f79fd0;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f79fd0;continue;
}
0x10f79fd0=>{
// 0x10f79fd0 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f79fd4 ldr x2, [x20, #0x140]
{let address=(cpu.x[20]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f79fd8 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f79fdc mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10f79fe0 bl #0x10f8bfc0
cpu.x[30]=0x10f79fe4;super::chunk11::f_10f8bfc0(cpu,rt,0x10f8bfc0)?;
pc=0x10f79fe4;continue;
}
0x10f79fe4=>{
// 0x10f79fe4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79fe8 ldrb w1, [x20, #0xf8]
{let address=(cpu.x[20]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f79fec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ff0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ff4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f79ff8;super::dispatch(cpu,rt,target)?;}
pc=0x10f79ff8;continue;
}
0x10f79ff8=>{
// 0x10f79ff8 ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ffc ldrb w8, [x8, #0xbc]
{let address=(cpu.x[8]).wrapping_add(0xbcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f7a000;continue;
}
0x10f7a000=>{
// 0x10f7a000 cbz w8, #0x10f7a014
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7a014;continue;}pc=0x10f7a004;continue;
}
0x10f7a004=>{
// 0x10f7a004 add x1, x20, #0x120
{let a=((cpu.x[20]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f7a008 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7a00c mov w2, #-1
cpu.x[2]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f7a010 bl #0x111dd468
cpu.x[30]=0x10f7a014;super::chunk3::f_111dd468(cpu,rt,0x111dd468)?;
pc=0x10f7a014;continue;
}
0x10f7a014=>{
// 0x10f7a014 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a018 ldr w1, [x20, #0x14c]
{let address=(cpu.x[20]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7a01c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7a020 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a024 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a028;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a028;continue;
}
0x10f7a028=>{
// 0x10f7a028 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a02c ldr w1, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7a030 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7a034 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7a038 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7a03c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f7a040 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f7a044 br x2
{let target=cpu.x[2];if target>=0x10f79e7c && target<0x10f7a048{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f79e7c")),}}}
#[inline]
pub(super) unsafe fn f_10f7e064(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7e064=>{
// 0x10f7e064 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7e068 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e06c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7e070 ldrb w8, [x0, #0x110]
{let address=(cpu.x[0]).wrapping_add(0x110u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e074 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7e078 add x20, x0, #0x114
{let a=((cpu.x[0]));let b=((0x114u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f7e07c cbz w8, #0x10f7e09c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7e09c;continue;}pc=0x10f7e080;continue;
}
0x10f7e080=>{
// 0x10f7e080 ldp x8, x9, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[8]=(v0) as u64;cpu.x[9]=(v1) as u64;}
// 0x10f7e084 cbz x8, #0x10f7e08c
if (cpu.x[8]) == 0 {pc=0x10f7e08c;continue;}pc=0x10f7e088;continue;
}
0x10f7e088=>{
// 0x10f7e088 str x9, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7e08c;continue;
}
0x10f7e08c=>{
// 0x10f7e08c cbz x9, #0x10f7e094
if (cpu.x[9]) == 0 {pc=0x10f7e094;continue;}pc=0x10f7e090;continue;
}
0x10f7e090=>{
// 0x10f7e090 str x8, [x9, #0x100]
{let address=(cpu.x[9]).wrapping_add(0x100u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7e094;continue;
}
0x10f7e094=>{
// 0x10f7e094 stp xzr, xzr, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e098 strb wzr, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10f7e09c;continue;
}
0x10f7e09c=>{
// 0x10f7e09c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7e0a0 bl #0x10f93eb0
cpu.x[30]=0x10f7e0a4;super::chunk6::f_10f93eb0(cpu,rt,0x10f93eb0)?;
pc=0x10f7e0a4;continue;
}
0x10f7e0a4=>{
// 0x10f7e0a4 adrp x8, #0x102b1000
cpu.x[8]=(0x102b1000u64) as u64;
// 0x10f7e0a8 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10f7e0ac adrp x9, #0x102b3000
cpu.x[9]=(0x102b3000u64) as u64;
// 0x10f7e0b0 ldr d1, [x8, #0xfa8]
{let address=(cpu.x[8]).wrapping_add(0xfa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f7e0b4 adrp x8, #0x102b2000
cpu.x[8]=(0x102b2000u64) as u64;
// 0x10f7e0b8 ldr q2, [x9, #0x650]
{let address=(cpu.x[9]).wrapping_add(0x650u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[2]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f7e0bc add x0, x19, #0x198
{let a=((cpu.x[19]));let b=((0x198u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f7e0c0 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f7e0c4 str wzr, [x19, #0x134]
{let address=(cpu.x[19]).wrapping_add(0x134u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e0c8 str d1, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0cc ldr q1, [x8, #0xaf0]
{let address=(cpu.x[8]).wrapping_add(0xaf0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f7e0d0 add x8, x19, #0x168
{let a=((cpu.x[19]));let b=((0x168u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7e0d4 str xzr, [x19, #0x180]
{let address=(cpu.x[19]).wrapping_add(0x180u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0d8 str d0, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0dc str d0, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0e0 stp q1, q2, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[2] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f7e0e4 stp q0, q0, [x19, #0x140]
{let address=(cpu.x[19]).wrapping_add(0x140u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f7e0e8 str wzr, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e0ec str xzr, [x19, #0x168]
{let address=(cpu.x[19]).wrapping_add(0x168u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0f0 stur xzr, [x8, #6]
{let address=(cpu.x[8]).wrapping_add(0x6u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e0f4 stp xzr, xzr, [x19, #0x1a0]
{let address=(cpu.x[19]).wrapping_add(0x1a0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e0f8 str wzr, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e0fc bl #0x112941f8
cpu.x[30]=0x10f7e100;super::chunk10::f_112941f8(cpu,rt,0x112941f8)?;
pc=0x10f7e100;continue;
}
0x10f7e100=>{
// 0x10f7e100 add x8, x19, #0x1bc
{let a=((cpu.x[19]));let b=((0x1bcu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7e104 add x9, x19, #0x1b4
{let a=((cpu.x[19]));let b=((0x1b4u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f7e108 str wzr, [x19, #0x178]
{let address=(cpu.x[19]).wrapping_add(0x178u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e10c str xzr, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e110 mov w8, #0x100
cpu.x[8]=(((0x100u64))) as u32 as u64;
// 0x10f7e114 str xzr, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e118 str xzr, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7e11c strh w8, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10f7e120 str wzr, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e124 strb wzr, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f7e128 stp xzr, xzr, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e12c str wzr, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7e130 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7e134 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f7e138 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7e064")),}}}
#[inline]
pub(super) unsafe fn f_10f825e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f825e0=>{
// 0x10f825e0 b #0x10f8915c
super::chunk9::f_10f8915c(cpu,rt,0x10f8915c)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f825e0")),}}}
#[inline]
pub(super) unsafe fn f_10f82cb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f82cb0=>{
// 0x10f82cb0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f82cb4 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f82cb8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f82cbc ldr w8, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82cc0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f82cc4 cmp w8, #0xe
{let a=((cpu.x[8] as u32 as u64));let b=((0xeu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82cc8 b.hs #0x10f82cd8
if cpu.condition(2){pc=0x10f82cd8;continue;}pc=0x10f82ccc;continue;
}
0x10f82ccc=>{
// 0x10f82ccc mov w9, #0x3515
cpu.x[9]=(((0x3515u64))) as u32 as u64;
// 0x10f82cd0 lsr w8, w9, w8
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as u32).wrapping_shr(((((cpu.x[8] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f82cd4 tbnz w8, #0, #0x10f82cec
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f82cec;continue;}pc=0x10f82cd8;continue;
}
0x10f82cd8=>{
// 0x10f82cd8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82cdc bl #0x10f9433c
cpu.x[30]=0x10f82ce0;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10f82ce0;continue;
}
0x10f82ce0=>{
// 0x10f82ce0 cbz x0, #0x10f82cfc
if (cpu.x[0]) == 0 {pc=0x10f82cfc;continue;}pc=0x10f82ce4;continue;
}
0x10f82ce4=>{
// 0x10f82ce4 bl #0x10fc40ec
cpu.x[30]=0x10f82ce8;super::chunk15::f_10fc40ec(cpu,rt,0x10fc40ec)?;
pc=0x10f82ce8;continue;
}
0x10f82ce8=>{
// 0x10f82ce8 tbz w0, #0, #0x10f82cfc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82cfc;continue;}pc=0x10f82cec;continue;
}
0x10f82cec=>{
// 0x10f82cec mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82cf0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f82cf4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82cf8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82cfc=>{
// 0x10f82cfc ldr w8, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82d00 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82d04 cset w0, lt
cpu.x[0]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10f82d08 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f82d0c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82d10 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f82cb0")),}}}
#[inline]
pub(super) unsafe fn f_10f84c80(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f84c80=>{
// 0x10f84c80 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f84c84 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f84c88 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f84c8c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f84c90 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f84c94 bl #0x10f94310
cpu.x[30]=0x10f84c98;super::chunk7::f_10f94310(cpu,rt,0x10f94310)?;
pc=0x10f84c98;continue;
}
0x10f84c98=>{
// 0x10f84c98 cbz x0, #0x10f84cf4
if (cpu.x[0]) == 0 {pc=0x10f84cf4;continue;}pc=0x10f84c9c;continue;
}
0x10f84c9c=>{
// 0x10f84c9c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f84ca0 bl #0x10f94298
cpu.x[30]=0x10f84ca4;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f84ca4;continue;
}
0x10f84ca4=>{
// 0x10f84ca4 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f84ca8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f84cac bl #0x10f942a0
cpu.x[30]=0x10f84cb0;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f84cb0;continue;
}
0x10f84cb0=>{
// 0x10f84cb0 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f84cb4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f84cb8 bl #0x10f94298
cpu.x[30]=0x10f84cbc;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f84cbc;continue;
}
0x10f84cbc=>{
// 0x10f84cbc mov x22, x19
cpu.x[22]=(((cpu.x[19]))) as u64;
// 0x10f84cc0 sub w8, w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f84cc4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f84cc8 str w8, [x22, #0xf8]!
{let address=(cpu.x[22]).wrapping_add(0xf8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[22]=(address) as u64;}
// 0x10f84ccc bl #0x10f942a0
cpu.x[30]=0x10f84cd0;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f84cd0;continue;
}
0x10f84cd0=>{
// 0x10f84cd0 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f84cd4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f84cd8 mov w1, #0x100
cpu.x[1]=(((0x100u64))) as u32 as u64;
// 0x10f84cdc sub w8, w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f84ce0 str w8, [x19, #0xfc]
{let address=(cpu.x[19]).wrapping_add(0xfcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f84ce4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f84ce8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f84cec ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f84cf0 b #0x11294d74
super::chunk2::f_11294d74(cpu,rt,0x11294d74)?;return Ok(());
}
0x10f84cf4=>{
// 0x10f84cf4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f84cf8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f84cfc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f84d00 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f84c80")),}}}
#[inline]
pub(super) unsafe fn f_10f85420(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85420=>{
// 0x10f85420 ldr w8, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f85424 cmp w8, #0xb
{let a=((cpu.x[8] as u32 as u64));let b=((0xbu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f85428 b.ne #0x10f85434
if cpu.condition(1){pc=0x10f85434;continue;}pc=0x10f8542c;continue;
}
0x10f8542c=>{
// 0x10f8542c mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f85430 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f85434=>{
// 0x10f85434 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f85438 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8543c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f85440 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85444 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f85448 ldr x8, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8544c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f85450;super::dispatch(cpu,rt,target)?;}
pc=0x10f85450;continue;
}
0x10f85450=>{
// 0x10f85450 tbz w0, #0, #0x10f85464
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f85464;continue;}pc=0x10f85454;continue;
}
0x10f85454=>{
// 0x10f85454 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f85458 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8545c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f85460 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f85464=>{
// 0x10f85464 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85468 bl #0x10de1240
cpu.x[30]=0x10f8546c;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
pc=0x10f8546c;continue;
}
0x10f8546c=>{
// 0x10f8546c tbnz w0, #0, #0x10f8547c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f8547c;continue;}pc=0x10f85470;continue;
}
0x10f85470=>{
// 0x10f85470 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85474 bl #0x10de1248
cpu.x[30]=0x10f85478;super::chunk8::f_10de1248(cpu,rt,0x10de1248)?;
pc=0x10f85478;continue;
}
0x10f85478=>{
// 0x10f85478 tbz w0, #0, #0x10f8549c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8549c;continue;}pc=0x10f8547c;continue;
}
0x10f8547c=>{
// 0x10f8547c ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85480 ldr w20, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10f85484 bl #0x10de1278
cpu.x[30]=0x10f85488;super::chunk1::f_10de1278(cpu,rt,0x10de1278)?;
// 0x10f85488 cmp w20, w0
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8548c cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f85490 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85494 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f85498 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8549c=>{
// 0x10f8549c adrp x8, #0x11a66000
cpu.x[8]=(0x11a66000u64) as u64;
// 0x10f854a0 add x8, x8, #0x20
{let a=((cpu.x[8]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f854a4 ldr x9, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f854a8 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f854ac tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10f854b0 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f854b4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f854b8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f854bc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85420")),}}}
#[inline]
pub(super) unsafe fn f_10f859f8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f859f8=>{
// 0x10f859f8 ldr w8, [x0, #0x1d8]
{let address=(cpu.x[0]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f859fc add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f85a00 str w8, [x0, #0x1d8]
{let address=(cpu.x[0]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f85a04 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f859f8")),}}}
#[inline]
pub(super) unsafe fn f_10f85d10(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85d10=>{
// 0x10f85d10 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f85d14 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f85d18 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85d1c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f85d20 ldr x19, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f85d24 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f85d28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85d2c bl #0x10de01b4
cpu.x[30]=0x10f85d30;super::chunk10::f_10de01b4(cpu,rt,0x10de01b4)?;
pc=0x10f85d30;continue;
}
0x10f85d30=>{
// 0x10f85d30 ldr w8, [x19, #0x3d4]
{let address=(cpu.x[19]).wrapping_add(0x3d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f85d34 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f85d38 b.lt #0x10f85d70
if cpu.condition(11){pc=0x10f85d70;continue;}pc=0x10f85d3c;continue;
}
0x10f85d3c=>{
// 0x10f85d3c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f85d40 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f85d44 bl #0x10f94608
cpu.x[30]=0x10f85d48;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10f85d48;continue;
}
0x10f85d48=>{
// 0x10f85d48 ldr w1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f85d4c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85d50 bl #0x10de0eec
cpu.x[30]=0x10f85d54;super::chunk13::f_10de0eec(cpu,rt,0x10de0eec)?;
pc=0x10f85d54;continue;
}
0x10f85d54=>{
// 0x10f85d54 ldr x9, [x19, #0x2a8]
{let address=(cpu.x[19]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f85d58 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f85d5c mov w0, w21
cpu.x[0]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f85d60 ldr x8, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85d64 ldr w8, [x8, #0xfc]
{let address=(cpu.x[8]).wrapping_add(0xfcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f85d68 cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f85d6c csel w0, w21, w8, eq
cpu.x[0]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
pc=0x10f85d70;continue;
}
0x10f85d70=>{
// 0x10f85d70 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85d74 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f85d78 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f85d7c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85d10")),}}}
#[inline]
pub(super) unsafe fn f_10f86bb8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86bb8=>{
// 0x10f86bb8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86bbc str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86bc0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86bc4 ldr w8, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f86bc8 cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f86bcc b.eq #0x10f86bf0
if cpu.condition(0){pc=0x10f86bf0;continue;}pc=0x10f86bd0;continue;
}
0x10f86bd0=>{
// 0x10f86bd0 ldr x8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86bd4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86bd8 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f86bdc bl #0x10f93d2c
cpu.x[30]=0x10f86be0;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f86be0;continue;
}
0x10f86be0=>{
// 0x10f86be0 cbz x0, #0x10f86bf0
if (cpu.x[0]) == 0 {pc=0x10f86bf0;continue;}pc=0x10f86be4;continue;
}
0x10f86be4=>{
// 0x10f86be4 ldr w1, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f86be8 cmp w1, #0x64
{let a=((cpu.x[1] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f86bec b.ne #0x10f86c00
if cpu.condition(1){pc=0x10f86c00;continue;}pc=0x10f86bf0;continue;
}
0x10f86bf0=>{
// 0x10f86bf0 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f86bf4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f86bf8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86bfc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f86c00=>{
// 0x10f86c00 bl #0x10d32c84
cpu.x[30]=0x10f86c04;super::chunk15::f_10d32c84(cpu,rt,0x10d32c84)?;
pc=0x10f86c04;continue;
}
0x10f86c04=>{
// 0x10f86c04 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86c08 ldr w1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f86c0c ldr x2, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f86c10 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f86c14 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86c18 br x2
{let target=cpu.x[2];if target>=0x10f86bb8 && target<0x10f86c1c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86bb8")),}}}
#[inline]
pub(super) unsafe fn f_10f87910(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87910=>{
// 0x10f87910 str w1, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87914 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87910")),}}}
#[inline]
pub(super) unsafe fn f_10f879e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f879e0=>{
// 0x10f879e0 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f879e4 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879e8 stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879ec stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879f0 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879f4 stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879f8 stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f879fc add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f87a00 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f87a04 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f87a08 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f87a0c bl #0x10f93d3c
cpu.x[30]=0x10f87a10;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f87a10;continue;
}
0x10f87a10=>{
// 0x10f87a10 stur w0, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87a14 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87a18 bl #0x10f95c00
cpu.x[30]=0x10f87a1c;super::chunk11::f_10f95c00(cpu,rt,0x10f95c00)?;
pc=0x10f87a1c;continue;
}
0x10f87a1c=>{
// 0x10f87a1c str w0, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87a20 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87a24 bl #0x10f95bec
cpu.x[30]=0x10f87a28;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10f87a28;continue;
}
0x10f87a28=>{
// 0x10f87a28 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f87a2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87a30 bl #0x10f95c08
cpu.x[30]=0x10f87a34;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f87a34;continue;
}
0x10f87a34=>{
// 0x10f87a34 ldrb w28, [x20, #0xa4]
{let address=(cpu.x[20]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10f87a38 ldr w21, [x20, #0xac]
{let address=(cpu.x[20]).wrapping_add(0xacu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f87a3c mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f87a40 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87a44 bl #0x10f95f8c
cpu.x[30]=0x10f87a48;super::chunk7::f_10f95f8c(cpu,rt,0x10f95f8c)?;
pc=0x10f87a48;continue;
}
0x10f87a48=>{
// 0x10f87a48 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f87a4c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87a50 bl #0x10f942c0
cpu.x[30]=0x10f87a54;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10f87a54;continue;
}
0x10f87a54=>{
// 0x10f87a54 ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f87a58 ldr w22, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f87a5c mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10f87a60 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f87a64 bl #0x10e0b624
cpu.x[30]=0x10f87a68;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
pc=0x10f87a68;continue;
}
0x10f87a68=>{
// 0x10f87a68 ldr w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87a6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f87a70 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f87a74 b.eq #0x10f87a88
if cpu.condition(0){pc=0x10f87a88;continue;}pc=0x10f87a78;continue;
}
0x10f87a78=>{
// 0x10f87a78 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f87a7c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f87a80 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f87a84 bl #0x1128df18
cpu.x[30]=0x10f87a88;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10f87a88;continue;
}
0x10f87a88=>{
// 0x10f87a88 ldur w9, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f87a8c ldr w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87a90 stp w23, w24, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[23] as u32 as u64;let v1=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f87a94 stp w21, w25, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=cpu.x[21] as u32 as u64;let v1=cpu.x[25] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f87a98 stp w9, w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f87a9c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f87aa0 ldr d0, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f87aa4 str x20, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87aa8 strb w28, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=cpu.x[28] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f87aac strb w8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f87ab0 stur d0, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87ab4 stp w22, w27, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=cpu.x[22] as u32 as u64;let v1=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f87ab8 ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f87abc ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f87ac0 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f87ac4 ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f87ac8 ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f87acc ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f87ad0 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f87ad4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f879e0")),}}}
#[inline]
pub(super) unsafe fn f_10f88114(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f88114=>{
// 0x10f88114 ldrsw x8, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f88118 cbz w8, #0x10f88174
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f88174;continue;}pc=0x10f8811c;continue;
}
0x10f8811c=>{
// 0x10f8811c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f88120 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f88124 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f88128 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8812c ldr x21, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f88130 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f88134 mov x19, x4
cpu.x[19]=(((cpu.x[4]))) as u64;
// 0x10f88138 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f8813c b #0x10f8814c
pc=0x10f8814c;continue;
}
0x10f88140=>{
// 0x10f88140 subs x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f88144 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f88148 b.eq #0x10f88168
if cpu.condition(0){pc=0x10f88168;continue;}pc=0x10f8814c;continue;
}
0x10f8814c=>{
// 0x10f8814c ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f88150 ldr x8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88154 cmp x8, x20
{let a=((cpu.x[8]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88158 b.ne #0x10f88140
if cpu.condition(1){pc=0x10f88140;continue;}pc=0x10f8815c;continue;
}
0x10f8815c=>{
// 0x10f8815c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f88160 bl #0x10f87ad8
cpu.x[30]=0x10f88164;super::chunk8::f_10f87ad8(cpu,rt,0x10f87ad8)?;
pc=0x10f88164;continue;
}
0x10f88164=>{
// 0x10f88164 b #0x10f88140
pc=0x10f88140;continue;
}
0x10f88168=>{
// 0x10f88168 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8816c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f88170 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10f88174;continue;
}
0x10f88174=>{
// 0x10f88174 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f88114")),}}}
#[inline]
pub(super) unsafe fn f_10f88a8c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f88a8c=>{
// 0x10f88a8c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88a90 ldrb w9, [x8, #0xe8]
{let address=(cpu.x[8]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f88a94 ldrb w8, [x8, #0xe9]
{let address=(cpu.x[8]).wrapping_add(0xe9u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f88a98 orr w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f88a9c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f88aa0 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f88aa4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f88a8c")),}}}
#[inline]
pub(super) unsafe fn f_10f8bea8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8bea8=>{
// 0x10f8bea8 cbz w1, #0x10f8befc
if (cpu.x[1] as u32 as u64) == 0 {pc=0x10f8befc;continue;}pc=0x10f8beac;continue;
}
0x10f8beac=>{
// 0x10f8beac ldr w10, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8beb0 cbz w10, #0x10f8befc
if (cpu.x[10] as u32 as u64) == 0 {pc=0x10f8befc;continue;}pc=0x10f8beb4;continue;
}
0x10f8beb4=>{
// 0x10f8beb4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8beb8 cmp w10, #2
{let a=((cpu.x[10] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bebc b.lt #0x10f8bf04
if cpu.condition(11){pc=0x10f8bf04;continue;}pc=0x10f8bec0;continue;
}
0x10f8bec0=>{
// 0x10f8bec0 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10f8bec4 sub w10, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f8bec8 b #0x10f8bed8
pc=0x10f8bed8;continue;
}
0x10f8becc=>{
// 0x10f8becc cmp w11, w9
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bed0 mov w10, w11
cpu.x[10]=(((cpu.x[11] as u32 as u64))) as u32 as u64;
// 0x10f8bed4 b.le #0x10f8bf08
if cpu.condition(13){pc=0x10f8bf08;continue;}pc=0x10f8bed8;continue;
}
0x10f8bed8=>{
// 0x10f8bed8 add w11, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f8bedc asr w11, w11, #1
cpu.x[11]=(((((cpu.x[11] as u32 as u64))) as i32).wrapping_shr(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f8bee0 ldr x12, [x8, w11, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[11] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f8bee4 ldr w12, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10f8bee8 cmp w12, w1
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8beec b.ge #0x10f8becc
if cpu.condition(10){pc=0x10f8becc;continue;}pc=0x10f8bef0;continue;
}
0x10f8bef0=>{
// 0x10f8bef0 add w9, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f8bef4 mov w11, w10
cpu.x[11]=(((cpu.x[10] as u32 as u64))) as u32 as u64;
// 0x10f8bef8 b #0x10f8becc
pc=0x10f8becc;continue;
}
0x10f8befc=>{
// 0x10f8befc mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f8bf00 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8bf04=>{
// 0x10f8bf04 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
pc=0x10f8bf08;continue;
}
0x10f8bf08=>{
// 0x10f8bf08 ldr x8, [x8, w9, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[9] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bf0c ldr w8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bf10 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bf14 csinv w0, w9, wzr, eq
cpu.x[0]=(if cpu.condition(0){((cpu.x[9] as u32 as u64))}else{!(((0u64)))}) as u32 as u64;
// 0x10f8bf18 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8bea8")),}}}
#[inline]
pub(super) unsafe fn f_10f8c0ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8c0ac=>{
// 0x10f8c0ac mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f8c0b0 cbz x2, #0x10f8c0bc
if (cpu.x[2]) == 0 {pc=0x10f8c0bc;continue;}pc=0x10f8c0b4;continue;
}
0x10f8c0b4=>{
// 0x10f8c0b4 ldr w1, [x2, #8]
{let address=(cpu.x[2]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8c0b8 b #0x111dd758
super::chunk13::f_111dd758(cpu,rt,0x111dd758)?;return Ok(());
}
0x10f8c0bc=>{
// 0x10f8c0bc mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f8c0c0 b #0x111dd758
super::chunk13::f_111dd758(cpu,rt,0x111dd758)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8c0ac")),}}}
#[inline]
pub(super) unsafe fn f_10f8e020(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8e020=>{
// 0x10f8e020 ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e024 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8e020")),}}}
#[inline]
pub(super) unsafe fn f_10f8ff74(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8ff74=>{
// 0x10f8ff74 sub sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f8ff78 stp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff7c stp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff80 stp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff84 stp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff88 stp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff8c stp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8ff90 add x29, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f8ff94 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ff98 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8ff9c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f8ffa0 mov w23, w4
cpu.x[23]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f8ffa4 mov w27, w3
cpu.x[27]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f8ffa8 mov w28, w2
cpu.x[28]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8ffac ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ffb0 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f8ffb4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ffb8;super::dispatch(cpu,rt,target)?;}
pc=0x10f8ffb8;continue;
}
0x10f8ffb8=>{
// 0x10f8ffb8 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8ffbc b.ne #0x10f8ffd4
if cpu.condition(1){pc=0x10f8ffd4;continue;}pc=0x10f8ffc0;continue;
}
0x10f8ffc0=>{
// 0x10f8ffc0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ffc4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8ffc8 ldr x8, [x8, #0x90]
{let address=(cpu.x[8]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ffcc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ffd0;super::dispatch(cpu,rt,target)?;}
// 0x10f8ffd0 b #0x10f90070
pc=0x10f90070;continue;
}
0x10f8ffd4=>{
// 0x10f8ffd4 ldr x26, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10f8ffd8 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10f8ffdc bl #0x10e5aec8
cpu.x[30]=0x10f8ffe0;super::chunk11::f_10e5aec8(cpu,rt,0x10e5aec8)?;
pc=0x10f8ffe0;continue;
}
0x10f8ffe0=>{
// 0x10f8ffe0 tbz w0, #0, #0x10f90000
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90000;continue;}pc=0x10f8ffe4;continue;
}
0x10f8ffe4=>{
// 0x10f8ffe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8ffe8 bl #0x10f94124
cpu.x[30]=0x10f8ffec;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10f8ffec;continue;
}
0x10f8ffec=>{
// 0x10f8ffec mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8fff0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8fff4 bl #0x10f94124
cpu.x[30]=0x10f8fff8;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10f8fff8;continue;
}
0x10f8fff8=>{
// 0x10f8fff8 cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8fffc b.eq #0x10f90070
if cpu.condition(0){pc=0x10f90070;continue;}pc=0x10f90000;continue;
}
0x10f90000=>{
// 0x10f90000 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90004 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90008 ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9000c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f90010;super::dispatch(cpu,rt,target)?;}
pc=0x10f90010;continue;
}
0x10f90010=>{
// 0x10f90010 tbz w0, #0, #0x10f90044
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90044;continue;}pc=0x10f90014;continue;
}
0x10f90014=>{
// 0x10f90014 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90018 bl #0x10f93d2c
cpu.x[30]=0x10f9001c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f9001c bl #0x10d36e90
cpu.x[30]=0x10f90020;super::chunk6::f_10d36e90(cpu,rt,0x10d36e90)?;
// 0x10f90020 tbz w0, #0, #0x10f90044
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90044;continue;}pc=0x10f90024;continue;
}
0x10f90024=>{
// 0x10f90024 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90028 bl #0x10fa3348
cpu.x[30]=0x10f9002c;super::chunk4::f_10fa3348(cpu,rt,0x10fa3348)?;
// 0x10f9002c tbz w0, #0, #0x10f90044
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90044;continue;}pc=0x10f90030;continue;
}
0x10f90030=>{
// 0x10f90030 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90034 bl #0x10f93d2c
cpu.x[30]=0x10f90038;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f90038 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f9003c bl #0x10d3c2ac
cpu.x[30]=0x10f90040;super::dispatch(cpu,rt,0x10d3c2ac)?;
// 0x10f90040 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
pc=0x10f90044;continue;
}
0x10f90044=>{
// 0x10f90044 mov x24, x19
cpu.x[24]=(((cpu.x[19]))) as u64;
// 0x10f90048 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9004c ldr x9, [x24, #0x60]!
{let address=(cpu.x[24]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;cpu.x[24]=(address) as u64;}
// 0x10f90050 ldr w10, [x24, #0xc]
{let address=(cpu.x[24]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f90054 sub x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10f90058;continue;
}
0x10f90058=>{
// 0x10f90058 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9005c b.lt #0x10f90094
if cpu.condition(11){pc=0x10f90094;continue;}pc=0x10f90060;continue;
}
0x10f90060=>{
// 0x10f90060 ldr w11, [x9, x10, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f90064 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f90068 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9006c b.ne #0x10f90058
if cpu.condition(1){pc=0x10f90058;continue;}pc=0x10f90070;continue;
}
0x10f90070=>{
// 0x10f90070 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
pc=0x10f90074;continue;
}
0x10f90074=>{
// 0x10f90074 ldp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f90078 ldp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9007c ldp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f90080 ldp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f90084 ldp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f90088 ldp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f9008c add sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f90090 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f90094=>{
// 0x10f90094 ldrb w9, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f90098 cbz w9, #0x10f900dc
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f900dc;continue;}pc=0x10f9009c;continue;
}
0x10f9009c=>{
// 0x10f9009c mov x10, x19
cpu.x[10]=(((cpu.x[19]))) as u64;
pc=0x10f900a0;continue;
}
0x10f900a0=>{
// 0x10f900a0 mov x9, x10
cpu.x[9]=(((cpu.x[10]))) as u64;
// 0x10f900a4 ldr x10, [x10, #0x150]
{let address=(cpu.x[10]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f900a8 cbnz x10, #0x10f900a0
if (cpu.x[10]) != 0 {pc=0x10f900a0;continue;}pc=0x10f900ac;continue;
}
0x10f900ac=>{
// 0x10f900ac ldr x9, [x9, #0x168]
{let address=(cpu.x[9]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f900b0 cbz x9, #0x10f900dc
if (cpu.x[9]) == 0 {pc=0x10f900dc;continue;}pc=0x10f900b4;continue;
}
0x10f900b4=>{
// 0x10f900b4 ldr x11, [x9, #0x10]
{let address=(cpu.x[9]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f900b8 ldr w10, [x9, #0x1c]
{let address=(cpu.x[9]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f900bc sub x9, x11, #4
{let a=((cpu.x[11]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10f900c0;continue;
}
0x10f900c0=>{
// 0x10f900c0 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f900c4 b.lt #0x10f900dc
if cpu.condition(11){pc=0x10f900dc;continue;}pc=0x10f900c8;continue;
}
0x10f900c8=>{
// 0x10f900c8 ldr w11, [x9, x10, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f900cc sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f900d0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f900d4 b.ne #0x10f900c0
if cpu.condition(1){pc=0x10f900c0;continue;}pc=0x10f900d8;continue;
}
0x10f900d8=>{
// 0x10f900d8 b #0x10f90070
pc=0x10f90070;continue;
}
0x10f900dc=>{
// 0x10f900dc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f900e0 ldrb w22, [x26, #0x185]
{let address=(cpu.x[26]).wrapping_add(0x185u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f900e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f900e8 ldrb w21, [x26, #0x186]
{let address=(cpu.x[26]).wrapping_add(0x186u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f900ec ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f900f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f900f4;super::dispatch(cpu,rt,target)?;}
pc=0x10f900f4;continue;
}
0x10f900f4=>{
// 0x10f900f4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f900f8 b.ne #0x10f90160
if cpu.condition(1){pc=0x10f90160;continue;}pc=0x10f900fc;continue;
}
0x10f900fc=>{
// 0x10f900fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90100 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90104 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f90108 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f9010c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90110 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f90114;super::dispatch(cpu,rt,target)?;}
pc=0x10f90114;continue;
}
0x10f90114=>{
// 0x10f90114 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90118 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f9011c tbz w8, #0, #0x10f90074
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90074;continue;}pc=0x10f90120;continue;
}
0x10f90120=>{
// 0x10f90120 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90124 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f90128 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f9012c bl #0x10f817b0
cpu.x[30]=0x10f90130;super::chunk12::f_10f817b0(cpu,rt,0x10f817b0)?;
pc=0x10f90130;continue;
}
0x10f90130=>{
// 0x10f90130 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90134 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f90138 tbnz w8, #0, #0x10f90074
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f90074;continue;}pc=0x10f9013c;continue;
}
0x10f9013c=>{
// 0x10f9013c ldr w8, [x20, #0x1a4]
{let address=(cpu.x[20]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90140 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90144 b.lt #0x10f90574
if cpu.condition(11){pc=0x10f90574;continue;}pc=0x10f90148;continue;
}
0x10f90148=>{
// 0x10f90148 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9014c sub x1, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10f90150 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f90154 stur w8, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f90158 bl #0x10d374e8
cpu.x[30]=0x10f9015c;super::dispatch(cpu,rt,0x10d374e8)?;
// 0x10f9015c b #0x10f90070
pc=0x10f90070;continue;
}
0x10f90160=>{
// 0x10f90160 stur xzr, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f90164;continue;
}
0x10f90164=>{
// 0x10f90164 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90168 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9016c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90170 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f90174;super::dispatch(cpu,rt,target)?;}
pc=0x10f90174;continue;
}
0x10f90174=>{
// 0x10f90174 cbz x0, #0x10f90074
if (cpu.x[0]) == 0 {pc=0x10f90074;continue;}pc=0x10f90178;continue;
}
0x10f90178=>{
// 0x10f90178 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f9017c ldr x21, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f90180 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90184 bl #0x10f94298
cpu.x[30]=0x10f90188;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f90188;continue;
}
0x10f90188=>{
// 0x10f90188 sub w28, w0, w28
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[28]=(result) as u32 as u64;}
// 0x10f9018c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90190 bl #0x10f942a0
cpu.x[30]=0x10f90194;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f90194;continue;
}
0x10f90194=>{
// 0x10f90194 ldr x8, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90198 sub w2, w0, w27
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10f9019c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f901a0 stp w28, w2, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[28] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f901a4 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f901a8 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f901ac mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f901b0 ldr x9, [x9, #0xe8]
{let address=(cpu.x[9]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f901b4 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10f901b8;super::dispatch(cpu,rt,target)?;}
pc=0x10f901b8;continue;
}
0x10f901b8=>{
// 0x10f901b8 ldr w1, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f901bc mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10f901c0 bl #0x10e5ae54
cpu.x[30]=0x10f901c4;super::chunk9::f_10e5ae54(cpu,rt,0x10e5ae54)?;
pc=0x10f901c4;continue;
}
0x10f901c4=>{
// 0x10f901c4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f901c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f901cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f901d0 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f901d4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f901d8;super::dispatch(cpu,rt,target)?;}
pc=0x10f901d8;continue;
}
0x10f901d8=>{
// 0x10f901d8 tbz w0, #0, #0x10f901ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f901ec;continue;}pc=0x10f901dc;continue;
}
0x10f901dc=>{
// 0x10f901dc ldr w1, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f901e0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f901e4 bl #0x10e5aed8
cpu.x[30]=0x10f901e8;super::chunk6::f_10e5aed8(cpu,rt,0x10e5aed8)?;
// 0x10f901e8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10f901ec;continue;
}
0x10f901ec=>{
// 0x10f901ec ldur x22, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f901f0 ldr x27, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10f901f4 mov x28, x21
cpu.x[28]=(((cpu.x[21]))) as u64;
// 0x10f901f8 cbz x27, #0x10f9022c
if (cpu.x[27]) == 0 {pc=0x10f9022c;continue;}pc=0x10f901fc;continue;
}
0x10f901fc=>{
// 0x10f901fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90200 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90204 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90208 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9020c;super::dispatch(cpu,rt,target)?;}
// 0x10f9020c ldr w4, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[4]=(v0) as u32 as u64;}
// 0x10f90210 and w3, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10f90214 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10f90218 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f9021c mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f90220 mov x5, x20
cpu.x[5]=(((cpu.x[20]))) as u64;
// 0x10f90224 bl #0x10f77e90
cpu.x[30]=0x10f90228;super::chunk12::f_10f77e90(cpu,rt,0x10f77e90)?;
// 0x10f90228 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10f9022c;continue;
}
0x10f9022c=>{
// 0x10f9022c ldrb w8, [x28, #0x231]
{let address=(cpu.x[28]).wrapping_add(0x231u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90230 stur w8, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f90234 cbz w8, #0x10f9026c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9026c;continue;}pc=0x10f90238;continue;
}
0x10f90238=>{
// 0x10f90238 ldr x2, [x28, #0x120]
{let address=(cpu.x[28]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f9023c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90240 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f90244 bl #0x10f8f890
cpu.x[30]=0x10f90248;super::chunk3::f_10f8f890(cpu,rt,0x10f8f890)?;
// 0x10f90248 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9024c mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f90250 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f90254 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f90258 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10f9025c mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10f90260 bl #0x10f94824
cpu.x[30]=0x10f90264;super::chunk5::f_10f94824(cpu,rt,0x10f94824)?;
// 0x10f90264 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10f90268 csel w26, w26, wzr, ne
cpu.x[26]=(if cpu.condition(1){((cpu.x[26] as u32 as u64))}else{((0u64))}) as u32 as u64;
pc=0x10f9026c;continue;
}
0x10f9026c=>{
// 0x10f9026c ldp w9, w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f90270 ldr w21, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f90274 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90278 b.ne #0x10f902a0
if cpu.condition(1){pc=0x10f902a0;continue;}pc=0x10f9027c;continue;
}
0x10f9027c=>{
// 0x10f9027c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f90280 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10f90284 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90288 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9028c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90290 b.ge #0x10f902a0
if cpu.condition(10){pc=0x10f902a0;continue;}pc=0x10f90294;continue;
}
0x10f90294=>{
// 0x10f90294 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f90298 bl #0x1078f4d4
cpu.x[30]=0x10f9029c;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10f9029c;continue;
}
0x10f9029c=>{
// 0x10f9029c ldr w8, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f902a0;continue;
}
0x10f902a0=>{
// 0x10f902a0 ldr x9, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f902a4 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f902a8 str w10, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f902ac str w21, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f902b0 cbz x22, #0x10f90364
if (cpu.x[22]) == 0 {pc=0x10f90364;continue;}pc=0x10f902b4;continue;
}
0x10f902b4=>{
// 0x10f902b4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f902b8 bl #0x10f7e050
cpu.x[30]=0x10f902bc;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10f902bc;continue;
}
0x10f902bc=>{
// 0x10f902bc bl #0x10de0710
cpu.x[30]=0x10f902c0;super::chunk15::f_10de0710(cpu,rt,0x10de0710)?;
pc=0x10f902c0;continue;
}
0x10f902c0=>{
// 0x10f902c0 tbz w0, #0, #0x10f90364
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90364;continue;}pc=0x10f902c4;continue;
}
0x10f902c4=>{
// 0x10f902c4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f902c8 str x28, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[28];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f902cc bl #0x10f823a0
cpu.x[30]=0x10f902d0;super::chunk13::f_10f823a0(cpu,rt,0x10f823a0)?;
// 0x10f902d0 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f902d4 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10f902d8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f902dc b.lt #0x10f90348
if cpu.condition(11){pc=0x10f90348;continue;}pc=0x10f902e0;continue;
}
0x10f902e0=>{
// 0x10f902e0 mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10f902e4 mov w21, #5
cpu.x[21]=(((0x5u64))) as u32 as u64;
// 0x10f902e8 b #0x10f9030c
pc=0x10f9030c;continue;
}
0x10f902ec=>{
// 0x10f902ec ldr x9, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f902f0 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f902f4 add x28, x28, #1
{let a=((cpu.x[28]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10f902f8 str w10, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f902fc str w22, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f90300 ldrsw x8, [x27, #0xc]
{let address=(cpu.x[27]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f90304 cmp x28, x8
{let a=((cpu.x[28]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f90308 b.ge #0x10f90348
if cpu.condition(10){pc=0x10f90348;continue;}pc=0x10f9030c;continue;
}
0x10f9030c=>{
// 0x10f9030c ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90310 ldr x9, [x8, x28, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[28])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f90314 ldp w10, w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f90318 ldr w22, [x9, #8]
{let address=(cpu.x[9]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f9031c cmp w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90320 b.ne #0x10f902ec
if cpu.condition(1){pc=0x10f902ec;continue;}pc=0x10f90324;continue;
}
0x10f90324=>{
// 0x10f90324 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f90328 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9032c csel w1, w21, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f90330 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90334 b.ge #0x10f902ec
if cpu.condition(10){pc=0x10f902ec;continue;}pc=0x10f90338;continue;
}
0x10f90338=>{
// 0x10f90338 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f9033c bl #0x1078f4d4
cpu.x[30]=0x10f90340;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10f90340 ldr w8, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90344 b #0x10f902ec
pc=0x10f902ec;continue;
}
0x10f90348=>{
// 0x10f90348 ldr x0, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9034c cbz x0, #0x10f90354
if (cpu.x[0]) == 0 {pc=0x10f90354;continue;}pc=0x10f90350;continue;
}
0x10f90350=>{
// 0x10f90350 bl #0x1187d360
cpu.x[30]=0x10f90354;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f90354;continue;
}
0x10f90354=>{
// 0x10f90354 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10f90358 bl #0x1187d2c0
cpu.x[30]=0x10f9035c;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f9035c ldur x22, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f90360 ldr x28, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[28]=(v0) as u64;}
pc=0x10f90364;continue;
}
0x10f90364=>{
// 0x10f90364 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f90368 bl #0x10fca234
cpu.x[30]=0x10f9036c;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10f9036c;continue;
}
0x10f9036c=>{
// 0x10f9036c ldr w27, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10f90370 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90374 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90378 bl #0x10f93d3c
cpu.x[30]=0x10f9037c;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f9037c;continue;
}
0x10f9037c=>{
// 0x10f9037c ldr x6, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[6]=(v0) as u64;}
// 0x10f90380 mov w5, w0
cpu.x[5]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90384 sub x7, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[7]=(result) as u64;}
// 0x10f90388 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f9038c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f90390 mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f90394 mov w3, w27
cpu.x[3]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f90398 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f9039c strb wzr, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f903a0 strb wzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f903a4 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f903a8 bl #0x10fca3d4
cpu.x[30]=0x10f903ac;super::chunk7::f_10fca3d4(cpu,rt,0x10fca3d4)?;
pc=0x10f903ac;continue;
}
0x10f903ac=>{
// 0x10f903ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f903b0 bl #0x10f89114
cpu.x[30]=0x10f903b4;super::chunk6::f_10f89114(cpu,rt,0x10f89114)?;
pc=0x10f903b4;continue;
}
0x10f903b4=>{
// 0x10f903b4 bl #0x10e16208
cpu.x[30]=0x10f903b8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10f903b8;continue;
}
0x10f903b8=>{
// 0x10f903b8 ldrb w8, [x0, #0x8b4]
{let address=(cpu.x[0]).wrapping_add(0x8b4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f903bc cbz w8, #0x10f903d4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f903d4;continue;}pc=0x10f903c0;continue;
}
0x10f903c0=>{
// 0x10f903c0 ldp w10, w9, [x25, #0x28]
{let address=(cpu.x[25]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10f903c4 ldr w8, [x25, #0x10]
{let address=(cpu.x[25]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f903c8 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f903cc cmp w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f903d0 b #0x10f903d8
pc=0x10f903d8;continue;
}
0x10f903d4=>{
// 0x10f903d4 cmp w26, w24
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10f903d8;continue;
}
0x10f903d8=>{
// 0x10f903d8 cset w8, ge
cpu.x[8]=((cpu.condition(10)) as u64) as u32 as u64;
// 0x10f903dc cbz w8, #0x10f903f4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f903f4;continue;}pc=0x10f903e0;continue;
}
0x10f903e0=>{
// 0x10f903e0 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f903e4 cbz w8, #0x10f903f4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f903f4;continue;}pc=0x10f903e8;continue;
}
0x10f903e8=>{
// 0x10f903e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f903ec mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f903f0 bl #0x10f929d8
cpu.x[30]=0x10f903f4;super::dispatch(cpu,rt,0x10f929d8)?;
pc=0x10f903f4;continue;
}
0x10f903f4=>{
// 0x10f903f4 ldr x2, [x28, #0x2c0]
{let address=(cpu.x[28]).wrapping_add(0x2c0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f903f8 cbz x2, #0x10f90444
if (cpu.x[2]) == 0 {pc=0x10f90444;continue;}pc=0x10f903fc;continue;
}
0x10f903fc=>{
// 0x10f903fc ldr x9, [x19, #0x208]
{let address=(cpu.x[19]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f90400 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90404 ldr w10, [x19, #0x214]
{let address=(cpu.x[19]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f90408 sub x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f9040c stur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f90410;continue;
}
0x10f90410=>{
// 0x10f90410 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90414 b.lt #0x10f9042c
if cpu.condition(11){pc=0x10f9042c;continue;}pc=0x10f90418;continue;
}
0x10f90418=>{
// 0x10f90418 ldr w11, [x9, x10, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f9041c sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f90420 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90424 b.ne #0x10f90410
if cpu.condition(1){pc=0x10f90410;continue;}pc=0x10f90428;continue;
}
0x10f90428=>{
// 0x10f90428 b #0x10f90444
pc=0x10f90444;continue;
}
0x10f9042c=>{
// 0x10f9042c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90430 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f90434 bl #0x10f89954
cpu.x[30]=0x10f90438;super::chunk11::f_10f89954(cpu,rt,0x10f89954)?;
// 0x10f90438 add x0, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9043c sub x1, x29, #0xc
{let a=((cpu.x[29]));let b=((0xcu64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10f90440 bl #0x10b18df0
cpu.x[30]=0x10f90444;super::dispatch(cpu,rt,0x10b18df0)?;
pc=0x10f90444;continue;
}
0x10f90444=>{
// 0x10f90444 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90448 bl #0x10f942c8
cpu.x[30]=0x10f9044c;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f9044c;continue;
}
0x10f9044c=>{
// 0x10f9044c cbz x0, #0x10f904cc
if (cpu.x[0]) == 0 {pc=0x10f904cc;continue;}pc=0x10f90450;continue;
}
0x10f90450=>{
// 0x10f90450 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f90454 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f90458 bl #0x10e5aeb0
cpu.x[30]=0x10f9045c;super::chunk6::f_10e5aeb0(cpu,rt,0x10e5aeb0)?;
pc=0x10f9045c;continue;
}
0x10f9045c=>{
// 0x10f9045c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f90460 b.lt #0x10f904cc
if cpu.condition(11){pc=0x10f904cc;continue;}pc=0x10f90464;continue;
}
0x10f90464=>{
// 0x10f90464 ldrb w21, [x28, #0x2d8]
{let address=(cpu.x[28]).wrapping_add(0x2d8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f90468 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9046c bl #0x10f94298
cpu.x[30]=0x10f90470;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f90470 cbz w21, #0x10f905f0
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10f905f0;continue;}pc=0x10f90474;continue;
}
0x10f90474=>{
// 0x10f90474 ldr w8, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90478 sub w25, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10f9047c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90480 bl #0x10f942a0
cpu.x[30]=0x10f90484;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f90484 ldr w8, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90488 sub w26, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10f9048c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f90490 bl #0x10e5aeb0
cpu.x[30]=0x10f90494;super::chunk6::f_10e5aeb0(cpu,rt,0x10e5aeb0)?;
// 0x10f90494 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90498 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f9049c bl #0x10e5aeb8
cpu.x[30]=0x10f904a0;super::dispatch(cpu,rt,0x10e5aeb8)?;
// 0x10f904a0 and w5, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10f904a4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f904a8 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f904ac mov w2, w26
cpu.x[2]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f904b0 mov w3, w27
cpu.x[3]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f904b4 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f904b8 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f904bc mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f904c0 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f904c4 bl #0x10fcfe18
cpu.x[30]=0x10f904c8;super::dispatch(cpu,rt,0x10fcfe18)?;
// 0x10f904c8 b #0x10f90640
pc=0x10f90640;continue;
}
0x10f904cc=>{
// 0x10f904cc mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f904d0 bl #0x10e5aeb0
cpu.x[30]=0x10f904d4;super::chunk6::f_10e5aeb0(cpu,rt,0x10e5aeb0)?;
pc=0x10f904d4;continue;
}
0x10f904d4=>{
// 0x10f904d4 cmp w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f904d8 b.lt #0x10f90640
if cpu.condition(11){pc=0x10f90640;continue;}pc=0x10f904dc;continue;
}
0x10f904dc=>{
// 0x10f904dc cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f904e0 b.lt #0x10f90640
if cpu.condition(11){pc=0x10f90640;continue;}pc=0x10f904e4;continue;
}
0x10f904e4=>{
// 0x10f904e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f904e8 bl #0x10f942f0
cpu.x[30]=0x10f904ec;super::chunk11::f_10f942f0(cpu,rt,0x10f942f0)?;
// 0x10f904ec cbz x0, #0x10f90640
if (cpu.x[0]) == 0 {pc=0x10f90640;continue;}pc=0x10f904f0;continue;
}
0x10f904f0=>{
// 0x10f904f0 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f904f4 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f904f8 bl #0x10e5aeb8
cpu.x[30]=0x10f904fc;super::dispatch(cpu,rt,0x10e5aeb8)?;
// 0x10f904fc and w1, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10f90500 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f90504 bl #0x10fcfff0
cpu.x[30]=0x10f90508;super::dispatch(cpu,rt,0x10fcfff0)?;
// 0x10f90508 tbz w0, #0, #0x10f90640
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90640;continue;}pc=0x10f9050c;continue;
}
0x10f9050c=>{
// 0x10f9050c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90510 bl #0x10f95b64
cpu.x[30]=0x10f90514;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
// 0x10f90514 cbz x0, #0x10f9055c
if (cpu.x[0]) == 0 {pc=0x10f9055c;continue;}pc=0x10f90518;continue;
}
0x10f90518=>{
// 0x10f90518 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f9051c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90520 bl #0x10f95bec
cpu.x[30]=0x10f90524;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
// 0x10f90524 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90528 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9052c bl #0x10d3cac8
cpu.x[30]=0x10f90530;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
// 0x10f90530 cbz x0, #0x10f9055c
if (cpu.x[0]) == 0 {pc=0x10f9055c;continue;}pc=0x10f90534;continue;
}
0x10f90534=>{
// 0x10f90534 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f90538 ldr x23, [x23, #0x10]
{let address=(cpu.x[23]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f9053c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90540 bl #0x10f95c08
cpu.x[30]=0x10f90544;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
// 0x10f90544 ldrb w3, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f90548 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9054c mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f90550 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f90554 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10f90558 bl #0x10d549b8
cpu.x[30]=0x10f9055c;super::dispatch(cpu,rt,0x10d549b8)?;
pc=0x10f9055c;continue;
}
0x10f9055c=>{
// 0x10f9055c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90560 mov w1, #0xa
cpu.x[1]=(((0xau64))) as u32 as u64;
// 0x10f90564 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10f90568 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10f9056c bl #0x10f94078
cpu.x[30]=0x10f90570;super::chunk8::f_10f94078(cpu,rt,0x10f94078)?;
// 0x10f90570 b #0x10f90640
pc=0x10f90640;continue;
}
0x10f90574=>{
// 0x10f90574 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90578 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9057c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90580 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f90584;super::dispatch(cpu,rt,target)?;}
pc=0x10f90584;continue;
}
0x10f90584=>{
// 0x10f90584 cbnz w22, #0x10f9058c
if (cpu.x[22] as u32 as u64) != 0 {pc=0x10f9058c;continue;}pc=0x10f90588;continue;
}
0x10f90588=>{
// 0x10f90588 tbnz w0, #0, #0x10f90070
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f90070;continue;}pc=0x10f9058c;continue;
}
0x10f9058c=>{
// 0x10f9058c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90590 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90594 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90598 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9059c;super::dispatch(cpu,rt,target)?;}
pc=0x10f9059c;continue;
}
0x10f9059c=>{
// 0x10f9059c cbnz w21, #0x10f905a4
if (cpu.x[21] as u32 as u64) != 0 {pc=0x10f905a4;continue;}pc=0x10f905a0;continue;
}
0x10f905a0=>{
// 0x10f905a0 tbz w0, #0, #0x10f90070
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f90070;continue;}pc=0x10f905a4;continue;
}
0x10f905a4=>{
// 0x10f905a4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f905a8 bl #0x10f817a8
cpu.x[30]=0x10f905ac;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10f905ac;continue;
}
0x10f905ac=>{
// 0x10f905ac cbnz w22, #0x10f905b8
if (cpu.x[22] as u32 as u64) != 0 {pc=0x10f905b8;continue;}pc=0x10f905b0;continue;
}
0x10f905b0=>{
// 0x10f905b0 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f905b4 b.eq #0x10f90070
if cpu.condition(0){pc=0x10f90070;continue;}pc=0x10f905b8;continue;
}
0x10f905b8=>{
// 0x10f905b8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f905bc bl #0x10f817a8
cpu.x[30]=0x10f905c0;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10f905c0;continue;
}
0x10f905c0=>{
// 0x10f905c0 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f905c4 stur x20, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f905c8 b.ne #0x10f90164
if cpu.condition(1){pc=0x10f90164;continue;}pc=0x10f905cc;continue;
}
0x10f905cc=>{
// 0x10f905cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f905d0 bl #0x10f7e050
cpu.x[30]=0x10f905d4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10f905d4 bl #0x10de0604
cpu.x[30]=0x10f905d8;super::chunk12::f_10de0604(cpu,rt,0x10de0604)?;
// 0x10f905d8 stur x20, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f905dc cbnz w22, #0x10f90164
if (cpu.x[22] as u32 as u64) != 0 {pc=0x10f90164;continue;}pc=0x10f905e0;continue;
}
0x10f905e0=>{
// 0x10f905e0 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f905e4 stur x20, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f905e8 b.gt #0x10f90070
if cpu.condition(12){pc=0x10f90070;continue;}pc=0x10f905ec;continue;
}
0x10f905ec=>{
// 0x10f905ec b #0x10f90164
pc=0x10f90164;continue;
}
0x10f905f0=>{
// 0x10f905f0 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f905f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f905f8 bl #0x10f942a0
cpu.x[30]=0x10f905fc;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f905fc mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90600 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f90604 bl #0x10e5aeb0
cpu.x[30]=0x10f90608;super::chunk6::f_10e5aeb0(cpu,rt,0x10e5aeb0)?;
// 0x10f90608 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9060c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f90610 bl #0x10e5aeb8
cpu.x[30]=0x10f90614;super::dispatch(cpu,rt,0x10e5aeb8)?;
// 0x10f90614 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f90618 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f9061c and w5, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10f90620 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f90624 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f90628 mov w2, w26
cpu.x[2]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f9062c mov w3, w27
cpu.x[3]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f90630 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f90634 strb wzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f90638 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f9063c bl #0x10fcfac4
cpu.x[30]=0x10f90640;super::chunk6::f_10fcfac4(cpu,rt,0x10fcfac4)?;
pc=0x10f90640;continue;
}
0x10f90640=>{
// 0x10f90640 ldrb w8, [x28, #0x1a2]
{let address=(cpu.x[28]).wrapping_add(0x1a2u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90644 cbz w8, #0x10f90690
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f90690;continue;}pc=0x10f90648;continue;
}
0x10f90648=>{
// 0x10f90648 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9064c bl #0x10f94298
cpu.x[30]=0x10f90650;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f90650;continue;
}
0x10f90650=>{
// 0x10f90650 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f90654 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90658 bl #0x10f94298
cpu.x[30]=0x10f9065c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f9065c;continue;
}
0x10f9065c=>{
// 0x10f9065c sub w23, w23, w0
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10f90660 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f90664 bl #0x10f942a0
cpu.x[30]=0x10f90668;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f90668;continue;
}
0x10f90668=>{
// 0x10f90668 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9066c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f90670 bl #0x10f942a0
cpu.x[30]=0x10f90674;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f90674;continue;
}
0x10f90674=>{
// 0x10f90674 ldr x8, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f90678 sub w2, w25, w0
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10f9067c mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f90680 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f90684 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f90688 ldr x9, [x9, #0xf0]
{let address=(cpu.x[9]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9068c blr x9
{let target=cpu.x[9];cpu.x[30]=0x10f90690;super::dispatch(cpu,rt,target)?;}
pc=0x10f90690;continue;
}
0x10f90690=>{
// 0x10f90690 ldur w8, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f90694 cbnz w8, #0x10f906a8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f906a8;continue;}pc=0x10f90698;continue;
}
0x10f90698=>{
// 0x10f90698 ldr x2, [x28, #0x120]
{let address=(cpu.x[28]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f9069c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906a0 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f906a4 bl #0x10f8f890
cpu.x[30]=0x10f906a8;super::chunk3::f_10f8f890(cpu,rt,0x10f8f890)?;
pc=0x10f906a8;continue;
}
0x10f906a8=>{
// 0x10f906a8 cmp w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f906ac stur xzr, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f906b0 b.lt #0x10f90070
if cpu.condition(11){pc=0x10f90070;continue;}pc=0x10f906b4;continue;
}
0x10f906b4=>{
// 0x10f906b4 ldrb w8, [x28, #0x1a4]
{let address=(cpu.x[28]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f906b8 cbz w8, #0x10f90070
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f90070;continue;}pc=0x10f906bc;continue;
}
0x10f906bc=>{
// 0x10f906bc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906c0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f906c4 bl #0x10f91790
cpu.x[30]=0x10f906c8;super::dispatch(cpu,rt,0x10f91790)?;
// 0x10f906c8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f906cc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906d0 bl #0x10f94298
cpu.x[30]=0x10f906d4;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f906d4 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f906d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906dc bl #0x10f942a0
cpu.x[30]=0x10f906e0;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f906e0 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f906e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906e8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f906ec mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f906f0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f906f4 bl #0x10f8f978
cpu.x[30]=0x10f906f8;super::chunk14::f_10f8f978(cpu,rt,0x10f8f978)?;
// 0x10f906f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f906fc bl #0x10f8eed8
cpu.x[30]=0x10f90700;super::chunk9::f_10f8eed8(cpu,rt,0x10f8eed8)?;
// 0x10f90700 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f90704 b #0x10f90074
pc=0x10f90074;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8ff74")),}}}
#[inline]
pub(super) unsafe fn f_10f942c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f942c8=>{
// 0x10f942c8 ldrb w9, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f942cc mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f942d0 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f942d4 tbz w9, #1, #0x10f942ec
if ((cpu.x[9] as u32 as u64) & (1u64 << 1)) == 0 {pc=0x10f942ec;continue;}pc=0x10f942d8;continue;
}
0x10f942d8=>{
// 0x10f942d8 ldr w9, [x8, #0x24]
{let address=(cpu.x[8]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f942dc cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f942e0 b.lt #0x10f942ec
if cpu.condition(11){pc=0x10f942ec;continue;}pc=0x10f942e4;continue;
}
0x10f942e4=>{
// 0x10f942e4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f942e8 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f942ec;continue;
}
0x10f942ec=>{
// 0x10f942ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f942c8")),}}}
#[inline]
pub(super) unsafe fn f_10f97b48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f97b48=>{
// 0x10f97b48 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f97b4c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f97b50 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f97b54 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f97b58 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f97b5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f97b60 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f97b64 bl #0x10f93d3c
cpu.x[30]=0x10f97b68;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f97b68;continue;
}
0x10f97b68=>{
// 0x10f97b68 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f97b6c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f97b70 b.lt #0x10f97bd4
if cpu.condition(11){pc=0x10f97bd4;continue;}pc=0x10f97b74;continue;
}
0x10f97b74=>{
// 0x10f97b74 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f97b78 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f97b7c mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10f97b80 b #0x10f97b98
pc=0x10f97b98;continue;
}
0x10f97b84=>{
// 0x10f97b84 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f97b88 bl #0x10f8584c
cpu.x[30]=0x10f97b8c;super::chunk15::f_10f8584c(cpu,rt,0x10f8584c)?;
pc=0x10f97b8c;continue;
}
0x10f97b8c=>{
// 0x10f97b8c add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10f97b90 cmp x22, x23
{let a=((cpu.x[22]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f97b94 b.eq #0x10f97bd4
if cpu.condition(0){pc=0x10f97bd4;continue;}pc=0x10f97b98;continue;
}
0x10f97b98=>{
// 0x10f97b98 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f97b9c ldr x21, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f97ba0 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f97ba4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f97ba8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f97bac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f97bb0;super::dispatch(cpu,rt,target)?;}
pc=0x10f97bb0;continue;
}
0x10f97bb0=>{
// 0x10f97bb0 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f97bb4 b.ne #0x10f97b8c
if cpu.condition(1){pc=0x10f97b8c;continue;}pc=0x10f97bb8;continue;
}
0x10f97bb8=>{
// 0x10f97bb8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f97bbc bl #0x10f93d3c
cpu.x[30]=0x10f97bc0;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f97bc0;continue;
}
0x10f97bc0=>{
// 0x10f97bc0 cmp w0, w20
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f97bc4 b.eq #0x10f97b84
if cpu.condition(0){pc=0x10f97b84;continue;}pc=0x10f97bc8;continue;
}
0x10f97bc8=>{
// 0x10f97bc8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f97bcc bl #0x10f85848
cpu.x[30]=0x10f97bd0;super::chunk15::f_10f85848(cpu,rt,0x10f85848)?;
pc=0x10f97bd0;continue;
}
0x10f97bd0=>{
// 0x10f97bd0 b #0x10f97b8c
pc=0x10f97b8c;continue;
}
0x10f97bd4=>{
// 0x10f97bd4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f97bd8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f97bdc ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f97be0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f97be4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f97b48")),}}}
#[inline]
pub(super) unsafe fn f_10f992e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f992e0=>{
// 0x10f992e0 ldr x0, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f992e4 cbz x0, #0x10f992ec
if (cpu.x[0]) == 0 {pc=0x10f992ec;continue;}pc=0x10f992e8;continue;
}
0x10f992e8=>{
// 0x10f992e8 b #0x10fd08cc
super::chunk4::f_10fd08cc(cpu,rt,0x10fd08cc)?;return Ok(());
}
0x10f992ec=>{
// 0x10f992ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f992e0")),}}}
#[inline]
pub(super) unsafe fn f_10f99478(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99478=>{
// 0x10f99478 mov w0, #4
cpu.x[0]=(((0x4u64))) as u32 as u64;
// 0x10f9947c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99478")),}}}
#[inline]
pub(super) unsafe fn f_10f99dc4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99dc4=>{
// 0x10f99dc4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f99dc8 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99dcc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f99dd0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f99dd4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f99dd8 adrp x8, #0x11936000
cpu.x[8]=(0x11936000u64) as u64;
// 0x10f99ddc add x8, x8, #0x4f8
{let a=((cpu.x[8]));let b=((0x4f8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99de0 ldr x0, [x20, #0x20]!
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x10f99de4 str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f99de8 cbz x0, #0x10f99df0
if (cpu.x[0]) == 0 {pc=0x10f99df0;continue;}pc=0x10f99dec;continue;
}
0x10f99dec=>{
// 0x10f99dec bl #0x1187d360
cpu.x[30]=0x10f99df0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f99df0;continue;
}
0x10f99df0=>{
// 0x10f99df0 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f99df4 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99df8 cbz x0, #0x10f99e00
if (cpu.x[0]) == 0 {pc=0x10f99e00;continue;}pc=0x10f99dfc;continue;
}
0x10f99dfc=>{
// 0x10f99dfc bl #0x1187d360
cpu.x[30]=0x10f99e00;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f99e00;continue;
}
0x10f99e00=>{
// 0x10f99e00 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f99e04 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f99e08 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f99e0c b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99dc4")),}}}
#[inline]
pub(super) unsafe fn f_10f9dec0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9dec0=>{
// 0x10f9dec0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9dec4 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9dec8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9decc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9ded0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9ded4 bl #0x10f93d2c
cpu.x[30]=0x10f9ded8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f9ded8;continue;
}
0x10f9ded8=>{
// 0x10f9ded8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f9dedc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9dee0 bl #0x10f93fc0
cpu.x[30]=0x10f9dee4;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f9dee4;continue;
}
0x10f9dee4=>{
// 0x10f9dee4 bl #0x112942b8
cpu.x[30]=0x10f9dee8;super::chunk12::f_112942b8(cpu,rt,0x112942b8)?;
pc=0x10f9dee8;continue;
}
0x10f9dee8=>{
// 0x10f9dee8 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9deec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9def0 bl #0x10f93fc0
cpu.x[30]=0x10f9def4;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f9def4;continue;
}
0x10f9def4=>{
// 0x10f9def4 bl #0x112942c0
cpu.x[30]=0x10f9def8;super::chunk6::f_112942c0(cpu,rt,0x112942c0)?;
pc=0x10f9def8;continue;
}
0x10f9def8=>{
// 0x10f9def8 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9defc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9df00 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f9df04 bl #0x10d33a9c
cpu.x[30]=0x10f9df08;super::chunk4::f_10d33a9c(cpu,rt,0x10d33a9c)?;
pc=0x10f9df08;continue;
}
0x10f9df08=>{
// 0x10f9df08 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9df0c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9df10 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9df14 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f9df18 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9df1c b #0x10d32c84
super::chunk15::f_10d32c84(cpu,rt,0x10d32c84)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9dec0")),}}}
#[inline]
pub(super) unsafe fn f_10fa57b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa57b8=>{
// 0x10fa57b8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa57bc stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa57c0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa57c4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fa57c8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fa57cc add x1, x0, #8
{let a=((cpu.x[0]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10fa57d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa57d4 bl #0x111dd4e8
cpu.x[30]=0x10fa57d8;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
pc=0x10fa57d8;continue;
}
0x10fa57d8=>{
// 0x10fa57d8 add x1, x20, #0x18
{let a=((cpu.x[20]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10fa57dc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa57e0 bl #0x111dd4e8
cpu.x[30]=0x10fa57e4;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
pc=0x10fa57e4;continue;
}
0x10fa57e4=>{
// 0x10fa57e4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa57e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa57ec ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa57f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa57f4;super::dispatch(cpu,rt,target)?;}
pc=0x10fa57f4;continue;
}
0x10fa57f4=>{
// 0x10fa57f4 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa57f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa57fc str w8, [x20], #0x28
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[20]=((cpu.x[20]).wrapping_add(((0x28u64)))) as u64;}
pc=0x10fa5800;continue;
}
0x10fa5800=>{
// 0x10fa5800 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fa5804 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa5808 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa580c b #0x111dd4e8
super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa57b8")),}}}
#[inline]
pub(super) unsafe fn f_10fa5810(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa5810=>{
// 0x10fa5810 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa5814 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5818 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa581c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa5820 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa5824 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fa5828 tbnz w1, #0x1f, #0x10fa58d0
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fa58d0;continue;}pc=0x10fa582c;continue;
}
0x10fa582c=>{
// 0x10fa582c ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa5830 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5834 b.le #0x10fa58d0
if cpu.condition(13){pc=0x10fa58d0;continue;}pc=0x10fa5838;continue;
}
0x10fa5838=>{
// 0x10fa5838 ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa583c ldr w0, [x9, w1, uxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fa5840 cmn w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fa5844 b.eq #0x10fa58d0
if cpu.condition(0){pc=0x10fa58d0;continue;}pc=0x10fa5848;continue;
}
0x10fa5848=>{
// 0x10fa5848 ldr x8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa584c ldr w10, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa5850 sub x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
pc=0x10fa5854;continue;
}
0x10fa5854=>{
// 0x10fa5854 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5858 b.lt #0x10fa5870
if cpu.condition(11){pc=0x10fa5870;continue;}pc=0x10fa585c;continue;
}
0x10fa585c=>{
// 0x10fa585c ldr w11, [x8, x10, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fa5860 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10fa5864 cmp w11, w0
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5868 b.ne #0x10fa5854
if cpu.condition(1){pc=0x10fa5854;continue;}pc=0x10fa586c;continue;
}
0x10fa586c=>{
// 0x10fa586c b #0x10fa58c8
pc=0x10fa58c8;continue;
}
0x10fa5870=>{
// 0x10fa5870 ldp w8, w10, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x10fa5874 cmp w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5878 b.ne #0x10fa58b8
if cpu.condition(1){pc=0x10fa58b8;continue;}pc=0x10fa587c;continue;
}
0x10fa587c=>{
// 0x10fa587c lsl w8, w10, #1
cpu.x[8]=(((((cpu.x[10] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa5880 mov w11, #5
cpu.x[11]=(((0x5u64))) as u32 as u64;
// 0x10fa5884 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5888 csel w8, w11, w8, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fa588c cmp w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5890 b.ge #0x10fa58b8
if cpu.condition(10){pc=0x10fa58b8;continue;}pc=0x10fa5894;continue;
}
0x10fa5894=>{
// 0x10fa5894 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa5898 add x0, x19, #0x18
{let a=((cpu.x[19]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa589c mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fa58a0 mov w1, w8
cpu.x[1]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10fa58a4 bl #0x1078f4d4
cpu.x[30]=0x10fa58a8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10fa58a8;continue;
}
0x10fa58a8=>{
// 0x10fa58a8 ldr w10, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa58ac ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa58b0 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fa58b4 mov w0, w20
cpu.x[0]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
pc=0x10fa58b8;continue;
}
0x10fa58b8=>{
// 0x10fa58b8 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa58bc add w11, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa58c0 str w11, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa58c4 str w0, [x8, w10, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[10] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fa58c8;continue;
}
0x10fa58c8=>{
// 0x10fa58c8 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fa58cc str w8, [x9, w1, uxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fa58d0;continue;
}
0x10fa58d0=>{
// 0x10fa58d0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa58d4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa58d8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa58dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa5810")),}}}
#[inline]
pub(super) unsafe fn f_10fa8824(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa8824=>{
// 0x10fa8824 b #0x10f73d08
super::chunk12::f_10f73d08(cpu,rt,0x10f73d08)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa8824")),}}}
#[inline]
pub(super) unsafe fn f_10fb792c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fb792c=>{
// 0x10fb792c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fb7930 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fb7934 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fb7938 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fb793c bl #0x10f74060
cpu.x[30]=0x10fb7940;super::chunk2::f_10f74060(cpu,rt,0x10f74060)?;
pc=0x10fb7940;continue;
}
0x10fb7940=>{
// 0x10fb7940 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fb7944 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fb7948 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fb794c b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fb792c")),}}}
#[inline]
pub(super) unsafe fn f_10fbb93c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbb93c=>{
// 0x10fbb93c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbb940 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbb944 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbb948 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fbb94c ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fbb950 bl #0x10f93d2c
cpu.x[30]=0x10fbb954;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fbb954;continue;
}
0x10fbb954=>{
// 0x10fbb954 ldr x0, [x0, #0x330]
{let address=(cpu.x[0]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fbb958 mov w1, #0xcc23
cpu.x[1]=(((0xcc23u64))) as u32 as u64;
// 0x10fbb95c add x2, x19, #0x60
{let a=((cpu.x[19]));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u64;}
// 0x10fbb960 movk w1, #0xc9d, lsl #16
cpu.x[1]=((cpu.x[1] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10fbb964 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10fbb968 bl #0x111fb104
cpu.x[30]=0x10fbb96c;super::chunk0::f_111fb104(cpu,rt,0x111fb104)?;
pc=0x10fbb96c;continue;
}
0x10fbb96c=>{
// 0x10fbb96c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbb970 str wzr, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbb974 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fbb978 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbb97c b #0x10f73c30
super::chunk7::f_10f73c30(cpu,rt,0x10f73c30)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbb93c")),}}}
#[inline]
pub(super) unsafe fn f_10fbbbcc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbbbcc=>{
// 0x10fbbbcc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbbbd0 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbbbd4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbbbd8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbbbdc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fbbbe0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fbbbe4 bl #0x10f73f5c
cpu.x[30]=0x10fbbbe8;super::chunk12::f_10f73f5c(cpu,rt,0x10f73f5c)?;
pc=0x10fbbbe8;continue;
}
0x10fbbbe8=>{
// 0x10fbbbe8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbbec ldr w1, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbbbf0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbbf4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbbf8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbbfc;super::dispatch(cpu,rt,target)?;}
pc=0x10fbbbfc;continue;
}
0x10fbbbfc=>{
// 0x10fbbbfc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10fbbc00;continue;
}
0x10fbbc00=>{
// 0x10fbbc00 ldr w21, [x20, #0x7c]
{let address=(cpu.x[20]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fbbc04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbc08 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc0c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fbbc10 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbc14;super::dispatch(cpu,rt,target)?;}
pc=0x10fbbc14;continue;
}
0x10fbbc14=>{
// 0x10fbbc14 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbc18 b.lt #0x10fbbc48
if cpu.condition(11){pc=0x10fbbc48;continue;}pc=0x10fbbc1c;continue;
}
0x10fbbc1c=>{
// 0x10fbbc1c lsl x21, x21, #2
cpu.x[21]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fbbc20 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10fbbc24;continue;
}
0x10fbbc24=>{
// 0x10fbbc24 ldr x8, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc28 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fbbc2c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbc30 ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbbc34 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc38 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbc3c;super::dispatch(cpu,rt,target)?;}
// 0x10fbbc3c add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10fbbc40 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fbbc44 b.ne #0x10fbbc24
if cpu.condition(1){pc=0x10fbbc24;continue;}pc=0x10fbbc48;continue;
}
0x10fbbc48=>{
// 0x10fbbc48 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbbc4c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fbbc50 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fbbc54 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbbbcc")),}}}
#[inline]
pub(super) unsafe fn f_10fbbd44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbbd44=>{
// 0x10fbbd44 ldr w8, [x1, #8]
{let address=(cpu.x[1]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fbbd48 mov w9, #0xcc23
cpu.x[9]=(((0xcc23u64))) as u32 as u64;
// 0x10fbbd4c movk w9, #0xc9d, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10fbbd50 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbd54 b.ne #0x10fbbd60
if cpu.condition(1){pc=0x10fbbd60;continue;}pc=0x10fbbd58;continue;
}
0x10fbbd58=>{
// 0x10fbbd58 sub x0, x0, #0x60
{let a=((cpu.x[0]));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10fbbd5c b #0x10fbb9f0
super::chunk2::f_10fbb9f0(cpu,rt,0x10fbb9f0)?;return Ok(());
}
0x10fbbd60=>{
// 0x10fbbd60 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbbd44")),}}}
#[inline]
pub(super) unsafe fn f_10fc0500(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc0500=>{
// 0x10fc0500 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc0504 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc0508 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc050c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fc0510 mov x1, x2
cpu.x[1]=(((cpu.x[2]))) as u64;
// 0x10fc0514 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc0518 bl #0x10fd0470
cpu.x[30]=0x10fc051c;super::chunk1::f_10fd0470(cpu,rt,0x10fd0470)?;
pc=0x10fc051c;continue;
}
0x10fc051c=>{
// 0x10fc051c adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fc0520 add x8, x8, #0x4f8
{let a=((cpu.x[8]));let b=((0x4f8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc0524 strb wzr, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc0528 str x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc052c adrp x8, #0x102b3000
cpu.x[8]=(0x102b3000u64) as u64;
// 0x10fc0530 ldr q0, [x8, #0xd50]
{let address=(cpu.x[8]).wrapping_add(0xd50u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fc0534 stp xzr, xzr, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc0538 stp xzr, xzr, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc053c str xzr, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc0540 str q0, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fc0544 str wzr, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc0548 stp x19, xzr, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=cpu.x[19];let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc054c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc0550 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fc0554 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc0500")),}}}
#[inline]
pub(super) unsafe fn f_10fc23f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc23f0=>{
// 0x10fc23f0 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10fc23f4 add x8, x8, #0xfd0
{let a=((cpu.x[8]));let b=((0xfd0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc23f8 ldr x9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc23fc ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10fc2400;continue;
}
0x10fc2400=>{
// 0x10fc2400 ldr x9, [x9, #0xb0]
{let address=(cpu.x[9]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc2404 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fc2408 b.eq #0x10fc2414
if cpu.condition(0){pc=0x10fc2414;continue;}pc=0x10fc240c;continue;
}
0x10fc240c=>{
// 0x10fc240c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fc2410 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc2414=>{
// 0x10fc2414 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc2418 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc241c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2420 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2424 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc2428 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc242c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc2430 mov w20, #0x64
cpu.x[20]=(((0x64u64))) as u32 as u64;
// 0x10fc2434 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2438 b.lt #0x10fc249c
if cpu.condition(11){pc=0x10fc249c;continue;}pc=0x10fc243c;continue;
}
0x10fc243c=>{
// 0x10fc243c mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10fc2440 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10fc2444 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10fc2448 b #0x10fc2464
pc=0x10fc2464;continue;
}
0x10fc244c=>{
// 0x10fc244c cmp w0, w21
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2450 csel w21, w0, w21, gt
cpu.x[21]=(if cpu.condition(12){((cpu.x[0] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
pc=0x10fc2454;continue;
}
0x10fc2454=>{
// 0x10fc2454 ldrsw x8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fc2458 add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10fc245c cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2460 b.ge #0x10fc2490
if cpu.condition(10){pc=0x10fc2490;continue;}pc=0x10fc2464;continue;
}
0x10fc2464=>{
// 0x10fc2464 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2468 ldr x8, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc246c ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2470 bl #0x10ddace8
cpu.x[30]=0x10fc2474;super::chunk11::f_10ddace8(cpu,rt,0x10ddace8)?;
pc=0x10fc2474;continue;
}
0x10fc2474=>{
// 0x10fc2474 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2478 b.ge #0x10fc244c
if cpu.condition(10){pc=0x10fc244c;continue;}pc=0x10fc247c;continue;
}
0x10fc247c=>{
// 0x10fc247c tbz w0, #0x1f, #0x10fc2454
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fc2454;continue;}pc=0x10fc2480;continue;
}
0x10fc2480=>{
// 0x10fc2480 neg w8, w0
cpu.x[8]=(0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10fc2484 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2488 csneg w22, w22, w0, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[22] as u32 as u64))}else{0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))}) as u32 as u64;
// 0x10fc248c b #0x10fc2454
pc=0x10fc2454;continue;
}
0x10fc2490=>{
// 0x10fc2490 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc2494 sub w8, w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc2498 b #0x10fc24a4
pc=0x10fc24a4;continue;
}
0x10fc249c=>{
// 0x10fc249c mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc24a0 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
pc=0x10fc24a4;continue;
}
0x10fc24a4=>{
// 0x10fc24a4 cmp w8, #0x64
{let a=((cpu.x[8] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc24a8 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10fc24ac csel w8, w8, w20, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[20] as u32 as u64))}) as u32 as u64;
// 0x10fc24b0 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fc24b4 bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc24b8 mul w8, w8, w21
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[21] as u32 as u64)))) as u32 as u64;
// 0x10fc24bc smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fc24c0 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fc24c4 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fc24c8 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fc24cc ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc24d0 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fc24d4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc24d8 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fc24dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc23f0")),}}}
#[inline]
pub(super) unsafe fn f_10fc9958(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc9958=>{
// 0x10fc9958 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc995c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc9960 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc9964 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc9968 bl #0x10fc96e0
cpu.x[30]=0x10fc996c;super::chunk14::f_10fc96e0(cpu,rt,0x10fc96e0)?;
pc=0x10fc996c;continue;
}
0x10fc996c=>{
// 0x10fc996c ldr w8, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9970 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc9974 b.lt #0x10fc99a4
if cpu.condition(11){pc=0x10fc99a4;continue;}pc=0x10fc9978;continue;
}
0x10fc9978=>{
// 0x10fc9978 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x10fc997c b #0x10fc998c
pc=0x10fc998c;continue;
}
0x10fc9980=>{
// 0x10fc9980 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10fc9984 cmp x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc9988 b.ge #0x10fc99a4
if cpu.condition(10){pc=0x10fc99a4;continue;}pc=0x10fc998c;continue;
}
0x10fc998c=>{
// 0x10fc998c ldr x9, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc9990 ldr x0, [x9, x20, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc9994 cbz x0, #0x10fc9980
if (cpu.x[0]) == 0 {pc=0x10fc9980;continue;}pc=0x10fc9998;continue;
}
0x10fc9998=>{
// 0x10fc9998 bl #0x1187d2c0
cpu.x[30]=0x10fc999c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10fc999c;continue;
}
0x10fc999c=>{
// 0x10fc999c ldr w8, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc99a0 b #0x10fc9980
pc=0x10fc9980;continue;
}
0x10fc99a4=>{
// 0x10fc99a4 adrp x8, #0x102b2000
cpu.x[8]=(0x102b2000u64) as u64;
// 0x10fc99a8 str wzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc99ac ldr q0, [x8, #0x590]
{let address=(cpu.x[8]).wrapping_add(0x590u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fc99b0 str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc99b4 str wzr, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc99b8 str q0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fc99bc str wzr, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc99c0 str wzr, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc99c4 stur xzr, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc99c8 stur xzr, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc99cc stur xzr, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc99d0 str wzr, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc99d4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc99d8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fc99dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc9958")),}}}
#[inline]
pub(super) unsafe fn f_10fcedb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcedb0=>{
// 0x10fcedb0 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcedb4 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcedb8 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcedbc stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcedc0 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcedc4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcedc8 ldr x20, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fcedcc mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcedd0 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fcedd4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcedd8 bl #0x10f7e050
cpu.x[30]=0x10fceddc;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fceddc;continue;
}
0x10fceddc=>{
// 0x10fceddc mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fcede0 bl #0x10de0580
cpu.x[30]=0x10fcede4;super::chunk3::f_10de0580(cpu,rt,0x10de0580)?;
pc=0x10fcede4;continue;
}
0x10fcede4=>{
// 0x10fcede4 tbz w0, #0, #0x10fcee04
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcee04;continue;}pc=0x10fcede8;continue;
}
0x10fcede8=>{
// 0x10fcede8 ldrb w8, [x22, #0x3a0]
{let address=(cpu.x[22]).wrapping_add(0x3a0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcedec cbz w8, #0x10fcee04
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcee04;continue;}pc=0x10fcedf0;continue;
}
0x10fcedf0=>{
// 0x10fcedf0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcedf4 bl #0x10f94310
cpu.x[30]=0x10fcedf8;super::chunk7::f_10f94310(cpu,rt,0x10f94310)?;
// 0x10fcedf8 cbz x0, #0x10fcee04
if (cpu.x[0]) == 0 {pc=0x10fcee04;continue;}pc=0x10fcedfc;continue;
}
0x10fcedfc=>{
// 0x10fcedfc mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fcee00 b #0x10fcef10
pc=0x10fcef10;continue;
}
0x10fcee04=>{
// 0x10fcee04 ldr w8, [x21, #0x38]
{let address=(cpu.x[21]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcee08 cbz w8, #0x10fceeb4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fceeb4;continue;}pc=0x10fcee0c;continue;
}
0x10fcee0c=>{
// 0x10fcee0c add x8, x21, w8, sxtw #2
{let a=((cpu.x[21]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fcee10 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcee14 ldr w26, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10fcee18 bl #0x10f93d2c
cpu.x[30]=0x10fcee1c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fcee1c;continue;
}
0x10fcee1c=>{
// 0x10fcee1c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10fcee20 bl #0x10d358a0
cpu.x[30]=0x10fcee24;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
pc=0x10fcee24;continue;
}
0x10fcee24=>{
// 0x10fcee24 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcee28 sdiv w25, w26, w0
{let a=((cpu.x[26] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[25]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcee2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcee30 bl #0x10f94310
cpu.x[30]=0x10fcee34;super::chunk7::f_10f94310(cpu,rt,0x10f94310)?;
pc=0x10fcee34;continue;
}
0x10fcee34=>{
// 0x10fcee34 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fcee38 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcee3c bl #0x10fd04a8
cpu.x[30]=0x10fcee40;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fcee40;continue;
}
0x10fcee40=>{
// 0x10fcee40 bl #0x10e0b6b4
cpu.x[30]=0x10fcee44;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fcee44;continue;
}
0x10fcee44=>{
// 0x10fcee44 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcee48 bl #0x10e16208
cpu.x[30]=0x10fcee4c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fcee4c;continue;
}
0x10fcee4c=>{
// 0x10fcee4c mov w21, #0xfa
cpu.x[21]=(((0xfau64))) as u32 as u64;
// 0x10fcee50 cbnz x24, #0x10fcee84
if (cpu.x[24]) != 0 {pc=0x10fcee84;continue;}pc=0x10fcee54;continue;
}
0x10fcee54=>{
// 0x10fcee54 ldrb w8, [x0, #0x3c6]
{let address=(cpu.x[0]).wrapping_add(0x3c6u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcee58 cbz w8, #0x10fcee84
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcee84;continue;}pc=0x10fcee5c;continue;
}
0x10fcee5c=>{
// 0x10fcee5c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcee60 bl #0x10d36084
cpu.x[30]=0x10fcee64;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10fcee64;continue;
}
0x10fcee64=>{
// 0x10fcee64 tbz w0, #0, #0x10fcee84
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcee84;continue;}pc=0x10fcee68;continue;
}
0x10fcee68=>{
// 0x10fcee68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcee6c bl #0x10f817a8
cpu.x[30]=0x10fcee70;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
// 0x10fcee70 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcee74 b.ne #0x10fcee84
if cpu.condition(1){pc=0x10fcee84;continue;}pc=0x10fcee78;continue;
}
0x10fcee78=>{
// 0x10fcee78 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcee7c bl #0x10f94298
cpu.x[30]=0x10fcee80;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10fcee80 b #0x10fcee90
pc=0x10fcee90;continue;
}
0x10fcee84=>{
// 0x10fcee84 msub w8, w25, w22, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_sub((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64))))) as u32 as u64;
// 0x10fcee88 mov w9, #0x1f4
cpu.x[9]=(((0x1f4u64))) as u32 as u64;
// 0x10fcee8c madd w0, w8, w9, w21
cpu.x[0]=((((cpu.x[21] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64))))) as u32 as u64;
pc=0x10fcee90;continue;
}
0x10fcee90=>{
// 0x10fcee90 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fcee94 madd w8, w25, w8, w21
cpu.x[8]=((((cpu.x[21] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fcee98 stp w0, w8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcee9c ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fceea0 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fceea4 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fceea8 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fceeac ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10fceeb0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fceeb4=>{
// 0x10fceeb4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fceeb8 bl #0x10f94310
cpu.x[30]=0x10fceebc;super::chunk7::f_10f94310(cpu,rt,0x10f94310)?;
pc=0x10fceebc;continue;
}
0x10fceebc=>{
// 0x10fceebc cbz x0, #0x10fceee0
if (cpu.x[0]) == 0 {pc=0x10fceee0;continue;}pc=0x10fceec0;continue;
}
0x10fceec0=>{
// 0x10fceec0 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fceec4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fceec8 bl #0x10f94608
cpu.x[30]=0x10fceecc;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10fceecc;continue;
}
0x10fceecc=>{
// 0x10fceecc cbz x0, #0x10fceee0
if (cpu.x[0]) == 0 {pc=0x10fceee0;continue;}pc=0x10fceed0;continue;
}
0x10fceed0=>{
// 0x10fceed0 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fceed4 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10fceed8 bl #0x10fc3f74
cpu.x[30]=0x10fceedc;super::chunk15::f_10fc3f74(cpu,rt,0x10fc3f74)?;
pc=0x10fceedc;continue;
}
0x10fceedc=>{
// 0x10fceedc tbz w0, #0, #0x10fcef10
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcef10;continue;}pc=0x10fceee0;continue;
}
0x10fceee0=>{
// 0x10fceee0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fceee4 bl #0x10f94298
cpu.x[30]=0x10fceee8;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fceee8;continue;
}
0x10fceee8=>{
// 0x10fceee8 str w0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceeec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fceef0 bl #0x10f942a0
cpu.x[30]=0x10fceef4;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fceef4;continue;
}
0x10fceef4=>{
// 0x10fceef4 str w0, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceef8 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fceefc ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcef00 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcef04 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcef08 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10fcef0c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcef10=>{
// 0x10fcef10 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcef14 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcef18 ldr x8, [x8, #0x128]
{let address=(cpu.x[8]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcef1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcef20;super::dispatch(cpu,rt,target)?;}
pc=0x10fcef20;continue;
}
0x10fcef20=>{
// 0x10fcef20 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcef24 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcef28 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fcef2c mov x3, x19
cpu.x[3]=(((cpu.x[19]))) as u64;
// 0x10fcef30 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcef34 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcef38 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcef3c ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcef40 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10fcef44 b #0x10fce380
super::chunk10::f_10fce380(cpu,rt,0x10fce380)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcedb0")),}}}
#[inline]
pub(super) unsafe fn f_10fd0470(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd0470=>{
// 0x10fd0470 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fd0474 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0478 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fd047c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fd0480 bl #0x10fc967c
cpu.x[30]=0x10fd0484;super::chunk12::f_10fc967c(cpu,rt,0x10fc967c)?;
pc=0x10fd0484;continue;
}
0x10fd0484=>{
// 0x10fd0484 adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fd0488 add x8, x8, #0x750
{let a=((cpu.x[8]));let b=((0x750u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fd048c str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0490 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fd0494 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fd0498 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd0470")),}}}
#[inline]
pub(super) unsafe fn f_10fd3f6c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd3f6c=>{
// 0x10fd3f6c adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fd3f70 add x8, x8, #0xab8
{let a=((cpu.x[8]));let b=((0xab8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fd3f74 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd3f78 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd3f6c")),}}}
#[inline]
pub(super) unsafe fn f_10fe83d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fe83d0=>{
// 0x10fe83d0 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fe83d4 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fe83d8 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fe83dc stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fe83e0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fe83e4 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe83e8 str wzr, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe83ec mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fe83f0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10fe83f4 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10fe83f8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe83fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8400;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8400;continue;
}
0x10fe8400=>{
// 0x10fe8400 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe8404 cmp w0, #2, lsl #12
{let a=((cpu.x[0] as u32 as u64));let b=(((0x2u64)) as u32).wrapping_shl(12) as u64;let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8408 b.le #0x10fe8428
if cpu.condition(13){pc=0x10fe8428;continue;}pc=0x10fe840c;continue;
}
0x10fe840c=>{
// 0x10fe840c bl #0x10d19aa4
cpu.x[30]=0x10fe8410;super::dispatch(cpu,rt,0x10d19aa4)?;
// 0x10fe8410 tbnz w0, #0, #0x10fe842c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fe842c;continue;}pc=0x10fe8414;continue;
}
0x10fe8414=>{
// 0x10fe8414 adrp x0, #0x102c3000
cpu.x[0]=(0x102c3000u64) as u64;
// 0x10fe8418 add x0, x0, #0xac5
{let a=((cpu.x[0]));let b=((0xac5u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe841c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fe8420 bl #0x1128df18
cpu.x[30]=0x10fe8424;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fe8424 b #0x10fe842c
pc=0x10fe842c;continue;
}
0x10fe8428=>{
// 0x10fe8428 tbnz w21, #0x1f, #0x10fe84b8
if ((cpu.x[21] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fe84b8;continue;}pc=0x10fe842c;continue;
}
0x10fe842c=>{
// 0x10fe842c cmp w21, #0x400
{let a=((cpu.x[21] as u32 as u64));let b=((0x400u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8430 mov w8, #0x400
cpu.x[8]=(((0x400u64))) as u32 as u64;
// 0x10fe8434 ldr w9, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fe8438 csel w1, w21, w8, lt
cpu.x[1]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fe843c cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8440 b.ge #0x10fe844c
if cpu.condition(10){pc=0x10fe844c;continue;}pc=0x10fe8444;continue;
}
0x10fe8444=>{
// 0x10fe8444 add x0, x19, #0x28
{let a=((cpu.x[19]));let b=((0x28u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8448 bl #0x1078f4d4
cpu.x[30]=0x10fe844c;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10fe844c;continue;
}
0x10fe844c=>{
// 0x10fe844c cbz w21, #0x10fe84b8
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10fe84b8;continue;}pc=0x10fe8450;continue;
}
0x10fe8450=>{
// 0x10fe8450 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fe8454 b #0x10fe8470
pc=0x10fe8470;continue;
}
0x10fe8458=>{
// 0x10fe8458 ldr x9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fe845c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fe8460 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fe8464 str w10, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe8468 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe846c b.eq #0x10fe84b8
if cpu.condition(0){pc=0x10fe84b8;continue;}pc=0x10fe8470;continue;
}
0x10fe8470=>{
// 0x10fe8470 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8474 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8478 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe847c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8480;super::dispatch(cpu,rt,target)?;}
// 0x10fe8480 ldp w9, w8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fe8484 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8488 b.ne #0x10fe8458
if cpu.condition(1){pc=0x10fe8458;continue;}pc=0x10fe848c;continue;
}
0x10fe848c=>{
// 0x10fe848c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fe8490 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8494 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fe8498 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe849c b.ge #0x10fe8458
if cpu.condition(10){pc=0x10fe8458;continue;}pc=0x10fe84a0;continue;
}
0x10fe84a0=>{
// 0x10fe84a0 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe84a4 add x0, x19, #0x28
{let a=((cpu.x[19]));let b=((0x28u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe84a8 bl #0x1078f4d4
cpu.x[30]=0x10fe84ac;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fe84ac ldr w8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fe84b0 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fe84b4 b #0x10fe8458
pc=0x10fe8458;continue;
}
0x10fe84b8=>{
// 0x10fe84b8 ldr w8, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fe84bc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe84c0 b.lt #0x10fe8514
if cpu.condition(11){pc=0x10fe8514;continue;}pc=0x10fe84c4;continue;
}
0x10fe84c4=>{
// 0x10fe84c4 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fe84c8 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10fe84cc b #0x10fe84dc
pc=0x10fe84dc;continue;
}
0x10fe84d0=>{
// 0x10fe84d0 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10fe84d4 cmp x22, x23
{let a=((cpu.x[22]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fe84d8 b.eq #0x10fe8514
if cpu.condition(0){pc=0x10fe8514;continue;}pc=0x10fe84dc;continue;
}
0x10fe84dc=>{
// 0x10fe84dc ldur x8, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe84e0 ldr x21, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fe84e4 cbz x21, #0x10fe84d0
if (cpu.x[21]) == 0 {pc=0x10fe84d0;continue;}pc=0x10fe84e8;continue;
}
0x10fe84e8=>{
// 0x10fe84e8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fe84ec bl #0x10fe6d44
cpu.x[30]=0x10fe84f0;super::dispatch(cpu,rt,0x10fe6d44)?;
// 0x10fe84f0 add x0, x21, #0x40
{let a=((cpu.x[21]));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe84f4 bl #0x1142fcc4
cpu.x[30]=0x10fe84f8;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10fe84f8 add x0, x21, #0x20
{let a=((cpu.x[21]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe84fc bl #0x1142fcc4
cpu.x[30]=0x10fe8500;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10fe8500 add x0, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8504 bl #0x1142fcc4
cpu.x[30]=0x10fe8508;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10fe8508 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fe850c bl #0x1187d2c0
cpu.x[30]=0x10fe8510;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10fe8510 b #0x10fe84d0
pc=0x10fe84d0;continue;
}
0x10fe8514=>{
// 0x10fe8514 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8518 str wzr, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe851c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8520 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8524 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8528;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8528;continue;
}
0x10fe8528=>{
// 0x10fe8528 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe852c cmp w0, #4, lsl #12
{let a=((cpu.x[0] as u32 as u64));let b=(((0x4u64)) as u32).wrapping_shl(12) as u64;let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8530 b.le #0x10fe8550
if cpu.condition(13){pc=0x10fe8550;continue;}pc=0x10fe8534;continue;
}
0x10fe8534=>{
// 0x10fe8534 bl #0x10d19aa4
cpu.x[30]=0x10fe8538;super::dispatch(cpu,rt,0x10d19aa4)?;
// 0x10fe8538 tbnz w0, #0, #0x10fe8554
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fe8554;continue;}pc=0x10fe853c;continue;
}
0x10fe853c=>{
// 0x10fe853c adrp x0, #0x10329000
cpu.x[0]=(0x10329000u64) as u64;
// 0x10fe8540 add x0, x0, #0x43c
{let a=((cpu.x[0]));let b=((0x43cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8544 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fe8548 bl #0x1128df18
cpu.x[30]=0x10fe854c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fe854c b #0x10fe8554
pc=0x10fe8554;continue;
}
0x10fe8550=>{
// 0x10fe8550 tbnz w21, #0x1f, #0x10fe8614
if ((cpu.x[21] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fe8614;continue;}pc=0x10fe8554;continue;
}
0x10fe8554=>{
// 0x10fe8554 cmp w21, #0x400
{let a=((cpu.x[21] as u32 as u64));let b=((0x400u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8558 mov w8, #0x400
cpu.x[8]=(((0x400u64))) as u32 as u64;
// 0x10fe855c ldr w9, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fe8560 csel w1, w21, w8, lt
cpu.x[1]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fe8564 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8568 b.ge #0x10fe8574
if cpu.condition(10){pc=0x10fe8574;continue;}pc=0x10fe856c;continue;
}
0x10fe856c=>{
// 0x10fe856c add x0, x19, #0x50
{let a=((cpu.x[19]));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8570 bl #0x10fe89dc
cpu.x[30]=0x10fe8574;super::dispatch(cpu,rt,0x10fe89dc)?;
pc=0x10fe8574;continue;
}
0x10fe8574=>{
// 0x10fe8574 cbz w21, #0x10fe8614
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10fe8614;continue;}pc=0x10fe8578;continue;
}
0x10fe8578=>{
// 0x10fe8578 mov w24, #5
cpu.x[24]=(((0x5u64))) as u32 as u64;
// 0x10fe857c b #0x10fe8598
pc=0x10fe8598;continue;
}
0x10fe8580=>{
// 0x10fe8580 ldr x9, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fe8584 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fe8588 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fe858c str w10, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe8590 str x22, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fe8594 b.eq #0x10fe8614
if cpu.condition(0){pc=0x10fe8614;continue;}pc=0x10fe8598;continue;
}
0x10fe8598=>{
// 0x10fe8598 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe859c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe85a0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe85a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe85a8;super::dispatch(cpu,rt,target)?;}
// 0x10fe85a8 tbz w0, #0, #0x10fe85e0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fe85e0;continue;}pc=0x10fe85ac;continue;
}
0x10fe85ac=>{
// 0x10fe85ac ldr x23, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fe85b0 mov w0, #0x50
cpu.x[0]=(((0x50u64))) as u32 as u64;
// 0x10fe85b4 bl #0x1187d2b0
cpu.x[30]=0x10fe85b8;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10fe85b8 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10fe85bc mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fe85c0 bl #0x10fe68b4
cpu.x[30]=0x10fe85c4;super::dispatch(cpu,rt,0x10fe68b4)?;
// 0x10fe85c4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fe85c8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fe85cc bl #0x10fe6b44
cpu.x[30]=0x10fe85d0;super::dispatch(cpu,rt,0x10fe6b44)?;
// 0x10fe85d0 ldp w9, w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fe85d4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe85d8 b.ne #0x10fe8580
if cpu.condition(1){pc=0x10fe8580;continue;}pc=0x10fe85dc;continue;
}
0x10fe85dc=>{
// 0x10fe85dc b #0x10fe85f0
pc=0x10fe85f0;continue;
}
0x10fe85e0=>{
// 0x10fe85e0 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10fe85e4 ldp w9, w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fe85e8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe85ec b.ne #0x10fe8580
if cpu.condition(1){pc=0x10fe8580;continue;}pc=0x10fe85f0;continue;
}
0x10fe85f0=>{
// 0x10fe85f0 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fe85f4 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe85f8 csel w1, w24, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[24] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fe85fc cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8600 b.ge #0x10fe8580
if cpu.condition(10){pc=0x10fe8580;continue;}pc=0x10fe8604;continue;
}
0x10fe8604=>{
// 0x10fe8604 add x0, x19, #0x50
{let a=((cpu.x[19]));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8608 bl #0x10fe89dc
cpu.x[30]=0x10fe860c;super::dispatch(cpu,rt,0x10fe89dc)?;
// 0x10fe860c ldr w8, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fe8610 b #0x10fe8580
pc=0x10fe8580;continue;
}
0x10fe8614=>{
// 0x10fe8614 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8618 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe861c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8620 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8624;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8624;continue;
}
0x10fe8624=>{
// 0x10fe8624 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8628 str w0, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe862c cbz x8, #0x10fe8648
if (cpu.x[8]) == 0 {pc=0x10fe8648;continue;}pc=0x10fe8630;continue;
}
0x10fe8630=>{
// 0x10fe8630 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fe8634 bl #0x10fe9490
cpu.x[30]=0x10fe8638;super::dispatch(cpu,rt,0x10fe9490)?;
// 0x10fe8638 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fe863c cbz x0, #0x10fe8644
if (cpu.x[0]) == 0 {pc=0x10fe8644;continue;}pc=0x10fe8640;continue;
}
0x10fe8640=>{
// 0x10fe8640 bl #0x1187d2c0
cpu.x[30]=0x10fe8644;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10fe8644;continue;
}
0x10fe8644=>{
// 0x10fe8644 str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fe8648;continue;
}
0x10fe8648=>{
// 0x10fe8648 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe864c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8650 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8654 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8658;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8658;continue;
}
0x10fe8658=>{
// 0x10fe8658 tbz w0, #0, #0x10fe8684
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fe8684;continue;}pc=0x10fe865c;continue;
}
0x10fe865c=>{
// 0x10fe865c ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fe8660 mov w0, #0x28
cpu.x[0]=(((0x28u64))) as u32 as u64;
// 0x10fe8664 bl #0x1187d2b0
cpu.x[30]=0x10fe8668;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10fe8668 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fe866c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fe8670 bl #0x10fe90d0
cpu.x[30]=0x10fe8674;super::dispatch(cpu,rt,0x10fe90d0)?;
// 0x10fe8674 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fe8678 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fe867c str x22, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fe8680 bl #0x10fe940c
cpu.x[30]=0x10fe8684;super::dispatch(cpu,rt,0x10fe940c)?;
pc=0x10fe8684;continue;
}
0x10fe8684=>{
// 0x10fe8684 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8688 str wzr, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe868c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8690 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8694 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8698;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8698;continue;
}
0x10fe8698=>{
// 0x10fe8698 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe869c cmp w0, #2, lsl #12
{let a=((cpu.x[0] as u32 as u64));let b=(((0x2u64)) as u32).wrapping_shl(12) as u64;let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe86a0 b.le #0x10fe86c0
if cpu.condition(13){pc=0x10fe86c0;continue;}pc=0x10fe86a4;continue;
}
0x10fe86a4=>{
// 0x10fe86a4 bl #0x10d19aa4
cpu.x[30]=0x10fe86a8;super::dispatch(cpu,rt,0x10d19aa4)?;
// 0x10fe86a8 tbnz w0, #0, #0x10fe86c4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fe86c4;continue;}pc=0x10fe86ac;continue;
}
0x10fe86ac=>{
// 0x10fe86ac adrp x0, #0x102bf000
cpu.x[0]=(0x102bf000u64) as u64;
// 0x10fe86b0 add x0, x0, #0xbe1
{let a=((cpu.x[0]));let b=((0xbe1u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe86b4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fe86b8 bl #0x1128df18
cpu.x[30]=0x10fe86bc;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fe86bc b #0x10fe86c4
pc=0x10fe86c4;continue;
}
0x10fe86c0=>{
// 0x10fe86c0 tbnz w21, #0x1f, #0x10fe8750
if ((cpu.x[21] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fe8750;continue;}pc=0x10fe86c4;continue;
}
0x10fe86c4=>{
// 0x10fe86c4 cmp w21, #0x400
{let a=((cpu.x[21] as u32 as u64));let b=((0x400u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe86c8 mov w8, #0x400
cpu.x[8]=(((0x400u64))) as u32 as u64;
// 0x10fe86cc ldr w9, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fe86d0 csel w1, w21, w8, lt
cpu.x[1]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fe86d4 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe86d8 b.ge #0x10fe86e4
if cpu.condition(10){pc=0x10fe86e4;continue;}pc=0x10fe86dc;continue;
}
0x10fe86dc=>{
// 0x10fe86dc add x0, x19, #0x18
{let a=((cpu.x[19]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe86e0 bl #0x1078f4d4
cpu.x[30]=0x10fe86e4;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10fe86e4;continue;
}
0x10fe86e4=>{
// 0x10fe86e4 cbz w21, #0x10fe8750
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10fe8750;continue;}pc=0x10fe86e8;continue;
}
0x10fe86e8=>{
// 0x10fe86e8 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fe86ec b #0x10fe8708
pc=0x10fe8708;continue;
}
0x10fe86f0=>{
// 0x10fe86f0 ldr x9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fe86f4 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fe86f8 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fe86fc str w10, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe8700 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe8704 b.eq #0x10fe8750
if cpu.condition(0){pc=0x10fe8750;continue;}pc=0x10fe8708;continue;
}
0x10fe8708=>{
// 0x10fe8708 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe870c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8710 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8714 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8718;super::dispatch(cpu,rt,target)?;}
// 0x10fe8718 ldp w9, w8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fe871c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8720 b.ne #0x10fe86f0
if cpu.condition(1){pc=0x10fe86f0;continue;}pc=0x10fe8724;continue;
}
0x10fe8724=>{
// 0x10fe8724 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fe8728 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe872c csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fe8730 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8734 b.ge #0x10fe86f0
if cpu.condition(10){pc=0x10fe86f0;continue;}pc=0x10fe8738;continue;
}
0x10fe8738=>{
// 0x10fe8738 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe873c add x0, x19, #0x18
{let a=((cpu.x[19]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8740 bl #0x1078f4d4
cpu.x[30]=0x10fe8744;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fe8744 ldr w8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fe8748 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fe874c b #0x10fe86f0
pc=0x10fe86f0;continue;
}
0x10fe8750=>{
// 0x10fe8750 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8754 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe8758 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe875c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8760;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8760;continue;
}
0x10fe8760=>{
// 0x10fe8760 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8764 stp wzr, w0, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=0u64;let v1=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fe8768 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe876c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe8770 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe8774;super::dispatch(cpu,rt,target)?;}
pc=0x10fe8774;continue;
}
0x10fe8774=>{
// 0x10fe8774 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe8778 cmp w0, #2, lsl #12
{let a=((cpu.x[0] as u32 as u64));let b=(((0x2u64)) as u32).wrapping_shl(12) as u64;let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe877c b.le #0x10fe879c
if cpu.condition(13){pc=0x10fe879c;continue;}pc=0x10fe8780;continue;
}
0x10fe8780=>{
// 0x10fe8780 bl #0x10d19aa4
cpu.x[30]=0x10fe8784;super::dispatch(cpu,rt,0x10d19aa4)?;
// 0x10fe8784 tbnz w0, #0, #0x10fe87a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fe87a0;continue;}pc=0x10fe8788;continue;
}
0x10fe8788=>{
// 0x10fe8788 adrp x0, #0x1032d000
cpu.x[0]=(0x1032d000u64) as u64;
// 0x10fe878c add x0, x0, #0x54d
{let a=((cpu.x[0]));let b=((0x54du64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe8790 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fe8794 bl #0x1128df18
cpu.x[30]=0x10fe8798;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fe8798 b #0x10fe87a0
pc=0x10fe87a0;continue;
}
0x10fe879c=>{
// 0x10fe879c tbnz w21, #0x1f, #0x10fe882c
if ((cpu.x[21] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fe882c;continue;}pc=0x10fe87a0;continue;
}
0x10fe87a0=>{
// 0x10fe87a0 cmp w21, #0x400
{let a=((cpu.x[21] as u32 as u64));let b=((0x400u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe87a4 mov w8, #0x400
cpu.x[8]=(((0x400u64))) as u32 as u64;
// 0x10fe87a8 ldr w9, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fe87ac csel w1, w21, w8, lt
cpu.x[1]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fe87b0 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe87b4 b.ge #0x10fe87c0
if cpu.condition(10){pc=0x10fe87c0;continue;}pc=0x10fe87b8;continue;
}
0x10fe87b8=>{
// 0x10fe87b8 add x0, x19, #0x38
{let a=((cpu.x[19]));let b=((0x38u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe87bc bl #0x1078f4d4
cpu.x[30]=0x10fe87c0;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10fe87c0;continue;
}
0x10fe87c0=>{
// 0x10fe87c0 cbz w21, #0x10fe882c
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10fe882c;continue;}pc=0x10fe87c4;continue;
}
0x10fe87c4=>{
// 0x10fe87c4 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fe87c8 b #0x10fe87e4
pc=0x10fe87e4;continue;
}
0x10fe87cc=>{
// 0x10fe87cc ldr x9, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fe87d0 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fe87d4 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fe87d8 str w10, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe87dc str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fe87e0 b.eq #0x10fe882c
if cpu.condition(0){pc=0x10fe882c;continue;}pc=0x10fe87e4;continue;
}
0x10fe87e4=>{
// 0x10fe87e4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe87e8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fe87ec ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fe87f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fe87f4;super::dispatch(cpu,rt,target)?;}
// 0x10fe87f4 ldp w9, w8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fe87f8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe87fc b.ne #0x10fe87cc
if cpu.condition(1){pc=0x10fe87cc;continue;}pc=0x10fe8800;continue;
}
0x10fe8800=>{
// 0x10fe8800 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fe8804 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8808 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fe880c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fe8810 b.ge #0x10fe87cc
if cpu.condition(10){pc=0x10fe87cc;continue;}pc=0x10fe8814;continue;
}
0x10fe8814=>{
// 0x10fe8814 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fe8818 add x0, x19, #0x38
{let a=((cpu.x[19]));let b=((0x38u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fe881c bl #0x1078f4d4
cpu.x[30]=0x10fe8820;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fe8820 ldr w8, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fe8824 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fe8828 b #0x10fe87cc
pc=0x10fe87cc;continue;
}
0x10fe882c=>{
// 0x10fe882c ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fe8830 bl #0x10e1c95c
cpu.x[30]=0x10fe8834;super::chunk9::f_10e1c95c(cpu,rt,0x10e1c95c)?;
pc=0x10fe8834;continue;
}
0x10fe8834=>{
// 0x10fe8834 add x1, x19, #0x50
{let a=((cpu.x[19]));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10fe8838 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fe883c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fe8840 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fe8844 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fe8848 b #0x10fe8bb4
super::chunk5::f_10fe8bb4(cpu,rt,0x10fe8bb4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fe83d0")),}}}
#[inline]
pub(super) unsafe fn f_10feabb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10feabb0=>{
// 0x10feabb0 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10feabb4 b #0x10feabb8
super::chunk11::f_10feabb8(cpu,rt,0x10feabb8)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10feabb0")),}}}
#[inline]
pub(super) unsafe fn f_10feda88(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10feda88=>{
// 0x10feda88 ldr w0, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10feda8c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10feda88")),}}}
#[inline]
pub(super) unsafe fn f_11027dbc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11027dbc=>{
// 0x11027dbc stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11027dc0 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11027dc4 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11027dc8 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11027dcc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11027dd0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11027dd4 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027dd8 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x11027ddc cbz x0, #0x11027dfc
if (cpu.x[0]) == 0 {pc=0x11027dfc;continue;}pc=0x11027de0;continue;
}
0x11027de0=>{
// 0x11027de0 bl #0x1101b380
cpu.x[30]=0x11027de4;super::chunk3::f_1101b380(cpu,rt,0x1101b380)?;
pc=0x11027de4;continue;
}
0x11027de4=>{
// 0x11027de4 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027de8 cbz x0, #0x11027df8
if (cpu.x[0]) == 0 {pc=0x11027df8;continue;}pc=0x11027dec;continue;
}
0x11027dec=>{
// 0x11027dec ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027df0 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027df4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027df8;super::dispatch(cpu,rt,target)?;}
pc=0x11027df8;continue;
}
0x11027df8=>{
// 0x11027df8 str xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11027dfc;continue;
}
0x11027dfc=>{
// 0x11027dfc ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11027e00 mov w0, #0xd0
cpu.x[0]=(((0xd0u64))) as u32 as u64;
// 0x11027e04 bl #0x1187d2b0
cpu.x[30]=0x11027e08;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x11027e08;continue;
}
0x11027e08=>{
// 0x11027e08 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x11027e0c mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x11027e10 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11027e14 bl #0x1101b278
cpu.x[30]=0x11027e18;super::chunk0::f_1101b278(cpu,rt,0x1101b278)?;
pc=0x11027e18;continue;
}
0x11027e18=>{
// 0x11027e18 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x11027e1c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11027e20 str x22, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027e24 bl #0x1101b954
cpu.x[30]=0x11027e28;super::chunk13::f_1101b954(cpu,rt,0x1101b954)?;
pc=0x11027e28;continue;
}
0x11027e28=>{
// 0x11027e28 ldr x1, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x11027e2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027e30 bl #0x111dd1b0
cpu.x[30]=0x11027e34;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x11027e34;continue;
}
0x11027e34=>{
// 0x11027e34 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e38 str x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027e3c cbz x8, #0x11027e60
if (cpu.x[8]) == 0 {pc=0x11027e60;continue;}pc=0x11027e40;continue;
}
0x11027e40=>{
// 0x11027e40 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x11027e44 bl #0x111d15b4
cpu.x[30]=0x11027e48;super::chunk10::f_111d15b4(cpu,rt,0x111d15b4)?;
pc=0x11027e48;continue;
}
0x11027e48=>{
// 0x11027e48 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027e4c cbz x0, #0x11027e5c
if (cpu.x[0]) == 0 {pc=0x11027e5c;continue;}pc=0x11027e50;continue;
}
0x11027e50=>{
// 0x11027e50 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e54 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e58 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027e5c;super::dispatch(cpu,rt,target)?;}
pc=0x11027e5c;continue;
}
0x11027e5c=>{
// 0x11027e5c str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11027e60;continue;
}
0x11027e60=>{
// 0x11027e60 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027e68 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e6c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027e70;super::dispatch(cpu,rt,target)?;}
pc=0x11027e70;continue;
}
0x11027e70=>{
// 0x11027e70 tbz w0, #0, #0x11027ee0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11027ee0;continue;}pc=0x11027e74;continue;
}
0x11027e74=>{
// 0x11027e74 mov w0, #0x60
cpu.x[0]=(((0x60u64))) as u32 as u64;
// 0x11027e78 bl #0x1187d2b0
cpu.x[30]=0x11027e7c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x11027e7c;continue;
}
0x11027e7c=>{
// 0x11027e7c ldr x1, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x11027e80 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x11027e84 bl #0x111d1354
cpu.x[30]=0x11027e88;super::chunk13::f_111d1354(cpu,rt,0x111d1354)?;
pc=0x11027e88;continue;
}
0x11027e88=>{
// 0x11027e88 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11027e8c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11027e90 str x21, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027e94 bl #0x111d171c
cpu.x[30]=0x11027e98;super::chunk13::f_111d171c(cpu,rt,0x111d171c)?;
pc=0x11027e98;continue;
}
0x11027e98=>{
// 0x11027e98 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027e9c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027ea0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027ea4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027ea8;super::dispatch(cpu,rt,target)?;}
pc=0x11027ea8;continue;
}
0x11027ea8=>{
// 0x11027ea8 tbnz w0, #0, #0x11027ef4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x11027ef4;continue;}pc=0x11027eac;continue;
}
0x11027eac=>{
// 0x11027eac ldr x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027eb0 cbz x0, #0x11027f18
if (cpu.x[0]) == 0 {pc=0x11027f18;continue;}pc=0x11027eb4;continue;
}
0x11027eb4=>{
// 0x11027eb4 bl #0x10fe9b8c
cpu.x[30]=0x11027eb8;super::chunk13::f_10fe9b8c(cpu,rt,0x10fe9b8c)?;
pc=0x11027eb8;continue;
}
0x11027eb8=>{
// 0x11027eb8 ldr x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027ebc cbz x0, #0x11027ec4
if (cpu.x[0]) == 0 {pc=0x11027ec4;continue;}pc=0x11027ec0;continue;
}
0x11027ec0=>{
// 0x11027ec0 bl #0x1187d2c0
cpu.x[30]=0x11027ec4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x11027ec4;continue;
}
0x11027ec4=>{
// 0x11027ec4 str xzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027ec8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027ecc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027ed0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027ed4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027ed8;super::dispatch(cpu,rt,target)?;}
pc=0x11027ed8;continue;
}
0x11027ed8=>{
// 0x11027ed8 tbnz w0, #0, #0x11027f2c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x11027f2c;continue;}pc=0x11027edc;continue;
}
0x11027edc=>{
// 0x11027edc b #0x11027f54
pc=0x11027f54;continue;
}
0x11027ee0=>{
// 0x11027ee0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027ee4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027ee8 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027eec blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027ef0;super::dispatch(cpu,rt,target)?;}
// 0x11027ef0 tbz w0, #0, #0x11027eac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11027eac;continue;}pc=0x11027ef4;continue;
}
0x11027ef4=>{
// 0x11027ef4 mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x11027ef8 bl #0x1187d2b0
cpu.x[30]=0x11027efc;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x11027efc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x11027f00 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11027f04 str x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027f08 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027f0c bl #0x111dd4e8
cpu.x[30]=0x11027f10;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
// 0x11027f10 ldr x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027f14 cbnz x0, #0x11027eb4
if (cpu.x[0]) != 0 {pc=0x11027eb4;continue;}pc=0x11027f18;continue;
}
0x11027f18=>{
// 0x11027f18 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f1c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027f20 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f24 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027f28;super::dispatch(cpu,rt,target)?;}
// 0x11027f28 tbz w0, #0, #0x11027f54
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11027f54;continue;}pc=0x11027f2c;continue;
}
0x11027f2c=>{
// 0x11027f2c ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11027f30 mov w0, #0x28
cpu.x[0]=(((0x28u64))) as u32 as u64;
// 0x11027f34 bl #0x1187d2b0
cpu.x[30]=0x11027f38;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x11027f38;continue;
}
0x11027f38=>{
// 0x11027f38 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11027f3c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11027f40 bl #0x10fe9568
cpu.x[30]=0x11027f44;super::chunk6::f_10fe9568(cpu,rt,0x10fe9568)?;
pc=0x11027f44;continue;
}
0x11027f44=>{
// 0x11027f44 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x11027f48 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11027f4c str x22, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11027f50 bl #0x10fe9a34
cpu.x[30]=0x11027f54;super::chunk10::f_10fe9a34(cpu,rt,0x10fe9a34)?;
pc=0x11027f54;continue;
}
0x11027f54=>{
// 0x11027f54 ldr x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027f58 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11027f5c bl #0x10fe83d0
cpu.x[30]=0x11027f60;super::chunk1::f_10fe83d0(cpu,rt,0x10fe83d0)?;
pc=0x11027f60;continue;
}
0x11027f60=>{
// 0x11027f60 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027f68 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f6c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027f70;super::dispatch(cpu,rt,target)?;}
pc=0x11027f70;continue;
}
0x11027f70=>{
// 0x11027f70 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f74 str w0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11027f78 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027f7c ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027f84;super::dispatch(cpu,rt,target)?;}
pc=0x11027f84;continue;
}
0x11027f84=>{
// 0x11027f84 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f88 str w0, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11027f8c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027f90 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f94 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027f98;super::dispatch(cpu,rt,target)?;}
pc=0x11027f98;continue;
}
0x11027f98=>{
// 0x11027f98 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027f9c str w0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11027fa0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027fa4 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027fa8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027fac;super::dispatch(cpu,rt,target)?;}
pc=0x11027fac;continue;
}
0x11027fac=>{
// 0x11027fac ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027fb0 str w0, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11027fb4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027fb8 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027fbc blr x8
{let target=cpu.x[8];cpu.x[30]=0x11027fc0;super::dispatch(cpu,rt,target)?;}
pc=0x11027fc0;continue;
}
0x11027fc0=>{
// 0x11027fc0 ldr x8, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027fc4 str w0, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11027fc8 cbz x8, #0x11027ff4
if (cpu.x[8]) == 0 {pc=0x11027ff4;continue;}pc=0x11027fcc;continue;
}
0x11027fcc=>{
// 0x11027fcc mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x11027fd0 bl #0x11021d90
cpu.x[30]=0x11027fd4;super::dispatch(cpu,rt,0x11021d90)?;
// 0x11027fd4 ldr x21, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11027fd8 cbz x21, #0x11027ff0
if (cpu.x[21]) == 0 {pc=0x11027ff0;continue;}pc=0x11027fdc;continue;
}
0x11027fdc=>{
// 0x11027fdc ldr x0, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11027fe0 cbz x0, #0x11027fe8
if (cpu.x[0]) == 0 {pc=0x11027fe8;continue;}pc=0x11027fe4;continue;
}
0x11027fe4=>{
// 0x11027fe4 bl #0x1187d360
cpu.x[30]=0x11027fe8;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x11027fe8;continue;
}
0x11027fe8=>{
// 0x11027fe8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11027fec bl #0x1187d2c0
cpu.x[30]=0x11027ff0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x11027ff0;continue;
}
0x11027ff0=>{
// 0x11027ff0 str xzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11027ff4;continue;
}
0x11027ff4=>{
// 0x11027ff4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11027ff8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11027ffc ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x11028000;continue;
}
0x11028000=>{
// 0x11028000 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028004;super::dispatch(cpu,rt,target)?;}
pc=0x11028004;continue;
}
0x11028004=>{
// 0x11028004 tbz w0, #0, #0x11028030
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11028030;continue;}pc=0x11028008;continue;
}
0x11028008=>{
// 0x11028008 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1102800c mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x11028010 bl #0x1187d2b0
cpu.x[30]=0x11028014;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x11028014 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11028018 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x1102801c bl #0x110219a8
cpu.x[30]=0x11028020;super::dispatch(cpu,rt,0x110219a8)?;
// 0x11028020 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x11028024 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11028028 str x22, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1102802c bl #0x11021c08
cpu.x[30]=0x11028030;super::dispatch(cpu,rt,0x11021c08)?;
pc=0x11028030;continue;
}
0x11028030=>{
// 0x11028030 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028034 cbz x0, #0x1102805c
if (cpu.x[0]) == 0 {pc=0x1102805c;continue;}pc=0x11028038;continue;
}
0x11028038=>{
// 0x11028038 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102803c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028040 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028044;super::dispatch(cpu,rt,target)?;}
// 0x11028044 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028048 cbz x0, #0x11028058
if (cpu.x[0]) == 0 {pc=0x11028058;continue;}pc=0x1102804c;continue;
}
0x1102804c=>{
// 0x1102804c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028050 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028054 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028058;super::dispatch(cpu,rt,target)?;}
pc=0x11028058;continue;
}
0x11028058=>{
// 0x11028058 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x1102805c;continue;
}
0x1102805c=>{
// 0x1102805c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028060 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028064 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028068 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102806c;super::dispatch(cpu,rt,target)?;}
pc=0x1102806c;continue;
}
0x1102806c=>{
// 0x1102806c tbz w0, #0, #0x110280a4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110280a4;continue;}pc=0x11028070;continue;
}
0x11028070=>{
// 0x11028070 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11028074 mov w0, #0x40
cpu.x[0]=(((0x40u64))) as u32 as u64;
// 0x11028078 bl #0x1187d2b0
cpu.x[30]=0x1102807c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x1102807c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11028080 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11028084 bl #0x10ff2644
cpu.x[30]=0x11028088;super::dispatch(cpu,rt,0x10ff2644)?;
// 0x11028088 adrp x8, #0x1193c000
cpu.x[8]=(0x1193c000u64) as u64;
// 0x1102808c add x8, x8, #0x630
{let a=((cpu.x[8]));let b=((0x630u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x11028090 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x11028094 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11028098 str x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1102809c str x22, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110280a0 bl #0x10fea130
cpu.x[30]=0x110280a4;super::dispatch(cpu,rt,0x10fea130)?;
pc=0x110280a4;continue;
}
0x110280a4=>{
// 0x110280a4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110280a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110280ac ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110280b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110280b4;super::dispatch(cpu,rt,target)?;}
pc=0x110280b4;continue;
}
0x110280b4=>{
// 0x110280b4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110280b8 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x110280bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110280c0 strb w9, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x110280c4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110280c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110280cc;super::dispatch(cpu,rt,target)?;}
pc=0x110280cc;continue;
}
0x110280cc=>{
// 0x110280cc tbz w0, #0, #0x11028108
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11028108;continue;}pc=0x110280d0;continue;
}
0x110280d0=>{
// 0x110280d0 ldr x21, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110280d4 cbz x21, #0x110280ec
if (cpu.x[21]) == 0 {pc=0x110280ec;continue;}pc=0x110280d8;continue;
}
0x110280d8=>{
// 0x110280d8 ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110280dc cbz x0, #0x110280e4
if (cpu.x[0]) == 0 {pc=0x110280e4;continue;}pc=0x110280e0;continue;
}
0x110280e0=>{
// 0x110280e0 bl #0x1187d360
cpu.x[30]=0x110280e4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110280e4;continue;
}
0x110280e4=>{
// 0x110280e4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x110280e8 bl #0x1187d2c0
cpu.x[30]=0x110280ec;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110280ec;continue;
}
0x110280ec=>{
// 0x110280ec mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x110280f0 bl #0x1187d2b0
cpu.x[30]=0x110280f4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x110280f4;continue;
}
0x110280f4=>{
// 0x110280f4 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x110280f8 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110280fc str x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11028100 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028104 bl #0x111dd4e8
cpu.x[30]=0x11028108;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
pc=0x11028108;continue;
}
0x11028108=>{
// 0x11028108 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1102810c cbz x0, #0x11028124
if (cpu.x[0]) == 0 {pc=0x11028124;continue;}pc=0x11028110;continue;
}
0x11028110=>{
// 0x11028110 bl #0x1105fc94
cpu.x[30]=0x11028114;super::dispatch(cpu,rt,0x1105fc94)?;
// 0x11028114 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028118 cbz x0, #0x11028120
if (cpu.x[0]) == 0 {pc=0x11028120;continue;}pc=0x1102811c;continue;
}
0x1102811c=>{
// 0x1102811c bl #0x1187d2c0
cpu.x[30]=0x11028120;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x11028120;continue;
}
0x11028120=>{
// 0x11028120 str xzr, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11028124;continue;
}
0x11028124=>{
// 0x11028124 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028128 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1102812c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028130 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028134;super::dispatch(cpu,rt,target)?;}
pc=0x11028134;continue;
}
0x11028134=>{
// 0x11028134 tbz w0, #0, #0x11028168
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11028168;continue;}pc=0x11028138;continue;
}
0x11028138=>{
// 0x11028138 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1102813c mov w0, #0x78
cpu.x[0]=(((0x78u64))) as u32 as u64;
// 0x11028140 bl #0x1187d2b0
cpu.x[30]=0x11028144;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x11028144 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11028148 mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x1102814c mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x11028150 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11028154 bl #0x1105f078
cpu.x[30]=0x11028158;super::dispatch(cpu,rt,0x1105f078)?;
// 0x11028158 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x1102815c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11028160 str x22, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11028164 bl #0x1105f3d0
cpu.x[30]=0x11028168;super::dispatch(cpu,rt,0x1105f3d0)?;
pc=0x11028168;continue;
}
0x11028168=>{
// 0x11028168 ldr x0, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1102816c cbz x0, #0x110281c8
if (cpu.x[0]) == 0 {pc=0x110281c8;continue;}pc=0x11028170;continue;
}
0x11028170=>{
// 0x11028170 bl #0x11067fc4
cpu.x[30]=0x11028174;super::dispatch(cpu,rt,0x11067fc4)?;
// 0x11028174 ldr x21, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11028178 cbz x21, #0x110281c4
if (cpu.x[21]) == 0 {pc=0x110281c4;continue;}pc=0x1102817c;continue;
}
0x1102817c=>{
// 0x1102817c add x0, x21, #0x90
{let a=((cpu.x[21]));let b=((0x90u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x11028180 bl #0x1142fcc4
cpu.x[30]=0x11028184;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x11028184 add x0, x21, #0x80
{let a=((cpu.x[21]));let b=((0x80u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x11028188 bl #0x1142fcc4
cpu.x[30]=0x1102818c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x1102818c add x0, x21, #0x70
{let a=((cpu.x[21]));let b=((0x70u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x11028190 bl #0x1142fcc4
cpu.x[30]=0x11028194;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x11028194 add x0, x21, #0x60
{let a=((cpu.x[21]));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x11028198 bl #0x1142fcc4
cpu.x[30]=0x1102819c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x1102819c add x0, x21, #0x48
{let a=((cpu.x[21]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110281a0 bl #0x1142fcc4
cpu.x[30]=0x110281a4;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110281a4 add x0, x21, #0x38
{let a=((cpu.x[21]));let b=((0x38u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110281a8 bl #0x1142fcc4
cpu.x[30]=0x110281ac;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110281ac add x0, x21, #0x28
{let a=((cpu.x[21]));let b=((0x28u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110281b0 bl #0x1142fcc4
cpu.x[30]=0x110281b4;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110281b4 add x0, x21, #0x18
{let a=((cpu.x[21]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110281b8 bl #0x1142fcc4
cpu.x[30]=0x110281bc;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110281bc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x110281c0 bl #0x1187d2c0
cpu.x[30]=0x110281c4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110281c4;continue;
}
0x110281c4=>{
// 0x110281c4 str xzr, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110281c8;continue;
}
0x110281c8=>{
// 0x110281c8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110281cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110281d0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110281d4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110281d8;super::dispatch(cpu,rt,target)?;}
pc=0x110281d8;continue;
}
0x110281d8=>{
// 0x110281d8 tbz w0, #0, #0x110281fc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110281fc;continue;}pc=0x110281dc;continue;
}
0x110281dc=>{
// 0x110281dc mov w0, #0xa8
cpu.x[0]=(((0xa8u64))) as u32 as u64;
// 0x110281e0 bl #0x1187d2b0
cpu.x[30]=0x110281e4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x110281e4 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x110281e8 bl #0x11067f50
cpu.x[30]=0x110281ec;super::dispatch(cpu,rt,0x11067f50)?;
// 0x110281ec mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x110281f0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x110281f4 str x21, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110281f8 bl #0x11068110
cpu.x[30]=0x110281fc;super::dispatch(cpu,rt,0x11068110)?;
pc=0x110281fc;continue;
}
0x110281fc=>{
// 0x110281fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028200 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028204 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028208 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102820c;super::dispatch(cpu,rt,target)?;}
pc=0x1102820c;continue;
}
0x1102820c=>{
// 0x1102820c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028210 str w0, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028214 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028218 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102821c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028220;super::dispatch(cpu,rt,target)?;}
pc=0x11028220;continue;
}
0x11028220=>{
// 0x11028220 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11028224 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028228 tbz w8, #0, #0x1102825c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x1102825c;continue;}pc=0x1102822c;continue;
}
0x1102822c=>{
// 0x1102822c cbnz x0, #0x11028250
if (cpu.x[0]) != 0 {pc=0x11028250;continue;}pc=0x11028230;continue;
}
0x11028230=>{
// 0x11028230 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11028234 mov w0, #0x50
cpu.x[0]=(((0x50u64))) as u32 as u64;
// 0x11028238 bl #0x1187d2b0
cpu.x[30]=0x1102823c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x1102823c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11028240 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11028244 bl #0x1117aa08
cpu.x[30]=0x11028248;super::dispatch(cpu,rt,0x1117aa08)?;
// 0x11028248 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x1102824c str x22, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11028250;continue;
}
0x11028250=>{
// 0x11028250 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11028254 bl #0x1117b020
cpu.x[30]=0x11028258;super::dispatch(cpu,rt,0x1117b020)?;
// 0x11028258 b #0x1102827c
pc=0x1102827c;continue;
}
0x1102825c=>{
// 0x1102825c cbz x0, #0x1102827c
if (cpu.x[0]) == 0 {pc=0x1102827c;continue;}pc=0x11028260;continue;
}
0x11028260=>{
// 0x11028260 bl #0x1117ae80
cpu.x[30]=0x11028264;super::dispatch(cpu,rt,0x1117ae80)?;
// 0x11028264 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028268 cbz x0, #0x11028278
if (cpu.x[0]) == 0 {pc=0x11028278;continue;}pc=0x1102826c;continue;
}
0x1102826c=>{
// 0x1102826c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028270 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028274 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028278;super::dispatch(cpu,rt,target)?;}
pc=0x11028278;continue;
}
0x11028278=>{
// 0x11028278 str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x1102827c;continue;
}
0x1102827c=>{
// 0x1102827c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028280 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028284 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028288 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102828c;super::dispatch(cpu,rt,target)?;}
pc=0x1102828c;continue;
}
0x1102828c=>{
// 0x1102828c tbz w0, #0, #0x110282a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110282a0;continue;}pc=0x11028290;continue;
}
0x11028290=>{
// 0x11028290 ldr x1, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x11028294 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028298 bl #0x111dd1b0
cpu.x[30]=0x1102829c;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
// 0x1102829c str x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110282a0;continue;
}
0x110282a0=>{
// 0x110282a0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110282a4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110282a8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110282ac blr x8
{let target=cpu.x[8];cpu.x[30]=0x110282b0;super::dispatch(cpu,rt,target)?;}
pc=0x110282b0;continue;
}
0x110282b0=>{
// 0x110282b0 ldr x21, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110282b4 str w0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110282b8 cbz x21, #0x11028328
if (cpu.x[21]) == 0 {pc=0x11028328;continue;}pc=0x110282bc;continue;
}
0x110282bc=>{
// 0x110282bc ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110282c0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110282c4 b.lt #0x11028310
if cpu.condition(11){pc=0x11028310;continue;}pc=0x110282c8;continue;
}
0x110282c8=>{
// 0x110282c8 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x110282cc b #0x110282ec
pc=0x110282ec;continue;
}
0x110282d0=>{
// 0x110282d0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x110282d4 bl #0x1187d2c0
cpu.x[30]=0x110282d8;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x110282d8 ldr x21, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
pc=0x110282dc;continue;
}
0x110282dc=>{
// 0x110282dc ldrsw x8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x110282e0 add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x110282e4 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110282e8 b.ge #0x11028310
if cpu.condition(10){pc=0x11028310;continue;}pc=0x110282ec;continue;
}
0x110282ec=>{
// 0x110282ec ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110282f0 ldr x22, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x110282f4 cbz x22, #0x110282dc
if (cpu.x[22]) == 0 {pc=0x110282dc;continue;}pc=0x110282f8;continue;
}
0x110282f8=>{
// 0x110282f8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x110282fc bl #0x10fef06c
cpu.x[30]=0x11028300;super::dispatch(cpu,rt,0x10fef06c)?;
// 0x11028300 ldr x0, [x22, #0x30]
{let address=(cpu.x[22]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028304 cbz x0, #0x110282d0
if (cpu.x[0]) == 0 {pc=0x110282d0;continue;}pc=0x11028308;continue;
}
0x11028308=>{
// 0x11028308 bl #0x1187d360
cpu.x[30]=0x1102830c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
// 0x1102830c b #0x110282d0
pc=0x110282d0;continue;
}
0x11028310=>{
// 0x11028310 ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028314 cbz x0, #0x1102831c
if (cpu.x[0]) == 0 {pc=0x1102831c;continue;}pc=0x11028318;continue;
}
0x11028318=>{
// 0x11028318 bl #0x1187d360
cpu.x[30]=0x1102831c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1102831c;continue;
}
0x1102831c=>{
// 0x1102831c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11028320 bl #0x1187d2c0
cpu.x[30]=0x11028324;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x11028324 str xzr, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11028328;continue;
}
0x11028328=>{
// 0x11028328 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102832c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028330 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028334 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028338;super::dispatch(cpu,rt,target)?;}
pc=0x11028338;continue;
}
0x11028338=>{
// 0x11028338 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x1102833c mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x11028340 bl #0x1187d2b0
cpu.x[30]=0x11028344;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x11028344;continue;
}
0x11028344=>{
// 0x11028344 ubfiz x8, x21, #3, #0x20
cpu.x[8]=(((((cpu.x[21])) & 0xffffffffu64) << 3)) as u64;
// 0x11028348 cmp w21, #0
{let a=((cpu.x[21] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1102834c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x11028350 str xzr, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11028354 str w21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028358 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x1102835c bl #0x1187d520
cpu.x[30]=0x11028360;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x11028360;continue;
}
0x11028360=>{
// 0x11028360 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028364 str x0, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11028368 str x22, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1102836c b.lt #0x11028430
if cpu.condition(11){pc=0x11028430;continue;}pc=0x11028370;continue;
}
0x11028370=>{
// 0x11028370 mov w24, #5
cpu.x[24]=(((0x5u64))) as u32 as u64;
// 0x11028374 b #0x11028394
pc=0x11028394;continue;
}
0x11028378=>{
// 0x11028378 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
pc=0x1102837c;continue;
}
0x1102837c=>{
// 0x1102837c ldr x9, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x11028380 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11028384 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x11028388 str w10, [x22, #0xc]
{let address=(cpu.x[22]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1102838c str x23, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11028390 b.eq #0x11028430
if cpu.condition(0){pc=0x11028430;continue;}pc=0x11028394;continue;
}
0x11028394=>{
// 0x11028394 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028398 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1102839c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110283a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110283a4;super::dispatch(cpu,rt,target)?;}
// 0x110283a4 tbz w0, #0, #0x110283f8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110283f8;continue;}pc=0x110283a8;continue;
}
0x110283a8=>{
// 0x110283a8 mov w0, #0x48
cpu.x[0]=(((0x48u64))) as u32 as u64;
// 0x110283ac bl #0x1187d2b0
cpu.x[30]=0x110283b0;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x110283b0 ldr x1, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x110283b4 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x110283b8 bl #0x10feefd8
cpu.x[30]=0x110283bc;super::dispatch(cpu,rt,0x10feefd8)?;
// 0x110283bc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x110283c0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x110283c4 bl #0x10fef180
cpu.x[30]=0x110283c8;super::dispatch(cpu,rt,0x10fef180)?;
// 0x110283c8 ldr x22, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x110283cc ldp w9, w8, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x110283d0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110283d4 b.ne #0x1102837c
if cpu.condition(1){pc=0x1102837c;continue;}pc=0x110283d8;continue;
}
0x110283d8=>{
// 0x110283d8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x110283dc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110283e0 csel w1, w24, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[24] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x110283e4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110283e8 b.ge #0x1102837c
if cpu.condition(10){pc=0x1102837c;continue;}pc=0x110283ec;continue;
}
0x110283ec=>{
// 0x110283ec mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x110283f0 bl #0x10dad610
cpu.x[30]=0x110283f4;super::dispatch(cpu,rt,0x10dad610)?;
// 0x110283f4 b #0x11028428
pc=0x11028428;continue;
}
0x110283f8=>{
// 0x110283f8 ldr x22, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x110283fc ldp w9, w8, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x11028400 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028404 b.ne #0x11028378
if cpu.condition(1){pc=0x11028378;continue;}pc=0x11028408;continue;
}
0x11028408=>{
// 0x11028408 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x1102840c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028410 csel w1, w24, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[24] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x11028414 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028418 b.ge #0x11028378
if cpu.condition(10){pc=0x11028378;continue;}pc=0x1102841c;continue;
}
0x1102841c=>{
// 0x1102841c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x11028420 bl #0x10dad610
cpu.x[30]=0x11028424;super::dispatch(cpu,rt,0x10dad610)?;
// 0x11028424 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
pc=0x11028428;continue;
}
0x11028428=>{
// 0x11028428 ldr w8, [x22, #0xc]
{let address=(cpu.x[22]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1102842c b #0x1102837c
pc=0x1102837c;continue;
}
0x11028430=>{
// 0x11028430 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028434 cbz x0, #0x11028498
if (cpu.x[0]) == 0 {pc=0x11028498;continue;}pc=0x11028438;continue;
}
0x11028438=>{
// 0x11028438 bl #0x10fee3c0
cpu.x[30]=0x1102843c;super::dispatch(cpu,rt,0x10fee3c0)?;
// 0x1102843c ldr x21, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11028440 cbz x21, #0x11028494
if (cpu.x[21]) == 0 {pc=0x11028494;continue;}pc=0x11028444;continue;
}
0x11028444=>{
// 0x11028444 mov x22, x21
cpu.x[22]=(((cpu.x[21]))) as u64;
// 0x11028448 ldr x0, [x22, #0x38]!
{let address=(cpu.x[22]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x1102844c cbz x0, #0x11028454
if (cpu.x[0]) == 0 {pc=0x11028454;continue;}pc=0x11028450;continue;
}
0x11028450=>{
// 0x11028450 bl #0x1187d360
cpu.x[30]=0x11028454;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x11028454;continue;
}
0x11028454=>{
// 0x11028454 mov x23, x21
cpu.x[23]=(((cpu.x[21]))) as u64;
// 0x11028458 stp xzr, xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1102845c ldr x0, [x23, #0x28]!
{let address=(cpu.x[23]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[23]=(address) as u64;}
// 0x11028460 cbz x0, #0x11028468
if (cpu.x[0]) == 0 {pc=0x11028468;continue;}pc=0x11028464;continue;
}
0x11028464=>{
// 0x11028464 bl #0x1187d360
cpu.x[30]=0x11028468;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x11028468;continue;
}
0x11028468=>{
// 0x11028468 mov x22, x21
cpu.x[22]=(((cpu.x[21]))) as u64;
// 0x1102846c stp xzr, xzr, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11028470 ldr x0, [x22, #0x18]!
{let address=(cpu.x[22]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x11028474 cbz x0, #0x1102847c
if (cpu.x[0]) == 0 {pc=0x1102847c;continue;}pc=0x11028478;continue;
}
0x11028478=>{
// 0x11028478 bl #0x1187d360
cpu.x[30]=0x1102847c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1102847c;continue;
}
0x1102847c=>{
// 0x1102847c ldr x0, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028480 stp xzr, xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11028484 cbz x0, #0x1102848c
if (cpu.x[0]) == 0 {pc=0x1102848c;continue;}pc=0x11028488;continue;
}
0x11028488=>{
// 0x11028488 bl #0x1187d360
cpu.x[30]=0x1102848c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1102848c;continue;
}
0x1102848c=>{
// 0x1102848c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11028490 bl #0x1187d2c0
cpu.x[30]=0x11028494;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x11028494;continue;
}
0x11028494=>{
// 0x11028494 str xzr, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11028498;continue;
}
0x11028498=>{
// 0x11028498 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102849c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110284a0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110284a8;super::dispatch(cpu,rt,target)?;}
pc=0x110284a8;continue;
}
0x110284a8=>{
// 0x110284a8 tbz w0, #0, #0x110284d4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110284d4;continue;}pc=0x110284ac;continue;
}
0x110284ac=>{
// 0x110284ac ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110284b0 mov w0, #0x58
cpu.x[0]=(((0x58u64))) as u32 as u64;
// 0x110284b4 bl #0x1187d2b0
cpu.x[30]=0x110284b8;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x110284b8 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x110284bc mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x110284c0 bl #0x10fedd14
cpu.x[30]=0x110284c4;super::dispatch(cpu,rt,0x10fedd14)?;
// 0x110284c4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x110284c8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x110284cc str x22, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110284d0 bl #0x10fee00c
cpu.x[30]=0x110284d4;super::dispatch(cpu,rt,0x10fee00c)?;
pc=0x110284d4;continue;
}
0x110284d4=>{
// 0x110284d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110284dc ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110284e4;super::dispatch(cpu,rt,target)?;}
pc=0x110284e4;continue;
}
0x110284e4=>{
// 0x110284e4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284e8 str w0, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110284ec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110284f0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110284f8;super::dispatch(cpu,rt,target)?;}
pc=0x110284f8;continue;
}
0x110284f8=>{
// 0x110284f8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110284fc str w0, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028500 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028504 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028508 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102850c;super::dispatch(cpu,rt,target)?;}
pc=0x1102850c;continue;
}
0x1102850c=>{
// 0x1102850c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028510 str w0, [x19, #0x84]
{let address=(cpu.x[19]).wrapping_add(0x84u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028514 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028518 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102851c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028520;super::dispatch(cpu,rt,target)?;}
pc=0x11028520;continue;
}
0x11028520=>{
// 0x11028520 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028524 str w0, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028528 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1102852c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028530 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028534;super::dispatch(cpu,rt,target)?;}
pc=0x11028534;continue;
}
0x11028534=>{
// 0x11028534 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028538 str w0, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1102853c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028540 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028544 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028548;super::dispatch(cpu,rt,target)?;}
pc=0x11028548;continue;
}
0x11028548=>{
// 0x11028548 tbz w0, #0, #0x1102855c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x1102855c;continue;}pc=0x1102854c;continue;
}
0x1102854c=>{
// 0x1102854c ldr x1, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x11028550 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028554 bl #0x111dd1b0
cpu.x[30]=0x11028558;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
// 0x11028558 str x0, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x1102855c;continue;
}
0x1102855c=>{
// 0x1102855c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028560 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028564 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028568 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102856c;super::dispatch(cpu,rt,target)?;}
pc=0x1102856c;continue;
}
0x1102856c=>{
// 0x1102856c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028570 str w0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028574 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028578 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102857c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028580;super::dispatch(cpu,rt,target)?;}
pc=0x11028580;continue;
}
0x11028580=>{
// 0x11028580 ldr x21, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11028584 str w0, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028588 cbz x21, #0x110285a0
if (cpu.x[21]) == 0 {pc=0x110285a0;continue;}pc=0x1102858c;continue;
}
0x1102858c=>{
// 0x1102858c ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11028590 cbz x0, #0x11028598
if (cpu.x[0]) == 0 {pc=0x11028598;continue;}pc=0x11028594;continue;
}
0x11028594=>{
// 0x11028594 bl #0x1187d360
cpu.x[30]=0x11028598;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x11028598;continue;
}
0x11028598=>{
// 0x11028598 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x1102859c bl #0x1187d2c0
cpu.x[30]=0x110285a0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110285a0;continue;
}
0x110285a0=>{
// 0x110285a0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110285a4 str xzr, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110285a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110285ac ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110285b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110285b4;super::dispatch(cpu,rt,target)?;}
pc=0x110285b4;continue;
}
0x110285b4=>{
// 0x110285b4 tbz w0, #0, #0x1102866c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x1102866c;continue;}pc=0x110285b8;continue;
}
0x110285b8=>{
// 0x110285b8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110285bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110285c0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110285c4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110285c8;super::dispatch(cpu,rt,target)?;}
// 0x110285c8 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x110285cc mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x110285d0 bl #0x1187d2b0
cpu.x[30]=0x110285d4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x110285d4 ubfiz x8, x21, #2, #0x20
cpu.x[8]=(((((cpu.x[21])) & 0xffffffffu64) << 2)) as u64;
// 0x110285d8 cmp w21, #0
{let a=((cpu.x[21] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110285dc mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x110285e0 str xzr, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110285e4 str w21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110285e8 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x110285ec bl #0x1187d520
cpu.x[30]=0x110285f0;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
// 0x110285f0 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110285f4 str x0, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110285f8 str x22, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110285fc b.lt #0x1102866c
if cpu.condition(11){pc=0x1102866c;continue;}pc=0x11028600;continue;
}
0x11028600=>{
// 0x11028600 mov w23, #5
cpu.x[23]=(((0x5u64))) as u32 as u64;
// 0x11028604 b #0x11028620
pc=0x11028620;continue;
}
0x11028608=>{
// 0x11028608 ldr x9, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1102860c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11028610 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x11028614 str w10, [x22, #0xc]
{let address=(cpu.x[22]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028618 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1102861c b.eq #0x1102866c
if cpu.condition(0){pc=0x1102866c;continue;}pc=0x11028620;continue;
}
0x11028620=>{
// 0x11028620 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028624 ldr x22, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x11028628 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1102862c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028630 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028634;super::dispatch(cpu,rt,target)?;}
// 0x11028634 ldp w9, w8, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x11028638 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1102863c b.ne #0x11028608
if cpu.condition(1){pc=0x11028608;continue;}pc=0x11028640;continue;
}
0x11028640=>{
// 0x11028640 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x11028644 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028648 csel w1, w23, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[23] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x1102864c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11028650 b.ge #0x11028608
if cpu.condition(10){pc=0x11028608;continue;}pc=0x11028654;continue;
}
0x11028654=>{
// 0x11028654 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11028658 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x1102865c bl #0x1078f4d4
cpu.x[30]=0x11028660;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x11028660 ldr w8, [x22, #0xc]
{let address=(cpu.x[22]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11028664 mov w0, w24
cpu.x[0]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x11028668 b #0x11028608
pc=0x11028608;continue;
}
0x1102866c=>{
// 0x1102866c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028670 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028674 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028678 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1102867c;super::dispatch(cpu,rt,target)?;}
pc=0x1102867c;continue;
}
0x1102867c=>{
// 0x1102867c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028680 str w0, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028684 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11028688 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1102868c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11028690;super::dispatch(cpu,rt,target)?;}
pc=0x11028690;continue;
}
0x11028690=>{
// 0x11028690 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11028694 str w0, [x19, #0xec]
{let address=(cpu.x[19]).wrapping_add(0xecu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11028698 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1102869c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110286a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110286a4;super::dispatch(cpu,rt,target)?;}
pc=0x110286a4;continue;
}
0x110286a4=>{
// 0x110286a4 str w0, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110286a8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110286ac ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110286b0 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x110286b4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x110286b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11027dbc")),}}}
#[inline]
pub(super) unsafe fn f_11057784(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11057784=>{
// 0x11057784 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11057788 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1105778c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11057790 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11057794 bl #0x10ff04f0
cpu.x[30]=0x11057798;super::chunk12::f_10ff04f0(cpu,rt,0x10ff04f0)?;
pc=0x11057798;continue;
}
0x11057798=>{
// 0x11057798 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1105779c cbz x0, #0x110577c0
if (cpu.x[0]) == 0 {pc=0x110577c0;continue;}pc=0x110577a0;continue;
}
0x110577a0=>{
// 0x110577a0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110577a4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110577a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110577ac;super::dispatch(cpu,rt,target)?;}
// 0x110577ac ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110577b0 cbz x0, #0x110577c0
if (cpu.x[0]) == 0 {pc=0x110577c0;continue;}pc=0x110577b4;continue;
}
0x110577b4=>{
// 0x110577b4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110577b8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110577bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110577c0;super::dispatch(cpu,rt,target)?;}
pc=0x110577c0;continue;
}
0x110577c0=>{
// 0x110577c0 adrp x8, #0x102b1000
cpu.x[8]=(0x102b1000u64) as u64;
// 0x110577c4 mov w9, #-1
cpu.x[9]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x110577c8 strh wzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x110577cc ldr d0, [x8, #0xa98]
{let address=(cpu.x[8]).wrapping_add(0xa98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x110577d0 str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110577d4 str w9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110577d8 str d0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110577dc str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110577e0 stp wzr, wzr, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=0u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x110577e4 stur xzr, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110577e8 stur xzr, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110577ec str wzr, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110577f0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x110577f4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x110577f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11057784")),}}}
#[inline]
pub(super) unsafe fn f_11057d08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11057d08=>{
// 0x11057d08 ldr w0, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x11057d0c b #0x10dd4bdc
super::chunk6::f_10dd4bdc(cpu,rt,0x10dd4bdc)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11057d08")),}}}
#[inline]
pub(super) unsafe fn f_11058798(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11058798=>{
// 0x11058798 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1105879c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11058798")),}}}
#[inline]
pub(super) unsafe fn f_11058af8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11058af8=>{
// 0x11058af8 str wzr, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11058afc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11058af8")),}}}
#[inline]
pub(super) unsafe fn f_111d15d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d15d0=>{
// 0x111d15d0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d15d4 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d15d8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d15dc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d15e0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d15e4 ldr x0, [x0, #0x50]
{let address=(cpu.x[0]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d15e8 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111d15ec cbz x0, #0x111d1604
if (cpu.x[0]) == 0 {pc=0x111d1604;continue;}pc=0x111d15f0;continue;
}
0x111d15f0=>{
// 0x111d15f0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d15f4 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d15f8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d15fc;super::dispatch(cpu,rt,target)?;}
pc=0x111d15fc;continue;
}
0x111d15fc=>{
// 0x111d15fc mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d1600 b #0x111d1608
pc=0x111d1608;continue;
}
0x111d1604=>{
// 0x111d1604 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
pc=0x111d1608;continue;
}
0x111d1608=>{
// 0x111d1608 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d160c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d1610 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1614 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d1618;super::dispatch(cpu,rt,target)?;}
pc=0x111d1618;continue;
}
0x111d1618=>{
// 0x111d1618 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d161c ldr w1, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d1620 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d1624 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1628 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d162c;super::dispatch(cpu,rt,target)?;}
pc=0x111d162c;continue;
}
0x111d162c=>{
// 0x111d162c ldr w8, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d1630 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d1634 b.lt #0x111d1664
if cpu.condition(11){pc=0x111d1664;continue;}pc=0x111d1638;continue;
}
0x111d1638=>{
// 0x111d1638 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x111d163c;continue;
}
0x111d163c=>{
// 0x111d163c ldr x8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1640 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d1644 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d1648 ldr w1, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d164c ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1650 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d1654;super::dispatch(cpu,rt,target)?;}
pc=0x111d1654;continue;
}
0x111d1654=>{
// 0x111d1654 ldrsw x8, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x111d1658 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x111d165c cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d1660 b.lt #0x111d163c
if cpu.condition(11){pc=0x111d163c;continue;}pc=0x111d1664;continue;
}
0x111d1664=>{
// 0x111d1664 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1668 ldr w1, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d166c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d1670 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1674 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d1678;super::dispatch(cpu,rt,target)?;}
pc=0x111d1678;continue;
}
0x111d1678=>{
// 0x111d1678 ldr w8, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d167c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d1680 b.lt #0x111d16b0
if cpu.condition(11){pc=0x111d16b0;continue;}pc=0x111d1684;continue;
}
0x111d1684=>{
// 0x111d1684 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x111d1688;continue;
}
0x111d1688=>{
// 0x111d1688 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d168c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d1690 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d1694 ldr w1, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d1698 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d169c blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d16a0;super::dispatch(cpu,rt,target)?;}
pc=0x111d16a0;continue;
}
0x111d16a0=>{
// 0x111d16a0 ldrsw x8, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x111d16a4 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x111d16a8 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d16ac b.lt #0x111d1688
if cpu.condition(11){pc=0x111d1688;continue;}pc=0x111d16b0;continue;
}
0x111d16b0=>{
// 0x111d16b0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d16b4 ldr w1, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d16b8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d16bc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d16c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d16c4;super::dispatch(cpu,rt,target)?;}
pc=0x111d16c4;continue;
}
0x111d16c4=>{
// 0x111d16c4 ldr w8, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d16c8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d16cc b.lt #0x111d16fc
if cpu.condition(11){pc=0x111d16fc;continue;}pc=0x111d16d0;continue;
}
0x111d16d0=>{
// 0x111d16d0 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x111d16d4;continue;
}
0x111d16d4=>{
// 0x111d16d4 ldr x8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d16d8 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d16dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d16e0 ldr w1, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d16e4 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d16e8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d16ec;super::dispatch(cpu,rt,target)?;}
// 0x111d16ec ldrsw x8, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x111d16f0 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x111d16f4 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d16f8 b.lt #0x111d16d4
if cpu.condition(11){pc=0x111d16d4;continue;}pc=0x111d16fc;continue;
}
0x111d16fc=>{
// 0x111d16fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d1700 ldrb w1, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d1704 ldr x2, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111d1708 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d170c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d1710 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d1714 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111d1718 br x2
{let target=cpu.x[2];if target>=0x111d15d0 && target<0x111d171c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d15d0")),}}}
#[inline]
pub(super) unsafe fn f_111d8948(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d8948=>{
// 0x111d8948 ldrb w9, [x0, #0x70]
{let address=(cpu.x[0]).wrapping_add(0x70u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d894c and w8, w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d8950 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8954 b.eq #0x111d8a74
if cpu.condition(0){pc=0x111d8a74;continue;}pc=0x111d8958;continue;
}
0x111d8958=>{
// 0x111d8958 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d895c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8960 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8964 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8968 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d896c ldp w9, w10, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x111d8970 ldr x21, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d8974 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d8978 strb w8, [x0, #0x70]
{let address=(cpu.x[0]).wrapping_add(0x70u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d897c strb wzr, [x0, #0x71]
{let address=(cpu.x[0]).wrapping_add(0x71u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d8980 mul w20, w10, w9
cpu.x[20]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x111d8984 str wzr, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8988 cmp w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d898c b.lt #0x111d89b8
if cpu.condition(11){pc=0x111d89b8;continue;}pc=0x111d8990;continue;
}
0x111d8990=>{
// 0x111d8990 ldr w8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d8994 cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8998 b.ge #0x111d89c8
if cpu.condition(10){pc=0x111d89c8;continue;}pc=0x111d899c;continue;
}
0x111d899c=>{
// 0x111d899c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d89a0 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x111d89a4 bl #0x1078f4d4
cpu.x[30]=0x111d89a8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111d89a8 ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d89ac cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d89b0 b.lt #0x111d89d4
if cpu.condition(11){pc=0x111d89d4;continue;}pc=0x111d89b4;continue;
}
0x111d89b4=>{
// 0x111d89b4 b #0x111d89ec
pc=0x111d89ec;continue;
}
0x111d89b8=>{
// 0x111d89b8 tbnz w20, #0x1f, #0x111d89ec
if ((cpu.x[20] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d89ec;continue;}pc=0x111d89bc;continue;
}
0x111d89bc=>{
// 0x111d89bc ldr x8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d89c0 str wzr, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d89c4 b #0x111d8a60
pc=0x111d8a60;continue;
}
0x111d89c8=>{
// 0x111d89c8 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x111d89cc cmp wzr, w20
{let a=((0u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d89d0 b.ge #0x111d89ec
if cpu.condition(10){pc=0x111d89ec;continue;}pc=0x111d89d4;continue;
}
0x111d89d4=>{
// 0x111d89d4 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d89d8 sub x10, x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x111d89dc mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d89e0 add x0, x9, w8, sxtw #2
{let a=((cpu.x[9]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111d89e4 lsl x2, x10, #2
cpu.x[2]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x111d89e8 bl #0x1187d480
cpu.x[30]=0x111d89ec;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x111d89ec;continue;
}
0x111d89ec=>{
// 0x111d89ec ldp w8, w9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x111d89f0 ldr x23, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d89f4 str w20, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d89f8 mul w22, w9, w8
cpu.x[22]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x111d89fc str wzr, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8a00 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8a04 b.lt #0x111d8a30
if cpu.condition(11){pc=0x111d8a30;continue;}pc=0x111d8a08;continue;
}
0x111d8a08=>{
// 0x111d8a08 ldr w8, [x23, #8]
{let address=(cpu.x[23]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d8a0c cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8a10 b.ge #0x111d8a38
if cpu.condition(10){pc=0x111d8a38;continue;}pc=0x111d8a14;continue;
}
0x111d8a14=>{
// 0x111d8a14 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x111d8a18 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d8a1c bl #0x1078f4d4
cpu.x[30]=0x111d8a20;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111d8a20 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d8a24 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8a28 b.lt #0x111d8a44
if cpu.condition(11){pc=0x111d8a44;continue;}pc=0x111d8a2c;continue;
}
0x111d8a2c=>{
// 0x111d8a2c b #0x111d8a5c
pc=0x111d8a5c;continue;
}
0x111d8a30=>{
// 0x111d8a30 tbz w22, #0x1f, #0x111d8a60
if ((cpu.x[22] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111d8a60;continue;}pc=0x111d8a34;continue;
}
0x111d8a34=>{
// 0x111d8a34 b #0x111d8a5c
pc=0x111d8a5c;continue;
}
0x111d8a38=>{
// 0x111d8a38 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x111d8a3c cmp wzr, w22
{let a=((0u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8a40 b.ge #0x111d8a5c
if cpu.condition(10){pc=0x111d8a5c;continue;}pc=0x111d8a44;continue;
}
0x111d8a44=>{
// 0x111d8a44 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d8a48 sub x10, x22, w8, sxtw
{let a=((cpu.x[22]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x111d8a4c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d8a50 add x0, x9, w8, sxtw #2
{let a=((cpu.x[9]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111d8a54 lsl x2, x10, #2
cpu.x[2]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x111d8a58 bl #0x1187d480
cpu.x[30]=0x111d8a5c;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x111d8a5c;continue;
}
0x111d8a5c=>{
// 0x111d8a5c str w22, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d8a60;continue;
}
0x111d8a60=>{
// 0x111d8a60 str wzr, [x19, #0x8c]
{let address=(cpu.x[19]).wrapping_add(0x8cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8a64 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d8a68 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d8a6c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d8a70 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
pc=0x111d8a74;continue;
}
0x111d8a74=>{
// 0x111d8a74 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d8948")),}}}
#[inline]
pub(super) unsafe fn f_111d8a78(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d8a78=>{
// 0x111d8a78 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d8a7c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8a80 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d8a84 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d8a88 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d8a8c ldr x0, [x20, #0x40]!
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x111d8a90 cbz x0, #0x111d8aa4
if (cpu.x[0]) == 0 {pc=0x111d8aa4;continue;}pc=0x111d8a94;continue;
}
0x111d8a94=>{
// 0x111d8a94 bl #0x111d8694
cpu.x[30]=0x111d8a98;super::chunk9::f_111d8694(cpu,rt,0x111d8694)?;
pc=0x111d8a98;continue;
}
0x111d8a98=>{
// 0x111d8a98 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d8a9c cbz x0, #0x111d8aa4
if (cpu.x[0]) == 0 {pc=0x111d8aa4;continue;}pc=0x111d8aa0;continue;
}
0x111d8aa0=>{
// 0x111d8aa0 bl #0x1187d2c0
cpu.x[30]=0x111d8aa4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x111d8aa4;continue;
}
0x111d8aa4=>{
// 0x111d8aa4 str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8aa8 stur xzr, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8aac strh wzr, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x111d8ab0 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8ab4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d8ab8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111d8abc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d8a78")),}}}
#[inline]
pub(super) unsafe fn f_111d8ac0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d8ac0=>{
// 0x111d8ac0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d8ac4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8ac8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d8acc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111d8ad0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d8ad4 add x1, x0, #0x74
{let a=((cpu.x[0]));let b=((0x74u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d8ad8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d8adc mov w2, #2
cpu.x[2]=(((0x2u64))) as u32 as u64;
// 0x111d8ae0 bl #0x111dd1ec
cpu.x[30]=0x111d8ae4;super::chunk3::f_111dd1ec(cpu,rt,0x111dd1ec)?;
pc=0x111d8ae4;continue;
}
0x111d8ae4=>{
// 0x111d8ae4 add x1, x20, #0x80
{let a=((cpu.x[20]));let b=((0x80u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d8ae8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d8aec mov w2, #-1
cpu.x[2]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x111d8af0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d8af4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111d8af8 b #0x111dd468
super::chunk3::f_111dd468(cpu,rt,0x111dd468)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d8ac0")),}}}
#[inline]
pub(super) unsafe fn f_111d9130(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d9130=>{
// 0x111d9130 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d9134 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9138 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d913c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d9140 ldr w9, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9144 add x12, x0, #0x50
{let a=((cpu.x[0]));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x111d9148 add x13, x0, #0x60
{let a=((cpu.x[0]));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x111d914c ldp w10, w11, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[11]=(v1) as u32 as u64;}
// 0x111d9150 eor w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x111d9154 mov w8, #0x60
cpu.x[8]=(((0x60u64))) as u32 as u64;
// 0x111d9158 strb wzr, [x0, #0x71]
{let address=(cpu.x[0]).wrapping_add(0x71u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d915c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9160 str w9, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d9164 mul w19, w11, w10
cpu.x[19]=((((cpu.x[11] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x111d9168 csel x20, x12, x13, eq
cpu.x[20]=(if cpu.condition(0){((cpu.x[12]))}else{((cpu.x[13]))}) as u64;
// 0x111d916c csel x9, x13, x12, eq
cpu.x[9]=(if cpu.condition(0){((cpu.x[13]))}else{((cpu.x[12]))}) as u64;
// 0x111d9170 stp x20, x9, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[20];let v1=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9174 mov w9, #0x50
cpu.x[9]=(((0x50u64))) as u32 as u64;
// 0x111d9178 csel x21, x9, x8, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[9]))}else{((cpu.x[8]))}) as u64;
// 0x111d917c str wzr, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d9180 cmp w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9184 b.lt #0x111d91b8
if cpu.condition(11){pc=0x111d91b8;continue;}pc=0x111d9188;continue;
}
0x111d9188=>{
// 0x111d9188 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d918c cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9190 b.ge #0x111d91cc
if cpu.condition(10){pc=0x111d91cc;continue;}pc=0x111d9194;continue;
}
0x111d9194=>{
// 0x111d9194 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x111d9198 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d919c mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x111d91a0 bl #0x1078f4d4
cpu.x[30]=0x111d91a4;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111d91a4 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d91a8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x111d91ac cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d91b0 b.lt #0x111d91d8
if cpu.condition(11){pc=0x111d91d8;continue;}pc=0x111d91b4;continue;
}
0x111d91b4=>{
// 0x111d91b4 b #0x111d91f0
pc=0x111d91f0;continue;
}
0x111d91b8=>{
// 0x111d91b8 tbnz w19, #0x1f, #0x111d91f0
if ((cpu.x[19] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d91f0;continue;}pc=0x111d91bc;continue;
}
0x111d91bc=>{
// 0x111d91bc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d91c0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d91c4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111d91c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111d91cc=>{
// 0x111d91cc mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x111d91d0 cmp wzr, w19
{let a=((0u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d91d4 b.ge #0x111d91f0
if cpu.condition(10){pc=0x111d91f0;continue;}pc=0x111d91d8;continue;
}
0x111d91d8=>{
// 0x111d91d8 ldr x9, [x0, x21]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d91dc sub x10, x19, w8, sxtw
{let a=((cpu.x[19]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x111d91e0 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d91e4 add x0, x9, w8, sxtw #2
{let a=((cpu.x[9]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111d91e8 lsl x2, x10, #2
cpu.x[2]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x111d91ec bl #0x1187d480
cpu.x[30]=0x111d91f0;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x111d91f0;continue;
}
0x111d91f0=>{
// 0x111d91f0 str w19, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d91f4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d91f8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d91fc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111d9200 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d9130")),}}}
#[inline]
pub(super) unsafe fn f_111d9240(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d9240=>{
// 0x111d9240 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x111d9244 orr w9, w2, w1
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x111d9248 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d924c tbnz w9, #0x1f, #0x111d9300
if ((cpu.x[9] as u32 as u64) & (1u64 << 31)) != 0 {{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d9250 ldr w9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9254 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9258 b.le #0x111d9300
if cpu.condition(13){{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d925c ldr w9, [x8, #4]
{let address=(cpu.x[8]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9260 cmp w9, w2
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9264 b.le #0x111d9300
if cpu.condition(13){{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d9268 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d926c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d9270 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9274 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9278 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d927c mov x20, x4
cpu.x[20]=(((cpu.x[4]))) as u64;
// 0x111d9280 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x111d9284 mov x23, x8
cpu.x[23]=(((cpu.x[8]))) as u64;
// 0x111d9288 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x111d928c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d9290 cbz x4, #0x111d92cc
if (cpu.x[4]) == 0 {{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d9294 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d929c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d92a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d92a4;super::dispatch(cpu,rt,target)?;}
{// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x111d92f0;continue;}}pc=0x111d92e4;continue;}}}pc=0x111d92ac;continue;}}}}}
}
0x111d9250=>{
// 0x111d9250 ldr w9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9254 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9258 b.le #0x111d9300
if cpu.condition(13){{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d925c ldr w9, [x8, #4]
{let address=(cpu.x[8]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9260 cmp w9, w2
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9264 b.le #0x111d9300
if cpu.condition(13){{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d9268 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d926c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d9270 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9274 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9278 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d927c mov x20, x4
cpu.x[20]=(((cpu.x[4]))) as u64;
// 0x111d9280 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x111d9284 mov x23, x8
cpu.x[23]=(((cpu.x[8]))) as u64;
// 0x111d9288 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x111d928c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d9290 cbz x4, #0x111d92cc
if (cpu.x[4]) == 0 {{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d9294 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d929c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d92a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d92a4;super::dispatch(cpu,rt,target)?;}
{// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
pc=0x111d9300;continue;}}}pc=0x111d92e4;continue;}}}pc=0x111d92ac;continue;}}}}
}
0x111d925c=>{
// 0x111d925c ldr w9, [x8, #4]
{let address=(cpu.x[8]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9260 cmp w9, w2
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9264 b.le #0x111d9300
if cpu.condition(13){{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x111d9268 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d926c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d9270 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9274 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9278 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d927c mov x20, x4
cpu.x[20]=(((cpu.x[4]))) as u64;
// 0x111d9280 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x111d9284 mov x23, x8
cpu.x[23]=(((cpu.x[8]))) as u64;
// 0x111d9288 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x111d928c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d9290 cbz x4, #0x111d92cc
if (cpu.x[4]) == 0 {{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d9294 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d929c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d92a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d92a4;super::dispatch(cpu,rt,target)?;}
{// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
pc=0x111d92f0;continue;}}}}pc=0x111d92ac;continue;}}}
}
0x111d9268=>{
// 0x111d9268 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d926c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d9270 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9274 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9278 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d927c mov x20, x4
cpu.x[20]=(((cpu.x[4]))) as u64;
// 0x111d9280 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x111d9284 mov x23, x8
cpu.x[23]=(((cpu.x[8]))) as u64;
// 0x111d9288 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x111d928c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d9290 cbz x4, #0x111d92cc
if (cpu.x[4]) == 0 {{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d9294 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d929c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d92a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d92a4;super::dispatch(cpu,rt,target)?;}
{// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92b0 bl #0x10f817a8
cpu.x[30]=0x111d92b4;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
{// 0x111d92b4 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92b8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x111d92f0;continue;}}pc=0x111d92d0;continue;}}pc=0x111d92bc;continue;}}}}
}
0x111d9294=>{
// 0x111d9294 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d929c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d92a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d92a4;super::dispatch(cpu,rt,target)?;}
{// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92b0 bl #0x10f817a8
cpu.x[30]=0x111d92b4;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
{// 0x111d92b4 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92b8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92c0 bl #0x10f7e050
cpu.x[30]=0x111d92c4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d92c4 bl #0x10de06d0
cpu.x[30]=0x111d92c8;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d92c8 cbnz x0, #0x111d92ec
if (cpu.x[0]) != 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x111d92f0;continue;}}{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x111d92f0;continue;}}pc=0x111d92d0;continue;}}}}}
}
0x111d92a4=>{
// 0x111d92a4 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92a8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92b0 bl #0x10f817a8
cpu.x[30]=0x111d92b4;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
{// 0x111d92b4 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92b8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92c0 bl #0x10f7e050
cpu.x[30]=0x111d92c4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d92c4 bl #0x10de06d0
cpu.x[30]=0x111d92c8;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d92c8 cbnz x0, #0x111d92ec
if (cpu.x[0]) != 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}pc=0x111d92cc;continue;}}}
}
0x111d92ac=>{
// 0x111d92ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92b0 bl #0x10f817a8
cpu.x[30]=0x111d92b4;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
{// 0x111d92b4 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92b8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92c0 bl #0x10f7e050
cpu.x[30]=0x111d92c4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d92c4 bl #0x10de06d0
cpu.x[30]=0x111d92c8;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d92c8 cbnz x0, #0x111d92ec
if (cpu.x[0]) != 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}}
}
0x111d92b4=>{
// 0x111d92b4 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d92b8 b.ne #0x111d92cc
if cpu.condition(1){{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}{// 0x111d92bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92c0 bl #0x10f7e050
cpu.x[30]=0x111d92c4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d92c4 bl #0x10de06d0
cpu.x[30]=0x111d92c8;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d92c8 cbnz x0, #0x111d92ec
if (cpu.x[0]) != 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}}
}
0x111d92bc=>{
// 0x111d92bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d92c0 bl #0x10f7e050
cpu.x[30]=0x111d92c4;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d92c4 bl #0x10de06d0
cpu.x[30]=0x111d92c8;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d92c8 cbnz x0, #0x111d92ec
if (cpu.x[0]) != 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}}
}
0x111d92cc=>{
// 0x111d92cc tbz w19, #0, #0x111d92ec
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}
}
0x111d92d0=>{
// 0x111d92d0 ldr x0, [x23, #0x48]
{let address=(cpu.x[23]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d92d4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d92d8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d92dc bl #0x10e8b6e0
cpu.x[30]=0x111d92e0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
// 0x111d92e0 tbz w0, #0, #0x111d92ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}}
}
0x111d92e4=>{
// 0x111d92e4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d92e8 b #0x111d92f0
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}
}
0x111d92ec=>{
// 0x111d92ec mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
{// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}}
}
0x111d92f0=>{
// 0x111d92f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d92f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d92f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d92fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
{// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x111d9300=>{
// 0x111d9300 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d9240")),}}}
#[inline]
pub(super) unsafe fn f_111d988c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d988c=>{
// 0x111d988c sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111d9890 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9894 stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d9898 stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d989c stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d98a0 stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d98a4 stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d98a8 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111d98ac ldp w8, w27, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[27]=(v1) as u32 as u64;}
// 0x111d98b0 mov w9, #0x4dd3
cpu.x[9]=(((0x4dd3u64))) as u32 as u64;
// 0x111d98b4 mov w28, #0x1f4
cpu.x[28]=(((0x1f4u64))) as u32 as u64;
// 0x111d98b8 mov x24, x4
cpu.x[24]=(((cpu.x[4]))) as u64;
// 0x111d98bc movk w9, #0x1062, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x111d98c0 mov w25, w3
cpu.x[25]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x111d98c4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111d98c8 smull x10, w8, w9
cpu.x[10]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x111d98cc mov w26, w2
cpu.x[26]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d98d0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d98d4 smull x9, w27, w9
cpu.x[9]=(((((cpu.x[27] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x111d98d8 stur w5, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[5] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d98dc lsr x11, x10, #0x3f
cpu.x[11]=(((((cpu.x[10]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x111d98e0 asr x10, x10, #0x25
cpu.x[10]=(((((cpu.x[10]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x111d98e4 add w22, w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x111d98e8 lsr x10, x9, #0x3f
cpu.x[10]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x111d98ec asr x9, x9, #0x25
cpu.x[9]=(((((cpu.x[9]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x111d98f0 msub w23, w22, w28, w8
cpu.x[23]=((((cpu.x[8] as u32 as u64))).wrapping_sub((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64))))) as u32 as u64;
// 0x111d98f4 add w8, w8, w2
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d98f8 add w11, w27, w3
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[3] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x111d98fc add w21, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x111d9900 stp w8, w11, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x111d9904 cbz x4, #0x111d9950
if (cpu.x[4]) == 0 {pc=0x111d9950;continue;}pc=0x111d9908;continue;
}
0x111d9908=>{
// 0x111d9908 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d990c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9910 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9914 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d9918;super::dispatch(cpu,rt,target)?;}
pc=0x111d9918;continue;
}
0x111d9918=>{
// 0x111d9918 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d991c b.ne #0x111d9950
if cpu.condition(1){pc=0x111d9950;continue;}pc=0x111d9920;continue;
}
0x111d9920=>{
// 0x111d9920 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9924 tbnz w8, #0, #0x111d9950
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d9950;continue;}pc=0x111d9928;continue;
}
0x111d9928=>{
// 0x111d9928 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d992c bl #0x10f817a8
cpu.x[30]=0x111d9930;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x111d9930;continue;
}
0x111d9930=>{
// 0x111d9930 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9934 b.ne #0x111d99dc
if cpu.condition(1){pc=0x111d99dc;continue;}pc=0x111d9938;continue;
}
0x111d9938=>{
// 0x111d9938 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d993c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9940 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9944 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d9948;super::dispatch(cpu,rt,target)?;}
pc=0x111d9948;continue;
}
0x111d9948=>{
// 0x111d9948 eor w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d994c stur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d9950;continue;
}
0x111d9950=>{
// 0x111d9950 msub w27, w21, w28, w27
cpu.x[27]=((((cpu.x[27] as u32 as u64))).wrapping_sub((((cpu.x[21] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64))))) as u32 as u64;
// 0x111d9954 add w8, w23, w26
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d9958 tbnz w26, #0x1f, #0x111d99ec
if ((cpu.x[26] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d99ec;continue;}pc=0x111d995c;continue;
}
0x111d995c=>{
// 0x111d995c cbz w26, #0x111d9ab8
if (cpu.x[26] as u32 as u64) == 0 {pc=0x111d9ab8;continue;}pc=0x111d9960;continue;
}
0x111d9960=>{
// 0x111d9960 cmp w8, #0x1f4
{let a=((cpu.x[8] as u32 as u64));let b=((0x1f4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9964 b.lt #0x111d9ab8
if cpu.condition(11){pc=0x111d9ab8;continue;}pc=0x111d9968;continue;
}
0x111d9968=>{
// 0x111d9968 add w26, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u32 as u64;}
// 0x111d996c orr w8, w21, w26
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9970 tbnz w8, #0x1f, #0x111d9a80
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d9a80;continue;}pc=0x111d9974;continue;
}
0x111d9974=>{
// 0x111d9974 ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9978 cmp w8, w26
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d997c b.le #0x111d9a80
if cpu.condition(13){pc=0x111d9a80;continue;}pc=0x111d9980;continue;
}
0x111d9980=>{
// 0x111d9980 ldr w8, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9984 cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9988 b.le #0x111d9a80
if cpu.condition(13){pc=0x111d9a80;continue;}pc=0x111d998c;continue;
}
0x111d998c=>{
// 0x111d998c cbz x24, #0x111d9a64
if (cpu.x[24]) == 0 {pc=0x111d9a64;continue;}pc=0x111d9990;continue;
}
0x111d9990=>{
// 0x111d9990 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9994 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9998 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d999c blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d99a0;super::dispatch(cpu,rt,target)?;}
pc=0x111d99a0;continue;
}
0x111d99a0=>{
// 0x111d99a0 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d99a4 b.ne #0x111d9a64
if cpu.condition(1){pc=0x111d9a64;continue;}pc=0x111d99a8;continue;
}
0x111d99a8=>{
// 0x111d99a8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d99ac bl #0x10f817a8
cpu.x[30]=0x111d99b0;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x111d99b0;continue;
}
0x111d99b0=>{
// 0x111d99b0 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d99b4 b.ne #0x111d9a64
if cpu.condition(1){pc=0x111d9a64;continue;}pc=0x111d99b8;continue;
}
0x111d99b8=>{
// 0x111d99b8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d99bc bl #0x10f7e050
cpu.x[30]=0x111d99c0;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d99c0 bl #0x10de06d0
cpu.x[30]=0x111d99c4;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d99c4 cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d99c8 ldur w9, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d99cc cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x111d99d0 and w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d99d4 tbnz w8, #0, #0x111d9a6c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d9a6c;continue;}pc=0x111d99d8;continue;
}
0x111d99d8=>{
// 0x111d99d8 b #0x111d9ab8
pc=0x111d9ab8;continue;
}
0x111d99dc=>{
// 0x111d99dc stur wzr, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d99e0 msub w27, w21, w28, w27
cpu.x[27]=((((cpu.x[27] as u32 as u64))).wrapping_sub((((cpu.x[21] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64))))) as u32 as u64;
// 0x111d99e4 add w8, w23, w26
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d99e8 tbz w26, #0x1f, #0x111d995c
if ((cpu.x[26] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111d995c;continue;}pc=0x111d99ec;continue;
}
0x111d99ec=>{
// 0x111d99ec tbz w8, #0x1f, #0x111d9ab8
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111d9ab8;continue;}pc=0x111d99f0;continue;
}
0x111d99f0=>{
// 0x111d99f0 sub w26, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u32 as u64;}
// 0x111d99f4 orr w8, w21, w26
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d99f8 tbnz w8, #0x1f, #0x111d9aac
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d9aac;continue;}pc=0x111d99fc;continue;
}
0x111d99fc=>{
// 0x111d99fc ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9a00 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9a04 b.lt #0x111d9aac
if cpu.condition(11){pc=0x111d9aac;continue;}pc=0x111d9a08;continue;
}
0x111d9a08=>{
// 0x111d9a08 ldr w8, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9a0c cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9a10 b.le #0x111d9aac
if cpu.condition(13){pc=0x111d9aac;continue;}pc=0x111d9a14;continue;
}
0x111d9a14=>{
// 0x111d9a14 cbz x24, #0x111d9a90
if (cpu.x[24]) == 0 {pc=0x111d9a90;continue;}pc=0x111d9a18;continue;
}
0x111d9a18=>{
// 0x111d9a18 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9a1c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9a20 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9a24 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d9a28;super::dispatch(cpu,rt,target)?;}
pc=0x111d9a28;continue;
}
0x111d9a28=>{
// 0x111d9a28 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9a2c b.ne #0x111d9a90
if cpu.condition(1){pc=0x111d9a90;continue;}pc=0x111d9a30;continue;
}
0x111d9a30=>{
// 0x111d9a30 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9a34 bl #0x10f817a8
cpu.x[30]=0x111d9a38;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x111d9a38;continue;
}
0x111d9a38=>{
// 0x111d9a38 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9a3c b.ne #0x111d9a90
if cpu.condition(1){pc=0x111d9a90;continue;}pc=0x111d9a40;continue;
}
0x111d9a40=>{
// 0x111d9a40 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9a44 bl #0x10f7e050
cpu.x[30]=0x111d9a48;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d9a48 bl #0x10de06d0
cpu.x[30]=0x111d9a4c;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d9a4c cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d9a50 ldur w9, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9a54 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x111d9a58 and w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9a5c tbnz w8, #0, #0x111d9a98
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d9a98;continue;}pc=0x111d9a60;continue;
}
0x111d9a60=>{
// 0x111d9a60 b #0x111d9ab8
pc=0x111d9ab8;continue;
}
0x111d9a64=>{
// 0x111d9a64 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9a68 tbz w8, #0, #0x111d9ab8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9ab8;continue;}pc=0x111d9a6c;continue;
}
0x111d9a6c=>{
// 0x111d9a6c ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d9a70 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x111d9a74 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d9a78 bl #0x10e8b6e0
cpu.x[30]=0x111d9a7c;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
pc=0x111d9a7c;continue;
}
0x111d9a7c=>{
// 0x111d9a7c tbz w0, #0, #0x111d9ab8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9ab8;continue;}pc=0x111d9a80;continue;
}
0x111d9a80=>{
// 0x111d9a80 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x111d9a84 mov w9, #0x1f3
cpu.x[9]=(((0x1f3u64))) as u32 as u64;
// 0x111d9a88 madd w8, w22, w8, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x111d9a8c b #0x111d9ab4
pc=0x111d9ab4;continue;
}
0x111d9a90=>{
// 0x111d9a90 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9a94 tbz w8, #0, #0x111d9ab8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9ab8;continue;}pc=0x111d9a98;continue;
}
0x111d9a98=>{
// 0x111d9a98 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d9a9c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x111d9aa0 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111d9aa4 bl #0x10e8b6e0
cpu.x[30]=0x111d9aa8;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
pc=0x111d9aa8;continue;
}
0x111d9aa8=>{
// 0x111d9aa8 tbz w0, #0, #0x111d9ab8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9ab8;continue;}pc=0x111d9aac;continue;
}
0x111d9aac=>{
// 0x111d9aac mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x111d9ab0 mul w8, w22, w8
cpu.x[8]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
pc=0x111d9ab4;continue;
}
0x111d9ab4=>{
// 0x111d9ab4 str w8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d9ab8;continue;
}
0x111d9ab8=>{
// 0x111d9ab8 add w8, w27, w25
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d9abc tbnz w25, #0x1f, #0x111d9b40
if ((cpu.x[25] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d9b40;continue;}pc=0x111d9ac0;continue;
}
0x111d9ac0=>{
// 0x111d9ac0 cbz w25, #0x111d9c0c
if (cpu.x[25] as u32 as u64) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9ac4;continue;
}
0x111d9ac4=>{
// 0x111d9ac4 cmp w8, #0x1f4
{let a=((cpu.x[8] as u32 as u64));let b=((0x1f4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9ac8 b.lt #0x111d9c0c
if cpu.condition(11){pc=0x111d9c0c;continue;}pc=0x111d9acc;continue;
}
0x111d9acc=>{
// 0x111d9acc add w25, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x111d9ad0 orr w8, w25, w22
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9ad4 tbnz w8, #0x1f, #0x111d9bd4
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d9bd4;continue;}pc=0x111d9ad8;continue;
}
0x111d9ad8=>{
// 0x111d9ad8 ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9adc cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9ae0 b.le #0x111d9bd4
if cpu.condition(13){pc=0x111d9bd4;continue;}pc=0x111d9ae4;continue;
}
0x111d9ae4=>{
// 0x111d9ae4 ldr w8, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9ae8 cmp w8, w25
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9aec b.le #0x111d9bd4
if cpu.condition(13){pc=0x111d9bd4;continue;}pc=0x111d9af0;continue;
}
0x111d9af0=>{
// 0x111d9af0 cbz x24, #0x111d9bb8
if (cpu.x[24]) == 0 {pc=0x111d9bb8;continue;}pc=0x111d9af4;continue;
}
0x111d9af4=>{
// 0x111d9af4 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9af8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9afc ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9b00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d9b04;super::dispatch(cpu,rt,target)?;}
pc=0x111d9b04;continue;
}
0x111d9b04=>{
// 0x111d9b04 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b08 b.ne #0x111d9bb8
if cpu.condition(1){pc=0x111d9bb8;continue;}pc=0x111d9b0c;continue;
}
0x111d9b0c=>{
// 0x111d9b0c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9b10 bl #0x10f817a8
cpu.x[30]=0x111d9b14;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x111d9b14;continue;
}
0x111d9b14=>{
// 0x111d9b14 cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b18 b.ne #0x111d9bb8
if cpu.condition(1){pc=0x111d9bb8;continue;}pc=0x111d9b1c;continue;
}
0x111d9b1c=>{
// 0x111d9b1c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9b20 bl #0x10f7e050
cpu.x[30]=0x111d9b24;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d9b24 bl #0x10de06d0
cpu.x[30]=0x111d9b28;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d9b28 cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d9b2c ldur w9, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9b30 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x111d9b34 and w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9b38 tbnz w8, #0, #0x111d9bc0
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d9bc0;continue;}pc=0x111d9b3c;continue;
}
0x111d9b3c=>{
// 0x111d9b3c b #0x111d9c0c
pc=0x111d9c0c;continue;
}
0x111d9b40=>{
// 0x111d9b40 tbz w8, #0x1f, #0x111d9c0c
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9b44;continue;
}
0x111d9b44=>{
// 0x111d9b44 sub w25, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[25]=(result) as u32 as u64;}
// 0x111d9b48 orr w8, w25, w22
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9b4c tbnz w8, #0x1f, #0x111d9c00
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d9c00;continue;}pc=0x111d9b50;continue;
}
0x111d9b50=>{
// 0x111d9b50 ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9b54 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b58 b.le #0x111d9c00
if cpu.condition(13){pc=0x111d9c00;continue;}pc=0x111d9b5c;continue;
}
0x111d9b5c=>{
// 0x111d9b5c ldr w8, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9b60 cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b64 b.lt #0x111d9c00
if cpu.condition(11){pc=0x111d9c00;continue;}pc=0x111d9b68;continue;
}
0x111d9b68=>{
// 0x111d9b68 cbz x24, #0x111d9be4
if (cpu.x[24]) == 0 {pc=0x111d9be4;continue;}pc=0x111d9b6c;continue;
}
0x111d9b6c=>{
// 0x111d9b6c ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9b70 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9b74 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d9b78 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d9b7c;super::dispatch(cpu,rt,target)?;}
pc=0x111d9b7c;continue;
}
0x111d9b7c=>{
// 0x111d9b7c cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b80 b.ne #0x111d9be4
if cpu.condition(1){pc=0x111d9be4;continue;}pc=0x111d9b84;continue;
}
0x111d9b84=>{
// 0x111d9b84 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9b88 bl #0x10f817a8
cpu.x[30]=0x111d9b8c;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x111d9b8c;continue;
}
0x111d9b8c=>{
// 0x111d9b8c cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9b90 b.ne #0x111d9be4
if cpu.condition(1){pc=0x111d9be4;continue;}pc=0x111d9b94;continue;
}
0x111d9b94=>{
// 0x111d9b94 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d9b98 bl #0x10f7e050
cpu.x[30]=0x111d9b9c;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x111d9b9c bl #0x10de06d0
cpu.x[30]=0x111d9ba0;super::chunk7::f_10de06d0(cpu,rt,0x10de06d0)?;
// 0x111d9ba0 cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d9ba4 ldur w9, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9ba8 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x111d9bac and w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9bb0 tbnz w8, #0, #0x111d9bec
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d9bec;continue;}pc=0x111d9bb4;continue;
}
0x111d9bb4=>{
// 0x111d9bb4 b #0x111d9c0c
pc=0x111d9c0c;continue;
}
0x111d9bb8=>{
// 0x111d9bb8 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9bbc tbz w8, #0, #0x111d9c0c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9bc0;continue;
}
0x111d9bc0=>{
// 0x111d9bc0 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d9bc4 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d9bc8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d9bcc bl #0x10e8b6e0
cpu.x[30]=0x111d9bd0;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
pc=0x111d9bd0;continue;
}
0x111d9bd0=>{
// 0x111d9bd0 tbz w0, #0, #0x111d9c0c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9bd4;continue;
}
0x111d9bd4=>{
// 0x111d9bd4 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x111d9bd8 mov w9, #0x1f3
cpu.x[9]=(((0x1f3u64))) as u32 as u64;
// 0x111d9bdc madd w8, w21, w8, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[21] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x111d9be0 b #0x111d9c08
pc=0x111d9c08;continue;
}
0x111d9be4=>{
// 0x111d9be4 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9be8 tbz w8, #0, #0x111d9c0c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9bec;continue;
}
0x111d9bec=>{
// 0x111d9bec ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d9bf0 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d9bf4 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d9bf8 bl #0x10e8b6e0
cpu.x[30]=0x111d9bfc;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
pc=0x111d9bfc;continue;
}
0x111d9bfc=>{
// 0x111d9bfc tbz w0, #0, #0x111d9c0c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d9c0c;continue;}pc=0x111d9c00;continue;
}
0x111d9c00=>{
// 0x111d9c00 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x111d9c04 mul w8, w21, w8
cpu.x[8]=((((cpu.x[21] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
pc=0x111d9c08;continue;
}
0x111d9c08=>{
// 0x111d9c08 str w8, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d9c0c;continue;
}
0x111d9c0c=>{
// 0x111d9c0c ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d9c10 ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d9c14 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x111d9c18 ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x111d9c1c ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x111d9c20 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111d9c24 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111d9c28 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d988c")),}}}
#[inline]
pub(super) unsafe fn f_111dc5a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dc5a8=>{
// 0x111dc5a8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dc5ac str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dc5b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dc5b4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dc5b8 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111dc5bc bl #0x110cbf10
cpu.x[30]=0x111dc5c0;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x111dc5c0;continue;
}
0x111dc5c0=>{
// 0x111dc5c0 bl #0x10e16208
cpu.x[30]=0x111dc5c4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x111dc5c4;continue;
}
0x111dc5c4=>{
// 0x111dc5c4 ldrb w8, [x0, #0x7a6]
{let address=(cpu.x[0]).wrapping_add(0x7a6u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111dc5c8 ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dc5cc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dc5d0 csel x0, xzr, x9, eq
cpu.x[0]=(if cpu.condition(0){((0u64))}else{((cpu.x[9]))}) as u64;
// 0x111dc5d4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111dc5d8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111dc5dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dc5a8")),}}}
#[inline]
pub(super) unsafe fn f_111dd424(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd424=>{
// 0x111dd424 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd428 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd42c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd430 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd434 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dd438 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111dd43c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd440 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd444;super::dispatch(cpu,rt,target)?;}
pc=0x111dd444;continue;
}
0x111dd444=>{
// 0x111dd444 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd448 str w0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd44c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111dd450 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd454 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd458;super::dispatch(cpu,rt,target)?;}
pc=0x111dd458;continue;
}
0x111dd458=>{
// 0x111dd458 str w0, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd45c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd460 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111dd464 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd424")),}}}
#[inline]
pub(super) unsafe fn f_111ddf84(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111ddf84=>{
// 0x111ddf84 sub sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111ddf88 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111ddf8c str x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111ddf90 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111ddf94 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111ddf98 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111ddf9c add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111ddfa0 ldr x8, [x1, #0x18]
{let address=(cpu.x[1]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111ddfa4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111ddfa8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111ddfac cbz x8, #0x111ddff0
if (cpu.x[8]) == 0 {pc=0x111ddff0;continue;}pc=0x111ddfb0;continue;
}
0x111ddfb0=>{
// 0x111ddfb0 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111ddfb4 cbz x0, #0x111ddfc8
if (cpu.x[0]) == 0 {pc=0x111ddfc8;continue;}pc=0x111ddfb8;continue;
}
0x111ddfb8=>{
// 0x111ddfb8 ldr x21, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111ddfbc bl #0x1187d2c0
cpu.x[30]=0x111ddfc0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x111ddfc0;continue;
}
0x111ddfc0=>{
// 0x111ddfc0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111ddfc4 cbnz x21, #0x111ddfb8
if (cpu.x[21]) != 0 {pc=0x111ddfb8;continue;}pc=0x111ddfc8;continue;
}
0x111ddfc8=>{
// 0x111ddfc8 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111ddfcc str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111ddfd0 cbz x8, #0x111ddfec
if (cpu.x[8]) == 0 {pc=0x111ddfec;continue;}pc=0x111ddfd4;continue;
}
0x111ddfd4=>{
// 0x111ddfd4 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x111ddfd8;continue;
}
0x111ddfd8=>{
// 0x111ddfd8 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x111ddfdc str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111ddfe0 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x111ddfe4 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111ddfe8 b.ne #0x111ddfd8
if cpu.condition(1){pc=0x111ddfd8;continue;}pc=0x111ddfec;continue;
}
0x111ddfec=>{
// 0x111ddfec str xzr, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x111ddff0;continue;
}
0x111ddff0=>{
// 0x111ddff0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111ddff4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111ddff8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111ddffc blr x8
{let target=cpu.x[8];cpu.x[30]=0x111de000;super::dispatch(cpu,rt,target)?;}
pc=0x111de000;continue;
}
0x111de000=>{
// 0x111de000 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111de004 b.lt #0x111de06c
if cpu.condition(11){pc=0x111de06c;continue;}pc=0x111de008;continue;
}
0x111de008=>{
// 0x111de008 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111de00c add x25, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x111de010 adrp x22, #0x1033b000
cpu.x[22]=(0x1033b000u64) as u64;
// 0x111de014 add x22, x22, #0x836
{let a=((cpu.x[22]));let b=((0x836u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
pc=0x111de018;continue;
}
0x111de018=>{
// 0x111de018 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111de01c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111de020 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111de024 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111de028;super::dispatch(cpu,rt,target)?;}
pc=0x111de028;continue;
}
0x111de028=>{
// 0x111de028 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111de02c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111de030 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111de034 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111de038 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111de03c;super::dispatch(cpu,rt,target)?;}
pc=0x111de03c;continue;
}
0x111de03c=>{
// 0x111de03c mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111de040 add x1, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111de044 add x3, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[3]=(result) as u64;}
// 0x111de048 sub x4, x29, #1
{let a=((cpu.x[29]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[4]=(result) as u64;}
// 0x111de04c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111de050 mov x2, x22
cpu.x[2]=(((cpu.x[22]))) as u64;
// 0x111de054 str w23, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111de058 str x25, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111de05c bl #0x10790864
cpu.x[30]=0x111de060;super::chunk1::f_10790864(cpu,rt,0x10790864)?;
pc=0x111de060;continue;
}
0x111de060=>{
// 0x111de060 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111de064 str w24, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111de068 b.ne #0x111de018
if cpu.condition(1){pc=0x111de018;continue;}pc=0x111de06c;continue;
}
0x111de06c=>{
// 0x111de06c ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111de070 ldr x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x111de074 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111de078 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x111de07c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111de080 add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111de084 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111ddf84")),}}}
#[inline]
pub(super) unsafe fn f_11289eb4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11289eb4=>{
// 0x11289eb4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11289eb8 ldr w2, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x11289ebc mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x11289ec0 b #0x1128a854
super::chunk8::f_1128a854(cpu,rt,0x1128a854)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11289eb4")),}}}
#[inline]
pub(super) unsafe fn f_1128a6e4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128a6e4=>{
// 0x1128a6e4 sub sp, sp, #0x30
{let a=((cpu.sp));let b=((0x30u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x1128a6e8 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128a6ec stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128a6f0 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x1128a6f4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1128a6f8 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x1128a6fc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x1128a700 bl #0x1187d2d0
cpu.x[30]=0x1128a704;super::dispatch(cpu,rt,0x1187d2d0)?;
pc=0x1128a704;continue;
}
0x1128a704=>{
// 0x1128a704 stp x19, x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[19];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128a708 mov x1, sp
cpu.x[1]=(((cpu.sp))) as u64;
// 0x1128a70c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1128a710 bl #0x1128b714
cpu.x[30]=0x1128a714;super::chunk15::f_1128b714(cpu,rt,0x1128b714)?;
pc=0x1128a714;continue;
}
0x1128a714=>{
// 0x1128a714 cbz x0, #0x1128a72c
if (cpu.x[0]) == 0 {pc=0x1128a72c;continue;}pc=0x1128a718;continue;
}
0x1128a718=>{
// 0x1128a718 ldr w0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x1128a71c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128a720 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x1128a724 add sp, sp, #0x30
{let a=((cpu.sp));let b=((0x30u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x1128a728 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128a72c=>{
// 0x1128a72c mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x1128a730 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128a734 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x1128a738 add sp, sp, #0x30
{let a=((cpu.sp));let b=((0x30u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x1128a73c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128a6e4")),}}}
#[inline]
pub(super) unsafe fn f_1128bc24(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128bc24=>{
// 0x1128bc24 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128bc28 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1128bc2c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128bc30 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128bc34 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128bc38 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x1128bc3c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1128bc40 bl #0x1128dbb0
cpu.x[30]=0x1128bc44;super::chunk1::f_1128dbb0(cpu,rt,0x1128dbb0)?;
pc=0x1128bc44;continue;
}
0x1128bc44=>{
// 0x1128bc44 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x1128bc48 bl #0x112ada18
cpu.x[30]=0x1128bc4c;super::chunk7::f_112ada18(cpu,rt,0x112ada18)?;
pc=0x1128bc4c;continue;
}
0x1128bc4c=>{
// 0x1128bc4c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x1128bc50 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1128bc54 bl #0x112aef98
cpu.x[30]=0x1128bc58;super::chunk15::f_112aef98(cpu,rt,0x112aef98)?;
pc=0x1128bc58;continue;
}
0x1128bc58=>{
// 0x1128bc58 mov w8, #0xbba1
cpu.x[8]=(((0xbba1u64))) as u32 as u64;
// 0x1128bc5c movk w8, #0xd, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0xd0000u64) as u32 as u64;
// 0x1128bc60 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bc64 b.lt #0x1128bc84
if cpu.condition(11){pc=0x1128bc84;continue;}pc=0x1128bc68;continue;
}
0x1128bc68=>{
// 0x1128bc68 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1128bc6c mov w1, #-1
cpu.x[1]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x1128bc70 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128bc74 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bc78 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128bc7c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1128bc80 b #0x1128bd30
super::chunk10::f_1128bd30(cpu,rt,0x1128bd30)?;return Ok(());
}
0x1128bc84=>{
// 0x1128bc84 ldr w22, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x1128bc88 add w8, w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bc8c ldr w9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128bc90 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x1128bc94 add w10, w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128bc98 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bc9c b.le #0x1128bcec
if cpu.condition(13){pc=0x1128bcec;continue;}pc=0x1128bca0;continue;
}
0x1128bca0=>{
// 0x1128bca0 add w10, w9, w9, lsr #1
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shr(1) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128bca4 cmp w9, #0x100
{let a=((cpu.x[9] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bca8 mov w11, #0x64
cpu.x[11]=(((0x64u64))) as u32 as u64;
// 0x1128bcac add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bcb0 ldr x23, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bcb4 csel w10, w11, w10, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x1128bcb8 add w8, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bcbc sxtw x0, w8
cpu.x[0]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1128bcc0 str w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bcc4 bl #0x1187d520
cpu.x[30]=0x1128bcc8;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
// 0x1128bcc8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x1128bccc mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x1128bcd0 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128bcd4 mov w4, w22
cpu.x[4]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x1128bcd8 stp x0, x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128bcdc bl #0x112aef80
cpu.x[30]=0x1128bce0;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
// 0x1128bce0 cbz x23, #0x1128bcec
if (cpu.x[23]) == 0 {pc=0x1128bcec;continue;}pc=0x1128bce4;continue;
}
0x1128bce4=>{
// 0x1128bce4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x1128bce8 bl #0x1187d360
cpu.x[30]=0x1128bcec;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1128bcec;continue;
}
0x1128bcec=>{
// 0x1128bcec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1128bcf0 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1128bcf4 bl #0x1128bd30
cpu.x[30]=0x1128bcf8;super::chunk10::f_1128bd30(cpu,rt,0x1128bd30)?;
pc=0x1128bcf8;continue;
}
0x1128bcf8=>{
// 0x1128bcf8 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1128bcfc ldr w1, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x1128bd00 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x1128bd04 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128bd08 mov w4, w21
cpu.x[4]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1128bd0c bl #0x112aef80
cpu.x[30]=0x1128bd10;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
pc=0x1128bd10;continue;
}
0x1128bd10=>{
// 0x1128bd10 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128bd14 add w8, w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bd18 str w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bd1c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128bd20 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bd24 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128bd28 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1128bd2c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128bc24")),}}}
#[inline]
pub(super) unsafe fn f_1128dbb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128dbb0=>{
// 0x1128dbb0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128dbb4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128dbb8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128dbbc ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128dbc0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1128dbc4 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x1128dbc8 ror w20, w8, #0x1f
cpu.x[20]=(((((cpu.x[8] as u32 as u64))) as u32).rotate_right(((((0x1fu64))) & 31) as u32) as u64) as u32 as u64;
// 0x1128dbcc bl #0x112ae7fc
cpu.x[30]=0x1128dbd0;super::chunk12::f_112ae7fc(cpu,rt,0x112ae7fc)?;
pc=0x1128dbd0;continue;
}
0x1128dbd0=>{
// 0x1128dbd0 add w8, w0, w20
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128dbd4 add w8, w8, #0x26
{let a=((cpu.x[8] as u32 as u64));let b=((0x26u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128dbd8 str w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128dbdc ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128dbe0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x1128dbe4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128dbb0")),}}}
#[inline]
pub(super) unsafe fn f_1128de2c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128de2c=>{
// 0x1128de2c mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x1128de30 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x1128de34 mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x1128de38 b #0x112942c8
super::chunk0::f_112942c8(cpu,rt,0x112942c8)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128de2c")),}}}
#[inline]
pub(super) unsafe fn f_1128de58(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128de58=>{
// 0x1128de58 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x1128de5c stp wzr, wzr, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=0u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x1128de60 strb w8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x1128de64 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128de58")),}}}
#[inline]
pub(super) unsafe fn f_112941e4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112941e4=>{
// 0x112941e4 stp w1, w2, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x112941e8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112941e4")),}}}
#[inline]
pub(super) unsafe fn f_112948b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112948b4=>{
// 0x112948b4 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112948b8 mov w8, #0x35
cpu.x[8]=(((0x35u64))) as u32 as u64;
// 0x112948bc cneg w9, w0, mi
cpu.x[9]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x112948c0 cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112948c4 cneg w10, w1, mi
cpu.x[10]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[1] as u32 as u64)))}else{((cpu.x[1] as u32 as u64))}) as u32 as u64;
// 0x112948c8 cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112948cc csel w11, w9, w10, lt
cpu.x[11]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x112948d0 csel w9, w9, w10, gt
cpu.x[9]=(if cpu.condition(12){((cpu.x[9] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x112948d4 mul w8, w11, w8
cpu.x[8]=((((cpu.x[11] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x112948d8 add w0, w9, w8, lsr #7
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as u32).wrapping_shr(7) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x112948dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112948b4")),}}}
#[inline]
pub(super) unsafe fn f_301329f8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x301329f8=>{
// 0x301329f8 adrp x16, #0x30142000
cpu.x[16]=(0x30142000u64) as u64;
// 0x301329fc ldr x17, [x16, #0x98]
{let address=(cpu.x[16]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x30132a00 add x16, x16, #0x98
{let a=((cpu.x[16]));let b=((0x98u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x30132a04 br x17
{let target=cpu.x[17];if target>=0x301329f8 && target<0x30132a08{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x301329f8")),}}}
