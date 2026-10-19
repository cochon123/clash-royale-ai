#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_1078f4d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1078f4d4=>{
// 0x1078f4d4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1078f4d8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1078f4dc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1078f4e0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1078f4e4 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x1078f4e8 cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1078f4ec mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1078f4f0 ubfiz x8, x19, #2, #0x20
cpu.x[8]=(((((cpu.x[19])) & 0xffffffffu64) << 2)) as u64;
// 0x1078f4f4 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x1078f4f8 bl #0x1187d520
cpu.x[30]=0x1078f4fc;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x1078f4fc;continue;
}
0x1078f4fc=>{
// 0x1078f4fc ldr w9, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1078f500 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1078f504 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x1078f508 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1078f50c b.lt #0x1078f594
if cpu.condition(11){pc=0x1078f594;continue;}pc=0x1078f510;continue;
}
0x1078f510=>{
// 0x1078f510 cmp w9, #8
{let a=((cpu.x[9] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1078f514 b.lo #0x1078f574
if cpu.condition(3){pc=0x1078f574;continue;}pc=0x1078f518;continue;
}
0x1078f518=>{
// 0x1078f518 sub x10, x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x1078f51c mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
// 0x1078f520 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1078f524 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x1078f528 b.lo #0x1078f57c
if cpu.condition(3){pc=0x1078f57c;continue;}pc=0x1078f52c;continue;
}
0x1078f52c=>{
// 0x1078f52c mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x1078f530 add x14, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x1078f534 add x15, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x1078f538 and x13, x12, #0x7ffffff8
{let a=((cpu.x[12]));let b=((0x7ffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x1078f53c lsl x11, x13, #2
cpu.x[11]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x1078f540 sub w9, w9, w13
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1078f544 mov x16, x13
cpu.x[16]=(((cpu.x[13]))) as u64;
// 0x1078f548 add x10, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x1078f54c add x11, x21, x11
{let a=((cpu.x[21]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
pc=0x1078f550;continue;
}
0x1078f550=>{
// 0x1078f550 ldp q0, q1, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x1078f554 subs x16, x16, #8
{let a=((cpu.x[16]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x1078f558 add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x1078f55c stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x1078f560 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x1078f564 b.ne #0x1078f550
if cpu.condition(1){pc=0x1078f550;continue;}pc=0x1078f568;continue;
}
0x1078f568=>{
// 0x1078f568 cmp x13, x12
{let a=((cpu.x[13]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1078f56c b.ne #0x1078f57c
if cpu.condition(1){pc=0x1078f57c;continue;}pc=0x1078f570;continue;
}
0x1078f570=>{
// 0x1078f570 b #0x1078f594
pc=0x1078f594;continue;
}
0x1078f574=>{
// 0x1078f574 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x1078f578 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
pc=0x1078f57c;continue;
}
0x1078f57c=>{
// 0x1078f57c add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
pc=0x1078f580;continue;
}
0x1078f580=>{
// 0x1078f580 ldr w12, [x10], #4
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x4u64)))) as u64;}
// 0x1078f584 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1078f588 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1078f58c str w12, [x11], #4
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x4u64)))) as u64;}
// 0x1078f590 b.hi #0x1078f580
if cpu.condition(8){pc=0x1078f580;continue;}pc=0x1078f594;continue;
}
0x1078f594=>{
// 0x1078f594 cbz x8, #0x1078f5a0
if (cpu.x[8]) == 0 {pc=0x1078f5a0;continue;}pc=0x1078f598;continue;
}
0x1078f598=>{
// 0x1078f598 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x1078f59c bl #0x1187d360
cpu.x[30]=0x1078f5a0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1078f5a0;continue;
}
0x1078f5a0=>{
// 0x1078f5a0 str x21, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1078f5a4 str w19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1078f5a8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1078f5ac ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1078f5b0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x1078f5b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1078f4d4")),}}}
#[inline]
pub(super) unsafe fn f_1083472c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1083472c=>{
// 0x1083472c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1083472c")),}}}
#[inline]
pub(super) unsafe fn f_10d22840(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d22840=>{
// 0x10d22840 ldr x9, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d22844 mov x8, x1
cpu.x[8]=(((cpu.x[1]))) as u64;
// 0x10d22848 ldr w1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2284c mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d22850 ldr x2, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d22854 br x2
{let target=cpu.x[2];if target>=0x10d22840 && target<0x10d22858{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d22840")),}}}
#[inline]
pub(super) unsafe fn f_10d39c14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d39c14=>{
// 0x10d39c14 sub sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d39c18 stp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c1c stp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c20 stp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c24 stp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c28 stp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c2c stp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d39c30 add x29, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d39c34 ldr x8, [x0, #0xb8]
{let address=(cpu.x[0]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39c38 mov w22, w3
cpu.x[22]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d39c3c ldr x3, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d39c40 mov x21, x2
cpu.x[21]=(((cpu.x[2]))) as u64;
// 0x10d39c44 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d39c48 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10d39c4c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d39c50 mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x10d39c54 mov w2, #0xf
cpu.x[2]=(((0xfu64))) as u32 as u64;
// 0x10d39c58 bl #0x111dd77c
cpu.x[30]=0x10d39c5c;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d39c5c;continue;
}
0x10d39c5c=>{
// 0x10d39c5c ldr x0, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39c60 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d39c64 bl #0x10d17ea8
cpu.x[30]=0x10d39c68;super::chunk4::f_10d17ea8(cpu,rt,0x10d17ea8)?;
pc=0x10d39c68;continue;
}
0x10d39c68=>{
// 0x10d39c68 ldr w8, [x19, #0x36c]
{let address=(cpu.x[19]).wrapping_add(0x36cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39c6c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39c70 b.lt #0x10d39cac
if cpu.condition(11){pc=0x10d39cac;continue;}pc=0x10d39c74;continue;
}
0x10d39c74=>{
// 0x10d39c74 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d39c78 b #0x10d39c88
pc=0x10d39c88;continue;
}
0x10d39c7c=>{
// 0x10d39c7c add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d39c80 cmp x23, w8, sxtw
{let a=((cpu.x[23]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d39c84 b.ge #0x10d39cac
if cpu.condition(10){pc=0x10d39cac;continue;}pc=0x10d39c88;continue;
}
0x10d39c88=>{
// 0x10d39c88 ldr x9, [x19, #0x360]
{let address=(cpu.x[19]).wrapping_add(0x360u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39c8c ldr x0, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39c90 cbz x0, #0x10d39c7c
if (cpu.x[0]) == 0 {pc=0x10d39c7c;continue;}pc=0x10d39c94;continue;
}
0x10d39c94=>{
// 0x10d39c94 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39c98 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d39c9c ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39ca0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39ca4;super::dispatch(cpu,rt,target)?;}
pc=0x10d39ca4;continue;
}
0x10d39ca4=>{
// 0x10d39ca4 ldr w8, [x19, #0x36c]
{let address=(cpu.x[19]).wrapping_add(0x36cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39ca8 b #0x10d39c7c
pc=0x10d39c7c;continue;
}
0x10d39cac=>{
// 0x10d39cac ldr x0, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39cb0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d39cb4 bl #0x10d55d28
cpu.x[30]=0x10d39cb8;super::chunk4::f_10d55d28(cpu,rt,0x10d55d28)?;
pc=0x10d39cb8;continue;
}
0x10d39cb8=>{
// 0x10d39cb8 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39cbc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39cc0 b.lt #0x10d39d18
if cpu.condition(11){pc=0x10d39d18;continue;}pc=0x10d39cc4;continue;
}
0x10d39cc4=>{
// 0x10d39cc4 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d39cc8 add x24, x19, #0x7c
{let a=((cpu.x[19]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
pc=0x10d39ccc;continue;
}
0x10d39ccc=>{
// 0x10d39ccc add x8, x24, x23, lsl #3
{let a=((cpu.x[24]));let b=(((cpu.x[23])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d39cd0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39cd4 ldur x1, [x8, #-0x4c]
{let address=(cpu.x[8]).wrapping_add(0xffffffffffffffb4u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d39cd8 bl #0x111dd3d0
cpu.x[30]=0x10d39cdc;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
pc=0x10d39cdc;continue;
}
0x10d39cdc=>{
// 0x10d39cdc add x8, x24, x23, lsl #2
{let a=((cpu.x[24]));let b=(((cpu.x[23])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d39ce0 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39ce4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39ce8 ldur w1, [x8, #-0x18]
{let address=(cpu.x[8]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39cec ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39cf0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39cf4;super::dispatch(cpu,rt,target)?;}
pc=0x10d39cf4;continue;
}
0x10d39cf4=>{
// 0x10d39cf4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39cf8 ldrb w1, [x24, x23]
{let address=((cpu.x[24]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39cfc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d00 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d04 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39d08;super::dispatch(cpu,rt,target)?;}
pc=0x10d39d08;continue;
}
0x10d39d08=>{
// 0x10d39d08 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d39d0c add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d39d10 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d39d14 b.lt #0x10d39ccc
if cpu.condition(11){pc=0x10d39ccc;continue;}pc=0x10d39d18;continue;
}
0x10d39d18=>{
// 0x10d39d18 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39d1c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d39d20 bl #0x111d8ac0
cpu.x[30]=0x10d39d24;super::chunk1::f_111d8ac0(cpu,rt,0x111d8ac0)?;
pc=0x10d39d24;continue;
}
0x10d39d24=>{
// 0x10d39d24 ldr x1, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d39d28 ldr x3, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d39d2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d30 mov w2, #0x12
cpu.x[2]=(((0x12u64))) as u32 as u64;
// 0x10d39d34 bl #0x111dd77c
cpu.x[30]=0x10d39d38;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d39d38;continue;
}
0x10d39d38=>{
// 0x10d39d38 ldr x1, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d39d3c ldr x3, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d39d40 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d44 mov w2, #0xa9
cpu.x[2]=(((0xa9u64))) as u32 as u64;
// 0x10d39d48 bl #0x111dd77c
cpu.x[30]=0x10d39d4c;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d39d4c;continue;
}
0x10d39d4c=>{
// 0x10d39d4c ldr x1, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d39d50 ldr x3, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d39d54 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d58 mov w2, #0xa9
cpu.x[2]=(((0xa9u64))) as u32 as u64;
// 0x10d39d5c bl #0x111dd77c
cpu.x[30]=0x10d39d60;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d39d60;continue;
}
0x10d39d60=>{
// 0x10d39d60 add x1, x19, #0x168
{let a=((cpu.x[19]));let b=((0x168u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d39d64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d68 mov w2, #8
cpu.x[2]=(((0x8u64))) as u32 as u64;
// 0x10d39d6c bl #0x111dd1ec
cpu.x[30]=0x10d39d70;super::chunk3::f_111dd1ec(cpu,rt,0x111dd1ec)?;
pc=0x10d39d70;continue;
}
0x10d39d70=>{
// 0x10d39d70 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d74 ldrb w1, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39d78 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d7c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39d84;super::dispatch(cpu,rt,target)?;}
pc=0x10d39d84;continue;
}
0x10d39d84=>{
// 0x10d39d84 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d88 ldrb w1, [x19, #0x1e1]
{let address=(cpu.x[19]).wrapping_add(0x1e1u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39d8c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39d90 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d94 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39d98;super::dispatch(cpu,rt,target)?;}
pc=0x10d39d98;continue;
}
0x10d39d98=>{
// 0x10d39d98 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39d9c ldrb w1, [x19, #0x1e2]
{let address=(cpu.x[19]).wrapping_add(0x1e2u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39da0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39da4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39da8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39dac;super::dispatch(cpu,rt,target)?;}
pc=0x10d39dac;continue;
}
0x10d39dac=>{
// 0x10d39dac ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39db0 ldrb w1, [x19, #0x31c]
{let address=(cpu.x[19]).wrapping_add(0x31cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39db4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39db8 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39dbc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39dc0;super::dispatch(cpu,rt,target)?;}
pc=0x10d39dc0;continue;
}
0x10d39dc0=>{
// 0x10d39dc0 ldr w8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39dc4 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39dc8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39dcc cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39dd0 ldr x9, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39dd4 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10d39dd8 orr w25, w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a | b;cpu.x[25]=(result) as u32 as u64;}
// 0x10d39ddc ands w1, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;cpu.logical_flags(result,32);}
// 0x10d39de0 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10d39de4;super::dispatch(cpu,rt,target)?;}
pc=0x10d39de4;continue;
}
0x10d39de4=>{
// 0x10d39de4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39de8 ldr w9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d39dec tst w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d39df0 mov w10, #4
cpu.x[10]=(((0x4u64))) as u32 as u64;
// 0x10d39df4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39df8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39dfc csel w1, w10, w9, ne
cpu.x[1]=(if cpu.condition(1){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d39e00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39e04;super::dispatch(cpu,rt,target)?;}
pc=0x10d39e04;continue;
}
0x10d39e04=>{
// 0x10d39e04 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e08 ldr w1, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39e0c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39e10 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39e18;super::dispatch(cpu,rt,target)?;}
pc=0x10d39e18;continue;
}
0x10d39e18=>{
// 0x10d39e18 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e1c ldr w1, [x19, #0x1b8]
{let address=(cpu.x[19]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39e20 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39e24 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e28 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39e2c;super::dispatch(cpu,rt,target)?;}
pc=0x10d39e2c;continue;
}
0x10d39e2c=>{
// 0x10d39e2c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e30 ldr w1, [x19, #0x320]
{let address=(cpu.x[19]).wrapping_add(0x320u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39e34 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39e38 ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e3c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39e40;super::dispatch(cpu,rt,target)?;}
pc=0x10d39e40;continue;
}
0x10d39e40=>{
// 0x10d39e40 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e44 ldr w1, [x19, #0x324]
{let address=(cpu.x[19]).wrapping_add(0x324u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39e48 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d39e4c ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39e50 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39e54;super::dispatch(cpu,rt,target)?;}
pc=0x10d39e54;continue;
}
0x10d39e54=>{
// 0x10d39e54 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d39e58 str wzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d39e5c strh wzr, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10d39e60 str wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d39e64 strh wzr, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10d39e68 stp q0, q0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d39e6c str q0, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d39e70 cbz x21, #0x10d39ee8
if (cpu.x[21]) == 0 {pc=0x10d39ee8;continue;}pc=0x10d39e74;continue;
}
0x10d39e74=>{
// 0x10d39e74 ldr w8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39e78 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39e7c b.eq #0x10d39e84
if cpu.condition(0){pc=0x10d39e84;continue;}pc=0x10d39e80;continue;
}
0x10d39e80=>{
// 0x10d39e80 cbnz w8, #0x10d39ee8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d39ee8;continue;}pc=0x10d39e84;continue;
}
0x10d39e84=>{
// 0x10d39e84 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d39e88 bl #0x10d22e78
cpu.x[30]=0x10d39e8c;super::chunk0::f_10d22e78(cpu,rt,0x10d22e78)?;
// 0x10d39e8c ldr w10, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d39e90 ldp w8, w9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10d39e94 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39e98 b.lt #0x10d39efc
if cpu.condition(11){pc=0x10d39efc;continue;}pc=0x10d39e9c;continue;
}
0x10d39e9c=>{
// 0x10d39e9c mov x11, xzr
cpu.x[11]=(((0u64))) as u64;
// 0x10d39ea0 add x12, x19, #0x30
{let a=((cpu.x[19]));let b=((0x30u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10d39ea4 b #0x10d39eb4
pc=0x10d39eb4;continue;
}
0x10d39ea8=>{
// 0x10d39ea8 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10d39eac cmp x10, x11
{let a=((cpu.x[10]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d39eb0 b.eq #0x10d39efc
if cpu.condition(0){pc=0x10d39efc;continue;}pc=0x10d39eb4;continue;
}
0x10d39eb4=>{
// 0x10d39eb4 ldr x13, [x12, x11, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[11])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10d39eb8 ldr w14, [x13]
{let address=(cpu.x[13]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10d39ebc cmp w14, w8
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39ec0 b.ne #0x10d39ea8
if cpu.condition(1){pc=0x10d39ea8;continue;}pc=0x10d39ec4;continue;
}
0x10d39ec4=>{
// 0x10d39ec4 ldr w13, [x13, #4]
{let address=(cpu.x[13]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10d39ec8 cmp w13, w9
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39ecc b.ne #0x10d39ea8
if cpu.condition(1){pc=0x10d39ea8;continue;}pc=0x10d39ed0;continue;
}
0x10d39ed0=>{
// 0x10d39ed0 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d39ed4 bic w24, w8, w11
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a & !b;cpu.x[24]=(result) as u32 as u64;}
// 0x10d39ed8 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39edc ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39ee0 cbnz x0, #0x10d39f24
if (cpu.x[0]) != 0 {pc=0x10d39f24;continue;}pc=0x10d39ee4;continue;
}
0x10d39ee4=>{
// 0x10d39ee4 b #0x10d39f2c
pc=0x10d39f2c;continue;
}
0x10d39ee8=>{
// 0x10d39ee8 mov w24, #-1
cpu.x[24]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d39eec ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39ef0 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39ef4 cbnz x0, #0x10d39f24
if (cpu.x[0]) != 0 {pc=0x10d39f24;continue;}pc=0x10d39ef8;continue;
}
0x10d39ef8=>{
// 0x10d39ef8 b #0x10d39f2c
pc=0x10d39f2c;continue;
}
0x10d39efc=>{
// 0x10d39efc ldp w11, w10, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[11]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x10d39f00 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39f04 ccmp w11, w8, #0, eq
if cpu.condition(0){{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10d39f08 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10d39f0c csinv w11, w8, wzr, eq
cpu.x[11]=(if cpu.condition(0){((cpu.x[8] as u32 as u64))}else{!(((0u64)))}) as u32 as u64;
// 0x10d39f10 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d39f14 bic w24, w8, w11
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a & !b;cpu.x[24]=(result) as u32 as u64;}
// 0x10d39f18 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39f1c ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39f20 cbz x0, #0x10d39f2c
if (cpu.x[0]) == 0 {pc=0x10d39f2c;continue;}pc=0x10d39f24;continue;
}
0x10d39f24=>{
// 0x10d39f24 ldrb w8, [x0, #0x118]
{let address=(cpu.x[0]).wrapping_add(0x118u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39f28 cbnz w8, #0x10d39f48
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d39f48;continue;}pc=0x10d39f2c;continue;
}
0x10d39f2c=>{
// 0x10d39f2c ldrb w8, [x0, #0x181]
{let address=(cpu.x[0]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39f30 cbnz w8, #0x10d39f48
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d39f48;continue;}pc=0x10d39f34;continue;
}
0x10d39f34=>{
// 0x10d39f34 ldr w8, [x0, #0x184]
{let address=(cpu.x[0]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39f38 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39f3c b.gt #0x10d39f48
if cpu.condition(12){pc=0x10d39f48;continue;}pc=0x10d39f40;continue;
}
0x10d39f40=>{
// 0x10d39f40 bl #0x10e2d82c
cpu.x[30]=0x10d39f44;super::chunk0::f_10e2d82c(cpu,rt,0x10e2d82c)?;
pc=0x10d39f44;continue;
}
0x10d39f44=>{
// 0x10d39f44 tbz w0, #0, #0x10d39fc0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d39fc0;continue;}pc=0x10d39f48;continue;
}
0x10d39f48=>{
// 0x10d39f48 tbz w25, #0, #0x10d39f64
if ((cpu.x[25] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d39f64;continue;}pc=0x10d39f4c;continue;
}
0x10d39f4c=>{
// 0x10d39f4c ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39f50 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39f54 ldrb w9, [x8, #0x181]
{let address=(cpu.x[8]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d39f58 cbz w9, #0x10d39f70
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10d39f70;continue;}pc=0x10d39f5c;continue;
}
0x10d39f5c=>{
// 0x10d39f5c mov w27, #-1
cpu.x[27]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d39f60 b #0x10d3a1ec
pc=0x10d3a1ec;continue;
}
0x10d39f64=>{
// 0x10d39f64 mov w26, wzr
cpu.x[26]=(((0u64))) as u32 as u64;
// 0x10d39f68 mov w27, #-1
cpu.x[27]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d39f6c b #0x10d3a3c0
pc=0x10d3a3c0;continue;
}
0x10d39f70=>{
// 0x10d39f70 ldr w8, [x8, #0x184]
{let address=(cpu.x[8]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39f74 mov w27, #-1
cpu.x[27]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d39f78 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39f7c b.gt #0x10d3a1ec
if cpu.condition(12){pc=0x10d3a1ec;continue;}pc=0x10d39f80;continue;
}
0x10d39f80=>{
// 0x10d39f80 cmn w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10d39f84 b.eq #0x10d3a1ec
if cpu.condition(0){pc=0x10d3a1ec;continue;}pc=0x10d39f88;continue;
}
0x10d39f88=>{
// 0x10d39f88 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39f8c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39f90 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d39f94;continue;
}
0x10d39f94=>{
// 0x10d39f94 ldr x22, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d39f98 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d39f9c bl #0x10f94124
cpu.x[30]=0x10d39fa0;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d39fa0 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39fa4 b.ne #0x10d39fe8
if cpu.condition(1){pc=0x10d39fe8;continue;}pc=0x10d39fa8;continue;
}
0x10d39fa8=>{
// 0x10d39fa8 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39fac mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d39fb0 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d39fb4 strb w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d39fb8 bl #0x10f9c5c0
cpu.x[30]=0x10d39fbc;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d39fbc b #0x10d39fec
pc=0x10d39fec;continue;
}
0x10d39fc0=>{
// 0x10d39fc0 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39fc4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39fc8 ldr w8, [x8, #0x74]
{let address=(cpu.x[8]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39fcc cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39fd0 b.eq #0x10d39f48
if cpu.condition(0){pc=0x10d39f48;continue;}pc=0x10d39fd4;continue;
}
0x10d39fd4=>{
// 0x10d39fd4 ldr x8, [x19, #0x2a0]
{let address=(cpu.x[19]).wrapping_add(0x2a0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39fd8 cbnz x8, #0x10d39f48
if (cpu.x[8]) != 0 {pc=0x10d39f48;continue;}pc=0x10d39fdc;continue;
}
0x10d39fdc=>{
// 0x10d39fdc cmn w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10d39fe0 b.ne #0x10d39f88
if cpu.condition(1){pc=0x10d39f88;continue;}pc=0x10d39fe4;continue;
}
0x10d39fe4=>{
// 0x10d39fe4 b #0x10d39f5c
pc=0x10d39f5c;continue;
}
0x10d39fe8=>{
// 0x10d39fe8 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d39fec;continue;
}
0x10d39fec=>{
// 0x10d39fec ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d39ff0 str x22, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d39ff4 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d39ff8 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d39ffc;continue;
}
0x10d39ffc=>{
// 0x10d39ffc ldr x22, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a000 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a004 bl #0x10f94124
cpu.x[30]=0x10d3a008;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a008 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a00c b.ne #0x10d3a028
if cpu.condition(1){pc=0x10d3a028;continue;}pc=0x10d3a010;continue;
}
0x10d3a010=>{
// 0x10d3a010 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a014 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a018 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a01c strb w8, [sp, #9]
{let address=(cpu.sp).wrapping_add(0x9u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a020 bl #0x10f9c5c0
cpu.x[30]=0x10d3a024;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a024 b #0x10d3a02c
pc=0x10d3a02c;continue;
}
0x10d3a028=>{
// 0x10d3a028 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d3a02c;continue;
}
0x10d3a02c=>{
// 0x10d3a02c ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a030 str x22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a034 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a038 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d3a03c;continue;
}
0x10d3a03c=>{
// 0x10d3a03c ldr x22, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a040 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a044 bl #0x10f94124
cpu.x[30]=0x10d3a048;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a048 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a04c b.ne #0x10d3a068
if cpu.condition(1){pc=0x10d3a068;continue;}pc=0x10d3a050;continue;
}
0x10d3a050=>{
// 0x10d3a050 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a054 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a058 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a05c strb w8, [sp, #0xa]
{let address=(cpu.sp).wrapping_add(0xau64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a060 bl #0x10f9c5c0
cpu.x[30]=0x10d3a064;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a064 b #0x10d3a06c
pc=0x10d3a06c;continue;
}
0x10d3a068=>{
// 0x10d3a068 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d3a06c;continue;
}
0x10d3a06c=>{
// 0x10d3a06c ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a070 str x22, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a074 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a078 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d3a07c;continue;
}
0x10d3a07c=>{
// 0x10d3a07c ldr x22, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a080 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a084 bl #0x10f94124
cpu.x[30]=0x10d3a088;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a088 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a08c b.ne #0x10d3a0a8
if cpu.condition(1){pc=0x10d3a0a8;continue;}pc=0x10d3a090;continue;
}
0x10d3a090=>{
// 0x10d3a090 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a094 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a098 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a09c strb w8, [sp, #0xb]
{let address=(cpu.sp).wrapping_add(0xbu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a0a0 bl #0x10f9c5c0
cpu.x[30]=0x10d3a0a4;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a0a4 b #0x10d3a0ac
pc=0x10d3a0ac;continue;
}
0x10d3a0a8=>{
// 0x10d3a0a8 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d3a0ac;continue;
}
0x10d3a0ac=>{
// 0x10d3a0ac ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a0b0 str x22, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a0b4 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a0b8 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d3a0bc;continue;
}
0x10d3a0bc=>{
// 0x10d3a0bc ldr x22, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a0c0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a0c4 bl #0x10f94124
cpu.x[30]=0x10d3a0c8;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a0c8 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a0cc b.ne #0x10d3a0e8
if cpu.condition(1){pc=0x10d3a0e8;continue;}pc=0x10d3a0d0;continue;
}
0x10d3a0d0=>{
// 0x10d3a0d0 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a0d4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a0d8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a0dc strb w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a0e0 bl #0x10f9c5c0
cpu.x[30]=0x10d3a0e4;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a0e4 b #0x10d3a0ec
pc=0x10d3a0ec;continue;
}
0x10d3a0e8=>{
// 0x10d3a0e8 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d3a0ec;continue;
}
0x10d3a0ec=>{
// 0x10d3a0ec ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a0f0 str x22, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a0f4 cmp w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a0f8 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d3a0fc;continue;
}
0x10d3a0fc=>{
// 0x10d3a0fc ldr x22, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a100 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a104 bl #0x10f94124
cpu.x[30]=0x10d3a108;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a108 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a10c b.ne #0x10d3a128
if cpu.condition(1){pc=0x10d3a128;continue;}pc=0x10d3a110;continue;
}
0x10d3a110=>{
// 0x10d3a110 ldrb w8, [x22, #0x316]
{let address=(cpu.x[22]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a114 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a118 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a11c strb w8, [sp, #0xd]
{let address=(cpu.sp).wrapping_add(0xdu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a120 bl #0x10f9c5c0
cpu.x[30]=0x10d3a124;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a124 b #0x10d3a12c
pc=0x10d3a12c;continue;
}
0x10d3a128=>{
// 0x10d3a128 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10d3a12c;continue;
}
0x10d3a12c=>{
// 0x10d3a12c ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a130 str x22, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a134 cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a138 b.lt #0x10d3a1e8
if cpu.condition(11){pc=0x10d3a1e8;continue;}pc=0x10d3a13c;continue;
}
0x10d3a13c=>{
// 0x10d3a13c sub x8, x29, #0x10
{let a=((cpu.x[29]));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10d3a140 mov w22, #6
cpu.x[22]=(((0x6u64))) as u32 as u64;
// 0x10d3a144 add x27, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d3a148 add x28, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10d3a14c add x26, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d3a150 adrp x23, #0x102e0000
cpu.x[23]=(0x102e0000u64) as u64;
// 0x10d3a154 add x23, x23, #0x6ab
{let a=((cpu.x[23]));let b=((0x6abu64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d3a158 b #0x10d3a170
pc=0x10d3a170;continue;
}
0x10d3a15c=>{
// 0x10d3a15c ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3a160 str xzr, [x26, x22, lsl #3]
{let address=((cpu.x[26]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3a164 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3a168 cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a16c b.ge #0x10d3a1e8
if cpu.condition(10){pc=0x10d3a1e8;continue;}pc=0x10d3a170;continue;
}
0x10d3a170=>{
// 0x10d3a170 cmp x22, #0x64
{let a=((cpu.x[22]));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a174 b.ne #0x10d3a18c
if cpu.condition(1){pc=0x10d3a18c;continue;}pc=0x10d3a178;continue;
}
0x10d3a178=>{
// 0x10d3a178 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10d3a17c bl #0x10f94124
cpu.x[30]=0x10d3a180;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a180 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a184 b.ne #0x10d3a15c
if cpu.condition(1){pc=0x10d3a15c;continue;}pc=0x10d3a188;continue;
}
0x10d3a188=>{
// 0x10d3a188 b #0x10d3a1cc
pc=0x10d3a1cc;continue;
}
0x10d3a18c=>{
// 0x10d3a18c sub x8, x29, #0x10
{let a=((cpu.x[29]));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10d3a190 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a194 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d3a198 bl #0x114328cc
cpu.x[30]=0x10d3a19c;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x10d3a19c ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a1a0 ldur x9, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3a1a4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3a1a8 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a1ac csel x0, x28, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[28]))}else{((cpu.x[9]))}) as u64;
// 0x10d3a1b0 bl #0x1128df18
cpu.x[30]=0x10d3a1b4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d3a1b4 sub x0, x29, #0x10
{let a=((cpu.x[29]));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10d3a1b8 bl #0x1142fcc4
cpu.x[30]=0x10d3a1bc;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d3a1bc mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10d3a1c0 bl #0x10f94124
cpu.x[30]=0x10d3a1c4;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10d3a1c4 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a1c8 b.ne #0x10d3a15c
if cpu.condition(1){pc=0x10d3a15c;continue;}pc=0x10d3a1cc;continue;
}
0x10d3a1cc=>{
// 0x10d3a1cc mov w8, #0x316
cpu.x[8]=(((0x316u64))) as u32 as u64;
// 0x10d3a1d0 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10d3a1d4 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a1d8 ldrb w8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a1dc strb w8, [x27, x22]
{let address=((cpu.x[27]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a1e0 bl #0x10f9c5c0
cpu.x[30]=0x10d3a1e4;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a1e4 b #0x10d3a15c
pc=0x10d3a15c;continue;
}
0x10d3a1e8=>{
// 0x10d3a1e8 mov w27, w24
cpu.x[27]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
pc=0x10d3a1ec;continue;
}
0x10d3a1ec=>{
// 0x10d3a1ec ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a1f0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a1f4 cset w8, gt
cpu.x[8]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10d3a1f8 and w8, w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10d3a1fc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a200 b.ne #0x10d3a3bc
if cpu.condition(1){pc=0x10d3a3bc;continue;}pc=0x10d3a204;continue;
}
0x10d3a204=>{
// 0x10d3a204 ldr x8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a208 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a20c ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a210 bl #0x110cc5b4
cpu.x[30]=0x10d3a214;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a214 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a218 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a21c bl #0x10d395b4
cpu.x[30]=0x10d3a220;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a220 ldrb w26, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10d3a224 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a228 strb w26, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a22c bl #0x10f9c5c0
cpu.x[30]=0x10d3a230;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a230 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a234 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a238 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a23c;continue;
}
0x10d3a23c=>{
// 0x10d3a23c ldr x8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a240 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a244 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a248 bl #0x110cc5b4
cpu.x[30]=0x10d3a24c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a24c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a250 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a254 bl #0x10d395b4
cpu.x[30]=0x10d3a258;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a258 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a25c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a260 strb w8, [sp, #1]
{let address=(cpu.sp).wrapping_add(0x1u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a264 bl #0x10f9c5c0
cpu.x[30]=0x10d3a268;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a268 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a26c cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a270 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a274;continue;
}
0x10d3a274=>{
// 0x10d3a274 ldr x8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a278 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a27c ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a280 bl #0x110cc5b4
cpu.x[30]=0x10d3a284;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a284 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a288 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a28c bl #0x10d395b4
cpu.x[30]=0x10d3a290;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a290 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a294 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a298 strb w8, [sp, #2]
{let address=(cpu.sp).wrapping_add(0x2u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a29c bl #0x10f9c5c0
cpu.x[30]=0x10d3a2a0;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a2a0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a2a4 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a2a8 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a2ac;continue;
}
0x10d3a2ac=>{
// 0x10d3a2ac ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a2b0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a2b4 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a2b8 bl #0x110cc5b4
cpu.x[30]=0x10d3a2bc;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a2bc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a2c0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a2c4 bl #0x10d395b4
cpu.x[30]=0x10d3a2c8;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a2c8 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a2cc mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a2d0 strb w8, [sp, #3]
{let address=(cpu.sp).wrapping_add(0x3u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a2d4 bl #0x10f9c5c0
cpu.x[30]=0x10d3a2d8;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a2d8 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a2dc cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a2e0 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a2e4;continue;
}
0x10d3a2e4=>{
// 0x10d3a2e4 ldr x8, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a2e8 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a2ec ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a2f0 bl #0x110cc5b4
cpu.x[30]=0x10d3a2f4;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a2f4 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a2f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a2fc bl #0x10d395b4
cpu.x[30]=0x10d3a300;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a300 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a304 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a308 strb w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a30c bl #0x10f9c5c0
cpu.x[30]=0x10d3a310;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a310 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a314 cmp w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a318 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a31c;continue;
}
0x10d3a31c=>{
// 0x10d3a31c ldr x8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a320 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a324 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a328 bl #0x110cc5b4
cpu.x[30]=0x10d3a32c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a32c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a330 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a334 bl #0x10d395b4
cpu.x[30]=0x10d3a338;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a338 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a33c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a340 strb w8, [sp, #5]
{let address=(cpu.sp).wrapping_add(0x5u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a344 bl #0x10f9c5c0
cpu.x[30]=0x10d3a348;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a348 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a34c cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a350 b.lt #0x10d3a3c0
if cpu.condition(11){pc=0x10d3a3c0;continue;}pc=0x10d3a354;continue;
}
0x10d3a354=>{
// 0x10d3a354 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x10d3a358 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10d3a35c adrp x22, #0x102d1000
cpu.x[22]=(0x102d1000u64) as u64;
// 0x10d3a360 add x22, x22, #0xfd6
{let a=((cpu.x[22]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3a364 add x23, x8, #6
{let a=((cpu.x[8]));let b=((0x6u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
pc=0x10d3a368;continue;
}
0x10d3a368=>{
// 0x10d3a368 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a36c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3a370 bl #0x1128df18
cpu.x[30]=0x10d3a374;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d3a374 add x8, x19, x24, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[24])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3a378 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a37c ldr x8, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a380 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a384 bl #0x110cc5b4
cpu.x[30]=0x10d3a388;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a388 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a38c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a390 bl #0x10d395b4
cpu.x[30]=0x10d3a394;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a394 ldrb w8, [x0, #0x316]
{let address=(cpu.x[0]).wrapping_add(0x316u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a398 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a39c strb w8, [x23, x24]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[24])));let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3a3a0 bl #0x10f9c5c0
cpu.x[30]=0x10d3a3a4;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a3a4 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3a3a8 add x9, x24, #7
{let a=((cpu.x[24]));let b=((0x7u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d3a3ac add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d3a3b0 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a3b4 b.lt #0x10d3a368
if cpu.condition(11){pc=0x10d3a368;continue;}pc=0x10d3a3b8;continue;
}
0x10d3a3b8=>{
// 0x10d3a3b8 b #0x10d3a3c0
pc=0x10d3a3c0;continue;
}
0x10d3a3bc=>{
// 0x10d3a3bc mov w26, wzr
cpu.x[26]=(((0u64))) as u32 as u64;
pc=0x10d3a3c0;continue;
}
0x10d3a3c0=>{
// 0x10d3a3c0 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a3c4 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a3c8 ldr x22, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d3a3cc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a3d0 bl #0x10f96afc
cpu.x[30]=0x10d3a3d4;super::chunk11::f_10f96afc(cpu,rt,0x10f96afc)?;
pc=0x10d3a3d4;continue;
}
0x10d3a3d4=>{
// 0x10d3a3d4 eor w28, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[28]=(result) as u32 as u64;}
// 0x10d3a3d8 tbnz w28, #0, #0x10d3a440
if ((cpu.x[28] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3a440;continue;}pc=0x10d3a3dc;continue;
}
0x10d3a3dc=>{
// 0x10d3a3dc ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a3e0 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a3e4 ldrb w9, [x8, #0x181]
{let address=(cpu.x[8]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d3a3e8 cbnz w9, #0x10d3a3f8
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10d3a3f8;continue;}pc=0x10d3a3ec;continue;
}
0x10d3a3ec=>{
// 0x10d3a3ec ldr w8, [x8, #0x184]
{let address=(cpu.x[8]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a3f0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a3f4 b.lt #0x10d3a440
if cpu.condition(11){pc=0x10d3a440;continue;}pc=0x10d3a3f8;continue;
}
0x10d3a3f8=>{
// 0x10d3a3f8 cbz w27, #0x10d3a4e4
if (cpu.x[27] as u32 as u64) == 0 {pc=0x10d3a4e4;continue;}pc=0x10d3a3fc;continue;
}
0x10d3a3fc=>{
// 0x10d3a3fc ldr x8, [x19, #0x270]
{let address=(cpu.x[19]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a400 ldr x23, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d3a404 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a408 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a40c ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a410 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a414;super::dispatch(cpu,rt,target)?;}
// 0x10d3a414 tbz w0, #0, #0x10d3a4fc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a4fc;continue;}pc=0x10d3a418;continue;
}
0x10d3a418=>{
// 0x10d3a418 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a41c bl #0x10fec370
cpu.x[30]=0x10d3a420;super::dispatch(cpu,rt,0x10fec370)?;
// 0x10d3a420 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a424 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3a428 and w1, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10d3a42c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a430 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a434 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a438;super::dispatch(cpu,rt,target)?;}
// 0x10d3a438 tbnz w24, #0, #0x10d3a510
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3a510;continue;}pc=0x10d3a43c;continue;
}
0x10d3a43c=>{
// 0x10d3a43c b #0x10d3a558
pc=0x10d3a558;continue;
}
0x10d3a440=>{
// 0x10d3a440 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a444 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a448 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a44c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a450;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a450;continue;
}
0x10d3a450=>{
// 0x10d3a450 tbz w0, #0, #0x10d3a4c0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a4c0;continue;}pc=0x10d3a454;continue;
}
0x10d3a454=>{
// 0x10d3a454 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a458 bl #0x10fec370
cpu.x[30]=0x10d3a45c;super::dispatch(cpu,rt,0x10fec370)?;
// 0x10d3a45c cbz w27, #0x10d3a4c8
if (cpu.x[27] as u32 as u64) == 0 {pc=0x10d3a4c8;continue;}pc=0x10d3a460;continue;
}
0x10d3a460=>{
// 0x10d3a460 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a464 and w23, w0, w28
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a & b;cpu.x[23]=(result) as u32 as u64;}
// 0x10d3a468 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a46c and w1, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10d3a470 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a474 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a478;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a478;continue;
}
0x10d3a478=>{
// 0x10d3a478 tbz w23, #0, #0x10d3a4b8
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a4b8;continue;}pc=0x10d3a47c;continue;
}
0x10d3a47c=>{
// 0x10d3a47c ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a480 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a484 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a488 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a48c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a490;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a490;continue;
}
0x10d3a490=>{
// 0x10d3a490 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a494 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a498 ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a49c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a4a0;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a4a0;continue;
}
0x10d3a4a0=>{
// 0x10d3a4a0 tbz w0, #0, #0x10d3a4b8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a4b8;continue;}pc=0x10d3a4a4;continue;
}
0x10d3a4a4=>{
// 0x10d3a4a4 ldr x0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a4a8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a4ac ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4b0 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a4b8;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a4b8;continue;
}
0x10d3a4b8=>{
// 0x10d3a4b8 tbnz w25, #0, #0x10d3a558
if ((cpu.x[25] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3a558;continue;}pc=0x10d3a4bc;continue;
}
0x10d3a4bc=>{
// 0x10d3a4bc b #0x10d3a5c0
pc=0x10d3a5c0;continue;
}
0x10d3a4c0=>{
// 0x10d3a4c0 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10d3a4c4 cbnz w27, #0x10d3a460
if (cpu.x[27] as u32 as u64) != 0 {pc=0x10d3a460;continue;}pc=0x10d3a4c8;continue;
}
0x10d3a4c8=>{
// 0x10d3a4c8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a4d0 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a4d4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a4dc;super::dispatch(cpu,rt,target)?;}
// 0x10d3a4dc tbz w25, #0, #0x10d3a5c0
if ((cpu.x[25] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a5c0;continue;}pc=0x10d3a4e0;continue;
}
0x10d3a4e0=>{
// 0x10d3a4e0 b #0x10d3a558
pc=0x10d3a558;continue;
}
0x10d3a4e4=>{
// 0x10d3a4e4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4e8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a4ec mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a4f0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a4f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a4f8;super::dispatch(cpu,rt,target)?;}
// 0x10d3a4f8 b #0x10d3a558
pc=0x10d3a558;continue;
}
0x10d3a4fc=>{
// 0x10d3a4fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a500 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a504 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3a508 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a50c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a510;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a510;continue;
}
0x10d3a510=>{
// 0x10d3a510 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a514 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a518 ldr x8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a51c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a520;super::dispatch(cpu,rt,target)?;}
// 0x10d3a520 eor w24, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[24]=(result) as u32 as u64;}
// 0x10d3a524 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a528 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a52c and w2, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d3a530 bl #0x10feabb8
cpu.x[30]=0x10d3a534;super::chunk11::f_10feabb8(cpu,rt,0x10feabb8)?;
// 0x10d3a534 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a538 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a53c ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a540 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a544;super::dispatch(cpu,rt,target)?;}
// 0x10d3a544 tbz w0, #0, #0x10d3a558
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a558;continue;}pc=0x10d3a548;continue;
}
0x10d3a548=>{
// 0x10d3a548 ldr x0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a54c and w2, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d3a550 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a554 bl #0x10feabb8
cpu.x[30]=0x10d3a558;super::chunk11::f_10feabb8(cpu,rt,0x10feabb8)?;
pc=0x10d3a558;continue;
}
0x10d3a558=>{
// 0x10d3a558 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a55c ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a560 ldrb w9, [x8, #0x181]
{let address=(cpu.x[8]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d3a564 cbnz w9, #0x10d3a574
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10d3a574;continue;}pc=0x10d3a568;continue;
}
0x10d3a568=>{
// 0x10d3a568 ldr w8, [x8, #0x184]
{let address=(cpu.x[8]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a56c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a570 b.le #0x10d3a5c0
if cpu.condition(13){pc=0x10d3a5c0;continue;}pc=0x10d3a574;continue;
}
0x10d3a574=>{
// 0x10d3a574 cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a578 b.eq #0x10d3a64c
if cpu.condition(0){pc=0x10d3a64c;continue;}pc=0x10d3a57c;continue;
}
0x10d3a57c=>{
// 0x10d3a57c ldr x8, [x19, #0x278]
{let address=(cpu.x[19]).wrapping_add(0x278u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a580 ldr x23, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d3a584 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a588 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a58c ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a590 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a594;super::dispatch(cpu,rt,target)?;}
// 0x10d3a594 tbz w0, #0, #0x10d3ae2c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ae2c;continue;}pc=0x10d3a598;continue;
}
0x10d3a598=>{
// 0x10d3a598 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3a59c bl #0x10fec370
cpu.x[30]=0x10d3a5a0;super::dispatch(cpu,rt,0x10fec370)?;
// 0x10d3a5a0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5a4 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3a5a8 and w1, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10d3a5ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a5b0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a5b8;super::dispatch(cpu,rt,target)?;}
// 0x10d3a5b8 tbz w24, #0, #0x10d3a660
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a660;continue;}pc=0x10d3a5bc;continue;
}
0x10d3a5bc=>{
// 0x10d3a5bc b #0x10d3ae40
pc=0x10d3ae40;continue;
}
0x10d3a5c0=>{
// 0x10d3a5c0 ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a5c4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5c8 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a5d0;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a5d0;continue;
}
0x10d3a5d0=>{
// 0x10d3a5d0 tbz w0, #0, #0x10d3a640
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a640;continue;}pc=0x10d3a5d4;continue;
}
0x10d3a5d4=>{
// 0x10d3a5d4 ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a5d8 bl #0x10fec370
cpu.x[30]=0x10d3a5dc;super::dispatch(cpu,rt,0x10fec370)?;
// 0x10d3a5dc cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a5e0 b.eq #0x10d3a64c
if cpu.condition(0){pc=0x10d3a64c;continue;}pc=0x10d3a5e4;continue;
}
0x10d3a5e4=>{
// 0x10d3a5e4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5e8 and w23, w0, w28
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a & b;cpu.x[23]=(result) as u32 as u64;}
// 0x10d3a5ec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a5f0 and w1, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10d3a5f4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a5f8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a5fc;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a5fc;continue;
}
0x10d3a5fc=>{
// 0x10d3a5fc tbz w23, #0, #0x10d3a660
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a660;continue;}pc=0x10d3a600;continue;
}
0x10d3a600=>{
// 0x10d3a600 ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a604 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a608 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a60c ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a610 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a614;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a614;continue;
}
0x10d3a614=>{
// 0x10d3a614 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a618 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a61c ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a620 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a624;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a624;continue;
}
0x10d3a624=>{
// 0x10d3a624 tbz w0, #0, #0x10d3a660
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a660;continue;}pc=0x10d3a628;continue;
}
0x10d3a628=>{
// 0x10d3a628 ldr x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a62c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a630 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a634 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a638 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a63c;super::dispatch(cpu,rt,target)?;}
// 0x10d3a63c b #0x10d3a660
pc=0x10d3a660;continue;
}
0x10d3a640=>{
// 0x10d3a640 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10d3a644 cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a648 b.ne #0x10d3a5e4
if cpu.condition(1){pc=0x10d3a5e4;continue;}pc=0x10d3a64c;continue;
}
0x10d3a64c=>{
// 0x10d3a64c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a650 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a654 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d3a658 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a65c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a660;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a660;continue;
}
0x10d3a660=>{
// 0x10d3a660 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a664 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a668 b.lt #0x10d3a698
if cpu.condition(11){pc=0x10d3a698;continue;}pc=0x10d3a66c;continue;
}
0x10d3a66c=>{
// 0x10d3a66c mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d3a670 add x24, x19, #0xe0
{let a=((cpu.x[19]));let b=((0xe0u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
pc=0x10d3a674;continue;
}
0x10d3a674=>{
// 0x10d3a674 ldr x2, [x24, x23, lsl #3]
{let address=((cpu.x[24]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d3a678 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a67c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a680 mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10d3a684 bl #0x10f8bfc0
cpu.x[30]=0x10d3a688;super::chunk11::f_10f8bfc0(cpu,rt,0x10f8bfc0)?;
pc=0x10d3a688;continue;
}
0x10d3a688=>{
// 0x10d3a688 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3a68c add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d3a690 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a694 b.lt #0x10d3a674
if cpu.condition(11){pc=0x10d3a674;continue;}pc=0x10d3a698;continue;
}
0x10d3a698=>{
// 0x10d3a698 ldr x24, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d3a69c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a6a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a6a4 ldr w23, [x24, #0xc]
{let address=(cpu.x[24]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10d3a6a8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a6ac mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d3a6b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a6b4;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a6b4;continue;
}
0x10d3a6b4=>{
// 0x10d3a6b4 cmp w23, #0
{let a=((cpu.x[23] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a6b8 b.le #0x10d3a6e8
if cpu.condition(13){pc=0x10d3a6e8;continue;}pc=0x10d3a6bc;continue;
}
0x10d3a6bc=>{
// 0x10d3a6bc lsl x23, x23, #3
cpu.x[23]=(((((cpu.x[23]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d3a6c0 mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
pc=0x10d3a6c4;continue;
}
0x10d3a6c4=>{
// 0x10d3a6c4 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a6c8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a6cc mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a6d0 mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10d3a6d4 ldr x2, [x8, x27]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[27])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d3a6d8 bl #0x10f8bfc0
cpu.x[30]=0x10d3a6dc;super::chunk11::f_10f8bfc0(cpu,rt,0x10f8bfc0)?;
pc=0x10d3a6dc;continue;
}
0x10d3a6dc=>{
// 0x10d3a6dc add x27, x27, #8
{let a=((cpu.x[27]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d3a6e0 cmp x23, x27
{let a=((cpu.x[23]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a6e4 b.ne #0x10d3a6c4
if cpu.condition(1){pc=0x10d3a6c4;continue;}pc=0x10d3a6e8;continue;
}
0x10d3a6e8=>{
// 0x10d3a6e8 ldr x24, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d3a6ec ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a6f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a6f4 ldr w23, [x24, #0xc]
{let address=(cpu.x[24]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10d3a6f8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a6fc mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d3a700 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a704;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a704;continue;
}
0x10d3a704=>{
// 0x10d3a704 cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a708 b.lt #0x10d3a738
if cpu.condition(11){pc=0x10d3a738;continue;}pc=0x10d3a70c;continue;
}
0x10d3a70c=>{
// 0x10d3a70c lsl x23, x23, #3
cpu.x[23]=(((((cpu.x[23]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d3a710 mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
pc=0x10d3a714;continue;
}
0x10d3a714=>{
// 0x10d3a714 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a718 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a71c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a720 mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10d3a724 ldr x2, [x8, x27]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[27])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d3a728 bl #0x10f8bfc0
cpu.x[30]=0x10d3a72c;super::chunk11::f_10f8bfc0(cpu,rt,0x10f8bfc0)?;
pc=0x10d3a72c;continue;
}
0x10d3a72c=>{
// 0x10d3a72c add x27, x27, #8
{let a=((cpu.x[27]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d3a730 cmp x23, x27
{let a=((cpu.x[23]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a734 b.ne #0x10d3a714
if cpu.condition(1){pc=0x10d3a714;continue;}pc=0x10d3a738;continue;
}
0x10d3a738=>{
// 0x10d3a738 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a73c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a740 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a744;continue;
}
0x10d3a744=>{
// 0x10d3a744 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10d3a748 add x23, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d3a74c add x24, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d3a750 b #0x10d3a760
pc=0x10d3a760;continue;
}
0x10d3a754=>{
// 0x10d3a754 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3a758 cmp x22, w8, sxtw
{let a=((cpu.x[22]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a75c b.ge #0x10d3a778
if cpu.condition(10){pc=0x10d3a778;continue;}pc=0x10d3a760;continue;
}
0x10d3a760=>{
// 0x10d3a760 ldr x0, [x23, x22, lsl #3]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a764 cbz x0, #0x10d3a754
if (cpu.x[0]) == 0 {pc=0x10d3a754;continue;}pc=0x10d3a768;continue;
}
0x10d3a768=>{
// 0x10d3a768 ldrb w1, [x24, x22]
{let address=((cpu.x[24]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a76c bl #0x10f9c5c0
cpu.x[30]=0x10d3a770;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a770 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a774 b #0x10d3a754
pc=0x10d3a754;continue;
}
0x10d3a778=>{
// 0x10d3a778 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a77c cset w8, lt
cpu.x[8]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10d3a780 orn w8, w8, w25
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a | !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10d3a784 tbnz w8, #0, #0x10d3a908
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3a908;continue;}pc=0x10d3a788;continue;
}
0x10d3a788=>{
// 0x10d3a788 ldr x8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a78c ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a790 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a794 bl #0x110cc5b4
cpu.x[30]=0x10d3a798;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a798 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a79c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a7a0 bl #0x10d395b4
cpu.x[30]=0x10d3a7a4;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a7a4 cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a7a8 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3a7ac bl #0x10f9c5c0
cpu.x[30]=0x10d3a7b0;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a7b0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a7b4 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a7b8 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a7bc;continue;
}
0x10d3a7bc=>{
// 0x10d3a7bc ldr x8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a7c0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a7c4 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a7c8 bl #0x110cc5b4
cpu.x[30]=0x10d3a7cc;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a7cc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a7d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a7d4 bl #0x10d395b4
cpu.x[30]=0x10d3a7d8;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a7d8 ldrb w1, [sp, #1]
{let address=(cpu.sp).wrapping_add(0x1u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a7dc bl #0x10f9c5c0
cpu.x[30]=0x10d3a7e0;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a7e0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a7e4 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a7e8 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a7ec;continue;
}
0x10d3a7ec=>{
// 0x10d3a7ec ldr x8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a7f0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a7f4 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a7f8 bl #0x110cc5b4
cpu.x[30]=0x10d3a7fc;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a7fc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a800 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a804 bl #0x10d395b4
cpu.x[30]=0x10d3a808;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a808 ldrb w1, [sp, #2]
{let address=(cpu.sp).wrapping_add(0x2u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a80c bl #0x10f9c5c0
cpu.x[30]=0x10d3a810;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a810 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a814 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a818 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a81c;continue;
}
0x10d3a81c=>{
// 0x10d3a81c ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a820 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a824 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a828 bl #0x110cc5b4
cpu.x[30]=0x10d3a82c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a82c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a830 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a834 bl #0x10d395b4
cpu.x[30]=0x10d3a838;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a838 ldrb w1, [sp, #3]
{let address=(cpu.sp).wrapping_add(0x3u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a83c bl #0x10f9c5c0
cpu.x[30]=0x10d3a840;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a840 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a844 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a848 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a84c;continue;
}
0x10d3a84c=>{
// 0x10d3a84c ldr x8, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a850 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a854 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a858 bl #0x110cc5b4
cpu.x[30]=0x10d3a85c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a85c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a860 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a864 bl #0x10d395b4
cpu.x[30]=0x10d3a868;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a868 ldrb w1, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a86c bl #0x10f9c5c0
cpu.x[30]=0x10d3a870;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a870 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a874 cmp w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a878 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a87c;continue;
}
0x10d3a87c=>{
// 0x10d3a87c ldr x8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a880 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a884 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a888 bl #0x110cc5b4
cpu.x[30]=0x10d3a88c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a88c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a890 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a894 bl #0x10d395b4
cpu.x[30]=0x10d3a898;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a898 ldrb w1, [sp, #5]
{let address=(cpu.sp).wrapping_add(0x5u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a89c bl #0x10f9c5c0
cpu.x[30]=0x10d3a8a0;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a8a0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a8a4 cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a8a8 b.lt #0x10d3a908
if cpu.condition(11){pc=0x10d3a908;continue;}pc=0x10d3a8ac;continue;
}
0x10d3a8ac=>{
// 0x10d3a8ac mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x10d3a8b0 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10d3a8b4 adrp x22, #0x102d1000
cpu.x[22]=(0x102d1000u64) as u64;
// 0x10d3a8b8 add x22, x22, #0xfd6
{let a=((cpu.x[22]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3a8bc add x23, x8, #6
{let a=((cpu.x[8]));let b=((0x6u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
pc=0x10d3a8c0;continue;
}
0x10d3a8c0=>{
// 0x10d3a8c0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3a8c4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3a8c8 bl #0x1128df18
cpu.x[30]=0x10d3a8cc;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d3a8cc add x8, x19, x24, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[24])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3a8d0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a8d4 ldr x8, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a8d8 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d3a8dc bl #0x110cc5b4
cpu.x[30]=0x10d3a8e0;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
// 0x10d3a8e0 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3a8e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3a8e8 bl #0x10d395b4
cpu.x[30]=0x10d3a8ec;super::dispatch(cpu,rt,0x10d395b4)?;
// 0x10d3a8ec ldrb w1, [x23, x24]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[24])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a8f0 bl #0x10f9c5c0
cpu.x[30]=0x10d3a8f4;super::dispatch(cpu,rt,0x10f9c5c0)?;
// 0x10d3a8f4 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3a8f8 add x9, x24, #7
{let a=((cpu.x[24]));let b=((0x7u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d3a8fc add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d3a900 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a904 b.lt #0x10d3a8c0
if cpu.condition(11){pc=0x10d3a8c0;continue;}pc=0x10d3a908;continue;
}
0x10d3a908=>{
// 0x10d3a908 ldrb w8, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a90c cbz w8, #0x10d3a938
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d3a938;continue;}pc=0x10d3a910;continue;
}
0x10d3a910=>{
// 0x10d3a910 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a914 ldr w1, [x19, #0x1e4]
{let address=(cpu.x[19]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a918 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a91c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a920 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a924;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a924;continue;
}
0x10d3a924=>{
// 0x10d3a924 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a928 ldr w1, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a92c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a930 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a934 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a938;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a938;continue;
}
0x10d3a938=>{
// 0x10d3a938 add x1, x19, #0x1bc
{let a=((cpu.x[19]));let b=((0x1bcu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d3a93c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a940 mov w2, #6
cpu.x[2]=(((0x6u64))) as u32 as u64;
// 0x10d3a944 bl #0x111dd1ec
cpu.x[30]=0x10d3a948;super::chunk3::f_111dd1ec(cpu,rt,0x111dd1ec)?;
pc=0x10d3a948;continue;
}
0x10d3a948=>{
// 0x10d3a948 ldr x8, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a94c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3a950 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a954 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a958 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a95c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3a960 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a964;super::dispatch(cpu,rt,target)?;}
pc=0x10d3a964;continue;
}
0x10d3a964=>{
// 0x10d3a964 tbz w0, #0, #0x10d3a9f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a9f4;continue;}pc=0x10d3a968;continue;
}
0x10d3a968=>{
// 0x10d3a968 ldr x1, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d3a96c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a970 bl #0x111dd3d0
cpu.x[30]=0x10d3a974;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
// 0x10d3a974 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a978 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a97c bl #0x10e2d82c
cpu.x[30]=0x10d3a980;super::chunk0::f_10e2d82c(cpu,rt,0x10e2d82c)?;
// 0x10d3a980 tbnz w0, #0, #0x10d3aa04
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3aa04;continue;}pc=0x10d3a984;continue;
}
0x10d3a984=>{
// 0x10d3a984 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a988 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a98c ldr w8, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a990 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3a994 b.ne #0x10d3aa64
if cpu.condition(1){pc=0x10d3aa64;continue;}pc=0x10d3a998;continue;
}
0x10d3a998=>{
// 0x10d3a998 ldr x8, [x19, #0x238]
{let address=(cpu.x[19]).wrapping_add(0x238u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a99c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3a9a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a9a4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3a9a8 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a9ac cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3a9b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a9b4;super::dispatch(cpu,rt,target)?;}
// 0x10d3a9b4 ldr x0, [x19, #0x238]
{let address=(cpu.x[19]).wrapping_add(0x238u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a9b8 cbz x0, #0x10d3a9cc
if (cpu.x[0]) == 0 {pc=0x10d3a9cc;continue;}pc=0x10d3a9bc;continue;
}
0x10d3a9bc=>{
// 0x10d3a9bc and w3, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d3a9c0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3a9c4 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10d3a9c8 bl #0x10d52c48
cpu.x[30]=0x10d3a9cc;super::dispatch(cpu,rt,0x10d52c48)?;
pc=0x10d3a9cc;continue;
}
0x10d3a9cc=>{
// 0x10d3a9cc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a9d0 ldr w1, [x19, #0x19c]
{let address=(cpu.x[19]).wrapping_add(0x19cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3a9d4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3a9d8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a9dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3a9e0;super::dispatch(cpu,rt,target)?;}
// 0x10d3a9e0 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a9e4 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a9e8 ldrb w8, [x0, #0x181]
{let address=(cpu.x[0]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3a9ec cbnz w8, #0x10d3aa78
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d3aa78;continue;}pc=0x10d3a9f0;continue;
}
0x10d3a9f0=>{
// 0x10d3a9f0 b #0x10d3aa6c
pc=0x10d3aa6c;continue;
}
0x10d3a9f4=>{
// 0x10d3a9f4 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3a9f8 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3a9fc bl #0x10e2d82c
cpu.x[30]=0x10d3aa00;super::chunk0::f_10e2d82c(cpu,rt,0x10e2d82c)?;
pc=0x10d3aa00;continue;
}
0x10d3aa00=>{
// 0x10d3aa00 tbz w0, #0, #0x10d3a984
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a984;continue;}pc=0x10d3aa04;continue;
}
0x10d3aa04=>{
// 0x10d3aa04 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aa08 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aa0c b.lt #0x10d3aa3c
if cpu.condition(11){pc=0x10d3aa3c;continue;}pc=0x10d3aa10;continue;
}
0x10d3aa10=>{
// 0x10d3aa10 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10d3aa14 add x23, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
pc=0x10d3aa18;continue;
}
0x10d3aa18=>{
// 0x10d3aa18 ldr x0, [x23, x22, lsl #3]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3aa1c and w3, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d3aa20 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3aa24 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10d3aa28 bl #0x10d49894
cpu.x[30]=0x10d3aa2c;super::dispatch(cpu,rt,0x10d49894)?;
// 0x10d3aa2c ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3aa30 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3aa34 cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3aa38 b.lt #0x10d3aa18
if cpu.condition(11){pc=0x10d3aa18;continue;}pc=0x10d3aa3c;continue;
}
0x10d3aa3c=>{
// 0x10d3aa3c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aa40 ldr w1, [x19, #0x19c]
{let address=(cpu.x[19]).wrapping_add(0x19cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3aa44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3aa48 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aa4c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3aa50;super::dispatch(cpu,rt,target)?;}
// 0x10d3aa50 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aa54 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3aa58 ldr w8, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aa5c cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aa60 b.eq #0x10d3a998
if cpu.condition(0){pc=0x10d3a998;continue;}pc=0x10d3aa64;continue;
}
0x10d3aa64=>{
// 0x10d3aa64 ldrb w8, [x0, #0x181]
{let address=(cpu.x[0]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aa68 cbnz w8, #0x10d3aa78
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d3aa78;continue;}pc=0x10d3aa6c;continue;
}
0x10d3aa6c=>{
// 0x10d3aa6c ldr w8, [x0, #0x184]
{let address=(cpu.x[0]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aa70 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aa74 b.lt #0x10d3ab30
if cpu.condition(11){pc=0x10d3ab30;continue;}pc=0x10d3aa78;continue;
}
0x10d3aa78=>{
// 0x10d3aa78 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aa7c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aa80 b.lt #0x10d3aab0
if cpu.condition(11){pc=0x10d3aab0;continue;}pc=0x10d3aa84;continue;
}
0x10d3aa84=>{
// 0x10d3aa84 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10d3aa88 add x23, x19, #0x270
{let a=((cpu.x[19]));let b=((0x270u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
pc=0x10d3aa8c;continue;
}
0x10d3aa8c=>{
// 0x10d3aa8c ldr x0, [x23, x22, lsl #3]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3aa90 and w3, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d3aa94 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3aa98 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10d3aa9c bl #0x10d4905c
cpu.x[30]=0x10d3aaa0;super::dispatch(cpu,rt,0x10d4905c)?;
// 0x10d3aaa0 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3aaa4 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3aaa8 cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3aaac b.lt #0x10d3aa8c
if cpu.condition(11){pc=0x10d3aa8c;continue;}pc=0x10d3aab0;continue;
}
0x10d3aab0=>{
// 0x10d3aab0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aab4 ldr w1, [x19, #0x19c]
{let address=(cpu.x[19]).wrapping_add(0x19cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3aab8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3aabc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aac0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3aac4;super::dispatch(cpu,rt,target)?;}
// 0x10d3aac4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aac8 ldr w1, [x19, #0x1a8]
{let address=(cpu.x[19]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3aacc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3aad0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aad4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3aad8;super::dispatch(cpu,rt,target)?;}
// 0x10d3aad8 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aadc ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3aae0 ldr w8, [x0, #0x184]
{let address=(cpu.x[0]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aae4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aae8 b.lt #0x10d3ab30
if cpu.condition(11){pc=0x10d3ab30;continue;}pc=0x10d3aaec;continue;
}
0x10d3aaec=>{
// 0x10d3aaec ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3aaf0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3aaf4 b.lt #0x10d3ab30
if cpu.condition(11){pc=0x10d3ab30;continue;}pc=0x10d3aaf8;continue;
}
0x10d3aaf8=>{
// 0x10d3aaf8 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10d3aafc add x23, x19, #0x240
{let a=((cpu.x[19]));let b=((0x240u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
pc=0x10d3ab00;continue;
}
0x10d3ab00=>{
// 0x10d3ab00 ldr x0, [x23, x22, lsl #3]
{let address=((cpu.x[23]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ab04 and w3, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d3ab08 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ab0c mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10d3ab10 bl #0x10d489a8
cpu.x[30]=0x10d3ab14;super::dispatch(cpu,rt,0x10d489a8)?;
// 0x10d3ab14 ldrsw x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d3ab18 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d3ab1c cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3ab20 b.lt #0x10d3ab00
if cpu.condition(11){pc=0x10d3ab00;continue;}pc=0x10d3ab24;continue;
}
0x10d3ab24=>{
// 0x10d3ab24 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab28 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ab2c cbz x0, #0x10d3ab88
if (cpu.x[0]) == 0 {pc=0x10d3ab88;continue;}pc=0x10d3ab30;continue;
}
0x10d3ab30=>{
// 0x10d3ab30 bl #0x10e2d5a4
cpu.x[30]=0x10d3ab34;super::chunk4::f_10e2d5a4(cpu,rt,0x10e2d5a4)?;
pc=0x10d3ab34;continue;
}
0x10d3ab34=>{
// 0x10d3ab34 tbz w0, #0, #0x10d3ab88
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ab88;continue;}pc=0x10d3ab38;continue;
}
0x10d3ab38=>{
// 0x10d3ab38 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab3c ldr w1, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ab40 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ab44 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab48 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ab4c;super::dispatch(cpu,rt,target)?;}
// 0x10d3ab4c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab50 ldr w1, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ab54 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ab58 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ab60;super::dispatch(cpu,rt,target)?;}
// 0x10d3ab60 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab64 ldr w1, [x19, #0x1a0]
{let address=(cpu.x[19]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ab68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ab6c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab70 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ab74;super::dispatch(cpu,rt,target)?;}
// 0x10d3ab74 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab78 ldrb w1, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ab7c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ab80 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ab88;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ab88;continue;
}
0x10d3ab88=>{
// 0x10d3ab88 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab8c ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ab90 cbz x8, #0x10d3ac6c
if (cpu.x[8]) == 0 {pc=0x10d3ac6c;continue;}pc=0x10d3ab94;continue;
}
0x10d3ab94=>{
// 0x10d3ab94 ldrb w8, [x8, #0x116]
{let address=(cpu.x[8]).wrapping_add(0x116u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3ab98 cbz w8, #0x10d3ac6c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d3ac6c;continue;}pc=0x10d3ab9c;continue;
}
0x10d3ab9c=>{
// 0x10d3ab9c ldr x8, [x19, #0x158]
{let address=(cpu.x[19]).wrapping_add(0x158u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aba0 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3aba4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3aba8 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3abac ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3abb0 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3abb4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3abb8;super::dispatch(cpu,rt,target)?;}
// 0x10d3abb8 tbz w0, #0, #0x10d3ac04
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ac04;continue;}pc=0x10d3abbc;continue;
}
0x10d3abbc=>{
// 0x10d3abbc ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3abc0 bl #0x10e16208
cpu.x[30]=0x10d3abc4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d3abc4 bl #0x10e3e590
cpu.x[30]=0x10d3abc8;super::dispatch(cpu,rt,0x10e3e590)?;
// 0x10d3abc8 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3abcc cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3abd0 b.lt #0x10d3ac04
if cpu.condition(11){pc=0x10d3ac04;continue;}pc=0x10d3abd4;continue;
}
0x10d3abd4=>{
// 0x10d3abd4 lsl x8, x8, #1
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x10d3abd8 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d3abdc and x22, x8, #0x1fffffffc
{let a=((cpu.x[8]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[22]=(result) as u64;}
pc=0x10d3abe0;continue;
}
0x10d3abe0=>{
// 0x10d3abe0 ldr x8, [x19, #0x158]
{let address=(cpu.x[19]).wrapping_add(0x158u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3abe4 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3abe8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3abec ldr w1, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3abf0 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3abf4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3abf8;super::dispatch(cpu,rt,target)?;}
// 0x10d3abf8 add x21, x21, #4
{let a=((cpu.x[21]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d3abfc cmp x22, x21
{let a=((cpu.x[22]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3ac00 b.ne #0x10d3abe0
if cpu.condition(1){pc=0x10d3abe0;continue;}pc=0x10d3ac04;continue;
}
0x10d3ac04=>{
// 0x10d3ac04 ldr x8, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac08 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3ac0c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ac10 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3ac14 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac18 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3ac1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ac20;super::dispatch(cpu,rt,target)?;}
// 0x10d3ac20 tbz w0, #0, #0x10d3ac6c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ac6c;continue;}pc=0x10d3ac24;continue;
}
0x10d3ac24=>{
// 0x10d3ac24 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ac28 bl #0x10e16208
cpu.x[30]=0x10d3ac2c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d3ac2c bl #0x10e3e590
cpu.x[30]=0x10d3ac30;super::dispatch(cpu,rt,0x10e3e590)?;
// 0x10d3ac30 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3ac34 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3ac38 b.le #0x10d3ac6c
if cpu.condition(13){pc=0x10d3ac6c;continue;}pc=0x10d3ac3c;continue;
}
0x10d3ac3c=>{
// 0x10d3ac3c lsl x8, x8, #1
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x10d3ac40 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d3ac44 and x22, x8, #0x1fffffffc
{let a=((cpu.x[8]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[22]=(result) as u64;}
pc=0x10d3ac48;continue;
}
0x10d3ac48=>{
// 0x10d3ac48 ldr x8, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac4c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3ac50 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ac54 ldr w1, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ac58 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ac60;super::dispatch(cpu,rt,target)?;}
// 0x10d3ac60 add x21, x21, #4
{let a=((cpu.x[21]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d3ac64 cmp x22, x21
{let a=((cpu.x[22]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3ac68 b.ne #0x10d3ac48
if cpu.condition(1){pc=0x10d3ac48;continue;}pc=0x10d3ac6c;continue;
}
0x10d3ac6c=>{
// 0x10d3ac6c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac70 ldr w1, [x19, #0x2c0]
{let address=(cpu.x[19]).wrapping_add(0x2c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ac74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ac78 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ac80;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ac80;continue;
}
0x10d3ac80=>{
// 0x10d3ac80 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac84 ldrb w1, [x19, #0x2c4]
{let address=(cpu.x[19]).wrapping_add(0x2c4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ac88 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ac8c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac90 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ac94;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ac94;continue;
}
0x10d3ac94=>{
// 0x10d3ac94 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ac98 ldrb w1, [x19, #0x2f0]
{let address=(cpu.x[19]).wrapping_add(0x2f0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ac9c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3aca0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3aca4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3aca8;super::dispatch(cpu,rt,target)?;}
pc=0x10d3aca8;continue;
}
0x10d3aca8=>{
// 0x10d3aca8 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3acac ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3acb0 ldr w8, [x8, #0x174]
{let address=(cpu.x[8]).wrapping_add(0x174u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3acb4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3acb8 b.lt #0x10d3ace4
if cpu.condition(11){pc=0x10d3ace4;continue;}pc=0x10d3acbc;continue;
}
0x10d3acbc=>{
// 0x10d3acbc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3acc0 ldr w1, [x19, #0x300]
{let address=(cpu.x[19]).wrapping_add(0x300u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3acc4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3acc8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3accc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3acd0;super::dispatch(cpu,rt,target)?;}
// 0x10d3acd0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3acd4 ldr w1, [x19, #0x304]
{let address=(cpu.x[19]).wrapping_add(0x304u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3acd8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3acdc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ace0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ace4;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ace4;continue;
}
0x10d3ace4=>{
// 0x10d3ace4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ace8 ldr w1, [x19, #0x308]
{let address=(cpu.x[19]).wrapping_add(0x308u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3acec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3acf0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3acf4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3acf8;super::dispatch(cpu,rt,target)?;}
pc=0x10d3acf8;continue;
}
0x10d3acf8=>{
// 0x10d3acf8 ldr w8, [x19, #0x30c]
{let address=(cpu.x[19]).wrapping_add(0x30cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3acfc ldr w9, [x19, #0x310]
{let address=(cpu.x[19]).wrapping_add(0x310u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d3ad00 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ad04 ldr x10, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d3ad08 orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10d3ad0c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3ad10 ldr x8, [x10, #0x40]
{let address=(cpu.x[10]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad14 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3ad18 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ad1c;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ad1c;continue;
}
0x10d3ad1c=>{
// 0x10d3ad1c tbz w0, #0, #0x10d3ad48
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ad48;continue;}pc=0x10d3ad20;continue;
}
0x10d3ad20=>{
// 0x10d3ad20 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad24 ldr w1, [x19, #0x30c]
{let address=(cpu.x[19]).wrapping_add(0x30cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ad28 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ad2c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ad34;super::dispatch(cpu,rt,target)?;}
// 0x10d3ad34 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad38 ldr w1, [x19, #0x310]
{let address=(cpu.x[19]).wrapping_add(0x310u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3ad3c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ad40 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad44 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ad48;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ad48;continue;
}
0x10d3ad48=>{
// 0x10d3ad48 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad4c ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad50 ldrb w9, [x8, #0x181]
{let address=(cpu.x[8]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d3ad54 cbz w9, #0x10d3ae08
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10d3ae08;continue;}pc=0x10d3ad58;continue;
}
0x10d3ad58=>{
// 0x10d3ad58 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3ad5c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad60 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ad64 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad68 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ad6c;super::dispatch(cpu,rt,target)?;}
// 0x10d3ad6c tbz w0, #0, #0x10d3ad7c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3ad7c;continue;}pc=0x10d3ad70;continue;
}
0x10d3ad70=>{
// 0x10d3ad70 ldr x0, [x19, #0x2f8]
{let address=(cpu.x[19]).wrapping_add(0x2f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ad74 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ad78 bl #0x10d44dd4
cpu.x[30]=0x10d3ad7c;super::dispatch(cpu,rt,0x10d44dd4)?;
pc=0x10d3ad7c;continue;
}
0x10d3ad7c=>{
// 0x10d3ad7c ldr x8, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad80 ldr w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d3ad84 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3ad88 b.lt #0x10d3adb4
if cpu.condition(11){pc=0x10d3adb4;continue;}pc=0x10d3ad8c;continue;
}
0x10d3ad8c=>{
// 0x10d3ad8c mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10d3ad90;continue;
}
0x10d3ad90=>{
// 0x10d3ad90 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ad94 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ad98 ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ad9c bl #0x110286bc
cpu.x[30]=0x10d3ada0;super::chunk14::f_110286bc(cpu,rt,0x110286bc)?;
pc=0x10d3ada0;continue;
}
0x10d3ada0=>{
// 0x10d3ada0 ldr x8, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ada4 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d3ada8 ldrsw x9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[9]=(v0) as u64;}
// 0x10d3adac cmp x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3adb0 b.lt #0x10d3ad90
if cpu.condition(11){pc=0x10d3ad90;continue;}pc=0x10d3adb4;continue;
}
0x10d3adb4=>{
// 0x10d3adb4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3adb8 ldr w1, [x19, #0x314]
{let address=(cpu.x[19]).wrapping_add(0x314u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3adbc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3adc0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3adc4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3adc8;super::dispatch(cpu,rt,target)?;}
pc=0x10d3adc8;continue;
}
0x10d3adc8=>{
// 0x10d3adc8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3adcc ldr w1, [x19, #0x318]
{let address=(cpu.x[19]).wrapping_add(0x318u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d3add0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3add4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3add8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3addc;super::dispatch(cpu,rt,target)?;}
pc=0x10d3addc;continue;
}
0x10d3addc=>{
// 0x10d3addc ldr x0, [x19, #0x338]
{let address=(cpu.x[19]).wrapping_add(0x338u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ade0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ade4 bl #0x10d56a1c
cpu.x[30]=0x10d3ade8;super::chunk13::f_10d56a1c(cpu,rt,0x10d56a1c)?;
pc=0x10d3ade8;continue;
}
0x10d3ade8=>{
// 0x10d3ade8 ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3adec ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d3adf0 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d3adf4 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d3adf8 ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d3adfc ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d3ae00 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d3ae04 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d3ae08=>{
// 0x10d3ae08 ldr w8, [x8, #0x184]
{let address=(cpu.x[8]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3ae0c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3ae10 cset w1, gt
cpu.x[1]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10d3ae14 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ae1c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ae24;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ae24;continue;
}
0x10d3ae24=>{
// 0x10d3ae24 tbnz w0, #0, #0x10d3ad70
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3ad70;continue;}pc=0x10d3ae28;continue;
}
0x10d3ae28=>{
// 0x10d3ae28 b #0x10d3ad7c
pc=0x10d3ad7c;continue;
}
0x10d3ae2c=>{
// 0x10d3ae2c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae30 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ae34 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3ae38 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae3c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ae40;super::dispatch(cpu,rt,target)?;}
pc=0x10d3ae40;continue;
}
0x10d3ae40=>{
// 0x10d3ae40 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3ae48 ldr x8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae4c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ae50;super::dispatch(cpu,rt,target)?;}
// 0x10d3ae50 eor w24, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[24]=(result) as u32 as u64;}
// 0x10d3ae54 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3ae58 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ae5c and w2, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d3ae60 bl #0x10feabb8
cpu.x[30]=0x10d3ae64;super::chunk11::f_10feabb8(cpu,rt,0x10feabb8)?;
// 0x10d3ae64 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ae68 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae6c ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3ae70 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3ae74;super::dispatch(cpu,rt,target)?;}
// 0x10d3ae74 tbz w0, #0, #0x10d3a660
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3a660;continue;}pc=0x10d3ae78;continue;
}
0x10d3ae78=>{
// 0x10d3ae78 ldr x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ae7c and w2, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d3ae80 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3ae84 bl #0x10feabb8
cpu.x[30]=0x10d3ae88;super::chunk11::f_10feabb8(cpu,rt,0x10feabb8)?;
// 0x10d3ae88 b #0x10d3a660
pc=0x10d3a660;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d39c14")),}}}
#[inline]
pub(super) unsafe fn f_10d43334(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d43334=>{
// 0x10d43334 ldr x8, [x0, #0x2d8]
{let address=(cpu.x[0]).wrapping_add(0x2d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d43338 ldr x9, [x0, #0xd0]
{let address=(cpu.x[0]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d4333c cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d43340 csel x0, x9, x8, eq
cpu.x[0]=(if cpu.condition(0){((cpu.x[9]))}else{((cpu.x[8]))}) as u64;
// 0x10d43344 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d43334")),}}}
#[inline]
pub(super) unsafe fn f_10d441b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d441b4=>{
// 0x10d441b4 ldr w9, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d441b8 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d441bc b.lt #0x10d44208
if cpu.condition(11){pc=0x10d44208;continue;}pc=0x10d441c0;continue;
}
0x10d441c0=>{
// 0x10d441c0 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d441c4 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d441c8 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d441cc stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d441d0 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d441d4 ldr x8, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d441d8 cbz x8, #0x10d441e4
if (cpu.x[8]) == 0 {pc=0x10d441e4;continue;}pc=0x10d441dc;continue;
}
0x10d441dc=>{
// 0x10d441dc ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d441e0 cbz x8, #0x10d441f0
if (cpu.x[8]) == 0 {pc=0x10d441f0;continue;}pc=0x10d441e4;continue;
}
0x10d441e4=>{
// 0x10d441e4 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d441e8 b.ne #0x10d44210
if cpu.condition(1){pc=0x10d44210;continue;}pc=0x10d441ec;continue;
}
0x10d441ec=>{
// 0x10d441ec mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10d441f0;continue;
}
0x10d441f0=>{
// 0x10d441f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d441f4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d441f8 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d441fc add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d44200 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d44204 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d44208=>{
// 0x10d44208 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d4420c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d44210=>{
// 0x10d44210 ldr x8, [x0, #0xe8]
{let address=(cpu.x[0]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d44214 cbz x8, #0x10d44220
if (cpu.x[8]) == 0 {pc=0x10d44220;continue;}pc=0x10d44218;continue;
}
0x10d44218=>{
// 0x10d44218 ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4421c cbz x8, #0x10d44314
if (cpu.x[8]) == 0 {pc=0x10d44314;continue;}pc=0x10d44220;continue;
}
0x10d44220=>{
// 0x10d44220 cmp w9, #3
{let a=((cpu.x[9] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d44224 b.lo #0x10d442a0
if cpu.condition(3){pc=0x10d442a0;continue;}pc=0x10d44228;continue;
}
0x10d44228=>{
// 0x10d44228 ldr x8, [x0, #0xf0]
{let address=(cpu.x[0]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4422c cbz x8, #0x10d44238
if (cpu.x[8]) == 0 {pc=0x10d44238;continue;}pc=0x10d44230;continue;
}
0x10d44230=>{
// 0x10d44230 ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d44234 cbz x8, #0x10d44330
if (cpu.x[8]) == 0 {pc=0x10d44330;continue;}pc=0x10d44238;continue;
}
0x10d44238=>{
// 0x10d44238 cmp w9, #3
{let a=((cpu.x[9] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4423c b.eq #0x10d442a0
if cpu.condition(0){pc=0x10d442a0;continue;}pc=0x10d44240;continue;
}
0x10d44240=>{
// 0x10d44240 ldr x8, [x0, #0xf8]
{let address=(cpu.x[0]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d44244 cbz x8, #0x10d44250
if (cpu.x[8]) == 0 {pc=0x10d44250;continue;}pc=0x10d44248;continue;
}
0x10d44248=>{
// 0x10d44248 ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4424c cbz x8, #0x10d4434c
if (cpu.x[8]) == 0 {pc=0x10d4434c;continue;}pc=0x10d44250;continue;
}
0x10d44250=>{
// 0x10d44250 cmp w9, #5
{let a=((cpu.x[9] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d44254 b.lo #0x10d442a0
if cpu.condition(3){pc=0x10d442a0;continue;}pc=0x10d44258;continue;
}
0x10d44258=>{
// 0x10d44258 ldr x8, [x0, #0x100]
{let address=(cpu.x[0]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4425c cbz x8, #0x10d44268
if (cpu.x[8]) == 0 {pc=0x10d44268;continue;}pc=0x10d44260;continue;
}
0x10d44260=>{
// 0x10d44260 ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d44264 cbz x8, #0x10d44368
if (cpu.x[8]) == 0 {pc=0x10d44368;continue;}pc=0x10d44268;continue;
}
0x10d44268=>{
// 0x10d44268 cmp w9, #5
{let a=((cpu.x[9] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d4426c b.eq #0x10d442a0
if cpu.condition(0){pc=0x10d442a0;continue;}pc=0x10d44270;continue;
}
0x10d44270=>{
// 0x10d44270 ldr x8, [x0, #0x108]
{let address=(cpu.x[0]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d44274 cbz x8, #0x10d44280
if (cpu.x[8]) == 0 {pc=0x10d44280;continue;}pc=0x10d44278;continue;
}
0x10d44278=>{
// 0x10d44278 ldr x8, [x8, #0x330]
{let address=(cpu.x[8]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4427c cbz x8, #0x10d44384
if (cpu.x[8]) == 0 {pc=0x10d44384;continue;}pc=0x10d44280;continue;
}
0x10d44280=>{
// 0x10d44280 cmp w9, #7
{let a=((cpu.x[9] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d44284 b.lo #0x10d442a0
if cpu.condition(3){pc=0x10d442a0;continue;}pc=0x10d44288;continue;
}
0x10d44288=>{
// 0x10d44288 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x10d4428c mov w19, #6
cpu.x[19]=(((0x6u64))) as u32 as u64;
// 0x10d44290 adrp x20, #0x102e0000
cpu.x[20]=(0x102e0000u64) as u64;
// 0x10d44294 add x20, x20, #0x6ab
{let a=((cpu.x[20]));let b=((0x6abu64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10d44298 add x21, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d4429c b #0x10d442cc
pc=0x10d442cc;continue;
}
0x10d442a0=>{
// 0x10d442a0 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d442a4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d442a8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d442ac ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d442b0 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d442b4 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d442b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d442bc=>{
// 0x10d442bc add x19, x19, #1
{let a=((cpu.x[19]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u64;}
// 0x10d442c0 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d442c4 cmp x19, w9, sxtw
{let a=((cpu.x[19]));let b=(((cpu.x[9] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d442c8 b.ge #0x10d441f0
if cpu.condition(10){pc=0x10d441f0;continue;}pc=0x10d442cc;continue;
}
0x10d442cc=>{
// 0x10d442cc cmp x19, #0x64
{let a=((cpu.x[19]));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d442d0 b.eq #0x10d442bc
if cpu.condition(0){pc=0x10d442bc;continue;}pc=0x10d442d4;continue;
}
0x10d442d4=>{
// 0x10d442d4 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x10d442d8 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d442dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d442e0 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d442e4 bl #0x114328cc
cpu.x[30]=0x10d442e8;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x10d442e8 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d442ec ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d442f0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d442f4 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d442f8 csel x0, x21, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[21]))}else{((cpu.x[9]))}) as u64;
// 0x10d442fc bl #0x1128df18
cpu.x[30]=0x10d44300;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d44300 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x10d44304 bl #0x1142fcc4
cpu.x[30]=0x10d44308;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d44308 ldr w9, [x22, #0x60]
{let address=(cpu.x[22]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d4430c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d44310 b #0x10d442bc
pc=0x10d442bc;continue;
}
0x10d44314=>{
// 0x10d44314 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d44318 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d4431c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d44320 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d44324 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d44328 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d4432c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d44330=>{
// 0x10d44330 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10d44334 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d44338 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d4433c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d44340 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d44344 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d44348 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d4434c=>{
// 0x10d4434c mov w8, #3
cpu.x[8]=(((0x3u64))) as u32 as u64;
// 0x10d44350 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d44354 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d44358 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d4435c add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d44360 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d44364 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d44368=>{
// 0x10d44368 mov w8, #4
cpu.x[8]=(((0x4u64))) as u32 as u64;
// 0x10d4436c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d44370 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d44374 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d44378 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d4437c mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d44380 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d44384=>{
// 0x10d44384 mov w8, #5
cpu.x[8]=(((0x5u64))) as u32 as u64;
// 0x10d44388 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d4438c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d44390 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d44394 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d44398 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d4439c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d441b4")),}}}
#[inline]
pub(super) unsafe fn f_10d44cbc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d44cbc=>{
// 0x10d44cbc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d44cc0 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44cc4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44cc8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d44ccc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d44cd0 ldr x0, [x0, #0x108]
{let address=(cpu.x[0]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d44cd4 cbz x0, #0x10d44d10
if (cpu.x[0]) == 0 {pc=0x10d44d10;continue;}pc=0x10d44cd8;continue;
}
0x10d44cd8=>{
// 0x10d44cd8 bl #0x10d553d8
cpu.x[30]=0x10d44cdc;super::chunk4::f_10d553d8(cpu,rt,0x10d553d8)?;
pc=0x10d44cdc;continue;
}
0x10d44cdc=>{
// 0x10d44cdc ldr x20, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d44ce0 cbz x20, #0x10d44d0c
if (cpu.x[20]) == 0 {pc=0x10d44d0c;continue;}pc=0x10d44ce4;continue;
}
0x10d44ce4=>{
// 0x10d44ce4 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10d44ce8 ldr x0, [x21, #0x18]!
{let address=(cpu.x[21]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10d44cec cbz x0, #0x10d44cf4
if (cpu.x[0]) == 0 {pc=0x10d44cf4;continue;}pc=0x10d44cf0;continue;
}
0x10d44cf0=>{
// 0x10d44cf0 bl #0x1187d360
cpu.x[30]=0x10d44cf4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d44cf4;continue;
}
0x10d44cf4=>{
// 0x10d44cf4 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d44cf8 stp xzr, xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44cfc cbz x0, #0x10d44d04
if (cpu.x[0]) == 0 {pc=0x10d44d04;continue;}pc=0x10d44d00;continue;
}
0x10d44d00=>{
// 0x10d44d00 bl #0x1187d360
cpu.x[30]=0x10d44d04;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d44d04;continue;
}
0x10d44d04=>{
// 0x10d44d04 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d44d08 bl #0x1187d2c0
cpu.x[30]=0x10d44d0c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d44d0c;continue;
}
0x10d44d0c=>{
// 0x10d44d0c str xzr, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d44d10;continue;
}
0x10d44d10=>{
// 0x10d44d10 ldr x0, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d44d14 cbz x0, #0x10d44d50
if (cpu.x[0]) == 0 {pc=0x10d44d50;continue;}pc=0x10d44d18;continue;
}
0x10d44d18=>{
// 0x10d44d18 bl #0x10d553d8
cpu.x[30]=0x10d44d1c;super::chunk4::f_10d553d8(cpu,rt,0x10d553d8)?;
pc=0x10d44d1c;continue;
}
0x10d44d1c=>{
// 0x10d44d1c ldr x20, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d44d20 cbz x20, #0x10d44d4c
if (cpu.x[20]) == 0 {pc=0x10d44d4c;continue;}pc=0x10d44d24;continue;
}
0x10d44d24=>{
// 0x10d44d24 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10d44d28 ldr x0, [x21, #0x18]!
{let address=(cpu.x[21]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10d44d2c cbz x0, #0x10d44d34
if (cpu.x[0]) == 0 {pc=0x10d44d34;continue;}pc=0x10d44d30;continue;
}
0x10d44d30=>{
// 0x10d44d30 bl #0x1187d360
cpu.x[30]=0x10d44d34;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d44d34;continue;
}
0x10d44d34=>{
// 0x10d44d34 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d44d38 stp xzr, xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d3c cbz x0, #0x10d44d44
if (cpu.x[0]) == 0 {pc=0x10d44d44;continue;}pc=0x10d44d40;continue;
}
0x10d44d40=>{
// 0x10d44d40 bl #0x1187d360
cpu.x[30]=0x10d44d44;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d44d44;continue;
}
0x10d44d44=>{
// 0x10d44d44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d44d48 bl #0x1187d2c0
cpu.x[30]=0x10d44d4c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d44d4c;continue;
}
0x10d44d4c=>{
// 0x10d44d4c str xzr, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d44d50;continue;
}
0x10d44d50=>{
// 0x10d44d50 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d44d54 mov x8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u64;
// 0x10d44d58 str wzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44d5c stur x8, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44d60 str w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44d64 stp xzr, xzr, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d68 stp xzr, xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d6c stur q0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d70 stur q0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d74 stur q0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d78 stur q0, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d7c stp xzr, xzr, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d80 stur q0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d84 stur q0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d88 stp xzr, xzr, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d8c stp x8, x8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=cpu.x[8];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d90 stp x8, x8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44d94 stur q0, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d98 stur q0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44d9c stp xzr, xzr, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44da0 str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44da4 stur q0, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44da8 stur q0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44dac stp xzr, xzr, [x19, #0x150]
{let address=(cpu.x[19]).wrapping_add(0x150u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44db0 str xzr, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44db4 stur q0, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44db8 stur q0, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44dbc str wzr, [x19, #0x180]
{let address=(cpu.x[19]).wrapping_add(0x180u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44dc0 stp xzr, xzr, [x19, #0x170]
{let address=(cpu.x[19]).wrapping_add(0x170u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44dc4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d44dc8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d44dcc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d44dd0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d44cbc")),}}}
#[inline]
pub(super) unsafe fn f_10d48998(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d48998=>{
// 0x10d48998 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d4899c stp wzr, wzr, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=0u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d489a0 str w8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d489a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d48998")),}}}
#[inline]
pub(super) unsafe fn f_10d54230(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d54230=>{
// 0x10d54230 sub sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d54234 stp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54238 stp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d5423c stp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54240 stp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54244 stp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54248 stp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d5424c add x29, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d54250 ldrb w9, [x29, #0x78]
{let address=(cpu.x[29]).wrapping_add(0x78u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d54254 ldrb w8, [x29, #0x70]
{let address=(cpu.x[29]).wrapping_add(0x70u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54258 mov x25, x4
cpu.x[25]=(((cpu.x[4]))) as u64;
// 0x10d5425c mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10d54260 mov w24, w7
cpu.x[24]=(((cpu.x[7] as u32 as u64))) as u32 as u64;
// 0x10d54264 mov w23, w6
cpu.x[23]=(((cpu.x[6] as u32 as u64))) as u32 as u64;
// 0x10d54268 stp w8, w9, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d5426c ldrb w8, [x29, #0x68]
{let address=(cpu.x[29]).wrapping_add(0x68u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54270 mov w26, w5
cpu.x[26]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10d54274 mov w21, w3
cpu.x[21]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d54278 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d5427c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10d54280 stur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d54284 ldrb w8, [x29, #0x60]
{let address=(cpu.x[29]).wrapping_add(0x60u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54288 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d5428c str w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d54290 bl #0x10d54530
cpu.x[30]=0x10d54294;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d54294;continue;
}
0x10d54294=>{
// 0x10d54294 ldr x27, [x25, #8]
{let address=(cpu.x[25]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d54298 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d5429c mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d542a0 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d542a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d542a8;continue;
}
0x10d542a8=>{
// 0x10d542a8 tbz w0, #0, #0x10d54370
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d54370;continue;}pc=0x10d542ac;continue;
}
0x10d542ac=>{
// 0x10d542ac ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542b0 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d542b4 str w23, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d542b8 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d542c0;super::dispatch(cpu,rt,target)?;}
pc=0x10d542c0;continue;
}
0x10d542c0=>{
// 0x10d542c0 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542c4 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d542c8 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d542cc ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d542d4;super::dispatch(cpu,rt,target)?;}
pc=0x10d542d4;continue;
}
0x10d542d4=>{
// 0x10d542d4 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542d8 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d542dc mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d542e0 ldr x8, [x8, #0x188]
{let address=(cpu.x[8]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542e4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d542e8;super::dispatch(cpu,rt,target)?;}
pc=0x10d542e8;continue;
}
0x10d542e8=>{
// 0x10d542e8 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542ec orr w23, w28, w0
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a | b;cpu.x[23]=(result) as u32 as u64;}
// 0x10d542f0 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d542f4 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d542f8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d542fc;super::dispatch(cpu,rt,target)?;}
pc=0x10d542fc;continue;
}
0x10d542fc=>{
// 0x10d542fc tst w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d54300 mov w8, #0x1d
cpu.x[8]=(((0x1du64))) as u32 as u64;
// 0x10d54304 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54308 cinc w27, w8, eq
cpu.x[27]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d5430c tbz w0, #0, #0x10d5439c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d5439c;continue;}pc=0x10d54310;continue;
}
0x10d54310=>{
// 0x10d54310 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54314 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d54318 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d5431c mov w4, #6
cpu.x[4]=(((0x6u64))) as u32 as u64;
// 0x10d54320 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54324 bl #0x10d54530
cpu.x[30]=0x10d54328;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d54328;continue;
}
0x10d54328=>{
// 0x10d54328 mov w25, #3
cpu.x[25]=(((0x3u64))) as u32 as u64;
// 0x10d5432c tbz w23, #0, #0x10d543dc
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d543dc;continue;}pc=0x10d54330;continue;
}
0x10d54330=>{
// 0x10d54330 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54334 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54338 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d5433c mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54340 mov w4, #4
cpu.x[4]=(((0x4u64))) as u32 as u64;
// 0x10d54344 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54348 mov w28, #1
cpu.x[28]=(((0x1u64))) as u32 as u64;
// 0x10d5434c bl #0x10d54530
cpu.x[30]=0x10d54350;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d54350;continue;
}
0x10d54350=>{
// 0x10d54350 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54354 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54358 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d5435c mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54360 mov w4, w27
cpu.x[4]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10d54364 mov w5, w26
cpu.x[5]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10d54368 bl #0x10d54530
cpu.x[30]=0x10d5436c;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d5436c;continue;
}
0x10d5436c=>{
// 0x10d5436c b #0x10d543e0
pc=0x10d543e0;continue;
}
0x10d54370=>{
// 0x10d54370 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54374 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54378 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d5437c mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54380 mov w4, #5
cpu.x[4]=(((0x5u64))) as u32 as u64;
// 0x10d54384 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54388 bl #0x10d54530
cpu.x[30]=0x10d5438c;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
// 0x10d5438c mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10d54390 mov w25, #2
cpu.x[25]=(((0x2u64))) as u32 as u64;
// 0x10d54394 tbnz w24, #0, #0x10d543e8
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d543e8;continue;}pc=0x10d54398;continue;
}
0x10d54398=>{
// 0x10d54398 b #0x10d54404
pc=0x10d54404;continue;
}
0x10d5439c=>{
// 0x10d5439c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d543a0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d543a4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d543a8 mov w4, #5
cpu.x[4]=(((0x5u64))) as u32 as u64;
// 0x10d543ac mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d543b0 bl #0x10d54530
cpu.x[30]=0x10d543b4;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d543b4;continue;
}
0x10d543b4=>{
// 0x10d543b4 tbz w25, #0, #0x10d543d4
if ((cpu.x[25] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d543d4;continue;}pc=0x10d543b8;continue;
}
0x10d543b8=>{
// 0x10d543b8 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d543bc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d543c0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d543c4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d543c8 mov w4, #7
cpu.x[4]=(((0x7u64))) as u32 as u64;
// 0x10d543cc mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d543d0 bl #0x10d54530
cpu.x[30]=0x10d543d4;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d543d4;continue;
}
0x10d543d4=>{
// 0x10d543d4 mov w25, #2
cpu.x[25]=(((0x2u64))) as u32 as u64;
// 0x10d543d8 tbnz w23, #0, #0x10d54330
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d54330;continue;}pc=0x10d543dc;continue;
}
0x10d543dc=>{
// 0x10d543dc mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
pc=0x10d543e0;continue;
}
0x10d543e0=>{
// 0x10d543e0 ldr w23, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10d543e4 tbz w24, #0, #0x10d54404
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d54404;continue;}pc=0x10d543e8;continue;
}
0x10d543e8=>{
// 0x10d543e8 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d543ec mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d543f0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d543f4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d543f8 mov w4, #0xc
cpu.x[4]=(((0xcu64))) as u32 as u64;
// 0x10d543fc mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54400 bl #0x10d54530
cpu.x[30]=0x10d54404;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d54404;continue;
}
0x10d54404=>{
// 0x10d54404 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54408 tbnz w8, #0, #0x10d5442c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d5442c;continue;}pc=0x10d5440c;continue;
}
0x10d5440c=>{
// 0x10d5440c ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54410 tbnz w8, #0, #0x10d54450
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d54450;continue;}pc=0x10d54414;continue;
}
0x10d54414=>{
// 0x10d54414 ldur w8, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54418 tbnz w8, #0, #0x10d54474
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d54474;continue;}pc=0x10d5441c;continue;
}
0x10d5441c=>{
// 0x10d5441c ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54420 tbnz w8, #0, #0x10d54498
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d54498;continue;}pc=0x10d54424;continue;
}
0x10d54424=>{
// 0x10d54424 tbnz w23, #0, #0x10d544b8
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d544b8;continue;}pc=0x10d54428;continue;
}
0x10d54428=>{
// 0x10d54428 b #0x10d54510
pc=0x10d54510;continue;
}
0x10d5442c=>{
// 0x10d5442c and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54430 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54434 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d54438 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d5443c mov w4, #0x13
cpu.x[4]=(((0x13u64))) as u32 as u64;
// 0x10d54440 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54444 bl #0x10d54530
cpu.x[30]=0x10d54448;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d54448;continue;
}
0x10d54448=>{
// 0x10d54448 ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d5444c tbz w8, #0, #0x10d54414
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d54414;continue;}pc=0x10d54450;continue;
}
0x10d54450=>{
// 0x10d54450 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54454 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54458 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d5445c mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54460 mov w4, #0x14
cpu.x[4]=(((0x14u64))) as u32 as u64;
// 0x10d54464 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54468 bl #0x10d54530
cpu.x[30]=0x10d5446c;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
// 0x10d5446c ldur w8, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54470 tbz w8, #0, #0x10d5441c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d5441c;continue;}pc=0x10d54474;continue;
}
0x10d54474=>{
// 0x10d54474 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d54478 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d5447c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d54480 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54484 mov w4, #0x15
cpu.x[4]=(((0x15u64))) as u32 as u64;
// 0x10d54488 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d5448c bl #0x10d54530
cpu.x[30]=0x10d54490;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
// 0x10d54490 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54494 tbz w8, #0, #0x10d54424
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d54424;continue;}pc=0x10d54498;continue;
}
0x10d54498=>{
// 0x10d54498 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d5449c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d544a0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d544a4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d544a8 mov w4, #0x1a
cpu.x[4]=(((0x1au64))) as u32 as u64;
// 0x10d544ac mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d544b0 bl #0x10d54530
cpu.x[30]=0x10d544b4;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d544b4;continue;
}
0x10d544b4=>{
// 0x10d544b4 tbz w23, #0, #0x10d54510
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d54510;continue;}pc=0x10d544b8;continue;
}
0x10d544b8=>{
// 0x10d544b8 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d544bc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d544c0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d544c4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d544c8 mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d544cc mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d544d0 bl #0x10d54530
cpu.x[30]=0x10d544d4;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d544d4;continue;
}
0x10d544d4=>{
// 0x10d544d4 cbz w28, #0x10d54510
if (cpu.x[28] as u32 as u64) == 0 {pc=0x10d54510;continue;}pc=0x10d544d8;continue;
}
0x10d544d8=>{
// 0x10d544d8 and w3, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[3]=(result) as u32 as u64;}
// 0x10d544dc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d544e0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d544e4 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d544e8 mov w4, #0x1f
cpu.x[4]=(((0x1fu64))) as u32 as u64;
// 0x10d544ec mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d544f0 ldp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d544f4 ldp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d544f8 ldp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d544fc ldp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d54500 ldp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d54504 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d54508 add sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d5450c b #0x10d54530
super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;return Ok(());
}
0x10d54510=>{
// 0x10d54510 ldp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d54514 ldp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d54518 ldp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d5451c ldp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d54520 ldp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d54524 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d54528 add sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d5452c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d54230")),}}}
#[inline]
pub(super) unsafe fn f_10d56e38(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d56e38=>{
// 0x10d56e38 str wzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56e3c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d56e38")),}}}
#[inline]
pub(super) unsafe fn f_10d58a1c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d58a1c=>{
// 0x10d58a1c sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d58a20 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58a24 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58a28 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58a2c add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d58a30 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d58a34 cmp w1, #3
{let a=((cpu.x[1] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d58a38 mov x19, x8
cpu.x[19]=(((cpu.x[8]))) as u64;
// 0x10d58a3c b.eq #0x10d58a78
if cpu.condition(0){pc=0x10d58a78;continue;}pc=0x10d58a40;continue;
}
0x10d58a40=>{
// 0x10d58a40 cmp w20, #2
{let a=((cpu.x[20] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d58a44 b.eq #0x10d58a6c
if cpu.condition(0){pc=0x10d58a6c;continue;}pc=0x10d58a48;continue;
}
0x10d58a48=>{
// 0x10d58a48 cmp w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d58a4c b.ne #0x10d58aa4
if cpu.condition(1){pc=0x10d58aa4;continue;}pc=0x10d58a50;continue;
}
0x10d58a50=>{
// 0x10d58a50 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d58a54 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58a58 bl #0x1187d2b0
cpu.x[30]=0x10d58a5c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d58a5c;continue;
}
0x10d58a5c=>{
// 0x10d58a5c mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10d58a60 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58a64 bl #0x10d556c0
cpu.x[30]=0x10d58a68;super::chunk7::f_10d556c0(cpu,rt,0x10d556c0)?;
pc=0x10d58a68;continue;
}
0x10d58a68=>{
// 0x10d58a68 b #0x10d58a84
pc=0x10d58a84;continue;
}
0x10d58a6c=>{
// 0x10d58a6c add x8, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d58a70 bl #0x10d58abc
cpu.x[30]=0x10d58a74;super::chunk1::f_10d58abc(cpu,rt,0x10d58abc)?;
pc=0x10d58a74;continue;
}
0x10d58a74=>{
// 0x10d58a74 b #0x10d58a80
pc=0x10d58a80;continue;
}
0x10d58a78=>{
// 0x10d58a78 add x8, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d58a7c bl #0x10d58bf8
cpu.x[30]=0x10d58a80;super::chunk3::f_10d58bf8(cpu,rt,0x10d58bf8)?;
pc=0x10d58a80;continue;
}
0x10d58a80=>{
// 0x10d58a80 ldr x21, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
pc=0x10d58a84;continue;
}
0x10d58a84=>{
// 0x10d58a84 mov w8, #0x12
cpu.x[8]=(((0x12u64))) as u32 as u64;
// 0x10d58a88 stp w20, w8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=cpu.x[20] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d58a8c str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58a90 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58a94 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58a98 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d58a9c add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d58aa0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58aa4=>{
// 0x10d58aa4 str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58aa8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58aac ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58ab0 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d58ab4 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d58ab8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d58a1c")),}}}
#[inline]
pub(super) unsafe fn f_10d72718(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d72718=>{
// 0x10d72718 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d7271c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d72720 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d72724 adrp x8, #0x11910000
cpu.x[8]=(0x11910000u64) as u64;
// 0x10d72728 add x8, x8, #0x190
{let a=((cpu.x[8]));let b=((0x190u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d7272c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d72730 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d72734 str x8, [x0], #0x14
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[0]=((cpu.x[0]).wrapping_add(((0x14u64)))) as u64;}
// 0x10d72738 bl #0x11294204
cpu.x[30]=0x10d7273c;super::chunk14::f_11294204(cpu,rt,0x11294204)?;
pc=0x10d7273c;continue;
}
0x10d7273c=>{
// 0x10d7273c movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d72740 strb wzr, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d72744 stp wzr, wzr, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=0u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d72748 stur d0, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d7274c str x19, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d72750 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d72754 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d72758 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d72718")),}}}
#[inline]
pub(super) unsafe fn f_10d72c60(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d72c60=>{
// 0x10d72c60 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10d72c64 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d72c60")),}}}
#[inline]
pub(super) unsafe fn f_10d75c14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d75c14=>{
// 0x10d75c14 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d75c18 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75c1c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d75c20 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d75c24 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d75c28 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d75c2c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d75c30 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d75c34 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d75c38 bl #0x1128db1c
cpu.x[30]=0x10d75c3c;super::chunk12::f_1128db1c(cpu,rt,0x1128db1c)?;
pc=0x10d75c3c;continue;
}
0x10d75c3c=>{
// 0x10d75c3c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d75c44 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c48 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d75c4c;super::dispatch(cpu,rt,target)?;}
pc=0x10d75c4c;continue;
}
0x10d75c4c=>{
// 0x10d75c4c ldr w8, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d75c50 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d75c54 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75c58 b.lt #0x10d75ca4
if cpu.condition(11){pc=0x10d75ca4;continue;}pc=0x10d75c5c;continue;
}
0x10d75c5c=>{
// 0x10d75c5c sub x23, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u64;}
// 0x10d75c60 b #0x10d75c74
pc=0x10d75c74;continue;
}
0x10d75c64=>{
// 0x10d75c64 add x8, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d75c68 sub x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u64;}
// 0x10d75c6c cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d75c70 b.ls #0x10d75ca4
if cpu.condition(9){pc=0x10d75ca4;continue;}pc=0x10d75c74;continue;
}
0x10d75c74=>{
// 0x10d75c74 ldur x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c78 ldr x22, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d75c7c cbz x22, #0x10d75c64
if (cpu.x[22]) == 0 {pc=0x10d75c64;continue;}pc=0x10d75c80;continue;
}
0x10d75c80=>{
// 0x10d75c80 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c84 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d75c88 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c8c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d75c90;super::dispatch(cpu,rt,target)?;}
pc=0x10d75c90;continue;
}
0x10d75c90=>{
// 0x10d75c90 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c94 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d75c98 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75c9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d75ca0;super::dispatch(cpu,rt,target)?;}
pc=0x10d75ca0;continue;
}
0x10d75ca0=>{
// 0x10d75ca0 b #0x10d75c64
pc=0x10d75c64;continue;
}
0x10d75ca4=>{
// 0x10d75ca4 ldr w8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d75ca8 str wzr, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d75cac cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75cb0 b.ge #0x10d75cc0
if cpu.condition(10){pc=0x10d75cc0;continue;}pc=0x10d75cb4;continue;
}
0x10d75cb4=>{
// 0x10d75cb4 add x0, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d75cb8 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d75cbc bl #0x1081b608
cpu.x[30]=0x10d75cc0;super::chunk12::f_1081b608(cpu,rt,0x1081b608)?;
pc=0x10d75cc0;continue;
}
0x10d75cc0=>{
// 0x10d75cc0 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75cc4 b.lt #0x10d75d4c
if cpu.condition(11){pc=0x10d75d4c;continue;}pc=0x10d75cc8;continue;
}
0x10d75cc8=>{
// 0x10d75cc8 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10d75ccc b #0x10d75cec
pc=0x10d75cec;continue;
}
0x10d75cd0=>{
// 0x10d75cd0 ldr x9, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d75cd4 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d75cd8 str w10, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d75cdc ldr x10, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d75ce0 str x10, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75ce4 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d75ce8 b.eq #0x10d75d4c
if cpu.condition(0){pc=0x10d75d4c;continue;}pc=0x10d75cec;continue;
}
0x10d75cec=>{
// 0x10d75cec ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d75cf0 add x8, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d75cf4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d75cf8 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10d75cfc mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10d75d00 bl #0x10d75d68
cpu.x[30]=0x10d75d04;super::chunk11::f_10d75d68(cpu,rt,0x10d75d68)?;
pc=0x10d75d04;continue;
}
0x10d75d04=>{
// 0x10d75d04 ldp w9, w8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d75d08 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75d0c b.ne #0x10d75cd0
if cpu.condition(1){pc=0x10d75cd0;continue;}pc=0x10d75d10;continue;
}
0x10d75d10=>{
// 0x10d75d10 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d75d14 ldr x23, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d75d18 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75d1c csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d75d20 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75d24 b.ge #0x10d75d34
if cpu.condition(10){pc=0x10d75d34;continue;}pc=0x10d75d28;continue;
}
0x10d75d28=>{
// 0x10d75d28 add x0, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d75d2c bl #0x1081b608
cpu.x[30]=0x10d75d30;super::chunk12::f_1081b608(cpu,rt,0x1081b608)?;
// 0x10d75d30 ldr w8, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d75d34;continue;
}
0x10d75d34=>{
// 0x10d75d34 ldr x9, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d75d38 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d75d3c str w10, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d75d40 str x23, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75d44 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d75d48 b.ne #0x10d75cec
if cpu.condition(1){pc=0x10d75cec;continue;}pc=0x10d75d4c;continue;
}
0x10d75d4c=>{
// 0x10d75d4c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d75d50 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d75d54 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d75d58 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d75d5c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d75d60 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d75d64 b #0x1128db1c
super::chunk12::f_1128db1c(cpu,rt,0x1128db1c)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d75c14")),}}}
#[inline]
pub(super) unsafe fn f_10d79754(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d79754=>{
// 0x10d79754 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d79758 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d7975c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d79760 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d79764 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d79768 bl #0x10d72784
cpu.x[30]=0x10d7976c;super::chunk12::f_10d72784(cpu,rt,0x10d72784)?;
pc=0x10d7976c;continue;
}
0x10d7976c=>{
// 0x10d7976c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79770 ldr w1, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d79774 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d79778 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d7977c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d79780;super::dispatch(cpu,rt,target)?;}
pc=0x10d79780;continue;
}
0x10d79780=>{
// 0x10d79780 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79784 ldr w1, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d79788 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d7978c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79790 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d79794;super::dispatch(cpu,rt,target)?;}
pc=0x10d79794;continue;
}
0x10d79794=>{
// 0x10d79794 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79798 ldr w1, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d7979c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d797a0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d797a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d797a8;continue;
}
0x10d797a8=>{
// 0x10d797a8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797ac ldr w1, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d797b0 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d797b4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d797b8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d797bc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d797c0 br x2
{let target=cpu.x[2];if target>=0x10d79754 && target<0x10d797c4{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d79754")),}}}
#[inline]
pub(super) unsafe fn f_10daaf10(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10daaf10=>{
// 0x10daaf10 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10daaf10")),}}}
#[inline]
pub(super) unsafe fn f_10ddab44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddab44=>{
// 0x10ddab44 ldr w0, [x0, #0xe4]
{let address=(cpu.x[0]).wrapping_add(0xe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10ddab48 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddab44")),}}}
#[inline]
pub(super) unsafe fn f_10de0334(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0334=>{
// 0x10de0334 ldr w9, [x0, #0x2bc]
{let address=(cpu.x[0]).wrapping_add(0x2bcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10de0338 mov w8, #-0x64
cpu.x[8]=(((0xffffffffffffff9cu64))) as u32 as u64;
// 0x10de033c cmn w9, #0x64
{let a=((cpu.x[9] as u32 as u64));let b=((0x64u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10de0340 csel w8, w9, w8, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10de0344 mov w9, #0x63
cpu.x[9]=(((0x63u64))) as u32 as u64;
// 0x10de0348 add w8, w8, #0x64
{let a=((cpu.x[8] as u32 as u64));let b=((0x64u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10de034c madd w8, w8, w1, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[1] as u32 as u64))))) as u32 as u64;
// 0x10de0350 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10de0354 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10de0358 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10de035c lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10de0360 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10de0364 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10de0368 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0334")),}}}
#[inline]
pub(super) unsafe fn f_10de047c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de047c=>{
// 0x10de047c ldr x0, [x0, #0x1d0]
{let address=(cpu.x[0]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de0480 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de047c")),}}}
#[inline]
pub(super) unsafe fn f_10de0494(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0494=>{
// 0x10de0494 ldr w0, [x0, #0x438]
{let address=(cpu.x[0]).wrapping_add(0x438u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0498 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0494")),}}}
#[inline]
pub(super) unsafe fn f_10de06b0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de06b0=>{
// 0x10de06b0 ldr w0, [x0, #0x580]
{let address=(cpu.x[0]).wrapping_add(0x580u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de06b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de06b0")),}}}
#[inline]
pub(super) unsafe fn f_10de0830(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0830=>{
// 0x10de0830 ldr x0, [x0, #0x310]
{let address=(cpu.x[0]).wrapping_add(0x310u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de0834 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0830")),}}}
#[inline]
pub(super) unsafe fn f_10de08d8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de08d8=>{
// 0x10de08d8 ldrb w0, [x0, #0x58d]
{let address=(cpu.x[0]).wrapping_add(0x58du64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de08dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de08d8")),}}}
#[inline]
pub(super) unsafe fn f_10de1280(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1280=>{
// 0x10de1280 ldr w0, [x0, #0x3e8]
{let address=(cpu.x[0]).wrapping_add(0x3e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de1284 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1280")),}}}
#[inline]
pub(super) unsafe fn f_10e12f08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e12f08=>{
// 0x10e12f08 ldr w8, [x1, #4]
{let address=(cpu.x[1]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e12f0c cbz w8, #0x10e12f5c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e12f5c;continue;}pc=0x10e12f10;continue;
}
0x10e12f10=>{
// 0x10e12f10 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e12f14 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10e12f18 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e12f1c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e12f20 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e12f24 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10e12f28 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e12f2c mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10e12f30 ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e12f34 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e12f38;super::dispatch(cpu,rt,target)?;}
pc=0x10e12f38;continue;
}
0x10e12f38=>{
// 0x10e12f38 cbz x0, #0x10e12f64
if (cpu.x[0]) == 0 {pc=0x10e12f64;continue;}pc=0x10e12f3c;continue;
}
0x10e12f3c=>{
// 0x10e12f3c ldr x20, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10e12f40 ldr x8, [x20, #0xfc0]
{let address=(cpu.x[20]).wrapping_add(0xfc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e12f44 cmp x0, x8
{let a=((cpu.x[0]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e12f48 b.eq #0x10e12f88
if cpu.condition(0){pc=0x10e12f88;continue;}pc=0x10e12f4c;continue;
}
0x10e12f4c=>{
// 0x10e12f4c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e12f50 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e12f54 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e12f58 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e12f5c=>{
// 0x10e12f5c mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10e12f60 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e12f64=>{
// 0x10e12f64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e12f68 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10e12f6c mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10e12f70 bl #0x10e12e3c
cpu.x[30]=0x10e12f74;super::dispatch(cpu,rt,0x10e12e3c)?;
// 0x10e12f74 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10e12f78 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e12f7c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e12f80 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e12f84 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e12f88=>{
// 0x10e12f88 ldr w8, [x20, #0xfd4]
{let address=(cpu.x[20]).wrapping_add(0xfd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e12f8c ldr x9, [x20, #0xfc8]
{let address=(cpu.x[20]).wrapping_add(0xfc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e12f90 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
pc=0x10e12f94;continue;
}
0x10e12f94=>{
// 0x10e12f94 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e12f98 b.lt #0x10e12fb4
if cpu.condition(11){pc=0x10e12fb4;continue;}pc=0x10e12f9c;continue;
}
0x10e12f9c=>{
// 0x10e12f9c add x11, x9, x10, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[10])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10e12fa0 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10e12fa4 ldur x11, [x11, #-8]
{let address=(cpu.x[11]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10e12fa8 cmp x11, x19
{let a=((cpu.x[11]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e12fac b.ne #0x10e12f94
if cpu.condition(1){pc=0x10e12f94;continue;}pc=0x10e12fb0;continue;
}
0x10e12fb0=>{
// 0x10e12fb0 b #0x10e12f4c
pc=0x10e12f4c;continue;
}
0x10e12fb4=>{
// 0x10e12fb4 ldr w10, [x20, #0xfd0]
{let address=(cpu.x[20]).wrapping_add(0xfd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e12fb8 cmp w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e12fbc b.ne #0x10e12ff0
if cpu.condition(1){pc=0x10e12ff0;continue;}pc=0x10e12fc0;continue;
}
0x10e12fc0=>{
// 0x10e12fc0 lsl w10, w8, #1
cpu.x[10]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10e12fc4 mov w11, #5
cpu.x[11]=(((0x5u64))) as u32 as u64;
// 0x10e12fc8 cmp w10, #0
{let a=((cpu.x[10] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e12fcc csel w1, w11, w10, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10e12fd0 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e12fd4 b.ge #0x10e12ff0
if cpu.condition(10){pc=0x10e12ff0;continue;}pc=0x10e12fd8;continue;
}
0x10e12fd8=>{
// 0x10e12fd8 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10e12fdc add x0, x20, #0xfc8
{let a=((cpu.x[20]));let b=((0xfc8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10e12fe0 bl #0x10c98db0
cpu.x[30]=0x10e12fe4;super::dispatch(cpu,rt,0x10c98db0)?;
// 0x10e12fe4 ldr w8, [x20, #0xfd4]
{let address=(cpu.x[20]).wrapping_add(0xfd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e12fe8 ldr x9, [x20, #0xfc8]
{let address=(cpu.x[20]).wrapping_add(0xfc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e12fec mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
pc=0x10e12ff0;continue;
}
0x10e12ff0=>{
// 0x10e12ff0 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10e12ff4 str x19, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10e12ff8 str w10, [x20, #0xfd4]
{let address=(cpu.x[20]).wrapping_add(0xfd4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10e12ffc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e13000 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e13004 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e13008 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e12f08")),}}}
#[inline]
pub(super) unsafe fn f_10e19cc8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e19cc8=>{
// 0x10e19cc8 ldr x0, [x0, #0xe60]
{let address=(cpu.x[0]).wrapping_add(0xe60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e19ccc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e19cc8")),}}}
#[inline]
pub(super) unsafe fn f_10e1b1c4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e1b1c4=>{
// 0x10e1b1c4 ldr x0, [x0, #0xf10]
{let address=(cpu.x[0]).wrapping_add(0xf10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1b1c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e1b1c4")),}}}
#[inline]
pub(super) unsafe fn f_10e3e218(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e3e218=>{
// 0x10e3e218 cmp w1, #2
{let a=((cpu.x[1] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e3e21c b.eq #0x10e3e230
if cpu.condition(0){pc=0x10e3e230;continue;}pc=0x10e3e220;continue;
}
0x10e3e220=>{
// 0x10e3e220 cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e3e224 b.ne #0x10e3e238
if cpu.condition(1){pc=0x10e3e238;continue;}pc=0x10e3e228;continue;
}
0x10e3e228=>{
// 0x10e3e228 ldr w0, [x0, #0x224]
{let address=(cpu.x[0]).wrapping_add(0x224u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e3e22c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e3e230=>{
// 0x10e3e230 mov w0, #0x64
cpu.x[0]=(((0x64u64))) as u32 as u64;
// 0x10e3e234 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e3e238=>{
// 0x10e3e238 ldr w0, [x0, #0x220]
{let address=(cpu.x[0]).wrapping_add(0x220u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e3e23c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e3e218")),}}}
#[inline]
pub(super) unsafe fn f_10e5ae4c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e5ae4c=>{
// 0x10e5ae4c ldr w0, [x0, #0x210]
{let address=(cpu.x[0]).wrapping_add(0x210u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e5ae50 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e5ae4c")),}}}
#[inline]
pub(super) unsafe fn f_10e69314(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e69314=>{
// 0x10e69314 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10e69318 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e69314")),}}}
#[inline]
pub(super) unsafe fn f_10e80610(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e80610=>{
// 0x10e80610 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e80614 ubfx w0, w8, #0x13, #3
cpu.x[0]=(((((cpu.x[8] as u32 as u64)) >> 19) & 0x7u64)) as u32 as u64;
// 0x10e80618 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e80610")),}}}
#[inline]
pub(super) unsafe fn f_10e8b6b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e8b6b4=>{
// 0x10e8b6b4 ldr w8, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e8b6b8 ldr x9, [x0, #0x98]
{let address=(cpu.x[0]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e8b6bc madd w8, w8, w2, w1
cpu.x[8]=((((cpu.x[1] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[2] as u32 as u64))))) as u32 as u64;
// 0x10e8b6c0 ldr w8, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e8b6c4 tbnz w8, #5, #0x10e8b6d8
if ((cpu.x[8] as u32 as u64) & (1u64 << 5)) != 0 {pc=0x10e8b6d8;continue;}pc=0x10e8b6c8;continue;
}
0x10e8b6c8=>{
// 0x10e8b6c8 mov w9, #0x50
cpu.x[9]=(((0x50u64))) as u32 as u64;
// 0x10e8b6cc tst w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10e8b6d0 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10e8b6d4 orr w4, w8, w3
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[3] as u32 as u64));let result=a | b;cpu.x[4]=(result) as u32 as u64;}
pc=0x10e8b6d8;continue;
}
0x10e8b6d8=>{
// 0x10e8b6d8 and w0, w4, #1
{let a=((cpu.x[4] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10e8b6dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e8b6b4")),}}}
#[inline]
pub(super) unsafe fn f_10ea4df4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ea4df4=>{
// 0x10ea4df4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10ea4df8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ea4df4")),}}}
#[inline]
pub(super) unsafe fn f_10f73d44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f73d44=>{
// 0x10f73d44 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f73d48 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f73d4c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f73d50 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f73d54 ldr x8, [x1, #0x10]
{let address=(cpu.x[1]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73d58 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f73d5c mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10f73d60 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f73d64 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73d68 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f73d6c bl #0x110cbf10
cpu.x[30]=0x10f73d70;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10f73d70;continue;
}
0x10f73d70=>{
// 0x10f73d70 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x10f73d74 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f73d78 mov w1, #0x85
cpu.x[1]=(((0x85u64))) as u32 as u64;
// 0x10f73d7c bl #0x111dd6dc
cpu.x[30]=0x10f73d80;super::chunk15::f_111dd6dc(cpu,rt,0x111dd6dc)?;
pc=0x10f73d80;continue;
}
0x10f73d80=>{
// 0x10f73d80 cbz x20, #0x10f73d90
if (cpu.x[20]) == 0 {pc=0x10f73d90;continue;}pc=0x10f73d84;continue;
}
0x10f73d84=>{
// 0x10f73d84 ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73d88 cmp x0, x8
{let a=((cpu.x[0]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f73d8c b.eq #0x10f73da4
if cpu.condition(0){pc=0x10f73da4;continue;}pc=0x10f73d90;continue;
}
0x10f73d90=>{
// 0x10f73d90 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73d94 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f73d98 ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73d9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f73da0;super::dispatch(cpu,rt,target)?;}
pc=0x10f73da0;continue;
}
0x10f73da0=>{
// 0x10f73da0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
pc=0x10f73da4;continue;
}
0x10f73da4=>{
// 0x10f73da4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73da8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f73dac mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f73db0 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73db4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f73db8;super::dispatch(cpu,rt,target)?;}
pc=0x10f73db8;continue;
}
0x10f73db8=>{
// 0x10f73db8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f73dbc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f73dc0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f73dc4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f73dc8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f73d44")),}}}
#[inline]
pub(super) unsafe fn f_10f74060(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74060=>{
// 0x10f74060 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f74064 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f74068 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7406c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f74070 ldr x8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f74074 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f74078 adrp x9, #0x11935000
cpu.x[9]=(0x11935000u64) as u64;
// 0x10f7407c add x9, x9, #0x560
{let a=((cpu.x[9]));let b=((0x560u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f74080 str x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f74084 cbz x8, #0x10f740d4
if (cpu.x[8]) == 0 {pc=0x10f740d4;continue;}pc=0x10f74088;continue;
}
0x10f74088=>{
// 0x10f74088 ldr x20, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f7408c cbz x20, #0x10f740ac
if (cpu.x[20]) == 0 {pc=0x10f740ac;continue;}pc=0x10f74090;continue;
}
0x10f74090=>{
// 0x10f74090 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f74094 ldr x21, [x0], #0x10
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;cpu.x[0]=((cpu.x[0]).wrapping_add(((0x10u64)))) as u64;}
// 0x10f74098 bl #0x1142fcc4
cpu.x[30]=0x10f7409c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10f7409c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f740a0 bl #0x1187d2c0
cpu.x[30]=0x10f740a4;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f740a4 mov x20, x21
cpu.x[20]=(((cpu.x[21]))) as u64;
// 0x10f740a8 cbnz x21, #0x10f74090
if (cpu.x[21]) != 0 {pc=0x10f74090;continue;}pc=0x10f740ac;continue;
}
0x10f740ac=>{
// 0x10f740ac ldr x8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f740b0 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f740b4 cbz x8, #0x10f740d0
if (cpu.x[8]) == 0 {pc=0x10f740d0;continue;}pc=0x10f740b8;continue;
}
0x10f740b8=>{
// 0x10f740b8 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10f740bc;continue;
}
0x10f740bc=>{
// 0x10f740bc ldr x10, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f740c0 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f740c4 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f740c8 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f740cc b.ne #0x10f740bc
if cpu.condition(1){pc=0x10f740bc;continue;}pc=0x10f740d0;continue;
}
0x10f740d0=>{
// 0x10f740d0 str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f740d4;continue;
}
0x10f740d4=>{
// 0x10f740d4 ldr x20, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f740d8 cbz x20, #0x10f740f8
if (cpu.x[20]) == 0 {pc=0x10f740f8;continue;}pc=0x10f740dc;continue;
}
0x10f740dc=>{
// 0x10f740dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f740e0 ldr x21, [x0], #0x10
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;cpu.x[0]=((cpu.x[0]).wrapping_add(((0x10u64)))) as u64;}
// 0x10f740e4 bl #0x1142fcc4
cpu.x[30]=0x10f740e8;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10f740e8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f740ec bl #0x1187d2c0
cpu.x[30]=0x10f740f0;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f740f0 mov x20, x21
cpu.x[20]=(((cpu.x[21]))) as u64;
// 0x10f740f4 cbnz x21, #0x10f740dc
if (cpu.x[21]) != 0 {pc=0x10f740dc;continue;}pc=0x10f740f8;continue;
}
0x10f740f8=>{
// 0x10f740f8 ldr x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f740fc str xzr, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f74100 cbz x0, #0x10f74114
if (cpu.x[0]) == 0 {pc=0x10f74114;continue;}pc=0x10f74104;continue;
}
0x10f74104=>{
// 0x10f74104 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f74108 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7410c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f74110 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
0x10f74114=>{
// 0x10f74114 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f74118 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7411c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f74120 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74060")),}}}
#[inline]
pub(super) unsafe fn f_10f777e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f777e0=>{
// 0x10f777e0 tbnz w1, #0x1f, #0x10f77880
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f77880;continue;}pc=0x10f777e4;continue;
}
0x10f777e4=>{
// 0x10f777e4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f777e8 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f777ec stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f777f0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f777f4 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f777f8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f777fc bl #0x10d36e90
cpu.x[30]=0x10f77800;super::chunk6::f_10d36e90(cpu,rt,0x10d36e90)?;
pc=0x10f77800;continue;
}
0x10f77800=>{
// 0x10f77800 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f77804 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f77808 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f7780c bl #0x10d3c460
cpu.x[30]=0x10f77810;super::chunk0::f_10d3c460(cpu,rt,0x10d3c460)?;
pc=0x10f77810;continue;
}
0x10f77810=>{
// 0x10f77810 ldr w22, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f77814 tbz w21, #0, #0x10f77888
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f77888;continue;}pc=0x10f77818;continue;
}
0x10f77818=>{
// 0x10f77818 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7781c b.lt #0x10f778b4
if cpu.condition(11){pc=0x10f778b4;continue;}pc=0x10f77820;continue;
}
0x10f77820=>{
// 0x10f77820 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10f77824 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
pc=0x10f77828;continue;
}
0x10f77828=>{
// 0x10f77828 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7782c mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f77830 bl #0x10d3c460
cpu.x[30]=0x10f77834;super::chunk0::f_10d3c460(cpu,rt,0x10d3c460)?;
// 0x10f77834 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77838 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7783c mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f77840 ldr x8, [x8, x22, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77844 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10f77848 ldr w8, [x8, #0x78]
{let address=(cpu.x[8]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7784c cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77850 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f77854 orr w21, w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a | b;cpu.x[21]=(result) as u32 as u64;}
// 0x10f77858 bl #0x10d3c460
cpu.x[30]=0x10f7785c;super::chunk0::f_10d3c460(cpu,rt,0x10d3c460)?;
// 0x10f7785c ldrsw x8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f77860 cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77864 b.lt #0x10f77828
if cpu.condition(11){pc=0x10f77828;continue;}pc=0x10f77868;continue;
}
0x10f77868=>{
// 0x10f77868 eor w8, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f7786c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77870 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f77874 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f77878 and w0, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f7787c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77880=>{
// 0x10f77880 and w0, wzr, #1
{let a=((0u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f77884 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77888=>{
// 0x10f77888 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7788c bl #0x10d3bff0
cpu.x[30]=0x10f77890;super::chunk6::f_10d3bff0(cpu,rt,0x10d3bff0)?;
pc=0x10f77890;continue;
}
0x10f77890=>{
// 0x10f77890 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77894 cinc w8, w0, lt
cpu.x[8]=(if cpu.condition(11){(((cpu.x[0] as u32 as u64))).wrapping_add(1)}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x10f77898 cmp w22, w8, asr #1
{let a=((cpu.x[22] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(1) as u64;let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7789c cset w8, lt
cpu.x[8]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10f778a0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f778a4 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f778a8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f778ac and w0, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f778b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f778b4=>{
// 0x10f778b4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f778b8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f778bc ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f778c0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f778c4 and w0, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f778c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f777e0")),}}}
#[inline]
pub(super) unsafe fn f_10f77b14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f77b14=>{
// 0x10f77b14 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f77b18 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f77b1c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f77b20 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f77b24 cmp w1, #2
{let a=((cpu.x[1] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b28 b.le #0x10f77b60
if cpu.condition(13){pc=0x10f77b60;continue;}pc=0x10f77b2c;continue;
}
0x10f77b2c=>{
// 0x10f77b2c cmp w1, #3
{let a=((cpu.x[1] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b30 b.eq #0x10f77b84
if cpu.condition(0){pc=0x10f77b84;continue;}pc=0x10f77b34;continue;
}
0x10f77b34=>{
// 0x10f77b34 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f77b38 cmp w1, #4
{let a=((cpu.x[1] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b3c mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f77b40 b.eq #0x10f77b54
if cpu.condition(0){pc=0x10f77b54;continue;}pc=0x10f77b44;continue;
}
0x10f77b44=>{
// 0x10f77b44 cmp w1, #5
{let a=((cpu.x[1] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b48 b.ne #0x10f77b74
if cpu.condition(1){pc=0x10f77b74;continue;}pc=0x10f77b4c;continue;
}
0x10f77b4c=>{
// 0x10f77b4c ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77b50 ldr w0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
pc=0x10f77b54;continue;
}
0x10f77b54=>{
// 0x10f77b54 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77b58 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f77b5c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77b60=>{
// 0x10f77b60 cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b64 b.eq #0x10f77bb8
if cpu.condition(0){pc=0x10f77bb8;continue;}pc=0x10f77b68;continue;
}
0x10f77b68=>{
// 0x10f77b68 cmp w1, #2
{let a=((cpu.x[1] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77b6c mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f77b70 b.eq #0x10f77b54
if cpu.condition(0){pc=0x10f77b54;continue;}pc=0x10f77b74;continue;
}
0x10f77b74=>{
// 0x10f77b74 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f77b78 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77b7c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f77b80 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77b84=>{
// 0x10f77b84 ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f77b88 ldr x9, [x0, #0xd0]
{let address=(cpu.x[0]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f77b8c cbz x9, #0x10f77c94
if (cpu.x[9]) == 0 {pc=0x10f77c94;continue;}pc=0x10f77b90;continue;
}
0x10f77b90=>{
// 0x10f77b90 fmov d0, x9
cpu.q[0]=(cpu.x[9]) as u128 & 0xffffffffffffffffu128;
// 0x10f77b94 sxtw x8, w19
cpu.x[8]=((((cpu.x[19] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10f77b98 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10f77b9c uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10f77ba0 fmov w10, s0
cpu.x[10]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10f77ba4 cmp x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77ba8 b.hi #0x10f77bf4
if cpu.condition(8){pc=0x10f77bf4;continue;}pc=0x10f77bac;continue;
}
0x10f77bac=>{
// 0x10f77bac sub x11, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u64;}
// 0x10f77bb0 and x13, x11, x8
{let a=((cpu.x[11]));let b=((cpu.x[8]));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f77bb4 b #0x10f77c08
pc=0x10f77c08;continue;
}
0x10f77bb8=>{
// 0x10f77bb8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f77bbc ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f77bc0 bl #0x10f93d2c
cpu.x[30]=0x10f77bc4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f77bc4 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f77bc8 bl #0x110cbf10
cpu.x[30]=0x10f77bcc;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10f77bcc mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f77bd0 bl #0x10e19dc8
cpu.x[30]=0x10f77bd4;super::dispatch(cpu,rt,0x10e19dc8)?;
// 0x10f77bd4 cbz x0, #0x10f77b54
if (cpu.x[0]) == 0 {pc=0x10f77b54;continue;}pc=0x10f77bd8;continue;
}
0x10f77bd8=>{
// 0x10f77bd8 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77bdc ldr x8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77be0 bics xzr, x0, x8
{let a=((cpu.x[0]));let b=((cpu.x[8]));let result=a & !b;let _ = result;cpu.logical_flags(result,64);}
// 0x10f77be4 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f77be8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77bec ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f77bf0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77bf4=>{
// 0x10f77bf4 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77bf8 mov x13, x8
cpu.x[13]=(((cpu.x[8]))) as u64;
// 0x10f77bfc b.hi #0x10f77c08
if cpu.condition(8){pc=0x10f77c08;continue;}pc=0x10f77c00;continue;
}
0x10f77c00=>{
// 0x10f77c00 udiv x11, x8, x9
{let a=((cpu.x[8])) as u64;let b=((cpu.x[9])) as u64;cpu.x[11]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f77c04 msub x13, x11, x9, x8
cpu.x[13]=((((cpu.x[8]))).wrapping_sub((((cpu.x[11]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10f77c08;continue;
}
0x10f77c08=>{
// 0x10f77c08 ldr x12, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f77c0c ldr x11, [x12, x13, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[13])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f77c10 cbz x11, #0x10f77c94
if (cpu.x[11]) == 0 {pc=0x10f77c94;continue;}pc=0x10f77c14;continue;
}
0x10f77c14=>{
// 0x10f77c14 ldr x14, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[14]=(v0) as u64;}
// 0x10f77c18 cbz x14, #0x10f77c94
if (cpu.x[14]) == 0 {pc=0x10f77c94;continue;}pc=0x10f77c1c;continue;
}
0x10f77c1c=>{
// 0x10f77c1c cmp x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c20 sub x11, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u64;}
// 0x10f77c24 b.hs #0x10f77c70
if cpu.condition(2){pc=0x10f77c70;continue;}pc=0x10f77c28;continue;
}
0x10f77c28=>{
// 0x10f77c28 b #0x10f77c40
pc=0x10f77c40;continue;
}
0x10f77c2c=>{
// 0x10f77c2c ldr w15, [x14, #0x10]
{let address=(cpu.x[14]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x10f77c30 cmp w15, w19
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77c34 b.eq #0x10f77cb4
if cpu.condition(0){pc=0x10f77cb4;continue;}pc=0x10f77c38;continue;
}
0x10f77c38=>{
// 0x10f77c38 ldr x14, [x14]
{let address=(cpu.x[14]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[14]=(v0) as u64;}
// 0x10f77c3c cbz x14, #0x10f77c94
if (cpu.x[14]) == 0 {pc=0x10f77c94;continue;}pc=0x10f77c40;continue;
}
0x10f77c40=>{
// 0x10f77c40 ldr x15, [x14, #8]
{let address=(cpu.x[14]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[15]=(v0) as u64;}
// 0x10f77c44 cmp x15, x8
{let a=((cpu.x[15]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c48 b.eq #0x10f77c2c
if cpu.condition(0){pc=0x10f77c2c;continue;}pc=0x10f77c4c;continue;
}
0x10f77c4c=>{
// 0x10f77c4c and x15, x15, x11
{let a=((cpu.x[15]));let b=((cpu.x[11]));let result=a & b;cpu.x[15]=(result) as u64;}
// 0x10f77c50 cmp x15, x13
{let a=((cpu.x[15]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c54 b.eq #0x10f77c38
if cpu.condition(0){pc=0x10f77c38;continue;}pc=0x10f77c58;continue;
}
0x10f77c58=>{
// 0x10f77c58 b #0x10f77c94
pc=0x10f77c94;continue;
}
0x10f77c5c=>{
// 0x10f77c5c ldr w15, [x14, #0x10]
{let address=(cpu.x[14]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x10f77c60 cmp w15, w19
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77c64 b.eq #0x10f77cb4
if cpu.condition(0){pc=0x10f77cb4;continue;}pc=0x10f77c68;continue;
}
0x10f77c68=>{
// 0x10f77c68 ldr x14, [x14]
{let address=(cpu.x[14]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[14]=(v0) as u64;}
// 0x10f77c6c cbz x14, #0x10f77c94
if (cpu.x[14]) == 0 {pc=0x10f77c94;continue;}pc=0x10f77c70;continue;
}
0x10f77c70=>{
// 0x10f77c70 ldr x15, [x14, #8]
{let address=(cpu.x[14]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[15]=(v0) as u64;}
// 0x10f77c74 cmp x15, x8
{let a=((cpu.x[15]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c78 b.eq #0x10f77c5c
if cpu.condition(0){pc=0x10f77c5c;continue;}pc=0x10f77c7c;continue;
}
0x10f77c7c=>{
// 0x10f77c7c cmp x15, x9
{let a=((cpu.x[15]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c80 b.lo #0x10f77c8c
if cpu.condition(3){pc=0x10f77c8c;continue;}pc=0x10f77c84;continue;
}
0x10f77c84=>{
// 0x10f77c84 udiv x16, x15, x9
{let a=((cpu.x[15])) as u64;let b=((cpu.x[9])) as u64;cpu.x[16]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f77c88 msub x15, x16, x9, x15
cpu.x[15]=((((cpu.x[15]))).wrapping_sub((((cpu.x[16]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10f77c8c;continue;
}
0x10f77c8c=>{
// 0x10f77c8c cmp x15, x13
{let a=((cpu.x[15]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77c90 b.eq #0x10f77c68
if cpu.condition(0){pc=0x10f77c68;continue;}pc=0x10f77c94;continue;
}
0x10f77c94=>{
// 0x10f77c94 bl #0x10f89114
cpu.x[30]=0x10f77c98;super::chunk6::f_10f89114(cpu,rt,0x10f89114)?;
pc=0x10f77c98;continue;
}
0x10f77c98=>{
// 0x10f77c98 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f77c9c bl #0x10e1abac
cpu.x[30]=0x10f77ca0;super::chunk14::f_10e1abac(cpu,rt,0x10e1abac)?;
pc=0x10f77ca0;continue;
}
0x10f77ca0=>{
// 0x10f77ca0 cbz x0, #0x10f77b54
if (cpu.x[0]) == 0 {pc=0x10f77b54;continue;}pc=0x10f77ca4;continue;
}
0x10f77ca4=>{
// 0x10f77ca4 ldr w0, [x0, #0x68]
{let address=(cpu.x[0]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f77ca8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77cac ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f77cb0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f77cb4=>{
// 0x10f77cb4 cmp x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77cb8 b.hi #0x10f77cc4
if cpu.condition(8){pc=0x10f77cc4;continue;}pc=0x10f77cbc;continue;
}
0x10f77cbc=>{
// 0x10f77cbc and x13, x11, x8
{let a=((cpu.x[11]));let b=((cpu.x[8]));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f77cc0 b #0x10f77cd8
pc=0x10f77cd8;continue;
}
0x10f77cc4=>{
// 0x10f77cc4 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77cc8 mov x13, x8
cpu.x[13]=(((cpu.x[8]))) as u64;
// 0x10f77ccc b.hi #0x10f77cd8
if cpu.condition(8){pc=0x10f77cd8;continue;}pc=0x10f77cd0;continue;
}
0x10f77cd0=>{
// 0x10f77cd0 udiv x13, x8, x9
{let a=((cpu.x[8])) as u64;let b=((cpu.x[9])) as u64;cpu.x[13]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f77cd4 msub x13, x13, x9, x8
cpu.x[13]=((((cpu.x[8]))).wrapping_sub((((cpu.x[13]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10f77cd8;continue;
}
0x10f77cd8=>{
// 0x10f77cd8 ldr x12, [x12, x13, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[13])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f77cdc cbz x12, #0x10f77b74
if (cpu.x[12]) == 0 {pc=0x10f77b74;continue;}pc=0x10f77ce0;continue;
}
0x10f77ce0=>{
// 0x10f77ce0 ldr x12, [x12]
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f77ce4 cbz x12, #0x10f77b74
if (cpu.x[12]) == 0 {pc=0x10f77b74;continue;}pc=0x10f77ce8;continue;
}
0x10f77ce8=>{
// 0x10f77ce8 cmp x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77cec b.hs #0x10f77d40
if cpu.condition(2){pc=0x10f77d40;continue;}pc=0x10f77cf0;continue;
}
0x10f77cf0=>{
// 0x10f77cf0 b #0x10f77d0c
pc=0x10f77d0c;continue;
}
0x10f77cf4=>{
// 0x10f77cf4 ldr w9, [x12, #0x10]
{let address=(cpu.x[12]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f77cf8 cmp w9, w19
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77cfc b.eq #0x10f77d68
if cpu.condition(0){pc=0x10f77d68;continue;}pc=0x10f77d00;continue;
}
0x10f77d00=>{
// 0x10f77d00 ldr x12, [x12]
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f77d04 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f77d08 cbz x12, #0x10f77b54
if (cpu.x[12]) == 0 {pc=0x10f77b54;continue;}pc=0x10f77d0c;continue;
}
0x10f77d0c=>{
// 0x10f77d0c ldr x9, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f77d10 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77d14 b.eq #0x10f77cf4
if cpu.condition(0){pc=0x10f77cf4;continue;}pc=0x10f77d18;continue;
}
0x10f77d18=>{
// 0x10f77d18 and x9, x9, x11
{let a=((cpu.x[9]));let b=((cpu.x[11]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10f77d1c cmp x9, x13
{let a=((cpu.x[9]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77d20 b.eq #0x10f77d00
if cpu.condition(0){pc=0x10f77d00;continue;}pc=0x10f77d24;continue;
}
0x10f77d24=>{
// 0x10f77d24 b #0x10f77b74
pc=0x10f77b74;continue;
}
0x10f77d28=>{
// 0x10f77d28 ldr w10, [x12, #0x10]
{let address=(cpu.x[12]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f77d2c cmp w10, w19
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77d30 b.eq #0x10f77d68
if cpu.condition(0){pc=0x10f77d68;continue;}pc=0x10f77d34;continue;
}
0x10f77d34=>{
// 0x10f77d34 ldr x12, [x12]
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f77d38 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f77d3c cbz x12, #0x10f77b54
if (cpu.x[12]) == 0 {pc=0x10f77b54;continue;}pc=0x10f77d40;continue;
}
0x10f77d40=>{
// 0x10f77d40 ldr x10, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f77d44 cmp x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77d48 b.eq #0x10f77d28
if cpu.condition(0){pc=0x10f77d28;continue;}pc=0x10f77d4c;continue;
}
0x10f77d4c=>{
// 0x10f77d4c cmp x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77d50 b.lo #0x10f77d5c
if cpu.condition(3){pc=0x10f77d5c;continue;}pc=0x10f77d54;continue;
}
0x10f77d54=>{
// 0x10f77d54 udiv x11, x10, x9
{let a=((cpu.x[10])) as u64;let b=((cpu.x[9])) as u64;cpu.x[11]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f77d58 msub x10, x11, x9, x10
cpu.x[10]=((((cpu.x[10]))).wrapping_sub((((cpu.x[11]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10f77d5c;continue;
}
0x10f77d5c=>{
// 0x10f77d5c cmp x10, x13
{let a=((cpu.x[10]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77d60 b.eq #0x10f77d34
if cpu.condition(0){pc=0x10f77d34;continue;}pc=0x10f77d64;continue;
}
0x10f77d64=>{
// 0x10f77d64 b #0x10f77b74
pc=0x10f77b74;continue;
}
0x10f77d68=>{
// 0x10f77d68 ldr w0, [x12, #0x14]
{let address=(cpu.x[12]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f77d6c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77d70 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f77d74 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f77b14")),}}}
#[inline]
pub(super) unsafe fn f_10f77d7c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f77d7c=>{
// 0x10f77d7c ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77d80 adrp x9, #0x11908000
cpu.x[9]=(0x11908000u64) as u64;
// 0x10f77d84 add x9, x9, #0xe60
{let a=((cpu.x[9]));let b=((0xe60u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f77d88 str x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f77d8c cbz x8, #0x10f77db4
if (cpu.x[8]) == 0 {pc=0x10f77db4;continue;}pc=0x10f77d90;continue;
}
0x10f77d90=>{
// 0x10f77d90 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f77d94 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f77d98 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f77d9c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f77da0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f77da4 bl #0x1187d360
cpu.x[30]=0x10f77da8;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f77da8;continue;
}
0x10f77da8=>{
// 0x10f77da8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f77dac ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f77db0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
pc=0x10f77db4;continue;
}
0x10f77db4=>{
// 0x10f77db4 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f77d7c")),}}}
#[inline]
pub(super) unsafe fn f_10f7a048(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7a048=>{
// 0x10f7a048 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7a04c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a050 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7a054 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7a058 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7a05c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f7a060 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7a064 bl #0x10f93d44
cpu.x[30]=0x10f7a068;super::chunk8::f_10f93d44(cpu,rt,0x10f93d44)?;
pc=0x10f7a068;continue;
}
0x10f7a068=>{
// 0x10f7a068 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a06c ldr x21, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7a070 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a074 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a078 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a07c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a07c;continue;
}
0x10f7a07c=>{
// 0x10f7a07c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a080 str w0, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a084 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a088 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a08c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a090;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a090;continue;
}
0x10f7a090=>{
// 0x10f7a090 str w0, [x19, #0xfc]
{let address=(cpu.x[19]).wrapping_add(0xfcu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a094 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a098 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a09c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a0a4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a0a4;continue;
}
0x10f7a0a4=>{
// 0x10f7a0a4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0a8 str w0, [x19, #0x104]
{let address=(cpu.x[19]).wrapping_add(0x104u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a0ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a0b0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a0b8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a0b8;continue;
}
0x10f7a0b8=>{
// 0x10f7a0b8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0bc str w0, [x19, #0x114]
{let address=(cpu.x[19]).wrapping_add(0x114u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a0c0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a0c4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a0cc;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a0cc;continue;
}
0x10f7a0cc=>{
// 0x10f7a0cc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0d0 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f7a0d4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a0d8 strb w9, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f7a0dc ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a0e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a0e4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a0e4;continue;
}
0x10f7a0e4=>{
// 0x10f7a0e4 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7a0e8 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7a0ec and w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f7a0f0 strb w8, [x19, #0x119]
{let address=(cpu.x[19]).wrapping_add(0x119u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f7a0f4 cbz x0, #0x10f7a0fc
if (cpu.x[0]) == 0 {pc=0x10f7a0fc;continue;}pc=0x10f7a0f8;continue;
}
0x10f7a0f8=>{
// 0x10f7a0f8 bl #0x1187d360
cpu.x[30]=0x10f7a0fc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f7a0fc;continue;
}
0x10f7a0fc=>{
// 0x10f7a0fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a100 str xzr, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a104 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a108 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a10c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a110;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a110;continue;
}
0x10f7a110=>{
// 0x10f7a110 tbz w0, #0, #0x10f7a190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7a190;continue;}pc=0x10f7a114;continue;
}
0x10f7a114=>{
// 0x10f7a114 ldr w9, [x19, #0x114]
{let address=(cpu.x[19]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7a118 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a11c tbz w9, #0x1f, #0x10f7a124
if ((cpu.x[9] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10f7a124;continue;}pc=0x10f7a120;continue;
}
0x10f7a120=>{
// 0x10f7a120 ldr w9, [x8, #0x148]
{let address=(cpu.x[8]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f7a124;continue;
}
0x10f7a124=>{
// 0x10f7a124 ldr w10, [x8, #0x114]
{let address=(cpu.x[8]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f7a128 ldr w11, [x8, #0x10c]
{let address=(cpu.x[8]).wrapping_add(0x10cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f7a12c ldr w8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7a130 sub w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f7a134 sdiv w9, w9, w11
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[11] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f7a138 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7a13c csel w10, w8, w9, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f7a140 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7a144 csel w22, w9, w10, eq
cpu.x[22]=(if cpu.condition(0){((cpu.x[9] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10f7a148 sbfiz x8, x22, #2, #0x20
cpu.x[8]=((((((cpu.x[22])) << 32) as i64 >> 32) as u64).wrapping_shl(2)) as u64;
// 0x10f7a14c cmp w22, #0
{let a=((cpu.x[22] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7a150 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x10f7a154 bl #0x1187d520
cpu.x[30]=0x10f7a158;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10f7a158;continue;
}
0x10f7a158=>{
// 0x10f7a158 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7a15c str x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a160 b.lt #0x10f7a190
if cpu.condition(11){pc=0x10f7a190;continue;}pc=0x10f7a164;continue;
}
0x10f7a164=>{
// 0x10f7a164 ubfiz x22, x22, #2, #0x20
cpu.x[22]=(((((cpu.x[22])) & 0xffffffffu64) << 2)) as u64;
// 0x10f7a168 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
pc=0x10f7a16c;continue;
}
0x10f7a16c=>{
// 0x10f7a16c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a170 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a174 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a178 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a17c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a17c;continue;
}
0x10f7a17c=>{
// 0x10f7a17c ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a180 str w0, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a184 add x23, x23, #4
{let a=((cpu.x[23]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10f7a188 cmp x22, x23
{let a=((cpu.x[22]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7a18c b.ne #0x10f7a16c
if cpu.condition(1){pc=0x10f7a16c;continue;}pc=0x10f7a190;continue;
}
0x10f7a190=>{
// 0x10f7a190 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a194 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a198 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a19c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a1a0;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a1a0;continue;
}
0x10f7a1a0=>{
// 0x10f7a1a0 str w0, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a1a4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7a1a8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7a1ac bl #0x10f992f0
cpu.x[30]=0x10f7a1b0;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10f7a1b0;continue;
}
0x10f7a1b0=>{
// 0x10f7a1b0 str x0, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a1b4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7a1b8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7a1bc bl #0x10f992f0
cpu.x[30]=0x10f7a1c0;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10f7a1c0;continue;
}
0x10f7a1c0=>{
// 0x10f7a1c0 str x0, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a1c4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7a1c8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7a1cc mov w2, #5
cpu.x[2]=(((0x5u64))) as u32 as u64;
// 0x10f7a1d0 bl #0x10f8bfe0
cpu.x[30]=0x10f7a1d4;super::chunk2::f_10f8bfe0(cpu,rt,0x10f8bfe0)?;
pc=0x10f7a1d4;continue;
}
0x10f7a1d4=>{
// 0x10f7a1d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a1d8 str x0, [x19, #0x140]
{let address=(cpu.x[19]).wrapping_add(0x140u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7a1dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a1e0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a1e4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a1e8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a1e8;continue;
}
0x10f7a1e8=>{
// 0x10f7a1e8 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a1ec and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f7a1f0 strb w9, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f7a1f4 ldrb w8, [x8, #0xbc]
{let address=(cpu.x[8]).wrapping_add(0xbcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7a1f8 cbz w8, #0x10f7a208
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7a208;continue;}pc=0x10f7a1fc;continue;
}
0x10f7a1fc=>{
// 0x10f7a1fc add x1, x19, #0x120
{let a=((cpu.x[19]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f7a200 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a204 bl #0x111dd4e8
cpu.x[30]=0x10f7a208;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
pc=0x10f7a208;continue;
}
0x10f7a208=>{
// 0x10f7a208 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a20c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a210 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a214 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a218;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a218;continue;
}
0x10f7a218=>{
// 0x10f7a218 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a21c str w0, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a220 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7a224 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7a228 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7a22c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7a22c;continue;
}
0x10f7a22c=>{
// 0x10f7a22c str w0, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7a230 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7a234 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f7a238 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f7a23c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f7a240 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7a048")),}}}
#[inline]
pub(super) unsafe fn f_10f7b3cc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7b3cc=>{
// 0x10f7b3cc mov w0, #3
cpu.x[0]=(((0x3u64))) as u32 as u64;
// 0x10f7b3d0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7b3cc")),}}}
#[inline]
pub(super) unsafe fn f_10f81f60(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f81f60=>{
// 0x10f81f60 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f81f64 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f81f68 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f81f6c cbnz w3, #0x10f81fc4
if (cpu.x[3] as u32 as u64) != 0 {pc=0x10f81fc4;continue;}pc=0x10f81f70;continue;
}
0x10f81f70=>{
// 0x10f81f70 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f81f74 ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f81f78 mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10f81f7c bl #0x10de0be8
cpu.x[30]=0x10f81f80;super::chunk6::f_10de0be8(cpu,rt,0x10de0be8)?;
pc=0x10f81f80;continue;
}
0x10f81f80=>{
// 0x10f81f80 tbz w0, #0, #0x10f81fc4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f81fc4;continue;}pc=0x10f81f84;continue;
}
0x10f81f84=>{
// 0x10f81f84 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f81f88 bl #0x10f9433c
cpu.x[30]=0x10f81f8c;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
// 0x10f81f8c cbz x0, #0x10f81fc4
if (cpu.x[0]) == 0 {pc=0x10f81fc4;continue;}pc=0x10f81f90;continue;
}
0x10f81f90=>{
// 0x10f81f90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f81f94 bl #0x10f9433c
cpu.x[30]=0x10f81f98;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
// 0x10f81f98 bl #0x10fc3ca0
cpu.x[30]=0x10f81f9c;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f81f9c cmp x0, x20
{let a=((cpu.x[0]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f81fa0 b.ne #0x10f81fc4
if cpu.condition(1){pc=0x10f81fc4;continue;}pc=0x10f81fa4;continue;
}
0x10f81fa4=>{
// 0x10f81fa4 ldr w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f81fa8 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f81fac b.ne #0x10f81fc4
if cpu.condition(1){pc=0x10f81fc4;continue;}pc=0x10f81fb0;continue;
}
0x10f81fb0=>{
// 0x10f81fb0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f81fb4 bl #0x10f9433c
cpu.x[30]=0x10f81fb8;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
// 0x10f81fb8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f81fbc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f81fc0 b #0x10fc3fd4
super::dispatch(cpu,rt,0x10fc3fd4)?;return Ok(());
}
0x10f81fc4=>{
// 0x10f81fc4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f81fc8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f81fcc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f81f60")),}}}
#[inline]
pub(super) unsafe fn f_10f82058(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f82058=>{
// 0x10f82058 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8205c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f82060 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f82064 ldr w8, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82068 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8206c mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f82070 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82074 b.ne #0x10f82098
if cpu.condition(1){pc=0x10f82098;continue;}pc=0x10f82078;continue;
}
0x10f82078=>{
// 0x10f82078 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8207c bl #0x10de08c0
cpu.x[30]=0x10f82080;super::chunk3::f_10de08c0(cpu,rt,0x10de08c0)?;
// 0x10f82080 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82084 b.le #0x10f82098
if cpu.condition(13){pc=0x10f82098;continue;}pc=0x10f82088;continue;
}
0x10f82088=>{
// 0x10f82088 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f8208c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f82090 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82094 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82098=>{
// 0x10f82098 ldr w8, [x19, #0x1a4]
{let address=(cpu.x[19]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8209c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f820a0 cset w8, gt
cpu.x[8]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10f820a4 and w0, w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f820a8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f820ac ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f820b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f82058")),}}}
#[inline]
pub(super) unsafe fn f_10f82e68(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f82e68=>{
// 0x10f82e68 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10f82e6c add x8, x8, #0xfd8
{let a=((cpu.x[8]));let b=((0xfd8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f82e70 ldr x9, [x0, #0xb0]
{let address=(cpu.x[0]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f82e74 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82e78 and x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10f82e7c ldr x9, [x0, #0x180]
{let address=(cpu.x[0]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f82e80 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f82e84 ccmp x9, #0, #0, eq
if cpu.condition(0){{let a=((cpu.x[9]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10f82e88 b.eq #0x10f82e94
if cpu.condition(0){pc=0x10f82e94;continue;}pc=0x10f82e8c;continue;
}
0x10f82e8c=>{
// 0x10f82e8c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82e90 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82e94=>{
// 0x10f82e94 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f82e98 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f82e9c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f82ea0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f82ea4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f82ea8 tbz w2, #0, #0x10f82f24
if ((cpu.x[2] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82f24;continue;}pc=0x10f82eac;continue;
}
0x10f82eac=>{
// 0x10f82eac ldr w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82eb0 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82eb4 b.ne #0x10f82ed0
if cpu.condition(1){pc=0x10f82ed0;continue;}pc=0x10f82eb8;continue;
}
0x10f82eb8=>{
// 0x10f82eb8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f82ebc mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f82ec0 bl #0x10de0b2c
cpu.x[30]=0x10f82ec4;super::chunk13::f_10de0b2c(cpu,rt,0x10de0b2c)?;
// 0x10f82ec4 ldr w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82ec8 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82ecc b.gt #0x10f82f00
if cpu.condition(12){pc=0x10f82f00;continue;}pc=0x10f82ed0;continue;
}
0x10f82ed0=>{
// 0x10f82ed0 cmp w8, #0xc
{let a=((cpu.x[8] as u32 as u64));let b=((0xcu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82ed4 b.hi #0x10f82f00
if cpu.condition(8){pc=0x10f82f00;continue;}pc=0x10f82ed8;continue;
}
0x10f82ed8=>{
// 0x10f82ed8 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
// 0x10f82edc mov w10, #0x1028
cpu.x[10]=(((0x1028u64))) as u32 as u64;
// 0x10f82ee0 lsl w9, w9, w8
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as u32).wrapping_shl(((((cpu.x[8] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f82ee4 tst w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10f82ee8 b.eq #0x10f82f00
if cpu.condition(0){pc=0x10f82f00;continue;}pc=0x10f82eec;continue;
}
0x10f82eec=>{
// 0x10f82eec mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82ef0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f82ef4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f82ef8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f82efc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82f00=>{
// 0x10f82f00 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82f04 b.ne #0x10f82f24
if cpu.condition(1){pc=0x10f82f24;continue;}pc=0x10f82f08;continue;
}
0x10f82f08=>{
// 0x10f82f08 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f0c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82f10 ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f82f18;super::dispatch(cpu,rt,target)?;}
pc=0x10f82f18;continue;
}
0x10f82f18=>{
// 0x10f82f18 tbz w0, #0, #0x10f82f24
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82f24;continue;}pc=0x10f82f1c;continue;
}
0x10f82f1c=>{
// 0x10f82f1c ldrb w8, [x19, #0x1f4]
{let address=(cpu.x[19]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82f20 cbz w8, #0x10f82eec
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f82eec;continue;}pc=0x10f82f24;continue;
}
0x10f82f24=>{
// 0x10f82f24 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82f2c ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f82f34;super::dispatch(cpu,rt,target)?;}
pc=0x10f82f34;continue;
}
0x10f82f34=>{
// 0x10f82f34 tbz w0, #0, #0x10f82fc4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82fc4;continue;}pc=0x10f82f38;continue;
}
0x10f82f38=>{
// 0x10f82f38 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f82f3c bl #0x10e0b614
cpu.x[30]=0x10f82f40;super::chunk0::f_10e0b614(cpu,rt,0x10e0b614)?;
pc=0x10f82f40;continue;
}
0x10f82f40=>{
// 0x10f82f40 cmp w0, #0x22
{let a=((cpu.x[0] as u32 as u64));let b=((0x22u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82f44 b.ne #0x10f82fc4
if cpu.condition(1){pc=0x10f82fc4;continue;}pc=0x10f82f48;continue;
}
0x10f82f48=>{
// 0x10f82f48 ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f82f4c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82f50 bl #0x10f94298
cpu.x[30]=0x10f82f54;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f82f54 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f82f58 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82f5c bl #0x10f942a0
cpu.x[30]=0x10f82f60;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f82f60 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f82f64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f82f68 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f82f6c mov w3, #0x7d0
cpu.x[3]=(((0x7d0u64))) as u32 as u64;
// 0x10f82f70 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f82f74 bl #0x10f98db8
cpu.x[30]=0x10f82f78;super::chunk10::f_10f98db8(cpu,rt,0x10f98db8)?;
// 0x10f82f78 cbz x0, #0x10f82fc4
if (cpu.x[0]) == 0 {pc=0x10f82fc4;continue;}pc=0x10f82f7c;continue;
}
0x10f82f7c=>{
// 0x10f82f7c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f82f80 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f82f84 bl #0x10f94f34
cpu.x[30]=0x10f82f88;super::chunk8::f_10f94f34(cpu,rt,0x10f94f34)?;
// 0x10f82f88 bl #0x112944a4
cpu.x[30]=0x10f82f8c;super::chunk0::f_112944a4(cpu,rt,0x112944a4)?;
// 0x10f82f8c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f90 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f82f94 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f82f98 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82f9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f82fa0;super::dispatch(cpu,rt,target)?;}
// 0x10f82fa0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82fa4 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f82fa8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f82fac ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82fb0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f82fb4;super::dispatch(cpu,rt,target)?;}
// 0x10f82fb4 sub w8, w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f82fb8 sub w8, w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f82fbc cmp w8, #0x1f4
{let a=((cpu.x[8] as u32 as u64));let b=((0x1f4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82fc0 b.lt #0x10f82eec
if cpu.condition(11){pc=0x10f82eec;continue;}pc=0x10f82fc4;continue;
}
0x10f82fc4=>{
// 0x10f82fc4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f82fc8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f82fcc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f82fd0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f82fd4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f82e68")),}}}
#[inline]
pub(super) unsafe fn f_10f848e8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f848e8=>{
// 0x10f848e8 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f848ec str x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f848f0 stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f848f4 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f848f8 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f848fc stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f84900 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f84904 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f84908 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8490c bl #0x10f95bec
cpu.x[30]=0x10f84910;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10f84910;continue;
}
0x10f84910=>{
// 0x10f84910 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f84914 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f84918 bl #0x10d3cac8
cpu.x[30]=0x10f8491c;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10f8491c;continue;
}
0x10f8491c=>{
// 0x10f8491c cbz x0, #0x10f84a20
if (cpu.x[0]) == 0 {pc=0x10f84a20;continue;}pc=0x10f84920;continue;
}
0x10f84920=>{
// 0x10f84920 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f84924 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f84928 bl #0x10f95b64
cpu.x[30]=0x10f8492c;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
pc=0x10f8492c;continue;
}
0x10f8492c=>{
// 0x10f8492c cbz x0, #0x10f84a20
if (cpu.x[0]) == 0 {pc=0x10f84a20;continue;}pc=0x10f84930;continue;
}
0x10f84930=>{
// 0x10f84930 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f84934 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f84938 bl #0x10f95c08
cpu.x[30]=0x10f8493c;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f8493c;continue;
}
0x10f8493c=>{
// 0x10f8493c ldr w8, [x21, #0x14]
{let address=(cpu.x[21]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f84940 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f84944 b.lt #0x10f84a20
if cpu.condition(11){pc=0x10f84a20;continue;}pc=0x10f84948;continue;
}
0x10f84948=>{
// 0x10f84948 ldr x23, [x23, #0x10]
{let address=(cpu.x[23]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f8494c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f84950 mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
// 0x10f84954 mov w27, wzr
cpu.x[27]=(((0u64))) as u32 as u64;
// 0x10f84958 b #0x10f8496c
pc=0x10f8496c;continue;
}
0x10f8495c=>{
// 0x10f8495c ldrsw x8, [x21, #0x14]
{let address=(cpu.x[21]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f84960 add x26, x26, #1
{let a=((cpu.x[26]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10f84964 cmp x26, x8
{let a=((cpu.x[26]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f84968 b.ge #0x10f849c4
if cpu.condition(10){pc=0x10f849c4;continue;}pc=0x10f8496c;continue;
}
0x10f8496c=>{
// 0x10f8496c ldr x8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f84970 ldr x24, [x8, x26, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[26])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f84974 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f84978 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f8497c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f84980 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f84984;super::dispatch(cpu,rt,target)?;}
pc=0x10f84984;continue;
}
0x10f84984=>{
// 0x10f84984 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f84988 b.ne #0x10f8495c
if cpu.condition(1){pc=0x10f8495c;continue;}pc=0x10f8498c;continue;
}
0x10f8498c=>{
// 0x10f8498c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f84990 bl #0x10f95b64
cpu.x[30]=0x10f84994;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
pc=0x10f84994;continue;
}
0x10f84994=>{
// 0x10f84994 cbz x0, #0x10f8495c
if (cpu.x[0]) == 0 {pc=0x10f8495c;continue;}pc=0x10f84998;continue;
}
0x10f84998=>{
// 0x10f84998 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f8499c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f849a0 bl #0x10f95c08
cpu.x[30]=0x10f849a4;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f849a4;continue;
}
0x10f849a4=>{
// 0x10f849a4 cmp w0, w22
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f849a8 b.ne #0x10f8495c
if cpu.condition(1){pc=0x10f8495c;continue;}pc=0x10f849ac;continue;
}
0x10f849ac=>{
// 0x10f849ac ldr x0, [x25, #0x10]
{let address=(cpu.x[25]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f849b0 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f849b4 bl #0x10e0bc20
cpu.x[30]=0x10f849b8;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10f849b8;continue;
}
0x10f849b8=>{
// 0x10f849b8 and w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f849bc add w27, w27, w8
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10f849c0 b #0x10f8495c
pc=0x10f8495c;continue;
}
0x10f849c4=>{
// 0x10f849c4 cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f849c8 b.ne #0x10f84a20
if cpu.condition(1){pc=0x10f84a20;continue;}pc=0x10f849cc;continue;
}
0x10f849cc=>{
// 0x10f849cc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f849d0 bl #0x10f93d2c
cpu.x[30]=0x10f849d4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f849d4;continue;
}
0x10f849d4=>{
// 0x10f849d4 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f849d8 bl #0x110ca998
cpu.x[30]=0x10f849dc;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x10f849dc;continue;
}
0x10f849dc=>{
// 0x10f849dc ldr w8, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f849e0 ldr w9, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f849e4 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f849e8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f849ec csel w8, w9, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10f849f0 mov w9, #0x32
cpu.x[9]=(((0x32u64))) as u32 as u64;
// 0x10f849f4 sub w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f849f8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f849fc mul w3, w8, w9
cpu.x[3]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10f84a00 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f84a04 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f84a08 ldr x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10f84a0c ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f84a10 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f84a14 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f84a18 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10f84a1c b #0x10d54b28
super::chunk0::f_10d54b28(cpu,rt,0x10d54b28)?;return Ok(());
}
0x10f84a20=>{
// 0x10f84a20 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f84a24 ldr x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10f84a28 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f84a2c ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f84a30 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f84a34 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10f84a38 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f848e8")),}}}
#[inline]
pub(super) unsafe fn f_10f85d04(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85d04=>{
// 0x10f85d04 ldr x8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85d08 ldr x0, [x8, #0x8d8]
{let address=(cpu.x[8]).wrapping_add(0x8d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85d0c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85d04")),}}}
#[inline]
pub(super) unsafe fn f_10f86ce0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86ce0=>{
// 0x10f86ce0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86ce4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86ce8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86cec mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86cf0 ldr x0, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f86cf4 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f86cf8 cbz x0, #0x10f86d04
if (cpu.x[0]) == 0 {pc=0x10f86d04;continue;}pc=0x10f86cfc;continue;
}
0x10f86cfc=>{
// 0x10f86cfc bl #0x10f878a0
cpu.x[30]=0x10f86d00;super::chunk14::f_10f878a0(cpu,rt,0x10f878a0)?;
// 0x10f86d00 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f86d04;continue;
}
0x10f86d04=>{
// 0x10f86d04 cbz x20, #0x10f86d44
if (cpu.x[20]) == 0 {pc=0x10f86d44;continue;}pc=0x10f86d08;continue;
}
0x10f86d08=>{
// 0x10f86d08 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86d0c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86d10 ldr x8, [x8, #0x198]
{let address=(cpu.x[8]).wrapping_add(0x198u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86d14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f86d18;super::dispatch(cpu,rt,target)?;}
pc=0x10f86d18;continue;
}
0x10f86d18=>{
// 0x10f86d18 tbz w0, #0, #0x10f86d44
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f86d44;continue;}pc=0x10f86d1c;continue;
}
0x10f86d1c=>{
// 0x10f86d1c ldr x8, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86d20 cbz x8, #0x10f86d44
if (cpu.x[8]) == 0 {pc=0x10f86d44;continue;}pc=0x10f86d24;continue;
}
0x10f86d24=>{
// 0x10f86d24 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86d28 bl #0x10f93f94
cpu.x[30]=0x10f86d2c;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10f86d2c;continue;
}
0x10f86d2c=>{
// 0x10f86d2c bl #0x10f9915c
cpu.x[30]=0x10f86d30;super::chunk11::f_10f9915c(cpu,rt,0x10f9915c)?;
pc=0x10f86d30;continue;
}
0x10f86d30=>{
// 0x10f86d30 ldr x1, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f86d34 str x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86d38 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86d3c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86d40 b #0x10f879e0
super::chunk1::f_10f879e0(cpu,rt,0x10f879e0)?;return Ok(());
}
0x10f86d44=>{
// 0x10f86d44 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86d48 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86d4c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86ce0")),}}}
#[inline]
pub(super) unsafe fn f_10f86db0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86db0=>{
// 0x10f86db0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86db4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86db8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86dbc ldr x8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86dc0 cbz x8, #0x10f86df8
if (cpu.x[8]) == 0 {pc=0x10f86df8;continue;}pc=0x10f86dc4;continue;
}
0x10f86dc4=>{
// 0x10f86dc4 ldr x20, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f86dc8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86dcc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86dd0 bl #0x10f93d2c
cpu.x[30]=0x10f86dd4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f86dd4;continue;
}
0x10f86dd4=>{
// 0x10f86dd4 cbz x0, #0x10f86df8
if (cpu.x[0]) == 0 {pc=0x10f86df8;continue;}pc=0x10f86dd8;continue;
}
0x10f86dd8=>{
// 0x10f86dd8 ldr w8, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f86ddc tbnz w8, #0x1f, #0x10f86df8
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f86df8;continue;}pc=0x10f86de0;continue;
}
0x10f86de0=>{
// 0x10f86de0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86de4 bl #0x10f93d2c
cpu.x[30]=0x10f86de8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f86de8;continue;
}
0x10f86de8=>{
// 0x10f86de8 ldr w1, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f86dec ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86df0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86df4 b #0x10d39264
super::chunk4::f_10d39264(cpu,rt,0x10d39264)?;return Ok(());
}
0x10f86df8=>{
// 0x10f86df8 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f86dfc ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86e00 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86e04 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86db0")),}}}
#[inline]
pub(super) unsafe fn f_10f86f48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86f48=>{
// 0x10f86f48 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86f4c str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86f50 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86f54 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10f86f58 adrp x8, #0x102b3000
cpu.x[8]=(0x102b3000u64) as u64;
// 0x10f86f5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86f60 ldr q1, [x8, #0x660]
{let address=(cpu.x[8]).wrapping_add(0x660u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f86f64 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f86f68 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86f6c str wzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86f70 str xzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86f74 stur q1, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f86f78 stur d0, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86f7c str w8, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86f80 stp xzr, xzr, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86f84 add x0, x0, #0x48
{let a=((cpu.x[0]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f86f88 bl #0x1142f9f0
cpu.x[30]=0x10f86f8c;super::chunk4::f_1142f9f0(cpu,rt,0x1142f9f0)?;
pc=0x10f86f8c;continue;
}
0x10f86f8c=>{
// 0x10f86f8c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f86f90 stur xzr, [x19, #0x5d]
{let address=(cpu.x[19]).wrapping_add(0x5du64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86f94 str xzr, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86f98 strb w8, [x19, #0x65]
{let address=(cpu.x[19]).wrapping_add(0x65u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f86f9c stur wzr, [x19, #0x66]
{let address=(cpu.x[19]).wrapping_add(0x66u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86fa0 strb wzr, [x19, #0x6a]
{let address=(cpu.x[19]).wrapping_add(0x6au64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f86fa4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f86fa8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86fac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86f48")),}}}
#[inline]
pub(super) unsafe fn f_10f870ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f870ac=>{
// 0x10f870ac stp w1, w2, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f870b0 str w3, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f870b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f870ac")),}}}
#[inline]
pub(super) unsafe fn f_10f870d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f870d0=>{
// 0x10f870d0 str w1, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f870d4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f870d0")),}}}
#[inline]
pub(super) unsafe fn f_10f8861c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8861c=>{
// 0x10f8861c stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f88620 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88624 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f88628 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8862c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f88630 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88634 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f88638 ldr w1, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8863c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f88640 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f88644 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88648 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8864c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8864c;continue;
}
0x10f8864c=>{
// 0x10f8864c ldrsw x8, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f88650 cbz w8, #0x10f88698
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f88698;continue;}pc=0x10f88654;continue;
}
0x10f88654=>{
// 0x10f88654 ldr x22, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f88658 lsl x23, x8, #3
cpu.x[23]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
pc=0x10f8865c;continue;
}
0x10f8865c=>{
// 0x10f8865c ldr x21, [x22], #8
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;cpu.x[22]=((cpu.x[22]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f88660 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88664 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f88668 ldr w1, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8866c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88670 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f88674;super::dispatch(cpu,rt,target)?;}
pc=0x10f88674;continue;
}
0x10f88674=>{
// 0x10f88674 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f88678 bl #0x10f87918
cpu.x[30]=0x10f8867c;super::chunk5::f_10f87918(cpu,rt,0x10f87918)?;
pc=0x10f8867c;continue;
}
0x10f8867c=>{
// 0x10f8867c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88680 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f88684 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f88688 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8868c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f88690;super::dispatch(cpu,rt,target)?;}
pc=0x10f88690;continue;
}
0x10f88690=>{
// 0x10f88690 subs x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f88694 b.ne #0x10f8865c
if cpu.condition(1){pc=0x10f8865c;continue;}pc=0x10f88698;continue;
}
0x10f88698=>{
// 0x10f88698 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8869c ldr w1, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f886a0 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f886a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f886a8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f886ac ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f886b0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f886b4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f886b8 br x2
{let target=cpu.x[2];if target>=0x10f8861c && target<0x10f886bc{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8861c")),}}}
#[inline]
pub(super) unsafe fn f_10f88de8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f88de8=>{
// 0x10f88de8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f88dec stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f88df0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f88df4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f88df8 cbz x1, #0x10f88e8c
if (cpu.x[1]) == 0 {pc=0x10f88e8c;continue;}pc=0x10f88dfc;continue;
}
0x10f88dfc=>{
// 0x10f88dfc lsr x8, x1, #0x3d
cpu.x[8]=(((((cpu.x[1]))) as u64).wrapping_shr(((((0x3du64))) & 63) as u32) as u64) as u64;
// 0x10f88e00 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f88e04 cbnz x8, #0x10f88fbc
if (cpu.x[8]) != 0 {pc=0x10f88fbc;continue;}pc=0x10f88e08;continue;
}
0x10f88e08=>{
// 0x10f88e08 lsl x0, x20, #3
cpu.x[0]=(((((cpu.x[20]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f88e0c bl #0x1187d2b0
cpu.x[30]=0x10f88e10;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10f88e10;continue;
}
0x10f88e10=>{
// 0x10f88e10 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f88e14 str x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88e18 cbz x8, #0x10f88e24
if (cpu.x[8]) == 0 {pc=0x10f88e24;continue;}pc=0x10f88e1c;continue;
}
0x10f88e1c=>{
// 0x10f88e1c mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f88e20 bl #0x1187d2c0
cpu.x[30]=0x10f88e24;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f88e24;continue;
}
0x10f88e24=>{
// 0x10f88e24 mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10f88e28 str x20, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f88e2c;continue;
}
0x10f88e2c=>{
// 0x10f88e2c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f88e30 str xzr, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88e34 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f88e38 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88e3c b.ne #0x10f88e2c
if cpu.condition(1){pc=0x10f88e2c;continue;}pc=0x10f88e40;continue;
}
0x10f88e40=>{
// 0x10f88e40 mov x10, x19
cpu.x[10]=(((cpu.x[19]))) as u64;
// 0x10f88e44 ldr x8, [x10, #0x10]!
{let address=(cpu.x[10]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;cpu.x[10]=(address) as u64;}
// 0x10f88e48 cbz x8, #0x10f88e80
if (cpu.x[8]) == 0 {pc=0x10f88e80;continue;}pc=0x10f88e4c;continue;
}
0x10f88e4c=>{
// 0x10f88e4c fmov d0, x20
cpu.q[0]=(cpu.x[20]) as u128 & 0xffffffffffffffffu128;
// 0x10f88e50 ldr x9, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f88e54 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10f88e58 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10f88e5c fmov w11, s0
cpu.x[11]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10f88e60 cmp x11, #2
{let a=((cpu.x[11]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88e64 b.hs #0x10f88eac
if cpu.condition(2){pc=0x10f88eac;continue;}pc=0x10f88e68;continue;
}
0x10f88e68=>{
// 0x10f88e68 sub x11, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u64;}
// 0x10f88e6c ldr x12, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88e70 and x9, x9, x11
{let a=((cpu.x[9]));let b=((cpu.x[11]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10f88e74 str x10, [x12, x9, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88e78 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f88e7c cbnz x10, #0x10f88ed4
if (cpu.x[10]) != 0 {pc=0x10f88ed4;continue;}pc=0x10f88e80;continue;
}
0x10f88e80=>{
// 0x10f88e80 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f88e84 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f88e88 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f88e8c=>{
// 0x10f88e8c ldr x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f88e90 str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88e94 cbz x0, #0x10f88e9c
if (cpu.x[0]) == 0 {pc=0x10f88e9c;continue;}pc=0x10f88e98;continue;
}
0x10f88e98=>{
// 0x10f88e98 bl #0x1187d2c0
cpu.x[30]=0x10f88e9c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f88e9c;continue;
}
0x10f88e9c=>{
// 0x10f88e9c str xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88ea0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f88ea4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f88ea8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f88eac=>{
// 0x10f88eac cmp x9, x20
{let a=((cpu.x[9]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88eb0 b.lo #0x10f88ebc
if cpu.condition(3){pc=0x10f88ebc;continue;}pc=0x10f88eb4;continue;
}
0x10f88eb4=>{
// 0x10f88eb4 udiv x12, x9, x20
{let a=((cpu.x[9])) as u64;let b=((cpu.x[20])) as u64;cpu.x[12]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f88eb8 msub x9, x12, x20, x9
cpu.x[9]=((((cpu.x[9]))).wrapping_sub((((cpu.x[12]))).wrapping_mul(((cpu.x[20]))))) as u64;
pc=0x10f88ebc;continue;
}
0x10f88ebc=>{
// 0x10f88ebc ldr x12, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88ec0 str x10, [x12, x9, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88ec4 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f88ec8 cbz x10, #0x10f88e80
if (cpu.x[10]) == 0 {pc=0x10f88e80;continue;}pc=0x10f88ecc;continue;
}
0x10f88ecc=>{
// 0x10f88ecc cmp x11, #2
{let a=((cpu.x[11]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88ed0 b.hs #0x10f88f7c
if cpu.condition(2){pc=0x10f88f7c;continue;}pc=0x10f88ed4;continue;
}
0x10f88ed4=>{
// 0x10f88ed4 sub x11, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u64;}
// 0x10f88ed8 b #0x10f88ee8
pc=0x10f88ee8;continue;
}
0x10f88edc=>{
// 0x10f88edc mov x8, x10
cpu.x[8]=(((cpu.x[10]))) as u64;
pc=0x10f88ee0;continue;
}
0x10f88ee0=>{
// 0x10f88ee0 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f88ee4 cbz x10, #0x10f88e80
if (cpu.x[10]) == 0 {pc=0x10f88e80;continue;}pc=0x10f88ee8;continue;
}
0x10f88ee8=>{
// 0x10f88ee8 ldr x12, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88eec and x12, x12, x11
{let a=((cpu.x[12]));let b=((cpu.x[11]));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10f88ef0 cmp x12, x9
{let a=((cpu.x[12]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88ef4 b.eq #0x10f88edc
if cpu.condition(0){pc=0x10f88edc;continue;}pc=0x10f88ef8;continue;
}
0x10f88ef8=>{
// 0x10f88ef8 ldr x13, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88efc ldr x14, [x13, x12, lsl #3]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[12])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[14]=(v0) as u64;}
// 0x10f88f00 cbz x14, #0x10f88f30
if (cpu.x[14]) == 0 {pc=0x10f88f30;continue;}pc=0x10f88f04;continue;
}
0x10f88f04=>{
// 0x10f88f04 ldr x13, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88f08 lsl x12, x12, #3
cpu.x[12]=(((((cpu.x[12]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f88f0c str x13, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[13];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f10 ldr x13, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88f14 ldr x13, [x13, x12]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88f18 ldr x13, [x13]
{let address=(cpu.x[13]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88f1c str x13, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=cpu.x[13];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f20 ldr x13, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88f24 ldr x12, [x13, x12]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f28 str x10, [x12]
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f2c b #0x10f88ee0
pc=0x10f88ee0;continue;
}
0x10f88f30=>{
// 0x10f88f30 str x8, [x13, x12, lsl #3]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[12])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f34 mov x8, x10
cpu.x[8]=(((cpu.x[10]))) as u64;
// 0x10f88f38 mov x9, x12
cpu.x[9]=(((cpu.x[12]))) as u64;
// 0x10f88f3c b #0x10f88ee0
pc=0x10f88ee0;continue;
}
0x10f88f40=>{
// 0x10f88f40 ldr x12, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f44 lsl x11, x11, #3
cpu.x[11]=(((((cpu.x[11]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f88f48 str x12, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f4c ldr x12, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f50 ldr x12, [x12, x11]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f54 ldr x12, [x12]
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f58 str x12, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f5c ldr x12, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f60 ldr x11, [x12, x11]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f88f64 str x10, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88f68 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
pc=0x10f88f6c;continue;
}
0x10f88f6c=>{
// 0x10f88f6c mov x11, x9
cpu.x[11]=(((cpu.x[9]))) as u64;
// 0x10f88f70 mov x8, x10
cpu.x[8]=(((cpu.x[10]))) as u64;
// 0x10f88f74 ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f88f78 cbz x10, #0x10f88e80
if (cpu.x[10]) == 0 {pc=0x10f88e80;continue;}pc=0x10f88f7c;continue;
}
0x10f88f7c=>{
// 0x10f88f7c ldr x11, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f88f80 cmp x11, x20
{let a=((cpu.x[11]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88f84 b.lo #0x10f88f90
if cpu.condition(3){pc=0x10f88f90;continue;}pc=0x10f88f88;continue;
}
0x10f88f88=>{
// 0x10f88f88 udiv x12, x11, x20
{let a=((cpu.x[11])) as u64;let b=((cpu.x[20])) as u64;cpu.x[12]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10f88f8c msub x11, x12, x20, x11
cpu.x[11]=((((cpu.x[11]))).wrapping_sub((((cpu.x[12]))).wrapping_mul(((cpu.x[20]))))) as u64;
pc=0x10f88f90;continue;
}
0x10f88f90=>{
// 0x10f88f90 cmp x11, x9
{let a=((cpu.x[11]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f88f94 b.eq #0x10f88f6c
if cpu.condition(0){pc=0x10f88f6c;continue;}pc=0x10f88f98;continue;
}
0x10f88f98=>{
// 0x10f88f98 ldr x12, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f88f9c ldr x13, [x12, x11, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[11])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f88fa0 cbnz x13, #0x10f88f40
if (cpu.x[13]) != 0 {pc=0x10f88f40;continue;}pc=0x10f88fa4;continue;
}
0x10f88fa4=>{
// 0x10f88fa4 str x8, [x12, x11, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[11])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f88fa8 mov x8, x10
cpu.x[8]=(((cpu.x[10]))) as u64;
// 0x10f88fac ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f88fb0 mov x9, x11
cpu.x[9]=(((cpu.x[11]))) as u64;
// 0x10f88fb4 cbnz x10, #0x10f88f7c
if (cpu.x[10]) != 0 {pc=0x10f88f7c;continue;}pc=0x10f88fb8;continue;
}
0x10f88fb8=>{
// 0x10f88fb8 b #0x10f88e80
pc=0x10f88e80;continue;
}
0x10f88fbc=>{
// 0x10f88fbc bl #0x10778a60
cpu.x[30]=0x10f88fc0;super::dispatch(cpu,rt,0x10778a60)?;
return Err("fell through function 0x10f88de8".into());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f88de8")),}}}
#[inline]
pub(super) unsafe fn f_10f89524(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89524=>{
// 0x10f89524 ldr x0, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89528 cbz x0, #0x10f89530
if (cpu.x[0]) == 0 {pc=0x10f89530;continue;}pc=0x10f8952c;continue;
}
0x10f8952c=>{
// 0x10f8952c b #0x10f7909c
super::chunk4::f_10f7909c(cpu,rt,0x10f7909c)?;return Ok(());
}
0x10f89530=>{
// 0x10f89530 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89524")),}}}
#[inline]
pub(super) unsafe fn f_10f89534(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89534=>{
// 0x10f89534 ldr x0, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89538 cbz x0, #0x10f89540
if (cpu.x[0]) == 0 {pc=0x10f89540;continue;}pc=0x10f8953c;continue;
}
0x10f8953c=>{
// 0x10f8953c b #0x10f78b9c
super::chunk6::f_10f78b9c(cpu,rt,0x10f78b9c)?;return Ok(());
}
0x10f89540=>{
// 0x10f89540 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89534")),}}}
#[inline]
pub(super) unsafe fn f_10f8bfe0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8bfe0=>{
// 0x10f8bfe0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8bfe4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8bfe8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8bfec mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8bff0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f8bff4 mov w1, w2
cpu.x[1]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8bff8 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8bffc bl #0x111dd770
cpu.x[30]=0x10f8c000;super::chunk2::f_111dd770(cpu,rt,0x111dd770)?;
pc=0x10f8c000;continue;
}
0x10f8c000=>{
// 0x10f8c000 cbz w0, #0x10f8c098
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10f8c098;continue;}pc=0x10f8c004;continue;
}
0x10f8c004=>{
// 0x10f8c004 ldr w10, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8c008 cbz w10, #0x10f8c098
if (cpu.x[10] as u32 as u64) == 0 {pc=0x10f8c098;continue;}pc=0x10f8c00c;continue;
}
0x10f8c00c=>{
// 0x10f8c00c ldr x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c010 cmp w10, #2
{let a=((cpu.x[10] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c014 b.lt #0x10f8c054
if cpu.condition(11){pc=0x10f8c054;continue;}pc=0x10f8c018;continue;
}
0x10f8c018=>{
// 0x10f8c018 mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10f8c01c sub w10, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f8c020 b #0x10f8c030
pc=0x10f8c030;continue;
}
0x10f8c024=>{
// 0x10f8c024 cmp w11, w9
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c028 mov w10, w11
cpu.x[10]=(((cpu.x[11] as u32 as u64))) as u32 as u64;
// 0x10f8c02c b.le #0x10f8c068
if cpu.condition(13){pc=0x10f8c068;continue;}pc=0x10f8c030;continue;
}
0x10f8c030=>{
// 0x10f8c030 add w11, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f8c034 asr w11, w11, #1
cpu.x[11]=(((((cpu.x[11] as u32 as u64))) as i32).wrapping_shr(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f8c038 ldr x12, [x8, w11, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[11] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f8c03c ldr w12, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10f8c040 cmp w12, w0
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c044 b.ge #0x10f8c024
if cpu.condition(10){pc=0x10f8c024;continue;}pc=0x10f8c048;continue;
}
0x10f8c048=>{
// 0x10f8c048 add w9, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f8c04c mov w11, w10
cpu.x[11]=(((cpu.x[10] as u32 as u64))) as u32 as u64;
// 0x10f8c050 b #0x10f8c024
pc=0x10f8c024;continue;
}
0x10f8c054=>{
// 0x10f8c054 ldr x20, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8c058 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8c05c cmp w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c060 b.eq #0x10f8c080
if cpu.condition(0){pc=0x10f8c080;continue;}pc=0x10f8c064;continue;
}
0x10f8c064=>{
// 0x10f8c064 b #0x10f8c098
pc=0x10f8c098;continue;
}
0x10f8c068=>{
// 0x10f8c068 cmn w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f8c06c b.eq #0x10f8c098
if cpu.condition(0){pc=0x10f8c098;continue;}pc=0x10f8c070;continue;
}
0x10f8c070=>{
// 0x10f8c070 ldr x20, [x8, w9, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[9] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8c074 ldr w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8c078 cmp w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c07c b.ne #0x10f8c098
if cpu.condition(1){pc=0x10f8c098;continue;}pc=0x10f8c080;continue;
}
0x10f8c080=>{
// 0x10f8c080 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c084 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8c088 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c08c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8c090;super::dispatch(cpu,rt,target)?;}
pc=0x10f8c090;continue;
}
0x10f8c090=>{
// 0x10f8c090 cmp w0, w19
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c094 b.eq #0x10f8c09c
if cpu.condition(0){pc=0x10f8c09c;continue;}pc=0x10f8c098;continue;
}
0x10f8c098=>{
// 0x10f8c098 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
pc=0x10f8c09c;continue;
}
0x10f8c09c=>{
// 0x10f8c09c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8c0a0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8c0a4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8c0a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8bfe0")),}}}
#[inline]
pub(super) unsafe fn f_10f8dc48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8dc48=>{
// 0x10f8dc48 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8dc4c stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8dc50 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8dc54 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8dc58 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f8dc5c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8dc60 bl #0x10f93bd8
cpu.x[30]=0x10f8dc64;super::chunk14::f_10f93bd8(cpu,rt,0x10f93bd8)?;
pc=0x10f8dc64;continue;
}
0x10f8dc64=>{
// 0x10f8dc64 ldr x21, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8dc68 add x1, x20, #0x120
{let a=((cpu.x[20]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8dc6c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dc70 bl #0x111dd2a0
cpu.x[30]=0x10f8dc74;super::chunk12::f_111dd2a0(cpu,rt,0x111dd2a0)?;
pc=0x10f8dc74;continue;
}
0x10f8dc74=>{
// 0x10f8dc74 add x1, x20, #0x12c
{let a=((cpu.x[20]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8dc78 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dc7c bl #0x111dd2a0
cpu.x[30]=0x10f8dc80;super::chunk12::f_111dd2a0(cpu,rt,0x111dd2a0)?;
pc=0x10f8dc80;continue;
}
0x10f8dc80=>{
// 0x10f8dc80 ldr x2, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8dc84 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8dc88 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8dc8c bl #0x10f8c0ac
cpu.x[30]=0x10f8dc90;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f8dc90;continue;
}
0x10f8dc90=>{
// 0x10f8dc90 ldr x2, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8dc94 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8dc98 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8dc9c bl #0x10f8c0ac
cpu.x[30]=0x10f8dca0;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f8dca0;continue;
}
0x10f8dca0=>{
// 0x10f8dca0 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8dca4 ldr x22, [x20, #0x190]
{let address=(cpu.x[20]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f8dca8 bl #0x10e0b6b4
cpu.x[30]=0x10f8dcac;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10f8dcac;continue;
}
0x10f8dcac=>{
// 0x10f8dcac ldr x3, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10f8dcb0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dcb4 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f8dcb8 mov w2, #0xb
cpu.x[2]=(((0xbu64))) as u32 as u64;
// 0x10f8dcbc bl #0x111dd77c
cpu.x[30]=0x10f8dcc0;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10f8dcc0;continue;
}
0x10f8dcc0=>{
// 0x10f8dcc0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dcc4 ldr w1, [x20, #0x198]
{let address=(cpu.x[20]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dcc8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dccc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dcd0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dcd4;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dcd4;continue;
}
0x10f8dcd4=>{
// 0x10f8dcd4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dcd8 ldr w1, [x20, #0x128]
{let address=(cpu.x[20]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dcdc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dce0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dce4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dce8;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dce8;continue;
}
0x10f8dce8=>{
// 0x10f8dce8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dcec ldr w1, [x20, #0x134]
{let address=(cpu.x[20]).wrapping_add(0x134u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dcf0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dcf4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dcf8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dcfc;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dcfc;continue;
}
0x10f8dcfc=>{
// 0x10f8dcfc ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd00 ldr w8, [x8, #0x1c0]
{let address=(cpu.x[8]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8dd04 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8dd08 b.lt #0x10f8dd48
if cpu.condition(11){pc=0x10f8dd48;continue;}pc=0x10f8dd0c;continue;
}
0x10f8dd0c=>{
// 0x10f8dd0c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd10 ldr w1, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dd14 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd18 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dd20;super::dispatch(cpu,rt,target)?;}
// 0x10f8dd20 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd24 ldr w1, [x20, #0x13c]
{let address=(cpu.x[20]).wrapping_add(0x13cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dd28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd2c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dd34;super::dispatch(cpu,rt,target)?;}
// 0x10f8dd34 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd38 ldr w1, [x20, #0x140]
{let address=(cpu.x[20]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dd3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd40 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd44 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dd48;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dd48;continue;
}
0x10f8dd48=>{
// 0x10f8dd48 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd4c ldr w1, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dd50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd54 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd58 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dd5c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dd5c;continue;
}
0x10f8dd5c=>{
// 0x10f8dd5c ldr w8, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8dd60 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8dd64 b.lt #0x10f8dd88
if cpu.condition(11){pc=0x10f8dd88;continue;}pc=0x10f8dd68;continue;
}
0x10f8dd68=>{
// 0x10f8dd68 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8dd6c ldr x22, [x20, #0x1a8]
{let address=(cpu.x[20]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f8dd70 bl #0x10e0b6b4
cpu.x[30]=0x10f8dd74;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f8dd74 ldr x3, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10f8dd78 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd7c mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f8dd80 mov w2, #0xb
cpu.x[2]=(((0xbu64))) as u32 as u64;
// 0x10f8dd84 bl #0x111dd77c
cpu.x[30]=0x10f8dd88;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10f8dd88;continue;
}
0x10f8dd88=>{
// 0x10f8dd88 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd8c ldrb w1, [x20, #0x1dd]
{let address=(cpu.x[20]).wrapping_add(0x1ddu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dd90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dd94 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dd98 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dd9c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dd9c;continue;
}
0x10f8dd9c=>{
// 0x10f8dd9c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dda0 ldrb w1, [x20, #0x188]
{let address=(cpu.x[20]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dda4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dda8 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ddb0;super::dispatch(cpu,rt,target)?;}
pc=0x10f8ddb0;continue;
}
0x10f8ddb0=>{
// 0x10f8ddb0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddb4 ldrb w1, [x20, #0x189]
{let address=(cpu.x[20]).wrapping_add(0x189u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8ddb8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ddbc ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddc0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ddc4;super::dispatch(cpu,rt,target)?;}
pc=0x10f8ddc4;continue;
}
0x10f8ddc4=>{
// 0x10f8ddc4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddc8 ldrb w1, [x20, #0x170]
{let address=(cpu.x[20]).wrapping_add(0x170u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8ddcc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ddd0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddd4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ddd8;super::dispatch(cpu,rt,target)?;}
pc=0x10f8ddd8;continue;
}
0x10f8ddd8=>{
// 0x10f8ddd8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dddc ldrb w1, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dde0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dde4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dde8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8ddec;super::dispatch(cpu,rt,target)?;}
pc=0x10f8ddec;continue;
}
0x10f8ddec=>{
// 0x10f8ddec ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddf0 ldrb w1, [x20, #0x14c]
{let address=(cpu.x[20]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8ddf4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ddf8 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ddfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8de00;super::dispatch(cpu,rt,target)?;}
pc=0x10f8de00;continue;
}
0x10f8de00=>{
// 0x10f8de00 ldr x8, [x20, #0x1d0]
{let address=(cpu.x[20]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de04 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8de08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8de0c cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8de10 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de14 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f8de18 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8de1c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8de1c;continue;
}
0x10f8de1c=>{
// 0x10f8de1c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de20 ldr w1, [x20, #0x144]
{let address=(cpu.x[20]).wrapping_add(0x144u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8de24 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8de28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8de2c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8de34;super::dispatch(cpu,rt,target)?;}
pc=0x10f8de34;continue;
}
0x10f8de34=>{
// 0x10f8de34 ldr x2, [x20, #0x118]
{let address=(cpu.x[20]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8de38 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8de3c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8de40 bl #0x10f8c0ac
cpu.x[30]=0x10f8de44;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f8de44;continue;
}
0x10f8de44=>{
// 0x10f8de44 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8de48 ldr w8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8de4c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8de50 b.lt #0x10f8de6c
if cpu.condition(11){pc=0x10f8de6c;continue;}pc=0x10f8de54;continue;
}
0x10f8de54=>{
// 0x10f8de54 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de58 ldr w1, [x20, #0x1b8]
{let address=(cpu.x[20]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8de5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8de60 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8de68;super::dispatch(cpu,rt,target)?;}
// 0x10f8de68 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f8de6c;continue;
}
0x10f8de6c=>{
// 0x10f8de6c ldr w8, [x0, #0x200]
{let address=(cpu.x[0]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8de70 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8de74 b.gt #0x10f8de90
if cpu.condition(12){pc=0x10f8de90;continue;}pc=0x10f8de78;continue;
}
0x10f8de78=>{
// 0x10f8de78 ldr w8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8de7c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8de80 b.gt #0x10f8de90
if cpu.condition(12){pc=0x10f8de90;continue;}pc=0x10f8de84;continue;
}
0x10f8de84=>{
// 0x10f8de84 ldr w8, [x0, #0x204]
{let address=(cpu.x[0]).wrapping_add(0x204u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8de88 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8de8c b.lt #0x10f8dea8
if cpu.condition(11){pc=0x10f8dea8;continue;}pc=0x10f8de90;continue;
}
0x10f8de90=>{
// 0x10f8de90 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8de94 ldr w1, [x20, #0x1b0]
{let address=(cpu.x[20]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8de98 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8de9c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dea0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dea4;super::dispatch(cpu,rt,target)?;}
// 0x10f8dea4 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f8dea8;continue;
}
0x10f8dea8=>{
// 0x10f8dea8 bl #0x10e5ae3c
cpu.x[30]=0x10f8deac;super::chunk11::f_10e5ae3c(cpu,rt,0x10e5ae3c)?;
pc=0x10f8deac;continue;
}
0x10f8deac=>{
// 0x10f8deac cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8deb0 b.lt #0x10f8ded8
if cpu.condition(11){pc=0x10f8ded8;continue;}pc=0x10f8deb4;continue;
}
0x10f8deb4=>{
// 0x10f8deb4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8deb8 ldr w1, [x20, #0x1bc]
{let address=(cpu.x[20]).wrapping_add(0x1bcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8debc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dec0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dec4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dec8;super::dispatch(cpu,rt,target)?;}
// 0x10f8dec8 ldr x2, [x20, #0x110]
{let address=(cpu.x[20]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8decc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8ded0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8ded4 bl #0x10f8c0ac
cpu.x[30]=0x10f8ded8;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f8ded8;continue;
}
0x10f8ded8=>{
// 0x10f8ded8 ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dedc ldr w8, [x8, #0x1dc]
{let address=(cpu.x[8]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8dee0 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8dee4 b.ne #0x10f8defc
if cpu.condition(1){pc=0x10f8defc;continue;}pc=0x10f8dee8;continue;
}
0x10f8dee8=>{
// 0x10f8dee8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8deec ldr w1, [x20, #0x1c0]
{let address=(cpu.x[20]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8def0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8def4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8def8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8defc;super::dispatch(cpu,rt,target)?;}
pc=0x10f8defc;continue;
}
0x10f8defc=>{
// 0x10f8defc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df00 ldr w1, [x20, #0x1b4]
{let address=(cpu.x[20]).wrapping_add(0x1b4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8df04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df08 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df0c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8df10;super::dispatch(cpu,rt,target)?;}
pc=0x10f8df10;continue;
}
0x10f8df10=>{
// 0x10f8df10 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df14 ldr w1, [x20, #0x1c4]
{let address=(cpu.x[20]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8df18 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df1c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8df24;super::dispatch(cpu,rt,target)?;}
pc=0x10f8df24;continue;
}
0x10f8df24=>{
// 0x10f8df24 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df28 ldr w1, [x20, #0x1c8]
{let address=(cpu.x[20]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8df2c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df30 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df34 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8df38;super::dispatch(cpu,rt,target)?;}
pc=0x10f8df38;continue;
}
0x10f8df38=>{
// 0x10f8df38 tbz w22, #0, #0x10f8df48
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8df48;continue;}pc=0x10f8df3c;continue;
}
0x10f8df3c=>{
// 0x10f8df3c ldr x1, [x20, #0x1d0]
{let address=(cpu.x[20]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f8df40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df44 bl #0x111dd6a4
cpu.x[30]=0x10f8df48;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10f8df48;continue;
}
0x10f8df48=>{
// 0x10f8df48 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df4c ldr w1, [x20, #0x1d8]
{let address=(cpu.x[20]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8df50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df54 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df58 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8df5c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8df5c;continue;
}
0x10f8df5c=>{
// 0x10f8df5c ldrb w8, [x20, #0x14c]
{let address=(cpu.x[20]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8df60 cbz w8, #0x10f8dfa4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8dfa4;continue;}pc=0x10f8df64;continue;
}
0x10f8df64=>{
// 0x10f8df64 ldr x2, [x20, #0x150]
{let address=(cpu.x[20]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8df68 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8df6c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8df70 bl #0x10f8c0ac
cpu.x[30]=0x10f8df74;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
// 0x10f8df74 ldr x2, [x20, #0x158]
{let address=(cpu.x[20]).wrapping_add(0x158u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8df78 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8df7c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8df80 bl #0x10f8c0ac
cpu.x[30]=0x10f8df84;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
// 0x10f8df84 add x1, x20, #0x160
{let a=((cpu.x[20]));let b=((0x160u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8df88 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8df8c bl #0x111dd2a0
cpu.x[30]=0x10f8df90;super::chunk12::f_111dd2a0(cpu,rt,0x111dd2a0)?;
// 0x10f8df90 ldr x8, [x20, #0x150]
{let address=(cpu.x[20]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8df94 cbnz x8, #0x10f8dfa4
if (cpu.x[8]) != 0 {pc=0x10f8dfa4;continue;}pc=0x10f8df98;continue;
}
0x10f8df98=>{
// 0x10f8df98 ldr x0, [x20, #0x168]
{let address=(cpu.x[20]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8df9c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8dfa0 bl #0x10f93790
cpu.x[30]=0x10f8dfa4;super::dispatch(cpu,rt,0x10f93790)?;
pc=0x10f8dfa4;continue;
}
0x10f8dfa4=>{
// 0x10f8dfa4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfa8 ldr w1, [x20, #0x200]
{let address=(cpu.x[20]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dfac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dfb0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfb4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dfb8;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dfb8;continue;
}
0x10f8dfb8=>{
// 0x10f8dfb8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfbc ldr w1, [x20, #0xf8]
{let address=(cpu.x[20]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dfc0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dfc4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfc8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dfcc;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dfcc;continue;
}
0x10f8dfcc=>{
// 0x10f8dfcc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfd0 ldr w1, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dfd4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dfd8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dfdc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8dfe0;super::dispatch(cpu,rt,target)?;}
pc=0x10f8dfe0;continue;
}
0x10f8dfe0=>{
// 0x10f8dfe0 ldr w8, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8dfe4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8dfe8 b.lt #0x10f8e010
if cpu.condition(11){pc=0x10f8e010;continue;}pc=0x10f8dfec;continue;
}
0x10f8dfec=>{
// 0x10f8dfec mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10f8dff0;continue;
}
0x10f8dff0=>{
// 0x10f8dff0 ldr x8, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8dff4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8dff8 ldr w1, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8dffc bl #0x111dd758
cpu.x[30]=0x10f8e000;super::chunk13::f_111dd758(cpu,rt,0x111dd758)?;
// 0x10f8e000 ldrsw x8, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f8e004 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f8e008 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8e00c b.lt #0x10f8dff0
if cpu.condition(11){pc=0x10f8dff0;continue;}pc=0x10f8e010;continue;
}
0x10f8e010=>{
// 0x10f8e010 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8e014 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8e018 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f8e01c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8dc48")),}}}
#[inline]
pub(super) unsafe fn f_10f8f03c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8f03c=>{
// 0x10f8f03c sub sp, sp, #0x130
{let a=((cpu.sp));let b=((0x130u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f8f040 stp x29, x30, [sp, #0xd0]
{let address=(cpu.sp).wrapping_add(0xd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f044 stp x28, x27, [sp, #0xe0]
{let address=(cpu.sp).wrapping_add(0xe0u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f048 stp x26, x25, [sp, #0xf0]
{let address=(cpu.sp).wrapping_add(0xf0u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f04c stp x24, x23, [sp, #0x100]
{let address=(cpu.sp).wrapping_add(0x100u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f050 stp x22, x21, [sp, #0x110]
{let address=(cpu.sp).wrapping_add(0x110u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f054 stp x20, x19, [sp, #0x120]
{let address=(cpu.sp).wrapping_add(0x120u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f058 add x29, sp, #0xd0
{let a=((cpu.sp));let b=((0xd0u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f8f05c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8f060 ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f064 ldrb w25, [x29, #0x80]
{let address=(cpu.x[29]).wrapping_add(0x80u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[25]=(v0) as u32 as u64;}
// 0x10f8f068 ldr w20, [x29, #0x78]
{let address=(cpu.x[29]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10f8f06c ldr w27, [x29, #0x70]
{let address=(cpu.x[29]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10f8f070 mov w23, w7
cpu.x[23]=(((cpu.x[7] as u32 as u64))) as u32 as u64;
// 0x10f8f074 ldr w21, [x29, #0x68]
{let address=(cpu.x[29]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f8f078 ldr w28, [x29, #0x60]
{let address=(cpu.x[29]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10f8f07c mov w26, w5
cpu.x[26]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10f8f080 stp w6, w3, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[6] as u32 as u64;let v1=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f8f084 mov w22, w2
cpu.x[22]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8f088 mov w24, w1
cpu.x[24]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f8f08c stur w4, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[4] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f090 bl #0x10e5a908
cpu.x[30]=0x10f8f094;super::chunk5::f_10e5a908(cpu,rt,0x10e5a908)?;
pc=0x10f8f094;continue;
}
0x10f8f094=>{
// 0x10f8f094 tbz w0, #0, #0x10f8f0d4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f0d4;continue;}pc=0x10f8f098;continue;
}
0x10f8f098=>{
// 0x10f8f098 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f09c cbz w8, #0x10f8f1c0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8f1c0;continue;}pc=0x10f8f0a0;continue;
}
0x10f8f0a0=>{
// 0x10f8f0a0 ldr w1, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8f0a4 ldr w2, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f8f0a8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8f0ac ldr w3, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f8f0b0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f8f0b4 ldp x20, x19, [sp, #0x120]
{let address=(cpu.sp).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f0b8 ldp x22, x21, [sp, #0x110]
{let address=(cpu.sp).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f0bc ldp x24, x23, [sp, #0x100]
{let address=(cpu.sp).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f8f0c0 ldp x26, x25, [sp, #0xf0]
{let address=(cpu.sp).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f8f0c4 ldp x28, x27, [sp, #0xe0]
{let address=(cpu.sp).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f8f0c8 ldp x29, x30, [sp, #0xd0]
{let address=(cpu.sp).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f0cc add sp, sp, #0x130
{let a=((cpu.sp));let b=((0x130u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f0d0 b #0x10f8f2d0
super::chunk2::f_10f8f2d0(cpu,rt,0x10f8f2d0)?;return Ok(());
}
0x10f8f0d4=>{
// 0x10f8f0d4 cmp w23, #0
{let a=((cpu.x[23] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f0d8 cset w8, gt
cpu.x[8]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10f8f0dc cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f0e0 b.gt #0x10f8f16c
if cpu.condition(12){pc=0x10f8f16c;continue;}pc=0x10f8f0e4;continue;
}
0x10f8f0e4=>{
// 0x10f8f0e4 and w8, w8, w25
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f8f0e8 cbnz w8, #0x10f8f16c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f8f16c;continue;}pc=0x10f8f0ec;continue;
}
0x10f8f0ec=>{
// 0x10f8f0ec cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f0f0 b.lt #0x10f8f1d0
if cpu.condition(11){pc=0x10f8f1d0;continue;}pc=0x10f8f0f4;continue;
}
0x10f8f0f4=>{
// 0x10f8f0f4 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f0f8 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f0fc mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10f8f100 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f8f104 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f8f108 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f8f10c ldrb w9, [x8, #0x185]
{let address=(cpu.x[8]).wrapping_add(0x185u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f110 ldrb w8, [x8, #0x186]
{let address=(cpu.x[8]).wrapping_add(0x186u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f114 str w20, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f118 str w27, [sp, #0x48]
{let address=(cpu.sp).wrapping_add(0x48u64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f11c mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f8f120 strb w8, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f124 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f8f128 str xzr, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f12c str wzr, [sp, #0x88]
{let address=(cpu.sp).wrapping_add(0x88u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f130 str wzr, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f134 str wzr, [sp, #0x78]
{let address=(cpu.sp).wrapping_add(0x78u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f138 str xzr, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f13c str wzr, [sp, #0x68]
{let address=(cpu.sp).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f140 strb wzr, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f144 str wzr, [sp, #0x58]
{let address=(cpu.sp).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f148 str wzr, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f14c strb w9, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f150 strb w8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f154 str w28, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f158 str w22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f15c str w24, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f160 str w23, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f164 ldp w3, w4, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[3]=(v0) as u32 as u64;cpu.x[4]=(v1) as u32 as u64;}
// 0x10f8f168 b #0x10f8f278
pc=0x10f8f278;continue;
}
0x10f8f16c=>{
// 0x10f8f16c ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f170 ldr x9, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8f174 stp w20, w28, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=cpu.x[20] as u32 as u64;let v1=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f8f178 stur w21, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f17c stur x9, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f180 cbz w8, #0x10f8f1d4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8f1d4;continue;}pc=0x10f8f184;continue;
}
0x10f8f184=>{
// 0x10f8f184 mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f8f188;continue;
}
0x10f8f188=>{
// 0x10f8f188 mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f8f18c ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8f190 cbnz x9, #0x10f8f188
if (cpu.x[9]) != 0 {pc=0x10f8f188;continue;}pc=0x10f8f194;continue;
}
0x10f8f194=>{
// 0x10f8f194 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f198 cbz x8, #0x10f8f1d4
if (cpu.x[8]) == 0 {pc=0x10f8f1d4;continue;}pc=0x10f8f19c;continue;
}
0x10f8f19c=>{
// 0x10f8f19c ldr w9, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f1a0 add x10, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f8f1a4 cbz w9, #0x10f8f2a0
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f8f2a0;continue;}pc=0x10f8f1a8;continue;
}
0x10f8f1a8=>{
// 0x10f8f1a8 mov w25, wzr
cpu.x[25]=(((0u64))) as u32 as u64;
// 0x10f8f1ac stur wzr, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f1b0 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10f8f1b4 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
pc=0x10f8f1b8;continue;
}
0x10f8f1b8=>{
// 0x10f8f1b8 mov x28, x10
cpu.x[28]=(((cpu.x[10]))) as u64;
// 0x10f8f1bc b #0x10f8f1e8
pc=0x10f8f1e8;continue;
}
0x10f8f1c0=>{
// 0x10f8f1c0 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f1c4 ldr w8, [x8, #0x1c0]
{let address=(cpu.x[8]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f1c8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f1cc b.le #0x10f8f0a0
if cpu.condition(13){pc=0x10f8f0a0;continue;}pc=0x10f8f1d0;continue;
}
0x10f8f1d0=>{
// 0x10f8f1d0 b #0x10f8f280
pc=0x10f8f280;continue;
}
0x10f8f1d4=>{
// 0x10f8f1d4 mov w25, wzr
cpu.x[25]=(((0u64))) as u32 as u64;
// 0x10f8f1d8 stur wzr, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f1dc mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10f8f1e0 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10f8f1e4 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
pc=0x10f8f1e8;continue;
}
0x10f8f1e8=>{
// 0x10f8f1e8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f1ec bl #0x10e5aec8
cpu.x[30]=0x10f8f1f0;super::chunk11::f_10e5aec8(cpu,rt,0x10e5aec8)?;
pc=0x10f8f1f0;continue;
}
0x10f8f1f0=>{
// 0x10f8f1f0 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f1f4 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f8f1f8 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f8f1fc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f8f200 mov w5, w26
cpu.x[5]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f8f204 ldrb w9, [x8, #0x185]
{let address=(cpu.x[8]).wrapping_add(0x185u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f208 ldrb w8, [x8, #0x186]
{let address=(cpu.x[8]).wrapping_add(0x186u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f20c str w25, [sp, #0x88]
{let address=(cpu.sp).wrapping_add(0x88u64);let v0=cpu.x[25] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f210 str xzr, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f214 str w20, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f218 str w21, [sp, #0x78]
{let address=(cpu.sp).wrapping_add(0x78u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f21c str x28, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[28];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f220 ldur w10, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8f224 strb wzr, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f228 str wzr, [sp, #0x58]
{let address=(cpu.sp).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f22c str w10, [sp, #0x68]
{let address=(cpu.sp).wrapping_add(0x68u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f230 ldur w10, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8f234 str w27, [sp, #0x48]
{let address=(cpu.sp).wrapping_add(0x48u64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f238 str w10, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f23c ldur w10, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8f240 strb w8, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f244 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f8f248 bic w8, w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f8f24c strb w9, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f250 str w10, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f254 strb w8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f258 ldur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f25c str w22, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f260 str w24, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f264 str w8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f268 str w23, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f26c ldp w3, w4, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[3]=(v0) as u32 as u64;cpu.x[4]=(v1) as u32 as u64;}
// 0x10f8f270 ldur x0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f274 ldur w6, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[6]=(v0) as u32 as u64;}
pc=0x10f8f278;continue;
}
0x10f8f278=>{
// 0x10f8f278 str x19, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f27c bl #0x10f97be8
cpu.x[30]=0x10f8f280;super::chunk12::f_10f97be8(cpu,rt,0x10f97be8)?;
pc=0x10f8f280;continue;
}
0x10f8f280=>{
// 0x10f8f280 ldp x20, x19, [sp, #0x120]
{let address=(cpu.sp).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f284 ldp x22, x21, [sp, #0x110]
{let address=(cpu.sp).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f288 ldp x24, x23, [sp, #0x100]
{let address=(cpu.sp).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f8f28c ldp x26, x25, [sp, #0xf0]
{let address=(cpu.sp).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f8f290 ldp x28, x27, [sp, #0xe0]
{let address=(cpu.sp).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f8f294 ldp x29, x30, [sp, #0xd0]
{let address=(cpu.sp).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f298 add sp, sp, #0x130
{let a=((cpu.sp));let b=((0x130u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f29c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8f2a0=>{
// 0x10f8f2a0 ldp w20, w25, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[20]=(v0) as u32 as u64;cpu.x[25]=(v1) as u32 as u64;}
// 0x10f8f2a4 ldrb w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f2a8 ldr w21, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f8f2ac mov w11, #1
cpu.x[11]=(((0x1u64))) as u32 as u64;
// 0x10f8f2b0 cbz w9, #0x10f8f2c4
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f8f2c4;continue;}pc=0x10f8f2b4;continue;
}
0x10f8f2b4=>{
// 0x10f8f2b4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f8f2b8 mov x28, x10
cpu.x[28]=(((cpu.x[10]))) as u64;
// 0x10f8f2bc stur w8, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f2c0 b #0x10f8f1e8
pc=0x10f8f1e8;continue;
}
0x10f8f2c4=>{
// 0x10f8f2c4 strb w11, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8f2c8 stur wzr, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f2cc b #0x10f8f1b8
pc=0x10f8f1b8;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8f03c")),}}}
#[inline]
pub(super) unsafe fn f_10f8f2d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8f2d0=>{
// 0x10f8f2d0 sub sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f8f2d4 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f2d8 str x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f2dc stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f2e0 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f2e4 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f2e8 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f8f2ec ldr x24, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f8f2f0 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8f2f4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8f2f8 mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f8f2fc ldr w9, [x24, #0x168]
{let address=(cpu.x[24]).wrapping_add(0x168u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f300 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f304 b.lt #0x10f8f42c
if cpu.condition(11){pc=0x10f8f42c;continue;}pc=0x10f8f308;continue;
}
0x10f8f308=>{
// 0x10f8f308 ldr x10, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f8f30c ldr w8, [x24, #0x16c]
{let address=(cpu.x[24]).wrapping_add(0x16cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f310 add w3, w9, w4
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[4] as u32 as u64));let result=a.wrapping_add(b);cpu.x[3]=(result) as u32 as u64;}
// 0x10f8f314 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f8f318 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f8f31c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10f8f320 ldr x22, [x10, #0xa0]
{let address=(cpu.x[10]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f8f324 mov w10, #-1
cpu.x[10]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f8f328 mov w4, w8
cpu.x[4]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10f8f32c mov w6, #1
cpu.x[6]=(((0x1u64))) as u32 as u64;
// 0x10f8f330 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f8f334 str w10, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f338 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f8f33c bl #0x10f9b008
cpu.x[30]=0x10f8f340;super::chunk12::f_10f9b008(cpu,rt,0x10f9b008)?;
pc=0x10f8f340;continue;
}
0x10f8f340=>{
// 0x10f8f340 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f8f344 str x0, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f348 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f8f34c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f8f350 bl #0x10e5ac7c
cpu.x[30]=0x10f8f354;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
pc=0x10f8f354;continue;
}
0x10f8f354=>{
// 0x10f8f354 tbnz w0, #0, #0x10f8f3c8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f8f3c8;continue;}pc=0x10f8f358;continue;
}
0x10f8f358=>{
// 0x10f8f358 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f35c mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f8f360 bl #0x10e5ac7c
cpu.x[30]=0x10f8f364;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
pc=0x10f8f364;continue;
}
0x10f8f364=>{
// 0x10f8f364 tbz w0, #0, #0x10f8f370
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f370;continue;}pc=0x10f8f368;continue;
}
0x10f8f368=>{
// 0x10f8f368 ldr w8, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f36c cbz w8, #0x10f8f3c8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8f3c8;continue;}pc=0x10f8f370;continue;
}
0x10f8f370=>{
// 0x10f8f370 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f374 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f378 b.lt #0x10f8f3c8
if cpu.condition(11){pc=0x10f8f3c8;continue;}pc=0x10f8f37c;continue;
}
0x10f8f37c=>{
// 0x10f8f37c mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10f8f380 b #0x10f8f394
pc=0x10f8f394;continue;
}
0x10f8f384=>{
// 0x10f8f384 ldrsw x8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f8f388 add x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10f8f38c cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8f390 b.ge #0x10f8f3c8
if cpu.condition(10){pc=0x10f8f3c8;continue;}pc=0x10f8f394;continue;
}
0x10f8f394=>{
// 0x10f8f394 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f398 ldr x24, [x8, x25, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[25])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f8f39c ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f3a0 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f8f3a4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f3a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8f3ac;super::dispatch(cpu,rt,target)?;}
pc=0x10f8f3ac;continue;
}
0x10f8f3ac=>{
// 0x10f8f3ac cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f3b0 b.ne #0x10f8f384
if cpu.condition(1){pc=0x10f8f384;continue;}pc=0x10f8f3b4;continue;
}
0x10f8f3b4=>{
// 0x10f8f3b4 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f8f3b8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f8f3bc bl #0x10f7b46c
cpu.x[30]=0x10f8f3c0;super::dispatch(cpu,rt,0x10f7b46c)?;
// 0x10f8f3c0 tbz w0, #0, #0x10f8f384
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f384;continue;}pc=0x10f8f3c4;continue;
}
0x10f8f3c4=>{
// 0x10f8f3c4 b #0x10f8f41c
pc=0x10f8f41c;continue;
}
0x10f8f3c8=>{
// 0x10f8f3c8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8f3cc bl #0x10f93d2c
cpu.x[30]=0x10f8f3d0;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f8f3d0;continue;
}
0x10f8f3d0=>{
// 0x10f8f3d0 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f3d4 ldr w9, [x0, #0x324]
{let address=(cpu.x[0]).wrapping_add(0x324u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8f3d8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8f3dc add w24, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10f8f3e0 str w24, [x0, #0x324]
{let address=(cpu.x[0]).wrapping_add(0x324u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8f3e4 b.lt #0x10f8f41c
if cpu.condition(11){pc=0x10f8f41c;continue;}pc=0x10f8f3e8;continue;
}
0x10f8f3e8=>{
// 0x10f8f3e8 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
pc=0x10f8f3ec;continue;
}
0x10f8f3ec=>{
// 0x10f8f3ec ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f3f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8f3f4 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f8f3f8 mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f8f3fc mov w4, w24
cpu.x[4]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
pc=0x10f8f400;continue;
}
0x10f8f400=>{
// 0x10f8f400 ldr x1, [x8, x25, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[25])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f8f404 bl #0x10f8ff74
cpu.x[30]=0x10f8f408;super::chunk1::f_10f8ff74(cpu,rt,0x10f8ff74)?;
pc=0x10f8f408;continue;
}
0x10f8f408=>{
// 0x10f8f408 tbnz w0, #0, #0x10f8f41c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f8f41c;continue;}pc=0x10f8f40c;continue;
}
0x10f8f40c=>{
// 0x10f8f40c ldrsw x8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f8f410 add x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10f8f414 cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8f418 b.lt #0x10f8f3ec
if cpu.condition(11){pc=0x10f8f3ec;continue;}pc=0x10f8f41c;continue;
}
0x10f8f41c=>{
// 0x10f8f41c add x1, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8f420 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f8f424 bl #0x10f9abb8
cpu.x[30]=0x10f8f428;super::chunk9::f_10f9abb8(cpu,rt,0x10f9abb8)?;
pc=0x10f8f428;continue;
}
0x10f8f428=>{
// 0x10f8f428 b #0x10f8f44c
pc=0x10f8f44c;continue;
}
0x10f8f42c=>{
// 0x10f8f42c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f8f430 mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f8f434 mov w23, w3
cpu.x[23]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f8f438 mov w22, w4
cpu.x[22]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f8f43c bl #0x10e5ac7c
cpu.x[30]=0x10f8f440;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
// 0x10f8f440 tbz w0, #0, #0x10f8f468
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f468;continue;}pc=0x10f8f444;continue;
}
0x10f8f444=>{
// 0x10f8f444 ldr w8, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f448 cbnz w8, #0x10f8f468
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f8f468;continue;}pc=0x10f8f44c;continue;
}
0x10f8f44c=>{
// 0x10f8f44c ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f450 ldr x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f8f454 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f458 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f8f45c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f460 add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f464 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8f468=>{
// 0x10f8f468 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8f46c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f8f470 mov w2, w19
cpu.x[2]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f8f474 mov w3, w23
cpu.x[3]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f8f478 mov w4, w22
cpu.x[4]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f8f47c ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f480 ldr x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f8f484 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f488 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f8f48c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f490 add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f494 b #0x10f913f4
super::chunk9::f_10f913f4(cpu,rt,0x10f913f4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8f2d0")),}}}
#[inline]
pub(super) unsafe fn f_10f93458(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93458=>{
// 0x10f93458 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9345c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f93460 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f93464 cbz x1, #0x10f9349c
if (cpu.x[1]) == 0 {pc=0x10f9349c;continue;}pc=0x10f93468;continue;
}
0x10f93468=>{
// 0x10f93468 ldr x8, [x0, #0x108]
{let address=(cpu.x[0]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9346c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f93470 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f93474 cmp x8, x1
{let a=((cpu.x[8]));let b=((cpu.x[1]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f93478 b.ne #0x10f9349c
if cpu.condition(1){pc=0x10f9349c;continue;}pc=0x10f9347c;continue;
}
0x10f9347c=>{
// 0x10f9347c ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f93480 bl #0x10e5acdc
cpu.x[30]=0x10f93484;super::chunk13::f_10e5acdc(cpu,rt,0x10e5acdc)?;
pc=0x10f93484;continue;
}
0x10f93484=>{
// 0x10f93484 tbz w0, #0, #0x10f9349c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9349c;continue;}pc=0x10f93488;continue;
}
0x10f93488=>{
// 0x10f93488 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9348c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f93490 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f93494 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f93498 b #0x10f8ef58
super::chunk6::f_10f8ef58(cpu,rt,0x10f8ef58)?;return Ok(());
}
0x10f9349c=>{
// 0x10f9349c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f934a0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f934a4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f934a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93458")),}}}
#[inline]
pub(super) unsafe fn f_10f93bd4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93bd4=>{
// 0x10f93bd4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93bd4")),}}}
#[inline]
pub(super) unsafe fn f_10f93fe8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93fe8=>{
// 0x10f93fe8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f93fec str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f93ff0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f93ff4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f93ff8 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f93ffc bl #0x10f966a8
cpu.x[30]=0x10f94000;super::chunk2::f_10f966a8(cpu,rt,0x10f966a8)?;
pc=0x10f94000;continue;
}
0x10f94000=>{
// 0x10f94000 ldr w1, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f94004 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f94008 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f9400c b #0x10d3c980
super::chunk13::f_10d3c980(cpu,rt,0x10d3c980)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93fe8")),}}}
#[inline]
pub(super) unsafe fn f_10f94298(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94298=>{
// 0x10f94298 ldr w0, [x0, #0x7c]
{let address=(cpu.x[0]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f9429c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94298")),}}}
#[inline]
pub(super) unsafe fn f_10f942c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f942c0=>{
// 0x10f942c0 add x0, x0, #0x7c
{let a=((cpu.x[0]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f942c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f942c0")),}}}
#[inline]
pub(super) unsafe fn f_10f966a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f966a8=>{
// 0x10f966a8 ldr x0, [x0, #0x98]
{let address=(cpu.x[0]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f966ac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f966a8")),}}}
#[inline]
pub(super) unsafe fn f_10f99668(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99668=>{
// 0x10f99668 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9966c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f99670 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99674 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f99678 ldrsw x8, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f9967c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f99680 cbz w8, #0x10f996ac
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f996ac;continue;}pc=0x10f99684;continue;
}
0x10f99684=>{
// 0x10f99684 ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f99688 lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f9968c b #0x10f9969c
pc=0x10f9969c;continue;
}
0x10f99690=>{
// 0x10f99690 subs x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99694 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f99698 b.eq #0x10f996ac
if cpu.condition(0){pc=0x10f996ac;continue;}pc=0x10f9969c;continue;
}
0x10f9969c=>{
// 0x10f9969c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f996a0 cbz x0, #0x10f99690
if (cpu.x[0]) == 0 {pc=0x10f99690;continue;}pc=0x10f996a4;continue;
}
0x10f996a4=>{
// 0x10f996a4 bl #0x1187d2c0
cpu.x[30]=0x10f996a8;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f996a8 b #0x10f99690
pc=0x10f99690;continue;
}
0x10f996ac=>{
// 0x10f996ac ldrsw x8, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f996b0 str wzr, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f996b4 cbz w8, #0x10f996e0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f996e0;continue;}pc=0x10f996b8;continue;
}
0x10f996b8=>{
// 0x10f996b8 ldr x20, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f996bc lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f996c0 b #0x10f996d0
pc=0x10f996d0;continue;
}
0x10f996c4=>{
// 0x10f996c4 subs x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f996c8 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f996cc b.eq #0x10f996e0
if cpu.condition(0){pc=0x10f996e0;continue;}pc=0x10f996d0;continue;
}
0x10f996d0=>{
// 0x10f996d0 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f996d4 cbz x0, #0x10f996c4
if (cpu.x[0]) == 0 {pc=0x10f996c4;continue;}pc=0x10f996d8;continue;
}
0x10f996d8=>{
// 0x10f996d8 bl #0x1187d2c0
cpu.x[30]=0x10f996dc;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f996dc b #0x10f996c4
pc=0x10f996c4;continue;
}
0x10f996e0=>{
// 0x10f996e0 str wzr, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f996e4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f996e8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f996ec ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f996f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99668")),}}}
#[inline]
pub(super) unsafe fn f_10f9ada0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9ada0=>{
// 0x10f9ada0 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9ada4 str x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9ada8 stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9adac stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9adb0 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9adb4 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9adb8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9adbc ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9adc0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f9adc4 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f9adc8 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f9adcc ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9add0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9add4;super::dispatch(cpu,rt,target)?;}
pc=0x10f9add4;continue;
}
0x10f9add4=>{
// 0x10f9add4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9add8 b.lt #0x10f9ae50
if cpu.condition(11){pc=0x10f9ae50;continue;}pc=0x10f9addc;continue;
}
0x10f9addc=>{
// 0x10f9addc mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9ade0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9ade4 bl #0x10f942c8
cpu.x[30]=0x10f9ade8;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f9ade8;continue;
}
0x10f9ade8=>{
// 0x10f9ade8 add w8, w21, #0xfa
{let a=((cpu.x[21] as u32 as u64));let b=((0xfau64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9adec cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9adf0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9adf4 csel w23, w21, w8, eq
cpu.x[23]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10f9adf8 bl #0x10f94298
cpu.x[30]=0x10f9adfc;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f9adfc;continue;
}
0x10f9adfc=>{
// 0x10f9adfc sub w8, w0, w23
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9ae00 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9ae04 asr w22, w8, #0xa
cpu.x[22]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9ae08 bl #0x10f94298
cpu.x[30]=0x10f9ae0c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f9ae0c;continue;
}
0x10f9ae0c=>{
// 0x10f9ae0c add w8, w0, w23
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9ae10 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9ae14 asr w24, w8, #0xa
cpu.x[24]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9ae18 bl #0x10f942a0
cpu.x[30]=0x10f9ae1c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f9ae1c;continue;
}
0x10f9ae1c=>{
// 0x10f9ae1c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9ae20 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9ae24 bl #0x10f942a0
cpu.x[30]=0x10f9ae28;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f9ae28;continue;
}
0x10f9ae28=>{
// 0x10f9ae28 cmp w22, w24
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ae2c b.gt #0x10f9ae48
if cpu.condition(12){pc=0x10f9ae48;continue;}pc=0x10f9ae30;continue;
}
0x10f9ae30=>{
// 0x10f9ae30 sub w8, w21, w23
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9ae34 add w9, w0, w23
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f9ae38 asr w23, w8, #0xa
cpu.x[23]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9ae3c asr w8, w9, #0xa
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9ae40 cmp w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ae44 b.le #0x10f9ae6c
if cpu.condition(13){pc=0x10f9ae6c;continue;}pc=0x10f9ae48;continue;
}
0x10f9ae48=>{
// 0x10f9ae48 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f9ae4c strb w8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10f9ae50;continue;
}
0x10f9ae50=>{
// 0x10f9ae50 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9ae54 ldr x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10f9ae58 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9ae5c ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f9ae60 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f9ae64 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10f9ae68 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f9ae6c=>{
// 0x10f9ae6c cmp w22, w24
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ae70 add w25, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10f9ae74 mov w26, #5
cpu.x[26]=(((0x5u64))) as u32 as u64;
// 0x10f9ae78 csel w24, w22, w24, gt
cpu.x[24]=(if cpu.condition(12){((cpu.x[22] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10f9ae7c b #0x10f9ae8c
pc=0x10f9ae8c;continue;
}
0x10f9ae80=>{
// 0x10f9ae80 cmp w22, w24
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ae84 add w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10f9ae88 b.eq #0x10f9ae48
if cpu.condition(0){pc=0x10f9ae48;continue;}pc=0x10f9ae8c;continue;
}
0x10f9ae8c=>{
// 0x10f9ae8c tbnz w22, #0x1f, #0x10f9ae80
if ((cpu.x[22] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f9ae80;continue;}pc=0x10f9ae90;continue;
}
0x10f9ae90=>{
// 0x10f9ae90 ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9ae94 mov w27, w23
cpu.x[27]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f9ae98 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ae9c b.gt #0x10f9aec0
if cpu.condition(12){pc=0x10f9aec0;continue;}pc=0x10f9aea0;continue;
}
0x10f9aea0=>{
// 0x10f9aea0 b #0x10f9ae80
pc=0x10f9ae80;continue;
}
0x10f9aea4=>{
// 0x10f9aea4 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9aea8 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f9aeac str w10, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9aeb0 str x19, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f9aeb4;continue;
}
0x10f9aeb4=>{
// 0x10f9aeb4 add w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10f9aeb8 cmp w25, w27
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9aebc b.eq #0x10f9ae80
if cpu.condition(0){pc=0x10f9ae80;continue;}pc=0x10f9aec0;continue;
}
0x10f9aec0=>{
// 0x10f9aec0 ldr w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9aec4 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9aec8 b.le #0x10f9aeb4
if cpu.condition(13){pc=0x10f9aeb4;continue;}pc=0x10f9aecc;continue;
}
0x10f9aecc=>{
// 0x10f9aecc tbnz w27, #0x1f, #0x10f9aeb4
if ((cpu.x[27] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f9aeb4;continue;}pc=0x10f9aed0;continue;
}
0x10f9aed0=>{
// 0x10f9aed0 ldr w9, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9aed4 cmp w9, w27
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9aed8 b.le #0x10f9aeb4
if cpu.condition(13){pc=0x10f9aeb4;continue;}pc=0x10f9aedc;continue;
}
0x10f9aedc=>{
// 0x10f9aedc madd w8, w8, w27, w22
cpu.x[8]=((((cpu.x[22] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[27] as u32 as u64))))) as u32 as u64;
// 0x10f9aee0 ldr x9, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9aee4 ldr x21, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f9aee8 cbz x21, #0x10f9aeb4
if (cpu.x[21]) == 0 {pc=0x10f9aeb4;continue;}pc=0x10f9aeec;continue;
}
0x10f9aeec=>{
// 0x10f9aeec ldp w9, w8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f9aef0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9aef4 b.ne #0x10f9aea4
if cpu.condition(1){pc=0x10f9aea4;continue;}pc=0x10f9aef8;continue;
}
0x10f9aef8=>{
// 0x10f9aef8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9aefc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9af00 csel w1, w26, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[26] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9af04 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9af08 b.ge #0x10f9aea4
if cpu.condition(10){pc=0x10f9aea4;continue;}pc=0x10f9af0c;continue;
}
0x10f9af0c=>{
// 0x10f9af0c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f9af10 bl #0x108d7c9c
cpu.x[30]=0x10f9af14;super::chunk10::f_108d7c9c(cpu,rt,0x108d7c9c)?;
// 0x10f9af14 ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9af18 b #0x10f9aea4
pc=0x10f9aea4;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9ada0")),}}}
#[inline]
pub(super) unsafe fn f_10f9afc8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9afc8=>{
// 0x10f9afc8 sub sp, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f9afcc stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9afd0 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f9afd4 mov w8, w7
cpu.x[8]=(((cpu.x[7] as u32 as u64))) as u32 as u64;
// 0x10f9afd8 mov w7, w6
cpu.x[7]=(((cpu.x[6] as u32 as u64))) as u32 as u64;
// 0x10f9afdc mov w6, w5
cpu.x[6]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10f9afe0 mov w5, w4
cpu.x[5]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f9afe4 mov w4, w3
cpu.x[4]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f9afe8 mov w3, w2
cpu.x[3]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f9afec ldp w9, w2, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10f9aff0 str w8, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9aff4 mov w1, w9
cpu.x[1]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x10f9aff8 bl #0x10f9b008
cpu.x[30]=0x10f9affc;super::chunk12::f_10f9b008(cpu,rt,0x10f9b008)?;
pc=0x10f9affc;continue;
}
0x10f9affc=>{
// 0x10f9affc ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
pc=0x10f9b000;continue;
}
0x10f9b000=>{
// 0x10f9b000 add sp, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f9b004 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9afc8")),}}}
#[inline]
pub(super) unsafe fn f_10fa04a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa04a8=>{
// 0x10fa04a8 cmp w1, #2
{let a=((cpu.x[1] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa04ac b.eq #0x10fa04c0
if cpu.condition(0){pc=0x10fa04c0;continue;}pc=0x10fa04b0;continue;
}
0x10fa04b0=>{
// 0x10fa04b0 cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa04b4 b.ne #0x10fa04c8
if cpu.condition(1){pc=0x10fa04c8;continue;}pc=0x10fa04b8;continue;
}
0x10fa04b8=>{
// 0x10fa04b8 ldr x0, [x0, #0x3a0]
{let address=(cpu.x[0]).wrapping_add(0x3a0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa04bc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa04c0=>{
// 0x10fa04c0 ldr x0, [x0, #0x3a8]
{let address=(cpu.x[0]).wrapping_add(0x3a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa04c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa04c8=>{
// 0x10fa04c8 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10fa04cc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa04a8")),}}}
#[inline]
pub(super) unsafe fn f_10fa87b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa87b8=>{
// 0x10fa87b8 b #0x10f73c30
super::chunk7::f_10f73c30(cpu,rt,0x10f73c30)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa87b8")),}}}
#[inline]
pub(super) unsafe fn f_10facb68(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10facb68=>{
// 0x10facb68 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10facb6c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facb70 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10facb74 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10facb78 mov x21, x2
cpu.x[21]=(((cpu.x[2]))) as u64;
// 0x10facb7c mov x2, x1
cpu.x[2]=(((cpu.x[1]))) as u64;
// 0x10facb80 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10facb84 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10facb88 bl #0x10f73c00
cpu.x[30]=0x10facb8c;super::chunk14::f_10f73c00(cpu,rt,0x10f73c00)?;
pc=0x10facb8c;continue;
}
0x10facb8c=>{
// 0x10facb8c adrp x8, #0x11937000
cpu.x[8]=(0x11937000u64) as u64;
// 0x10facb90 add x8, x8, #0x8f0
{let a=((cpu.x[8]));let b=((0x8f0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10facb94 mov x20, x19
cpu.x[20]=(((cpu.x[19]))) as u64;
// 0x10facb98 add x9, x8, #0x118
{let a=((cpu.x[8]));let b=((0x118u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10facb9c str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facba0 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10facba4 str x9, [x20, #0x60]!
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[20]=(address) as u64;}
// 0x10facba8 adrp x9, #0x102b3000
cpu.x[9]=(0x102b3000u64) as u64;
// 0x10facbac str w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10facbb0 ldr q0, [x9, #0x290]
{let address=(cpu.x[9]).wrapping_add(0x290u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10facbb4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10facbb8 str w8, [x19, #0x8c]
{let address=(cpu.x[19]).wrapping_add(0x8cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10facbbc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10facbc0 str w8, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10facbc4 mov x8, #-0x100000000
cpu.x[8]=(((0xffffffff00000000u64))) as u64;
// 0x10facbc8 str xzr, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facbcc stur q0, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10facbd0 strb wzr, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10facbd4 str xzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facbd8 stp xzr, xzr, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10facbdc str x8, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facbe0 bl #0x10f89114
cpu.x[30]=0x10facbe4;super::chunk6::f_10f89114(cpu,rt,0x10f89114)?;
pc=0x10facbe4;continue;
}
0x10facbe4=>{
// 0x10facbe4 str x0, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facbe8 mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x10facbec bl #0x1187d2b0
cpu.x[30]=0x10facbf0;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10facbf0;continue;
}
0x10facbf0=>{
// 0x10facbf0 mov w8, #8
cpu.x[8]=(((0x8u64))) as u32 as u64;
// 0x10facbf4 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10facbf8 str x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facbfc mov w0, #0x40
cpu.x[0]=(((0x40u64))) as u32 as u64;
pc=0x10facc00;continue;
}
0x10facc00=>{
// 0x10facc00 bl #0x1187d520
cpu.x[30]=0x10facc04;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10facc04;continue;
}
0x10facc04=>{
// 0x10facc04 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10facc08 str x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facc0c str x21, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10facc10 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10facc14 bl #0x10f93d2c
cpu.x[30]=0x10facc18;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10facc18;continue;
}
0x10facc18=>{
// 0x10facc18 ldr x0, [x0, #0x330]
{let address=(cpu.x[0]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10facc1c mov w1, #0xcc20
cpu.x[1]=(((0xcc20u64))) as u32 as u64;
// 0x10facc20 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10facc24 movk w1, #0xc9d, lsl #16
cpu.x[1]=((cpu.x[1] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10facc28 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10facc2c mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10facc30 mov w21, #0xcc20
cpu.x[21]=(((0xcc20u64))) as u32 as u64;
// 0x10facc34 movk w21, #0xc9d, lsl #16
cpu.x[21]=((cpu.x[21] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10facc38 bl #0x111fac24
cpu.x[30]=0x10facc3c;super::chunk14::f_111fac24(cpu,rt,0x111fac24)?;
pc=0x10facc3c;continue;
}
0x10facc3c=>{
// 0x10facc3c ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10facc40 bl #0x10f93d2c
cpu.x[30]=0x10facc44;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10facc44;continue;
}
0x10facc44=>{
// 0x10facc44 ldr x0, [x0, #0x330]
{let address=(cpu.x[0]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10facc48 orr w1, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a | b;cpu.x[1]=(result) as u32 as u64;}
// 0x10facc4c mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10facc50 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10facc54 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10facc58 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10facc5c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10facc60 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10facc64 b #0x111fac24
super::chunk14::f_111fac24(cpu,rt,0x111fac24)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10facb68")),}}}
#[inline]
pub(super) unsafe fn f_10fb7700(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fb7700=>{
// 0x10fb7700 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fb7704 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fb7708 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fb770c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fb7710 bl #0x10f73c60
cpu.x[30]=0x10fb7714;super::chunk7::f_10f73c60(cpu,rt,0x10f73c60)?;
pc=0x10fb7714;continue;
}
0x10fb7714=>{
// 0x10fb7714 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fb7718 ldr w8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb771c str w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fb7720 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fb7724 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fb7728 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fb7700")),}}}
#[inline]
pub(super) unsafe fn f_10fb77c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fb77c0=>{
// 0x10fb77c0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fb77c4 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fb77c8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fb77cc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fb77d0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fb77d4 bl #0x10f73d08
cpu.x[30]=0x10fb77d8;super::chunk12::f_10f73d08(cpu,rt,0x10f73d08)?;
pc=0x10fb77d8;continue;
}
0x10fb77d8=>{
// 0x10fb77d8 ldp x21, x20, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[21]=(v0) as u64;cpu.x[20]=(v1) as u64;}
// 0x10fb77dc ldr x8, [x21, #0x108]
{let address=(cpu.x[21]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fb77e0 ldr x9, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fb77e4 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fb77e8 b.eq #0x10fb77fc
if cpu.condition(0){pc=0x10fb77fc;continue;}pc=0x10fb77ec;continue;
}
0x10fb77ec=>{
// 0x10fb77ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fb77f0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fb77f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fb77f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fb77fc=>{
// 0x10fb77fc ldr w8, [x21, #0x110]
{let address=(cpu.x[21]).wrapping_add(0x110u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10fb7800;continue;
}
0x10fb7800=>{
// 0x10fb7800 cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fb7804 b.eq #0x10fb7828
if cpu.condition(0){pc=0x10fb7828;continue;}pc=0x10fb7808;continue;
}
0x10fb7808=>{
// 0x10fb7808 ldr w9, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fb780c cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fb7810 b.lt #0x10fb7828
if cpu.condition(11){pc=0x10fb7828;continue;}pc=0x10fb7814;continue;
}
0x10fb7814=>{
// 0x10fb7814 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fb7818 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fb781c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fb7820 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fb7824 b #0x10f73ce4
super::chunk9::f_10f73ce4(cpu,rt,0x10f73ce4)?;return Ok(());
}
0x10fb7828=>{
// 0x10fb7828 ldrb w8, [x21, #0x100]
{let address=(cpu.x[21]).wrapping_add(0x100u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb782c cbz w8, #0x10fb7850
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fb7850;continue;}pc=0x10fb7830;continue;
}
0x10fb7830=>{
// 0x10fb7830 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fb7834 bl #0x10f940fc
cpu.x[30]=0x10fb7838;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fb7838 cbz x0, #0x10fb789c
if (cpu.x[0]) == 0 {pc=0x10fb789c;continue;}pc=0x10fb783c;continue;
}
0x10fb783c=>{
// 0x10fb783c mov w1, #0x64
cpu.x[1]=(((0x64u64))) as u32 as u64;
// 0x10fb7840 bl #0x10fc27b8
cpu.x[30]=0x10fb7844;super::chunk9::f_10fc27b8(cpu,rt,0x10fc27b8)?;
// 0x10fb7844 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fb7848 b.ge #0x10fb78a0
if cpu.condition(10){pc=0x10fb78a0;continue;}pc=0x10fb784c;continue;
}
0x10fb784c=>{
// 0x10fb784c b #0x10fb77ec
pc=0x10fb77ec;continue;
}
0x10fb7850=>{
// 0x10fb7850 ldrb w8, [x21, #0x101]
{let address=(cpu.x[21]).wrapping_add(0x101u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb7854 cbz w8, #0x10fb7878
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fb7878;continue;}pc=0x10fb7858;continue;
}
0x10fb7858=>{
// 0x10fb7858 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fb785c bl #0x10f940fc
cpu.x[30]=0x10fb7860;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fb7860 cbz x0, #0x10fb789c
if (cpu.x[0]) == 0 {pc=0x10fb789c;continue;}pc=0x10fb7864;continue;
}
0x10fb7864=>{
// 0x10fb7864 mov w1, #0x64
cpu.x[1]=(((0x64u64))) as u32 as u64;
// 0x10fb7868 bl #0x10fc26d4
cpu.x[30]=0x10fb786c;super::chunk9::f_10fc26d4(cpu,rt,0x10fc26d4)?;
// 0x10fb786c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fb7870 b.ge #0x10fb78a0
if cpu.condition(10){pc=0x10fb78a0;continue;}pc=0x10fb7874;continue;
}
0x10fb7874=>{
// 0x10fb7874 b #0x10fb77ec
pc=0x10fb77ec;continue;
}
0x10fb7878=>{
// 0x10fb7878 ldrb w8, [x21, #0x102]
{let address=(cpu.x[21]).wrapping_add(0x102u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb787c cbz w8, #0x10fb789c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fb789c;continue;}pc=0x10fb7880;continue;
}
0x10fb7880=>{
// 0x10fb7880 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fb7884 bl #0x10f940fc
cpu.x[30]=0x10fb7888;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fb7888 cbz x0, #0x10fb789c
if (cpu.x[0]) == 0 {pc=0x10fb789c;continue;}pc=0x10fb788c;continue;
}
0x10fb788c=>{
// 0x10fb788c bl #0x10fc23f0
cpu.x[30]=0x10fb7890;super::chunk1::f_10fc23f0(cpu,rt,0x10fc23f0)?;
// 0x10fb7890 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fb7894 b.ge #0x10fb78a0
if cpu.condition(10){pc=0x10fb78a0;continue;}pc=0x10fb7898;continue;
}
0x10fb7898=>{
// 0x10fb7898 b #0x10fb77ec
pc=0x10fb77ec;continue;
}
0x10fb789c=>{
// 0x10fb789c mov w0, #0x64
cpu.x[0]=(((0x64u64))) as u32 as u64;
pc=0x10fb78a0;continue;
}
0x10fb78a0=>{
// 0x10fb78a0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb78a4 sub w8, w8, w0, lsr #1
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[0] as u32 as u64)) as u32).wrapping_shr(1) as u64;let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fb78a8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fb78ac str w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fb78b0 b.gt #0x10fb77ec
if cpu.condition(12){pc=0x10fb77ec;continue;}pc=0x10fb78b4;continue;
}
0x10fb78b4=>{
// 0x10fb78b4 ldr w9, [x21, #0xec]
{let address=(cpu.x[21]).wrapping_add(0xecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fb78b8 ldr x21, [x21, #0xf8]
{let address=(cpu.x[21]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fb78bc add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fb78c0 str w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fb78c4 cbz x21, #0x10fb7910
if (cpu.x[21]) == 0 {pc=0x10fb7910;continue;}pc=0x10fb78c8;continue;
}
0x10fb78c8=>{
// 0x10fb78c8 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fb78cc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fb78d0 ldr x8, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fb78d4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fb78d8;super::dispatch(cpu,rt,target)?;}
pc=0x10fb78d8;continue;
}
0x10fb78d8=>{
// 0x10fb78d8 ldr x22, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fb78dc tbz w0, #0, #0x10fb78f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fb78f4;continue;}pc=0x10fb78e0;continue;
}
0x10fb78e0=>{
// 0x10fb78e0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fb78e4 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fb78e8 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10fb78ec bl #0x10f89720
cpu.x[30]=0x10fb78f0;super::chunk13::f_10f89720(cpu,rt,0x10f89720)?;
// 0x10fb78f0 b #0x10fb7910
pc=0x10fb7910;continue;
}
0x10fb78f4=>{
// 0x10fb78f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fb78f8 bl #0x10f73cc4
cpu.x[30]=0x10fb78fc;super::chunk6::f_10f73cc4(cpu,rt,0x10f73cc4)?;
pc=0x10fb78fc;continue;
}
0x10fb78fc=>{
// 0x10fb78fc mov x3, x0
cpu.x[3]=(((cpu.x[0]))) as u64;
// 0x10fb7900 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fb7904 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fb7908 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10fb790c bl #0x10f89abc
cpu.x[30]=0x10fb7910;super::chunk8::f_10f89abc(cpu,rt,0x10f89abc)?;
pc=0x10fb7910;continue;
}
0x10fb7910=>{
// 0x10fb7910 ldr w8, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fb7914 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fb7918 str w8, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fb791c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fb7920 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fb7924 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fb7928 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fb77c0")),}}}
#[inline]
pub(super) unsafe fn f_10fbb9f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbb9f0=>{
// 0x10fbb9f0 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbb9f4 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbb9f8 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbb9fc stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbba00 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbba04 ldp x22, x21, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fbba08 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fbba0c ldr x20, [x1, #0x10]
{let address=(cpu.x[1]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fbba10 ldrb w8, [x22, #0x118]
{let address=(cpu.x[22]).wrapping_add(0x118u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fbba14 cbz w8, #0x10fbbafc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fbbafc;continue;}pc=0x10fbba18;continue;
}
0x10fbba18=>{
// 0x10fbba18 ldr w11, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fbba1c cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbba20 b.lt #0x10fbbbb8
if cpu.condition(11){pc=0x10fbbbb8;continue;}pc=0x10fbba24;continue;
}
0x10fbba24=>{
// 0x10fbba24 ldr w13, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10fbba28 ldr x10, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fbba2c lsl x14, x11, #2
cpu.x[14]=(((((cpu.x[11]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fbba30 mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10fbba34 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
// 0x10fbba38 sub w12, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
pc=0x10fbba3c;continue;
}
0x10fbba3c=>{
// 0x10fbba3c ldr w15, [x10, x8]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x10fbba40 cmp w13, w15
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[15] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbba44 b.eq #0x10fbba60
if cpu.condition(0){pc=0x10fbba60;continue;}pc=0x10fbba48;continue;
}
0x10fbba48=>{
// 0x10fbba48 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fbba4c sub w12, w12, #1
{let a=((cpu.x[12] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10fbba50 sub x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10fbba54 cmp x14, x8
{let a=((cpu.x[14]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fbba58 b.ne #0x10fbba3c
if cpu.condition(1){pc=0x10fbba3c;continue;}pc=0x10fbba5c;continue;
}
0x10fbba5c=>{
// 0x10fbba5c b #0x10fbbbb8
pc=0x10fbbbb8;continue;
}
0x10fbba60=>{
// 0x10fbba60 neg w13, w9
cpu.x[13]=(0u64.wrapping_sub(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10fbba64 sub w14, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u32 as u64;}
// 0x10fbba68 cmp w14, w13
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbba6c str w14, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbba70 b.le #0x10fbbafc
if cpu.condition(13){pc=0x10fbbafc;continue;}pc=0x10fbba74;continue;
}
0x10fbba74=>{
// 0x10fbba74 add x13, x14, x9
{let a=((cpu.x[14]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fbba78 cmp w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbba7c b.lt #0x10fbbafc
if cpu.condition(11){pc=0x10fbbafc;continue;}pc=0x10fbba80;continue;
}
0x10fbba80=>{
// 0x10fbba80 cmp w13, #7
{let a=((cpu.x[13] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbba84 add x11, x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fbba88 b.ls #0x10fbbadc
if cpu.condition(9){pc=0x10fbbadc;continue;}pc=0x10fbba8c;continue;
}
0x10fbba8c=>{
// 0x10fbba8c mov w12, w12
cpu.x[12]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x10fbba90 lsr x15, x12, #3
cpu.x[15]=(((((cpu.x[12]))) as u64).wrapping_shr(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fbba94 and x12, x13, #0xffffffff
{let a=((cpu.x[13]));let b=((0xffffffffu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fbba98 neg w16, w15, lsl #3
cpu.x[16]=(0u64.wrapping_sub((((cpu.x[15] as u32 as u64)) as u32).wrapping_shl(3) as u64)) as u32 as u64;
// 0x10fbba9c add x15, x10, x15, lsl #5
{let a=((cpu.x[10]));let b=(((cpu.x[15])) as u64).wrapping_shl(5) as u64;let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10fbbaa0 and x10, x12, #0xfffffff8
{let a=((cpu.x[12]));let b=((0xfffffff8u64));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10fbbaa4 add x16, x16, w14, uxtw
{let a=((cpu.x[16]));let b=(((cpu.x[14] as u32 as u64) as u32 as u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x10fbbaa8 add x14, x11, #0x10
{let a=((cpu.x[11]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10fbbaac add x11, x15, x8
{let a=((cpu.x[15]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fbbab0 and x8, x13, #0xfffffff8
{let a=((cpu.x[13]));let b=((0xfffffff8u64));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10fbbab4 add x9, x16, x9
{let a=((cpu.x[16]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x10fbbab8;continue;
}
0x10fbbab8=>{
// 0x10fbbab8 ldur q0, [x14, #-0xc]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fbbabc ldur q1, [x14, #4]
{let address=(cpu.x[14]).wrapping_add(0x4u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fbbac0 subs x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fbbac4 stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fbbac8 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10fbbacc b.ne #0x10fbbab8
if cpu.condition(1){pc=0x10fbbab8;continue;}pc=0x10fbbad0;continue;
}
0x10fbbad0=>{
// 0x10fbbad0 cmp x10, x12
{let a=((cpu.x[10]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fbbad4 b.eq #0x10fbbafc
if cpu.condition(0){pc=0x10fbbafc;continue;}pc=0x10fbbad8;continue;
}
0x10fbbad8=>{
// 0x10fbbad8 mov w13, w9
cpu.x[13]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
pc=0x10fbbadc;continue;
}
0x10fbbadc=>{
// 0x10fbbadc add w8, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fbbae0 add x9, x11, #4
{let a=((cpu.x[11]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x10fbbae4;continue;
}
0x10fbbae4=>{
// 0x10fbbae4 ldr w10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fbbae8 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fbbaec cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbaf0 stur w10, [x9, #-4]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbaf4 add x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10fbbaf8 b.hi #0x10fbbae4
if cpu.condition(8){pc=0x10fbbae4;continue;}pc=0x10fbbafc;continue;
}
0x10fbbafc=>{
// 0x10fbbafc ldrb w8, [x22, #0x119]
{let address=(cpu.x[22]).wrapping_add(0x119u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fbbb00 cbz w8, #0x10fbbb20
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fbbb20;continue;}pc=0x10fbbb04;continue;
}
0x10fbbb04=>{
// 0x10fbbb04 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fbbb08 bl #0x10f93d3c
cpu.x[30]=0x10fbbb0c;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10fbbb0c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fbbb10 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fbbb14 bl #0x10f93d3c
cpu.x[30]=0x10fbbb18;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10fbbb18 cmp w23, w0
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbb1c b.ne #0x10fbbbb8
if cpu.condition(1){pc=0x10fbbbb8;continue;}pc=0x10fbbb20;continue;
}
0x10fbbb20=>{
// 0x10fbbb20 ldr x0, [x22, #0x110]
{let address=(cpu.x[22]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fbbb24 cbz x0, #0x10fbbb38
if (cpu.x[0]) == 0 {pc=0x10fbbb38;continue;}pc=0x10fbbb28;continue;
}
0x10fbbb28=>{
// 0x10fbbb28 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fbbb2c mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10fbbb30 bl #0x10e31fc8
cpu.x[30]=0x10fbbb34;super::chunk7::f_10e31fc8(cpu,rt,0x10e31fc8)?;
pc=0x10fbbb34;continue;
}
0x10fbbb34=>{
// 0x10fbbb34 tbz w0, #0, #0x10fbbbb8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fbbbb8;continue;}pc=0x10fbbb38;continue;
}
0x10fbbb38=>{
// 0x10fbbb38 ldr w23, [x22, #0xfc]
{let address=(cpu.x[22]).wrapping_add(0xfcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10fbbb3c cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbb40 b.lt #0x10fbbb5c
if cpu.condition(11){pc=0x10fbbb5c;continue;}pc=0x10fbbb44;continue;
}
0x10fbbb44=>{
// 0x10fbbb44 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fbbb48 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fbbb4c bl #0x10f94f34
cpu.x[30]=0x10fbbb50;super::chunk8::f_10f94f34(cpu,rt,0x10f94f34)?;
// 0x10fbbb50 mul w8, w23, w23
cpu.x[8]=((((cpu.x[23] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x10fbbb54 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbb58 b.gt #0x10fbbbb8
if cpu.condition(12){pc=0x10fbbbb8;continue;}pc=0x10fbbb5c;continue;
}
0x10fbbb5c=>{
// 0x10fbbb5c ldr x1, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fbbb60 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fbbb64 bl #0x10ec6c6c
cpu.x[30]=0x10fbbb68;super::chunk6::f_10ec6c6c(cpu,rt,0x10ec6c6c)?;
pc=0x10fbbb68;continue;
}
0x10fbbb68=>{
// 0x10fbbb68 tbz w0, #0, #0x10fbbbb8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fbbbb8;continue;}pc=0x10fbbb6c;continue;
}
0x10fbbb6c=>{
// 0x10fbbb6c ldr x22, [x22, #0xf0]
{let address=(cpu.x[22]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fbbb70 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbb74 bl #0x10f73cc4
cpu.x[30]=0x10fbbb78;super::chunk6::f_10f73cc4(cpu,rt,0x10f73cc4)?;
pc=0x10fbbb78;continue;
}
0x10fbbb78=>{
// 0x10fbbb78 mov x3, x0
cpu.x[3]=(((cpu.x[0]))) as u64;
// 0x10fbbb7c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fbbb80 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fbbb84 mov x2, x22
cpu.x[2]=(((cpu.x[22]))) as u64;
// 0x10fbbb88 bl #0x10f89abc
cpu.x[30]=0x10fbbb8c;super::chunk8::f_10f89abc(cpu,rt,0x10f89abc)?;
pc=0x10fbbb8c;continue;
}
0x10fbbb8c=>{
// 0x10fbbb8c ldr w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fbbb90 subs w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fbbb94 b.lt #0x10fbbbb8
if cpu.condition(11){pc=0x10fbbbb8;continue;}pc=0x10fbbb98;continue;
}
0x10fbbb98=>{
// 0x10fbbb98 str w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbb9c b.ne #0x10fbbbb8
if cpu.condition(1){pc=0x10fbbbb8;continue;}pc=0x10fbbba0;continue;
}
0x10fbbba0=>{
// 0x10fbbba0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbba4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbbba8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fbbbac ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fbbbb0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fbbbb4 b #0x10f73ce4
super::chunk9::f_10f73ce4(cpu,rt,0x10f73ce4)?;return Ok(());
}
0x10fbbbb8=>{
// 0x10fbbbb8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbbbbc ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fbbbc0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fbbbc4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fbbbc8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbb9f0")),}}}
#[inline]
pub(super) unsafe fn f_10fbc914(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbc914=>{
// 0x10fbc914 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbc918 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbc91c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbc920 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fbc924 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fbc928 bl #0x10f73fd8
cpu.x[30]=0x10fbc92c;super::chunk9::f_10f73fd8(cpu,rt,0x10f73fd8)?;
pc=0x10fbc92c;continue;
}
0x10fbc92c=>{
// 0x10fbc92c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc930 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc934 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc938 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbc93c;super::dispatch(cpu,rt,target)?;}
pc=0x10fbc93c;continue;
}
0x10fbc93c=>{
// 0x10fbc93c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc940 str w0, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbc944 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc948 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc94c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbc950;super::dispatch(cpu,rt,target)?;}
pc=0x10fbc950;continue;
}
0x10fbc950=>{
// 0x10fbc950 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc954 str w0, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbc958 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc95c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc960 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbc964;super::dispatch(cpu,rt,target)?;}
pc=0x10fbc964;continue;
}
0x10fbc964=>{
// 0x10fbc964 str w0, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbc968 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbc96c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbc970 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbc914")),}}}
#[inline]
pub(super) unsafe fn f_10fc08f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc08f0=>{
// 0x10fc08f0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc08f4 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc08f8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc08fc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc0900 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fc0904 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc0908 bl #0x10fc96e4
cpu.x[30]=0x10fc090c;super::chunk5::f_10fc96e4(cpu,rt,0x10fc96e4)?;
pc=0x10fc090c;continue;
}
0x10fc090c=>{
// 0x10fc090c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0910 ldr w1, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc0914 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc0918 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc091c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc0920;super::dispatch(cpu,rt,target)?;}
pc=0x10fc0920;continue;
}
0x10fc0920=>{
// 0x10fc0920 ldr w8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc0924 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc0928 b.lt #0x10fc0950
if cpu.condition(11){pc=0x10fc0950;continue;}pc=0x10fc092c;continue;
}
0x10fc092c=>{
// 0x10fc092c mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10fc0930;continue;
}
0x10fc0930=>{
// 0x10fc0930 ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0934 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fc0938 ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc093c bl #0x10f86190
cpu.x[30]=0x10fc0940;super::chunk6::f_10f86190(cpu,rt,0x10f86190)?;
pc=0x10fc0940;continue;
}
0x10fc0940=>{
// 0x10fc0940 ldrsw x8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fc0944 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10fc0948 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc094c b.lt #0x10fc0930
if cpu.condition(11){pc=0x10fc0930;continue;}pc=0x10fc0950;continue;
}
0x10fc0950=>{
// 0x10fc0950 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0954 ldr w1, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc0958 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc095c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0960 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc0964;super::dispatch(cpu,rt,target)?;}
pc=0x10fc0964;continue;
}
0x10fc0964=>{
// 0x10fc0964 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0968 ldr w1, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc096c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc0970 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0974 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc0978;super::dispatch(cpu,rt,target)?;}
pc=0x10fc0978;continue;
}
0x10fc0978=>{
// 0x10fc0978 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc097c ldr w1, [x20, #0x44]
{let address=(cpu.x[20]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc0980 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc0984 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0988 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc098c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc098c;continue;
}
0x10fc098c=>{
// 0x10fc098c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc0990 ldr w1, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc0994 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc0998 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc099c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc09a0;super::dispatch(cpu,rt,target)?;}
pc=0x10fc09a0;continue;
}
0x10fc09a0=>{
// 0x10fc09a0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09a4 ldr w1, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc09a8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc09ac ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc09b4;super::dispatch(cpu,rt,target)?;}
pc=0x10fc09b4;continue;
}
0x10fc09b4=>{
// 0x10fc09b4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09b8 ldr w1, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc09bc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc09c0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09c4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc09c8;super::dispatch(cpu,rt,target)?;}
pc=0x10fc09c8;continue;
}
0x10fc09c8=>{
// 0x10fc09c8 ldr x1, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fc09cc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc09d0 bl #0x111dd6a4
cpu.x[30]=0x10fc09d4;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10fc09d4;continue;
}
0x10fc09d4=>{
// 0x10fc09d4 ldr x8, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09d8 cbz x8, #0x10fc09fc
if (cpu.x[8]) == 0 {pc=0x10fc09fc;continue;}pc=0x10fc09dc;continue;
}
0x10fc09dc=>{
// 0x10fc09dc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc09e0 ldr w1, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc09e4 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10fc09e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc09ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc09f0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc09f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc09f8 br x2
{let target=cpu.x[2];if target>=0x10fc08f0 && target<0x10fc0a0c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10fc09fc=>{
// 0x10fc09fc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc0a00 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc0a04 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc0a08 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc08f0")),}}}
#[inline]
pub(super) unsafe fn f_10fc2288(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc2288=>{
// 0x10fc2288 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc228c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2290 b.lt #0x10fc22b4
if cpu.condition(11){pc=0x10fc22b4;continue;}pc=0x10fc2294;continue;
}
0x10fc2294=>{
// 0x10fc2294 ldr x9, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
pc=0x10fc2298;continue;
}
0x10fc2298=>{
// 0x10fc2298 ldr x10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc229c ldr x10, [x10, #0x18]
{let address=(cpu.x[10]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc22a0 ldr x0, [x10, #0x1b0]
{let address=(cpu.x[10]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc22a4 cbnz x0, #0x10fc22b8
if (cpu.x[0]) != 0 {pc=0x10fc22b8;continue;}pc=0x10fc22a8;continue;
}
0x10fc22a8=>{
// 0x10fc22a8 subs x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fc22ac add x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10fc22b0 b.ne #0x10fc2298
if cpu.condition(1){pc=0x10fc2298;continue;}pc=0x10fc22b4;continue;
}
0x10fc22b4=>{
// 0x10fc22b4 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
pc=0x10fc22b8;continue;
}
0x10fc22b8=>{
// 0x10fc22b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc2288")),}}}
#[inline]
pub(super) unsafe fn f_10fc2a68(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc2a68=>{
// 0x10fc2a68 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc2a6c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc2a70 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2a74 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2a78 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc2a7c ldr w20, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10fc2a80 cmp w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2a84 b.lt #0x10fc2b10
if cpu.condition(11){pc=0x10fc2b10;continue;}pc=0x10fc2a88;continue;
}
0x10fc2a88=>{
// 0x10fc2a88 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc2a8c b #0x10fc2a98
pc=0x10fc2a98;continue;
}
0x10fc2a90=>{
// 0x10fc2a90 cmp x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2a94 b.le #0x10fc2b10
if cpu.condition(13){pc=0x10fc2b10;continue;}pc=0x10fc2a98;continue;
}
0x10fc2a98=>{
// 0x10fc2a98 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2a9c mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fc2aa0 sub x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u64;}
// 0x10fc2aa4 ldr x8, [x8, x20, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2aa8 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2aac bl #0x10ddad00
cpu.x[30]=0x10fc2ab0;super::chunk13::f_10ddad00(cpu,rt,0x10ddad00)?;
pc=0x10fc2ab0;continue;
}
0x10fc2ab0=>{
// 0x10fc2ab0 tbz w0, #0, #0x10fc2a90
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc2a90;continue;}pc=0x10fc2ab4;continue;
}
0x10fc2ab4=>{
// 0x10fc2ab4 ldr w22, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10fc2ab8 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2abc b.lt #0x10fc2a90
if cpu.condition(11){pc=0x10fc2a90;continue;}pc=0x10fc2ac0;continue;
}
0x10fc2ac0=>{
// 0x10fc2ac0 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2ac4 ldr x8, [x8, x20, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2ac8 ldr x23, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fc2acc b #0x10fc2ae0
pc=0x10fc2ae0;continue;
}
0x10fc2ad0=>{
// 0x10fc2ad0 sub x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x10fc2ad4 add x8, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc2ad8 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2adc b.ls #0x10fc2a90
if cpu.condition(9){pc=0x10fc2a90;continue;}pc=0x10fc2ae0;continue;
}
0x10fc2ae0=>{
// 0x10fc2ae0 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2ae4 add x8, x8, x22, lsl #3
{let a=((cpu.x[8]));let b=(((cpu.x[22])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc2ae8 ldur x8, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2aec ldr x9, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc2af0 cbnz x9, #0x10fc2ad0
if (cpu.x[9]) != 0 {pc=0x10fc2ad0;continue;}pc=0x10fc2af4;continue;
}
0x10fc2af4=>{
// 0x10fc2af4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2af8 cmp x8, x23
{let a=((cpu.x[8]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2afc b.ne #0x10fc2ad0
if cpu.condition(1){pc=0x10fc2ad0;continue;}pc=0x10fc2b00;continue;
}
0x10fc2b00=>{
// 0x10fc2b00 sub w1, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10fc2b04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc2b08 bl #0x10fc0558
cpu.x[30]=0x10fc2b0c;super::chunk5::f_10fc0558(cpu,rt,0x10fc0558)?;
// 0x10fc2b0c b #0x10fc2ad0
pc=0x10fc2ad0;continue;
}
0x10fc2b10=>{
// 0x10fc2b10 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc2b14 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fc2b18 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc2b1c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fc2b20 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc2a68")),}}}
#[inline]
pub(super) unsafe fn f_10fc9cbc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc9cbc=>{
// 0x10fc9cbc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc9cc0 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc9cc4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc9cc8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc9ccc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fc9cd0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc9cd4 bl #0x10fc96e4
cpu.x[30]=0x10fc9cd8;super::chunk5::f_10fc96e4(cpu,rt,0x10fc96e4)?;
pc=0x10fc9cd8;continue;
}
0x10fc9cd8=>{
// 0x10fc9cd8 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc9cdc bl #0x10f7e050
cpu.x[30]=0x10fc9ce0;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc9ce0;continue;
}
0x10fc9ce0=>{
// 0x10fc9ce0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9ce4 ldr w1, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9ce8 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fc9cec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9cf0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9cf4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9cf8;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9cf8;continue;
}
0x10fc9cf8=>{
// 0x10fc9cf8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9cfc ldr w1, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d00 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d04 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d0c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d0c;continue;
}
0x10fc9d0c=>{
// 0x10fc9d0c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d10 ldr w1, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d14 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d18 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d20;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d20;continue;
}
0x10fc9d20=>{
// 0x10fc9d20 ldr w8, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9d24 cbz w8, #0x10fc9d3c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc9d3c;continue;}pc=0x10fc9d28;continue;
}
0x10fc9d28=>{
// 0x10fc9d28 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d2c ldr w1, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d30 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d34 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d38 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d3c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d3c;continue;
}
0x10fc9d3c=>{
// 0x10fc9d3c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d40 ldr w1, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d44 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d48 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d4c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d50;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d50;continue;
}
0x10fc9d50=>{
// 0x10fc9d50 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d54 ldr w1, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d58 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d5c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d60 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d64;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d64;continue;
}
0x10fc9d64=>{
// 0x10fc9d64 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc9d68 bl #0x10ddff74
cpu.x[30]=0x10fc9d6c;super::chunk11::f_10ddff74(cpu,rt,0x10ddff74)?;
pc=0x10fc9d6c;continue;
}
0x10fc9d6c=>{
// 0x10fc9d6c tbnz w0, #0, #0x10fc9e54
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc9e54;continue;}pc=0x10fc9d70;continue;
}
0x10fc9d70=>{
// 0x10fc9d70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc9d74 bl #0x10de0ba8
cpu.x[30]=0x10fc9d78;super::chunk12::f_10de0ba8(cpu,rt,0x10de0ba8)?;
pc=0x10fc9d78;continue;
}
0x10fc9d78=>{
// 0x10fc9d78 tbnz w0, #0, #0x10fc9e74
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc9e74;continue;}pc=0x10fc9d7c;continue;
}
0x10fc9d7c=>{
// 0x10fc9d7c ldr w8, [x21, #0x674]
{let address=(cpu.x[21]).wrapping_add(0x674u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9d80 cbz w8, #0x10fc9d98
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc9d98;continue;}pc=0x10fc9d84;continue;
}
0x10fc9d84=>{
// 0x10fc9d84 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d88 ldr w1, [x20, #0x44]
{let address=(cpu.x[20]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9d8c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9d90 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d94 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9d98;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9d98;continue;
}
0x10fc9d98=>{
// 0x10fc9d98 ldr x8, [x21, #0x638]
{let address=(cpu.x[21]).wrapping_add(0x638u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9d9c cbz x8, #0x10fc9de0
if (cpu.x[8]) == 0 {pc=0x10fc9de0;continue;}pc=0x10fc9da0;continue;
}
0x10fc9da0=>{
// 0x10fc9da0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9da4 ldr w1, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9da8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9dac ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9db0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9db4;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9db4;continue;
}
0x10fc9db4=>{
// 0x10fc9db4 ldrsw x8, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fc9db8 cbz w8, #0x10fc9de0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc9de0;continue;}pc=0x10fc9dbc;continue;
}
0x10fc9dbc=>{
// 0x10fc9dbc ldr x21, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc9dc0 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
pc=0x10fc9dc4;continue;
}
0x10fc9dc4=>{
// 0x10fc9dc4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9dc8 ldr x1, [x21], #8
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;cpu.x[21]=((cpu.x[21]).wrapping_add(((0x8u64)))) as u64;}
// 0x10fc9dcc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9dd0 ldr x8, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9dd4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9dd8;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9dd8;continue;
}
0x10fc9dd8=>{
// 0x10fc9dd8 subs x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fc9ddc b.ne #0x10fc9dc4
if cpu.condition(1){pc=0x10fc9dc4;continue;}pc=0x10fc9de0;continue;
}
0x10fc9de0=>{
// 0x10fc9de0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9de4 ldr w21, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fc9de8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9dec ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9df0 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fc9df4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9df8;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9df8;continue;
}
0x10fc9df8=>{
// 0x10fc9df8 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc9dfc b.lt #0x10fc9e44
if cpu.condition(11){pc=0x10fc9e44;continue;}pc=0x10fc9e00;continue;
}
0x10fc9e00=>{
// 0x10fc9e00 lsl x21, x21, #2
cpu.x[21]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fc9e04 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10fc9e08;continue;
}
0x10fc9e08=>{
// 0x10fc9e08 ldr x8, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e0c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc9e10 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9e14 ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9e18 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9e20;super::dispatch(cpu,rt,target)?;}
// 0x10fc9e20 ldr x8, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e24 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc9e28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9e2c ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9e30 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e34 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9e38;super::dispatch(cpu,rt,target)?;}
// 0x10fc9e38 add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10fc9e3c cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc9e40 b.ne #0x10fc9e08
if cpu.condition(1){pc=0x10fc9e08;continue;}pc=0x10fc9e44;continue;
}
0x10fc9e44=>{
// 0x10fc9e44 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc9e48 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc9e4c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc9e50 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc9e54=>{
// 0x10fc9e54 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e58 ldr w1, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9e5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9e60 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9e68;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9e68;continue;
}
0x10fc9e68=>{
// 0x10fc9e68 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc9e6c bl #0x10de0ba8
cpu.x[30]=0x10fc9e70;super::chunk12::f_10de0ba8(cpu,rt,0x10de0ba8)?;
pc=0x10fc9e70;continue;
}
0x10fc9e70=>{
// 0x10fc9e70 tbz w0, #0, #0x10fc9d7c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc9d7c;continue;}pc=0x10fc9e74;continue;
}
0x10fc9e74=>{
// 0x10fc9e74 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e78 ldr w1, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9e7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9e80 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9e88;super::dispatch(cpu,rt,target)?;}
// 0x10fc9e88 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e8c ldr w1, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc9e90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9e94 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9e98 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9e9c;super::dispatch(cpu,rt,target)?;}
// 0x10fc9e9c ldr w8, [x21, #0x674]
{let address=(cpu.x[21]).wrapping_add(0x674u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9ea0 cbnz w8, #0x10fc9d84
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fc9d84;continue;}pc=0x10fc9ea4;continue;
}
0x10fc9ea4=>{
// 0x10fc9ea4 b #0x10fc9d98
pc=0x10fc9d98;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc9cbc")),}}}
#[inline]
pub(super) unsafe fn f_10fca234(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fca234=>{
// 0x10fca234 ldr w0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fca238 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fca234")),}}}
#[inline]
pub(super) unsafe fn f_10fcc464(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcc464=>{
// 0x10fcc464 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcc468 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcc46c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcc470 ldr x19, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fcc474 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fcc478 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcc47c bl #0x10f7e050
cpu.x[30]=0x10fcc480;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fcc480;continue;
}
0x10fcc480=>{
// 0x10fcc480 bl #0x10de0494
cpu.x[30]=0x10fcc484;super::chunk2::f_10de0494(cpu,rt,0x10de0494)?;
pc=0x10fcc484;continue;
}
0x10fcc484=>{
// 0x10fcc484 cbz w0, #0x10fcc4a8
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fcc4a8;continue;}pc=0x10fcc488;continue;
}
0x10fcc488=>{
// 0x10fcc488 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcc48c str wzr, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcc490 bl #0x10f94608
cpu.x[30]=0x10fcc494;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10fcc494;continue;
}
0x10fcc494=>{
// 0x10fcc494 cbz x0, #0x10fcc49c
if (cpu.x[0]) == 0 {pc=0x10fcc49c;continue;}pc=0x10fcc498;continue;
}
0x10fcc498=>{
// 0x10fcc498 strb wzr, [x0, #0x3f]
{let address=(cpu.x[0]).wrapping_add(0x3fu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10fcc49c;continue;
}
0x10fcc49c=>{
// 0x10fcc49c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcc4a0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fcc4a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcc4a8=>{
// 0x10fcc4a8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcc4ac bl #0x10f940fc
cpu.x[30]=0x10fcc4b0;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcc4b0;continue;
}
0x10fcc4b0=>{
// 0x10fcc4b0 cbz x0, #0x10fcc4bc
if (cpu.x[0]) == 0 {pc=0x10fcc4bc;continue;}pc=0x10fcc4b4;continue;
}
0x10fcc4b4=>{
// 0x10fcc4b4 bl #0x10fc2980
cpu.x[30]=0x10fcc4b8;super::chunk4::f_10fc2980(cpu,rt,0x10fc2980)?;
pc=0x10fcc4b8;continue;
}
0x10fcc4b8=>{
// 0x10fcc4b8 cbnz w0, #0x10fcc488
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcc488;continue;}pc=0x10fcc4bc;continue;
}
0x10fcc4bc=>{
// 0x10fcc4bc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fcc4c0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcc4c4 str w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcc4c8 bl #0x10f94608
cpu.x[30]=0x10fcc4cc;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10fcc4cc;continue;
}
0x10fcc4cc=>{
// 0x10fcc4cc cbnz x0, #0x10fcc498
if (cpu.x[0]) != 0 {pc=0x10fcc498;continue;}pc=0x10fcc4d0;continue;
}
0x10fcc4d0=>{
// 0x10fcc4d0 b #0x10fcc49c
pc=0x10fcc49c;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcc464")),}}}
#[inline]
pub(super) unsafe fn f_10fd0468(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd0468=>{
// 0x10fd0468 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10fd046c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd0468")),}}}
#[inline]
pub(super) unsafe fn f_10fd04a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd04a8=>{
// 0x10fd04a8 ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd04ac ldr x0, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fd04b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd04a8")),}}}
#[inline]
pub(super) unsafe fn f_10fd04d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd04d0=>{
// 0x10fd04d0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd04d0")),}}}
#[inline]
pub(super) unsafe fn f_10fea43c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fea43c=>{
// 0x10fea43c stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fea440 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea444 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fea448 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fea44c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fea450 adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fea454 add x8, x8, #0xcb0
{let a=((cpu.x[8]));let b=((0xcb0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea458 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fea45c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fea460 stp x8, xzr, [x0], #0x10
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.x[0]=((cpu.x[0]).wrapping_add(((0x10u64)))) as u64;}
// 0x10fea464 bl #0x11058798
cpu.x[30]=0x10fea468;super::chunk1::f_11058798(cpu,rt,0x11058798)?;
pc=0x10fea468;continue;
}
0x10fea468=>{
// 0x10fea468 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10fea46c mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fea470 mov w1, #8
cpu.x[1]=(((0x8u64))) as u32 as u64;
// 0x10fea474 str xzr, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea478 mov w23, #8
cpu.x[23]=(((0x8u64))) as u32 as u64;
// 0x10fea47c str q0, [x21, #0x40]!
{let address=(cpu.x[21]).wrapping_add(0x40u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);cpu.x[21]=(address) as u64;}
// 0x10fea480 mov x22, x21
cpu.x[22]=(((cpu.x[21]))) as u64;
// 0x10fea484 str q0, [x22, #-0x20]!
{let address=(cpu.x[22]).wrapping_add(0xffffffffffffffe0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);cpu.x[22]=(address) as u64;}
// 0x10fea488 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fea48c stur q0, [x21, #-0x10]
{let address=(cpu.x[21]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fea490 str wzr, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea494 bl #0x108c5574
cpu.x[30]=0x10fea498;super::chunk12::f_108c5574(cpu,rt,0x108c5574)?;
pc=0x10fea498;continue;
}
0x10fea498=>{
// 0x10fea498 ldursw x8, [x21, #-0x14]
{let address=(cpu.x[21]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fea49c cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea4a0 b.gt #0x10fea4b8
if cpu.condition(12){pc=0x10fea4b8;continue;}pc=0x10fea4a4;continue;
}
0x10fea4a4=>{
// 0x10fea4a4 ldr x9, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fea4a8 str xzr, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea4ac add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea4b0 cmp x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fea4b4 b.ne #0x10fea4a4
if cpu.condition(1){pc=0x10fea4a4;continue;}pc=0x10fea4b8;continue;
}
0x10fea4b8=>{
// 0x10fea4b8 ldr w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea4bc str w23, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea4c0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea4c4 b.le #0x10fea4e8
if cpu.condition(13){pc=0x10fea4e8;continue;}pc=0x10fea4c8;continue;
}
0x10fea4c8=>{
// 0x10fea4c8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea4cc b.ne #0x10fea528
if cpu.condition(1){pc=0x10fea528;continue;}pc=0x10fea4d0;continue;
}
0x10fea4d0=>{
// 0x10fea4d0 str x19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea4d4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fea4d8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fea4dc ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fea4e0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fea4e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fea4e8=>{
// 0x10fea4e8 ldr w9, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fea4ec cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea4f0 b.le #0x10fea510
if cpu.condition(13){pc=0x10fea510;continue;}pc=0x10fea4f4;continue;
}
0x10fea4f4=>{
// 0x10fea4f4 sxtw x8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
pc=0x10fea4f8;continue;
}
0x10fea4f8=>{
// 0x10fea4f8 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fea4fc str xzr, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea500 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea504 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fea508 b.ne #0x10fea4f8
if cpu.condition(1){pc=0x10fea4f8;continue;}pc=0x10fea50c;continue;
}
0x10fea50c=>{
// 0x10fea50c b #0x10fea528
pc=0x10fea528;continue;
}
0x10fea510=>{
// 0x10fea510 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fea514 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fea518 bl #0x10feda98
cpu.x[30]=0x10fea51c;super::chunk0::f_10feda98(cpu,rt,0x10feda98)?;
pc=0x10fea51c;continue;
}
0x10fea51c=>{
// 0x10fea51c ldr w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea520 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea524 b.le #0x10fea4f4
if cpu.condition(13){pc=0x10fea4f4;continue;}pc=0x10fea528;continue;
}
0x10fea528=>{
// 0x10fea528 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fea52c str w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea530 str x19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea534 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fea538 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fea53c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fea540 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fea544 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fea43c")),}}}
#[inline]
pub(super) unsafe fn f_11056c0c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11056c0c=>{
// 0x11056c0c stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11056c10 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11056c14 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11056c18 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11056c1c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11056c20 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x11056c24 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x11056c28 mov x0, x2
cpu.x[0]=(((cpu.x[2]))) as u64;
// 0x11056c2c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11056c30 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x11056c34 bl #0x111dd1b0
cpu.x[30]=0x11056c38;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x11056c38;continue;
}
0x11056c38=>{
// 0x11056c38 cbz x20, #0x11056c80
if (cpu.x[20]) == 0 {pc=0x11056c80;continue;}pc=0x11056c3c;continue;
}
0x11056c3c=>{
// 0x11056c3c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x11056c40 bl #0x10e0b624
cpu.x[30]=0x11056c44;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
pc=0x11056c44;continue;
}
0x11056c44=>{
// 0x11056c44 ldr x8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c48 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11056c4c mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x11056c50 bl #0x10e0b624
cpu.x[30]=0x11056c54;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
pc=0x11056c54;continue;
}
0x11056c54=>{
// 0x11056c54 cmp w22, w0
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11056c58 b.eq #0x11056c94
if cpu.condition(0){pc=0x11056c94;continue;}pc=0x11056c5c;continue;
}
0x11056c5c=>{
// 0x11056c5c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c60 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11056c64 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c68 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11056c6c;super::dispatch(cpu,rt,target)?;}
// 0x11056c6c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11056c74 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c78 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11056c7c;super::dispatch(cpu,rt,target)?;}
// 0x11056c7c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
pc=0x11056c80;continue;
}
0x11056c80=>{
// 0x11056c80 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c84 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x11056c88 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c8c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11056c90;super::dispatch(cpu,rt,target)?;}
pc=0x11056c90;continue;
}
0x11056c90=>{
// 0x11056c90 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
pc=0x11056c94;continue;
}
0x11056c94=>{
// 0x11056c94 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056c98 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11056c9c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x11056ca0 ldr x8, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11056ca4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11056ca8;super::dispatch(cpu,rt,target)?;}
pc=0x11056ca8;continue;
}
0x11056ca8=>{
// 0x11056ca8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11056cac ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x11056cb0 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x11056cb4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x11056cb8 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x11056cbc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11056c0c")),}}}
#[inline]
pub(super) unsafe fn f_11057058(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11057058=>{
// 0x11057058 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1105705c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11057060 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11057064 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11057068 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1105706c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x11057070 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x11057074 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11057078 blr x8
{let target=cpu.x[8];cpu.x[30]=0x1105707c;super::dispatch(cpu,rt,target)?;}
pc=0x1105707c;continue;
}
0x1105707c=>{
// 0x1105707c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11057080 str w0, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11057084 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x11057088 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1105708c blr x8
{let target=cpu.x[8];cpu.x[30]=0x11057090;super::dispatch(cpu,rt,target)?;}
pc=0x11057090;continue;
}
0x11057090=>{
// 0x11057090 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11057094 str w0, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11057098 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1105709c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110570a4;super::dispatch(cpu,rt,target)?;}
pc=0x110570a4;continue;
}
0x110570a4=>{
// 0x110570a4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570a8 str w0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110570ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110570b0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110570b8;super::dispatch(cpu,rt,target)?;}
pc=0x110570b8;continue;
}
0x110570b8=>{
// 0x110570b8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570bc str w0, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110570c0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110570c4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110570cc;super::dispatch(cpu,rt,target)?;}
pc=0x110570cc;continue;
}
0x110570cc=>{
// 0x110570cc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570d0 str w0, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110570d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110570d8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110570dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110570e0;super::dispatch(cpu,rt,target)?;}
pc=0x110570e0;continue;
}
0x110570e0=>{
// 0x110570e0 str w0, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110570e4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110570e8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x110570ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11057058")),}}}
#[inline]
pub(super) unsafe fn f_111dd4e8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd4e8=>{
// 0x111dd4e8 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd4ec str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dd4f0 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd4f4 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd4f8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd4fc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd500 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dd504 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111dd508 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd50c blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd510;super::dispatch(cpu,rt,target)?;}
pc=0x111dd510;continue;
}
0x111dd510=>{
// 0x111dd510 bic w8, w0, w0, asr #31
{let a=((cpu.x[0] as u32 as u64));let b=(((cpu.x[0] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x111dd514 mov w9, #0x86a0
cpu.x[9]=(((0x86a0u64))) as u32 as u64;
// 0x111dd518 ldr w10, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x111dd51c movk w9, #1, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x10000u64) as u32 as u64;
// 0x111dd520 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111dd524 str wzr, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd528 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd52c csel w21, w8, w9, lt
cpu.x[21]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111dd530 cmp w10, w21
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd534 b.ge #0x111dd544
if cpu.condition(10){pc=0x111dd544;continue;}pc=0x111dd538;continue;
}
0x111dd538=>{
// 0x111dd538 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dd53c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111dd540 bl #0x1078f4d4
cpu.x[30]=0x111dd544;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x111dd544;continue;
}
0x111dd544=>{
// 0x111dd544 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd548 b.lt #0x111dd5b4
if cpu.condition(11){pc=0x111dd5b4;continue;}pc=0x111dd54c;continue;
}
0x111dd54c=>{
// 0x111dd54c mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x111dd550 b #0x111dd56c
pc=0x111dd56c;continue;
}
0x111dd554=>{
// 0x111dd554 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd558 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111dd55c subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111dd560 str w10, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd564 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd568 b.eq #0x111dd5b4
if cpu.condition(0){pc=0x111dd5b4;continue;}pc=0x111dd56c;continue;
}
0x111dd56c=>{
// 0x111dd56c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd570 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111dd574 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd578 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd57c;super::dispatch(cpu,rt,target)?;}
pc=0x111dd57c;continue;
}
0x111dd57c=>{
// 0x111dd57c ldp w9, w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111dd580 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd584 b.ne #0x111dd554
if cpu.condition(1){pc=0x111dd554;continue;}pc=0x111dd588;continue;
}
0x111dd588=>{
// 0x111dd588 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111dd58c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd590 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111dd594 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd598 b.ge #0x111dd554
if cpu.condition(10){pc=0x111dd554;continue;}pc=0x111dd59c;continue;
}
0x111dd59c=>{
// 0x111dd59c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111dd5a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dd5a4 bl #0x1078f4d4
cpu.x[30]=0x111dd5a8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111dd5a8 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111dd5ac mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x111dd5b0 b #0x111dd554
pc=0x111dd554;continue;
}
0x111dd5b4=>{
// 0x111dd5b4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd5b8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111dd5bc ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111dd5c0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x111dd5c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd4e8")),}}}
#[inline]
pub(super) unsafe fn f_111dd770(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd770=>{
// 0x111dd770 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd774 ldr x1, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111dd778 br x1
{let target=cpu.x[1];if target>=0x111dd770 && target<0x111dd77c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd770")),}}}
#[inline]
pub(super) unsafe fn f_1128dbe8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128dbe8=>{
// 0x1128dbe8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128dbec ldr x2, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x1128dbf0 br x2
{let target=cpu.x[2];if target>=0x1128dbe8 && target<0x1128dbf4{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128dbe8")),}}}
#[inline]
pub(super) unsafe fn f_1128dc50(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128dc50=>{
// 0x1128dc50 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128dc54 ror w8, w8, #0x1f
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as u32).rotate_right(((((0x1fu64))) & 31) as u32) as u64) as u32 as u64;
// 0x1128dc58 add w8, w1, w8
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128dc5c add w8, w8, #9
{let a=((cpu.x[8] as u32 as u64));let b=((0x9u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128dc60 str w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128dc64 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128dc50")),}}}
#[inline]
pub(super) unsafe fn f_11294d18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11294d18=>{
// 0x11294d18 ldr w8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294d1c mov w9, #-0xb505
cpu.x[9]=(((0xffffffffffff4afbu64))) as u32 as u64;
// 0x11294d20 mov w11, #0x95f7
cpu.x[11]=(((0x95f7u64))) as u32 as u64;
// 0x11294d24 movk w11, #0xfffe, lsl #16
cpu.x[11]=((cpu.x[11] as u32 as u64 & !(0xffff0000u64)) | 0xfffe0000u64) as u32 as u64;
// 0x11294d28 sub w8, w1, w8
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294d2c add w10, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294d30 cmp w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294d34 b.hs #0x11294d40
if cpu.condition(2){pc=0x11294d40;continue;}pc=0x11294d38;continue;
}
0x11294d38=>{
// 0x11294d38 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294d3c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x11294d40=>{
// 0x11294d40 ldr w10, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x11294d44 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294d48 sub w10, w2, w10
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294d4c add w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x11294d50 cmp w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294d54 b.lo #0x11294d70
if cpu.condition(3){pc=0x11294d70;continue;}pc=0x11294d58;continue;
}
0x11294d58=>{
// 0x11294d58 mul w8, w8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x11294d5c mul w9, w10, w10
cpu.x[9]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x11294d60 eor w10, w8, #0x7fffffff
{let a=((cpu.x[8] as u32 as u64));let b=((0x7fffffffu64));let result=a ^ b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294d64 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294d68 cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294d6c csel w0, w8, w0, lo
cpu.x[0]=(if cpu.condition(3){((cpu.x[8] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
pc=0x11294d70;continue;
}
0x11294d70=>{
// 0x11294d70 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11294d18")),}}}
#[inline]
pub(super) unsafe fn f_11294d74(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11294d74=>{
// 0x11294d74 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11294d78 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11294d7c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11294d80 ldr w8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294d84 mov w11, #-0xb505
cpu.x[11]=(((0xffffffffffff4afbu64))) as u32 as u64;
// 0x11294d88 mov w10, #0x95f7
cpu.x[10]=(((0x95f7u64))) as u32 as u64;
// 0x11294d8c movk w10, #0xfffe, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0xfffe0000u64) as u32 as u64;
// 0x11294d90 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11294d94 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x11294d98 add w9, w8, w11
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x11294d9c cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294da0 b.hs #0x11294dac
if cpu.condition(2){pc=0x11294dac;continue;}pc=0x11294da4;continue;
}
0x11294da4=>{
// 0x11294da4 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294da8 b #0x11294dd8
pc=0x11294dd8;continue;
}
0x11294dac=>{
// 0x11294dac ldr w9, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x11294db0 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294db4 add w11, w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x11294db8 cmp w11, w10
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294dbc b.lo #0x11294dd8
if cpu.condition(3){pc=0x11294dd8;continue;}pc=0x11294dc0;continue;
}
0x11294dc0=>{
// 0x11294dc0 mul w8, w8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x11294dc4 mul w9, w9, w9
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x11294dc8 eor w10, w8, #0x7fffffff
{let a=((cpu.x[8] as u32 as u64));let b=((0x7fffffffu64));let result=a ^ b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294dcc add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294dd0 cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294dd4 csel w0, w8, w0, lo
cpu.x[0]=(if cpu.condition(3){((cpu.x[8] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
pc=0x11294dd8;continue;
}
0x11294dd8=>{
// 0x11294dd8 bl #0x112944a4
cpu.x[30]=0x11294ddc;super::chunk0::f_112944a4(cpu,rt,0x112944a4)?;
pc=0x11294ddc;continue;
}
0x11294ddc=>{
// 0x11294ddc cbz w0, #0x11294df8
if (cpu.x[0] as u32 as u64) == 0 {pc=0x11294df8;continue;}pc=0x11294de0;continue;
}
0x11294de0=>{
// 0x11294de0 ldp w8, w9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x11294de4 mul w8, w8, w20
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[20] as u32 as u64)))) as u32 as u64;
// 0x11294de8 mul w9, w9, w20
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[20] as u32 as u64)))) as u32 as u64;
// 0x11294dec sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x11294df0 sdiv w9, w9, w0
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x11294df4 stp w8, w9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
pc=0x11294df8;continue;
}
0x11294df8=>{
// 0x11294df8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x11294dfc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x11294e00 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11294d74")),}}}
#[inline]
pub(super) unsafe fn f_112af1b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112af1b8=>{
// 0x112af1b8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x112af1bc str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x112af1c0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x112af1c4 adrp x8, #0x11a4c000
cpu.x[8]=(0x11a4c000u64) as u64;
// 0x112af1c8 add x8, x8, #0xd68
{let a=((cpu.x[8]));let b=((0xd68u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x112af1cc ldarb w9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x112af1d0 adrp x8, #0x11a4c000
cpu.x[8]=(0x11a4c000u64) as u64;
// 0x112af1d4 tbz w9, #0, #0x112af204
if ((cpu.x[9] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x112af204;continue;}pc=0x112af1d8;continue;
}
0x112af1d8=>{
// 0x112af1d8 ldr x19, [x8, #0xd60]
{let address=(cpu.x[8]).wrapping_add(0xd60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x112af1dc bl #0x11467408
cpu.x[30]=0x112af1e0;super::chunk10::f_11467408(cpu,rt,0x11467408)?;
pc=0x112af1e0;continue;
}
0x112af1e0=>{
// 0x112af1e0 sub x8, x0, x19
{let a=((cpu.x[0]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x112af1e4 scvtf d0, x8
cpu.q[0]=(((cpu.x[8]) as i64 as f64).to_bits()) as u128 & 0xffffffffffffffffu128;
// 0x112af1e8 adrp x8, #0x102b1000
cpu.x[8]=(0x102b1000u64) as u64;
// 0x112af1ec ldr d1, [x8, #0x980]
{let address=(cpu.x[8]).wrapping_add(0x980u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x112af1f0 fmul d0, d0, d1
fp_domain(cpu,&[f64::from_bits(((cpu.q[0] & 0xffffffffffffffffu128) as u64) as u64) as f64,f64::from_bits(((cpu.q[1] & 0xffffffffffffffffu128) as u64) as u64) as f64])?;cpu.q[0]=((f64::from_bits(((cpu.q[0] & 0xffffffffffffffffu128) as u64) as u64)*f64::from_bits(((cpu.q[1] & 0xffffffffffffffffu128) as u64) as u64)).to_bits()) as u128 & 0xffffffffffffffffu128;fp_domain(cpu,&[f64::from_bits(((cpu.q[0] & 0xffffffffffffffffu128) as u64) as u64) as f64])?;
// 0x112af1f4 fcvtzs w0, d0
fp_domain(cpu,&[f64::from_bits(((cpu.q[0] & 0xffffffffffffffffu128) as u64) as u64) as f64])?;cpu.x[0]=((f64::from_bits(((cpu.q[0] & 0xffffffffffffffffu128) as u64) as u64).trunc() as i32) as u64) as u32 as u64;
// 0x112af1f8 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x112af1fc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x112af200 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x112af204=>{
// 0x112af204 adrp x0, #0x11a4c000
cpu.x[0]=(0x11a4c000u64) as u64;
// 0x112af208 add x0, x0, #0xd68
{let a=((cpu.x[0]));let b=((0xd68u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x112af20c bl #0x1187d2f0
cpu.x[30]=0x112af210;super::dispatch(cpu,rt,0x1187d2f0)?;
// 0x112af210 adrp x8, #0x11a4c000
cpu.x[8]=(0x11a4c000u64) as u64;
// 0x112af214 cbz w0, #0x112af1d8
if (cpu.x[0] as u32 as u64) == 0 {pc=0x112af1d8;continue;}pc=0x112af218;continue;
}
0x112af218=>{
// 0x112af218 bl #0x11467408
cpu.x[30]=0x112af21c;super::chunk10::f_11467408(cpu,rt,0x11467408)?;
// 0x112af21c adrp x8, #0x11a4c000
cpu.x[8]=(0x11a4c000u64) as u64;
// 0x112af220 str x0, [x8, #0xd60]
{let address=(cpu.x[8]).wrapping_add(0xd60u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x112af224 adrp x0, #0x11a4c000
cpu.x[0]=(0x11a4c000u64) as u64;
// 0x112af228 add x0, x0, #0xd68
{let a=((cpu.x[0]));let b=((0xd68u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x112af22c bl #0x1187d300
cpu.x[30]=0x112af230;super::dispatch(cpu,rt,0x1187d300)?;
// 0x112af230 adrp x8, #0x11a4c000
cpu.x[8]=(0x11a4c000u64) as u64;
// 0x112af234 b #0x112af1d8
pc=0x112af1d8;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112af1b8")),}}}
